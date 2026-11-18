#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const TIME_SYNCHRONIZATION: ClusterDef = ClusterDef {
  id: 0x0038,
  name: "Time Synchronization",
  features: &[
    FeatureDef { bit: 0x0000, code: "TZ", name: "TimeZone", summary: "Server supports time zone" },
    FeatureDef { bit: 0x0001, code: "NTPC", name: "NTPClient", summary: "Server supports an NTP or SNTP client" },
    FeatureDef { bit: 0x0002, code: "NTPS", name: "NTPServer", summary: "Server supports an NTP server role" },
    FeatureDef { bit: 0x0003, code: "TSC", name: "TimeSyncClient", summary: "Time synchronization client cluster is present" },
  ],
  attributes: &[
    attr::UTCTime::ATTR,
    attr::Granularity::ATTR,
    attr::TimeSource::ATTR,
    attr::TrustedTimeSource::ATTR,
    attr::DefaultNTP::ATTR,
    attr::TimeZone::ATTR,
    attr::DSTOffset::ATTR,
    attr::LocalTime::ATTR,
    attr::TimeZoneDatabase::ATTR,
    attr::NTPServerAvailable::ATTR,
    attr::TimeZoneListMaxSize::ATTR,
    attr::DSTOffsetListMaxSize::ATTR,
    attr::SupportsDNSResolve::ATTR,
  ],
  events: &[
    event::DSTTableEmpty::EVENT,
    event::DSTStatus::EVENT,
    event::TimeZoneStatus::EVENT,
    event::TimeFailure::EVENT,
    event::MissingTrustedTimeSource::EVENT,
  ],
  commands: &[
    command::SetUTCTime::COMMAND,
    command::SetTrustedTimeSource::COMMAND,
    command::SetTimeZone::COMMAND,
    command::SetDSTOffset::COMMAND,
    command::SetDefaultNTP::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct UTCTime;

  impl Attr for UTCTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "UTCTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* EpochUs */>;
  }


  pub struct Granularity;

  impl Attr for Granularity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "Granularity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::GranularityEnum;
  }


  pub struct TimeSource;

  impl Attr for TimeSource {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "TimeSource",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::TimeSourceEnum;
  }


  pub struct TrustedTimeSource;

  impl Attr for TrustedTimeSource {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "TrustedTimeSource",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = Nullable<ty::TrustedTimeSourceStruct>;
  }


  pub struct DefaultNTP;

  impl Attr for DefaultNTP {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "DefaultNTP",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Nullable<String>;
  }


  pub struct TimeZone;

  impl Attr for TimeZone {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "TimeZone",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<ty::TimeZoneStruct>;
  }


  pub struct DSTOffset;

  impl Attr for DSTOffset {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "DSTOffset",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<ty::DSTOffsetStruct>;
  }


  pub struct LocalTime;

  impl Attr for LocalTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "LocalTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<TypeUnsupported /* EpochUs */>;
  }


  pub struct TimeZoneDatabase;

  impl Attr for TimeZoneDatabase {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "TimeZoneDatabase",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = ty::TimeZoneDatabaseEnum;
  }


  pub struct NTPServerAvailable;

  impl Attr for NTPServerAvailable {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "NTPServerAvailable",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = bool;
  }


  pub struct TimeZoneListMaxSize;

  impl Attr for TimeZoneListMaxSize {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "TimeZoneListMaxSize",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u8;
  }


  pub struct DSTOffsetListMaxSize;

  impl Attr for DSTOffsetListMaxSize {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "DSTOffsetListMaxSize",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u8;
  }


  pub struct SupportsDNSResolve;

  impl Attr for SupportsDNSResolve {
    const ATTR: AttrDef = AttrDef {
      id: 0x000c,
      name: "SupportsDNSResolve",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = bool;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct DSTTableEmpty {
}

impl Event for DSTTableEmpty {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "DSTTableEmpty",
    conformance: Conformance::Feature(0),
    fields: &[
    ],
  };
}

impl TlvDecode for DSTTableEmpty {
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
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DSTStatus {
  pub dst_offset_active: bool,
}

impl Event for DSTStatus {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "DSTStatus",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "DSTOffsetActive", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for DSTStatus {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut dst_offset_active: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { dst_offset_active = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      dst_offset_active: dst_offset_active.ok_or(TlvDecodeErr::MissingField("DSTOffsetActive"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct TimeZoneStatus {
  pub offset: i32,
  pub name: String,
}

impl Event for TimeZoneStatus {
  const EVENT: EventDef = EventDef {
    id: 0x0002,
    name: "TimeZoneStatus",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "Offset", attr_ty: <i32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}

impl TlvDecode for TimeZoneStatus {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut offset: Option<i32> = None;
    let mut name: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { offset = Some(<i32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      offset: offset.ok_or(TlvDecodeErr::MissingField("Offset"))?,
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct TimeFailure {
}

impl Event for TimeFailure {
  const EVENT: EventDef = EventDef {
    id: 0x0003,
    name: "TimeFailure",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}

impl TlvDecode for TimeFailure {
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
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct MissingTrustedTimeSource {
}

impl Event for MissingTrustedTimeSource {
  const EVENT: EventDef = EventDef {
    id: 0x0004,
    name: "MissingTrustedTimeSource",
    conformance: Conformance::Feature(3),
    fields: &[
    ],
  };
}

impl TlvDecode for MissingTrustedTimeSource {
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
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SetUTCTime {
  pub utc_time: TypeUnsupported /* EpochUs */,
  pub granularity: ty::GranularityEnum,
  pub time_source: ty::TimeSourceEnum,
}
impl Command for SetUTCTime {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "SetUTCTime",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "UTCTime", attr_ty: <TypeUnsupported /* EpochUs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Granularity", attr_ty: <ty::GranularityEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TimeSource", attr_ty: <ty::TimeSourceEnum>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for SetUTCTime {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetUTCTime {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.utc_time.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.granularity.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.time_source.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetTrustedTimeSource {
  pub trusted_time_source: ty::FabricScopedTrustedTimeSourceStruct,
}
impl Command for SetTrustedTimeSource {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "SetTrustedTimeSource",
    conformance: Conformance::Feature(3),
    fields: &[
      AttrDef { id: 0x0000, name: "TrustedTimeSource", attr_ty: <ty::FabricScopedTrustedTimeSourceStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SetTrustedTimeSource {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetTrustedTimeSource {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.trusted_time_source.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetTimeZone {
  pub time_zone: Vec<ty::TimeZoneStruct>,
}
impl Command for SetTimeZone {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "SetTimeZone",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "TimeZone", attr_ty: <Vec<ty::TimeZoneStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SetTimeZone {
  type Response = SetTimeZoneResponse;
}
impl TlvEncode for SetTimeZone {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.time_zone.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetTimeZoneResponse {
  pub dst_offset_required: bool,
}
impl Command for SetTimeZoneResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "SetTimeZoneResponse",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "DSTOffsetRequired", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for SetTimeZoneResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct SetDSTOffset {
  pub dst_offset: Vec<ty::DSTOffsetStruct>,
}
impl Command for SetDSTOffset {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "SetDSTOffset",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "DSTOffset", attr_ty: <Vec<ty::DSTOffsetStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SetDSTOffset {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetDSTOffset {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.dst_offset.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetDefaultNTP {
  pub default_ntp: String,
}
impl Command for SetDefaultNTP {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "SetDefaultNTP",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "DefaultNTP", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SetDefaultNTP {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetDefaultNTP {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.default_ntp.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum GranularityEnum {
  ///This indicates that the node is not currently synchronized with a UTC Time source and its clock is based on the Last Known Good UTC Time only.
  NoTimeGranularity = 0x0000,
  ///This indicates the node was synchronized to an upstream source in the past, but sufficient clock drift has occurred such that the clock error is now > 5 seconds.
  MinutesGranularity = 0x0001,
  ///This indicates the node is synchronized to an upstream source using a low resolution protocol. UTC Time is accurate to ± 5 seconds.
  SecondsGranularity = 0x0002,
  ///This indicates the node is synchronized to an upstream source using high resolution time-synchronization protocol such as NTP, or has built-in GNSS with some amount of jitter applying its GNSS timestamp. UTC Time is accurate to ± 50 ms.
  MillisecondsGranularity = 0x0003,
  ///This indicates the node is synchronized to an upstream source using a highly precise time-synchronization protocol such as PTP, or has built-in GNSS. UTC time is accurate to ± 10 μs.
  MicrosecondsGranularity = 0x0004,
}
impl AttrType for GranularityEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "NoTimeGranularity"),
  (0x0001, "MinutesGranularity"),
  (0x0002, "SecondsGranularity"),
  (0x0003, "MillisecondsGranularity"),
  (0x0004, "MicrosecondsGranularity"),
  ]);
}
impl TlvPut for GranularityEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for GranularityEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::NoTimeGranularity),
      0x0001 => Ok(Self::MinutesGranularity),
      0x0002 => Ok(Self::SecondsGranularity),
      0x0003 => Ok(Self::MillisecondsGranularity),
      0x0004 => Ok(Self::MicrosecondsGranularity),
      _ => Err(())
    }
  }
}
impl TlvDecode for GranularityEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StatusCodeEnum {
  ///Node rejected the attempt to set the UTC time
  TimeNotAccepted = 0x0002,
}
impl AttrType for StatusCodeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0002, "TimeNotAccepted"),
  ]);
}
impl TlvPut for StatusCodeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StatusCodeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0002 => Ok(Self::TimeNotAccepted),
      _ => Err(())
    }
  }
}
impl TlvDecode for StatusCodeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TimeSourceEnum {
  ///Node is not currently synchronized with a UTC Time source.
  None = 0x0000,
  ///Node uses an unlisted time source.
  Unknown = 0x0001,
  ///Node received time from a client using the SetUTCTime Command.
  Admin = 0x0002,
  ///Synchronized time by querying the Time Synchronization cluster of another Node.
  NodeTimeCluster = 0x0003,
  ///SNTP from a server not in the Matter network. NTS is not used.
  NonMatterSNTP = 0x0004,
  ///NTP from servers not in the Matter network. None of the servers used NTS.
  NonMatterNTP = 0x0005,
  ///SNTP from a server within the Matter network. NTS is not used.
  MatterSNTP = 0x0006,
  ///NTP from servers within the Matter network. None of the servers used NTS.
  MatterNTP = 0x0007,
  ///NTP from multiple servers in the Matter network and external. None of the servers used NTS.
  MixedNTP = 0x0008,
  ///SNTP from a server not in the Matter network. NTS is used.
  NonMatterSNTPNTS = 0x0009,
  ///NTP from servers not in the Matter network. NTS is used on at least one server.
  NonMatterNTPNTS = 0x000a,
  ///SNTP from a server within the Matter network. NTS is used.
  MatterSNTPNTS = 0x000b,
  ///NTP from a server within the Matter network. NTS is used on at least one server.
  MatterNTPNTS = 0x000c,
  ///NTP from multiple servers in the Matter network and external. NTS is used on at least one server.
  MixedNTPNTS = 0x000d,
  ///Time synchronization comes from a vendor cloud-based source (e.g. "Date" header in authenticated HTTPS connection).
  CloudSource = 0x000e,
  ///Time synchronization comes from PTP.
  PTP = 0x000f,
  ///Time synchronization comes from a GNSS source.
  GNSS = 0x0010,
}
impl AttrType for TimeSourceEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "None"),
  (0x0001, "Unknown"),
  (0x0002, "Admin"),
  (0x0003, "NodeTimeCluster"),
  (0x0004, "NonMatterSNTP"),
  (0x0005, "NonMatterNTP"),
  (0x0006, "MatterSNTP"),
  (0x0007, "MatterNTP"),
  (0x0008, "MixedNTP"),
  (0x0009, "NonMatterSNTPNTS"),
  (0x000a, "NonMatterNTPNTS"),
  (0x000b, "MatterSNTPNTS"),
  (0x000c, "MatterNTPNTS"),
  (0x000d, "MixedNTPNTS"),
  (0x000e, "CloudSource"),
  (0x000f, "PTP"),
  (0x0010, "GNSS"),
  ]);
}
impl TlvPut for TimeSourceEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TimeSourceEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::None),
      0x0001 => Ok(Self::Unknown),
      0x0002 => Ok(Self::Admin),
      0x0003 => Ok(Self::NodeTimeCluster),
      0x0004 => Ok(Self::NonMatterSNTP),
      0x0005 => Ok(Self::NonMatterNTP),
      0x0006 => Ok(Self::MatterSNTP),
      0x0007 => Ok(Self::MatterNTP),
      0x0008 => Ok(Self::MixedNTP),
      0x0009 => Ok(Self::NonMatterSNTPNTS),
      0x000a => Ok(Self::NonMatterNTPNTS),
      0x000b => Ok(Self::MatterSNTPNTS),
      0x000c => Ok(Self::MatterNTPNTS),
      0x000d => Ok(Self::MixedNTPNTS),
      0x000e => Ok(Self::CloudSource),
      0x000f => Ok(Self::PTP),
      0x0010 => Ok(Self::GNSS),
      _ => Err(())
    }
  }
}
impl TlvDecode for TimeSourceEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TimeZoneDatabaseEnum {
  ///Node has a full list of the available time zones
  Full = 0x0000,
  ///Node has a partial list of the available time zones
  Partial = 0x0001,
  ///Node does not have a time zone database
  None = 0x0002,
}
impl AttrType for TimeZoneDatabaseEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Full"),
  (0x0001, "Partial"),
  (0x0002, "None"),
  ]);
}
impl TlvPut for TimeZoneDatabaseEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TimeZoneDatabaseEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Full),
      0x0001 => Ok(Self::Partial),
      0x0002 => Ok(Self::None),
      _ => Err(())
    }
  }
}
impl TlvDecode for TimeZoneDatabaseEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DSTOffsetStruct {
  pub offset: i32,
  pub valid_starting: TypeUnsupported /* EpochUs */,
  pub valid_until: TypeUnsupported /* EpochUs */,
}
impl AttrType for DSTOffsetStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Offset", attr_ty: <i32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ValidStarting", attr_ty: <TypeUnsupported /* EpochUs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "ValidUntil", attr_ty: <TypeUnsupported /* EpochUs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DSTOffsetStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.offset.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.valid_starting.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.valid_until.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DSTOffsetStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut offset: Option<i32> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { offset = Some(<i32>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      offset: offset.ok_or(TlvDecodeErr::MissingField("Offset"))?,
      valid_starting: TypeUnsupported,
      valid_until: TypeUnsupported,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FabricScopedTrustedTimeSourceStruct {
  pub node_id: TypeUnsupported /* NodeId */,
  pub endpoint: TypeUnsupported /* EndpointNo */,
}
impl AttrType for FabricScopedTrustedTimeSourceStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Endpoint", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for FabricScopedTrustedTimeSourceStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.endpoint.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for FabricScopedTrustedTimeSourceStruct {
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
      node_id: TypeUnsupported,
      endpoint: TypeUnsupported,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeZoneStruct {
  pub offset: i32,
  pub valid_at: TypeUnsupported /* EpochUs */,
  pub name: String,
}
impl AttrType for TimeZoneStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Offset", attr_ty: <i32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ValidAt", attr_ty: <TypeUnsupported /* EpochUs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TimeZoneStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.offset.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.valid_at.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.name.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TimeZoneStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut offset: Option<i32> = None;
    let mut name: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { offset = Some(<i32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      offset: offset.ok_or(TlvDecodeErr::MissingField("Offset"))?,
      valid_at: TypeUnsupported,
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrustedTimeSourceStruct {
  pub fabric_index: TypeUnsupported /* FabricIdx */,
  pub node_id: TypeUnsupported /* NodeId */,
  pub endpoint: TypeUnsupported /* EndpointNo */,
}
impl AttrType for TrustedTimeSourceStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "FabricIndex", attr_ty: <TypeUnsupported /* FabricIdx */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Endpoint", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TrustedTimeSourceStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.fabric_index.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.endpoint.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TrustedTimeSourceStruct {
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
      fabric_index: TypeUnsupported,
      node_id: TypeUnsupported,
      endpoint: TypeUnsupported,
    })
  }
}

}
