#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const JOINT_FABRIC_DATASTORE: ClusterDef = ClusterDef {
  id: 0x0752,
  name: "Joint Fabric Datastore",
  features: &[
  ],
  attributes: &[
    attr::AnchorRootCA::ATTR,
    attr::AnchorNodeID::ATTR,
    attr::AnchorVendorID::ATTR,
    attr::FriendlyName::ATTR,
    attr::GroupKeySetList::ATTR,
    attr::GroupList::ATTR,
    attr::NodeList::ATTR,
    attr::AdminList::ATTR,
    attr::Status::ATTR,
    attr::EndpointGroupIDList::ATTR,
    attr::EndpointBindingList::ATTR,
    attr::NodeKeySetList::ATTR,
    attr::NodeACLList::ATTR,
    attr::NodeEndpointList::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::AddKeySet::COMMAND,
    command::UpdateKeySet::COMMAND,
    command::RemoveKeySet::COMMAND,
    command::AddGroup::COMMAND,
    command::UpdateGroup::COMMAND,
    command::RemoveGroup::COMMAND,
    command::AddAdmin::COMMAND,
    command::UpdateAdmin::COMMAND,
    command::RemoveAdmin::COMMAND,
    command::AddPendingNode::COMMAND,
    command::RefreshNode::COMMAND,
    command::UpdateNode::COMMAND,
    command::RemoveNode::COMMAND,
    command::UpdateEndpointForNode::COMMAND,
    command::AddGroupIDToEndpointForNode::COMMAND,
    command::RemoveGroupIDFromEndpointForNode::COMMAND,
    command::AddBindingToEndpointForNode::COMMAND,
    command::RemoveBindingFromEndpointForNode::COMMAND,
    command::AddACLToNode::COMMAND,
    command::RemoveACLFromNode::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct AnchorRootCA;

  impl Attr for AnchorRootCA {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "AnchorRootCA",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = TypeUnsupported /* Octstr */;
  }


  pub struct AnchorNodeID;

  impl Attr for AnchorNodeID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "AnchorNodeID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = TypeUnsupported /* NodeId */;
  }


  pub struct AnchorVendorID;

  impl Attr for AnchorVendorID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "AnchorVendorID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = TypeUnsupported /* VendorId */;
  }


  pub struct FriendlyName;

  impl Attr for FriendlyName {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "FriendlyName",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = String;
  }


  pub struct GroupKeySetList;

  impl Attr for GroupKeySetList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "GroupKeySetList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::DatastoreGroupKeySetStruct>;
  }


  pub struct GroupList;

  impl Attr for GroupList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "GroupList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::DatastoreGroupInformationEntryStruct>;
  }


  pub struct NodeList;

  impl Attr for NodeList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "NodeList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::DatastoreNodeInformationEntryStruct>;
  }


  pub struct AdminList;

  impl Attr for AdminList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "AdminList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::DatastoreAdministratorInformationEntryStruct>;
  }


  pub struct Status;

  impl Attr for Status {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "Status",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::DatastoreStatusEntryStruct;
  }


  pub struct EndpointGroupIDList;

  impl Attr for EndpointGroupIDList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "EndpointGroupIDList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::DatastoreEndpointGroupIDEntryStruct>;
  }


  pub struct EndpointBindingList;

  impl Attr for EndpointBindingList {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "EndpointBindingList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::DatastoreEndpointBindingEntryStruct>;
  }


  pub struct NodeKeySetList;

  impl Attr for NodeKeySetList {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "NodeKeySetList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::DatastoreNodeKeySetEntryStruct>;
  }


  pub struct NodeACLList;

  impl Attr for NodeACLList {
    const ATTR: AttrDef = AttrDef {
      id: 0x000c,
      name: "NodeACLList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::DatastoreACLEntryStruct>;
  }


  pub struct NodeEndpointList;

  impl Attr for NodeEndpointList {
    const ATTR: AttrDef = AttrDef {
      id: 0x000d,
      name: "NodeEndpointList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::DatastoreEndpointEntryStruct>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct AddKeySet {
  pub group_key_set: ty::DatastoreGroupKeySetStruct,
}
impl Command for AddKeySet {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "AddKeySet",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupKeySet", attr_ty: <ty::DatastoreGroupKeySetStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddKeySet {
  type Response = BasicAccResponse;
}
impl TlvEncode for AddKeySet {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_key_set.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateKeySet {
  pub group_key_set: ty::DatastoreGroupKeySetStruct,
}
impl Command for UpdateKeySet {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "UpdateKeySet",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupKeySet", attr_ty: <ty::DatastoreGroupKeySetStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for UpdateKeySet {
  type Response = BasicAccResponse;
}
impl TlvEncode for UpdateKeySet {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_key_set.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveKeySet {
  pub group_key_set_id: u16,
}
impl Command for RemoveKeySet {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "RemoveKeySet",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupKeySetID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveKeySet {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveKeySet {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_key_set_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddGroup {
  pub group_id: TypeUnsupported /* GroupId */,
  pub friendly_name: String,
  pub group_key_set_id: u16,
  pub group_cat: u16,
  pub group_cat_version: u16,
  pub group_permission: ty::DatastoreAccessControlEntryPrivilegeEnum,
}
impl Command for AddGroup {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "AddGroup",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "FriendlyName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "GroupKeySetID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "GroupCAT", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "GroupCATVersion", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0005, name: "GroupPermission", attr_ty: <ty::DatastoreAccessControlEntryPrivilegeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddGroup {
  type Response = BasicAccResponse;
}
impl TlvEncode for AddGroup {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.friendly_name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.group_key_set_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.group_cat.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.group_cat_version.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.group_permission.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateGroup {
  pub group_id: TypeUnsupported /* GroupId */,
  pub friendly_name: String,
  pub group_key_set_id: u16,
  pub group_cat: u16,
  pub group_cat_version: u16,
  pub group_permission: ty::DatastoreAccessControlEntryPrivilegeEnum,
}
impl Command for UpdateGroup {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "UpdateGroup",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "FriendlyName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "GroupKeySetID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "GroupCAT", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "GroupCATVersion", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0005, name: "GroupPermission", attr_ty: <ty::DatastoreAccessControlEntryPrivilegeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for UpdateGroup {
  type Response = BasicAccResponse;
}
impl TlvEncode for UpdateGroup {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.friendly_name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.group_key_set_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.group_cat.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.group_cat_version.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.group_permission.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveGroup {
  pub group_id: TypeUnsupported /* GroupId */,
}
impl Command for RemoveGroup {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "RemoveGroup",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveGroup {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveGroup {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddAdmin {
  pub node_id: TypeUnsupported /* NodeId */,
  pub friendly_name: String,
  pub vendor_id: TypeUnsupported /* VendorId */,
  pub icac: TypeUnsupported /* Octstr */,
}
impl Command for AddAdmin {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "AddAdmin",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "FriendlyName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "VendorID", attr_ty: <TypeUnsupported /* VendorId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "ICAC", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddAdmin {
  type Response = BasicAccResponse;
}
impl TlvEncode for AddAdmin {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.friendly_name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.vendor_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.icac.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateAdmin {
  pub node_id: TypeUnsupported /* NodeId */,
  pub friendly_name: String,
  pub icac: TypeUnsupported /* Octstr */,
}
impl Command for UpdateAdmin {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "UpdateAdmin",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "FriendlyName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "ICAC", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for UpdateAdmin {
  type Response = BasicAccResponse;
}
impl TlvEncode for UpdateAdmin {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.friendly_name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.icac.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveAdmin {
  pub node_id: TypeUnsupported /* NodeId */,
}
impl Command for RemoveAdmin {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0008,
    name: "RemoveAdmin",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveAdmin {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveAdmin {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddPendingNode {
  pub node_id: TypeUnsupported /* NodeId */,
  pub friendly_name: String,
}
impl Command for AddPendingNode {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0009,
    name: "AddPendingNode",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "FriendlyName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddPendingNode {
  type Response = BasicAccResponse;
}
impl TlvEncode for AddPendingNode {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.friendly_name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RefreshNode {
  pub node_id: TypeUnsupported /* NodeId */,
}
impl Command for RefreshNode {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000a,
    name: "RefreshNode",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RefreshNode {
  type Response = BasicAccResponse;
}
impl TlvEncode for RefreshNode {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateNode {
  pub node_id: TypeUnsupported /* NodeId */,
  pub friendly_name: String,
}
impl Command for UpdateNode {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000b,
    name: "UpdateNode",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "FriendlyName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for UpdateNode {
  type Response = BasicAccResponse;
}
impl TlvEncode for UpdateNode {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.friendly_name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveNode {
  pub node_id: TypeUnsupported /* NodeId */,
}
impl Command for RemoveNode {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000c,
    name: "RemoveNode",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveNode {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveNode {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateEndpointForNode {
  pub endpoint_id: TypeUnsupported /* EndpointNo */,
  pub node_id: TypeUnsupported /* NodeId */,
  pub friendly_name: String,
}
impl Command for UpdateEndpointForNode {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000d,
    name: "UpdateEndpointForNode",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "EndpointID", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "FriendlyName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for UpdateEndpointForNode {
  type Response = BasicAccResponse;
}
impl TlvEncode for UpdateEndpointForNode {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.endpoint_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.friendly_name.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddGroupIDToEndpointForNode {
  pub node_id: TypeUnsupported /* NodeId */,
  pub endpoint_id: TypeUnsupported /* EndpointNo */,
  pub group_id: TypeUnsupported /* GroupId */,
}
impl Command for AddGroupIDToEndpointForNode {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000e,
    name: "AddGroupIDToEndpointForNode",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "EndpointID", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddGroupIDToEndpointForNode {
  type Response = BasicAccResponse;
}
impl TlvEncode for AddGroupIDToEndpointForNode {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.endpoint_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveGroupIDFromEndpointForNode {
  pub node_id: TypeUnsupported /* NodeId */,
  pub endpoint_id: TypeUnsupported /* EndpointNo */,
  pub group_id: TypeUnsupported /* GroupId */,
}
impl Command for RemoveGroupIDFromEndpointForNode {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000f,
    name: "RemoveGroupIDFromEndpointForNode",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "EndpointID", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveGroupIDFromEndpointForNode {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveGroupIDFromEndpointForNode {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.endpoint_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddBindingToEndpointForNode {
  pub node_id: TypeUnsupported /* NodeId */,
  pub endpoint_id: TypeUnsupported /* EndpointNo */,
  pub binding: ty::DatastoreBindingTargetStruct,
}
impl Command for AddBindingToEndpointForNode {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0010,
    name: "AddBindingToEndpointForNode",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "EndpointID", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Binding", attr_ty: <ty::DatastoreBindingTargetStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddBindingToEndpointForNode {
  type Response = BasicAccResponse;
}
impl TlvEncode for AddBindingToEndpointForNode {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.endpoint_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.binding.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveBindingFromEndpointForNode {
  pub list_id: u16,
  pub endpoint_id: TypeUnsupported /* EndpointNo */,
  pub node_id: TypeUnsupported /* NodeId */,
}
impl Command for RemoveBindingFromEndpointForNode {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0011,
    name: "RemoveBindingFromEndpointForNode",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ListID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "EndpointID", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveBindingFromEndpointForNode {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveBindingFromEndpointForNode {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.list_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.endpoint_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddACLToNode {
  pub node_id: TypeUnsupported /* NodeId */,
  pub acl_entry: ty::DatastoreAccessControlEntryStruct,
}
impl Command for AddACLToNode {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0012,
    name: "AddACLToNode",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ACLEntry", attr_ty: <ty::DatastoreAccessControlEntryStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddACLToNode {
  type Response = BasicAccResponse;
}
impl TlvEncode for AddACLToNode {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.acl_entry.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveACLFromNode {
  pub list_id: u16,
  pub node_id: TypeUnsupported /* NodeId */,
}
impl Command for RemoveACLFromNode {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0013,
    name: "RemoveACLFromNode",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ListID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveACLFromNode {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveACLFromNode {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.list_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
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
pub enum DatastoreAccessControlEntryAuthModeEnum {
  ///Passcode authenticated session
  PASE = 0x0001,
  ///Certificate authenticated session
  CASE = 0x0002,
  ///Group authenticated session
  Group = 0x0003,
}
impl AttrType for DatastoreAccessControlEntryAuthModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0001, "PASE"),
  (0x0002, "CASE"),
  (0x0003, "Group"),
  ]);
}
impl TlvPut for DatastoreAccessControlEntryAuthModeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for DatastoreAccessControlEntryAuthModeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0001 => Ok(Self::PASE),
      0x0002 => Ok(Self::CASE),
      0x0003 => Ok(Self::Group),
      _ => Err(())
    }
  }
}
impl TlvDecode for DatastoreAccessControlEntryAuthModeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum DatastoreAccessControlEntryPrivilegeEnum {
  ///Can read and observe all (except Access Control Cluster)
  View = 0x0001,
  ///View privileges, and can perform the primary function of this Node (except Access Control Cluster)
  Operate = 0x0003,
  ///Operate privileges, and can modify persistent configuration of this Node (except Access Control Cluster)
  Manage = 0x0004,
  ///Manage privileges, and can observe and modify the Access Control Cluster
  Administer = 0x0005,
}
impl AttrType for DatastoreAccessControlEntryPrivilegeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0001, "View"),
  (0x0003, "Operate"),
  (0x0004, "Manage"),
  (0x0005, "Administer"),
  ]);
}
impl TlvPut for DatastoreAccessControlEntryPrivilegeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for DatastoreAccessControlEntryPrivilegeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0001 => Ok(Self::View),
      0x0003 => Ok(Self::Operate),
      0x0004 => Ok(Self::Manage),
      0x0005 => Ok(Self::Administer),
      _ => Err(())
    }
  }
}
impl TlvDecode for DatastoreAccessControlEntryPrivilegeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum DatastoreGroupKeyMulticastPolicyEnum {
  ///Indicates filtering of multicast messages for a specific Group ID
  PerGroupID = 0x0000,
  ///Indicates not filtering of multicast messages
  AllNodes = 0x0001,
}
impl AttrType for DatastoreGroupKeyMulticastPolicyEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "PerGroupID"),
  (0x0001, "AllNodes"),
  ]);
}
impl TlvPut for DatastoreGroupKeyMulticastPolicyEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for DatastoreGroupKeyMulticastPolicyEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::PerGroupID),
      0x0001 => Ok(Self::AllNodes),
      _ => Err(())
    }
  }
}
impl TlvDecode for DatastoreGroupKeyMulticastPolicyEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum DatastoreGroupKeySecurityPolicyEnum {
  ///Message counter synchronization using trust-first
  TrustFirst = 0x0000,
}
impl AttrType for DatastoreGroupKeySecurityPolicyEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "TrustFirst"),
  ]);
}
impl TlvPut for DatastoreGroupKeySecurityPolicyEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for DatastoreGroupKeySecurityPolicyEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::TrustFirst),
      _ => Err(())
    }
  }
}
impl TlvDecode for DatastoreGroupKeySecurityPolicyEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum DatastoreStateEnum {
  ///Target device operation is pending
  Pending = 0x0000,
  ///Target device operation has been committed
  Committed = 0x0001,
  ///Target device delete operation is pending
  DeletePending = 0x0002,
  ///Target device operation has failed
  CommitFailed = 0x0003,
}
impl AttrType for DatastoreStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Pending"),
  (0x0001, "Committed"),
  (0x0002, "DeletePending"),
  (0x0003, "CommitFailed"),
  ]);
}
impl TlvPut for DatastoreStateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for DatastoreStateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Pending),
      0x0001 => Ok(Self::Committed),
      0x0002 => Ok(Self::DeletePending),
      0x0003 => Ok(Self::CommitFailed),
      _ => Err(())
    }
  }
}
impl TlvDecode for DatastoreStateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct DatastoreACLEntryStruct {
  pub node_id: TypeUnsupported /* NodeId */,
  pub list_id: u16,
  pub acl_entry: DatastoreAccessControlEntryStruct,
  pub status_entry: DatastoreStatusEntryStruct,
}
impl AttrType for DatastoreACLEntryStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ListID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "ACLEntry", attr_ty: <DatastoreAccessControlEntryStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "StatusEntry", attr_ty: <DatastoreStatusEntryStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DatastoreACLEntryStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.list_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.acl_entry.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.status_entry.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DatastoreACLEntryStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut list_id: Option<u16> = None;
    let mut acl_entry: Option<DatastoreAccessControlEntryStruct> = None;
    let mut status_entry: Option<DatastoreStatusEntryStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { list_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { acl_entry = Some(<DatastoreAccessControlEntryStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0003), kind: _ }) => { status_entry = Some(<DatastoreStatusEntryStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      node_id: TypeUnsupported,
      list_id: list_id.ok_or(TlvDecodeErr::MissingField("ListID"))?,
      acl_entry: acl_entry.ok_or(TlvDecodeErr::MissingField("ACLEntry"))?,
      status_entry: status_entry.ok_or(TlvDecodeErr::MissingField("StatusEntry"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatastoreAccessControlEntryStruct {
  pub privilege: DatastoreAccessControlEntryPrivilegeEnum,
  pub auth_mode: DatastoreAccessControlEntryAuthModeEnum,
  pub subjects: Vec<TypeUnsupported /* SubjectId */>,
  pub targets: Vec<DatastoreAccessControlTargetStruct>,
}
impl AttrType for DatastoreAccessControlEntryStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0001, name: "Privilege", attr_ty: <DatastoreAccessControlEntryPrivilegeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "AuthMode", attr_ty: <DatastoreAccessControlEntryAuthModeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "Subjects", attr_ty: <Vec<TypeUnsupported /* SubjectId */>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "Targets", attr_ty: <Vec<DatastoreAccessControlTargetStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DatastoreAccessControlEntryStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.privilege.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.auth_mode.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.subjects.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.targets.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DatastoreAccessControlEntryStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut privilege: Option<DatastoreAccessControlEntryPrivilegeEnum> = None;
    let mut auth_mode: Option<DatastoreAccessControlEntryAuthModeEnum> = None;
    let mut subjects: Option<Vec<TypeUnsupported /* SubjectId */>> = None;
    let mut targets: Option<Vec<DatastoreAccessControlTargetStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { privilege = Some(<DatastoreAccessControlEntryPrivilegeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { auth_mode = Some(<DatastoreAccessControlEntryAuthModeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0003), kind: _ }) => { subjects = Some(<Vec<TypeUnsupported /* SubjectId */>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0004), kind: _ }) => { targets = Some(<Vec<DatastoreAccessControlTargetStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      privilege: privilege.ok_or(TlvDecodeErr::MissingField("Privilege"))?,
      auth_mode: auth_mode.ok_or(TlvDecodeErr::MissingField("AuthMode"))?,
      subjects: subjects.ok_or(TlvDecodeErr::MissingField("Subjects"))?,
      targets: targets.ok_or(TlvDecodeErr::MissingField("Targets"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatastoreAccessControlTargetStruct {
  pub cluster: TypeUnsupported /* ClusterId */,
  pub endpoint: TypeUnsupported /* EndpointNo */,
  pub device_type: TypeUnsupported /* DevtypeId */,
}
impl AttrType for DatastoreAccessControlTargetStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Cluster", attr_ty: <TypeUnsupported /* ClusterId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Endpoint", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "DeviceType", attr_ty: <TypeUnsupported /* DevtypeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DatastoreAccessControlTargetStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.cluster.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.endpoint.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.device_type.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DatastoreAccessControlTargetStruct {
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
      cluster: TypeUnsupported,
      endpoint: TypeUnsupported,
      device_type: TypeUnsupported,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatastoreAdministratorInformationEntryStruct {
  pub node_id: TypeUnsupported /* NodeId */,
  pub friendly_name: String,
  pub vendor_id: TypeUnsupported /* VendorId */,
  pub icac: TypeUnsupported /* Octstr */,
}
impl AttrType for DatastoreAdministratorInformationEntryStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "FriendlyName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "VendorID", attr_ty: <TypeUnsupported /* VendorId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "ICAC", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DatastoreAdministratorInformationEntryStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.friendly_name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.vendor_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.icac.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DatastoreAdministratorInformationEntryStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut friendly_name: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { friendly_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      node_id: TypeUnsupported,
      friendly_name: friendly_name.ok_or(TlvDecodeErr::MissingField("FriendlyName"))?,
      vendor_id: TypeUnsupported,
      icac: TypeUnsupported,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatastoreBindingTargetStruct {
  pub node: TypeUnsupported /* NodeId */,
  pub group: TypeUnsupported /* GroupId */,
  pub endpoint: TypeUnsupported /* EndpointNo */,
  pub cluster: TypeUnsupported /* ClusterId */,
}
impl AttrType for DatastoreBindingTargetStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0001, name: "Node", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Group", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "Endpoint", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "Cluster", attr_ty: <TypeUnsupported /* ClusterId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DatastoreBindingTargetStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.node.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.group.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.endpoint.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.cluster.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DatastoreBindingTargetStruct {
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
      node: TypeUnsupported,
      group: TypeUnsupported,
      endpoint: TypeUnsupported,
      cluster: TypeUnsupported,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatastoreEndpointBindingEntryStruct {
  pub node_id: TypeUnsupported /* NodeId */,
  pub endpoint_id: TypeUnsupported /* EndpointNo */,
  pub list_id: u16,
  pub binding: DatastoreBindingTargetStruct,
  pub status_entry: DatastoreStatusEntryStruct,
}
impl AttrType for DatastoreEndpointBindingEntryStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "EndpointID", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "ListID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "Binding", attr_ty: <DatastoreBindingTargetStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "StatusEntry", attr_ty: <DatastoreStatusEntryStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DatastoreEndpointBindingEntryStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.endpoint_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.list_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.binding.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.status_entry.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DatastoreEndpointBindingEntryStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut list_id: Option<u16> = None;
    let mut binding: Option<DatastoreBindingTargetStruct> = None;
    let mut status_entry: Option<DatastoreStatusEntryStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { list_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0003), kind: _ }) => { binding = Some(<DatastoreBindingTargetStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0004), kind: _ }) => { status_entry = Some(<DatastoreStatusEntryStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      node_id: TypeUnsupported,
      endpoint_id: TypeUnsupported,
      list_id: list_id.ok_or(TlvDecodeErr::MissingField("ListID"))?,
      binding: binding.ok_or(TlvDecodeErr::MissingField("Binding"))?,
      status_entry: status_entry.ok_or(TlvDecodeErr::MissingField("StatusEntry"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatastoreEndpointEntryStruct {
  pub endpoint_id: TypeUnsupported /* EndpointNo */,
  pub node_id: TypeUnsupported /* NodeId */,
  pub friendly_name: String,
}
impl AttrType for DatastoreEndpointEntryStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "EndpointID", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "FriendlyName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DatastoreEndpointEntryStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.endpoint_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.friendly_name.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DatastoreEndpointEntryStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut friendly_name: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { friendly_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      endpoint_id: TypeUnsupported,
      node_id: TypeUnsupported,
      friendly_name: friendly_name.ok_or(TlvDecodeErr::MissingField("FriendlyName"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatastoreEndpointGroupIDEntryStruct {
  pub node_id: TypeUnsupported /* NodeId */,
  pub endpoint_id: TypeUnsupported /* EndpointNo */,
  pub group_id: TypeUnsupported /* GroupId */,
  pub status_entry: DatastoreStatusEntryStruct,
}
impl AttrType for DatastoreEndpointGroupIDEntryStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "EndpointID", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "StatusEntry", attr_ty: <DatastoreStatusEntryStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DatastoreEndpointGroupIDEntryStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.endpoint_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.status_entry.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DatastoreEndpointGroupIDEntryStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut status_entry: Option<DatastoreStatusEntryStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0003), kind: _ }) => { status_entry = Some(<DatastoreStatusEntryStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      node_id: TypeUnsupported,
      endpoint_id: TypeUnsupported,
      group_id: TypeUnsupported,
      status_entry: status_entry.ok_or(TlvDecodeErr::MissingField("StatusEntry"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatastoreGroupInformationEntryStruct {
  pub group_id: u64,
  pub friendly_name: String,
  pub group_key_set_id: u16,
  pub group_cat: u16,
  pub group_cat_version: u16,
  pub group_permission: DatastoreAccessControlEntryPrivilegeEnum,
}
impl AttrType for DatastoreGroupInformationEntryStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "GroupID", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "FriendlyName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "GroupKeySetID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "GroupCAT", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "GroupCATVersion", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "GroupPermission", attr_ty: <DatastoreAccessControlEntryPrivilegeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DatastoreGroupInformationEntryStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.friendly_name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.group_key_set_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.group_cat.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.group_cat_version.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.group_permission.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DatastoreGroupInformationEntryStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut group_id: Option<u64> = None;
    let mut friendly_name: Option<String> = None;
    let mut group_key_set_id: Option<u16> = None;
    let mut group_cat: Option<u16> = None;
    let mut group_cat_version: Option<u16> = None;
    let mut group_permission: Option<DatastoreAccessControlEntryPrivilegeEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { group_id = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { friendly_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { group_key_set_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { group_cat = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { group_cat_version = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { group_permission = Some(<DatastoreAccessControlEntryPrivilegeEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      group_id: group_id.ok_or(TlvDecodeErr::MissingField("GroupID"))?,
      friendly_name: friendly_name.ok_or(TlvDecodeErr::MissingField("FriendlyName"))?,
      group_key_set_id: group_key_set_id.ok_or(TlvDecodeErr::MissingField("GroupKeySetID"))?,
      group_cat: group_cat.ok_or(TlvDecodeErr::MissingField("GroupCAT"))?,
      group_cat_version: group_cat_version.ok_or(TlvDecodeErr::MissingField("GroupCATVersion"))?,
      group_permission: group_permission.ok_or(TlvDecodeErr::MissingField("GroupPermission"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatastoreGroupKeySetStruct {
  pub group_key_set_id: u16,
  pub group_key_security_policy: DatastoreGroupKeySecurityPolicyEnum,
  pub epoch_key0: TypeUnsupported /* Octstr */,
  pub epoch_start_time0: TypeUnsupported /* EpochUs */,
  pub epoch_key1: TypeUnsupported /* Octstr */,
  pub epoch_start_time1: TypeUnsupported /* EpochUs */,
  pub epoch_key2: TypeUnsupported /* Octstr */,
  pub epoch_start_time2: TypeUnsupported /* EpochUs */,
  pub group_key_multicast_policy: DatastoreGroupKeyMulticastPolicyEnum,
}
impl AttrType for DatastoreGroupKeySetStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "GroupKeySetID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "GroupKeySecurityPolicy", attr_ty: <DatastoreGroupKeySecurityPolicyEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "EpochKey0", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "EpochStartTime0", attr_ty: <TypeUnsupported /* EpochUs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "EpochKey1", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "EpochStartTime1", attr_ty: <TypeUnsupported /* EpochUs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "EpochKey2", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "EpochStartTime2", attr_ty: <TypeUnsupported /* EpochUs */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0008, name: "GroupKeyMulticastPolicy", attr_ty: <DatastoreGroupKeyMulticastPolicyEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DatastoreGroupKeySetStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.group_key_set_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.group_key_security_policy.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.epoch_key0.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.epoch_start_time0.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.epoch_key1.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.epoch_start_time1.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.epoch_key2.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.epoch_start_time2.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.group_key_multicast_policy.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DatastoreGroupKeySetStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut group_key_set_id: Option<u16> = None;
    let mut group_key_security_policy: Option<DatastoreGroupKeySecurityPolicyEnum> = None;
    let mut group_key_multicast_policy: Option<DatastoreGroupKeyMulticastPolicyEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { group_key_set_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { group_key_security_policy = Some(<DatastoreGroupKeySecurityPolicyEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0008), value: _ }) => { group_key_multicast_policy = Some(<DatastoreGroupKeyMulticastPolicyEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      group_key_set_id: group_key_set_id.ok_or(TlvDecodeErr::MissingField("GroupKeySetID"))?,
      group_key_security_policy: group_key_security_policy.ok_or(TlvDecodeErr::MissingField("GroupKeySecurityPolicy"))?,
      epoch_key0: TypeUnsupported,
      epoch_start_time0: TypeUnsupported,
      epoch_key1: TypeUnsupported,
      epoch_start_time1: TypeUnsupported,
      epoch_key2: TypeUnsupported,
      epoch_start_time2: TypeUnsupported,
      group_key_multicast_policy: group_key_multicast_policy.ok_or(TlvDecodeErr::MissingField("GroupKeyMulticastPolicy"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatastoreNodeInformationEntryStruct {
  pub node_id: TypeUnsupported /* NodeId */,
  pub friendly_name: String,
  pub commissioning_status_entry: DatastoreStatusEntryStruct,
}
impl AttrType for DatastoreNodeInformationEntryStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "FriendlyName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "CommissioningStatusEntry", attr_ty: <DatastoreStatusEntryStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DatastoreNodeInformationEntryStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.friendly_name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.commissioning_status_entry.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DatastoreNodeInformationEntryStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut friendly_name: Option<String> = None;
    let mut commissioning_status_entry: Option<DatastoreStatusEntryStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { friendly_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { commissioning_status_entry = Some(<DatastoreStatusEntryStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      node_id: TypeUnsupported,
      friendly_name: friendly_name.ok_or(TlvDecodeErr::MissingField("FriendlyName"))?,
      commissioning_status_entry: commissioning_status_entry.ok_or(TlvDecodeErr::MissingField("CommissioningStatusEntry"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatastoreNodeKeySetEntryStruct {
  pub node_id: TypeUnsupported /* NodeId */,
  pub group_key_set_id: u16,
  pub status_entry: DatastoreStatusEntryStruct,
}
impl AttrType for DatastoreNodeKeySetEntryStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "NodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "GroupKeySetID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "StatusEntry", attr_ty: <DatastoreStatusEntryStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DatastoreNodeKeySetEntryStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.node_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.group_key_set_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.status_entry.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DatastoreNodeKeySetEntryStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut group_key_set_id: Option<u16> = None;
    let mut status_entry: Option<DatastoreStatusEntryStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { group_key_set_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { status_entry = Some(<DatastoreStatusEntryStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      node_id: TypeUnsupported,
      group_key_set_id: group_key_set_id.ok_or(TlvDecodeErr::MissingField("GroupKeySetID"))?,
      status_entry: status_entry.ok_or(TlvDecodeErr::MissingField("StatusEntry"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatastoreStatusEntryStruct {
  pub state: DatastoreStateEnum,
  pub update_timestamp: TypeUnsupported /* EpochS */,
  pub failure_code: TypeUnsupported /* Status */,
}
impl AttrType for DatastoreStatusEntryStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "State", attr_ty: <DatastoreStateEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "UpdateTimestamp", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "FailureCode", attr_ty: <TypeUnsupported /* Status */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for DatastoreStatusEntryStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.state.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.update_timestamp.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.failure_code.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for DatastoreStatusEntryStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut state: Option<DatastoreStateEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { state = Some(<DatastoreStateEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      state: state.ok_or(TlvDecodeErr::MissingField("State"))?,
      update_timestamp: TypeUnsupported,
      failure_code: TypeUnsupported,
    })
  }
}

}
