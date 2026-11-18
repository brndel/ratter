use crate::cluster_def::*;
pub const SCENES_MANAGEMENT: ClusterDef = ClusterDef {
  id: 0x0062,
  name: "Scenes Management",
  features: &[
    "SN",
  ],
  attributes: &[
    // attr::DoNotUse::ATTR, (type None is not yet supported)
    attr::SceneTableSize::ATTR,
    // attr::FabricSceneInfo::ATTR, (type Some(List) is not yet supported)
  ],
  events: &[],
};
pub mod attr {
  use super::*;
  // pub struct DoNotUse;
  // type None is not supported yet


  pub struct SceneTableSize;

  impl Attr for SceneTableSize {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "SceneTableSize",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u16;
  }


  // pub struct FabricSceneInfo;
  // type Some(List) is not supported yet


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  pub struct CopyModeBitmap: u32 {
    ///Copy all scenes in the scene table
    const CopyAllScenes = 1 << 0;
  }
}
impl AttrType for CopyModeBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "CopyAllScenes"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub struct AttributeValuePairStruct {
  // attributeid: AttributeId (type currently unsupported)
  valueunsigned8: u8,
  valuesigned8: i8,
  valueunsigned16: u16,
  valuesigned16: i16,
  valueunsigned32: u32,
  valuesigned32: i32,
  valueunsigned64: u64,
  valuesigned64: i64,
}
impl AttrType for AttributeValuePairStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
  // field AttributeID: AttributeId (type currently unsupported)
    AttrDef { id: 0x0001, name: "ValueUnsigned8", attr_ty: u8::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "ValueSigned8", attr_ty: i8::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "ValueUnsigned16", attr_ty: u16::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "ValueSigned16", attr_ty: i16::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "ValueUnsigned32", attr_ty: u32::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "ValueSigned32", attr_ty: i32::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "ValueUnsigned64", attr_ty: u64::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0008, name: "ValueSigned64", attr_ty: i64::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
#[derive(Debug, Clone, PartialEq)]
pub struct ExtensionFieldSetStruct {
  // clusterid: ClusterId (type currently unsupported)
  // attributevaluelist: List (type currently unsupported)
}
impl AttrType for ExtensionFieldSetStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
  // field ClusterID: ClusterId (type currently unsupported)
  // field AttributeValueList: List (type currently unsupported)
  ]});
}
#[derive(Debug, Clone, PartialEq)]
pub struct SceneInfoStruct {
  scenecount: u8,
  currentscene: u8,
  // currentgroup: GroupId (type currently unsupported)
  scenevalid: bool,
  remainingcapacity: u8,
}
impl AttrType for SceneInfoStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "SceneCount", attr_ty: u8::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "CurrentScene", attr_ty: u8::ATTR_TYPE, conformance: Conformance::Mandatory },
  // field CurrentGroup: GroupId (type currently unsupported)
    AttrDef { id: 0x0003, name: "SceneValid", attr_ty: bool::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "RemainingCapacity", attr_ty: u8::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
}
