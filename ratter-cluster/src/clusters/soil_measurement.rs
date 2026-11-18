#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const SOIL_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x0430,
  name: "Soil Measurement",
  features: &[
  ],
  attributes: &[
    attr::SoilMoistureMeasurementLimits::ATTR,
    attr::SoilMoistureMeasuredValue::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct SoilMoistureMeasurementLimits;

  impl Attr for SoilMoistureMeasurementLimits {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "SoilMoistureMeasurementLimits",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::MeasurementAccuracyStruct;
  }


  pub struct SoilMoistureMeasuredValue;

  impl Attr for SoilMoistureMeasuredValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "SoilMoistureMeasuredValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* Percent */>;
  }


}
pub mod ty {
  pub use crate::globals::*;
}
