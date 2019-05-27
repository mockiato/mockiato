use super::ArgumentMatcher;
use crate::internal::argument::Argument;
use std::fmt::Write;
use std::fmt::{self, Display};

impl Argument {
    /// Crates an argument matcher that matches any value.
    ///
    /// # Examples
    /// ```
    /// use mockiato::{mockable, Argument};
    ///
    /// # const IGNORED: &str = "
    /// #[cfg_attr(test, mockable)]
    /// # ";
    /// # #[mockable]
    /// trait MessageSender {
    ///     fn send_message(&self, message: &str);
    /// }
    ///
    /// let mut sender = MessageSenderMock::new();
    /// let message = "Don't make lemonade";
    /// sender.expect_send_message(Argument::any).returns(());
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
