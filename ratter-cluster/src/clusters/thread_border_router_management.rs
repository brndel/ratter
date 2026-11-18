#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const THREAD_BORDER_ROUTER_MANAGEMENT: ClusterDef = ClusterDef {
  id: 0x0452,
  name: "Thread Border Router Management",
  features: &[
    FeatureDef { bit: 0x0000, code: "PC", name: "PANChange", summary: "The ability to change PAN configuration with pending dataset setting request." },
  ],
  attributes: &[
    attr::BorderRouterName::ATTR,
    attr::BorderAgentID::ATTR,
    attr::ThreadVersion::ATTR,
    attr::InterfaceEnabled::ATTR,
    attr::ActiveDatasetTimestamp::ATTR,
    attr::PendingDatasetTimestamp::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::GetActiveDatasetRequest::COMMAND,
    command::GetPendingDatasetRequest::COMMAND,
    command::SetActiveDatasetRequest::COMMAND,
    command::SetPendingDatasetRequest::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct BorderRouterName;

  impl Attr for BorderRouterName {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "BorderRouterName",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = String;
  }


  pub struct BorderAgentID;

  impl Attr for BorderAgentID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "BorderAgentID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = TypeUnsupported /* Octstr */;
  }


  pub struct ThreadVersion;

  impl Attr for ThreadVersion {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "ThreadVersion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u16;
  }


  pub struct InterfaceEnabled;

  impl Attr for InterfaceEnabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "InterfaceEnabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


  pub struct ActiveDatasetTimestamp;

  impl Attr for ActiveDatasetTimestamp {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "ActiveDatasetTimestamp",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u64>;
  }


  pub struct PendingDatasetTimestamp;

  impl Attr for PendingDatasetTimestamp {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "PendingDatasetTimestamp",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u64>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct GetActiveDatasetRequest {
}
impl Command for GetActiveDatasetRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "GetActiveDatasetRequest",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for GetActiveDatasetRequest {
  type Response = DatasetResponse;
}
impl TlvEncode for GetActiveDatasetRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetPendingDatasetRequest {
}
impl Command for GetPendingDatasetRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "GetPendingDatasetRequest",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for GetPendingDatasetRequest {
  type Response = DatasetResponse;
}
impl TlvEncode for GetPendingDatasetRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct DatasetResponse {
  pub dataset: TypeUnsupported /* Octstr */,
}
impl Command for DatasetResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "DatasetResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Dataset", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for DatasetResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct SetActiveDatasetRequest {
  pub active_dataset: TypeUnsupported /* Octstr */,
  pub breadcrumb: u64,
}
impl Command for SetActiveDatasetRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "SetActiveDatasetRequest",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ActiveDataset", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Breadcrumb", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for SetActiveDatasetRequest {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetActiveDatasetRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.active_dataset.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.breadcrumb.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetPendingDatasetRequest {
  pub pending_dataset: TypeUnsupported /* Octstr */,
}
impl Command for SetPendingDatasetRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "SetPendingDatasetRequest",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "PendingDataset", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SetPendingDatasetRequest {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetPendingDatasetRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.pending_dataset.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

}
pub mod ty {
  pub use crate::globals::*;
}
