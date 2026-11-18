#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const LAUNDRY_WASHER_CONTROLS: ClusterDef = ClusterDef {
  id: 0x0053,
  name: "Laundry Washer Controls",
  features: &[
    FeatureDef { bit: 0x0000, code: "SPIN", name: "Spin", summary: "Multiple spin speeds supported" },
    FeatureDef { bit: 0x0001, code: "RINSE", name: "Rinse", summary: "Multiple rinse cycles supported" },
  ],
  attributes: &[
    attr::SpinSpeeds::ATTR,
    attr::SpinSpeedCurrent::ATTR,
    attr::NumberOfRinses::ATTR,
    attr::SupportedRinses::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct SpinSpeeds;

  impl Attr for SpinSpeeds {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "SpinSpeeds",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<String>;
  }


  pub struct SpinSpeedCurrent;

  impl Attr for SpinSpeedCurrent {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "SpinSpeedCurrent",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<u8>;
  }


  pub struct NumberOfRinses;

  impl Attr for NumberOfRinses {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "NumberOfRinses",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = ty::NumberOfRinsesEnum;
  }


  pub struct SupportedRinses;

  impl Attr for SupportedRinses {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "SupportedRinses",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Vec<ty::NumberOfRinsesEnum>;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum NumberOfRinsesEnum {
  ///This laundry washer mode does not perform rinse cycles
  None = 0x0000,
  ///This laundry washer mode performs normal rinse cycles determined by the manufacturer
  Normal = 0x0001,
  ///This laundry washer mode performs an extra rinse cycle
  Extra = 0x0002,
  ///This laundry washer mode performs the maximum number of rinse cycles determined by the manufacturer
  Max = 0x0003,
}
impl AttrType for NumberOfRinsesEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "None"),
  (0x0001, "Normal"),
  (0x0002, "Extra"),
  (0x0003, "Max"),
  ]);
}
impl TlvPut for NumberOfRinsesEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for NumberOfRinsesEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::None),
      0x0001 => Ok(Self::Normal),
      0x0002 => Ok(Self::Extra),
      0x0003 => Ok(Self::Max),
      _ => Err(())
    }
  }
}
impl TlvDecode for NumberOfRinsesEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
