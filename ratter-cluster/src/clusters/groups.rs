#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const GROUPS: ClusterDef = ClusterDef {
  id: 0x0004,
  name: "Groups",
  features: &[
    FeatureDef { bit: 0x0000, code: "GN", name: "GroupNames", summary: "The ability to store a name for a group" },
  ],
  attributes: &[
    attr::NameSupport::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::AddGroup::COMMAND,
    command::ViewGroup::COMMAND,
    command::GetGroupMembership::COMMAND,
    command::RemoveGroup::COMMAND,
    command::RemoveAllGroups::COMMAND,
    command::AddGroupIfIdentifying::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct NameSupport;

  impl Attr for NameSupport {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "NameSupport",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::NameSupportBitmap;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct AddGroup {
  pub group_id: TypeUnsupported /* GroupId */,
  pub group_name: String,
}
impl Command for AddGroup {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "AddGroup",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "GroupName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddGroup {
  type Response = AddGroupResponse;
}
impl TlvEncode for AddGroup {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.group_name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddGroupResponse {
  pub status: TypeUnsupported /* Enum8 */,
  pub group_id: TypeUnsupported /* GroupId */,
}
impl Command for AddGroupResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "AddGroupResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <TypeUnsupported /* Enum8 */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for AddGroupResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct ViewGroup {
  pub group_id: TypeUnsupported /* GroupId */,
}
impl Command for ViewGroup {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "ViewGroup",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ViewGroup {
  type Response = ViewGroupResponse;
}
impl TlvEncode for ViewGroup {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ViewGroupResponse {
  pub status: TypeUnsupported /* Enum8 */,
  pub group_id: TypeUnsupported /* GroupId */,
  pub group_name: String,
}
impl Command for ViewGroupResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "ViewGroupResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <TypeUnsupported /* Enum8 */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "GroupName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for ViewGroupResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct GetGroupMembership {
  pub group_list: Vec<TypeUnsupported /* GroupId */>,
}
impl Command for GetGroupMembership {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "GetGroupMembership",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupList", attr_ty: <Vec<TypeUnsupported /* GroupId */>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for GetGroupMembership {
  type Response = GetGroupMembershipResponse;
}
impl TlvEncode for GetGroupMembership {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_list.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetGroupMembershipResponse {
  pub capacity: u8,
  pub group_list: Vec<TypeUnsupported /* GroupId */>,
}
impl Command for GetGroupMembershipResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "GetGroupMembershipResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Capacity", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "GroupList", attr_ty: <Vec<TypeUnsupported /* GroupId */>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for GetGroupMembershipResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveGroup {
  pub group_id: TypeUnsupported /* GroupId */,
}
impl Command for RemoveGroup {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "RemoveGroup",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveGroup {
  type Response = RemoveGroupResponse;
}
impl TlvEncode for RemoveGroup {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveGroupResponse {
  pub status: TypeUnsupported /* Enum8 */,
  pub group_id: TypeUnsupported /* GroupId */,
}
impl Command for RemoveGroupResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "RemoveGroupResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <TypeUnsupported /* Enum8 */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for RemoveGroupResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveAllGroups {
}
impl Command for RemoveAllGroups {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "RemoveAllGroups",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for RemoveAllGroups {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveAllGroups {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddGroupIfIdentifying {
  pub group_id: TypeUnsupported /* GroupId */,
  pub group_name: String,
}
impl Command for AddGroupIfIdentifying {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "AddGroupIfIdentifying",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "GroupID", attr_ty: <TypeUnsupported /* GroupId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "GroupName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddGroupIfIdentifying {
  type Response = BasicAccResponse;
}
impl TlvEncode for AddGroupIfIdentifying {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.group_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.group_name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct NameSupportBitmap: u32 {
    ///The ability to store a name for a group.
    const GroupNames = 1 << 7;
  }
}
impl AttrType for NameSupportBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (7, "GroupNames"),
  ]);
}
impl TlvPut for NameSupportBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for NameSupportBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for NameSupportBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
}
