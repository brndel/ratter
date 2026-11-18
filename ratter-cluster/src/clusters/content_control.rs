#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const CONTENT_CONTROL: ClusterDef = ClusterDef {
  id: 0x050f,
  name: "Content Control",
  features: &[
    FeatureDef { bit: 0x0000, code: "ST", name: "ScreenTime", summary: "Supports managing screen time limits" },
    FeatureDef { bit: 0x0001, code: "PM", name: "PINManagement", summary: "Supports managing a PIN code which is used for restricting access to configuration of this feature" },
    FeatureDef { bit: 0x0002, code: "BU", name: "BlockUnrated", summary: "Supports managing content controls for unrated content" },
    FeatureDef { bit: 0x0003, code: "OCR", name: "OnDemandContentRating", summary: "Supports managing content controls based upon rating threshold for on demand content" },
    FeatureDef { bit: 0x0004, code: "SCR", name: "ScheduledContentRating", summary: "Supports managing content controls based upon rating threshold for scheduled content" },
    FeatureDef { bit: 0x0005, code: "BC", name: "BlockChannels", summary: "Supports managing a set of channels that are prohibited" },
    FeatureDef { bit: 0x0006, code: "BA", name: "BlockApplications", summary: "Supports managing a set of applications that are prohibited" },
    FeatureDef { bit: 0x0007, code: "BTW", name: "BlockContentTimeWindow", summary: "Supports managing content controls based upon setting time window in which all contents and applications SHALL be blocked" },
  ],
  attributes: &[
    attr::Enabled::ATTR,
    attr::OnDemandRatings::ATTR,
    attr::OnDemandRatingThreshold::ATTR,
    attr::ScheduledContentRatings::ATTR,
    attr::ScheduledContentRatingThreshold::ATTR,
    attr::ScreenDailyTime::ATTR,
    attr::RemainingScreenTime::ATTR,
    attr::BlockUnrated::ATTR,
    attr::BlockChannelList::ATTR,
    attr::BlockApplicationList::ATTR,
    attr::BlockContentTimeWindow::ATTR,
  ],
  events: &[
    event::RemainingScreenTimeExpired::EVENT,
    event::EnteringBlockContentTimeWindow::EVENT,
  ],
  commands: &[
    command::UpdatePIN::COMMAND,
    command::ResetPIN::COMMAND,
    command::Enable::COMMAND,
    command::Disable::COMMAND,
    command::AddBonusTime::COMMAND,
    command::SetScreenDailyTime::COMMAND,
    command::BlockUnratedContent::COMMAND,
    command::UnblockUnratedContent::COMMAND,
    command::SetOnDemandRatingThreshold::COMMAND,
    command::SetScheduledContentRatingThreshold::COMMAND,
    command::AddBlockChannels::COMMAND,
    command::RemoveBlockChannels::COMMAND,
    command::AddBlockApplications::COMMAND,
    command::RemoveBlockApplications::COMMAND,
    command::SetBlockContentTimeWindow::COMMAND,
    command::RemoveBlockContentTimeWindow::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct Enabled;

  impl Attr for Enabled {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "Enabled",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = bool;
  }


  pub struct OnDemandRatings;

  impl Attr for OnDemandRatings {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "OnDemandRatings",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = Vec<ty::RatingNameStruct>;
  }


  pub struct OnDemandRatingThreshold;

  impl Attr for OnDemandRatingThreshold {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "OnDemandRatingThreshold",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(3),
    };
    type Value = String;
  }


  pub struct ScheduledContentRatings;

  impl Attr for ScheduledContentRatings {
    const ATTR: AttrDef = AttrDef {
      id: 0x0003,
      name: "ScheduledContentRatings",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = Vec<ty::RatingNameStruct>;
  }


  pub struct ScheduledContentRatingThreshold;

  impl Attr for ScheduledContentRatingThreshold {
    const ATTR: AttrDef = AttrDef {
      id: 0x0004,
      name: "ScheduledContentRatingThreshold",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(4),
    };
    type Value = String;
  }


  pub struct ScreenDailyTime;

  impl Attr for ScreenDailyTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0005,
      name: "ScreenDailyTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = TypeUnsupported /* ElapsedS */;
  }


  pub struct RemainingScreenTime;

  impl Attr for RemainingScreenTime {
    const ATTR: AttrDef = AttrDef {
      id: 0x0006,
      name: "RemainingScreenTime",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = TypeUnsupported /* ElapsedS */;
  }


  pub struct BlockUnrated;

  impl Attr for BlockUnrated {
    const ATTR: AttrDef = AttrDef {
      id: 0x0007,
      name: "BlockUnrated",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(2),
    };
    type Value = bool;
  }


  pub struct BlockChannelList;

  impl Attr for BlockChannelList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0008,
      name: "BlockChannelList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(5),
    };
    type Value = Vec<ty::BlockChannelStruct>;
  }


  pub struct BlockApplicationList;

  impl Attr for BlockApplicationList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0009,
      name: "BlockApplicationList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(6),
    };
    type Value = Vec<ty::AppInfoStruct>;
  }


  pub struct BlockContentTimeWindow;

  impl Attr for BlockContentTimeWindow {
    const ATTR: AttrDef = AttrDef {
      id: 0x000a,
      name: "BlockContentTimeWindow",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(7),
    };
    type Value = Vec<ty::TimeWindowStruct>;
  }


}
pub mod event {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct RemainingScreenTimeExpired {
}

impl Event for RemainingScreenTimeExpired {
  const EVENT: EventDef = EventDef {
    id: 0x0000,
    name: "RemainingScreenTimeExpired",
    conformance: Conformance::Feature(0),
    fields: &[
    ],
  };
}

impl TlvDecode for RemainingScreenTimeExpired {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    
    loop {
      match reader.next_ref()? {
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct EnteringBlockContentTimeWindow {
}

impl Event for EnteringBlockContentTimeWindow {
  const EVENT: EventDef = EventDef {
    id: 0x0001,
    name: "EnteringBlockContentTimeWindow",
    conformance: Conformance::Feature(7),
    fields: &[
    ],
  };
}

impl TlvDecode for EnteringBlockContentTimeWindow {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    
    loop {
      match reader.next_ref()? {
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
    })
  }
}


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct UpdatePIN {
  pub old_pin: String,
  pub new_pin: String,
}
impl Command for UpdatePIN {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "UpdatePIN",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "OldPIN", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "NewPIN", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for UpdatePIN {
  type Response = BasicAccResponse;
}
impl TlvEncode for UpdatePIN {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.old_pin.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.new_pin.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResetPIN {
}
impl Command for ResetPIN {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "ResetPIN",
    conformance: Conformance::Feature(1),
    fields: &[
    ],
  };
}
impl CommandInvoke for ResetPIN {
  type Response = ResetPINResponse;
}
impl TlvEncode for ResetPIN {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResetPINResponse {
  pub pin_code: String,
}
impl Command for ResetPINResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "ResetPINResponse",
    conformance: Conformance::Feature(1),
    fields: &[
      AttrDef { id: 0x0000, name: "PINCode", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for ResetPINResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct Enable {
}
impl Command for Enable {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "Enable",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for Enable {
  type Response = BasicAccResponse;
}
impl TlvEncode for Enable {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Disable {
}
impl Command for Disable {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "Disable",
    conformance: Conformance::Mandatory,
    fields: &[
    ],
  };
}
impl CommandInvoke for Disable {
  type Response = BasicAccResponse;
}
impl TlvEncode for Disable {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddBonusTime {
  pub pin_code: String,
  pub bonus_time: TypeUnsupported /* ElapsedS */,
}
impl Command for AddBonusTime {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "AddBonusTime",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "PINCode", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0001, name: "BonusTime", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddBonusTime {
  type Response = BasicAccResponse;
}
impl TlvEncode for AddBonusTime {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.pin_code.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.bonus_time.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetScreenDailyTime {
  pub screen_time: TypeUnsupported /* ElapsedS */,
}
impl Command for SetScreenDailyTime {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "SetScreenDailyTime",
    conformance: Conformance::Feature(0),
    fields: &[
      AttrDef { id: 0x0000, name: "ScreenTime", attr_ty: <TypeUnsupported /* ElapsedS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SetScreenDailyTime {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetScreenDailyTime {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.screen_time.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockUnratedContent {
}
impl Command for BlockUnratedContent {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "BlockUnratedContent",
    conformance: Conformance::Feature(2),
    fields: &[
    ],
  };
}
impl CommandInvoke for BlockUnratedContent {
  type Response = BasicAccResponse;
}
impl TlvEncode for BlockUnratedContent {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnblockUnratedContent {
}
impl Command for UnblockUnratedContent {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0008,
    name: "UnblockUnratedContent",
    conformance: Conformance::Feature(2),
    fields: &[
    ],
  };
}
impl CommandInvoke for UnblockUnratedContent {
  type Response = BasicAccResponse;
}
impl TlvEncode for UnblockUnratedContent {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetOnDemandRatingThreshold {
  pub rating: String,
}
impl Command for SetOnDemandRatingThreshold {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0009,
    name: "SetOnDemandRatingThreshold",
    conformance: Conformance::Feature(3),
    fields: &[
      AttrDef { id: 0x0000, name: "Rating", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SetOnDemandRatingThreshold {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetOnDemandRatingThreshold {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.rating.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetScheduledContentRatingThreshold {
  pub rating: String,
}
impl Command for SetScheduledContentRatingThreshold {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000a,
    name: "SetScheduledContentRatingThreshold",
    conformance: Conformance::Feature(4),
    fields: &[
      AttrDef { id: 0x0000, name: "Rating", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SetScheduledContentRatingThreshold {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetScheduledContentRatingThreshold {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.rating.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddBlockChannels {
  pub channels: Vec<ty::BlockChannelStruct>,
}
impl Command for AddBlockChannels {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000b,
    name: "AddBlockChannels",
    conformance: Conformance::Feature(5),
    fields: &[
      AttrDef { id: 0x0000, name: "Channels", attr_ty: <Vec<ty::BlockChannelStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddBlockChannels {
  type Response = BasicAccResponse;
}
impl TlvEncode for AddBlockChannels {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.channels.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveBlockChannels {
  pub channel_indexes: Vec<u16>,
}
impl Command for RemoveBlockChannels {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000c,
    name: "RemoveBlockChannels",
    conformance: Conformance::Feature(5),
    fields: &[
      AttrDef { id: 0x0000, name: "ChannelIndexes", attr_ty: <Vec<u16>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveBlockChannels {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveBlockChannels {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.channel_indexes.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AddBlockApplications {
  pub applications: Vec<ty::AppInfoStruct>,
}
impl Command for AddBlockApplications {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000d,
    name: "AddBlockApplications",
    conformance: Conformance::Feature(6),
    fields: &[
      AttrDef { id: 0x0000, name: "Applications", attr_ty: <Vec<ty::AppInfoStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for AddBlockApplications {
  type Response = BasicAccResponse;
}
impl TlvEncode for AddBlockApplications {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.applications.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveBlockApplications {
  pub applications: Vec<ty::AppInfoStruct>,
}
impl Command for RemoveBlockApplications {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000e,
    name: "RemoveBlockApplications",
    conformance: Conformance::Feature(6),
    fields: &[
      AttrDef { id: 0x0000, name: "Applications", attr_ty: <Vec<ty::AppInfoStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveBlockApplications {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveBlockApplications {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.applications.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetBlockContentTimeWindow {
  pub time_window: ty::TimeWindowStruct,
}
impl Command for SetBlockContentTimeWindow {
  const COMMAND: CommandDef = CommandDef {
    id: 0x000f,
    name: "SetBlockContentTimeWindow",
    conformance: Conformance::Feature(7),
    fields: &[
      AttrDef { id: 0x0000, name: "TimeWindow", attr_ty: <ty::TimeWindowStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SetBlockContentTimeWindow {
  type Response = BasicAccResponse;
}
impl TlvEncode for SetBlockContentTimeWindow {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.time_window.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RemoveBlockContentTimeWindow {
  pub time_window_indexes: Vec<u16>,
}
impl Command for RemoveBlockContentTimeWindow {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0010,
    name: "RemoveBlockContentTimeWindow",
    conformance: Conformance::Feature(7),
    fields: &[
      AttrDef { id: 0x0000, name: "TimeWindowIndexes", attr_ty: <Vec<u16>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for RemoveBlockContentTimeWindow {
  type Response = BasicAccResponse;
}
impl TlvEncode for RemoveBlockContentTimeWindow {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.time_window_indexes.tlv_put(matter_codec::Tag::Context(0x0000), writer);
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
pub enum StatusCodeEnum {
  ///Provided PIN Code does not match the current PIN code
  InvalidPINCode = 0x0002,
  ///Provided Rating is out of scope of the corresponding Rating list
  InvalidRating = 0x0003,
  ///Provided Channel(s) is invalid
  InvalidChannel = 0x0004,
  ///Provided Channel(s) already exists
  ChannelAlreadyExist = 0x0005,
  ///Provided Channel(s) doesn't exist in BlockChannelList attribute
  ChannelNotExist = 0x0006,
  ///Provided Application(s) is not identified
  UnidentifiableApplication = 0x0007,
  ///Provided Application(s) already exists
  ApplicationAlreadyExist = 0x0008,
  ///Provided Application(s) doesn't exist in BlockApplicationList attribute
  ApplicationNotExist = 0x0009,
  ///Provided time Window already exists in BlockContentTimeWindow attribute
  TimeWindowAlreadyExist = 0x000a,
  ///Provided time window doesn't exist in BlockContentTimeWindow attribute
  TimeWindowNotExist = 0x000b,
}
impl AttrType for StatusCodeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0002, "InvalidPINCode"),
  (0x0003, "InvalidRating"),
  (0x0004, "InvalidChannel"),
  (0x0005, "ChannelAlreadyExist"),
  (0x0006, "ChannelNotExist"),
  (0x0007, "UnidentifiableApplication"),
  (0x0008, "ApplicationAlreadyExist"),
  (0x0009, "ApplicationNotExist"),
  (0x000a, "TimeWindowAlreadyExist"),
  (0x000b, "TimeWindowNotExist"),
  ]);
}
impl TlvPut for StatusCodeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for StatusCodeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0002 => Ok(Self::InvalidPINCode),
      0x0003 => Ok(Self::InvalidRating),
      0x0004 => Ok(Self::InvalidChannel),
      0x0005 => Ok(Self::ChannelAlreadyExist),
      0x0006 => Ok(Self::ChannelNotExist),
      0x0007 => Ok(Self::UnidentifiableApplication),
      0x0008 => Ok(Self::ApplicationAlreadyExist),
      0x0009 => Ok(Self::ApplicationNotExist),
      0x000a => Ok(Self::TimeWindowAlreadyExist),
      0x000b => Ok(Self::TimeWindowNotExist),
      _ => Err(())
    }
  }
}
impl TlvDecode for StatusCodeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct DayOfWeekBitmap: u32 {
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
  }
}
impl AttrType for DayOfWeekBitmap {
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
impl TlvPut for DayOfWeekBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for DayOfWeekBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for DayOfWeekBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct AppInfoStruct {
  pub catalog_vendor_id: u16,
  pub application_id: String,
}
impl AttrType for AppInfoStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "CatalogVendorID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ApplicationID", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for AppInfoStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.catalog_vendor_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.application_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for AppInfoStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut catalog_vendor_id: Option<u16> = None;
    let mut application_id: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { catalog_vendor_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { application_id = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      catalog_vendor_id: catalog_vendor_id.ok_or(TlvDecodeErr::MissingField("CatalogVendorID"))?,
      application_id: application_id.ok_or(TlvDecodeErr::MissingField("ApplicationID"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockChannelStruct {
  pub block_channel_index: u16,
  pub major_number: u16,
  pub minor_number: u16,
  pub identifier: String,
}
impl AttrType for BlockChannelStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "BlockChannelIndex", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "MajorNumber", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "MinorNumber", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "Identifier", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for BlockChannelStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.block_channel_index.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.major_number.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.minor_number.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.identifier.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for BlockChannelStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut block_channel_index: Option<u16> = None;
    let mut major_number: Option<u16> = None;
    let mut minor_number: Option<u16> = None;
    let mut identifier: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { block_channel_index = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { major_number = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { minor_number = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { identifier = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      block_channel_index: block_channel_index.ok_or(TlvDecodeErr::MissingField("BlockChannelIndex"))?,
      major_number: major_number.ok_or(TlvDecodeErr::MissingField("MajorNumber"))?,
      minor_number: minor_number.ok_or(TlvDecodeErr::MissingField("MinorNumber"))?,
      identifier: identifier.ok_or(TlvDecodeErr::MissingField("Identifier"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RatingNameStruct {
  pub rating_name: String,
  pub rating_name_desc: String,
}
impl AttrType for RatingNameStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "RatingName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "RatingNameDesc", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for RatingNameStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.rating_name.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.rating_name_desc.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for RatingNameStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut rating_name: Option<String> = None;
    let mut rating_name_desc: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { rating_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { rating_name_desc = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      rating_name: rating_name.ok_or(TlvDecodeErr::MissingField("RatingName"))?,
      rating_name_desc: rating_name_desc.ok_or(TlvDecodeErr::MissingField("RatingNameDesc"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimePeriodStruct {
  pub start_hour: u8,
  pub start_minute: u8,
  pub end_hour: u8,
  pub end_minute: u8,
}
impl AttrType for TimePeriodStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "StartHour", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "StartMinute", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "EndHour", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "EndMinute", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TimePeriodStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.start_hour.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.start_minute.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.end_hour.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.end_minute.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TimePeriodStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut start_hour: Option<u8> = None;
    let mut start_minute: Option<u8> = None;
    let mut end_hour: Option<u8> = None;
    let mut end_minute: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { start_hour = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { start_minute = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { end_hour = Some(<u8>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { end_minute = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      start_hour: start_hour.ok_or(TlvDecodeErr::MissingField("StartHour"))?,
      start_minute: start_minute.ok_or(TlvDecodeErr::MissingField("StartMinute"))?,
      end_hour: end_hour.ok_or(TlvDecodeErr::MissingField("EndHour"))?,
      end_minute: end_minute.ok_or(TlvDecodeErr::MissingField("EndMinute"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TimeWindowStruct {
  pub time_window_index: u16,
  pub day_of_week: DayOfWeekBitmap,
  pub time_period: Vec<TimePeriodStruct>,
}
impl AttrType for TimeWindowStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "TimeWindowIndex", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "DayOfWeek", attr_ty: <DayOfWeekBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "TimePeriod", attr_ty: <Vec<TimePeriodStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for TimeWindowStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.time_window_index.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.day_of_week.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.time_period.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for TimeWindowStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut time_window_index: Option<u16> = None;
    let mut day_of_week: Option<DayOfWeekBitmap> = None;
    let mut time_period: Option<Vec<TimePeriodStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { time_window_index = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { day_of_week = Some(<DayOfWeekBitmap>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0002), kind: _ }) => { time_period = Some(<Vec<TimePeriodStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      time_window_index: time_window_index.ok_or(TlvDecodeErr::MissingField("TimeWindowIndex"))?,
      day_of_week: day_of_week.ok_or(TlvDecodeErr::MissingField("DayOfWeek"))?,
      time_period: time_period.ok_or(TlvDecodeErr::MissingField("TimePeriod"))?,
    })
  }
}

}
