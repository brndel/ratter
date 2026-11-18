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
pub struct CommandEntry {
    #[serde(rename = "@id")]
    id: IntValue,
    #[serde(rename = "@name")]
    pub name: Ident,
    #[serde(rename = "@direction")]
    direction: String,
    #[serde(rename = "@response")]
    response: Option<String>,
    #[serde(flatten)]
    conformance: ConformanceFields,
    #[serde(default)]
    #[serde(rename = "field")]
    fields: Vec<FieldEntry>,
}

#[nest_struct]
pub struct Command<'a> {
    id: IntValue,
    name: Ident,
    fields: Vec<Field<'a>>,
    conformance: Conformance,
    response: CommandResponse,
}

enum CommandResponse {
    ResponseFromServer, // this command is a responseFromServer
    Y,
    N,
    Name(String),
}

impl CommandResponse {
    pub fn response_name(&self) -> Option<&str> {
        match self {
            CommandResponse::ResponseFromServer => None,
            CommandResponse::Y => Some("BasicAccResponse"),
            CommandResponse::N => Some("()"),
            CommandResponse::Name(name) => Some(name.as_str()),
        }
    }
}

impl CommandEntry {
    pub fn entry(&self, features: FeatureList) -> Command<'_> {
        Command {
            id: self.id,
            name: self.name.clone(),
            fields: self
                .fields
                .iter()
                .map(|field| field.entry(features))
                .collect(),
            conformance: self.conformance.conformance(features),
            response: self.response(),
        }
    }

    pub fn is_to_server(&self) -> bool {
        self.direction == "commandToServer"
    } 

    fn response(&self) -> CommandResponse {
        match self.direction.as_str() {
            "commandToServer" => {
                let response = self.response.as_ref().unwrap();
                match response.as_str() {
                    "Y" => CommandResponse::Y,
                    "N" => CommandResponse::N,
                    _ => CommandResponse::Name(response.clone()),
                }
            }
            "responseFromServer" => {
                if let Some(response) = &self.response {
                    println!("Response has Type {response}");
                }
                CommandResponse::ResponseFromServer
            }
            dir => panic!("unkown direction {dir}"),
        }
    }
}

impl Display for Command<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", DEFAULT_DERIVE)?;
        writeln!(f, "pub struct {} {{", self.name)?;
        for field in &self.fields {
            writeln!(f, "  pub {}: {},", field.name.snake_case(), field.ty)?;
        }
        writeln!(f, "}}")?;
        
        writeln!(f, "impl Command for {} {{", self.name)?;
        writeln!(f, "  const COMMAND: CommandDef = CommandDef {{")?;
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


        if let Some(response) = self.response.response_name() {
            writeln!(f, "impl CommandInvoke for {} {{", self.name)?;
            writeln!(f, "  type Response = {};", response)?;
            writeln!(f, "}}")?;

            writeln!(f, "impl TlvEncode for {} {{", self.name)?;
            writeln!(f, "  fn tlv_encode(&self, writer: &mut matter_codec::TlvWriter<'_>) {{")?;
            writeln!(f, "    writer.start_structure(matter_codec::Tag::Anonymous).unwrap();")?;
            for field in &self.fields {
                writeln!(f, "    self.{}.tlv_put(matter_codec::Tag::Context({}), writer);", field.name.snake_case(), field.id)?;
            }
            writeln!(f, "    writer.end_container().unwrap();")?;
            writeln!(f, "  }}")?;
            writeln!(f, "}}")?;

        } else {
            writeln!(f, "impl CommandResponse for {} {{}}", self.name)?;
        }


        Ok(())
    }
}
