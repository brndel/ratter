#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const ENERGY_EVSE: ClusterDef = ClusterDef {
  id: 0x0099,
  name: "Energy EVSE",
  features: &[
    FeatureDef { bit: 0x0000, code: "PREF", name: "ChargingPreferences", summary: "EVSE supports storing user charging preferences" },
    FeatureDef { bit: 0x0001, code: "SOC", name: "SoCReporting", summary: "EVSE supports reporting of vehicle State of Charge (SoC)" },
    FeatureDef { bit: 0x0002, code: "PNC", name: "PlugAndCharge", summary: "EVSE supports PLC to support Plug and Charge" },
    FeatureDef { bit: 0x0003, code: "RFID", name: "RFID", summary: "EVSE is fitted with an RFID reader" },
    FeatureDef { bit: 0x0004, code: "V2X", name: "V2X", summary: "EVSE supports bi-directional charging / discharging" },
  ],
  attributes: &[
    attr::State::ATTR,
    attr::SupplyState::ATTR,
    attr::FaultState::ATTR,
    attr::ChargingEnabledUntil::ATTR,
    attr::DischargingEnabledUntil::ATTR,
    attr::CircuitCapacity::ATTR,
    attr::MinimumChargeCurrent::ATTR,
    attr::MaximumChargeCurrent::ATTR,
    attr::MaximumDischargeCurrent::ATTR,
    attr::UserMaximumChargeCurrent::ATTR,
    attr::RandomizationDelayWindow::ATTR,
    attr::NextChargeStartTime::ATTR,
    attr::NextChargeTargetTime::ATTR,
    attr::NextChargeRequiredEnergy::ATTR,
    attr::NextChargeTargetSoC::ATTR,
    attr::ApproximateEVEfficiency::ATTR,
    attr::StateOfCharge::ATTR,
    attr::BatteryCapacity::ATTR,
    attr::VehicleID::ATTR,
    attr::SessionID::ATTR,
    attr::SessionDuration::ATTR,
    attr::SessionEnergyCharged::ATTR,
    attr::SessionEnergyDischarged::ATTR,
  ],
  events: &[
    event::EVConnected::EVENT,
    event::EVNotDetected::EVENT,
    event::EnergyTransferStarted::EVENT,
    event::EnergyTransferStopped::EVENT,
    event::Fault::EVENT,
    event::RFID::EVENT,
  ],
  commands: &[
    command::Disable::COMMAND,
    command::EnableCharging::COMMAND,
    command::EnableDischarging::COMMAND,
    command::StartDiagnostics::COMMAND,
    command::SetTargets::COMMAND,
    command::GetTargets::COMMAND,
    command::ClearTargets::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct State;

  impl Attr for State {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "State",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::StateEnum>;
  }


  pub struct SupplyState;

  impl Attr for SupplyState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "SupplyState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::SupplyStateEnum;
  }


  pub struct FaultState;

  impl Attr for FaultState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "FaultState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::FaultStateEnum;
  }


  pub struct ChargingEnabledUntil;

  impl Attr for ChargingEnabledUntil {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "ChargingEnabledUntil",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* EpochS */>;
  }


  pub struct DischargingEnabledUntil;

  impl Attr for DischargingEnabledUntil {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "DischargingEnabledUntil",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = Nullable<TypeUnsupported /* EpochS */>;
  }


  pub struct CircuitCapacity;

  impl Attr for CircuitCapacity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "CircuitCapacity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = AmperageMa;
  }


  pub struct MinimumChargeCurrent;

  impl Attr for MinimumChargeCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "MinimumChargeCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = AmperageMa;
  }


  pub struct MaximumChargeCurrent;

  impl Attr for MaximumChargeCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "MaximumChargeCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = AmperageMa;
  }


  pub struct MaximumDischargeCurrent;

  impl Attr for MaximumDischargeCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "MaximumDischargeCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = AmperageMa;
  }


  pub struct UserMaximumChargeCurrent;

  impl Attr for UserMaximumChargeCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "UserMaximumChargeCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = AmperageMa;
  }


  pub struct RandomizationDelayWindow;

  impl Attr for RandomizationDelayWindow {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "RandomizationDelayWindow",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = TypeUnsupported /* ElapsedS */;
  }


  pub struct NextChargeStartTime;

  impl Attr for NextChargeStartTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0023,
      name: "NextChargeStartTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<TypeUnsupported /* EpochS */>;
  }


  pub struct NextChargeTargetTime;

  impl Attr for NextChargeTargetTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0024,
      name: "NextChargeTargetTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<TypeUnsupported /* EpochS */>;
  }


  pub struct NextChargeRequiredEnergy;

  impl Attr for NextChargeRequiredEnergy {
    const ATTR: AttrDef = AttrDef {
      id: 0x0025,
      name: "NextChargeRequiredEnergy",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<EnergyMwh>;
  }


  pub struct NextChargeTargetSoC;

  impl Attr for NextChargeTargetSoC {
    const ATTR: AttrDef = AttrDef {
      id: 0x0026,
      name: "NextChargeTargetSoC",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<TypeUnsupported /* Percent */>;
  }


  pub struct ApproximateEVEfficiency;

  impl Attr for ApproximateEVEfficiency {
    const ATTR: AttrDef = AttrDef {
      id: 0x0027,
      name: "ApproximateEVEfficiency",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u16>;
  }


  pub struct StateOfCharge;

  impl Attr for StateOfCharge {
    const ATTR: AttrDef = AttrDef {
      id: 0x0030,
      name: "StateOfCharge",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Nullable<TypeUnsupported /* Percent */>;
  }


  pub struct BatteryCapacity;

  impl Attr for BatteryCapacity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0031,
      name: "BatteryCapacity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Nullable<EnergyMwh>;
  }


  pub struct VehicleID;

  impl Attr for VehicleID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0032,
      name: "VehicleID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = Nullable<String>;
  }


  pub struct SessionID;

  impl Attr for SessionID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0040,
      name: "SessionID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u32>;
  }


  pub struct SessionDuration;

  impl Attr for SessionDuration {
    const ATTR: AttrDef = AttrDef {
      id: 0x0041,
      name: "SessionDuration",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* ElapsedS */>;
  }


  pub struct SessionEnergyCharged;

  impl Attr for SessionEnergyCharged {
    const ATTR: AttrDef = AttrDef {
      id: 0x0042,
      name: "SessionEnergyCharged",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<EnergyMwh>;
  }


  pub struct SessionEnergyDischarged;

  impl Attr for SessionEnergyDischarged {
    const ATTR: AttrDef = AttrDef {
      id: 0x0043,
      name: "SessionEnergyDischarged",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = Nullable<EnergyMwh>;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct EVConnected {
  pub session_id: u32,
}

impl Event for EVConnected {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "EVConnected",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "SessionID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for EVConnected {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut session_id: Option<u32> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { session_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      session_id: session_id.ok_or(TlvDecodeErr::MissingField("SessionID"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct EVNotDetected {
  pub session_id: u32,
  pub state: ty::StateEnum,
  pub session_duration: TypeUnsupported /* ElapsedS */,
  pub session_energy_charged: EnergyMwh,
  pub session_energy_discharged: EnergyMwh,
}

impl Event for EVNotDetected {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "EVNotDetected",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "SessionID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "State", attr_ty: <ty::StateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "SessionDuration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "SessionEnergyCharged", attr_ty: <EnergyMwh>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "SessionEnergyDischarged", attr_ty: <EnergyMwh>::ATTR_TYPE, conformance: Conformance::Feature(4) },
    ],
  };
}

impl TlvDecode for EVNotDetected {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut session_id: Option<u32> = None;
    let mut state: Option<ty::StateEnum> = None;
    let mut session_energy_charged: Option<EnergyMwh> = None;
    let mut session_energy_discharged: Option<EnergyMwh> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { session_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { state = Some(<ty::StateEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { session_energy_charged = Some(<EnergyMwh>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { session_energy_discharged = Some(<EnergyMwh>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      session_id: session_id.ok_or(TlvDecodeErr::MissingField("SessionID"))?,
      state: state.ok_or(TlvDecodeErr::MissingField("State"))?,
      session_duration: TypeUnsupported,
      session_energy_charged: session_energy_charged.ok_or(TlvDecodeErr::MissingField("SessionEnergyCharged"))?,
      session_energy_discharged: session_energy_discharged.ok_or(TlvDecodeErr::MissingField("SessionEnergyDischarged"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct EnergyTransferStarted {
  pub session_id: u32,
  pub state: ty::StateEnum,
  pub maximum_current: AmperageMa,
  pub maximum_discharge_current: AmperageMa,
}

impl Event for EnergyTransferStarted {
  const EVENT: EventDef = EventDef {
    id: 0x0002,
    name: "EnergyTransferStarted",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "SessionID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "State", attr_ty: <ty::StateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "MaximumCurrent", attr_ty: <AmperageMa>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "MaximumDischargeCurrent", attr_ty: <AmperageMa>::ATTR_TYPE, conformance: Conformance::Feature(4) },
    ],
  };
}

impl TlvDecode for EnergyTransferStarted {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut session_id: Option<u32> = None;
    let mut state: Option<ty::StateEnum> = None;
    let mut maximum_current: Option<AmperageMa> = None;
    let mut maximum_discharge_current: Option<AmperageMa> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { session_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { state = Some(<ty::StateEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { maximum_current = Some(<AmperageMa>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { maximum_discharge_current = Some(<AmperageMa>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      session_id: session_id.ok_or(TlvDecodeErr::MissingField("SessionID"))?,
      state: state.ok_or(TlvDecodeErr::MissingField("State"))?,
      maximum_current: maximum_current.ok_or(TlvDecodeErr::MissingField("MaximumCurrent"))?,
      maximum_discharge_current: maximum_discharge_current.ok_or(TlvDecodeErr::MissingField("MaximumDischargeCurrent"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct EnergyTransferStopped {
  pub session_id: u32,
  pub state: ty::StateEnum,
  pub reason: ty::EnergyTransferStoppedReasonEnum,
  pub energy_transferred: EnergyMwh,
  pub energy_discharged: EnergyMwh,
}

impl Event for EnergyTransferStopped {
  const EVENT: EventDef = EventDef {
    id: 0x0003,
    name: "EnergyTransferStopped",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "SessionID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "State", attr_ty: <ty::StateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Reason", attr_ty: <ty::EnergyTransferStoppedReasonEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "EnergyTransferred", attr_ty: <EnergyMwh>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0005, name: "EnergyDischarged", attr_ty: <EnergyMwh>::ATTR_TYPE, conformance: Conformance::Feature(4) },
    ],
  };
}

impl TlvDecode for EnergyTransferStopped {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut session_id: Option<u32> = None;
    let mut state: Option<ty::StateEnum> = None;
    let mut reason: Option<ty::EnergyTransferStoppedReasonEnum> = None;
    let mut energy_transferred: Option<EnergyMwh> = None;
    let mut energy_discharged: Option<EnergyMwh> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { session_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { state = Some(<ty::StateEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { reason = Some(<ty::EnergyTransferStoppedReasonEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { energy_transferred = Some(<EnergyMwh>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { energy_discharged = Some(<EnergyMwh>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      session_id: session_id.ok_or(TlvDecodeErr::MissingField("SessionID"))?,
      state: state.ok_or(TlvDecodeErr::MissingField("State"))?,
      reason: reason.ok_or(TlvDecodeErr::MissingField("Reason"))?,
      energy_transferred: energy_transferred.ok_or(TlvDecodeErr::MissingField("EnergyTransferred"))?,
      energy_discharged: energy_discharged.ok_or(TlvDecodeErr::MissingField("EnergyDischarged"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct Fault {
  pub session_id: u32,
  pub state: ty::StateEnum,
  pub fault_state_previous_state: ty::FaultStateEnum,
  pub fault_state_current_state: ty::FaultStateEnum,
}

impl Event for Fault {
  const EVENT: EventDef = EventDef {
    id: 0x0004,
    name: "Fault",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "SessionID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "State", attr_ty: <ty::StateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "FaultStatePreviousState", attr_ty: <ty::FaultStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "FaultStateCurrentState", attr_ty: <ty::FaultStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for Fault {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut session_id: Option<u32> = None;
    let mut state: Option<ty::StateEnum> = None;
    let mut fault_state_previous_state: Option<ty::FaultStateEnum> = None;
    let mut fault_state_current_state: Option<ty::FaultStateEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { session_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { state = Some(<ty::StateEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { fault_state_previous_state = Some(<ty::FaultStateEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { fault_state_current_state = Some(<ty::FaultStateEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      session_id: session_id.ok_or(TlvDecodeErr::MissingField("SessionID"))?,
      state: state.ok_or(TlvDecodeErr::MissingField("State"))?,
      fault_state_previous_state: fault_state_previous_state.ok_or(TlvDecodeErr::MissingField("FaultStatePreviousState"))?,
      fault_state_current_state: fault_state_current_state.ok_or(TlvDecodeErr::MissingField("FaultStateCurrentState"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct RFID {
  pub uid: TypeUnsupported /* Octstr */,
}

impl Event for RFID {
  const EVENT: EventDef = EventDef {
    id: 0x0005,
    name: "RFID",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "UID", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for RFID {
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
      uid: TypeUnsupported,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct GetTargetsResponse {
  pub charging_target_schedules: Vec<ty::ChargingTargetScheduleStruct>,
}
impl Command for GetTargetsResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "GetTargetsResponse",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "ChargingTargetSchedules", attr_ty: <Vec<ty::ChargingTargetScheduleStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for GetTargetsResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
}
impl Command for Disable {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "Disable",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for Disable {
  type Response = BasicAccResponse;
}
impl TlvEncode for Disable {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnableCharging {
  pub charging_enabled_until: TypeUnsupported /* EpochS */,
  pub minimum_charge_current: AmperageMa,
  pub maximum_charge_current: AmperageMa,
}
impl Command for EnableCharging {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "EnableCharging",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ChargingEnabledUntil", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "MinimumChargeCurrent", attr_ty: <AmperageMa>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "MaximumChargeCurrent", attr_ty: <AmperageMa>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for EnableCharging {
  type Response = BasicAccResponse;
}
impl TlvEncode for EnableCharging {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.charging_enabled_until.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.minimum_charge_current.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.maximum_charge_current.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnableDischarging {
  pub discharging_enabled_until: TypeUnsupported /* EpochS */,
  pub maximum_discharge_current: AmperageMa,
}
impl Command for EnableDischarging {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "EnableDischarging",
    conformance: Conformance::Feature(4),
    fields: &[
      AttrDef { id: 0x0000, name: "DischargingEnabledUntil", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "MaximumDischargeCurrent", attr_ty: <AmperageMa>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for EnableDischarging {
  type Response = BasicAccResponse;
}
impl TlvEncode for EnableDischarging {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.discharging_enabled_until.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.maximum_discharge_current.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StartDiagnostics {
}
impl Command for StartDiagnostics {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "StartDiagnostics",
    conformance: Conformance::Optional,
    fields: &[
    ],
  };
}
impl CommandInvoke for StartDiagnostics {
  type Response = BasicAccResponse;
}
impl TlvEncode for StartDiagnostics {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetTargets {
  pub charging_target_schedules: Vec<ty::ChargingTargetScheduleStruct>,
}
impl Command for SetTargets {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "SetTargets",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "ChargingTargetSchedules", attr_ty: <Vec<ty::ChargingTargetScheduleStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SetTargets {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetTargets {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.charging_target_schedules.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetTargets {
}
impl Command for GetTargets {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "GetTargets",
    conformance: Conformance::Feature(0),
    fields: &[
    ],
  };
}
impl CommandInvoke for GetTargets {
  type Response = GetTargetsResponse;
}
impl TlvEncode for GetTargets {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClearTargets {
}
impl Command for ClearTargets {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "ClearTargets",
    conformance: Conformance::Feature(0),
    fields: &[
    ],
  };
}
impl CommandInvoke for ClearTargets {
  type Response = BasicAccResponse;
}
impl TlvEncode for ClearTargets {
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
pub enum EnergyTransferStoppedReasonEnum {
  ///The EV decided to stop
  EVStopped = 0x0000,
  ///The EVSE decided to stop
  EVSEStopped = 0x0001,
  ///An other unknown reason
  Other = 0x0002,
}
impl AttrType for EnergyTransferStoppedReasonEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "EVStopped"),
  (0x0001, "EVSEStopped"),
  (0x0002, "Other"),
  ]);
}
impl TlvPut for EnergyTransferStoppedReasonEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for EnergyTransferStoppedReasonEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::EVStopped),
      0x0001 => Ok(Self::EVSEStopped),
      0x0002 => Ok(Self::Other),
      _ => Err(())
    }
  }
}
impl TlvDecode for EnergyTransferStoppedReasonEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum FaultStateEnum {
  ///The EVSE is not in an error state.
  NoError = 0x0000,
  ///The EVSE is unable to obtain electrical measurements.
  MeterFailure = 0x0001,
  ///The EVSE input voltage level is too high.
  OverVoltage = 0x0002,
  ///The EVSE input voltage level is too low.
  UnderVoltage = 0x0003,
  ///The EVSE detected charging current higher than allowed by charger.
  OverCurrent = 0x0004,
  ///The EVSE detected voltage on charging pins when the contactor is open.
  ContactWetFailure = 0x0005,
  ///The EVSE detected absence of voltage after enabling contactor.
  ContactDryFailure = 0x0006,
  ///The EVSE has an unbalanced current supply.
  GroundFault = 0x0007,
  ///The EVSE has detected a loss in power.
  PowerLoss = 0x0008,
  ///The EVSE has detected another power quality issue (e.g. phase imbalance).
  PowerQuality = 0x0009,
  ///The EVSE pilot signal amplitude short circuited to ground.
  PilotShortCircuit = 0x000a,
  ///The emergency stop button was pressed.
  EmergencyStop = 0x000b,
  ///The EVSE detected that the cable has been disconnected.
  EVDisconnected = 0x000c,
  ///The EVSE could not determine proper power supply level.
  WrongPowerSupply = 0x000d,
  ///The EVSE detected Live and Neutral are swapped.
  LiveNeutralSwap = 0x000e,
  ///The EVSE internal temperature is too high.
  OverTemperature = 0x000f,
  ///Any other reason.
  Other = 0x00ff,
}
impl AttrType for FaultStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "NoError"),
  (0x0001, "MeterFailure"),
  (0x0002, "OverVoltage"),
  (0x0003, "UnderVoltage"),
  (0x0004, "OverCurrent"),
  (0x0005, "ContactWetFailure"),
  (0x0006, "ContactDryFailure"),
  (0x0007, "GroundFault"),
  (0x0008, "PowerLoss"),
  (0x0009, "PowerQuality"),
  (0x000a, "PilotShortCircuit"),
  (0x000b, "EmergencyStop"),
  (0x000c, "EVDisconnected"),
  (0x000d, "WrongPowerSupply"),
  (0x000e, "LiveNeutralSwap"),
  (0x000f, "OverTemperature"),
  (0x00ff, "Other"),
  ]);
}
impl TlvPut for FaultStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for FaultStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::NoError),
      0x0001 => Ok(Self::MeterFailure),
      0x0002 => Ok(Self::OverVoltage),
      0x0003 => Ok(Self::UnderVoltage),
      0x0004 => Ok(Self::OverCurrent),
      0x0005 => Ok(Self::ContactWetFailure),
      0x0006 => Ok(Self::ContactDryFailure),
      0x0007 => Ok(Self::GroundFault),
      0x0008 => Ok(Self::PowerLoss),
      0x0009 => Ok(Self::PowerQuality),
      0x000a => Ok(Self::PilotShortCircuit),
      0x000b => Ok(Self::EmergencyStop),
      0x000c => Ok(Self::EVDisconnected),
      0x000d => Ok(Self::WrongPowerSupply),
      0x000e => Ok(Self::LiveNeutralSwap),
      0x000f => Ok(Self::OverTemperature),
      0x00ff => Ok(Self::Other),
      _ => Err(())
    }
  }
}
impl TlvDecode for FaultStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StateEnum {
  ///The EV is not plugged in.
  NotPluggedIn = 0x0000,
  ///The EV is plugged in, but not demanding current.
  PluggedInNoDemand = 0x0001,
  ///The EV is plugged in and is demanding current, but EVSE is not allowing current to flow.
  PluggedInDemand = 0x0002,
  ///The EV is plugged in, charging is in progress, and current is flowing
  PluggedInCharging = 0x0003,
  ///The EV is plugged in, discharging is in progress, and current is flowing
  PluggedInDischarging = 0x0004,
  ///The EVSE is transitioning from any plugged-in state to NotPluggedIn
  SessionEnding = 0x0005,
  ///There is a fault, further details in the FaultState attribute
  Fault = 0x0006,
}
impl AttrType for StateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "NotPluggedIn"),
  (0x0001, "PluggedInNoDemand"),
  (0x0002, "PluggedInDemand"),
  (0x0003, "PluggedInCharging"),
  (0x0004, "PluggedInDischarging"),
  (0x0005, "SessionEnding"),
  (0x0006, "Fault"),
  ]);
}
impl TlvPut for StateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::NotPluggedIn),
      0x0001 => Ok(Self::PluggedInNoDemand),
      0x0002 => Ok(Self::PluggedInDemand),
      0x0003 => Ok(Self::PluggedInCharging),
      0x0004 => Ok(Self::PluggedInDischarging),
      0x0005 => Ok(Self::SessionEnding),
      0x0006 => Ok(Self::Fault),
      _ => Err(())
    }
  }
}
impl TlvDecode for StateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum SupplyStateEnum {
  ///The EV is not currently allowed to charge or discharge
  Disabled = 0x0000,
  ///The EV is currently allowed to charge
  ChargingEnabled = 0x0001,
  ///The EV is currently allowed to discharge
  DischargingEnabled = 0x0002,
  ///The EV is not currently allowed to charge or discharge due to an error, the error must be cleared before operation can continue
  DisabledError = 0x0003,
  ///The EV is not currently allowed to charge or discharge due to self-diagnostics mode.
  DisabledDiagnostics = 0x0004,
  ///The EV is currently allowed to charge and discharge
  Enabled = 0x0005,
}
impl AttrType for SupplyStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Disabled"),
  (0x0001, "ChargingEnabled"),
  (0x0002, "DischargingEnabled"),
  (0x0003, "DisabledError"),
  (0x0004, "DisabledDiagnostics"),
  (0x0005, "Enabled"),
  ]);
}
impl TlvPut for SupplyStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for SupplyStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Disabled),
      0x0001 => Ok(Self::ChargingEnabled),
      0x0002 => Ok(Self::DischargingEnabled),
      0x0003 => Ok(Self::DisabledError),
      0x0004 => Ok(Self::DisabledDiagnostics),
      0x0005 => Ok(Self::Enabled),
      _ => Err(())
    }
  }
}
impl TlvDecode for SupplyStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct TargetDayOfWeekBitmap: u32 {
    ///Sunday
    const Sunday = 1 << 0;
    ///Monday
    const Monday = 1 << 1;
    ///Tuesday
    const Tuesday = 1 << 2;
    ///Wednesday
    const Wednesday = 1 << 3;
    ///Thursday
    const Thursday = 1 << 4;
    ///Friday
    const Friday = 1 << 5;
    ///Saturday
    const Saturday = 1 << 6;
  }
}
impl AttrType for TargetDayOfWeekBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Sunday"),
    (1, "Monday"),
    (2, "Tuesday"),
    (3, "Wednesday"),
    (4, "Thursday"),
    (5, "Friday"),
    (6, "Saturday"),
  ]);
}
impl TlvPut for TargetDayOfWeekBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for TargetDayOfWeekBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for TargetDayOfWeekBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ChargingTargetScheduleStruct {
  pub day_of_week_for_sequence: TargetDayOfWeekBitmap,
  pub charging_targets: Vec<ChargingTargetStruct>,
}
impl AttrType for ChargingTargetScheduleStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "DayOfWeekForSequence", attr_ty: <TargetDayOfWeekBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ChargingTargets", attr_ty: <Vec<ChargingTargetStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ChargingTargetScheduleStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.day_of_week_for_sequence.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.charging_targets.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ChargingTargetScheduleStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut day_of_week_for_sequence: Option<TargetDayOfWeekBitmap> = None;
    let mut charging_targets: Option<Vec<ChargingTargetStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { day_of_week_for_sequence = Some(<TargetDayOfWeekBitmap>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { charging_targets = Some(<Vec<ChargingTargetStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      day_of_week_for_sequence: day_of_week_for_sequence.ok_or(TlvDecodeErr::MissingField("DayOfWeekForSequence"))?,
      charging_targets: charging_targets.ok_or(TlvDecodeErr::MissingField("ChargingTargets"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChargingTargetStruct {
  pub target_time_minutes_past_midnight: u16,
  pub target_so_c: TypeUnsupported /* Percent */,
  pub added_energy: EnergyMwh,
}
impl AttrType for ChargingTargetStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "TargetTimeMinutesPastMidnight", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "TargetSoC", attr_ty: <TypeUnsupported /* Percent */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "AddedEnergy", attr_ty: <EnergyMwh>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ChargingTargetStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.target_time_minutes_past_midnight.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.target_so_c.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.added_energy.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ChargingTargetStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut target_time_minutes_past_midnight: Option<u16> = None;
    let mut added_energy: Option<EnergyMwh> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { target_time_minutes_past_midnight = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { added_energy = Some(<EnergyMwh>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      target_time_minutes_past_midnight: target_time_minutes_past_midnight.ok_or(TlvDecodeErr::MissingField("TargetTimeMinutesPastMidnight"))?,
      target_so_c: TypeUnsupported,
      added_energy: added_energy.ok_or(TlvDecodeErr::MissingField("AddedEnergy"))?,
    })
  }
}

}
