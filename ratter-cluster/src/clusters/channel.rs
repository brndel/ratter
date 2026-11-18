#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const CHANNEL: ClusterDef = ClusterDef {
  id: 0x0504,
  name: "Channel",
  features: &[
    FeatureDef { bit: 0x0000, code: "CL", name: "ChannelList", summary: "Provides list of available channels" },
    FeatureDef { bit: 0x0001, code: "LI", name: "LineupInfo", summary: "Provides lineup info, which is a reference to an external source of lineup information" },
    FeatureDef { bit: 0x0002, code: "EG", name: "ElectronicGuide", summary: "Provides electronic program guide information" },
    FeatureDef { bit: 0x0003, code: "RP", name: "RecordProgram", summary: "Provides ability to record program" },
  ],
  attributes: &[
    attr::ChannelList::ATTR,
    attr::Lineup::ATTR,
    attr::CurrentChannel::ATTR,
  ],
  events: &[
  ],
  commands: &[
    command::ChangeChannel::COMMAND,
    command::ChangeChannelByNumber::COMMAND,
    command::SkipChannel::COMMAND,
    command::GetProgramGuide::COMMAND,
    command::RecordProgram::COMMAND,
    command::CancelRecordProgram::COMMAND,
  ],
};
pub mod attr {
  use super::*;
  pub struct ChannelList;

