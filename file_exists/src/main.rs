use nu_errors::ShellError;
use nu_plugin::{serve_plugin, Plugin};
use nu_protocol::{
    CallInfo, Primitive, ReturnSuccess, ReturnValue, Signature, SyntaxShape, Type, UntaggedValue,
    Value,
};
use std::path::{Path, PathBuf};

struct Exists;

impl Exists {
    fn new() -> Exists {
        Exists
    }

    fn check(&mut self, path: &Path) -> Result<UntaggedValue, ShellError> {
        let value = path.exists();
        Ok(UntaggedValue::boolean(value))
    }

    fn exists(&mut self, value: &Value) -> Result<Value, ShellError> {
        match &value.value {
            UntaggedValue::Primitive(Primitive::Path(path)) => Ok(Value {
                value: self.check(path)?,
                tag: value.tag.clone(),
            }),
            UntaggedValue::Primitive(Primitive::String(string)) => Ok(Value {
                value: self.check(&PathBuf::from(&string))?,
                tag: value.tag.clone(),
            }),
            _ => Err(ShellError::labeled_error(
                "Unrecognized type in stream",
                "'from nbt' given non-binary info by this",
                value.tag.span,
            )),
        }
    }
}

impl Plugin for Exists {
    fn config(&mut self) -> Result<Signature, ShellError> {
        Ok(Signature::build("exists?")
            .desc("Returns whether or not a file exists.")
            .required("path", SyntaxShape::Path, "The path to check")
            .yields(Type::Boolean)
            .filter())
    }

    fn begin_filter(&mut self, call_info: CallInfo) -> Result<Vec<ReturnValue>, ShellError> {
        let arg = call_info.args.expect_nth(0)?;
        let result = self.exists(arg)?;
        Ok(vec![ReturnSuccess::value(result)])
    }
}

fn main() {
    serve_plugin(&mut Exists::new());
}
