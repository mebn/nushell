use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{Category, Example, PipelineData, ShellError, Signature, Type, Value};
use ulid::Generator;

#[derive(Clone)]
pub struct SubCommand;

impl Command for SubCommand {
    fn name(&self) -> &str {
        "random ulid"
    }

    fn signature(&self) -> Signature {
        Signature::build("random ulid")
            .category(Category::Random)
            .input_output_types(vec![(Type::Nothing, Type::String)])
            .allow_variants_without_examples(true)
    }

    fn usage(&self) -> &str {
        "Generate a random ulid string."
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["generate", "ulid"]
    }

    fn run(
        &self,
        _engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        ulid(call)
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Generate a random ulid string",
            example: "random ulid",
            result: None,
        }]
    }
}

fn ulid(call: &Call) -> Result<PipelineData, ShellError> {
    let span = call.head;
    let mut gen = Generator::new();
    let new_ulid = gen.generate().unwrap();

    Ok(PipelineData::Value(Value::string(new_ulid, span), None))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;

        test_examples(SubCommand {})
    }
}
