#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const HEPA_FILTER_MONITORING: ClusterDef = ClusterDef {
  id: 0x0071,
  name: "HEPA Filter Monitoring",
  features: &[
    FeatureDef { bit: 0x0000, code: "CON", name: "Condition", summary: "Supports monitoring the condition of the resource in percentage" },
    FeatureDef { bit: 0x0001, code: "WRN", name: "Warning", summary: "Supports warning indication" },
    FeatureDef { bit: 0x0002, code: "REP", name: "ReplacementProductList", summary: "Supports specifying the list of replacement products" },
  ],
  attributes: &[
    attr::Condition::ATTR,
    attr::DegradationDirection::ATTR,
    attr::ChangeIndication::ATTR,
    attr::InPlaceIndicator::ATTR,
    attr::LastChangedTime::ATTR,
    attr::ReplacementProductList::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::ResetCondition::COMMAND,
  ],
};
pub const ACTIVATED_CARBON_FILTER_MONITORING: ClusterDef = ClusterDef {
  id: 0x0072,
  name: "Activated Carbon Filter Monitoring",
  features: &[
    FeatureDef { bit: 0x0000, code: "CON", name: "Condition", summary: "Supports monitoring the condition of the resource in percentage" },
    FeatureDef { bit: 0x0001, code: "WRN", name: "Warning", summary: "Supports warning indication" },
    FeatureDef { bit: 0x0002, code: "REP", name: "ReplacementProductList", summary: "Supports specifying the list of replacement products" },
  ],
  attributes: &[
    attr::Condition::ATTR,
    attr::DegradationDirection::ATTR,
    attr::ChangeIndication::ATTR,
    attr::InPlaceIndicator::ATTR,
    attr::LastChangedTime::ATTR,
    attr::ReplacementProductList::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::ResetCondition::COMMAND,
  ],
};
pub const WATER_TANK_LEVEL_MONITORING: ClusterDef = ClusterDef {
  id: 0x0079,
  name: "Water Tank Level Monitoring",
  features: &[
    FeatureDef { bit: 0x0000, code: "CON", name: "Condition", summary: "Supports monitoring the condition of the resource in percentage" },
    FeatureDef { bit: 0x0001, code: "WRN", name: "Warning", summary: "Supports warning indication" },
    FeatureDef { bit: 0x0002, code: "REP", name: "ReplacementProductList", summary: "Supports specifying the list of replacement products" },
  ],
  attributes: &[
    attr::Condition::ATTR,
    attr::DegradationDirection::ATTR,
    attr::ChangeIndication::ATTR,
    attr::InPlaceIndicator::ATTR,
    attr::LastChangedTime::ATTR,
    attr::ReplacementProductList::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::ResetCondition::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct Condition;

  impl Attr for Condition {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "Condition",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = TypeUnsupported /* Percent */;
  }


  pub struct DegradationDirection;

  impl Attr for DegradationDirection {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "DegradationDirection",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = ty::DegradationDirectionEnum;
  }


  pub struct ChangeIndication;

  impl Attr for ChangeIndication {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "ChangeIndication",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::ChangeIndicationEnum;
  }


  pub struct InPlaceIndicator;

  impl Attr for InPlaceIndicator {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "InPlaceIndicator",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct LastChangedTime;

  impl Attr for LastChangedTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "LastChangedTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<TypeUnsupported /* EpochS */>;
  }


  pub struct ReplacementProductList;

  impl Attr for ReplacementProductList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "ReplacementProductList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = Vec<ty::ReplacementProductStruct>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ResetCondition {
}
impl Command for ResetCondition {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "ResetCondition",
    conformance: Conformance::Optional,
    fields: &[
    ],
  };
}
impl CommandInvoke for ResetCondition {
  type Response = BasicAccResponse;
}
impl TlvEncode for ResetCondition {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
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
pub enum ChangeIndicationEnum {
  ///Resource is in good condition, no intervention required
  OK = 0x0000,
  ///Resource will be exhausted soon, intervention will shortly be required
  Warning = 0x0001,
  ///Resource is exhausted, immediate intervention is required
  Critical = 0x0002,
}
impl AttrType for ChangeIndicationEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "OK"),
  (0x0001, "Warning"),
  (0x0002, "Critical"),
  ]);
}
impl TlvPut for ChangeIndicationEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ChangeIndicationEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::OK),
      0x0001 => Ok(Self::Warning),
      0x0002 => Ok(Self::Critical),
      _ => Err(())
    }
  }
}
impl TlvDecode for ChangeIndicationEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum DegradationDirectionEnum {
  ///The degradation of the resource is indicated by an upwards moving/increasing value
  Up = 0x0000,
  ///The degradation of the resource is indicated by a downwards moving/decreasing value
  Down = 0x0001,
}
impl AttrType for DegradationDirectionEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Up"),
  (0x0001, "Down"),
  ]);
}
impl TlvPut for DegradationDirectionEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for DegradationDirectionEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Up),
      0x0001 => Ok(Self::Down),
      _ => Err(())
    }
  }
}
impl TlvDecode for DegradationDirectionEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ProductIdentifierTypeEnum {
  ///12-digit Universal Product Code
  UPC = 0x0000,
  ///8-digit Global Trade Item Number
  GTIN_8 = 0x0001,
  ///13-digit European Article Number
  EAN = 0x0002,
  ///14-digit Global Trade Item Number
  GTIN_14 = 0x0003,
  ///Original Equipment Manufacturer part number
  OEM = 0x0004,
}
impl AttrType for ProductIdentifierTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "UPC"),
  (0x0001, "GTIN_8"),
  (0x0002, "EAN"),
  (0x0003, "GTIN_14"),
  (0x0004, "OEM"),
  ]);
}
impl TlvPut for ProductIdentifierTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ProductIdentifierTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::UPC),
      0x0001 => Ok(Self::GTIN_8),
      0x0002 => Ok(Self::EAN),
      0x0003 => Ok(Self::GTIN_14),
      0x0004 => Ok(Self::OEM),
      _ => Err(())
    }
  }
}
impl TlvDecode for ProductIdentifierTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ReplacementProductStruct {
  pub product_identifier_type: ProductIdentifierTypeEnum,
  pub product_identifier_value: String,
}
impl AttrType for ReplacementProductStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ProductIdentifierType", attr_ty: <ProductIdentifierTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ProductIdentifierValue", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ReplacementProductStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.product_identifier_type.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.product_identifier_value.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ReplacementProductStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut product_identifier_type: Option<ProductIdentifierTypeEnum> = None;
    let mut product_identifier_value: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { product_identifier_type = Some(<ProductIdentifierTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { product_identifier_value = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      product_identifier_type: product_identifier_type.ok_or(TlvDecodeErr::MissingField("ProductIdentifierType"))?,
      product_identifier_value: product_identifier_value.ok_or(TlvDecodeErr::MissingField("ProductIdentifierValue"))?,
    })
  }
}

}
