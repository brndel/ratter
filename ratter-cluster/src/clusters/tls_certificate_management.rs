#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const TLS_CERTIFICATE_MANAGEMENT: ClusterDef = ClusterDef {
  id: 0x0801,
  name: "TLS Certificate Management",
  features: &[
  ],
  attributes: &[
    attr::MaxRootCertificates::ATTR,
    attr::ProvisionedRootCertificates::ATTR,
    attr::MaxClientCertificates::ATTR,
    attr::ProvisionedClientCertificates::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::ProvisionRootCertificate::COMMAND,
    command::FindRootCertificate::COMMAND,
    command::LookupRootCertificate::COMMAND,
    command::RemoveRootCertificate::COMMAND,
    command::ClientCSR::COMMAND,
    command::ProvisionClientCertificate::COMMAND,
    command::FindClientCertificate::COMMAND,
    command::LookupClientCertificate::COMMAND,
    command::RemoveClientCertificate::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct MaxRootCertificates;

  impl Attr for MaxRootCertificates {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "MaxRootCertificates",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct ProvisionedRootCertificates;

  impl Attr for ProvisionedRootCertificates {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "ProvisionedRootCertificates",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::TLSCertStruct>;
  }


  pub struct MaxClientCertificates;

  impl Attr for MaxClientCertificates {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "MaxClientCertificates",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct ProvisionedClientCertificates;

  impl Attr for ProvisionedClientCertificates {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "ProvisionedClientCertificates",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::TLSClientCertificateDetailStruct>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ProvisionRootCertificate {
  pub certificate: TypeUnsupported /* Octstr */,
  pub caid: u16,
}
impl Command for ProvisionRootCertificate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "ProvisionRootCertificate",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Certificate", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "CAID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ProvisionRootCertificate {
  type Response = ProvisionRootCertificateResponse;
}
impl TlvEncode for ProvisionRootCertificate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.certificate.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.caid.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProvisionRootCertificateResponse {
  pub caid: u16,
}
impl Command for ProvisionRootCertificateResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "ProvisionRootCertificateResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CAID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for ProvisionRootCertificateResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct FindRootCertificate {
  pub caid: u16,
}
impl Command for FindRootCertificate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "FindRootCertificate",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CAID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for FindRootCertificate {
  type Response = FindRootCertificateResponse;
}
impl TlvEncode for FindRootCertificate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.caid.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FindRootCertificateResponse {
  pub certificate_details: Vec<ty::TLSCertStruct>,
}
impl Command for FindRootCertificateResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "FindRootCertificateResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CertificateDetails", attr_ty: <Vec<ty::TLSCertStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for FindRootCertificateResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct LookupRootCertificate {
  pub fingerprint: TypeUnsupported /* Octstr */,
}
impl Command for LookupRootCertificate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "LookupRootCertificate",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Fingerprint", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for LookupRootCertificate {
  type Response = LookupRootCertificateResponse;
}
impl TlvEncode for LookupRootCertificate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.fingerprint.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LookupRootCertificateResponse {
  pub caid: u16,
}
impl Command for LookupRootCertificateResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "LookupRootCertificateResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CAID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for LookupRootCertificateResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveRootCertificate {
  pub caid: u16,
}
impl Command for RemoveRootCertificate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "RemoveRootCertificate",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CAID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveRootCertificate {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveRootCertificate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.caid.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClientCSR {
  pub nonce: TypeUnsupported /* Octstr */,
  pub ccdid: u16,
}
impl Command for ClientCSR {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "ClientCSR",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Nonce", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "CCDID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ClientCSR {
  type Response = ClientCSRResponse;
}
impl TlvEncode for ClientCSR {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.nonce.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.ccdid.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClientCSRResponse {
  pub ccdid: u16,
  pub csr: TypeUnsupported /* Octstr */,
  pub nonce_signature: TypeUnsupported /* Octstr */,
}
impl Command for ClientCSRResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0008,
    name: "ClientCSRResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CCDID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "CSR", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "NonceSignature", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for ClientCSRResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct ProvisionClientCertificate {
  pub ccdid: u16,
  pub client_certificate: TypeUnsupported /* Octstr */,
  pub intermediate_certificates: Vec<TypeUnsupported /* Octstr */>,
}
impl Command for ProvisionClientCertificate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0009,
    name: "ProvisionClientCertificate",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CCDID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ClientCertificate", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "IntermediateCertificates", attr_ty: <Vec<TypeUnsupported /* Octstr */>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ProvisionClientCertificate {
  type Response = BasicAccResponse;
}
impl TlvEncode for ProvisionClientCertificate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.ccdid.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.client_certificate.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.intermediate_certificates.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FindClientCertificate {
  pub ccdid: u16,
}
impl Command for FindClientCertificate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000a,
    name: "FindClientCertificate",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CCDID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for FindClientCertificate {
  type Response = FindClientCertificateResponse;
}
impl TlvEncode for FindClientCertificate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.ccdid.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FindClientCertificateResponse {
  pub certificate_details: Vec<ty::TLSClientCertificateDetailStruct>,
}
impl Command for FindClientCertificateResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000b,
    name: "FindClientCertificateResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CertificateDetails", attr_ty: <Vec<ty::TLSClientCertificateDetailStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for FindClientCertificateResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct LookupClientCertificate {
  pub fingerprint: TypeUnsupported /* Octstr */,
}
impl Command for LookupClientCertificate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000c,
    name: "LookupClientCertificate",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Fingerprint", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for LookupClientCertificate {
  type Response = LookupClientCertificateResponse;
}
impl TlvEncode for LookupClientCertificate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.fingerprint.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LookupClientCertificateResponse {
  pub ccdid: u16,
}
impl Command for LookupClientCertificateResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000d,
    name: "LookupClientCertificateResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CCDID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for LookupClientCertificateResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveClientCertificate {
  pub ccdid: u16,
}
impl Command for RemoveClientCertificate {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000e,
    name: "RemoveClientCertificate",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CCDID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveClientCertificate {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveClientCertificate {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.ccdid.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct TLSCertStruct {
  pub caid: u16,
  pub certificate: TypeUnsupported /* Octstr */,
}
impl AttrType for TLSCertStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "CAID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Certificate", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TLSCertStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.caid.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.certificate.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TLSCertStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut caid: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { caid = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      caid: caid.ok_or(TlvDecodeErr::MissingField("CAID"))?,
      certificate: TypeUnsupported,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TLSClientCertificateDetailStruct {
  pub ccdid: u16,
  pub client_certificate: TypeUnsupported /* Octstr */,
  pub intermediate_certificates: Vec<TypeUnsupported /* Octstr */>,
}
impl AttrType for TLSClientCertificateDetailStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "CCDID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ClientCertificate", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "IntermediateCertificates", attr_ty: <Vec<TypeUnsupported /* Octstr */>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TLSClientCertificateDetailStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.ccdid.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.client_certificate.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.intermediate_certificates.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TLSClientCertificateDetailStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut ccdid: Option<u16> = None;
    let mut intermediate_certificates: Option<Vec<TypeUnsupported /* Octstr */>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { ccdid = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { intermediate_certificates = Some(<Vec<TypeUnsupported /* Octstr */>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      ccdid: ccdid.ok_or(TlvDecodeErr::MissingField("CCDID"))?,
      client_certificate: TypeUnsupported,
      intermediate_certificates: intermediate_certificates.ok_or(TlvDecodeErr::MissingField("IntermediateCertificates"))?,
    })
  }
}

}
