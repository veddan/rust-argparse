#![feature(globs)]

extern crate collections;
extern crate arena;

pub use self::parser::{ArgumentParser, Ref};

pub mod action;
pub mod parser;
mod generic;
mod help;

mod bool;
mod num;

pub struct StoreTrue;

pub struct StoreFalse;

pub struct StoreConst<T>(pub T);

pub struct Store<T>;

pub struct StoreOption<T>;

pub struct List<T>;

pub struct Collect<T>;

pub struct IncrBy<T>(pub T);

pub struct DecrBy<T>(pub T);

pub type StoreBool = Store<bool>;

#[cfg(test)]
mod tests {
    pub use super::*;
    pub use self::common::{check_ok, check_err, check_exit};

    mod test_parser;
    mod test_bool;
    mod test_int;
    mod test_float;
    mod test_str;
    mod test_enum;
    mod test_pos;
    mod test_many;
    mod test_optional;
    mod test_usage;
    mod test_help;
    mod test_env;
    mod common;
}
