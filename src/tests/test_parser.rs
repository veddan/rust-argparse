use parser::ArgumentParser;
use super::{check_ok, check_err, check_exit};

#[test]
fn test_no_arg() {
    let ap = ArgumentParser::new();
    check_ok(&ap, ["./argparse_test"]);
    check_err(&ap, ["./argparse_test", "a"]);
    check_err(&ap, ["./argparse_test", "-a"]);
    check_err(&ap, ["./argparse_test", "--an-option"]);
}

#[test]
fn test_help() {
    let ap = ArgumentParser::new();
    check_ok(&ap, ["./argparse_test"]);
    check_exit(&ap, ["./argparse_test", "--help"]);
}

