#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const MICROWAVE_OVEN_CONTROL: ClusterDef = ClusterDef {
  id: 0x005f,
  name: "Microwave Oven Control",
  features: &[
    FeatureDef { bit: 0x0000, code: "PWRNUM", name: "PowerAsNumber", summary: "Power is specified as a unitless number or a percentage" },
    FeatureDef { bit: 0x0001, code: "WATTS", name: "PowerInWatts", summary: "Power is specified in Watts" },
    FeatureDef { bit: 0x0002, code: "PWRLMTS", name: "PowerNumberLimits", summary: "Supports the limit attributes used with the PWRNUM feature" },
  ],
  attributes: &[
    attr::CookTime::ATTR,
    attr::MaxCookTime::ATTR,
    attr::PowerSetting::ATTR,
    attr::MinPower::ATTR,
    attr::MaxPower::ATTR,
    attr::PowerStep::ATTR,
    attr::SupportedWatts::ATTR,
    attr::SelectedWattIndex::ATTR,
    attr::WattRating::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::SetCookingParameters::COMMAND,
    command::AddMoreTime::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct CookTime;

  impl Attr for CookTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "CookTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = TypeUnsupported /* ElapsedS */;
  }


  pub struct MaxCookTime;

  impl Attr for MaxCookTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "MaxCookTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = TypeUnsupported /* ElapsedS */;
  }


  pub struct PowerSetting;

  impl Attr for PowerSetting {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "PowerSetting",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u8;
  }


  pub struct MinPower;

  impl Attr for MinPower {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "MinPower",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = u8;
  }


  pub struct MaxPower;

  impl Attr for MaxPower {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "MaxPower",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = u8;
  }


  pub struct PowerStep;

  impl Attr for PowerStep {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "PowerStep",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = u8;
  }


  pub struct SupportedWatts;

  impl Attr for SupportedWatts {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "SupportedWatts",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Vec<u16>;
  }


  pub struct SelectedWattIndex;

  impl Attr for SelectedWattIndex {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "SelectedWattIndex",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u8;
  }


  pub struct WattRating;

  impl Attr for WattRating {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "WattRating",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SetCookingParameters {
  pub cook_mode: u8,
  pub cook_time: TypeUnsupported /* ElapsedS */,
  pub power_setting: u8,
  pub watt_setting_index: u8,
  pub start_after_setting: bool,
}
impl Command for SetCookingParameters {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "SetCookingParameters",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CookMode", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0001, name: "CookTime", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "PowerSetting", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0003, name: "WattSettingIndex", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0004, name: "StartAfterSetting", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for SetCookingParameters {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetCookingParameters {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.cook_mode.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.cook_time.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.power_setting.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.watt_setting_index.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.start_after_setting.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddMoreTime {
  pub time_to_add: TypeUnsupported /* ElapsedS */,
}
impl Command for AddMoreTime {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "AddMoreTime",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "TimeToAdd", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddMoreTime {
  type Response = BasicAccResponse;
}
impl TlvEncode for AddMoreTime {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.time_to_add.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

}
pub mod ty {
  pub use crate::globals::*;
}
