#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const SWITCH: ClusterDef = ClusterDef {
  id: 0x003b,
  name: "Switch",
  features: &[
    FeatureDef { bit: 0x0000, code: "LS", name: "LatchingSwitch", summary: "Switch is latching" },
    FeatureDef { bit: 0x0001, code: "MS", name: "MomentarySwitch", summary: "Switch is momentary" },
    FeatureDef { bit: 0x0002, code: "MSR", name: "MomentarySwitchRelease", summary: "Switch supports release events generation" },
    FeatureDef { bit: 0x0003, code: "MSL", name: "MomentarySwitchLongPress", summary: "Switch supports long press detection" },
    FeatureDef { bit: 0x0004, code: "MSM", name: "MomentarySwitchMultiPress", summary: "Switch supports multi-press detection" },
    FeatureDef { bit: 0x0005, code: "AS", name: "ActionSwitch", summary: "Switch is momentary, targeted at specific user actions (focus on multi-press and optionally long press) with a reduced event generation scheme" },
  ],
  attributes: &[
    attr::NumberOfPositions::ATTR,
    attr::CurrentPosition::ATTR,
    attr::MultiPressMax::ATTR,
  ],
  events: &[
    event::SwitchLatched::EVENT,
    event::InitialPress::EVENT,
    event::LongPress::EVENT,
    event::ShortRelease::EVENT,
    event::LongRelease::EVENT,
    event::MultiPressOngoing::EVENT,
    event::MultiPressComplete::EVENT,
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct NumberOfPositions;

  impl Attr for NumberOfPositions {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "NumberOfPositions",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct CurrentPosition;

  impl Attr for CurrentPosition {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "CurrentPosition",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct MultiPressMax;

  impl Attr for MultiPressMax {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "MultiPressMax",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = u8;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SwitchLatched {
  pub new_position: u8,
}

impl Event for SwitchLatched {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "SwitchLatched",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "NewPosition", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for SwitchLatched {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut new_position: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { new_position = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      new_position: new_position.ok_or(TlvDecodeErr::MissingField("NewPosition"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct InitialPress {
  pub new_position: u8,
}

impl Event for InitialPress {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "InitialPress",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "NewPosition", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for InitialPress {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut new_position: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { new_position = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      new_position: new_position.ok_or(TlvDecodeErr::MissingField("NewPosition"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct LongPress {
  pub new_position: u8,
}

impl Event for LongPress {
  const EVENT: EventDef = EventDef {
    id: 0x0002,
    name: "LongPress",
    conformance: Conformance::Feature(3),
    fields: &[
      AttrDef { id: 0x0000, name: "NewPosition", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for LongPress {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut new_position: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { new_position = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      new_position: new_position.ok_or(TlvDecodeErr::MissingField("NewPosition"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ShortRelease {
  pub previous_position: u8,
}

impl Event for ShortRelease {
  const EVENT: EventDef = EventDef {
    id: 0x0003,
    name: "ShortRelease",
    conformance: Conformance::Feature(2),
    fields: &[
      AttrDef { id: 0x0000, name: "PreviousPosition", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for ShortRelease {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut previous_position: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { previous_position = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      previous_position: previous_position.ok_or(TlvDecodeErr::MissingField("PreviousPosition"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct LongRelease {
  pub previous_position: u8,
}

impl Event for LongRelease {
  const EVENT: EventDef = EventDef {
    id: 0x0004,
    name: "LongRelease",
    conformance: Conformance::Feature(3),
    fields: &[
      AttrDef { id: 0x0000, name: "PreviousPosition", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for LongRelease {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut previous_position: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { previous_position = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      previous_position: previous_position.ok_or(TlvDecodeErr::MissingField("PreviousPosition"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct MultiPressOngoing {
  pub new_position: u8,
  pub current_number_of_presses_counted: u8,
}

impl Event for MultiPressOngoing {
  const EVENT: EventDef = EventDef {
    id: 0x0005,
    name: "MultiPressOngoing",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NewPosition", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "CurrentNumberOfPressesCounted", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for MultiPressOngoing {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut new_position: Option<u8> = None;
    let mut current_number_of_presses_counted: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { new_position = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { current_number_of_presses_counted = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      new_position: new_position.ok_or(TlvDecodeErr::MissingField("NewPosition"))?,
      current_number_of_presses_counted: current_number_of_presses_counted.ok_or(TlvDecodeErr::MissingField("CurrentNumberOfPressesCounted"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct MultiPressComplete {
  pub previous_position: u8,
  pub total_number_of_presses_counted: u8,
}

impl Event for MultiPressComplete {
  const EVENT: EventDef = EventDef {
    id: 0x0006,
    name: "MultiPressComplete",
    conformance: Conformance::Feature(4),
    fields: &[
      AttrDef { id: 0x0000, name: "PreviousPosition", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "TotalNumberOfPressesCounted", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for MultiPressComplete {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut previous_position: Option<u8> = None;
    let mut total_number_of_presses_counted: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { previous_position = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { total_number_of_presses_counted = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      previous_position: previous_position.ok_or(TlvDecodeErr::MissingField("PreviousPosition"))?,
      total_number_of_presses_counted: total_number_of_presses_counted.ok_or(TlvDecodeErr::MissingField("TotalNumberOfPressesCounted"))?,
    })
  }
}


}
pub mod ty {
  pub use crate::globals::*;
}
