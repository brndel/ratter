#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const THREAD_NETWORK_DIAGNOSTICS: ClusterDef = ClusterDef {
  id: 0x0035,
  name: "Thread Network Diagnostics",
  features: &[
    FeatureDef { bit: 0x0000, code: "PKTCNT", name: "PacketCounts", summary: "Server supports the counts for the number of received and transmitted packets on the Thread interface" },
    FeatureDef { bit: 0x0001, code: "ERRCNT", name: "ErrorCounts", summary: "Server supports the counts for the number of errors that have occurred during the reception and transmission of packets on the Thread interface" },
    FeatureDef { bit: 0x0002, code: "MLECNT", name: "MLECounts", summary: "Server supports the counts for various MLE layer happenings" },
    FeatureDef { bit: 0x0003, code: "MACCNT", name: "MACCounts", summary: "Server supports the counts for various MAC layer happenings" },
  ],
  attributes: &[
    attr::Channel::ATTR,
    attr::RoutingRole::ATTR,
    attr::NetworkName::ATTR,
    attr::PanId::ATTR,
    attr::ExtendedPanId::ATTR,
    attr::MeshLocalPrefix::ATTR,
    attr::OverrunCount::ATTR,
    attr::NeighborTable::ATTR,
    attr::RouteTable::ATTR,
    attr::PartitionId::ATTR,
    attr::Weighting::ATTR,
    attr::DataVersion::ATTR,
    attr::StableDataVersion::ATTR,
    attr::LeaderRouterId::ATTR,
    attr::DetachedRoleCount::ATTR,
    attr::ChildRoleCount::ATTR,
    attr::RouterRoleCount::ATTR,
    attr::LeaderRoleCount::ATTR,
    attr::AttachAttemptCount::ATTR,
    attr::PartitionIdChangeCount::ATTR,
    attr::BetterPartitionAttachAttemptCount::ATTR,
    attr::ParentChangeCount::ATTR,
    attr::TxTotalCount::ATTR,
    attr::TxUnicastCount::ATTR,
    attr::TxBroadcastCount::ATTR,
    attr::TxAckRequestedCount::ATTR,
    attr::TxAckedCount::ATTR,
    attr::TxNoAckRequestedCount::ATTR,
    attr::TxDataCount::ATTR,
    attr::TxDataPollCount::ATTR,
    attr::TxBeaconCount::ATTR,
    attr::TxBeaconRequestCount::ATTR,
    attr::TxOtherCount::ATTR,
    attr::TxRetryCount::ATTR,
    attr::TxDirectMaxRetryExpiryCount::ATTR,
    attr::TxIndirectMaxRetryExpiryCount::ATTR,
    attr::TxErrCcaCount::ATTR,
    attr::TxErrAbortCount::ATTR,
    attr::TxErrBusyChannelCount::ATTR,
    attr::RxTotalCount::ATTR,
    attr::RxUnicastCount::ATTR,
    attr::RxBroadcastCount::ATTR,
    attr::RxDataCount::ATTR,
    attr::RxDataPollCount::ATTR,
    attr::RxBeaconCount::ATTR,
    attr::RxBeaconRequestCount::ATTR,
    attr::RxOtherCount::ATTR,
    attr::RxAddressFilteredCount::ATTR,
    attr::RxDestAddrFilteredCount::ATTR,
    attr::RxDuplicatedCount::ATTR,
    attr::RxErrNoFrameCount::ATTR,
    attr::RxErrUnknownNeighborCount::ATTR,
    attr::RxErrInvalidSrcAddrCount::ATTR,
    attr::RxErrSecCount::ATTR,
    attr::RxErrFcsCount::ATTR,
    attr::RxErrOtherCount::ATTR,
    attr::ActiveTimestamp::ATTR,
    attr::PendingTimestamp::ATTR,
    attr::Delay::ATTR,
    attr::SecurityPolicy::ATTR,
    attr::ChannelPage0Mask::ATTR,
    attr::OperationalDatasetComponents::ATTR,
    attr::ActiveNetworkFaultsList::ATTR,
    attr::ExtAddress::ATTR,
    attr::Rloc16::ATTR,
  ],
  events: &[
    event::ConnectionStatus::EVENT,
    event::NetworkFaultChange::EVENT,
  ],
  commands: &[
    command::ResetCounts::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct Channel;

  impl Attr for Channel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "Channel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u16>;
  }


  pub struct RoutingRole;

  impl Attr for RoutingRole {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "RoutingRole",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::RoutingRoleEnum>;
  }


  pub struct NetworkName;

  impl Attr for NetworkName {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "NetworkName",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<String>;
  }


  pub struct PanId;

  impl Attr for PanId {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "PanId",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u16>;
  }


  pub struct ExtendedPanId;

  impl Attr for ExtendedPanId {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "ExtendedPanId",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u64>;
  }


  pub struct MeshLocalPrefix;

  impl Attr for MeshLocalPrefix {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "MeshLocalPrefix",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* Ipv6pre */>;
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


  pub struct NeighborTable;

  impl Attr for NeighborTable {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "NeighborTable",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::NeighborTableStruct>;
  }


  pub struct RouteTable;

  impl Attr for RouteTable {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "RouteTable",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::RouteTableStruct>;
  }


  pub struct PartitionId;

  impl Attr for PartitionId {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "PartitionId",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u32>;
  }


  pub struct Weighting;

  impl Attr for Weighting {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "Weighting",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u16>;
  }


  pub struct DataVersion;

  impl Attr for DataVersion {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "DataVersion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u16>;
  }


  pub struct StableDataVersion;

  impl Attr for StableDataVersion {
    const ATTR: AttrDef = AttrDef {
      id: 0x000c,
      name: "StableDataVersion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u16>;
  }


  pub struct LeaderRouterId;

  impl Attr for LeaderRouterId {
    const ATTR: AttrDef = AttrDef {
      id: 0x000d,
      name: "LeaderRouterId",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u8>;
  }


  pub struct DetachedRoleCount;

  impl Attr for DetachedRoleCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x000e,
      name: "DetachedRoleCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct ChildRoleCount;

  impl Attr for ChildRoleCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x000f,
      name: "ChildRoleCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct RouterRoleCount;

  impl Attr for RouterRoleCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0010,
      name: "RouterRoleCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct LeaderRoleCount;

  impl Attr for LeaderRoleCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0011,
      name: "LeaderRoleCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct AttachAttemptCount;

  impl Attr for AttachAttemptCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0012,
      name: "AttachAttemptCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct PartitionIdChangeCount;

  impl Attr for PartitionIdChangeCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0013,
      name: "PartitionIdChangeCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct BetterPartitionAttachAttemptCount;

  impl Attr for BetterPartitionAttachAttemptCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0014,
      name: "BetterPartitionAttachAttemptCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct ParentChangeCount;

  impl Attr for ParentChangeCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0015,
      name: "ParentChangeCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct TxTotalCount;

  impl Attr for TxTotalCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0016,
      name: "TxTotalCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxUnicastCount;

  impl Attr for TxUnicastCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0017,
      name: "TxUnicastCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxBroadcastCount;

  impl Attr for TxBroadcastCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0018,
      name: "TxBroadcastCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxAckRequestedCount;

  impl Attr for TxAckRequestedCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0019,
      name: "TxAckRequestedCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxAckedCount;

  impl Attr for TxAckedCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x001a,
      name: "TxAckedCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxNoAckRequestedCount;

  impl Attr for TxNoAckRequestedCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x001b,
      name: "TxNoAckRequestedCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxDataCount;

  impl Attr for TxDataCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x001c,
      name: "TxDataCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxDataPollCount;

  impl Attr for TxDataPollCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x001d,
      name: "TxDataPollCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxBeaconCount;

  impl Attr for TxBeaconCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x001e,
      name: "TxBeaconCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxBeaconRequestCount;

  impl Attr for TxBeaconRequestCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x001f,
      name: "TxBeaconRequestCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxOtherCount;

  impl Attr for TxOtherCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0020,
      name: "TxOtherCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxRetryCount;

  impl Attr for TxRetryCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0021,
      name: "TxRetryCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxDirectMaxRetryExpiryCount;

  impl Attr for TxDirectMaxRetryExpiryCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0022,
      name: "TxDirectMaxRetryExpiryCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxIndirectMaxRetryExpiryCount;

  impl Attr for TxIndirectMaxRetryExpiryCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0023,
      name: "TxIndirectMaxRetryExpiryCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxErrCcaCount;

  impl Attr for TxErrCcaCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0024,
      name: "TxErrCcaCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxErrAbortCount;

  impl Attr for TxErrAbortCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0025,
      name: "TxErrAbortCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct TxErrBusyChannelCount;

  impl Attr for TxErrBusyChannelCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0026,
      name: "TxErrBusyChannelCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxTotalCount;

  impl Attr for RxTotalCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0027,
      name: "RxTotalCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxUnicastCount;

  impl Attr for RxUnicastCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0028,
      name: "RxUnicastCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxBroadcastCount;

  impl Attr for RxBroadcastCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0029,
      name: "RxBroadcastCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxDataCount;

  impl Attr for RxDataCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x002a,
      name: "RxDataCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxDataPollCount;

  impl Attr for RxDataPollCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x002b,
      name: "RxDataPollCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxBeaconCount;

  impl Attr for RxBeaconCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x002c,
      name: "RxBeaconCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxBeaconRequestCount;

  impl Attr for RxBeaconRequestCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x002d,
      name: "RxBeaconRequestCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxOtherCount;

  impl Attr for RxOtherCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x002e,
      name: "RxOtherCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxAddressFilteredCount;

  impl Attr for RxAddressFilteredCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x002f,
      name: "RxAddressFilteredCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxDestAddrFilteredCount;

  impl Attr for RxDestAddrFilteredCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0030,
      name: "RxDestAddrFilteredCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxDuplicatedCount;

  impl Attr for RxDuplicatedCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0031,
      name: "RxDuplicatedCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxErrNoFrameCount;

  impl Attr for RxErrNoFrameCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0032,
      name: "RxErrNoFrameCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxErrUnknownNeighborCount;

  impl Attr for RxErrUnknownNeighborCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0033,
      name: "RxErrUnknownNeighborCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxErrInvalidSrcAddrCount;

  impl Attr for RxErrInvalidSrcAddrCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0034,
      name: "RxErrInvalidSrcAddrCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxErrSecCount;

  impl Attr for RxErrSecCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0035,
      name: "RxErrSecCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxErrFcsCount;

  impl Attr for RxErrFcsCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0036,
      name: "RxErrFcsCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct RxErrOtherCount;

  impl Attr for RxErrOtherCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0037,
      name: "RxErrOtherCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct ActiveTimestamp;

  impl Attr for ActiveTimestamp {
    const ATTR: AttrDef = AttrDef {
      id: 0x0038,
      name: "ActiveTimestamp",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u64>;
  }


  pub struct PendingTimestamp;

  impl Attr for PendingTimestamp {
    const ATTR: AttrDef = AttrDef {
      id: 0x0039,
      name: "PendingTimestamp",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u64>;
  }


  pub struct Delay;

  impl Attr for Delay {
    const ATTR: AttrDef = AttrDef {
      id: 0x003a,
      name: "Delay",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u32>;
  }


  pub struct SecurityPolicy;

  impl Attr for SecurityPolicy {
    const ATTR: AttrDef = AttrDef {
      id: 0x003b,
      name: "SecurityPolicy",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::SecurityPolicy>;
  }


  pub struct ChannelPage0Mask;

  impl Attr for ChannelPage0Mask {
    const ATTR: AttrDef = AttrDef {
      id: 0x003c,
      name: "ChannelPage0Mask",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* Octstr */>;
  }


  pub struct OperationalDatasetComponents;

  impl Attr for OperationalDatasetComponents {
    const ATTR: AttrDef = AttrDef {
      id: 0x003d,
      name: "OperationalDatasetComponents",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::OperationalDatasetComponents>;
  }


  pub struct ActiveNetworkFaultsList;

  impl Attr for ActiveNetworkFaultsList {
    const ATTR: AttrDef = AttrDef {
      id: 0x003e,
      name: "ActiveNetworkFaultsList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::NetworkFaultEnum>;
  }


  pub struct ExtAddress;

  impl Attr for ExtAddress {
    const ATTR: AttrDef = AttrDef {
      id: 0x003f,
      name: "ExtAddress",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<u64>;
  }


  pub struct Rloc16;

  impl Attr for Rloc16 {
    const ATTR: AttrDef = AttrDef {
      id: 0x0040,
      name: "Rloc16",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<u16>;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ConnectionStatus {
  pub connection_status: ty::ConnectionStatusEnum,
}

impl Event for ConnectionStatus {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "ConnectionStatus",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "ConnectionStatus", attr_ty: <ty::ConnectionStatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for ConnectionStatus {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut connection_status: Option<ty::ConnectionStatusEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { connection_status = Some(<ty::ConnectionStatusEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      connection_status: connection_status.ok_or(TlvDecodeErr::MissingField("ConnectionStatus"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct NetworkFaultChange {
  pub current: Vec<ty::NetworkFaultEnum>,
  pub previous: Vec<ty::NetworkFaultEnum>,
}

impl Event for NetworkFaultChange {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "NetworkFaultChange",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "Current", attr_ty: <Vec<ty::NetworkFaultEnum>>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Previous", attr_ty: <Vec<ty::NetworkFaultEnum>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for NetworkFaultChange {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut current: Option<Vec<ty::NetworkFaultEnum>> = None;
    let mut previous: Option<Vec<ty::NetworkFaultEnum>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { current = Some(<Vec<ty::NetworkFaultEnum>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { previous = Some(<Vec<ty::NetworkFaultEnum>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      current: current.ok_or(TlvDecodeErr::MissingField("Current"))?,
      previous: previous.ok_or(TlvDecodeErr::MissingField("Previous"))?,
    })
  }
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
    conformance: Conformance::Feature(1),
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
pub enum ConnectionStatusEnum {
  ///Node is connected
  Connected = 0x0000,
  ///Node is not connected
  NotConnected = 0x0001,
}
impl AttrType for ConnectionStatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Connected"),
  (0x0001, "NotConnected"),
  ]);
}
impl TlvPut for ConnectionStatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ConnectionStatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Connected),
      0x0001 => Ok(Self::NotConnected),
      _ => Err(())
    }
  }
}
impl TlvDecode for ConnectionStatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum NetworkFaultEnum {
  ///Indicates an unspecified fault.
  Unspecified = 0x0000,
  ///Indicates the Thread link is down.
  LinkDown = 0x0001,
  ///Indicates there has been Thread hardware failure.
  HardwareFailure = 0x0002,
  ///Indicates the Thread network is jammed.
  NetworkJammed = 0x0003,
}
impl AttrType for NetworkFaultEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "LinkDown"),
  (0x0002, "HardwareFailure"),
  (0x0003, "NetworkJammed"),
  ]);
}
impl TlvPut for NetworkFaultEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for NetworkFaultEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unspecified),
      0x0001 => Ok(Self::LinkDown),
      0x0002 => Ok(Self::HardwareFailure),
      0x0003 => Ok(Self::NetworkJammed),
      _ => Err(())
    }
  }
}
impl TlvDecode for NetworkFaultEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum RoutingRoleEnum {
  ///Unspecified routing role.
  Unspecified = 0x0000,
  ///The Node does not currently have a role as a result of the Thread interface not currently being configured or operational.
  Unassigned = 0x0001,
  ///The Node acts as a Sleepy End Device with RX-off-when-idle sleepy radio behavior.
  SleepyEndDevice = 0x0002,
  ///The Node acts as an End Device without RX-off-when-idle sleepy radio behavior.
  EndDevice = 0x0003,
  ///The Node acts as an Router Eligible End Device.
  REED = 0x0004,
  ///The Node acts as a Router Device.
  Router = 0x0005,
  ///The Node acts as a Leader Device.
  Leader = 0x0006,
}
impl AttrType for RoutingRoleEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "Unassigned"),
  (0x0002, "SleepyEndDevice"),
  (0x0003, "EndDevice"),
  (0x0004, "REED"),
  (0x0005, "Router"),
  (0x0006, "Leader"),
  ]);
}
impl TlvPut for RoutingRoleEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for RoutingRoleEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unspecified),
      0x0001 => Ok(Self::Unassigned),
      0x0002 => Ok(Self::SleepyEndDevice),
      0x0003 => Ok(Self::EndDevice),
      0x0004 => Ok(Self::REED),
      0x0005 => Ok(Self::Router),
      0x0006 => Ok(Self::Leader),
      _ => Err(())
    }
  }
}
impl TlvDecode for RoutingRoleEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct NeighborTableStruct {
  pub ext_address: u64,
  pub age: u32,
  pub rloc16: u16,
  pub link_frame_counter: u32,
  pub mle_frame_counter: u32,
  pub lqi: u8,
  pub average_rssi: i8,
  pub last_rssi: i8,
  pub frame_error_rate: u8,
  pub message_error_rate: u8,
  pub rx_on_when_idle: bool,
  pub full_thread_device: bool,
  pub full_network_data: bool,
  pub is_child: bool,
}
impl AttrType for NeighborTableStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ExtAddress", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Age", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Rloc16", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "LinkFrameCounter", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "MleFrameCounter", attr_ty: <u32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "LQI", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "AverageRssi", attr_ty: <i8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "LastRssi", attr_ty: <i8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0008, name: "FrameErrorRate", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0009, name: "MessageErrorRate", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000a, name: "RxOnWhenIdle", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000b, name: "FullThreadDevice", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000c, name: "FullNetworkData", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000d, name: "IsChild", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for NeighborTableStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.ext_address.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.age.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.rloc16.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.link_frame_counter.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.mle_frame_counter.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.lqi.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.average_rssi.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.last_rssi.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.frame_error_rate.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    self.message_error_rate.tlv_put(matter_codec::Tag::Context(0x0009), writer);
    self.rx_on_when_idle.tlv_put(matter_codec::Tag::Context(0x000a), writer);
    self.full_thread_device.tlv_put(matter_codec::Tag::Context(0x000b), writer);
    self.full_network_data.tlv_put(matter_codec::Tag::Context(0x000c), writer);
    self.is_child.tlv_put(matter_codec::Tag::Context(0x000d), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for NeighborTableStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut ext_address: Option<u64> = None;
    let mut age: Option<u32> = None;
    let mut rloc16: Option<u16> = None;
    let mut link_frame_counter: Option<u32> = None;
    let mut mle_frame_counter: Option<u32> = None;
    let mut lqi: Option<u8> = None;
    let mut average_rssi: Option<i8> = None;
    let mut last_rssi: Option<i8> = None;
    let mut frame_error_rate: Option<u8> = None;
    let mut message_error_rate: Option<u8> = None;
    let mut rx_on_when_idle: Option<bool> = None;
    let mut full_thread_device: Option<bool> = None;
    let mut full_network_data: Option<bool> = None;
    let mut is_child: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { ext_address = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { age = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { rloc16 = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { link_frame_counter = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { mle_frame_counter = Some(<u32>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { lqi = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { average_rssi = Some(<i8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0007), value: _ }) => { last_rssi = Some(<i8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0008), value: _ }) => { frame_error_rate = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0009), value: _ }) => { message_error_rate = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000a), value: _ }) => { rx_on_when_idle = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000b), value: _ }) => { full_thread_device = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000c), value: _ }) => { full_network_data = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000d), value: _ }) => { is_child = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      ext_address: ext_address.ok_or(TlvDecodeErr::MissingField("ExtAddress"))?,
      age: age.ok_or(TlvDecodeErr::MissingField("Age"))?,
      rloc16: rloc16.ok_or(TlvDecodeErr::MissingField("Rloc16"))?,
      link_frame_counter: link_frame_counter.ok_or(TlvDecodeErr::MissingField("LinkFrameCounter"))?,
      mle_frame_counter: mle_frame_counter.ok_or(TlvDecodeErr::MissingField("MleFrameCounter"))?,
      lqi: lqi.ok_or(TlvDecodeErr::MissingField("LQI"))?,
      average_rssi: average_rssi.ok_or(TlvDecodeErr::MissingField("AverageRssi"))?,
      last_rssi: last_rssi.ok_or(TlvDecodeErr::MissingField("LastRssi"))?,
      frame_error_rate: frame_error_rate.ok_or(TlvDecodeErr::MissingField("FrameErrorRate"))?,
      message_error_rate: message_error_rate.ok_or(TlvDecodeErr::MissingField("MessageErrorRate"))?,
      rx_on_when_idle: rx_on_when_idle.ok_or(TlvDecodeErr::MissingField("RxOnWhenIdle"))?,
      full_thread_device: full_thread_device.ok_or(TlvDecodeErr::MissingField("FullThreadDevice"))?,
      full_network_data: full_network_data.ok_or(TlvDecodeErr::MissingField("FullNetworkData"))?,
      is_child: is_child.ok_or(TlvDecodeErr::MissingField("IsChild"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct OperationalDatasetComponents {
  pub active_timestamp_present: bool,
  pub pending_timestamp_present: bool,
  pub master_key_present: bool,
  pub network_name_present: bool,
  pub extended_pan_id_present: bool,
  pub mesh_local_prefix_present: bool,
  pub delay_present: bool,
  pub pan_id_present: bool,
  pub channel_present: bool,
  pub pskc_present: bool,
  pub security_policy_present: bool,
  pub channel_mask_present: bool,
}
impl AttrType for OperationalDatasetComponents {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ActiveTimestampPresent", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "PendingTimestampPresent", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "MasterKeyPresent", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "NetworkNamePresent", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "ExtendedPanIdPresent", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "MeshLocalPrefixPresent", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "DelayPresent", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "PanIdPresent", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0008, name: "ChannelPresent", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0009, name: "PskcPresent", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000a, name: "SecurityPolicyPresent", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000b, name: "ChannelMaskPresent", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for OperationalDatasetComponents {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.active_timestamp_present.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.pending_timestamp_present.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.master_key_present.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.network_name_present.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.extended_pan_id_present.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.mesh_local_prefix_present.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.delay_present.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.pan_id_present.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.channel_present.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    self.pskc_present.tlv_put(matter_codec::Tag::Context(0x0009), writer);
    self.security_policy_present.tlv_put(matter_codec::Tag::Context(0x000a), writer);
    self.channel_mask_present.tlv_put(matter_codec::Tag::Context(0x000b), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for OperationalDatasetComponents {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut active_timestamp_present: Option<bool> = None;
    let mut pending_timestamp_present: Option<bool> = None;
    let mut master_key_present: Option<bool> = None;
    let mut network_name_present: Option<bool> = None;
    let mut extended_pan_id_present: Option<bool> = None;
    let mut mesh_local_prefix_present: Option<bool> = None;
    let mut delay_present: Option<bool> = None;
    let mut pan_id_present: Option<bool> = None;
    let mut channel_present: Option<bool> = None;
    let mut pskc_present: Option<bool> = None;
    let mut security_policy_present: Option<bool> = None;
    let mut channel_mask_present: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { active_timestamp_present = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { pending_timestamp_present = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { master_key_present = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { network_name_present = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { extended_pan_id_present = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { mesh_local_prefix_present = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { delay_present = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0007), value: _ }) => { pan_id_present = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0008), value: _ }) => { channel_present = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0009), value: _ }) => { pskc_present = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000a), value: _ }) => { security_policy_present = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000b), value: _ }) => { channel_mask_present = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      active_timestamp_present: active_timestamp_present.ok_or(TlvDecodeErr::MissingField("ActiveTimestampPresent"))?,
      pending_timestamp_present: pending_timestamp_present.ok_or(TlvDecodeErr::MissingField("PendingTimestampPresent"))?,
      master_key_present: master_key_present.ok_or(TlvDecodeErr::MissingField("MasterKeyPresent"))?,
      network_name_present: network_name_present.ok_or(TlvDecodeErr::MissingField("NetworkNamePresent"))?,
      extended_pan_id_present: extended_pan_id_present.ok_or(TlvDecodeErr::MissingField("ExtendedPanIdPresent"))?,
      mesh_local_prefix_present: mesh_local_prefix_present.ok_or(TlvDecodeErr::MissingField("MeshLocalPrefixPresent"))?,
      delay_present: delay_present.ok_or(TlvDecodeErr::MissingField("DelayPresent"))?,
      pan_id_present: pan_id_present.ok_or(TlvDecodeErr::MissingField("PanIdPresent"))?,
      channel_present: channel_present.ok_or(TlvDecodeErr::MissingField("ChannelPresent"))?,
      pskc_present: pskc_present.ok_or(TlvDecodeErr::MissingField("PskcPresent"))?,
      security_policy_present: security_policy_present.ok_or(TlvDecodeErr::MissingField("SecurityPolicyPresent"))?,
      channel_mask_present: channel_mask_present.ok_or(TlvDecodeErr::MissingField("ChannelMaskPresent"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RouteTableStruct {
  pub ext_address: u64,
  pub rloc16: u16,
  pub router_id: u8,
  pub next_hop: u8,
  pub path_cost: u8,
  pub lqi_in: u8,
  pub lqi_out: u8,
  pub age: u8,
  pub allocated: bool,
  pub link_established: bool,
}
impl AttrType for RouteTableStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ExtAddress", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Rloc16", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "RouterId", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "NextHop", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "PathCost", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "LQIIn", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "LQIOut", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "Age", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0008, name: "Allocated", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0009, name: "LinkEstablished", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for RouteTableStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.ext_address.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.rloc16.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.router_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.next_hop.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.path_cost.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.lqi_in.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.lqi_out.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.age.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.allocated.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    self.link_established.tlv_put(matter_codec::Tag::Context(0x0009), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for RouteTableStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut ext_address: Option<u64> = None;
    let mut rloc16: Option<u16> = None;
    let mut router_id: Option<u8> = None;
    let mut next_hop: Option<u8> = None;
    let mut path_cost: Option<u8> = None;
    let mut lqi_in: Option<u8> = None;
    let mut lqi_out: Option<u8> = None;
    let mut age: Option<u8> = None;
    let mut allocated: Option<bool> = None;
    let mut link_established: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { ext_address = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { rloc16 = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { router_id = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { next_hop = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { path_cost = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { lqi_in = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { lqi_out = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0007), value: _ }) => { age = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0008), value: _ }) => { allocated = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0009), value: _ }) => { link_established = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      ext_address: ext_address.ok_or(TlvDecodeErr::MissingField("ExtAddress"))?,
      rloc16: rloc16.ok_or(TlvDecodeErr::MissingField("Rloc16"))?,
      router_id: router_id.ok_or(TlvDecodeErr::MissingField("RouterId"))?,
      next_hop: next_hop.ok_or(TlvDecodeErr::MissingField("NextHop"))?,
      path_cost: path_cost.ok_or(TlvDecodeErr::MissingField("PathCost"))?,
      lqi_in: lqi_in.ok_or(TlvDecodeErr::MissingField("LQIIn"))?,
      lqi_out: lqi_out.ok_or(TlvDecodeErr::MissingField("LQIOut"))?,
      age: age.ok_or(TlvDecodeErr::MissingField("Age"))?,
      allocated: allocated.ok_or(TlvDecodeErr::MissingField("Allocated"))?,
      link_established: link_established.ok_or(TlvDecodeErr::MissingField("LinkEstablished"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SecurityPolicy {
  pub rotation_time: u16,
  pub flags: u16,
}
impl AttrType for SecurityPolicy {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "RotationTime", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Flags", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for SecurityPolicy {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.rotation_time.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.flags.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for SecurityPolicy {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut rotation_time: Option<u16> = None;
    let mut flags: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { rotation_time = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { flags = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      rotation_time: rotation_time.ok_or(TlvDecodeErr::MissingField("RotationTime"))?,
      flags: flags.ok_or(TlvDecodeErr::MissingField("Flags"))?,
    })
  }
}

}
