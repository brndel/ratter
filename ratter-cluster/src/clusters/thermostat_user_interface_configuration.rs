#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const THERMOSTAT_USER_INTERFACE_CONFIGURATION: ClusterDef = ClusterDef {
  id: 0x0204,
  name: "Thermostat User Interface Configuration",
  features: &[
  ],
  attributes: &[
    attr::TemperatureDisplayMode::ATTR,
    attr::KeypadLockout::ATTR,
    attr::ScheduleProgrammingVisibility::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct TemperatureDisplayMode;

  impl Attr for TemperatureDisplayMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "TemperatureDisplayMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::TemperatureDisplayModeEnum;
  }


  pub struct KeypadLockout;

  impl Attr for KeypadLockout {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "KeypadLockout",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::KeypadLockoutEnum;
  }


  pub struct ScheduleProgrammingVisibility;

  impl Attr for ScheduleProgrammingVisibility {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "ScheduleProgrammingVisibility",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::ScheduleProgrammingVisibilityEnum;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum KeypadLockoutEnum {
  ///All functionality available to the user
  NoLockout = 0x0000,
  ///Level 1 reduced functionality
  Lockout1 = 0x0001,
  ///Level 2 reduced functionality
  Lockout2 = 0x0002,
  ///Level 3 reduced functionality
  Lockout3 = 0x0003,
  ///Level 4 reduced functionality
  Lockout4 = 0x0004,
  ///Least functionality available to the user
  Lockout5 = 0x0005,
}
impl AttrType for KeypadLockoutEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "NoLockout"),
  (0x0001, "Lockout1"),
  (0x0002, "Lockout2"),
  (0x0003, "Lockout3"),
  (0x0004, "Lockout4"),
  (0x0005, "Lockout5"),
  ]);
}
impl TlvPut for KeypadLockoutEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for KeypadLockoutEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::NoLockout),
      0x0001 => Ok(Self::Lockout1),
      0x0002 => Ok(Self::Lockout2),
      0x0003 => Ok(Self::Lockout3),
      0x0004 => Ok(Self::Lockout4),
      0x0005 => Ok(Self::Lockout5),
      _ => Err(())
    }
  }
}
impl TlvDecode for KeypadLockoutEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ScheduleProgrammingVisibilityEnum {
  ///Local schedule programming functionality is enabled at the thermostat
  ScheduleProgrammingPermitted = 0x0000,
  ///Local schedule programming functionality is disabled at the thermostat
  ScheduleProgrammingDenied = 0x0001,
}
impl AttrType for ScheduleProgrammingVisibilityEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "ScheduleProgrammingPermitted"),
  (0x0001, "ScheduleProgrammingDenied"),
  ]);
}
impl TlvPut for ScheduleProgrammingVisibilityEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ScheduleProgrammingVisibilityEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::ScheduleProgrammingPermitted),
      0x0001 => Ok(Self::ScheduleProgrammingDenied),
      _ => Err(())
    }
  }
}
impl TlvDecode for ScheduleProgrammingVisibilityEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TemperatureDisplayModeEnum {
  ///Temperature displayed in °C
  Celsius = 0x0000,
  ///Temperature displayed in °F
  Fahrenheit = 0x0001,
}
impl AttrType for TemperatureDisplayModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Celsius"),
  (0x0001, "Fahrenheit"),
  ]);
}
impl TlvPut for TemperatureDisplayModeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TemperatureDisplayModeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Celsius),
      0x0001 => Ok(Self::Fahrenheit),
      _ => Err(())
    }
  }
}
impl TlvDecode for TemperatureDisplayModeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
