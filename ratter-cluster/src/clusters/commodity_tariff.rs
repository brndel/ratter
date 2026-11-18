#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const COMMODITY_TARIFF: ClusterDef = ClusterDef {
  id: 0x0700,
  name: "Commodity Tariff",
  features: &[
    FeatureDef { bit: 0x0000, code: "PRICE", name: "Pricing", summary: "Supports information about commodity pricing" },
    FeatureDef { bit: 0x0001, code: "FCRED", name: "FriendlyCredit", summary: "Supports information about when friendly credit periods begin and end" },
    FeatureDef { bit: 0x0002, code: "AUXLD", name: "AuxiliaryLoad", summary: "Supports information about when auxiliary loads should be enabled or disabled" },
    FeatureDef { bit: 0x0003, code: "PEAKP", name: "PeakPeriod", summary: "Supports information about peak periods" },
    FeatureDef { bit: 0x0004, code: "PWRTHLD", name: "PowerThreshold", summary: "Supports information about power threshold" },
    FeatureDef { bit: 0x0005, code: "RNDM", name: "Randomization", summary: "Supports information about randomization of calendar day entries" },
  ],
  attributes: &[
    attr::TariffInfo::ATTR,
    attr::TariffUnit::ATTR,
    attr::StartDate::ATTR,
    attr::DayEntries::ATTR,
    attr::DayPatterns::ATTR,
    attr::CalendarPeriods::ATTR,
    attr::IndividualDays::ATTR,
    attr::CurrentDay::ATTR,
    attr::NextDay::ATTR,
    attr::CurrentDayEntry::ATTR,
    attr::CurrentDayEntryDate::ATTR,
    attr::NextDayEntry::ATTR,
    attr::NextDayEntryDate::ATTR,
    attr::TariffComponents::ATTR,
    attr::TariffPeriods::ATTR,
    attr::CurrentTariffComponents::ATTR,
    attr::NextTariffComponents::ATTR,
    attr::DefaultRandomizationOffset::ATTR,
    attr::DefaultRandomizationType::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::GetTariffComponent::COMMAND,
    command::GetDayEntry::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct TariffInfo;

  impl Attr for TariffInfo {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "TariffInfo",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::TariffInformationStruct>;
  }


  pub struct TariffUnit;

  impl Attr for TariffUnit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "TariffUnit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::TariffUnitEnum>;
  }


  pub struct StartDate;

  impl Attr for StartDate {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "StartDate",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* EpochS */>;
  }


  pub struct DayEntries;

  impl Attr for DayEntries {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "DayEntries",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<Vec<ty::DayEntryStruct>>;
  }


  pub struct DayPatterns;

  impl Attr for DayPatterns {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "DayPatterns",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<Vec<ty::DayPatternStruct>>;
  }


  pub struct CalendarPeriods;

  impl Attr for CalendarPeriods {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "CalendarPeriods",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<Vec<ty::CalendarPeriodStruct>>;
  }


  pub struct IndividualDays;

  impl Attr for IndividualDays {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "IndividualDays",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<Vec<ty::DayStruct>>;
  }


  pub struct CurrentDay;

  impl Attr for CurrentDay {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "CurrentDay",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::DayStruct>;
  }


  pub struct NextDay;

  impl Attr for NextDay {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "NextDay",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::DayStruct>;
  }


  pub struct CurrentDayEntry;

  impl Attr for CurrentDayEntry {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "CurrentDayEntry",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::DayEntryStruct>;
  }


  pub struct CurrentDayEntryDate;

  impl Attr for CurrentDayEntryDate {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "CurrentDayEntryDate",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* EpochS */>;
  }


  pub struct NextDayEntry;

  impl Attr for NextDayEntry {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "NextDayEntry",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::DayEntryStruct>;
  }


  pub struct NextDayEntryDate;

  impl Attr for NextDayEntryDate {
    const ATTR: AttrDef = AttrDef {
      id: 0x000c,
      name: "NextDayEntryDate",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* EpochS */>;
  }


  pub struct TariffComponents;

  impl Attr for TariffComponents {
    const ATTR: AttrDef = AttrDef {
      id: 0x000d,
      name: "TariffComponents",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<Vec<ty::TariffComponentStruct>>;
  }


  pub struct TariffPeriods;

  impl Attr for TariffPeriods {
    const ATTR: AttrDef = AttrDef {
      id: 0x000e,
      name: "TariffPeriods",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<Vec<ty::TariffPeriodStruct>>;
  }


  pub struct CurrentTariffComponents;

  impl Attr for CurrentTariffComponents {
    const ATTR: AttrDef = AttrDef {
      id: 0x000f,
      name: "CurrentTariffComponents",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<Vec<ty::TariffComponentStruct>>;
  }


  pub struct NextTariffComponents;

  impl Attr for NextTariffComponents {
    const ATTR: AttrDef = AttrDef {
      id: 0x0010,
      name: "NextTariffComponents",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<Vec<ty::TariffComponentStruct>>;
  }


  pub struct DefaultRandomizationOffset;

  impl Attr for DefaultRandomizationOffset {
    const ATTR: AttrDef = AttrDef {
      id: 0x0011,
      name: "DefaultRandomizationOffset",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(5),
    };
    type Value = Nullable<i16>;
  }


  pub struct DefaultRandomizationType;

  impl Attr for DefaultRandomizationType {
    const ATTR: AttrDef = AttrDef {
      id: 0x0012,
      name: "DefaultRandomizationType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(5),
    };
    type Value = Nullable<ty::DayEntryRandomizationTypeEnum>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct GetTariffComponent {
  pub tariff_component_id: u32,
}
impl Command for GetTariffComponent {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "GetTariffComponent",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "TariffComponentID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for GetTariffComponent {
  type Response = GetTariffComponentResponse;
}
impl TlvEncode for GetTariffComponent {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.tariff_component_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetTariffComponentResponse {
  pub label: String,
  pub day_entry_i_ds: Vec<u32>,
  pub tariff_component: ty::TariffComponentStruct,
}
impl Command for GetTariffComponentResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "GetTariffComponentResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Label", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "DayEntryIDs", attr_ty: <Vec<u32>>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TariffComponent", attr_ty: <ty::TariffComponentStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for GetTariffComponentResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct GetDayEntry {
  pub day_entry_id: u32,
}
impl Command for GetDayEntry {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "GetDayEntry",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "DayEntryID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for GetDayEntry {
  type Response = GetDayEntryResponse;
}
impl TlvEncode for GetDayEntry {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.day_entry_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetDayEntryResponse {
  pub day_entry: ty::DayEntryStruct,
}
impl Command for GetDayEntryResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "GetDayEntryResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "DayEntry", attr_ty: <ty::DayEntryStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for GetDayEntryResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum AuxiliaryLoadSettingEnum {
  ///The switch should be in the OFF state
  Off = 0x0000,
  ///The switch should be in the ON state
  On = 0x0001,
  ///No state is required
  None = 0x0002,
}
impl AttrType for AuxiliaryLoadSettingEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Off"),
  (0x0001, "On"),
  (0x0002, "None"),
  ]);
}
impl TlvPut for AuxiliaryLoadSettingEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for AuxiliaryLoadSettingEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Off),
      0x0001 => Ok(Self::On),
      0x0002 => Ok(Self::None),
      _ => Err(())
    }
  }
}
impl TlvDecode for AuxiliaryLoadSettingEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum BlockModeEnum {
  ///Tariff has no usage blocks
  NoBlock = 0x0000,
  ///Usage is metered in combined blocks
  Combined = 0x0001,
  ///Usage is metered separately by tariff component
  Individual = 0x0002,
}
impl AttrType for BlockModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "NoBlock"),
  (0x0001, "Combined"),
  (0x0002, "Individual"),
  ]);
}
impl TlvPut for BlockModeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for BlockModeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::NoBlock),
      0x0001 => Ok(Self::Combined),
      0x0002 => Ok(Self::Individual),
      _ => Err(())
    }
  }
}
impl TlvDecode for BlockModeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum DayEntryRandomizationTypeEnum {
  ///No randomization applied
  None = 0x0000,
  ///An unchanging offset
  Fixed = 0x0001,
  ///A random value
  Random = 0x0002,
  ///A random positive value
  RandomPositive = 0x0003,
  ///A random negative value
  RandomNegative = 0x0004,
}
impl AttrType for DayEntryRandomizationTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "None"),
  (0x0001, "Fixed"),
  (0x0002, "Random"),
  (0x0003, "RandomPositive"),
  (0x0004, "RandomNegative"),
  ]);
}
impl TlvPut for DayEntryRandomizationTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for DayEntryRandomizationTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::None),
      0x0001 => Ok(Self::Fixed),
      0x0002 => Ok(Self::Random),
      0x0003 => Ok(Self::RandomPositive),
      0x0004 => Ok(Self::RandomNegative),
      _ => Err(())
    }
  }
}
impl TlvDecode for DayEntryRandomizationTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum DayTypeEnum {
  ///Standard
  Standard = 0x0000,
  ///Holiday
  Holiday = 0x0001,
  ///Dynamic Pricing
  Dynamic = 0x0002,
  ///Individual Events
  Event = 0x0003,
}
impl AttrType for DayTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Standard"),
  (0x0001, "Holiday"),
  (0x0002, "Dynamic"),
  (0x0003, "Event"),
  ]);
}
impl TlvPut for DayTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for DayTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Standard),
      0x0001 => Ok(Self::Holiday),
      0x0002 => Ok(Self::Dynamic),
      0x0003 => Ok(Self::Event),
      _ => Err(())
    }
  }
}
impl TlvDecode for DayTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum PeakPeriodSeverityEnum {
  ///Unused
  Unused = 0x0000,
  ///Low
  Low = 0x0001,
  ///Medium
  Medium = 0x0002,
  ///High
  High = 0x0003,
}
impl AttrType for PeakPeriodSeverityEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unused"),
  (0x0001, "Low"),
  (0x0002, "Medium"),
  (0x0003, "High"),
  ]);
}
impl TlvPut for PeakPeriodSeverityEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for PeakPeriodSeverityEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unused),
      0x0001 => Ok(Self::Low),
      0x0002 => Ok(Self::Medium),
      0x0003 => Ok(Self::High),
      _ => Err(())
    }
  }
}
impl TlvDecode for PeakPeriodSeverityEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct DayPatternDayOfWeekBitmap: u32 {
    ///Sunday
    const Sunday = 1 << 0;
    ///Monday
    const Monday = 1 << 1;
    ///Tuesday
    const Tuesday = 1 << 2;
    ///Wednesday
    const Wednesday = 1 << 3;
    ///Thursday
    const Thursday = 1 << 4;
    ///Friday
    const Friday = 1 << 5;
    ///Saturday
    const Saturday = 1 << 6;
  }
}
impl AttrType for DayPatternDayOfWeekBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Sunday"),
    (1, "Monday"),
    (2, "Tuesday"),
    (3, "Wednesday"),
    (4, "Thursday"),
    (5, "Friday"),
    (6, "Saturday"),
  ]);
}
impl TlvPut for DayPatternDayOfWeekBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for DayPatternDayOfWeekBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for DayPatternDayOfWeekBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AuxiliaryLoadSwitchSettingsStruct {
  pub number: u8,
  pub required_state: AuxiliaryLoadSettingEnum,
}
impl AttrType for AuxiliaryLoadSwitchSettingsStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Number", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "RequiredState", attr_ty: <AuxiliaryLoadSettingEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for AuxiliaryLoadSwitchSettingsStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.number.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.required_state.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for AuxiliaryLoadSwitchSettingsStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut number: Option<u8> = None;
    let mut required_state: Option<AuxiliaryLoadSettingEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { number = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { required_state = Some(<AuxiliaryLoadSettingEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      number: number.ok_or(TlvDecodeErr::MissingField("Number"))?,
      required_state: required_state.ok_or(TlvDecodeErr::MissingField("RequiredState"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuxiliaryLoadSwitchesSettingsStruct {
  pub switch_states: Vec<AuxiliaryLoadSwitchSettingsStruct>,
}
impl AttrType for AuxiliaryLoadSwitchesSettingsStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "SwitchStates", attr_ty: <Vec<AuxiliaryLoadSwitchSettingsStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for AuxiliaryLoadSwitchesSettingsStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.switch_states.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for AuxiliaryLoadSwitchesSettingsStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut switch_states: Option<Vec<AuxiliaryLoadSwitchSettingsStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { switch_states = Some(<Vec<AuxiliaryLoadSwitchSettingsStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      switch_states: switch_states.ok_or(TlvDecodeErr::MissingField("SwitchStates"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalendarPeriodStruct {
  pub start_date: TypeUnsupported /* EpochS */,
  pub day_pattern_i_ds: Vec<u32>,
}
impl AttrType for CalendarPeriodStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "StartDate", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "DayPatternIDs", attr_ty: <Vec<u32>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for CalendarPeriodStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.start_date.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.day_pattern_i_ds.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for CalendarPeriodStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut day_pattern_i_ds: Option<Vec<u32>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { day_pattern_i_ds = Some(<Vec<u32>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      start_date: TypeUnsupported,
      day_pattern_i_ds: day_pattern_i_ds.ok_or(TlvDecodeErr::MissingField("DayPatternIDs"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DayEntryStruct {
  pub day_entry_id: u32,
  pub start_time: u16,
  pub duration: u16,
  pub randomization_offset: i16,
  pub randomization_type: DayEntryRandomizationTypeEnum,
}
impl AttrType for DayEntryStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "DayEntryID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "StartTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Duration", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "RandomizationOffset", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "RandomizationType", attr_ty: <DayEntryRandomizationTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DayEntryStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.day_entry_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.start_time.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.duration.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.randomization_offset.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.randomization_type.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DayEntryStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut day_entry_id: Option<u32> = None;
    let mut start_time: Option<u16> = None;
    let mut duration: Option<u16> = None;
    let mut randomization_offset: Option<i16> = None;
    let mut randomization_type: Option<DayEntryRandomizationTypeEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { day_entry_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { start_time = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { duration = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { randomization_offset = Some(<i16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { randomization_type = Some(<DayEntryRandomizationTypeEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      day_entry_id: day_entry_id.ok_or(TlvDecodeErr::MissingField("DayEntryID"))?,
      start_time: start_time.ok_or(TlvDecodeErr::MissingField("StartTime"))?,
      duration: duration.ok_or(TlvDecodeErr::MissingField("Duration"))?,
      randomization_offset: randomization_offset.ok_or(TlvDecodeErr::MissingField("RandomizationOffset"))?,
      randomization_type: randomization_type.ok_or(TlvDecodeErr::MissingField("RandomizationType"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DayPatternStruct {
  pub day_pattern_id: u32,
  pub days_of_week: DayPatternDayOfWeekBitmap,
  pub day_entry_i_ds: Vec<u32>,
}
impl AttrType for DayPatternStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "DayPatternID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "DaysOfWeek", attr_ty: <DayPatternDayOfWeekBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "DayEntryIDs", attr_ty: <Vec<u32>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DayPatternStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.day_pattern_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.days_of_week.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.day_entry_i_ds.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DayPatternStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut day_pattern_id: Option<u32> = None;
    let mut days_of_week: Option<DayPatternDayOfWeekBitmap> = None;
    let mut day_entry_i_ds: Option<Vec<u32>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { day_pattern_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { days_of_week = Some(<DayPatternDayOfWeekBitmap>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { day_entry_i_ds = Some(<Vec<u32>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      day_pattern_id: day_pattern_id.ok_or(TlvDecodeErr::MissingField("DayPatternID"))?,
      days_of_week: days_of_week.ok_or(TlvDecodeErr::MissingField("DaysOfWeek"))?,
      day_entry_i_ds: day_entry_i_ds.ok_or(TlvDecodeErr::MissingField("DayEntryIDs"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DayStruct {
  pub date: TypeUnsupported /* EpochS */,
  pub day_type: DayTypeEnum,
  pub day_entry_i_ds: Vec<u32>,
}
impl AttrType for DayStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Date", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "DayType", attr_ty: <DayTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "DayEntryIDs", attr_ty: <Vec<u32>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DayStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.date.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.day_type.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.day_entry_i_ds.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DayStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut day_type: Option<DayTypeEnum> = None;
    let mut day_entry_i_ds: Option<Vec<u32>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { day_type = Some(<DayTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { day_entry_i_ds = Some(<Vec<u32>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      date: TypeUnsupported,
      day_type: day_type.ok_or(TlvDecodeErr::MissingField("DayType"))?,
      day_entry_i_ds: day_entry_i_ds.ok_or(TlvDecodeErr::MissingField("DayEntryIDs"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PeakPeriodStruct {
  pub severity: PeakPeriodSeverityEnum,
  pub peak_period: u16,
}
impl AttrType for PeakPeriodStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Severity", attr_ty: <PeakPeriodSeverityEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "PeakPeriod", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for PeakPeriodStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.severity.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.peak_period.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for PeakPeriodStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut severity: Option<PeakPeriodSeverityEnum> = None;
    let mut peak_period: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { severity = Some(<PeakPeriodSeverityEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { peak_period = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      severity: severity.ok_or(TlvDecodeErr::MissingField("Severity"))?,
      peak_period: peak_period.ok_or(TlvDecodeErr::MissingField("PeakPeriod"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TariffComponentStruct {
  pub tariff_component_id: u32,
  pub price: TariffPriceStruct,
  pub friendly_credit: bool,
  pub auxiliary_load: AuxiliaryLoadSwitchSettingsStruct,
  pub peak_period: PeakPeriodStruct,
  pub power_threshold: PowerThresholdStruct,
  pub threshold: i64,
  pub label: String,
  pub predicted: bool,
}
impl AttrType for TariffComponentStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "TariffComponentID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Price", attr_ty: <TariffPriceStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "FriendlyCredit", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "AuxiliaryLoad", attr_ty: <AuxiliaryLoadSwitchSettingsStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "PeakPeriod", attr_ty: <PeakPeriodStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "PowerThreshold", attr_ty: <PowerThresholdStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "Threshold", attr_ty: <i64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "Label", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0008, name: "Predicted", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TariffComponentStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.tariff_component_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.price.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.friendly_credit.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.auxiliary_load.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.peak_period.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.power_threshold.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.threshold.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.label.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.predicted.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TariffComponentStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut tariff_component_id: Option<u32> = None;
    let mut price: Option<TariffPriceStruct> = None;
    let mut friendly_credit: Option<bool> = None;
    let mut auxiliary_load: Option<AuxiliaryLoadSwitchSettingsStruct> = None;
    let mut peak_period: Option<PeakPeriodStruct> = None;
    let mut power_threshold: Option<PowerThresholdStruct> = None;
    let mut threshold: Option<i64> = None;
    let mut label: Option<String> = None;
    let mut predicted: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { tariff_component_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { price = Some(<TariffPriceStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { friendly_credit = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0003), kind: _ }) => { auxiliary_load = Some(<AuxiliaryLoadSwitchSettingsStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0004), kind: _ }) => { peak_period = Some(<PeakPeriodStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0005), kind: _ }) => { power_threshold = Some(<PowerThresholdStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { threshold = Some(<i64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0007), value: _ }) => { label = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0008), value: _ }) => { predicted = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      tariff_component_id: tariff_component_id.ok_or(TlvDecodeErr::MissingField("TariffComponentID"))?,
      price: price.ok_or(TlvDecodeErr::MissingField("Price"))?,
      friendly_credit: friendly_credit.ok_or(TlvDecodeErr::MissingField("FriendlyCredit"))?,
      auxiliary_load: auxiliary_load.ok_or(TlvDecodeErr::MissingField("AuxiliaryLoad"))?,
      peak_period: peak_period.ok_or(TlvDecodeErr::MissingField("PeakPeriod"))?,
      power_threshold: power_threshold.ok_or(TlvDecodeErr::MissingField("PowerThreshold"))?,
      threshold: threshold.ok_or(TlvDecodeErr::MissingField("Threshold"))?,
      label: label.ok_or(TlvDecodeErr::MissingField("Label"))?,
      predicted: predicted.ok_or(TlvDecodeErr::MissingField("Predicted"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TariffInformationStruct {
  pub tariff_label: String,
  pub provider_name: String,
  pub currency: CurrencyStruct,
  pub block_mode: BlockModeEnum,
}
impl AttrType for TariffInformationStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "TariffLabel", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ProviderName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Currency", attr_ty: <CurrencyStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "BlockMode", attr_ty: <BlockModeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TariffInformationStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.tariff_label.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.provider_name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.currency.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.block_mode.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TariffInformationStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut tariff_label: Option<String> = None;
    let mut provider_name: Option<String> = None;
    let mut currency: Option<CurrencyStruct> = None;
    let mut block_mode: Option<BlockModeEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { tariff_label = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { provider_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { currency = Some(<CurrencyStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { block_mode = Some(<BlockModeEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      tariff_label: tariff_label.ok_or(TlvDecodeErr::MissingField("TariffLabel"))?,
      provider_name: provider_name.ok_or(TlvDecodeErr::MissingField("ProviderName"))?,
      currency: currency.ok_or(TlvDecodeErr::MissingField("Currency"))?,
      block_mode: block_mode.ok_or(TlvDecodeErr::MissingField("BlockMode"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TariffPeriodStruct {
  pub label: String,
  pub day_entry_i_ds: Vec<u32>,
  pub tariff_component_i_ds: Vec<u32>,
}
impl AttrType for TariffPeriodStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Label", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "DayEntryIDs", attr_ty: <Vec<u32>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "TariffComponentIDs", attr_ty: <Vec<u32>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TariffPeriodStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.label.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.day_entry_i_ds.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.tariff_component_i_ds.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TariffPeriodStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut label: Option<String> = None;
    let mut day_entry_i_ds: Option<Vec<u32>> = None;
    let mut tariff_component_i_ds: Option<Vec<u32>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { label = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { day_entry_i_ds = Some(<Vec<u32>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { tariff_component_i_ds = Some(<Vec<u32>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      label: label.ok_or(TlvDecodeErr::MissingField("Label"))?,
      day_entry_i_ds: day_entry_i_ds.ok_or(TlvDecodeErr::MissingField("DayEntryIDs"))?,
      tariff_component_i_ds: tariff_component_i_ds.ok_or(TlvDecodeErr::MissingField("TariffComponentIDs"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TariffPriceStruct {
  pub price_type: TariffPriceTypeEnum,
  pub price: TypeUnsupported /* Money */,
  pub price_level: i16,
}
impl AttrType for TariffPriceStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "PriceType", attr_ty: <TariffPriceTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Price", attr_ty: <TypeUnsupported /* Money */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "PriceLevel", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TariffPriceStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.price_type.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.price.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.price_level.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TariffPriceStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut price_type: Option<TariffPriceTypeEnum> = None;
    let mut price_level: Option<i16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { price_type = Some(<TariffPriceTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { price_level = Some(<i16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      price_type: price_type.ok_or(TlvDecodeErr::MissingField("PriceType"))?,
      price: TypeUnsupported,
      price_level: price_level.ok_or(TlvDecodeErr::MissingField("PriceLevel"))?,
    })
  }
}

}
