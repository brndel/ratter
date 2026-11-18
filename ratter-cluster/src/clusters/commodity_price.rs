#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const COMMODITY_PRICE: ClusterDef = ClusterDef {
  id: 0x0095,
  name: "Commodity Price",
  features: &[
    FeatureDef { bit: 0x0000, code: "FORE", name: "Forecasting", summary: "Forecasts upcoming pricing" },
  ],
  attributes: &[
    attr::TariffUnit::ATTR,
    attr::Currency::ATTR,
    attr::CurrentPrice::ATTR,
    attr::PriceForecast::ATTR,
  ],
  events: &[
    event::PriceChange::EVENT,
  ],
  commands: &[
    command::GetDetailedPriceRequest::COMMAND,
    command::GetDetailedForecastRequest::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct TariffUnit;

  impl Attr for TariffUnit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "TariffUnit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::TariffUnitEnum;
  }


  pub struct Currency;

  impl Attr for Currency {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "Currency",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::CurrencyStruct>;
  }


  pub struct CurrentPrice;

  impl Attr for CurrentPrice {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "CurrentPrice",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::CommodityPriceStruct>;
  }


  pub struct PriceForecast;

  impl Attr for PriceForecast {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "PriceForecast",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<ty::CommodityPriceStruct>;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct PriceChange {
  pub current_price: ty::CommodityPriceStruct,
}

impl Event for PriceChange {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "PriceChange",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "CurrentPrice", attr_ty: <ty::CommodityPriceStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for PriceChange {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut current_price: Option<ty::CommodityPriceStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { current_price = Some(<ty::CommodityPriceStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      current_price: current_price.ok_or(TlvDecodeErr::MissingField("CurrentPrice"))?,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct GetDetailedPriceRequest {
  pub details: ty::CommodityPriceDetailBitmap,
}
impl Command for GetDetailedPriceRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "GetDetailedPriceRequest",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "Details", attr_ty: <ty::CommodityPriceDetailBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for GetDetailedPriceRequest {
  type Response = GetDetailedPriceResponse;
}
impl TlvEncode for GetDetailedPriceRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.details.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetDetailedPriceResponse {
  pub current_price: ty::CommodityPriceStruct,
}
impl Command for GetDetailedPriceResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "GetDetailedPriceResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CurrentPrice", attr_ty: <ty::CommodityPriceStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for GetDetailedPriceResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct GetDetailedForecastRequest {
  pub details: ty::CommodityPriceDetailBitmap,
}
impl Command for GetDetailedForecastRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "GetDetailedForecastRequest",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "Details", attr_ty: <ty::CommodityPriceDetailBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for GetDetailedForecastRequest {
  type Response = GetDetailedForecastResponse;
}
impl TlvEncode for GetDetailedForecastRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.details.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetDetailedForecastResponse {
  pub price_forecast: Vec<ty::CommodityPriceStruct>,
}
impl Command for GetDetailedForecastResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "GetDetailedForecastResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "PriceForecast", attr_ty: <Vec<ty::CommodityPriceStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for GetDetailedForecastResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct CommodityPriceDetailBitmap: u32 {
    ///A textual description of a price; e.g. the name of a rate plan.
    const Description = 1 << 0;
    ///A breakdown of the component parts of a price; e.g. generation, delivery, etc.
    const Components = 1 << 1;
  }
}
impl AttrType for CommodityPriceDetailBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Description"),
    (1, "Components"),
  ]);
}
impl TlvPut for CommodityPriceDetailBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for CommodityPriceDetailBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for CommodityPriceDetailBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CommodityPriceComponentStruct {
  pub price: TypeUnsupported /* Money */,
  pub source: TariffPriceTypeEnum,
  pub description: String,
  pub tariff_component_id: u32,
}
impl AttrType for CommodityPriceComponentStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Price", attr_ty: <TypeUnsupported /* Money */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Source", attr_ty: <TariffPriceTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Description", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "TariffComponentID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for CommodityPriceComponentStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.price.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.source.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.description.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.tariff_component_id.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for CommodityPriceComponentStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut source: Option<TariffPriceTypeEnum> = None;
    let mut description: Option<String> = None;
    let mut tariff_component_id: Option<u32> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { source = Some(<TariffPriceTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { description = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { tariff_component_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      price: TypeUnsupported,
      source: source.ok_or(TlvDecodeErr::MissingField("Source"))?,
      description: description.ok_or(TlvDecodeErr::MissingField("Description"))?,
      tariff_component_id: tariff_component_id.ok_or(TlvDecodeErr::MissingField("TariffComponentID"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommodityPriceStruct {
  pub period_start: TypeUnsupported /* EpochS */,
  pub period_end: TypeUnsupported /* EpochS */,
  pub price: TypeUnsupported /* Money */,
  pub price_level: i16,
  pub description: String,
  pub components: Vec<CommodityPriceComponentStruct>,
}
impl AttrType for CommodityPriceStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "PeriodStart", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "PeriodEnd", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Price", attr_ty: <TypeUnsupported /* Money */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "PriceLevel", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "Description", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "Components", attr_ty: <Vec<CommodityPriceComponentStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for CommodityPriceStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.period_start.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.period_end.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.price.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.price_level.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.description.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.components.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for CommodityPriceStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut price_level: Option<i16> = None;
    let mut description: Option<String> = None;
    let mut components: Option<Vec<CommodityPriceComponentStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { price_level = Some(<i16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { description = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0005), kind: _ }) => { components = Some(<Vec<CommodityPriceComponentStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      period_start: TypeUnsupported,
      period_end: TypeUnsupported,
      price: TypeUnsupported,
      price_level: price_level.ok_or(TlvDecodeErr::MissingField("PriceLevel"))?,
      description: description.ok_or(TlvDecodeErr::MissingField("Description"))?,
      components: components.ok_or(TlvDecodeErr::MissingField("Components"))?,
    })
  }
}

}
