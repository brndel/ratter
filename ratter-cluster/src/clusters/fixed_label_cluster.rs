#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const FIXED_LABEL: ClusterDef = ClusterDef {
  id: 0x0040,
  name: "Fixed Label",
  features: &[
  ],
  attributes: &[
    attr::LabelList::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct LabelList;

  impl Attr for LabelList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "LabelList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::LabelStruct>;
  }


}
pub mod ty {
  pub use crate::globals::*;
  pub use crate::clusters::label_cluster::ty::*;
}
