use super::ArgumentMatcher;
use crate::internal::argument_matcher_factory::ArgumentMatcherFactory;
use std::fmt::Write;
use std::fmt::{self, Display};

impl ArgumentMatcherFactory {
    /// Crates an argument matcher that matches any value.
    ///
    /// # Examples
    /// ```
    /// use mockiato::mockable;
    ///
    /// #[cfg_attr(test, mockable)]
    /// trait MessageSender {
    ///     fn send_message(&self, message: &str);
    /// }
    ///
    /// let mut sender = MessageSenderMock::new();
    /// let message = "Don't make lemonade";
    /// sender.expect_send_message(|f| f.any());
    /// sender.send_message(message);
    /// ```
    pub fn any(&self) -> AnyArgumentMatcher {
        AnyArgumentMatcher
    }
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
