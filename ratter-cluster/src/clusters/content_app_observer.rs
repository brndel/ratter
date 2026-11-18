#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const CONTENT_APP_OBSERVER: ClusterDef = ClusterDef {
  id: 0x0510,
  name: "Content App Observer",
  features: &[
  ],
  attributes: &[
  ],
  events: &[
  ],
  commands: &[
    command::ContentAppMessage::COMMAND,
  ],
};
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ContentAppMessage {
  pub data: String,
  pub encoding_hint: String,
}
impl Command for ContentAppMessage {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "ContentAppMessage",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Data", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "EncodingHint", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for ContentAppMessage {
  type Response = ContentAppMessageResponse;
}
impl TlvEncode for ContentAppMessage {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.data.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.encoding_hint.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContentAppMessageResponse {
  pub status: ty::StatusEnum,
  pub data: String,
  pub encoding_hint: String,
}
impl Command for ContentAppMessageResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "ContentAppMessageResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <ty::StatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Data", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "EncodingHint", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandResponse for ContentAppMessageResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StatusEnum {
  ///Command succeeded
  Success = 0x0000,
  ///Data field in command was not understood by the Observer
  UnexpectedData = 0x0001,
}
impl AttrType for StatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Success"),
  (0x0001, "UnexpectedData"),
  ]);
}
impl TlvPut for StatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Success),
      0x0001 => Ok(Self::UnexpectedData),
      _ => Err(())
    }
  }
}
impl TlvDecode for StatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
