use crate::cluster_def::*;
pub const DOOR_LOCK: ClusterDef = ClusterDef {
  id: 0x0101,
  name: "Door Lock",
  features: &[
    "PIN",
    "RID",
    "FGP",
    "WDSCH",
    "DPS",
    "FACE",
    "COTA",
    "USR",
    "YDSCH",
    "HDSCH",
    "UBOLT",
    "ALIRO",
    "ALBU",
  ],
  attributes: &[
    attr::LockState::ATTR,
    attr::LockType::ATTR,
    attr::ActuatorEnabled::ATTR,
    attr::DoorState::ATTR,
    attr::DoorOpenEvents::ATTR,
    attr::DoorClosedEvents::ATTR,
    attr::OpenPeriod::ATTR,
    attr::NumberOfTotalUsersSupported::ATTR,
    attr::NumberOfPINUsersSupported::ATTR,
    attr::NumberOfRFIDUsersSupported::ATTR,
    attr::NumberOfWeekDaySchedulesSupportedPerUser::ATTR,
    attr::NumberOfYearDaySchedulesSupportedPerUser::ATTR,
    attr::NumberOfHolidaySchedulesSupported::ATTR,
    attr::MaxPINCodeLength::ATTR,
    attr::MinPINCodeLength::ATTR,
    attr::MaxRFIDCodeLength::ATTR,
    attr::MinRFIDCodeLength::ATTR,
    attr::CredentialRulesSupport::ATTR,
    attr::NumberOfCredentialsSupportedPerUser::ATTR,
    attr::Language::ATTR,
    attr::LEDSettings::ATTR,
    attr::AutoRelockTime::ATTR,
    attr::SoundVolume::ATTR,
    attr::OperatingMode::ATTR,
    attr::SupportedOperatingModes::ATTR,
    attr::DefaultConfigurationRegister::ATTR,
    attr::EnableLocalProgramming::ATTR,
    attr::EnableOneTouchLocking::ATTR,
    attr::EnableInsideStatusLED::ATTR,
    attr::EnablePrivacyModeButton::ATTR,
    attr::LocalProgrammingFeatures::ATTR,
    attr::WrongCodeEntryLimit::ATTR,
    attr::UserCodeTemporaryDisableTime::ATTR,
    attr::SendPINOverTheAir::ATTR,
    attr::RequirePINforRemoteOperation::ATTR,
    // attr::SecurityLevel::ATTR, (type None is not yet supported)
    attr::ExpiringUserTimeout::ATTR,
    // attr::AliroReaderVerificationKey::ATTR, (type Some(Octstr) is not yet supported)
    // attr::AliroReaderGroupIdentifier::ATTR, (type Some(Octstr) is not yet supported)
    // attr::AliroReaderGroupSubIdentifier::ATTR, (type Some(Octstr) is not yet supported)
    // attr::AliroExpeditedTransactionSupportedProtocolVersions::ATTR, (type Some(List) is not yet supported)
    // attr::AliroGroupResolvingKey::ATTR, (type Some(Octstr) is not yet supported)
    // attr::AliroSupportedBLEUWBProtocolVersions::ATTR, (type Some(List) is not yet supported)
    attr::AliroBLEAdvertisingVersion::ATTR,
    attr::NumberOfAliroCredentialIssuerKeysSupported::ATTR,
    attr::NumberOfAliroEndpointKeysSupported::ATTR,
  ],
  events: &[],
};
pub mod attr {
  use super::*;
  pub struct LockState;

