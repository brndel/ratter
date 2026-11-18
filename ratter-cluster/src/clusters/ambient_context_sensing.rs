#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const AMBIENT_CONTEXT_SENSING: ClusterDef = ClusterDef {
  id: 0x0431,
  name: "Ambient Context Sensing",
  features: &[
    FeatureDef { bit: 0x0000, code: "HA", name: "HumanActivity", summary: "Supports various human actions and activities classification" },
    FeatureDef { bit: 0x0001, code: "OC", name: "ObjectCounting", summary: "Supports object counting" },
    FeatureDef { bit: 0x0002, code: "OI", name: "ObjectIdentification", summary: "Supports object identification" },
    FeatureDef { bit: 0x0003, code: "AUD", name: "SoundIdentification", summary: "Supports sound identification" },
    FeatureDef { bit: 0x0004, code: "PRED", name: "PredictedActivity", summary: "Supports predicting various human actions and activities" },
  ],
  attributes: &[
    attr::HumanActivityDetected::ATTR,
    attr::ObjectIdentified::ATTR,
    attr::AudioContextDetected::ATTR,
    attr::AmbientContextType::ATTR,
    attr::AmbientContextTypeSupported::ATTR,
    attr::ObjectCountThresholdReached::ATTR,
    attr::ObjectCountConfig::ATTR,
    attr::ObjectCount::ATTR,
    attr::SimultaneousDetectionLimit::ATTR,
    attr::HoldTime::ATTR,
    attr::HoldTimeLimits::ATTR,
    attr::PredictedActivity::ATTR,
  ],
  events: &[
    event::AmbientContextDetectStarted::EVENT,
    event::AmbientContextDetectEnded::EVENT,
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct HumanActivityDetected;

  impl Attr for HumanActivityDetected {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "HumanActivityDetected",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = bool;
  }


  pub struct ObjectIdentified;

  impl Attr for ObjectIdentified {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "ObjectIdentified",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = bool;
  }


  pub struct AudioContextDetected;

  impl Attr for AudioContextDetected {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "AudioContextDetected",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = bool;
  }


  pub struct AmbientContextType;

  impl Attr for AmbientContextType {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "AmbientContextType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Vec<ty::AmbientContextTypeStruct>;
  }


  pub struct AmbientContextTypeSupported;

  impl Attr for AmbientContextTypeSupported {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "AmbientContextTypeSupported",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Vec<ty::SemanticTagStruct>;
  }


  pub struct ObjectCountThresholdReached;

  impl Attr for ObjectCountThresholdReached {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "ObjectCountThresholdReached",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = bool;
  }


  pub struct ObjectCountConfig;

  impl Attr for ObjectCountConfig {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "ObjectCountConfig",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = ty::ObjectCountConfigStruct;
  }


  pub struct ObjectCount;

  impl Attr for ObjectCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "ObjectCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct SimultaneousDetectionLimit;

  impl Attr for SimultaneousDetectionLimit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "SimultaneousDetectionLimit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u8;
  }


  pub struct HoldTime;

  impl Attr for HoldTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "HoldTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct HoldTimeLimits;

  impl Attr for HoldTimeLimits {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "HoldTimeLimits",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = ty::HoldTimeLimitsStruct;
  }


  pub struct PredictedActivity;

  impl Attr for PredictedActivity {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "PredictedActivity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Vec<ty::PredictedActivityStruct>;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct AmbientContextDetectStarted {
  pub ambient_context_detected: ty::AmbientContextTypeStruct,
  pub object_count_threshold_reached: bool,
  pub object_count: u16,
}

impl Event for AmbientContextDetectStarted {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "AmbientContextDetectStarted",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "AmbientContextDetected", attr_ty: <ty::AmbientContextTypeStruct>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x0001, name: "ObjectCountThresholdReached", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x0002, name: "ObjectCount", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::TooComplicated },
    ],
  };
}

