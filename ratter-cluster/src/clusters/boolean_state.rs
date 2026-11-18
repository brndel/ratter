#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const BOOLEAN_STATE: ClusterDef = ClusterDef {
  id: 0x0045,
  name: "Boolean State",
  features: &[
    FeatureDef { bit: 0x0000, code: "CHGEVENT", name: "ChangeEvent", summary: "Supports reporting change events" },
  ],
  attributes: &[
    attr::StateValue::ATTR,
  ],
  events: &[
    event::StateChange::EVENT,
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct StateValue;

  impl Attr for StateValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "StateValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct StateChange {
  pub state_value: bool,
}

impl Event for StateChange {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "StateChange",
    conformance: Conformance::TooComplicated,
    fields: &[
      AttrDef { id: 0x0000, name: "StateValue", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for StateChange {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut state_value: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { state_value = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      state_value: state_value.ok_or(TlvDecodeErr::MissingField("StateValue"))?,
    })
  }
}


}
pub mod ty {
  pub use crate::globals::*;
}
