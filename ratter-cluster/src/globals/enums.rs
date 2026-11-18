use crate::cluster_def::*;
use crate::tlv::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum AtomicRequestTypeEnum {
  ///Begin an atomic write
  BeginWrite = 0x0000,
  ///Commit an atomic write
  CommitWrite = 0x0001,
  ///Rollback an atomic write, discarding any pending changes
  RollbackWrite = 0x0002,
}
impl AttrType for AtomicRequestTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "BeginWrite"),
  (0x0001, "CommitWrite"),
  (0x0002, "RollbackWrite"),
  ]);
}
impl TlvPut for AtomicRequestTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for AtomicRequestTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::BeginWrite),
      0x0001 => Ok(Self::CommitWrite),
      0x0002 => Ok(Self::RollbackWrite),
      _ => Err(())
    }
  }
}
impl TlvDecode for AtomicRequestTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum CertificationTypeEnum {
  ///DeviceAttestationPKI
  DeviceAttestationPKI = 0x0000,
  ///OperationalPKI
  OperationalPKI = 0x0001,
  ///VIDSignerPKI
  VIDSignerPKI = 0x0002,
}
impl AttrType for CertificationTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "DeviceAttestationPKI"),
  (0x0001, "OperationalPKI"),
  (0x0002, "VIDSignerPKI"),
  ]);
}
impl TlvPut for CertificationTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for CertificationTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::DeviceAttestationPKI),
      0x0001 => Ok(Self::OperationalPKI),
      0x0002 => Ok(Self::VIDSignerPKI),
      _ => Err(())
    }
  }
}
impl TlvDecode for CertificationTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum MeasurementTypeEnum {
  Unspecified = 0x0000,
  ///Voltage in millivolts (mV)
  Voltage = 0x0001,
  ///Active current in milliamps (mA)
  ActiveCurrent = 0x0002,
  ///Reactive current in milliamps (mA)
  ReactiveCurrent = 0x0003,
  ///Apparent current in milliamps (mA)
  ApparentCurrent = 0x0004,
  ///Active power in milliwatts (mW)
  ActivePower = 0x0005,
  ///Reactive power in millivolt-amps reactive (mVAR)
  ReactivePower = 0x0006,
  ///Apparent power in millivolt-amps (mVA)
  ApparentPower = 0x0007,
  ///Root mean squared voltage in millivolts (mV)
  RMSVoltage = 0x0008,
  ///Root mean squared current in milliamps (mA)
  RMSCurrent = 0x0009,
  ///Root mean squared power in milliwatts (mW)
  RMSPower = 0x000a,
  ///AC frequency in millihertz (mHz)
  Frequency = 0x000b,
  ///Power Factor ratio in +/- 1/100ths of a percent.
  PowerFactor = 0x000c,
  ///AC neutral current in milliamps (mA)
  NeutralCurrent = 0x000d,
  ///Electrical energy in milliwatt-hours (mWh)
  ElectricalEnergy = 0x000e,
  ///Reactive power in millivolt-amp-hours reactive (mVARh)
  ReactiveEnergy = 0x000f,
  ///Apparent power in millivolt-amp-hours (mVAh)
  ApparentEnergy = 0x0010,
  ///Soil moisture in percent
  SoilMoisture = 0x0011,
}
impl AttrType for MeasurementTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Unspecified"),
  (0x0001, "Voltage"),
  (0x0002, "ActiveCurrent"),
  (0x0003, "ReactiveCurrent"),
  (0x0004, "ApparentCurrent"),
  (0x0005, "ActivePower"),
  (0x0006, "ReactivePower"),
  (0x0007, "ApparentPower"),
  (0x0008, "RMSVoltage"),
  (0x0009, "RMSCurrent"),
  (0x000a, "RMSPower"),
  (0x000b, "Frequency"),
  (0x000c, "PowerFactor"),
  (0x000d, "NeutralCurrent"),
  (0x000e, "ElectricalEnergy"),
  (0x000f, "ReactiveEnergy"),
  (0x0010, "ApparentEnergy"),
  (0x0011, "SoilMoisture"),
  ]);
}
impl TlvPut for MeasurementTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for MeasurementTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Unspecified),
      0x0001 => Ok(Self::Voltage),
      0x0002 => Ok(Self::ActiveCurrent),
      0x0003 => Ok(Self::ReactiveCurrent),
      0x0004 => Ok(Self::ApparentCurrent),
      0x0005 => Ok(Self::ActivePower),
      0x0006 => Ok(Self::ReactivePower),
      0x0007 => Ok(Self::ApparentPower),
      0x0008 => Ok(Self::RMSVoltage),
      0x0009 => Ok(Self::RMSCurrent),
      0x000a => Ok(Self::RMSPower),
      0x000b => Ok(Self::Frequency),
      0x000c => Ok(Self::PowerFactor),
      0x000d => Ok(Self::NeutralCurrent),
      0x000e => Ok(Self::ElectricalEnergy),
      0x000f => Ok(Self::ReactiveEnergy),
      0x0010 => Ok(Self::ApparentEnergy),
      0x0011 => Ok(Self::SoilMoisture),
      _ => Err(())
    }
  }
}
impl TlvDecode for MeasurementTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum PowerThresholdSourceEnum {
  ///The power threshold comes from a signed contract
  Contract = 0x0000,
  ///The power threshold comes from a legal regulator
  Regulator = 0x0001,
  ///The power threshold comes from a certified limits of the meter
  Equipment = 0x0002,
}
impl AttrType for PowerThresholdSourceEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Contract"),
  (0x0001, "Regulator"),
  (0x0002, "Equipment"),
  ]);
}
impl TlvPut for PowerThresholdSourceEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for PowerThresholdSourceEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Contract),
      0x0001 => Ok(Self::Regulator),
      0x0002 => Ok(Self::Equipment),
      _ => Err(())
    }
  }
}
impl TlvDecode for PowerThresholdSourceEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum SoftwareVersionCertificationStatusEnum {
  ///dev-test
  dev_test = 0x0000,
  ///provisional
  provisional = 0x0001,
  ///certified
  certified = 0x0002,
  ///revoked
  revoked = 0x0003,
}
impl AttrType for SoftwareVersionCertificationStatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "dev_test"),
  (0x0001, "provisional"),
  (0x0002, "certified"),
  (0x0003, "revoked"),
  ]);
}
impl TlvPut for SoftwareVersionCertificationStatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for SoftwareVersionCertificationStatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::dev_test),
      0x0001 => Ok(Self::provisional),
      0x0002 => Ok(Self::certified),
      0x0003 => Ok(Self::revoked),
      _ => Err(())
    }
  }
}
impl TlvDecode for SoftwareVersionCertificationStatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StreamUsageEnum {
  ///Internal video stream.
  Internal = 0x0000,
  ///Stream for recording clips.
  Recording = 0x0001,
  ///Stream for analysis and entity detection.
  Analysis = 0x0002,
  ///Stream for liveview.
  LiveView = 0x0003,
}
impl AttrType for StreamUsageEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Internal"),
  (0x0001, "Recording"),
  (0x0002, "Analysis"),
  (0x0003, "LiveView"),
  ]);
}
impl TlvPut for StreamUsageEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StreamUsageEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Internal),
      0x0001 => Ok(Self::Recording),
      0x0002 => Ok(Self::Analysis),
      0x0003 => Ok(Self::LiveView),
      _ => Err(())
    }
  }
}
impl TlvDecode for StreamUsageEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TariffPriceTypeEnum {
  ///Standard tariff price
  Standard = 0x0000,
  ///Price during CPP events
  Critical = 0x0001,
  ///Price during VPP events
  Virtual = 0x0002,
  ///Price incentives
  Incentive = 0x0003,
  ///Price incentive signals
  IncentiveSignal = 0x0004,
}
impl AttrType for TariffPriceTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Standard"),
  (0x0001, "Critical"),
  (0x0002, "Virtual"),
  (0x0003, "Incentive"),
  (0x0004, "IncentiveSignal"),
  ]);
}
impl TlvPut for TariffPriceTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TariffPriceTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Standard),
      0x0001 => Ok(Self::Critical),
      0x0002 => Ok(Self::Virtual),
      0x0003 => Ok(Self::Incentive),
      0x0004 => Ok(Self::IncentiveSignal),
      _ => Err(())
    }
  }
}
impl TlvDecode for TariffPriceTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum TariffUnitEnum {
  ///Kilowatt Hours
  kWh = 0x0000,
  ///Kilovolt-amp Hours
  kVAh = 0x0001,
}
impl AttrType for TariffUnitEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "kWh"),
  (0x0001, "kVAh"),
  ]);
}
impl TlvPut for TariffUnitEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for TariffUnitEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::kWh),
      0x0001 => Ok(Self::kVAh),
      _ => Err(())
    }
  }
}
impl TlvDecode for TariffUnitEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ThreeLevelAutoEnum {
  ///Automatic Level
  Auto = 0x0000,
  ///Low Level
  Low = 0x0001,
  ///Medium Level
  Medium = 0x0002,
  ///High Level
  High = 0x0003,
}
impl AttrType for ThreeLevelAutoEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Auto"),
  (0x0001, "Low"),
  (0x0002, "Medium"),
  (0x0003, "High"),
  ]);
}
impl TlvPut for ThreeLevelAutoEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ThreeLevelAutoEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Auto),
      0x0001 => Ok(Self::Low),
      0x0002 => Ok(Self::Medium),
      0x0003 => Ok(Self::High),
      _ => Err(())
    }
  }
}
impl TlvDecode for ThreeLevelAutoEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}

