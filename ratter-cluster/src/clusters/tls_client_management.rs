#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const TLS_CLIENT_MANAGEMENT: ClusterDef = ClusterDef {
  id: 0x0802,
  name: "TLS Client Management",
  features: &[
  ],
  attributes: &[
    attr::MaxProvisioned::ATTR,
    attr::ProvisionedEndpoints::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::ProvisionEndpoint::COMMAND,
    command::FindEndpoint::COMMAND,
    command::RemoveEndpoint::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct MaxProvisioned;

  impl Attr for MaxProvisioned {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "MaxProvisioned",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct ProvisionedEndpoints;

  impl Attr for ProvisionedEndpoints {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "ProvisionedEndpoints",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::TLSEndpointStruct>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ProvisionEndpoint {
  pub hostname: TypeUnsupported /* Octstr */,
  pub port: u16,
  pub caid: u16,
  pub ccdid: u16,
  pub endpoint_id: u16,
}
impl Command for ProvisionEndpoint {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "ProvisionEndpoint",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Hostname", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Port", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "CAID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "CCDID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "EndpointID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ProvisionEndpoint {
  type Response = ProvisionEndpointResponse;
}
impl TlvEncode for ProvisionEndpoint {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.hostname.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.port.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.caid.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.ccdid.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.endpoint_id.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProvisionEndpointResponse {
  pub endpoint_id: u16,
}
impl Command for ProvisionEndpointResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "ProvisionEndpointResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "EndpointID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for ProvisionEndpointResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct FindEndpoint {
  pub endpoint_id: u16,
}
impl Command for FindEndpoint {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "FindEndpoint",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "EndpointID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for FindEndpoint {
  type Response = FindEndpointResponse;
}
impl TlvEncode for FindEndpoint {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.endpoint_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FindEndpointResponse {
  pub endpoint: ty::TLSEndpointStruct,
}
impl Command for FindEndpointResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "FindEndpointResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Endpoint", attr_ty: <ty::TLSEndpointStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for FindEndpointResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveEndpoint {
  pub endpoint_id: u16,
}
impl Command for RemoveEndpoint {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "RemoveEndpoint",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "EndpointID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveEndpoint {
  type Response = ();
}
impl TlvEncode for RemoveEndpoint {
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
pub enum StatusCodeEnum {
  ///The endpoint is already installed.
  EndpointAlreadyInstalled = 0x0002,
  ///No root certificate exists for this CAID.
  RootCertificateNotFound = 0x0003,
  ///No client certificate exists for this CCDID.
  ClientCertificateNotFound = 0x0004,
  ///The endpoint is in use and cannot be removed.
  EndpointInUse = 0x0005,
  ///Time sync has not yet occurred.
  InvalidTime = 0x0006,
}
impl AttrType for StatusCodeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0002, "EndpointAlreadyInstalled"),
  (0x0003, "RootCertificateNotFound"),
  (0x0004, "ClientCertificateNotFound"),
  (0x0005, "EndpointInUse"),
  (0x0006, "InvalidTime"),
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
      0x0002 => Ok(Self::EndpointAlreadyInstalled),
      0x0003 => Ok(Self::RootCertificateNotFound),
      0x0004 => Ok(Self::ClientCertificateNotFound),
      0x0005 => Ok(Self::EndpointInUse),
      0x0006 => Ok(Self::InvalidTime),
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
pub struct TLSEndpointStruct {
  pub endpoint_id: u16,
  pub hostname: TypeUnsupported /* Octstr */,
  pub port: u16,
  pub caid: u16,
  pub ccdid: u16,
  pub reference_count: u8,
}
impl AttrType for TLSEndpointStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "EndpointID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Hostname", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Port", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "CAID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "CCDID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "ReferenceCount", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TLSEndpointStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.endpoint_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.hostname.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.port.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.caid.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.ccdid.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.reference_count.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TLSEndpointStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut endpoint_id: Option<u16> = None;
    let mut port: Option<u16> = None;
    let mut caid: Option<u16> = None;
    let mut ccdid: Option<u16> = None;
    let mut reference_count: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { endpoint_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { port = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { caid = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { ccdid = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { reference_count = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      endpoint_id: endpoint_id.ok_or(TlvDecodeErr::MissingField("EndpointID"))?,
      hostname: TypeUnsupported,
      port: port.ok_or(TlvDecodeErr::MissingField("Port"))?,
      caid: caid.ok_or(TlvDecodeErr::MissingField("CAID"))?,
      ccdid: ccdid.ok_or(TlvDecodeErr::MissingField("CCDID"))?,
      reference_count: reference_count.ok_or(TlvDecodeErr::MissingField("ReferenceCount"))?,
    })
  }
}

}
