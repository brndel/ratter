#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const LAUNDRY_DRYER_CONTROLS: ClusterDef = ClusterDef {
  id: 0x004a,
  name: "Laundry Dryer Controls",
  features: &[
  ],
  attributes: &[
    attr::SupportedDrynessLevels::ATTR,
    attr::SelectedDrynessLevel::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct SupportedDrynessLevels;

  impl Attr for SupportedDrynessLevels {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "SupportedDrynessLevels",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::DrynessLevelEnum>;
  }


  pub struct SelectedDrynessLevel;

  impl Attr for SelectedDrynessLevel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "SelectedDrynessLevel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::DrynessLevelEnum>;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum DrynessLevelEnum {
  ///Provides a low dryness level for the selected mode
  Low = 0x0000,
  ///Provides the normal level of dryness for the selected mode
  Normal = 0x0001,
  ///Provides an extra dryness level for the selected mode
  Extra = 0x0002,
  ///Provides the max dryness level for the selected mode
  Max = 0x0003,
}
impl AttrType for DrynessLevelEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Low"),
  (0x0001, "Normal"),
  (0x0002, "Extra"),
  (0x0003, "Max"),
  ]);
}
impl TlvPut for DrynessLevelEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for DrynessLevelEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Low),
      0x0001 => Ok(Self::Normal),
      0x0002 => Ok(Self::Extra),
      0x0003 => Ok(Self::Max),
      _ => Err(())
    }
  }
}
impl TlvDecode for DrynessLevelEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
