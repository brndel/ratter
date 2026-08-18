use std::collections::BTreeMap;

use dioxus_stores::Store;
use serde::{Deserialize, Serialize};

use crate::device::clusters::Clusters;

#[derive(Debug, Clone, Serialize, Deserialize, Store)]
pub struct Device {
    pub product_name: String,
    pub vendor_name: String,
    pub endpoints: BTreeMap<u16, Endpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Store)]
pub struct Endpoint {
    pub device_types: Vec<u32>,
    pub clusters: Clusters,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceCommissionMode {
    Ble,
    SharedCode,
}

#[cfg(feature = "backend")]
mod impl_from_traits {
    use anyhow;
    use matc::controller::Connection;

    use super::*;
    use crate::backend::{FromConnection, FromEndpoint};

    impl FromConnection for Device {
        async fn from_connection(connection: &Connection) -> anyhow::Result<Self> {
            use matc::clusters::codec::{basic_information_cluster, descriptor_cluster};

            let product_name = basic_information_cluster::read_product_name(connection, 0).await?;

            let vendor_name = basic_information_cluster::read_vendor_name(connection, 0).await?;

            let endpoints = {
                let endpoint_ids = descriptor_cluster::read_parts_list(connection, 0).await?;

                let mut endpoints = BTreeMap::new();
                endpoints.insert(0, Endpoint::from_endpoint(connection, 0).await?);

                for id in endpoint_ids {
                    let endpoint = Endpoint::from_endpoint(connection, id).await?;
                    endpoints.insert(id, endpoint);
                }

                endpoints
            };

            Ok(Device {
                product_name,
                vendor_name,
                endpoints,
            })
        }
    }

    impl FromEndpoint for Endpoint {
        async fn from_endpoint(connection: &Connection, endpoint: u16) -> anyhow::Result<Self> {
            use matc::clusters::codec::descriptor_cluster;

            let device_types =
                descriptor_cluster::read_device_type_list(connection, endpoint).await?;
            let device_types = device_types
                .into_iter()
                .map(|device_type| {
                    device_type
                        .device_type
                        .ok_or(anyhow::anyhow!("Empty device type"))
                })
                .collect::<anyhow::Result<Vec<_>>>()?;

            let clusters = Clusters::from_endpoint(connection, endpoint).await?;

            Ok(Self {
                device_types,
                clusters,
            })
        }
    }
}
