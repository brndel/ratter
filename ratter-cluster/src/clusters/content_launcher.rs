#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const CONTENT_LAUNCHER: ClusterDef = ClusterDef {
  id: 0x050a,
  name: "Content Launcher",
  features: &[
    FeatureDef { bit: 0x0000, code: "CS", name: "ContentSearch", summary: "Device supports content search (non-app specific)" },
    FeatureDef { bit: 0x0001, code: "UP", name: "URLPlayback", summary: "Device supports basic URL-based file playback" },
    FeatureDef { bit: 0x0002, code: "AS", name: "AdvancedSeek", summary: "Enables clients to implement more advanced media seeking behavior in their user interface, such as for example a \"seek bar\"" },
    FeatureDef { bit: 0x0003, code: "TT", name: "TextTracks", summary: "Device or app supports Text Tracks" },
    FeatureDef { bit: 0x0004, code: "AT", name: "AudioTracks", summary: "Device or app supports Audio Tracks" },
  ],
  attributes: &[
    attr::AcceptHeader::ATTR,
    attr::SupportedStreamingProtocols::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::LaunchContent::COMMAND,
    command::LaunchURL::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct AcceptHeader;

  impl Attr for AcceptHeader {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "AcceptHeader",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Vec<String>;
  }


  pub struct SupportedStreamingProtocols;

  impl Attr for SupportedStreamingProtocols {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "SupportedStreamingProtocols",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = ty::SupportedProtocolsBitmap;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct LaunchContent {
  pub search: ty::ContentSearchStruct,
  pub auto_play: bool,
  pub data: String,
  pub playback_preferences: ty::PlaybackPreferencesStruct,
  pub use_current_context: bool,
}
impl Command for LaunchContent {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "LaunchContent",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "Search", attr_ty: <ty::ContentSearchStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "AutoPlay", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Data", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0003, name: "PlaybackPreferences", attr_ty: <ty::PlaybackPreferencesStruct>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0004, name: "UseCurrentContext", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for LaunchContent {
  type Response = LauncherResponse;
}
impl TlvEncode for LaunchContent {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.search.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.auto_play.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.data.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.playback_preferences.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.use_current_context.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LaunchURL {
  pub content_url: String,
  pub display_string: String,
  pub branding_information: ty::BrandingInformationStruct,
  pub playback_preferences: ty::PlaybackPreferencesStruct,
}
impl Command for LaunchURL {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "LaunchURL",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "ContentURL", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "DisplayString", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "BrandingInformation", attr_ty: <ty::BrandingInformationStruct>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0003, name: "PlaybackPreferences", attr_ty: <ty::PlaybackPreferencesStruct>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for LaunchURL {
  type Response = LauncherResponse;
}
impl TlvEncode for LaunchURL {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.content_url.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.display_string.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.branding_information.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.playback_preferences.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LauncherResponse {
  pub status: ty::StatusEnum,
  pub data: String,
}
impl Command for LauncherResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "LauncherResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <ty::StatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Data", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandResponse for LauncherResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  pub use crate::clusters::media_playback::ty::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum MetricTypeEnum {
  ///Dimensions defined in a number of Pixels
  Pixels = 0x0000,
  ///Dimensions defined as a percentage
  Percentage = 0x0001,
}
impl AttrType for MetricTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Pixels"),
  (0x0001, "Percentage"),
  ]);
}
impl TlvPut for MetricTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for MetricTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Pixels),
      0x0001 => Ok(Self::Percentage),
      _ => Err(())
    }
  }
}
impl TlvDecode for MetricTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ParameterEnum {
  ///Actor represents an actor credited in video media content; for example, “Gaby Hoffman”
  Actor = 0x0000,
  ///Channel represents the identifying data for a television channel; for example, "PBS"
  Channel = 0x0001,
  ///A character represented in video media content; for example, “Snow White”
  Character = 0x0002,
  ///A director of the video media content; for example, “Spike Lee”
  Director = 0x0003,
  ///An event is a reference to a type of event; examples would include sports, music, or other types of events; for example, searching for "Football games" would search for a 'game' event entity and a 'football' sport entity
  Event = 0x0004,
  ///A franchise is a video entity which can represent a number of video entities, like movies or TV shows; for example, take the fictional franchise "Intergalactic Wars" which represents a collection of movie trilogies, as well as animated and live action TV shows
  Franchise = 0x0005,
  ///Genre represents the genre of video media content such as action, drama or comedy
  Genre = 0x0006,
  ///League represents the categorical information for a sporting league; for example, "NCAA"
  League = 0x0007,
  ///Popularity indicates whether the user asks for popular content
  Popularity = 0x0008,
  ///The provider (MSP) the user wants this media to be played on; for example, "Netflix"
  Provider = 0x0009,
  ///Sport represents the categorical information of a sport; for example, football
  Sport = 0x000a,
  ///SportsTeam represents the categorical information of a professional sports team; for example, "University of Washington Huskies"
  SportsTeam = 0x000b,
  ///The type of content requested. Supported types are "Movie", "MovieSeries", "TVSeries", "TVSeason", "TVEpisode", "Trailer", "SportsEvent", "LiveEvent", and "Video"
  Type = 0x000c,
  ///Video represents the identifying data for a specific piece of video content; for example, "Manchester by the Sea"
  Video = 0x000d,
  ///Season represents the specific season number within a TV series
  Season = 0x000e,
  ///Episode represents a specific episode number within a Season in a TV series
  Episode = 0x000f,
  ///Represents a search text input across many parameter types or even outside of the defined param types
  Any = 0x0010,
}
impl AttrType for ParameterEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Actor"),
  (0x0001, "Channel"),
  (0x0002, "Character"),
  (0x0003, "Director"),
  (0x0004, "Event"),
  (0x0005, "Franchise"),
  (0x0006, "Genre"),
  (0x0007, "League"),
  (0x0008, "Popularity"),
  (0x0009, "Provider"),
  (0x000a, "Sport"),
  (0x000b, "SportsTeam"),
  (0x000c, "Type"),
  (0x000d, "Video"),
  (0x000e, "Season"),
  (0x000f, "Episode"),
  (0x0010, "Any"),
  ]);
}
impl TlvPut for ParameterEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ParameterEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Actor),
      0x0001 => Ok(Self::Channel),
      0x0002 => Ok(Self::Character),
      0x0003 => Ok(Self::Director),
      0x0004 => Ok(Self::Event),
      0x0005 => Ok(Self::Franchise),
      0x0006 => Ok(Self::Genre),
      0x0007 => Ok(Self::League),
      0x0008 => Ok(Self::Popularity),
      0x0009 => Ok(Self::Provider),
      0x000a => Ok(Self::Sport),
      0x000b => Ok(Self::SportsTeam),
      0x000c => Ok(Self::Type),
      0x000d => Ok(Self::Video),
      0x000e => Ok(Self::Season),
      0x000f => Ok(Self::Episode),
      0x0010 => Ok(Self::Any),
      _ => Err(())
    }
  }
}
impl TlvDecode for ParameterEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StatusEnum {
  ///Command succeeded
  Success = 0x0000,
  ///Requested URL could not be reached by device
  URLNotAvailable = 0x0001,
  ///Requested URL returned 401 error code.
  AuthFailed = 0x0002,
  ///Requested Text Track (in PlaybackPreferences) not available
  TextTrackNotAvailable = 0x0003,
  ///Requested Audio Track (in PlaybackPreferences) not available
  AudioTrackNotAvailable = 0x0004,
}
impl AttrType for StatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Success"),
  (0x0001, "URLNotAvailable"),
  (0x0002, "AuthFailed"),
  (0x0003, "TextTrackNotAvailable"),
  (0x0004, "AudioTrackNotAvailable"),
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
      0x0001 => Ok(Self::URLNotAvailable),
      0x0002 => Ok(Self::AuthFailed),
      0x0003 => Ok(Self::TextTrackNotAvailable),
      0x0004 => Ok(Self::AudioTrackNotAvailable),
      _ => Err(())
    }
  }
}
impl TlvDecode for StatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct SupportedProtocolsBitmap: u32 {
    ///Device supports Dynamic Adaptive Streaming over HTTP (DASH)
    const DASH = 1 << 0;
    ///Device supports HTTP Live Streaming (HLS)
    const HLS = 1 << 1;
  }
}
impl AttrType for SupportedProtocolsBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "DASH"),
    (1, "HLS"),
  ]);
}
impl TlvPut for SupportedProtocolsBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for SupportedProtocolsBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for SupportedProtocolsBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AdditionalInfoStruct {
  pub name: String,
  pub value: String,
}
impl AttrType for AdditionalInfoStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Value", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for AdditionalInfoStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.name.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.value.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for AdditionalInfoStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut name: Option<String> = None;
    let mut value: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { value = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
      value: value.ok_or(TlvDecodeErr::MissingField("Value"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BrandingInformationStruct {
  pub provider_name: String,
  pub background: StyleInformationStruct,
  pub logo: StyleInformationStruct,
  pub progress_bar: StyleInformationStruct,
  pub splash: StyleInformationStruct,
  pub water_mark: StyleInformationStruct,
}
impl AttrType for BrandingInformationStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ProviderName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Background", attr_ty: <StyleInformationStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Logo", attr_ty: <StyleInformationStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "ProgressBar", attr_ty: <StyleInformationStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "Splash", attr_ty: <StyleInformationStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "WaterMark", attr_ty: <StyleInformationStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for BrandingInformationStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.provider_name.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.background.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.logo.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.progress_bar.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.splash.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.water_mark.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for BrandingInformationStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut provider_name: Option<String> = None;
    let mut background: Option<StyleInformationStruct> = None;
    let mut logo: Option<StyleInformationStruct> = None;
    let mut progress_bar: Option<StyleInformationStruct> = None;
    let mut splash: Option<StyleInformationStruct> = None;
    let mut water_mark: Option<StyleInformationStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { provider_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { background = Some(<StyleInformationStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { logo = Some(<StyleInformationStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0003), kind: _ }) => { progress_bar = Some(<StyleInformationStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0004), kind: _ }) => { splash = Some(<StyleInformationStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0005), kind: _ }) => { water_mark = Some(<StyleInformationStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      provider_name: provider_name.ok_or(TlvDecodeErr::MissingField("ProviderName"))?,
      background: background.ok_or(TlvDecodeErr::MissingField("Background"))?,
      logo: logo.ok_or(TlvDecodeErr::MissingField("Logo"))?,
      progress_bar: progress_bar.ok_or(TlvDecodeErr::MissingField("ProgressBar"))?,
      splash: splash.ok_or(TlvDecodeErr::MissingField("Splash"))?,
      water_mark: water_mark.ok_or(TlvDecodeErr::MissingField("WaterMark"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContentSearchStruct {
  pub parameter_list: Vec<ParameterStruct>,
}
impl AttrType for ContentSearchStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ParameterList", attr_ty: <Vec<ParameterStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ContentSearchStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.parameter_list.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ContentSearchStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut parameter_list: Option<Vec<ParameterStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { parameter_list = Some(<Vec<ParameterStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      parameter_list: parameter_list.ok_or(TlvDecodeErr::MissingField("ParameterList"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DimensionStruct {
  pub width: f64,
  pub height: f64,
  pub metric: MetricTypeEnum,
}
impl AttrType for DimensionStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Width", attr_ty: <f64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Height", attr_ty: <f64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Metric", attr_ty: <MetricTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DimensionStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.width.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.height.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.metric.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DimensionStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut width: Option<f64> = None;
    let mut height: Option<f64> = None;
    let mut metric: Option<MetricTypeEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { width = Some(<f64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { height = Some(<f64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { metric = Some(<MetricTypeEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      width: width.ok_or(TlvDecodeErr::MissingField("Width"))?,
      height: height.ok_or(TlvDecodeErr::MissingField("Height"))?,
      metric: metric.ok_or(TlvDecodeErr::MissingField("Metric"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParameterStruct {
  pub r#type: ParameterEnum,
  pub value: String,
  pub external_id_list: Vec<AdditionalInfoStruct>,
}
impl AttrType for ParameterStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Type", attr_ty: <ParameterEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Value", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "ExternalIDList", attr_ty: <Vec<AdditionalInfoStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ParameterStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.r#type.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.value.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.external_id_list.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ParameterStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut r#type: Option<ParameterEnum> = None;
    let mut value: Option<String> = None;
    let mut external_id_list: Option<Vec<AdditionalInfoStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { r#type = Some(<ParameterEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { value = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { external_id_list = Some(<Vec<AdditionalInfoStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      r#type: r#type.ok_or(TlvDecodeErr::MissingField("Type"))?,
      value: value.ok_or(TlvDecodeErr::MissingField("Value"))?,
      external_id_list: external_id_list.ok_or(TlvDecodeErr::MissingField("ExternalIDList"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PlaybackPreferencesStruct {
  pub playback_position: u64,
  pub text_track: TrackPreferenceStruct,
  pub audio_tracks: Vec<TrackPreferenceStruct>,
}
impl AttrType for PlaybackPreferencesStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "PlaybackPosition", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "TextTrack", attr_ty: <TrackPreferenceStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "AudioTracks", attr_ty: <Vec<TrackPreferenceStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for PlaybackPreferencesStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.playback_position.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.text_track.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.audio_tracks.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for PlaybackPreferencesStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut playback_position: Option<u64> = None;
    let mut text_track: Option<TrackPreferenceStruct> = None;
    let mut audio_tracks: Option<Vec<TrackPreferenceStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { playback_position = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { text_track = Some(<TrackPreferenceStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { audio_tracks = Some(<Vec<TrackPreferenceStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      playback_position: playback_position.ok_or(TlvDecodeErr::MissingField("PlaybackPosition"))?,
      text_track: text_track.ok_or(TlvDecodeErr::MissingField("TextTrack"))?,
      audio_tracks: audio_tracks.ok_or(TlvDecodeErr::MissingField("AudioTracks"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StyleInformationStruct {
  pub image_url: String,
  pub color: String,
  pub size: DimensionStruct,
}
impl AttrType for StyleInformationStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ImageURL", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Color", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Size", attr_ty: <DimensionStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for StyleInformationStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.image_url.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.color.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.size.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for StyleInformationStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut image_url: Option<String> = None;
    let mut color: Option<String> = None;
    let mut size: Option<DimensionStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { image_url = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { color = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { size = Some(<DimensionStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      image_url: image_url.ok_or(TlvDecodeErr::MissingField("ImageURL"))?,
      color: color.ok_or(TlvDecodeErr::MissingField("Color"))?,
      size: size.ok_or(TlvDecodeErr::MissingField("Size"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrackPreferenceStruct {
  pub language_code: String,
  pub characteristics: Vec<CharacteristicEnum>,
  pub audio_output_index: u8,
}
impl AttrType for TrackPreferenceStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "LanguageCode", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Characteristics", attr_ty: <Vec<CharacteristicEnum>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "AudioOutputIndex", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TrackPreferenceStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.language_code.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.characteristics.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.audio_output_index.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TrackPreferenceStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut language_code: Option<String> = None;
    let mut characteristics: Option<Vec<CharacteristicEnum>> = None;
    let mut audio_output_index: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { language_code = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { characteristics = Some(<Vec<CharacteristicEnum>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { audio_output_index = Some(<u8>::tlv_decode(v, reader)?); },
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
      audio_output_index: audio_output_index.ok_or(TlvDecodeErr::MissingField("AudioOutputIndex"))?,
    })
  }
}

}
