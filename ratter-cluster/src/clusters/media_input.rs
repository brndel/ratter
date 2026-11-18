#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const MEDIA_INPUT: ClusterDef = ClusterDef {
  id: 0x0507,
  name: "Media Input",
  features: &[
    FeatureDef { bit: 0x0000, code: "NU", name: "NameUpdates", summary: "Supports updates to the input names" },
  ],
  attributes: &[
    attr::InputList::ATTR,
    attr::CurrentInput::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::SelectInput::COMMAND,
    command::ShowInputStatus::COMMAND,
    command::HideInputStatus::COMMAND,
    command::RenameInput::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct InputList;

  impl Attr for InputList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "InputList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::InputInfoStruct>;
  }


  pub struct CurrentInput;

  impl Attr for CurrentInput {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "CurrentInput",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SelectInput {
  pub index: u8,
}
impl Command for SelectInput {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "SelectInput",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Index", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SelectInput {
  type Response = BasicAccResponse;
}
impl TlvEncode for SelectInput {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.index.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShowInputStatus {
}
impl Command for ShowInputStatus {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "ShowInputStatus",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for ShowInputStatus {
  type Response = BasicAccResponse;
}
impl TlvEncode for ShowInputStatus {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HideInputStatus {
}
impl Command for HideInputStatus {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "HideInputStatus",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for HideInputStatus {
  type Response = BasicAccResponse;
}
impl TlvEncode for HideInputStatus {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenameInput {
  pub index: u8,
  pub name: String,
}
impl Command for RenameInput {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "RenameInput",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "Index", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RenameInput {
  type Response = BasicAccResponse;
}
impl TlvEncode for RenameInput {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.index.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
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
pub enum InputTypeEnum {
  ///Indicates content not coming from a physical input.
  Internal = 0x0000,
  Aux = 0x0001,
  Coax = 0x0002,
  Composite = 0x0003,
  HDMI = 0x0004,
  Input = 0x0005,
  Line = 0x0006,
  Optical = 0x0007,
  Video = 0x0008,
  SCART = 0x0009,
  USB = 0x000a,
  Other = 0x000b,
}
impl AttrType for InputTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Internal"),
  (0x0001, "Aux"),
  (0x0002, "Coax"),
  (0x0003, "Composite"),
  (0x0004, "HDMI"),
  (0x0005, "Input"),
  (0x0006, "Line"),
  (0x0007, "Optical"),
  (0x0008, "Video"),
  (0x0009, "SCART"),
  (0x000a, "USB"),
  (0x000b, "Other"),
  ]);
}
impl TlvPut for InputTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for InputTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Internal),
      0x0001 => Ok(Self::Aux),
      0x0002 => Ok(Self::Coax),
      0x0003 => Ok(Self::Composite),
      0x0004 => Ok(Self::HDMI),
      0x0005 => Ok(Self::Input),
      0x0006 => Ok(Self::Line),
      0x0007 => Ok(Self::Optical),
      0x0008 => Ok(Self::Video),
      0x0009 => Ok(Self::SCART),
      0x000a => Ok(Self::USB),
      0x000b => Ok(Self::Other),
      _ => Err(())
    }
  }
}
impl TlvDecode for InputTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct InputInfoStruct {
  pub index: u8,
  pub input_type: InputTypeEnum,
  pub name: String,
  pub description: String,
}
impl AttrType for InputInfoStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Index", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "InputType", attr_ty: <InputTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "Description", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for InputInfoStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.index.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.input_type.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.name.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.description.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for InputInfoStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut index: Option<u8> = None;
    let mut input_type: Option<InputTypeEnum> = None;
    let mut name: Option<String> = None;
    let mut description: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { index = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { input_type = Some(<InputTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { description = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      index: index.ok_or(TlvDecodeErr::MissingField("Index"))?,
      input_type: input_type.ok_or(TlvDecodeErr::MissingField("InputType"))?,
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
      description: description.ok_or(TlvDecodeErr::MissingField("Description"))?,
    })
  }
}

}
