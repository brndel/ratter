#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const ZONE_MANAGEMENT: ClusterDef = ClusterDef {
  id: 0x0550,
  name: "Zone Management",
  features: &[
    FeatureDef { bit: 0x0000, code: "TWODCART", name: "TwoDimensionalCartesianZone", summary: "Supports Two Dimensional Cartesian Zones" },
    FeatureDef { bit: 0x0001, code: "PERZONESENS", name: "PerZoneSensitivity", summary: "Supports a sensitivity value per Zone" },
    FeatureDef { bit: 0x0002, code: "USERDEFINED", name: "UserDefined", summary: "Supports user defined zones" },
    FeatureDef { bit: 0x0003, code: "FOCUSZONES", name: "FocusZones", summary: "Supports user defined focus zones" },
  ],
  attributes: &[
    attr::MaxUserDefinedZones::ATTR,
    attr::MaxZones::ATTR,
    attr::Zones::ATTR,
    attr::Triggers::ATTR,
    attr::SensitivityMax::ATTR,
    attr::Sensitivity::ATTR,
    attr::TwoDCartesianMax::ATTR,
  ],
  events: &[
    event::ZoneTriggered::EVENT,
    event::ZoneStopped::EVENT,
  ],
  commands: &[
    command::CreateTwoDCartesianZone::COMMAND,
    command::UpdateTwoDCartesianZone::COMMAND,
    command::RemoveZone::COMMAND,
    command::CreateOrUpdateTrigger::COMMAND,
    command::RemoveTrigger::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct MaxUserDefinedZones;

  impl Attr for MaxUserDefinedZones {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "MaxUserDefinedZones",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = u8;
  }


  pub struct MaxZones;

  impl Attr for MaxZones {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "MaxZones",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct Zones;

  impl Attr for Zones {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "Zones",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::ZoneInformationStruct>;
  }


  pub struct Triggers;

  impl Attr for Triggers {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "Triggers",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::ZoneTriggerControlStruct>;
  }


  pub struct SensitivityMax;

  impl Attr for SensitivityMax {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "SensitivityMax",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct Sensitivity;

  impl Attr for Sensitivity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "Sensitivity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct TwoDCartesianMax;

  impl Attr for TwoDCartesianMax {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "TwoDCartesianMax",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = ty::TwoDCartesianVertexStruct;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ZoneTriggered {
  pub zone: u16,
  pub reason: ty::ZoneEventTriggeredReasonEnum,
}

impl Event for ZoneTriggered {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "ZoneTriggered",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Zone", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Reason", attr_ty: <ty::ZoneEventTriggeredReasonEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for ZoneTriggered {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut zone: Option<u16> = None;
    let mut reason: Option<ty::ZoneEventTriggeredReasonEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { zone = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { reason = Some(<ty::ZoneEventTriggeredReasonEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      zone: zone.ok_or(TlvDecodeErr::MissingField("Zone"))?,
      reason: reason.ok_or(TlvDecodeErr::MissingField("Reason"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ZoneStopped {
  pub zone: u16,
  pub reason: ty::ZoneEventStoppedReasonEnum,
}

impl Event for ZoneStopped {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "ZoneStopped",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Zone", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Reason", attr_ty: <ty::ZoneEventStoppedReasonEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for ZoneStopped {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut zone: Option<u16> = None;
    let mut reason: Option<ty::ZoneEventStoppedReasonEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { zone = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { reason = Some(<ty::ZoneEventStoppedReasonEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      zone: zone.ok_or(TlvDecodeErr::MissingField("Zone"))?,
      reason: reason.ok_or(TlvDecodeErr::MissingField("Reason"))?,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct CreateTwoDCartesianZone {
  pub zone: ty::TwoDCartesianZoneStruct,
}
impl Command for CreateTwoDCartesianZone {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "CreateTwoDCartesianZone",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Zone", attr_ty: <ty::TwoDCartesianZoneStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for CreateTwoDCartesianZone {
  type Response = CreateTwoDCartesianZoneResponse;
}
impl TlvEncode for CreateTwoDCartesianZone {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.zone.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateTwoDCartesianZoneResponse {
  pub zone_id: u16,
}
impl Command for CreateTwoDCartesianZoneResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "CreateTwoDCartesianZoneResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ZoneID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for CreateTwoDCartesianZoneResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateTwoDCartesianZone {
  pub zone_id: u16,
  pub zone: ty::TwoDCartesianZoneStruct,
}
impl Command for UpdateTwoDCartesianZone {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "UpdateTwoDCartesianZone",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ZoneID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Zone", attr_ty: <ty::TwoDCartesianZoneStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for UpdateTwoDCartesianZone {
  type Response = BasicAccResponse;
}
impl TlvEncode for UpdateTwoDCartesianZone {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.zone_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.zone.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveZone {
  pub zone_id: u16,
}
impl Command for RemoveZone {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "RemoveZone",
    conformance: Conformance::Feature(2),
    fields: &[
      AttrDef { id: 0x0000, name: "ZoneID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveZone {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveZone {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.zone_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CreateOrUpdateTrigger {
  pub trigger: ty::ZoneTriggerControlStruct,
}
impl Command for CreateOrUpdateTrigger {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "CreateOrUpdateTrigger",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Trigger", attr_ty: <ty::ZoneTriggerControlStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for CreateOrUpdateTrigger {
  type Response = BasicAccResponse;
}
impl TlvEncode for CreateOrUpdateTrigger {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.trigger.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveTrigger {
  pub zone_id: u16,
}
impl Command for RemoveTrigger {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "RemoveTrigger",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ZoneID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveTrigger {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveTrigger {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.zone_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
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
pub enum ZoneEventStoppedReasonEnum {
  ///Indicates that whatever triggered the Zone event has stopped being detected.
  ActionStopped = 0x0000,
  ///Indicates that the max duration for detecting triggering activity has been reached.
  Timeout = 0x0001,
}
impl AttrType for ZoneEventStoppedReasonEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "ActionStopped"),
  (0x0001, "Timeout"),
  ]);
}
impl TlvPut for ZoneEventStoppedReasonEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ZoneEventStoppedReasonEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::ActionStopped),
      0x0001 => Ok(Self::Timeout),
      _ => Err(())
    }
  }
}
impl TlvDecode for ZoneEventStoppedReasonEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ZoneEventTriggeredReasonEnum {
  ///Zone event triggered because motion is detected
  Motion = 0x0000,
}
impl AttrType for ZoneEventTriggeredReasonEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Motion"),
  ]);
}
impl TlvPut for ZoneEventTriggeredReasonEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ZoneEventTriggeredReasonEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Motion),
      _ => Err(())
    }
  }
}
impl TlvDecode for ZoneEventTriggeredReasonEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ZoneSourceEnum {
  ///Indicates a Manufacturer defined Zone.
  Mfg = 0x0000,
  ///Indicates a User defined Zone.
  User = 0x0001,
}
impl AttrType for ZoneSourceEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Mfg"),
  (0x0001, "User"),
  ]);
}
impl TlvPut for ZoneSourceEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ZoneSourceEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Mfg),
      0x0001 => Ok(Self::User),
      _ => Err(())
    }
  }
}
impl TlvDecode for ZoneSourceEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ZoneTypeEnum {
  ///Indicates a Two Dimensional Cartesian Zone
  TwoDCARTZone = 0x0000,
}
impl AttrType for ZoneTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "TwoDCARTZone"),
  ]);
}
impl TlvPut for ZoneTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ZoneTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::TwoDCARTZone),
      _ => Err(())
    }
  }
}
impl TlvDecode for ZoneTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ZoneUseEnum {
  ///Indicates Zone is intended to detect Motion
  Motion = 0x0000,
  ///Indicates Zone is intended to protect privacy
  Privacy = 0x0001,
  ///Indicates Zone provides a focus area
  Focus = 0x0002,
}
impl AttrType for ZoneUseEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Motion"),
  (0x0001, "Privacy"),
  (0x0002, "Focus"),
  ]);
}
impl TlvPut for ZoneUseEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ZoneUseEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Motion),
      0x0001 => Ok(Self::Privacy),
      0x0002 => Ok(Self::Focus),
      _ => Err(())
    }
  }
}
impl TlvDecode for ZoneUseEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct TwoDCartesianVertexStruct {
  pub x: u16,
  pub y: u16,
}
impl AttrType for TwoDCartesianVertexStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "X", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Y", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TwoDCartesianVertexStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.x.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.y.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TwoDCartesianVertexStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut x: Option<u16> = None;
    let mut y: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { x = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { y = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      x: x.ok_or(TlvDecodeErr::MissingField("X"))?,
      y: y.ok_or(TlvDecodeErr::MissingField("Y"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TwoDCartesianZoneStruct {
  pub name: String,
  pub r#use: ZoneUseEnum,
  pub vertices: Vec<TwoDCartesianVertexStruct>,
  pub color: String,
}
impl AttrType for TwoDCartesianZoneStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Use", attr_ty: <ZoneUseEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Vertices", attr_ty: <Vec<TwoDCartesianVertexStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "Color", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TwoDCartesianZoneStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.name.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.r#use.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.vertices.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.color.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TwoDCartesianZoneStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut name: Option<String> = None;
    let mut r#use: Option<ZoneUseEnum> = None;
    let mut vertices: Option<Vec<TwoDCartesianVertexStruct>> = None;
    let mut color: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { r#use = Some(<ZoneUseEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { vertices = Some(<Vec<TwoDCartesianVertexStruct>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { color = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
      r#use: r#use.ok_or(TlvDecodeErr::MissingField("Use"))?,
      vertices: vertices.ok_or(TlvDecodeErr::MissingField("Vertices"))?,
      color: color.ok_or(TlvDecodeErr::MissingField("Color"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ZoneInformationStruct {
  pub zone_id: u16,
  pub zone_type: ZoneTypeEnum,
  pub zone_source: ZoneSourceEnum,
  pub two_d_cartesian_zone: TwoDCartesianZoneStruct,
}
impl AttrType for ZoneInformationStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ZoneID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ZoneType", attr_ty: <ZoneTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "ZoneSource", attr_ty: <ZoneSourceEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "TwoDCartesianZone", attr_ty: <TwoDCartesianZoneStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ZoneInformationStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.zone_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.zone_type.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.zone_source.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.two_d_cartesian_zone.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ZoneInformationStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut zone_id: Option<u16> = None;
    let mut zone_type: Option<ZoneTypeEnum> = None;
    let mut zone_source: Option<ZoneSourceEnum> = None;
    let mut two_d_cartesian_zone: Option<TwoDCartesianZoneStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { zone_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { zone_type = Some(<ZoneTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { zone_source = Some(<ZoneSourceEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0003), kind: _ }) => { two_d_cartesian_zone = Some(<TwoDCartesianZoneStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      zone_id: zone_id.ok_or(TlvDecodeErr::MissingField("ZoneID"))?,
      zone_type: zone_type.ok_or(TlvDecodeErr::MissingField("ZoneType"))?,
      zone_source: zone_source.ok_or(TlvDecodeErr::MissingField("ZoneSource"))?,
      two_d_cartesian_zone: two_d_cartesian_zone.ok_or(TlvDecodeErr::MissingField("TwoDCartesianZone"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ZoneTriggerControlStruct {
  pub zone_id: u16,
  pub initial_duration: TypeUnsupported /* ElapsedS */,
  pub augmentation_duration: TypeUnsupported /* ElapsedS */,
  pub max_duration: TypeUnsupported /* ElapsedS */,
  pub blind_duration: TypeUnsupported /* ElapsedS */,
  pub sensitivity: u8,
}
impl AttrType for ZoneTriggerControlStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ZoneID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "InitialDuration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "AugmentationDuration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "MaxDuration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "BlindDuration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "Sensitivity", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ZoneTriggerControlStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.zone_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.initial_duration.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.augmentation_duration.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.max_duration.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.blind_duration.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.sensitivity.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ZoneTriggerControlStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut zone_id: Option<u16> = None;
    let mut sensitivity: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { zone_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { sensitivity = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      zone_id: zone_id.ok_or(TlvDecodeErr::MissingField("ZoneID"))?,
      initial_duration: TypeUnsupported,
      augmentation_duration: TypeUnsupported,
      max_duration: TypeUnsupported,
      blind_duration: TypeUnsupported,
      sensitivity: sensitivity.ok_or(TlvDecodeErr::MissingField("Sensitivity"))?,
    })
  }
}

}
