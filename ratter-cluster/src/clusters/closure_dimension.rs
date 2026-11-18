#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const CLOSURE_DIMENSION: ClusterDef = ClusterDef {
  id: 0x0105,
  name: "Closure Dimension",
  features: &[
    FeatureDef { bit: 0x0000, code: "PS", name: "Positioning", summary: "Supports Positioning in the range from 0.00% to 100.00%" },
    FeatureDef { bit: 0x0001, code: "LT", name: "MotionLatching", summary: "Supports a latch to secure the closure to a position or state" },
    FeatureDef { bit: 0x0002, code: "UT", name: "Unit", summary: "Specifies the relevant unit and range for this dimension (mm, degrees etc.)" },
    FeatureDef { bit: 0x0003, code: "LM", name: "Limitation", summary: "Supports limitation of the operating range" },
    FeatureDef { bit: 0x0004, code: "SP", name: "Speed", summary: "Supports speed motion throttling." },
    FeatureDef { bit: 0x0005, code: "TR", name: "Translation", summary: "Drives a translation motion" },
    FeatureDef { bit: 0x0006, code: "RO", name: "Rotation", summary: "Drives a rotation motion" },
    FeatureDef { bit: 0x0007, code: "MD", name: "Modulation", summary: "Modulates a particular flow level (light, air, privacy ...)" },
  ],
  attributes: &[
    attr::CurrentState::ATTR,
    attr::TargetState::ATTR,
    attr::Resolution::ATTR,
    attr::StepValue::ATTR,
    attr::Unit::ATTR,
    attr::UnitRange::ATTR,
    attr::LimitRange::ATTR,
    attr::TranslationDirection::ATTR,
    attr::RotationAxis::ATTR,
    attr::Overflow::ATTR,
    attr::ModulationType::ATTR,
    attr::LatchControlModes::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::SetTarget::COMMAND,
    command::Step::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct CurrentState;

  impl Attr for CurrentState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "CurrentState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::DimensionStateStruct>;
  }


  pub struct TargetState;

  impl Attr for TargetState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "TargetState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::DimensionStateStruct>;
  }


  pub struct Resolution;

  impl Attr for Resolution {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "Resolution",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = TypeUnsupported /* Percent100ths */;
  }


  pub struct StepValue;

  impl Attr for StepValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "StepValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = TypeUnsupported /* Percent100ths */;
  }


  pub struct Unit;

  impl Attr for Unit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "Unit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = ty::ClosureUnitEnum;
  }


  pub struct UnitRange;

  impl Attr for UnitRange {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "UnitRange",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = Nullable<ty::UnitRangeStruct>;
  }


  pub struct LimitRange;

  impl Attr for LimitRange {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "LimitRange",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = ty::RangePercent100thsStruct;
  }


  pub struct TranslationDirection;

  impl Attr for TranslationDirection {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "TranslationDirection",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(5),
    };
    type Value = ty::TranslationDirectionEnum;
  }


  pub struct RotationAxis;

  impl Attr for RotationAxis {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "RotationAxis",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(6),
    };
    type Value = ty::RotationAxisEnum;
  }


  pub struct Overflow;

  impl Attr for Overflow {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "Overflow",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(6),
    };
    type Value = ty::OverflowEnum;
  }


  pub struct ModulationType;

  impl Attr for ModulationType {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "ModulationType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(7),
    };
    type Value = ty::ModulationTypeEnum;
  }


  pub struct LatchControlModes;

  impl Attr for LatchControlModes {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "LatchControlModes",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = ty::LatchControlModesBitmap;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SetTarget {
  pub position: TypeUnsupported /* Percent100ths */,
  pub latch: bool,
  pub speed: ty::ThreeLevelAutoEnum,
}
impl Command for SetTarget {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "SetTarget",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Position", attr_ty: <TypeUnsupported /* Percent100ths */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0001, name: "Latch", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "Speed", attr_ty: <ty::ThreeLevelAutoEnum>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for SetTarget {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetTarget {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.position.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.latch.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.speed.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Step {
  pub direction: ty::StepDirectionEnum,
  pub number_of_steps: u16,
  pub speed: ty::ThreeLevelAutoEnum,
}
impl Command for Step {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "Step",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "Direction", attr_ty: <ty::StepDirectionEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "NumberOfSteps", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Speed", attr_ty: <ty::ThreeLevelAutoEnum>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for Step {
  type Response = BasicAccResponse;
}
impl TlvEncode for Step {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.direction.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.number_of_steps.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.speed.tlv_put(matter_codec::Tag::Context(0x0002), writer);
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
pub enum ClosureUnitEnum {
  ///Millimeter used as unit
  Millimeter = 0x0000,
  ///Degree used as unit
  Degree = 0x0001,
}
impl AttrType for ClosureUnitEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Millimeter"),
  (0x0001, "Degree"),
  ]);
}
impl TlvPut for ClosureUnitEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ClosureUnitEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Millimeter),
      0x0001 => Ok(Self::Degree),
      _ => Err(())
    }
  }
}
impl TlvDecode for ClosureUnitEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ModulationTypeEnum {
  ///Orientation of the slats
  SlatsOrientation = 0x0000,
  ///Aperture of the slats
  SlatsOpenwork = 0x0001,
  ///Alignment of blind stripes (Zebra)
  StripesAlignment = 0x0002,
  ///Opacity of a surface
  Opacity = 0x0003,
  ///Ventilation control
  Ventilation = 0x0004,
}
impl AttrType for ModulationTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "SlatsOrientation"),
  (0x0001, "SlatsOpenwork"),
  (0x0002, "StripesAlignment"),
  (0x0003, "Opacity"),
  (0x0004, "Ventilation"),
  ]);
}
impl TlvPut for ModulationTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ModulationTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::SlatsOrientation),
      0x0001 => Ok(Self::SlatsOpenwork),
      0x0002 => Ok(Self::StripesAlignment),
      0x0003 => Ok(Self::Opacity),
      0x0004 => Ok(Self::Ventilation),
      _ => Err(())
    }
  }
}
impl TlvDecode for ModulationTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum OverflowEnum {
  ///No overflow
  NoOverflow = 0x0000,
  ///Inside overflow
  Inside = 0x0001,
  ///Outside overflow
  Outside = 0x0002,
  ///Top inside overflow
  TopInside = 0x0003,
  ///Top outside overflow
  TopOutside = 0x0004,
  ///Bottom inside overflow
  BottomInside = 0x0005,
  ///Bottom outside overflow
  BottomOutside = 0x0006,
  ///Left inside overflow
  LeftInside = 0x0007,
  ///Left outside overflow
  LeftOutside = 0x0008,
  ///Right inside overflow
  RightInside = 0x0009,
  ///Right outside overflow
  RightOutside = 0x000a,
}
impl AttrType for OverflowEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "NoOverflow"),
  (0x0001, "Inside"),
  (0x0002, "Outside"),
  (0x0003, "TopInside"),
  (0x0004, "TopOutside"),
  (0x0005, "BottomInside"),
  (0x0006, "BottomOutside"),
  (0x0007, "LeftInside"),
  (0x0008, "LeftOutside"),
  (0x0009, "RightInside"),
  (0x000a, "RightOutside"),
  ]);
}
impl TlvPut for OverflowEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for OverflowEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::NoOverflow),
      0x0001 => Ok(Self::Inside),
      0x0002 => Ok(Self::Outside),
      0x0003 => Ok(Self::TopInside),
      0x0004 => Ok(Self::TopOutside),
      0x0005 => Ok(Self::BottomInside),
      0x0006 => Ok(Self::BottomOutside),
      0x0007 => Ok(Self::LeftInside),
      0x0008 => Ok(Self::LeftOutside),
      0x0009 => Ok(Self::RightInside),
      0x000a => Ok(Self::RightOutside),
      _ => Err(())
    }
  }
}
impl TlvDecode for OverflowEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum RotationAxisEnum {
  ///The panel rotates around a vertical axis located on the left side of the panel
  Left = 0x0000,
  ///The panel rotates around a vertical axis located in the center of the panel
  CenteredVertical = 0x0001,
  ///The panels rotates around vertical axes located on the left and right sides of the panel
  LeftAndRight = 0x0002,
  ///The panel rotates around a vertical axis located on the right side of the panel
  Right = 0x0003,
  ///The panel rotates around a horizontal axis located on the top of the panel
  Top = 0x0004,
  ///The panel rotates around a horizontal axis located in the center of the panel
  CenteredHorizontal = 0x0005,
  ///The panels rotates around horizontal axes located on the top and bottom of the panel
  TopAndBottom = 0x0006,
  ///The panel rotates around a horizontal axis located on the bottom of the panel
  Bottom = 0x0007,
  ///The barrier tilts around an axis located at the left end of the barrier
  LeftBarrier = 0x0008,
  ///The dual barriers tilt around axes located at each side of the composite barrier
  LeftAndRightBarriers = 0x0009,
  ///The barrier tilts around an axis located at the right end of the barrier
  RightBarrier = 0x000a,
}
impl AttrType for RotationAxisEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Left"),
  (0x0001, "CenteredVertical"),
  (0x0002, "LeftAndRight"),
  (0x0003, "Right"),
  (0x0004, "Top"),
  (0x0005, "CenteredHorizontal"),
  (0x0006, "TopAndBottom"),
  (0x0007, "Bottom"),
  (0x0008, "LeftBarrier"),
  (0x0009, "LeftAndRightBarriers"),
  (0x000a, "RightBarrier"),
  ]);
}
impl TlvPut for RotationAxisEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for RotationAxisEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Left),
      0x0001 => Ok(Self::CenteredVertical),
      0x0002 => Ok(Self::LeftAndRight),
      0x0003 => Ok(Self::Right),
      0x0004 => Ok(Self::Top),
      0x0005 => Ok(Self::CenteredHorizontal),
      0x0006 => Ok(Self::TopAndBottom),
      0x0007 => Ok(Self::Bottom),
      0x0008 => Ok(Self::LeftBarrier),
      0x0009 => Ok(Self::LeftAndRightBarriers),
      0x000a => Ok(Self::RightBarrier),
      _ => Err(())
    }
  }
}
impl TlvDecode for RotationAxisEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StepDirectionEnum {
  ///Decrease towards 0.00%
  Decrease = 0x0000,
  ///Increase towards 100.00%
  Increase = 0x0001,
}
impl AttrType for StepDirectionEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Decrease"),
  (0x0001, "Increase"),
  ]);
}
impl TlvPut for StepDirectionEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StepDirectionEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Decrease),
      0x0001 => Ok(Self::Increase),
      _ => Err(())
    }
  }
}
impl TlvDecode for StepDirectionEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TranslationDirectionEnum {
  ///Downward translation
  Downward = 0x0000,
  ///Upward translation
  Upward = 0x0001,
  ///Vertical mask translation
  VerticalMask = 0x0002,
  ///Vertical symmetry translation
  VerticalSymmetry = 0x0003,
  ///Leftward translation
  Leftward = 0x0004,
  ///Rightward translation
  Rightward = 0x0005,
  ///Horizontal mask translation
  HorizontalMask = 0x0006,
  ///Horizontal symmetry translation
  HorizontalSymmetry = 0x0007,
  ///Forward translation
  Forward = 0x0008,
  ///Backward translation
  Backward = 0x0009,
  ///Depth mask translation
  DepthMask = 0x000a,
  ///Depth symmetry translation
  DepthSymmetry = 0x000b,
}
impl AttrType for TranslationDirectionEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Downward"),
  (0x0001, "Upward"),
  (0x0002, "VerticalMask"),
  (0x0003, "VerticalSymmetry"),
  (0x0004, "Leftward"),
  (0x0005, "Rightward"),
  (0x0006, "HorizontalMask"),
  (0x0007, "HorizontalSymmetry"),
  (0x0008, "Forward"),
  (0x0009, "Backward"),
  (0x000a, "DepthMask"),
  (0x000b, "DepthSymmetry"),
  ]);
}
impl TlvPut for TranslationDirectionEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TranslationDirectionEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Downward),
      0x0001 => Ok(Self::Upward),
      0x0002 => Ok(Self::VerticalMask),
      0x0003 => Ok(Self::VerticalSymmetry),
      0x0004 => Ok(Self::Leftward),
      0x0005 => Ok(Self::Rightward),
      0x0006 => Ok(Self::HorizontalMask),
      0x0007 => Ok(Self::HorizontalSymmetry),
      0x0008 => Ok(Self::Forward),
      0x0009 => Ok(Self::Backward),
      0x000a => Ok(Self::DepthMask),
      0x000b => Ok(Self::DepthSymmetry),
      _ => Err(())
    }
  }
}
impl TlvDecode for TranslationDirectionEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct LatchControlModesBitmap: u32 {
    ///Remote latching capability
    const RemoteLatching = 1 << 0;
    ///Remote unlatching capability
    const RemoteUnlatching = 1 << 1;
  }
}
impl AttrType for LatchControlModesBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "RemoteLatching"),
    (1, "RemoteUnlatching"),
  ]);
}
impl TlvPut for LatchControlModesBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for LatchControlModesBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for LatchControlModesBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DimensionStateStruct {
  pub position: TypeUnsupported /* Percent100ths */,
  pub latch: bool,
  pub speed: ThreeLevelAutoEnum,
}
impl AttrType for DimensionStateStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Position", attr_ty: <TypeUnsupported /* Percent100ths */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Latch", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Speed", attr_ty: <ThreeLevelAutoEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DimensionStateStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.position.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.latch.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.speed.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DimensionStateStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut latch: Option<bool> = None;
    let mut speed: Option<ThreeLevelAutoEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { latch = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { speed = Some(<ThreeLevelAutoEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      position: TypeUnsupported,
      latch: latch.ok_or(TlvDecodeErr::MissingField("Latch"))?,
      speed: speed.ok_or(TlvDecodeErr::MissingField("Speed"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RangePercent100thsStruct {
  pub min: TypeUnsupported /* Percent100ths */,
  pub max: TypeUnsupported /* Percent100ths */,
}
impl AttrType for RangePercent100thsStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Min", attr_ty: <TypeUnsupported /* Percent100ths */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Max", attr_ty: <TypeUnsupported /* Percent100ths */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for RangePercent100thsStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.min.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.max.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for RangePercent100thsStruct {
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
      min: TypeUnsupported,
      max: TypeUnsupported,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnitRangeStruct {
  pub min: i16,
  pub max: i16,
}
impl AttrType for UnitRangeStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Min", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Max", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for UnitRangeStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.min.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.max.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for UnitRangeStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut min: Option<i16> = None;
    let mut max: Option<i16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { min = Some(<i16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { max = Some(<i16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      min: min.ok_or(TlvDecodeErr::MissingField("Min"))?,
      max: max.ok_or(TlvDecodeErr::MissingField("Max"))?,
    })
  }
}

}
