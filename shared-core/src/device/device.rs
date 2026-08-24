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
    use matter_controller::Node;

    use super::*;
    use crate::{backend::{FromEndpoint, FromNode}, read_decode};

    impl FromNode for Device {
        async fn from_node(node: &Node) -> anyhow::Result<Self> {
            let endpoint = 0;
            read_decode!(
                node, endpoint, [
                    product_name = {basic_information, PRODUCT_NAME, decode_product_name},
                    vendor_name = {basic_information, VENDOR_NAME, decode_vendor_name},
                    parts = {descriptor, PARTS_LIST, decode_parts_list}
                ]
            );

            let endpoints = {
                let endpoint_ids = parts;

                let mut endpoints = BTreeMap::new();
                endpoints.insert(0, Endpoint::from_endpoint(node, 0).await?);

                for id in endpoint_ids {
                    let endpoint = Endpoint::from_endpoint(node, id).await?;
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
        async fn from_endpoint(node: &Node, endpoint: u16) -> anyhow::Result<Self> {
            read_decode!(
                node, endpoint, [
                    device_types = {descriptor, DEVICE_TYPE_LIST, decode_device_type_list}
                ]
            );

            let device_types = device_types
                .into_iter()
                .map(|device_type| {
                    device_type
                        .device_type
                })
                .collect();

            let clusters = Clusters::from_endpoint(node, endpoint).await?;

            Ok(Self {
                device_types,
                clusters,
            })
        }
    }
}
