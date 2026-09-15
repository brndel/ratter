use std::{
    collections::HashMap, path::{Path, PathBuf},
};

use base64::{Engine, prelude::BASE64_STANDARD};
use dioxus::fullstack::reqwest;
use nest_struct::nest_struct;
use serde::Deserialize;
use sha2::{Digest, digest::DynDigest};
use shared_core::ota::{OtaChecksumType, OtaManagerClient, OtaManagerClientEvent, OtaProductId, OtaVersion};

use crate::event_bus::ClientBusSender;

pub struct OtaManager {
    sender: ClientBusSender<OtaManagerClientEvent>,
    versions: HashMap<OtaProductId, Result<Vec<OtaVersion>, String>>,
}

#[derive(Debug, thiserror::Error)]
pub enum FetchOtaError {
    #[error("Version not loaded")]
    NotLoaded,
    #[error("This version does not have an image url available")]
    NoImageUrl,
    #[error("Loaded image has wrong checksum")]
    ChecksumMissmatch,
    #[error("Invalid checksum type {0}")]
    InvalidChecksumType(u8),
    #[error("Reqwest error: {0:?}")]
    Reqwest(#[from] reqwest::Error),
    #[error("Error while loading cached image from disk: {0:?}")]
    LocalWriteFailed(#[from] tokio::io::Error),
}

impl OtaManager {
    const LOCAL_IMAGE_DIRECTORY: &str = "ota_images";
    const BASE_URL: &str = "https://on.dcl.csa-iot.org/dcl/model/versions/";

    pub async fn new(sender: ClientBusSender<OtaManagerClientEvent>) -> Self {
        tokio::fs::create_dir_all(Self::LOCAL_IMAGE_DIRECTORY)
            .await
            .unwrap();
        Self {
            sender,
            versions: Default::default(),
        }
    }

    pub fn client(&self) -> OtaManagerClient {
        OtaManagerClient::new(self.versions.clone())
    }

    pub async fn fetch_ota_versions(&mut self, product: OtaProductId) {
        let new_versions = Self::fetch_ota_versions_inner(product)
            .await
            .map_err(|err| err.to_string());

        if let Some(current_versions) = self.versions.get(&product) {
            if current_versions == &new_versions {
                return;
            }
        }

        let _ = self.sender.send_if(|| OtaManagerClientEvent::VersionChanged {
            product,
            versions: new_versions.clone(),
        });
        self.versions.insert(product, new_versions);
    }

    async fn fetch_ota_versions_inner(product: OtaProductId) -> Result<Vec<OtaVersion>, FetchOtaError> {
        let url = format!(
            "{}/{}/{}",
            Self::BASE_URL,
            product.vendor_id,
            product.product_id
        );

        let result = reqwest::get(&url)
            .await?
            .json::<OtaDeviceVersionsResponse>()
            .await?;

        let mut versions = Vec::new();

        for version in result.model_versions.software_versions {
            let url = format!(
                "{}/{}/{}/{}",
                Self::BASE_URL,
                product.vendor_id,
                product.product_id,
                version
            );
            let result = reqwest::get(&url)
                .await?
                .json::<OtaVersionResponse>()
                .await?;

            let mut version = OtaVersion::try_from(result).map_err(FetchOtaError::InvalidChecksumType)?;
            if Self::is_image_on_disk(&version.ota_url).await {
                version.is_image_downloaded = true;
            }

            versions.push(version)
        }

        Ok(versions)
    }

    pub async fn fetch_ota_image(
        &mut self,
        product: OtaProductId,
        version: u32,
    ) -> Result<Vec<u8>, FetchOtaError> {
        let ota_version = self
            .versions
            .get_mut(&product)
            .and_then(|x| x.as_mut().ok())
            .and_then(|versions| versions.iter_mut().find(|ota| ota.version == version))
            .ok_or(FetchOtaError::NotLoaded)?;

        let image = {
            let path = Self::ota_url_to_local_path(&ota_version.ota_url)
                .ok_or(FetchOtaError::NoImageUrl)?;

            if let Ok(image) = tokio::fs::read(&path).await {
                image
            } else {
                let image = Self::download_image(&ota_version.ota_url).await?;

                tokio::fs::write(&path, &image).await?;

                ota_version.is_image_downloaded = true;
                let _ = self
                    .sender
                    .send(OtaManagerClientEvent::ImageDownloaded { product, version });

                image
            }
        };

        if !Self::validate_checksum(&image, &ota_version) {
            return Err(FetchOtaError::ChecksumMissmatch);
        }

        Ok(image)
    }

    fn ota_url_to_local_path(ota_url: &str) -> Option<PathBuf> {
        if ota_url.len() == 0 {
            return None;
        }

        let as_path: &Path = ota_url.as_ref();

        let filename = as_path.file_name()?.to_str()?;
        let mut path = PathBuf::new();

        path.push(Self::LOCAL_IMAGE_DIRECTORY);
        path.push(filename);

        Some(path)
    }

    async fn is_image_on_disk(ota_url: &str) -> bool {
        let Some(path) = Self::ota_url_to_local_path(ota_url) else {
            return false;
        };

        tokio::fs::try_exists(&path).await.is_ok_and(|x| x)
    }

    async fn download_image(ota_url: &str) -> Result<Vec<u8>, reqwest::Error> {
        let image = reqwest::get(ota_url).await?.bytes().await?;

        Ok(image.to_vec())
    }

    fn validate_checksum(image: &[u8], version: &OtaVersion) -> bool {
        let mut hasher: Box<dyn DynDigest> = match version.ota_checksum_type {
            OtaChecksumType::None => return false,
            OtaChecksumType::Sha256 => Box::new(sha2::Sha256::new()),
            OtaChecksumType::Sha384 => Box::new(sha2::Sha384::new()),
            OtaChecksumType::Sha512 => Box::new(sha2::Sha512::new()),
            OtaChecksumType::Sha3_256 => Box::new(sha3::Sha3_256::new()),
            OtaChecksumType::Sha3_384 => Box::new(sha3::Sha3_384::new()),
            OtaChecksumType::Sha3_512 => Box::new(sha3::Sha3_512::new()),
        };

        hasher.update(image);
        let image_checksum = hasher.finalize();
        let Ok(version_checksum) = BASE64_STANDARD.decode(&version.ota_checksum) else {
            return false;
        };

        image_checksum.as_ref() == version_checksum.as_slice()
    }
}

#[nest_struct]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct OtaDeviceVersionsResponse {
    model_versions: nest! {
        software_versions: Vec<u32>
    },
}

#[nest_struct]
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtaVersionResponse {
    model_version: nest! {
        software_version: u32,
        software_version_string: String,
        ota_url: String,
        // ota_file_size: String,
        ota_checksum: String,
        ota_checksum_type: u8,
        min_applicable_software_version: u32,
        max_applicable_software_version: u32,
        release_notes_url: String,
    },
}

impl TryFrom<OtaVersionResponse> for OtaVersion {
    type Error = <OtaChecksumType as TryFrom<u8>>::Error;

    fn try_from(value: OtaVersionResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            version: value.model_version.software_version,
            version_string: value.model_version.software_version_string,
            ota_url: value.model_version.ota_url,
            ota_checksum: value.model_version.ota_checksum,
            ota_checksum_type: value.model_version.ota_checksum_type.try_into()?,
            applicable_version_range: value.model_version.min_applicable_software_version
                ..=value.model_version.max_applicable_software_version,
            release_notes_url: value.model_version.release_notes_url,
            is_image_downloaded: false,
        })
    }
}
