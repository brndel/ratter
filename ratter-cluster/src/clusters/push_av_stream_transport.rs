#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const PUSH_AV_STREAM_TRANSPORT: ClusterDef = ClusterDef {
  id: 0x0555,
  name: "Push AV Stream Transport",
  features: &[
    FeatureDef { bit: 0x0000, code: "PERZONESENS", name: "PerZoneSensitivity", summary: "Supports a sensitivity value per Zone" },
    FeatureDef { bit: 0x0001, code: "METADATA", name: "Metadata", summary: "Supports metadata transmission in Push transports" },
  ],
  attributes: &[
    attr::SupportedFormats::ATTR,
    attr::CurrentConnections::ATTR,
  ],
  events: &[
    event::PushTransportBegin::EVENT,
    event::PushTransportEnd::EVENT,
  ],
  commands: &[
    command::AllocatePushTransport::COMMAND,
    command::DeallocatePushTransport::COMMAND,
    command::ModifyPushTransport::COMMAND,
    command::SetTransportStatus::COMMAND,
    command::ManuallyTriggerTransport::COMMAND,
    command::FindTransport::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct SupportedFormats;

  impl Attr for SupportedFormats {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "SupportedFormats",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::SupportedFormatStruct>;
  }


  pub struct CurrentConnections;

  impl Attr for CurrentConnections {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "CurrentConnections",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::TransportConfigurationStruct>;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct PushTransportBegin {
  pub connection_id: u16,
  pub trigger_type: ty::TransportTriggerTypeEnum,
  pub activation_reason: ty::TriggerActivationReasonEnum,
  pub container_type: ty::ContainerFormatEnum,
  pub cmaf_session_number: u64,
}

impl Event for PushTransportBegin {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "PushTransportBegin",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ConnectionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "TriggerType", attr_ty: <ty::TransportTriggerTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "ActivationReason", attr_ty: <ty::TriggerActivationReasonEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "ContainerType", attr_ty: <ty::ContainerFormatEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "CMAFSessionNumber", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for PushTransportBegin {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut connection_id: Option<u16> = None;
    let mut trigger_type: Option<ty::TransportTriggerTypeEnum> = None;
    let mut activation_reason: Option<ty::TriggerActivationReasonEnum> = None;
    let mut container_type: Option<ty::ContainerFormatEnum> = None;
    let mut cmaf_session_number: Option<u64> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { connection_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { trigger_type = Some(<ty::TransportTriggerTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { activation_reason = Some(<ty::TriggerActivationReasonEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { container_type = Some(<ty::ContainerFormatEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { cmaf_session_number = Some(<u64>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      connection_id: connection_id.ok_or(TlvDecodeErr::MissingField("ConnectionID"))?,
      trigger_type: trigger_type.ok_or(TlvDecodeErr::MissingField("TriggerType"))?,
      activation_reason: activation_reason.ok_or(TlvDecodeErr::MissingField("ActivationReason"))?,
      container_type: container_type.ok_or(TlvDecodeErr::MissingField("ContainerType"))?,
      cmaf_session_number: cmaf_session_number.ok_or(TlvDecodeErr::MissingField("CMAFSessionNumber"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct PushTransportEnd {
  pub connection_id: u16,
  pub container_type: ty::ContainerFormatEnum,
  pub cmaf_session_number: u64,
}

impl Event for PushTransportEnd {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "PushTransportEnd",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ConnectionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ContainerType", attr_ty: <ty::ContainerFormatEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "CMAFSessionNumber", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for PushTransportEnd {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut connection_id: Option<u16> = None;
    let mut container_type: Option<ty::ContainerFormatEnum> = None;
    let mut cmaf_session_number: Option<u64> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { connection_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { container_type = Some(<ty::ContainerFormatEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { cmaf_session_number = Some(<u64>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      connection_id: connection_id.ok_or(TlvDecodeErr::MissingField("ConnectionID"))?,
      container_type: container_type.ok_or(TlvDecodeErr::MissingField("ContainerType"))?,
      cmaf_session_number: cmaf_session_number.ok_or(TlvDecodeErr::MissingField("CMAFSessionNumber"))?,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct AllocatePushTransport {
  pub transport_options: ty::TransportOptionsStruct,
}
impl Command for AllocatePushTransport {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "AllocatePushTransport",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "TransportOptions", attr_ty: <ty::TransportOptionsStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AllocatePushTransport {
  type Response = AllocatePushTransportResponse;
}
impl TlvEncode for AllocatePushTransport {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.transport_options.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AllocatePushTransportResponse {
  pub transport_configuration: ty::TransportConfigurationStruct,
}
impl Command for AllocatePushTransportResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "AllocatePushTransportResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "TransportConfiguration", attr_ty: <ty::TransportConfigurationStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for AllocatePushTransportResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct DeallocatePushTransport {
  pub connection_id: u16,
}
impl Command for DeallocatePushTransport {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "DeallocatePushTransport",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ConnectionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for DeallocatePushTransport {
  type Response = BasicAccResponse;
}
impl TlvEncode for DeallocatePushTransport {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.connection_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ModifyPushTransport {
  pub connection_id: u16,
  pub transport_options: ty::TransportOptionsStruct,
}
impl Command for ModifyPushTransport {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "ModifyPushTransport",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ConnectionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "TransportOptions", attr_ty: <ty::TransportOptionsStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ModifyPushTransport {
  type Response = BasicAccResponse;
}
impl TlvEncode for ModifyPushTransport {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.connection_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.transport_options.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetTransportStatus {
  pub connection_id: u16,
  pub transport_status: ty::TransportStatusEnum,
}
impl Command for SetTransportStatus {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "SetTransportStatus",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ConnectionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "TransportStatus", attr_ty: <ty::TransportStatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SetTransportStatus {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetTransportStatus {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.connection_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.transport_status.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ManuallyTriggerTransport {
  pub connection_id: u16,
  pub activation_reason: ty::TriggerActivationReasonEnum,
  pub time_control: ty::TransportMotionTriggerTimeControlStruct,
  pub user_defined: TypeUnsupported /* Octstr */,
}
impl Command for ManuallyTriggerTransport {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "ManuallyTriggerTransport",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ConnectionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ActivationReason", attr_ty: <ty::TriggerActivationReasonEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TimeControl", attr_ty: <ty::TransportMotionTriggerTimeControlStruct>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0003, name: "UserDefined", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for ManuallyTriggerTransport {
  type Response = BasicAccResponse;
}
impl TlvEncode for ManuallyTriggerTransport {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.connection_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.activation_reason.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.time_control.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.user_defined.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FindTransport {
  pub connection_id: u16,
}
impl Command for FindTransport {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "FindTransport",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ConnectionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for FindTransport {
  type Response = FindTransportResponse;
}
impl TlvEncode for FindTransport {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.connection_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FindTransportResponse {
  pub transport_configurations: Vec<ty::TransportConfigurationStruct>,
}
impl Command for FindTransportResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "FindTransportResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "TransportConfigurations", attr_ty: <Vec<ty::TransportConfigurationStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for FindTransportResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum CMAFInterfaceEnum {
  ///CMAF Interface-1 Mode
  Interface1 = 0x0000,
  ///CMAF Interface-2 Mode with DASH Support
  Interface2DASH = 0x0001,
  ///CMAF Interface-2 Mode with HLS Support
  Interface2HLS = 0x0002,
}
impl AttrType for CMAFInterfaceEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Interface1"),
  (0x0001, "Interface2DASH"),
  (0x0002, "Interface2HLS"),
  ]);
}
impl TlvPut for CMAFInterfaceEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for CMAFInterfaceEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Interface1),
      0x0001 => Ok(Self::Interface2DASH),
      0x0002 => Ok(Self::Interface2HLS),
      _ => Err(())
    }
  }
}
impl TlvDecode for CMAFInterfaceEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ContainerFormatEnum {
  ///CMAF container format
  CMAF = 0x0000,
}
impl AttrType for ContainerFormatEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "CMAF"),
  ]);
}
impl TlvPut for ContainerFormatEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ContainerFormatEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::CMAF),
      _ => Err(())
    }
  }
}
impl TlvDecode for ContainerFormatEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum IngestMethodsEnum {
  ///CMAF ingestion format
  CMAFIngest = 0x0000,
}
impl AttrType for IngestMethodsEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "CMAFIngest"),
  ]);
}
impl TlvPut for IngestMethodsEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for IngestMethodsEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::CMAFIngest),
      _ => Err(())
    }
  }
}
impl TlvDecode for IngestMethodsEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StatusCodeEnum {
  ///The specified TLSEndpointID cannot be found.
  InvalidTLSEndpoint = 0x0002,
  ///The specified VideoStreamID or AudioStreamID cannot be found.
  InvalidStream = 0x0003,
  ///The specified URL is invalid.
  InvalidURL = 0x0004,
  ///A specified ZoneID was invalid.
  InvalidZone = 0x0005,
  ///The specified combination of Ingestion method and Container format is not supported.
  InvalidCombination = 0x0006,
  ///The trigger type is invalid for this command.
  InvalidTriggerType = 0x0007,
  ///The Stream Transport Status is invalid for this command.
  InvalidTransportStatus = 0x0008,
  ///The requested Container options are not supported with the streams indicated.
  InvalidOptions = 0x0009,
  ///The requested StreamUsage is not allowed.
  InvalidStreamUsage = 0x000a,
  ///Time sync has not occurred yet.
  InvalidTime = 0x000b,
  ///The requested pre roll length is not compatible with the streams key frame interval.
  InvalidPreRollLength = 0x000c,
  ///The requested Streams info contained duplicate entries.
  DuplicateStreamValues = 0x000d,
}
impl AttrType for StatusCodeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0002, "InvalidTLSEndpoint"),
  (0x0003, "InvalidStream"),
  (0x0004, "InvalidURL"),
  (0x0005, "InvalidZone"),
  (0x0006, "InvalidCombination"),
  (0x0007, "InvalidTriggerType"),
  (0x0008, "InvalidTransportStatus"),
  (0x0009, "InvalidOptions"),
  (0x000a, "InvalidStreamUsage"),
  (0x000b, "InvalidTime"),
  (0x000c, "InvalidPreRollLength"),
  (0x000d, "DuplicateStreamValues"),
  ]);
}
impl TlvPut for StatusCodeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StatusCodeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0002 => Ok(Self::InvalidTLSEndpoint),
      0x0003 => Ok(Self::InvalidStream),
      0x0004 => Ok(Self::InvalidURL),
      0x0005 => Ok(Self::InvalidZone),
      0x0006 => Ok(Self::InvalidCombination),
      0x0007 => Ok(Self::InvalidTriggerType),
      0x0008 => Ok(Self::InvalidTransportStatus),
      0x0009 => Ok(Self::InvalidOptions),
      0x000a => Ok(Self::InvalidStreamUsage),
      0x000b => Ok(Self::InvalidTime),
      0x000c => Ok(Self::InvalidPreRollLength),
      0x000d => Ok(Self::DuplicateStreamValues),
      _ => Err(())
    }
  }
}
impl TlvDecode for StatusCodeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TransportStatusEnum {
  ///Push Transport can transport AV Streams
  Active = 0x0000,
  ///Push Transport cannot transport AV Streams
  Inactive = 0x0001,
}
impl AttrType for TransportStatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Active"),
  (0x0001, "Inactive"),
  ]);
}
impl TlvPut for TransportStatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TransportStatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Active),
      0x0001 => Ok(Self::Inactive),
      _ => Err(())
    }
  }
}
impl TlvDecode for TransportStatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TransportTriggerTypeEnum {
  ///Triggered only via a command invocation
  Command = 0x0000,
  ///Triggered via motion detection or command
  Motion = 0x0001,
  ///Triggered always when transport status is Active
  Continuous = 0x0002,
}
impl AttrType for TransportTriggerTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Command"),
  (0x0001, "Motion"),
  (0x0002, "Continuous"),
  ]);
}
impl TlvPut for TransportTriggerTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TransportTriggerTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Command),
      0x0001 => Ok(Self::Motion),
      0x0002 => Ok(Self::Continuous),
      _ => Err(())
    }
  }
}
impl TlvDecode for TransportTriggerTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TriggerActivationReasonEnum {
  ///Trigger has been activated by user action
  UserInitiated = 0x0000,
  ///Trigger has been activated by automation
  Automation = 0x0001,
  ///Trigger has been activated for emergency reasons
  Emergency = 0x0002,
  ///Trigger has been activated by a doorbell press
  DoorbellPressed = 0x0003,
}
impl AttrType for TriggerActivationReasonEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "UserInitiated"),
  (0x0001, "Automation"),
  (0x0002, "Emergency"),
  (0x0003, "DoorbellPressed"),
  ]);
}
impl TlvPut for TriggerActivationReasonEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TriggerActivationReasonEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::UserInitiated),
      0x0001 => Ok(Self::Automation),
      0x0002 => Ok(Self::Emergency),
      0x0003 => Ok(Self::DoorbellPressed),
      _ => Err(())
    }
  }
}
impl TlvDecode for TriggerActivationReasonEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AudioStreamStruct {
  pub audio_stream_name: String,
  pub audio_stream_id: u16,
}
impl AttrType for AudioStreamStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "AudioStreamName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "AudioStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for AudioStreamStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.audio_stream_name.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.audio_stream_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for AudioStreamStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut audio_stream_name: Option<String> = None;
    let mut audio_stream_id: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { audio_stream_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { audio_stream_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      audio_stream_name: audio_stream_name.ok_or(TlvDecodeErr::MissingField("AudioStreamName"))?,
      audio_stream_id: audio_stream_id.ok_or(TlvDecodeErr::MissingField("AudioStreamID"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CMAFContainerOptionsStruct {
  pub cmaf_interface: CMAFInterfaceEnum,
  pub segment_duration: u16,
  pub chunk_duration: u16,
  pub session_group: u8,
  pub track_name: String,
  pub metadata_enabled: bool,
}
impl AttrType for CMAFContainerOptionsStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "CMAFInterface", attr_ty: <CMAFInterfaceEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "SegmentDuration", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "ChunkDuration", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "SessionGroup", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "TrackName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "MetadataEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for CMAFContainerOptionsStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.cmaf_interface.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.segment_duration.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.chunk_duration.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.session_group.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.track_name.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.metadata_enabled.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for CMAFContainerOptionsStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut cmaf_interface: Option<CMAFInterfaceEnum> = None;
    let mut segment_duration: Option<u16> = None;
    let mut chunk_duration: Option<u16> = None;
    let mut session_group: Option<u8> = None;
    let mut track_name: Option<String> = None;
    let mut metadata_enabled: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { cmaf_interface = Some(<CMAFInterfaceEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { segment_duration = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { chunk_duration = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { session_group = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { track_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0007), value: _ }) => { metadata_enabled = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      cmaf_interface: cmaf_interface.ok_or(TlvDecodeErr::MissingField("CMAFInterface"))?,
      segment_duration: segment_duration.ok_or(TlvDecodeErr::MissingField("SegmentDuration"))?,
      chunk_duration: chunk_duration.ok_or(TlvDecodeErr::MissingField("ChunkDuration"))?,
      session_group: session_group.ok_or(TlvDecodeErr::MissingField("SessionGroup"))?,
      track_name: track_name.ok_or(TlvDecodeErr::MissingField("TrackName"))?,
      metadata_enabled: metadata_enabled.ok_or(TlvDecodeErr::MissingField("MetadataEnabled"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContainerOptionsStruct {
  pub container_type: ContainerFormatEnum,
  pub cmaf_container_options: CMAFContainerOptionsStruct,
}
impl AttrType for ContainerOptionsStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ContainerType", attr_ty: <ContainerFormatEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "CMAFContainerOptions", attr_ty: <CMAFContainerOptionsStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ContainerOptionsStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.container_type.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.cmaf_container_options.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ContainerOptionsStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut container_type: Option<ContainerFormatEnum> = None;
    let mut cmaf_container_options: Option<CMAFContainerOptionsStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { container_type = Some(<ContainerFormatEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { cmaf_container_options = Some(<CMAFContainerOptionsStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      container_type: container_type.ok_or(TlvDecodeErr::MissingField("ContainerType"))?,
      cmaf_container_options: cmaf_container_options.ok_or(TlvDecodeErr::MissingField("CMAFContainerOptions"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SupportedFormatStruct {
  pub container_format: ContainerFormatEnum,
  pub ingest_method: IngestMethodsEnum,
}
impl AttrType for SupportedFormatStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ContainerFormat", attr_ty: <ContainerFormatEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "IngestMethod", attr_ty: <IngestMethodsEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for SupportedFormatStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.container_format.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.ingest_method.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for SupportedFormatStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut container_format: Option<ContainerFormatEnum> = None;
    let mut ingest_method: Option<IngestMethodsEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { container_format = Some(<ContainerFormatEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { ingest_method = Some(<IngestMethodsEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      container_format: container_format.ok_or(TlvDecodeErr::MissingField("ContainerFormat"))?,
      ingest_method: ingest_method.ok_or(TlvDecodeErr::MissingField("IngestMethod"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransportConfigurationStruct {
  pub connection_id: u16,
  pub transport_status: TransportStatusEnum,
  pub transport_options: TransportOptionsStruct,
}
impl AttrType for TransportConfigurationStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ConnectionID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "TransportStatus", attr_ty: <TransportStatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "TransportOptions", attr_ty: <TransportOptionsStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TransportConfigurationStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.connection_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.transport_status.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.transport_options.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TransportConfigurationStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut connection_id: Option<u16> = None;
    let mut transport_status: Option<TransportStatusEnum> = None;
    let mut transport_options: Option<TransportOptionsStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { connection_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { transport_status = Some(<TransportStatusEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { transport_options = Some(<TransportOptionsStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      connection_id: connection_id.ok_or(TlvDecodeErr::MissingField("ConnectionID"))?,
      transport_status: transport_status.ok_or(TlvDecodeErr::MissingField("TransportStatus"))?,
      transport_options: transport_options.ok_or(TlvDecodeErr::MissingField("TransportOptions"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransportMotionTriggerTimeControlStruct {
  pub initial_duration: u16,
  pub augmentation_duration: u16,
  pub max_duration: TypeUnsupported /* ElapsedS */,
  pub blind_duration: u16,
}
impl AttrType for TransportMotionTriggerTimeControlStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "InitialDuration", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "AugmentationDuration", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "MaxDuration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "BlindDuration", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TransportMotionTriggerTimeControlStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.initial_duration.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.augmentation_duration.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.max_duration.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.blind_duration.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TransportMotionTriggerTimeControlStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut initial_duration: Option<u16> = None;
    let mut augmentation_duration: Option<u16> = None;
    let mut blind_duration: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { initial_duration = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { augmentation_duration = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { blind_duration = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      initial_duration: initial_duration.ok_or(TlvDecodeErr::MissingField("InitialDuration"))?,
      augmentation_duration: augmentation_duration.ok_or(TlvDecodeErr::MissingField("AugmentationDuration"))?,
      max_duration: TypeUnsupported,
      blind_duration: blind_duration.ok_or(TlvDecodeErr::MissingField("BlindDuration"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransportOptionsStruct {
  pub stream_usage: StreamUsageEnum,
  pub video_stream_id: u16,
  pub audio_stream_id: u16,
  pub tls_endpoint_id: u16,
  pub url: String,
  pub trigger_options: TransportTriggerOptionsStruct,
  pub ingest_method: IngestMethodsEnum,
  pub container_options: ContainerOptionsStruct,
  pub expiry_time: TypeUnsupported /* EpochS */,
  pub video_streams: Vec<VideoStreamStruct>,
  pub audio_streams: Vec<AudioStreamStruct>,
}
impl AttrType for TransportOptionsStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "StreamUsage", attr_ty: <StreamUsageEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "VideoStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "AudioStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "TLSEndpointID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "URL", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "TriggerOptions", attr_ty: <TransportTriggerOptionsStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "IngestMethod", attr_ty: <IngestMethodsEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "ContainerOptions", attr_ty: <ContainerOptionsStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0008, name: "ExpiryTime", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0009, name: "VideoStreams", attr_ty: <Vec<VideoStreamStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000a, name: "AudioStreams", attr_ty: <Vec<AudioStreamStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TransportOptionsStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.stream_usage.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.video_stream_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.audio_stream_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.tls_endpoint_id.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.url.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.trigger_options.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.ingest_method.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.container_options.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.expiry_time.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    self.video_streams.tlv_put(matter_codec::Tag::Context(0x0009), writer);
    self.audio_streams.tlv_put(matter_codec::Tag::Context(0x000a), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TransportOptionsStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut stream_usage: Option<StreamUsageEnum> = None;
    let mut video_stream_id: Option<u16> = None;
    let mut audio_stream_id: Option<u16> = None;
    let mut tls_endpoint_id: Option<u16> = None;
    let mut url: Option<String> = None;
    let mut trigger_options: Option<TransportTriggerOptionsStruct> = None;
    let mut ingest_method: Option<IngestMethodsEnum> = None;
    let mut container_options: Option<ContainerOptionsStruct> = None;
    let mut video_streams: Option<Vec<VideoStreamStruct>> = None;
    let mut audio_streams: Option<Vec<AudioStreamStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { stream_usage = Some(<StreamUsageEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { video_stream_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { audio_stream_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { tls_endpoint_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { url = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0005), kind: _ }) => { trigger_options = Some(<TransportTriggerOptionsStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { ingest_method = Some(<IngestMethodsEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0007), kind: _ }) => { container_options = Some(<ContainerOptionsStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0009), kind: _ }) => { video_streams = Some(<Vec<VideoStreamStruct>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x000a), kind: _ }) => { audio_streams = Some(<Vec<AudioStreamStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      stream_usage: stream_usage.ok_or(TlvDecodeErr::MissingField("StreamUsage"))?,
      video_stream_id: video_stream_id.ok_or(TlvDecodeErr::MissingField("VideoStreamID"))?,
      audio_stream_id: audio_stream_id.ok_or(TlvDecodeErr::MissingField("AudioStreamID"))?,
      tls_endpoint_id: tls_endpoint_id.ok_or(TlvDecodeErr::MissingField("TLSEndpointID"))?,
      url: url.ok_or(TlvDecodeErr::MissingField("URL"))?,
      trigger_options: trigger_options.ok_or(TlvDecodeErr::MissingField("TriggerOptions"))?,
      ingest_method: ingest_method.ok_or(TlvDecodeErr::MissingField("IngestMethod"))?,
      container_options: container_options.ok_or(TlvDecodeErr::MissingField("ContainerOptions"))?,
      expiry_time: TypeUnsupported,
      video_streams: video_streams.ok_or(TlvDecodeErr::MissingField("VideoStreams"))?,
      audio_streams: audio_streams.ok_or(TlvDecodeErr::MissingField("AudioStreams"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransportTriggerOptionsStruct {
  pub trigger_type: TransportTriggerTypeEnum,
  pub motion_zones: Vec<TransportZoneOptionsStruct>,
  pub motion_sensitivity: u8,
  pub motion_time_control: TransportMotionTriggerTimeControlStruct,
  pub max_pre_roll_len: u16,
}
impl AttrType for TransportTriggerOptionsStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "TriggerType", attr_ty: <TransportTriggerTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "MotionZones", attr_ty: <Vec<TransportZoneOptionsStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "MotionSensitivity", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "MotionTimeControl", attr_ty: <TransportMotionTriggerTimeControlStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "MaxPreRollLen", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TransportTriggerOptionsStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.trigger_type.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.motion_zones.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.motion_sensitivity.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.motion_time_control.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.max_pre_roll_len.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TransportTriggerOptionsStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut trigger_type: Option<TransportTriggerTypeEnum> = None;
    let mut motion_zones: Option<Vec<TransportZoneOptionsStruct>> = None;
    let mut motion_sensitivity: Option<u8> = None;
    let mut motion_time_control: Option<TransportMotionTriggerTimeControlStruct> = None;
    let mut max_pre_roll_len: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { trigger_type = Some(<TransportTriggerTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { motion_zones = Some(<Vec<TransportZoneOptionsStruct>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { motion_sensitivity = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0003), kind: _ }) => { motion_time_control = Some(<TransportMotionTriggerTimeControlStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { max_pre_roll_len = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      trigger_type: trigger_type.ok_or(TlvDecodeErr::MissingField("TriggerType"))?,
      motion_zones: motion_zones.ok_or(TlvDecodeErr::MissingField("MotionZones"))?,
      motion_sensitivity: motion_sensitivity.ok_or(TlvDecodeErr::MissingField("MotionSensitivity"))?,
      motion_time_control: motion_time_control.ok_or(TlvDecodeErr::MissingField("MotionTimeControl"))?,
      max_pre_roll_len: max_pre_roll_len.ok_or(TlvDecodeErr::MissingField("MaxPreRollLen"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransportZoneOptionsStruct {
  pub zone: u16,
  pub sensitivity: u8,
}
impl AttrType for TransportZoneOptionsStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Zone", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Sensitivity", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TransportZoneOptionsStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.zone.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.sensitivity.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TransportZoneOptionsStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut zone: Option<u16> = None;
    let mut sensitivity: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { zone = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { sensitivity = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      zone: zone.ok_or(TlvDecodeErr::MissingField("Zone"))?,
      sensitivity: sensitivity.ok_or(TlvDecodeErr::MissingField("Sensitivity"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VideoStreamStruct {
  pub video_stream_name: String,
  pub video_stream_id: u16,
}
impl AttrType for VideoStreamStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "VideoStreamName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "VideoStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for VideoStreamStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.video_stream_name.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.video_stream_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for VideoStreamStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut video_stream_name: Option<String> = None;
    let mut video_stream_id: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { video_stream_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { video_stream_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      video_stream_name: video_stream_name.ok_or(TlvDecodeErr::MissingField("VideoStreamName"))?,
      video_stream_id: video_stream_id.ok_or(TlvDecodeErr::MissingField("VideoStreamID"))?,
    })
  }
}

}
