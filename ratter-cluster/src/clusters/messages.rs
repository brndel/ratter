use crate::cluster_def::*;
pub mod attr {
  use super::*;
  // pub struct Messages;
  // type Some(List) is not supported yet


  // pub struct ActiveMessageIDs;
  // type Some(List) is not supported yet


}
pub mod ty {
  use super::*;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FutureMessagePreferenceEnum {
  ///Similar messages are allowed
  Allowed = 0x0000,
  ///Similar messages should be sent more often
  Increased = 0x0001,
  ///Similar messages should be sent less often
  Reduced = 0x0002,
  ///Similar messages should not be sent
  Disallowed = 0x0003,
  ///No further messages should be sent
  Banned = 0x0004,
}
impl AttrType for FutureMessagePreferenceEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Allowed"),
  (0x0001, "Increased"),
  (0x0002, "Reduced"),
  (0x0003, "Disallowed"),
  (0x0004, "Banned"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MessagePriorityEnum {
  ///Message to be transferred with a low level of importance
  Low = 0x0000,
  ///Message to be transferred with a medium level of importance
  Medium = 0x0001,
  ///Message to be transferred with a high level of importance
  High = 0x0002,
  ///Message to be transferred with a critical level of importance
  Critical = 0x0003,
}
impl AttrType for MessagePriorityEnum {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::enum8(&[
  (0x0000, "Low"),
  (0x0001, "Medium"),
  (0x0002, "High"),
  (0x0003, "Critical"),
  ]);
}
bitflags::bitflags! {
  #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
  pub struct MessageControlBitmap: u32 {
    ///Message requires confirmation from user
    const ConfirmationRequired = 1 << 0;
    ///Message requires response from user
    const ResponseRequired = 1 << 1;
    ///Message supports reply message from user
    const ReplyMessage = 1 << 2;
    ///Message has already been confirmed
    const MessageConfirmed = 1 << 3;
    ///Message required PIN/password protection
    const MessageProtected = 1 << 4;
  }
}
impl AttrType for MessageControlBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "ConfirmationRequired"),
    (1, "ResponseRequired"),
    (2, "ReplyMessage"),
    (3, "MessageConfirmed"),
    (4, "MessageProtected"),
  ]);
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageResponseOptionStruct {
  message_response_id: u32,
  // label: String (type currently unsupported)
}
impl AttrType for MessageResponseOptionStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "MessageResponseID", attr_ty: u32::ATTR_TYPE, conformance: Conformance::Mandatory },
  // field Label: String (type currently unsupported)
  ]});
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageStruct {
  // message_id: MessageId (type currently unsupported)
  priority: MessagePriorityEnum,
  message_control: MessageControlBitmap,
  // start_time: EpochS (type currently unsupported)
  duration: u64,
  // message_text: String (type currently unsupported)
  // responses: List (type currently unsupported)
}
impl AttrType for MessageStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
  // field MessageID: MessageId (type currently unsupported)
    AttrDef { id: 0x0001, name: "Priority", attr_ty: MessagePriorityEnum::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "MessageControl", attr_ty: MessageControlBitmap::ATTR_TYPE, conformance: Conformance::Mandatory },
  // field StartTime: EpochS (type currently unsupported)
    AttrDef { id: 0x0004, name: "Duration", attr_ty: u64::ATTR_TYPE, conformance: Conformance::Mandatory },
  // field MessageText: String (type currently unsupported)
  // field Responses: List (type currently unsupported)
  ]});
}
}
