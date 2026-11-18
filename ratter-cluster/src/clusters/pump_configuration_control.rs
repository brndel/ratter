use crate::cluster_def::*;
pub const PUMP_CONFIGURATION_AND_CONTROL: ClusterDef = ClusterDef {
  id: 0x0200,
  name: "Pump Configuration and Control",
  features: &[
    "PRSCONST",
    "PRSCOMP",
    "FLW",
    "SPD",
    "TEMP",
    "AUTO",
    "LOCAL",
  ],
  attributes: &[
    attr::MaxPressure::ATTR,
    attr::MaxSpeed::ATTR,
    attr::MaxFlow::ATTR,
    attr::MinConstPressure::ATTR,
    attr::MaxConstPressure::ATTR,
    attr::MinCompPressure::ATTR,
    attr::MaxCompPressure::ATTR,
    attr::MinConstSpeed::ATTR,
    attr::MaxConstSpeed::ATTR,
    attr::MinConstFlow::ATTR,
    attr::MaxConstFlow::ATTR,
    attr::MinConstTemp::ATTR,
    attr::MaxConstTemp::ATTR,
    attr::PumpStatus::ATTR,
    attr::EffectiveOperationMode::ATTR,
    attr::EffectiveControlMode::ATTR,
    attr::Capacity::ATTR,
    attr::Speed::ATTR,
    attr::LifetimeRunningHours::ATTR,
    attr::Power::ATTR,
    attr::LifetimeEnergyConsumed::ATTR,
    attr::OperationMode::ATTR,
    attr::ControlMode::ATTR,
    // attr::AlarmMask::ATTR, (type None is not yet supported)
  ],
  events: &[],
};
pub mod attr {
  use super::*;
  pub struct MaxPressure;

  impl Attr for MaxPressure {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "MaxPressure",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<i16>;
  }


  pub struct MaxSpeed;

  impl Attr for MaxSpeed {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "MaxSpeed",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u16>;
  }


  pub struct MaxFlow;

  impl Attr for MaxFlow {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "MaxFlow",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<u16>;
  }


  pub struct MinConstPressure;

  impl Attr for MinConstPressure {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "MinConstPressure",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<i16>;
  }


  pub struct MaxConstPressure;

  impl Attr for MaxConstPressure {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "MaxConstPressure",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<i16>;
  }


  pub struct MinCompPressure;

  impl Attr for MinCompPressure {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "MinCompPressure",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<i16>;
  }


  pub struct MaxCompPressure;

  impl Attr for MaxCompPressure {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "MaxCompPressure",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<i16>;
  }


  pub struct MinConstSpeed;

  impl Attr for MinConstSpeed {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "MinConstSpeed",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<u16>;
  }


  pub struct MaxConstSpeed;

  impl Attr for MaxConstSpeed {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "MaxConstSpeed",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<u16>;
  }


  pub struct MinConstFlow;

  impl Attr for MinConstFlow {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "MinConstFlow",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<u16>;
  }


  pub struct MaxConstFlow;

  impl Attr for MaxConstFlow {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "MaxConstFlow",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<u16>;
  }


  pub struct MinConstTemp;

  impl Attr for MinConstTemp {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "MinConstTemp",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<i16>;
  }


  pub struct MaxConstTemp;

  impl Attr for MaxConstTemp {
    const ATTR: AttrDef = AttrDef {
      id: 0x000c,
      name: "MaxConstTemp",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<i16>;
  }


  pub struct PumpStatus;

  impl Attr for PumpStatus {
    const ATTR: AttrDef = AttrDef {
      id: 0x0010,
      name: "PumpStatus",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::PumpStatusBitmap;
  }


  pub struct EffectiveOperationMode;

  impl Attr for EffectiveOperationMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0011,
      name: "EffectiveOperationMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::OperationModeEnum;
  }


  pub struct EffectiveControlMode;

  impl Attr for EffectiveControlMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0012,
      name: "EffectiveControlMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::ControlModeEnum;
  }


  pub struct Capacity;

