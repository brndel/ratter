#![allow(unused_imports)]

use crate::cluster_def::*;
use crate::tlv::*;
pub const LOCALIZATION_CONFIGURATION: ClusterDef = ClusterDef {
  id: 0x002b,
  name: "Localization Configuration",
  features: &[
  ],
  attributes: &[
    attr::ActiveLocale::ATTR,
    attr::SupportedLocales::ATTR,
  ],
  events: &[
  ],
  commands: &[
  ],
};
pub mod attr {
  use super::*;
  pub struct ActiveLocale;

  impl Attr for ActiveLocale {
    const ATTR: AttrDef = AttrDef {
      id: 0x0000,
      name: "ActiveLocale",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = String;
  }


  pub struct SupportedLocales;

  impl Attr for SupportedLocales {
    const ATTR: AttrDef = AttrDef {
      id: 0x0001,
      name: "SupportedLocales",
      attr_ty: Self::Value::ATTR_TYPE,
      conformance: Conformance::Mandatory,
    };
    type Value = Vec<String>;
  }


}
pub mod ty {
  pub use crate::globals::*;
}
