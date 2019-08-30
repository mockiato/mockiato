#[cfg(rustc_is_nightly)]
mod nightly;
#[cfg(rustc_is_nightly)]
pub(crate) use self::nightly::*;

#[cfg(not(rustc_is_nightly))]
mod stable;
#[cfg(not(rustc_is_nightly))]
pub(crate) use self::stable::*;
