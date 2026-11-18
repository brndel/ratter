#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const COLOR_CONTROL: ClusterDef = ClusterDef {
  id: 0x0300,
  name: "Color Control",
  features: &[
    FeatureDef { bit: 0x0000, code: "HS", name: "HueSaturation", summary: "Supports color specification via hue/saturation." },
    FeatureDef { bit: 0x0001, code: "EHUE", name: "EnhancedHue", summary: "Enhanced hue is supported." },
    FeatureDef { bit: 0x0002, code: "CL", name: "ColorLoop", summary: "Color loop is supported." },
    FeatureDef { bit: 0x0003, code: "XY", name: "XY", summary: "Supports color specification via XY." },
    FeatureDef { bit: 0x0004, code: "CT", name: "ColorTemperature", summary: "Supports specification of color temperature." },
  ],
  attributes: &[
    attr::CurrentHue::ATTR,
    attr::CurrentSaturation::ATTR,
    attr::RemainingTime::ATTR,
    attr::CurrentX::ATTR,
    attr::CurrentY::ATTR,
    attr::DriftCompensation::ATTR,
    attr::CompensationText::ATTR,
    attr::ColorTemperatureMireds::ATTR,
    attr::ColorMode::ATTR,
    attr::Options::ATTR,
    attr::NumberOfPrimaries::ATTR,
    attr::Primary1X::ATTR,
    attr::Primary1Y::ATTR,
    attr::Primary1Intensity::ATTR,
    attr::Primary2X::ATTR,
    attr::Primary2Y::ATTR,
    attr::Primary2Intensity::ATTR,
    attr::Primary3X::ATTR,
    attr::Primary3Y::ATTR,
    attr::Primary3Intensity::ATTR,
    attr::Primary4X::ATTR,
    attr::Primary4Y::ATTR,
    attr::Primary4Intensity::ATTR,
    attr::Primary5X::ATTR,
    attr::Primary5Y::ATTR,
    attr::Primary5Intensity::ATTR,
    attr::Primary6X::ATTR,
    attr::Primary6Y::ATTR,
    attr::Primary6Intensity::ATTR,
    attr::WhitePointX::ATTR,
    attr::WhitePointY::ATTR,
    attr::ColorPointRX::ATTR,
    attr::ColorPointRY::ATTR,
    attr::ColorPointRIntensity::ATTR,
    attr::ColorPointGX::ATTR,
    attr::ColorPointGY::ATTR,
    attr::ColorPointGIntensity::ATTR,
    attr::ColorPointBX::ATTR,
    attr::ColorPointBY::ATTR,
    attr::ColorPointBIntensity::ATTR,
    attr::EnhancedCurrentHue::ATTR,
    attr::EnhancedColorMode::ATTR,
    attr::ColorLoopActive::ATTR,
    attr::ColorLoopDirection::ATTR,
    attr::ColorLoopTime::ATTR,
    attr::ColorLoopStartEnhancedHue::ATTR,
    attr::ColorLoopStoredEnhancedHue::ATTR,
    attr::ColorCapabilities::ATTR,
    attr::ColorTempPhysicalMinMireds::ATTR,
    attr::ColorTempPhysicalMaxMireds::ATTR,
    attr::CoupleColorTempToLevelMinMireds::ATTR,
    attr::StartUpColorTemperatureMireds::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::MoveToHue::COMMAND,
    command::MoveHue::COMMAND,
    command::StepHue::COMMAND,
    command::MoveToSaturation::COMMAND,
    command::MoveSaturation::COMMAND,
    command::StepSaturation::COMMAND,
    command::MoveToHueAndSaturation::COMMAND,
    command::MoveToColor::COMMAND,
    command::MoveColor::COMMAND,
    command::StepColor::COMMAND,
    command::MoveToColorTemperature::COMMAND,
    command::EnhancedMoveToHue::COMMAND,
    command::EnhancedMoveHue::COMMAND,
    command::EnhancedStepHue::COMMAND,
    command::EnhancedMoveToHueAndSaturation::COMMAND,
    command::ColorLoopSet::COMMAND,
    command::StopMoveStep::COMMAND,
    command::MoveColorTemperature::COMMAND,
    command::StepColorTemperature::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct CurrentHue;

  impl Attr for CurrentHue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "CurrentHue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u8;
  }


  pub struct CurrentSaturation;

  impl Attr for CurrentSaturation {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "CurrentSaturation",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u8;
  }


  pub struct RemainingTime;

  impl Attr for RemainingTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "RemainingTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct CurrentX;

  impl Attr for CurrentX {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "CurrentX",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = u16;
  }


  pub struct CurrentY;

  impl Attr for CurrentY {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "CurrentY",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = u16;
  }


  pub struct DriftCompensation;

  impl Attr for DriftCompensation {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "DriftCompensation",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::DriftCompensationEnum;
  }


  pub struct CompensationText;

  impl Attr for CompensationText {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "CompensationText",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = String;
  }


  pub struct ColorTemperatureMireds;

  impl Attr for ColorTemperatureMireds {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "ColorTemperatureMireds",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = u16;
  }


  pub struct ColorMode;

  impl Attr for ColorMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "ColorMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::ColorModeEnum;
  }


  pub struct Options;

  impl Attr for Options {
    const ATTR: AttrDef = AttrDef {
      id: 0x000f,
      name: "Options",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::OptionsBitmap;
  }


  pub struct NumberOfPrimaries;

  impl Attr for NumberOfPrimaries {
    const ATTR: AttrDef = AttrDef {
      id: 0x0010,
      name: "NumberOfPrimaries",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u8>;
  }


  pub struct Primary1X;

  impl Attr for Primary1X {
    const ATTR: AttrDef = AttrDef {
      id: 0x0011,
      name: "Primary1X",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct Primary1Y;

  impl Attr for Primary1Y {
    const ATTR: AttrDef = AttrDef {
      id: 0x0012,
      name: "Primary1Y",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct Primary1Intensity;

  impl Attr for Primary1Intensity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0013,
      name: "Primary1Intensity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<u8>;
  }


  pub struct Primary2X;

  impl Attr for Primary2X {
    const ATTR: AttrDef = AttrDef {
      id: 0x0015,
      name: "Primary2X",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct Primary2Y;

  impl Attr for Primary2Y {
    const ATTR: AttrDef = AttrDef {
      id: 0x0016,
      name: "Primary2Y",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct Primary2Intensity;

  impl Attr for Primary2Intensity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0017,
      name: "Primary2Intensity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<u8>;
  }


  pub struct Primary3X;

  impl Attr for Primary3X {
    const ATTR: AttrDef = AttrDef {
      id: 0x0019,
      name: "Primary3X",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct Primary3Y;

  impl Attr for Primary3Y {
    const ATTR: AttrDef = AttrDef {
      id: 0x001a,
      name: "Primary3Y",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct Primary3Intensity;

  impl Attr for Primary3Intensity {
    const ATTR: AttrDef = AttrDef {
      id: 0x001b,
      name: "Primary3Intensity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<u8>;
  }


  pub struct Primary4X;

  impl Attr for Primary4X {
    const ATTR: AttrDef = AttrDef {
      id: 0x0020,
      name: "Primary4X",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct Primary4Y;

  impl Attr for Primary4Y {
    const ATTR: AttrDef = AttrDef {
      id: 0x0021,
      name: "Primary4Y",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct Primary4Intensity;

  impl Attr for Primary4Intensity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0022,
      name: "Primary4Intensity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<u8>;
  }


  pub struct Primary5X;

  impl Attr for Primary5X {
    const ATTR: AttrDef = AttrDef {
      id: 0x0024,
      name: "Primary5X",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct Primary5Y;

  impl Attr for Primary5Y {
    const ATTR: AttrDef = AttrDef {
      id: 0x0025,
      name: "Primary5Y",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct Primary5Intensity;

  impl Attr for Primary5Intensity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0026,
      name: "Primary5Intensity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<u8>;
  }


  pub struct Primary6X;

  impl Attr for Primary6X {
    const ATTR: AttrDef = AttrDef {
      id: 0x0028,
      name: "Primary6X",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct Primary6Y;

  impl Attr for Primary6Y {
    const ATTR: AttrDef = AttrDef {
      id: 0x0029,
      name: "Primary6Y",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = u16;
  }


  pub struct Primary6Intensity;

  impl Attr for Primary6Intensity {
    const ATTR: AttrDef = AttrDef {
      id: 0x002a,
      name: "Primary6Intensity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<u8>;
  }


  pub struct WhitePointX;

  impl Attr for WhitePointX {
    const ATTR: AttrDef = AttrDef {
      id: 0x0030,
      name: "WhitePointX",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct WhitePointY;

  impl Attr for WhitePointY {
    const ATTR: AttrDef = AttrDef {
      id: 0x0031,
      name: "WhitePointY",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct ColorPointRX;

  impl Attr for ColorPointRX {
    const ATTR: AttrDef = AttrDef {
      id: 0x0032,
      name: "ColorPointRX",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct ColorPointRY;

  impl Attr for ColorPointRY {
    const ATTR: AttrDef = AttrDef {
      id: 0x0033,
      name: "ColorPointRY",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct ColorPointRIntensity;

  impl Attr for ColorPointRIntensity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0034,
      name: "ColorPointRIntensity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u8>;
  }


  pub struct ColorPointGX;

  impl Attr for ColorPointGX {
    const ATTR: AttrDef = AttrDef {
      id: 0x0036,
      name: "ColorPointGX",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct ColorPointGY;

  impl Attr for ColorPointGY {
    const ATTR: AttrDef = AttrDef {
      id: 0x0037,
      name: "ColorPointGY",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct ColorPointGIntensity;

  impl Attr for ColorPointGIntensity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0038,
      name: "ColorPointGIntensity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u8>;
  }


  pub struct ColorPointBX;

  impl Attr for ColorPointBX {
    const ATTR: AttrDef = AttrDef {
      id: 0x003a,
      name: "ColorPointBX",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct ColorPointBY;

  impl Attr for ColorPointBY {
    const ATTR: AttrDef = AttrDef {
      id: 0x003b,
      name: "ColorPointBY",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct ColorPointBIntensity;

  impl Attr for ColorPointBIntensity {
    const ATTR: AttrDef = AttrDef {
      id: 0x003c,
      name: "ColorPointBIntensity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u8>;
  }


  pub struct EnhancedCurrentHue;

  impl Attr for EnhancedCurrentHue {
    const ATTR: AttrDef = AttrDef {
      id: 0x4000,
      name: "EnhancedCurrentHue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = u16;
  }


  pub struct EnhancedColorMode;

  impl Attr for EnhancedColorMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x4001,
      name: "EnhancedColorMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::EnhancedColorModeEnum;
  }


  pub struct ColorLoopActive;

  impl Attr for ColorLoopActive {
    const ATTR: AttrDef = AttrDef {
      id: 0x4002,
      name: "ColorLoopActive",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = u8;
  }


  pub struct ColorLoopDirection;

  impl Attr for ColorLoopDirection {
    const ATTR: AttrDef = AttrDef {
      id: 0x4003,
      name: "ColorLoopDirection",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = ty::ColorLoopDirectionEnum;
  }


  pub struct ColorLoopTime;

  impl Attr for ColorLoopTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x4004,
      name: "ColorLoopTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = u16;
  }


  pub struct ColorLoopStartEnhancedHue;

  impl Attr for ColorLoopStartEnhancedHue {
    const ATTR: AttrDef = AttrDef {
      id: 0x4005,
      name: "ColorLoopStartEnhancedHue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = u16;
  }


  pub struct ColorLoopStoredEnhancedHue;

  impl Attr for ColorLoopStoredEnhancedHue {
    const ATTR: AttrDef = AttrDef {
      id: 0x4006,
      name: "ColorLoopStoredEnhancedHue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = u16;
  }


  pub struct ColorCapabilities;

  impl Attr for ColorCapabilities {
    const ATTR: AttrDef = AttrDef {
      id: 0x400a,
      name: "ColorCapabilities",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::ColorCapabilitiesBitmap;
  }


  pub struct ColorTempPhysicalMinMireds;

  impl Attr for ColorTempPhysicalMinMireds {
    const ATTR: AttrDef = AttrDef {
      id: 0x400b,
      name: "ColorTempPhysicalMinMireds",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = u16;
  }


  pub struct ColorTempPhysicalMaxMireds;

  impl Attr for ColorTempPhysicalMaxMireds {
    const ATTR: AttrDef = AttrDef {
      id: 0x400c,
      name: "ColorTempPhysicalMaxMireds",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = u16;
  }


  pub struct CoupleColorTempToLevelMinMireds;

  impl Attr for CoupleColorTempToLevelMinMireds {
    const ATTR: AttrDef = AttrDef {
      id: 0x400d,
      name: "CoupleColorTempToLevelMinMireds",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = u16;
  }


  pub struct StartUpColorTemperatureMireds;

  impl Attr for StartUpColorTemperatureMireds {
    const ATTR: AttrDef = AttrDef {
      id: 0x4010,
      name: "StartUpColorTemperatureMireds",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = Nullable<u16>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct MoveToHue {
  pub hue: u8,
  pub direction: ty::DirectionEnum,
  pub transition_time: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for MoveToHue {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "MoveToHue",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "Hue", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Direction", attr_ty: <ty::DirectionEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TransitionTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for MoveToHue {
  type Response = BasicAccResponse;
}
impl TlvEncode for MoveToHue {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.hue.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.direction.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveHue {
  pub move_mode: ty::MoveModeEnum,
  pub rate: u8,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for MoveHue {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "MoveHue",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "MoveMode", attr_ty: <ty::MoveModeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Rate", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for MoveHue {
  type Response = BasicAccResponse;
}
impl TlvEncode for MoveHue {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.move_mode.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.rate.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StepHue {
  pub step_mode: ty::StepModeEnum,
  pub step_size: u8,
  pub transition_time: u8,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for StepHue {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "StepHue",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "StepMode", attr_ty: <ty::StepModeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "StepSize", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TransitionTime", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for StepHue {
  type Response = BasicAccResponse;
}
impl TlvEncode for StepHue {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.step_mode.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.step_size.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveToSaturation {
  pub saturation: u8,
  pub transition_time: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for MoveToSaturation {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "MoveToSaturation",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "Saturation", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "TransitionTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for MoveToSaturation {
  type Response = BasicAccResponse;
}
impl TlvEncode for MoveToSaturation {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.saturation.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveSaturation {
  pub move_mode: ty::MoveModeEnum,
  pub rate: u8,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for MoveSaturation {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "MoveSaturation",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "MoveMode", attr_ty: <ty::MoveModeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Rate", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for MoveSaturation {
  type Response = BasicAccResponse;
}
impl TlvEncode for MoveSaturation {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.move_mode.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.rate.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StepSaturation {
  pub step_mode: ty::StepModeEnum,
  pub step_size: u8,
  pub transition_time: u8,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for StepSaturation {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "StepSaturation",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "StepMode", attr_ty: <ty::StepModeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "StepSize", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TransitionTime", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for StepSaturation {
  type Response = BasicAccResponse;
}
impl TlvEncode for StepSaturation {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.step_mode.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.step_size.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveToHueAndSaturation {
  pub hue: u8,
  pub saturation: u8,
  pub transition_time: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for MoveToHueAndSaturation {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "MoveToHueAndSaturation",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "Hue", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Saturation", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TransitionTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for MoveToHueAndSaturation {
  type Response = BasicAccResponse;
}
impl TlvEncode for MoveToHueAndSaturation {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.hue.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.saturation.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveToColor {
  pub color_x: u16,
  pub color_y: u16,
  pub transition_time: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for MoveToColor {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "MoveToColor",
    conformance: Conformance::Feature(3),
    fields: &[
      AttrDef { id: 0x0000, name: "ColorX", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ColorY", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TransitionTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for MoveToColor {
  type Response = BasicAccResponse;
}
impl TlvEncode for MoveToColor {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.color_x.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.color_y.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveColor {
  pub rate_x: i16,
  pub rate_y: i16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for MoveColor {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0008,
    name: "MoveColor",
    conformance: Conformance::Feature(3),
    fields: &[
      AttrDef { id: 0x0000, name: "RateX", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "RateY", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for MoveColor {
  type Response = BasicAccResponse;
}
impl TlvEncode for MoveColor {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.rate_x.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.rate_y.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StepColor {
  pub step_x: i16,
  pub step_y: i16,
  pub transition_time: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for StepColor {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0009,
    name: "StepColor",
    conformance: Conformance::Feature(3),
    fields: &[
      AttrDef { id: 0x0000, name: "StepX", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "StepY", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TransitionTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for StepColor {
  type Response = BasicAccResponse;
}
impl TlvEncode for StepColor {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.step_x.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.step_y.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveToColorTemperature {
  pub color_temperature_mireds: u16,
  pub transition_time: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for MoveToColorTemperature {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000a,
    name: "MoveToColorTemperature",
    conformance: Conformance::Feature(4),
    fields: &[
      AttrDef { id: 0x0000, name: "ColorTemperatureMireds", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "TransitionTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for MoveToColorTemperature {
  type Response = BasicAccResponse;
}
impl TlvEncode for MoveToColorTemperature {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.color_temperature_mireds.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnhancedMoveToHue {
  pub enhanced_hue: u16,
  pub direction: ty::DirectionEnum,
  pub transition_time: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for EnhancedMoveToHue {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0040,
    name: "EnhancedMoveToHue",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "EnhancedHue", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Direction", attr_ty: <ty::DirectionEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TransitionTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for EnhancedMoveToHue {
  type Response = BasicAccResponse;
}
impl TlvEncode for EnhancedMoveToHue {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.enhanced_hue.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.direction.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnhancedMoveHue {
  pub move_mode: ty::MoveModeEnum,
  pub rate: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for EnhancedMoveHue {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0041,
    name: "EnhancedMoveHue",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "MoveMode", attr_ty: <ty::MoveModeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Rate", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for EnhancedMoveHue {
  type Response = BasicAccResponse;
}
impl TlvEncode for EnhancedMoveHue {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.move_mode.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.rate.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnhancedStepHue {
  pub step_mode: ty::StepModeEnum,
  pub step_size: u16,
  pub transition_time: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for EnhancedStepHue {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0042,
    name: "EnhancedStepHue",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "StepMode", attr_ty: <ty::StepModeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "StepSize", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TransitionTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for EnhancedStepHue {
  type Response = BasicAccResponse;
}
impl TlvEncode for EnhancedStepHue {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.step_mode.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.step_size.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnhancedMoveToHueAndSaturation {
  pub enhanced_hue: u16,
  pub saturation: u8,
  pub transition_time: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for EnhancedMoveToHueAndSaturation {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0043,
    name: "EnhancedMoveToHueAndSaturation",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "EnhancedHue", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Saturation", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TransitionTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for EnhancedMoveToHueAndSaturation {
  type Response = BasicAccResponse;
}
impl TlvEncode for EnhancedMoveToHueAndSaturation {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.enhanced_hue.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.saturation.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColorLoopSet {
  pub update_flags: ty::UpdateFlagsBitmap,
  pub action: ty::ColorLoopActionEnum,
  pub direction: ty::ColorLoopDirectionEnum,
  pub time: u16,
  pub start_hue: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for ColorLoopSet {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0044,
    name: "ColorLoopSet",
    conformance: Conformance::Feature(2),
    fields: &[
      AttrDef { id: 0x0000, name: "UpdateFlags", attr_ty: <ty::UpdateFlagsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Action", attr_ty: <ty::ColorLoopActionEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Direction", attr_ty: <ty::ColorLoopDirectionEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "Time", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "StartHue", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0005, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0006, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ColorLoopSet {
  type Response = BasicAccResponse;
}
impl TlvEncode for ColorLoopSet {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.update_flags.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.action.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.direction.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.time.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.start_hue.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StopMoveStep {
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for StopMoveStep {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0047,
    name: "StopMoveStep",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for StopMoveStep {
  type Response = BasicAccResponse;
}
impl TlvEncode for StopMoveStep {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct MoveColorTemperature {
  pub move_mode: ty::MoveModeEnum,
  pub rate: u16,
  pub color_temperature_minimum_mireds: u16,
  pub color_temperature_maximum_mireds: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for MoveColorTemperature {
  const COMMAND: CommandDef = CommandDef {
    id: 0x004b,
    name: "MoveColorTemperature",
    conformance: Conformance::Feature(4),
    fields: &[
      AttrDef { id: 0x0000, name: "MoveMode", attr_ty: <ty::MoveModeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Rate", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "ColorTemperatureMinimumMireds", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "ColorTemperatureMaximumMireds", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0005, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for MoveColorTemperature {
  type Response = BasicAccResponse;
}
impl TlvEncode for MoveColorTemperature {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.move_mode.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.rate.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.color_temperature_minimum_mireds.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.color_temperature_maximum_mireds.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StepColorTemperature {
  pub step_mode: ty::StepModeEnum,
  pub step_size: u16,
  pub transition_time: u16,
  pub color_temperature_minimum_mireds: u16,
  pub color_temperature_maximum_mireds: u16,
  pub options_mask: ty::OptionsBitmap,
  pub options_override: ty::OptionsBitmap,
}
impl Command for StepColorTemperature {
  const COMMAND: CommandDef = CommandDef {
    id: 0x004c,
    name: "StepColorTemperature",
    conformance: Conformance::Feature(4),
    fields: &[
      AttrDef { id: 0x0000, name: "StepMode", attr_ty: <ty::StepModeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "StepSize", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "TransitionTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "ColorTemperatureMinimumMireds", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "ColorTemperatureMaximumMireds", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0005, name: "OptionsMask", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0006, name: "OptionsOverride", attr_ty: <ty::OptionsBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for StepColorTemperature {
  type Response = BasicAccResponse;
}
impl TlvEncode for StepColorTemperature {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.step_mode.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.step_size.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.transition_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.color_temperature_minimum_mireds.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.color_temperature_maximum_mireds.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.options_mask.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.options_override.tlv_put(matter_codec::Tag::Context(0x0006), writer);
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
pub enum ColorLoopActionEnum {
  ///De-activate the color loop.
  Deactivate = 0x0000,
  ///Activate the color loop from the value in the ColorLoopStartEnhancedHue field.
  ActivateFromColorLoopStartEnhancedHue = 0x0001,
  ///Activate the color loop from the value of the EnhancedCurrentHue attribute.
  ActivateFromEnhancedCurrentHue = 0x0002,
}
impl AttrType for ColorLoopActionEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Deactivate"),
  (0x0001, "ActivateFromColorLoopStartEnhancedHue"),
  (0x0002, "ActivateFromEnhancedCurrentHue"),
  ]);
}
impl TlvPut for ColorLoopActionEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ColorLoopActionEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Deactivate),
      0x0001 => Ok(Self::ActivateFromColorLoopStartEnhancedHue),
      0x0002 => Ok(Self::ActivateFromEnhancedCurrentHue),
      _ => Err(())
    }
  }
}
impl TlvDecode for ColorLoopActionEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ColorLoopDirectionEnum {
  ///Decrement the hue in the color loop.
  Decrement = 0x0000,
  ///Increment the hue in the color loop.
  Increment = 0x0001,
}
impl AttrType for ColorLoopDirectionEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Decrement"),
  (0x0001, "Increment"),
  ]);
}
impl TlvPut for ColorLoopDirectionEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ColorLoopDirectionEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Decrement),
      0x0001 => Ok(Self::Increment),
      _ => Err(())
    }
  }
}
impl TlvDecode for ColorLoopDirectionEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ColorModeEnum {
  ///The current hue and saturation attributes determine the color.
  CurrentHueAndCurrentSaturation = 0x0000,
  ///The current X and Y attributes determine the color.
  CurrentXAndCurrentY = 0x0001,
  ///The color temperature attribute determines the color.
  ColorTemperatureMireds = 0x0002,
}
impl AttrType for ColorModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "CurrentHueAndCurrentSaturation"),
  (0x0001, "CurrentXAndCurrentY"),
  (0x0002, "ColorTemperatureMireds"),
  ]);
}
impl TlvPut for ColorModeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ColorModeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::CurrentHueAndCurrentSaturation),
      0x0001 => Ok(Self::CurrentXAndCurrentY),
      0x0002 => Ok(Self::ColorTemperatureMireds),
      _ => Err(())
    }
  }
}
impl TlvDecode for ColorModeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum DirectionEnum {
  ///Shortest distance
  Shortest = 0x0000,
  ///Longest distance
  Longest = 0x0001,
  ///Up
  Up = 0x0002,
  ///Down
  Down = 0x0003,
}
impl AttrType for DirectionEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Shortest"),
  (0x0001, "Longest"),
  (0x0002, "Up"),
  (0x0003, "Down"),
  ]);
}
impl TlvPut for DirectionEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for DirectionEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Shortest),
      0x0001 => Ok(Self::Longest),
      0x0002 => Ok(Self::Up),
      0x0003 => Ok(Self::Down),
      _ => Err(())
    }
  }
}
impl TlvDecode for DirectionEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum DriftCompensationEnum {
  ///There is no compensation.
  None = 0x0000,
  ///The compensation is based on other or unknown mechanism.
  OtherOrUnknown = 0x0001,
  ///The compensation is based on temperature monitoring.
  TemperatureMonitoring = 0x0002,
  ///The compensation is based on optical luminance monitoring and feedback.
  OpticalLuminanceMonitoringAndFeedback = 0x0003,
  ///The compensation is based on optical color monitoring and feedback.
  OpticalColorMonitoringAndFeedback = 0x0004,
}
impl AttrType for DriftCompensationEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "None"),
  (0x0001, "OtherOrUnknown"),
  (0x0002, "TemperatureMonitoring"),
  (0x0003, "OpticalLuminanceMonitoringAndFeedback"),
  (0x0004, "OpticalColorMonitoringAndFeedback"),
  ]);
}
impl TlvPut for DriftCompensationEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for DriftCompensationEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::None),
      0x0001 => Ok(Self::OtherOrUnknown),
      0x0002 => Ok(Self::TemperatureMonitoring),
      0x0003 => Ok(Self::OpticalLuminanceMonitoringAndFeedback),
      0x0004 => Ok(Self::OpticalColorMonitoringAndFeedback),
      _ => Err(())
    }
  }
}
impl TlvDecode for DriftCompensationEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum EnhancedColorModeEnum {
  ///The current hue and saturation attributes determine the color.
  CurrentHueAndCurrentSaturation = 0x0000,
  ///The current X and Y attributes determine the color.
  CurrentXAndCurrentY = 0x0001,
  ///The color temperature attribute determines the color.
  ColorTemperatureMireds = 0x0002,
  ///The enhanced current hue and saturation attributes determine the color.
  EnhancedCurrentHueAndCurrentSaturation = 0x0003,
}
impl AttrType for EnhancedColorModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "CurrentHueAndCurrentSaturation"),
  (0x0001, "CurrentXAndCurrentY"),
  (0x0002, "ColorTemperatureMireds"),
  (0x0003, "EnhancedCurrentHueAndCurrentSaturation"),
  ]);
}
impl TlvPut for EnhancedColorModeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for EnhancedColorModeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::CurrentHueAndCurrentSaturation),
      0x0001 => Ok(Self::CurrentXAndCurrentY),
      0x0002 => Ok(Self::ColorTemperatureMireds),
      0x0003 => Ok(Self::EnhancedCurrentHueAndCurrentSaturation),
      _ => Err(())
    }
  }
}
impl TlvDecode for EnhancedColorModeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum MoveModeEnum {
  ///Stop the movement
  Stop = 0x0000,
  ///Move in an upwards direction
  Up = 0x0001,
  ///Move in a downwards direction
  Down = 0x0003,
}
impl AttrType for MoveModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Stop"),
  (0x0001, "Up"),
  (0x0003, "Down"),
  ]);
}
impl TlvPut for MoveModeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for MoveModeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Stop),
      0x0001 => Ok(Self::Up),
      0x0003 => Ok(Self::Down),
      _ => Err(())
    }
  }
}
impl TlvDecode for MoveModeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StepModeEnum {
  ///Step in an upwards direction
  Up = 0x0001,
  ///Step in a downwards direction
  Down = 0x0003,
}
impl AttrType for StepModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0001, "Up"),
  (0x0003, "Down"),
  ]);
}
impl TlvPut for StepModeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StepModeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0001 => Ok(Self::Up),
      0x0003 => Ok(Self::Down),
      _ => Err(())
    }
  }
}
impl TlvDecode for StepModeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct ColorCapabilitiesBitmap: u32 {
    ///Supports color specification via hue/saturation.
    const HueSaturation = 1 << 0;
    ///Enhanced hue is supported.
    const EnhancedHue = 1 << 1;
    ///Color loop is supported.
    const ColorLoop = 1 << 2;
    ///Supports color specification via XY.
    const XY = 1 << 3;
    ///Supports color specification via color temperature.
    const ColorTemperature = 1 << 4;
  }
}
impl AttrType for ColorCapabilitiesBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "HueSaturation"),
    (1, "EnhancedHue"),
    (2, "ColorLoop"),
    (3, "XY"),
    (4, "ColorTemperature"),
  ]);
}
impl TlvPut for ColorCapabilitiesBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for ColorCapabilitiesBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for ColorCapabilitiesBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct OptionsBitmap: u32 {
    ///Dependency on On/Off cluster
    const ExecuteIfOff = 1 << 0;
  }
}
impl AttrType for OptionsBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "ExecuteIfOff"),
  ]);
}
impl TlvPut for OptionsBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for OptionsBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for OptionsBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct UpdateFlagsBitmap: u32 {
    ///Device adheres to the associated action field.
    const UpdateAction = 1 << 0;
    ///Device updates the associated direction attribute.
    const UpdateDirection = 1 << 1;
    ///Device updates the associated time attribute.
    const UpdateTime = 1 << 2;
    ///Device updates the associated start hue attribute.
    const UpdateStartHue = 1 << 3;
  }
}
impl AttrType for UpdateFlagsBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "UpdateAction"),
    (1, "UpdateDirection"),
    (2, "UpdateTime"),
    (3, "UpdateStartHue"),
  ]);
}
impl TlvPut for UpdateFlagsBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for UpdateFlagsBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for UpdateFlagsBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
}
