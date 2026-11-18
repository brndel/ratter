#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const VALVE_CONFIGURATION_AND_CONTROL: ClusterDef = ClusterDef {
  id: 0x0081,
  name: "Valve Configuration and Control",
  features: &[
    FeatureDef { bit: 0x0000, code: "TS", name: "TimeSync", summary: "UTC time is used for time indications" },
    FeatureDef { bit: 0x0001, code: "LVL", name: "Level", summary: "Device supports setting the specific position of the valve" },
  ],
  attributes: &[
    attr::OpenDuration::ATTR,
    attr::DefaultOpenDuration::ATTR,
    attr::AutoCloseTime::ATTR,
    attr::RemainingDuration::ATTR,
    attr::CurrentState::ATTR,
    attr::TargetState::ATTR,
    attr::CurrentLevel::ATTR,
    attr::TargetLevel::ATTR,
    attr::DefaultOpenLevel::ATTR,
    attr::ValveFault::ATTR,
    attr::LevelStep::ATTR,
  ],
  events: &[
    event::ValveStateChanged::EVENT,
    event::ValveFault::EVENT,
  ],
  commands: &[
    command::Open::COMMAND,
    command::Close::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct OpenDuration;

  impl Attr for OpenDuration {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "OpenDuration",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* ElapsedS */>;
  }


  pub struct DefaultOpenDuration;

  impl Attr for DefaultOpenDuration {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "DefaultOpenDuration",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* ElapsedS */>;
  }


  pub struct AutoCloseTime;

  impl Attr for AutoCloseTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "AutoCloseTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<TypeUnsupported /* EpochUs */>;
  }


  pub struct RemainingDuration;

  impl Attr for RemainingDuration {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "RemainingDuration",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* ElapsedS */>;
  }


  pub struct CurrentState;

  impl Attr for CurrentState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "CurrentState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::ValveStateEnum>;
  }


  pub struct TargetState;

  impl Attr for TargetState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "TargetState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::ValveStateEnum>;
  }


  pub struct CurrentLevel;

  impl Attr for CurrentLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "CurrentLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Nullable<TypeUnsupported /* Percent */>;
  }


  pub struct TargetLevel;

  impl Attr for TargetLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "TargetLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Nullable<TypeUnsupported /* Percent */>;
  }


  pub struct DefaultOpenLevel;

  impl Attr for DefaultOpenLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "DefaultOpenLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = TypeUnsupported /* Percent */;
  }


  pub struct ValveFault;

  impl Attr for ValveFault {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "ValveFault",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::ValveFaultBitmap;
  }


  pub struct LevelStep;

  impl Attr for LevelStep {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "LevelStep",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u8;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ValveStateChanged {
  pub valve_state: ty::ValveStateEnum,
  pub valve_level: TypeUnsupported /* Percent */,
}

impl Event for ValveStateChanged {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "ValveStateChanged",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "ValveState", attr_ty: <ty::ValveStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ValveLevel", attr_ty: <TypeUnsupported /* Percent */>::ATTR_TYPE, conformance: Conformance::Feature(1) },
    ],
  };
}

impl TlvDecode for ValveStateChanged {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut valve_state: Option<ty::ValveStateEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { valve_state = Some(<ty::ValveStateEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      valve_state: valve_state.ok_or(TlvDecodeErr::MissingField("ValveState"))?,
      valve_level: TypeUnsupported,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ValveFault {
  pub valve_fault: ty::ValveFaultBitmap,
}

impl Event for ValveFault {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "ValveFault",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "ValveFault", attr_ty: <ty::ValveFaultBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for ValveFault {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut valve_fault: Option<ty::ValveFaultBitmap> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { valve_fault = Some(<ty::ValveFaultBitmap>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      valve_fault: valve_fault.ok_or(TlvDecodeErr::MissingField("ValveFault"))?,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Open {
  pub open_duration: TypeUnsupported /* ElapsedS */,
  pub target_level: TypeUnsupported /* Percent */,
}
impl Command for Open {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "Open",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "OpenDuration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0001, name: "TargetLevel", attr_ty: <TypeUnsupported /* Percent */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for Open {
  type Response = BasicAccResponse;
}
impl TlvEncode for Open {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.open_duration.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.target_level.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Close {
}
impl Command for Close {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "Close",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for Close {
  type Response = BasicAccResponse;
}
impl TlvEncode for Close {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
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
pub enum StatusCodeEnum {
  ///The requested action could not be performed due to a fault on the valve.
  FailureDueToFault = 0x0002,
}
impl AttrType for StatusCodeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0002, "FailureDueToFault"),
  ]);
}
impl TlvPut for StatusCodeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StatusCodeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0002 => Ok(Self::FailureDueToFault),
      _ => Err(())
    }
  }
}
impl TlvDecode for StatusCodeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ValveStateEnum {
  ///Valve is in closed position
  Closed = 0x0000,
  ///Valve is in open position
  Open = 0x0001,
  ///Valve is transitioning between closed and open positions or between levels
  Transitioning = 0x0002,
}
impl AttrType for ValveStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Closed"),
  (0x0001, "Open"),
  (0x0002, "Transitioning"),
  ]);
}
impl TlvPut for ValveStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ValveStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Closed),
      0x0001 => Ok(Self::Open),
      0x0002 => Ok(Self::Transitioning),
      _ => Err(())
    }
  }
}
impl TlvDecode for ValveStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct ValveFaultBitmap: u32 {
    ///Unspecified fault detected
    const GeneralFault = 1 << 0;
    ///Valve is blocked
    const Blocked = 1 << 1;
    ///Valve has detected a leak
    const Leaking = 1 << 2;
    ///No valve is connected to controller
    const NotConnected = 1 << 3;
    ///Short circuit is detected
    const ShortCircuit = 1 << 4;
    ///The available current has been exceeded
    const CurrentExceeded = 1 << 5;
  }
}
impl AttrType for ValveFaultBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "GeneralFault"),
    (1, "Blocked"),
    (2, "Leaking"),
    (3, "NotConnected"),
    (4, "ShortCircuit"),
    (5, "CurrentExceeded"),
  ]);
}
impl TlvPut for ValveFaultBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for ValveFaultBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for ValveFaultBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
}
