use crate::cluster_def::*;
pub mod attr {
  use super::*;
  pub struct LocalTemperature;

  impl Attr for LocalTemperature {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "LocalTemperature",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  pub struct OutdoorTemperature;

  impl Attr for OutdoorTemperature {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "OutdoorTemperature",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  pub struct Occupancy;

  impl Attr for Occupancy {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "Occupancy",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::OccupancyBitmap;
  }


  pub struct AbsMinHeatSetpointLimit;

  impl Attr for AbsMinHeatSetpointLimit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "AbsMinHeatSetpointLimit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  pub struct AbsMaxHeatSetpointLimit;

  impl Attr for AbsMaxHeatSetpointLimit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "AbsMaxHeatSetpointLimit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  pub struct AbsMinCoolSetpointLimit;

  impl Attr for AbsMinCoolSetpointLimit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "AbsMinCoolSetpointLimit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  pub struct AbsMaxCoolSetpointLimit;

  impl Attr for AbsMaxCoolSetpointLimit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "AbsMaxCoolSetpointLimit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  // pub struct PICoolingDemand;
  // type None is not supported yet


  // pub struct PIHeatingDemand;
  // type None is not supported yet


  // pub struct HVACSystemTypeConfiguration;
  // type None is not supported yet


  pub struct LocalTemperatureCalibration;

  impl Attr for LocalTemperatureCalibration {
    const ATTR: AttrDef = AttrDef {
      id: 0x0010,
      name: "LocalTemperatureCalibration",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::SignedTemperature;
  }


  pub struct OccupiedCoolingSetpoint;

