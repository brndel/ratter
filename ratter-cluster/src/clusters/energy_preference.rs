#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const ENERGY_PREFERENCE: ClusterDef = ClusterDef {
  id: 0x009b,
  name: "Energy Preference",
  features: &[
    FeatureDef { bit: 0x0000, code: "BALA", name: "EnergyBalance", summary: "Device can balance energy consumption vs. another priority" },
    FeatureDef { bit: 0x0001, code: "LPMS", name: "LowPowerModeSensitivity", summary: "Device can adjust the conditions for entering a low power mode" },
  ],
  attributes: &[
    attr::EnergyBalances::ATTR,
    attr::CurrentEnergyBalance::ATTR,
    attr::EnergyPriorities::ATTR,
    attr::LowPowerModeSensitivities::ATTR,
    attr::CurrentLowPowerModeSensitivity::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct EnergyBalances;

  impl Attr for EnergyBalances {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "EnergyBalances",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<ty::BalanceStruct>;
  }


  pub struct CurrentEnergyBalance;

  impl Attr for CurrentEnergyBalance {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "CurrentEnergyBalance",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u8;
  }


  pub struct EnergyPriorities;

  impl Attr for EnergyPriorities {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "EnergyPriorities",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<ty::EnergyPriorityEnum>;
  }


  pub struct LowPowerModeSensitivities;

  impl Attr for LowPowerModeSensitivities {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "LowPowerModeSensitivities",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Vec<ty::BalanceStruct>;
  }


  pub struct CurrentLowPowerModeSensitivity;

  impl Attr for CurrentLowPowerModeSensitivity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "CurrentLowPowerModeSensitivity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = u8;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum EnergyPriorityEnum {
  ///User comfort
  Comfort = 0x0000,
  ///Speed of operation
  Speed = 0x0001,
  ///Amount of Energy consumed by the device
  Efficiency = 0x0002,
  ///Amount of water consumed by the device
  WaterConsumption = 0x0003,
}
impl AttrType for EnergyPriorityEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Comfort"),
  (0x0001, "Speed"),
  (0x0002, "Efficiency"),
  (0x0003, "WaterConsumption"),
  ]);
}
impl TlvPut for EnergyPriorityEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for EnergyPriorityEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Comfort),
      0x0001 => Ok(Self::Speed),
      0x0002 => Ok(Self::Efficiency),
      0x0003 => Ok(Self::WaterConsumption),
      _ => Err(())
    }
  }
}
impl TlvDecode for EnergyPriorityEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct BalanceStruct {
  pub step: TypeUnsupported /* Percent */,
  pub label: String,
}
impl AttrType for BalanceStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Step", attr_ty: <TypeUnsupported /* Percent */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Label", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for BalanceStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.step.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.label.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for BalanceStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut label: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { label = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      step: TypeUnsupported,
      label: label.ok_or(TlvDecodeErr::MissingField("Label"))?,
    })
  }
}

}
