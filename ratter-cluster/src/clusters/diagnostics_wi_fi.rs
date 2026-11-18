#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const WI_FI_NETWORK_DIAGNOSTICS: ClusterDef = ClusterDef {
  id: 0x0036,
  name: "Wi_Fi Network Diagnostics",
  features: &[
    FeatureDef { bit: 0x0000, code: "PKTCNT", name: "PacketCounts", summary: "Node makes available the counts for the number of received and transmitted packets on the Wi-Fi interface" },
    FeatureDef { bit: 0x0001, code: "ERRCNT", name: "ErrorCounts", summary: "Node makes available the counts for the number of errors that have occurred during the reception and transmission of packets on the Wi-Fi interface" },
  ],
  attributes: &[
    attr::BSSID::ATTR,
    attr::SecurityType::ATTR,
    attr::WiFiVersion::ATTR,
    attr::ChannelNumber::ATTR,
    attr::RSSI::ATTR,
    attr::BeaconLostCount::ATTR,
    attr::BeaconRxCount::ATTR,
    attr::PacketMulticastRxCount::ATTR,
    attr::PacketMulticastTxCount::ATTR,
    attr::PacketUnicastRxCount::ATTR,
    attr::PacketUnicastTxCount::ATTR,
    attr::CurrentMaxRate::ATTR,
    attr::OverrunCount::ATTR,
  ],
  events: &[
    event::Disconnection::EVENT,
    event::AssociationFailure::EVENT,
    event::ConnectionStatus::EVENT,
  ],
  commands: &[
    command::ResetCounts::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct BSSID;

  impl Attr for BSSID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "BSSID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* Octstr */>;
  }


  pub struct SecurityType;

  impl Attr for SecurityType {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "SecurityType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::SecurityTypeEnum>;
  }


  pub struct WiFiVersion;

  impl Attr for WiFiVersion {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "WiFiVersion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::WiFiVersionEnum>;
  }


  pub struct ChannelNumber;

  impl Attr for ChannelNumber {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "ChannelNumber",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u16>;
  }


  pub struct RSSI;

  impl Attr for RSSI {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "RSSI",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<i8>;
  }


  pub struct BeaconLostCount;

  impl Attr for BeaconLostCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "BeaconLostCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Nullable<u32>;
  }


  pub struct BeaconRxCount;

  impl Attr for BeaconRxCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "BeaconRxCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<u32>;
  }


  pub struct PacketMulticastRxCount;

  impl Attr for PacketMulticastRxCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "PacketMulticastRxCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<u32>;
  }


  pub struct PacketMulticastTxCount;

  impl Attr for PacketMulticastTxCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "PacketMulticastTxCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<u32>;
  }


  pub struct PacketUnicastRxCount;

  impl Attr for PacketUnicastRxCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "PacketUnicastRxCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<u32>;
  }


  pub struct PacketUnicastTxCount;

  impl Attr for PacketUnicastTxCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "PacketUnicastTxCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<u32>;
  }


  pub struct CurrentMaxRate;

  impl Attr for CurrentMaxRate {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "CurrentMaxRate",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u64>;
  }


  pub struct OverrunCount;

  impl Attr for OverrunCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x000c,
      name: "OverrunCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Nullable<u64>;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct Disconnection {
  pub reason_code: u16,
}

