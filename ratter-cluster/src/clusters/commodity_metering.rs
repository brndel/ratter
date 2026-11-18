#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const COMMODITY_METERING: ClusterDef = ClusterDef {
  id: 0x0b07,
  name: "Commodity Metering",
  features: &[
  ],
  attributes: &[
    attr::MeteredQuantity::ATTR,
    attr::MeteredQuantityTimestamp::ATTR,
    attr::TariffUnit::ATTR,
    attr::MaximumMeteredQuantities::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct MeteredQuantity;

  impl Attr for MeteredQuantity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "MeteredQuantity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<Vec<ty::MeteredQuantityStruct>>;
  }


  pub struct MeteredQuantityTimestamp;

  impl Attr for MeteredQuantityTimestamp {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "MeteredQuantityTimestamp",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* EpochS */>;
  }


  pub struct TariffUnit;

  impl Attr for TariffUnit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "TariffUnit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::TariffUnitEnum>;
  }


  pub struct MaximumMeteredQuantities;

  impl Attr for MaximumMeteredQuantities {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "MaximumMeteredQuantities",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u16>;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct MeteredQuantityStruct {
  pub tariff_component_i_ds: Vec<u32>,
  pub quantity: i64,
}
impl AttrType for MeteredQuantityStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "TariffComponentIDs", attr_ty: <Vec<u32>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Quantity", attr_ty: <i64>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for MeteredQuantityStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.tariff_component_i_ds.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.quantity.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for MeteredQuantityStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut tariff_component_i_ds: Option<Vec<u32>> = None;
    let mut quantity: Option<i64> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { tariff_component_i_ds = Some(<Vec<u32>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { quantity = Some(<i64>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      tariff_component_i_ds: tariff_component_i_ds.ok_or(TlvDecodeErr::MissingField("TariffComponentIDs"))?,
      quantity: quantity.ok_or(TlvDecodeErr::MissingField("Quantity"))?,
    })
  }
}

}
