#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const GENERAL_COMMISSIONING: ClusterDef = ClusterDef {
  id: 0x0030,
  name: "General Commissioning",
  features: &[
    FeatureDef { bit: 0x0000, code: "TC", name: "TermsAndConditions", summary: "Supports Terms & Conditions acknowledgement" },
    FeatureDef { bit: 0x0001, code: "NR", name: "NetworkRecovery", summary: "Supports Network Recovery" },
  ],
  attributes: &[
    attr::Breadcrumb::ATTR,
    attr::BasicCommissioningInfo::ATTR,
    attr::RegulatoryConfig::ATTR,
    attr::LocationCapability::ATTR,
    attr::SupportsConcurrentConnection::ATTR,
    attr::TCAcceptedVersion::ATTR,
    attr::TCMinRequiredVersion::ATTR,
    attr::TCAcknowledgements::ATTR,
    attr::TCAcknowledgementsRequired::ATTR,
    attr::TCUpdateDeadline::ATTR,
    attr::RecoveryIdentifier::ATTR,
    attr::NetworkRecoveryReason::ATTR,
    attr::IsCommissioningWithoutPower::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::ArmFailSafe::COMMAND,
    command::SetRegulatoryConfig::COMMAND,
    command::CommissioningComplete::COMMAND,
    command::SetTCAcknowledgements::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct Breadcrumb;

  impl Attr for Breadcrumb {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "Breadcrumb",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = u64;
  }


  pub struct BasicCommissioningInfo;

  impl Attr for BasicCommissioningInfo {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "BasicCommissioningInfo",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::BasicCommissioningInfo;
  }


  pub struct RegulatoryConfig;

  impl Attr for RegulatoryConfig {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "RegulatoryConfig",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::RegulatoryLocationTypeEnum;
  }


  pub struct LocationCapability;

  impl Attr for LocationCapability {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "LocationCapability",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = ty::RegulatoryLocationTypeEnum;
  }


  pub struct SupportsConcurrentConnection;

  impl Attr for SupportsConcurrentConnection {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "SupportsConcurrentConnection",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


  pub struct TCAcceptedVersion;

  impl Attr for TCAcceptedVersion {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "TCAcceptedVersion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u16;
  }


  pub struct TCMinRequiredVersion;

  impl Attr for TCMinRequiredVersion {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "TCMinRequiredVersion",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = u16;
  }


  pub struct TCAcknowledgements;

  impl Attr for TCAcknowledgements {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "TCAcknowledgements",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = TypeUnsupported /* Map16 */;
  }


  pub struct TCAcknowledgementsRequired;

  impl Attr for TCAcknowledgementsRequired {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "TCAcknowledgementsRequired",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = bool;
  }


  pub struct TCUpdateDeadline;

  impl Attr for TCUpdateDeadline {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "TCUpdateDeadline",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Nullable<u32>;
  }


  pub struct RecoveryIdentifier;

  impl Attr for RecoveryIdentifier {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "RecoveryIdentifier",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = TypeUnsupported /* Octstr */;
  }


  pub struct NetworkRecoveryReason;

  impl Attr for NetworkRecoveryReason {
    const ATTR: AttrDef = AttrDef {
      id: 0x000b,
      name: "NetworkRecoveryReason",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::TooComplicated,
    };
    type Value = Nullable<ty::NetworkRecoveryReasonEnum>;
  }


  pub struct IsCommissioningWithoutPower;

  impl Attr for IsCommissioningWithoutPower {
    const ATTR: AttrDef = AttrDef {
      id: 0x000c,
      name: "IsCommissioningWithoutPower",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = bool;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ArmFailSafe {
  pub expiry_length_seconds: u16,
  pub breadcrumb: u64,
}
impl Command for ArmFailSafe {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "ArmFailSafe",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ExpiryLengthSeconds", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Breadcrumb", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ArmFailSafe {
  type Response = ArmFailSafeResponse;
}
impl TlvEncode for ArmFailSafe {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.expiry_length_seconds.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.breadcrumb.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArmFailSafeResponse {
  pub error_code: ty::CommissioningErrorEnum,
  pub debug_text: String,
}
impl Command for ArmFailSafeResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "ArmFailSafeResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ErrorCode", attr_ty: <ty::CommissioningErrorEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "DebugText", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for ArmFailSafeResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct SetRegulatoryConfig {
  pub new_regulatory_config: ty::RegulatoryLocationTypeEnum,
  pub country_code: String,
  pub breadcrumb: u64,
}
impl Command for SetRegulatoryConfig {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "SetRegulatoryConfig",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "NewRegulatoryConfig", attr_ty: <ty::RegulatoryLocationTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "CountryCode", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "Breadcrumb", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SetRegulatoryConfig {
  type Response = SetRegulatoryConfigResponse;
}
impl TlvEncode for SetRegulatoryConfig {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.new_regulatory_config.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.country_code.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.breadcrumb.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetRegulatoryConfigResponse {
  pub error_code: ty::CommissioningErrorEnum,
  pub debug_text: String,
}
impl Command for SetRegulatoryConfigResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "SetRegulatoryConfigResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ErrorCode", attr_ty: <ty::CommissioningErrorEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "DebugText", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for SetRegulatoryConfigResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct CommissioningComplete {
}
impl Command for CommissioningComplete {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "CommissioningComplete",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for CommissioningComplete {
  type Response = CommissioningCompleteResponse;
}
impl TlvEncode for CommissioningComplete {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommissioningCompleteResponse {
  pub error_code: ty::CommissioningErrorEnum,
  pub debug_text: String,
}
impl Command for CommissioningCompleteResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "CommissioningCompleteResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ErrorCode", attr_ty: <ty::CommissioningErrorEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "DebugText", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for CommissioningCompleteResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct SetTCAcknowledgements {
  pub tc_version: u16,
  pub tc_user_response: TypeUnsupported /* Map16 */,
}
impl Command for SetTCAcknowledgements {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "SetTCAcknowledgements",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "TCVersion", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "TCUserResponse", attr_ty: <TypeUnsupported /* Map16 */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SetTCAcknowledgements {
  type Response = SetTCAcknowledgementsResponse;
}
impl TlvEncode for SetTCAcknowledgements {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.tc_version.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.tc_user_response.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetTCAcknowledgementsResponse {
  pub error_code: ty::CommissioningErrorEnum,
}
impl Command for SetTCAcknowledgementsResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "SetTCAcknowledgementsResponse",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "ErrorCode", attr_ty: <ty::CommissioningErrorEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for SetTCAcknowledgementsResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum CommissioningErrorEnum {
  ///No error
  OK = 0x0000,
  ///Attempting to set regulatory configuration to a region or indoor/outdoor mode for which the server does not have proper configuration.
  ValueOutsideRange = 0x0001,
  ///Executed CommissioningComplete outside CASE session.
  InvalidAuthentication = 0x0002,
  ///Executed CommissioningComplete when there was no active Fail-Safe context.
  NoFailSafe = 0x0003,
  ///Attempting to arm fail-safe or execute CommissioningComplete from a fabric different than the one associated with the current fail-safe context.
  BusyWithOtherAdmin = 0x0004,
  ///One or more required TC features from the Enhanced Setup Flow were not accepted.
  RequiredTCNotAccepted = 0x0005,
  ///TCAcknowledgementsNotReceived No or insufficient acknowledgements from the user for the TC features were received.
  TCAcknowledgementsNotReceived = 0x0006,
  ///TCMinVersionNotMet The version of the TC features acknowledged by the user did not meet the minimum required version.
  TCMinVersionNotMet = 0x0007,
}
impl AttrType for CommissioningErrorEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "OK"),
  (0x0001, "ValueOutsideRange"),
  (0x0002, "InvalidAuthentication"),
  (0x0003, "NoFailSafe"),
  (0x0004, "BusyWithOtherAdmin"),
  (0x0005, "RequiredTCNotAccepted"),
  (0x0006, "TCAcknowledgementsNotReceived"),
  (0x0007, "TCMinVersionNotMet"),
  ]);
}
impl TlvPut for CommissioningErrorEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for CommissioningErrorEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::OK),
      0x0001 => Ok(Self::ValueOutsideRange),
      0x0002 => Ok(Self::InvalidAuthentication),
      0x0003 => Ok(Self::NoFailSafe),
      0x0004 => Ok(Self::BusyWithOtherAdmin),
      0x0005 => Ok(Self::RequiredTCNotAccepted),
      0x0006 => Ok(Self::TCAcknowledgementsNotReceived),
      0x0007 => Ok(Self::TCMinVersionNotMet),
      _ => Err(())
    }
  }
}
impl TlvDecode for CommissioningErrorEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum NetworkRecoveryReasonEnum {
  ///Unspecified / unknown reason of network failure
  Unspecified = 0x0000,
  ///Credentials for the configured operational network are not valid
  Auth = 0x0001,
  ///Configured network cannot be found (e.g. the device cannot see the configured Wi-Fi SSID, Thread end-node is unable to find a parent router on the PAN)
  Visibility = 0x0002,
}
impl AttrType for NetworkRecoveryReasonEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "Auth"),
  (0x0002, "Visibility"),
  ]);
}
impl TlvPut for NetworkRecoveryReasonEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for NetworkRecoveryReasonEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unspecified),
      0x0001 => Ok(Self::Auth),
      0x0002 => Ok(Self::Visibility),
      _ => Err(())
    }
  }
}
impl TlvDecode for NetworkRecoveryReasonEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum RegulatoryLocationTypeEnum {
  ///Indoor only
  Indoor = 0x0000,
  ///Outdoor only
  Outdoor = 0x0001,
  ///Indoor/Outdoor
  IndoorOutdoor = 0x0002,
}
impl AttrType for RegulatoryLocationTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Indoor"),
  (0x0001, "Outdoor"),
  (0x0002, "IndoorOutdoor"),
  ]);
}
impl TlvPut for RegulatoryLocationTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for RegulatoryLocationTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Indoor),
      0x0001 => Ok(Self::Outdoor),
      0x0002 => Ok(Self::IndoorOutdoor),
      _ => Err(())
    }
  }
}
impl TlvDecode for RegulatoryLocationTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct BasicCommissioningInfo {
  pub fail_safe_expiry_length_seconds: u16,
  pub max_cumulative_failsafe_seconds: u16,
}
impl AttrType for BasicCommissioningInfo {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "FailSafeExpiryLengthSeconds", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "MaxCumulativeFailsafeSeconds", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for BasicCommissioningInfo {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.fail_safe_expiry_length_seconds.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.max_cumulative_failsafe_seconds.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for BasicCommissioningInfo {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut fail_safe_expiry_length_seconds: Option<u16> = None;
    let mut max_cumulative_failsafe_seconds: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { fail_safe_expiry_length_seconds = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { max_cumulative_failsafe_seconds = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      fail_safe_expiry_length_seconds: fail_safe_expiry_length_seconds.ok_or(TlvDecodeErr::MissingField("FailSafeExpiryLengthSeconds"))?,
      max_cumulative_failsafe_seconds: max_cumulative_failsafe_seconds.ok_or(TlvDecodeErr::MissingField("MaxCumulativeFailsafeSeconds"))?,
    })
  }
}

}
