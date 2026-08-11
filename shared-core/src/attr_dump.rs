use std::{collections::BTreeMap, fmt::Display};

use serde::{Deserialize, Serialize};

use crate::{
    device::clusters::get_cluster_name,
    id::{AttrId, ClusterId, EndpointId},
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttrDump {
    pub endpoint: EndpointId,
    pub cluster: ClusterId,
    pub attr: AttrId,
    pub value: AttrDumpValue,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AttrDumpContainer {
    endpoints: BTreeMap<EndpointId, BTreeMap<ClusterId, BTreeMap<AttrId, AttrDumpValue>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttrDumpValue {
    pub attr_name: String,
    pub value: Result<String, String>,
}

impl AttrDumpContainer {
    pub fn add_attr(&mut self, attr: AttrDump) {
        let endpoint = self.endpoints.entry(attr.endpoint).or_default();
        let cluster = endpoint.entry(attr.cluster).or_default();

        cluster.insert(attr.attr, attr.value);
    }
}

impl Display for AttrDumpContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (endpoint, clusters) in &self.endpoints {
            writeln!(f, "endpoint {}", endpoint)?;

            for (cluster, attrs) in clusters {
                writeln!(
                    f,
                    "  0x{:x} {}",
                    cluster,
                    get_cluster_name(*cluster).unwrap_or("<unkown>")
                )?;

                for (attr, value) in attrs {
                    write!(f, "    0x{:x} {}: ", attr, value.attr_name)?;

                    match &value.value {
                        Ok(value) => {
                            writeln!(f, "{value}",)?;
                        }
                        Err(err) => writeln!(f, "Error: '{err}'")?,
                    }
                }
            }
        }

        Ok(())
    }
}
