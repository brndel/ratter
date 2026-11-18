#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const DEVICE_ENERGY_MANAGEMENT: ClusterDef = ClusterDef {
  id: 0x0098,
  name: "Device Energy Management",
  features: &[
    FeatureDef { bit: 0x0000, code: "PA", name: "PowerAdjustment", summary: "Allows an EMS to make a temporary power adjustment (within the limits offered by the ESA)." },
    FeatureDef { bit: 0x0001, code: "PFR", name: "PowerForecastReporting", summary: "Allows an ESA to advertise its indicative future power consumption vs time." },
    FeatureDef { bit: 0x0002, code: "SFR", name: "StateForecastReporting", summary: "Allows an ESA to advertise its indicative future state vs time." },
    FeatureDef { bit: 0x0003, code: "STA", name: "StartTimeAdjustment", summary: "Allows an EMS to delay an ESA's planned operation." },
    FeatureDef { bit: 0x0004, code: "PAU", name: "Pausable", summary: "Allows an EMS to pause an ESA's planned operation." },
    FeatureDef { bit: 0x0005, code: "FA", name: "ForecastAdjustment", summary: "Allows an EMS to adjust an ESA's planned operation." },
    FeatureDef { bit: 0x0006, code: "CON", name: "ConstraintBasedAdjustment", summary: "Allows an EMS to request constraints to an ESA's planned operation." },
  ],
  attributes: &[
    attr::ESAType::ATTR,
    attr::ESACanGenerate::ATTR,
    attr::ESAState::ATTR,
    attr::AbsMinPower::ATTR,
    attr::AbsMaxPower::ATTR,
    attr::PowerAdjustmentCapability::ATTR,
    attr::Forecast::ATTR,
    attr::OptOutState::ATTR,
  ],
  events: &[
    event::PowerAdjustStart::EVENT,
    event::PowerAdjustEnd::EVENT,
    event::Paused::EVENT,
    event::Resumed::EVENT,
  ],
  commands: &[
    command::PowerAdjustRequest::COMMAND,
    command::CancelPowerAdjustRequest::COMMAND,
    command::StartTimeAdjustRequest::COMMAND,
    command::PauseRequest::COMMAND,
    command::ResumeRequest::COMMAND,
    command::ModifyForecastRequest::COMMAND,
    command::RequestConstraintBasedForecast::COMMAND,
    command::CancelRequest::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct ESAType;

  impl Attr for ESAType {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "ESAType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::ESATypeEnum;
  }


  pub struct ESACanGenerate;

  impl Attr for ESACanGenerate {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "ESACanGenerate",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


  pub struct ESAState;

  impl Attr for ESAState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "ESAState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::ESAStateEnum;
  }


  pub struct AbsMinPower;

  impl Attr for AbsMinPower {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "AbsMinPower",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = PowerMw;
  }


  pub struct AbsMaxPower;

  impl Attr for AbsMaxPower {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "AbsMaxPower",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = PowerMw;
  }


  pub struct PowerAdjustmentCapability;

  impl Attr for PowerAdjustmentCapability {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "PowerAdjustmentCapability",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<ty::PowerAdjustCapabilityStruct>;
  }


  pub struct Forecast;

  impl Attr for Forecast {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "Forecast",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::ForecastStruct>;
  }


  pub struct OptOutState;

  impl Attr for OptOutState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "OptOutState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::OptOutStateEnum;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct PowerAdjustStart {
}

impl Event for PowerAdjustStart {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "PowerAdjustStart",
    conformance: Conformance::Feature(0),
    fields: &[
    ],
  };
}