  impl Attr for OccupiedCoolingSetpoint {
    const ATTR: AttrDef = AttrDef {
      id: 0x0011,
      name: "OccupiedCoolingSetpoint",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  pub struct OccupiedHeatingSetpoint;

  impl Attr for OccupiedHeatingSetpoint {
    const ATTR: AttrDef = AttrDef {
      id: 0x0012,
      name: "OccupiedHeatingSetpoint",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  pub struct UnoccupiedCoolingSetpoint;

  impl Attr for UnoccupiedCoolingSetpoint {
    const ATTR: AttrDef = AttrDef {
      id: 0x0013,
      name: "UnoccupiedCoolingSetpoint",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  pub struct UnoccupiedHeatingSetpoint;

  impl Attr for UnoccupiedHeatingSetpoint {
    const ATTR: AttrDef = AttrDef {
      id: 0x0014,
      name: "UnoccupiedHeatingSetpoint",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  pub struct MinHeatSetpointLimit;

  impl Attr for MinHeatSetpointLimit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0015,
      name: "MinHeatSetpointLimit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  pub struct MaxHeatSetpointLimit;

  impl Attr for MaxHeatSetpointLimit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0016,
      name: "MaxHeatSetpointLimit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  pub struct MinCoolSetpointLimit;

  impl Attr for MinCoolSetpointLimit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0017,
      name: "MinCoolSetpointLimit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  pub struct MaxCoolSetpointLimit;

  impl Attr for MaxCoolSetpointLimit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0018,
      name: "MaxCoolSetpointLimit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  pub struct MinSetpointDeadBand;

  impl Attr for MinSetpointDeadBand {
    const ATTR: AttrDef = AttrDef {
      id: 0x0019,
      name: "MinSetpointDeadBand",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::SignedTemperature;
  }


  pub struct RemoteSensing;

  impl Attr for RemoteSensing {
    const ATTR: AttrDef = AttrDef {
      id: 0x001a,
      name: "RemoteSensing",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::RemoteSensingBitmap;
  }


  pub struct ControlSequenceOfOperation;

  impl Attr for ControlSequenceOfOperation {
    const ATTR: AttrDef = AttrDef {
      id: 0x001b,
      name: "ControlSequenceOfOperation",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::ControlSequenceOfOperationEnum;
  }


  pub struct SystemMode;

  impl Attr for SystemMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x001c,
      name: "SystemMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::SystemModeEnum;
  }


  pub struct ThermostatRunningMode;

  impl Attr for ThermostatRunningMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x001e,
      name: "ThermostatRunningMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::ThermostatRunningModeEnum;
  }


  pub struct TemperatureSetpointHold;

  impl Attr for TemperatureSetpointHold {
    const ATTR: AttrDef = AttrDef {
      id: 0x0023,
      name: "TemperatureSetpointHold",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::TemperatureSetpointHoldEnum;
  }


  pub struct TemperatureSetpointHoldDuration;

  impl Attr for TemperatureSetpointHoldDuration {
    const ATTR: AttrDef = AttrDef {
      id: 0x0024,
      name: "TemperatureSetpointHoldDuration",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = u16;
  }


  // pub struct ThermostatProgrammingOperationMode;
  // type None is not supported yet


  pub struct ThermostatRunningState;

  impl Attr for ThermostatRunningState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0029,
      name: "ThermostatRunningState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::RelayStateBitmap;
  }


  pub struct SetpointChangeSource;

  impl Attr for SetpointChangeSource {
    const ATTR: AttrDef = AttrDef {
      id: 0x0030,
      name: "SetpointChangeSource",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::SetpointChangeSourceEnum;
  }


  pub struct SetpointChangeAmount;

  impl Attr for SetpointChangeAmount {
    const ATTR: AttrDef = AttrDef {
      id: 0x0031,
      name: "SetpointChangeAmount",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = i16;
  }


  // pub struct SetpointChangeSourceTimestamp;
  // type Some(EpochS) is not supported yet


  // pub struct OccupiedSetback;
  // type None is not supported yet


  // pub struct OccupiedSetbackMin;
  // type None is not supported yet


  // pub struct OccupiedSetbackMax;
  // type None is not supported yet


  // pub struct UnoccupiedSetback;
  // type None is not supported yet


  // pub struct UnoccupiedSetbackMin;
  // type None is not supported yet


  // pub struct UnoccupiedSetbackMax;
  // type None is not supported yet


  pub struct EmergencyHeatDelta;

  impl Attr for EmergencyHeatDelta {
    const ATTR: AttrDef = AttrDef {
      id: 0x003a,
      name: "EmergencyHeatDelta",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::UnsignedTemperature;
  }


  pub struct ACType;

  impl Attr for ACType {
    const ATTR: AttrDef = AttrDef {
      id: 0x0040,
      name: "ACType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::ACTypeEnum;
  }


  pub struct ACCapacity;

  impl Attr for ACCapacity {
    const ATTR: AttrDef = AttrDef {
      id: 0x0041,
      name: "ACCapacity",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = u16;
  }


  pub struct ACRefrigerantType;

  impl Attr for ACRefrigerantType {
    const ATTR: AttrDef = AttrDef {
      id: 0x0042,
      name: "ACRefrigerantType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::ACRefrigerantTypeEnum;
  }


  pub struct ACCompressorType;

  impl Attr for ACCompressorType {
    const ATTR: AttrDef = AttrDef {
      id: 0x0043,
      name: "ACCompressorType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::ACCompressorTypeEnum;
  }


  pub struct ACErrorCode;

  impl Attr for ACErrorCode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0044,
      name: "ACErrorCode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::ACErrorCodeBitmap;
  }


  pub struct ACLouverPosition;

  impl Attr for ACLouverPosition {
    const ATTR: AttrDef = AttrDef {
      id: 0x0045,
      name: "ACLouverPosition",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::ACLouverPositionEnum;
  }


  pub struct ACCoilTemperature;

  impl Attr for ACCoilTemperature {
    const ATTR: AttrDef = AttrDef {
      id: 0x0046,
      name: "ACCoilTemperature",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = Temperature;
  }


  pub struct ACCapacityFormat;

  impl Attr for ACCapacityFormat {
    const ATTR: AttrDef = AttrDef {
      id: 0x0047,
      name: "ACCapacityFormat",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::ACCapacityFormatEnum;
  }


  // pub struct PresetTypes;
  // type Some(List) is not supported yet


  // pub struct ScheduleTypes;
  // type Some(List) is not supported yet


  pub struct NumberOfPresets;

  impl Attr for NumberOfPresets {
    const ATTR: AttrDef = AttrDef {
      id: 0x004a,
      name: "NumberOfPresets",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = u8;
  }


  pub struct NumberOfSchedules;

  impl Attr for NumberOfSchedules {
    const ATTR: AttrDef = AttrDef {
      id: 0x004b,
      name: "NumberOfSchedules",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = u8;
  }


  pub struct NumberOfScheduleTransitions;

  impl Attr for NumberOfScheduleTransitions {
    const ATTR: AttrDef = AttrDef {
      id: 0x004c,
      name: "NumberOfScheduleTransitions",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = u8;
  }


  pub struct NumberOfScheduleTransitionPerDay;

  impl Attr for NumberOfScheduleTransitionPerDay {
    const ATTR: AttrDef = AttrDef {
      id: 0x004d,
      name: "NumberOfScheduleTransitionPerDay",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = u8;
  }


  // pub struct ActivePresetHandle;
  // type Some(Octstr) is not supported yet


  // pub struct ActiveScheduleHandle;
  // type Some(Octstr) is not supported yet


  // pub struct Presets;
  // type Some(List) is not supported yet


  // pub struct Schedules;
  // type Some(List) is not supported yet


  // pub struct SetpointHoldExpiryTimestamp;
  // type Some(EpochS) is not supported yet


  pub struct MaxThermostatSuggestions;

  impl Attr for MaxThermostatSuggestions {
    const ATTR: AttrDef = AttrDef {
      id: 0x0053,
      name: "MaxThermostatSuggestions",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = u8;
  }


  // pub struct ThermostatSuggestions;
  // type Some(List) is not supported yet


  pub struct CurrentThermostatSuggestion;

  impl Attr for CurrentThermostatSuggestion {
    const ATTR: AttrDef = AttrDef {
      id: 0x0055,
      name: "CurrentThermostatSuggestion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::ThermostatSuggestionStruct;
  }


  pub struct ThermostatSuggestionNotFollowingReason;

  impl Attr for ThermostatSuggestionNotFollowingReason {
    const ATTR: AttrDef = AttrDef {
      id: 0x0056,
      name: "ThermostatSuggestionNotFollowingReason",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: todo!(),
    };
    type Value = ty::ThermostatSuggestionNotFollowingReasonBitmap;
  }


}
pub mod ty {
  use super::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ACCapacityFormatEnum {
  ///British Thermal Unit per Hour
  BTUh = 0x0000,
}
impl AttrType for ACCapacityFormatEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "BTUh"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ACCompressorTypeEnum {
  ///Unknown compressor type
  Unknown = 0x0000,
  ///Max working ambient 43 °C
  T1 = 0x0001,
  ///Max working ambient 35 °C
  T2 = 0x0002,
  ///Max working ambient 52 °C
  T3 = 0x0003,
}
impl AttrType for ACCompressorTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unknown"),
  (0x0001, "T1"),
  (0x0002, "T2"),
  (0x0003, "T3"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ACLouverPositionEnum {
  ///Fully Closed
  Closed = 0x0001,
  ///Fully Open
  Open = 0x0002,
  ///Quarter Open
  Quarter = 0x0003,
  ///Half Open
  Half = 0x0004,
  ///Three Quarters Open
  ThreeQuarters = 0x0005,
}
impl AttrType for ACLouverPositionEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0001, "Closed"),
  (0x0002, "Open"),
  (0x0003, "Quarter"),
  (0x0004, "Half"),
  (0x0005, "ThreeQuarters"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ACRefrigerantTypeEnum {
  ///Unknown Refrigerant Type
  Unknown = 0x0000,
  ///R22 Refrigerant
  R22 = 0x0001,
  ///R410a Refrigerant
  R410a = 0x0002,
  ///R407c Refrigerant
  R407c = 0x0003,
}
impl AttrType for ACRefrigerantTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unknown"),
  (0x0001, "R22"),
  (0x0002, "R410a"),
  (0x0003, "R407c"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ACTypeEnum {
  ///Unknown AC Type
  Unknown = 0x0000,
  ///Cooling and Fixed Speed
  CoolingFixed = 0x0001,
  ///Heat Pump and Fixed Speed
  HeatPumpFixed = 0x0002,
  ///Cooling and Inverter
  CoolingInverter = 0x0003,
  ///Heat Pump and Inverter
  HeatPumpInverter = 0x0004,
}
impl AttrType for ACTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unknown"),
  (0x0001, "CoolingFixed"),
  (0x0002, "HeatPumpFixed"),
  (0x0003, "CoolingInverter"),
  (0x0004, "HeatPumpInverter"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlSequenceOfOperationEnum {
  ///Heat and Emergency are not possible
  CoolingOnly = 0x0000,
  ///Heat and Emergency are not possible
  CoolingWithReheat = 0x0001,
  ///Cool and precooling (see Terms) are not possible
  HeatingOnly = 0x0002,
  ///Cool and precooling are not possible
  HeatingWithReheat = 0x0003,
  ///All modes are possible
  CoolingAndHeating = 0x0004,
  ///All modes are possible
  CoolingAndHeatingWithReheat = 0x0005,
}
impl AttrType for ControlSequenceOfOperationEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "CoolingOnly"),
  (0x0001, "CoolingWithReheat"),
  (0x0002, "HeatingOnly"),
  (0x0003, "HeatingWithReheat"),
  (0x0004, "CoolingAndHeating"),
  (0x0005, "CoolingAndHeatingWithReheat"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PresetScenarioEnum {
  ///The thermostat-controlled area is occupied
  Occupied = 0x0001,
  ///The thermostat-controlled area is unoccupied
  Unoccupied = 0x0002,
  ///Users are likely to be sleeping
  Sleep = 0x0003,
  ///Users are likely to be waking up
  Wake = 0x0004,
  ///Users are on vacation
  Vacation = 0x0005,
  ///Users are likely to be going to sleep
  GoingToSleep = 0x0006,
  ///Custom presets
  UserDefined = 0x00fe,
}
impl AttrType for PresetScenarioEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0001, "Occupied"),
  (0x0002, "Unoccupied"),
  (0x0003, "Sleep"),
  (0x0004, "Wake"),
  (0x0005, "Vacation"),
  (0x0006, "GoingToSleep"),
  (0x00fe, "UserDefined"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SetpointChangeSourceEnum {
  ///Manual, user-initiated setpoint change via the thermostat
  Manual = 0x0000,
  ///Schedule/internal programming-initiated setpoint change
  Schedule = 0x0001,
  ///Externally-initiated setpoint change (e.g., DRLC cluster command, attribute write)
  External = 0x0002,
}
impl AttrType for SetpointChangeSourceEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Manual"),
  (0x0001, "Schedule"),
  (0x0002, "External"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SetpointRaiseLowerModeEnum {
  ///Adjust Heat Setpoint
  Heat = 0x0000,
  ///Adjust Cool Setpoint
  Cool = 0x0001,
  ///Adjust Heat Setpoint and Cool Setpoint
  Both = 0x0002,
}
impl AttrType for SetpointRaiseLowerModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Heat"),
  (0x0001, "Cool"),
  (0x0002, "Both"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StartOfWeekEnum {
  Sunday = 0x0000,
  Monday = 0x0001,
  Tuesday = 0x0002,
  Wednesday = 0x0003,
  Thursday = 0x0004,
  Friday = 0x0005,
  Saturday = 0x0006,
}
impl AttrType for StartOfWeekEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Sunday"),
  (0x0001, "Monday"),
  (0x0002, "Tuesday"),
  (0x0003, "Wednesday"),
  (0x0004, "Thursday"),
  (0x0005, "Friday"),
  (0x0006, "Saturday"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SystemModeEnum {
  ///The Thermostat does not generate demand for Cooling or Heating
  Off = 0x0000,
  ///Demand is generated for either Cooling or Heating, as required
  Auto = 0x0001,
  ///Demand is only generated for Cooling
  Cool = 0x0003,
  ///Demand is only generated for Heating
  Heat = 0x0004,
  ///2nd stage heating is in use to achieve desired temperature
  EmergencyHeat = 0x0005,
  ///(see Terms)
  Precooling = 0x0006,
  FanOnly = 0x0007,
  Dry = 0x0008,
  Sleep = 0x0009,
}
impl AttrType for SystemModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Off"),
  (0x0001, "Auto"),
  (0x0003, "Cool"),
  (0x0004, "Heat"),
  (0x0005, "EmergencyHeat"),
  (0x0006, "Precooling"),
  (0x0007, "FanOnly"),
  (0x0008, "Dry"),
  (0x0009, "Sleep"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TemperatureSetpointHoldEnum {
  ///Follow scheduling program
  SetpointHoldOff = 0x0000,
  ///Maintain current setpoint, regardless of schedule transitions
  SetpointHoldOn = 0x0001,
}
impl AttrType for TemperatureSetpointHoldEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "SetpointHoldOff"),
  (0x0001, "SetpointHoldOn"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ThermostatRunningModeEnum {
  ///The Thermostat does not generate demand for Cooling or Heating
  Off = 0x0000,
  ///Demand is only generated for Cooling
  Cool = 0x0003,
  ///Demand is only generated for Heating
  Heat = 0x0004,
}
impl AttrType for ThermostatRunningModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Off"),
  (0x0003, "Cool"),
  (0x0004, "Heat"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub struct ACErrorCodeBitmap: u32 {
    ///Compressor Failure or Refrigerant Leakage
    const CompressorFail = 1 << 0;
    ///Room Temperature Sensor Failure
    const RoomSensorFail = 1 << 1;
    ///Outdoor Temperature Sensor Failure
    const OutdoorSensorFail = 1 << 2;
    ///Indoor Coil Temperature Sensor Failure
    const CoilSensorFail = 1 << 3;
    ///Fan Failure
    const FanFail = 1 << 4;
  }
}
impl AttrType for ACErrorCodeBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "CompressorFail"),
    (1, "RoomSensorFail"),
    (2, "OutdoorSensorFail"),
    (3, "CoilSensorFail"),
    (4, "FanFail"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub struct OccupancyBitmap: u32 {
    ///Indicates the occupancy state
    const Occupied = 1 << 0;
  }
}
impl AttrType for OccupancyBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Occupied"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub struct PresetTypeFeaturesBitmap: u32 {
    ///Preset may be automatically activated by the thermostat
    const Automatic = 1 << 0;
    ///Preset supports user-provided names
    const SupportsNames = 1 << 1;
  }
}
impl AttrType for PresetTypeFeaturesBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Automatic"),
    (1, "SupportsNames"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub struct RelayStateBitmap: u32 {
    ///Heat Stage On
    const Heat = 1 << 0;
    ///Cool Stage On
    const Cool = 1 << 1;
    ///Fan Stage On
    const Fan = 1 << 2;
    ///Heat 2nd Stage On
    const HeatStage2 = 1 << 3;
    ///Cool 2nd Stage On
    const CoolStage2 = 1 << 4;
    ///Fan 2nd Stage On
    const FanStage2 = 1 << 5;
    ///Fan 3rd Stage On
    const FanStage3 = 1 << 6;
  }
}
impl AttrType for RelayStateBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Heat"),
    (1, "Cool"),
    (2, "Fan"),
    (3, "HeatStage2"),
    (4, "CoolStage2"),
    (5, "FanStage2"),
    (6, "FanStage3"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub struct RemoteSensingBitmap: u32 {
    ///Calculated Local Temperature is derived from a remote node
    const LocalTemperature = 1 << 0;
    ///OutdoorTemperature is derived from a remote node
    const OutdoorTemperature = 1 << 1;
    ///Occupancy is derived from a remote node
    const Occupancy = 1 << 2;
  }
}
impl AttrType for RemoteSensingBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "LocalTemperature"),
    (1, "OutdoorTemperature"),
    (2, "Occupancy"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub struct ScheduleDayOfWeekBitmap: u32 {
    ///Sunday
    const Sunday = 1 << 0;
    ///Monday
    const Monday = 1 << 1;
    ///Tuesday
    const Tuesday = 1 << 2;
    ///Wednesday
    const Wednesday = 1 << 3;
    ///Thursday
    const Thursday = 1 << 4;
    ///Friday
    const Friday = 1 << 5;
    ///Saturday
    const Saturday = 1 << 6;
    ///Away or Vacation
    const Away = 1 << 7;
  }
}
impl AttrType for ScheduleDayOfWeekBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Sunday"),
    (1, "Monday"),
    (2, "Tuesday"),
    (3, "Wednesday"),
    (4, "Thursday"),
    (5, "Friday"),
    (6, "Saturday"),
    (7, "Away"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub struct ScheduleModeBitmap: u32 {
    ///Adjust Heat Setpoint
    const HeatSetpointPresent = 1 << 0;
    ///Adjust Cool Setpoint
    const CoolSetpointPresent = 1 << 1;
  }
}
impl AttrType for ScheduleModeBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "HeatSetpointPresent"),
    (1, "CoolSetpointPresent"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub struct ScheduleTypeFeaturesBitmap: u32 {
    ///Supports presets
    const SupportsPresets = 1 << 0;
    ///Supports setpoints
    const SupportsSetpoints = 1 << 1;
    ///Supports user-provided names
    const SupportsNames = 1 << 2;
    ///Supports transitioning to SystemModeOff
    const SupportsOff = 1 << 3;
  }
}
impl AttrType for ScheduleTypeFeaturesBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "SupportsPresets"),
    (1, "SupportsSetpoints"),
    (2, "SupportsNames"),
    (3, "SupportsOff"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub struct ThermostatSuggestionNotFollowingReasonBitmap: u32 {
    ///Thermostat is responding to a Demand Response event
    const DemandResponseEvent = 1 << 0;
    ///Thermostat has an ongoing setpoint hold
    const OngoingHold = 1 << 1;
    ///Thermostat is following a schedule
    const Schedule = 1 << 2;
    ///Thermostat is following the occupancy signal
    const Occupancy = 1 << 3;
    ///Thermostat is set to Vacation mode
    const VacationMode = 1 << 4;
    ///Thermostat is following a Time Of Use based cost savings plan
    const TimeOfUseCostSavings = 1 << 5;
    ///Thermostat is precooling or preheating based on an energy forecast signal
    const PreCoolingOrPreHeating = 1 << 6;
    ///Thermostat has conflicting suggestions and is unable to choose one
    const ConflictingSuggestions = 1 << 7;
  }
}
impl AttrType for ThermostatSuggestionNotFollowingReasonBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "DemandResponseEvent"),
    (1, "OngoingHold"),
    (2, "Schedule"),
    (3, "Occupancy"),
    (4, "VacationMode"),
    (5, "TimeOfUseCostSavings"),
    (6, "PreCoolingOrPreHeating"),
    (7, "ConflictingSuggestions"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PresetStruct {
  // preset_handle: Octstr (type currently unsupported)
  preset_scenario: PresetScenarioEnum,
  // name: String (type currently unsupported)
  cooling_setpoint: Temperature,
  heating_setpoint: Temperature,
  built_in: bool,
}
impl AttrType for PresetStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
  // field PresetHandle: Octstr (type currently unsupported)
    AttrDef { id: 0x0001, name: "PresetScenario", attr_ty: PresetScenarioEnum::ATTR_TYPE, conformance: Conformance::Mandatory },
  // field Name: String (type currently unsupported)
    AttrDef { id: 0x0003, name: "CoolingSetpoint", attr_ty: Temperature::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "HeatingSetpoint", attr_ty: Temperature::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "BuiltIn", attr_ty: bool::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PresetTypeStruct {
  preset_scenario: PresetScenarioEnum,
  number_of_presets: u8,
  preset_type_features: PresetTypeFeaturesBitmap,
}
impl AttrType for PresetTypeStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "PresetScenario", attr_ty: PresetScenarioEnum::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "NumberOfPresets", attr_ty: u8::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "PresetTypeFeatures", attr_ty: PresetTypeFeaturesBitmap::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScheduleStruct {
  // schedule_handle: Octstr (type currently unsupported)
  system_mode: SystemModeEnum,
  // name: String (type currently unsupported)
  // preset_handle: Octstr (type currently unsupported)
  // transitions: List (type currently unsupported)
  built_in: bool,
}
impl AttrType for ScheduleStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
  // field ScheduleHandle: Octstr (type currently unsupported)
    AttrDef { id: 0x0001, name: "SystemMode", attr_ty: SystemModeEnum::ATTR_TYPE, conformance: Conformance::Mandatory },
  // field Name: String (type currently unsupported)
  // field PresetHandle: Octstr (type currently unsupported)
  // field Transitions: List (type currently unsupported)
    AttrDef { id: 0x0005, name: "BuiltIn", attr_ty: bool::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScheduleTransitionStruct {
  day_of_week: ScheduleDayOfWeekBitmap,
  transition_time: u16,
  // preset_handle: Octstr (type currently unsupported)
  system_mode: SystemModeEnum,
  cooling_setpoint: Temperature,
  heating_setpoint: Temperature,
}
impl AttrType for ScheduleTransitionStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "DayOfWeek", attr_ty: ScheduleDayOfWeekBitmap::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "TransitionTime", attr_ty: u16::ATTR_TYPE, conformance: Conformance::Mandatory },
  // field PresetHandle: Octstr (type currently unsupported)
    AttrDef { id: 0x0003, name: "SystemMode", attr_ty: SystemModeEnum::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "CoolingSetpoint", attr_ty: Temperature::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "HeatingSetpoint", attr_ty: Temperature::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScheduleTypeStruct {
  system_mode: SystemModeEnum,
  number_of_schedules: u8,
  schedule_type_features: ScheduleTypeFeaturesBitmap,
}
impl AttrType for ScheduleTypeStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "SystemMode", attr_ty: SystemModeEnum::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "NumberOfSchedules", attr_ty: u8::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "ScheduleTypeFeatures", attr_ty: ScheduleTypeFeaturesBitmap::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThermostatSuggestionStruct {
  unique_id: u8,
  // preset_handle: Octstr (type currently unsupported)
  // effective_time: EpochS (type currently unsupported)
  // expiration_time: EpochS (type currently unsupported)
}
impl AttrType for ThermostatSuggestionStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "UniqueID", attr_ty: u8::ATTR_TYPE, conformance: Conformance::Mandatory },
  // field PresetHandle: Octstr (type currently unsupported)
  // field EffectiveTime: EpochS (type currently unsupported)
  // field ExpirationTime: EpochS (type currently unsupported)
  ]});
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WeeklyScheduleTransitionStruct {
  transition_time: u16,
  heat_setpoint: Temperature,
  cool_setpoint: Temperature,
}
impl AttrType for WeeklyScheduleTransitionStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "TransitionTime", attr_ty: u16::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "HeatSetpoint", attr_ty: Temperature::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "CoolSetpoint", attr_ty: Temperature::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
}
