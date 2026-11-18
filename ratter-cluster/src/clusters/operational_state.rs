#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const OPERATIONAL_STATE: ClusterDef = ClusterDef {
  id: 0x0060,
  name: "Operational State",
  features: &[
  ],
  attributes: &[
    attr::PhaseList::ATTR,
    attr::CurrentPhase::ATTR,
    attr::CountdownTime::ATTR,
    attr::OperationalStateList::ATTR,
    attr::OperationalState::ATTR,
    attr::OperationalError::ATTR,
  ],
  events: &[
    event::OperationalError::EVENT,
    event::OperationCompletion::EVENT,
  ],
  commands: &[
    command::Pause::COMMAND,
    command::Stop::COMMAND,
    command::Start::COMMAND,
    command::Resume::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct PhaseList;

  impl Attr for PhaseList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "PhaseList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<Vec<String>>;
  }


  pub struct CurrentPhase;

  impl Attr for CurrentPhase {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "CurrentPhase",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u8>;
  }


  pub struct CountdownTime;

  impl Attr for CountdownTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "CountdownTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<TypeUnsupported /* ElapsedS */>;
  }


  pub struct OperationalStateList;

  impl Attr for OperationalStateList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "OperationalStateList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::OperationalStateStruct>;
  }


  pub struct OperationalState;

  impl Attr for OperationalState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "OperationalState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::OperationalStateEnum;
  }


  pub struct OperationalError;

  impl Attr for OperationalError {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "OperationalError",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::ErrorStateStruct;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct OperationalError {
  pub error_state: ty::ErrorStateStruct,
}

impl Event for OperationalError {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "OperationalError",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ErrorState", attr_ty: <ty::ErrorStateStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for OperationalError {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut error_state: Option<ty::ErrorStateStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { error_state = Some(<ty::ErrorStateStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      error_state: error_state.ok_or(TlvDecodeErr::MissingField("ErrorState"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct OperationCompletion {
  pub completion_error_code: TypeUnsupported /* Enum8 */,
  pub total_operational_time: TypeUnsupported /* ElapsedS */,
  pub paused_time: TypeUnsupported /* ElapsedS */,
}

impl Event for OperationCompletion {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "OperationCompletion",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "CompletionErrorCode", attr_ty: <TypeUnsupported /* Enum8 */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "TotalOperationalTime", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "PausedTime", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}

impl TlvDecode for OperationCompletion {
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
      completion_error_code: TypeUnsupported,
      total_operational_time: TypeUnsupported,
      paused_time: TypeUnsupported,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Pause {
}
impl Command for Pause {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "Pause",
    conformance: Conformance::TooComplicated,
    fields: &[
    ],
  };
}
impl CommandInvoke for Pause {
  type Response = OperationalCommandResponse;
}
impl TlvEncode for Pause {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stop {
}
impl Command for Stop {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "Stop",
    conformance: Conformance::TooComplicated,
    fields: &[
    ],
  };
}
impl CommandInvoke for Stop {
  type Response = OperationalCommandResponse;
}
impl TlvEncode for Stop {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Start {
}
impl Command for Start {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "Start",
    conformance: Conformance::Optional,
    fields: &[
    ],
  };
}
impl CommandInvoke for Start {
  type Response = OperationalCommandResponse;
}
impl TlvEncode for Start {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Resume {
}
impl Command for Resume {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "Resume",
    conformance: Conformance::TooComplicated,
    fields: &[
    ],
  };
}
impl CommandInvoke for Resume {
  type Response = OperationalCommandResponse;
}
impl TlvEncode for Resume {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperationalCommandResponse {
  pub command_response_state: ty::ErrorStateStruct,
}
impl Command for OperationalCommandResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "OperationalCommandResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CommandResponseState", attr_ty: <ty::ErrorStateStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for OperationalCommandResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ErrorStateEnum {
  ///The device is not in an error state
  NoError = 0x0000,
  ///The device is unable to start or resume operation
  UnableToStartOrResume = 0x0001,
  ///The device was unable to complete the current operation
  UnableToCompleteOperation = 0x0002,
  ///The device cannot process the command in its current state
  CommandInvalidInState = 0x0003,
}
impl AttrType for ErrorStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "NoError"),
  (0x0001, "UnableToStartOrResume"),
  (0x0002, "UnableToCompleteOperation"),
  (0x0003, "CommandInvalidInState"),
  ]);
}
impl TlvPut for ErrorStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ErrorStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::NoError),
      0x0001 => Ok(Self::UnableToStartOrResume),
      0x0002 => Ok(Self::UnableToCompleteOperation),
      0x0003 => Ok(Self::CommandInvalidInState),
      _ => Err(())
    }
  }
}
impl TlvDecode for ErrorStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum OperationalStateEnum {
  ///The device is stopped
  Stopped = 0x0000,
  ///The device is operating
  Running = 0x0001,
  ///The device is paused during an operation
  Paused = 0x0002,
  ///The device is in an error state
  Error = 0x0003,
}
impl AttrType for OperationalStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Stopped"),
  (0x0001, "Running"),
  (0x0002, "Paused"),
  (0x0003, "Error"),
  ]);
}
impl TlvPut for OperationalStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for OperationalStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Stopped),
      0x0001 => Ok(Self::Running),
      0x0002 => Ok(Self::Paused),
      0x0003 => Ok(Self::Error),
      _ => Err(())
    }
  }
}
impl TlvDecode for OperationalStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorStateStruct {
  pub error_state_id: ErrorStateEnum,
  pub error_state_label: String,
  pub error_state_details: String,
}
impl AttrType for ErrorStateStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ErrorStateID", attr_ty: <ErrorStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ErrorStateLabel", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "ErrorStateDetails", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ErrorStateStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.error_state_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.error_state_label.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.error_state_details.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ErrorStateStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut error_state_id: Option<ErrorStateEnum> = None;
    let mut error_state_label: Option<String> = None;
    let mut error_state_details: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { error_state_id = Some(<ErrorStateEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { error_state_label = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { error_state_details = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      error_state_id: error_state_id.ok_or(TlvDecodeErr::MissingField("ErrorStateID"))?,
      error_state_label: error_state_label.ok_or(TlvDecodeErr::MissingField("ErrorStateLabel"))?,
      error_state_details: error_state_details.ok_or(TlvDecodeErr::MissingField("ErrorStateDetails"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperationalStateStruct {
  pub operational_state_id: OperationalStateEnum,
  pub operational_state_label: String,
}
impl AttrType for OperationalStateStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "OperationalStateID", attr_ty: <OperationalStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "OperationalStateLabel", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for OperationalStateStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.operational_state_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.operational_state_label.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for OperationalStateStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut operational_state_id: Option<OperationalStateEnum> = None;
    let mut operational_state_label: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { operational_state_id = Some(<OperationalStateEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { operational_state_label = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      operational_state_id: operational_state_id.ok_or(TlvDecodeErr::MissingField("OperationalStateID"))?,
      operational_state_label: operational_state_label.ok_or(TlvDecodeErr::MissingField("OperationalStateLabel"))?,
    })
  }
}

}
