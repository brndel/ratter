#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const WATER_HEATER_MANAGEMENT: ClusterDef = ClusterDef {
  id: 0x0094,
  name: "Water Heater Management",
  features: &[
    FeatureDef { bit: 0x0000, code: "EM", name: "EnergyManagement", summary: "Allows energy management control of the tank" },
    FeatureDef { bit: 0x0001, code: "TP", name: "TankPercent", summary: "Supports monitoring the percentage of hot water in the tank" },
  ],
  attributes: &[
    attr::HeaterTypes::ATTR,
    attr::HeatDemand::ATTR,
    attr::TankVolume::ATTR,
    attr::EstimatedHeatRequired::ATTR,
    attr::TankPercentage::ATTR,
    attr::BoostState::ATTR,
  ],
  events: &[
    event::BoostStarted::EVENT,
    event::BoostEnded::EVENT,
  ],
  commands: &[
    command::Boost::COMMAND,
    command::CancelBoost::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct HeaterTypes;

  impl Attr for HeaterTypes {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "HeaterTypes",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::WaterHeaterHeatSourceBitmap;
  }


  pub struct HeatDemand;

  impl Attr for HeatDemand {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "HeatDemand",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::WaterHeaterHeatSourceBitmap;
  }


  pub struct TankVolume;

  impl Attr for TankVolume {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "TankVolume",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u16;
  }


  pub struct EstimatedHeatRequired;

  impl Attr for EstimatedHeatRequired {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "EstimatedHeatRequired",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = EnergyMwh;
  }


  pub struct TankPercentage;

  impl Attr for TankPercentage {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "TankPercentage",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = TypeUnsupported /* Percent */;
  }


  pub struct BoostState;

  impl Attr for BoostState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "BoostState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::BoostStateEnum;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct BoostStarted {
  pub boost_info: ty::WaterHeaterBoostInfoStruct,
}

impl Event for BoostStarted {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "BoostStarted",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "BoostInfo", attr_ty: <ty::WaterHeaterBoostInfoStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for BoostStarted {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut boost_info: Option<ty::WaterHeaterBoostInfoStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { boost_info = Some(<ty::WaterHeaterBoostInfoStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      boost_info: boost_info.ok_or(TlvDecodeErr::MissingField("BoostInfo"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct BoostEnded {
}

impl Event for BoostEnded {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "BoostEnded",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}

impl TlvDecode for BoostEnded {
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
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Boost {
  pub boost_info: ty::WaterHeaterBoostInfoStruct,
}
impl Command for Boost {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "Boost",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "BoostInfo", attr_ty: <ty::WaterHeaterBoostInfoStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for Boost {
  type Response = BasicAccResponse;
}
impl TlvEncode for Boost {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.boost_info.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CancelBoost {
}
impl Command for CancelBoost {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "CancelBoost",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for CancelBoost {
  type Response = BasicAccResponse;
}
impl TlvEncode for CancelBoost {
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
pub enum BoostStateEnum {
  ///Boost is not currently active
  Inactive = 0x0000,
  ///Boost is currently active
  Active = 0x0001,
}
impl AttrType for BoostStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Inactive"),
  (0x0001, "Active"),
  ]);
}
impl TlvPut for BoostStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for BoostStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Inactive),
      0x0001 => Ok(Self::Active),
      _ => Err(())
    }
  }
}
impl TlvDecode for BoostStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct WaterHeaterHeatSourceBitmap: u32 {
    ///Immersion Heating Element 1
    const ImmersionElement1 = 1 << 0;
    ///Immersion Heating Element 2
    const ImmersionElement2 = 1 << 1;
    ///Heat pump Heating
    const HeatPump = 1 << 2;
    ///Boiler Heating (e.g. Gas or Oil)
    const Boiler = 1 << 3;
    ///Other Heating
    const Other = 1 << 4;
  }
}
impl AttrType for WaterHeaterHeatSourceBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "ImmersionElement1"),
    (1, "ImmersionElement2"),
    (2, "HeatPump"),
    (3, "Boiler"),
    (4, "Other"),
  ]);
}
impl TlvPut for WaterHeaterHeatSourceBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for WaterHeaterHeatSourceBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for WaterHeaterHeatSourceBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct WaterHeaterBoostInfoStruct {
  pub duration: TypeUnsupported /* ElapsedS */,
  pub one_shot: bool,
  pub emergency_boost: bool,
  pub temporary_setpoint: Temperature,
  pub target_percentage: TypeUnsupported /* Percent */,
  pub target_reheat: TypeUnsupported /* Percent */,
}
impl AttrType for WaterHeaterBoostInfoStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Duration", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "OneShot", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "EmergencyBoost", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "TemporarySetpoint", attr_ty: <Temperature>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "TargetPercentage", attr_ty: <TypeUnsupported /* Percent */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "TargetReheat", attr_ty: <TypeUnsupported /* Percent */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for WaterHeaterBoostInfoStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.duration.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.one_shot.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.emergency_boost.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.temporary_setpoint.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.target_percentage.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.target_reheat.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for WaterHeaterBoostInfoStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut one_shot: Option<bool> = None;
    let mut emergency_boost: Option<bool> = None;
    let mut temporary_setpoint: Option<Temperature> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { one_shot = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { emergency_boost = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { temporary_setpoint = Some(<Temperature>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      duration: TypeUnsupported,
      one_shot: one_shot.ok_or(TlvDecodeErr::MissingField("OneShot"))?,
      emergency_boost: emergency_boost.ok_or(TlvDecodeErr::MissingField("EmergencyBoost"))?,
      temporary_setpoint: temporary_setpoint.ok_or(TlvDecodeErr::MissingField("TemporarySetpoint"))?,
      target_percentage: TypeUnsupported,
      target_reheat: TypeUnsupported,
    })
  }
}

}
