#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const BASIC_INFORMATION: ClusterDef = ClusterDef {
  id: 0x0028,
  name: "Basic Information",
  features: &[
  ],
  attributes: &[
    attr::DataModelRevision::ATTR,
    attr::VendorName::ATTR,
    attr::VendorID::ATTR,
    attr::ProductName::ATTR,
    attr::ProductID::ATTR,
    attr::NodeLabel::ATTR,
    attr::Location::ATTR,
    attr::HardwareVersion::ATTR,
    attr::HardwareVersionString::ATTR,
    attr::SoftwareVersion::ATTR,
    attr::SoftwareVersionString::ATTR,
    attr::ManufacturingDate::ATTR,
    attr::PartNumber::ATTR,
    attr::ProductURL::ATTR,
    attr::ProductLabel::ATTR,
    attr::SerialNumber::ATTR,
    attr::LocalConfigDisabled::ATTR,
    attr::Reachable::ATTR,
    attr::UniqueID::ATTR,
    attr::CapabilityMinima::ATTR,
    attr::ProductAppearance::ATTR,
    attr::SpecificationVersion::ATTR,
    attr::MaxPathsPerInvoke::ATTR,
    attr::ConfigurationVersion::ATTR,
  ],
  events: &[
    event::StartUp::EVENT,
    event::ShutDown::EVENT,
    event::Leave::EVENT,
    event::ReachableChanged::EVENT,
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct DataModelRevision;

  impl Attr for DataModelRevision {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "DataModelRevision",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u16;
  }


  pub struct VendorName;

  impl Attr for VendorName {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "VendorName",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = String;
  }


  pub struct VendorID;

  impl Attr for VendorID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "VendorID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = TypeUnsupported /* VendorId */;
  }


  pub struct ProductName;

  impl Attr for ProductName {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "ProductName",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = String;
  }


  pub struct ProductID;

  impl Attr for ProductID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "ProductID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u16;
  }


  pub struct NodeLabel;

  impl Attr for NodeLabel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "NodeLabel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = String;
  }


  pub struct Location;

  impl Attr for Location {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "Location",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = String;
  }


  pub struct HardwareVersion;

  impl Attr for HardwareVersion {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "HardwareVersion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u16;
  }


  pub struct HardwareVersionString;

  impl Attr for HardwareVersionString {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "HardwareVersionString",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = String;
  }


  pub struct SoftwareVersion;

  impl Attr for SoftwareVersion {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "SoftwareVersion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u32;
  }


  pub struct SoftwareVersionString;

  impl Attr for SoftwareVersionString {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "SoftwareVersionString",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = String;
  }


  pub struct ManufacturingDate;

  impl Attr for ManufacturingDate {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "ManufacturingDate",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = String;
  }


  pub struct PartNumber;

  impl Attr for PartNumber {
    const ATTR: AttrDef = AttrDef {
      id: 0x000c,
      name: "PartNumber",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = String;
  }


  pub struct ProductURL;

  impl Attr for ProductURL {
    const ATTR: AttrDef = AttrDef {
      id: 0x000d,
      name: "ProductURL",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = String;
  }


  pub struct ProductLabel;

  impl Attr for ProductLabel {
    const ATTR: AttrDef = AttrDef {
      id: 0x000e,
      name: "ProductLabel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = String;
  }


  pub struct SerialNumber;

  impl Attr for SerialNumber {
    const ATTR: AttrDef = AttrDef {
      id: 0x000f,
      name: "SerialNumber",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = String;
  }


  pub struct LocalConfigDisabled;

  impl Attr for LocalConfigDisabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x0010,
      name: "LocalConfigDisabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct Reachable;

  impl Attr for Reachable {
    const ATTR: AttrDef = AttrDef {
      id: 0x0011,
      name: "Reachable",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct UniqueID;

  impl Attr for UniqueID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0012,
      name: "UniqueID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = String;
  }


  pub struct CapabilityMinima;

  impl Attr for CapabilityMinima {
    const ATTR: AttrDef = AttrDef {
      id: 0x0013,
      name: "CapabilityMinima",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::CapabilityMinimaStruct;
  }


  pub struct ProductAppearance;

  impl Attr for ProductAppearance {
    const ATTR: AttrDef = AttrDef {
      id: 0x0014,
      name: "ProductAppearance",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::ProductAppearanceStruct;
  }


  pub struct SpecificationVersion;

  impl Attr for SpecificationVersion {
    const ATTR: AttrDef = AttrDef {
      id: 0x0015,
      name: "SpecificationVersion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u32;
  }


  pub struct MaxPathsPerInvoke;

  impl Attr for MaxPathsPerInvoke {
    const ATTR: AttrDef = AttrDef {
      id: 0x0016,
      name: "MaxPathsPerInvoke",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u16;
  }


  pub struct ConfigurationVersion;

  impl Attr for ConfigurationVersion {
    const ATTR: AttrDef = AttrDef {
      id: 0x0018,
      name: "ConfigurationVersion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u32;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct StartUp {
  pub software_version: u32,
}

impl Event for StartUp {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "StartUp",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "SoftwareVersion", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for StartUp {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut software_version: Option<u32> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { software_version = Some(<u32>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      software_version: software_version.ok_or(TlvDecodeErr::MissingField("SoftwareVersion"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ShutDown {
}

impl Event for ShutDown {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "ShutDown",
    conformance: Conformance::Optional,
    fields: &[
    ],
  };
}

impl TlvDecode for ShutDown {
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


#[derive(Debug, Clone, PartialEq)]
pub struct Leave {
  pub fabric_index: TypeUnsupported /* FabricIdx */,
}

impl Event for Leave {
  const EVENT: EventDef = EventDef {
    id: 0x0002,
    name: "Leave",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "FabricIndex", attr_ty: <TypeUnsupported /* FabricIdx */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for Leave {
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
      fabric_index: TypeUnsupported,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ReachableChanged {
  pub reachable_new_value: bool,
}

impl Event for ReachableChanged {
  const EVENT: EventDef = EventDef {
    id: 0x0003,
    name: "ReachableChanged",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ReachableNewValue", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for ReachableChanged {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut reachable_new_value: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { reachable_new_value = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      reachable_new_value: reachable_new_value.ok_or(TlvDecodeErr::MissingField("ReachableNewValue"))?,
    })
  }
}


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ColorEnum {
  ///Approximately RGB #000000.
  Black = 0x0000,
  ///Approximately RGB #000080.
  Navy = 0x0001,
  ///Approximately RGB #008000.
  Green = 0x0002,
  ///Approximately RGB #008080.
  Teal = 0x0003,
  ///Approximately RGB #800000.
  Maroon = 0x0004,
  ///Approximately RGB #800080.
  Purple = 0x0005,
  ///Approximately RGB #808000.
  Olive = 0x0006,
  ///Approximately RGB #808080.
  Gray = 0x0007,
  ///Approximately RGB #0000FF.
  Blue = 0x0008,
  ///Approximately RGB #00FF00.
  Lime = 0x0009,
  ///Approximately RGB #00FFFF.
  Aqua = 0x000a,
  ///Approximately RGB #FF0000.
  Red = 0x000b,
  ///Approximately RGB #FF00FF.
  Fuchsia = 0x000c,
  ///Approximately RGB #FFFF00.
  Yellow = 0x000d,
  ///Approximately RGB #FFFFFF.
  White = 0x000e,
  ///Typical hardware "Nickel" color.
  Nickel = 0x000f,
  ///Typical hardware "Chrome" color.
  Chrome = 0x0010,
  ///Typical hardware "Brass" color.
  Brass = 0x0011,
  ///Typical hardware "Copper" color.
  Copper = 0x0012,
  ///Typical hardware "Silver" color.
  Silver = 0x0013,
  ///Typical hardware "Gold" color.
  Gold = 0x0014,
}
impl AttrType for ColorEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Black"),
  (0x0001, "Navy"),
  (0x0002, "Green"),
  (0x0003, "Teal"),
  (0x0004, "Maroon"),
  (0x0005, "Purple"),
  (0x0006, "Olive"),
  (0x0007, "Gray"),
  (0x0008, "Blue"),
  (0x0009, "Lime"),
  (0x000a, "Aqua"),
  (0x000b, "Red"),
  (0x000c, "Fuchsia"),
  (0x000d, "Yellow"),
  (0x000e, "White"),
  (0x000f, "Nickel"),
  (0x0010, "Chrome"),
  (0x0011, "Brass"),
  (0x0012, "Copper"),
  (0x0013, "Silver"),
  (0x0014, "Gold"),
  ]);
}
impl TlvPut for ColorEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ColorEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Black),
      0x0001 => Ok(Self::Navy),
      0x0002 => Ok(Self::Green),
      0x0003 => Ok(Self::Teal),
      0x0004 => Ok(Self::Maroon),
      0x0005 => Ok(Self::Purple),
      0x0006 => Ok(Self::Olive),
      0x0007 => Ok(Self::Gray),
      0x0008 => Ok(Self::Blue),
      0x0009 => Ok(Self::Lime),
      0x000a => Ok(Self::Aqua),
      0x000b => Ok(Self::Red),
      0x000c => Ok(Self::Fuchsia),
      0x000d => Ok(Self::Yellow),
      0x000e => Ok(Self::White),
      0x000f => Ok(Self::Nickel),
      0x0010 => Ok(Self::Chrome),
      0x0011 => Ok(Self::Brass),
      0x0012 => Ok(Self::Copper),
      0x0013 => Ok(Self::Silver),
      0x0014 => Ok(Self::Gold),
      _ => Err(())
    }
  }
}
impl TlvDecode for ColorEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ProductFinishEnum {
  ///Product has some other finish not listed below.
  Other = 0x0000,
  ///Product has a matte finish.
  Matte = 0x0001,
  ///Product has a satin finish.
  Satin = 0x0002,
  ///Product has a polished or shiny finish.
  Polished = 0x0003,
  ///Product has a rugged finish.
  Rugged = 0x0004,
  ///Product has a fabric finish.
  Fabric = 0x0005,
}
impl AttrType for ProductFinishEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Other"),
  (0x0001, "Matte"),
  (0x0002, "Satin"),
  (0x0003, "Polished"),
  (0x0004, "Rugged"),
  (0x0005, "Fabric"),
  ]);
}
impl TlvPut for ProductFinishEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ProductFinishEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Other),
      0x0001 => Ok(Self::Matte),
      0x0002 => Ok(Self::Satin),
      0x0003 => Ok(Self::Polished),
      0x0004 => Ok(Self::Rugged),
      0x0005 => Ok(Self::Fabric),
      _ => Err(())
    }
  }
}
impl TlvDecode for ProductFinishEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct CapabilityMinimaStruct {
  pub case_sessions_per_fabric: u16,
  pub subscriptions_per_fabric: u16,
  pub simultaneous_invocations_supported: u16,
  pub simultaneous_writes_supported: u16,
  pub read_paths_supported: u16,
  pub subscribe_paths_supported: u16,
}
impl AttrType for CapabilityMinimaStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "CaseSessionsPerFabric", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "SubscriptionsPerFabric", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "SimultaneousInvocationsSupported", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "SimultaneousWritesSupported", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "ReadPathsSupported", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "SubscribePathsSupported", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for CapabilityMinimaStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.case_sessions_per_fabric.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.subscriptions_per_fabric.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.simultaneous_invocations_supported.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.simultaneous_writes_supported.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.read_paths_supported.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.subscribe_paths_supported.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for CapabilityMinimaStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut case_sessions_per_fabric: Option<u16> = None;
    let mut subscriptions_per_fabric: Option<u16> = None;
    let mut simultaneous_invocations_supported: Option<u16> = None;
    let mut simultaneous_writes_supported: Option<u16> = None;
    let mut read_paths_supported: Option<u16> = None;
    let mut subscribe_paths_supported: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { case_sessions_per_fabric = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { subscriptions_per_fabric = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { simultaneous_invocations_supported = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { simultaneous_writes_supported = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { read_paths_supported = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { subscribe_paths_supported = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      case_sessions_per_fabric: case_sessions_per_fabric.ok_or(TlvDecodeErr::MissingField("CaseSessionsPerFabric"))?,
      subscriptions_per_fabric: subscriptions_per_fabric.ok_or(TlvDecodeErr::MissingField("SubscriptionsPerFabric"))?,
      simultaneous_invocations_supported: simultaneous_invocations_supported.ok_or(TlvDecodeErr::MissingField("SimultaneousInvocationsSupported"))?,
      simultaneous_writes_supported: simultaneous_writes_supported.ok_or(TlvDecodeErr::MissingField("SimultaneousWritesSupported"))?,
      read_paths_supported: read_paths_supported.ok_or(TlvDecodeErr::MissingField("ReadPathsSupported"))?,
      subscribe_paths_supported: subscribe_paths_supported.ok_or(TlvDecodeErr::MissingField("SubscribePathsSupported"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProductAppearanceStruct {
  pub finish: ProductFinishEnum,
  pub primary_color: ColorEnum,
}
impl AttrType for ProductAppearanceStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Finish", attr_ty: <ProductFinishEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "PrimaryColor", attr_ty: <ColorEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ProductAppearanceStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.finish.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.primary_color.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ProductAppearanceStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut finish: Option<ProductFinishEnum> = None;
    let mut primary_color: Option<ColorEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { finish = Some(<ProductFinishEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { primary_color = Some(<ColorEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      finish: finish.ok_or(TlvDecodeErr::MissingField("Finish"))?,
      primary_color: primary_color.ok_or(TlvDecodeErr::MissingField("PrimaryColor"))?,
    })
  }
}

}
