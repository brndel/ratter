#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const WEB_RTC_TRANSPORT_REQUESTOR: ClusterDef = ClusterDef {
  id: 0x0554,
  name: "WebRTC Transport Requestor",
  features: &[
  ],
  attributes: &[
    attr::CurrentSessions::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::Offer::COMMAND,
    command::Answer::COMMAND,
    command::ICECandidates::COMMAND,
    command::End::COMMAND,
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
pub struct Offer {
  pub web_rtc_session_id: u16,
  pub sdp: String,
  pub ice_servers: Vec<ty::ICEServerStruct>,
  pub ice_transport_policy: String,
}
impl Command for Offer {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "Offer",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "WebRTCSessionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "SDP", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "ICEServers", attr_ty: <Vec<ty::ICEServerStruct>>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0003, name: "ICETransportPolicy", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for Offer {
  type Response = BasicAccResponse;
}
impl TlvEncode for Offer {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.web_rtc_session_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.sdp.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.ice_servers.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.ice_transport_policy.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Answer {
  pub web_rtc_session_id: u16,
  pub sdp: String,
}
impl Command for Answer {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "Answer",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "WebRTCSessionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "SDP", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for Answer {
  type Response = BasicAccResponse;
}
impl TlvEncode for Answer {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.web_rtc_session_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.sdp.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ICECandidates {
  pub web_rtc_session_id: u16,
  pub ice_candidates: Vec<ty::ICECandidateStruct>,
}
impl Command for ICECandidates {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "ICECandidates",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "WebRTCSessionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ICECandidates", attr_ty: <Vec<ty::ICECandidateStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ICECandidates {
  type Response = BasicAccResponse;
}
impl TlvEncode for ICECandidates {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.web_rtc_session_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.ice_candidates.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct End {
  pub web_rtc_session_id: u16,
  pub reason: ty::WebRTCEndReasonEnum,
}
impl Command for End {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "End",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "WebRTCSessionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Reason", attr_ty: <ty::WebRTCEndReasonEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for End {
  type Response = BasicAccResponse;
}
impl TlvEncode for End {
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
