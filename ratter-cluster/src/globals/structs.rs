use crate::cluster_def::*;
use crate::tlv::*;
use super::*;
#[derive(Debug, Clone, PartialEq)]
pub struct AtomicAttributeStatusStruct {
  pub attribute_id: TypeUnsupported /* AttributeId */,
  pub status_code: TypeUnsupported /* Status */,
}
impl AttrType for AtomicAttributeStatusStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "AttributeID", attr_ty: <TypeUnsupported /* AttributeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "StatusCode", attr_ty: <TypeUnsupported /* Status */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for AtomicAttributeStatusStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.attribute_id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.status_code.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for AtomicAttributeStatusStruct {
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
      attribute_id: TypeUnsupported,
      status_code: TypeUnsupported,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct CurrencyStruct {
  pub currency: u16,
  pub decimal_points: u8,
}
impl AttrType for CurrencyStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Currency", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "DecimalPoints", attr_ty: <u8>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for CurrencyStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.currency.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.decimal_points.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for CurrencyStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut currency: Option<u16> = None;
    let mut decimal_points: Option<u8> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { currency = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { decimal_points = Some(<u8>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      currency: currency.ok_or(TlvDecodeErr::MissingField("Currency"))?,
      decimal_points: decimal_points.ok_or(TlvDecodeErr::MissingField("DecimalPoints"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ICECandidateStruct {
  pub candidate: String,
  pub sdp_mid: String,
  pub sdpm_line_index: u16,
}
impl AttrType for ICECandidateStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Candidate", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "SDPMid", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "SDPMLineIndex", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ICECandidateStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.candidate.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.sdp_mid.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.sdpm_line_index.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ICECandidateStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut candidate: Option<String> = None;
    let mut sdp_mid: Option<String> = None;
    let mut sdpm_line_index: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { candidate = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { sdp_mid = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { sdpm_line_index = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      candidate: candidate.ok_or(TlvDecodeErr::MissingField("Candidate"))?,
      sdp_mid: sdp_mid.ok_or(TlvDecodeErr::MissingField("SDPMid"))?,
      sdpm_line_index: sdpm_line_index.ok_or(TlvDecodeErr::MissingField("SDPMLineIndex"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ICEServerStruct {
  pub ur_ls: Vec<String>,
  pub username: String,
  pub credential: String,
  pub caid: u16,
}
impl AttrType for ICEServerStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "URLs", attr_ty: <Vec<String>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Username", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Credential", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "CAID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ICEServerStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.ur_ls.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.username.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.credential.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.caid.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ICEServerStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut ur_ls: Option<Vec<String>> = None;
    let mut username: Option<String> = None;
    let mut credential: Option<String> = None;
    let mut caid: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0000), kind: _ }) => { ur_ls = Some(<Vec<String>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { username = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { credential = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { caid = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      ur_ls: ur_ls.ok_or(TlvDecodeErr::MissingField("URLs"))?,
      username: username.ok_or(TlvDecodeErr::MissingField("Username"))?,
      credential: credential.ok_or(TlvDecodeErr::MissingField("Credential"))?,
      caid: caid.ok_or(TlvDecodeErr::MissingField("CAID"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct LocationDescriptorStruct {
  pub location_name: String,
  pub floor_number: i16,
  pub area_type: TypeUnsupported /* Tag */,
}
impl AttrType for LocationDescriptorStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "LocationName", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "FloorNumber", attr_ty: <i16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "AreaType", attr_ty: <TypeUnsupported /* Tag */>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for LocationDescriptorStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.location_name.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.floor_number.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.area_type.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for LocationDescriptorStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut location_name: Option<String> = None;
    let mut floor_number: Option<i16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { location_name = Some(<String>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { floor_number = Some(<i16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      location_name: location_name.ok_or(TlvDecodeErr::MissingField("LocationName"))?,
      floor_number: floor_number.ok_or(TlvDecodeErr::MissingField("FloorNumber"))?,
      area_type: TypeUnsupported,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementAccuracyRangeStruct {
  pub range_min: i64,
  pub range_max: i64,
  pub percent_max: TypeUnsupported /* Percent100ths */,
  pub percent_min: TypeUnsupported /* Percent100ths */,
  pub percent_typical: TypeUnsupported /* Percent100ths */,
  pub fixed_max: u64,
  pub fixed_min: u64,
  pub fixed_typical: u64,
}
impl AttrType for MeasurementAccuracyRangeStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "RangeMin", attr_ty: <i64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "RangeMax", attr_ty: <i64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "PercentMax", attr_ty: <TypeUnsupported /* Percent100ths */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "PercentMin", attr_ty: <TypeUnsupported /* Percent100ths */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "PercentTypical", attr_ty: <TypeUnsupported /* Percent100ths */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "FixedMax", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "FixedMin", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "FixedTypical", attr_ty: <u64>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for MeasurementAccuracyRangeStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.range_min.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.range_max.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.percent_max.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.percent_min.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.percent_typical.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.fixed_max.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.fixed_min.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.fixed_typical.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for MeasurementAccuracyRangeStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut range_min: Option<i64> = None;
    let mut range_max: Option<i64> = None;
    let mut fixed_max: Option<u64> = None;
    let mut fixed_min: Option<u64> = None;
    let mut fixed_typical: Option<u64> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { range_min = Some(<i64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { range_max = Some(<i64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { fixed_max = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { fixed_min = Some(<u64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0007), value: _ }) => { fixed_typical = Some(<u64>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      range_min: range_min.ok_or(TlvDecodeErr::MissingField("RangeMin"))?,
      range_max: range_max.ok_or(TlvDecodeErr::MissingField("RangeMax"))?,
      percent_max: TypeUnsupported,
      percent_min: TypeUnsupported,
      percent_typical: TypeUnsupported,
      fixed_max: fixed_max.ok_or(TlvDecodeErr::MissingField("FixedMax"))?,
      fixed_min: fixed_min.ok_or(TlvDecodeErr::MissingField("FixedMin"))?,
      fixed_typical: fixed_typical.ok_or(TlvDecodeErr::MissingField("FixedTypical"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct MeasurementAccuracyStruct {
  pub measurement_type: MeasurementTypeEnum,
  pub measured: bool,
  pub min_measured_value: i64,
  pub max_measured_value: i64,
  pub accuracy_ranges: Vec<MeasurementAccuracyRangeStruct>,
}
impl AttrType for MeasurementAccuracyStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "MeasurementType", attr_ty: <MeasurementTypeEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Measured", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "MinMeasuredValue", attr_ty: <i64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "MaxMeasuredValue", attr_ty: <i64>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "AccuracyRanges", attr_ty: <Vec<MeasurementAccuracyRangeStruct>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for MeasurementAccuracyStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.measurement_type.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.measured.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.min_measured_value.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.max_measured_value.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.accuracy_ranges.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for MeasurementAccuracyStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut measurement_type: Option<MeasurementTypeEnum> = None;
    let mut measured: Option<bool> = None;
    let mut min_measured_value: Option<i64> = None;
    let mut max_measured_value: Option<i64> = None;
    let mut accuracy_ranges: Option<Vec<MeasurementAccuracyRangeStruct>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { measurement_type = Some(<MeasurementTypeEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { measured = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { min_measured_value = Some(<i64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { max_measured_value = Some(<i64>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0004), kind: _ }) => { accuracy_ranges = Some(<Vec<MeasurementAccuracyRangeStruct>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      measurement_type: measurement_type.ok_or(TlvDecodeErr::MissingField("MeasurementType"))?,
      measured: measured.ok_or(TlvDecodeErr::MissingField("Measured"))?,
      min_measured_value: min_measured_value.ok_or(TlvDecodeErr::MissingField("MinMeasuredValue"))?,
      max_measured_value: max_measured_value.ok_or(TlvDecodeErr::MissingField("MaxMeasuredValue"))?,
      accuracy_ranges: accuracy_ranges.ok_or(TlvDecodeErr::MissingField("AccuracyRanges"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct PowerThresholdStruct {
  pub power_threshold: PowerMw,
  pub apparent_power_threshold: TypeUnsupported /* PowerMVA */,
  pub power_threshold_source: PowerThresholdSourceEnum,
}
impl AttrType for PowerThresholdStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "PowerThreshold", attr_ty: <PowerMw>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "ApparentPowerThreshold", attr_ty: <TypeUnsupported /* PowerMVA */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "PowerThresholdSource", attr_ty: <PowerThresholdSourceEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for PowerThresholdStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.power_threshold.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.apparent_power_threshold.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.power_threshold_source.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for PowerThresholdStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut power_threshold: Option<PowerMw> = None;
    let mut power_threshold_source: Option<PowerThresholdSourceEnum> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { power_threshold = Some(<PowerMw>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { power_threshold_source = Some(<PowerThresholdSourceEnum>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      power_threshold: power_threshold.ok_or(TlvDecodeErr::MissingField("PowerThreshold"))?,
      apparent_power_threshold: TypeUnsupported,
      power_threshold_source: power_threshold_source.ok_or(TlvDecodeErr::MissingField("PowerThresholdSource"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct PriceStruct {
  pub amount: TypeUnsupported /* Money */,
  pub currency: CurrencyStruct,
}
impl AttrType for PriceStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "Amount", attr_ty: <TypeUnsupported /* Money */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Currency", attr_ty: <CurrencyStruct>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for PriceStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.amount.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.currency.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for PriceStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut currency: Option<CurrencyStruct> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0001), kind: _ }) => { currency = Some(<CurrencyStruct>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      amount: TypeUnsupported,
      currency: currency.ok_or(TlvDecodeErr::MissingField("Currency"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct SemanticTagStruct {
  pub mfg_code: TypeUnsupported /* VendorId */,
  pub namespace_id: TypeUnsupported /* Namespace */,
  pub tag: TypeUnsupported /* Tag */,
  pub label: String,
}
impl AttrType for SemanticTagStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "MfgCode", attr_ty: <TypeUnsupported /* VendorId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "NamespaceID", attr_ty: <TypeUnsupported /* Namespace */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "Tag", attr_ty: <TypeUnsupported /* Tag */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "Label", attr_ty: <String>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for SemanticTagStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.mfg_code.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.namespace_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.tag.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.label.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for SemanticTagStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut label: Option<String> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { label = Some(<String>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      mfg_code: TypeUnsupported,
      namespace_id: TypeUnsupported,
      tag: TypeUnsupported,
      label: label.ok_or(TlvDecodeErr::MissingField("Label"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct ViewportStruct {
  pub x1: u16,
  pub y1: u16,
  pub x2: u16,
  pub y2: u16,
}
impl AttrType for ViewportStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "X1", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "Y1", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "X2", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "Y2", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for ViewportStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.x1.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.y1.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.x2.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.y2.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for ViewportStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut x1: Option<u16> = None;
    let mut y1: Option<u16> = None;
    let mut x2: Option<u16> = None;
    let mut y2: Option<u16> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { x1 = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0001), value: _ }) => { y1 = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0002), value: _ }) => { x2 = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { y2 = Some(<u16>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      x1: x1.ok_or(TlvDecodeErr::MissingField("X1"))?,
      y1: y1.ok_or(TlvDecodeErr::MissingField("Y1"))?,
      x2: x2.ok_or(TlvDecodeErr::MissingField("X2"))?,
      y2: y2.ok_or(TlvDecodeErr::MissingField("Y2"))?,
    })
  }
}


#[derive(Debug, Clone, PartialEq)]
pub struct WebRTCSessionStruct {
  pub id: u16,
  pub peer_node_id: TypeUnsupported /* NodeId */,
  pub peer_endpoint_id: TypeUnsupported /* EndpointNo */,
  pub stream_usage: StreamUsageEnum,
  pub video_stream_id: u16,
  pub audio_stream_id: u16,
  pub metadata_enabled: bool,
  pub video_streams: Vec<u16>,
  pub audio_streams: Vec<u16>,
}
impl AttrType for WebRTCSessionStruct {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::new(AttrValueType::Struct { fields: &[
    AttrDef { id: 0x0000, name: "ID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0001, name: "PeerNodeID", attr_ty: <TypeUnsupported /* NodeId */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0002, name: "PeerEndpointID", attr_ty: <TypeUnsupported /* EndpointNo */>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0003, name: "StreamUsage", attr_ty: <StreamUsageEnum>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0004, name: "VideoStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0005, name: "AudioStreamID", attr_ty: <u16>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0006, name: "MetadataEnabled", attr_ty: <bool>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0007, name: "VideoStreams", attr_ty: <Vec<u16>>::ATTR_TYPE, conformance: Conformance::Mandatory },
    AttrDef { id: 0x0008, name: "AudioStreams", attr_ty: <Vec<u16>>::ATTR_TYPE, conformance: Conformance::Mandatory },
  ]});
}
impl TlvPut for WebRTCSessionStruct {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.start_structure(tag).unwrap();
    self.id.tlv_put(matter_codec::Tag::Context(0x0000), writer);
    self.peer_node_id.tlv_put(matter_codec::Tag::Context(0x0001), writer);
    self.peer_endpoint_id.tlv_put(matter_codec::Tag::Context(0x0002), writer);
    self.stream_usage.tlv_put(matter_codec::Tag::Context(0x0003), writer);
    self.video_stream_id.tlv_put(matter_codec::Tag::Context(0x0004), writer);
    self.audio_stream_id.tlv_put(matter_codec::Tag::Context(0x0005), writer);
    self.metadata_enabled.tlv_put(matter_codec::Tag::Context(0x0006), writer);
    self.video_streams.tlv_put(matter_codec::Tag::Context(0x0007), writer);
    self.audio_streams.tlv_put(matter_codec::Tag::Context(0x0008), writer);
    writer.end_container().unwrap();
  }
}
impl TlvDecode for WebRTCSessionStruct {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
    if !matches!(element, matter_codec::ElementRef::ContainerStart { kind: matter_codec::ContainerKind::Structure, .. }) { return Err(TlvDecodeErr::WrongType); }
    let mut id: Option<u16> = None;
    let mut stream_usage: Option<StreamUsageEnum> = None;
    let mut video_stream_id: Option<u16> = None;
    let mut audio_stream_id: Option<u16> = None;
    let mut metadata_enabled: Option<bool> = None;
    let mut video_streams: Option<Vec<u16>> = None;
    let mut audio_streams: Option<Vec<u16>> = None;
    
    loop {
      match reader.next_ref()? {
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0000), value: _ }) => { id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0003), value: _ }) => { stream_usage = Some(<StreamUsageEnum>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0004), value: _ }) => { video_stream_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0005), value: _ }) => { audio_stream_id = Some(<u16>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::Scalar { tag: matter_codec::Tag::Context(0x0006), value: _ }) => { metadata_enabled = Some(<bool>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0007), kind: _ }) => { video_streams = Some(<Vec<u16>>::tlv_decode(v, reader)?); },
        Some(v @ matter_codec::ElementRef::ContainerStart { tag: matter_codec::Tag::Context(0x0008), kind: _ }) => { audio_streams = Some(<Vec<u16>>::tlv_decode(v, reader)?); },
        Some(matter_codec::ElementRef::Scalar { .. }) => (),
        Some(matter_codec::ElementRef::ContainerStart { .. }) => reader.skip_container()?,
        Some(matter_codec::ElementRef::ContainerEnd) => break,
        Some(_) => (),
        None => return Err(TlvDecodeErr::UnclosedContainer),
      }
    }
    Ok(Self {
      id: id.ok_or(TlvDecodeErr::MissingField("ID"))?,
      peer_node_id: TypeUnsupported,
      peer_endpoint_id: TypeUnsupported,
      stream_usage: stream_usage.ok_or(TlvDecodeErr::MissingField("StreamUsage"))?,
      video_stream_id: video_stream_id.ok_or(TlvDecodeErr::MissingField("VideoStreamID"))?,
      audio_stream_id: audio_stream_id.ok_or(TlvDecodeErr::MissingField("AudioStreamID"))?,
      metadata_enabled: metadata_enabled.ok_or(TlvDecodeErr::MissingField("MetadataEnabled"))?,
      video_streams: video_streams.ok_or(TlvDecodeErr::MissingField("VideoStreams"))?,
      audio_streams: audio_streams.ok_or(TlvDecodeErr::MissingField("AudioStreams"))?,
    })
  }
}


