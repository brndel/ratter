#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const PRESSURE_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x0403,
  name: "Pressure Measurement",
  features: &[
    FeatureDef { bit: 0x0000, code: "EXT", name: "Extended", summary: "Extended range and resolution" },
  ],
  attributes: &[
    attr::MeasuredValue::ATTR,
    attr::MinMeasuredValue::ATTR,
    attr::MaxMeasuredValue::ATTR,
    attr::Tolerance::ATTR,
    attr::ScaledValue::ATTR,
    attr::MinScaledValue::ATTR,
    attr::MaxScaledValue::ATTR,
    attr::ScaledTolerance::ATTR,
    attr::Scale::ATTR,
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
    type Value = Nullable<i16>;
  }


  pub struct MinMeasuredValue;

  impl Attr for MinMeasuredValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "MinMeasuredValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<i16>;
  }


  pub struct MaxMeasuredValue;

  impl Attr for MaxMeasuredValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "MaxMeasuredValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<i16>;
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


  pub struct ScaledValue;

  impl Attr for ScaledValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0010,
      name: "ScaledValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<i16>;
  }


  pub struct MinScaledValue;

  impl Attr for MinScaledValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0011,
      name: "MinScaledValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<i16>;
  }


  pub struct MaxScaledValue;

  impl Attr for MaxScaledValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0012,
      name: "MaxScaledValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<i16>;
  }


  pub struct ScaledTolerance;

  impl Attr for ScaledTolerance {
    const ATTR: AttrDef = AttrDef {
      id: 0x0013,
      name: "ScaledTolerance",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct Scale;

  impl Attr for Scale {
    const ATTR: AttrDef = AttrDef {
      id: 0x0014,
      name: "Scale",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = i8;
  }


}
pub mod ty {
  pub use crate::globals::*;
}
