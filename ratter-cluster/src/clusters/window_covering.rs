#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const WINDOW_COVERING: ClusterDef = ClusterDef {
  id: 0x0102,
  name: "Window Covering",
  features: &[
    FeatureDef { bit: 0x0000, code: "LF", name: "Lift", summary: "Lift control and behavior for lifting/sliding window coverings" },
    FeatureDef { bit: 0x0001, code: "TL", name: "Tilt", summary: "Tilt control and behavior for tilting window coverings" },
    FeatureDef { bit: 0x0002, code: "PA_LF", name: "PositionAwareLift", summary: "Position aware lift control is supported." },
    FeatureDef { bit: 0x0004, code: "PA_TL", name: "PositionAwareTilt", summary: "Position aware tilt control is supported." },
  ],
  attributes: &[
    attr::Type::ATTR,
    attr::NumberOfActuationsLift::ATTR,
    attr::NumberOfActuationsTilt::ATTR,
    attr::ConfigStatus::ATTR,
    attr::CurrentPositionLiftPercentage::ATTR,
    attr::CurrentPositionTiltPercentage::ATTR,
    attr::OperationalStatus::ATTR,
    attr::TargetPositionLiftPercent100ths::ATTR,
    attr::TargetPositionTiltPercent100ths::ATTR,
    attr::EndProductType::ATTR,
    attr::CurrentPositionLiftPercent100ths::ATTR,
    attr::CurrentPositionTiltPercent100ths::ATTR,
    attr::Mode::ATTR,
    attr::SafetyStatus::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::UpOrOpen::COMMAND,
    command::DownOrClose::COMMAND,
    command::StopMotion::COMMAND,
    command::GoToLiftPercentage::COMMAND,
    command::GoToTiltPercentage::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct Type;

  impl Attr for Type {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "Type",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::TypeEnum;
  }


  pub struct NumberOfActuationsLift;

  impl Attr for NumberOfActuationsLift {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "NumberOfActuationsLift",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct NumberOfActuationsTilt;

  impl Attr for NumberOfActuationsTilt {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "NumberOfActuationsTilt",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct ConfigStatus;

  impl Attr for ConfigStatus {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "ConfigStatus",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::ConfigStatusBitmap;
  }


  pub struct CurrentPositionLiftPercentage;

  impl Attr for CurrentPositionLiftPercentage {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "CurrentPositionLiftPercentage",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<TypeUnsupported /* Percent */>;
  }


  pub struct CurrentPositionTiltPercentage;

  impl Attr for CurrentPositionTiltPercentage {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "CurrentPositionTiltPercentage",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<TypeUnsupported /* Percent */>;
  }


  pub struct OperationalStatus;

  impl Attr for OperationalStatus {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "OperationalStatus",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::OperationalStatusBitmap;
  }


  pub struct TargetPositionLiftPercent100ths;

  impl Attr for TargetPositionLiftPercent100ths {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "TargetPositionLiftPercent100ths",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* Percent100ths */>;
  }


  pub struct TargetPositionTiltPercent100ths;

  impl Attr for TargetPositionTiltPercent100ths {
    const ATTR: AttrDef = AttrDef {
      id: 0x000c,
      name: "TargetPositionTiltPercent100ths",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* Percent100ths */>;
  }


  pub struct EndProductType;

  impl Attr for EndProductType {
    const ATTR: AttrDef = AttrDef {
      id: 0x000d,
      name: "EndProductType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::EndProductTypeEnum;
  }


  pub struct CurrentPositionLiftPercent100ths;

  impl Attr for CurrentPositionLiftPercent100ths {
    const ATTR: AttrDef = AttrDef {
      id: 0x000e,
      name: "CurrentPositionLiftPercent100ths",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* Percent100ths */>;
  }


  pub struct CurrentPositionTiltPercent100ths;

  impl Attr for CurrentPositionTiltPercent100ths {
    const ATTR: AttrDef = AttrDef {
      id: 0x000f,
      name: "CurrentPositionTiltPercent100ths",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* Percent100ths */>;
  }


  pub struct Mode;

  impl Attr for Mode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0017,
      name: "Mode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::ModeBitmap;
  }


  pub struct SafetyStatus;

  impl Attr for SafetyStatus {
    const ATTR: AttrDef = AttrDef {
      id: 0x001a,
      name: "SafetyStatus",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::SafetyStatusBitmap;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct UpOrOpen {
}
impl Command for UpOrOpen {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "UpOrOpen",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for UpOrOpen {
  type Response = BasicAccResponse;
}
impl TlvEncode for UpOrOpen {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DownOrClose {
}
impl Command for DownOrClose {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "DownOrClose",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for DownOrClose {
  type Response = BasicAccResponse;
}
impl TlvEncode for DownOrClose {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StopMotion {
}
impl Command for StopMotion {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "StopMotion",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for StopMotion {
  type Response = BasicAccResponse;
}
impl TlvEncode for StopMotion {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoToLiftPercentage {
  pub lift_percent100ths_value: TypeUnsupported /* Percent100ths */,
}
impl Command for GoToLiftPercentage {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "GoToLiftPercentage",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "LiftPercent100thsValue", attr_ty: <TypeUnsupported /* Percent100ths */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for GoToLiftPercentage {
  type Response = BasicAccResponse;
}
impl TlvEncode for GoToLiftPercentage {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.lift_percent100ths_value.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GoToTiltPercentage {
  pub tilt_percent100ths_value: TypeUnsupported /* Percent100ths */,
}
impl Command for GoToTiltPercentage {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0008,
    name: "GoToTiltPercentage",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "TiltPercent100thsValue", attr_ty: <TypeUnsupported /* Percent100ths */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for GoToTiltPercentage {
  type Response = BasicAccResponse;
}
impl TlvEncode for GoToTiltPercentage {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.tilt_percent100ths_value.tlv_put(matter_codec::Tag::Context(0x0000), writer);
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
pub enum EndProductTypeEnum {
  ///Simple Roller Shade
  RollerShade = 0x0000,
  ///Roman Shade
  RomanShade = 0x0001,
  ///Balloon Shade
  BalloonShade = 0x0002,
  ///Woven Wood
  WovenWood = 0x0003,
  ///Pleated Shade
  PleatedShade = 0x0004,
  ///Cellular Shade
  CellularShade = 0x0005,
  ///Layered Shade
  LayeredShade = 0x0006,
  ///Layered Shade 2D
  LayeredShade2D = 0x0007,
  ///Sheer Shade
  SheerShade = 0x0008,
  ///Tilt Only Interior Blind
  TiltOnlyInteriorBlind = 0x0009,
  ///Interior Blind
  InteriorBlind = 0x000a,
  ///Vertical Blind, Strip Curtain
  VerticalBlindStripCurtain = 0x000b,
  ///Interior Venetian Blind
  InteriorVenetianBlind = 0x000c,
  ///Exterior Venetian Blind
  ExteriorVenetianBlind = 0x000d,
  ///Lateral Left Curtain
  LateralLeftCurtain = 0x000e,
  ///Lateral Right Curtain
  LateralRightCurtain = 0x000f,
  ///Central Curtain
  CentralCurtain = 0x0010,
  ///Roller Shutter
  RollerShutter = 0x0011,
  ///Exterior Vertical Screen
  ExteriorVerticalScreen = 0x0012,
  ///Awning Terrace (Patio)
  AwningTerracePatio = 0x0013,
  ///Awning Vertical Screen
  AwningVerticalScreen = 0x0014,
  ///Tilt Only Pergola
  TiltOnlyPergola = 0x0015,
  ///Swinging Shutter
  SwingingShutter = 0x0016,
  ///Sliding Shutter
  SlidingShutter = 0x0017,
  ///Unknown
  Unknown = 0x00ff,
}
impl AttrType for EndProductTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "RollerShade"),
  (0x0001, "RomanShade"),
  (0x0002, "BalloonShade"),
  (0x0003, "WovenWood"),
  (0x0004, "PleatedShade"),
  (0x0005, "CellularShade"),
  (0x0006, "LayeredShade"),
  (0x0007, "LayeredShade2D"),
  (0x0008, "SheerShade"),
  (0x0009, "TiltOnlyInteriorBlind"),
  (0x000a, "InteriorBlind"),
  (0x000b, "VerticalBlindStripCurtain"),
  (0x000c, "InteriorVenetianBlind"),
  (0x000d, "ExteriorVenetianBlind"),
  (0x000e, "LateralLeftCurtain"),
  (0x000f, "LateralRightCurtain"),
  (0x0010, "CentralCurtain"),
  (0x0011, "RollerShutter"),
  (0x0012, "ExteriorVerticalScreen"),
  (0x0013, "AwningTerracePatio"),
  (0x0014, "AwningVerticalScreen"),
  (0x0015, "TiltOnlyPergola"),
  (0x0016, "SwingingShutter"),
  (0x0017, "SlidingShutter"),
  (0x00ff, "Unknown"),
  ]);
}
impl TlvPut for EndProductTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for EndProductTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::RollerShade),
      0x0001 => Ok(Self::RomanShade),
      0x0002 => Ok(Self::BalloonShade),
      0x0003 => Ok(Self::WovenWood),
      0x0004 => Ok(Self::PleatedShade),
      0x0005 => Ok(Self::CellularShade),
      0x0006 => Ok(Self::LayeredShade),
      0x0007 => Ok(Self::LayeredShade2D),
      0x0008 => Ok(Self::SheerShade),
      0x0009 => Ok(Self::TiltOnlyInteriorBlind),
      0x000a => Ok(Self::InteriorBlind),
      0x000b => Ok(Self::VerticalBlindStripCurtain),
      0x000c => Ok(Self::InteriorVenetianBlind),
      0x000d => Ok(Self::ExteriorVenetianBlind),
      0x000e => Ok(Self::LateralLeftCurtain),
      0x000f => Ok(Self::LateralRightCurtain),
      0x0010 => Ok(Self::CentralCurtain),
      0x0011 => Ok(Self::RollerShutter),
      0x0012 => Ok(Self::ExteriorVerticalScreen),
      0x0013 => Ok(Self::AwningTerracePatio),
      0x0014 => Ok(Self::AwningVerticalScreen),
      0x0015 => Ok(Self::TiltOnlyPergola),
      0x0016 => Ok(Self::SwingingShutter),
      0x0017 => Ok(Self::SlidingShutter),
      0x00ff => Ok(Self::Unknown),
      _ => Err(())
    }
  }
}
impl TlvDecode for EndProductTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TypeEnum {
  ///RollerShade
  RollerShade = 0x0000,
  ///RollerShade - 2 Motor
  RollerShade2Motor = 0x0001,
  ///RollerShade - Exterior
  RollerShadeExterior = 0x0002,
  ///RollerShade - Exterior - 2 Motor
  RollerShadeExterior2Motor = 0x0003,
  ///Drapery (curtain)
  Drapery = 0x0004,
  ///Awning
  Awning = 0x0005,
  ///Shutter
  Shutter = 0x0006,
  ///Tilt Blind - Tilt Only
  TiltBlindTiltOnly = 0x0007,
  ///Tilt Blind - Lift & Tilt
  TiltBlindLiftAndTilt = 0x0008,
  ///Projector Screen
  ProjectorScreen = 0x0009,
  ///Unknown
  Unknown = 0x00ff,
}
impl AttrType for TypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "RollerShade"),
  (0x0001, "RollerShade2Motor"),
  (0x0002, "RollerShadeExterior"),
  (0x0003, "RollerShadeExterior2Motor"),
  (0x0004, "Drapery"),
  (0x0005, "Awning"),
  (0x0006, "Shutter"),
  (0x0007, "TiltBlindTiltOnly"),
  (0x0008, "TiltBlindLiftAndTilt"),
  (0x0009, "ProjectorScreen"),
  (0x00ff, "Unknown"),
  ]);
}
impl TlvPut for TypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::RollerShade),
      0x0001 => Ok(Self::RollerShade2Motor),
      0x0002 => Ok(Self::RollerShadeExterior),
      0x0003 => Ok(Self::RollerShadeExterior2Motor),
      0x0004 => Ok(Self::Drapery),
      0x0005 => Ok(Self::Awning),
      0x0006 => Ok(Self::Shutter),
      0x0007 => Ok(Self::TiltBlindTiltOnly),
      0x0008 => Ok(Self::TiltBlindLiftAndTilt),
      0x0009 => Ok(Self::ProjectorScreen),
      0x00ff => Ok(Self::Unknown),
      _ => Err(())
    }
  }
}
impl TlvDecode for TypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct ConfigStatusBitmap: u32 {
    ///Device is operational.
    const Operational = 1 << 0;
    ///
    const OnlineReserved = 1 << 1;
    ///The lift movement is reversed.
    const LiftMovementReversed = 1 << 2;
    ///Supports the PositionAwareLift feature (PA_LF).
    const LiftPositionAware = 1 << 3;
    ///Supports the PositionAwareTilt feature (PA_TL).
    const TiltPositionAware = 1 << 4;
    ///Uses an encoder for lift.
    const LiftEncoderControlled = 1 << 5;
    ///Uses an encoder for tilt.
    const TiltEncoderControlled = 1 << 6;
  }
}
impl AttrType for ConfigStatusBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Operational"),
    (1, "OnlineReserved"),
    (2, "LiftMovementReversed"),
    (3, "LiftPositionAware"),
    (4, "TiltPositionAware"),
    (5, "LiftEncoderControlled"),
    (6, "TiltEncoderControlled"),
  ]);
}
impl TlvPut for ConfigStatusBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for ConfigStatusBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for ConfigStatusBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct ModeBitmap: u32 {
    ///Reverse the lift direction.
    const MotorDirectionReversed = 1 << 0;
    ///Perform a calibration.
    const CalibrationMode = 1 << 1;
    ///Freeze all motions for maintenance.
    const MaintenanceMode = 1 << 2;
    ///Control the LEDs feedback.
    const LedFeedback = 1 << 3;
  }
}
impl AttrType for ModeBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "MotorDirectionReversed"),
    (1, "CalibrationMode"),
    (2, "MaintenanceMode"),
    (3, "LedFeedback"),
  ]);
}
impl TlvPut for ModeBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for ModeBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for ModeBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct OperationalStatusBitmap: u32 {
    ///Global operational state.
    const Global = (0| 0| 1);
    ///Lift operational state.
    const Lift = (0| 2| 3);
    ///Tilt operational state.
    const Tilt = (0| 4| 5);
  }
}
impl AttrType for OperationalStatusBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Global"),
    (1, "Global"),
    (2, "Lift"),
    (3, "Lift"),
    (4, "Tilt"),
    (5, "Tilt"),
  ]);
}
impl TlvPut for OperationalStatusBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for OperationalStatusBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for OperationalStatusBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct SafetyStatusBitmap: u32 {
    ///Movement commands are ignored (locked out). e.g. not granted authorization, outside some time/date range.
    const RemoteLockout = 1 << 0;
    ///Tampering detected on sensors or any other safety equipment. Ex: a device has been forcedly moved without its actuator(s).
    const TamperDetection = 1 << 1;
    ///Communication failure to sensors or other safety equipment.
    const FailedCommunication = 1 << 2;
    ///Device has failed to reach the desired position. e.g. with position aware device, time expired before TargetPosition is reached.
    const PositionFailure = 1 << 3;
    ///Motor(s) and/or electric circuit thermal protection activated.
    const ThermalProtection = 1 << 4;
    ///An obstacle is preventing actuator movement.
    const ObstacleDetected = 1 << 5;
    ///Device has power related issue or limitation e.g. device is running w/ the help of a backup battery or power might not be fully available at the moment.
    const Power = 1 << 6;
    ///Local safety sensor (not a direct obstacle) is preventing movements (e.g. Safety EU Standard EN60335).
    const StopInput = 1 << 7;
    ///Mechanical problem related to the motor(s) detected.
    const MotorJammed = 1 << 8;
    ///PCB, fuse and other electrics problems.
    const HardwareFailure = 1 << 9;
    ///Actuator is manually operated and is preventing actuator movement (e.g. actuator is disengaged/decoupled).
    const ManualOperation = 1 << 10;
    ///Protection is activated.
    const Protection = 1 << 11;
  }
}
impl AttrType for SafetyStatusBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "RemoteLockout"),
    (1, "TamperDetection"),
    (2, "FailedCommunication"),
    (3, "PositionFailure"),
    (4, "ThermalProtection"),
    (5, "ObstacleDetected"),
    (6, "Power"),
    (7, "StopInput"),
    (8, "MotorJammed"),
    (9, "HardwareFailure"),
    (10, "ManualOperation"),
    (11, "Protection"),
  ]);
}
impl TlvPut for SafetyStatusBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for SafetyStatusBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for SafetyStatusBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
}
