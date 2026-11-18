#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const ELECTRICAL_POWER_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x0090,
  name: "Electrical Power Measurement",
  features: &[
    FeatureDef { bit: 0x0000, code: "DIRC", name: "DirectCurrent", summary: "Supports measurement of direct current" },
    FeatureDef { bit: 0x0001, code: "ALTC", name: "AlternatingCurrent", summary: "Supports measurement of alternating current" },
    FeatureDef { bit: 0x0002, code: "POLY", name: "PolyphasePower", summary: "Supports polyphase measurements" },
    FeatureDef { bit: 0x0003, code: "HARM", name: "Harmonics", summary: "Supports measurement of AC harmonics" },
    FeatureDef { bit: 0x0004, code: "PWRQ", name: "PowerQuality", summary: "Supports measurement of AC harmonic phases" },
  ],
  attributes: &[
    attr::PowerMode::ATTR,
    attr::NumberOfMeasurementTypes::ATTR,
    attr::Accuracy::ATTR,
    attr::Ranges::ATTR,
    attr::Voltage::ATTR,
    attr::ActiveCurrent::ATTR,
    attr::ReactiveCurrent::ATTR,
    attr::ApparentCurrent::ATTR,
    attr::ActivePower::ATTR,
    attr::ReactivePower::ATTR,
    attr::ApparentPower::ATTR,
    attr::RMSVoltage::ATTR,
    attr::RMSCurrent::ATTR,
    attr::RMSPower::ATTR,
    attr::Frequency::ATTR,
    attr::HarmonicCurrents::ATTR,
    attr::HarmonicPhases::ATTR,
    attr::PowerFactor::ATTR,
    attr::NeutralCurrent::ATTR,
  ],
  events: &[
    event::MeasurementPeriodRanges::EVENT,
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct PowerMode;

  impl Attr for PowerMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "PowerMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::PowerModeEnum;
  }


  pub struct NumberOfMeasurementTypes;

  impl Attr for NumberOfMeasurementTypes {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "NumberOfMeasurementTypes",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct Accuracy;

  impl Attr for Accuracy {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "Accuracy",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::MeasurementAccuracyStruct>;
  }


  pub struct Ranges;

  impl Attr for Ranges {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "Ranges",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Vec<ty::MeasurementRangeStruct>;
  }


  pub struct Voltage;

  impl Attr for Voltage {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "Voltage",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<VoltageMv>;
  }


  pub struct ActiveCurrent;

  impl Attr for ActiveCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "ActiveCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<AmperageMa>;
  }


  pub struct ReactiveCurrent;

  impl Attr for ReactiveCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "ReactiveCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<AmperageMa>;
  }


  pub struct ApparentCurrent;

  impl Attr for ApparentCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "ApparentCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<AmperageMa>;
  }


  pub struct ActivePower;

  impl Attr for ActivePower {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "ActivePower",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<PowerMw>;
  }


  pub struct ReactivePower;

  impl Attr for ReactivePower {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "ReactivePower",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<TypeUnsupported /* PowerMVAR */>;
  }


  pub struct ApparentPower;

  impl Attr for ApparentPower {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "ApparentPower",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<TypeUnsupported /* PowerMVA */>;
  }


  pub struct RMSVoltage;

  impl Attr for RMSVoltage {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "RMSVoltage",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<VoltageMv>;
  }


  pub struct RMSCurrent;

  impl Attr for RMSCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x000c,
      name: "RMSCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<AmperageMa>;
  }


  pub struct RMSPower;

  impl Attr for RMSPower {
    const ATTR: AttrDef = AttrDef {
      id: 0x000d,
      name: "RMSPower",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<PowerMw>;
  }


  pub struct Frequency;

  impl Attr for Frequency {
    const ATTR: AttrDef = AttrDef {
      id: 0x000e,
      name: "Frequency",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<i64>;
  }


  pub struct HarmonicCurrents;

  impl Attr for HarmonicCurrents {
    const ATTR: AttrDef = AttrDef {
      id: 0x000f,
      name: "HarmonicCurrents",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = Nullable<Vec<ty::HarmonicMeasurementStruct>>;
  }


  pub struct HarmonicPhases;

  impl Attr for HarmonicPhases {
    const ATTR: AttrDef = AttrDef {
      id: 0x0010,
      name: "HarmonicPhases",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = Nullable<Vec<ty::HarmonicMeasurementStruct>>;
  }


  pub struct PowerFactor;

  impl Attr for PowerFactor {
    const ATTR: AttrDef = AttrDef {
      id: 0x0011,
      name: "PowerFactor",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<i64>;
  }


  pub struct NeutralCurrent;

  impl Attr for NeutralCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0012,
      name: "NeutralCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<AmperageMa>;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementPeriodRanges {
  pub ranges: Vec<ty::MeasurementRangeStruct>,
}

impl Event for MeasurementPeriodRanges {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "MeasurementPeriodRanges",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Ranges", attr_ty: <Vec<ty::MeasurementRangeStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for MeasurementPeriodRanges {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut ranges: Option<Vec<ty::MeasurementRangeStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { ranges = Some(<Vec<ty::MeasurementRangeStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      ranges: ranges.ok_or(TlvDecodeErr::MissingField("Ranges"))?,
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
pub enum MeasurementTypeEnum {
  Unspecified = 0x0000,
  ///Voltage in millivolts (mV)
  Voltage = 0x0001,
  ///Active current in milliamps (mA)
  ActiveCurrent = 0x0002,
  ///Reactive current in milliamps (mA)
  ReactiveCurrent = 0x0003,
  ///Apparent current in milliamps (mA)
  ApparentCurrent = 0x0004,
  ///Active power in milliwatts (mW)
  ActivePower = 0x0005,
  ///Reactive power in millivolt-amps reactive (mVAR)
  ReactivePower = 0x0006,
  ///Apparent power in millivolt-amps (mVA)
  ApparentPower = 0x0007,
  ///Root mean squared voltage in millivolts (mV)
  RMSVoltage = 0x0008,
  ///Root mean squared current in milliamps (mA)
  RMSCurrent = 0x0009,
  ///Root mean squared power in milliwatts (mW)
  RMSPower = 0x000a,
  ///AC frequency in millihertz (mHz)
  Frequency = 0x000b,
  ///Power Factor ratio in +/- 1/100ths of a percent.
  PowerFactor = 0x000c,
  ///AC neutral current in milliamps (mA)
  NeutralCurrent = 0x000d,
  ///Electrical energy in milliwatt-hours (mWh)
  ElectricalEnergy = 0x000e,
  ///Reactive power in millivolt-amp-hours reactive (mVARh)
  ReactiveEnergy = 0x000f,
  ///Apparent power in millivolt-amp-hours (mVAh)
  ApparentEnergy = 0x0010,
}
impl AttrType for MeasurementTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "Voltage"),
  (0x0002, "ActiveCurrent"),
  (0x0003, "ReactiveCurrent"),
  (0x0004, "ApparentCurrent"),
  (0x0005, "ActivePower"),
  (0x0006, "ReactivePower"),
  (0x0007, "ApparentPower"),
  (0x0008, "RMSVoltage"),
  (0x0009, "RMSCurrent"),
  (0x000a, "RMSPower"),
  (0x000b, "Frequency"),
  (0x000c, "PowerFactor"),
  (0x000d, "NeutralCurrent"),
  (0x000e, "ElectricalEnergy"),
  (0x000f, "ReactiveEnergy"),
  (0x0010, "ApparentEnergy"),
  ]);
}
impl TlvPut for MeasurementTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for MeasurementTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unspecified),
      0x0001 => Ok(Self::Voltage),
      0x0002 => Ok(Self::ActiveCurrent),
      0x0003 => Ok(Self::ReactiveCurrent),
      0x0004 => Ok(Self::ApparentCurrent),
      0x0005 => Ok(Self::ActivePower),
      0x0006 => Ok(Self::ReactivePower),
      0x0007 => Ok(Self::ApparentPower),
      0x0008 => Ok(Self::RMSVoltage),
      0x0009 => Ok(Self::RMSCurrent),
      0x000a => Ok(Self::RMSPower),
      0x000b => Ok(Self::Frequency),
      0x000c => Ok(Self::PowerFactor),
      0x000d => Ok(Self::NeutralCurrent),
      0x000e => Ok(Self::ElectricalEnergy),
      0x000f => Ok(Self::ReactiveEnergy),
      0x0010 => Ok(Self::ApparentEnergy),
      _ => Err(())
    }
  }
}
impl TlvDecode for MeasurementTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum PowerModeEnum {
  Unknown = 0x0000,
  ///Direct current
  DC = 0x0001,
  ///Alternating current, either single-phase or polyphase
  AC = 0x0002,
}
impl AttrType for PowerModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unknown"),
  (0x0001, "DC"),
  (0x0002, "AC"),
  ]);
}
impl TlvPut for PowerModeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for PowerModeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unknown),
      0x0001 => Ok(Self::DC),
      0x0002 => Ok(Self::AC),
      _ => Err(())
    }
  }
}
impl TlvDecode for PowerModeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct HarmonicMeasurementStruct {
  pub order: u8,
  pub measurement: i64,
}
impl AttrType for HarmonicMeasurementStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Order", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Measurement", attr_ty: <i64>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for HarmonicMeasurementStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.order.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.measurement.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for HarmonicMeasurementStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut order: Option<u8> = None;
    let mut measurement: Option<i64> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { order = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { measurement = Some(<i64>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      order: order.ok_or(TlvDecodeErr::MissingField("Order"))?,
      measurement: measurement.ok_or(TlvDecodeErr::MissingField("Measurement"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementAccuracyRangeStruct {
  pub range_min: i64,
  pub range_max: i64,
  pub percent_max: TypeUnsupported /* Percent100ths */,
  pub percent_min: TypeUnsupported /* Percent100ths */,
  pub percent_typical: TypeUnsupported /* Percent100ths */,
  pub fixed_max: u64,
  pub fixed_min: u64,
  pub fixed_typical: u64,
}
impl AttrType for MeasurementAccuracyRangeStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "RangeMin", attr_ty: <i64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "RangeMax", attr_ty: <i64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "PercentMax", attr_ty: <TypeUnsupported /* Percent100ths */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "PercentMin", attr_ty: <TypeUnsupported /* Percent100ths */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "PercentTypical", attr_ty: <TypeUnsupported /* Percent100ths */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "FixedMax", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "FixedMin", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "FixedTypical", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for MeasurementAccuracyRangeStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.range_min.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.range_max.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.percent_max.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.percent_min.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.percent_typical.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.fixed_max.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.fixed_min.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.fixed_typical.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for MeasurementAccuracyRangeStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut range_min: Option<i64> = None;
    let mut range_max: Option<i64> = None;
    let mut fixed_max: Option<u64> = None;
    let mut fixed_min: Option<u64> = None;
    let mut fixed_typical: Option<u64> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { range_min = Some(<i64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { range_max = Some(<i64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { fixed_max = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { fixed_min = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0007), value: _ }) => { fixed_typical = Some(<u64>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      range_min: range_min.ok_or(TlvDecodeErr::MissingField("RangeMin"))?,
      range_max: range_max.ok_or(TlvDecodeErr::MissingField("RangeMax"))?,
      percent_max: TypeUnsupported,
      percent_min: TypeUnsupported,
      percent_typical: TypeUnsupported,
      fixed_max: fixed_max.ok_or(TlvDecodeErr::MissingField("FixedMax"))?,
      fixed_min: fixed_min.ok_or(TlvDecodeErr::MissingField("FixedMin"))?,
      fixed_typical: fixed_typical.ok_or(TlvDecodeErr::MissingField("FixedTypical"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementAccuracyStruct {
  pub measurement_type: MeasurementTypeEnum,
  pub measured: bool,
  pub min_measured_value: i64,
  pub max_measured_value: i64,
  pub accuracy_ranges: Vec<MeasurementAccuracyRangeStruct>,
}
impl AttrType for MeasurementAccuracyStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "MeasurementType", attr_ty: <MeasurementTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Measured", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "MinMeasuredValue", attr_ty: <i64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "MaxMeasuredValue", attr_ty: <i64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "AccuracyRanges", attr_ty: <Vec<MeasurementAccuracyRangeStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for MeasurementAccuracyStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.measurement_type.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.measured.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.min_measured_value.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.max_measured_value.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.accuracy_ranges.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for MeasurementAccuracyStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut measurement_type: Option<MeasurementTypeEnum> = None;
    let mut measured: Option<bool> = None;
    let mut min_measured_value: Option<i64> = None;
    let mut max_measured_value: Option<i64> = None;
    let mut accuracy_ranges: Option<Vec<MeasurementAccuracyRangeStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { measurement_type = Some(<MeasurementTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { measured = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { min_measured_value = Some(<i64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { max_measured_value = Some(<i64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0004), kind: _ }) => { accuracy_ranges = Some(<Vec<MeasurementAccuracyRangeStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      measurement_type: measurement_type.ok_or(TlvDecodeErr::MissingField("MeasurementType"))?,
      measured: measured.ok_or(TlvDecodeErr::MissingField("Measured"))?,
      min_measured_value: min_measured_value.ok_or(TlvDecodeErr::MissingField("MinMeasuredValue"))?,
      max_measured_value: max_measured_value.ok_or(TlvDecodeErr::MissingField("MaxMeasuredValue"))?,
      accuracy_ranges: accuracy_ranges.ok_or(TlvDecodeErr::MissingField("AccuracyRanges"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementRangeStruct {
  pub measurement_type: MeasurementTypeEnum,
  pub min: i64,
  pub max: i64,
  pub start_timestamp: TypeUnsupported /* EpochS */,
  pub end_timestamp: TypeUnsupported /* EpochS */,
  pub min_timestamp: TypeUnsupported /* EpochS */,
  pub max_timestamp: TypeUnsupported /* EpochS */,
  pub start_systime: TypeUnsupported /* SystimeMs */,
  pub end_systime: TypeUnsupported /* SystimeMs */,
  pub min_systime: TypeUnsupported /* SystimeMs */,
  pub max_systime: TypeUnsupported /* SystimeMs */,
}
impl AttrType for MeasurementRangeStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "MeasurementType", attr_ty: <MeasurementTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Min", attr_ty: <i64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Max", attr_ty: <i64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "StartTimestamp", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "EndTimestamp", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "MinTimestamp", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "MaxTimestamp", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "StartSystime", attr_ty: <TypeUnsupported /* SystimeMs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0008, name: "EndSystime", attr_ty: <TypeUnsupported /* SystimeMs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0009, name: "MinSystime", attr_ty: <TypeUnsupported /* SystimeMs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000a, name: "MaxSystime", attr_ty: <TypeUnsupported /* SystimeMs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for MeasurementRangeStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.measurement_type.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.min.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.max.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.start_timestamp.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.end_timestamp.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.min_timestamp.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.max_timestamp.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.start_systime.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.end_systime.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    self.min_systime.tlv_put(matter_codec::Tag::Context(0x0009), writer);
    self.max_systime.tlv_put(matter_codec::Tag::Context(0x000a), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for MeasurementRangeStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut measurement_type: Option<MeasurementTypeEnum> = None;
    let mut min: Option<i64> = None;
    let mut max: Option<i64> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { measurement_type = Some(<MeasurementTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { min = Some(<i64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { max = Some(<i64>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      measurement_type: measurement_type.ok_or(TlvDecodeErr::MissingField("MeasurementType"))?,
      min: min.ok_or(TlvDecodeErr::MissingField("Min"))?,
      max: max.ok_or(TlvDecodeErr::MissingField("Max"))?,
      start_timestamp: TypeUnsupported,
      end_timestamp: TypeUnsupported,
      min_timestamp: TypeUnsupported,
      max_timestamp: TypeUnsupported,
      start_systime: TypeUnsupported,
      end_systime: TypeUnsupported,
      min_systime: TypeUnsupported,
      max_systime: TypeUnsupported,
    })
  }
}

}
