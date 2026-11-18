#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const BOOLEAN_STATE_CONFIGURATION: ClusterDef = ClusterDef {
  id: 0x0080,
  name: "Boolean State Configuration",
  features: &[
    FeatureDef { bit: 0x0000, code: "VIS", name: "Visual", summary: "Supports visual alarms" },
    FeatureDef { bit: 0x0001, code: "AUD", name: "Audible", summary: "Supports audible alarms" },
    FeatureDef { bit: 0x0002, code: "SPRS", name: "AlarmSuppress", summary: "Supports ability to suppress or acknowledge alarms" },
    FeatureDef { bit: 0x0003, code: "SENSLVL", name: "SensitivityLevel", summary: "Supports ability to set sensor sensitivity" },
    FeatureDef { bit: 0x0004, code: "FAULTEV", name: "FaultEvents", summary: "Supports reporting fault events" },
  ],
  attributes: &[
    attr::CurrentSensitivityLevel::ATTR,
    attr::SupportedSensitivityLevels::ATTR,
    attr::DefaultSensitivityLevel::ATTR,
    attr::AlarmsActive::ATTR,
    attr::AlarmsSuppressed::ATTR,
    attr::AlarmsEnabled::ATTR,
    attr::AlarmsSupported::ATTR,
    attr::SensorFault::ATTR,
  ],
  events: &[
    event::AlarmsStateChanged::EVENT,
    event::SensorFault::EVENT,
  ],
  commands: &[
    command::SuppressAlarm::COMMAND,
    command::EnableDisableAlarm::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct CurrentSensitivityLevel;

  impl Attr for CurrentSensitivityLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "CurrentSensitivityLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = u8;
  }


  pub struct SupportedSensitivityLevels;

  impl Attr for SupportedSensitivityLevels {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "SupportedSensitivityLevels",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = u8;
  }


  pub struct DefaultSensitivityLevel;

  impl Attr for DefaultSensitivityLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "DefaultSensitivityLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u8;
  }


  pub struct AlarmsActive;

  impl Attr for AlarmsActive {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "AlarmsActive",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::AlarmModeBitmap;
  }


  pub struct AlarmsSuppressed;

  impl Attr for AlarmsSuppressed {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "AlarmsSuppressed",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = ty::AlarmModeBitmap;
  }


  pub struct AlarmsEnabled;

  impl Attr for AlarmsEnabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "AlarmsEnabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::AlarmModeBitmap;
  }


  pub struct AlarmsSupported;

  impl Attr for AlarmsSupported {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "AlarmsSupported",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::AlarmModeBitmap;
  }


  pub struct SensorFault;

  impl Attr for SensorFault {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "SensorFault",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::SensorFaultBitmap;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct AlarmsStateChanged {
  pub alarms_active: ty::AlarmModeBitmap,
  pub alarms_suppressed: ty::AlarmModeBitmap,
}

impl Event for AlarmsStateChanged {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "AlarmsStateChanged",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "AlarmsActive", attr_ty: <ty::AlarmModeBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "AlarmsSuppressed", attr_ty: <ty::AlarmModeBitmap>::ATTR_TYPE, conformance: Conformance::Feature(2) },
    ],
  };
}

impl TlvDecode for AlarmsStateChanged {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut alarms_active: Option<ty::AlarmModeBitmap> = None;
    let mut alarms_suppressed: Option<ty::AlarmModeBitmap> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { alarms_active = Some(<ty::AlarmModeBitmap>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { alarms_suppressed = Some(<ty::AlarmModeBitmap>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      alarms_active: alarms_active.ok_or(TlvDecodeErr::MissingField("AlarmsActive"))?,
      alarms_suppressed: alarms_suppressed.ok_or(TlvDecodeErr::MissingField("AlarmsSuppressed"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct SensorFault {
  pub sensor_fault: ty::SensorFaultBitmap,
}

impl Event for SensorFault {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "SensorFault",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "SensorFault", attr_ty: <ty::SensorFaultBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for SensorFault {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut sensor_fault: Option<ty::SensorFaultBitmap> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { sensor_fault = Some(<ty::SensorFaultBitmap>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      sensor_fault: sensor_fault.ok_or(TlvDecodeErr::MissingField("SensorFault"))?,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SuppressAlarm {
  pub alarms_to_suppress: ty::AlarmModeBitmap,
}
impl Command for SuppressAlarm {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "SuppressAlarm",
    conformance: Conformance::Feature(2),
    fields: &[
      AttrDef { id: 0x0000, name: "AlarmsToSuppress", attr_ty: <ty::AlarmModeBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SuppressAlarm {
  type Response = BasicAccResponse;
}
impl TlvEncode for SuppressAlarm {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.alarms_to_suppress.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnableDisableAlarm {
  pub alarms_to_enable_disable: ty::AlarmModeBitmap,
}
impl Command for EnableDisableAlarm {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "EnableDisableAlarm",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "AlarmsToEnableDisable", attr_ty: <ty::AlarmModeBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for EnableDisableAlarm {
  type Response = BasicAccResponse;
}
impl TlvEncode for EnableDisableAlarm {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.alarms_to_enable_disable.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct AlarmModeBitmap: u32 {
    ///Visual alarming
    const Visual = 1 << 0;
    ///Audible alarming
    const Audible = 1 << 1;
  }
}
impl AttrType for AlarmModeBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Visual"),
    (1, "Audible"),
  ]);
}
impl TlvPut for AlarmModeBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for AlarmModeBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for AlarmModeBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct SensorFaultBitmap: u32 {
    ///Unspecified fault detected
    const GeneralFault = 1 << 0;
  }
}
impl AttrType for SensorFaultBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "GeneralFault"),
  ]);
}
impl TlvPut for SensorFaultBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for SensorFaultBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for SensorFaultBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
}
