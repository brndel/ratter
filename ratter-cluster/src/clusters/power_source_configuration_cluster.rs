#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const POWER_SOURCE_CONFIGURATION: ClusterDef = ClusterDef {
  id: 0x002e,
  name: "Power Source Configuration",
  features: &[
  ],
  attributes: &[
    attr::Sources::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct Sources;

  impl Attr for Sources {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "Sources",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<TypeUnsupported /* EndpointNo */>;
  }


}
pub mod ty {
  pub use crate::globals::*;
}
