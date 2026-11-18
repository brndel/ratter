#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const CHIME: ClusterDef = ClusterDef {
  id: 0x0556,
  name: "Chime",
  features: &[
  ],
  attributes: &[
    attr::InstalledChimeSounds::ATTR,
    attr::SelectedChime::ATTR,
    attr::Enabled::ATTR,
  ],
  events: &[
    event::ChimeStartedPlaying::EVENT,
  ],
  commands: &[
    command::PlayChimeSound::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct InstalledChimeSounds;

  impl Attr for InstalledChimeSounds {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "InstalledChimeSounds",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::ChimeSoundStruct>;
  }


  pub struct SelectedChime;

  impl Attr for SelectedChime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "SelectedChime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct Enabled;

  impl Attr for Enabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "Enabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ChimeStartedPlaying {
  pub chime_id: u8,
}

impl Event for ChimeStartedPlaying {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "ChimeStartedPlaying",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ChimeID", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for ChimeStartedPlaying {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut chime_id: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { chime_id = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      chime_id: chime_id.ok_or(TlvDecodeErr::MissingField("ChimeID"))?,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct PlayChimeSound {
  pub chime_id: u8,
}
impl Command for PlayChimeSound {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "PlayChimeSound",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ChimeID", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for PlayChimeSound {
  type Response = ();
}
impl TlvEncode for PlayChimeSound {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.chime_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ChimeSoundStruct {
  pub chime_id: u8,
  pub name: String,
}
impl AttrType for ChimeSoundStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ChimeID", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ChimeSoundStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.chime_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ChimeSoundStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut chime_id: Option<u8> = None;
    let mut name: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { chime_id = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      chime_id: chime_id.ok_or(TlvDecodeErr::MissingField("ChimeID"))?,
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
    })
  }
}

}
