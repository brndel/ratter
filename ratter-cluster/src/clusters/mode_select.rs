#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const MODE_SELECT: ClusterDef = ClusterDef {
  id: 0x0050,
  name: "Mode Select",
  features: &[
    FeatureDef { bit: 0x0000, code: "DEPONOFF", name: "OnOff", summary: "Dependency with the OnOff cluster" },
  ],
  attributes: &[
    attr::Description::ATTR,
    attr::StandardNamespace::ATTR,
    attr::SupportedModes::ATTR,
    attr::CurrentMode::ATTR,
    attr::StartUpMode::ATTR,
    attr::OnMode::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::ChangeToMode::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct Description;

  impl Attr for Description {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "Description",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = String;
  }


  pub struct StandardNamespace;

  impl Attr for StandardNamespace {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "StandardNamespace",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* Enum16 */>;
  }


  pub struct SupportedModes;

  impl Attr for SupportedModes {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "SupportedModes",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::ModeOptionStruct>;
  }


  pub struct CurrentMode;

  impl Attr for CurrentMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "CurrentMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct StartUpMode;

  impl Attr for StartUpMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "StartUpMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u8>;
  }


  pub struct OnMode;

  impl Attr for OnMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
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
  type Response = BasicAccResponse;
}
impl TlvEncode for ChangeToMode {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.new_mode.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ModeOptionStruct {
  pub label: String,
  pub mode: u8,
  pub semantic_tags: Vec<SemanticTagStruct>,
}
impl AttrType for ModeOptionStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Label", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Mode", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "SemanticTags", attr_ty: <Vec<SemanticTagStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ModeOptionStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.label.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.mode.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.semantic_tags.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ModeOptionStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut label: Option<String> = None;
    let mut mode: Option<u8> = None;
    let mut semantic_tags: Option<Vec<SemanticTagStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { label = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { mode = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { semantic_tags = Some(<Vec<SemanticTagStruct>>::tlv_decode(v, reader)?); },
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
      semantic_tags: semantic_tags.ok_or(TlvDecodeErr::MissingField("SemanticTags"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SemanticTagStruct {
  pub mfg_code: TypeUnsupported /* VendorId */,
  pub value: TypeUnsupported /* Enum16 */,
}
impl AttrType for SemanticTagStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "MfgCode", attr_ty: <TypeUnsupported /* VendorId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Value", attr_ty: <TypeUnsupported /* Enum16 */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for SemanticTagStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.mfg_code.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.value.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for SemanticTagStruct {
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
