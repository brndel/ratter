#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const WI_FI_NETWORK_MANAGEMENT: ClusterDef = ClusterDef {
  id: 0x0451,
  name: "Wi_Fi Network Management",
  features: &[
  ],
  attributes: &[
    attr::SSID::ATTR,
    attr::PassphraseSurrogate::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::NetworkPassphraseRequest::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct SSID;

  impl Attr for SSID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "SSID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* Octstr */>;
  }


  pub struct PassphraseSurrogate;

  impl Attr for PassphraseSurrogate {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "PassphraseSurrogate",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u64>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkPassphraseRequest {
}
impl Command for NetworkPassphraseRequest {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "NetworkPassphraseRequest",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for NetworkPassphraseRequest {
  type Response = NetworkPassphraseResponse;
}
impl TlvEncode for NetworkPassphraseRequest {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkPassphraseResponse {
  pub passphrase: TypeUnsupported /* Octstr */,
}
impl Command for NetworkPassphraseResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "NetworkPassphraseResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Passphrase", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for NetworkPassphraseResponse {}

}
pub mod ty {
  pub use crate::globals::*;
}
