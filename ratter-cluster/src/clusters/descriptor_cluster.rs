#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const DESCRIPTOR: ClusterDef = ClusterDef {
  id: 0x001d,
  name: "Descriptor",
  features: &[
    FeatureDef { bit: 0x0000, code: "TAGLIST", name: "TagList", summary: "The TagList attribute is present" },
  ],
  attributes: &[
    attr::DeviceTypeList::ATTR,
    attr::ServerList::ATTR,
    attr::ClientList::ATTR,
    attr::PartsList::ATTR,
    attr::TagList::ATTR,
    attr::EndpointUniqueID::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct DeviceTypeList;

  impl Attr for DeviceTypeList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "DeviceTypeList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::DeviceTypeStruct>;
  }


  pub struct ServerList;

  impl Attr for ServerList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "ServerList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<TypeUnsupported /* ClusterId */>;
  }


  pub struct ClientList;

  impl Attr for ClientList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "ClientList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<TypeUnsupported /* ClusterId */>;
  }


  pub struct PartsList;

  impl Attr for PartsList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "PartsList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<TypeUnsupported /* EndpointNo */>;
  }


  pub struct TagList;

  impl Attr for TagList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "TagList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<ty::SemanticTagStruct>;
  }


  pub struct EndpointUniqueID;

  impl Attr for EndpointUniqueID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "EndpointUniqueID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = String;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceTypeStruct {
  pub device_type: TypeUnsupported /* DevtypeId */,
  pub revision: u16,
}
impl AttrType for DeviceTypeStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "DeviceType", attr_ty: <TypeUnsupported /* DevtypeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Revision", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DeviceTypeStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.device_type.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.revision.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DeviceTypeStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut revision: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { revision = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      device_type: TypeUnsupported,
      revision: revision.ok_or(TlvDecodeErr::MissingField("Revision"))?,
    })
  }
}

}
