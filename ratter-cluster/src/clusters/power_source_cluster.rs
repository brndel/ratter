#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const POWER_SOURCE: ClusterDef = ClusterDef {
  id: 0x002f,
  name: "Power Source",
  features: &[
    FeatureDef { bit: 0x0000, code: "WIRED", name: "Wired", summary: "A wired power source" },
    FeatureDef { bit: 0x0001, code: "BAT", name: "Battery", summary: "A battery power source" },
    FeatureDef { bit: 0x0002, code: "RECHG", name: "Rechargeable", summary: "A rechargeable battery power source" },
    FeatureDef { bit: 0x0003, code: "REPLC", name: "Replaceable", summary: "A replaceable battery power source" },
  ],
  attributes: &[
    attr::Status::ATTR,
    attr::Order::ATTR,
    attr::Description::ATTR,
    attr::WiredAssessedInputVoltage::ATTR,
    attr::WiredAssessedInputFrequency::ATTR,
    attr::WiredCurrentType::ATTR,
    attr::WiredAssessedCurrent::ATTR,
    attr::WiredNominalVoltage::ATTR,
    attr::WiredMaximumCurrent::ATTR,
    attr::WiredPresent::ATTR,
    attr::ActiveWiredFaults::ATTR,
    attr::BatVoltage::ATTR,
    attr::BatPercentRemaining::ATTR,
    attr::BatTimeRemaining::ATTR,
    attr::BatChargeLevel::ATTR,
    attr::BatReplacementNeeded::ATTR,
    attr::BatReplaceability::ATTR,
    attr::BatPresent::ATTR,
    attr::ActiveBatFaults::ATTR,
    attr::BatReplacementDescription::ATTR,
    attr::BatCommonDesignation::ATTR,
    attr::BatANSIDesignation::ATTR,
    attr::BatIECDesignation::ATTR,
    attr::BatApprovedChemistry::ATTR,
    attr::BatCapacity::ATTR,
    attr::BatQuantity::ATTR,
    attr::BatChargeState::ATTR,
    attr::BatTimeToFullCharge::ATTR,
    attr::BatFunctionalWhileCharging::ATTR,
    attr::BatChargingCurrent::ATTR,
    attr::ActiveBatChargeFaults::ATTR,
    attr::EndpointList::ATTR,
  ],
  events: &[
    event::WiredFaultChange::EVENT,
    event::BatFaultChange::EVENT,
    event::BatChargeFaultChange::EVENT,
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct Status;

  impl Attr for Status {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "Status",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::PowerSourceStatusEnum;
  }


  pub struct Order;

  impl Attr for Order {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "Order",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct Description;

  impl Attr for Description {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "Description",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = String;
  }


  pub struct WiredAssessedInputVoltage;

  impl Attr for WiredAssessedInputVoltage {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "WiredAssessedInputVoltage",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u32>;
  }


  pub struct WiredAssessedInputFrequency;

  impl Attr for WiredAssessedInputFrequency {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "WiredAssessedInputFrequency",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u16>;
  }


  pub struct WiredCurrentType;

  impl Attr for WiredCurrentType {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "WiredCurrentType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = ty::WiredCurrentTypeEnum;
  }


  pub struct WiredAssessedCurrent;

  impl Attr for WiredAssessedCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "WiredAssessedCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u32>;
  }


  pub struct WiredNominalVoltage;

  impl Attr for WiredNominalVoltage {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "WiredNominalVoltage",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct WiredMaximumCurrent;

  impl Attr for WiredMaximumCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "WiredMaximumCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct WiredPresent;

  impl Attr for WiredPresent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "WiredPresent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct ActiveWiredFaults;

  impl Attr for ActiveWiredFaults {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "ActiveWiredFaults",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Vec<ty::WiredFaultEnum>;
  }


  pub struct BatVoltage;

  impl Attr for BatVoltage {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "BatVoltage",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u32>;
  }


  pub struct BatPercentRemaining;

  impl Attr for BatPercentRemaining {
    const ATTR: AttrDef = AttrDef {
      id: 0x000c,
      name: "BatPercentRemaining",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u8>;
  }


  pub struct BatTimeRemaining;

  impl Attr for BatTimeRemaining {
    const ATTR: AttrDef = AttrDef {
      id: 0x000d,
      name: "BatTimeRemaining",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u32>;
  }


  pub struct BatChargeLevel;

  impl Attr for BatChargeLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x000e,
      name: "BatChargeLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = ty::BatChargeLevelEnum;
  }


  pub struct BatReplacementNeeded;

  impl Attr for BatReplacementNeeded {
    const ATTR: AttrDef = AttrDef {
      id: 0x000f,
      name: "BatReplacementNeeded",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = bool;
  }


  pub struct BatReplaceability;

  impl Attr for BatReplaceability {
    const ATTR: AttrDef = AttrDef {
      id: 0x0010,
      name: "BatReplaceability",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = ty::BatReplaceabilityEnum;
  }


  pub struct BatPresent;

  impl Attr for BatPresent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0011,
      name: "BatPresent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct ActiveBatFaults;

  impl Attr for ActiveBatFaults {
    const ATTR: AttrDef = AttrDef {
      id: 0x0012,
      name: "ActiveBatFaults",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Vec<ty::BatFaultEnum>;
  }


  pub struct BatReplacementDescription;

  impl Attr for BatReplacementDescription {
    const ATTR: AttrDef = AttrDef {
      id: 0x0013,
      name: "BatReplacementDescription",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = String;
  }


  pub struct BatCommonDesignation;

  impl Attr for BatCommonDesignation {
    const ATTR: AttrDef = AttrDef {
      id: 0x0014,
      name: "BatCommonDesignation",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::BatCommonDesignationEnum;
  }


  pub struct BatANSIDesignation;

  impl Attr for BatANSIDesignation {
    const ATTR: AttrDef = AttrDef {
      id: 0x0015,
      name: "BatANSIDesignation",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = String;
  }


  pub struct BatIECDesignation;

  impl Attr for BatIECDesignation {
    const ATTR: AttrDef = AttrDef {
      id: 0x0016,
      name: "BatIECDesignation",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = String;
  }


  pub struct BatApprovedChemistry;

  impl Attr for BatApprovedChemistry {
    const ATTR: AttrDef = AttrDef {
      id: 0x0017,
      name: "BatApprovedChemistry",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::BatApprovedChemistryEnum;
  }


  pub struct BatCapacity;

  impl Attr for BatCapacity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0018,
      name: "BatCapacity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct BatQuantity;

  impl Attr for BatQuantity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0019,
      name: "BatQuantity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = u8;
  }


  pub struct BatChargeState;

  impl Attr for BatChargeState {
    const ATTR: AttrDef = AttrDef {
      id: 0x001a,
      name: "BatChargeState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = ty::BatChargeStateEnum;
  }


  pub struct BatTimeToFullCharge;

  impl Attr for BatTimeToFullCharge {
    const ATTR: AttrDef = AttrDef {
      id: 0x001b,
      name: "BatTimeToFullCharge",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u32>;
  }


  pub struct BatFunctionalWhileCharging;

  impl Attr for BatFunctionalWhileCharging {
    const ATTR: AttrDef = AttrDef {
      id: 0x001c,
      name: "BatFunctionalWhileCharging",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = bool;
  }


  pub struct BatChargingCurrent;

  impl Attr for BatChargingCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x001d,
      name: "BatChargingCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u32>;
  }


  pub struct ActiveBatChargeFaults;

  impl Attr for ActiveBatChargeFaults {
    const ATTR: AttrDef = AttrDef {
      id: 0x001e,
      name: "ActiveBatChargeFaults",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Vec<ty::BatChargeFaultEnum>;
  }


  pub struct EndpointList;

  impl Attr for EndpointList {
    const ATTR: AttrDef = AttrDef {
      id: 0x001f,
      name: "EndpointList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<TypeUnsupported /* EndpointNo */>;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct WiredFaultChange {
  pub current: Vec<ty::WiredFaultEnum>,
  pub previous: Vec<ty::WiredFaultEnum>,
}

impl Event for WiredFaultChange {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "WiredFaultChange",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "Current", attr_ty: <Vec<ty::WiredFaultEnum>>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Previous", attr_ty: <Vec<ty::WiredFaultEnum>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for WiredFaultChange {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut current: Option<Vec<ty::WiredFaultEnum>> = None;
    let mut previous: Option<Vec<ty::WiredFaultEnum>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { current = Some(<Vec<ty::WiredFaultEnum>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { previous = Some(<Vec<ty::WiredFaultEnum>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      current: current.ok_or(TlvDecodeErr::MissingField("Current"))?,
      previous: previous.ok_or(TlvDecodeErr::MissingField("Previous"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct BatFaultChange {
  pub current: Vec<ty::BatFaultEnum>,
  pub previous: Vec<ty::BatFaultEnum>,
}

impl Event for BatFaultChange {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "BatFaultChange",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "Current", attr_ty: <Vec<ty::BatFaultEnum>>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Previous", attr_ty: <Vec<ty::BatFaultEnum>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for BatFaultChange {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut current: Option<Vec<ty::BatFaultEnum>> = None;
    let mut previous: Option<Vec<ty::BatFaultEnum>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { current = Some(<Vec<ty::BatFaultEnum>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { previous = Some(<Vec<ty::BatFaultEnum>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      current: current.ok_or(TlvDecodeErr::MissingField("Current"))?,
      previous: previous.ok_or(TlvDecodeErr::MissingField("Previous"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct BatChargeFaultChange {
  pub current: Vec<ty::BatChargeFaultEnum>,
  pub previous: Vec<ty::BatChargeFaultEnum>,
}

impl Event for BatChargeFaultChange {
  const EVENT: EventDef = EventDef {
    id: 0x0002,
    name: "BatChargeFaultChange",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "Current", attr_ty: <Vec<ty::BatChargeFaultEnum>>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Previous", attr_ty: <Vec<ty::BatChargeFaultEnum>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for BatChargeFaultChange {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut current: Option<Vec<ty::BatChargeFaultEnum>> = None;
    let mut previous: Option<Vec<ty::BatChargeFaultEnum>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { current = Some(<Vec<ty::BatChargeFaultEnum>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { previous = Some(<Vec<ty::BatChargeFaultEnum>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      current: current.ok_or(TlvDecodeErr::MissingField("Current"))?,
      previous: previous.ok_or(TlvDecodeErr::MissingField("Previous"))?,
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
pub enum BatApprovedChemistryEnum {
  ///Cell chemistry is unspecified or unknown
  Unspecified = 0x0000,
  ///Cell chemistry is alkaline
  Alkaline = 0x0001,
  ///Cell chemistry is lithium carbon fluoride
  LithiumCarbonFluoride = 0x0002,
  ///Cell chemistry is lithium chromium oxide
  LithiumChromiumOxide = 0x0003,
  ///Cell chemistry is lithium copper oxide
  LithiumCopperOxide = 0x0004,
  ///Cell chemistry is lithium iron disulfide
  LithiumIronDisulfide = 0x0005,
  ///Cell chemistry is lithium manganese dioxide
  LithiumManganeseDioxide = 0x0006,
  ///Cell chemistry is lithium thionyl chloride
  LithiumThionylChloride = 0x0007,
  ///Cell chemistry is magnesium
  Magnesium = 0x0008,
  ///Cell chemistry is mercury oxide
  MercuryOxide = 0x0009,
  ///Cell chemistry is nickel oxyhydride
  NickelOxyhydride = 0x000a,
  ///Cell chemistry is silver oxide
  SilverOxide = 0x000b,
  ///Cell chemistry is zinc air
  ZincAir = 0x000c,
  ///Cell chemistry is zinc carbon
  ZincCarbon = 0x000d,
  ///Cell chemistry is zinc chloride
  ZincChloride = 0x000e,
  ///Cell chemistry is zinc manganese dioxide
  ZincManganeseDioxide = 0x000f,
  ///Cell chemistry is lead acid
  LeadAcid = 0x0010,
  ///Cell chemistry is lithium cobalt oxide
  LithiumCobaltOxide = 0x0011,
  ///Cell chemistry is lithium ion
  LithiumIon = 0x0012,
  ///Cell chemistry is lithium ion polymer
  LithiumIonPolymer = 0x0013,
  ///Cell chemistry is lithium iron phosphate
  LithiumIronPhosphate = 0x0014,
  ///Cell chemistry is lithium sulfur
  LithiumSulfur = 0x0015,
  ///Cell chemistry is lithium titanate
  LithiumTitanate = 0x0016,
  ///Cell chemistry is nickel cadmium
  NickelCadmium = 0x0017,
  ///Cell chemistry is nickel hydrogen
  NickelHydrogen = 0x0018,
  ///Cell chemistry is nickel iron
  NickelIron = 0x0019,
  ///Cell chemistry is nickel metal hydride
  NickelMetalHydride = 0x001a,
  ///Cell chemistry is nickel zinc
  NickelZinc = 0x001b,
  ///Cell chemistry is silver zinc
  SilverZinc = 0x001c,
  ///Cell chemistry is sodium ion
  SodiumIon = 0x001d,
  ///Cell chemistry is sodium sulfur
  SodiumSulfur = 0x001e,
  ///Cell chemistry is zinc bromide
  ZincBromide = 0x001f,
  ///Cell chemistry is zinc cerium
  ZincCerium = 0x0020,
}
impl AttrType for BatApprovedChemistryEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "Alkaline"),
  (0x0002, "LithiumCarbonFluoride"),
  (0x0003, "LithiumChromiumOxide"),
  (0x0004, "LithiumCopperOxide"),
  (0x0005, "LithiumIronDisulfide"),
  (0x0006, "LithiumManganeseDioxide"),
  (0x0007, "LithiumThionylChloride"),
  (0x0008, "Magnesium"),
  (0x0009, "MercuryOxide"),
  (0x000a, "NickelOxyhydride"),
  (0x000b, "SilverOxide"),
  (0x000c, "ZincAir"),
  (0x000d, "ZincCarbon"),
  (0x000e, "ZincChloride"),
  (0x000f, "ZincManganeseDioxide"),
  (0x0010, "LeadAcid"),
  (0x0011, "LithiumCobaltOxide"),
  (0x0012, "LithiumIon"),
  (0x0013, "LithiumIonPolymer"),
  (0x0014, "LithiumIronPhosphate"),
  (0x0015, "LithiumSulfur"),
  (0x0016, "LithiumTitanate"),
  (0x0017, "NickelCadmium"),
  (0x0018, "NickelHydrogen"),
  (0x0019, "NickelIron"),
  (0x001a, "NickelMetalHydride"),
  (0x001b, "NickelZinc"),
  (0x001c, "SilverZinc"),
  (0x001d, "SodiumIon"),
  (0x001e, "SodiumSulfur"),
  (0x001f, "ZincBromide"),
  (0x0020, "ZincCerium"),
  ]);
}
impl TlvPut for BatApprovedChemistryEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for BatApprovedChemistryEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unspecified),
      0x0001 => Ok(Self::Alkaline),
      0x0002 => Ok(Self::LithiumCarbonFluoride),
      0x0003 => Ok(Self::LithiumChromiumOxide),
      0x0004 => Ok(Self::LithiumCopperOxide),
      0x0005 => Ok(Self::LithiumIronDisulfide),
      0x0006 => Ok(Self::LithiumManganeseDioxide),
      0x0007 => Ok(Self::LithiumThionylChloride),
      0x0008 => Ok(Self::Magnesium),
      0x0009 => Ok(Self::MercuryOxide),
      0x000a => Ok(Self::NickelOxyhydride),
      0x000b => Ok(Self::SilverOxide),
      0x000c => Ok(Self::ZincAir),
      0x000d => Ok(Self::ZincCarbon),
      0x000e => Ok(Self::ZincChloride),
      0x000f => Ok(Self::ZincManganeseDioxide),
      0x0010 => Ok(Self::LeadAcid),
      0x0011 => Ok(Self::LithiumCobaltOxide),
      0x0012 => Ok(Self::LithiumIon),
      0x0013 => Ok(Self::LithiumIonPolymer),
      0x0014 => Ok(Self::LithiumIronPhosphate),
      0x0015 => Ok(Self::LithiumSulfur),
      0x0016 => Ok(Self::LithiumTitanate),
      0x0017 => Ok(Self::NickelCadmium),
      0x0018 => Ok(Self::NickelHydrogen),
      0x0019 => Ok(Self::NickelIron),
      0x001a => Ok(Self::NickelMetalHydride),
      0x001b => Ok(Self::NickelZinc),
      0x001c => Ok(Self::SilverZinc),
      0x001d => Ok(Self::SodiumIon),
      0x001e => Ok(Self::SodiumSulfur),
      0x001f => Ok(Self::ZincBromide),
      0x0020 => Ok(Self::ZincCerium),
      _ => Err(())
    }
  }
}
impl TlvDecode for BatApprovedChemistryEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum BatChargeFaultEnum {
  ///The Node detects an unspecified fault on this battery source.
  Unspecified = 0x0000,
  ///The Node detects the ambient temperature is above the nominal range for this battery source.
  AmbientTooHot = 0x0001,
  ///The Node detects the ambient temperature is below the nominal range for this battery source.
  AmbientTooCold = 0x0002,
  ///The Node detects the temperature of this battery source is above the nominal range.
  BatteryTooHot = 0x0003,
  ///The Node detects the temperature of this battery source is below the nominal range.
  BatteryTooCold = 0x0004,
  ///The Node detects this battery source is not present.
  BatteryAbsent = 0x0005,
  ///The Node detects this battery source is over voltage.
  BatteryOverVoltage = 0x0006,
  ///The Node detects this battery source is under voltage.
  BatteryUnderVoltage = 0x0007,
  ///The Node detects the charger for this battery source is over voltage.
  ChargerOverVoltage = 0x0008,
  ///The Node detects the charger for this battery source is under voltage.
  ChargerUnderVoltage = 0x0009,
  ///The Node detects a charging safety timeout for this battery source.
  SafetyTimeout = 0x000a,
}
impl AttrType for BatChargeFaultEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "AmbientTooHot"),
  (0x0002, "AmbientTooCold"),
  (0x0003, "BatteryTooHot"),
  (0x0004, "BatteryTooCold"),
  (0x0005, "BatteryAbsent"),
  (0x0006, "BatteryOverVoltage"),
  (0x0007, "BatteryUnderVoltage"),
  (0x0008, "ChargerOverVoltage"),
  (0x0009, "ChargerUnderVoltage"),
  (0x000a, "SafetyTimeout"),
  ]);
}
impl TlvPut for BatChargeFaultEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for BatChargeFaultEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unspecified),
      0x0001 => Ok(Self::AmbientTooHot),
      0x0002 => Ok(Self::AmbientTooCold),
      0x0003 => Ok(Self::BatteryTooHot),
      0x0004 => Ok(Self::BatteryTooCold),
      0x0005 => Ok(Self::BatteryAbsent),
      0x0006 => Ok(Self::BatteryOverVoltage),
      0x0007 => Ok(Self::BatteryUnderVoltage),
      0x0008 => Ok(Self::ChargerOverVoltage),
      0x0009 => Ok(Self::ChargerUnderVoltage),
      0x000a => Ok(Self::SafetyTimeout),
      _ => Err(())
    }
  }
}
impl TlvDecode for BatChargeFaultEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum BatChargeLevelEnum {
  ///Charge level is nominal
  OK = 0x0000,
  ///Charge level is low, intervention may soon be required.
  Warning = 0x0001,
  ///Charge level is critical, immediate intervention is required
  Critical = 0x0002,
}
impl AttrType for BatChargeLevelEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "OK"),
  (0x0001, "Warning"),
  (0x0002, "Critical"),
  ]);
}
impl TlvPut for BatChargeLevelEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for BatChargeLevelEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::OK),
      0x0001 => Ok(Self::Warning),
      0x0002 => Ok(Self::Critical),
      _ => Err(())
    }
  }
}
impl TlvDecode for BatChargeLevelEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum BatChargeStateEnum {
  ///Unable to determine the charging state
  Unknown = 0x0000,
  ///The battery is charging
  IsCharging = 0x0001,
  ///The battery is at full charge
  IsAtFullCharge = 0x0002,
  ///The battery is not charging
  IsNotCharging = 0x0003,
}
impl AttrType for BatChargeStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unknown"),
  (0x0001, "IsCharging"),
  (0x0002, "IsAtFullCharge"),
  (0x0003, "IsNotCharging"),
  ]);
}
impl TlvPut for BatChargeStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for BatChargeStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unknown),
      0x0001 => Ok(Self::IsCharging),
      0x0002 => Ok(Self::IsAtFullCharge),
      0x0003 => Ok(Self::IsNotCharging),
      _ => Err(())
    }
  }
}
impl TlvDecode for BatChargeStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum BatCommonDesignationEnum {
  ///Common type is unknown or unspecified
  Unspecified = 0x0000,
  ///Common type is as specified
  AAA = 0x0001,
  ///Common type is as specified
  AA = 0x0002,
  ///Common type is as specified
  C = 0x0003,
  ///Common type is as specified
  D = 0x0004,
  ///Common type is as specified
  _4v5 = 0x0005,
  ///Common type is as specified
  _6v0 = 0x0006,
  ///Common type is as specified
  _9v0 = 0x0007,
  ///Common type is as specified
  _1_2AA = 0x0008,
  ///Common type is as specified
  AAAA = 0x0009,
  ///Common type is as specified
  A = 0x000a,
  ///Common type is as specified
  B = 0x000b,
  ///Common type is as specified
  F = 0x000c,
  ///Common type is as specified
  N = 0x000d,
  ///Common type is as specified
  No6 = 0x000e,
  ///Common type is as specified
  SubC = 0x000f,
  ///Common type is as specified
  A23 = 0x0010,
  ///Common type is as specified
  A27 = 0x0011,
  ///Common type is as specified
  BA5800 = 0x0012,
  ///Common type is as specified
  Duplex = 0x0013,
  ///Common type is as specified
  _4SR44 = 0x0014,
  ///Common type is as specified
  _523 = 0x0015,
  ///Common type is as specified
  _531 = 0x0016,
  ///Common type is as specified
  _15v0 = 0x0017,
  ///Common type is as specified
  _22v5 = 0x0018,
  ///Common type is as specified
  _30v0 = 0x0019,
  ///Common type is as specified
  _45v0 = 0x001a,
  ///Common type is as specified
  _67v5 = 0x001b,
  ///Common type is as specified
  J = 0x001c,
  ///Common type is as specified
  CR123A = 0x001d,
  ///Common type is as specified
  CR2 = 0x001e,
  ///Common type is as specified
  _2CR5 = 0x001f,
  ///Common type is as specified
  CR_P2 = 0x0020,
  ///Common type is as specified
  CR_V3 = 0x0021,
  ///Common type is as specified
  SR41 = 0x0022,
  ///Common type is as specified
  SR43 = 0x0023,
  ///Common type is as specified
  SR44 = 0x0024,
  ///Common type is as specified
  SR45 = 0x0025,
  ///Common type is as specified
  SR48 = 0x0026,
  ///Common type is as specified
  SR54 = 0x0027,
  ///Common type is as specified
  SR55 = 0x0028,
  ///Common type is as specified
  SR57 = 0x0029,
  ///Common type is as specified
  SR58 = 0x002a,
  ///Common type is as specified
  SR59 = 0x002b,
  ///Common type is as specified
  SR60 = 0x002c,
  ///Common type is as specified
  SR63 = 0x002d,
  ///Common type is as specified
  SR64 = 0x002e,
  ///Common type is as specified
  SR65 = 0x002f,
  ///Common type is as specified
  SR66 = 0x0030,
  ///Common type is as specified
  SR67 = 0x0031,
  ///Common type is as specified
  SR68 = 0x0032,
  ///Common type is as specified
  SR69 = 0x0033,
  ///Common type is as specified
  SR516 = 0x0034,
  ///Common type is as specified
  SR731 = 0x0035,
  ///Common type is as specified
  SR712 = 0x0036,
  ///Common type is as specified
  LR932 = 0x0037,
  ///Common type is as specified
  A5 = 0x0038,
  ///Common type is as specified
  A10 = 0x0039,
  ///Common type is as specified
  A13 = 0x003a,
  ///Common type is as specified
  A312 = 0x003b,
  ///Common type is as specified
  A675 = 0x003c,
  ///Common type is as specified
  AC41E = 0x003d,
  ///Common type is as specified
  _10180 = 0x003e,
  ///Common type is as specified
  _10280 = 0x003f,
  ///Common type is as specified
  _10440 = 0x0040,
  ///Common type is as specified
  _14250 = 0x0041,
  ///Common type is as specified
  _14430 = 0x0042,
  ///Common type is as specified
  _14500 = 0x0043,
  ///Common type is as specified
  _14650 = 0x0044,
  ///Common type is as specified
  _15270 = 0x0045,
  ///Common type is as specified
  _16340 = 0x0046,
  ///Common type is as specified
  RCR123A = 0x0047,
  ///Common type is as specified
  _17500 = 0x0048,
  ///Common type is as specified
  _17670 = 0x0049,
  ///Common type is as specified
  _18350 = 0x004a,
  ///Common type is as specified
  _18500 = 0x004b,
  ///Common type is as specified
  _18650 = 0x004c,
  ///Common type is as specified
  _19670 = 0x004d,
  ///Common type is as specified
  _25500 = 0x004e,
  ///Common type is as specified
  _26650 = 0x004f,
  ///Common type is as specified
  _32600 = 0x0050,
}
impl AttrType for BatCommonDesignationEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "AAA"),
  (0x0002, "AA"),
  (0x0003, "C"),
  (0x0004, "D"),
  (0x0005, "_4v5"),
  (0x0006, "_6v0"),
  (0x0007, "_9v0"),
  (0x0008, "_1_2AA"),
  (0x0009, "AAAA"),
  (0x000a, "A"),
  (0x000b, "B"),
  (0x000c, "F"),
  (0x000d, "N"),
  (0x000e, "No6"),
  (0x000f, "SubC"),
  (0x0010, "A23"),
  (0x0011, "A27"),
  (0x0012, "BA5800"),
  (0x0013, "Duplex"),
  (0x0014, "_4SR44"),
  (0x0015, "_523"),
  (0x0016, "_531"),
  (0x0017, "_15v0"),
  (0x0018, "_22v5"),
  (0x0019, "_30v0"),
  (0x001a, "_45v0"),
  (0x001b, "_67v5"),
  (0x001c, "J"),
  (0x001d, "CR123A"),
  (0x001e, "CR2"),
  (0x001f, "_2CR5"),
  (0x0020, "CR_P2"),
  (0x0021, "CR_V3"),
  (0x0022, "SR41"),
  (0x0023, "SR43"),
  (0x0024, "SR44"),
  (0x0025, "SR45"),
  (0x0026, "SR48"),
  (0x0027, "SR54"),
  (0x0028, "SR55"),
  (0x0029, "SR57"),
  (0x002a, "SR58"),
  (0x002b, "SR59"),
  (0x002c, "SR60"),
  (0x002d, "SR63"),
  (0x002e, "SR64"),
  (0x002f, "SR65"),
  (0x0030, "SR66"),
  (0x0031, "SR67"),
  (0x0032, "SR68"),
  (0x0033, "SR69"),
  (0x0034, "SR516"),
  (0x0035, "SR731"),
  (0x0036, "SR712"),
  (0x0037, "LR932"),
  (0x0038, "A5"),
  (0x0039, "A10"),
  (0x003a, "A13"),
  (0x003b, "A312"),
  (0x003c, "A675"),
  (0x003d, "AC41E"),
  (0x003e, "_10180"),
  (0x003f, "_10280"),
  (0x0040, "_10440"),
  (0x0041, "_14250"),
  (0x0042, "_14430"),
  (0x0043, "_14500"),
  (0x0044, "_14650"),
  (0x0045, "_15270"),
  (0x0046, "_16340"),
  (0x0047, "RCR123A"),
  (0x0048, "_17500"),
  (0x0049, "_17670"),
  (0x004a, "_18350"),
  (0x004b, "_18500"),
  (0x004c, "_18650"),
  (0x004d, "_19670"),
  (0x004e, "_25500"),
  (0x004f, "_26650"),
  (0x0050, "_32600"),
  ]);
}
impl TlvPut for BatCommonDesignationEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for BatCommonDesignationEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unspecified),
      0x0001 => Ok(Self::AAA),
      0x0002 => Ok(Self::AA),
      0x0003 => Ok(Self::C),
      0x0004 => Ok(Self::D),
      0x0005 => Ok(Self::_4v5),
      0x0006 => Ok(Self::_6v0),
      0x0007 => Ok(Self::_9v0),
      0x0008 => Ok(Self::_1_2AA),
      0x0009 => Ok(Self::AAAA),
      0x000a => Ok(Self::A),
      0x000b => Ok(Self::B),
      0x000c => Ok(Self::F),
      0x000d => Ok(Self::N),
      0x000e => Ok(Self::No6),
      0x000f => Ok(Self::SubC),
      0x0010 => Ok(Self::A23),
      0x0011 => Ok(Self::A27),
      0x0012 => Ok(Self::BA5800),
      0x0013 => Ok(Self::Duplex),
      0x0014 => Ok(Self::_4SR44),
      0x0015 => Ok(Self::_523),
      0x0016 => Ok(Self::_531),
      0x0017 => Ok(Self::_15v0),
      0x0018 => Ok(Self::_22v5),
      0x0019 => Ok(Self::_30v0),
      0x001a => Ok(Self::_45v0),
      0x001b => Ok(Self::_67v5),
      0x001c => Ok(Self::J),
      0x001d => Ok(Self::CR123A),
      0x001e => Ok(Self::CR2),
      0x001f => Ok(Self::_2CR5),
      0x0020 => Ok(Self::CR_P2),
      0x0021 => Ok(Self::CR_V3),
      0x0022 => Ok(Self::SR41),
      0x0023 => Ok(Self::SR43),
      0x0024 => Ok(Self::SR44),
      0x0025 => Ok(Self::SR45),
      0x0026 => Ok(Self::SR48),
      0x0027 => Ok(Self::SR54),
      0x0028 => Ok(Self::SR55),
      0x0029 => Ok(Self::SR57),
      0x002a => Ok(Self::SR58),
      0x002b => Ok(Self::SR59),
      0x002c => Ok(Self::SR60),
      0x002d => Ok(Self::SR63),
      0x002e => Ok(Self::SR64),
      0x002f => Ok(Self::SR65),
      0x0030 => Ok(Self::SR66),
      0x0031 => Ok(Self::SR67),
      0x0032 => Ok(Self::SR68),
      0x0033 => Ok(Self::SR69),
      0x0034 => Ok(Self::SR516),
      0x0035 => Ok(Self::SR731),
      0x0036 => Ok(Self::SR712),
      0x0037 => Ok(Self::LR932),
      0x0038 => Ok(Self::A5),
      0x0039 => Ok(Self::A10),
      0x003a => Ok(Self::A13),
      0x003b => Ok(Self::A312),
      0x003c => Ok(Self::A675),
      0x003d => Ok(Self::AC41E),
      0x003e => Ok(Self::_10180),
      0x003f => Ok(Self::_10280),
      0x0040 => Ok(Self::_10440),
      0x0041 => Ok(Self::_14250),
      0x0042 => Ok(Self::_14430),
      0x0043 => Ok(Self::_14500),
      0x0044 => Ok(Self::_14650),
      0x0045 => Ok(Self::_15270),
      0x0046 => Ok(Self::_16340),
      0x0047 => Ok(Self::RCR123A),
      0x0048 => Ok(Self::_17500),
      0x0049 => Ok(Self::_17670),
      0x004a => Ok(Self::_18350),
      0x004b => Ok(Self::_18500),
      0x004c => Ok(Self::_18650),
      0x004d => Ok(Self::_19670),
      0x004e => Ok(Self::_25500),
      0x004f => Ok(Self::_26650),
      0x0050 => Ok(Self::_32600),
      _ => Err(())
    }
  }
}
impl TlvDecode for BatCommonDesignationEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum BatFaultEnum {
  ///The Node detects an unspecified fault on this battery power source.
  Unspecified = 0x0000,
  ///The Node detects the temperature of this battery power source is above ideal operating conditions.
  OverTemp = 0x0001,
  ///The Node detects the temperature of this battery power source is below ideal operating conditions.
  UnderTemp = 0x0002,
}
impl AttrType for BatFaultEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "OverTemp"),
  (0x0002, "UnderTemp"),
  ]);
}
impl TlvPut for BatFaultEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for BatFaultEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unspecified),
      0x0001 => Ok(Self::OverTemp),
      0x0002 => Ok(Self::UnderTemp),
      _ => Err(())
    }
  }
}
impl TlvDecode for BatFaultEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum BatReplaceabilityEnum {
  ///The replaceability is unspecified or unknown.
  Unspecified = 0x0000,
  ///The battery is not replaceable.
  NotReplaceable = 0x0001,
  ///The battery is replaceable by the user or customer.
  UserReplaceable = 0x0002,
  ///The battery is replaceable by an authorized factory technician.
  FactoryReplaceable = 0x0003,
}
impl AttrType for BatReplaceabilityEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "NotReplaceable"),
  (0x0002, "UserReplaceable"),
  (0x0003, "FactoryReplaceable"),
  ]);
}
impl TlvPut for BatReplaceabilityEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for BatReplaceabilityEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unspecified),
      0x0001 => Ok(Self::NotReplaceable),
      0x0002 => Ok(Self::UserReplaceable),
      0x0003 => Ok(Self::FactoryReplaceable),
      _ => Err(())
    }
  }
}
impl TlvDecode for BatReplaceabilityEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum PowerSourceStatusEnum {
  ///Indicate the source status is not specified
  Unspecified = 0x0000,
  ///Indicate the source is available and currently supplying power
  Active = 0x0001,
  ///Indicate the source is available, but is not currently supplying power
  Standby = 0x0002,
  ///Indicate the source is not currently available to supply power
  Unavailable = 0x0003,
}
impl AttrType for PowerSourceStatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "Active"),
  (0x0002, "Standby"),
  (0x0003, "Unavailable"),
  ]);
}
impl TlvPut for PowerSourceStatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for PowerSourceStatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unspecified),
      0x0001 => Ok(Self::Active),
      0x0002 => Ok(Self::Standby),
      0x0003 => Ok(Self::Unavailable),
      _ => Err(())
    }
  }
}
impl TlvDecode for PowerSourceStatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum WiredCurrentTypeEnum {
  ///Indicates AC current
  AC = 0x0000,
  ///Indicates DC current
  DC = 0x0001,
}
impl AttrType for WiredCurrentTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "AC"),
  (0x0001, "DC"),
  ]);
}
impl TlvPut for WiredCurrentTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for WiredCurrentTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::AC),
      0x0001 => Ok(Self::DC),
      _ => Err(())
    }
  }
}
impl TlvDecode for WiredCurrentTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum WiredFaultEnum {
  ///The Node detects an unspecified fault on this wired power source.
  Unspecified = 0x0000,
  ///The Node detects the supplied voltage is above maximum supported value for this wired power source.
  OverVoltage = 0x0001,
  ///The Node detects the supplied voltage is below maximum supported value for this wired power source.
  UnderVoltage = 0x0002,
}
impl AttrType for WiredFaultEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "OverVoltage"),
  (0x0002, "UnderVoltage"),
  ]);
}
impl TlvPut for WiredFaultEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for WiredFaultEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unspecified),
      0x0001 => Ok(Self::OverVoltage),
      0x0002 => Ok(Self::UnderVoltage),
      _ => Err(())
    }
  }
}
impl TlvDecode for WiredFaultEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
