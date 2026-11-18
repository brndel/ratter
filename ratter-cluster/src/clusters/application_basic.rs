#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const APPLICATION_BASIC: ClusterDef = ClusterDef {
  id: 0x050d,
  name: "Application Basic",
  features: &[
  ],
  attributes: &[
    attr::VendorName::ATTR,
    attr::VendorID::ATTR,
    attr::ApplicationName::ATTR,
    attr::ProductID::ATTR,
    attr::Application::ATTR,
    attr::Status::ATTR,
    attr::ApplicationVersion::ATTR,
    attr::AllowedVendorList::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct VendorName;

  impl Attr for VendorName {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "VendorName",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = String;
  }


  pub struct VendorID;

  impl Attr for VendorID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "VendorID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = TypeUnsupported /* VendorId */;
  }


  pub struct ApplicationName;

  impl Attr for ApplicationName {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "ApplicationName",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = String;
  }


  pub struct ProductID;

  impl Attr for ProductID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "ProductID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct Application;

  impl Attr for Application {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "Application",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::ApplicationStruct;
  }


  pub struct Status;

  impl Attr for Status {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "Status",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::ApplicationStatusEnum;
  }


  pub struct ApplicationVersion;

  impl Attr for ApplicationVersion {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "ApplicationVersion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = String;
  }


  pub struct AllowedVendorList;

  impl Attr for AllowedVendorList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "AllowedVendorList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<TypeUnsupported /* VendorId */>;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ApplicationStatusEnum {
  ///Application is not running.
  Stopped = 0x0000,
  ///Application is running, is visible to the user, and is the active target for input.
  ActiveVisibleFocus = 0x0001,
  ///Application is running but not visible to the user.
  ActiveHidden = 0x0002,
  ///Application is running and visible, but is not the active target for input.
  ActiveVisibleNotFocus = 0x0003,
}
impl AttrType for ApplicationStatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Stopped"),
  (0x0001, "ActiveVisibleFocus"),
  (0x0002, "ActiveHidden"),
  (0x0003, "ActiveVisibleNotFocus"),
  ]);
}
impl TlvPut for ApplicationStatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ApplicationStatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Stopped),
      0x0001 => Ok(Self::ActiveVisibleFocus),
      0x0002 => Ok(Self::ActiveHidden),
      0x0003 => Ok(Self::ActiveVisibleNotFocus),
      _ => Err(())
    }
  }
}
impl TlvDecode for ApplicationStatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationStruct {
  pub catalog_vendor_id: u16,
  pub application_id: String,
}
impl AttrType for ApplicationStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "CatalogVendorID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ApplicationID", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ApplicationStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.catalog_vendor_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.application_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ApplicationStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut catalog_vendor_id: Option<u16> = None;
    let mut application_id: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { catalog_vendor_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { application_id = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      catalog_vendor_id: catalog_vendor_id.ok_or(TlvDecodeErr::MissingField("CatalogVendorID"))?,
      application_id: application_id.ok_or(TlvDecodeErr::MissingField("ApplicationID"))?,
    })
  }
}

}
