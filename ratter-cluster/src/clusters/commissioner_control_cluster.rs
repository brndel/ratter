#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const COMMISSIONER_CONTROL: ClusterDef = ClusterDef {
  id: 0x0751,
  name: "Commissioner Control",
  features: &[
  ],
  attributes: &[
    attr::SupportedDeviceCategories::ATTR,
  ],
  events: &[
    event::CommissioningRequestResult::EVENT,
  ],
  commands: &[
    command::RequestCommissioningApproval::COMMAND,
    command::CommissionNode::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct SupportedDeviceCategories;

  impl Attr for SupportedDeviceCategories {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "SupportedDeviceCategories",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::SupportedDeviceCategoryBitmap;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct CommissioningRequestResult {
  pub request_id: u64,
  pub client_node_id: TypeUnsupported /* NodeId */,
  pub status_code: TypeUnsupported /* Status */,
}

impl Event for CommissioningRequestResult {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "CommissioningRequestResult",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "RequestID", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ClientNodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "StatusCode", attr_ty: <TypeUnsupported /* Status */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for CommissioningRequestResult {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut request_id: Option<u64> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { request_id = Some(<u64>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      request_id: request_id.ok_or(TlvDecodeErr::MissingField("RequestID"))?,
      client_node_id: TypeUnsupported,
      status_code: TypeUnsupported,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct RequestCommissioningApproval {
  pub request_id: u64,
  pub vendor_id: TypeUnsupported /* VendorId */,
  pub product_id: u16,
  pub label: String,
}
impl Command for RequestCommissioningApproval {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "RequestCommissioningApproval",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "RequestID", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "VendorID", attr_ty: <TypeUnsupported /* VendorId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "ProductID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "Label", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for RequestCommissioningApproval {
  type Response = BasicAccResponse;
}
impl TlvEncode for RequestCommissioningApproval {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.request_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.vendor_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.product_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.label.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommissionNode {
  pub request_id: u64,
  pub response_timeout_seconds: u16,
}
impl Command for CommissionNode {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "CommissionNode",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "RequestID", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ResponseTimeoutSeconds", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for CommissionNode {
  type Response = ReverseOpenCommissioningWindow;
}
impl TlvEncode for CommissionNode {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.request_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.response_timeout_seconds.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReverseOpenCommissioningWindow {
  pub commissioning_timeout: u16,
  pub pake_passcode_verifier: TypeUnsupported /* Octstr */,
  pub discriminator: u16,
  pub iterations: u32,
  pub salt: TypeUnsupported /* Octstr */,
}
impl Command for ReverseOpenCommissioningWindow {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "ReverseOpenCommissioningWindow",
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
impl CommandResponse for ReverseOpenCommissioningWindow {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct SupportedDeviceCategoryBitmap: u32 {
    ///Aggregators which support Fabric Synchronization may be commissioned.
    const FabricSynchronization = 1 << 0;
  }
}
impl AttrType for SupportedDeviceCategoryBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "FabricSynchronization"),
  ]);
}
impl TlvPut for SupportedDeviceCategoryBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for SupportedDeviceCategoryBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for SupportedDeviceCategoryBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
}
