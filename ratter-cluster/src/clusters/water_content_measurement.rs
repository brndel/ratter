#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const RELATIVE_HUMIDITY_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x0405,
  name: "Relative Humidity Measurement",
  features: &[
  ],
  attributes: &[
    attr::MeasuredValue::ATTR,
    attr::MinMeasuredValue::ATTR,
    attr::MaxMeasuredValue::ATTR,
    attr::Tolerance::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct MeasuredValue;

  impl Attr for MeasuredValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "MeasuredValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u16>;
  }


  pub struct MinMeasuredValue;

  impl Attr for MinMeasuredValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "MinMeasuredValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u16>;
  }


  pub struct MaxMeasuredValue;

  impl Attr for MaxMeasuredValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "MaxMeasuredValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u16>;
  }


  pub struct Tolerance;

  impl Attr for Tolerance {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "Tolerance",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


}
pub mod ty {
  pub use crate::globals::*;
}
