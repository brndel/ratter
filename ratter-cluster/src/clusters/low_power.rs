#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const LOW_POWER: ClusterDef = ClusterDef {
  id: 0x0508,
  name: "Low Power",
  features: &[
  ],
  attributes: &[
  ],
  events: &[
  ],
  commands: &[
    command::Sleep::COMMAND,
  ],
};
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Sleep {
}
impl Command for Sleep {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "Sleep",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for Sleep {
  type Response = BasicAccResponse;
}
impl TlvEncode for Sleep {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

}
pub mod ty {
  pub use crate::globals::*;
}
