#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const TARGET_NAVIGATOR: ClusterDef = ClusterDef {
  id: 0x0505,
  name: "Target Navigator",
  features: &[
  ],
  attributes: &[
    attr::TargetList::ATTR,
    attr::CurrentTarget::ATTR,
  ],
  events: &[
    event::TargetUpdated::EVENT,
  ],
  commands: &[
    command::NavigateTarget::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct TargetList;

  impl Attr for TargetList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "TargetList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::TargetInfoStruct>;
  }


  pub struct CurrentTarget;

  impl Attr for CurrentTarget {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "CurrentTarget",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u8;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct TargetUpdated {
  pub target_list: Vec<ty::TargetInfoStruct>,
  pub current_target: u8,
  pub data: TypeUnsupported /* Octstr */,
}

impl Event for TargetUpdated {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "TargetUpdated",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "TargetList", attr_ty: <Vec<ty::TargetInfoStruct>>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0001, name: "CurrentTarget", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "Data", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}

impl TlvDecode for TargetUpdated {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut target_list: Option<Vec<ty::TargetInfoStruct>> = None;
    let mut current_target: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { target_list = Some(<Vec<ty::TargetInfoStruct>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { current_target = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      target_list: target_list.ok_or(TlvDecodeErr::MissingField("TargetList"))?,
      current_target: current_target.ok_or(TlvDecodeErr::MissingField("CurrentTarget"))?,
      data: TypeUnsupported,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct NavigateTarget {
  pub target: u8,
  pub data: String,
}
impl Command for NavigateTarget {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "NavigateTarget",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Target", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Data", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for NavigateTarget {
  type Response = NavigateTargetResponse;
}
impl TlvEncode for NavigateTarget {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.target.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.data.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NavigateTargetResponse {
  pub status: ty::StatusEnum,
  pub data: String,
}
impl Command for NavigateTargetResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "NavigateTargetResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <ty::StatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Data", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandResponse for NavigateTargetResponse {}

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
  ///Requested target was not found in the TargetList
  TargetNotFound = 0x0001,
  ///Target request is not allowed in current state.
  NotAllowed = 0x0002,
}
impl AttrType for StatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Success"),
  (0x0001, "TargetNotFound"),
  (0x0002, "NotAllowed"),
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
      0x0001 => Ok(Self::TargetNotFound),
      0x0002 => Ok(Self::NotAllowed),
      _ => Err(())
    }
  }
}
impl TlvDecode for StatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct TargetInfoStruct {
  pub identifier: u8,
  pub name: String,
}
impl AttrType for TargetInfoStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Identifier", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TargetInfoStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.identifier.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TargetInfoStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut identifier: Option<u8> = None;
    let mut name: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { identifier = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      identifier: identifier.ok_or(TlvDecodeErr::MissingField("Identifier"))?,
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
    })
  }
}

}