impl Event for Disconnection {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "Disconnection",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "ReasonCode", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for Disconnection {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut reason_code: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { reason_code = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      reason_code: reason_code.ok_or(TlvDecodeErr::MissingField("ReasonCode"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct AssociationFailure {
  pub association_failure_cause: ty::AssociationFailureCauseEnum,
  pub status: u16,
}

impl Event for AssociationFailure {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "AssociationFailure",
    conformance: Conformance::Optional,
    fields: &[
      AttrDef { id: 0x0000, name: "AssociationFailureCause", attr_ty: <ty::AssociationFailureCauseEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Status", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}

impl TlvDecode for AssociationFailure {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut association_failure_cause: Option<ty::AssociationFailureCauseEnum> = None;
    let mut status: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { association_failure_cause = Some(<ty::AssociationFailureCauseEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { status = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      association_failure_cause: association_failure_cause.ok_or(TlvDecodeErr::MissingField("AssociationFailureCause"))?,
      status: status.ok_or(TlvDecodeErr::MissingField("Status"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ConnectionStatus {
  pub connection_status: ty::ConnectionStatusEnum,
}

impl Event for ConnectionStatus {
  const EVENT: EventDef = EventDef {
    id: 0x0002,
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
pub enum AssociationFailureCauseEnum {
  ///The reason for the failure is unknown.
  Unknown = 0x0000,
  ///An error occurred during association.
  AssociationFailed = 0x0001,
  ///An error occurred during authentication.
  AuthenticationFailed = 0x0002,
  ///The specified SSID could not be found.
  SsidNotFound = 0x0003,
}
impl AttrType for AssociationFailureCauseEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unknown"),
  (0x0001, "AssociationFailed"),
  (0x0002, "AuthenticationFailed"),
  (0x0003, "SsidNotFound"),
  ]);
}
impl TlvPut for AssociationFailureCauseEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for AssociationFailureCauseEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unknown),
      0x0001 => Ok(Self::AssociationFailed),
      0x0002 => Ok(Self::AuthenticationFailed),
      0x0003 => Ok(Self::SsidNotFound),
      _ => Err(())
    }
  }
}
impl TlvDecode for AssociationFailureCauseEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ConnectionStatusEnum {
  ///Indicate the node is connected
  Connected = 0x0000,
  ///Indicate the node is not connected
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
pub enum SecurityTypeEnum {
  ///Indicate the usage of an unspecified Wi-Fi security type
  Unspecified = 0x0000,
  ///Indicate the usage of no Wi-Fi security
  None = 0x0001,
  ///Indicate the usage of WEP Wi-Fi security
  WEP = 0x0002,
  ///Indicate the usage of WPA Wi-Fi security
  WPA = 0x0003,
  ///Indicate the usage of WPA2 Wi-Fi security
  WPA2 = 0x0004,
  ///Indicate the usage of WPA3 Wi-Fi security
  WPA3 = 0x0005,
}
impl AttrType for SecurityTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "None"),
  (0x0002, "WEP"),
  (0x0003, "WPA"),
  (0x0004, "WPA2"),
  (0x0005, "WPA3"),
  ]);
}
impl TlvPut for SecurityTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for SecurityTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unspecified),
      0x0001 => Ok(Self::None),
      0x0002 => Ok(Self::WEP),
      0x0003 => Ok(Self::WPA),
      0x0004 => Ok(Self::WPA2),
      0x0005 => Ok(Self::WPA3),
      _ => Err(())
    }
  }
}
impl TlvDecode for SecurityTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum WiFiVersionEnum {
  ///Indicate the network interface is currently using IEEE 802.11a against the wireless access point.
  a = 0x0000,
  ///Indicate the network interface is currently using IEEE 802.11b against the wireless access point.
  b = 0x0001,
  ///Indicate the network interface is currently using IEEE 802.11g against the wireless access point.
  g = 0x0002,
  ///Indicate the network interface is currently using IEEE 802.11n against the wireless access point.
  n = 0x0003,
  ///Indicate the network interface is currently using IEEE 802.11ac against the wireless access point.
  ac = 0x0004,
  ///Indicate the network interface is currently using IEEE 802.11ax against the wireless access point.
  ax = 0x0005,
  ///Indicate the network interface is currently using IEEE 802.11ah against the wireless access point.
  ah = 0x0006,
}
impl AttrType for WiFiVersionEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "a"),
  (0x0001, "b"),
  (0x0002, "g"),
  (0x0003, "n"),
  (0x0004, "ac"),
  (0x0005, "ax"),
  (0x0006, "ah"),
  ]);
}
impl TlvPut for WiFiVersionEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for WiFiVersionEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::a),
      0x0001 => Ok(Self::b),
      0x0002 => Ok(Self::g),
      0x0003 => Ok(Self::n),
      0x0004 => Ok(Self::ac),
      0x0005 => Ok(Self::ax),
      0x0006 => Ok(Self::ah),
      _ => Err(())
    }
  }
}
impl TlvDecode for WiFiVersionEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
