#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const OTA_SOFTWARE_UPDATE_PROVIDER: ClusterDef = ClusterDef {
  id: 0x0029,
  name: "OTA Software Update Provider",
  features: &[
  ],
  attributes: &[
  ],
  events: &[
  ],
  commands: &[
    command::QueryImage::COMMAND,
    command::ApplyUpdateRequest::COMMAND,
    command::NotifyUpdateApplied::COMMAND,
  ],
};
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct QueryImage {
  pub vendor_id: TypeUnsupported /* VendorId */,
  pub product_id: u16,
  pub software_version: u32,
  pub protocols_supported: Vec<ty::DownloadProtocolEnum>,
  pub hardware_version: u16,
  pub location: String,
  pub requestor_can_consent: bool,
  pub metadata_for_provider: TypeUnsupported /* Octstr */,
}
impl Command for QueryImage {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "QueryImage",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "VendorID", attr_ty: <TypeUnsupported /* VendorId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ProductID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "SoftwareVersion", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "ProtocolsSupported", attr_ty: <Vec<ty::DownloadProtocolEnum>>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "HardwareVersion", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0005, name: "Location", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0006, name: "RequestorCanConsent", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0007, name: "MetadataForProvider", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for QueryImage {
  type Response = QueryImageResponse;
}
impl TlvEncode for QueryImage {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.vendor_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.product_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.software_version.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.protocols_supported.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.hardware_version.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.location.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.requestor_can_consent.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.metadata_for_provider.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct QueryImageResponse {
  pub status: ty::StatusEnum,
  pub delayed_action_time: u32,
  pub image_uri: String,
  pub software_version: u32,
  pub software_version_string: String,
  pub update_token: TypeUnsupported /* Octstr */,
  pub user_consent_needed: bool,
  pub metadata_for_requestor: TypeUnsupported /* Octstr */,
}
impl Command for QueryImageResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "QueryImageResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <ty::StatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "DelayedActionTime", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x0002, name: "ImageURI", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "SoftwareVersion", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "SoftwareVersionString", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0005, name: "UpdateToken", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0006, name: "UserConsentNeeded", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0007, name: "MetadataForRequestor", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandResponse for QueryImageResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct ApplyUpdateRequest {
  pub update_token: TypeUnsupported /* Octstr */,
  pub new_version: u32,
}
impl Command for ApplyUpdateRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "ApplyUpdateRequest",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "UpdateToken", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "NewVersion", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ApplyUpdateRequest {
  type Response = ApplyUpdateResponse;
}
impl TlvEncode for ApplyUpdateRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.update_token.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.new_version.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApplyUpdateResponse {
  pub action: ty::ApplyUpdateActionEnum,
  pub delayed_action_time: u32,
}
impl Command for ApplyUpdateResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "ApplyUpdateResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Action", attr_ty: <ty::ApplyUpdateActionEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "DelayedActionTime", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for ApplyUpdateResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct NotifyUpdateApplied {
  pub update_token: TypeUnsupported /* Octstr */,
  pub software_version: u32,
}
impl Command for NotifyUpdateApplied {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "NotifyUpdateApplied",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "UpdateToken", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "SoftwareVersion", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for NotifyUpdateApplied {
  type Response = BasicAccResponse;
}
impl TlvEncode for NotifyUpdateApplied {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.update_token.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.software_version.tlv_put(matter_codec::Tag::Context(0x0001), writer);
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
pub enum ApplyUpdateActionEnum {
  ///Apply the update.
  Proceed = 0x0000,
  ///Wait at least the given delay time.
  AwaitNextAction = 0x0001,
  ///The OTA Provider is conveying a desire to rescind a previously provided Software Image.
  Discontinue = 0x0002,
}
impl AttrType for ApplyUpdateActionEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Proceed"),
  (0x0001, "AwaitNextAction"),
  (0x0002, "Discontinue"),
  ]);
}
impl TlvPut for ApplyUpdateActionEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ApplyUpdateActionEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Proceed),
      0x0001 => Ok(Self::AwaitNextAction),
      0x0002 => Ok(Self::Discontinue),
      _ => Err(())
    }
  }
}
impl TlvDecode for ApplyUpdateActionEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum DownloadProtocolEnum {
  ///Indicates support for synchronous BDX.
  BDXSynchronous = 0x0000,
  ///Indicates support for asynchronous BDX.
  BDXAsynchronous = 0x0001,
  ///Indicates support for HTTPS.
  HTTPS = 0x0002,
  ///Indicates support for vendor specific protocol.
  VendorSpecific = 0x0003,
}
impl AttrType for DownloadProtocolEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "BDXSynchronous"),
  (0x0001, "BDXAsynchronous"),
  (0x0002, "HTTPS"),
  (0x0003, "VendorSpecific"),
  ]);
}
impl TlvPut for DownloadProtocolEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for DownloadProtocolEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::BDXSynchronous),
      0x0001 => Ok(Self::BDXAsynchronous),
      0x0002 => Ok(Self::HTTPS),
      0x0003 => Ok(Self::VendorSpecific),
      _ => Err(())
    }
  }
}
impl TlvDecode for DownloadProtocolEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StatusEnum {
  ///Indicates that the OTA Provider has an update available.
  UpdateAvailable = 0x0000,
  ///Indicates OTA Provider may have an update, but it is not ready yet.
  Busy = 0x0001,
  ///Indicates that there is definitely no update currently available from the OTA Provider.
  NotAvailable = 0x0002,
  ///Indicates that the requested download protocol is not supported by the OTA Provider.
  DownloadProtocolNotSupported = 0x0003,
}
impl AttrType for StatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "UpdateAvailable"),
  (0x0001, "Busy"),
  (0x0002, "NotAvailable"),
  (0x0003, "DownloadProtocolNotSupported"),
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
      0x0000 => Ok(Self::UpdateAvailable),
      0x0001 => Ok(Self::Busy),
      0x0002 => Ok(Self::NotAvailable),
      0x0003 => Ok(Self::DownloadProtocolNotSupported),
      _ => Err(())
    }
  }
}
impl TlvDecode for StatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