  impl Attr for LockState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "LockState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Nullable<ty::LockStateEnum>;
  }


  pub struct LockType;

  impl Attr for LockType {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "LockType",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::LockTypeEnum;
  }


  pub struct ActuatorEnabled;

  impl Attr for ActuatorEnabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "ActuatorEnabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


  pub struct DoorState;

  impl Attr for DoorState {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "DoorState",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(5),
    };
    type Value = Nullable<ty::DoorStateEnum>;
  }


  pub struct DoorOpenEvents;

  impl Attr for DoorOpenEvents {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "DoorOpenEvents",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct DoorClosedEvents;

  impl Attr for DoorClosedEvents {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "DoorClosedEvents",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct OpenPeriod;

  impl Attr for OpenPeriod {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "OpenPeriod",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  pub struct NumberOfTotalUsersSupported;

  impl Attr for NumberOfTotalUsersSupported {
    const ATTR: AttrDef = AttrDef {
      id: 0x0011,
      name: "NumberOfTotalUsersSupported",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(8),
    };
    type Value = u16;
  }


  pub struct NumberOfPINUsersSupported;

  impl Attr for NumberOfPINUsersSupported {
    const ATTR: AttrDef = AttrDef {
      id: 0x0012,
      name: "NumberOfPINUsersSupported",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u16;
  }


  pub struct NumberOfRFIDUsersSupported;

  impl Attr for NumberOfRFIDUsersSupported {
    const ATTR: AttrDef = AttrDef {
      id: 0x0013,
      name: "NumberOfRFIDUsersSupported",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = u16;
  }


  pub struct NumberOfWeekDaySchedulesSupportedPerUser;

  impl Attr for NumberOfWeekDaySchedulesSupportedPerUser {
    const ATTR: AttrDef = AttrDef {
      id: 0x0014,
      name: "NumberOfWeekDaySchedulesSupportedPerUser",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = u8;
  }


  pub struct NumberOfYearDaySchedulesSupportedPerUser;

  impl Attr for NumberOfYearDaySchedulesSupportedPerUser {
    const ATTR: AttrDef = AttrDef {
      id: 0x0015,
      name: "NumberOfYearDaySchedulesSupportedPerUser",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(10),
    };
    type Value = u8;
  }


  pub struct NumberOfHolidaySchedulesSupported;

  impl Attr for NumberOfHolidaySchedulesSupported {
    const ATTR: AttrDef = AttrDef {
      id: 0x0016,
      name: "NumberOfHolidaySchedulesSupported",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(11),
    };
    type Value = u8;
  }


  pub struct MaxPINCodeLength;

  impl Attr for MaxPINCodeLength {
    const ATTR: AttrDef = AttrDef {
      id: 0x0017,
      name: "MaxPINCodeLength",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u8;
  }


  pub struct MinPINCodeLength;

  impl Attr for MinPINCodeLength {
    const ATTR: AttrDef = AttrDef {
      id: 0x0018,
      name: "MinPINCodeLength",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u8;
  }


  pub struct MaxRFIDCodeLength;

  impl Attr for MaxRFIDCodeLength {
    const ATTR: AttrDef = AttrDef {
      id: 0x0019,
      name: "MaxRFIDCodeLength",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = u8;
  }


  pub struct MinRFIDCodeLength;

  impl Attr for MinRFIDCodeLength {
    const ATTR: AttrDef = AttrDef {
      id: 0x001a,
      name: "MinRFIDCodeLength",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = u8;
  }


  pub struct CredentialRulesSupport;

  impl Attr for CredentialRulesSupport {
    const ATTR: AttrDef = AttrDef {
      id: 0x001b,
      name: "CredentialRulesSupport",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(8),
    };
    type Value = ty::CredentialRulesBitmap;
  }


  pub struct NumberOfCredentialsSupportedPerUser;

  impl Attr for NumberOfCredentialsSupportedPerUser {
    const ATTR: AttrDef = AttrDef {
      id: 0x001c,
      name: "NumberOfCredentialsSupportedPerUser",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(8),
    };
    type Value = u8;
  }


  pub struct Language;

  impl Attr for Language {
    const ATTR: AttrDef = AttrDef {
      id: 0x0021,
      name: "Language",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = String;
  }


  pub struct LEDSettings;

  impl Attr for LEDSettings {
    const ATTR: AttrDef = AttrDef {
      id: 0x0022,
      name: "LEDSettings",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::LEDSettingEnum;
  }


  pub struct AutoRelockTime;

  impl Attr for AutoRelockTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0023,
      name: "AutoRelockTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u32;
  }


  pub struct SoundVolume;

  impl Attr for SoundVolume {
    const ATTR: AttrDef = AttrDef {
      id: 0x0024,
      name: "SoundVolume",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::SoundVolumeEnum;
  }


  pub struct OperatingMode;

  impl Attr for OperatingMode {
    const ATTR: AttrDef = AttrDef {
      id: 0x0025,
      name: "OperatingMode",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::OperatingModeEnum;
  }


  pub struct SupportedOperatingModes;

  impl Attr for SupportedOperatingModes {
    const ATTR: AttrDef = AttrDef {
      id: 0x0026,
      name: "SupportedOperatingModes",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::OperatingModesBitmap;
  }


  pub struct DefaultConfigurationRegister;

  impl Attr for DefaultConfigurationRegister {
    const ATTR: AttrDef = AttrDef {
      id: 0x0027,
      name: "DefaultConfigurationRegister",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::ConfigurationRegisterBitmap;
  }


  pub struct EnableLocalProgramming;

  impl Attr for EnableLocalProgramming {
    const ATTR: AttrDef = AttrDef {
      id: 0x0028,
      name: "EnableLocalProgramming",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct EnableOneTouchLocking;

  impl Attr for EnableOneTouchLocking {
    const ATTR: AttrDef = AttrDef {
      id: 0x0029,
      name: "EnableOneTouchLocking",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct EnableInsideStatusLED;

  impl Attr for EnableInsideStatusLED {
    const ATTR: AttrDef = AttrDef {
      id: 0x002a,
      name: "EnableInsideStatusLED",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct EnablePrivacyModeButton;

  impl Attr for EnablePrivacyModeButton {
    const ATTR: AttrDef = AttrDef {
      id: 0x002b,
      name: "EnablePrivacyModeButton",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct LocalProgrammingFeatures;

  impl Attr for LocalProgrammingFeatures {
    const ATTR: AttrDef = AttrDef {
      id: 0x002c,
      name: "LocalProgrammingFeatures",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = ty::LocalProgrammingFeaturesBitmap;
  }


  pub struct WrongCodeEntryLimit;

  impl Attr for WrongCodeEntryLimit {
    const ATTR: AttrDef = AttrDef {
      id: 0x0030,
      name: "WrongCodeEntryLimit",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct UserCodeTemporaryDisableTime;

  impl Attr for UserCodeTemporaryDisableTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0031,
      name: "UserCodeTemporaryDisableTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u8;
  }


  pub struct SendPINOverTheAir;

  impl Attr for SendPINOverTheAir {
    const ATTR: AttrDef = AttrDef {
      id: 0x0032,
      name: "SendPINOverTheAir",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


  pub struct RequirePINforRemoteOperation;

  impl Attr for RequirePINforRemoteOperation {
    const ATTR: AttrDef = AttrDef {
      id: 0x0033,
      name: "RequirePINforRemoteOperation",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


  // pub struct SecurityLevel;
  // type None is not supported yet


  pub struct ExpiringUserTimeout;

  impl Attr for ExpiringUserTimeout {
    const ATTR: AttrDef = AttrDef {
      id: 0x0035,
      name: "ExpiringUserTimeout",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = u16;
  }


  // pub struct AliroReaderVerificationKey;
  // type Some(Octstr) is not supported yet


  // pub struct AliroReaderGroupIdentifier;
  // type Some(Octstr) is not supported yet


  // pub struct AliroReaderGroupSubIdentifier;
  // type Some(Octstr) is not supported yet


  // pub struct AliroExpeditedTransactionSupportedProtocolVersions;
  // type Some(List) is not supported yet


  // pub struct AliroGroupResolvingKey;
  // type Some(Octstr) is not supported yet


  // pub struct AliroSupportedBLEUWBProtocolVersions;
  // type Some(List) is not supported yet


  pub struct AliroBLEAdvertisingVersion;

  impl Attr for AliroBLEAdvertisingVersion {
    const ATTR: AttrDef = AttrDef {
      id: 0x0086,
      name: "AliroBLEAdvertisingVersion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(14),
    };
    type Value = u8;
  }


  pub struct NumberOfAliroCredentialIssuerKeysSupported;

  impl Attr for NumberOfAliroCredentialIssuerKeysSupported {
    const ATTR: AttrDef = AttrDef {
      id: 0x0087,
      name: "NumberOfAliroCredentialIssuerKeysSupported",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(13),
    };
    type Value = u16;
  }


  pub struct NumberOfAliroEndpointKeysSupported;

  impl Attr for NumberOfAliroEndpointKeysSupported {
    const ATTR: AttrDef = AttrDef {
      id: 0x0088,
      name: "NumberOfAliroEndpointKeysSupported",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(13),
    };
    type Value = u16;
  }


}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub enum AlarmCodeEnum {
  ///Locking Mechanism Jammed
  LockJammed = 0x0000,
  ///Lock Reset to Factory Defaults
  LockFactoryReset = 0x0001,
  ///Lock Radio Power Cycled
  LockRadioPowerCycled = 0x0003,
  ///Tamper Alarm - wrong code entry limit
  WrongCodeEntryLimit = 0x0004,
  ///Tamper Alarm - front escutcheon removed from main
  FrontEsceutcheonRemoved = 0x0005,
  ///Forced Door Open under Door Locked Condition
  DoorForcedOpen = 0x0006,
  ///Door ajar
  DoorAjar = 0x0007,
  ///Force User SOS alarm
  ForcedUser = 0x0008,
}
impl AttrType for AlarmCodeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "LockJammed"),
  (0x0001, "LockFactoryReset"),
  (0x0003, "LockRadioPowerCycled"),
  (0x0004, "WrongCodeEntryLimit"),
  (0x0005, "FrontEsceutcheonRemoved"),
  (0x0006, "DoorForcedOpen"),
  (0x0007, "DoorAjar"),
  (0x0008, "ForcedUser"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum CredentialRuleEnum {
  ///Only one credential is required for lock operation
  Single = 0x0000,
  ///Any two credentials are required for lock operation
  Dual = 0x0001,
  ///Any three credentials are required for lock operation
  Tri = 0x0002,
}
impl AttrType for CredentialRuleEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Single"),
  (0x0001, "Dual"),
  (0x0002, "Tri"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum CredentialTypeEnum {
  ///Programming PIN code credential type
  ProgrammingPIN = 0x0000,
  ///PIN code credential type
  PIN = 0x0001,
  ///RFID identifier credential type
  RFID = 0x0002,
  ///Fingerprint identifier credential type
  Fingerprint = 0x0003,
  ///Finger vein identifier credential type
  FingerVein = 0x0004,
  ///Face identifier credential type
  Face = 0x0005,
  ///A Credential Issuer public key as defined in Aliro
  AliroCredentialIssuerKey = 0x0006,
  ///An Endpoint public key as defined in Aliro which can be evicted if space is needed for another endpoint key
  AliroEvictableEndpointKey = 0x0007,
  ///An Endpoint public key as defined in Aliro which cannot be evicted if space is needed for another endpoint key
  AliroNonEvictableEndpointKey = 0x0008,
}
impl AttrType for CredentialTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "ProgrammingPIN"),
  (0x0001, "PIN"),
  (0x0002, "RFID"),
  (0x0003, "Fingerprint"),
  (0x0004, "FingerVein"),
  (0x0005, "Face"),
  (0x0006, "AliroCredentialIssuerKey"),
  (0x0007, "AliroEvictableEndpointKey"),
  (0x0008, "AliroNonEvictableEndpointKey"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum DataOperationTypeEnum {
  ///Data is being added or was added
  Add = 0x0000,
  ///Data is being cleared or was cleared
  Clear = 0x0001,
  ///Data is being modified or was modified
  Modify = 0x0002,
}
impl AttrType for DataOperationTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Add"),
  (0x0001, "Clear"),
  (0x0002, "Modify"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum DoorStateEnum {
  ///Door state is open
  DoorOpen = 0x0000,
  ///Door state is closed
  DoorClosed = 0x0001,
  ///Door state is jammed
  DoorJammed = 0x0002,
  ///Door state is currently forced open
  DoorForcedOpen = 0x0003,
  ///Door state is invalid for unspecified reason
  DoorUnspecifiedError = 0x0004,
  ///Door state is ajar
  DoorAjar = 0x0005,
}
impl AttrType for DoorStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "DoorOpen"),
  (0x0001, "DoorClosed"),
  (0x0002, "DoorJammed"),
  (0x0003, "DoorForcedOpen"),
  (0x0004, "DoorUnspecifiedError"),
  (0x0005, "DoorAjar"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum EventTypeEnum {
  ///Event type is operation
  Operation = 0x0000,
  ///Event type is programming
  Programming = 0x0001,
  ///Event type is alarm
  Alarm = 0x0002,
}
impl AttrType for EventTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Operation"),
  (0x0001, "Programming"),
  (0x0002, "Alarm"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum LEDSettingEnum {
  ///Never use LED for signalization
  NoLEDSignal = 0x0000,
  ///Use LED signalization except for access allowed events
  NoLEDSignalAccessAllowed = 0x0001,
  ///Use LED signalization for all events
  LEDSignalAll = 0x0002,
}
impl AttrType for LEDSettingEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "NoLEDSignal"),
  (0x0001, "NoLEDSignalAccessAllowed"),
  (0x0002, "LEDSignalAll"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum LockDataTypeEnum {
  ///Unspecified or manufacturer specific lock user data added, cleared, or modified.
  Unspecified = 0x0000,
  ///Lock programming PIN code was added, cleared, or modified.
  ProgrammingCode = 0x0001,
  ///Lock user index was added, cleared, or modified.
  UserIndex = 0x0002,
  ///Lock user week day schedule was added, cleared, or modified.
  WeekDaySchedule = 0x0003,
  ///Lock user year day schedule was added, cleared, or modified.
  YearDaySchedule = 0x0004,
  ///Lock holiday schedule was added, cleared, or modified.
  HolidaySchedule = 0x0005,
  ///Lock user PIN code was added, cleared, or modified.
  PIN = 0x0006,
  ///Lock user RFID code was added, cleared, or modified.
  RFID = 0x0007,
  ///Lock user fingerprint was added, cleared, or modified.
  Fingerprint = 0x0008,
  ///Lock user finger-vein information was added, cleared, or modified.
  FingerVein = 0x0009,
  ///Lock user face information was added, cleared, or modified.
  Face = 0x000a,
  ///An Aliro credential issuer key credential was added, cleared, or modified.
  AliroCredentialIssuerKey = 0x000b,
  ///An Aliro endpoint key credential which can be evicted credential was added, cleared, or modified.
  AliroEvictableEndpointKey = 0x000c,
  ///An Aliro endpoint key credential which cannot be evicted was added, cleared, or modified.
  AliroNonEvictableEndpointKey = 0x000d,
}
impl AttrType for LockDataTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "ProgrammingCode"),
  (0x0002, "UserIndex"),
  (0x0003, "WeekDaySchedule"),
  (0x0004, "YearDaySchedule"),
  (0x0005, "HolidaySchedule"),
  (0x0006, "PIN"),
  (0x0007, "RFID"),
  (0x0008, "Fingerprint"),
  (0x0009, "FingerVein"),
  (0x000a, "Face"),
  (0x000b, "AliroCredentialIssuerKey"),
  (0x000c, "AliroEvictableEndpointKey"),
  (0x000d, "AliroNonEvictableEndpointKey"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum LockOperationTypeEnum {
  ///Lock operation
  Lock = 0x0000,
  ///Unlock operation
  Unlock = 0x0001,
  ///Triggered by keypad entry for user with User Type set to Non Access User
  NonAccessUserEvent = 0x0002,
  ///Triggered by using a user with UserType set to Forced User
  ForcedUserEvent = 0x0003,
  ///Unlatch operation
  Unlatch = 0x0004,
}
impl AttrType for LockOperationTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Lock"),
  (0x0001, "Unlock"),
  (0x0002, "NonAccessUserEvent"),
  (0x0003, "ForcedUserEvent"),
  (0x0004, "Unlatch"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum LockStateEnum {
  ///Lock state is not fully locked
  NotFullyLocked = 0x0000,
  ///Lock state is fully locked
  Locked = 0x0001,
  ///Lock state is fully unlocked
  Unlocked = 0x0002,
  ///Lock state is fully unlocked and the latch is pulled
  Unlatched = 0x0003,
}
impl AttrType for LockStateEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "NotFullyLocked"),
  (0x0001, "Locked"),
  (0x0002, "Unlocked"),
  (0x0003, "Unlatched"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum LockTypeEnum {
  ///Physical lock type is dead bolt
  DeadBolt = 0x0000,
  ///Physical lock type is magnetic
  Magnetic = 0x0001,
  ///Physical lock type is other
  Other = 0x0002,
  ///Physical lock type is mortise
  Mortise = 0x0003,
  ///Physical lock type is rim
  Rim = 0x0004,
  ///Physical lock type is latch bolt
  LatchBolt = 0x0005,
  ///Physical lock type is cylindrical lock
  CylindricalLock = 0x0006,
  ///Physical lock type is tubular lock
  TubularLock = 0x0007,
  ///Physical lock type is interconnected lock
  InterconnectedLock = 0x0008,
  ///Physical lock type is dead latch
  DeadLatch = 0x0009,
  ///Physical lock type is door furniture
  DoorFurniture = 0x000a,
  ///Physical lock type is euro cylinder
  Eurocylinder = 0x000b,
}
impl AttrType for LockTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "DeadBolt"),
  (0x0001, "Magnetic"),
  (0x0002, "Other"),
  (0x0003, "Mortise"),
  (0x0004, "Rim"),
  (0x0005, "LatchBolt"),
  (0x0006, "CylindricalLock"),
  (0x0007, "TubularLock"),
  (0x0008, "InterconnectedLock"),
  (0x0009, "DeadLatch"),
  (0x000a, "DoorFurniture"),
  (0x000b, "Eurocylinder"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum OperatingModeEnum {
  Normal = 0x0000,
  Vacation = 0x0001,
  Privacy = 0x0002,
  NoRemoteLockUnlock = 0x0003,
  Passage = 0x0004,
}
impl AttrType for OperatingModeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Normal"),
  (0x0001, "Vacation"),
  (0x0002, "Privacy"),
  (0x0003, "NoRemoteLockUnlock"),
  (0x0004, "Passage"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum OperationErrorEnum {
  ///Lock/unlock error caused by unknown or unspecified source
  Unspecified = 0x0000,
  ///Lock/unlock error caused by invalid PIN, RFID, fingerprint or other credential
  InvalidCredential = 0x0001,
  ///Lock/unlock error caused by disabled USER or credential
  DisabledUserDenied = 0x0002,
  ///Lock/unlock error caused by schedule restriction
  Restricted = 0x0003,
  ///Lock/unlock error caused by insufficient battery power left to safely actuate the lock
  InsufficientBattery = 0x0004,
}
impl AttrType for OperationErrorEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "InvalidCredential"),
  (0x0002, "DisabledUserDenied"),
  (0x0003, "Restricted"),
  (0x0004, "InsufficientBattery"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum OperationSourceEnum {
  ///Lock/unlock operation came from unspecified source
  Unspecified = 0x0000,
  ///Lock/unlock operation came from manual operation (key, thumbturn, handle, etc).
  Manual = 0x0001,
  ///Lock/unlock operation came from proprietary remote source (e.g. vendor app/cloud)
  ProprietaryRemote = 0x0002,
  ///Lock/unlock operation came from keypad
  Keypad = 0x0003,
  ///Lock/unlock operation came from lock automatically (e.g. relock timer)
  Auto = 0x0004,
  ///Lock/unlock operation came from lock button (e.g. one touch or button)
  Button = 0x0005,
  ///Lock/unlock operation came from lock due to a schedule
  Schedule = 0x0006,
  ///Lock/unlock operation came from remote node
  Remote = 0x0007,
  ///Lock/unlock operation came from RFID card
  RFID = 0x0008,
  ///Lock/unlock operation came from biometric source (e.g. face, fingerprint/fingervein)
  Biometric = 0x0009,
  ///Lock/unlock operation came from an interaction defined in Aliro, or user change operation was a step-up credential provisioning as defined in Aliro
  Aliro = 0x000a,
}
impl AttrType for OperationSourceEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "Manual"),
  (0x0002, "ProprietaryRemote"),
  (0x0003, "Keypad"),
  (0x0004, "Auto"),
  (0x0005, "Button"),
  (0x0006, "Schedule"),
  (0x0007, "Remote"),
  (0x0008, "RFID"),
  (0x0009, "Biometric"),
  (0x000a, "Aliro"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum SoundVolumeEnum {
  ///Silent Mode
  Silent = 0x0000,
  ///Low Volume
  Low = 0x0001,
  ///High Volume
  High = 0x0002,
  ///Medium Volume
  Medium = 0x0003,
}
impl AttrType for SoundVolumeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Silent"),
  (0x0001, "Low"),
  (0x0002, "High"),
  (0x0003, "Medium"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum StatusCodeEnum {
  ///Entry would cause a duplicate credential/ID.
  Duplicate = 0x0002,
  ///Entry would replace an occupied slot.
  Occupied = 0x0003,
}
impl AttrType for StatusCodeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0002, "Duplicate"),
  (0x0003, "Occupied"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum UserStatusEnum {
  ///The user ID is available
  Available = 0x0000,
  ///The user ID is occupied and enabled
  OccupiedEnabled = 0x0001,
  ///The user ID is occupied and disabled
  OccupiedDisabled = 0x0003,
}
impl AttrType for UserStatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Available"),
  (0x0001, "OccupiedEnabled"),
  (0x0003, "OccupiedDisabled"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub enum UserTypeEnum {
  ///The user ID type is unrestricted
  UnrestrictedUser = 0x0000,
  ///The user ID type is schedule
  YearDayScheduleUser = 0x0001,
  ///The user ID type is schedule
  WeekDayScheduleUser = 0x0002,
  ///The user ID type is programming
  ProgrammingUser = 0x0003,
  ///The user ID type is non access
  NonAccessUser = 0x0004,
  ///The user ID type is forced
  ForcedUser = 0x0005,
  ///The user ID type is disposable
  DisposableUser = 0x0006,
  ///The user ID type is expiring
  ExpiringUser = 0x0007,
  ///The user ID type is schedule restricted
  ScheduleRestrictedUser = 0x0008,
  ///The user ID type is remote only
  RemoteOnlyUser = 0x0009,
}
impl AttrType for UserTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "UnrestrictedUser"),
  (0x0001, "YearDayScheduleUser"),
  (0x0002, "WeekDayScheduleUser"),
  (0x0003, "ProgrammingUser"),
  (0x0004, "NonAccessUser"),
  (0x0005, "ForcedUser"),
  (0x0006, "DisposableUser"),
  (0x0007, "ExpiringUser"),
  (0x0008, "ScheduleRestrictedUser"),
  (0x0009, "RemoteOnlyUser"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  pub struct AlarmMaskBitmap: u32 {
    ///Locking Mechanism Jammed
    const LockJammed = 1 << 0;
    ///Lock Reset to Factory Defaults
    const LockFactoryReset = 1 << 1;
    ///RF Module Power Cycled
    const LockRadioPowerCycled = 1 << 3;
    ///Tamper Alarm - wrong code entry limit
    const WrongCodeEntryLimit = 1 << 4;
    ///Tamper Alarm - front escutcheon removed from main
    const FrontEscutcheonRemoved = 1 << 5;
    ///Forced Door Open under Door Locked Condition
    const DoorForcedOpen = 1 << 6;
  }
}
impl AttrType for AlarmMaskBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "LockJammed"),
    (1, "LockFactoryReset"),
    (3, "LockRadioPowerCycled"),
    (4, "WrongCodeEntryLimit"),
    (5, "FrontEscutcheonRemoved"),
    (6, "DoorForcedOpen"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  pub struct ConfigurationRegisterBitmap: u32 {
    ///The state of local programming functionality
    const LocalProgramming = 1 << 0;
    ///The state of the keypad interface
    const KeypadInterface = 1 << 1;
    ///The state of the remote interface
    const RemoteInterface = 1 << 2;
    ///Sound volume is set to Silent value
    const SoundVolume = 1 << 5;
    ///Auto relock time it set to 0
    const AutoRelockTime = 1 << 6;
    ///LEDs is disabled
    const LEDSettings = 1 << 7;
  }
}
impl AttrType for ConfigurationRegisterBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "LocalProgramming"),
    (1, "KeypadInterface"),
    (2, "RemoteInterface"),
    (5, "SoundVolume"),
    (6, "AutoRelockTime"),
    (7, "LEDSettings"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  pub struct CredentialRulesBitmap: u32 {
    ///Only one credential is required for lock operation
    const Single = 1 << 0;
    ///Any two credentials are required for lock operation
    const Dual = 1 << 1;
    ///Any three credentials are required for lock operation
    const Tri = 1 << 2;
  }
}
impl AttrType for CredentialRulesBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Single"),
    (1, "Dual"),
    (2, "Tri"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  pub struct DaysMaskBitmap: u32 {
    ///Schedule is applied on Sunday
    const Sunday = 1 << 0;
    ///Schedule is applied on Monday
    const Monday = 1 << 1;
    ///Schedule is applied on Tuesday
    const Tuesday = 1 << 2;
    ///Schedule is applied on Wednesday
    const Wednesday = 1 << 3;
    ///Schedule is applied on Thursday
    const Thursday = 1 << 4;
    ///Schedule is applied on Friday
    const Friday = 1 << 5;
    ///Schedule is applied on Saturday
    const Saturday = 1 << 6;
  }
}
impl AttrType for DaysMaskBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Sunday"),
    (1, "Monday"),
    (2, "Tuesday"),
    (3, "Wednesday"),
    (4, "Thursday"),
    (5, "Friday"),
    (6, "Saturday"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  pub struct LocalProgrammingFeaturesBitmap: u32 {
    ///The state of the ability to add users, credentials or schedules on the device
    const AddUsersCredentialsSchedules = 1 << 0;
    ///The state of the ability to modify users, credentials or schedules on the device
    const ModifyUsersCredentialsSchedules = 1 << 1;
    ///The state of the ability to clear users, credentials or schedules on the device
    const ClearUsersCredentialsSchedules = 1 << 2;
    ///The state of the ability to adjust settings on the device
    const AdjustSettings = 1 << 3;
  }
}
impl AttrType for LocalProgrammingFeaturesBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "AddUsersCredentialsSchedules"),
    (1, "ModifyUsersCredentialsSchedules"),
    (2, "ClearUsersCredentialsSchedules"),
    (3, "AdjustSettings"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  pub struct OperatingModesBitmap: u32 {
    ///Normal operation mode
    const Normal = 1 << 0;
    ///Vacation operation mode
    const Vacation = 1 << 1;
    ///Privacy operation mode
    const Privacy = 1 << 2;
    ///No remote lock and unlock operation mode
    const NoRemoteLockUnlock = 1 << 3;
    ///Passage operation mode
    const Passage = 1 << 4;
  }
}
impl AttrType for OperatingModesBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Normal"),
    (1, "Vacation"),
    (2, "Privacy"),
    (3, "NoRemoteLockUnlock"),
    (4, "Passage"),
  ]);
}
#[derive(Debug, Clone, PartialEq)]
pub struct CredentialStruct {
  credentialtype: CredentialTypeEnum,
  credentialindex: u16,
}
impl AttrType for CredentialStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "CredentialType", attr_ty: CredentialTypeEnum::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "CredentialIndex", attr_ty: u16::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
}
