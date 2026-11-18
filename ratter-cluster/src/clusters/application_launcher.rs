#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const APPLICATION_LAUNCHER: ClusterDef = ClusterDef {
  id: 0x050c,
  name: "Application Launcher",
  features: &[
    FeatureDef { bit: 0x0000, code: "AP", name: "ApplicationPlatform", summary: "Support for attributes and commands required for endpoint to support launching any application within the supported application catalogs" },
  ],
  attributes: &[
    attr::CatalogList::ATTR,
    attr::CurrentApp::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::LaunchApp::COMMAND,
    command::StopApp::COMMAND,
    command::HideApp::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct CatalogList;

  impl Attr for CatalogList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "CatalogList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<u16>;
  }


  pub struct CurrentApp;

  impl Attr for CurrentApp {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "CurrentApp",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<ty::ApplicationEPStruct>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct LaunchApp {
  pub application: ty::ApplicationStruct,
  pub data: TypeUnsupported /* Octstr */,
}
impl Command for LaunchApp {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "LaunchApp",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Application", attr_ty: <ty::ApplicationStruct>::ATTR_TYPE, conformance: Conformance::Feature(0) },
      AttrDef { id: 0x0001, name: "Data", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for LaunchApp {
  type Response = LauncherResponse;
}
impl TlvEncode for LaunchApp {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.application.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.data.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StopApp {
  pub application: ty::ApplicationStruct,
}
impl Command for StopApp {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "StopApp",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Application", attr_ty: <ty::ApplicationStruct>::ATTR_TYPE, conformance: Conformance::Feature(0) },
    ],
  };
}
impl CommandInvoke for StopApp {
  type Response = LauncherResponse;
}
impl TlvEncode for StopApp {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.application.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HideApp {
  pub application: ty::ApplicationStruct,
}
impl Command for HideApp {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "HideApp",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Application", attr_ty: <ty::ApplicationStruct>::ATTR_TYPE, conformance: Conformance::Feature(0) },
    ],
  };
}
impl CommandInvoke for HideApp {
  type Response = LauncherResponse;
}
impl TlvEncode for HideApp {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.application.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LauncherResponse {
  pub status: ty::StatusEnum,
  pub data: TypeUnsupported /* Octstr */,
}
impl Command for LauncherResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "LauncherResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <ty::StatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Data", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandResponse for LauncherResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StatusEnum {
  ///Command succeeded
  Success = 0x0000,
  ///Requested app is not available
  AppNotAvailable = 0x0001,
  ///Video platform unable to honor command
  SystemBusy = 0x0002,
  ///User approval for app download is pending
  PendingUserApproval = 0x0003,
  ///Downloading the requested app
  Downloading = 0x0004,
  ///Installing the requested app
  Installing = 0x0005,
}
impl AttrType for StatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Success"),
  (0x0001, "AppNotAvailable"),
  (0x0002, "SystemBusy"),
  (0x0003, "PendingUserApproval"),
  (0x0004, "Downloading"),
  (0x0005, "Installing"),
  ]);
}
impl TlvPut for StatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Success),
      0x0001 => Ok(Self::AppNotAvailable),
      0x0002 => Ok(Self::SystemBusy),
      0x0003 => Ok(Self::PendingUserApproval),
      0x0004 => Ok(Self::Downloading),
      0x0005 => Ok(Self::Installing),
      _ => Err(())
    }
  }
}
impl TlvDecode for StatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ApplicationEPStruct {
  pub application: ApplicationStruct,
  pub endpoint: TypeUnsupported /* EndpointNo */,
}
impl AttrType for ApplicationEPStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Application", attr_ty: <ApplicationStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Endpoint", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ApplicationEPStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.application.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.endpoint.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ApplicationEPStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut application: Option<ApplicationStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { application = Some(<ApplicationStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      application: application.ok_or(TlvDecodeErr::MissingField("Application"))?,
      endpoint: TypeUnsupported,
    })
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
