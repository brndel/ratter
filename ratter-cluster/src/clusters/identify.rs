#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const IDENTIFY: ClusterDef = ClusterDef {
  id: 0x0003,
  name: "Identify",
  features: &[
  ],
  attributes: &[
    attr::IdentifyTime::ATTR,
    attr::IdentifyType::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::Identify::COMMAND,
    command::TriggerEffect::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct IdentifyTime;

  impl Attr for IdentifyTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "IdentifyTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u16;
  }


  pub struct IdentifyType;

  impl Attr for IdentifyType {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "IdentifyType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::IdentifyTypeEnum;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Identify {
  pub identify_time: u16,
}
impl Command for Identify {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "Identify",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "IdentifyTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for Identify {
  type Response = BasicAccResponse;
}
impl TlvEncode for Identify {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.identify_time.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TriggerEffect {
  pub effect_identifier: ty::EffectIdentifierEnum,
  pub effect_variant: ty::EffectVariantEnum,
}
impl Command for TriggerEffect {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0040,
    name: "TriggerEffect",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "EffectIdentifier", attr_ty: <ty::EffectIdentifierEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "EffectVariant", attr_ty: <ty::EffectVariantEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for TriggerEffect {
  type Response = BasicAccResponse;
}
impl TlvEncode for TriggerEffect {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.effect_identifier.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.effect_variant.tlv_put(matter_codec::Tag::Context(0x0001), writer);
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
pub enum EffectIdentifierEnum {
  ///e.g., Light is turned on/off once.
  Blink = 0x0000,
  ///e.g., Light is turned on/off over 1 second and repeated 15 times.
  Breathe = 0x0001,
  ///e.g., Colored light turns green for 1 second; non-colored light flashes twice.
  Okay = 0x0002,
  ///e.g., Colored light turns orange for 8 seconds; non-colored light switches to the maximum brightness for 0.5s and then minimum brightness for 7.5s.
  ChannelChange = 0x000b,
  ///Complete the current effect sequence before terminating. e.g., if in the middle of a breathe effect (as above), first complete the current 1s breathe effect and then terminate the effect.
  FinishEffect = 0x00fe,
  ///Terminate the effect as soon as possible.
  StopEffect = 0x00ff,
}
impl AttrType for EffectIdentifierEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Blink"),
  (0x0001, "Breathe"),
  (0x0002, "Okay"),
  (0x000b, "ChannelChange"),
  (0x00fe, "FinishEffect"),
  (0x00ff, "StopEffect"),
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
      0x0000 => Ok(Self::Blink),
      0x0001 => Ok(Self::Breathe),
      0x0002 => Ok(Self::Okay),
      0x000b => Ok(Self::ChannelChange),
      0x00fe => Ok(Self::FinishEffect),
      0x00ff => Ok(Self::StopEffect),
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
pub enum EffectVariantEnum {
  ///Indicates the default effect is used
  Default = 0x0000,
}
impl AttrType for EffectVariantEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Default"),
  ]);
}
impl TlvPut for EffectVariantEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for EffectVariantEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Default),
      _ => Err(())
    }
  }
}
impl TlvDecode for EffectVariantEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum IdentifyTypeEnum {
  ///No presentation.
  None = 0x0000,
  ///Light output of a lighting product.
  LightOutput = 0x0001,
  ///Typically a small LED.
  VisibleIndicator = 0x0002,
  AudibleBeep = 0x0003,
  ///Presentation will be visible on display screen.
  Display = 0x0004,
  ///Presentation will be conveyed by actuator functionality such as through a window blind operation or in-wall relay.
  Actuator = 0x0005,
}
impl AttrType for IdentifyTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "None"),
  (0x0001, "LightOutput"),
  (0x0002, "VisibleIndicator"),
  (0x0003, "AudibleBeep"),
  (0x0004, "Display"),
  (0x0005, "Actuator"),
  ]);
}
impl TlvPut for IdentifyTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for IdentifyTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::None),
      0x0001 => Ok(Self::LightOutput),
      0x0002 => Ok(Self::VisibleIndicator),
      0x0003 => Ok(Self::AudibleBeep),
      0x0004 => Ok(Self::Display),
      0x0005 => Ok(Self::Actuator),
      _ => Err(())
    }
  }
}
impl TlvDecode for IdentifyTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