#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum WebRTCEndReasonEnum {
  ///No media connection could be established to the other party
  ICEFailed = 0x0000,
  ///The call timed out whilst waiting for ICE candidate gathering to complete
  ICETimeout = 0x0001,
  ///The user chose to end the call
  UserHangup = 0x0002,
  ///The remote party is busy
  UserBusy = 0x0003,
  ///The call was replaced by another call
  Replaced = 0x0004,
  ///An error code when there is no local mic/camera to use. This may be because the hardware isn't plugged in, or the user has explicitly denied access
  NoUserMedia = 0x0005,
  ///The call timed out whilst waiting for the offer/answer step to complete
  InviteTimeout = 0x0006,
  ///The call was answered from a different device
  AnsweredElsewhere = 0x0007,
  ///The was unable to continue due to not enough resources or available streams
  OutOfResources = 0x0008,
  ///The call ended due to a media timeout
  MediaTimeout = 0x0009,
  ///The call ended due to hitting a low power condition
  LowPower = 0x000a,
  ///The call ended due to the camera being set into a privacy mode.
  PrivacyMode = 0x000b,
  ///Unknown or unspecified reason
  UnknownReason = 0x000c,
}
impl AttrType for WebRTCEndReasonEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "ICEFailed"),
  (0x0001, "ICETimeout"),
  (0x0002, "UserHangup"),
  (0x0003, "UserBusy"),
  (0x0004, "Replaced"),
  (0x0005, "NoUserMedia"),
  (0x0006, "InviteTimeout"),
  (0x0007, "AnsweredElsewhere"),
  (0x0008, "OutOfResources"),
  (0x0009, "MediaTimeout"),
  (0x000a, "LowPower"),
  (0x000b, "PrivacyMode"),
  (0x000c, "UnknownReason"),
  ]);
}
impl TlvPut for WebRTCEndReasonEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for WebRTCEndReasonEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::ICEFailed),
      0x0001 => Ok(Self::ICETimeout),
      0x0002 => Ok(Self::UserHangup),
      0x0003 => Ok(Self::UserBusy),
      0x0004 => Ok(Self::Replaced),
      0x0005 => Ok(Self::NoUserMedia),
      0x0006 => Ok(Self::InviteTimeout),
      0x0007 => Ok(Self::AnsweredElsewhere),
      0x0008 => Ok(Self::OutOfResources),
      0x0009 => Ok(Self::MediaTimeout),
      0x000a => Ok(Self::LowPower),
      0x000b => Ok(Self::PrivacyMode),
      0x000c => Ok(Self::UnknownReason),
      _ => Err(())
    }
  }
}
impl TlvDecode for WebRTCEndReasonEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}

