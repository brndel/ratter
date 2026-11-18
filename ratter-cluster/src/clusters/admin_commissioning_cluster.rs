#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const ADMINISTRATOR_COMMISSIONING: ClusterDef = ClusterDef {
  id: 0x003c,
  name: "Administrator Commissioning",
  features: &[
    FeatureDef { bit: 0x0000, code: "BC", name: "Basic", summary: "Node supports Basic Commissioning Method" },
  ],
  attributes: &[
    attr::WindowStatus::ATTR,
    attr::AdminFabricIndex::ATTR,
    attr::AdminVendorId::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::OpenCommissioningWindow::COMMAND,
    command::OpenBasicCommissioningWindow::COMMAND,
    command::RevokeCommissioning::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct WindowStatus;

  impl Attr for WindowStatus {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "WindowStatus",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::CommissioningWindowStatusEnum;
  }


  pub struct AdminFabricIndex;

  impl Attr for AdminFabricIndex {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "AdminFabricIndex",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* FabricIdx */>;
  }


  pub struct AdminVendorId;

  impl Attr for AdminVendorId {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "AdminVendorId",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* VendorId */>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct OpenCommissioningWindow {
  pub commissioning_timeout: u16,
  pub pake_passcode_verifier: TypeUnsupported /* Octstr */,
  pub discriminator: u16,
  pub iterations: u32,
  pub salt: TypeUnsupported /* Octstr */,
}
impl Command for OpenCommissioningWindow {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "OpenCommissioningWindow",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "CommissioningTimeout", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "PAKEPasscodeVerifier", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Discriminator", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0003, name: "Iterations", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0004, name: "Salt", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for OpenCommissioningWindow {
  type Response = BasicAccResponse;
}
impl TlvEncode for OpenCommissioningWindow {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.commissioning_timeout.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.pake_passcode_verifier.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.discriminator.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.iterations.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.salt.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OpenBasicCommissioningWindow {
  pub commissioning_timeout: u16,
}
impl Command for OpenBasicCommissioningWindow {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "OpenBasicCommissioningWindow",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "CommissioningTimeout", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for OpenBasicCommissioningWindow {
  type Response = BasicAccResponse;
}
impl TlvEncode for OpenBasicCommissioningWindow {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.commissioning_timeout.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RevokeCommissioning {
}
impl Command for RevokeCommissioning {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "RevokeCommissioning",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for RevokeCommissioning {
  type Response = BasicAccResponse;
}
impl TlvEncode for RevokeCommissioning {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
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
pub enum CommissioningWindowStatusEnum {
  ///Commissioning window not open
  WindowNotOpen = 0x0000,
  ///An Enhanced Commissioning Method window is open
  EnhancedWindowOpen = 0x0001,
  ///A Basic Commissioning Method window is open
  BasicWindowOpen = 0x0002,
}
impl AttrType for CommissioningWindowStatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "WindowNotOpen"),
  (0x0001, "EnhancedWindowOpen"),
  (0x0002, "BasicWindowOpen"),
  ]);
}
impl TlvPut for CommissioningWindowStatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for CommissioningWindowStatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::WindowNotOpen),
      0x0001 => Ok(Self::EnhancedWindowOpen),
      0x0002 => Ok(Self::BasicWindowOpen),
      _ => Err(())
    }
  }
}
impl TlvDecode for CommissioningWindowStatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StatusCodeEnum {
  ///Could not be completed because another commissioning is in progress
  Busy = 0x0002,
  ///Provided PAKE parameters were incorrectly formatted or otherwise invalid
  PAKEParameterError = 0x0003,
  ///No commissioning window was currently open
  WindowNotOpen = 0x0004,
}
impl AttrType for StatusCodeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0002, "Busy"),
  (0x0003, "PAKEParameterError"),
  (0x0004, "WindowNotOpen"),
  ]);
}
impl TlvPut for StatusCodeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StatusCodeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0002 => Ok(Self::Busy),
      0x0003 => Ok(Self::PAKEParameterError),
      0x0004 => Ok(Self::WindowNotOpen),
      _ => Err(())
    }
  }
}
impl TlvDecode for StatusCodeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
