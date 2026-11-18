#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const SERVICE_AREA: ClusterDef = ClusterDef {
  id: 0x0150,
  name: "Service Area",
  features: &[
    FeatureDef { bit: 0x0000, code: "SELRUN", name: "SelectWhileRunning", summary: "The device allows changing the selected areas while running" },
    FeatureDef { bit: 0x0001, code: "PROG", name: "ProgressReporting", summary: "The device implements the progress reporting feature" },
    FeatureDef { bit: 0x0002, code: "MAPS", name: "Maps", summary: "The device has map support" },
  ],
  attributes: &[
    attr::SupportedAreas::ATTR,
    attr::SupportedMaps::ATTR,
    attr::SelectedAreas::ATTR,
    attr::CurrentArea::ATTR,
    attr::EstimatedEndTime::ATTR,
    attr::Progress::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::SelectAreas::COMMAND,
    command::SkipArea::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct SupportedAreas;

  impl Attr for SupportedAreas {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "SupportedAreas",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::AreaStruct>;
  }


  pub struct SupportedMaps;

  impl Attr for SupportedMaps {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "SupportedMaps",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = Vec<ty::MapStruct>;
  }


  pub struct SelectedAreas;

  impl Attr for SelectedAreas {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "SelectedAreas",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<u32>;
  }


  pub struct CurrentArea;

  impl Attr for CurrentArea {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "CurrentArea",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<u32>;
  }


  pub struct EstimatedEndTime;

  impl Attr for EstimatedEndTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "EstimatedEndTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<TypeUnsupported /* EpochS */>;
  }


  pub struct Progress;

  impl Attr for Progress {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "Progress",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Vec<ty::ProgressStruct>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SelectAreas {
  pub new_areas: Vec<u32>,
}
impl Command for SelectAreas {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "SelectAreas",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NewAreas", attr_ty: <Vec<u32>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SelectAreas {
  type Response = SelectAreasResponse;
}
impl TlvEncode for SelectAreas {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.new_areas.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SelectAreasResponse {
  pub status: ty::SelectAreasStatus,
  pub status_text: String,
}
impl Command for SelectAreasResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "SelectAreasResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <ty::SelectAreasStatus>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "StatusText", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for SelectAreasResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct SkipArea {
  pub skipped_area: u32,
}
impl Command for SkipArea {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "SkipArea",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "SkippedArea", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SkipArea {
  type Response = SkipAreaResponse;
}
impl TlvEncode for SkipArea {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.skipped_area.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SkipAreaResponse {
  pub status: ty::SkipAreaStatus,
  pub status_text: String,
}
impl Command for SkipAreaResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "SkipAreaResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <ty::SkipAreaStatus>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "StatusText", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for SkipAreaResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum OperationalStatusEnum {
  ///The device has not yet started operating at the given area, or has not finished operating at that area but it is not currently operating at the area
  Pending = 0x0000,
  ///The device is currently operating at the given area
  Operating = 0x0001,
  ///The device has skipped the given area, before or during operating at it, due to a SkipArea command, due an out of band command (e.g. from the vendor's application), due to a vendor specific reason, such as a time limit used by the device, or due the device ending operating unsuccessfully
  Skipped = 0x0002,
  ///The device has completed operating at the given area
  Completed = 0x0003,
}
impl AttrType for OperationalStatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Pending"),
  (0x0001, "Operating"),
  (0x0002, "Skipped"),
  (0x0003, "Completed"),
  ]);
}
impl TlvPut for OperationalStatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for OperationalStatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Pending),
      0x0001 => Ok(Self::Operating),
      0x0002 => Ok(Self::Skipped),
      0x0003 => Ok(Self::Completed),
      _ => Err(())
    }
  }
}
impl TlvDecode for OperationalStatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum SelectAreasStatus {
  ///Attempting to operate in the areas identified by the entries of the NewAreas field is allowed and possible, the SelectedAreas attribute is set to the value of the NewAreas field
  Success = 0x0000,
  ///The value of at least one of the entries of the NewAreas field doesn't match any entries in the SupportedAreas attribute
  UnsupportedArea = 0x0001,
  ///The received request cannot be handled due to the current mode of the device
  InvalidInMode = 0x0002,
  ///The set of values is invalid. For example, areas on different floors, that a robot knows it can't reach on its own
  InvalidSet = 0x0003,
}
impl AttrType for SelectAreasStatus {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Success"),
  (0x0001, "UnsupportedArea"),
  (0x0002, "InvalidInMode"),
  (0x0003, "InvalidSet"),
  ]);
}
impl TlvPut for SelectAreasStatus {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for SelectAreasStatus {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Success),
      0x0001 => Ok(Self::UnsupportedArea),
      0x0002 => Ok(Self::InvalidInMode),
      0x0003 => Ok(Self::InvalidSet),
      _ => Err(())
    }
  }
}
impl TlvDecode for SelectAreasStatus {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum SkipAreaStatus {
  ///Skipping the area is allowed and possible, or the device was operating at the last available area and has stopped
  Success = 0x0000,
  ///The SelectedAreas attribute is empty.
  InvalidAreaList = 0x0001,
  ///The received request cannot be handled due to the current mode of the device. For example, the CurrentArea attribute is null or the device is not operating
  InvalidInMode = 0x0002,
  ///The SkippedArea field doesn't match an entry in the SupportedAreas list
  InvalidSkippedArea = 0x0003,
}
impl AttrType for SkipAreaStatus {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Success"),
  (0x0001, "InvalidAreaList"),
  (0x0002, "InvalidInMode"),
  (0x0003, "InvalidSkippedArea"),
  ]);
}
impl TlvPut for SkipAreaStatus {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for SkipAreaStatus {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Success),
      0x0001 => Ok(Self::InvalidAreaList),
      0x0002 => Ok(Self::InvalidInMode),
      0x0003 => Ok(Self::InvalidSkippedArea),
      _ => Err(())
    }
  }
}
impl TlvDecode for SkipAreaStatus {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AreaInfoStruct {
  pub location_info: LocationDescriptorStruct,
  pub landmark_info: LandmarkInfoStruct,
}
impl AttrType for AreaInfoStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "LocationInfo", attr_ty: <LocationDescriptorStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "LandmarkInfo", attr_ty: <LandmarkInfoStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for AreaInfoStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.location_info.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.landmark_info.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for AreaInfoStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut location_info: Option<LocationDescriptorStruct> = None;
    let mut landmark_info: Option<LandmarkInfoStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { location_info = Some(<LocationDescriptorStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { landmark_info = Some(<LandmarkInfoStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      location_info: location_info.ok_or(TlvDecodeErr::MissingField("LocationInfo"))?,
      landmark_info: landmark_info.ok_or(TlvDecodeErr::MissingField("LandmarkInfo"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AreaStruct {
  pub area_id: u32,
  pub map_id: u32,
  pub area_info: AreaInfoStruct,
}
impl AttrType for AreaStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "AreaID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "MapID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "AreaInfo", attr_ty: <AreaInfoStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for AreaStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.area_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.map_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.area_info.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for AreaStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut area_id: Option<u32> = None;
    let mut map_id: Option<u32> = None;
    let mut area_info: Option<AreaInfoStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { area_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { map_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { area_info = Some(<AreaInfoStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      area_id: area_id.ok_or(TlvDecodeErr::MissingField("AreaID"))?,
      map_id: map_id.ok_or(TlvDecodeErr::MissingField("MapID"))?,
      area_info: area_info.ok_or(TlvDecodeErr::MissingField("AreaInfo"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LandmarkInfoStruct {
  pub landmark_tag: TypeUnsupported /* Tag */,
  pub relative_position_tag: TypeUnsupported /* Tag */,
}
impl AttrType for LandmarkInfoStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "LandmarkTag", attr_ty: <TypeUnsupported /* Tag */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "RelativePositionTag", attr_ty: <TypeUnsupported /* Tag */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for LandmarkInfoStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.landmark_tag.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.relative_position_tag.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for LandmarkInfoStruct {
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
      landmark_tag: TypeUnsupported,
      relative_position_tag: TypeUnsupported,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MapStruct {
  pub map_id: u32,
  pub name: String,
}
impl AttrType for MapStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "MapID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for MapStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.map_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for MapStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut map_id: Option<u32> = None;
    let mut name: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { map_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      map_id: map_id.ok_or(TlvDecodeErr::MissingField("MapID"))?,
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressStruct {
  pub area_id: u32,
  pub status: OperationalStatusEnum,
  pub total_operational_time: TypeUnsupported /* ElapsedS */,
  pub estimated_time: TypeUnsupported /* ElapsedS */,
}
impl AttrType for ProgressStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "AreaID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Status", attr_ty: <OperationalStatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "TotalOperationalTime", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "EstimatedTime", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ProgressStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.area_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.status.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.total_operational_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.estimated_time.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ProgressStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut area_id: Option<u32> = None;
    let mut status: Option<OperationalStatusEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { area_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { status = Some(<OperationalStatusEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      area_id: area_id.ok_or(TlvDecodeErr::MissingField("AreaID"))?,
      status: status.ok_or(TlvDecodeErr::MissingField("Status"))?,
      total_operational_time: TypeUnsupported,
      estimated_time: TypeUnsupported,
    })
  }
}

}
