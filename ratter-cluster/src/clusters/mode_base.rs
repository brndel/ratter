#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub mod attr {
  use super::*;
  pub struct SupportedModes;

  impl Attr for SupportedModes {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "SupportedModes",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::ModeOptionStruct>;
  }


  pub struct CurrentMode;

  impl Attr for CurrentMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "CurrentMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct StartUpMode;

  impl Attr for StartUpMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "StartUpMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u8>;
  }


  pub struct OnMode;

  impl Attr for OnMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "OnMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<u8>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ChangeToMode {
  pub new_mode: u8,
}
impl Command for ChangeToMode {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "ChangeToMode",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NewMode", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ChangeToMode {
  type Response = ChangeToModeResponse;
}
impl TlvEncode for ChangeToMode {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.new_mode.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChangeToModeResponse {
  pub status: TypeUnsupported /* Enum8 */,
  pub status_text: String,
}
impl Command for ChangeToModeResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "ChangeToModeResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <TypeUnsupported /* Enum8 */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "StatusText", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::TooComplicated },
    ],
  };
}
impl CommandResponse for ChangeToModeResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ModeOptionStruct {
  pub label: String,
  pub mode: u8,
  pub mode_tags: Vec<ModeTagStruct>,
}
impl AttrType for ModeOptionStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Label", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Mode", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "ModeTags", attr_ty: <Vec<ModeTagStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ModeOptionStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.label.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.mode.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.mode_tags.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ModeOptionStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut label: Option<String> = None;
    let mut mode: Option<u8> = None;
    let mut mode_tags: Option<Vec<ModeTagStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { label = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { mode = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { mode_tags = Some(<Vec<ModeTagStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      label: label.ok_or(TlvDecodeErr::MissingField("Label"))?,
      mode: mode.ok_or(TlvDecodeErr::MissingField("Mode"))?,
      mode_tags: mode_tags.ok_or(TlvDecodeErr::MissingField("ModeTags"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModeTagStruct {
  pub mfg_code: TypeUnsupported /* VendorId */,
  pub value: TypeUnsupported /* Enum16 */,
}
impl AttrType for ModeTagStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "MfgCode", attr_ty: <TypeUnsupported /* VendorId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Value", attr_ty: <TypeUnsupported /* Enum16 */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ModeTagStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.mfg_code.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.value.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ModeTagStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    
    loop {
      match reader.next_ref()? {
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      mfg_code: TypeUnsupported,
      value: TypeUnsupported,
    })
  }
}

}
