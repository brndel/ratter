#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const KEYPAD_INPUT: ClusterDef = ClusterDef {
  id: 0x0509,
  name: "Keypad Input",
  features: &[
    FeatureDef { bit: 0x0000, code: "NV", name: "NavigationKeyCodes", summary: "Supports UP, DOWN, LEFT, RIGHT, SELECT, BACK, EXIT, MENU" },
    FeatureDef { bit: 0x0001, code: "LK", name: "LocationKeys", summary: "Supports CEC keys 0x0A (Settings) and 0x09 (Home)" },
    FeatureDef { bit: 0x0002, code: "NK", name: "NumberKeys", summary: "Supports numeric input 0..9" },
  ],
  attributes: &[
  ],
  events: &[
  ],
  commands: &[
    command::SendKey::COMMAND,
  ],
};
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct SendKey {
  pub key_code: ty::CecKeyCodeEnum,
}
impl Command for SendKey {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "SendKey",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "KeyCode", attr_ty: <ty::CecKeyCodeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SendKey {
  type Response = SendKeyResponse;
}
impl TlvEncode for SendKey {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.key_code.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SendKeyResponse {
  pub status: ty::StatusEnum,
}
impl Command for SendKeyResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "SendKeyResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <ty::StatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for SendKeyResponse {}

}
pub mod ty {
  pub use crate::globals::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum CecKeyCodeEnum {
  Select = 0x0000,
  Up = 0x0001,
  Down = 0x0002,
  Left = 0x0003,
  Right = 0x0004,
  RightUp = 0x0005,
  RightDown = 0x0006,
  LeftUp = 0x0007,
  LeftDown = 0x0008,
  RootMenu = 0x0009,
  SetupMenu = 0x000a,
  ContentsMenu = 0x000b,
  FavoriteMenu = 0x000c,
  Exit = 0x000d,
  MediaTopMenu = 0x0010,
  MediaContextSensitiveMenu = 0x0011,
  NumberEntryMode = 0x001d,
  Number11 = 0x001e,
  Number12 = 0x001f,
  Number0OrNumber10 = 0x0020,
  Numbers1 = 0x0021,
  Numbers2 = 0x0022,
  Numbers3 = 0x0023,
  Numbers4 = 0x0024,
  Numbers5 = 0x0025,
  Numbers6 = 0x0026,
  Numbers7 = 0x0027,
  Numbers8 = 0x0028,
  Numbers9 = 0x0029,
  Dot = 0x002a,
  Enter = 0x002b,
  Clear = 0x002c,
  NextFavorite = 0x002f,
  ChannelUp = 0x0030,
  ChannelDown = 0x0031,
  PreviousChannel = 0x0032,
  SoundSelect = 0x0033,
  InputSelect = 0x0034,
  DisplayInformation = 0x0035,
  Help = 0x0036,
  PageUp = 0x0037,
  PageDown = 0x0038,
  Power = 0x0040,
  VolumeUp = 0x0041,
  VolumeDown = 0x0042,
  Mute = 0x0043,
  Play = 0x0044,
  Stop = 0x0045,
  Pause = 0x0046,
  Record = 0x0047,
  Rewind = 0x0048,
  FastForward = 0x0049,
  Eject = 0x004a,
  Forward = 0x004b,
  Backward = 0x004c,
  StopRecord = 0x004d,
  PauseRecord = 0x004e,
  Reserved = 0x004f,
  Angle = 0x0050,
  SubPicture = 0x0051,
  VideoOnDemand = 0x0052,
  ElectronicProgramGuide = 0x0053,
  TimerProgramming = 0x0054,
  InitialConfiguration = 0x0055,
  SelectBroadcastType = 0x0056,
  SelectSoundPresentation = 0x0057,
  PlayFunction = 0x0060,
  PausePlayFunction = 0x0061,
  RecordFunction = 0x0062,
  PauseRecordFunction = 0x0063,
  StopFunction = 0x0064,
  MuteFunction = 0x0065,
  RestoreVolumeFunction = 0x0066,
  TuneFunction = 0x0067,
  SelectMediaFunction = 0x0068,
  SelectAvInputFunction = 0x0069,
  SelectAudioInputFunction = 0x006a,
  PowerToggleFunction = 0x006b,
  PowerOffFunction = 0x006c,
  PowerOnFunction = 0x006d,
  F1Blue = 0x0071,
  F2Red = 0x0072,
  F3Green = 0x0073,
  F4Yellow = 0x0074,
  F5 = 0x0075,
  Data = 0x0076,
}
impl AttrType for CecKeyCodeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Select"),
  (0x0001, "Up"),
  (0x0002, "Down"),
  (0x0003, "Left"),
  (0x0004, "Right"),
  (0x0005, "RightUp"),
  (0x0006, "RightDown"),
  (0x0007, "LeftUp"),
  (0x0008, "LeftDown"),
  (0x0009, "RootMenu"),
  (0x000a, "SetupMenu"),
  (0x000b, "ContentsMenu"),
  (0x000c, "FavoriteMenu"),
  (0x000d, "Exit"),
  (0x0010, "MediaTopMenu"),
  (0x0011, "MediaContextSensitiveMenu"),
  (0x001d, "NumberEntryMode"),
  (0x001e, "Number11"),
  (0x001f, "Number12"),
  (0x0020, "Number0OrNumber10"),
  (0x0021, "Numbers1"),
  (0x0022, "Numbers2"),
  (0x0023, "Numbers3"),
  (0x0024, "Numbers4"),
  (0x0025, "Numbers5"),
  (0x0026, "Numbers6"),
  (0x0027, "Numbers7"),
  (0x0028, "Numbers8"),
  (0x0029, "Numbers9"),
  (0x002a, "Dot"),
  (0x002b, "Enter"),
  (0x002c, "Clear"),
  (0x002f, "NextFavorite"),
  (0x0030, "ChannelUp"),
  (0x0031, "ChannelDown"),
  (0x0032, "PreviousChannel"),
  (0x0033, "SoundSelect"),
  (0x0034, "InputSelect"),
  (0x0035, "DisplayInformation"),
  (0x0036, "Help"),
  (0x0037, "PageUp"),
  (0x0038, "PageDown"),
  (0x0040, "Power"),
  (0x0041, "VolumeUp"),
  (0x0042, "VolumeDown"),
  (0x0043, "Mute"),
  (0x0044, "Play"),
  (0x0045, "Stop"),
  (0x0046, "Pause"),
  (0x0047, "Record"),
  (0x0048, "Rewind"),
  (0x0049, "FastForward"),
  (0x004a, "Eject"),
  (0x004b, "Forward"),
  (0x004c, "Backward"),
  (0x004d, "StopRecord"),
  (0x004e, "PauseRecord"),
  (0x004f, "Reserved"),
  (0x0050, "Angle"),
  (0x0051, "SubPicture"),
  (0x0052, "VideoOnDemand"),
  (0x0053, "ElectronicProgramGuide"),
  (0x0054, "TimerProgramming"),
  (0x0055, "InitialConfiguration"),
  (0x0056, "SelectBroadcastType"),
  (0x0057, "SelectSoundPresentation"),
  (0x0060, "PlayFunction"),
  (0x0061, "PausePlayFunction"),
  (0x0062, "RecordFunction"),
  (0x0063, "PauseRecordFunction"),
  (0x0064, "StopFunction"),
  (0x0065, "MuteFunction"),
  (0x0066, "RestoreVolumeFunction"),
  (0x0067, "TuneFunction"),
  (0x0068, "SelectMediaFunction"),
  (0x0069, "SelectAvInputFunction"),
  (0x006a, "SelectAudioInputFunction"),
  (0x006b, "PowerToggleFunction"),
  (0x006c, "PowerOffFunction"),
  (0x006d, "PowerOnFunction"),
  (0x0071, "F1Blue"),
  (0x0072, "F2Red"),
  (0x0073, "F3Green"),
  (0x0074, "F4Yellow"),
  (0x0075, "F5"),
  (0x0076, "Data"),
  ]);
}
impl TlvPut for CecKeyCodeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for CecKeyCodeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Select),
      0x0001 => Ok(Self::Up),
      0x0002 => Ok(Self::Down),
      0x0003 => Ok(Self::Left),
      0x0004 => Ok(Self::Right),
      0x0005 => Ok(Self::RightUp),
      0x0006 => Ok(Self::RightDown),
      0x0007 => Ok(Self::LeftUp),
      0x0008 => Ok(Self::LeftDown),
      0x0009 => Ok(Self::RootMenu),
      0x000a => Ok(Self::SetupMenu),
      0x000b => Ok(Self::ContentsMenu),
      0x000c => Ok(Self::FavoriteMenu),
      0x000d => Ok(Self::Exit),
      0x0010 => Ok(Self::MediaTopMenu),
      0x0011 => Ok(Self::MediaContextSensitiveMenu),
      0x001d => Ok(Self::NumberEntryMode),
      0x001e => Ok(Self::Number11),
      0x001f => Ok(Self::Number12),
      0x0020 => Ok(Self::Number0OrNumber10),
      0x0021 => Ok(Self::Numbers1),
      0x0022 => Ok(Self::Numbers2),
      0x0023 => Ok(Self::Numbers3),
      0x0024 => Ok(Self::Numbers4),
      0x0025 => Ok(Self::Numbers5),
      0x0026 => Ok(Self::Numbers6),
      0x0027 => Ok(Self::Numbers7),
      0x0028 => Ok(Self::Numbers8),
      0x0029 => Ok(Self::Numbers9),
      0x002a => Ok(Self::Dot),
      0x002b => Ok(Self::Enter),
      0x002c => Ok(Self::Clear),
      0x002f => Ok(Self::NextFavorite),
      0x0030 => Ok(Self::ChannelUp),
      0x0031 => Ok(Self::ChannelDown),
      0x0032 => Ok(Self::PreviousChannel),
      0x0033 => Ok(Self::SoundSelect),
      0x0034 => Ok(Self::InputSelect),
      0x0035 => Ok(Self::DisplayInformation),
      0x0036 => Ok(Self::Help),
      0x0037 => Ok(Self::PageUp),
      0x0038 => Ok(Self::PageDown),
      0x0040 => Ok(Self::Power),
      0x0041 => Ok(Self::VolumeUp),
      0x0042 => Ok(Self::VolumeDown),
      0x0043 => Ok(Self::Mute),
      0x0044 => Ok(Self::Play),
      0x0045 => Ok(Self::Stop),
      0x0046 => Ok(Self::Pause),
      0x0047 => Ok(Self::Record),
      0x0048 => Ok(Self::Rewind),
      0x0049 => Ok(Self::FastForward),
      0x004a => Ok(Self::Eject),
      0x004b => Ok(Self::Forward),
      0x004c => Ok(Self::Backward),
      0x004d => Ok(Self::StopRecord),
      0x004e => Ok(Self::PauseRecord),
      0x004f => Ok(Self::Reserved),
      0x0050 => Ok(Self::Angle),
      0x0051 => Ok(Self::SubPicture),
      0x0052 => Ok(Self::VideoOnDemand),
      0x0053 => Ok(Self::ElectronicProgramGuide),
      0x0054 => Ok(Self::TimerProgramming),
      0x0055 => Ok(Self::InitialConfiguration),
      0x0056 => Ok(Self::SelectBroadcastType),
      0x0057 => Ok(Self::SelectSoundPresentation),
      0x0060 => Ok(Self::PlayFunction),
      0x0061 => Ok(Self::PausePlayFunction),
      0x0062 => Ok(Self::RecordFunction),
      0x0063 => Ok(Self::PauseRecordFunction),
      0x0064 => Ok(Self::StopFunction),
      0x0065 => Ok(Self::MuteFunction),
      0x0066 => Ok(Self::RestoreVolumeFunction),
      0x0067 => Ok(Self::TuneFunction),
      0x0068 => Ok(Self::SelectMediaFunction),
      0x0069 => Ok(Self::SelectAvInputFunction),
      0x006a => Ok(Self::SelectAudioInputFunction),
      0x006b => Ok(Self::PowerToggleFunction),
      0x006c => Ok(Self::PowerOffFunction),
      0x006d => Ok(Self::PowerOnFunction),
      0x0071 => Ok(Self::F1Blue),
      0x0072 => Ok(Self::F2Red),
      0x0073 => Ok(Self::F3Green),
      0x0074 => Ok(Self::F4Yellow),
      0x0075 => Ok(Self::F5),
      0x0076 => Ok(Self::Data),
      _ => Err(())
    }
  }
}
impl TlvDecode for CecKeyCodeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StatusEnum {
  ///Succeeded
  Success = 0x0000,
  ///Key code is not supported.
  UnsupportedKey = 0x0001,
  ///Requested key code is invalid in the context of the responder's current state.
  InvalidKeyInCurrentState = 0x0002,
}
impl AttrType for StatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Success"),
  (0x0001, "UnsupportedKey"),
  (0x0002, "InvalidKeyInCurrentState"),
  ]);
}
impl TlvPut for StatusEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StatusEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Success),
      0x0001 => Ok(Self::UnsupportedKey),
      0x0002 => Ok(Self::InvalidKeyInCurrentState),
      _ => Err(())
    }
  }
}
impl TlvDecode for StatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
}
