#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const TEMPERATURE_ALARM: ClusterDef = ClusterDef {
  id: 0x0064,
  name: "Temperature Alarm",
  features: &[
    FeatureDef { bit: 0x0014, code: "OVER", name: "OverTemperature", summary: "Supports activating alarms when a temperature measurement goes over a threshold" },
    FeatureDef { bit: 0x0015, code: "UNDER", name: "UnderTemperature", summary: "Supports activating alarms when a temperature measurement goes under a threshold" },
    FeatureDef { bit: 0x0016, code: "MAJOR", name: "MajorThreshold", summary: "Supports the major threshold for alarms" },
    FeatureDef { bit: 0x0017, code: "MINOR", name: "MinorThreshold", summary: "Supports the minor threshold for alarms" },
    FeatureDef { bit: 0x0018, code: "OCRIADJ", name: "OverCriticalAdjustable", summary: "Supports the ability to adjust the over critical temperature threshold" },
    FeatureDef { bit: 0x0019, code: "OMAJADJ", name: "OverMajorAdjustable", summary: "Supports the ability to adjust the over major temperature threshold" },
    FeatureDef { bit: 0x001a, code: "OMINADJ", name: "OverMinorAdjustable", summary: "Supports the ability to adjust the over minor temperature threshold" },
    FeatureDef { bit: 0x001b, code: "UMINADJ", name: "UnderMinorAdjustable", summary: "Supports the ability to adjust the under minor temperature threshold" },
    FeatureDef { bit: 0x001c, code: "UMAJADJ", name: "UnderMajorAdjustable", summary: "Supports the ability to adjust the under major temperature threshold" },
    FeatureDef { bit: 0x001d, code: "UCRIADJ", name: "UnderCriticalAdjustable", summary: "Supports the ability to adjust the under critical temperature threshold" },
  ],
  attributes: &[
    attr::CriticalOverTemperatureThreshold::ATTR,
    attr::MajorOverTemperatureThreshold::ATTR,
    attr::MinorOverTemperatureThreshold::ATTR,
    attr::MinorUnderTemperatureThreshold::ATTR,
    attr::MajorUnderTemperatureThreshold::ATTR,
    attr::CriticalUnderTemperatureThreshold::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::SetTemperatureAlarmThresholds::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct CriticalOverTemperatureThreshold;

  impl Attr for CriticalOverTemperatureThreshold {
    const ATTR: AttrDef = AttrDef {
      id: 0x0080,
      name: "CriticalOverTemperatureThreshold",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(20),
    };
    type Value = Temperature;
  }


  pub struct MajorOverTemperatureThreshold;

  impl Attr for MajorOverTemperatureThreshold {
    const ATTR: AttrDef = AttrDef {
      id: 0x0081,
      name: "MajorOverTemperatureThreshold",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Temperature;
  }


  pub struct MinorOverTemperatureThreshold;

  impl Attr for MinorOverTemperatureThreshold {
    const ATTR: AttrDef = AttrDef {
      id: 0x0082,
      name: "MinorOverTemperatureThreshold",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Temperature;
  }


  pub struct MinorUnderTemperatureThreshold;

  impl Attr for MinorUnderTemperatureThreshold {
    const ATTR: AttrDef = AttrDef {
      id: 0x0083,
      name: "MinorUnderTemperatureThreshold",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Temperature;
  }


  pub struct MajorUnderTemperatureThreshold;

  impl Attr for MajorUnderTemperatureThreshold {
    const ATTR: AttrDef = AttrDef {
      id: 0x0084,
      name: "MajorUnderTemperatureThreshold",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Temperature;
  }


  pub struct CriticalUnderTemperatureThreshold;

  impl Attr for CriticalUnderTemperatureThreshold {
    const ATTR: AttrDef = AttrDef {
      id: 0x0085,
      name: "CriticalUnderTemperatureThreshold",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(21),
    };
    type Value = Temperature;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SetTemperatureAlarmThresholds {
  pub critical_over_temperature_threshold: Temperature,
  pub major_over_temperature_threshold: Temperature,
  pub minor_over_temperature_threshold: Temperature,
  pub minor_under_temperature_threshold: Temperature,
  pub major_under_temperature_threshold: Temperature,
  pub critical_under_temperature_threshold: Temperature,
}
impl Command for SetTemperatureAlarmThresholds {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0080,
    name: "SetTemperatureAlarmThresholds",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CriticalOverTemperatureThreshold", attr_ty: <Temperature>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0001, name: "MajorOverTemperatureThreshold", attr_ty: <Temperature>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "MinorOverTemperatureThreshold", attr_ty: <Temperature>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0003, name: "MinorUnderTemperatureThreshold", attr_ty: <Temperature>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0004, name: "MajorUnderTemperatureThreshold", attr_ty: <Temperature>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0005, name: "CriticalUnderTemperatureThreshold", attr_ty: <Temperature>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for SetTemperatureAlarmThresholds {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetTemperatureAlarmThresholds {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.critical_over_temperature_threshold.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.major_over_temperature_threshold.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.minor_over_temperature_threshold.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.minor_under_temperature_threshold.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.major_under_temperature_threshold.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.critical_under_temperature_threshold.tlv_put(matter_codec::Tag::Context(0x0005), writer);
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
  pub struct AlarmBitmap: u32 {
    ///The measured temperature is above the critical threshold.
    const CriticalOverTemperatureAlarm = 1 << 0;
    ///The measured temperature is above the major threshold.
    const MajorOverTemperatureAlarm = 1 << 1;
    ///The measured temperature is above the minor threshold.
    const MinorOverTemperatureAlarm = 1 << 2;
    ///The measured temperature is below the minor threshold.
    const MinorUnderTemperatureAlarm = 1 << 3;
    ///The measured temperature is below the major threshold.
    const MajorUnderTemperatureAlarm = 1 << 4;
    ///The measured temperature is below the critical threshold.
    const CriticalUnderTemperatureAlarm = 1 << 5;
  }
}
impl AttrType for AlarmBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "CriticalOverTemperatureAlarm"),
    (1, "MajorOverTemperatureAlarm"),
    (2, "MinorOverTemperatureAlarm"),
    (3, "MinorUnderTemperatureAlarm"),
    (4, "MajorUnderTemperatureAlarm"),
    (5, "CriticalUnderTemperatureAlarm"),
  ]);
}
impl TlvPut for AlarmBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for AlarmBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for AlarmBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
}
