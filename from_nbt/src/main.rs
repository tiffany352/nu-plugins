use core::fmt::Display;
use indexmap::IndexMap;
use nobility::bin_decode::{Compound, Document, Tag};
use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{Primitive, ReturnSuccess, ReturnValue, Signature, Type, UntaggedValue, Value};
use nu_source::Span;
use std::io::Cursor;

struct Nbt;

impl Nbt {
    fn new() -> Nbt {
        Nbt
    }

    fn tag_to_value(tag: &Tag) -> Result<Value, ShellError> {
        match tag {
            Tag::Byte(value) => Ok(Value {
                value: UntaggedValue::int(*value),
                tag: Default::default(),
            }),
            Tag::Short(value) => Ok(Value {
                value: UntaggedValue::int(*value),
                tag: Default::default(),
            }),
            Tag::Int(value) => Ok(Value {
                value: UntaggedValue::int(*value),
                tag: Default::default(),
            }),
            Tag::Long(value) => Ok(Value {
                value: UntaggedValue::int(*value),
                tag: Default::default(),
            }),
            Tag::Float(value) => Ok(Value {
                value: UntaggedValue::decimal_from_float(*value as f64, Span::default()),
                tag: Default::default(),
            }),
            Tag::Double(value) => Ok(Value {
                value: UntaggedValue::decimal_from_float(*value, Span::default()),
                tag: Default::default(),
            }),
            Tag::ByteArray(value) => Ok(Value {
                value: UntaggedValue::binary(value.to_vec()),
                tag: Default::default(),
            }),
            Tag::String(value) => Ok(Value {
                value: UntaggedValue::string(value.decode().map_err(Self::from_parse_err)?),
                tag: Default::default(),
            }),
            Tag::Compound(compound) => Ok(Value {
                value: Self::compound_to_value(compound)?,
                tag: Default::default(),
            }),
            Tag::List(list) => {
                let values = list
                    .iter()
                    .map(|tag| Self::tag_to_value(&tag))
                    .collect::<Result<Vec<_>, ShellError>>()?;
                Ok(Value {
                    value: UntaggedValue::table(&values),
                    tag: Default::default(),
                })
            }
            Tag::IntArray(array) => {
                let values = array
                    .iter()
                    .map(|num| Value {
                        value: UntaggedValue::int(num),
                        tag: Default::default(),
                    })
                    .collect::<Vec<_>>();
                Ok(Value {
                    value: UntaggedValue::table(&values),
                    tag: Default::default(),
                })
            }
            Tag::LongArray(array) => {
                let values = array
                    .iter()
                    .map(|num| Value {
                        value: UntaggedValue::int(num),
                        tag: Default::default(),
                    })
                    .collect::<Vec<_>>();
                Ok(Value {
                    value: UntaggedValue::table(&values),
                    tag: Default::default(),
                })
            }
            tag => Err(ShellError::untagged_runtime_error(format!(
                "Unrecognized NBT tag {:?}",
                tag.tag_type()
            ))),
        }
    }

    fn from_parse_err(err: impl Display) -> ShellError {
        ShellError::untagged_runtime_error(format!("{}", err))
    }

    fn compound_to_value(compound: &Compound) -> Result<UntaggedValue, ShellError> {
        let mut map = IndexMap::new();
        for entry in compound.iter() {
            let key = entry
                .name()
                .decode()
                .map_err(Self::from_parse_err)?
                .into_owned();
            let value = Self::tag_to_value(entry.value())?;
            map.insert(key, value);
        }
        Ok(UntaggedValue::row(map))
    }

    fn from_nbt(&mut self, value: Value) -> Result<Value, ShellError> {
        match &value.value {
            UntaggedValue::Primitive(Primitive::Binary(data)) => {
                let cursor = Cursor::new(data);
                let doc = Document::load(cursor)?;
                let (_name, root) = doc.parse().map_err(Self::from_parse_err)?;
                Ok(Value {
                    value: Self::compound_to_value(&root)?,
                    tag: value.tag,
                })
            }
            _ => Err(ShellError::labeled_error(
                "Unrecognized type in stream",
                "'from nbt' given non-binary info by this",
                value.tag.span,
            )),
        }
    }
}

impl Plugin for Nbt {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("from nbt")
            .desc("Convert from .nbt binary into table")
            .input(Type::Binary)
            .filter())
    }

    fn filter(&mut self, input: Value) -> Result<Vec<ReturnValue>, ShellError> {
        Ok(vec![ReturnSuccess::value(self.from_nbt(input)?)])
    }
}

fn main() {
    serve_plugin(&mut Nbt::new());
}
