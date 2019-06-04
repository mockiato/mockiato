use mockiato::mockable;

#[derive(Debug)]
struct Message;

#[cfg_attr(test, mockable)]
trait MessageGenerator {
    fn generate_message(&self) -> Message;
}

#[test]
fn mocked_method_returns_expected_value() {
    let message_generator = message_generator_mock();
    let _ = message_generator.generate_message();
}

#[test]
#[should_panic]
fn mocked_method_panics_when_invoked_more_than_once() {
    let message_generator = message_generator_mock();
    let _ = message_generator.generate_message();
    let _ = message_generator.generate_message();
}

fn message_generator_mock() -> Box<dyn MessageGenerator> {
    let mut message_generator = MessageGeneratorMock::new();
    message_generator.expect_generate_message_calls_in_order();
    message_generator
        .expect_generate_message()
        .returns_once(Message);
    Box::new(message_generator)
}
