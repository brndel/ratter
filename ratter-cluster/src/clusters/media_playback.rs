#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const MEDIA_PLAYBACK: ClusterDef = ClusterDef {
  id: 0x0506,
  name: "Media Playback",
  features: &[
    FeatureDef { bit: 0x0000, code: "AS", name: "AdvancedSeek", summary: "Advanced media seeking" },
    FeatureDef { bit: 0x0001, code: "VS", name: "VariableSpeed", summary: "Variable speed playback" },
    FeatureDef { bit: 0x0002, code: "TT", name: "TextTracks", summary: "Text Tracks" },
    FeatureDef { bit: 0x0003, code: "AT", name: "AudioTracks", summary: "Audio Tracks" },
    FeatureDef { bit: 0x0004, code: "AA", name: "AudioAdvance", summary: "Can play audio during fast and slow playback speeds" },
  ],
  attributes: &[
    attr::CurrentState::ATTR,
    attr::StartTime::ATTR,
    attr::Duration::ATTR,
    attr::SampledPosition::ATTR,
    attr::PlaybackSpeed::ATTR,
    attr::SeekRangeEnd::ATTR,
    attr::SeekRangeStart::ATTR,
    attr::ActiveAudioTrack::ATTR,
    attr::AvailableAudioTracks::ATTR,
    attr::ActiveTextTrack::ATTR,
    attr::AvailableTextTracks::ATTR,
  ],
  events: &[
    event::StateChanged::EVENT,
  ],
  commands: &[
    command::Play::COMMAND,
    command::Pause::COMMAND,
    command::Stop::COMMAND,
    command::StartOver::COMMAND,
    command::Previous::COMMAND,
    command::Next::COMMAND,
    command::Rewind::COMMAND,
    command::FastForward::COMMAND,
    command::SkipForward::COMMAND,
    command::SkipBackward::COMMAND,
    command::Seek::COMMAND,
    command::ActivateAudioTrack::COMMAND,
    command::ActivateTextTrack::COMMAND,
    command::DeactivateTextTrack::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct CurrentState;

  impl Attr for CurrentState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "CurrentState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::PlaybackStateEnum;
  }


  pub struct StartTime;

  impl Attr for StartTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "StartTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<TypeUnsupported /* EpochUs */>;
  }


  pub struct Duration;

  impl Attr for Duration {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "Duration",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<u64>;
  }


  pub struct SampledPosition;

  impl Attr for SampledPosition {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "SampledPosition",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<ty::PlaybackPositionStruct>;
  }


  pub struct PlaybackSpeed;

  impl Attr for PlaybackSpeed {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "PlaybackSpeed",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = f32;
  }


  pub struct SeekRangeEnd;

  impl Attr for SeekRangeEnd {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "SeekRangeEnd",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<u64>;
  }


  pub struct SeekRangeStart;

  impl Attr for SeekRangeStart {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "SeekRangeStart",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<u64>;
  }


  pub struct ActiveAudioTrack;

  impl Attr for ActiveAudioTrack {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "ActiveAudioTrack",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = Nullable<ty::TrackStruct>;
  }


  pub struct AvailableAudioTracks;

  impl Attr for AvailableAudioTracks {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "AvailableAudioTracks",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = Nullable<Vec<ty::TrackStruct>>;
  }


  pub struct ActiveTextTrack;

  impl Attr for ActiveTextTrack {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "ActiveTextTrack",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = Nullable<ty::TrackStruct>;
  }


  pub struct AvailableTextTracks;

  impl Attr for AvailableTextTracks {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "AvailableTextTracks",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = Nullable<Vec<ty::TrackStruct>>;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct StateChanged {
  pub current_state: ty::PlaybackStateEnum,
  pub start_time: TypeUnsupported /* EpochUs */,
  pub duration: u64,
  pub sampled_position: ty::PlaybackPositionStruct,
  pub playback_speed: f32,
  pub seek_range_end: u64,
  pub seek_range_start: u64,
  pub data: TypeUnsupported /* Octstr */,
  pub audio_advance_unmuted: bool,
}

impl Event for StateChanged {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "StateChanged",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "CurrentState", attr_ty: <ty::PlaybackStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "StartTime", attr_ty: <TypeUnsupported /* EpochUs */>::ATTR_TYPE, conformance: Conformance::Feature(0) },
      AttrDef { id: 0x0002, name: "Duration", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Feature(0) },
      AttrDef { id: 0x0003, name: "SampledPosition", attr_ty: <ty::PlaybackPositionStruct>::ATTR_TYPE, conformance: Conformance::Feature(0) },
      AttrDef { id: 0x0004, name: "PlaybackSpeed", attr_ty: <f32>::ATTR_TYPE, conformance: Conformance::Feature(0) },
      AttrDef { id: 0x0005, name: "SeekRangeEnd", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Feature(0) },
      AttrDef { id: 0x0006, name: "SeekRangeStart", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Feature(0) },
      AttrDef { id: 0x0007, name: "Data", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0008, name: "AudioAdvanceUnmuted", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Feature(4) },
    ],
  };
}

