#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const WAKE_ON_LAN: ClusterDef = ClusterDef {
  id: 0x0503,
  name: "Wake On LAN",
  features: &[
  ],
  attributes: &[
    attr::MACAddress::ATTR,
    attr::LinkLocalAddress::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct MACAddress;

  impl Attr for MACAddress {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "MACAddress",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = String;
  }


  pub struct LinkLocalAddress;

  impl Attr for LinkLocalAddress {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "LinkLocalAddress",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = TypeUnsupported /* Ipv6adr */;
  }


}
pub mod ty {
  pub use crate::globals::*;
}
