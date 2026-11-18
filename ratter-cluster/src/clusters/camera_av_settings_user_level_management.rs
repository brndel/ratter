#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const CAMERA_AV_SETTINGS_USER_LEVEL_MANAGEMENT: ClusterDef = ClusterDef {
  id: 0x0552,
  name: "Camera AV Settings User Level Management",
  features: &[
    FeatureDef { bit: 0x0000, code: "DPTZ", name: "DigitalPTZ", summary: "Digital PTZ support" },
    FeatureDef { bit: 0x0001, code: "MPAN", name: "MechanicalPan", summary: "Mechanical Pan support" },
    FeatureDef { bit: 0x0002, code: "MTILT", name: "MechanicalTilt", summary: "Mechanical Tilt support" },
    FeatureDef { bit: 0x0003, code: "MZOOM", name: "MechanicalZoom", summary: "Mechanical Zoom support" },
    FeatureDef { bit: 0x0004, code: "MPRESETS", name: "MechanicalPresets", summary: "Mechanical saved presets support" },
  ],
  attributes: &[
    attr::MPTZPosition::ATTR,
    attr::MaxPresets::ATTR,
    attr::MPTZPresets::ATTR,
    attr::DPTZStreams::ATTR,
    attr::ZoomMax::ATTR,
    attr::TiltMin::ATTR,
    attr::TiltMax::ATTR,
    attr::PanMin::ATTR,
    attr::PanMax::ATTR,
    attr::MovementState::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::MPTZSetPosition::COMMAND,
    command::MPTZRelativeMove::COMMAND,
    command::MPTZMoveToPreset::COMMAND,
    command::MPTZSavePreset::COMMAND,
    command::MPTZRemovePreset::COMMAND,
    command::DPTZSetViewport::COMMAND,
    command::DPTZRelativeMove::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct MPTZPosition;

  impl Attr for MPTZPosition {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "MPTZPosition",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::MPTZStruct;
  }


  pub struct MaxPresets;

  impl Attr for MaxPresets {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "MaxPresets",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = u8;
  }


  pub struct MPTZPresets;

  impl Attr for MPTZPresets {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "MPTZPresets",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = Vec<ty::MPTZPresetStruct>;
  }


  pub struct DPTZStreams;

  impl Attr for DPTZStreams {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "DPTZStreams",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<ty::DPTZStruct>;
  }


  pub struct ZoomMax;

  impl Attr for ZoomMax {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "ZoomMax",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = u8;
  }


  pub struct TiltMin;

  impl Attr for TiltMin {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "TiltMin",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = i16;
  }


  pub struct TiltMax;

  impl Attr for TiltMax {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "TiltMax",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = i16;
  }


  pub struct PanMin;

  impl Attr for PanMin {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "PanMin",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = i16;
  }


  pub struct PanMax;

  impl Attr for PanMax {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "PanMax",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = i16;
  }


  pub struct MovementState;

  impl Attr for MovementState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "MovementState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::PhysicalMovementEnum;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct MPTZSetPosition {
  pub pan: i16,
  pub tilt: i16,
  pub zoom: u8,
}
impl Command for MPTZSetPosition {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "MPTZSetPosition",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Pan", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0001, name: "Tilt", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "Zoom", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for MPTZSetPosition {
  type Response = BasicAccResponse;
}
impl TlvEncode for MPTZSetPosition {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.pan.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.tilt.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.zoom.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MPTZRelativeMove {
  pub pan_delta: i16,
  pub tilt_delta: i16,
  pub zoom_delta: i8,
}
impl Command for MPTZRelativeMove {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "MPTZRelativeMove",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "PanDelta", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0001, name: "TiltDelta", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "ZoomDelta", attr_ty: <i8>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for MPTZRelativeMove {
  type Response = BasicAccResponse;
}
impl TlvEncode for MPTZRelativeMove {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.pan_delta.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.tilt_delta.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.zoom_delta.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MPTZMoveToPreset {
  pub preset_id: u8,
}
impl Command for MPTZMoveToPreset {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "MPTZMoveToPreset",
    conformance: Conformance::Feature(4),
    fields: &[
      AttrDef { id: 0x0000, name: "PresetID", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for MPTZMoveToPreset {
  type Response = BasicAccResponse;
}
impl TlvEncode for MPTZMoveToPreset {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.preset_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MPTZSavePreset {
  pub preset_id: u8,
  pub name: String,
}
impl Command for MPTZSavePreset {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "MPTZSavePreset",
    conformance: Conformance::Feature(4),
    fields: &[
      AttrDef { id: 0x0000, name: "PresetID", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0001, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for MPTZSavePreset {
  type Response = BasicAccResponse;
}
impl TlvEncode for MPTZSavePreset {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.preset_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MPTZRemovePreset {
  pub preset_id: u8,
}
impl Command for MPTZRemovePreset {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "MPTZRemovePreset",
    conformance: Conformance::Feature(4),
    fields: &[
      AttrDef { id: 0x0000, name: "PresetID", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for MPTZRemovePreset {
  type Response = BasicAccResponse;
}
impl TlvEncode for MPTZRemovePreset {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.preset_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DPTZSetViewport {
  pub video_stream_id: u16,
  pub viewport: ty::ViewportStruct,
}
impl Command for DPTZSetViewport {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "DPTZSetViewport",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "VideoStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Viewport", attr_ty: <ty::ViewportStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for DPTZSetViewport {
  type Response = BasicAccResponse;
}
impl TlvEncode for DPTZSetViewport {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.video_stream_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.viewport.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DPTZRelativeMove {
  pub video_stream_id: u16,
  pub delta_x: i16,
  pub delta_y: i16,
  pub zoom_delta: i8,
}
impl Command for DPTZRelativeMove {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "DPTZRelativeMove",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "VideoStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "DeltaX", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "DeltaY", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0003, name: "ZoomDelta", attr_ty: <i8>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for DPTZRelativeMove {
  type Response = BasicAccResponse;
}
impl TlvEncode for DPTZRelativeMove {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.video_stream_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.delta_x.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.delta_y.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.zoom_delta.tlv_put(matter_codec::Tag::Context(0x0003), writer);
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
pub enum PhysicalMovementEnum {
  ///The camera is idle.
  Idle = 0x0000,
  ///The camera is moving to a new value of Pan, Tilt, and/or Zoom.
  Moving = 0x0001,
}
impl AttrType for PhysicalMovementEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Idle"),
  (0x0001, "Moving"),
  ]);
}
impl TlvPut for PhysicalMovementEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for PhysicalMovementEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Idle),
      0x0001 => Ok(Self::Moving),
      _ => Err(())
    }
  }
}
impl TlvDecode for PhysicalMovementEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DPTZStruct {
  pub video_stream_id: u16,
  pub viewport: ViewportStruct,
}
impl AttrType for DPTZStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "VideoStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Viewport", attr_ty: <ViewportStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DPTZStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.video_stream_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.viewport.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DPTZStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut video_stream_id: Option<u16> = None;
    let mut viewport: Option<ViewportStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { video_stream_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { viewport = Some(<ViewportStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      video_stream_id: video_stream_id.ok_or(TlvDecodeErr::MissingField("VideoStreamID"))?,
      viewport: viewport.ok_or(TlvDecodeErr::MissingField("Viewport"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MPTZPresetStruct {
  pub preset_id: u8,
  pub name: String,
  pub settings: MPTZStruct,
}
impl AttrType for MPTZPresetStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "PresetID", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Settings", attr_ty: <MPTZStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for MPTZPresetStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.preset_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.settings.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for MPTZPresetStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut preset_id: Option<u8> = None;
    let mut name: Option<String> = None;
    let mut settings: Option<MPTZStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { preset_id = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { settings = Some(<MPTZStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      preset_id: preset_id.ok_or(TlvDecodeErr::MissingField("PresetID"))?,
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
      settings: settings.ok_or(TlvDecodeErr::MissingField("Settings"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MPTZStruct {
  pub pan: i16,
  pub tilt: i16,
  pub zoom: u8,
}
impl AttrType for MPTZStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Pan", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Tilt", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Zoom", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for MPTZStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.pan.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.tilt.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.zoom.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for MPTZStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut pan: Option<i16> = None;
    let mut tilt: Option<i16> = None;
    let mut zoom: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { pan = Some(<i16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { tilt = Some(<i16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { zoom = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      pan: pan.ok_or(TlvDecodeErr::MissingField("Pan"))?,
      tilt: tilt.ok_or(TlvDecodeErr::MissingField("Tilt"))?,
      zoom: zoom.ok_or(TlvDecodeErr::MissingField("Zoom"))?,
    })
  }
}

}
