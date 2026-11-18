#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const ELECTRICAL_GRID_CONDITIONS: ClusterDef = ClusterDef {
  id: 0x00a0,
  name: "Electrical Grid Conditions",
  features: &[
    FeatureDef { bit: 0x0000, code: "FORE", name: "Forecasting", summary: "Forecasts upcoming" },
  ],
  attributes: &[
    attr::LocalGenerationAvailable::ATTR,
    attr::CurrentConditions::ATTR,
    attr::ForecastConditions::ATTR,
  ],
  events: &[
    event::CurrentConditionsChanged::EVENT,
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct LocalGenerationAvailable;

  impl Attr for LocalGenerationAvailable {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "LocalGenerationAvailable",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<bool>;
  }


  pub struct CurrentConditions;

  impl Attr for CurrentConditions {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "CurrentConditions",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::ElectricalGridConditionsStruct>;
  }


  pub struct ForecastConditions;

  impl Attr for ForecastConditions {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "ForecastConditions",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<ty::ElectricalGridConditionsStruct>;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct CurrentConditionsChanged {
  pub current_conditions: ty::ElectricalGridConditionsStruct,
}

impl Event for CurrentConditionsChanged {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "CurrentConditionsChanged",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "CurrentConditions", attr_ty: <ty::ElectricalGridConditionsStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for CurrentConditionsChanged {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut current_conditions: Option<ty::ElectricalGridConditionsStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { current_conditions = Some(<ty::ElectricalGridConditionsStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      current_conditions: current_conditions.ok_or(TlvDecodeErr::MissingField("CurrentConditions"))?,
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
pub enum ThreeLevelEnum {
  ///Low
  Low = 0x0000,
  ///Medium
  Medium = 0x0001,
  ///High
  High = 0x0002,
}
impl AttrType for ThreeLevelEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Low"),
  (0x0001, "Medium"),
  (0x0002, "High"),
  ]);
}
impl TlvPut for ThreeLevelEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ThreeLevelEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Low),
      0x0001 => Ok(Self::Medium),
      0x0002 => Ok(Self::High),
      _ => Err(())
    }
  }
}
impl TlvDecode for ThreeLevelEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ElectricalGridConditionsStruct {
  pub period_start: TypeUnsupported /* EpochS */,
  pub period_end: TypeUnsupported /* EpochS */,
  pub grid_carbon_intensity: i16,
  pub grid_carbon_level: ThreeLevelEnum,
  pub local_carbon_intensity: i16,
  pub local_carbon_level: ThreeLevelEnum,
}
impl AttrType for ElectricalGridConditionsStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "PeriodStart", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "PeriodEnd", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "GridCarbonIntensity", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "GridCarbonLevel", attr_ty: <ThreeLevelEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "LocalCarbonIntensity", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "LocalCarbonLevel", attr_ty: <ThreeLevelEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ElectricalGridConditionsStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.period_start.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.period_end.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.grid_carbon_intensity.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.grid_carbon_level.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.local_carbon_intensity.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.local_carbon_level.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ElectricalGridConditionsStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut grid_carbon_intensity: Option<i16> = None;
    let mut grid_carbon_level: Option<ThreeLevelEnum> = None;
    let mut local_carbon_intensity: Option<i16> = None;
    let mut local_carbon_level: Option<ThreeLevelEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { grid_carbon_intensity = Some(<i16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { grid_carbon_level = Some(<ThreeLevelEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { local_carbon_intensity = Some(<i16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { local_carbon_level = Some(<ThreeLevelEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      period_start: TypeUnsupported,
      period_end: TypeUnsupported,
      grid_carbon_intensity: grid_carbon_intensity.ok_or(TlvDecodeErr::MissingField("GridCarbonIntensity"))?,
      grid_carbon_level: grid_carbon_level.ok_or(TlvDecodeErr::MissingField("GridCarbonLevel"))?,
      local_carbon_intensity: local_carbon_intensity.ok_or(TlvDecodeErr::MissingField("LocalCarbonIntensity"))?,
      local_carbon_level: local_carbon_level.ok_or(TlvDecodeErr::MissingField("LocalCarbonLevel"))?,
    })
  }
}

}
