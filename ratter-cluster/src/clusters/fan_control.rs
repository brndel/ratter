#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const FAN_CONTROL: ClusterDef = ClusterDef {
  id: 0x0202,
  name: "Fan Control",
  features: &[
    FeatureDef { bit: 0x0000, code: "SPD", name: "MultiSpeed", summary: "0-SpeedMax Fan Speeds" },
    FeatureDef { bit: 0x0001, code: "AUT", name: "Auto", summary: "Automatic mode supported for fan speed" },
    FeatureDef { bit: 0x0002, code: "RCK", name: "Rocking", summary: "Rocking movement supported" },
    FeatureDef { bit: 0x0003, code: "WND", name: "Wind", summary: "Wind emulation supported" },
    FeatureDef { bit: 0x0004, code: "STEP", name: "Step", summary: "Step command supported" },
    FeatureDef { bit: 0x0005, code: "DIR", name: "AirflowDirection", summary: "Airflow Direction attribute is supported" },
  ],
  attributes: &[
    attr::FanMode::ATTR,
    attr::FanModeSequence::ATTR,
    attr::PercentSetting::ATTR,
    attr::PercentCurrent::ATTR,
    attr::SpeedMax::ATTR,
    attr::SpeedSetting::ATTR,
    attr::SpeedCurrent::ATTR,
    attr::RockSupport::ATTR,
    attr::RockSetting::ATTR,
    attr::WindSupport::ATTR,
    attr::WindSetting::ATTR,
    attr::AirflowDirection::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::Step::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct FanMode;

  impl Attr for FanMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "FanMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::FanModeEnum;
  }


  pub struct FanModeSequence;

  impl Attr for FanModeSequence {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "FanModeSequence",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::FanModeSequenceEnum;
  }


  pub struct PercentSetting;

  impl Attr for PercentSetting {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "PercentSetting",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* Percent */>;
  }


  pub struct PercentCurrent;

  impl Attr for PercentCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "PercentCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = TypeUnsupported /* Percent */;
  }


  pub struct SpeedMax;

  impl Attr for SpeedMax {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "SpeedMax",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u8;
  }


  pub struct SpeedSetting;

  impl Attr for SpeedSetting {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "SpeedSetting",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<u8>;
  }


  pub struct SpeedCurrent;

  impl Attr for SpeedCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "SpeedCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u8;
  }


  pub struct RockSupport;

  impl Attr for RockSupport {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "RockSupport",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = ty::RockBitmap;
  }


  pub struct RockSetting;

  impl Attr for RockSetting {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "RockSetting",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = ty::RockBitmap;
  }


  pub struct WindSupport;

  impl Attr for WindSupport {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "WindSupport",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = ty::WindBitmap;
  }


  pub struct WindSetting;

  impl Attr for WindSetting {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "WindSetting",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = ty::WindBitmap;
  }


  pub struct AirflowDirection;

  impl Attr for AirflowDirection {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "AirflowDirection",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(5),
    };
    type Value = ty::AirflowDirectionEnum;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Step {
  pub direction: ty::StepDirectionEnum,
  pub wrap: bool,
  pub lowest_off: bool,
}
impl Command for Step {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "Step",
    conformance: Conformance::Feature(4),
    fields: &[
      AttrDef { id: 0x0000, name: "Direction", attr_ty: <ty::StepDirectionEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Wrap", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "LowestOff", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for Step {
  type Response = BasicAccResponse;
}
impl TlvEncode for Step {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.direction.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.wrap.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.lowest_off.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum AirflowDirectionEnum {
  ///Airflow is in the forward direction
  Forward = 0x0000,
  ///Airflow is in the reverse direction
  Reverse = 0x0001,
}
impl AttrType for AirflowDirectionEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Forward"),
  (0x0001, "Reverse"),
  ]);
}
impl TlvPut for AirflowDirectionEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for AirflowDirectionEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Forward),
      0x0001 => Ok(Self::Reverse),
      _ => Err(())
    }
  }
}
impl TlvDecode for AirflowDirectionEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum FanModeEnum {
  ///Fan is off
  Off = 0x0000,
  ///Fan using low speed
  Low = 0x0001,
  ///Fan using medium speed
  Medium = 0x0002,
  ///Fan using high speed
  High = 0x0003,
  On = 0x0004,
  ///Fan is using auto mode
  Auto = 0x0005,
  ///Fan is using smart mode
  Smart = 0x0006,
}
impl AttrType for FanModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Off"),
  (0x0001, "Low"),
  (0x0002, "Medium"),
  (0x0003, "High"),
  (0x0004, "On"),
  (0x0005, "Auto"),
  (0x0006, "Smart"),
  ]);
}
impl TlvPut for FanModeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for FanModeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Off),
      0x0001 => Ok(Self::Low),
      0x0002 => Ok(Self::Medium),
      0x0003 => Ok(Self::High),
      0x0004 => Ok(Self::On),
      0x0005 => Ok(Self::Auto),
      0x0006 => Ok(Self::Smart),
      _ => Err(())
    }
  }
}
impl TlvDecode for FanModeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum FanModeSequenceEnum {
  ///Fan is capable of off, low, medium and high modes
  OffLowMedHigh = 0x0000,
  ///Fan is capable of off, low and high modes
  OffLowHigh = 0x0001,
  ///Fan is capable of off, low, medium, high and auto modes
  OffLowMedHighAuto = 0x0002,
  ///Fan is capable of off, low, high and auto modes
  OffLowHighAuto = 0x0003,
  ///Fan is capable of off, high and auto modes
  OffHighAuto = 0x0004,
  ///Fan is capable of off and high modes
  OffHigh = 0x0005,
}
impl AttrType for FanModeSequenceEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "OffLowMedHigh"),
  (0x0001, "OffLowHigh"),
  (0x0002, "OffLowMedHighAuto"),
  (0x0003, "OffLowHighAuto"),
  (0x0004, "OffHighAuto"),
  (0x0005, "OffHigh"),
  ]);
}
impl TlvPut for FanModeSequenceEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for FanModeSequenceEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::OffLowMedHigh),
      0x0001 => Ok(Self::OffLowHigh),
      0x0002 => Ok(Self::OffLowMedHighAuto),
      0x0003 => Ok(Self::OffLowHighAuto),
      0x0004 => Ok(Self::OffHighAuto),
      0x0005 => Ok(Self::OffHigh),
      _ => Err(())
    }
  }
}
impl TlvDecode for FanModeSequenceEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StepDirectionEnum {
  ///Step moves in increasing direction
  Increase = 0x0000,
  ///Step moves in decreasing direction
  Decrease = 0x0001,
}
impl AttrType for StepDirectionEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Increase"),
  (0x0001, "Decrease"),
  ]);
}
impl TlvPut for StepDirectionEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StepDirectionEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Increase),
      0x0001 => Ok(Self::Decrease),
      _ => Err(())
    }
  }
}
impl TlvDecode for StepDirectionEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct RockBitmap: u32 {
    ///Indicate rock left to right
    const RockLeftRight = 1 << 0;
    ///Indicate rock up and down
    const RockUpDown = 1 << 1;
    ///Indicate rock around
    const RockRound = 1 << 2;
  }
}
impl AttrType for RockBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "RockLeftRight"),
    (1, "RockUpDown"),
    (2, "RockRound"),
  ]);
}
impl TlvPut for RockBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for RockBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for RockBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct WindBitmap: u32 {
    ///Indicate sleep wind
    const SleepWind = 1 << 0;
    ///Indicate natural wind
    const NaturalWind = 1 << 1;
  }
}
impl AttrType for WindBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "SleepWind"),
    (1, "NaturalWind"),
  ]);
}
impl TlvPut for WindBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for WindBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for WindBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
}
