#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const ELECTRICAL_ENERGY_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x0091,
  name: "Electrical Energy Measurement",
  features: &[
    FeatureDef { bit: 0x0000, code: "IMPE", name: "ImportedEnergy", summary: "Measurement of energy imported by the server" },
    FeatureDef { bit: 0x0001, code: "EXPE", name: "ExportedEnergy", summary: "Measurement of energy provided by the server" },
    FeatureDef { bit: 0x0002, code: "CUME", name: "CumulativeEnergy", summary: "Measurements are cumulative" },
    FeatureDef { bit: 0x0003, code: "PERE", name: "PeriodicEnergy", summary: "Measurements are periodic" },
    FeatureDef { bit: 0x0004, code: "APPE", name: "ApparentEnergy", summary: "Measurements report apparent energy" },
    FeatureDef { bit: 0x0005, code: "REAE", name: "ReactiveEnergy", summary: "Measurements report reactive energy" },
  ],
  attributes: &[
    attr::Accuracy::ATTR,
    attr::CumulativeEnergyImported::ATTR,
    attr::CumulativeEnergyExported::ATTR,
    attr::PeriodicEnergyImported::ATTR,
    attr::PeriodicEnergyExported::ATTR,
    attr::CumulativeEnergyReset::ATTR,
  ],
  events: &[
    event::CumulativeEnergyMeasured::EVENT,
    event::PeriodicEnergyMeasured::EVENT,
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct Accuracy;

  impl Attr for Accuracy {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "Accuracy",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::MeasurementAccuracyStruct;
  }


  pub struct CumulativeEnergyImported;

  impl Attr for CumulativeEnergyImported {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "CumulativeEnergyImported",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::EnergyMeasurementStruct>;
  }


  pub struct CumulativeEnergyExported;

  impl Attr for CumulativeEnergyExported {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "CumulativeEnergyExported",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::EnergyMeasurementStruct>;
  }


  pub struct PeriodicEnergyImported;

  impl Attr for PeriodicEnergyImported {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "PeriodicEnergyImported",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::EnergyMeasurementStruct>;
  }


  pub struct PeriodicEnergyExported;

  impl Attr for PeriodicEnergyExported {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "PeriodicEnergyExported",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::EnergyMeasurementStruct>;
  }


  pub struct CumulativeEnergyReset;

  impl Attr for CumulativeEnergyReset {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "CumulativeEnergyReset",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<ty::CumulativeEnergyResetStruct>;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct CumulativeEnergyMeasured {
  pub energy_imported: ty::EnergyMeasurementStruct,
  pub energy_exported: ty::EnergyMeasurementStruct,
}

impl Event for CumulativeEnergyMeasured {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "CumulativeEnergyMeasured",
    conformance: Conformance::Feature(2),
    fields: &[
      AttrDef { id: 0x0000, name: "EnergyImported", attr_ty: <ty::EnergyMeasurementStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "EnergyExported", attr_ty: <ty::EnergyMeasurementStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for CumulativeEnergyMeasured {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut energy_imported: Option<ty::EnergyMeasurementStruct> = None;
    let mut energy_exported: Option<ty::EnergyMeasurementStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { energy_imported = Some(<ty::EnergyMeasurementStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { energy_exported = Some(<ty::EnergyMeasurementStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      energy_imported: energy_imported.ok_or(TlvDecodeErr::MissingField("EnergyImported"))?,
      energy_exported: energy_exported.ok_or(TlvDecodeErr::MissingField("EnergyExported"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct PeriodicEnergyMeasured {
  pub energy_imported: ty::EnergyMeasurementStruct,
  pub energy_exported: ty::EnergyMeasurementStruct,
}

impl Event for PeriodicEnergyMeasured {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "PeriodicEnergyMeasured",
    conformance: Conformance::Feature(3),
    fields: &[
      AttrDef { id: 0x0000, name: "EnergyImported", attr_ty: <ty::EnergyMeasurementStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "EnergyExported", attr_ty: <ty::EnergyMeasurementStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for PeriodicEnergyMeasured {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut energy_imported: Option<ty::EnergyMeasurementStruct> = None;
    let mut energy_exported: Option<ty::EnergyMeasurementStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { energy_imported = Some(<ty::EnergyMeasurementStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { energy_exported = Some(<ty::EnergyMeasurementStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      energy_imported: energy_imported.ok_or(TlvDecodeErr::MissingField("EnergyImported"))?,
      energy_exported: energy_exported.ok_or(TlvDecodeErr::MissingField("EnergyExported"))?,
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
pub struct CumulativeEnergyResetStruct {
  pub imported_reset_timestamp: TypeUnsupported /* EpochS */,
  pub exported_reset_timestamp: TypeUnsupported /* EpochS */,
  pub imported_reset_systime: TypeUnsupported /* SystimeMs */,
  pub exported_reset_systime: TypeUnsupported /* SystimeMs */,
}
impl AttrType for CumulativeEnergyResetStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ImportedResetTimestamp", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ExportedResetTimestamp", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "ImportedResetSystime", attr_ty: <TypeUnsupported /* SystimeMs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "ExportedResetSystime", attr_ty: <TypeUnsupported /* SystimeMs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for CumulativeEnergyResetStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.imported_reset_timestamp.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.exported_reset_timestamp.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.imported_reset_systime.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.exported_reset_systime.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for CumulativeEnergyResetStruct {
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
      imported_reset_timestamp: TypeUnsupported,
      exported_reset_timestamp: TypeUnsupported,
      imported_reset_systime: TypeUnsupported,
      exported_reset_systime: TypeUnsupported,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnergyMeasurementStruct {
  pub energy: EnergyMwh,
  pub start_timestamp: TypeUnsupported /* EpochS */,
  pub end_timestamp: TypeUnsupported /* EpochS */,
  pub start_systime: TypeUnsupported /* SystimeMs */,
  pub end_systime: TypeUnsupported /* SystimeMs */,
  pub apparent_energy: TypeUnsupported /* EnergyMvah */,
  pub reactive_energy: TypeUnsupported /* EnergyMvarh */,
}
impl AttrType for EnergyMeasurementStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Energy", attr_ty: <EnergyMwh>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "StartTimestamp", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "EndTimestamp", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "StartSystime", attr_ty: <TypeUnsupported /* SystimeMs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "EndSystime", attr_ty: <TypeUnsupported /* SystimeMs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "ApparentEnergy", attr_ty: <TypeUnsupported /* EnergyMvah */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "ReactiveEnergy", attr_ty: <TypeUnsupported /* EnergyMvarh */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for EnergyMeasurementStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.energy.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.start_timestamp.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.end_timestamp.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.start_systime.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.end_systime.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.apparent_energy.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.reactive_energy.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for EnergyMeasurementStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut energy: Option<EnergyMwh> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { energy = Some(<EnergyMwh>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      energy: energy.ok_or(TlvDecodeErr::MissingField("Energy"))?,
      start_timestamp: TypeUnsupported,
      end_timestamp: TypeUnsupported,
      start_systime: TypeUnsupported,
      end_systime: TypeUnsupported,
      apparent_energy: TypeUnsupported,
      reactive_energy: TypeUnsupported,
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

}
