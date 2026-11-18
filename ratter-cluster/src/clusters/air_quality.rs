#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const AIR_QUALITY: ClusterDef = ClusterDef {
  id: 0x005b,
  name: "Air Quality",
  features: &[
    FeatureDef { bit: 0x0000, code: "FAIR", name: "Fair", summary: "Cluster supports the Fair air quality level" },
    FeatureDef { bit: 0x0001, code: "MOD", name: "Moderate", summary: "Cluster supports the Moderate air quality level" },
    FeatureDef { bit: 0x0002, code: "VPOOR", name: "VeryPoor", summary: "Cluster supports the Very poor air quality level" },
    FeatureDef { bit: 0x0003, code: "XPOOR", name: "ExtremelyPoor", summary: "Cluster supports the Extremely poor air quality level" },
  ],
  attributes: &[
    attr::AirQuality::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct AirQuality;

  impl Attr for AirQuality {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "AirQuality",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::AirQualityEnum;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum AirQualityEnum {
  ///The air quality is unknown.
  Unknown = 0x0000,
  ///The air quality is good.
  Good = 0x0001,
  ///The air quality is fair.
  Fair = 0x0002,
  ///The air quality is moderate.
  Moderate = 0x0003,
  ///The air quality is poor.
  Poor = 0x0004,
  ///The air quality is very poor.
  VeryPoor = 0x0005,
  ///The air quality is extremely poor.
  ExtremelyPoor = 0x0006,
}
impl AttrType for AirQualityEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unknown"),
  (0x0001, "Good"),
  (0x0002, "Fair"),
  (0x0003, "Moderate"),
  (0x0004, "Poor"),
  (0x0005, "VeryPoor"),
  (0x0006, "ExtremelyPoor"),
  ]);
}
impl TlvPut for AirQualityEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for AirQualityEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unknown),
      0x0001 => Ok(Self::Good),
      0x0002 => Ok(Self::Fair),
      0x0003 => Ok(Self::Moderate),
      0x0004 => Ok(Self::Poor),
      0x0005 => Ok(Self::VeryPoor),
      0x0006 => Ok(Self::ExtremelyPoor),
      _ => Err(())
    }
  }
}
impl TlvDecode for AirQualityEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
