#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const OPERATIONAL_CREDENTIALS: ClusterDef = ClusterDef {
  id: 0x003e,
  name: "Operational Credentials",
  features: &[
  ],
  attributes: &[
    attr::NOCs::ATTR,
    attr::Fabrics::ATTR,
    attr::SupportedFabrics::ATTR,
    attr::CommissionedFabrics::ATTR,
    attr::TrustedRootCertificates::ATTR,
    attr::CurrentFabricIndex::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::AttestationRequest::COMMAND,
    command::CertificateChainRequest::COMMAND,
    command::CSRRequest::COMMAND,
    command::AddNOC::COMMAND,
    command::UpdateNOC::COMMAND,
    command::UpdateFabricLabel::COMMAND,
    command::RemoveFabric::COMMAND,
    command::AddTrustedRootCertificate::COMMAND,
    command::SetVIDVerificationStatement::COMMAND,
    command::SignVIDVerificationRequest::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct NOCs;

  impl Attr for NOCs {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "NOCs",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::NOCStruct>;
  }


  pub struct Fabrics;

  impl Attr for Fabrics {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "Fabrics",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::FabricDescriptorStruct>;
  }


  pub struct SupportedFabrics;

  impl Attr for SupportedFabrics {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "SupportedFabrics",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct CommissionedFabrics;

  impl Attr for CommissionedFabrics {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "CommissionedFabrics",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct TrustedRootCertificates;

  impl Attr for TrustedRootCertificates {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "TrustedRootCertificates",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<TypeUnsupported /* Octstr */>;
  }


  pub struct CurrentFabricIndex;

  impl Attr for CurrentFabricIndex {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "CurrentFabricIndex",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct AttestationRequest {
  pub attestation_nonce: TypeUnsupported /* Octstr */,
}
impl Command for AttestationRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "AttestationRequest",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "AttestationNonce", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AttestationRequest {
  type Response = AttestationResponse;
}
impl TlvEncode for AttestationRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.attestation_nonce.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AttestationResponse {
  pub attestation_elements: TypeUnsupported /* Octstr */,
  pub attestation_signature: TypeUnsupported /* Octstr */,
}
impl Command for AttestationResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "AttestationResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "AttestationElements", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "AttestationSignature", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for AttestationResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct CertificateChainRequest {
  pub certificate_type: ty::CertificateChainTypeEnum,
}
impl Command for CertificateChainRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "CertificateChainRequest",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CertificateType", attr_ty: <ty::CertificateChainTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for CertificateChainRequest {
  type Response = CertificateChainResponse;
}
impl TlvEncode for CertificateChainRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.certificate_type.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CertificateChainResponse {
  pub certificate: TypeUnsupported /* Octstr */,
}
impl Command for CertificateChainResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "CertificateChainResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Certificate", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for CertificateChainResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct CSRRequest {
  pub csr_nonce: TypeUnsupported /* Octstr */,
  pub is_for_update_noc: bool,
}
impl Command for CSRRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "CSRRequest",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CSRNonce", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "IsForUpdateNOC", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for CSRRequest {
  type Response = CSRResponse;
}
impl TlvEncode for CSRRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.csr_nonce.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.is_for_update_noc.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CSRResponse {
  pub nocsr_elements: TypeUnsupported /* Octstr */,
  pub attestation_signature: TypeUnsupported /* Octstr */,
}
impl Command for CSRResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "CSRResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NOCSRElements", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "AttestationSignature", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for CSRResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct AddNOC {
  pub noc_value: TypeUnsupported /* Octstr */,
  pub icac_value: TypeUnsupported /* Octstr */,
  pub ipk_value: TypeUnsupported /* Octstr */,
  pub case_admin_subject: TypeUnsupported /* SubjectId */,
  pub admin_vendor_id: TypeUnsupported /* VendorId */,
}
impl Command for AddNOC {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "AddNOC",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NOCValue", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ICACValue", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "IPKValue", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "CaseAdminSubject", attr_ty: <TypeUnsupported /* SubjectId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "AdminVendorId", attr_ty: <TypeUnsupported /* VendorId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddNOC {
  type Response = NOCResponse;
}
impl TlvEncode for AddNOC {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.noc_value.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.icac_value.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.ipk_value.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.case_admin_subject.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.admin_vendor_id.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateNOC {
  pub noc_value: TypeUnsupported /* Octstr */,
  pub icac_value: TypeUnsupported /* Octstr */,
}
impl Command for UpdateNOC {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "UpdateNOC",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NOCValue", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ICACValue", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for UpdateNOC {
  type Response = NOCResponse;
}
impl TlvEncode for UpdateNOC {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.noc_value.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.icac_value.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NOCResponse {
  pub status_code: ty::NodeOperationalCertStatusEnum,
  pub fabric_index: TypeUnsupported /* FabricIdx */,
  pub debug_text: String,
}
impl Command for NOCResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0008,
    name: "NOCResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "StatusCode", attr_ty: <ty::NodeOperationalCertStatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "FabricIndex", attr_ty: <TypeUnsupported /* FabricIdx */>::ATTR_TYPE, conformance: Conformance::TooComplicated },
      AttrDef { id: 0x0002, name: "DebugText", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandResponse for NOCResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateFabricLabel {
  pub label: String,
}
impl Command for UpdateFabricLabel {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0009,
    name: "UpdateFabricLabel",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Label", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for UpdateFabricLabel {
  type Response = NOCResponse;
}
impl TlvEncode for UpdateFabricLabel {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.label.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveFabric {
  pub fabric_index: TypeUnsupported /* FabricIdx */,
}
impl Command for RemoveFabric {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000a,
    name: "RemoveFabric",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "FabricIndex", attr_ty: <TypeUnsupported /* FabricIdx */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveFabric {
  type Response = NOCResponse;
}
impl TlvEncode for RemoveFabric {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.fabric_index.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddTrustedRootCertificate {
  pub root_ca_certificate: TypeUnsupported /* Octstr */,
}
impl Command for AddTrustedRootCertificate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000b,
    name: "AddTrustedRootCertificate",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "RootCACertificate", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddTrustedRootCertificate {
  type Response = BasicAccResponse;
}
impl TlvEncode for AddTrustedRootCertificate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.root_ca_certificate.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetVIDVerificationStatement {
  pub vendor_id: TypeUnsupported /* VendorId */,
  pub vid_verification_statement: TypeUnsupported /* Octstr */,
  pub vvsc: TypeUnsupported /* Octstr */,
}
impl Command for SetVIDVerificationStatement {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000c,
    name: "SetVIDVerificationStatement",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "VendorID", attr_ty: <TypeUnsupported /* VendorId */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0001, name: "VIDVerificationStatement", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "VVSC", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for SetVIDVerificationStatement {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetVIDVerificationStatement {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.vendor_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.vid_verification_statement.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.vvsc.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SignVIDVerificationRequest {
  pub fabric_index: TypeUnsupported /* FabricIdx */,
  pub client_challenge: TypeUnsupported /* Octstr */,
}
impl Command for SignVIDVerificationRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000d,
    name: "SignVIDVerificationRequest",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "FabricIndex", attr_ty: <TypeUnsupported /* FabricIdx */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ClientChallenge", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SignVIDVerificationRequest {
  type Response = SignVIDVerificationResponse;
}
impl TlvEncode for SignVIDVerificationRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.fabric_index.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.client_challenge.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SignVIDVerificationResponse {
  pub fabric_index: TypeUnsupported /* FabricIdx */,
  pub fabric_binding_version: u8,
  pub signature: TypeUnsupported /* Octstr */,
}
impl Command for SignVIDVerificationResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000e,
    name: "SignVIDVerificationResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "FabricIndex", attr_ty: <TypeUnsupported /* FabricIdx */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "FabricBindingVersion", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Signature", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for SignVIDVerificationResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum CertificateChainTypeEnum {
  ///Request the DER-encoded DAC certificate
  DACCertificate = 0x0001,
  ///Request the DER-encoded PAI certificate
  PAICertificate = 0x0002,
}
impl AttrType for CertificateChainTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0001, "DACCertificate"),
  (0x0002, "PAICertificate"),
  ]);
}
impl TlvPut for CertificateChainTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for CertificateChainTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0001 => Ok(Self::DACCertificate),
      0x0002 => Ok(Self::PAICertificate),
      _ => Err(())
    }
  }
}
impl TlvDecode for CertificateChainTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum NodeOperationalCertStatusEnum {
  ///OK, no error
  OK = 0x0000,
  ///Public Key in the NOC does not match the public key in the NOCSR
  InvalidPublicKey = 0x0001,
  ///The Node Operational ID in the NOC is not formatted correctly.
  InvalidNodeOpId = 0x0002,
  ///Any other validation error in NOC chain
  InvalidNOC = 0x0003,
  ///No record of prior CSR for which this NOC could match
  MissingCsr = 0x0004,
  ///NOCs table full, cannot add another one
  TableFull = 0x0005,
  ///Invalid CaseAdminSubject field for an AddNOC command.
  InvalidAdminSubject = 0x0006,
  ///Trying to AddNOC instead of UpdateNOC against an existing Fabric.
  FabricConflict = 0x0009,
  ///Label already exists on another Fabric.
  LabelConflict = 0x000a,
  ///FabricIndex argument is invalid.
  InvalidFabricIndex = 0x000b,
}
impl AttrType for NodeOperationalCertStatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "OK"),
  (0x0001, "InvalidPublicKey"),
  (0x0002, "InvalidNodeOpId"),
  (0x0003, "InvalidNOC"),
  (0x0004, "MissingCsr"),
  (0x0005, "TableFull"),
  (0x0006, "InvalidAdminSubject"),
  (0x0009, "FabricConflict"),
  (0x000a, "LabelConflict"),
  (0x000b, "InvalidFabricIndex"),
  ]);
}
impl TlvPut for NodeOperationalCertStatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for NodeOperationalCertStatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::OK),
      0x0001 => Ok(Self::InvalidPublicKey),
      0x0002 => Ok(Self::InvalidNodeOpId),
      0x0003 => Ok(Self::InvalidNOC),
      0x0004 => Ok(Self::MissingCsr),
      0x0005 => Ok(Self::TableFull),
      0x0006 => Ok(Self::InvalidAdminSubject),
      0x0009 => Ok(Self::FabricConflict),
      0x000a => Ok(Self::LabelConflict),
      0x000b => Ok(Self::InvalidFabricIndex),
      _ => Err(())
    }
  }
}
impl TlvDecode for NodeOperationalCertStatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct FabricDescriptorStruct {
  pub root_public_key: TypeUnsupported /* Octstr */,
  pub vendor_id: TypeUnsupported /* VendorId */,
  pub fabric_id: TypeUnsupported /* FabricId */,
  pub node_id: TypeUnsupported /* NodeId */,
  pub label: String,
  pub vid_verification_statement: TypeUnsupported /* Octstr */,
}
impl AttrType for FabricDescriptorStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0001, name: "RootPublicKey", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "VendorID", attr_ty: <TypeUnsupported /* VendorId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "FabricID", attr_ty: <TypeUnsupported /* FabricId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "Label", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "VIDVerificationStatement", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for FabricDescriptorStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.root_public_key.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.vendor_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.fabric_id.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.label.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.vid_verification_statement.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for FabricDescriptorStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut label: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { label = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      root_public_key: TypeUnsupported,
      vendor_id: TypeUnsupported,
      fabric_id: TypeUnsupported,
      node_id: TypeUnsupported,
      label: label.ok_or(TlvDecodeErr::MissingField("Label"))?,
      vid_verification_statement: TypeUnsupported,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NOCStruct {
  pub noc: TypeUnsupported /* Octstr */,
  pub icac: TypeUnsupported /* Octstr */,
  pub vvsc: TypeUnsupported /* Octstr */,
}
impl AttrType for NOCStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0001, name: "NOC", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "ICAC", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "VVSC", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for NOCStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.noc.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.icac.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.vvsc.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for NOCStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    
    loop {
      match reader.next_ref()? {
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      noc: TypeUnsupported,
      icac: TypeUnsupported,
      vvsc: TypeUnsupported,
    })
  }
}

}
