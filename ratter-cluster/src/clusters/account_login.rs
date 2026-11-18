#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const ACCOUNT_LOGIN: ClusterDef = ClusterDef {
  id: 0x050e,
  name: "Account Login",
  features: &[
  ],
  attributes: &[
  ],
  events: &[
    event::LoggedOut::EVENT,
  ],
  commands: &[
    command::GetSetupPIN::COMMAND,
    command::Login::COMMAND,
    command::Logout::COMMAND,
  ],
};
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct LoggedOut {
  pub node: TypeUnsupported /* NodeId */,
}

impl Event for LoggedOut {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "LoggedOut",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "Node", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}

impl TlvDecode for LoggedOut {
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
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct GetSetupPIN {
  pub temp_account_identifier: String,
}
impl Command for GetSetupPIN {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "GetSetupPIN",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "TempAccountIdentifier", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for GetSetupPIN {
  type Response = GetSetupPINResponse;
}
impl TlvEncode for GetSetupPIN {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.temp_account_identifier.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetSetupPINResponse {
  pub setup_pin: String,
}
impl Command for GetSetupPINResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "GetSetupPINResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "SetupPIN", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for GetSetupPINResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct Login {
  pub temp_account_identifier: String,
  pub setup_pin: String,
  pub node: TypeUnsupported /* NodeId */,
}
impl Command for Login {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "Login",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "TempAccountIdentifier", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "SetupPIN", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Node", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for Login {
  type Response = BasicAccResponse;
}
impl TlvEncode for Login {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.temp_account_identifier.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.setup_pin.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.node.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Logout {
  pub node: TypeUnsupported /* NodeId */,
}
impl Command for Logout {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "Logout",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Node", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for Logout {
  type Response = BasicAccResponse;
}
impl TlvEncode for Logout {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.node.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

}
pub mod ty {
  pub use crate::globals::*;
}
