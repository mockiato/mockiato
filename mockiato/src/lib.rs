#![feature(specialization)]

mod arguments;
mod call;
pub mod debug;
mod expected_calls;
mod function;
mod matcher;
pub mod return_value;

pub use self::arguments::*;
pub use self::call::*;
pub use self::expected_calls::*;
pub use self::function::MockedFunction;
pub use self::matcher::*;
