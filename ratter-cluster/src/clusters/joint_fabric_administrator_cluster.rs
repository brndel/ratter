#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const JOINT_FABRIC_ADMINISTRATOR: ClusterDef = ClusterDef {
  id: 0x0753,
  name: "Joint Fabric Administrator",
  features: &[
  ],
  attributes: &[
    attr::AdministratorFabricIndex::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::ICACCSRRequest::COMMAND,
    command::AddICAC::COMMAND,
    command::OpenJointCommissioningWindow::COMMAND,
    command::TransferAnchorRequest::COMMAND,
    command::TransferAnchorComplete::COMMAND,
    command::AnnounceJointFabricAdministrator::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct AdministratorFabricIndex;

  impl Attr for AdministratorFabricIndex {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "AdministratorFabricIndex",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* FabricIdx */>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ICACCSRRequest {
}
impl Command for ICACCSRRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "ICACCSRRequest",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for ICACCSRRequest {
  type Response = ICACCSRResponse;
}
impl TlvEncode for ICACCSRRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ICACCSRResponse {
  pub status_code: ty::ICACCSRResponseStatusCodeEnum,
  pub icaccsr: TypeUnsupported /* Octstr */,
}
impl Command for ICACCSRResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "ICACCSRResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "StatusCode", attr_ty: <ty::ICACCSRResponseStatusCodeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ICACCSR", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::TooComplicated },
    ],
  };
}
impl CommandResponse for ICACCSRResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct AddICAC {
  pub icac_value: TypeUnsupported /* Octstr */,
}
impl Command for AddICAC {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "AddICAC",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ICACValue", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddICAC {
  type Response = ICACResponse;
}
impl TlvEncode for AddICAC {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.icac_value.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ICACResponse {
  pub status_code: ty::ICACResponseStatusEnum,
}
impl Command for ICACResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "ICACResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "StatusCode", attr_ty: <ty::ICACResponseStatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for ICACResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenJointCommissioningWindow {
  pub commissioning_timeout: u16,
  pub pake_passcode_verifier: TypeUnsupported /* Octstr */,
  pub discriminator: u16,
  pub iterations: u32,
  pub salt: TypeUnsupported /* Octstr */,
}
impl Command for OpenJointCommissioningWindow {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "OpenJointCommissioningWindow",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CommissioningTimeout", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "PAKEPasscodeVerifier", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Discriminator", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "Iterations", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "Salt", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for OpenJointCommissioningWindow {
  type Response = BasicAccResponse;
}
impl TlvEncode for OpenJointCommissioningWindow {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.commissioning_timeout.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.pake_passcode_verifier.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.discriminator.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.iterations.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.salt.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransferAnchorRequest {
}
impl Command for TransferAnchorRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "TransferAnchorRequest",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for TransferAnchorRequest {
  type Response = TransferAnchorResponse;
}
impl TlvEncode for TransferAnchorRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TransferAnchorResponse {
  pub status_code: ty::TransferAnchorResponseStatusEnum,
}
impl Command for TransferAnchorResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "TransferAnchorResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "StatusCode", attr_ty: <ty::TransferAnchorResponseStatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for TransferAnchorResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct TransferAnchorComplete {
}
impl Command for TransferAnchorComplete {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "TransferAnchorComplete",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for TransferAnchorComplete {
  type Response = BasicAccResponse;
}
impl TlvEncode for TransferAnchorComplete {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnnounceJointFabricAdministrator {
  pub endpoint_id: TypeUnsupported /* EndpointNo */,
}
impl Command for AnnounceJointFabricAdministrator {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0008,
    name: "AnnounceJointFabricAdministrator",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "EndpointID", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AnnounceJointFabricAdministrator {
  type Response = BasicAccResponse;
}
impl TlvEncode for AnnounceJointFabricAdministrator {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.endpoint_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
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
pub enum ICACCSRResponseStatusCodeEnum {
  ///No error
  OK = 0x0000,
  ///Could not be completed because another commissioning is in progress
  Busy = 0x0001,
  ///Provided PAKE parameters were incorrectly formatted or otherwise invalid
  PAKEParameterError = 0x0002,
  ///The Joint Commissioning Method commissioning window is not currently open
  WindowNotOpen = 0x0003,
  ///ICACCSRRequest command has been invoked by a peer against which Fabric Table VID Verification hasn't been executed
  VIDNotVerified = 0x0004,
  ///OpenJointCommissioningWindow command has been invoked but the AdministratorFabricIndex field has the value of null
  InvalidAdministratorFabricIndex = 0x0005,
}
impl AttrType for ICACCSRResponseStatusCodeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "OK"),
  (0x0001, "Busy"),
  (0x0002, "PAKEParameterError"),
  (0x0003, "WindowNotOpen"),
  (0x0004, "VIDNotVerified"),
  (0x0005, "InvalidAdministratorFabricIndex"),
  ]);
}
impl TlvPut for ICACCSRResponseStatusCodeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ICACCSRResponseStatusCodeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::OK),
      0x0001 => Ok(Self::Busy),
      0x0002 => Ok(Self::PAKEParameterError),
      0x0003 => Ok(Self::WindowNotOpen),
      0x0004 => Ok(Self::VIDNotVerified),
      0x0005 => Ok(Self::InvalidAdministratorFabricIndex),
      _ => Err(())
    }
  }
}
impl TlvDecode for ICACCSRResponseStatusCodeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ICACResponseStatusEnum {
  ///No error
  OK = 0x0000,
  ///Public Key in the ICAC is invalid
  InvalidPublicKey = 0x0001,
  ///ICAC chain validation failed / ICAC DN Encoding rules verification failed
  InvalidICAC = 0x0002,
}
impl AttrType for ICACResponseStatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "OK"),
  (0x0001, "InvalidPublicKey"),
  (0x0002, "InvalidICAC"),
  ]);
}
impl TlvPut for ICACResponseStatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ICACResponseStatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::OK),
      0x0001 => Ok(Self::InvalidPublicKey),
      0x0002 => Ok(Self::InvalidICAC),
      _ => Err(())
    }
  }
}
impl TlvDecode for ICACResponseStatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TransferAnchorResponseStatusEnum {
  ///No error
  OK = 0x0000,
  ///Anchor Transfer was not started due to on-going Datastore operations
  TransferAnchorStatusDatastoreBusy = 0x0001,
  ///User has not consented for Anchor Transfer
  TransferAnchorStatusNoUserConsent = 0x0002,
}
impl AttrType for TransferAnchorResponseStatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "OK"),
  (0x0001, "TransferAnchorStatusDatastoreBusy"),
  (0x0002, "TransferAnchorStatusNoUserConsent"),
  ]);
}
impl TlvPut for TransferAnchorResponseStatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TransferAnchorResponseStatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::OK),
      0x0001 => Ok(Self::TransferAnchorStatusDatastoreBusy),
      0x0002 => Ok(Self::TransferAnchorStatusNoUserConsent),
      _ => Err(())
    }
  }
}
impl TlvDecode for TransferAnchorResponseStatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