  impl Attr for Capacity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0013,
      name: "Capacity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<i16>;
  }


  pub struct Speed;

  impl Attr for Speed {
    const ATTR: AttrDef = AttrDef {
      id: 0x0014,
      name: "Speed",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u16>;
  }


  pub struct LifetimeRunningHours;

  impl Attr for LifetimeRunningHours {
    const ATTR: AttrDef = AttrDef {
      id: 0x0015,
      name: "LifetimeRunningHours",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u32>;
  }


  pub struct Power;

  impl Attr for Power {
    const ATTR: AttrDef = AttrDef {
      id: 0x0016,
      name: "Power",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u32>;
  }


  pub struct LifetimeEnergyConsumed;

  impl Attr for LifetimeEnergyConsumed {
    const ATTR: AttrDef = AttrDef {
      id: 0x0017,
      name: "LifetimeEnergyConsumed",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<u32>;
  }


  pub struct OperationMode;

  impl Attr for OperationMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0020,
      name: "OperationMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::OperationModeEnum;
  }


  pub struct ControlMode;

  impl Attr for ControlMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0021,
      name: "ControlMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::ControlModeEnum;
  }


  // pub struct AlarmMask;
  // type None is not supported yet


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub enum ControlModeEnum {
  ///The pump is running at a constant speed.
  ConstantSpeed = 0x0000,
  ///The pump will regulate its speed to maintain a constant differential pressure over its flanges.
  ConstantPressure = 0x0001,
  ///The pump will regulate its speed to maintain a constant differential pressure over its flanges.
  ProportionalPressure = 0x0002,
  ///The pump will regulate its speed to maintain a constant flow through the pump.
  ConstantFlow = 0x0003,
  ///The pump will regulate its speed to maintain a constant temperature.
  ConstantTemperature = 0x0005,
  ///The operation of the pump is automatically optimized to provide the most suitable performance with respect to comfort and energy savings.
  Automatic = 0x0007,
}
impl AttrType for ControlModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "ConstantSpeed"),
  (0x0001, "ConstantPressure"),
  (0x0002, "ProportionalPressure"),
  (0x0003, "ConstantFlow"),
  (0x0005, "ConstantTemperature"),
  (0x0007, "Automatic"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum OperationModeEnum {
  ///The pump is controlled by a setpoint, as defined by a connected remote sensor or by the ControlMode attribute.
  Normal = 0x0000,
  ///This value sets the pump to run at the minimum possible speed it can without being stopped.
  Minimum = 0x0001,
  ///This value sets the pump to run at its maximum possible speed.
  Maximum = 0x0002,
  ///This value sets the pump to run with the local settings of the pump, regardless of what these are.
  Local = 0x0003,
}
impl AttrType for OperationModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Normal"),
  (0x0001, "Minimum"),
  (0x0002, "Maximum"),
  (0x0003, "Local"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  pub struct PumpStatusBitmap: u32 {
    ///A fault related to the system or pump device is detected.
    const DeviceFault = 1 << 0;
    ///A fault related to the supply to the pump is detected.
    const SupplyFault = 1 << 1;
    ///Setpoint is too low to achieve.
    const SpeedLow = 1 << 2;
    ///Setpoint is too high to achieve.
    const SpeedHigh = 1 << 3;
    ///Device control is overridden by hardware, such as an external STOP button or via a local HMI.
    const LocalOverride = 1 << 4;
    ///Pump is currently running
    const Running = 1 << 5;
    ///A remote pressure sensor is used as the sensor for the regulation of the pump.
    const RemotePressure = 1 << 6;
    ///A remote flow sensor is used as the sensor for the regulation of the pump.
    const RemoteFlow = 1 << 7;
    ///A remote temperature sensor is used as the sensor for the regulation of the pump.
    const RemoteTemperature = 1 << 8;
  }
}
impl AttrType for PumpStatusBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "DeviceFault"),
    (1, "SupplyFault"),
    (2, "SpeedLow"),
    (3, "SpeedHigh"),
    (4, "LocalOverride"),
    (5, "Running"),
    (6, "RemotePressure"),
    (7, "RemoteFlow"),
    (8, "RemoteTemperature"),
  ]);
}
}
