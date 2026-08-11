use std::collections::BTreeMap;

use dioxus_stores::Store;
use serde::{Deserialize, Serialize};

use crate::device::clusters::Clusters ;

#[derive(Debug, Clone, Serialize, Deserialize, Store)]
pub struct Device {
    pub user_given_name: String,
    pub product_name: String,
    pub vendor_name: String,
    pub endpoints: BTreeMap<u16, Endpoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Store)]
pub struct Endpoint {
    pub device_types: Vec<u32>,
    pub clusters: Clusters,
}

#[cfg(feature = "backend")]
mod impl_from_endpoint {
    use anyhow;
    use matc::controller::Connection;

    use super::*;
    use crate::backend::FromEndpoint;

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
