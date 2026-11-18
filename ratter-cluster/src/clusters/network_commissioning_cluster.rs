#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const NETWORK_COMMISSIONING: ClusterDef = ClusterDef {
  id: 0x0031,
  name: "Network Commissioning",
  features: &[
    FeatureDef { bit: 0x0000, code: "WI", name: "WiFiNetworkInterface", summary: "Wi-Fi related features" },
    FeatureDef { bit: 0x0001, code: "TH", name: "ThreadNetworkInterface", summary: "Thread related features" },
    FeatureDef { bit: 0x0002, code: "ET", name: "EthernetNetworkInterface", summary: "Ethernet related features" },
  ],
  attributes: &[
    attr::MaxNetworks::ATTR,
    attr::Networks::ATTR,
    attr::ScanMaxTimeSeconds::ATTR,
    attr::ConnectMaxTimeSeconds::ATTR,
    attr::InterfaceEnabled::ATTR,
    attr::LastNetworkingStatus::ATTR,
    attr::LastNetworkID::ATTR,
    attr::LastConnectErrorValue::ATTR,
    attr::SupportedWiFiBands::ATTR,
    attr::SupportedThreadFeatures::ATTR,
    attr::ThreadVersion::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::ScanNetworks::COMMAND,
    command::AddOrUpdateWiFiNetwork::COMMAND,
    command::AddOrUpdateThreadNetwork::COMMAND,
    command::RemoveNetwork::COMMAND,
    command::ConnectNetwork::COMMAND,
    command::ReorderNetwork::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct MaxNetworks;

  impl Attr for MaxNetworks {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "MaxNetworks",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct Networks;

  impl Attr for Networks {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "Networks",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<ty::NetworkInfoStruct>;
  }


  pub struct ScanMaxTimeSeconds;

  impl Attr for ScanMaxTimeSeconds {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "ScanMaxTimeSeconds",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct ConnectMaxTimeSeconds;

  impl Attr for ConnectMaxTimeSeconds {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "ConnectMaxTimeSeconds",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct InterfaceEnabled;

  impl Attr for InterfaceEnabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "InterfaceEnabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


  pub struct LastNetworkingStatus;

  impl Attr for LastNetworkingStatus {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "LastNetworkingStatus",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::NetworkCommissioningStatusEnum>;
  }


  pub struct LastNetworkID;

  impl Attr for LastNetworkID {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "LastNetworkID",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<TypeUnsupported /* Octstr */>;
  }


  pub struct LastConnectErrorValue;

  impl Attr for LastConnectErrorValue {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "LastConnectErrorValue",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<i32>;
  }


  pub struct SupportedWiFiBands;

  impl Attr for SupportedWiFiBands {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "SupportedWiFiBands",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<ty::WiFiBandEnum>;
  }


  pub struct SupportedThreadFeatures;

  impl Attr for SupportedThreadFeatures {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "SupportedThreadFeatures",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = ty::ThreadCapabilitiesBitmap;
  }


  pub struct ThreadVersion;

  impl Attr for ThreadVersion {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "ThreadVersion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = u16;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ScanNetworks {
  pub ssid: TypeUnsupported /* Octstr */,
  pub breadcrumb: u64,
}
impl Command for ScanNetworks {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "ScanNetworks",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "SSID", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0001, name: "Breadcrumb", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for ScanNetworks {
  type Response = ScanNetworksResponse;
}
impl TlvEncode for ScanNetworks {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.ssid.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.breadcrumb.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ScanNetworksResponse {
  pub networking_status: ty::NetworkCommissioningStatusEnum,
  pub debug_text: String,
  pub wi_fi_scan_results: Vec<ty::WiFiInterfaceScanResultStruct>,
  pub thread_scan_results: Vec<ty::ThreadInterfaceScanResultStruct>,
}
impl Command for ScanNetworksResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "ScanNetworksResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NetworkingStatus", attr_ty: <ty::NetworkCommissioningStatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "DebugText", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "WiFiScanResults", attr_ty: <Vec<ty::WiFiInterfaceScanResultStruct>>::ATTR_TYPE, conformance: Conformance::Feature(0) },
      AttrDef { id: 0x0003, name: "ThreadScanResults", attr_ty: <Vec<ty::ThreadInterfaceScanResultStruct>>::ATTR_TYPE, conformance: Conformance::Feature(1) },
    ],
  };
}
impl CommandResponse for ScanNetworksResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct AddOrUpdateWiFiNetwork {
  pub ssid: TypeUnsupported /* Octstr */,
  pub credentials: TypeUnsupported /* Octstr */,
  pub breadcrumb: u64,
}
impl Command for AddOrUpdateWiFiNetwork {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "AddOrUpdateWiFiNetwork",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "SSID", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Credentials", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Breadcrumb", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for AddOrUpdateWiFiNetwork {
  type Response = NetworkConfigResponse;
}
impl TlvEncode for AddOrUpdateWiFiNetwork {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.ssid.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.credentials.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.breadcrumb.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddOrUpdateThreadNetwork {
  pub operational_dataset: TypeUnsupported /* Octstr */,
  pub breadcrumb: u64,
}
impl Command for AddOrUpdateThreadNetwork {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "AddOrUpdateThreadNetwork",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "OperationalDataset", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Breadcrumb", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for AddOrUpdateThreadNetwork {
  type Response = NetworkConfigResponse;
}
impl TlvEncode for AddOrUpdateThreadNetwork {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.operational_dataset.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.breadcrumb.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveNetwork {
  pub network_id: TypeUnsupported /* Octstr */,
  pub breadcrumb: u64,
}
impl Command for RemoveNetwork {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "RemoveNetwork",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NetworkID", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Breadcrumb", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for RemoveNetwork {
  type Response = NetworkConfigResponse;
}
impl TlvEncode for RemoveNetwork {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.network_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.breadcrumb.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetworkConfigResponse {
  pub networking_status: ty::NetworkCommissioningStatusEnum,
  pub debug_text: String,
  pub network_index: u8,
}
impl Command for NetworkConfigResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "NetworkConfigResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NetworkingStatus", attr_ty: <ty::NetworkCommissioningStatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "DebugText", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "NetworkIndex", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for NetworkConfigResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectNetwork {
  pub network_id: TypeUnsupported /* Octstr */,
  pub breadcrumb: u64,
}
impl Command for ConnectNetwork {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "ConnectNetwork",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NetworkID", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Breadcrumb", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for ConnectNetwork {
  type Response = ConnectNetworkResponse;
}
impl TlvEncode for ConnectNetwork {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.network_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.breadcrumb.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConnectNetworkResponse {
  pub networking_status: ty::NetworkCommissioningStatusEnum,
  pub debug_text: String,
  pub error_value: i32,
}
impl Command for ConnectNetworkResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "ConnectNetworkResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NetworkingStatus", attr_ty: <ty::NetworkCommissioningStatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "DebugText", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0002, name: "ErrorValue", attr_ty: <i32>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for ConnectNetworkResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct ReorderNetwork {
  pub network_id: TypeUnsupported /* Octstr */,
  pub network_index: u8,
  pub breadcrumb: u64,
}
impl Command for ReorderNetwork {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0008,
    name: "ReorderNetwork",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NetworkID", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "NetworkIndex", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Breadcrumb", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for ReorderNetwork {
  type Response = NetworkConfigResponse;
}
impl TlvEncode for ReorderNetwork {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.network_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.network_index.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.breadcrumb.tlv_put(matter_codec::Tag::Context(0x0002), writer);
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
pub enum NetworkCommissioningStatusEnum {
  ///OK, no error
  Success = 0x0000,
  ///Value Outside Range
  OutOfRange = 0x0001,
  ///A collection would exceed its size limit
  BoundsExceeded = 0x0002,
  ///The NetworkID is not among the collection of added networks
  NetworkIDNotFound = 0x0003,
  ///The NetworkID is already among the collection of added networks
  DuplicateNetworkID = 0x0004,
  ///Cannot find AP: SSID Not found
  NetworkNotFound = 0x0005,
  ///Cannot find AP: Mismatch on band/channels/regulatory domain / 2.4GHz vs 5GHz
  RegulatoryError = 0x0006,
  ///Cannot associate due to authentication failure
  AuthFailure = 0x0007,
  ///Cannot associate due to unsupported security mode
  UnsupportedSecurity = 0x0008,
  ///Other association failure
  OtherConnectionFailure = 0x0009,
  ///Failure to generate an IPv6 address
  IPV6Failed = 0x000a,
  ///Failure to bind Wi-Fi +<->+ IP interfaces
  IPBindFailed = 0x000b,
  ///Unknown error
  UnknownError = 0x000c,
}
impl AttrType for NetworkCommissioningStatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Success"),
  (0x0001, "OutOfRange"),
  (0x0002, "BoundsExceeded"),
  (0x0003, "NetworkIDNotFound"),
  (0x0004, "DuplicateNetworkID"),
  (0x0005, "NetworkNotFound"),
  (0x0006, "RegulatoryError"),
  (0x0007, "AuthFailure"),
  (0x0008, "UnsupportedSecurity"),
  (0x0009, "OtherConnectionFailure"),
  (0x000a, "IPV6Failed"),
  (0x000b, "IPBindFailed"),
  (0x000c, "UnknownError"),
  ]);
}
impl TlvPut for NetworkCommissioningStatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for NetworkCommissioningStatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Success),
      0x0001 => Ok(Self::OutOfRange),
      0x0002 => Ok(Self::BoundsExceeded),
      0x0003 => Ok(Self::NetworkIDNotFound),
      0x0004 => Ok(Self::DuplicateNetworkID),
      0x0005 => Ok(Self::NetworkNotFound),
      0x0006 => Ok(Self::RegulatoryError),
      0x0007 => Ok(Self::AuthFailure),
      0x0008 => Ok(Self::UnsupportedSecurity),
      0x0009 => Ok(Self::OtherConnectionFailure),
      0x000a => Ok(Self::IPV6Failed),
      0x000b => Ok(Self::IPBindFailed),
      0x000c => Ok(Self::UnknownError),
      _ => Err(())
    }
  }
}
impl TlvDecode for NetworkCommissioningStatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum WiFiBandEnum {
  ///2.4GHz - 2.401GHz to 2.495GHz (802.11b/g/n/ax)
  _2G4 = 0x0000,
  ///3.65GHz - 3.655GHz to 3.695GHz (802.11y)
  _3G65 = 0x0001,
  ///5GHz - 5.150GHz to 5.895GHz (802.11a/n/ac/ax)
  _5G = 0x0002,
  ///6GHz - 5.925GHz to 7.125GHz (802.11ax / Wi-Fi 6E)
  _6G = 0x0003,
  ///60GHz - 57.24GHz to 70.20GHz (802.11ad/ay)
  _60G = 0x0004,
  ///Sub-1GHz - 755MHz to 931MHz (802.11ah)
  _1G = 0x0005,
}
impl AttrType for WiFiBandEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "_2G4"),
  (0x0001, "_3G65"),
  (0x0002, "_5G"),
  (0x0003, "_6G"),
  (0x0004, "_60G"),
  (0x0005, "_1G"),
  ]);
}
impl TlvPut for WiFiBandEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for WiFiBandEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::_2G4),
      0x0001 => Ok(Self::_3G65),
      0x0002 => Ok(Self::_5G),
      0x0003 => Ok(Self::_6G),
      0x0004 => Ok(Self::_60G),
      0x0005 => Ok(Self::_1G),
      _ => Err(())
    }
  }
}
impl TlvDecode for WiFiBandEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct ThreadCapabilitiesBitmap: u32 {
    ///Thread Border Router functionality is present
    const IsBorderRouterCapable = 1 << 0;
    ///Router mode is supported (interface could be in router or REED mode)
    const IsRouterCapable = 1 << 1;
    ///Sleepy end-device mode is supported
    const IsSleepyEndDeviceCapable = 1 << 2;
    ///Device is a full Thread device (opposite of Minimal Thread Device)
    const IsFullThreadDevice = 1 << 3;
    ///Synchronized sleepy end-device mode is supported
    const IsSynchronizedSleepyEndDeviceCapable = 1 << 4;
  }
}
impl AttrType for ThreadCapabilitiesBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "IsBorderRouterCapable"),
    (1, "IsRouterCapable"),
    (2, "IsSleepyEndDeviceCapable"),
    (3, "IsFullThreadDevice"),
    (4, "IsSynchronizedSleepyEndDeviceCapable"),
  ]);
}
impl TlvPut for ThreadCapabilitiesBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for ThreadCapabilitiesBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for ThreadCapabilitiesBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct WiFiSecurityBitmap: u32 {
    ///Supports unencrypted Wi-Fi
    const Unencrypted = 1 << 0;
    ///Supports Wi-Fi using WEP security
    const WEP = 1 << 1;
    ///Supports Wi-Fi using WPA-Personal security
    const WPA_PERSONAL = 1 << 2;
    ///Supports Wi-Fi using WPA2-Personal security
    const WPA2_PERSONAL = 1 << 3;
    ///Supports Wi-Fi using WPA3-Personal security
    const WPA3_PERSONAL = 1 << 4;
  }
}
impl AttrType for WiFiSecurityBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Unencrypted"),
    (1, "WEP"),
    (2, "WPA_PERSONAL"),
    (3, "WPA2_PERSONAL"),
    (4, "WPA3_PERSONAL"),
  ]);
}
impl TlvPut for WiFiSecurityBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for WiFiSecurityBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for WiFiSecurityBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkInfoStruct {
  pub network_id: TypeUnsupported /* Octstr */,
  pub connected: bool,
}
impl AttrType for NetworkInfoStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "NetworkID", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Connected", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for NetworkInfoStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.network_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.connected.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for NetworkInfoStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut connected: Option<bool> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { connected = Some(<bool>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      network_id: TypeUnsupported,
      connected: connected.ok_or(TlvDecodeErr::MissingField("Connected"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThreadInterfaceScanResultStruct {
  pub pan_id: u16,
  pub extended_pan_id: u64,
  pub network_name: String,
  pub channel: u16,
  pub version: u8,
  pub extended_address: TypeUnsupported /* Hwadr */,
  pub rssi: i8,
  pub lqi: u8,
}
impl AttrType for ThreadInterfaceScanResultStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "PanId", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ExtendedPanId", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "NetworkName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "Channel", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "Version", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "ExtendedAddress", attr_ty: <TypeUnsupported /* Hwadr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "RSSI", attr_ty: <i8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "LQI", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ThreadInterfaceScanResultStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.pan_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.extended_pan_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.network_name.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.channel.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.version.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.extended_address.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.rssi.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.lqi.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ThreadInterfaceScanResultStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut pan_id: Option<u16> = None;
    let mut extended_pan_id: Option<u64> = None;
    let mut network_name: Option<String> = None;
    let mut channel: Option<u16> = None;
    let mut version: Option<u8> = None;
    let mut rssi: Option<i8> = None;
    let mut lqi: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { pan_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { extended_pan_id = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { network_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { channel = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { version = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { rssi = Some(<i8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0007), value: _ }) => { lqi = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      pan_id: pan_id.ok_or(TlvDecodeErr::MissingField("PanId"))?,
      extended_pan_id: extended_pan_id.ok_or(TlvDecodeErr::MissingField("ExtendedPanId"))?,
      network_name: network_name.ok_or(TlvDecodeErr::MissingField("NetworkName"))?,
      channel: channel.ok_or(TlvDecodeErr::MissingField("Channel"))?,
      version: version.ok_or(TlvDecodeErr::MissingField("Version"))?,
      extended_address: TypeUnsupported,
      rssi: rssi.ok_or(TlvDecodeErr::MissingField("RSSI"))?,
      lqi: lqi.ok_or(TlvDecodeErr::MissingField("LQI"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct WiFiInterfaceScanResultStruct {
  pub security: WiFiSecurityBitmap,
  pub ssid: TypeUnsupported /* Octstr */,
  pub bssid: TypeUnsupported /* Octstr */,
  pub channel: u16,
  pub wi_fi_band: WiFiBandEnum,
  pub rssi: i8,
}
impl AttrType for WiFiInterfaceScanResultStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Security", attr_ty: <WiFiSecurityBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "SSID", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "BSSID", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "Channel", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "WiFiBand", attr_ty: <WiFiBandEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "RSSI", attr_ty: <i8>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for WiFiInterfaceScanResultStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.security.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.ssid.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.bssid.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.channel.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.wi_fi_band.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.rssi.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for WiFiInterfaceScanResultStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut security: Option<WiFiSecurityBitmap> = None;
    let mut channel: Option<u16> = None;
    let mut wi_fi_band: Option<WiFiBandEnum> = None;
    let mut rssi: Option<i8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { security = Some(<WiFiSecurityBitmap>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { channel = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { wi_fi_band = Some(<WiFiBandEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { rssi = Some(<i8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      security: security.ok_or(TlvDecodeErr::MissingField("Security"))?,
      ssid: TypeUnsupported,
      bssid: TypeUnsupported,
      channel: channel.ok_or(TlvDecodeErr::MissingField("Channel"))?,
      wi_fi_band: wi_fi_band.ok_or(TlvDecodeErr::MissingField("WiFiBand"))?,
      rssi: rssi.ok_or(TlvDecodeErr::MissingField("RSSI"))?,
    })
  }
}

}
