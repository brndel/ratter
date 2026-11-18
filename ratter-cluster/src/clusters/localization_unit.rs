#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const UNIT_LOCALIZATION: ClusterDef = ClusterDef {
  id: 0x002d,
  name: "Unit Localization",
  features: &[
    FeatureDef { bit: 0x0000, code: "TEMP", name: "TemperatureUnit", summary: "The Node can be configured to use different units of temperature when conveying values to a user" },
  ],
  attributes: &[
    attr::TemperatureUnit::ATTR,
    attr::SupportedTemperatureUnits::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct TemperatureUnit;

  impl Attr for TemperatureUnit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "TemperatureUnit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = ty::TempUnitEnum;
  }


  pub struct SupportedTemperatureUnits;

  impl Attr for SupportedTemperatureUnits {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "SupportedTemperatureUnits",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<ty::TempUnitEnum>;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TempUnitEnum {
  ///Temperature conveyed in Fahrenheit
  Fahrenheit = 0x0000,
  ///Temperature conveyed in Celsius
  Celsius = 0x0001,
  ///Temperature conveyed in Kelvin
  Kelvin = 0x0002,
}
impl AttrType for TempUnitEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Fahrenheit"),
  (0x0001, "Celsius"),
  (0x0002, "Kelvin"),
  ]);
}
impl TlvPut for TempUnitEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TempUnitEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Fahrenheit),
      0x0001 => Ok(Self::Celsius),
      0x0002 => Ok(Self::Kelvin),
      _ => Err(())
    }
  }
}
impl TlvDecode for TempUnitEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
