#[macro_use]
extern crate log;
extern crate env_logger;
use cxx::{CxxString};

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn rust_from_cpp(message: &CxxString) -> ();
        fn init_rust_logger() -> ();
    }
}

pub fn rust_from_cpp(message: &CxxString) -> () {
    info!("{}", message);
}

pub fn init_rust_logger() -> ()
{
    let logger = Box::new(NoopLogger);

    log::set_boxed_logger(logger).unwrap();
    log::set_max_level(log::LevelFilter::Trace);
}

struct NoopLogger;

impl log::Log for NoopLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, _record: &log::Record) {
        // do nothing
    }

    fn flush(&self) {
        // do nothing
    }
}