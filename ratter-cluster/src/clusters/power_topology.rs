#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const POWER_TOPOLOGY: ClusterDef = ClusterDef {
  id: 0x009c,
  name: "Power Topology",
  features: &[
    FeatureDef { bit: 0x0000, code: "NODE", name: "NodeTopology", summary: "This endpoint provides or consumes power to/from the entire node" },
    FeatureDef { bit: 0x0001, code: "TREE", name: "TreeTopology", summary: "This endpoint provides or consumes power to/from itself and its child endpoints" },
    FeatureDef { bit: 0x0002, code: "SET", name: "SetTopology", summary: "This endpoint provides or consumes power to/from a specified set of endpoints" },
    FeatureDef { bit: 0x0003, code: "DYPF", name: "DynamicPowerFlow", summary: "The specified set of endpoints may change" },
  ],
  attributes: &[
    attr::AvailableEndpoints::ATTR,
    attr::ActiveEndpoints::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct AvailableEndpoints;

  impl Attr for AvailableEndpoints {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "AvailableEndpoints",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = Vec<TypeUnsupported /* EndpointNo */>;
  }


  pub struct ActiveEndpoints;

  impl Attr for ActiveEndpoints {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "ActiveEndpoints",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = Vec<TypeUnsupported /* EndpointNo */>;
  }


}
pub mod ty {
  pub use crate::globals::*;
}
