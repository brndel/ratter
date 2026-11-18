#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const THREAD_NETWORK_DIRECTORY: ClusterDef = ClusterDef {
  id: 0x0453,
  name: "Thread Network Directory",
  features: &[
  ],
  attributes: &[
    attr::PreferredExtendedPanID::ATTR,
    attr::ThreadNetworks::ATTR,
    attr::ThreadNetworkTableSize::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::AddNetwork::COMMAND,
    command::RemoveNetwork::COMMAND,
    command::GetOperationalDataset::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct PreferredExtendedPanID;

  impl Attr for PreferredExtendedPanID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "PreferredExtendedPanID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* Octstr */>;
  }


  pub struct ThreadNetworks;

  impl Attr for ThreadNetworks {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "ThreadNetworks",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::ThreadNetworkStruct>;
  }


  pub struct ThreadNetworkTableSize;

  impl Attr for ThreadNetworkTableSize {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "ThreadNetworkTableSize",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct AddNetwork {
  pub operational_dataset: TypeUnsupported /* Octstr */,
}
impl Command for AddNetwork {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "AddNetwork",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "OperationalDataset", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddNetwork {
  type Response = BasicAccResponse;
}
impl TlvEncode for AddNetwork {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.operational_dataset.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveNetwork {
  pub extended_pan_id: TypeUnsupported /* Octstr */,
}
impl Command for RemoveNetwork {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "RemoveNetwork",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ExtendedPanID", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveNetwork {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveNetwork {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.extended_pan_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetOperationalDataset {
  pub extended_pan_id: TypeUnsupported /* Octstr */,
}
impl Command for GetOperationalDataset {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "GetOperationalDataset",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ExtendedPanID", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for GetOperationalDataset {
  type Response = OperationalDatasetResponse;
}
impl TlvEncode for GetOperationalDataset {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.extended_pan_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperationalDatasetResponse {
  pub operational_dataset: TypeUnsupported /* Octstr */,
}
impl Command for OperationalDatasetResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "OperationalDatasetResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "OperationalDataset", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for OperationalDatasetResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ThreadNetworkStruct {
  pub extended_pan_id: TypeUnsupported /* Octstr */,
  pub network_name: String,
  pub channel: u16,
  pub active_timestamp: u64,
}
impl AttrType for ThreadNetworkStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ExtendedPanID", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "NetworkName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Channel", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "ActiveTimestamp", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ThreadNetworkStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.extended_pan_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.network_name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.channel.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.active_timestamp.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ThreadNetworkStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut network_name: Option<String> = None;
    let mut channel: Option<u16> = None;
    let mut active_timestamp: Option<u64> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { network_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { channel = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { active_timestamp = Some(<u64>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      extended_pan_id: TypeUnsupported,
      network_name: network_name.ok_or(TlvDecodeErr::MissingField("NetworkName"))?,
      channel: channel.ok_or(TlvDecodeErr::MissingField("Channel"))?,
      active_timestamp: active_timestamp.ok_or(TlvDecodeErr::MissingField("ActiveTimestamp"))?,
    })
  }
}

}
