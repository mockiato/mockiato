use super::ArgumentMatcher;
use std::fmt::Write;
use std::fmt::{self, Display};

/// Crates an argument matcher that matches any value.
///
/// # Examples
/// ```
/// use mockiato::{any, mockable};
///
/// #[cfg_attr(test, mockable)]
/// trait MessageSender {
///     fn send_message(&self, message: &str);
/// }
///
/// let mut sender = MessageSenderMock::new();
/// let message = "Don't make lemonade";
/// sender.expect_send_message(any());
/// sender.send_message(message);
/// ```
pub fn any() -> AnyArgumentMatcher {
    AnyArgumentMatcher
}

#[derive(Debug)]
pub struct AnyArgumentMatcher;

impl Display for AnyArgumentMatcher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_char('*')
    }
}

impl<'args, U> ArgumentMatcher<U> for AnyArgumentMatcher {
    fn matches_argument(&self, _input: &U) -> bool {
        true
    }
}