impl TlvDecode for StateChanged {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut current_state: Option<ty::PlaybackStateEnum> = None;
    let mut duration: Option<u64> = None;
    let mut sampled_position: Option<ty::PlaybackPositionStruct> = None;
    let mut playback_speed: Option<f32> = None;
    let mut seek_range_end: Option<u64> = None;
    let mut seek_range_start: Option<u64> = None;
    let mut audio_advance_unmuted: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { current_state = Some(<ty::PlaybackStateEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { duration = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0003), kind: _ }) => { sampled_position = Some(<ty::PlaybackPositionStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { playback_speed = Some(<f32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { seek_range_end = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { seek_range_start = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0008), value: _ }) => { audio_advance_unmuted = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      current_state: current_state.ok_or(TlvDecodeErr::MissingField("CurrentState"))?,
      start_time: TypeUnsupported,
      duration: duration.ok_or(TlvDecodeErr::MissingField("Duration"))?,
      sampled_position: sampled_position.ok_or(TlvDecodeErr::MissingField("SampledPosition"))?,
      playback_speed: playback_speed.ok_or(TlvDecodeErr::MissingField("PlaybackSpeed"))?,
      seek_range_end: seek_range_end.ok_or(TlvDecodeErr::MissingField("SeekRangeEnd"))?,
      seek_range_start: seek_range_start.ok_or(TlvDecodeErr::MissingField("SeekRangeStart"))?,
      data: TypeUnsupported,
      audio_advance_unmuted: audio_advance_unmuted.ok_or(TlvDecodeErr::MissingField("AudioAdvanceUnmuted"))?,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Play {
}
impl Command for Play {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "Play",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for Play {
  type Response = PlaybackResponse;
}
impl TlvEncode for Play {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pause {
}
impl Command for Pause {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "Pause",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for Pause {
  type Response = PlaybackResponse;
}
impl TlvEncode for Pause {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stop {
}
impl Command for Stop {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "Stop",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for Stop {
  type Response = PlaybackResponse;
}
impl TlvEncode for Stop {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StartOver {
}
impl Command for StartOver {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "StartOver",
    conformance: Conformance::Optional,
    fields: &[
    ],
  };
}
impl CommandInvoke for StartOver {
  type Response = PlaybackResponse;
}
impl TlvEncode for StartOver {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Previous {
}
impl Command for Previous {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "Previous",
    conformance: Conformance::Optional,
    fields: &[
    ],
  };
}
impl CommandInvoke for Previous {
  type Response = PlaybackResponse;
}
impl TlvEncode for Previous {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Next {
}
impl Command for Next {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "Next",
    conformance: Conformance::Optional,
    fields: &[
    ],
  };
}
impl CommandInvoke for Next {
  type Response = PlaybackResponse;
}
impl TlvEncode for Next {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Rewind {
  pub audio_advance_unmuted: bool,
}
impl Command for Rewind {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "Rewind",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "AudioAdvanceUnmuted", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Feature(4) },
    ],
  };
}
impl CommandInvoke for Rewind {
  type Response = PlaybackResponse;
}
impl TlvEncode for Rewind {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.audio_advance_unmuted.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FastForward {
  pub audio_advance_unmuted: bool,
}
impl Command for FastForward {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "FastForward",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "AudioAdvanceUnmuted", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Feature(4) },
    ],
  };
}
impl CommandInvoke for FastForward {
  type Response = PlaybackResponse;
}
impl TlvEncode for FastForward {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.audio_advance_unmuted.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SkipForward {
  pub delta_position_milliseconds: u64,
}
impl Command for SkipForward {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0008,
    name: "SkipForward",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "DeltaPositionMilliseconds", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SkipForward {
  type Response = PlaybackResponse;
}
impl TlvEncode for SkipForward {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.delta_position_milliseconds.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SkipBackward {
  pub delta_position_milliseconds: u64,
}
impl Command for SkipBackward {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0009,
    name: "SkipBackward",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "DeltaPositionMilliseconds", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SkipBackward {
  type Response = PlaybackResponse;
}
impl TlvEncode for SkipBackward {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.delta_position_milliseconds.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlaybackResponse {
  pub status: ty::StatusEnum,
  pub data: String,
}
impl Command for PlaybackResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000a,
    name: "PlaybackResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <ty::StatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Data", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandResponse for PlaybackResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct Seek {
  pub position: u64,
}
impl Command for Seek {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000b,
    name: "Seek",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "Position", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for Seek {
  type Response = PlaybackResponse;
}
impl TlvEncode for Seek {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.position.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActivateAudioTrack {
  pub track_id: String,
  pub audio_output_index: u8,
}
impl Command for ActivateAudioTrack {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000c,
    name: "ActivateAudioTrack",
    conformance: Conformance::Feature(3),
    fields: &[
      AttrDef { id: 0x0000, name: "TrackID", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "AudioOutputIndex", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Feature(3) },
    ],
  };
}
impl CommandInvoke for ActivateAudioTrack {
  type Response = BasicAccResponse;
}
impl TlvEncode for ActivateAudioTrack {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.track_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.audio_output_index.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ActivateTextTrack {
  pub track_id: String,
}
impl Command for ActivateTextTrack {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000d,
    name: "ActivateTextTrack",
    conformance: Conformance::Feature(2),
    fields: &[
      AttrDef { id: 0x0000, name: "TrackID", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ActivateTextTrack {
  type Response = BasicAccResponse;
}
impl TlvEncode for ActivateTextTrack {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.track_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DeactivateTextTrack {
}
impl Command for DeactivateTextTrack {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000e,
    name: "DeactivateTextTrack",
    conformance: Conformance::Feature(2),
    fields: &[
    ],
  };
}
impl CommandInvoke for DeactivateTextTrack {
  type Response = BasicAccResponse;
}
impl TlvEncode for DeactivateTextTrack {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
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
pub enum CharacteristicEnum {
  ///Textual information meant for display when no other text representation is selected. It is used to clarify dialogue, alternate languages, texted graphics or location/person IDs that are not otherwise covered in the dubbed/localized audio
  ForcedSubtitles = 0x0000,
  ///Textual or audio media component containing a textual description (intended for audio synthesis) or an audio description describing a visual component
  DescribesVideo = 0x0001,
  ///Simplified or reduced captions as specified in [United States Code Title 47 CFR 79.103(c)(9)]
  EasyToRead = 0x0002,
  ///A media characteristic that indicates that a track selection option includes frame-based content
  FrameBased = 0x0003,
  ///Main media component(s) which is/are intended for presentation if no other information is provided
  MainProgram = 0x0004,
  ///A media characteristic that indicates that a track or media selection option contains original content
  OriginalContent = 0x0005,
  ///A media characteristic that indicates that a track or media selection option contains a language translation and verbal interpretation of spoken dialog
  VoiceOverTranslation = 0x0006,
  ///Textual media component containing transcriptions of spoken dialog and auditory cues such as sound effects and music for the hearing impaired
  Caption = 0x0007,
  ///Textual transcriptions of spoken dialog.
  Subtitle = 0x0008,
  ///Textual media component containing transcriptions of spoken dialog and auditory cues such as sound effects and music for the hearing impaired
  Alternate = 0x0009,
  ///Media content component that is supplementary to a media content component of a different media component type
  Supplementary = 0x000a,
  ///Experience that contains a commentary (e.g. director’s commentary) (typically audio)
  Commentary = 0x000b,
  ///Experience that contains an element that is presented in a different language from the original (e.g. dubbed audio, translated captions)
  DubbedTranslation = 0x000c,
  ///Textual or audio media component containing a textual description (intended for audio synthesis) or an audio description describing a visual component
  Description = 0x000d,
  ///Media component containing information intended to be processed by application specific elements
  Metadata = 0x000e,
  ///Experience containing an element for improved intelligibility of the dialogue.
  EnhancedAudioIntelligibility = 0x000f,
  ///Experience that provides information, about a current emergency, that is intended to enable the protection of life, health, safety, and property, and may also include critical details regarding the emergency and how to respond to the emergency
  Emergency = 0x0010,
  ///Textual representation of a songs’ lyrics, usually in the same language as the associated song as specified in [SMPTE ST 2067-2]
  Karaoke = 0x0011,
}
impl AttrType for CharacteristicEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "ForcedSubtitles"),
  (0x0001, "DescribesVideo"),
  (0x0002, "EasyToRead"),
  (0x0003, "FrameBased"),
  (0x0004, "MainProgram"),
  (0x0005, "OriginalContent"),
  (0x0006, "VoiceOverTranslation"),
  (0x0007, "Caption"),
  (0x0008, "Subtitle"),
  (0x0009, "Alternate"),
  (0x000a, "Supplementary"),
  (0x000b, "Commentary"),
  (0x000c, "DubbedTranslation"),
  (0x000d, "Description"),
  (0x000e, "Metadata"),
  (0x000f, "EnhancedAudioIntelligibility"),
  (0x0010, "Emergency"),
  (0x0011, "Karaoke"),
  ]);
}
impl TlvPut for CharacteristicEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for CharacteristicEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::ForcedSubtitles),
      0x0001 => Ok(Self::DescribesVideo),
      0x0002 => Ok(Self::EasyToRead),
      0x0003 => Ok(Self::FrameBased),
      0x0004 => Ok(Self::MainProgram),
      0x0005 => Ok(Self::OriginalContent),
      0x0006 => Ok(Self::VoiceOverTranslation),
      0x0007 => Ok(Self::Caption),
      0x0008 => Ok(Self::Subtitle),
      0x0009 => Ok(Self::Alternate),
      0x000a => Ok(Self::Supplementary),
      0x000b => Ok(Self::Commentary),
      0x000c => Ok(Self::DubbedTranslation),
      0x000d => Ok(Self::Description),
      0x000e => Ok(Self::Metadata),
      0x000f => Ok(Self::EnhancedAudioIntelligibility),
      0x0010 => Ok(Self::Emergency),
      0x0011 => Ok(Self::Karaoke),
      _ => Err(())
    }
  }
}
impl TlvDecode for CharacteristicEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum PlaybackStateEnum {
  ///Media is currently playing (includes FF and REW)
  Playing = 0x0000,
  ///Media is currently paused
  Paused = 0x0001,
  ///Media is not currently playing
  NotPlaying = 0x0002,
  ///Media is not currently buffering and playback will start when buffer has been filled
  Buffering = 0x0003,
}
impl AttrType for PlaybackStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Playing"),
  (0x0001, "Paused"),
  (0x0002, "NotPlaying"),
  (0x0003, "Buffering"),
  ]);
}
impl TlvPut for PlaybackStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for PlaybackStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Playing),
      0x0001 => Ok(Self::Paused),
      0x0002 => Ok(Self::NotPlaying),
      0x0003 => Ok(Self::Buffering),
      _ => Err(())
    }
  }
}
impl TlvDecode for PlaybackStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StatusEnum {
  ///Succeeded
  Success = 0x0000,
  ///Requested playback command is invalid in the current playback state.
  InvalidStateForCommand = 0x0001,
  ///Requested playback command is not allowed in the current playback state For example, attempting to fast-forward during a commercial might return NotAllowed.
  NotAllowed = 0x0002,
  ///This endpoint is not active for playback.
  NotActive = 0x0003,
  ///The FastForward or Rewind Command was issued but the media is already playing back at the fastest speed supported by the server in the respective direction.
  SpeedOutOfRange = 0x0004,
  ///The Seek Command was issued with a value of position outside of the allowed seek range of the media.
  SeekOutOfRange = 0x0005,
}
impl AttrType for StatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Success"),
  (0x0001, "InvalidStateForCommand"),
  (0x0002, "NotAllowed"),
  (0x0003, "NotActive"),
  (0x0004, "SpeedOutOfRange"),
  (0x0005, "SeekOutOfRange"),
  ]);
}
impl TlvPut for StatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Success),
      0x0001 => Ok(Self::InvalidStateForCommand),
      0x0002 => Ok(Self::NotAllowed),
      0x0003 => Ok(Self::NotActive),
      0x0004 => Ok(Self::SpeedOutOfRange),
      0x0005 => Ok(Self::SeekOutOfRange),
      _ => Err(())
    }
  }
}
impl TlvDecode for StatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct PlaybackPositionStruct {
  pub updated_at: TypeUnsupported /* EpochUs */,
  pub position: u64,
}
impl AttrType for PlaybackPositionStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "UpdatedAt", attr_ty: <TypeUnsupported /* EpochUs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Position", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for PlaybackPositionStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.updated_at.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.position.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for PlaybackPositionStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut position: Option<u64> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { position = Some(<u64>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      updated_at: TypeUnsupported,
      position: position.ok_or(TlvDecodeErr::MissingField("Position"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrackAttributesStruct {
  pub language_code: String,
  pub characteristics: Vec<CharacteristicEnum>,
  pub display_name: String,
}
impl AttrType for TrackAttributesStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "LanguageCode", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Characteristics", attr_ty: <Vec<CharacteristicEnum>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "DisplayName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TrackAttributesStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.language_code.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.characteristics.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.display_name.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TrackAttributesStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut language_code: Option<String> = None;
    let mut characteristics: Option<Vec<CharacteristicEnum>> = None;
    let mut display_name: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { language_code = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { characteristics = Some(<Vec<CharacteristicEnum>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { display_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      language_code: language_code.ok_or(TlvDecodeErr::MissingField("LanguageCode"))?,
      characteristics: characteristics.ok_or(TlvDecodeErr::MissingField("Characteristics"))?,
      display_name: display_name.ok_or(TlvDecodeErr::MissingField("DisplayName"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrackStruct {
  pub id: String,
  pub track_attributes: TrackAttributesStruct,
}
impl AttrType for TrackStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ID", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "TrackAttributes", attr_ty: <TrackAttributesStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TrackStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.track_attributes.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TrackStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut id: Option<String> = None;
    let mut track_attributes: Option<TrackAttributesStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { id = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { track_attributes = Some(<TrackAttributesStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      id: id.ok_or(TlvDecodeErr::MissingField("ID"))?,
      track_attributes: track_attributes.ok_or(TlvDecodeErr::MissingField("TrackAttributes"))?,
    })
  }
}

}
