use super::ArgumentMatcher;
use crate::internal::argument::Argument;
use crate::internal::fmt::{MaybeDebug, MaybeDebugWrapper};
use nameof::name_of;
use std::fmt::{self, Debug, Display};

impl Argument {
    /// Creates an argument matcher that matches values using [`PartialEq`].
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
    /// let message = "Hello World";
    /// sender.expect_send_message(|a| a.partial_eq(message));
    /// sender.send_message(message);
    /// ```
    pub fn partial_eq<T>(&self, value: T) -> PartialEqArgumentMatcher<T> {
        PartialEqArgumentMatcher { value }
    }

    /// Creates an argument matcher that matches an owned value against references of itself using [`PartialEq`].
    ///
    /// # Examples
    /// ```
    /// use mockiato::mockable;
    ///
    /// #[derive(Clone, PartialEq)]
    /// enum Message {
    ///     Ping,
    /// }
    ///
    /// #[cfg_attr(test, mockable)]
    /// trait MessageSender {
    ///     fn send_message(&self, message: &Message);
    /// }
    ///
    /// # fn main() {
    /// let mut sender = MessageSenderMock::new();
    /// sender.expect_send_message(|a| a.partial_eq_owned(Message::Ping));
    /// sender.send_message(&Message::Ping);
    /// # }
    /// ```
    pub fn partial_eq_owned<T>(&self, value: T) -> OwnedPartialEqArgumentMatcher<T> {
        OwnedPartialEqArgumentMatcher { value }
    }
}

pub struct PartialEqArgumentMatcher<T>
where
    T: MaybeDebug,
{
    value: T,
}

impl<T> Display for PartialEqArgumentMatcher<T>
where
    T: MaybeDebug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(&self.value, f)
    }
}

impl<T> Debug for PartialEqArgumentMatcher<T>
where
    T: MaybeDebug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of!(type PartialEqArgumentMatcher<T>))
            .field(name_of!(value in Self), &MaybeDebugWrapper(&self.value))
            .finish()
    }
}

impl<T, U> ArgumentMatcher<U> for PartialEqArgumentMatcher<T>
where
    T: PartialEq<U> + MaybeDebug,
{
    fn matches_argument(&self, input: &U) -> bool {
        &self.value == input
    }
}

pub struct OwnedPartialEqArgumentMatcher<T>
where
    T: MaybeDebug,
{
    value: T,
}

impl<T> Display for OwnedPartialEqArgumentMatcher<T>
where
    T: MaybeDebug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        MaybeDebug::fmt(&self.value, f)
    }
}

impl<'args, T, U> ArgumentMatcher<&'args U> for OwnedPartialEqArgumentMatcher<T>
where
    T: PartialEq<U> + MaybeDebug,
{
    fn matches_argument(&self, input: &&U) -> bool {
        &self.value == *input
    }
}

impl<T> Debug for OwnedPartialEqArgumentMatcher<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(name_of!(type OwnedPartialEqArgumentMatcher<T>))
            .field(name_of!(value in Self), &MaybeDebugWrapper(&self.value))
            .finish()
    }
}
