#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const ILLUMINANCE_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x0400,
  name: "Illuminance Measurement",
  features: &[
  ],
  attributes: &[
    attr::MeasuredValue::ATTR,
    attr::MinMeasuredValue::ATTR,
    attr::MaxMeasuredValue::ATTR,
    attr::Tolerance::ATTR,
    attr::LightSensorType::ATTR,
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


  pub struct LightSensorType;

  impl Attr for LightSensorType {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "LightSensorType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<ty::LightSensorTypeEnum>;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum LightSensorTypeEnum {
  ///Indicates photodiode sensor type
  Photodiode = 0x0000,
  ///Indicates CMOS sensor type
  CMOS = 0x0001,
}
impl AttrType for LightSensorTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Photodiode"),
  (0x0001, "CMOS"),
  ]);
}
impl TlvPut for LightSensorTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for LightSensorTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Photodiode),
      0x0001 => Ok(Self::CMOS),
      _ => Err(())
    }
  }
}
impl TlvDecode for LightSensorTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
