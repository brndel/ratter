use std::fmt::Display;

use nest_struct::nest_struct;
use serde::Deserialize;

use crate::cluster_file::{
    DEFAULT_DERIVE, Ident, IntValue,
    conformance::{Conformance, ConformanceFields, FeatureList},
    field::{Field, FieldEntry},
};

#[nest_struct]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventEntry {
    #[serde(rename = "@id")]
    id: IntValue,
    #[serde(rename = "@name")]
    pub name: Ident,
    #[serde(default)]
    #[serde(rename = "field")]
    fields: Vec<FieldEntry>,
    #[serde(flatten)]
    conformance: ConformanceFields,
}

#[nest_struct]
pub struct Event<'a> {
    id: IntValue,
    name: Ident,
    fields: Vec<Field<'a>>,
    conformance: Conformance,
}

impl EventEntry {
    pub fn entry(&self, features: FeatureList) -> Event<'_> {
        Event {
            id: self.id,
            name: self.name.clone(),
            fields: self
                .fields
                .iter()
                .map(|field| field.entry(features))
                .collect(),
            conformance: self.conformance.conformance(features),
        }
    }
}

impl Display for Event<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", DEFAULT_DERIVE)?;
        writeln!(f, "pub struct {} {{", self.name)?;
        for field in &self.fields {
            writeln!(f, "  pub {}: {},", field.name.snake_case(), field.ty)?;
        }
        writeln!(f, "}}")?;
        writeln!(f, "")?;
        writeln!(f, "impl Event for {} {{", self.name)?;
        writeln!(f, "  const EVENT: EventDef = EventDef {{")?;
        writeln!(f, "    id: {},", self.id)?;
        writeln!(f, "    name: {},", self.name.string())?;
        writeln!(f, "    conformance: {},", self.conformance)?;
        writeln!(f, "    fields: &[")?;
        for field in &self.fields {
            writeln!(f, "      {field},")?;
        }
        writeln!(f, "    ],")?;
        writeln!(f, "  }};")?;
        writeln!(f, "}}")?;
        writeln!(f, "")?;
        writeln!(f, "{}", ImplTlvDecodeContainer { name: &self.name, fields: &self.fields })?;

        Ok(())
    }
}

pub struct ImplTlvDecodeContainer<'a> {
    pub name: &'a Ident,
    pub fields: &'a [Field<'a>]
}

impl Display for ImplTlvDecodeContainer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "impl TlvDecode for {} {{", self.name)?;
        writeln!(f, "  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>) -> Result<Self, TlvDecodeErr> {{")?;
        writeln!(f, "    if !matches!(element, matter_codec::ElementRef::ContainerStart {{ kind: matter_codec::ContainerKind::Structure, .. }}) {{ return Err(TlvDecodeErr::WrongType); }}")?;
        for field in self.fields {
            if field.ty.is_supported() {
                writeln!(f, "    let mut {}: Option<{}> = None;", field.name.snake_case(), field.ty)?;
            }
        }
        writeln!(f, "    ")?;
        writeln!(f, "    loop {{")?;
        writeln!(f, "      match reader.next_ref()? {{")?;
        for field in self.fields {
            if field.ty.is_supported() {
                if field.ty.is_scalar() {
                    writeln!(f, "        Some(v @ matter_codec::ElementRef::Scalar {{ tag: matter_codec::Tag::Context({}), value: _ }}) => {{ {} = Some(<{}>::tlv_decode(v, reader)?); }},", field.id, field.name.snake_case(), field.ty)?;
                } else {
                    writeln!(f, "        Some(v @ matter_codec::ElementRef::ContainerStart {{ tag: matter_codec::Tag::Context({}), kind: _ }}) => {{ {} = Some(<{}>::tlv_decode(v, reader)?); }},", field.id, field.name.snake_case(), field.ty)?;
                }
            }
        }
        writeln!(f, "        Some(matter_codec::ElementRef::Scalar {{ .. }}) => (),")?;
        writeln!(f, "        Some(matter_codec::ElementRef::ContainerStart {{ .. }}) => reader.skip_container()?,")?;
        writeln!(f, "        Some(matter_codec::ElementRef::ContainerEnd) => break,")?;
        writeln!(f, "        Some(_) => (),")?;
        writeln!(f, "        None => return Err(TlvDecodeErr::UnclosedContainer),")?;
        writeln!(f, "      }}")?;
        writeln!(f, "    }}")?;

        writeln!(f, "    Ok(Self {{")?;
        for field in self.fields {
            if field.ty.is_supported() {

                writeln!(f, "      {}: {}.ok_or(TlvDecodeErr::MissingField({}))?,", field.name.snake_case(), field.name.snake_case(), field.name.string())?;
            } else {
                writeln!(f, "      {}: TypeUnsupported,", field.name.snake_case())?;
            }
        }
        writeln!(f, "    }})")?;

        writeln!(f, "  }}")?;
        writeln!(f, "}}")?;
        Ok(())
    }
}