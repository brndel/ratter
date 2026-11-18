use matter_codec::{ContainerKind, ElementRef, Tag, TlvReader, TlvWriter, ValueRef};

use crate::cluster_def::{Nullable, TypeUnsupported};

pub trait TlvEncode: Sized {
    fn tlv_encode(&self, writer: &mut TlvWriter<'_>);
}

pub trait TlvDecode: Sized {
    fn tlv_decode(
        element: ElementRef<'_>, reader: &mut TlvReader<'_>,
    ) -> Result<Self, TlvDecodeErr>;
}

pub trait TlvPut {
    fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut TlvWriter<'_>);
}

// pub trait TlvDecodeScalar: Sized {
//     fn decode_scalar(value: ValueRef<'_>) -> Result<Self, TlvDecodeErr>;
// }

// pub trait TlvDecodeContainer: Sized {
//     fn decode_container(reader: &mut TlvReader<'_>) -> Result<Self, TlvDecodeErr>;
// }

#[derive(Debug, thiserror::Error)]
pub enum TlvDecodeErr {
    #[error("Codec error: {0}")]
    Codec(#[from] matter_codec::Error),
    #[error("Int is too big")]
    IntTooBig,
    #[error("Wrong type")]
    WrongType,
    #[error("Unkown enum variant")]
    UnkownEnumVariant,
    #[error("Unclosed Container")]
    UnclosedContainer,
    #[error("Missing Field '{0}'")]
    MissingField(&'static str),
}

macro_rules! impl_tlv_scalar {
    ($ty:ty, $ident:ident, $put:ident, $to_scalar_fn:ident) => {
        impl TlvPut for $ty {
            fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut TlvWriter<'_>) {
                writer.$put(tag, self.$to_scalar_fn().into()).unwrap();
            }
        }

        impl TlvDecode for $ty {
            fn tlv_decode(
                element: ElementRef<'_>,
                _: &mut TlvReader<'_>,
            ) -> Result<Self, TlvDecodeErr> {
                match element {
                    ElementRef::Scalar {
                        tag: _,
                        value: ValueRef::$ident(value),
                    } => Self::try_from(value).map_err(|_| TlvDecodeErr::IntTooBig),
                    _ => Err(TlvDecodeErr::WrongType),
                }
            }
        }
    };
}

impl_tlv_scalar!(bool, Bool, put_bool, clone);
impl_tlv_scalar!(u8, Uint, put_uint, clone);
impl_tlv_scalar!(u16, Uint, put_uint, clone);
impl_tlv_scalar!(u32, Uint, put_uint, clone);
impl_tlv_scalar!(u64, Uint, put_uint, clone);
impl_tlv_scalar!(i8, Int, put_int, clone);
impl_tlv_scalar!(i16, Int, put_int, clone);
impl_tlv_scalar!(i32, Int, put_int, clone);
impl_tlv_scalar!(i64, Int, put_int, clone);
impl_tlv_scalar!(f32, Float, put_float, clone);
impl_tlv_scalar!(f64, Double, put_double, clone);
impl_tlv_scalar!(String, Utf8, put_utf8, as_str);

impl TlvPut for TypeUnsupported {
    fn tlv_put(&self, _: matter_codec::Tag, _: &mut TlvWriter<'_>) {}
}

impl TlvDecode for TypeUnsupported {
    fn tlv_decode(
        _: ElementRef<'_>, _: &mut TlvReader<'_>,
    ) -> Result<Self, TlvDecodeErr> {
        Ok(Self)
    }
}

impl<T> TlvPut for Nullable<T>
where
    T: TlvPut,
{
    fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut TlvWriter<'_>) {
        match self {
            Nullable::Null => writer.put_null(tag).unwrap(),
            Nullable::Some(value) => value.tlv_put(tag, writer),
        }
    }
}

impl<T: TlvDecode> TlvDecode for Nullable<T> {
    fn tlv_decode(
        element: ElementRef<'_>,
        reader: &mut TlvReader<'_>,
    ) -> Result<Self, TlvDecodeErr> {
        match element {
            ElementRef::Scalar { tag: _, value: ValueRef::Null } => Ok(Self::Null),
            element => Ok(Self::Some(T::tlv_decode(element, reader)?)),
        }
    }
}

impl<T> TlvPut for Vec<T>
where
    T: TlvPut,
{
    fn tlv_put(&self, tag: matter_codec::Tag, writer: &mut TlvWriter<'_>) {
        writer.start_array(tag).unwrap();
        for element in self {
            element.tlv_put(Tag::Anonymous, writer);
        }
        writer.end_container().unwrap();
    }
}

impl<T: TlvDecode> TlvDecode for Vec<T> {
    fn tlv_decode(element: ElementRef<'_>, reader: &mut TlvReader<'_>) -> Result<Self, TlvDecodeErr> {
        if !matches!(element, ElementRef::ContainerStart { kind: ContainerKind::Array, .. }) {
            return Err(TlvDecodeErr::WrongType)
        }

        let mut this = Vec::new();
        loop {
            match reader.next_ref()? {
                Some(ElementRef::ContainerEnd) => break,
                Some(element) => {
                    let value = T::tlv_decode(element, reader)?;
                    this.push(value);
                }
                None => return Err(TlvDecodeErr::UnclosedContainer),
            }
        }

        Ok(this)
    }
}
