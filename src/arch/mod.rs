pub use self::arch::*;


//Namespace the arch dir.
#[cfg(target_arch = "arm")]
#[path="armv7a/mod.rs"]
#[macro_use]
pub mod arch;