  impl Attr for ChannelList {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "ChannelList",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(0),
    };
    type Value = Vec<ty::ChannelInfoStruct>;
  }


  pub struct Lineup;

  impl Attr for Lineup {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "Lineup",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Feature(1),
    };
    type Value = Nullable<ty::LineupInfoStruct>;
  }


  pub struct CurrentChannel;

  impl Attr for CurrentChannel {
    const ATTR: AttrDef = AttrDef {
      id: 0x0002,
      name: "CurrentChannel",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Optional,
    };
    type Value = Nullable<ty::ChannelInfoStruct>;
  }


}
pub mod command {
  use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct ChangeChannel {
  pub r#match: String,
}
impl Command for ChangeChannel {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0000,
    name: "ChangeChannel",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Match", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ChangeChannel {
  type Response = ChangeChannelResponse;
}
impl TlvEncode for ChangeChannel {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.r#match.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChangeChannelResponse {
  pub status: ty::StatusEnum,
  pub data: String,
}
impl Command for ChangeChannelResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0001,
    name: "ChangeChannelResponse",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Status", attr_ty: <ty::StatusEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "Data", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandResponse for ChangeChannelResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct ChangeChannelByNumber {
  pub major_number: u16,
  pub minor_number: u16,
}
impl Command for ChangeChannelByNumber {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0002,
    name: "ChangeChannelByNumber",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "MajorNumber", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "MinorNumber", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for ChangeChannelByNumber {
  type Response = BasicAccResponse;
}
impl TlvEncode for ChangeChannelByNumber {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.major_number.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.minor_number.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SkipChannel {
  pub count: i16,
}
impl Command for SkipChannel {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0003,
    name: "SkipChannel",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "Count", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandInvoke for SkipChannel {
  type Response = BasicAccResponse;
}
impl TlvEncode for SkipChannel {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.count.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetProgramGuide {
  pub start_time: TypeUnsupported /* EpochS */,
  pub end_time: TypeUnsupported /* EpochS */,
  pub channel_list: Vec<ty::ChannelInfoStruct>,
  pub page_token: ty::PageTokenStruct,
  pub recording_flag: ty::RecordingFlagBitmap,
  pub external_id_list: Vec<ty::AdditionalInfoStruct>,
  pub data: TypeUnsupported /* Octstr */,
}
impl Command for GetProgramGuide {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0004,
    name: "GetProgramGuide",
    conformance: Conformance::Feature(2),
    fields: &[
      AttrDef { id: 0x0000, name: "StartTime", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "EndTime", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "ChannelList", attr_ty: <Vec<ty::ChannelInfoStruct>>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0003, name: "PageToken", attr_ty: <ty::PageTokenStruct>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0005, name: "RecordingFlag", attr_ty: <ty::RecordingFlagBitmap>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0006, name: "ExternalIDList", attr_ty: <Vec<ty::AdditionalInfoStruct>>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0007, name: "Data", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for GetProgramGuide {
  type Response = ProgramGuideResponse;
}
impl TlvEncode for GetProgramGuide {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.start_time.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.end_time.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.channel_list.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.page_token.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.recording_flag.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.external_id_list.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.data.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramGuideResponse {
  pub paging: ty::ChannelPagingStruct,
  pub program_list: Vec<ty::ProgramStruct>,
}
impl Command for ProgramGuideResponse {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0005,
    name: "ProgramGuideResponse",
    conformance: Conformance::Feature(2),
    fields: &[
      AttrDef { id: 0x0000, name: "Paging", attr_ty: <ty::ChannelPagingStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ProgramList", attr_ty: <Vec<ty::ProgramStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    ],
  };
}
impl CommandResponse for ProgramGuideResponse {}

#[derive(Debug, Clone, PartialEq)]
pub struct RecordProgram {
  pub program_identifier: String,
  pub should_record_series: bool,
  pub external_id_list: Vec<ty::AdditionalInfoStruct>,
  pub data: TypeUnsupported /* Octstr */,
}
impl Command for RecordProgram {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0006,
    name: "RecordProgram",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ProgramIdentifier", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ShouldRecordSeries", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "ExternalIDList", attr_ty: <Vec<ty::AdditionalInfoStruct>>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0003, name: "Data", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for RecordProgram {
  type Response = BasicAccResponse;
}
impl TlvEncode for RecordProgram {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.program_identifier.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.should_record_series.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.external_id_list.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.data.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CancelRecordProgram {
  pub program_identifier: String,
  pub should_record_series: bool,
  pub external_id_list: Vec<ty::AdditionalInfoStruct>,
  pub data: TypeUnsupported /* Octstr */,
}
impl Command for CancelRecordProgram {
  const COMMAND: CommandDef = CommandDef {
    id: 0x0007,
    name: "CancelRecordProgram",
    conformance: Conformance::Mandatory,
    fields: &[
      AttrDef { id: 0x0000, name: "ProgramIdentifier", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0001, name: "ShouldRecordSeries", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
      AttrDef { id: 0x0002, name: "ExternalIDList", attr_ty: <Vec<ty::AdditionalInfoStruct>>::ATTR_TYPE, conformance: Conformance::Optional },
      AttrDef { id: 0x0003, name: "Data", attr_ty: <TypeUnsupported /* Octstr */>::ATTR_TYPE, conformance: Conformance::Optional },
    ],
  };
}
impl CommandInvoke for CancelRecordProgram {
  type Response = BasicAccResponse;
}
impl TlvEncode for CancelRecordProgram {
  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();
    self.program_identifier.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.should_record_series.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.external_id_list.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.data.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}

}
pub mod ty {
  pub use crate::globals::*;
  pub use crate::clusters::content_launcher::ty::*;
  use super::*;
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum ChannelTypeEnum {
  ///The channel is sourced from a satellite provider.
  Satellite = 0x0000,
  ///The channel is sourced from a cable provider.
  Cable = 0x0001,
  ///The channel is sourced from a terrestrial provider.
  Terrestrial = 0x0002,
  ///The channel is sourced from an OTT provider.
  OTT = 0x0003,
}
impl AttrType for ChannelTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Satellite"),
  (0x0001, "Cable"),
  (0x0002, "Terrestrial"),
  (0x0003, "OTT"),
  ]);
}
impl TlvPut for ChannelTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for ChannelTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::Satellite),
      0x0001 => Ok(Self::Cable),
      0x0002 => Ok(Self::Terrestrial),
      0x0003 => Ok(Self::OTT),
      _ => Err(())
    }
  }
}
impl TlvDecode for ChannelTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum LineupInfoTypeEnum {
  ///Multi System Operator
  MSO = 0x0000,
}
impl AttrType for LineupInfoTypeEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "MSO"),
  ]);
}
impl TlvPut for LineupInfoTypeEnum {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, *self as u64).unwrap();
  }
}
impl TryFrom<u64> for LineupInfoTypeEnum {
  type Error = ();
  fn try_from(value: u64) -> Result<Self, ()> {
    match value {
      0x0000 => Ok(Self::MSO),
      _ => Err(())
    }
  }
}
impl TlvDecode for LineupInfoTypeEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
#[derive(Debug, Clone, PartialEq)]
#[derive(Copy)]
#[allow(non_camel_case_types)]
pub enum StatusEnum {
  ///Command succeeded
  Success = 0x0000,
  ///More than one equal match for the ChannelInfoStruct passed in.
  MultipleMatches = 0x0001,
  ///No matches for the ChannelInfoStruct passed in.
  NoMatches = 0x0002,
}
impl AttrType for StatusEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Success"),
  (0x0001, "MultipleMatches"),
  (0x0002, "NoMatches"),
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
      0x0001 => Ok(Self::MultipleMatches),
      0x0002 => Ok(Self::NoMatches),
      _ => Err(())
    }
  }
}
impl TlvDecode for StatusEnum {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Self::try_from(u64::tlv_decode(element, reader)?).map_err(|()| TlvDecodeErr::UnkownEnumVariant)
  }
}
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct RecordingFlagBitmap: u32 {
    ///The program is scheduled for recording.
    const Scheduled = 1 << 0;
    ///The program series is scheduled for recording.
    const RecordSeries = 1 << 1;
    ///The program is recorded and available to be played.
    const Recorded = 1 << 2;
  }
}
impl AttrType for RecordingFlagBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "Scheduled"),
    (1, "RecordSeries"),
    (2, "Recorded"),
  ]);
}
impl TlvPut for RecordingFlagBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for RecordingFlagBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for RecordingFlagBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ChannelInfoStruct {
  pub major_number: u16,
  pub minor_number: u16,
  pub name: String,
  pub call_sign: String,
  pub affiliate_call_sign: String,
  pub identifier: String,
  pub r#type: ChannelTypeEnum,
}
impl AttrType for ChannelInfoStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "MajorNumber", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "MinorNumber", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "CallSign", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "AffiliateCallSign", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "Identifier", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "Type", attr_ty: <ChannelTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ChannelInfoStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.major_number.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.minor_number.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.name.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.call_sign.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.affiliate_call_sign.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.identifier.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.r#type.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ChannelInfoStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut major_number: Option<u16> = None;
    let mut minor_number: Option<u16> = None;
    let mut name: Option<String> = None;
    let mut call_sign: Option<String> = None;
    let mut affiliate_call_sign: Option<String> = None;
    let mut identifier: Option<String> = None;
    let mut r#type: Option<ChannelTypeEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { major_number = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { minor_number = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { call_sign = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { affiliate_call_sign = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { identifier = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { r#type = Some(<ChannelTypeEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      major_number: major_number.ok_or(TlvDecodeErr::MissingField("MajorNumber"))?,
      minor_number: minor_number.ok_or(TlvDecodeErr::MissingField("MinorNumber"))?,
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
      call_sign: call_sign.ok_or(TlvDecodeErr::MissingField("CallSign"))?,
      affiliate_call_sign: affiliate_call_sign.ok_or(TlvDecodeErr::MissingField("AffiliateCallSign"))?,
      identifier: identifier.ok_or(TlvDecodeErr::MissingField("Identifier"))?,
      r#type: r#type.ok_or(TlvDecodeErr::MissingField("Type"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelPagingStruct {
  pub previous_token: PageTokenStruct,
  pub next_token: PageTokenStruct,
}
impl AttrType for ChannelPagingStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "PreviousToken", attr_ty: <PageTokenStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "NextToken", attr_ty: <PageTokenStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ChannelPagingStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.previous_token.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.next_token.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ChannelPagingStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut previous_token: Option<PageTokenStruct> = None;
    let mut next_token: Option<PageTokenStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { previous_token = Some(<PageTokenStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { next_token = Some(<PageTokenStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      previous_token: previous_token.ok_or(TlvDecodeErr::MissingField("PreviousToken"))?,
      next_token: next_token.ok_or(TlvDecodeErr::MissingField("NextToken"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LineupInfoStruct {
  pub operator_name: String,
  pub lineup_name: String,
  pub postal_code: String,
  pub lineup_info_type: LineupInfoTypeEnum,
}
impl AttrType for LineupInfoStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "OperatorName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "LineupName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "PostalCode", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "LineupInfoType", attr_ty: <LineupInfoTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for LineupInfoStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.operator_name.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.lineup_name.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.postal_code.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.lineup_info_type.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for LineupInfoStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut operator_name: Option<String> = None;
    let mut lineup_name: Option<String> = None;
    let mut postal_code: Option<String> = None;
    let mut lineup_info_type: Option<LineupInfoTypeEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { operator_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { lineup_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { postal_code = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { lineup_info_type = Some(<LineupInfoTypeEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      operator_name: operator_name.ok_or(TlvDecodeErr::MissingField("OperatorName"))?,
      lineup_name: lineup_name.ok_or(TlvDecodeErr::MissingField("LineupName"))?,
      postal_code: postal_code.ok_or(TlvDecodeErr::MissingField("PostalCode"))?,
      lineup_info_type: lineup_info_type.ok_or(TlvDecodeErr::MissingField("LineupInfoType"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct PageTokenStruct {
  pub limit: u16,
  pub after: String,
  pub before: String,
}
impl AttrType for PageTokenStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Limit", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "After", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Before", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for PageTokenStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.limit.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.after.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.before.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for PageTokenStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut limit: Option<u16> = None;
    let mut after: Option<String> = None;
    let mut before: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { limit = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { after = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { before = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      limit: limit.ok_or(TlvDecodeErr::MissingField("Limit"))?,
      after: after.ok_or(TlvDecodeErr::MissingField("After"))?,
      before: before.ok_or(TlvDecodeErr::MissingField("Before"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramCastStruct {
  pub name: String,
  pub role: String,
}
impl AttrType for ProgramCastStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Name", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Role", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ProgramCastStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.name.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.role.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ProgramCastStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut name: Option<String> = None;
    let mut role: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { role = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      name: name.ok_or(TlvDecodeErr::MissingField("Name"))?,
      role: role.ok_or(TlvDecodeErr::MissingField("Role"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramCategoryStruct {
  pub category: String,
  pub sub_category: String,
}
impl AttrType for ProgramCategoryStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Category", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "SubCategory", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ProgramCategoryStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.category.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.sub_category.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ProgramCategoryStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut category: Option<String> = None;
    let mut sub_category: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { category = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { sub_category = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      category: category.ok_or(TlvDecodeErr::MissingField("Category"))?,
      sub_category: sub_category.ok_or(TlvDecodeErr::MissingField("SubCategory"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgramStruct {
  pub identifier: String,
  pub channel: ChannelInfoStruct,
  pub start_time: TypeUnsupported /* EpochS */,
  pub end_time: TypeUnsupported /* EpochS */,
  pub title: String,
  pub subtitle: String,
  pub description: String,
  pub audio_languages: Vec<String>,
  pub ratings: Vec<String>,
  pub thumbnail_url: String,
  pub poster_art_url: String,
  pub dvbi_url: String,
  pub release_date: String,
  pub parental_guidance_text: String,
  pub recording_flag: RecordingFlagBitmap,
  pub series_info: SeriesInfoStruct,
  pub category_list: Vec<ProgramCategoryStruct>,
  pub cast_list: Vec<ProgramCastStruct>,
  pub external_id_list: Vec<AdditionalInfoStruct>,
}
impl AttrType for ProgramStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Identifier", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Channel", attr_ty: <ChannelInfoStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "StartTime", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "EndTime", attr_ty: <TypeUnsupported /* EpochS */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "Title", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "Subtitle", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "Description", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "AudioLanguages", attr_ty: <Vec<String>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0008, name: "Ratings", attr_ty: <Vec<String>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0009, name: "ThumbnailUrl", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000a, name: "PosterArtUrl", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000b, name: "DvbiUrl", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000c, name: "ReleaseDate", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000d, name: "ParentalGuidanceText", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000e, name: "RecordingFlag", attr_ty: <RecordingFlagBitmap>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x000f, name: "SeriesInfo", attr_ty: <SeriesInfoStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0010, name: "CategoryList", attr_ty: <Vec<ProgramCategoryStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0011, name: "CastList", attr_ty: <Vec<ProgramCastStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0012, name: "ExternalIDList", attr_ty: <Vec<AdditionalInfoStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ProgramStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.identifier.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.channel.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.start_time.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.end_time.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.title.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.subtitle.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.description.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.audio_languages.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.ratings.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    self.thumbnail_url.tlv_put(matter_codec::Tag::Context(0x0009), writer);
    self.poster_art_url.tlv_put(matter_codec::Tag::Context(0x000a), writer);
    self.dvbi_url.tlv_put(matter_codec::Tag::Context(0x000b), writer);
    self.release_date.tlv_put(matter_codec::Tag::Context(0x000c), writer);
    self.parental_guidance_text.tlv_put(matter_codec::Tag::Context(0x000d), writer);
    self.recording_flag.tlv_put(matter_codec::Tag::Context(0x000e), writer);
    self.series_info.tlv_put(matter_codec::Tag::Context(0x000f), writer);
    self.category_list.tlv_put(matter_codec::Tag::Context(0x0010), writer);
    self.cast_list.tlv_put(matter_codec::Tag::Context(0x0011), writer);
    self.external_id_list.tlv_put(matter_codec::Tag::Context(0x0012), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ProgramStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut identifier: Option<String> = None;
    let mut channel: Option<ChannelInfoStruct> = None;
    let mut title: Option<String> = None;
    let mut subtitle: Option<String> = None;
    let mut description: Option<String> = None;
    let mut audio_languages: Option<Vec<String>> = None;
    let mut ratings: Option<Vec<String>> = None;
    let mut thumbnail_url: Option<String> = None;
    let mut poster_art_url: Option<String> = None;
    let mut dvbi_url: Option<String> = None;
    let mut release_date: Option<String> = None;
    let mut parental_guidance_text: Option<String> = None;
    let mut recording_flag: Option<RecordingFlagBitmap> = None;
    let mut series_info: Option<SeriesInfoStruct> = None;
    let mut category_list: Option<Vec<ProgramCategoryStruct>> = None;
    let mut cast_list: Option<Vec<ProgramCastStruct>> = None;
    let mut external_id_list: Option<Vec<AdditionalInfoStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { identifier = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { channel = Some(<ChannelInfoStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { title = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { subtitle = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { description = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0007), kind: _ }) => { audio_languages = Some(<Vec<String>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0008), kind: _ }) => { ratings = Some(<Vec<String>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0009), value: _ }) => { thumbnail_url = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000a), value: _ }) => { poster_art_url = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000b), value: _ }) => { dvbi_url = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000c), value: _ }) => { release_date = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000d), value: _ }) => { parental_guidance_text = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x000e), value: _ }) => { recording_flag = Some(<RecordingFlagBitmap>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x000f), kind: _ }) => { series_info = Some(<SeriesInfoStruct>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0010), kind: _ }) => { category_list = Some(<Vec<ProgramCategoryStruct>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0011), kind: _ }) => { cast_list = Some(<Vec<ProgramCastStruct>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0012), kind: _ }) => { external_id_list = Some(<Vec<AdditionalInfoStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      identifier: identifier.ok_or(TlvDecodeErr::MissingField("Identifier"))?,
      channel: channel.ok_or(TlvDecodeErr::MissingField("Channel"))?,
      start_time: TypeUnsupported,
      end_time: TypeUnsupported,
      title: title.ok_or(TlvDecodeErr::MissingField("Title"))?,
      subtitle: subtitle.ok_or(TlvDecodeErr::MissingField("Subtitle"))?,
      description: description.ok_or(TlvDecodeErr::MissingField("Description"))?,
      audio_languages: audio_languages.ok_or(TlvDecodeErr::MissingField("AudioLanguages"))?,
      ratings: ratings.ok_or(TlvDecodeErr::MissingField("Ratings"))?,
      thumbnail_url: thumbnail_url.ok_or(TlvDecodeErr::MissingField("ThumbnailUrl"))?,
      poster_art_url: poster_art_url.ok_or(TlvDecodeErr::MissingField("PosterArtUrl"))?,
      dvbi_url: dvbi_url.ok_or(TlvDecodeErr::MissingField("DvbiUrl"))?,
      release_date: release_date.ok_or(TlvDecodeErr::MissingField("ReleaseDate"))?,
      parental_guidance_text: parental_guidance_text.ok_or(TlvDecodeErr::MissingField("ParentalGuidanceText"))?,
      recording_flag: recording_flag.ok_or(TlvDecodeErr::MissingField("RecordingFlag"))?,
      series_info: series_info.ok_or(TlvDecodeErr::MissingField("SeriesInfo"))?,
      category_list: category_list.ok_or(TlvDecodeErr::MissingField("CategoryList"))?,
      cast_list: cast_list.ok_or(TlvDecodeErr::MissingField("CastList"))?,
      external_id_list: external_id_list.ok_or(TlvDecodeErr::MissingField("ExternalIDList"))?,
    })
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SeriesInfoStruct {
  pub season: String,
  pub episode: String,
}
impl AttrType for SeriesInfoStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Season", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Episode", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for SeriesInfoStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.season.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.episode.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for SeriesInfoStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut season: Option<String> = None;
    let mut episode: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { season = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { episode = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      season: season.ok_or(TlvDecodeErr::MissingField("Season"))?,
      episode: episode.ok_or(TlvDecodeErr::MissingField("Episode"))?,
    })
  }
}

}
