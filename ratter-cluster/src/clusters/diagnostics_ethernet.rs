#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const ETHERNET_NETWORK_DIAGNOSTICS: ClusterDef = ClusterDef {
  id: 0x0037,
  name: "Ethernet Network Diagnostics",
  features: &[
    FeatureDef { bit: 0x0000, code: "PKTCNT", name: "PacketCounts", summary: "Node makes available the counts for the number of received and transmitted packets on the ethernet interface" },
    FeatureDef { bit: 0x0001, code: "ERRCNT", name: "ErrorCounts", summary: "Node makes available the counts for the number of errors that have occurred during the reception and transmission of packets on the ethernet interface" },
  ],
  attributes: &[
    attr::PHYRate::ATTR,
    attr::FullDuplex::ATTR,
    attr::PacketRxCount::ATTR,
    attr::PacketTxCount::ATTR,
    attr::TxErrCount::ATTR,
    attr::CollisionCount::ATTR,
    attr::OverrunCount::ATTR,
    attr::CarrierDetect::ATTR,
    attr::TimeSinceReset::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::ResetCounts::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct PHYRate;

  impl Attr for PHYRate {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "PHYRate",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<ty::PHYRateEnum>;
  }


  pub struct FullDuplex;

  impl Attr for FullDuplex {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "FullDuplex",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<bool>;
  }


  pub struct PacketRxCount;

  impl Attr for PacketRxCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "PacketRxCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u64;
  }


  pub struct PacketTxCount;

  impl Attr for PacketTxCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "PacketTxCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u64;
  }


  pub struct TxErrCount;

  impl Attr for TxErrCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "TxErrCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = u64;
  }


  pub struct CollisionCount;

  impl Attr for CollisionCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "CollisionCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = u64;
  }


  pub struct OverrunCount;

  impl Attr for OverrunCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "OverrunCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = u64;
  }


  pub struct CarrierDetect;

  impl Attr for CarrierDetect {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "CarrierDetect",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<bool>;
  }


  pub struct TimeSinceReset;

  impl Attr for TimeSinceReset {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "TimeSinceReset",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u64;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ResetCounts {
}
impl Command for ResetCounts {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "ResetCounts",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for ResetCounts {
  type Response = BasicAccResponse;
}
impl TlvEncode for ResetCounts {
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
pub enum PHYRateEnum {
  ///PHY rate is 10Mbps
  Rate10M = 0x0000,
  ///PHY rate is 100Mbps
  Rate100M = 0x0001,
  ///PHY rate is 1Gbps
  Rate1G = 0x0002,
  ///PHY rate is 2.5Gbps
  Rate2_5G = 0x0003,
  ///PHY rate is 5Gbps
  Rate5G = 0x0004,
  ///PHY rate is 10Gbps
  Rate10G = 0x0005,
  ///PHY rate is 40Gbps
  Rate40G = 0x0006,
  ///PHY rate is 100Gbps
  Rate100G = 0x0007,
  ///PHY rate is 200Gbps
  Rate200G = 0x0008,
  ///PHY rate is 400Gbps
  Rate400G = 0x0009,
}
impl AttrType for PHYRateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Rate10M"),
  (0x0001, "Rate100M"),
  (0x0002, "Rate1G"),
  (0x0003, "Rate2_5G"),
  (0x0004, "Rate5G"),
  (0x0005, "Rate10G"),
  (0x0006, "Rate40G"),
  (0x0007, "Rate100G"),
  (0x0008, "Rate200G"),
  (0x0009, "Rate400G"),
  ]);
}
impl TlvPut for PHYRateEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for PHYRateEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Rate10M),
      0x0001 => Ok(Self::Rate100M),
      0x0002 => Ok(Self::Rate1G),
      0x0003 => Ok(Self::Rate2_5G),
      0x0004 => Ok(Self::Rate5G),
      0x0005 => Ok(Self::Rate10G),
      0x0006 => Ok(Self::Rate40G),
      0x0007 => Ok(Self::Rate100G),
      0x0008 => Ok(Self::Rate200G),
      0x0009 => Ok(Self::Rate400G),
      _ => Err(())
    }
  }
}
impl TlvDecode for PHYRateEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
