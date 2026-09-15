use std::{collections::HashMap, ops::RangeInclusive};

use serde::{Deserialize, Serialize};

use crate::id::{ProductId, VendorId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OtaManagerClientEvent {
    VersionChanged {
        product: OtaProductId,
        versions: Result<Vec<OtaVersion>, String>,
    },
    ImageDownloaded {
        product: OtaProductId,
        version: u32,
    },
}

#[derive(Default, Serialize, Deserialize)]
pub struct OtaManagerClient {
    versions: HashMap<OtaProductId, Result<Vec<OtaVersion>, String>>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct OtaProductId {
    pub vendor_id: VendorId,
    pub product_id: ProductId,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub struct OtaVersion {
    pub version: u32,
    pub version_string: String,
    pub ota_url: String,
    pub ota_checksum: String,
    pub ota_checksum_type: OtaChecksumType,
    pub applicable_version_range: RangeInclusive<u32>,
    pub release_notes_url: String,
    pub is_image_downloaded: bool,
}

#[derive(Debug, Clone, Hash, Serialize, Deserialize, PartialEq, Eq)]
pub enum OtaChecksumType {
    None = 0,
    Sha256 = 1,
    Sha384 = 7,
    Sha512 = 8,
    Sha3_256 = 10,
    Sha3_384 = 11,
    Sha3_512 = 12,
}

impl TryFrom<u8> for OtaChecksumType {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Sha256),
            7 => Ok(Self::Sha384),
            8 => Ok(Self::Sha512),
            10 => Ok(Self::Sha3_256),
            11 => Ok(Self::Sha3_384),
            12 => Ok(Self::Sha3_512),
            _ => Err(value),
        }
    }
}

impl OtaManagerClient {
    pub fn new(versions: HashMap<OtaProductId, Result<Vec<OtaVersion>, String>>) -> Self {
        Self { versions }
    }

    pub fn versions(&self, device: OtaProductId) -> Option<&Result<Vec<OtaVersion>, String>> {
        self.versions.get(&device)
    }


    pub fn handle_event(&mut self, event: OtaManagerClientEvent) {
        match event {
            OtaManagerClientEvent::VersionChanged { product, versions } => {
                self.versions.insert(product, versions);
            }
            OtaManagerClientEvent::ImageDownloaded { product, version } => {
                if let Some(Ok(versions)) = self.versions.get_mut(&product)
                    && let Some(ota_version) = versions
                        .iter_mut()
                        .find(|ota_version| ota_version.version == version)
                {
                    ota_version.is_image_downloaded = true
                }
            }
        }
    }
}
