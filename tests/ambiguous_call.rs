use mockiato::mockable;

#[mockable]
trait MessageSender {
    fn ping(&self);
}

#[test]
#[should_panic(
    expected = "The call MessageSenderMock::ping() matches more than one expected call:\n\
                ping() -> () exactly 1 time, was called 0 times\n\
                ping() -> () exactly 1 time, was called 0 times"
)]
fn panics_on_more_than_one_matching_call() {
    let mut message_sender = MessageSenderMock::new();
    message_sender.expect_ping().times(1);
    message_sender.expect_ping().times(1);
    message_sender.ping();
}
