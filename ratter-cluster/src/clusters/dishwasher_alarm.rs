#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const DISHWASHER_ALARM: ClusterDef = ClusterDef {
  id: 0x005d,
  name: "Dishwasher Alarm",
  features: &[
  ],
  attributes: &[
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod ty {
  pub use crate::globals::*;
  use super::*;
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct AlarmBitmap: u32 {
    ///Water inflow is abnormal
    const InflowError = 1 << 0;
    ///Water draining is abnormal
    const DrainError = 1 << 1;
    ///Door or door lock is abnormal
    const DoorError = 1 << 2;
    ///Unable to reach normal temperature
    const TempTooLow = 1 << 3;
    ///Temperature is too high
    const TempTooHigh = 1 << 4;
    ///Water level is abnormal
    const WaterLevelError = 1 << 5;
  }
}
impl AttrType for AlarmBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "InflowError"),
    (1, "DrainError"),
    (2, "DoorError"),
    (3, "TempTooLow"),
    (4, "TempTooHigh"),
    (5, "WaterLevelError"),
  ]);
}
impl TlvPut for AlarmBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for AlarmBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for AlarmBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
}