impl TlvDecode for PowerAdjustStart {
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
pub struct PowerAdjustEnd {
  pub cause: ty::CauseEnum,
  pub duration: TypeUnsupported /* ElapsedS */,
  pub energy_use: EnergyMwh,
}

impl Event for PowerAdjustEnd {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "PowerAdjustEnd",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "Cause", attr_ty: <ty::CauseEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Duration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "EnergyUse", attr_ty: <EnergyMwh>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for PowerAdjustEnd {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut cause: Option<ty::CauseEnum> = None;
    let mut energy_use: Option<EnergyMwh> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { cause = Some(<ty::CauseEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { energy_use = Some(<EnergyMwh>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      cause: cause.ok_or(TlvDecodeErr::MissingField("Cause"))?,
      duration: TypeUnsupported,
      energy_use: energy_use.ok_or(TlvDecodeErr::MissingField("EnergyUse"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Paused {
}

impl Event for Paused {
  const EVENT: EventDef = EventDef {
    id: 0x0002,
    name: "Paused",
    conformance: Conformance::Feature(4),
    fields: &[
    ],
  };
}

impl TlvDecode for Paused {
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
pub struct Resumed {
  pub cause: ty::CauseEnum,
}

impl Event for Resumed {
  const EVENT: EventDef = EventDef {
    id: 0x0003,
    name: "Resumed",
    conformance: Conformance::Feature(4),
    fields: &[
      AttrDef { id: 0x0000, name: "Cause", attr_ty: <ty::CauseEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for Resumed {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut cause: Option<ty::CauseEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { cause = Some(<ty::CauseEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      cause: cause.ok_or(TlvDecodeErr::MissingField("Cause"))?,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct PowerAdjustRequest {
  pub power: PowerMw,
  pub duration: TypeUnsupported /* ElapsedS */,
  pub cause: ty::AdjustmentCauseEnum,
}
impl Command for PowerAdjustRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "PowerAdjustRequest",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "Power", attr_ty: <PowerMw>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Duration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Cause", attr_ty: <ty::AdjustmentCauseEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for PowerAdjustRequest {
  type Response = BasicAccResponse;
}
impl TlvEncode for PowerAdjustRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.power.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.duration.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.cause.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CancelPowerAdjustRequest {
}
impl Command for CancelPowerAdjustRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "CancelPowerAdjustRequest",
    conformance: Conformance::Feature(0),
    fields: &[
    ],
  };
}
impl CommandInvoke for CancelPowerAdjustRequest {
  type Response = BasicAccResponse;
}
impl TlvEncode for CancelPowerAdjustRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StartTimeAdjustRequest {
  pub requested_start_time: TypeUnsupported /* EpochS */,
  pub cause: ty::AdjustmentCauseEnum,
}
impl Command for StartTimeAdjustRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "StartTimeAdjustRequest",
    conformance: Conformance::Feature(3),
    fields: &[
      AttrDef { id: 0x0000, name: "RequestedStartTime", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Cause", attr_ty: <ty::AdjustmentCauseEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for StartTimeAdjustRequest {
  type Response = BasicAccResponse;
}
impl TlvEncode for StartTimeAdjustRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.requested_start_time.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.cause.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PauseRequest {
  pub duration: TypeUnsupported /* ElapsedS */,
  pub cause: ty::AdjustmentCauseEnum,
}
impl Command for PauseRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "PauseRequest",
    conformance: Conformance::Feature(4),
    fields: &[
      AttrDef { id: 0x0000, name: "Duration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Cause", attr_ty: <ty::AdjustmentCauseEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for PauseRequest {
  type Response = BasicAccResponse;
}
impl TlvEncode for PauseRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.duration.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.cause.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResumeRequest {
}
impl Command for ResumeRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "ResumeRequest",
    conformance: Conformance::Feature(4),
    fields: &[
    ],
  };
}
impl CommandInvoke for ResumeRequest {
  type Response = BasicAccResponse;
}
impl TlvEncode for ResumeRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModifyForecastRequest {
  pub forecast_id: u32,
  pub slot_adjustments: Vec<ty::SlotAdjustmentStruct>,
  pub cause: ty::AdjustmentCauseEnum,
}
impl Command for ModifyForecastRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "ModifyForecastRequest",
    conformance: Conformance::Feature(5),
    fields: &[
      AttrDef { id: 0x0000, name: "ForecastID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "SlotAdjustments", attr_ty: <Vec<ty::SlotAdjustmentStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Cause", attr_ty: <ty::AdjustmentCauseEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ModifyForecastRequest {
  type Response = BasicAccResponse;
}
impl TlvEncode for ModifyForecastRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.forecast_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.slot_adjustments.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.cause.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RequestConstraintBasedForecast {
  pub constraints: Vec<ty::ConstraintsStruct>,
  pub cause: ty::AdjustmentCauseEnum,
}
impl Command for RequestConstraintBasedForecast {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "RequestConstraintBasedForecast",
    conformance: Conformance::Feature(6),
    fields: &[
      AttrDef { id: 0x0000, name: "Constraints", attr_ty: <Vec<ty::ConstraintsStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Cause", attr_ty: <ty::AdjustmentCauseEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RequestConstraintBasedForecast {
  type Response = BasicAccResponse;
}
impl TlvEncode for RequestConstraintBasedForecast {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.constraints.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.cause.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CancelRequest {
}
impl Command for CancelRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "CancelRequest",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for CancelRequest {
  type Response = BasicAccResponse;
}
impl TlvEncode for CancelRequest {
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
pub enum AdjustmentCauseEnum {
  ///The adjustment is to optimize the local energy usage
  LocalOptimization = 0x0000,
  ///The adjustment is to optimize the grid energy usage
  GridOptimization = 0x0001,
}
impl AttrType for AdjustmentCauseEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "LocalOptimization"),
  (0x0001, "GridOptimization"),
  ]);
}
impl TlvPut for AdjustmentCauseEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for AdjustmentCauseEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::LocalOptimization),
      0x0001 => Ok(Self::GridOptimization),
      _ => Err(())
    }
  }
}
impl TlvDecode for AdjustmentCauseEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum CauseEnum {
  ///The ESA completed the adjustment as requested
  NormalCompletion = 0x0000,
  ///The ESA was set to offline
  Offline = 0x0001,
  ///The ESA has developed a fault could not complete the adjustment
  Fault = 0x0002,
  ///The user has disabled the ESA's flexibility capability
  UserOptOut = 0x0003,
  ///The adjustment was cancelled by a client
  Cancelled = 0x0004,
}
impl AttrType for CauseEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "NormalCompletion"),
  (0x0001, "Offline"),
  (0x0002, "Fault"),
  (0x0003, "UserOptOut"),
  (0x0004, "Cancelled"),
  ]);
}
impl TlvPut for CauseEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for CauseEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::NormalCompletion),
      0x0001 => Ok(Self::Offline),
      0x0002 => Ok(Self::Fault),
      0x0003 => Ok(Self::UserOptOut),
      0x0004 => Ok(Self::Cancelled),
      _ => Err(())
    }
  }
}
impl TlvDecode for CauseEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum CostTypeEnum {
  ///Financial cost
  Financial = 0x0000,
  ///Grid CO2e grams cost
  GHGEmissions = 0x0001,
  ///Consumer comfort impact cost
  Comfort = 0x0002,
  ///Temperature impact cost
  Temperature = 0x0003,
}
impl AttrType for CostTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Financial"),
  (0x0001, "GHGEmissions"),
  (0x0002, "Comfort"),
  (0x0003, "Temperature"),
  ]);
}
impl TlvPut for CostTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for CostTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Financial),
      0x0001 => Ok(Self::GHGEmissions),
      0x0002 => Ok(Self::Comfort),
      0x0003 => Ok(Self::Temperature),
      _ => Err(())
    }
  }
}
impl TlvDecode for CostTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ESAStateEnum {
  ///The ESA is not available to the EMS (e.g. start-up, maintenance mode)
  Offline = 0x0000,
  ///The ESA is working normally and can be controlled by the EMS
  Online = 0x0001,
  ///The ESA has developed a fault and cannot provide service
  Fault = 0x0002,
  ///The ESA is in the middle of a power adjustment event
  PowerAdjustActive = 0x0003,
  ///The ESA is currently paused by a client using the PauseRequest command
  Paused = 0x0004,
}
impl AttrType for ESAStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Offline"),
  (0x0001, "Online"),
  (0x0002, "Fault"),
  (0x0003, "PowerAdjustActive"),
  (0x0004, "Paused"),
  ]);
}
impl TlvPut for ESAStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ESAStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Offline),
      0x0001 => Ok(Self::Online),
      0x0002 => Ok(Self::Fault),
      0x0003 => Ok(Self::PowerAdjustActive),
      0x0004 => Ok(Self::Paused),
      _ => Err(())
    }
  }
}
impl TlvDecode for ESAStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ESATypeEnum {
  ///EV Supply Equipment
  EVSE = 0x0000,
  ///Space heating appliance
  SpaceHeating = 0x0001,
  ///Water heating appliance
  WaterHeating = 0x0002,
  ///Space cooling appliance
  SpaceCooling = 0x0003,
  ///Space heating and cooling appliance
  SpaceHeatingCooling = 0x0004,
  ///Battery Electric Storage System
  BatteryStorage = 0x0005,
  ///Solar PV inverter
  SolarPV = 0x0006,
  ///Fridge / Freezer
  FridgeFreezer = 0x0007,
  ///Washing Machine
  WashingMachine = 0x0008,
  ///Dishwasher
  Dishwasher = 0x0009,
  ///Cooking appliance
  Cooking = 0x000a,
  ///Home water pump (e.g. drinking well)
  HomeWaterPump = 0x000b,
  ///Irrigation water pump
  IrrigationWaterPump = 0x000c,
  ///Pool pump
  PoolPump = 0x000d,
  ///Other appliance type
  Other = 0x00ff,
}
impl AttrType for ESATypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "EVSE"),
  (0x0001, "SpaceHeating"),
  (0x0002, "WaterHeating"),
  (0x0003, "SpaceCooling"),
  (0x0004, "SpaceHeatingCooling"),
  (0x0005, "BatteryStorage"),
  (0x0006, "SolarPV"),
  (0x0007, "FridgeFreezer"),
  (0x0008, "WashingMachine"),
  (0x0009, "Dishwasher"),
  (0x000a, "Cooking"),
  (0x000b, "HomeWaterPump"),
  (0x000c, "IrrigationWaterPump"),
  (0x000d, "PoolPump"),
  (0x00ff, "Other"),
  ]);
}
impl TlvPut for ESATypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ESATypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::EVSE),
      0x0001 => Ok(Self::SpaceHeating),
      0x0002 => Ok(Self::WaterHeating),
      0x0003 => Ok(Self::SpaceCooling),
      0x0004 => Ok(Self::SpaceHeatingCooling),
      0x0005 => Ok(Self::BatteryStorage),
      0x0006 => Ok(Self::SolarPV),
      0x0007 => Ok(Self::FridgeFreezer),
      0x0008 => Ok(Self::WashingMachine),
      0x0009 => Ok(Self::Dishwasher),
      0x000a => Ok(Self::Cooking),
      0x000b => Ok(Self::HomeWaterPump),
      0x000c => Ok(Self::IrrigationWaterPump),
      0x000d => Ok(Self::PoolPump),
      0x00ff => Ok(Self::Other),
      _ => Err(())
    }
  }
}
impl TlvDecode for ESATypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ForecastUpdateReasonEnum {
  ///The update was due to internal ESA device optimization
  InternalOptimization = 0x0000,
  ///The update was due to local EMS optimization
  LocalOptimization = 0x0001,
  ///The update was due to grid optimization
  GridOptimization = 0x0002,
}
impl AttrType for ForecastUpdateReasonEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "InternalOptimization"),
  (0x0001, "LocalOptimization"),
  (0x0002, "GridOptimization"),
  ]);
}
impl TlvPut for ForecastUpdateReasonEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ForecastUpdateReasonEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::InternalOptimization),
      0x0001 => Ok(Self::LocalOptimization),
      0x0002 => Ok(Self::GridOptimization),
      _ => Err(())
    }
  }
}
impl TlvDecode for ForecastUpdateReasonEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum OptOutStateEnum {
  ///The user has not opted out of either local or grid optimizations
  NoOptOut = 0x0000,
  ///The user has opted out of local EMS optimizations only
  LocalOptOut = 0x0001,
  ///The user has opted out of grid EMS optimizations only
  GridOptOut = 0x0002,
  ///The user has opted out of all external optimizations
  OptOut = 0x0003,
}
impl AttrType for OptOutStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "NoOptOut"),
  (0x0001, "LocalOptOut"),
  (0x0002, "GridOptOut"),
  (0x0003, "OptOut"),
  ]);
}
impl TlvPut for OptOutStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for OptOutStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::NoOptOut),
      0x0001 => Ok(Self::LocalOptOut),
      0x0002 => Ok(Self::GridOptOut),
      0x0003 => Ok(Self::OptOut),
      _ => Err(())
    }
  }
}
impl TlvDecode for OptOutStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum PowerAdjustReasonEnum {
  ///There is no Power Adjustment active
  NoAdjustment = 0x0000,
  ///There is PowerAdjustment active due to local EMS optimization
  LocalOptimizationAdjustment = 0x0001,
  ///There is PowerAdjustment active due to grid optimization
  GridOptimizationAdjustment = 0x0002,
}
impl AttrType for PowerAdjustReasonEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "NoAdjustment"),
  (0x0001, "LocalOptimizationAdjustment"),
  (0x0002, "GridOptimizationAdjustment"),
  ]);
}
impl TlvPut for PowerAdjustReasonEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for PowerAdjustReasonEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::NoAdjustment),
      0x0001 => Ok(Self::LocalOptimizationAdjustment),
      0x0002 => Ok(Self::GridOptimizationAdjustment),
      _ => Err(())
    }
  }
}
impl TlvDecode for PowerAdjustReasonEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ConstraintsStruct {
  pub start_time: TypeUnsupported /* EpochS */,
  pub duration: TypeUnsupported /* ElapsedS */,
  pub nominal_power: PowerMw,
  pub maximum_energy: EnergyMwh,
  pub load_control: i8,
}
impl AttrType for ConstraintsStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "StartTime", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Duration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "NominalPower", attr_ty: <PowerMw>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "MaximumEnergy", attr_ty: <EnergyMwh>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "LoadControl", attr_ty: <i8>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ConstraintsStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.start_time.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.duration.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.nominal_power.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.maximum_energy.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.load_control.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ConstraintsStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut nominal_power: Option<PowerMw> = None;
    let mut maximum_energy: Option<EnergyMwh> = None;
    let mut load_control: Option<i8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { nominal_power = Some(<PowerMw>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { maximum_energy = Some(<EnergyMwh>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { load_control = Some(<i8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      start_time: TypeUnsupported,
      duration: TypeUnsupported,
      nominal_power: nominal_power.ok_or(TlvDecodeErr::MissingField("NominalPower"))?,
      maximum_energy: maximum_energy.ok_or(TlvDecodeErr::MissingField("MaximumEnergy"))?,
      load_control: load_control.ok_or(TlvDecodeErr::MissingField("LoadControl"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CostStruct {
  pub cost_type: CostTypeEnum,
  pub value: i32,
  pub decimal_points: u8,
  pub currency: u16,
}
impl AttrType for CostStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "CostType", attr_ty: <CostTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Value", attr_ty: <i32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "DecimalPoints", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "Currency", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for CostStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.cost_type.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.value.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.decimal_points.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.currency.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for CostStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut cost_type: Option<CostTypeEnum> = None;
    let mut value: Option<i32> = None;
    let mut decimal_points: Option<u8> = None;
    let mut currency: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { cost_type = Some(<CostTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { value = Some(<i32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { decimal_points = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { currency = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      cost_type: cost_type.ok_or(TlvDecodeErr::MissingField("CostType"))?,
      value: value.ok_or(TlvDecodeErr::MissingField("Value"))?,
      decimal_points: decimal_points.ok_or(TlvDecodeErr::MissingField("DecimalPoints"))?,
      currency: currency.ok_or(TlvDecodeErr::MissingField("Currency"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForecastStruct {
  pub forecast_id: u32,
  pub active_slot_number: u16,
  pub start_time: TypeUnsupported /* EpochS */,
  pub end_time: TypeUnsupported /* EpochS */,
  pub earliest_start_time: TypeUnsupported /* EpochS */,
  pub latest_end_time: TypeUnsupported /* EpochS */,
  pub is_pausable: bool,
  pub slots: Vec<SlotStruct>,
  pub forecast_update_reason: ForecastUpdateReasonEnum,
}
impl AttrType for ForecastStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ForecastID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ActiveSlotNumber", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "StartTime", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "EndTime", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "EarliestStartTime", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "LatestEndTime", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "IsPausable", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "Slots", attr_ty: <Vec<SlotStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0008, name: "ForecastUpdateReason", attr_ty: <ForecastUpdateReasonEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ForecastStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.forecast_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.active_slot_number.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.start_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.end_time.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.earliest_start_time.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.latest_end_time.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.is_pausable.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.slots.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.forecast_update_reason.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ForecastStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut forecast_id: Option<u32> = None;
    let mut active_slot_number: Option<u16> = None;
    let mut is_pausable: Option<bool> = None;
    let mut slots: Option<Vec<SlotStruct>> = None;
    let mut forecast_update_reason: Option<ForecastUpdateReasonEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { forecast_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { active_slot_number = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { is_pausable = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0007), kind: _ }) => { slots = Some(<Vec<SlotStruct>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0008), value: _ }) => { forecast_update_reason = Some(<ForecastUpdateReasonEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      forecast_id: forecast_id.ok_or(TlvDecodeErr::MissingField("ForecastID"))?,
      active_slot_number: active_slot_number.ok_or(TlvDecodeErr::MissingField("ActiveSlotNumber"))?,
      start_time: TypeUnsupported,
      end_time: TypeUnsupported,
      earliest_start_time: TypeUnsupported,
      latest_end_time: TypeUnsupported,
      is_pausable: is_pausable.ok_or(TlvDecodeErr::MissingField("IsPausable"))?,
      slots: slots.ok_or(TlvDecodeErr::MissingField("Slots"))?,
      forecast_update_reason: forecast_update_reason.ok_or(TlvDecodeErr::MissingField("ForecastUpdateReason"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PowerAdjustCapabilityStruct {
  pub power_adjust_capability: Vec<PowerAdjustStruct>,
  pub cause: PowerAdjustReasonEnum,
}
impl AttrType for PowerAdjustCapabilityStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "PowerAdjustCapability", attr_ty: <Vec<PowerAdjustStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Cause", attr_ty: <PowerAdjustReasonEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for PowerAdjustCapabilityStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.power_adjust_capability.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.cause.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for PowerAdjustCapabilityStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut power_adjust_capability: Option<Vec<PowerAdjustStruct>> = None;
    let mut cause: Option<PowerAdjustReasonEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { power_adjust_capability = Some(<Vec<PowerAdjustStruct>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { cause = Some(<PowerAdjustReasonEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      power_adjust_capability: power_adjust_capability.ok_or(TlvDecodeErr::MissingField("PowerAdjustCapability"))?,
      cause: cause.ok_or(TlvDecodeErr::MissingField("Cause"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PowerAdjustStruct {
  pub min_power: PowerMw,
  pub max_power: PowerMw,
  pub min_duration: TypeUnsupported /* ElapsedS */,
  pub max_duration: TypeUnsupported /* ElapsedS */,
}
impl AttrType for PowerAdjustStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "MinPower", attr_ty: <PowerMw>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "MaxPower", attr_ty: <PowerMw>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "MinDuration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "MaxDuration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for PowerAdjustStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.min_power.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.max_power.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.min_duration.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.max_duration.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for PowerAdjustStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut min_power: Option<PowerMw> = None;
    let mut max_power: Option<PowerMw> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { min_power = Some(<PowerMw>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { max_power = Some(<PowerMw>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      min_power: min_power.ok_or(TlvDecodeErr::MissingField("MinPower"))?,
      max_power: max_power.ok_or(TlvDecodeErr::MissingField("MaxPower"))?,
      min_duration: TypeUnsupported,
      max_duration: TypeUnsupported,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SlotAdjustmentStruct {
  pub slot_index: u8,
  pub nominal_power: PowerMw,
  pub duration: TypeUnsupported /* ElapsedS */,
}
impl AttrType for SlotAdjustmentStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "SlotIndex", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "NominalPower", attr_ty: <PowerMw>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Duration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for SlotAdjustmentStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.slot_index.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.nominal_power.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.duration.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for SlotAdjustmentStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut slot_index: Option<u8> = None;
    let mut nominal_power: Option<PowerMw> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { slot_index = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { nominal_power = Some(<PowerMw>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      slot_index: slot_index.ok_or(TlvDecodeErr::MissingField("SlotIndex"))?,
      nominal_power: nominal_power.ok_or(TlvDecodeErr::MissingField("NominalPower"))?,
      duration: TypeUnsupported,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SlotStruct {
  pub min_duration: TypeUnsupported /* ElapsedS */,
  pub max_duration: TypeUnsupported /* ElapsedS */,
  pub default_duration: TypeUnsupported /* ElapsedS */,
  pub elapsed_slot_time: TypeUnsupported /* ElapsedS */,
  pub remaining_slot_time: TypeUnsupported /* ElapsedS */,
  pub slot_is_pausable: bool,
  pub min_pause_duration: TypeUnsupported /* ElapsedS */,
  pub max_pause_duration: TypeUnsupported /* ElapsedS */,
  pub manufacturer_esa_state: u16,
  pub nominal_power: PowerMw,
  pub min_power: PowerMw,
  pub max_power: PowerMw,
  pub nominal_energy: EnergyMwh,
  pub costs: Vec<CostStruct>,
  pub min_power_adjustment: PowerMw,
  pub max_power_adjustment: PowerMw,
  pub min_duration_adjustment: TypeUnsupported /* ElapsedS */,
  pub max_duration_adjustment: TypeUnsupported /* ElapsedS */,
}
impl AttrType for SlotStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "MinDuration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "MaxDuration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "DefaultDuration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "ElapsedSlotTime", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "RemainingSlotTime", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "SlotIsPausable", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "MinPauseDuration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "MaxPauseDuration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0008, name: "ManufacturerESAState", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0009, name: "NominalPower", attr_ty: <PowerMw>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000a, name: "MinPower", attr_ty: <PowerMw>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000b, name: "MaxPower", attr_ty: <PowerMw>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000c, name: "NominalEnergy", attr_ty: <EnergyMwh>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000d, name: "Costs", attr_ty: <Vec<CostStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000e, name: "MinPowerAdjustment", attr_ty: <PowerMw>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000f, name: "MaxPowerAdjustment", attr_ty: <PowerMw>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0010, name: "MinDurationAdjustment", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0011, name: "MaxDurationAdjustment", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for SlotStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.min_duration.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.max_duration.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.default_duration.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.elapsed_slot_time.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.remaining_slot_time.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.slot_is_pausable.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.min_pause_duration.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.max_pause_duration.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.manufacturer_esa_state.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    self.nominal_power.tlv_put(matter_codec::Tag::Context(0x0009), writer);
    self.min_power.tlv_put(matter_codec::Tag::Context(0x000a), writer);
    self.max_power.tlv_put(matter_codec::Tag::Context(0x000b), writer);
    self.nominal_energy.tlv_put(matter_codec::Tag::Context(0x000c), writer);
    self.costs.tlv_put(matter_codec::Tag::Context(0x000d), writer);
    self.min_power_adjustment.tlv_put(matter_codec::Tag::Context(0x000e), writer);
    self.max_power_adjustment.tlv_put(matter_codec::Tag::Context(0x000f), writer);
    self.min_duration_adjustment.tlv_put(matter_codec::Tag::Context(0x0010), writer);
    self.max_duration_adjustment.tlv_put(matter_codec::Tag::Context(0x0011), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for SlotStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut slot_is_pausable: Option<bool> = None;
    let mut manufacturer_esa_state: Option<u16> = None;
    let mut nominal_power: Option<PowerMw> = None;
    let mut min_power: Option<PowerMw> = None;
    let mut max_power: Option<PowerMw> = None;
    let mut nominal_energy: Option<EnergyMwh> = None;
    let mut costs: Option<Vec<CostStruct>> = None;
    let mut min_power_adjustment: Option<PowerMw> = None;
    let mut max_power_adjustment: Option<PowerMw> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { slot_is_pausable = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0008), value: _ }) => { manufacturer_esa_state = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0009), value: _ }) => { nominal_power = Some(<PowerMw>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000a), value: _ }) => { min_power = Some(<PowerMw>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000b), value: _ }) => { max_power = Some(<PowerMw>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000c), value: _ }) => { nominal_energy = Some(<EnergyMwh>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x000d), kind: _ }) => { costs = Some(<Vec<CostStruct>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000e), value: _ }) => { min_power_adjustment = Some(<PowerMw>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000f), value: _ }) => { max_power_adjustment = Some(<PowerMw>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      min_duration: TypeUnsupported,
      max_duration: TypeUnsupported,
      default_duration: TypeUnsupported,
      elapsed_slot_time: TypeUnsupported,
      remaining_slot_time: TypeUnsupported,
      slot_is_pausable: slot_is_pausable.ok_or(TlvDecodeErr::MissingField("SlotIsPausable"))?,
      min_pause_duration: TypeUnsupported,
      max_pause_duration: TypeUnsupported,
      manufacturer_esa_state: manufacturer_esa_state.ok_or(TlvDecodeErr::MissingField("ManufacturerESAState"))?,
      nominal_power: nominal_power.ok_or(TlvDecodeErr::MissingField("NominalPower"))?,
      min_power: min_power.ok_or(TlvDecodeErr::MissingField("MinPower"))?,
      max_power: max_power.ok_or(TlvDecodeErr::MissingField("MaxPower"))?,
      nominal_energy: nominal_energy.ok_or(TlvDecodeErr::MissingField("NominalEnergy"))?,
      costs: costs.ok_or(TlvDecodeErr::MissingField("Costs"))?,
      min_power_adjustment: min_power_adjustment.ok_or(TlvDecodeErr::MissingField("MinPowerAdjustment"))?,
      max_power_adjustment: max_power_adjustment.ok_or(TlvDecodeErr::MissingField("MaxPowerAdjustment"))?,
      min_duration_adjustment: TypeUnsupported,
      max_duration_adjustment: TypeUnsupported,
    })
  }
}

}
