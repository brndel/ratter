#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const DIAGNOSTIC_LOGS: ClusterDef = ClusterDef {
  id: 0x0032,
  name: "Diagnostic Logs",
  features: &[
  ],
  attributes: &[
  ],
  events: &[
  ],
  commands: &[
    command::RetrieveLogsRequest::COMMAND,
  ],
};
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct RetrieveLogsRequest {
  pub intent: ty::IntentEnum,
  pub requested_protocol: ty::TransferProtocolEnum,
  pub transfer_file_designator: String,
}
impl Command for RetrieveLogsRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "RetrieveLogsRequest",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Intent", attr_ty: <ty::IntentEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "RequestedProtocol", attr_ty: <ty::TransferProtocolEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TransferFileDesignator", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for RetrieveLogsRequest {
  type Response = RetrieveLogsResponse;
}
impl TlvEncode for RetrieveLogsRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.intent.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.requested_protocol.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.transfer_file_designator.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RetrieveLogsResponse {
  pub status: ty::StatusEnum,
  pub log_content: TypeUnsupported /* Octstr */,
  pub utc_time_stamp: TypeUnsupported /* EpochUs */,
  pub time_since_boot: TypeUnsupported /* SystemtimeUs */,
}
impl Command for RetrieveLogsResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "RetrieveLogsResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <ty::StatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "LogContent", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "UTCTimeStamp", attr_ty: <TypeUnsupported /* EpochUs */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0003, name: "TimeSinceBoot", attr_ty: <TypeUnsupported /* SystemtimeUs */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandResponse for RetrieveLogsResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum IntentEnum {
  ///Logs to be used for end-user support
  EndUserSupport = 0x0000,
  ///Logs to be used for network diagnostics
  NetworkDiag = 0x0001,
  ///Obtain crash logs from the Node
  CrashLogs = 0x0002,
}
impl AttrType for IntentEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "EndUserSupport"),
  (0x0001, "NetworkDiag"),
  (0x0002, "CrashLogs"),
  ]);
}
impl TlvPut for IntentEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for IntentEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::EndUserSupport),
      0x0001 => Ok(Self::NetworkDiag),
      0x0002 => Ok(Self::CrashLogs),
      _ => Err(())
    }
  }
}
impl TlvDecode for IntentEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StatusEnum {
  ///Successful transfer of logs
  Success = 0x0000,
  ///All logs have been transferred
  Exhausted = 0x0001,
  ///No logs of the requested type available
  NoLogs = 0x0002,
  ///Unable to handle request, retry later
  Busy = 0x0003,
  ///The request is denied, no logs being transferred
  Denied = 0x0004,
}
impl AttrType for StatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Success"),
  (0x0001, "Exhausted"),
  (0x0002, "NoLogs"),
  (0x0003, "Busy"),
  (0x0004, "Denied"),
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
      0x0001 => Ok(Self::Exhausted),
      0x0002 => Ok(Self::NoLogs),
      0x0003 => Ok(Self::Busy),
      0x0004 => Ok(Self::Denied),
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
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TransferProtocolEnum {
  ///Logs to be returned as a response
  ResponsePayload = 0x0000,
  ///Logs to be returned using BDX
  BDX = 0x0001,
}
impl AttrType for TransferProtocolEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "ResponsePayload"),
  (0x0001, "BDX"),
  ]);
}
impl TlvPut for TransferProtocolEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TransferProtocolEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::ResponsePayload),
      0x0001 => Ok(Self::BDX),
      _ => Err(())
    }
  }
}
impl TlvDecode for TransferProtocolEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
