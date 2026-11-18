#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const SOFTWARE_DIAGNOSTICS: ClusterDef = ClusterDef {
  id: 0x0034,
  name: "Software Diagnostics",
  features: &[
    FeatureDef { bit: 0x0000, code: "WTRMRK", name: "Watermarks", summary: "Node makes available the metrics for high watermark related to memory consumption" },
  ],
  attributes: &[
    attr::ThreadMetrics::ATTR,
    attr::CurrentHeapFree::ATTR,
    attr::CurrentHeapUsed::ATTR,
    attr::CurrentHeapHighWatermark::ATTR,
  ],
  events: &[
    event::SoftwareFault::EVENT,
  ],
  commands: &[
    command::ResetWatermarks::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct ThreadMetrics;

  impl Attr for ThreadMetrics {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "ThreadMetrics",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Vec<ty::ThreadMetricsStruct>;
  }


  pub struct CurrentHeapFree;

  impl Attr for CurrentHeapFree {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "CurrentHeapFree",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u64;
  }


  pub struct CurrentHeapUsed;

  impl Attr for CurrentHeapUsed {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "CurrentHeapUsed",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u64;
  }


  pub struct CurrentHeapHighWatermark;

  impl Attr for CurrentHeapHighWatermark {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "CurrentHeapHighWatermark",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u64;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SoftwareFault {
  pub id: u64,
  pub name: String,
  pub fault_recording: TypeUnsupported /* Octstr */,
}

impl Event for SoftwareFault {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "SoftwareFault",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "ID", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "FaultRecording", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}

impl TlvDecode for SoftwareFault {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut id: Option<u64> = None;
    let mut name: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { id = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      id: id.ok_or(TlvDecodeErr::MissingField("ID"))?,
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
      fault_recording: TypeUnsupported,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ResetWatermarks {
}
impl Command for ResetWatermarks {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "ResetWatermarks",
    conformance: Conformance::Feature(0),
    fields: &[
    ],
  };
}
impl CommandInvoke for ResetWatermarks {
  type Response = BasicAccResponse;
}
impl TlvEncode for ResetWatermarks {
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
pub struct ThreadMetricsStruct {
  pub id: u64,
  pub name: String,
  pub stack_free_current: u32,
  pub stack_free_minimum: u32,
  pub stack_size: u32,
}
impl AttrType for ThreadMetricsStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ID", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "StackFreeCurrent", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "StackFreeMinimum", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "StackSize", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ThreadMetricsStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.stack_free_current.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.stack_free_minimum.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.stack_size.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ThreadMetricsStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut id: Option<u64> = None;
    let mut name: Option<String> = None;
    let mut stack_free_current: Option<u32> = None;
    let mut stack_free_minimum: Option<u32> = None;
    let mut stack_size: Option<u32> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { id = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { stack_free_current = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { stack_free_minimum = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { stack_size = Some(<u32>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      id: id.ok_or(TlvDecodeErr::MissingField("ID"))?,
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
      stack_free_current: stack_free_current.ok_or(TlvDecodeErr::MissingField("StackFreeCurrent"))?,
      stack_free_minimum: stack_free_minimum.ok_or(TlvDecodeErr::MissingField("StackFreeMinimum"))?,
      stack_size: stack_size.ok_or(TlvDecodeErr::MissingField("StackSize"))?,
    })
  }
}

}
