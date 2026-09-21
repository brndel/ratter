use matter_clusters::r#gen::operational_credentials::FabricDescriptorStruct;
use serde::{Deserialize, Serialize};

use crate::device::clusters::define_cluster_macro::define_cluster;

define_cluster!(
struct OperationalCredentials, enum OperationalCredentialsChange, operational_credentials {
    fabrics: Vec<FabricDescriptor> => FABRICS "listen" as Fabrics { decode_fabrics => fabric_descriptors },
    supported_fabrics: u8 => SUPPORTED_FABRICS as SupportedFabrics { decode_supported_fabrics },
    commissioned_fabrics: u8 => COMMISSIONED_FABRICS "listen" as CommissionedFabrics { decode_commissioned_fabrics },
    current_fabric_index: u8 => CURRENT_FABRIC_INDEX "listen" as CurrentFabricIndex { decode_current_fabric_index }
}
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FabricDescriptor {
    // pub root_public_key: Vec<u8>,
    pub vendor_id: u16,
    pub fabric_id: u64,
    pub node_id: u64,
    pub label: String,
    // pub vid_verification_statement: Option<Vec<u8>>,
    pub fabric_index: u8,
}

fn fabric_descriptors(input: Vec<FabricDescriptorStruct>) -> Vec<FabricDescriptor> {
    input
        .into_iter()
        .map(|fabric| FabricDescriptor {
            // root_public_key: fabric.root_public_key,
            vendor_id: fabric.vendor_id,
            fabric_id: fabric.fabric_id,
            node_id: fabric.node_id,
            label: fabric.label,
            // vid_verification_statement: fabric.vid_verification_statement,
            fabric_index: fabric.fabric_index,
        })
        .collect()
}
