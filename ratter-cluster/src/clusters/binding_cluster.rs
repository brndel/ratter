#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const BINDING: ClusterDef = ClusterDef {
  id: 0x001e,
  name: "Binding",
  features: &[
  ],
  attributes: &[
    attr::Binding::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct Binding;

  impl Attr for Binding {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "Binding",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::TargetStruct>;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct TargetStruct {
  pub node: TypeUnsupported /* NodeId */,
  pub group: TypeUnsupported /* GroupId */,
  pub endpoint: TypeUnsupported /* EndpointNo */,
  pub cluster: TypeUnsupported /* ClusterId */,
}
impl AttrType for TargetStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0001, name: "Node", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Group", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "Endpoint", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "Cluster", attr_ty: <TypeUnsupported /* ClusterId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TargetStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.node.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.group.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.endpoint.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.cluster.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TargetStruct {
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
      node: TypeUnsupported,
      group: TypeUnsupported,
      endpoint: TypeUnsupported,
      cluster: TypeUnsupported,
    })
  }
}

}
