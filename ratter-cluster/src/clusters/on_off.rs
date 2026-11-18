#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const ON_OFF: ClusterDef = ClusterDef {
  id: 0x0006,
  name: "On/Off",
  features: &[
    FeatureDef { bit: 0x0000, code: "LT", name: "Lighting", summary: "Behavior that supports lighting applications" },
    FeatureDef { bit: 0x0001, code: "DF", name: "DeadFrontBehavior", summary: "Device has Dead Front behavior" },
    FeatureDef { bit: 0x0002, code: "OFFONLY", name: "OffOnly", summary: "Device supports the OffOnly Feature feature" },
  ],
  attributes: &[
    attr::OnOff::ATTR,
    attr::GlobalSceneControl::ATTR,
    attr::OnTime::ATTR,
    attr::OffWaitTime::ATTR,
    attr::StartUpOnOff::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::Off::COMMAND,
    command::On::COMMAND,
    command::Toggle::COMMAND,
    command::OffWithEffect::COMMAND,
    command::OnWithRecallGlobalScene::COMMAND,
    command::OnWithTimedOff::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct OnOff;

  impl Attr for OnOff {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "OnOff",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


  pub struct GlobalSceneControl;

  impl Attr for GlobalSceneControl {
    const ATTR: AttrDef = AttrDef {
      id: 0x4000,
      name: "GlobalSceneControl",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = bool;
  }


  pub struct OnTime;

  impl Attr for OnTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x4001,
      name: "OnTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u16;
  }


  pub struct OffWaitTime;

  impl Attr for OffWaitTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x4002,
      name: "OffWaitTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u16;
  }


  pub struct StartUpOnOff;

  impl Attr for StartUpOnOff {
    const ATTR: AttrDef = AttrDef {
      id: 0x4003,
      name: "StartUpOnOff",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<ty::StartUpOnOffEnum>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Off {
}
impl Command for Off {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "Off",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for Off {
  type Response = BasicAccResponse;
}
impl TlvEncode for Off {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct On {
}
impl Command for On {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "On",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for On {
  type Response = BasicAccResponse;
}
impl TlvEncode for On {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Toggle {
}
impl Command for Toggle {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "Toggle",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for Toggle {
  type Response = BasicAccResponse;
}
impl TlvEncode for Toggle {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OffWithEffect {
  pub effect_identifier: ty::EffectIdentifierEnum,
  pub effect_variant: TypeUnsupported /* Enum8 */,
}
impl Command for OffWithEffect {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0040,
    name: "OffWithEffect",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "EffectIdentifier", attr_ty: <ty::EffectIdentifierEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "EffectVariant", attr_ty: <TypeUnsupported /* Enum8 */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for OffWithEffect {
  type Response = BasicAccResponse;
}
impl TlvEncode for OffWithEffect {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.effect_identifier.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.effect_variant.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OnWithRecallGlobalScene {
}
impl Command for OnWithRecallGlobalScene {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0041,
    name: "OnWithRecallGlobalScene",
    conformance: Conformance::Feature(0),
    fields: &[
    ],
  };
}
impl CommandInvoke for OnWithRecallGlobalScene {
  type Response = BasicAccResponse;
}
impl TlvEncode for OnWithRecallGlobalScene {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OnWithTimedOff {
  pub on_off_control: ty::OnOffControlBitmap,
  pub on_time: u16,
  pub off_wait_time: u16,
}
impl Command for OnWithTimedOff {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0042,
    name: "OnWithTimedOff",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "OnOffControl", attr_ty: <ty::OnOffControlBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "OnTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "OffWaitTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for OnWithTimedOff {
  type Response = BasicAccResponse;
}
impl TlvEncode for OnWithTimedOff {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.on_off_control.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.on_time.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.off_wait_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
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
pub enum DelayedAllOffEffectVariantEnum {
  ///Fade to off in 0.8 seconds
  DelayedOffFastFade = 0x0000,
  ///No fade
  NoFade = 0x0001,
  ///50% dim down in 0.8 seconds then fade to off in 12 seconds
  DelayedOffSlowFade = 0x0002,
}
impl AttrType for DelayedAllOffEffectVariantEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "DelayedOffFastFade"),
  (0x0001, "NoFade"),
  (0x0002, "DelayedOffSlowFade"),
  ]);
}
impl TlvPut for DelayedAllOffEffectVariantEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for DelayedAllOffEffectVariantEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::DelayedOffFastFade),
      0x0001 => Ok(Self::NoFade),
      0x0002 => Ok(Self::DelayedOffSlowFade),
      _ => Err(())
    }
  }
}
impl TlvDecode for DelayedAllOffEffectVariantEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum DyingLightEffectVariantEnum {
  ///20% dim up in 0.5s then fade to off in 1 second
  DyingLightFadeOff = 0x0000,
}
impl AttrType for DyingLightEffectVariantEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "DyingLightFadeOff"),
  ]);
}
impl TlvPut for DyingLightEffectVariantEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for DyingLightEffectVariantEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::DyingLightFadeOff),
      _ => Err(())
    }
  }
}
impl TlvDecode for DyingLightEffectVariantEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum EffectIdentifierEnum {
  ///Delayed All Off
  DelayedAllOff = 0x0000,
  ///Dying Light
  DyingLight = 0x0001,
}
impl AttrType for EffectIdentifierEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "DelayedAllOff"),
  (0x0001, "DyingLight"),
  ]);
}
impl TlvPut for EffectIdentifierEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for EffectIdentifierEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::DelayedAllOff),
      0x0001 => Ok(Self::DyingLight),
      _ => Err(())
    }
  }
}
impl TlvDecode for EffectIdentifierEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StartUpOnOffEnum {
  ///Set the OnOff attribute to FALSE
  Off = 0x0000,
  ///Set the OnOff attribute to TRUE
  On = 0x0001,
  ///If the previous value of the OnOff attribute is equal to FALSE, set the OnOff attribute to TRUE. If the previous value of the OnOff attribute is equal to TRUE, set the OnOff attribute to FALSE (toggle).
  Toggle = 0x0002,
}
impl AttrType for StartUpOnOffEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Off"),
  (0x0001, "On"),
  (0x0002, "Toggle"),
  ]);
}
impl TlvPut for StartUpOnOffEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StartUpOnOffEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Off),
      0x0001 => Ok(Self::On),
      0x0002 => Ok(Self::Toggle),
      _ => Err(())
    }
  }
}
impl TlvDecode for StartUpOnOffEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct OnOffControlBitmap: u32 {
    ///Indicates a command is only accepted when in On state.
    const AcceptOnlyWhenOn = 1 << 0;
  }
}
impl AttrType for OnOffControlBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "AcceptOnlyWhenOn"),
  ]);
}
impl TlvPut for OnOffControlBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for OnOffControlBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for OnOffControlBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
}