impl TlvDecode for AmbientContextDetectStarted {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut ambient_context_detected: Option<ty::AmbientContextTypeStruct> = None;
    let mut object_count_threshold_reached: Option<bool> = None;
    let mut object_count: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { ambient_context_detected = Some(<ty::AmbientContextTypeStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { object_count_threshold_reached = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { object_count = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      ambient_context_detected: ambient_context_detected.ok_or(TlvDecodeErr::MissingField("AmbientContextDetected"))?,
      object_count_threshold_reached: object_count_threshold_reached.ok_or(TlvDecodeErr::MissingField("ObjectCountThresholdReached"))?,
      object_count: object_count.ok_or(TlvDecodeErr::MissingField("ObjectCount"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct AmbientContextDetectEnded {
  pub event_start_time_pos: TypeUnsupported /* PosixMs */,
  pub event_start_time_sys: TypeUnsupported /* SystimeMs */,
}

impl Event for AmbientContextDetectEnded {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "AmbientContextDetectEnded",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "EventStartTimePos", attr_ty: <TypeUnsupported /* PosixMs */>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x0001, name: "EventStartTimeSys", attr_ty: <TypeUnsupported /* SystimeMs */>::ATTR_TYPE, conformance: Conformance::TooComplicated },
    ],
  };
}

impl TlvDecode for AmbientContextDetectEnded {
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
      event_start_time_pos: TypeUnsupported,
      event_start_time_sys: TypeUnsupported,
    })
  }
}


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct AmbientContextTypeStruct {
  pub ambient_context_sensed: Vec<SemanticTagStruct>,
}
impl AttrType for AmbientContextTypeStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "AmbientContextSensed", attr_ty: <Vec<SemanticTagStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for AmbientContextTypeStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.ambient_context_sensed.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for AmbientContextTypeStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut ambient_context_sensed: Option<Vec<SemanticTagStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { ambient_context_sensed = Some(<Vec<SemanticTagStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      ambient_context_sensed: ambient_context_sensed.ok_or(TlvDecodeErr::MissingField("AmbientContextSensed"))?,
    })
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

#[derive(Debug, Clone, PartialEq)]
pub struct ObjectCountConfigStruct {
  pub counting_object: SemanticTagStruct,
  pub object_count_threshold: u16,
}
impl AttrType for ObjectCountConfigStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "CountingObject", attr_ty: <SemanticTagStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ObjectCountThreshold", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ObjectCountConfigStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.counting_object.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.object_count_threshold.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ObjectCountConfigStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut counting_object: Option<SemanticTagStruct> = None;
    let mut object_count_threshold: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { counting_object = Some(<SemanticTagStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { object_count_threshold = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      counting_object: counting_object.ok_or(TlvDecodeErr::MissingField("CountingObject"))?,
      object_count_threshold: object_count_threshold.ok_or(TlvDecodeErr::MissingField("ObjectCountThreshold"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PredictedActivityStruct {
  pub start_timestamp: TypeUnsupported /* EpochS */,
  pub end_timestamp: TypeUnsupported /* EpochS */,
  pub ambient_context_type: Vec<SemanticTagStruct>,
  pub crowd_detected: bool,
  pub crowd_count: u8,
  pub confidence: TypeUnsupported /* Percent */,
}
impl AttrType for PredictedActivityStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "StartTimestamp", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "EndTimestamp", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "AmbientContextType", attr_ty: <Vec<SemanticTagStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "CrowdDetected", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "CrowdCount", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "Confidence", attr_ty: <TypeUnsupported /* Percent */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for PredictedActivityStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.start_timestamp.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.end_timestamp.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.ambient_context_type.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.crowd_detected.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.crowd_count.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.confidence.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for PredictedActivityStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut ambient_context_type: Option<Vec<SemanticTagStruct>> = None;
    let mut crowd_detected: Option<bool> = None;
    let mut crowd_count: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { ambient_context_type = Some(<Vec<SemanticTagStruct>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { crowd_detected = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { crowd_count = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      start_timestamp: TypeUnsupported,
      end_timestamp: TypeUnsupported,
      ambient_context_type: ambient_context_type.ok_or(TlvDecodeErr::MissingField("AmbientContextType"))?,
      crowd_detected: crowd_detected.ok_or(TlvDecodeErr::MissingField("CrowdDetected"))?,
      crowd_count: crowd_count.ok_or(TlvDecodeErr::MissingField("CrowdCount"))?,
      confidence: TypeUnsupported,
    })
  }
}

}
