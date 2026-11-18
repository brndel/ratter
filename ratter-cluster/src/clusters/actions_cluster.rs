#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const ACTIONS: ClusterDef = ClusterDef {
  id: 0x0025,
  name: "Actions",
  features: &[
  ],
  attributes: &[
    attr::ActionList::ATTR,
    attr::EndpointLists::ATTR,
    attr::SetupURL::ATTR,
  ],
  events: &[
    event::StateChanged::EVENT,
    event::ActionFailed::EVENT,
  ],
  commands: &[
    command::InstantAction::COMMAND,
    command::InstantActionWithTransition::COMMAND,
    command::StartAction::COMMAND,
    command::StartActionWithDuration::COMMAND,
    command::StopAction::COMMAND,
    command::PauseAction::COMMAND,
    command::PauseActionWithDuration::COMMAND,
    command::ResumeAction::COMMAND,
    command::EnableAction::COMMAND,
    command::EnableActionWithDuration::COMMAND,
    command::DisableAction::COMMAND,
    command::DisableActionWithDuration::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct ActionList;

  impl Attr for ActionList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "ActionList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::ActionStruct>;
  }


  pub struct EndpointLists;

  impl Attr for EndpointLists {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "EndpointLists",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::EndpointListStruct>;
  }


  pub struct SetupURL;

  impl Attr for SetupURL {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "SetupURL",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = String;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct StateChanged {
  pub action_id: u16,
  pub invoke_id: u32,
  pub new_state: ty::ActionStateEnum,
}

impl Event for StateChanged {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "StateChanged",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "InvokeID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "NewState", attr_ty: <ty::ActionStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for StateChanged {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut action_id: Option<u16> = None;
    let mut invoke_id: Option<u32> = None;
    let mut new_state: Option<ty::ActionStateEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { action_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { invoke_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { new_state = Some(<ty::ActionStateEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      action_id: action_id.ok_or(TlvDecodeErr::MissingField("ActionID"))?,
      invoke_id: invoke_id.ok_or(TlvDecodeErr::MissingField("InvokeID"))?,
      new_state: new_state.ok_or(TlvDecodeErr::MissingField("NewState"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ActionFailed {
  pub action_id: u16,
  pub invoke_id: u32,
  pub new_state: ty::ActionStateEnum,
  pub error: ty::ActionErrorEnum,
}

impl Event for ActionFailed {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "ActionFailed",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "InvokeID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "NewState", attr_ty: <ty::ActionStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "Error", attr_ty: <ty::ActionErrorEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for ActionFailed {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut action_id: Option<u16> = None;
    let mut invoke_id: Option<u32> = None;
    let mut new_state: Option<ty::ActionStateEnum> = None;
    let mut error: Option<ty::ActionErrorEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { action_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { invoke_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { new_state = Some(<ty::ActionStateEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { error = Some(<ty::ActionErrorEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      action_id: action_id.ok_or(TlvDecodeErr::MissingField("ActionID"))?,
      invoke_id: invoke_id.ok_or(TlvDecodeErr::MissingField("InvokeID"))?,
      new_state: new_state.ok_or(TlvDecodeErr::MissingField("NewState"))?,
      error: error.ok_or(TlvDecodeErr::MissingField("Error"))?,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct InstantAction {
  pub action_id: u16,
  pub invoke_id: u32,
}
impl Command for InstantAction {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "InstantAction",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "InvokeID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for InstantAction {
  type Response = BasicAccResponse;
}
impl TlvEncode for InstantAction {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.action_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.invoke_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct InstantActionWithTransition {
  pub action_id: u16,
  pub invoke_id: u32,
  pub transition_time: u16,
}
impl Command for InstantActionWithTransition {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "InstantActionWithTransition",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "InvokeID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "TransitionTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for InstantActionWithTransition {
  type Response = BasicAccResponse;
}
impl TlvEncode for InstantActionWithTransition {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.action_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.invoke_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StartAction {
  pub action_id: u16,
  pub invoke_id: u32,
}
impl Command for StartAction {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "StartAction",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "InvokeID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for StartAction {
  type Response = BasicAccResponse;
}
impl TlvEncode for StartAction {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.action_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.invoke_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StartActionWithDuration {
  pub action_id: u16,
  pub invoke_id: u32,
  pub duration: u32,
}
impl Command for StartActionWithDuration {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "StartActionWithDuration",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "InvokeID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "Duration", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for StartActionWithDuration {
  type Response = BasicAccResponse;
}
impl TlvEncode for StartActionWithDuration {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.action_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.invoke_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.duration.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StopAction {
  pub action_id: u16,
  pub invoke_id: u32,
}
impl Command for StopAction {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "StopAction",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "InvokeID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for StopAction {
  type Response = BasicAccResponse;
}
impl TlvEncode for StopAction {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.action_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.invoke_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PauseAction {
  pub action_id: u16,
  pub invoke_id: u32,
}
impl Command for PauseAction {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "PauseAction",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "InvokeID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for PauseAction {
  type Response = BasicAccResponse;
}
impl TlvEncode for PauseAction {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.action_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.invoke_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PauseActionWithDuration {
  pub action_id: u16,
  pub invoke_id: u32,
  pub duration: u32,
}
impl Command for PauseActionWithDuration {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "PauseActionWithDuration",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "InvokeID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "Duration", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for PauseActionWithDuration {
  type Response = BasicAccResponse;
}
impl TlvEncode for PauseActionWithDuration {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.action_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.invoke_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.duration.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResumeAction {
  pub action_id: u16,
  pub invoke_id: u32,
}
impl Command for ResumeAction {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "ResumeAction",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "InvokeID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for ResumeAction {
  type Response = BasicAccResponse;
}
impl TlvEncode for ResumeAction {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.action_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.invoke_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnableAction {
  pub action_id: u16,
  pub invoke_id: u32,
}
impl Command for EnableAction {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0008,
    name: "EnableAction",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "InvokeID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for EnableAction {
  type Response = BasicAccResponse;
}
impl TlvEncode for EnableAction {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.action_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.invoke_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnableActionWithDuration {
  pub action_id: u16,
  pub invoke_id: u32,
  pub duration: u32,
}
impl Command for EnableActionWithDuration {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0009,
    name: "EnableActionWithDuration",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "InvokeID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "Duration", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for EnableActionWithDuration {
  type Response = BasicAccResponse;
}
impl TlvEncode for EnableActionWithDuration {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.action_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.invoke_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.duration.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisableAction {
  pub action_id: u16,
  pub invoke_id: u32,
}
impl Command for DisableAction {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000a,
    name: "DisableAction",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "InvokeID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for DisableAction {
  type Response = BasicAccResponse;
}
impl TlvEncode for DisableAction {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.action_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.invoke_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisableActionWithDuration {
  pub action_id: u16,
  pub invoke_id: u32,
  pub duration: u32,
}
impl Command for DisableActionWithDuration {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000b,
    name: "DisableActionWithDuration",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "InvokeID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "Duration", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for DisableActionWithDuration {
  type Response = BasicAccResponse;
}
impl TlvEncode for DisableActionWithDuration {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.action_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.invoke_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.duration.tlv_put(matter_codec::Tag::Context(0x0002), writer);
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
pub enum ActionErrorEnum {
  ///Other reason not listed in the row(s) below
  Unknown = 0x0000,
  ///The action was interrupted by another command or interaction
  Interrupted = 0x0001,
}
impl AttrType for ActionErrorEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unknown"),
  (0x0001, "Interrupted"),
  ]);
}
impl TlvPut for ActionErrorEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ActionErrorEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unknown),
      0x0001 => Ok(Self::Interrupted),
      _ => Err(())
    }
  }
}
impl TlvDecode for ActionErrorEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ActionStateEnum {
  ///The action is not active
  Inactive = 0x0000,
  ///The action is active
  Active = 0x0001,
  ///The action has been paused
  Paused = 0x0002,
  ///The action has been disabled
  Disabled = 0x0003,
}
impl AttrType for ActionStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Inactive"),
  (0x0001, "Active"),
  (0x0002, "Paused"),
  (0x0003, "Disabled"),
  ]);
}
impl TlvPut for ActionStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ActionStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Inactive),
      0x0001 => Ok(Self::Active),
      0x0002 => Ok(Self::Paused),
      0x0003 => Ok(Self::Disabled),
      _ => Err(())
    }
  }
}
impl TlvDecode for ActionStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ActionTypeEnum {
  ///Use this only when none of the other values applies
  Other = 0x0000,
  ///Bring the endpoints into a certain state
  Scene = 0x0001,
  ///A sequence of states with a certain time pattern
  Sequence = 0x0002,
  ///Control an automation (e.g. motion sensor controlling lights)
  Automation = 0x0003,
  ///Sequence that will run when something doesn't happen
  Exception = 0x0004,
  ///Use the endpoints to send a message to user
  Notification = 0x0005,
  ///Higher priority notification
  Alarm = 0x0006,
}
impl AttrType for ActionTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Other"),
  (0x0001, "Scene"),
  (0x0002, "Sequence"),
  (0x0003, "Automation"),
  (0x0004, "Exception"),
  (0x0005, "Notification"),
  (0x0006, "Alarm"),
  ]);
}
impl TlvPut for ActionTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ActionTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Other),
      0x0001 => Ok(Self::Scene),
      0x0002 => Ok(Self::Sequence),
      0x0003 => Ok(Self::Automation),
      0x0004 => Ok(Self::Exception),
      0x0005 => Ok(Self::Notification),
      0x0006 => Ok(Self::Alarm),
      _ => Err(())
    }
  }
}
impl TlvDecode for ActionTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum EndpointListTypeEnum {
  ///Another group of endpoints
  Other = 0x0000,
  ///User-configured group of endpoints where an endpoint can be in only one room
  Room = 0x0001,
  ///User-configured group of endpoints where an endpoint can be in any number of zones
  Zone = 0x0002,
}
impl AttrType for EndpointListTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Other"),
  (0x0001, "Room"),
  (0x0002, "Zone"),
  ]);
}
impl TlvPut for EndpointListTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for EndpointListTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Other),
      0x0001 => Ok(Self::Room),
      0x0002 => Ok(Self::Zone),
      _ => Err(())
    }
  }
}
impl TlvDecode for EndpointListTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct CommandBits: u32 {
    ///Indicate support for InstantAction command
    const InstantAction = 1 << 0;
    ///Indicate support for InstantActionWithTransition command
    const InstantActionWithTransition = 1 << 1;
    ///Indicate support for StartAction command
    const StartAction = 1 << 2;
    ///Indicate support for StartActionWithDuration command
    const StartActionWithDuration = 1 << 3;
    ///Indicate support for StopAction command
    const StopAction = 1 << 4;
    ///Indicate support for PauseAction command
    const PauseAction = 1 << 5;
    ///Indicate support for PauseActionWithDuration command
    const PauseActionWithDuration = 1 << 6;
    ///Indicate support for ResumeAction command
    const ResumeAction = 1 << 7;
    ///Indicate support for EnableAction command
    const EnableAction = 1 << 8;
    ///Indicate support for EnableActionWithDuration command
    const EnableActionWithDuration = 1 << 9;
    ///Indicate support for DisableAction command
    const DisableAction = 1 << 10;
    ///Indicate support for DisableActionWithDuration command
    const DisableActionWithDuration = 1 << 11;
  }
}
impl AttrType for CommandBits {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "InstantAction"),
    (1, "InstantActionWithTransition"),
    (2, "StartAction"),
    (3, "StartActionWithDuration"),
    (4, "StopAction"),
    (5, "PauseAction"),
    (6, "PauseActionWithDuration"),
    (7, "ResumeAction"),
    (8, "EnableAction"),
    (9, "EnableActionWithDuration"),
    (10, "DisableAction"),
    (11, "DisableActionWithDuration"),
  ]);
}
impl TlvPut for CommandBits {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for CommandBits {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for CommandBits {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ActionStruct {
  pub action_id: u16,
  pub name: String,
  pub r#type: ActionTypeEnum,
  pub endpoint_list_id: u16,
  pub supported_commands: CommandBits,
  pub state: ActionStateEnum,
}
impl AttrType for ActionStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ActionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Type", attr_ty: <ActionTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "EndpointListID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "SupportedCommands", attr_ty: <CommandBits>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "State", attr_ty: <ActionStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ActionStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.action_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.r#type.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.endpoint_list_id.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.supported_commands.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.state.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ActionStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut action_id: Option<u16> = None;
    let mut name: Option<String> = None;
    let mut r#type: Option<ActionTypeEnum> = None;
    let mut endpoint_list_id: Option<u16> = None;
    let mut supported_commands: Option<CommandBits> = None;
    let mut state: Option<ActionStateEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { action_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { r#type = Some(<ActionTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { endpoint_list_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { supported_commands = Some(<CommandBits>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { state = Some(<ActionStateEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      action_id: action_id.ok_or(TlvDecodeErr::MissingField("ActionID"))?,
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
      r#type: r#type.ok_or(TlvDecodeErr::MissingField("Type"))?,
      endpoint_list_id: endpoint_list_id.ok_or(TlvDecodeErr::MissingField("EndpointListID"))?,
      supported_commands: supported_commands.ok_or(TlvDecodeErr::MissingField("SupportedCommands"))?,
      state: state.ok_or(TlvDecodeErr::MissingField("State"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EndpointListStruct {
  pub endpoint_list_id: u16,
  pub name: String,
  pub r#type: EndpointListTypeEnum,
  pub endpoints: Vec<TypeUnsupported /* EndpointNo */>,
}
impl AttrType for EndpointListStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "EndpointListID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Type", attr_ty: <EndpointListTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "Endpoints", attr_ty: <Vec<TypeUnsupported /* EndpointNo */>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for EndpointListStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.endpoint_list_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.r#type.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.endpoints.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for EndpointListStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut endpoint_list_id: Option<u16> = None;
    let mut name: Option<String> = None;
    let mut r#type: Option<EndpointListTypeEnum> = None;
    let mut endpoints: Option<Vec<TypeUnsupported /* EndpointNo */>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { endpoint_list_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { r#type = Some(<EndpointListTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0003), kind: _ }) => { endpoints = Some(<Vec<TypeUnsupported /* EndpointNo */>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      endpoint_list_id: endpoint_list_id.ok_or(TlvDecodeErr::MissingField("EndpointListID"))?,
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
      r#type: r#type.ok_or(TlvDecodeErr::MissingField("Type"))?,
      endpoints: endpoints.ok_or(TlvDecodeErr::MissingField("Endpoints"))?,
    })
  }
}

}
