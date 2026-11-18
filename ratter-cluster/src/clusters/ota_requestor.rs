#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const OTA_SOFTWARE_UPDATE_REQUESTOR: ClusterDef = ClusterDef {
  id: 0x002a,
  name: "OTA Software Update Requestor",
  features: &[
  ],
  attributes: &[
    attr::DefaultOTAProviders::ATTR,
    attr::UpdatePossible::ATTR,
    attr::UpdateState::ATTR,
    attr::UpdateStateProgress::ATTR,
  ],
  events: &[
    event::StateTransition::EVENT,
    event::VersionApplied::EVENT,
    event::DownloadError::EVENT,
  ],
  commands: &[
    command::AnnounceOTAProvider::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct DefaultOTAProviders;

  impl Attr for DefaultOTAProviders {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "DefaultOTAProviders",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::ProviderLocation>;
  }


  pub struct UpdatePossible;

  impl Attr for UpdatePossible {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "UpdatePossible",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


  pub struct UpdateState;

  impl Attr for UpdateState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "UpdateState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::UpdateStateEnum;
  }


  pub struct UpdateStateProgress;

  impl Attr for UpdateStateProgress {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "UpdateStateProgress",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u8>;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct StateTransition {
  pub previous_state: ty::UpdateStateEnum,
  pub new_state: ty::UpdateStateEnum,
  pub reason: ty::ChangeReasonEnum,
  pub target_software_version: u32,
}

impl Event for StateTransition {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "StateTransition",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "PreviousState", attr_ty: <ty::UpdateStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "NewState", attr_ty: <ty::UpdateStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Reason", attr_ty: <ty::ChangeReasonEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "TargetSoftwareVersion", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for StateTransition {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut previous_state: Option<ty::UpdateStateEnum> = None;
    let mut new_state: Option<ty::UpdateStateEnum> = None;
    let mut reason: Option<ty::ChangeReasonEnum> = None;
    let mut target_software_version: Option<u32> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { previous_state = Some(<ty::UpdateStateEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { new_state = Some(<ty::UpdateStateEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { reason = Some(<ty::ChangeReasonEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { target_software_version = Some(<u32>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      previous_state: previous_state.ok_or(TlvDecodeErr::MissingField("PreviousState"))?,
      new_state: new_state.ok_or(TlvDecodeErr::MissingField("NewState"))?,
      reason: reason.ok_or(TlvDecodeErr::MissingField("Reason"))?,
      target_software_version: target_software_version.ok_or(TlvDecodeErr::MissingField("TargetSoftwareVersion"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct VersionApplied {
  pub software_version: u32,
  pub product_id: u16,
}

impl Event for VersionApplied {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "VersionApplied",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "SoftwareVersion", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ProductID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for VersionApplied {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut software_version: Option<u32> = None;
    let mut product_id: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { software_version = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { product_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      software_version: software_version.ok_or(TlvDecodeErr::MissingField("SoftwareVersion"))?,
      product_id: product_id.ok_or(TlvDecodeErr::MissingField("ProductID"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct DownloadError {
  pub software_version: u32,
  pub bytes_downloaded: u64,
  pub progress_percent: u8,
  pub platform_code: i64,
}

impl Event for DownloadError {
  const EVENT: EventDef = EventDef {
    id: 0x0002,
    name: "DownloadError",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "SoftwareVersion", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "BytesDownloaded", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "ProgressPercent", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "PlatformCode", attr_ty: <i64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for DownloadError {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut software_version: Option<u32> = None;
    let mut bytes_downloaded: Option<u64> = None;
    let mut progress_percent: Option<u8> = None;
    let mut platform_code: Option<i64> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { software_version = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { bytes_downloaded = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { progress_percent = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { platform_code = Some(<i64>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      software_version: software_version.ok_or(TlvDecodeErr::MissingField("SoftwareVersion"))?,
      bytes_downloaded: bytes_downloaded.ok_or(TlvDecodeErr::MissingField("BytesDownloaded"))?,
      progress_percent: progress_percent.ok_or(TlvDecodeErr::MissingField("ProgressPercent"))?,
      platform_code: platform_code.ok_or(TlvDecodeErr::MissingField("PlatformCode"))?,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct AnnounceOTAProvider {
  pub provider_node_id: TypeUnsupported /* NodeId */,
  pub vendor_id: TypeUnsupported /* VendorId */,
  pub announcement_reason: ty::AnnouncementReasonEnum,
  pub metadata_for_node: TypeUnsupported /* Octstr */,
  pub endpoint: TypeUnsupported /* EndpointNo */,
}
impl Command for AnnounceOTAProvider {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "AnnounceOTAProvider",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "ProviderNodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "VendorID", attr_ty: <TypeUnsupported /* VendorId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "AnnouncementReason", attr_ty: <ty::AnnouncementReasonEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "MetadataForNode", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0004, name: "Endpoint", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AnnounceOTAProvider {
  type Response = BasicAccResponse;
}
impl TlvEncode for AnnounceOTAProvider {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.provider_node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.vendor_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.announcement_reason.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.metadata_for_node.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.endpoint.tlv_put(matter_codec::Tag::Context(0x0004), writer);
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
pub enum AnnouncementReasonEnum {
  ///An OTA Provider is announcing its presence.
  SimpleAnnouncement = 0x0000,
  ///An OTA Provider is announcing, either to a single Node or to a group of Nodes, that a new Software Image MAY be available
  UpdateAvailable = 0x0001,
  ///An OTA Provider is announcing, either to a single Node or to a group of Nodes, that a new Software Image MAY be available, which contains an update that needs to be applied urgently.
  UrgentUpdateAvailable = 0x0002,
}
impl AttrType for AnnouncementReasonEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "SimpleAnnouncement"),
  (0x0001, "UpdateAvailable"),
  (0x0002, "UrgentUpdateAvailable"),
  ]);
}
impl TlvPut for AnnouncementReasonEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for AnnouncementReasonEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::SimpleAnnouncement),
      0x0001 => Ok(Self::UpdateAvailable),
      0x0002 => Ok(Self::UrgentUpdateAvailable),
      _ => Err(())
    }
  }
}
impl TlvDecode for AnnouncementReasonEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ChangeReasonEnum {
  ///The reason for a state change is unknown.
  Unknown = 0x0000,
  ///The reason for a state change is the success of a prior operation.
  Success = 0x0001,
  ///The reason for a state change is the failure of a prior operation.
  Failure = 0x0002,
  ///The reason for a state change is a time-out.
  TimeOut = 0x0003,
  ///The reason for a state change is a request by the OTA Provider to wait.
  DelayByProvider = 0x0004,
}
impl AttrType for ChangeReasonEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unknown"),
  (0x0001, "Success"),
  (0x0002, "Failure"),
  (0x0003, "TimeOut"),
  (0x0004, "DelayByProvider"),
  ]);
}
impl TlvPut for ChangeReasonEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ChangeReasonEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unknown),
      0x0001 => Ok(Self::Success),
      0x0002 => Ok(Self::Failure),
      0x0003 => Ok(Self::TimeOut),
      0x0004 => Ok(Self::DelayByProvider),
      _ => Err(())
    }
  }
}
impl TlvDecode for ChangeReasonEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum UpdateStateEnum {
  ///Current state is not yet determined.
  Unknown = 0x0000,
  ///Indicate a Node not yet in the process of software update.
  Idle = 0x0001,
  ///Indicate a Node in the process of querying an OTA Provider.
  Querying = 0x0002,
  ///Indicate a Node waiting after a Busy response.
  DelayedOnQuery = 0x0003,
  ///Indicate a Node currently in the process of downloading a software update.
  Downloading = 0x0004,
  ///Indicate a Node currently in the process of verifying and applying a software update.
  Applying = 0x0005,
  ///Indicate a Node waiting caused by AwaitNextAction response.
  DelayedOnApply = 0x0006,
  ///Indicate a Node in the process of recovering to a previous version.
  RollingBack = 0x0007,
  ///Indicate a Node is capable of user consent.
  DelayedOnUserConsent = 0x0008,
}
impl AttrType for UpdateStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unknown"),
  (0x0001, "Idle"),
  (0x0002, "Querying"),
  (0x0003, "DelayedOnQuery"),
  (0x0004, "Downloading"),
  (0x0005, "Applying"),
  (0x0006, "DelayedOnApply"),
  (0x0007, "RollingBack"),
  (0x0008, "DelayedOnUserConsent"),
  ]);
}
impl TlvPut for UpdateStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for UpdateStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unknown),
      0x0001 => Ok(Self::Idle),
      0x0002 => Ok(Self::Querying),
      0x0003 => Ok(Self::DelayedOnQuery),
      0x0004 => Ok(Self::Downloading),
      0x0005 => Ok(Self::Applying),
      0x0006 => Ok(Self::DelayedOnApply),
      0x0007 => Ok(Self::RollingBack),
      0x0008 => Ok(Self::DelayedOnUserConsent),
      _ => Err(())
    }
  }
}
impl TlvDecode for UpdateStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ProviderLocation {
  pub provider_node_id: TypeUnsupported /* NodeId */,
  pub endpoint: TypeUnsupported /* EndpointNo */,
}
impl AttrType for ProviderLocation {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0001, name: "ProviderNodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Endpoint", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ProviderLocation {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.provider_node_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.endpoint.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ProviderLocation {
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
      provider_node_id: TypeUnsupported,
      endpoint: TypeUnsupported,
    })
  }
}

}
