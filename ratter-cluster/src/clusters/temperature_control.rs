#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const TEMPERATURE_CONTROL: ClusterDef = ClusterDef {
  id: 0x0056,
  name: "Temperature Control",
  features: &[
    FeatureDef { bit: 0x0000, code: "TN", name: "TemperatureNumber", summary: "Use actual temperature numbers" },
    FeatureDef { bit: 0x0001, code: "TL", name: "TemperatureLevel", summary: "Use temperature levels" },
    FeatureDef { bit: 0x0002, code: "STEP", name: "TemperatureStep", summary: "Use step control with temperature numbers" },
  ],
  attributes: &[
    attr::TemperatureSetpoint::ATTR,
    attr::MinTemperature::ATTR,
    attr::MaxTemperature::ATTR,
    attr::Step::ATTR,
    attr::SelectedTemperatureLevel::ATTR,
    attr::SupportedTemperatureLevels::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::SetTemperature::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct TemperatureSetpoint;

  impl Attr for TemperatureSetpoint {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "TemperatureSetpoint",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Temperature;
  }


  pub struct MinTemperature;

  impl Attr for MinTemperature {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "MinTemperature",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Temperature;
  }


  pub struct MaxTemperature;

  impl Attr for MaxTemperature {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "MaxTemperature",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Temperature;
  }


  pub struct Step;

  impl Attr for Step {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "Step",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = Temperature;
  }


  pub struct SelectedTemperatureLevel;

  impl Attr for SelectedTemperatureLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "SelectedTemperatureLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = u8;
  }


  pub struct SupportedTemperatureLevels;

  impl Attr for SupportedTemperatureLevels {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "SupportedTemperatureLevels",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Vec<String>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SetTemperature {
  pub target_temperature: Temperature,
  pub target_temperature_level: u8,
}
impl Command for SetTemperature {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "SetTemperature",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "TargetTemperature", attr_ty: <Temperature>::ATTR_TYPE, conformance: Conformance::Feature(0) },
      AttrDef { id: 0x0001, name: "TargetTemperatureLevel", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Feature(1) },
    ],
  };
}
impl CommandInvoke for SetTemperature {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetTemperature {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.target_temperature.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.target_temperature_level.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

}
pub mod ty {
  pub use crate::globals::*;
}
