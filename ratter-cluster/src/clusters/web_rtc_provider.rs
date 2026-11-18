#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const WEB_RTC_TRANSPORT_PROVIDER: ClusterDef = ClusterDef {
  id: 0x0553,
  name: "WebRTC Transport Provider",
  features: &[
    FeatureDef { bit: 0x0000, code: "METADATA", name: "Metadata", summary: "Supports metadata transmission in WebRTC sessions" },
  ],
  attributes: &[
    attr::CurrentSessions::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::SolicitOffer::COMMAND,
    command::ProvideOffer::COMMAND,
    command::ProvideAnswer::COMMAND,
    command::ProvideICECandidates::COMMAND,
    command::EndSession::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct CurrentSessions;

  impl Attr for CurrentSessions {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "CurrentSessions",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::WebRTCSessionStruct>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SolicitOffer {
  pub stream_usage: ty::StreamUsageEnum,
  pub originating_endpoint_id: TypeUnsupported /* EndpointNo */,
  pub video_stream_id: u16,
  pub audio_stream_id: u16,
  pub ice_servers: Vec<ty::ICEServerStruct>,
  pub ice_transport_policy: String,
  pub metadata_enabled: bool,
  pub video_streams: Vec<u16>,
  pub audio_streams: Vec<u16>,
}
impl Command for SolicitOffer {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "SolicitOffer",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "StreamUsage", attr_ty: <ty::StreamUsageEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "OriginatingEndpointID", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "VideoStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x0003, name: "AudioStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x0004, name: "ICEServers", attr_ty: <Vec<ty::ICEServerStruct>>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0005, name: "ICETransportPolicy", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0006, name: "MetadataEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Feature(0) },
      AttrDef { id: 0x0008, name: "VideoStreams", attr_ty: <Vec<u16>>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x0009, name: "AudioStreams", attr_ty: <Vec<u16>>::ATTR_TYPE, conformance: Conformance::TooComplicated },
    ],
  };
}
impl CommandInvoke for SolicitOffer {
  type Response = SolicitOfferResponse;
}
impl TlvEncode for SolicitOffer {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.stream_usage.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.originating_endpoint_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.video_stream_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.audio_stream_id.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.ice_servers.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.ice_transport_policy.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.metadata_enabled.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.video_streams.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    self.audio_streams.tlv_put(matter_codec::Tag::Context(0x0009), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SolicitOfferResponse {
  pub web_rtc_session_id: u16,
  pub deferred_offer: bool,
  pub video_stream_id: u16,
  pub audio_stream_id: u16,
}
impl Command for SolicitOfferResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "SolicitOfferResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "WebRTCSessionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "DeferredOffer", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "VideoStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x0003, name: "AudioStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::TooComplicated },
    ],
  };
}
impl CommandResponse for SolicitOfferResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct ProvideOffer {
  pub web_rtc_session_id: u16,
  pub sdp: String,
  pub stream_usage: ty::StreamUsageEnum,
  pub originating_endpoint_id: TypeUnsupported /* EndpointNo */,
  pub video_stream_id: u16,
  pub audio_stream_id: u16,
  pub ice_servers: Vec<ty::ICEServerStruct>,
  pub ice_transport_policy: String,
  pub metadata_enabled: bool,
  pub video_streams: Vec<u16>,
  pub audio_streams: Vec<u16>,
}
impl Command for ProvideOffer {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "ProvideOffer",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "WebRTCSessionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "SDP", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "StreamUsage", attr_ty: <ty::StreamUsageEnum>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x0003, name: "OriginatingEndpointID", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x0004, name: "VideoStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x0005, name: "AudioStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x0006, name: "ICEServers", attr_ty: <Vec<ty::ICEServerStruct>>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0007, name: "ICETransportPolicy", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0008, name: "MetadataEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x000a, name: "VideoStreams", attr_ty: <Vec<u16>>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x000b, name: "AudioStreams", attr_ty: <Vec<u16>>::ATTR_TYPE, conformance: Conformance::TooComplicated },
    ],
  };
}
impl CommandInvoke for ProvideOffer {
  type Response = ProvideOfferResponse;
}
impl TlvEncode for ProvideOffer {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.web_rtc_session_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.sdp.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.stream_usage.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.originating_endpoint_id.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.video_stream_id.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.audio_stream_id.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.ice_servers.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.ice_transport_policy.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.metadata_enabled.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    self.video_streams.tlv_put(matter_codec::Tag::Context(0x000a), writer);
    self.audio_streams.tlv_put(matter_codec::Tag::Context(0x000b), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProvideOfferResponse {
  pub web_rtc_session_id: u16,
  pub video_stream_id: u16,
  pub audio_stream_id: u16,
}
impl Command for ProvideOfferResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "ProvideOfferResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "WebRTCSessionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "VideoStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x0002, name: "AudioStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::TooComplicated },
    ],
  };
}
impl CommandResponse for ProvideOfferResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct ProvideAnswer {
  pub web_rtc_session_id: u16,
  pub sdp: String,
}
impl Command for ProvideAnswer {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "ProvideAnswer",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "WebRTCSessionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "SDP", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ProvideAnswer {
  type Response = BasicAccResponse;
}
impl TlvEncode for ProvideAnswer {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.web_rtc_session_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.sdp.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProvideICECandidates {
  pub web_rtc_session_id: u16,
  pub ice_candidates: Vec<ty::ICECandidateStruct>,
}
impl Command for ProvideICECandidates {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "ProvideICECandidates",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "WebRTCSessionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ICECandidates", attr_ty: <Vec<ty::ICECandidateStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ProvideICECandidates {
  type Response = BasicAccResponse;
}
impl TlvEncode for ProvideICECandidates {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.web_rtc_session_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.ice_candidates.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EndSession {
  pub web_rtc_session_id: u16,
  pub reason: ty::WebRTCEndReasonEnum,
}
impl Command for EndSession {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "EndSession",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "WebRTCSessionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Reason", attr_ty: <ty::WebRTCEndReasonEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for EndSession {
  type Response = BasicAccResponse;
}
impl TlvEncode for EndSession {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.web_rtc_session_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.reason.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

}
pub mod ty {
  pub use crate::globals::*;
}
