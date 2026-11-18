#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const ECOSYSTEM_INFORMATION: ClusterDef = ClusterDef {
  id: 0x0750,
  name: "Ecosystem Information",
  features: &[
  ],
  attributes: &[
    attr::DeviceDirectory::ATTR,
    attr::LocationDirectory::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct DeviceDirectory;

  impl Attr for DeviceDirectory {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "DeviceDirectory",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::EcosystemDeviceStruct>;
  }


  pub struct LocationDirectory;

  impl Attr for LocationDirectory {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "LocationDirectory",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::EcosystemLocationStruct>;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceTypeStruct {
  pub device_type: TypeUnsupported /* DevtypeId */,
  pub revision: u16,
}
impl AttrType for DeviceTypeStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "DeviceType", attr_ty: <TypeUnsupported /* DevtypeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Revision", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DeviceTypeStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.device_type.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.revision.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DeviceTypeStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut revision: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { revision = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      device_type: TypeUnsupported,
      revision: revision.ok_or(TlvDecodeErr::MissingField("Revision"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EcosystemDeviceStruct {
  pub device_name: String,
  pub device_name_last_edit: TypeUnsupported /* EpochUs */,
  pub bridged_endpoint: TypeUnsupported /* EndpointNo */,
  pub original_endpoint: TypeUnsupported /* EndpointNo */,
  pub device_types: Vec<DeviceTypeStruct>,
  pub unique_location_i_ds: Vec<String>,
  pub unique_location_i_ds_last_edit: TypeUnsupported /* EpochUs */,
}
impl AttrType for EcosystemDeviceStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "DeviceName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "DeviceNameLastEdit", attr_ty: <TypeUnsupported /* EpochUs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "BridgedEndpoint", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "OriginalEndpoint", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "DeviceTypes", attr_ty: <Vec<DeviceTypeStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "UniqueLocationIDs", attr_ty: <Vec<String>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "UniqueLocationIDsLastEdit", attr_ty: <TypeUnsupported /* EpochUs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for EcosystemDeviceStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.device_name.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.device_name_last_edit.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.bridged_endpoint.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.original_endpoint.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.device_types.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.unique_location_i_ds.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.unique_location_i_ds_last_edit.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for EcosystemDeviceStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut device_name: Option<String> = None;
    let mut device_types: Option<Vec<DeviceTypeStruct>> = None;
    let mut unique_location_i_ds: Option<Vec<String>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { device_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0004), kind: _ }) => { device_types = Some(<Vec<DeviceTypeStruct>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0005), kind: _ }) => { unique_location_i_ds = Some(<Vec<String>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      device_name: device_name.ok_or(TlvDecodeErr::MissingField("DeviceName"))?,
      device_name_last_edit: TypeUnsupported,
      bridged_endpoint: TypeUnsupported,
      original_endpoint: TypeUnsupported,
      device_types: device_types.ok_or(TlvDecodeErr::MissingField("DeviceTypes"))?,
      unique_location_i_ds: unique_location_i_ds.ok_or(TlvDecodeErr::MissingField("UniqueLocationIDs"))?,
      unique_location_i_ds_last_edit: TypeUnsupported,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EcosystemLocationStruct {
  pub unique_location_id: String,
  pub location_descriptor: LocationDescriptorStruct,
  pub location_descriptor_last_edit: TypeUnsupported /* EpochUs */,
}
impl AttrType for EcosystemLocationStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "UniqueLocationID", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "LocationDescriptor", attr_ty: <LocationDescriptorStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "LocationDescriptorLastEdit", attr_ty: <TypeUnsupported /* EpochUs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for EcosystemLocationStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.unique_location_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.location_descriptor.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.location_descriptor_last_edit.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for EcosystemLocationStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut unique_location_id: Option<String> = None;
    let mut location_descriptor: Option<LocationDescriptorStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { unique_location_id = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { location_descriptor = Some(<LocationDescriptorStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      unique_location_id: unique_location_id.ok_or(TlvDecodeErr::MissingField("UniqueLocationID"))?,
      location_descriptor: location_descriptor.ok_or(TlvDecodeErr::MissingField("LocationDescriptor"))?,
      location_descriptor_last_edit: TypeUnsupported,
    })
  }
}

}
