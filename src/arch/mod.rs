pub use self::arch::*;


//Namespace the arch dir.
#[cfg(target_arch = "arm")]
#[path="armv7a/mod.rs"]
pub mod arch;
