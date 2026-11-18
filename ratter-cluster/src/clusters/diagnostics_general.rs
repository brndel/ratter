use crate::cluster_def::*;
pub const GENERAL_DIAGNOSTICS: ClusterDef = ClusterDef {
  id: 0x0033,
  name: "General Diagnostics",
  features: &[
    "DMTEST",
  ],
  attributes: &[
    // attr::NetworkInterfaces::ATTR, (type Some(List) is not yet supported)
    attr::RebootCount::ATTR,
    attr::UpTime::ATTR,
    attr::TotalOperationalHours::ATTR,
    attr::BootReason::ATTR,
    // attr::ActiveHardwareFaults::ATTR, (type Some(List) is not yet supported)
    // attr::ActiveRadioFaults::ATTR, (type Some(List) is not yet supported)
    // attr::ActiveNetworkFaults::ATTR, (type Some(List) is not yet supported)
    attr::TestEventTriggersEnabled::ATTR,
    // attr::DoNotUse::ATTR, (type None is not yet supported)
    attr::DeviceLoadStatus::ATTR,
  ],
  events: &[],
};
pub mod attr {
  use super::*;
  // pub struct NetworkInterfaces;
  // type Some(List) is not supported yet


  pub struct RebootCount;

  impl Attr for RebootCount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "RebootCount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u16;
  }


  pub struct UpTime;

  impl Attr for UpTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "UpTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u64;
  }


  pub struct TotalOperationalHours;

  impl Attr for TotalOperationalHours {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "TotalOperationalHours",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct BootReason;

  impl Attr for BootReason {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "BootReason",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::BootReasonEnum;
  }


  // pub struct ActiveHardwareFaults;
  // type Some(List) is not supported yet


  // pub struct ActiveRadioFaults;
  // type Some(List) is not supported yet


  // pub struct ActiveNetworkFaults;
  // type Some(List) is not supported yet


  pub struct TestEventTriggersEnabled;

  impl Attr for TestEventTriggersEnabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "TestEventTriggersEnabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


  // pub struct DoNotUse;
  // type None is not supported yet


  pub struct DeviceLoadStatus;

  impl Attr for DeviceLoadStatus {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "DeviceLoadStatus",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::DeviceLoadStruct;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub enum BootReasonEnum {
  ///The Node is unable to identify the Power-On reason as one of the other provided enumeration values.
  Unspecified = 0x0000,
  ///The Node has booted as the result of physical interaction with the device resulting in a reboot.
  PowerOnReboot = 0x0001,
  ///The Node has rebooted as the result of a brown-out of the Node's power supply.
  BrownOutReset = 0x0002,
  ///The Node has rebooted as the result of a software watchdog timer.
  SoftwareWatchdogReset = 0x0003,
  ///The Node has rebooted as the result of a hardware watchdog timer.
  HardwareWatchdogReset = 0x0004,
  ///The Node has rebooted as the result of a completed software update.
  SoftwareUpdateCompleted = 0x0005,
  ///The Node has rebooted as the result of a software initiated reboot.
  SoftwareReset = 0x0006,
}
impl AttrType for BootReasonEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "PowerOnReboot"),
  (0x0002, "BrownOutReset"),
  (0x0003, "SoftwareWatchdogReset"),
  (0x0004, "HardwareWatchdogReset"),
  (0x0005, "SoftwareUpdateCompleted"),
  (0x0006, "SoftwareReset"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum HardwareFaultEnum {
  ///The Node has encountered an unspecified fault.
  Unspecified = 0x0000,
  ///The Node has encountered a fault with at least one of its radios.
  Radio = 0x0001,
  ///The Node has encountered a fault with at least one of its sensors.
  Sensor = 0x0002,
  ///The Node has encountered an over-temperature fault that is resettable.
  ResettableOverTemp = 0x0003,
  ///The Node has encountered an over-temperature fault that is not resettable.
  NonResettableOverTemp = 0x0004,
  ///The Node has encountered a fault with at least one of its power sources.
  PowerSource = 0x0005,
  ///The Node has encountered a fault with at least one of its visual displays.
  VisualDisplayFault = 0x0006,
  ///The Node has encountered a fault with at least one of its audio outputs.
  AudioOutputFault = 0x0007,
  ///The Node has encountered a fault with at least one of its user interfaces.
  UserInterfaceFault = 0x0008,
  ///The Node has encountered a fault with its non-volatile memory.
  NonVolatileMemoryError = 0x0009,
  ///The Node has encountered disallowed physical tampering.
  TamperDetected = 0x000a,
}
impl AttrType for HardwareFaultEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "Radio"),
  (0x0002, "Sensor"),
  (0x0003, "ResettableOverTemp"),
  (0x0004, "NonResettableOverTemp"),
  (0x0005, "PowerSource"),
  (0x0006, "VisualDisplayFault"),
  (0x0007, "AudioOutputFault"),
  (0x0008, "UserInterfaceFault"),
  (0x0009, "NonVolatileMemoryError"),
  (0x000a, "TamperDetected"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum InterfaceTypeEnum {
  ///Indicates an interface of an unspecified type.
  Unspecified = 0x0000,
  ///Indicates a Wi-Fi interface.
  WiFi = 0x0001,
  ///Indicates a Ethernet interface.
  Ethernet = 0x0002,
  ///Indicates a Cellular interface.
  Cellular = 0x0003,
  ///Indicates a Thread interface.
  Thread = 0x0004,
}
impl AttrType for InterfaceTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "WiFi"),
  (0x0002, "Ethernet"),
  (0x0003, "Cellular"),
  (0x0004, "Thread"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum NetworkFaultEnum {
  ///The Node has encountered an unspecified fault.
  Unspecified = 0x0000,
  ///The Node has encountered a network fault as a result of a hardware failure.
  HardwareFailure = 0x0001,
  ///The Node has encountered a network fault as a result of a jammed network.
  NetworkJammed = 0x0002,
  ///The Node has encountered a network fault as a result of a failure to establish a connection.
  ConnectionFailed = 0x0003,
}
impl AttrType for NetworkFaultEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "HardwareFailure"),
  (0x0002, "NetworkJammed"),
  (0x0003, "ConnectionFailed"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum RadioFaultEnum {
  ///The Node has encountered an unspecified radio fault.
  Unspecified = 0x0000,
  ///The Node has encountered a fault with its Wi-Fi radio.
  WiFiFault = 0x0001,
  ///The Node has encountered a fault with its cellular radio.
  CellularFault = 0x0002,
  ///The Node has encountered a fault with its 802.15.4 radio.
  ThreadFault = 0x0003,
  ///The Node has encountered a fault with its NFC radio.
  NFCFault = 0x0004,
  ///The Node has encountered a fault with its BLE radio.
  BLEFault = 0x0005,
  ///The Node has encountered a fault with its Ethernet controller.
  EthernetFault = 0x0006,
}
impl AttrType for RadioFaultEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "WiFiFault"),
  (0x0002, "CellularFault"),
  (0x0003, "ThreadFault"),
  (0x0004, "NFCFault"),
  (0x0005, "BLEFault"),
  (0x0006, "EthernetFault"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceLoadStruct {
  currentsubscriptions: u16,
  currentsubscriptionsforfabric: u16,
  totalsubscriptionsestablished: u32,
  totalinteractionmodelmessagessent: u32,
  totalinteractionmodelmessagesreceived: u32,
}
impl AttrType for DeviceLoadStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "CurrentSubscriptions", attr_ty: u16::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "CurrentSubscriptionsForFabric", attr_ty: u16::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "TotalSubscriptionsEstablished", attr_ty: u32::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "TotalInteractionModelMessagesSent", attr_ty: u32::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "TotalInteractionModelMessagesReceived", attr_ty: u32::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
#[derive(Debug, Clone, PartialEq)]
pub struct NetworkInterface {
  name: String,
  isoperational: bool,
  offpremiseservicesreachableipv4: bool,
  offpremiseservicesreachableipv6: bool,
  // hardwareaddress: Hwadr (type currently unsupported)
  // ipv4addresses: List (type currently unsupported)
  // ipv6addresses: List (type currently unsupported)
  r#type: InterfaceTypeEnum,
}
impl AttrType for NetworkInterface {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Name", attr_ty: String::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "IsOperational", attr_ty: bool::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "OffPremiseServicesReachableIPv4", attr_ty: bool::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "OffPremiseServicesReachableIPv6", attr_ty: bool::ATTR_TYPE, conformance: Conformance::Mandatory },
  // field HardwareAddress: Hwadr (type currently unsupported)
  // field IPv4Addresses: List (type currently unsupported)
  // field IPv6Addresses: List (type currently unsupported)
    AttrDef { id: 0x0007, name: "Type", attr_ty: InterfaceTypeEnum::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
}
