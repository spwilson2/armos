use arch::drivers::uart;
use core::fmt::Write;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ({
            use core::fmt::Write;
            let mut writer = $crate::arch::print::UartWriter{};
            writer.write_fmt(format_args!($($arg)*)).unwrap();
    });
}
macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[cfg(debug)]
macro_rules! debug_print {
    ($fmt:expr) => (println!($fmt));
    ($fmt:expr, $($arg:tt)*) => (println!($fmt, $($arg)*));
}

#[cfg(debug)]
macro_rules! debug {
    ($fmt:expr) => ($fmt);
}

#[cfg(not(debug))]
macro_rules! debug {
    ($fmt:expr) => ($fmt);
}

#[cfg(not(debug))]
macro_rules! debug_print {
    ($_:expr) => {()};
    ($_:expr, $($__:tt)*) => {()};
}

pub struct UartWriter {
}

impl ::core::fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        uart::puts(s);
        Ok(())
    }
}
