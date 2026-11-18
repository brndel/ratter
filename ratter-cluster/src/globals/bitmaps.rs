use crate::cluster_def::*;
use crate::tlv::*;
bitflags::bitflags! {
  #[derive(Debug, Clone, PartialEq)]
  #[derive(Copy)]
  pub struct WildcardPathFlagsBitmap: u32 {
    ///Skip the Root Node endpoint (endpoint 0) during wildcard expansion
    const WildcardSkipRootNode = 1 << 0;
    ///Skip several large global attributes during wildcard expansion
    const WildcardSkipGlobalAttributes = 1 << 1;
    ///Skip the AttributeList global attribute during wildcard expansion
    const WildcardSkipAttributeList = 1 << 2;
    ///
    const DoNotUse = 1 << 3;
    ///Skip the AcceptedCommandList and GeneratedCommandList global attributes during wildcard expansion
    const WildcardSkipCommandLists = 1 << 4;
    ///Skip any manufacturer-specific clusters or attributes during wildcard expansion
    const WildcardSkipCustomElements = 1 << 5;
    ///Skip any Fixed (F) quality attributes during wildcard expansion
    const WildcardSkipFixedAttributes = 1 << 6;
    ///Skip any Changes Omitted ++(C)++ quality attributes during wildcard expansion
    const WildcardSkipChangesOmittedAttributes = 1 << 7;
    ///Skip all clusters with the Diagnostics (K) quality during wildcard expansion
    const WildcardSkipDiagnosticsClusters = 1 << 8;
  }
}
impl AttrType for WildcardPathFlagsBitmap {
  const ATTR_TYPE: AttrTypeDef = AttrTypeDef::map8(&[
    (0, "WildcardSkipRootNode"),
    (1, "WildcardSkipGlobalAttributes"),
    (2, "WildcardSkipAttributeList"),
    (3, "DoNotUse"),
    (4, "WildcardSkipCommandLists"),
    (5, "WildcardSkipCustomElements"),
    (6, "WildcardSkipFixedAttributes"),
    (7, "WildcardSkipChangesOmittedAttributes"),
    (8, "WildcardSkipDiagnosticsClusters"),
  ]);
}
impl TlvPut for WildcardPathFlagsBitmap {
  fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut matter_codec::TlvWriter<'_>) {
    writer.put_uint(tag, self.bits() as u64).unwrap();
  }
}
impl From<u64> for WildcardPathFlagsBitmap {
  fn from(value: u64) -> Self {
    Self::from_bits_retain(value as u32)
  }
}
impl TlvDecode for WildcardPathFlagsBitmap {
  fn tlv_decode(element: matter_codec::ElementRef<'_>, reader: &mut matter_codec::TlvReader<'_>,) -> Result<Self, TlvDecodeErr> {
    Ok(Self::from(u64::tlv_decode(element, reader)?))
  }
}

