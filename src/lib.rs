#[macro_use]
extern crate log;
extern crate env_logger;
use log::{Level, Metadata, Record};
use cxx::{CxxString};

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn rust_from_cpp(message: &CxxString) -> ();
        fn init_rust_logger() -> ();
    }
}

// struct NoopLogger
// {
//     level: Level,
// }

// impl log::Log for NoopLogger
// {
//     fn enabled(&self, metadata: &Metadata) -> bool
//     {
//         true
//     }

//     fn log(&self, _record: &Record)
//     {
//     }

//     fn flush(&self)
//     {
//     }
// }

// impl NoopLogger
// {
//     fn init(level: Level)
//     {
//         println!(" impl NoopLogger");
//         log::set_max_level(level.to_level_filter());
//         let logger = NoopLogger { level };
//         log::set_boxed_logger(Box::new(logger)).unwrap();
//     }
// }

pub fn rust_from_cpp(message: &CxxString) -> () {
    info!("{}", message);
}

pub fn init_rust_logger() -> ()
{
    // println!("init_rust_logger");
    // NoopLogger::init(Level::Debug);
    env_logger::init();
    // .filter(None, log::LevelFilter::Off)
    // .init();
    // let logger = Box::new(NoopLogger { level: Level::Error });

    // log::set_boxed_logger(logger).unwrap();
    // log::set_max_level(log::LevelFilter::Info);
}