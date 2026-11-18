#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const TIME_FORMAT_LOCALIZATION: ClusterDef = ClusterDef {
  id: 0x002c,
  name: "Time Format Localization",
  features: &[
    FeatureDef { bit: 0x0000, code: "CALFMT", name: "CalendarFormat", summary: "The Node can be configured to use different calendar formats when conveying values to a user" },
  ],
  attributes: &[
    attr::HourFormat::ATTR,
    attr::ActiveCalendarType::ATTR,
    attr::SupportedCalendarTypes::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct HourFormat;

  impl Attr for HourFormat {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "HourFormat",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::HourFormatEnum;
  }


  pub struct ActiveCalendarType;

  impl Attr for ActiveCalendarType {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "ActiveCalendarType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = ty::CalendarTypeEnum;
  }


  pub struct SupportedCalendarTypes;

  impl Attr for SupportedCalendarTypes {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "SupportedCalendarTypes",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<ty::CalendarTypeEnum>;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum CalendarTypeEnum {
  ///Dates conveyed using the Buddhist calendar
  Buddhist = 0x0000,
  ///Dates conveyed using the Chinese calendar
  Chinese = 0x0001,
  ///Dates conveyed using the Coptic calendar
  Coptic = 0x0002,
  ///Dates conveyed using the Ethiopian calendar
  Ethiopian = 0x0003,
  ///Dates conveyed using the Gregorian calendar
  Gregorian = 0x0004,
  ///Dates conveyed using the Hebrew calendar
  Hebrew = 0x0005,
  ///Dates conveyed using the Indian calendar
  Indian = 0x0006,
  ///Dates conveyed using the Islamic calendar
  Islamic = 0x0007,
  ///Dates conveyed using the Japanese calendar
  Japanese = 0x0008,
  ///Dates conveyed using the Korean calendar
  Korean = 0x0009,
  ///Dates conveyed using the Persian calendar
  Persian = 0x000a,
  ///Dates conveyed using the Taiwanese calendar
  Taiwanese = 0x000b,
  ///calendar implied from active locale
  UseActiveLocale = 0x00ff,
}
impl AttrType for CalendarTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Buddhist"),
  (0x0001, "Chinese"),
  (0x0002, "Coptic"),
  (0x0003, "Ethiopian"),
  (0x0004, "Gregorian"),
  (0x0005, "Hebrew"),
  (0x0006, "Indian"),
  (0x0007, "Islamic"),
  (0x0008, "Japanese"),
  (0x0009, "Korean"),
  (0x000a, "Persian"),
  (0x000b, "Taiwanese"),
  (0x00ff, "UseActiveLocale"),
  ]);
}
impl TlvPut for CalendarTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for CalendarTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Buddhist),
      0x0001 => Ok(Self::Chinese),
      0x0002 => Ok(Self::Coptic),
      0x0003 => Ok(Self::Ethiopian),
      0x0004 => Ok(Self::Gregorian),
      0x0005 => Ok(Self::Hebrew),
      0x0006 => Ok(Self::Indian),
      0x0007 => Ok(Self::Islamic),
      0x0008 => Ok(Self::Japanese),
      0x0009 => Ok(Self::Korean),
      0x000a => Ok(Self::Persian),
      0x000b => Ok(Self::Taiwanese),
      0x00ff => Ok(Self::UseActiveLocale),
      _ => Err(())
    }
  }
}
impl TlvDecode for CalendarTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum HourFormatEnum {
  ///Time conveyed with a 12-hour clock
  _12hr = 0x0000,
  ///Time conveyed with a 24-hour clock
  _24hr = 0x0001,
  ///Use active locale clock
  UseActiveLocale = 0x00ff,
}
impl AttrType for HourFormatEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "_12hr"),
  (0x0001, "_24hr"),
  (0x00ff, "UseActiveLocale"),
  ]);
}
impl TlvPut for HourFormatEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for HourFormatEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::_12hr),
      0x0001 => Ok(Self::_24hr),
      0x00ff => Ok(Self::UseActiveLocale),
      _ => Err(())
    }
  }
}
impl TlvDecode for HourFormatEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
