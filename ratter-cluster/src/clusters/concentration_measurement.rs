#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const CARBON_MONOXIDE_CONCENTRATION_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x040c,
  name: "Carbon Monoxide Concentration Measurement",
  features: &[
    FeatureDef { bit: 0x0000, code: "MEA", name: "NumericMeasurement", summary: "Cluster supports numeric measurement of substance" },
    FeatureDef { bit: 0x0001, code: "LEV", name: "LevelIndication", summary: "Cluster supports basic level indication for substance using the ConcentrationLevel enum" },
    FeatureDef { bit: 0x0002, code: "MED", name: "MediumLevel", summary: "Cluster supports the Medium Concentration Level" },
    FeatureDef { bit: 0x0003, code: "CRI", name: "CriticalLevel", summary: "Cluster supports the Critical Concentration Level" },
    FeatureDef { bit: 0x0004, code: "PEA", name: "PeakMeasurement", summary: "Cluster supports peak numeric measurement of substance" },
    FeatureDef { bit: 0x0005, code: "AVG", name: "AverageMeasurement", summary: "Cluster supports average numeric measurement of substance" },
  ],
  attributes: &[
    attr::MeasuredValue::ATTR,
    attr::MinMeasuredValue::ATTR,
    attr::MaxMeasuredValue::ATTR,
    attr::PeakMeasuredValue::ATTR,
    attr::PeakMeasuredValueWindow::ATTR,
    attr::AverageMeasuredValue::ATTR,
    attr::AverageMeasuredValueWindow::ATTR,
    attr::Uncertainty::ATTR,
    attr::MeasurementUnit::ATTR,
    attr::MeasurementMedium::ATTR,
    attr::LevelValue::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub const CARBON_DIOXIDE_CONCENTRATION_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x040d,
  name: "Carbon Dioxide Concentration Measurement",
  features: &[
    FeatureDef { bit: 0x0000, code: "MEA", name: "NumericMeasurement", summary: "Cluster supports numeric measurement of substance" },
    FeatureDef { bit: 0x0001, code: "LEV", name: "LevelIndication", summary: "Cluster supports basic level indication for substance using the ConcentrationLevel enum" },
    FeatureDef { bit: 0x0002, code: "MED", name: "MediumLevel", summary: "Cluster supports the Medium Concentration Level" },
    FeatureDef { bit: 0x0003, code: "CRI", name: "CriticalLevel", summary: "Cluster supports the Critical Concentration Level" },
    FeatureDef { bit: 0x0004, code: "PEA", name: "PeakMeasurement", summary: "Cluster supports peak numeric measurement of substance" },
    FeatureDef { bit: 0x0005, code: "AVG", name: "AverageMeasurement", summary: "Cluster supports average numeric measurement of substance" },
  ],
  attributes: &[
    attr::MeasuredValue::ATTR,
    attr::MinMeasuredValue::ATTR,
    attr::MaxMeasuredValue::ATTR,
    attr::PeakMeasuredValue::ATTR,
    attr::PeakMeasuredValueWindow::ATTR,
    attr::AverageMeasuredValue::ATTR,
    attr::AverageMeasuredValueWindow::ATTR,
    attr::Uncertainty::ATTR,
    attr::MeasurementUnit::ATTR,
    attr::MeasurementMedium::ATTR,
    attr::LevelValue::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub const NITROGEN_DIOXIDE_CONCENTRATION_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x0413,
  name: "Nitrogen Dioxide Concentration Measurement",
  features: &[
    FeatureDef { bit: 0x0000, code: "MEA", name: "NumericMeasurement", summary: "Cluster supports numeric measurement of substance" },
    FeatureDef { bit: 0x0001, code: "LEV", name: "LevelIndication", summary: "Cluster supports basic level indication for substance using the ConcentrationLevel enum" },
    FeatureDef { bit: 0x0002, code: "MED", name: "MediumLevel", summary: "Cluster supports the Medium Concentration Level" },
    FeatureDef { bit: 0x0003, code: "CRI", name: "CriticalLevel", summary: "Cluster supports the Critical Concentration Level" },
    FeatureDef { bit: 0x0004, code: "PEA", name: "PeakMeasurement", summary: "Cluster supports peak numeric measurement of substance" },
    FeatureDef { bit: 0x0005, code: "AVG", name: "AverageMeasurement", summary: "Cluster supports average numeric measurement of substance" },
  ],
  attributes: &[
    attr::MeasuredValue::ATTR,
    attr::MinMeasuredValue::ATTR,
    attr::MaxMeasuredValue::ATTR,
    attr::PeakMeasuredValue::ATTR,
    attr::PeakMeasuredValueWindow::ATTR,
    attr::AverageMeasuredValue::ATTR,
    attr::AverageMeasuredValueWindow::ATTR,
    attr::Uncertainty::ATTR,
    attr::MeasurementUnit::ATTR,
    attr::MeasurementMedium::ATTR,
    attr::LevelValue::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub const OZONE_CONCENTRATION_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x0415,
  name: "Ozone Concentration Measurement",
  features: &[
    FeatureDef { bit: 0x0000, code: "MEA", name: "NumericMeasurement", summary: "Cluster supports numeric measurement of substance" },
    FeatureDef { bit: 0x0001, code: "LEV", name: "LevelIndication", summary: "Cluster supports basic level indication for substance using the ConcentrationLevel enum" },
    FeatureDef { bit: 0x0002, code: "MED", name: "MediumLevel", summary: "Cluster supports the Medium Concentration Level" },
    FeatureDef { bit: 0x0003, code: "CRI", name: "CriticalLevel", summary: "Cluster supports the Critical Concentration Level" },
    FeatureDef { bit: 0x0004, code: "PEA", name: "PeakMeasurement", summary: "Cluster supports peak numeric measurement of substance" },
    FeatureDef { bit: 0x0005, code: "AVG", name: "AverageMeasurement", summary: "Cluster supports average numeric measurement of substance" },
  ],
  attributes: &[
    attr::MeasuredValue::ATTR,
    attr::MinMeasuredValue::ATTR,
    attr::MaxMeasuredValue::ATTR,
    attr::PeakMeasuredValue::ATTR,
    attr::PeakMeasuredValueWindow::ATTR,
    attr::AverageMeasuredValue::ATTR,
    attr::AverageMeasuredValueWindow::ATTR,
    attr::Uncertainty::ATTR,
    attr::MeasurementUnit::ATTR,
    attr::MeasurementMedium::ATTR,
    attr::LevelValue::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub const PM2_5_CONCENTRATION_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x042a,
  name: "PM2.5 Concentration Measurement",
  features: &[
    FeatureDef { bit: 0x0000, code: "MEA", name: "NumericMeasurement", summary: "Cluster supports numeric measurement of substance" },
    FeatureDef { bit: 0x0001, code: "LEV", name: "LevelIndication", summary: "Cluster supports basic level indication for substance using the ConcentrationLevel enum" },
    FeatureDef { bit: 0x0002, code: "MED", name: "MediumLevel", summary: "Cluster supports the Medium Concentration Level" },
    FeatureDef { bit: 0x0003, code: "CRI", name: "CriticalLevel", summary: "Cluster supports the Critical Concentration Level" },
    FeatureDef { bit: 0x0004, code: "PEA", name: "PeakMeasurement", summary: "Cluster supports peak numeric measurement of substance" },
    FeatureDef { bit: 0x0005, code: "AVG", name: "AverageMeasurement", summary: "Cluster supports average numeric measurement of substance" },
  ],
  attributes: &[
    attr::MeasuredValue::ATTR,
    attr::MinMeasuredValue::ATTR,
    attr::MaxMeasuredValue::ATTR,
    attr::PeakMeasuredValue::ATTR,
    attr::PeakMeasuredValueWindow::ATTR,
    attr::AverageMeasuredValue::ATTR,
    attr::AverageMeasuredValueWindow::ATTR,
    attr::Uncertainty::ATTR,
    attr::MeasurementUnit::ATTR,
    attr::MeasurementMedium::ATTR,
    attr::LevelValue::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub const FORMALDEHYDE_CONCENTRATION_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x042b,
  name: "Formaldehyde Concentration Measurement",
  features: &[
    FeatureDef { bit: 0x0000, code: "MEA", name: "NumericMeasurement", summary: "Cluster supports numeric measurement of substance" },
    FeatureDef { bit: 0x0001, code: "LEV", name: "LevelIndication", summary: "Cluster supports basic level indication for substance using the ConcentrationLevel enum" },
    FeatureDef { bit: 0x0002, code: "MED", name: "MediumLevel", summary: "Cluster supports the Medium Concentration Level" },
    FeatureDef { bit: 0x0003, code: "CRI", name: "CriticalLevel", summary: "Cluster supports the Critical Concentration Level" },
    FeatureDef { bit: 0x0004, code: "PEA", name: "PeakMeasurement", summary: "Cluster supports peak numeric measurement of substance" },
    FeatureDef { bit: 0x0005, code: "AVG", name: "AverageMeasurement", summary: "Cluster supports average numeric measurement of substance" },
  ],
  attributes: &[
    attr::MeasuredValue::ATTR,
    attr::MinMeasuredValue::ATTR,
    attr::MaxMeasuredValue::ATTR,
    attr::PeakMeasuredValue::ATTR,
    attr::PeakMeasuredValueWindow::ATTR,
    attr::AverageMeasuredValue::ATTR,
    attr::AverageMeasuredValueWindow::ATTR,
    attr::Uncertainty::ATTR,
    attr::MeasurementUnit::ATTR,
    attr::MeasurementMedium::ATTR,
    attr::LevelValue::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub const PM1_CONCENTRATION_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x042c,
  name: "PM1 Concentration Measurement",
  features: &[
    FeatureDef { bit: 0x0000, code: "MEA", name: "NumericMeasurement", summary: "Cluster supports numeric measurement of substance" },
    FeatureDef { bit: 0x0001, code: "LEV", name: "LevelIndication", summary: "Cluster supports basic level indication for substance using the ConcentrationLevel enum" },
    FeatureDef { bit: 0x0002, code: "MED", name: "MediumLevel", summary: "Cluster supports the Medium Concentration Level" },
    FeatureDef { bit: 0x0003, code: "CRI", name: "CriticalLevel", summary: "Cluster supports the Critical Concentration Level" },
    FeatureDef { bit: 0x0004, code: "PEA", name: "PeakMeasurement", summary: "Cluster supports peak numeric measurement of substance" },
    FeatureDef { bit: 0x0005, code: "AVG", name: "AverageMeasurement", summary: "Cluster supports average numeric measurement of substance" },
  ],
  attributes: &[
    attr::MeasuredValue::ATTR,
    attr::MinMeasuredValue::ATTR,
    attr::MaxMeasuredValue::ATTR,
    attr::PeakMeasuredValue::ATTR,
    attr::PeakMeasuredValueWindow::ATTR,
    attr::AverageMeasuredValue::ATTR,
    attr::AverageMeasuredValueWindow::ATTR,
    attr::Uncertainty::ATTR,
    attr::MeasurementUnit::ATTR,
    attr::MeasurementMedium::ATTR,
    attr::LevelValue::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub const PM10_CONCENTRATION_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x042d,
  name: "PM10 Concentration Measurement",
  features: &[
    FeatureDef { bit: 0x0000, code: "MEA", name: "NumericMeasurement", summary: "Cluster supports numeric measurement of substance" },
    FeatureDef { bit: 0x0001, code: "LEV", name: "LevelIndication", summary: "Cluster supports basic level indication for substance using the ConcentrationLevel enum" },
    FeatureDef { bit: 0x0002, code: "MED", name: "MediumLevel", summary: "Cluster supports the Medium Concentration Level" },
    FeatureDef { bit: 0x0003, code: "CRI", name: "CriticalLevel", summary: "Cluster supports the Critical Concentration Level" },
    FeatureDef { bit: 0x0004, code: "PEA", name: "PeakMeasurement", summary: "Cluster supports peak numeric measurement of substance" },
    FeatureDef { bit: 0x0005, code: "AVG", name: "AverageMeasurement", summary: "Cluster supports average numeric measurement of substance" },
  ],
  attributes: &[
    attr::MeasuredValue::ATTR,
    attr::MinMeasuredValue::ATTR,
    attr::MaxMeasuredValue::ATTR,
    attr::PeakMeasuredValue::ATTR,
    attr::PeakMeasuredValueWindow::ATTR,
    attr::AverageMeasuredValue::ATTR,
    attr::AverageMeasuredValueWindow::ATTR,
    attr::Uncertainty::ATTR,
    attr::MeasurementUnit::ATTR,
    attr::MeasurementMedium::ATTR,
    attr::LevelValue::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub const TOTAL_VOLATILE_ORGANIC_COMPOUNDS_CONCENTRATION_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x042e,
  name: "Total Volatile Organic Compounds Concentration Measurement",
  features: &[
    FeatureDef { bit: 0x0000, code: "MEA", name: "NumericMeasurement", summary: "Cluster supports numeric measurement of substance" },
    FeatureDef { bit: 0x0001, code: "LEV", name: "LevelIndication", summary: "Cluster supports basic level indication for substance using the ConcentrationLevel enum" },
    FeatureDef { bit: 0x0002, code: "MED", name: "MediumLevel", summary: "Cluster supports the Medium Concentration Level" },
    FeatureDef { bit: 0x0003, code: "CRI", name: "CriticalLevel", summary: "Cluster supports the Critical Concentration Level" },
    FeatureDef { bit: 0x0004, code: "PEA", name: "PeakMeasurement", summary: "Cluster supports peak numeric measurement of substance" },
    FeatureDef { bit: 0x0005, code: "AVG", name: "AverageMeasurement", summary: "Cluster supports average numeric measurement of substance" },
  ],
  attributes: &[
    attr::MeasuredValue::ATTR,
    attr::MinMeasuredValue::ATTR,
    attr::MaxMeasuredValue::ATTR,
    attr::PeakMeasuredValue::ATTR,
    attr::PeakMeasuredValueWindow::ATTR,
    attr::AverageMeasuredValue::ATTR,
    attr::AverageMeasuredValueWindow::ATTR,
    attr::Uncertainty::ATTR,
    attr::MeasurementUnit::ATTR,
    attr::MeasurementMedium::ATTR,
    attr::LevelValue::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub const RADON_CONCENTRATION_MEASUREMENT: ClusterDef = ClusterDef {
  id: 0x042f,
  name: "Radon Concentration Measurement",
  features: &[
    FeatureDef { bit: 0x0000, code: "MEA", name: "NumericMeasurement", summary: "Cluster supports numeric measurement of substance" },
    FeatureDef { bit: 0x0001, code: "LEV", name: "LevelIndication", summary: "Cluster supports basic level indication for substance using the ConcentrationLevel enum" },
    FeatureDef { bit: 0x0002, code: "MED", name: "MediumLevel", summary: "Cluster supports the Medium Concentration Level" },
    FeatureDef { bit: 0x0003, code: "CRI", name: "CriticalLevel", summary: "Cluster supports the Critical Concentration Level" },
    FeatureDef { bit: 0x0004, code: "PEA", name: "PeakMeasurement", summary: "Cluster supports peak numeric measurement of substance" },
    FeatureDef { bit: 0x0005, code: "AVG", name: "AverageMeasurement", summary: "Cluster supports average numeric measurement of substance" },
  ],
  attributes: &[
    attr::MeasuredValue::ATTR,
    attr::MinMeasuredValue::ATTR,
    attr::MaxMeasuredValue::ATTR,
    attr::PeakMeasuredValue::ATTR,
    attr::PeakMeasuredValueWindow::ATTR,
    attr::AverageMeasuredValue::ATTR,
    attr::AverageMeasuredValueWindow::ATTR,
    attr::Uncertainty::ATTR,
    attr::MeasurementUnit::ATTR,
    attr::MeasurementMedium::ATTR,
    attr::LevelValue::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct MeasuredValue;

  impl Attr for MeasuredValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "MeasuredValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<f32>;
  }


  pub struct MinMeasuredValue;

  impl Attr for MinMeasuredValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "MinMeasuredValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<f32>;
  }


  pub struct MaxMeasuredValue;

  impl Attr for MaxMeasuredValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "MaxMeasuredValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<f32>;
  }


  pub struct PeakMeasuredValue;

  impl Attr for PeakMeasuredValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "PeakMeasuredValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = Nullable<f32>;
  }


  pub struct PeakMeasuredValueWindow;

  impl Attr for PeakMeasuredValueWindow {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "PeakMeasuredValueWindow",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = TypeUnsupported /* ElapsedS */;
  }


  pub struct AverageMeasuredValue;

  impl Attr for AverageMeasuredValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "AverageMeasuredValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(5),
    };
    type Value = Nullable<f32>;
  }


  pub struct AverageMeasuredValueWindow;

  impl Attr for AverageMeasuredValueWindow {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "AverageMeasuredValueWindow",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(5),
    };
    type Value = TypeUnsupported /* ElapsedS */;
  }


  pub struct Uncertainty;

  impl Attr for Uncertainty {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "Uncertainty",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = f32;
  }


  pub struct MeasurementUnit;

  impl Attr for MeasurementUnit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "MeasurementUnit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = ty::MeasurementUnitEnum;
  }


  pub struct MeasurementMedium;

  impl Attr for MeasurementMedium {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "MeasurementMedium",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::MeasurementMediumEnum;
  }


  pub struct LevelValue;

  impl Attr for LevelValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "LevelValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = ty::LevelValueEnum;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum LevelValueEnum {
  ///The level is Unknown
  Unknown = 0x0000,
  ///The level is considered Low
  Low = 0x0001,
  ///The level is considered Medium
  Medium = 0x0002,
  ///The level is considered High
  High = 0x0003,
  ///The level is considered Critical
  Critical = 0x0004,
}
impl AttrType for LevelValueEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unknown"),
  (0x0001, "Low"),
  (0x0002, "Medium"),
  (0x0003, "High"),
  (0x0004, "Critical"),
  ]);
}
impl TlvPut for LevelValueEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for LevelValueEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unknown),
      0x0001 => Ok(Self::Low),
      0x0002 => Ok(Self::Medium),
      0x0003 => Ok(Self::High),
      0x0004 => Ok(Self::Critical),
      _ => Err(())
    }
  }
}
impl TlvDecode for LevelValueEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum MeasurementMediumEnum {
  ///The measurement is being made in Air
  Air = 0x0000,
  ///The measurement is being made in Water
  Water = 0x0001,
  ///The measurement is being made in Soil
  Soil = 0x0002,
}
impl AttrType for MeasurementMediumEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Air"),
  (0x0001, "Water"),
  (0x0002, "Soil"),
  ]);
}
impl TlvPut for MeasurementMediumEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for MeasurementMediumEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Air),
      0x0001 => Ok(Self::Water),
      0x0002 => Ok(Self::Soil),
      _ => Err(())
    }
  }
}
impl TlvDecode for MeasurementMediumEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum MeasurementUnitEnum {
  ///Parts per Million (10)
  PPM = 0x0000,
  ///Parts per Billion (10)
  PPB = 0x0001,
  ///Parts per Trillion (10)
  PPT = 0x0002,
  ///Milligram per m
  MGM3 = 0x0003,
  ///Microgram per m
  UGM3 = 0x0004,
  ///Nanogram per m
  NGM3 = 0x0005,
  ///Particles per m
  PM3 = 0x0006,
  ///Becquerel per m
  BQM3 = 0x0007,
}
impl AttrType for MeasurementUnitEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "PPM"),
  (0x0001, "PPB"),
  (0x0002, "PPT"),
  (0x0003, "MGM3"),
  (0x0004, "UGM3"),
  (0x0005, "NGM3"),
  (0x0006, "PM3"),
  (0x0007, "BQM3"),
  ]);
}
impl TlvPut for MeasurementUnitEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for MeasurementUnitEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::PPM),
      0x0001 => Ok(Self::PPB),
      0x0002 => Ok(Self::PPT),
      0x0003 => Ok(Self::MGM3),
      0x0004 => Ok(Self::UGM3),
      0x0005 => Ok(Self::NGM3),
      0x0006 => Ok(Self::PM3),
      0x0007 => Ok(Self::BQM3),
      _ => Err(())
    }
  }
}
impl TlvDecode for MeasurementUnitEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
