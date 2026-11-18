#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const LEVEL_CONTROL: ClusterDef = ClusterDef {
  id: 0x0008,
  name: "Level Control",
  features: &[
    FeatureDef { bit: 0x0000, code: "OO", name: "OnOff", summary: "Dependency with the On/Off cluster" },
    FeatureDef { bit: 0x0001, code: "LT", name: "Lighting", summary: "Behavior that supports lighting applications" },
    FeatureDef { bit: 0x0002, code: "FQ", name: "Frequency", summary: "Supports frequency attributes and behavior." },
  ],
  attributes: &[
    attr::CurrentLevel::ATTR,
    attr::RemainingTime::ATTR,
    attr::MinLevel::ATTR,
    attr::MaxLevel::ATTR,
    attr::CurrentFrequency::ATTR,
    attr::MinFrequency::ATTR,
    attr::MaxFrequency::ATTR,
    attr::Options::ATTR,
    attr::OnOffTransitionTime::ATTR,
    attr::OnLevel::ATTR,
    attr::OnTransitionTime::ATTR,
    attr::OffTransitionTime::ATTR,
    attr::DefaultMoveRate::ATTR,
    attr::StartUpCurrentLevel::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::MoveToLevel::COMMAND,
    command::Move::COMMAND,
    command::Step::COMMAND,
    command::Stop::COMMAND,
    command::MoveToLevelWithOnOff::COMMAND,
    command::MoveWithOnOff::COMMAND,
    command::StepWithOnOff::COMMAND,
    command::StopWithOnOff::COMMAND,
    command::MoveToClosestFrequency::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct CurrentLevel;

  impl Attr for CurrentLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "CurrentLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u8>;
  }


  pub struct RemainingTime;

  impl Attr for RemainingTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "RemainingTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = u16;
  }


  pub struct MinLevel;

  impl Attr for MinLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "MinLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u8;
  }


  pub struct MaxLevel;

  impl Attr for MaxLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "MaxLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u8;
  }


  pub struct CurrentFrequency;

  impl Attr for CurrentFrequency {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "CurrentFrequency",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = u16;
  }


  pub struct MinFrequency;

  impl Attr for MinFrequency {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "MinFrequency",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = u16;
  }


  pub struct MaxFrequency;

  impl Attr for MaxFrequency {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "MaxFrequency",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = u16;
  }


  pub struct Options;

  impl Attr for Options {
    const ATTR: AttrDef = AttrDef {
      id: 0x000f,
      name: "Options",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::OptionsBitmap;
  }


  pub struct OnOffTransitionTime;

  impl Attr for OnOffTransitionTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0010,
      name: "OnOffTransitionTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct OnLevel;

  impl Attr for OnLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0011,
      name: "OnLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u8>;
  }


  pub struct OnTransitionTime;

  impl Attr for OnTransitionTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0012,
      name: "OnTransitionTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u16>;
  }


  pub struct OffTransitionTime;

  impl Attr for OffTransitionTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0013,
      name: "OffTransitionTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u16>;
  }


  pub struct DefaultMoveRate;

  impl Attr for DefaultMoveRate {
    const ATTR: AttrDef = AttrDef {
      id: 0x0014,
      name: "DefaultMoveRate",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u8>;
  }


  pub struct StartUpCurrentLevel;

  impl Attr for StartUpCurrentLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x4000,
      name: "StartUpCurrentLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Nullable<u8>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct MoveToLevel {
  pub level: u8,
  pub transition_time: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for MoveToLevel {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "MoveToLevel",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Level", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "TransitionTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for MoveToLevel {
  type Response = BasicAccResponse;
}
impl TlvEncode for MoveToLevel {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.level.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Move {
  pub move_mode: ty::MoveModeEnum,
  pub rate: u8,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for Move {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "Move",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "MoveMode", attr_ty: <ty::MoveModeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Rate", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for Move {
  type Response = BasicAccResponse;
}
impl TlvEncode for Move {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.move_mode.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.rate.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Step {
  pub step_mode: ty::StepModeEnum,
  pub step_size: u8,
  pub transition_time: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for Step {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "Step",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "StepMode", attr_ty: <ty::StepModeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "StepSize", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TransitionTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for Step {
  type Response = BasicAccResponse;
}
impl TlvEncode for Step {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.step_mode.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.step_size.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stop {
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for Stop {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "Stop",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for Stop {
  type Response = BasicAccResponse;
}
impl TlvEncode for Stop {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveToLevelWithOnOff {
}
impl Command for MoveToLevelWithOnOff {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "MoveToLevelWithOnOff",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for MoveToLevelWithOnOff {
  type Response = BasicAccResponse;
}
impl TlvEncode for MoveToLevelWithOnOff {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveWithOnOff {
}
impl Command for MoveWithOnOff {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "MoveWithOnOff",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for MoveWithOnOff {
  type Response = BasicAccResponse;
}
impl TlvEncode for MoveWithOnOff {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StepWithOnOff {
}
impl Command for StepWithOnOff {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "StepWithOnOff",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for StepWithOnOff {
  type Response = BasicAccResponse;
}
impl TlvEncode for StepWithOnOff {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StopWithOnOff {
}
impl Command for StopWithOnOff {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "StopWithOnOff",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for StopWithOnOff {
  type Response = BasicAccResponse;
}
impl TlvEncode for StopWithOnOff {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveToClosestFrequency {
  pub frequency: u16,
}
impl Command for MoveToClosestFrequency {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0008,
    name: "MoveToClosestFrequency",
    conformance: Conformance::Feature(2),
    fields: &[
      AttrDef { id: 0x0000, name: "Frequency", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for MoveToClosestFrequency {
  type Response = BasicAccResponse;
}
impl TlvEncode for MoveToClosestFrequency {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.frequency.tlv_put(matter_codec::Tag::Context(0x0000), writer);
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
pub enum MoveModeEnum {
  ///Increase the level
  Up = 0x0000,
  ///Decrease the level
  Down = 0x0001,
}
impl AttrType for MoveModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Up"),
  (0x0001, "Down"),
  ]);
}
impl TlvPut for MoveModeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for MoveModeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Up),
      0x0001 => Ok(Self::Down),
      _ => Err(())
    }
  }
}
impl TlvDecode for MoveModeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StepModeEnum {
  ///Step upwards
  Up = 0x0000,
  ///Step downwards
  Down = 0x0001,
}
impl AttrType for StepModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Up"),
  (0x0001, "Down"),
  ]);
}
impl TlvPut for StepModeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StepModeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Up),
      0x0001 => Ok(Self::Down),
      _ => Err(())
    }
  }
}
impl TlvDecode for StepModeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct OptionsBitmap: u32 {
    ///Dependency on On/Off cluster
    const ExecuteIfOff = 1 << 0;
    ///Dependency on Color Control cluster
    const CoupleColorTempToLevel = 1 << 1;
  }
}
impl AttrType for OptionsBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "ExecuteIfOff"),
    (1, "CoupleColorTempToLevel"),
  ]);
}
impl TlvPut for OptionsBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for OptionsBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for OptionsBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
}
