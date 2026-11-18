#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const OCCUPANCY_SENSING: ClusterDef = ClusterDef {
  id: 0x0406,
  name: "Occupancy Sensing",
  features: &[
    FeatureDef { bit: 0x0000, code: "OTHER", name: "Other", summary: "Supports sensing using a modality not listed in the other bits" },
    FeatureDef { bit: 0x0001, code: "PIR", name: "PassiveInfrared", summary: "Supports sensing using PIR (Passive InfraRed)" },
    FeatureDef { bit: 0x0002, code: "US", name: "Ultrasonic", summary: "Supports sensing using UltraSound" },
    FeatureDef { bit: 0x0003, code: "PHY", name: "PhysicalContact", summary: "Supports sensing using a physical contact" },
    FeatureDef { bit: 0x0004, code: "AIR", name: "ActiveInfrared", summary: "Supports sensing using Active InfraRed measurement (e.g. time-of-flight or transflective/reflective IR sensing)" },
    FeatureDef { bit: 0x0005, code: "RAD", name: "Radar", summary: "Supports sensing using radar waves (microwave)" },
    FeatureDef { bit: 0x0006, code: "RFS", name: "RFSensing", summary: "Supports sensing using analysis of radio signals, e.g.: RSSI, CSI and/or any other metric from the signal" },
    FeatureDef { bit: 0x0007, code: "VIS", name: "Vision", summary: "Supports sensing based on analyzing images" },
    FeatureDef { bit: 0x0009, code: "OCCEVENT", name: "OccupancyEvent", summary: "Supports generating OccupancyChanged events" },
  ],
  attributes: &[
    attr::Occupancy::ATTR,
    attr::OccupancySensorType::ATTR,
    attr::OccupancySensorTypeBitmap::ATTR,
    attr::HoldTime::ATTR,
    attr::HoldTimeLimits::ATTR,
    attr::PIROccupiedToUnoccupiedDelay::ATTR,
    attr::PIRUnoccupiedToOccupiedDelay::ATTR,
    attr::PIRUnoccupiedToOccupiedThreshold::ATTR,
    attr::UltrasonicOccupiedToUnoccupiedDelay::ATTR,
    attr::UltrasonicUnoccupiedToOccupiedDelay::ATTR,
    attr::UltrasonicUnoccupiedToOccupiedThreshold::ATTR,
    attr::PhysicalContactOccupiedToUnoccupiedDelay::ATTR,
    attr::PhysicalContactUnoccupiedToOccupiedDelay::ATTR,
    attr::PhysicalContactUnoccupiedToOccupiedThreshold::ATTR,
  ],
  events: &[
    event::OccupancyChanged::EVENT,
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct Occupancy;

  impl Attr for Occupancy {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "Occupancy",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::OccupancyBitmap;
  }


  pub struct OccupancySensorType;

  impl Attr for OccupancySensorType {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "OccupancySensorType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = ty::OccupancySensorTypeEnum;
  }


  pub struct OccupancySensorTypeBitmap;

  impl Attr for OccupancySensorTypeBitmap {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "OccupancySensorTypeBitmap",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = ty::OccupancySensorTypeBitmap;
  }


  pub struct HoldTime;

  impl Attr for HoldTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "HoldTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct HoldTimeLimits;

  impl Attr for HoldTimeLimits {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "HoldTimeLimits",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::HoldTimeLimitsStruct;
  }


  pub struct PIROccupiedToUnoccupiedDelay;

  impl Attr for PIROccupiedToUnoccupiedDelay {
    const ATTR: AttrDef = AttrDef {
      id: 0x0010,
      name: "PIROccupiedToUnoccupiedDelay",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct PIRUnoccupiedToOccupiedDelay;

  impl Attr for PIRUnoccupiedToOccupiedDelay {
    const ATTR: AttrDef = AttrDef {
      id: 0x0011,
      name: "PIRUnoccupiedToOccupiedDelay",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct PIRUnoccupiedToOccupiedThreshold;

  impl Attr for PIRUnoccupiedToOccupiedThreshold {
    const ATTR: AttrDef = AttrDef {
      id: 0x0012,
      name: "PIRUnoccupiedToOccupiedThreshold",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u8;
  }


  pub struct UltrasonicOccupiedToUnoccupiedDelay;

  impl Attr for UltrasonicOccupiedToUnoccupiedDelay {
    const ATTR: AttrDef = AttrDef {
      id: 0x0020,
      name: "UltrasonicOccupiedToUnoccupiedDelay",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct UltrasonicUnoccupiedToOccupiedDelay;

  impl Attr for UltrasonicUnoccupiedToOccupiedDelay {
    const ATTR: AttrDef = AttrDef {
      id: 0x0021,
      name: "UltrasonicUnoccupiedToOccupiedDelay",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct UltrasonicUnoccupiedToOccupiedThreshold;

  impl Attr for UltrasonicUnoccupiedToOccupiedThreshold {
    const ATTR: AttrDef = AttrDef {
      id: 0x0022,
      name: "UltrasonicUnoccupiedToOccupiedThreshold",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u8;
  }


  pub struct PhysicalContactOccupiedToUnoccupiedDelay;

  impl Attr for PhysicalContactOccupiedToUnoccupiedDelay {
    const ATTR: AttrDef = AttrDef {
      id: 0x0030,
      name: "PhysicalContactOccupiedToUnoccupiedDelay",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct PhysicalContactUnoccupiedToOccupiedDelay;

  impl Attr for PhysicalContactUnoccupiedToOccupiedDelay {
    const ATTR: AttrDef = AttrDef {
      id: 0x0031,
      name: "PhysicalContactUnoccupiedToOccupiedDelay",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct PhysicalContactUnoccupiedToOccupiedThreshold;

  impl Attr for PhysicalContactUnoccupiedToOccupiedThreshold {
    const ATTR: AttrDef = AttrDef {
      id: 0x0032,
      name: "PhysicalContactUnoccupiedToOccupiedThreshold",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u8;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct OccupancyChanged {
  pub occupancy: ty::OccupancyBitmap,
}

impl Event for OccupancyChanged {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "OccupancyChanged",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "Occupancy", attr_ty: <ty::OccupancyBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for OccupancyChanged {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut occupancy: Option<ty::OccupancyBitmap> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { occupancy = Some(<ty::OccupancyBitmap>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      occupancy: occupancy.ok_or(TlvDecodeErr::MissingField("Occupancy"))?,
    })
  }
}


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum OccupancySensorTypeEnum {
  ///Indicates a passive infrared sensor.
  PIR = 0x0000,
  ///Indicates a ultrasonic sensor.
  Ultrasonic = 0x0001,
  ///Indicates a passive infrared and ultrasonic sensor.
  PIRAndUltrasonic = 0x0002,
  ///Indicates a physical contact sensor.
  PhysicalContact = 0x0003,
}
impl AttrType for OccupancySensorTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "PIR"),
  (0x0001, "Ultrasonic"),
  (0x0002, "PIRAndUltrasonic"),
  (0x0003, "PhysicalContact"),
  ]);
}
impl TlvPut for OccupancySensorTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for OccupancySensorTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::PIR),
      0x0001 => Ok(Self::Ultrasonic),
      0x0002 => Ok(Self::PIRAndUltrasonic),
      0x0003 => Ok(Self::PhysicalContact),
      _ => Err(())
    }
  }
}
impl TlvDecode for OccupancySensorTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct OccupancyBitmap: u32 {
    ///Indicates the sensed occupancy state
    const Occupied = 1 << 0;
  }
}
impl AttrType for OccupancyBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Occupied"),
  ]);
}
impl TlvPut for OccupancyBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for OccupancyBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for OccupancyBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct OccupancySensorTypeBitmap: u32 {
    ///Indicates a passive infrared sensor.
    const PIR = 1 << 0;
    ///Indicates a ultrasonic sensor.
    const Ultrasonic = 1 << 1;
    ///Indicates a physical contact sensor.
    const PhysicalContact = 1 << 2;
  }
}
impl AttrType for OccupancySensorTypeBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "PIR"),
    (1, "Ultrasonic"),
    (2, "PhysicalContact"),
  ]);
}
impl TlvPut for OccupancySensorTypeBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for OccupancySensorTypeBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for OccupancySensorTypeBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct HoldTimeLimitsStruct {
  pub hold_time_min: u16,
  pub hold_time_max: u16,
  pub hold_time_default: u16,
}
impl AttrType for HoldTimeLimitsStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "HoldTimeMin", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "HoldTimeMax", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "HoldTimeDefault", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for HoldTimeLimitsStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.hold_time_min.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.hold_time_max.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.hold_time_default.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for HoldTimeLimitsStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut hold_time_min: Option<u16> = None;
    let mut hold_time_max: Option<u16> = None;
    let mut hold_time_default: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { hold_time_min = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { hold_time_max = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { hold_time_default = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      hold_time_min: hold_time_min.ok_or(TlvDecodeErr::MissingField("HoldTimeMin"))?,
      hold_time_max: hold_time_max.ok_or(TlvDecodeErr::MissingField("HoldTimeMax"))?,
      hold_time_default: hold_time_default.ok_or(TlvDecodeErr::MissingField("HoldTimeDefault"))?,
    })
  }
}

}
