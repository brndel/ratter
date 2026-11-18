#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const CAMERA_AV_STREAM_MANAGEMENT: ClusterDef = ClusterDef {
  id: 0x0551,
  name: "Camera AV Stream Management",
  features: &[
    FeatureDef { bit: 0x0000, code: "ADO", name: "Audio", summary: "Audio Streams supported" },
    FeatureDef { bit: 0x0001, code: "VDO", name: "Video", summary: "Video Streams supported" },
    FeatureDef { bit: 0x0002, code: "SNP", name: "Snapshot", summary: "Snapshot Streams supported" },
    FeatureDef { bit: 0x0003, code: "PRIV", name: "Privacy", summary: "Privacy supported" },
    FeatureDef { bit: 0x0004, code: "SPKR", name: "Speaker", summary: "Speaker supported" },
    FeatureDef { bit: 0x0005, code: "ICTL", name: "ImageControl", summary: "Image control supported" },
    FeatureDef { bit: 0x0006, code: "WMARK", name: "Watermark", summary: "Watermark supported" },
    FeatureDef { bit: 0x0007, code: "OSD", name: "OnScreenDisplay", summary: "OSD supported" },
    FeatureDef { bit: 0x0008, code: "STOR", name: "LocalStorage", summary: "Local Storage available" },
    FeatureDef { bit: 0x0009, code: "HDR", name: "HighDynamicRange", summary: "High Dynamic Range supported" },
    FeatureDef { bit: 0x000a, code: "NV", name: "NightVision", summary: "Night Vision mode supported" },
  ],
  attributes: &[
    attr::MaxConcurrentEncoders::ATTR,
    attr::MaxEncodedPixelRate::ATTR,
    attr::VideoSensorParams::ATTR,
    attr::NightVisionUsesInfrared::ATTR,
    attr::MinViewportResolution::ATTR,
    attr::RateDistortionTradeOffPoints::ATTR,
    attr::MaxContentBufferSize::ATTR,
    attr::MicrophoneCapabilities::ATTR,
    attr::SpeakerCapabilities::ATTR,
    attr::TwoWayTalkSupport::ATTR,
    attr::SnapshotCapabilities::ATTR,
    attr::MaxNetworkBandwidth::ATTR,
    attr::CurrentFrameRate::ATTR,
    attr::HDRModeEnabled::ATTR,
    attr::SupportedStreamUsages::ATTR,
    attr::AllocatedVideoStreams::ATTR,
    attr::AllocatedAudioStreams::ATTR,
    attr::AllocatedSnapshotStreams::ATTR,
    attr::StreamUsagePriorities::ATTR,
    attr::SoftRecordingPrivacyModeEnabled::ATTR,
    attr::SoftLivestreamPrivacyModeEnabled::ATTR,
    attr::HardPrivacyModeOn::ATTR,
    attr::NightVision::ATTR,
    attr::NightVisionIllum::ATTR,
    attr::Viewport::ATTR,
    attr::SpeakerMuted::ATTR,
    attr::SpeakerVolumeLevel::ATTR,
    attr::SpeakerMaxLevel::ATTR,
    attr::SpeakerMinLevel::ATTR,
    attr::MicrophoneMuted::ATTR,
    attr::MicrophoneVolumeLevel::ATTR,
    attr::MicrophoneMaxLevel::ATTR,
    attr::MicrophoneMinLevel::ATTR,
    attr::MicrophoneAGCEnabled::ATTR,
    attr::ImageRotation::ATTR,
    attr::ImageFlipHorizontal::ATTR,
    attr::ImageFlipVertical::ATTR,
    attr::LocalVideoRecordingEnabled::ATTR,
    attr::LocalSnapshotRecordingEnabled::ATTR,
    attr::StatusLightEnabled::ATTR,
    attr::StatusLightBrightness::ATTR,
    attr::ImageRotationDiscreteAngles::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::AudioStreamAllocate::COMMAND,
    command::AudioStreamDeallocate::COMMAND,
    command::VideoStreamAllocate::COMMAND,
    command::VideoStreamModify::COMMAND,
    command::VideoStreamDeallocate::COMMAND,
    command::SnapshotStreamAllocate::COMMAND,
    command::SnapshotStreamModify::COMMAND,
    command::SnapshotStreamDeallocate::COMMAND,
    command::SetStreamPriorities::COMMAND,
    command::CaptureSnapshot::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct MaxConcurrentEncoders;

  impl Attr for MaxConcurrentEncoders {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "MaxConcurrentEncoders",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct MaxEncodedPixelRate;

  impl Attr for MaxEncodedPixelRate {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "MaxEncodedPixelRate",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u32;
  }


  pub struct VideoSensorParams;

  impl Attr for VideoSensorParams {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "VideoSensorParams",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = ty::VideoSensorParamsStruct;
  }


  pub struct NightVisionUsesInfrared;

  impl Attr for NightVisionUsesInfrared {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "NightVisionUsesInfrared",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(10),
    };
    type Value = bool;
  }


  pub struct MinViewportResolution;

  impl Attr for MinViewportResolution {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "MinViewportResolution",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = ty::VideoResolutionStruct;
  }


  pub struct RateDistortionTradeOffPoints;

  impl Attr for RateDistortionTradeOffPoints {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "RateDistortionTradeOffPoints",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Vec<ty::RateDistortionTradeOffPointsStruct>;
  }


  pub struct MaxContentBufferSize;

  impl Attr for MaxContentBufferSize {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "MaxContentBufferSize",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u32;
  }


  pub struct MicrophoneCapabilities;

  impl Attr for MicrophoneCapabilities {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "MicrophoneCapabilities",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = ty::AudioCapabilitiesStruct;
  }


  pub struct SpeakerCapabilities;

  impl Attr for SpeakerCapabilities {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "SpeakerCapabilities",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = ty::AudioCapabilitiesStruct;
  }


  pub struct TwoWayTalkSupport;

  impl Attr for TwoWayTalkSupport {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "TwoWayTalkSupport",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = ty::TwoWayTalkSupportTypeEnum;
  }


  pub struct SnapshotCapabilities;

  impl Attr for SnapshotCapabilities {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "SnapshotCapabilities",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = Vec<ty::SnapshotCapabilitiesStruct>;
  }


  pub struct MaxNetworkBandwidth;

  impl Attr for MaxNetworkBandwidth {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "MaxNetworkBandwidth",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u32;
  }


  pub struct CurrentFrameRate;

  impl Attr for CurrentFrameRate {
    const ATTR: AttrDef = AttrDef {
      id: 0x000c,
      name: "CurrentFrameRate",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = u16;
  }


  pub struct HDRModeEnabled;

  impl Attr for HDRModeEnabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x000d,
      name: "HDRModeEnabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(9),
    };
    type Value = bool;
  }


  pub struct SupportedStreamUsages;

  impl Attr for SupportedStreamUsages {
    const ATTR: AttrDef = AttrDef {
      id: 0x000e,
      name: "SupportedStreamUsages",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::StreamUsageEnum>;
  }


  pub struct AllocatedVideoStreams;

  impl Attr for AllocatedVideoStreams {
    const ATTR: AttrDef = AttrDef {
      id: 0x000f,
      name: "AllocatedVideoStreams",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Vec<ty::VideoStreamStruct>;
  }


  pub struct AllocatedAudioStreams;

  impl Attr for AllocatedAudioStreams {
    const ATTR: AttrDef = AttrDef {
      id: 0x0010,
      name: "AllocatedAudioStreams",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<ty::AudioStreamStruct>;
  }


  pub struct AllocatedSnapshotStreams;

  impl Attr for AllocatedSnapshotStreams {
    const ATTR: AttrDef = AttrDef {
      id: 0x0011,
      name: "AllocatedSnapshotStreams",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = Vec<ty::SnapshotStreamStruct>;
  }


  pub struct StreamUsagePriorities;

  impl Attr for StreamUsagePriorities {
    const ATTR: AttrDef = AttrDef {
      id: 0x0012,
      name: "StreamUsagePriorities",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::StreamUsageEnum>;
  }


  pub struct SoftRecordingPrivacyModeEnabled;

  impl Attr for SoftRecordingPrivacyModeEnabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x0013,
      name: "SoftRecordingPrivacyModeEnabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = bool;
  }


  pub struct SoftLivestreamPrivacyModeEnabled;

  impl Attr for SoftLivestreamPrivacyModeEnabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x0014,
      name: "SoftLivestreamPrivacyModeEnabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = bool;
  }


  pub struct HardPrivacyModeOn;

  impl Attr for HardPrivacyModeOn {
    const ATTR: AttrDef = AttrDef {
      id: 0x0015,
      name: "HardPrivacyModeOn",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct NightVision;

  impl Attr for NightVision {
    const ATTR: AttrDef = AttrDef {
      id: 0x0016,
      name: "NightVision",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(10),
    };
    type Value = ty::TriStateAutoEnum;
  }


  pub struct NightVisionIllum;

  impl Attr for NightVisionIllum {
    const ATTR: AttrDef = AttrDef {
      id: 0x0017,
      name: "NightVisionIllum",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::TriStateAutoEnum;
  }


  pub struct Viewport;

  impl Attr for Viewport {
    const ATTR: AttrDef = AttrDef {
      id: 0x0018,
      name: "Viewport",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = ty::ViewportStruct;
  }


  pub struct SpeakerMuted;

  impl Attr for SpeakerMuted {
    const ATTR: AttrDef = AttrDef {
      id: 0x0019,
      name: "SpeakerMuted",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = bool;
  }


  pub struct SpeakerVolumeLevel;

  impl Attr for SpeakerVolumeLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x001a,
      name: "SpeakerVolumeLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = u8;
  }


  pub struct SpeakerMaxLevel;

  impl Attr for SpeakerMaxLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x001b,
      name: "SpeakerMaxLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = u8;
  }


  pub struct SpeakerMinLevel;

  impl Attr for SpeakerMinLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x001c,
      name: "SpeakerMinLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = u8;
  }


  pub struct MicrophoneMuted;

  impl Attr for MicrophoneMuted {
    const ATTR: AttrDef = AttrDef {
      id: 0x001d,
      name: "MicrophoneMuted",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = bool;
  }


  pub struct MicrophoneVolumeLevel;

  impl Attr for MicrophoneVolumeLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x001e,
      name: "MicrophoneVolumeLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u8;
  }


  pub struct MicrophoneMaxLevel;

  impl Attr for MicrophoneMaxLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x001f,
      name: "MicrophoneMaxLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u8;
  }


  pub struct MicrophoneMinLevel;

  impl Attr for MicrophoneMinLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0020,
      name: "MicrophoneMinLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u8;
  }


  pub struct MicrophoneAGCEnabled;

  impl Attr for MicrophoneAGCEnabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x0021,
      name: "MicrophoneAGCEnabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct ImageRotation;

  impl Attr for ImageRotation {
    const ATTR: AttrDef = AttrDef {
      id: 0x0022,
      name: "ImageRotation",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct ImageFlipHorizontal;

  impl Attr for ImageFlipHorizontal {
    const ATTR: AttrDef = AttrDef {
      id: 0x0023,
      name: "ImageFlipHorizontal",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct ImageFlipVertical;

  impl Attr for ImageFlipVertical {
    const ATTR: AttrDef = AttrDef {
      id: 0x0024,
      name: "ImageFlipVertical",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct LocalVideoRecordingEnabled;

  impl Attr for LocalVideoRecordingEnabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x0025,
      name: "LocalVideoRecordingEnabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


  pub struct LocalSnapshotRecordingEnabled;

  impl Attr for LocalSnapshotRecordingEnabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x0026,
      name: "LocalSnapshotRecordingEnabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


  pub struct StatusLightEnabled;

  impl Attr for StatusLightEnabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x0027,
      name: "StatusLightEnabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct StatusLightBrightness;

  impl Attr for StatusLightBrightness {
    const ATTR: AttrDef = AttrDef {
      id: 0x0028,
      name: "StatusLightBrightness",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::ThreeLevelAutoEnum;
  }


  pub struct ImageRotationDiscreteAngles;

  impl Attr for ImageRotationDiscreteAngles {
    const ATTR: AttrDef = AttrDef {
      id: 0x0029,
      name: "ImageRotationDiscreteAngles",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct AudioStreamAllocate {
  pub stream_usage: ty::StreamUsageEnum,
  pub audio_codec: ty::AudioCodecEnum,
  pub channel_count: u8,
  pub sample_rate: u32,
  pub bit_rate: u32,
  pub bit_depth: u8,
}
impl Command for AudioStreamAllocate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "AudioStreamAllocate",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "StreamUsage", attr_ty: <ty::StreamUsageEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "AudioCodec", attr_ty: <ty::AudioCodecEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "ChannelCount", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "SampleRate", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "BitRate", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0005, name: "BitDepth", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AudioStreamAllocate {
  type Response = AudioStreamAllocateResponse;
}
impl TlvEncode for AudioStreamAllocate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.stream_usage.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.audio_codec.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.channel_count.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.sample_rate.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.bit_rate.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.bit_depth.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AudioStreamAllocateResponse {
  pub audio_stream_id: u16,
}
impl Command for AudioStreamAllocateResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "AudioStreamAllocateResponse",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "AudioStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for AudioStreamAllocateResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct AudioStreamDeallocate {
  pub audio_stream_id: u16,
}
impl Command for AudioStreamDeallocate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "AudioStreamDeallocate",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "AudioStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AudioStreamDeallocate {
  type Response = BasicAccResponse;
}
impl TlvEncode for AudioStreamDeallocate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.audio_stream_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoStreamAllocate {
  pub stream_usage: ty::StreamUsageEnum,
  pub video_codec: ty::VideoCodecEnum,
  pub min_frame_rate: u16,
  pub max_frame_rate: u16,
  pub min_resolution: ty::VideoResolutionStruct,
  pub max_resolution: ty::VideoResolutionStruct,
  pub min_bit_rate: u32,
  pub max_bit_rate: u32,
  pub key_frame_interval: u16,
  pub watermark_enabled: bool,
  pub osd_enabled: bool,
}
impl Command for VideoStreamAllocate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "VideoStreamAllocate",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "StreamUsage", attr_ty: <ty::StreamUsageEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "VideoCodec", attr_ty: <ty::VideoCodecEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "MinFrameRate", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "MaxFrameRate", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "MinResolution", attr_ty: <ty::VideoResolutionStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0005, name: "MaxResolution", attr_ty: <ty::VideoResolutionStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0006, name: "MinBitRate", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0007, name: "MaxBitRate", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0008, name: "KeyFrameInterval", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0009, name: "WatermarkEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Feature(6) },
      AttrDef { id: 0x000a, name: "OSDEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Feature(7) },
    ],
  };
}
impl CommandInvoke for VideoStreamAllocate {
  type Response = VideoStreamAllocateResponse;
}
impl TlvEncode for VideoStreamAllocate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.stream_usage.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.video_codec.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.min_frame_rate.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.max_frame_rate.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.min_resolution.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.max_resolution.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.min_bit_rate.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.max_bit_rate.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.key_frame_interval.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    self.watermark_enabled.tlv_put(matter_codec::Tag::Context(0x0009), writer);
    self.osd_enabled.tlv_put(matter_codec::Tag::Context(0x000a), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoStreamAllocateResponse {
  pub video_stream_id: u16,
}
impl Command for VideoStreamAllocateResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "VideoStreamAllocateResponse",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "VideoStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for VideoStreamAllocateResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoStreamModify {
  pub video_stream_id: u16,
  pub watermark_enabled: bool,
  pub osd_enabled: bool,
}
impl Command for VideoStreamModify {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "VideoStreamModify",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "VideoStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "WatermarkEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "OSDEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for VideoStreamModify {
  type Response = BasicAccResponse;
}
impl TlvEncode for VideoStreamModify {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.video_stream_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.watermark_enabled.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.osd_enabled.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoStreamDeallocate {
  pub video_stream_id: u16,
}
impl Command for VideoStreamDeallocate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "VideoStreamDeallocate",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "VideoStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for VideoStreamDeallocate {
  type Response = BasicAccResponse;
}
impl TlvEncode for VideoStreamDeallocate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.video_stream_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotStreamAllocate {
  pub image_codec: ty::ImageCodecEnum,
  pub max_frame_rate: u16,
  pub min_resolution: ty::VideoResolutionStruct,
  pub max_resolution: ty::VideoResolutionStruct,
  pub quality: u8,
  pub watermark_enabled: bool,
  pub osd_enabled: bool,
}
impl Command for SnapshotStreamAllocate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "SnapshotStreamAllocate",
    conformance: Conformance::Feature(2),
    fields: &[
      AttrDef { id: 0x0000, name: "ImageCodec", attr_ty: <ty::ImageCodecEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "MaxFrameRate", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "MinResolution", attr_ty: <ty::VideoResolutionStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "MaxResolution", attr_ty: <ty::VideoResolutionStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "Quality", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0005, name: "WatermarkEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Feature(6) },
      AttrDef { id: 0x0006, name: "OSDEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Feature(7) },
    ],
  };
}
impl CommandInvoke for SnapshotStreamAllocate {
  type Response = SnapshotStreamAllocateResponse;
}
impl TlvEncode for SnapshotStreamAllocate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.image_codec.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.max_frame_rate.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.min_resolution.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.max_resolution.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.quality.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.watermark_enabled.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.osd_enabled.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotStreamAllocateResponse {
  pub snapshot_stream_id: u16,
}
impl Command for SnapshotStreamAllocateResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0008,
    name: "SnapshotStreamAllocateResponse",
    conformance: Conformance::Feature(2),
    fields: &[
      AttrDef { id: 0x0000, name: "SnapshotStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for SnapshotStreamAllocateResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotStreamModify {
  pub snapshot_stream_id: u16,
  pub watermark_enabled: bool,
  pub osd_enabled: bool,
}
impl Command for SnapshotStreamModify {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0009,
    name: "SnapshotStreamModify",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "SnapshotStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "WatermarkEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "OSDEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for SnapshotStreamModify {
  type Response = BasicAccResponse;
}
impl TlvEncode for SnapshotStreamModify {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.snapshot_stream_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.watermark_enabled.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.osd_enabled.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotStreamDeallocate {
  pub snapshot_stream_id: u16,
}
impl Command for SnapshotStreamDeallocate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000a,
    name: "SnapshotStreamDeallocate",
    conformance: Conformance::Feature(2),
    fields: &[
      AttrDef { id: 0x0000, name: "SnapshotStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SnapshotStreamDeallocate {
  type Response = BasicAccResponse;
}
impl TlvEncode for SnapshotStreamDeallocate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.snapshot_stream_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetStreamPriorities {
  pub stream_priorities: Vec<ty::StreamUsageEnum>,
}
impl Command for SetStreamPriorities {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000b,
    name: "SetStreamPriorities",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "StreamPriorities", attr_ty: <Vec<ty::StreamUsageEnum>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SetStreamPriorities {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetStreamPriorities {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.stream_priorities.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CaptureSnapshot {
  pub snapshot_stream_id: u16,
  pub requested_resolution: ty::VideoResolutionStruct,
}
impl Command for CaptureSnapshot {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000c,
    name: "CaptureSnapshot",
    conformance: Conformance::Feature(2),
    fields: &[
      AttrDef { id: 0x0000, name: "SnapshotStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "RequestedResolution", attr_ty: <ty::VideoResolutionStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for CaptureSnapshot {
  type Response = CaptureSnapshotResponse;
}
impl TlvEncode for CaptureSnapshot {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.snapshot_stream_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.requested_resolution.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CaptureSnapshotResponse {
  pub data: TypeUnsupported /* Octstr */,
  pub image_codec: ty::ImageCodecEnum,
  pub resolution: ty::VideoResolutionStruct,
}
impl Command for CaptureSnapshotResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000d,
    name: "CaptureSnapshotResponse",
    conformance: Conformance::Feature(2),
    fields: &[
      AttrDef { id: 0x0000, name: "Data", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ImageCodec", attr_ty: <ty::ImageCodecEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Resolution", attr_ty: <ty::VideoResolutionStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for CaptureSnapshotResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum AudioCodecEnum {
  ///Open source IETF standard codec.
  OPUS = 0x0000,
  ///Advanced Audio Coding codec-Low Complexity
  AAC_LC = 0x0001,
}
impl AttrType for AudioCodecEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "OPUS"),
  (0x0001, "AAC_LC"),
  ]);
}
impl TlvPut for AudioCodecEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for AudioCodecEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::OPUS),
      0x0001 => Ok(Self::AAC_LC),
      _ => Err(())
    }
  }
}
impl TlvDecode for AudioCodecEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ImageCodecEnum {
  ///JPEG image codec.
  JPEG = 0x0000,
  ///HEIC image codec.
  HEIC = 0x0001,
}
impl AttrType for ImageCodecEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "JPEG"),
  (0x0001, "HEIC"),
  ]);
}
impl TlvPut for ImageCodecEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ImageCodecEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::JPEG),
      0x0001 => Ok(Self::HEIC),
      _ => Err(())
    }
  }
}
impl TlvDecode for ImageCodecEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TriStateAutoEnum {
  ///Off
  Off = 0x0000,
  ///On
  On = 0x0001,
  ///Automatic Operation
  Auto = 0x0002,
}
impl AttrType for TriStateAutoEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Off"),
  (0x0001, "On"),
  (0x0002, "Auto"),
  ]);
}
impl TlvPut for TriStateAutoEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TriStateAutoEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Off),
      0x0001 => Ok(Self::On),
      0x0002 => Ok(Self::Auto),
      _ => Err(())
    }
  }
}
impl TlvDecode for TriStateAutoEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TwoWayTalkSupportTypeEnum {
  ///Two-way Talk support is absent.
  NotSupported = 0x0000,
  ///Audio in one direction at a time.
  HalfDuplex = 0x0001,
  ///Audio in both directions simultaneously.
  FullDuplex = 0x0002,
}
impl AttrType for TwoWayTalkSupportTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "NotSupported"),
  (0x0001, "HalfDuplex"),
  (0x0002, "FullDuplex"),
  ]);
}
impl TlvPut for TwoWayTalkSupportTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TwoWayTalkSupportTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::NotSupported),
      0x0001 => Ok(Self::HalfDuplex),
      0x0002 => Ok(Self::FullDuplex),
      _ => Err(())
    }
  }
}
impl TlvDecode for TwoWayTalkSupportTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum VideoCodecEnum {
  ///Advanced Video Coding (H.264) codec.
  H264 = 0x0000,
  ///High efficiency Video Coding (H.265) codec.
  HEVC = 0x0001,
  ///Versatile Video Coding (H.266) codec.
  VVC = 0x0002,
  ///AOMedia Video 1 codec.
  AV1 = 0x0003,
}
impl AttrType for VideoCodecEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "H264"),
  (0x0001, "HEVC"),
  (0x0002, "VVC"),
  (0x0003, "AV1"),
  ]);
}
impl TlvPut for VideoCodecEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for VideoCodecEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::H264),
      0x0001 => Ok(Self::HEVC),
      0x0002 => Ok(Self::VVC),
      0x0003 => Ok(Self::AV1),
      _ => Err(())
    }
  }
}
impl TlvDecode for VideoCodecEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AVMetadataStruct {
  pub utc_time: TypeUnsupported /* EpochUs */,
  pub motion_zones_active: Vec<u16>,
  pub black_and_white_active: bool,
  pub user_defined: TypeUnsupported /* Octstr */,
}
impl AttrType for AVMetadataStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0001, name: "UTCTime", attr_ty: <TypeUnsupported /* EpochUs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "MotionZonesActive", attr_ty: <Vec<u16>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "BlackAndWhiteActive", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "UserDefined", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for AVMetadataStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.utc_time.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.motion_zones_active.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.black_and_white_active.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.user_defined.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for AVMetadataStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut motion_zones_active: Option<Vec<u16>> = None;
    let mut black_and_white_active: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { motion_zones_active = Some(<Vec<u16>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { black_and_white_active = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      utc_time: TypeUnsupported,
      motion_zones_active: motion_zones_active.ok_or(TlvDecodeErr::MissingField("MotionZonesActive"))?,
      black_and_white_active: black_and_white_active.ok_or(TlvDecodeErr::MissingField("BlackAndWhiteActive"))?,
      user_defined: TypeUnsupported,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AudioCapabilitiesStruct {
  pub max_number_of_channels: u8,
  pub supported_codecs: Vec<AudioCodecEnum>,
  pub supported_sample_rates: Vec<u32>,
  pub supported_bit_depths: Vec<u8>,
}
impl AttrType for AudioCapabilitiesStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "MaxNumberOfChannels", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "SupportedCodecs", attr_ty: <Vec<AudioCodecEnum>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "SupportedSampleRates", attr_ty: <Vec<u32>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "SupportedBitDepths", attr_ty: <Vec<u8>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for AudioCapabilitiesStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.max_number_of_channels.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.supported_codecs.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.supported_sample_rates.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.supported_bit_depths.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for AudioCapabilitiesStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut max_number_of_channels: Option<u8> = None;
    let mut supported_codecs: Option<Vec<AudioCodecEnum>> = None;
    let mut supported_sample_rates: Option<Vec<u32>> = None;
    let mut supported_bit_depths: Option<Vec<u8>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { max_number_of_channels = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { supported_codecs = Some(<Vec<AudioCodecEnum>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { supported_sample_rates = Some(<Vec<u32>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0003), kind: _ }) => { supported_bit_depths = Some(<Vec<u8>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      max_number_of_channels: max_number_of_channels.ok_or(TlvDecodeErr::MissingField("MaxNumberOfChannels"))?,
      supported_codecs: supported_codecs.ok_or(TlvDecodeErr::MissingField("SupportedCodecs"))?,
      supported_sample_rates: supported_sample_rates.ok_or(TlvDecodeErr::MissingField("SupportedSampleRates"))?,
      supported_bit_depths: supported_bit_depths.ok_or(TlvDecodeErr::MissingField("SupportedBitDepths"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AudioStreamStruct {
  pub audio_stream_id: u16,
  pub stream_usage: StreamUsageEnum,
  pub audio_codec: AudioCodecEnum,
  pub channel_count: u8,
  pub sample_rate: u32,
  pub bit_rate: u32,
  pub bit_depth: u8,
  pub reference_count: u8,
}
impl AttrType for AudioStreamStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "AudioStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "StreamUsage", attr_ty: <StreamUsageEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "AudioCodec", attr_ty: <AudioCodecEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "ChannelCount", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "SampleRate", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "BitRate", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "BitDepth", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "ReferenceCount", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for AudioStreamStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.audio_stream_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.stream_usage.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.audio_codec.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.channel_count.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.sample_rate.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.bit_rate.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.bit_depth.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.reference_count.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for AudioStreamStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut audio_stream_id: Option<u16> = None;
    let mut stream_usage: Option<StreamUsageEnum> = None;
    let mut audio_codec: Option<AudioCodecEnum> = None;
    let mut channel_count: Option<u8> = None;
    let mut sample_rate: Option<u32> = None;
    let mut bit_rate: Option<u32> = None;
    let mut bit_depth: Option<u8> = None;
    let mut reference_count: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { audio_stream_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { stream_usage = Some(<StreamUsageEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { audio_codec = Some(<AudioCodecEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { channel_count = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { sample_rate = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { bit_rate = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { bit_depth = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0007), value: _ }) => { reference_count = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      audio_stream_id: audio_stream_id.ok_or(TlvDecodeErr::MissingField("AudioStreamID"))?,
      stream_usage: stream_usage.ok_or(TlvDecodeErr::MissingField("StreamUsage"))?,
      audio_codec: audio_codec.ok_or(TlvDecodeErr::MissingField("AudioCodec"))?,
      channel_count: channel_count.ok_or(TlvDecodeErr::MissingField("ChannelCount"))?,
      sample_rate: sample_rate.ok_or(TlvDecodeErr::MissingField("SampleRate"))?,
      bit_rate: bit_rate.ok_or(TlvDecodeErr::MissingField("BitRate"))?,
      bit_depth: bit_depth.ok_or(TlvDecodeErr::MissingField("BitDepth"))?,
      reference_count: reference_count.ok_or(TlvDecodeErr::MissingField("ReferenceCount"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RateDistortionTradeOffPointsStruct {
  pub codec: VideoCodecEnum,
  pub resolution: VideoResolutionStruct,
  pub min_bit_rate: u32,
}
impl AttrType for RateDistortionTradeOffPointsStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Codec", attr_ty: <VideoCodecEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Resolution", attr_ty: <VideoResolutionStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "MinBitRate", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for RateDistortionTradeOffPointsStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.codec.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.resolution.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.min_bit_rate.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for RateDistortionTradeOffPointsStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut codec: Option<VideoCodecEnum> = None;
    let mut resolution: Option<VideoResolutionStruct> = None;
    let mut min_bit_rate: Option<u32> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { codec = Some(<VideoCodecEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { resolution = Some(<VideoResolutionStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { min_bit_rate = Some(<u32>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      codec: codec.ok_or(TlvDecodeErr::MissingField("Codec"))?,
      resolution: resolution.ok_or(TlvDecodeErr::MissingField("Resolution"))?,
      min_bit_rate: min_bit_rate.ok_or(TlvDecodeErr::MissingField("MinBitRate"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotCapabilitiesStruct {
  pub resolution: VideoResolutionStruct,
  pub max_frame_rate: u16,
  pub image_codec: ImageCodecEnum,
  pub requires_encoded_pixels: bool,
  pub requires_hardware_encoder: bool,
}
impl AttrType for SnapshotCapabilitiesStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Resolution", attr_ty: <VideoResolutionStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "MaxFrameRate", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "ImageCodec", attr_ty: <ImageCodecEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "RequiresEncodedPixels", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "RequiresHardwareEncoder", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for SnapshotCapabilitiesStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.resolution.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.max_frame_rate.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.image_codec.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.requires_encoded_pixels.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.requires_hardware_encoder.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for SnapshotCapabilitiesStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut resolution: Option<VideoResolutionStruct> = None;
    let mut max_frame_rate: Option<u16> = None;
    let mut image_codec: Option<ImageCodecEnum> = None;
    let mut requires_encoded_pixels: Option<bool> = None;
    let mut requires_hardware_encoder: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { resolution = Some(<VideoResolutionStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { max_frame_rate = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { image_codec = Some(<ImageCodecEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { requires_encoded_pixels = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { requires_hardware_encoder = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      resolution: resolution.ok_or(TlvDecodeErr::MissingField("Resolution"))?,
      max_frame_rate: max_frame_rate.ok_or(TlvDecodeErr::MissingField("MaxFrameRate"))?,
      image_codec: image_codec.ok_or(TlvDecodeErr::MissingField("ImageCodec"))?,
      requires_encoded_pixels: requires_encoded_pixels.ok_or(TlvDecodeErr::MissingField("RequiresEncodedPixels"))?,
      requires_hardware_encoder: requires_hardware_encoder.ok_or(TlvDecodeErr::MissingField("RequiresHardwareEncoder"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SnapshotStreamStruct {
  pub snapshot_stream_id: u16,
  pub image_codec: ImageCodecEnum,
  pub frame_rate: u16,
  pub min_resolution: VideoResolutionStruct,
  pub max_resolution: VideoResolutionStruct,
  pub quality: u8,
  pub reference_count: u8,
  pub encoded_pixels: bool,
  pub hardware_encoder: bool,
  pub watermark_enabled: bool,
  pub osd_enabled: bool,
}
impl AttrType for SnapshotStreamStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "SnapshotStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ImageCodec", attr_ty: <ImageCodecEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "FrameRate", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "MinResolution", attr_ty: <VideoResolutionStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "MaxResolution", attr_ty: <VideoResolutionStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "Quality", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "ReferenceCount", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "EncodedPixels", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0008, name: "HardwareEncoder", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0009, name: "WatermarkEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000a, name: "OSDEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for SnapshotStreamStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.snapshot_stream_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.image_codec.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.frame_rate.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.min_resolution.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.max_resolution.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.quality.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.reference_count.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.encoded_pixels.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.hardware_encoder.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    self.watermark_enabled.tlv_put(matter_codec::Tag::Context(0x0009), writer);
    self.osd_enabled.tlv_put(matter_codec::Tag::Context(0x000a), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for SnapshotStreamStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut snapshot_stream_id: Option<u16> = None;
    let mut image_codec: Option<ImageCodecEnum> = None;
    let mut frame_rate: Option<u16> = None;
    let mut min_resolution: Option<VideoResolutionStruct> = None;
    let mut max_resolution: Option<VideoResolutionStruct> = None;
    let mut quality: Option<u8> = None;
    let mut reference_count: Option<u8> = None;
    let mut encoded_pixels: Option<bool> = None;
    let mut hardware_encoder: Option<bool> = None;
    let mut watermark_enabled: Option<bool> = None;
    let mut osd_enabled: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { snapshot_stream_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { image_codec = Some(<ImageCodecEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { frame_rate = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0003), kind: _ }) => { min_resolution = Some(<VideoResolutionStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0004), kind: _ }) => { max_resolution = Some(<VideoResolutionStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { quality = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { reference_count = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0007), value: _ }) => { encoded_pixels = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0008), value: _ }) => { hardware_encoder = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0009), value: _ }) => { watermark_enabled = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000a), value: _ }) => { osd_enabled = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      snapshot_stream_id: snapshot_stream_id.ok_or(TlvDecodeErr::MissingField("SnapshotStreamID"))?,
      image_codec: image_codec.ok_or(TlvDecodeErr::MissingField("ImageCodec"))?,
      frame_rate: frame_rate.ok_or(TlvDecodeErr::MissingField("FrameRate"))?,
      min_resolution: min_resolution.ok_or(TlvDecodeErr::MissingField("MinResolution"))?,
      max_resolution: max_resolution.ok_or(TlvDecodeErr::MissingField("MaxResolution"))?,
      quality: quality.ok_or(TlvDecodeErr::MissingField("Quality"))?,
      reference_count: reference_count.ok_or(TlvDecodeErr::MissingField("ReferenceCount"))?,
      encoded_pixels: encoded_pixels.ok_or(TlvDecodeErr::MissingField("EncodedPixels"))?,
      hardware_encoder: hardware_encoder.ok_or(TlvDecodeErr::MissingField("HardwareEncoder"))?,
      watermark_enabled: watermark_enabled.ok_or(TlvDecodeErr::MissingField("WatermarkEnabled"))?,
      osd_enabled: osd_enabled.ok_or(TlvDecodeErr::MissingField("OSDEnabled"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoResolutionStruct {
  pub width: u16,
  pub height: u16,
}
impl AttrType for VideoResolutionStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Width", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Height", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for VideoResolutionStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.width.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.height.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for VideoResolutionStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut width: Option<u16> = None;
    let mut height: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { width = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { height = Some(<u16>::tlv_decode(v, reader)?); },
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
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoSensorParamsStruct {
  pub sensor_width: u16,
  pub sensor_height: u16,
  pub max_fps: u16,
  pub max_hdrfps: u16,
}
impl AttrType for VideoSensorParamsStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "SensorWidth", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "SensorHeight", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "MaxFPS", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "MaxHDRFPS", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for VideoSensorParamsStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.sensor_width.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.sensor_height.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.max_fps.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.max_hdrfps.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for VideoSensorParamsStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut sensor_width: Option<u16> = None;
    let mut sensor_height: Option<u16> = None;
    let mut max_fps: Option<u16> = None;
    let mut max_hdrfps: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { sensor_width = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { sensor_height = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { max_fps = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { max_hdrfps = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      sensor_width: sensor_width.ok_or(TlvDecodeErr::MissingField("SensorWidth"))?,
      sensor_height: sensor_height.ok_or(TlvDecodeErr::MissingField("SensorHeight"))?,
      max_fps: max_fps.ok_or(TlvDecodeErr::MissingField("MaxFPS"))?,
      max_hdrfps: max_hdrfps.ok_or(TlvDecodeErr::MissingField("MaxHDRFPS"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoStreamStruct {
  pub video_stream_id: u16,
  pub stream_usage: StreamUsageEnum,
  pub video_codec: VideoCodecEnum,
  pub min_frame_rate: u16,
  pub max_frame_rate: u16,
  pub min_resolution: VideoResolutionStruct,
  pub max_resolution: VideoResolutionStruct,
  pub min_bit_rate: u32,
  pub max_bit_rate: u32,
  pub key_frame_interval: u16,
  pub watermark_enabled: bool,
  pub osd_enabled: bool,
  pub reference_count: u8,
}
impl AttrType for VideoStreamStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "VideoStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "StreamUsage", attr_ty: <StreamUsageEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "VideoCodec", attr_ty: <VideoCodecEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "MinFrameRate", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "MaxFrameRate", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "MinResolution", attr_ty: <VideoResolutionStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "MaxResolution", attr_ty: <VideoResolutionStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "MinBitRate", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0008, name: "MaxBitRate", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0009, name: "KeyFrameInterval", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000a, name: "WatermarkEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000b, name: "OSDEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000c, name: "ReferenceCount", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for VideoStreamStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.video_stream_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.stream_usage.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.video_codec.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.min_frame_rate.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.max_frame_rate.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.min_resolution.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.max_resolution.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.min_bit_rate.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.max_bit_rate.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    self.key_frame_interval.tlv_put(matter_codec::Tag::Context(0x0009), writer);
    self.watermark_enabled.tlv_put(matter_codec::Tag::Context(0x000a), writer);
    self.osd_enabled.tlv_put(matter_codec::Tag::Context(0x000b), writer);
    self.reference_count.tlv_put(matter_codec::Tag::Context(0x000c), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for VideoStreamStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut video_stream_id: Option<u16> = None;
    let mut stream_usage: Option<StreamUsageEnum> = None;
    let mut video_codec: Option<VideoCodecEnum> = None;
    let mut min_frame_rate: Option<u16> = None;
    let mut max_frame_rate: Option<u16> = None;
    let mut min_resolution: Option<VideoResolutionStruct> = None;
    let mut max_resolution: Option<VideoResolutionStruct> = None;
    let mut min_bit_rate: Option<u32> = None;
    let mut max_bit_rate: Option<u32> = None;
    let mut key_frame_interval: Option<u16> = None;
    let mut watermark_enabled: Option<bool> = None;
    let mut osd_enabled: Option<bool> = None;
    let mut reference_count: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { video_stream_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { stream_usage = Some(<StreamUsageEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { video_codec = Some(<VideoCodecEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { min_frame_rate = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { max_frame_rate = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0005), kind: _ }) => { min_resolution = Some(<VideoResolutionStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0006), kind: _ }) => { max_resolution = Some(<VideoResolutionStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0007), value: _ }) => { min_bit_rate = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0008), value: _ }) => { max_bit_rate = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0009), value: _ }) => { key_frame_interval = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000a), value: _ }) => { watermark_enabled = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000b), value: _ }) => { osd_enabled = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000c), value: _ }) => { reference_count = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      video_stream_id: video_stream_id.ok_or(TlvDecodeErr::MissingField("VideoStreamID"))?,
      stream_usage: stream_usage.ok_or(TlvDecodeErr::MissingField("StreamUsage"))?,
      video_codec: video_codec.ok_or(TlvDecodeErr::MissingField("VideoCodec"))?,
      min_frame_rate: min_frame_rate.ok_or(TlvDecodeErr::MissingField("MinFrameRate"))?,
      max_frame_rate: max_frame_rate.ok_or(TlvDecodeErr::MissingField("MaxFrameRate"))?,
      min_resolution: min_resolution.ok_or(TlvDecodeErr::MissingField("MinResolution"))?,
      max_resolution: max_resolution.ok_or(TlvDecodeErr::MissingField("MaxResolution"))?,
      min_bit_rate: min_bit_rate.ok_or(TlvDecodeErr::MissingField("MinBitRate"))?,
      max_bit_rate: max_bit_rate.ok_or(TlvDecodeErr::MissingField("MaxBitRate"))?,
      key_frame_interval: key_frame_interval.ok_or(TlvDecodeErr::MissingField("KeyFrameInterval"))?,
      watermark_enabled: watermark_enabled.ok_or(TlvDecodeErr::MissingField("WatermarkEnabled"))?,
      osd_enabled: osd_enabled.ok_or(TlvDecodeErr::MissingField("OSDEnabled"))?,
      reference_count: reference_count.ok_or(TlvDecodeErr::MissingField("ReferenceCount"))?,
    })
  }
}

}
