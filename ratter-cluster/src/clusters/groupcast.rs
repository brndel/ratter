#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const GROUPCAST: ClusterDef = ClusterDef {
  id: 0x0065,
  name: "Groupcast",
  features: &[
    FeatureDef { bit: 0x0000, code: "LN", name: "Listener", summary: "Supports joining a multicast group of nodes as a listener" },
    FeatureDef { bit: 0x0001, code: "SD", name: "Sender", summary: "Supports sending multicast message to a targeted group of nodes" },
    FeatureDef { bit: 0x0002, code: "PGA", name: "PerGroup", summary: "Supports PerGroup multicast addresses" },
  ],
  attributes: &[
    attr::Membership::ATTR,
    attr::MaxMembershipCount::ATTR,
    attr::MaxMcastAddrCount::ATTR,
    attr::UsedMcastAddrCount::ATTR,
    attr::FabricUnderTest::ATTR,
  ],
  events: &[
    event::GroupcastTesting::EVENT,
  ],
  commands: &[
    command::JoinGroup::COMMAND,
    command::LeaveGroup::COMMAND,
    command::UpdateGroupKey::COMMAND,
    command::ConfigureAuxiliaryACL::COMMAND,
    command::GroupcastTesting::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct Membership;

  impl Attr for Membership {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "Membership",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::MembershipStruct>;
  }


  pub struct MaxMembershipCount;

  impl Attr for MaxMembershipCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "MaxMembershipCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u16;
  }


  pub struct MaxMcastAddrCount;

  impl Attr for MaxMcastAddrCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "MaxMcastAddrCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u16;
  }


  pub struct UsedMcastAddrCount;

  impl Attr for UsedMcastAddrCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "UsedMcastAddrCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u16;
  }


  pub struct FabricUnderTest;

  impl Attr for FabricUnderTest {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "FabricUnderTest",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = TypeUnsupported /* FabricIdx */;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct GroupcastTesting {
  pub source_ip_address: TypeUnsupported /* Ipv6adr */,
  pub destination_ip_address: TypeUnsupported /* Ipv6adr */,
  pub group_id: TypeUnsupported /* GroupId */,
  pub endpoint_id: TypeUnsupported /* EndpointId */,
  pub cluster_id: TypeUnsupported /* ClusterId */,
  pub element_id: u32,
  pub access_allowed: bool,
  pub groupcast_test_result: ty::GroupcastTestResultEnum,
}

impl Event for GroupcastTesting {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "GroupcastTesting",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "SourceIpAddress", attr_ty: <TypeUnsupported /* Ipv6adr */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0001, name: "DestinationIpAddress", attr_ty: <TypeUnsupported /* Ipv6adr */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0003, name: "EndpointID", attr_ty: <TypeUnsupported /* EndpointId */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0004, name: "ClusterID", attr_ty: <TypeUnsupported /* ClusterId */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0005, name: "ElementID", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0006, name: "AccessAllowed", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0007, name: "GroupcastTestResult", attr_ty: <ty::GroupcastTestResultEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for GroupcastTesting {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut element_id: Option<u32> = None;
    let mut access_allowed: Option<bool> = None;
    let mut groupcast_test_result: Option<ty::GroupcastTestResultEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { element_id = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { access_allowed = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0007), value: _ }) => { groupcast_test_result = Some(<ty::GroupcastTestResultEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      source_ip_address: TypeUnsupported,
      destination_ip_address: TypeUnsupported,
      group_id: TypeUnsupported,
      endpoint_id: TypeUnsupported,
      cluster_id: TypeUnsupported,
      element_id: element_id.ok_or(TlvDecodeErr::MissingField("ElementID"))?,
      access_allowed: access_allowed.ok_or(TlvDecodeErr::MissingField("AccessAllowed"))?,
      groupcast_test_result: groupcast_test_result.ok_or(TlvDecodeErr::MissingField("GroupcastTestResult"))?,
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct JoinGroup {
  pub group_id: TypeUnsupported /* GroupId */,
  pub endpoints: Vec<TypeUnsupported /* EndpointNo */>,
  pub key_set_id: u16,
  pub key: TypeUnsupported /* Octstr */,
  pub use_auxiliary_acl: bool,
  pub replace_endpoints: bool,
  pub mcast_addr_policy: ty::MulticastAddrPolicyEnum,
}
impl Command for JoinGroup {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "JoinGroup",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Endpoints", attr_ty: <Vec<TypeUnsupported /* EndpointNo */>>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "KeySetID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "Key", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0004, name: "UseAuxiliaryACL", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0005, name: "ReplaceEndpoints", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0006, name: "McastAddrPolicy", attr_ty: <ty::MulticastAddrPolicyEnum>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for JoinGroup {
  type Response = BasicAccResponse;
}
impl TlvEncode for JoinGroup {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.endpoints.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.key_set_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.key.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.use_auxiliary_acl.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.replace_endpoints.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.mcast_addr_policy.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LeaveGroup {
  pub group_id: TypeUnsupported /* GroupId */,
  pub endpoints: Vec<TypeUnsupported /* EndpointNo */>,
}
impl Command for LeaveGroup {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "LeaveGroup",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Endpoints", attr_ty: <Vec<TypeUnsupported /* EndpointNo */>>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for LeaveGroup {
  type Response = LeaveGroupResponse;
}
impl TlvEncode for LeaveGroup {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.endpoints.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LeaveGroupResponse {
  pub group_id: TypeUnsupported /* GroupId */,
  pub endpoints: Vec<TypeUnsupported /* EndpointNo */>,
}
impl Command for LeaveGroupResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "LeaveGroupResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Endpoints", attr_ty: <Vec<TypeUnsupported /* EndpointNo */>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for LeaveGroupResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateGroupKey {
  pub group_id: TypeUnsupported /* GroupId */,
  pub key_set_id: u16,
  pub key: TypeUnsupported /* Octstr */,
}
impl Command for UpdateGroupKey {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "UpdateGroupKey",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "KeySetID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Key", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for UpdateGroupKey {
  type Response = BasicAccResponse;
}
impl TlvEncode for UpdateGroupKey {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.key_set_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.key.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConfigureAuxiliaryACL {
  pub group_id: TypeUnsupported /* GroupId */,
  pub use_auxiliary_acl: bool,
}
impl Command for ConfigureAuxiliaryACL {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "ConfigureAuxiliaryACL",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "UseAuxiliaryACL", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ConfigureAuxiliaryACL {
  type Response = BasicAccResponse;
}
impl TlvEncode for ConfigureAuxiliaryACL {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.use_auxiliary_acl.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GroupcastTesting {
  pub test_operation: ty::GroupcastTestingEnum,
  pub duration_seconds: u16,
}
impl Command for GroupcastTesting {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "GroupcastTesting",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "TestOperation", attr_ty: <ty::GroupcastTestingEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "DurationSeconds", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for GroupcastTesting {
  type Response = BasicAccResponse;
}
impl TlvEncode for GroupcastTesting {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.test_operation.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.duration_seconds.tlv_put(matter_codec::Tag::Context(0x0001), writer);
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
pub enum GroupcastTestResultEnum {
  ///There was no failure
  Success = 0x0000,
  ///Other error during processing not otherwise covered
  GeneralError = 0x0001,
  ///Message counter was <= counter from last received
  MessageReplay = 0x0002,
  ///Message authentication failed
  FailedAuth = 0x0003,
  ///No key was found to process the message
  NoAvailableKey = 0x0004,
  ///Issue with attempting to send a multicast IPv6 message
  SendFailure = 0x0005,
}
impl AttrType for GroupcastTestResultEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Success"),
  (0x0001, "GeneralError"),
  (0x0002, "MessageReplay"),
  (0x0003, "FailedAuth"),
  (0x0004, "NoAvailableKey"),
  (0x0005, "SendFailure"),
  ]);
}
impl TlvPut for GroupcastTestResultEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for GroupcastTestResultEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Success),
      0x0001 => Ok(Self::GeneralError),
      0x0002 => Ok(Self::MessageReplay),
      0x0003 => Ok(Self::FailedAuth),
      0x0004 => Ok(Self::NoAvailableKey),
      0x0005 => Ok(Self::SendFailure),
      _ => Err(())
    }
  }
}
impl TlvDecode for GroupcastTestResultEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum GroupcastTestingEnum {
  ///Disable all test event generation
  DisableTesting = 0x0000,
  ///Enable Listener test event generation
  EnableListenerTesting = 0x0001,
  ///Enable Sender test event generation
  EnableSenderTesting = 0x0002,
}
impl AttrType for GroupcastTestingEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "DisableTesting"),
  (0x0001, "EnableListenerTesting"),
  (0x0002, "EnableSenderTesting"),
  ]);
}
impl TlvPut for GroupcastTestingEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for GroupcastTestingEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::DisableTesting),
      0x0001 => Ok(Self::EnableListenerTesting),
      0x0002 => Ok(Self::EnableSenderTesting),
      _ => Err(())
    }
  }
}
impl TlvDecode for GroupcastTestingEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum MulticastAddrPolicyEnum {
  ///Indicates group uses the IANA-assigned multicast address (default)
  IanaAddr = 0x0000,
  ///Group uses multicast address scoped to Fabric ID and Group ID
  PerGroup = 0x0001,
}
impl AttrType for MulticastAddrPolicyEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "IanaAddr"),
  (0x0001, "PerGroup"),
  ]);
}
impl TlvPut for MulticastAddrPolicyEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for MulticastAddrPolicyEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::IanaAddr),
      0x0001 => Ok(Self::PerGroup),
      _ => Err(())
    }
  }
}
impl TlvDecode for MulticastAddrPolicyEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct MembershipStruct {
  pub group_id: TypeUnsupported /* GroupId */,
  pub endpoints: Vec<TypeUnsupported /* EndpointNo */>,
  pub key_set_id: u16,
  pub has_auxiliary_acl: bool,
  pub mcast_addr_policy: MulticastAddrPolicyEnum,
}
impl AttrType for MembershipStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Endpoints", attr_ty: <Vec<TypeUnsupported /* EndpointNo */>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "KeySetID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "HasAuxiliaryACL", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "McastAddrPolicy", attr_ty: <MulticastAddrPolicyEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for MembershipStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.endpoints.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.key_set_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.has_auxiliary_acl.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.mcast_addr_policy.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for MembershipStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut endpoints: Option<Vec<TypeUnsupported /* EndpointNo */>> = None;
    let mut key_set_id: Option<u16> = None;
    let mut has_auxiliary_acl: Option<bool> = None;
    let mut mcast_addr_policy: Option<MulticastAddrPolicyEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { endpoints = Some(<Vec<TypeUnsupported /* EndpointNo */>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { key_set_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { has_auxiliary_acl = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { mcast_addr_policy = Some(<MulticastAddrPolicyEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      group_id: TypeUnsupported,
      endpoints: endpoints.ok_or(TlvDecodeErr::MissingField("Endpoints"))?,
      key_set_id: key_set_id.ok_or(TlvDecodeErr::MissingField("KeySetID"))?,
      has_auxiliary_acl: has_auxiliary_acl.ok_or(TlvDecodeErr::MissingField("HasAuxiliaryACL"))?,
      mcast_addr_policy: mcast_addr_policy.ok_or(TlvDecodeErr::MissingField("McastAddrPolicy"))?,
    })
  }
}

}
