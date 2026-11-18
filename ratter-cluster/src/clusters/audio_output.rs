#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const AUDIO_OUTPUT: ClusterDef = ClusterDef {
  id: 0x050b,
  name: "Audio Output",
  features: &[
    FeatureDef { bit: 0x0000, code: "NU", name: "NameUpdates", summary: "Supports updates to output names" },
  ],
  attributes: &[
    attr::OutputList::ATTR,
    attr::CurrentOutput::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::SelectOutput::COMMAND,
    command::RenameOutput::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct OutputList;

  impl Attr for OutputList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "OutputList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::OutputInfoStruct>;
  }


  pub struct CurrentOutput;

  impl Attr for CurrentOutput {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "CurrentOutput",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SelectOutput {
  pub index: u8,
}
impl Command for SelectOutput {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "SelectOutput",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Index", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SelectOutput {
  type Response = BasicAccResponse;
}
impl TlvEncode for SelectOutput {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.index.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RenameOutput {
  pub index: u8,
  pub name: String,
}
impl Command for RenameOutput {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "RenameOutput",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "Index", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RenameOutput {
  type Response = BasicAccResponse;
}
impl TlvEncode for RenameOutput {
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
pub enum OutputTypeEnum {
  ///HDMI
  HDMI = 0x0000,
  BT = 0x0001,
  Optical = 0x0002,
  Headphone = 0x0003,
  Internal = 0x0004,
  Other = 0x0005,
}
impl AttrType for OutputTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "HDMI"),
  (0x0001, "BT"),
  (0x0002, "Optical"),
  (0x0003, "Headphone"),
  (0x0004, "Internal"),
  (0x0005, "Other"),
  ]);
}
impl TlvPut for OutputTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for OutputTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::HDMI),
      0x0001 => Ok(Self::BT),
      0x0002 => Ok(Self::Optical),
      0x0003 => Ok(Self::Headphone),
      0x0004 => Ok(Self::Internal),
      0x0005 => Ok(Self::Other),
      _ => Err(())
    }
  }
}
impl TlvDecode for OutputTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct OutputInfoStruct {
  pub index: u8,
  pub output_type: OutputTypeEnum,
  pub name: String,
}
impl AttrType for OutputInfoStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Index", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "OutputType", attr_ty: <OutputTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for OutputInfoStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.index.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.output_type.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.name.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for OutputInfoStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut index: Option<u8> = None;
    let mut output_type: Option<OutputTypeEnum> = None;
    let mut name: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { index = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { output_type = Some(<OutputTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      index: index.ok_or(TlvDecodeErr::MissingField("Index"))?,
      output_type: output_type.ok_or(TlvDecodeErr::MissingField("OutputType"))?,
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
    })
  }
}

}
