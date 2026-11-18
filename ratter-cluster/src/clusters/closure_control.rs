#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const CLOSURE_CONTROL: ClusterDef = ClusterDef {
  id: 0x0104,
  name: "Closure Control",
  features: &[
    FeatureDef { bit: 0x0000, code: "PS", name: "Positioning", summary: "Supports Positioning with at least Fully Opened (0%) and Fully Closed (100%) discrete positions" },
    FeatureDef { bit: 0x0001, code: "LT", name: "MotionLatching", summary: "Supports a latch (securing a position, or a state)" },
    FeatureDef { bit: 0x0002, code: "IS", name: "Instantaneous", summary: "Supports the Instantaneous feature" },
    FeatureDef { bit: 0x0003, code: "SP", name: "Speed", summary: "Supports Speed motion throttling" },
    FeatureDef { bit: 0x0004, code: "VT", name: "Ventilation", summary: "Supports Ventilation discrete state" },
    FeatureDef { bit: 0x0005, code: "PD", name: "Pedestrian", summary: "Supports Pedestrian discrete state" },
    FeatureDef { bit: 0x0006, code: "CL", name: "Calibration", summary: "Supports the Calibration feature" },
    FeatureDef { bit: 0x0007, code: "PT", name: "Protection", summary: "Supports the Protection feature" },
    FeatureDef { bit: 0x0008, code: "MO", name: "ManuallyOperable", summary: "Supports the manual operation feature" },
  ],
  attributes: &[
    attr::CountdownTime::ATTR,
    attr::MainState::ATTR,
    attr::CurrentErrorList::ATTR,
    attr::OverallCurrentState::ATTR,
    attr::OverallTargetState::ATTR,
    attr::LatchControlModes::ATTR,
  ],
  events: &[
    event::OperationalError::EVENT,
    event::MovementCompleted::EVENT,
    event::EngageStateChanged::EVENT,
    event::SecureStateChanged::EVENT,
  ],
  commands: &[
    command::Stop::COMMAND,
    command::MoveTo::COMMAND,
    command::Calibrate::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct CountdownTime;

  impl Attr for CountdownTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "CountdownTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<TypeUnsupported /* ElapsedS */>;
  }


  pub struct MainState;

  impl Attr for MainState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "MainState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::MainStateEnum;
  }


  pub struct CurrentErrorList;

  impl Attr for CurrentErrorList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "CurrentErrorList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::ClosureErrorEnum>;
  }


  pub struct OverallCurrentState;

  impl Attr for OverallCurrentState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "OverallCurrentState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::OverallCurrentStateStruct>;
  }


  pub struct OverallTargetState;

  impl Attr for OverallTargetState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "OverallTargetState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::OverallTargetStateStruct>;
  }


  pub struct LatchControlModes;

  impl Attr for LatchControlModes {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "LatchControlModes",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = ty::LatchControlModesBitmap;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct OperationalError {
  pub error_state: Vec<ty::ClosureErrorEnum>,
}

impl Event for OperationalError {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "OperationalError",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ErrorState", attr_ty: <Vec<ty::ClosureErrorEnum>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for OperationalError {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut error_state: Option<Vec<ty::ClosureErrorEnum>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { error_state = Some(<Vec<ty::ClosureErrorEnum>>::tlv_decode(v, reader)?); },
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
pub struct MovementCompleted {
}

impl Event for MovementCompleted {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "MovementCompleted",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}

impl TlvDecode for MovementCompleted {
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
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct EngageStateChanged {
  pub engage_value: bool,
}

impl Event for EngageStateChanged {
  const EVENT: EventDef = EventDef {
    id: 0x0002,
    name: "EngageStateChanged",
    conformance: Conformance::Feature(8),
    fields: &[
      AttrDef { id: 0x0000, name: "EngageValue", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for EngageStateChanged {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut engage_value: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { engage_value = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      engage_value: engage_value.ok_or(TlvDecodeErr::MissingField("EngageValue"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct SecureStateChanged {
  pub secure_value: bool,
}

impl Event for SecureStateChanged {
  const EVENT: EventDef = EventDef {
    id: 0x0003,
    name: "SecureStateChanged",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "SecureValue", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for SecureStateChanged {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut secure_value: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { secure_value = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      secure_value: secure_value.ok_or(TlvDecodeErr::MissingField("SecureValue"))?,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Stop {
}
impl Command for Stop {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "Stop",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for Stop {
  type Response = BasicAccResponse;
}
impl TlvEncode for Stop {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveTo {
  pub position: ty::TargetPositionEnum,
  pub latch: bool,
  pub speed: ty::ThreeLevelAutoEnum,
}
impl Command for MoveTo {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "MoveTo",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Position", attr_ty: <ty::TargetPositionEnum>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0001, name: "Latch", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "Speed", attr_ty: <ty::ThreeLevelAutoEnum>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for MoveTo {
  type Response = BasicAccResponse;
}
impl TlvEncode for MoveTo {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.position.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.latch.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.speed.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Calibrate {
}
impl Command for Calibrate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "Calibrate",
    conformance: Conformance::Feature(6),
    fields: &[
    ],
  };
}
impl CommandInvoke for Calibrate {
  type Response = BasicAccResponse;
}
impl TlvEncode for Calibrate {
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
pub enum ClosureErrorEnum {
  ///An obstacle is blocking the closure movement
  PhysicallyBlocked = 0x0000,
  ///The closure is unsafe to move, as determined by a sensor (e.g. photoelectric sensor) before attempting movement
  BlockedBySensor = 0x0001,
  ///A warning raised by the closure that indicates an over-temperature, e.g. due to excessive drive or stall current
  TemperatureLimited = 0x0002,
  ///Some malfunctions that are not easily recoverable are detected, or urgent servicing is needed
  MaintenanceRequired = 0x0003,
  ///An internal element is prohibiting motion, e.g. an integrated door within a bigger garage door is open and prevents motion
  InternalInterference = 0x0004,
}
impl AttrType for ClosureErrorEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "PhysicallyBlocked"),
  (0x0001, "BlockedBySensor"),
  (0x0002, "TemperatureLimited"),
  (0x0003, "MaintenanceRequired"),
  (0x0004, "InternalInterference"),
  ]);
}
impl TlvPut for ClosureErrorEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ClosureErrorEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::PhysicallyBlocked),
      0x0001 => Ok(Self::BlockedBySensor),
      0x0002 => Ok(Self::TemperatureLimited),
      0x0003 => Ok(Self::MaintenanceRequired),
      0x0004 => Ok(Self::InternalInterference),
      _ => Err(())
    }
  }
}
impl TlvDecode for ClosureErrorEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum CurrentPositionEnum {
  ///Fully closed state
  FullyClosed = 0x0000,
  ///Fully opened state
  FullyOpened = 0x0001,
  ///Partially opened state (closure is not fully opened or fully closed)
  PartiallyOpened = 0x0002,
  ///Closure is in the Pedestrian position
  OpenedForPedestrian = 0x0003,
  ///Closure is in the Ventilation position
  OpenedForVentilation = 0x0004,
  ///Closure is in its "Signature position"
  OpenedAtSignature = 0x0005,
}
impl AttrType for CurrentPositionEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "FullyClosed"),
  (0x0001, "FullyOpened"),
  (0x0002, "PartiallyOpened"),
  (0x0003, "OpenedForPedestrian"),
  (0x0004, "OpenedForVentilation"),
  (0x0005, "OpenedAtSignature"),
  ]);
}
impl TlvPut for CurrentPositionEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for CurrentPositionEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::FullyClosed),
      0x0001 => Ok(Self::FullyOpened),
      0x0002 => Ok(Self::PartiallyOpened),
      0x0003 => Ok(Self::OpenedForPedestrian),
      0x0004 => Ok(Self::OpenedForVentilation),
      0x0005 => Ok(Self::OpenedAtSignature),
      _ => Err(())
    }
  }
}
impl TlvDecode for CurrentPositionEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum MainStateEnum {
  ///Closure is stopped
  Stopped = 0x0000,
  ///Closure is actively moving
  Moving = 0x0001,
  ///Closure is waiting before a motion (e.g. pre-heat, pre-check)
  WaitingForMotion = 0x0002,
  ///Closure is in an error state
  Error = 0x0003,
  ///Closure is currently calibrating its Opened and Closed limits to determine effective physical range
  Calibrating = 0x0004,
  ///Some protective measures are activated to prevent damage to the closure. Commands MAY be rejected.
  Protected = 0x0005,
  ///Closure has a disengaged element preventing any actuator movements
  Disengaged = 0x0006,
  ///Movement commands are ignored since the closure is not operational and requires further setup and/or calibration
  SetupRequired = 0x0007,
}
impl AttrType for MainStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Stopped"),
  (0x0001, "Moving"),
  (0x0002, "WaitingForMotion"),
  (0x0003, "Error"),
  (0x0004, "Calibrating"),
  (0x0005, "Protected"),
  (0x0006, "Disengaged"),
  (0x0007, "SetupRequired"),
  ]);
}
impl TlvPut for MainStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for MainStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Stopped),
      0x0001 => Ok(Self::Moving),
      0x0002 => Ok(Self::WaitingForMotion),
      0x0003 => Ok(Self::Error),
      0x0004 => Ok(Self::Calibrating),
      0x0005 => Ok(Self::Protected),
      0x0006 => Ok(Self::Disengaged),
      0x0007 => Ok(Self::SetupRequired),
      _ => Err(())
    }
  }
}
impl TlvDecode for MainStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TargetPositionEnum {
  ///Move to a fully closed state
  MoveToFullyClosed = 0x0000,
  ///Move to a fully open state
  MoveToFullyOpen = 0x0001,
  ///Move to the Pedestrian position
  MoveToPedestrianPosition = 0x0002,
  ///Move to the Ventilation position
  MoveToVentilationPosition = 0x0003,
  ///Move to the Signature position
  MoveToSignaturePosition = 0x0004,
}
impl AttrType for TargetPositionEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "MoveToFullyClosed"),
  (0x0001, "MoveToFullyOpen"),
  (0x0002, "MoveToPedestrianPosition"),
  (0x0003, "MoveToVentilationPosition"),
  (0x0004, "MoveToSignaturePosition"),
  ]);
}
impl TlvPut for TargetPositionEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TargetPositionEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::MoveToFullyClosed),
      0x0001 => Ok(Self::MoveToFullyOpen),
      0x0002 => Ok(Self::MoveToPedestrianPosition),
      0x0003 => Ok(Self::MoveToVentilationPosition),
      0x0004 => Ok(Self::MoveToSignaturePosition),
      _ => Err(())
    }
  }
}
impl TlvDecode for TargetPositionEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct LatchControlModesBitmap: u32 {
    ///Remote latching capability
    const RemoteLatching = 1 << 0;
    ///Remote unlatching capability
    const RemoteUnlatching = 1 << 1;
  }
}
impl AttrType for LatchControlModesBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "RemoteLatching"),
    (1, "RemoteUnlatching"),
  ]);
}
impl TlvPut for LatchControlModesBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for LatchControlModesBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for LatchControlModesBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct OverallCurrentStateStruct {
  pub position: CurrentPositionEnum,
  pub latch: bool,
  pub speed: ThreeLevelAutoEnum,
  pub secure_state: bool,
}
impl AttrType for OverallCurrentStateStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Position", attr_ty: <CurrentPositionEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Latch", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Speed", attr_ty: <ThreeLevelAutoEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "SecureState", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for OverallCurrentStateStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.position.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.latch.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.speed.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.secure_state.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for OverallCurrentStateStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut position: Option<CurrentPositionEnum> = None;
    let mut latch: Option<bool> = None;
    let mut speed: Option<ThreeLevelAutoEnum> = None;
    let mut secure_state: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { position = Some(<CurrentPositionEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { latch = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { speed = Some(<ThreeLevelAutoEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { secure_state = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      position: position.ok_or(TlvDecodeErr::MissingField("Position"))?,
      latch: latch.ok_or(TlvDecodeErr::MissingField("Latch"))?,
      speed: speed.ok_or(TlvDecodeErr::MissingField("Speed"))?,
      secure_state: secure_state.ok_or(TlvDecodeErr::MissingField("SecureState"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OverallTargetStateStruct {
  pub position: TargetPositionEnum,
  pub latch: bool,
  pub speed: ThreeLevelAutoEnum,
}
impl AttrType for OverallTargetStateStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Position", attr_ty: <TargetPositionEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Latch", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Speed", attr_ty: <ThreeLevelAutoEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for OverallTargetStateStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.position.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.latch.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.speed.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for OverallTargetStateStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut position: Option<TargetPositionEnum> = None;
    let mut latch: Option<bool> = None;
    let mut speed: Option<ThreeLevelAutoEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { position = Some(<TargetPositionEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { latch = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { speed = Some(<ThreeLevelAutoEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      position: position.ok_or(TlvDecodeErr::MissingField("Position"))?,
      latch: latch.ok_or(TlvDecodeErr::MissingField("Latch"))?,
      speed: speed.ok_or(TlvDecodeErr::MissingField("Speed"))?,
    })
  }
}

}
