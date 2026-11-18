#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const SMOKE_CO_ALARM: ClusterDef = ClusterDef {
  id: 0x005c,
  name: "Smoke CO Alarm",
  features: &[
    FeatureDef { bit: 0x0000, code: "SMOKE", name: "SmokeAlarm", summary: "Supports Smoke alarm" },
    FeatureDef { bit: 0x0001, code: "CO", name: "COAlarm", summary: "Supports CO alarm" },
  ],
  attributes: &[
    attr::ExpressedState::ATTR,
    attr::SmokeState::ATTR,
    attr::COState::ATTR,
    attr::BatteryAlert::ATTR,
    attr::DeviceMuted::ATTR,
    attr::TestInProgress::ATTR,
    attr::HardwareFaultAlert::ATTR,
    attr::EndOfServiceAlert::ATTR,
    attr::InterconnectSmokeAlarm::ATTR,
    attr::InterconnectCOAlarm::ATTR,
    attr::ContaminationState::ATTR,
    attr::SmokeSensitivityLevel::ATTR,
    attr::ExpiryDate::ATTR,
    attr::Unmounted::ATTR,
  ],
  events: &[
    event::SmokeAlarm::EVENT,
    event::COAlarm::EVENT,
    event::LowBattery::EVENT,
    event::HardwareFault::EVENT,
    event::EndOfService::EVENT,
    event::SelfTestComplete::EVENT,
    event::AlarmMuted::EVENT,
    event::MuteEnded::EVENT,
    event::InterconnectSmokeAlarm::EVENT,
    event::InterconnectCOAlarm::EVENT,
    event::AllClear::EVENT,
  ],
  commands: &[
    command::SelfTestRequest::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct ExpressedState;

  impl Attr for ExpressedState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "ExpressedState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::ExpressedStateEnum;
  }


  pub struct SmokeState;

  impl Attr for SmokeState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "SmokeState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = ty::AlarmStateEnum;
  }


  pub struct COState;

  impl Attr for COState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "COState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = ty::AlarmStateEnum;
  }


  pub struct BatteryAlert;

  impl Attr for BatteryAlert {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "BatteryAlert",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::AlarmStateEnum;
  }


  pub struct DeviceMuted;

  impl Attr for DeviceMuted {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "DeviceMuted",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::MuteStateEnum;
  }


  pub struct TestInProgress;

  impl Attr for TestInProgress {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "TestInProgress",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


  pub struct HardwareFaultAlert;

  impl Attr for HardwareFaultAlert {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "HardwareFaultAlert",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


  pub struct EndOfServiceAlert;

  impl Attr for EndOfServiceAlert {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "EndOfServiceAlert",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::EndOfServiceEnum;
  }


  pub struct InterconnectSmokeAlarm;

  impl Attr for InterconnectSmokeAlarm {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "InterconnectSmokeAlarm",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::AlarmStateEnum;
  }


  pub struct InterconnectCOAlarm;

  impl Attr for InterconnectCOAlarm {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "InterconnectCOAlarm",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::AlarmStateEnum;
  }


  pub struct ContaminationState;

  impl Attr for ContaminationState {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "ContaminationState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::ContaminationStateEnum;
  }


  pub struct SmokeSensitivityLevel;

  impl Attr for SmokeSensitivityLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "SmokeSensitivityLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::SensitivityEnum;
  }


  pub struct ExpiryDate;

  impl Attr for ExpiryDate {
    const ATTR: AttrDef = AttrDef {
      id: 0x000c,
      name: "ExpiryDate",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = TypeUnsupported /* EpochS */;
  }


  pub struct Unmounted;

  impl Attr for Unmounted {
    const ATTR: AttrDef = AttrDef {
      id: 0x000d,
      name: "Unmounted",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SmokeAlarm {
  pub alarm_severity_level: ty::AlarmStateEnum,
}

impl Event for SmokeAlarm {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "SmokeAlarm",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "AlarmSeverityLevel", attr_ty: <ty::AlarmStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for SmokeAlarm {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut alarm_severity_level: Option<ty::AlarmStateEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { alarm_severity_level = Some(<ty::AlarmStateEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      alarm_severity_level: alarm_severity_level.ok_or(TlvDecodeErr::MissingField("AlarmSeverityLevel"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct COAlarm {
  pub alarm_severity_level: ty::AlarmStateEnum,
}

impl Event for COAlarm {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "COAlarm",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "AlarmSeverityLevel", attr_ty: <ty::AlarmStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for COAlarm {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut alarm_severity_level: Option<ty::AlarmStateEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { alarm_severity_level = Some(<ty::AlarmStateEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      alarm_severity_level: alarm_severity_level.ok_or(TlvDecodeErr::MissingField("AlarmSeverityLevel"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct LowBattery {
  pub alarm_severity_level: ty::AlarmStateEnum,
}

impl Event for LowBattery {
  const EVENT: EventDef = EventDef {
    id: 0x0002,
    name: "LowBattery",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "AlarmSeverityLevel", attr_ty: <ty::AlarmStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for LowBattery {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut alarm_severity_level: Option<ty::AlarmStateEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { alarm_severity_level = Some(<ty::AlarmStateEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      alarm_severity_level: alarm_severity_level.ok_or(TlvDecodeErr::MissingField("AlarmSeverityLevel"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct HardwareFault {
}

impl Event for HardwareFault {
  const EVENT: EventDef = EventDef {
    id: 0x0003,
    name: "HardwareFault",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}

impl TlvDecode for HardwareFault {
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
pub struct EndOfService {
}

impl Event for EndOfService {
  const EVENT: EventDef = EventDef {
    id: 0x0004,
    name: "EndOfService",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}

impl TlvDecode for EndOfService {
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
pub struct SelfTestComplete {
}

impl Event for SelfTestComplete {
  const EVENT: EventDef = EventDef {
    id: 0x0005,
    name: "SelfTestComplete",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}

impl TlvDecode for SelfTestComplete {
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
pub struct AlarmMuted {
}

impl Event for AlarmMuted {
  const EVENT: EventDef = EventDef {
    id: 0x0006,
    name: "AlarmMuted",
    conformance: Conformance::Optional,
    fields: &[
    ],
  };
}

impl TlvDecode for AlarmMuted {
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
pub struct MuteEnded {
}

impl Event for MuteEnded {
  const EVENT: EventDef = EventDef {
    id: 0x0007,
    name: "MuteEnded",
    conformance: Conformance::Optional,
    fields: &[
    ],
  };
}

impl TlvDecode for MuteEnded {
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
pub struct InterconnectSmokeAlarm {
  pub alarm_severity_level: ty::AlarmStateEnum,
}

impl Event for InterconnectSmokeAlarm {
  const EVENT: EventDef = EventDef {
    id: 0x0008,
    name: "InterconnectSmokeAlarm",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "AlarmSeverityLevel", attr_ty: <ty::AlarmStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for InterconnectSmokeAlarm {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut alarm_severity_level: Option<ty::AlarmStateEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { alarm_severity_level = Some(<ty::AlarmStateEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      alarm_severity_level: alarm_severity_level.ok_or(TlvDecodeErr::MissingField("AlarmSeverityLevel"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct InterconnectCOAlarm {
  pub alarm_severity_level: ty::AlarmStateEnum,
}

impl Event for InterconnectCOAlarm {
  const EVENT: EventDef = EventDef {
    id: 0x0009,
    name: "InterconnectCOAlarm",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "AlarmSeverityLevel", attr_ty: <ty::AlarmStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for InterconnectCOAlarm {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut alarm_severity_level: Option<ty::AlarmStateEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { alarm_severity_level = Some(<ty::AlarmStateEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      alarm_severity_level: alarm_severity_level.ok_or(TlvDecodeErr::MissingField("AlarmSeverityLevel"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct AllClear {
}

impl Event for AllClear {
  const EVENT: EventDef = EventDef {
    id: 0x000a,
    name: "AllClear",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}

impl TlvDecode for AllClear {
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


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SelfTestRequest {
}
impl Command for SelfTestRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "SelfTestRequest",
    conformance: Conformance::Optional,
    fields: &[
    ],
  };
}
impl CommandInvoke for SelfTestRequest {
  type Response = BasicAccResponse;
}
impl TlvEncode for SelfTestRequest {
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
pub enum AlarmStateEnum {
  ///Nominal state, the device is not alarming
  Normal = 0x0000,
  ///Warning state
  Warning = 0x0001,
  ///Critical state
  Critical = 0x0002,
}
impl AttrType for AlarmStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Normal"),
  (0x0001, "Warning"),
  (0x0002, "Critical"),
  ]);
}
impl TlvPut for AlarmStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for AlarmStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Normal),
      0x0001 => Ok(Self::Warning),
      0x0002 => Ok(Self::Critical),
      _ => Err(())
    }
  }
}
impl TlvDecode for AlarmStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ContaminationStateEnum {
  ///Nominal state, the sensor is not contaminated
  Normal = 0x0000,
  ///Low contamination
  Low = 0x0001,
  ///Warning state
  Warning = 0x0002,
  ///Critical state, will cause nuisance alarms
  Critical = 0x0003,
}
impl AttrType for ContaminationStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Normal"),
  (0x0001, "Low"),
  (0x0002, "Warning"),
  (0x0003, "Critical"),
  ]);
}
impl TlvPut for ContaminationStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ContaminationStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Normal),
      0x0001 => Ok(Self::Low),
      0x0002 => Ok(Self::Warning),
      0x0003 => Ok(Self::Critical),
      _ => Err(())
    }
  }
}
impl TlvDecode for ContaminationStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum EndOfServiceEnum {
  ///Device has not expired
  Normal = 0x0000,
  ///Device has reached its end of service
  Expired = 0x0001,
}
impl AttrType for EndOfServiceEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Normal"),
  (0x0001, "Expired"),
  ]);
}
impl TlvPut for EndOfServiceEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for EndOfServiceEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Normal),
      0x0001 => Ok(Self::Expired),
      _ => Err(())
    }
  }
}
impl TlvDecode for EndOfServiceEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ExpressedStateEnum {
  ///Nominal state, the device is not alarming
  Normal = 0x0000,
  ///Smoke Alarm state
  SmokeAlarm = 0x0001,
  ///CO Alarm state
  COAlarm = 0x0002,
  ///Battery Alert State
  BatteryAlert = 0x0003,
  ///Test in Progress
  Testing = 0x0004,
  ///Hardware Fault Alert State
  HardwareFault = 0x0005,
  ///End of Service Alert State
  EndOfService = 0x0006,
  ///Interconnected Smoke Alarm State
  InterconnectSmoke = 0x0007,
  ///Interconnected CO Alarm State
  InterconnectCO = 0x0008,
  ///Hardware is not able to detect Smoke or CO
  Inoperative = 0x0009,
}
impl AttrType for ExpressedStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Normal"),
  (0x0001, "SmokeAlarm"),
  (0x0002, "COAlarm"),
  (0x0003, "BatteryAlert"),
  (0x0004, "Testing"),
  (0x0005, "HardwareFault"),
  (0x0006, "EndOfService"),
  (0x0007, "InterconnectSmoke"),
  (0x0008, "InterconnectCO"),
  (0x0009, "Inoperative"),
  ]);
}
impl TlvPut for ExpressedStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ExpressedStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Normal),
      0x0001 => Ok(Self::SmokeAlarm),
      0x0002 => Ok(Self::COAlarm),
      0x0003 => Ok(Self::BatteryAlert),
      0x0004 => Ok(Self::Testing),
      0x0005 => Ok(Self::HardwareFault),
      0x0006 => Ok(Self::EndOfService),
      0x0007 => Ok(Self::InterconnectSmoke),
      0x0008 => Ok(Self::InterconnectCO),
      0x0009 => Ok(Self::Inoperative),
      _ => Err(())
    }
  }
}
impl TlvDecode for ExpressedStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum MuteStateEnum {
  ///Not Muted
  NotMuted = 0x0000,
  ///Muted
  Muted = 0x0001,
}
impl AttrType for MuteStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "NotMuted"),
  (0x0001, "Muted"),
  ]);
}
impl TlvPut for MuteStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for MuteStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::NotMuted),
      0x0001 => Ok(Self::Muted),
      _ => Err(())
    }
  }
}
impl TlvDecode for MuteStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum SensitivityEnum {
  ///High sensitivity
  High = 0x0000,
  ///Standard Sensitivity
  Standard = 0x0001,
  ///Low sensitivity
  Low = 0x0002,
}
impl AttrType for SensitivityEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "High"),
  (0x0001, "Standard"),
  (0x0002, "Low"),
  ]);
}
impl TlvPut for SensitivityEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for SensitivityEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::High),
      0x0001 => Ok(Self::Standard),
      0x0002 => Ok(Self::Low),
      _ => Err(())
    }
  }
}
impl TlvDecode for SensitivityEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
