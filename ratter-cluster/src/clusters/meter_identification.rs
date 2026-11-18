#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const METER_IDENTIFICATION: ClusterDef = ClusterDef {
  id: 0x0b06,
  name: "Meter Identification",
  features: &[
    FeatureDef { bit: 0x0000, code: "PWRTHLD", name: "PowerThreshold", summary: "Supports information about power threshold" },
  ],
  attributes: &[
    attr::MeterType::ATTR,
    attr::PointOfDelivery::ATTR,
    attr::MeterSerialNumber::ATTR,
    attr::ProtocolVersion::ATTR,
    attr::PowerThreshold::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct MeterType;

  impl Attr for MeterType {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "MeterType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::MeterTypeEnum>;
  }


  pub struct PointOfDelivery;

  impl Attr for PointOfDelivery {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "PointOfDelivery",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<String>;
  }


  pub struct MeterSerialNumber;

  impl Attr for MeterSerialNumber {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "MeterSerialNumber",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<String>;
  }


  pub struct ProtocolVersion;

  impl Attr for ProtocolVersion {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "ProtocolVersion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<String>;
  }


  pub struct PowerThreshold;

  impl Attr for PowerThreshold {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "PowerThreshold",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<ty::PowerThresholdStruct>;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum MeterTypeEnum {
  ///Utility Meter
  Utility = 0x0000,
  ///Private Meter
  Private = 0x0001,
  ///Generic Meter
  Generic = 0x0002,
}
impl AttrType for MeterTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Utility"),
  (0x0001, "Private"),
  (0x0002, "Generic"),
  ]);
}
impl TlvPut for MeterTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for MeterTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Utility),
      0x0001 => Ok(Self::Private),
      0x0002 => Ok(Self::Generic),
      _ => Err(())
    }
  }
}
impl TlvDecode for MeterTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
