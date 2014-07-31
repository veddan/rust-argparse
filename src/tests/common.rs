use std::io::MemWriter;
use std::str::from_utf8;
use parser::ArgumentParser;

pub fn check_ok(ap: &ArgumentParser, args: &[&str]) {
    let mut stdout = MemWriter::new();
    let mut stderr = MemWriter::new();
    let mut owned_args = Vec::new();
    for x in args.iter() {
        owned_args.push(x.to_string());
    }
    let res = ap.parse(owned_args, &mut stdout, &mut stderr);
    match res {
        Ok(()) => return,
        Err(x) => fail!(
            from_utf8(stderr.unwrap().as_slice()).unwrap().to_string() +
            format!("Expected ok, but found Exit({})", x)),
    }
}

pub fn check_exit(ap: &ArgumentParser, args: &[&str]) {
    let mut stdout = MemWriter::new();
    let mut stderr = MemWriter::new();
    let mut owned_args = Vec::new();
    for x in args.iter() {
        owned_args.push(x.to_string());
    }
    let res = ap.parse(owned_args, &mut stdout, &mut stderr);
    match res {
        Err(0) => return,
        Err(x) => fail!(format!("Expected code {} got {}", 0u, x)),
        Ok(()) => fail!(format!("Expected failure, got success")),
    }
}

pub fn check_err(ap: &ArgumentParser, args: &[&str]) {
    let mut stdout = MemWriter::new();
    let mut stderr = MemWriter::new();
    let mut owned_args = Vec::new();
    for x in args.iter() {
        owned_args.push(x.to_string());
    }
    let res = ap.parse(owned_args, &mut stdout, &mut stderr);
    match res {
        Err(2) => return,
        Err(x) => fail!(format!("Expected code {} got {}", 2u, x)),
        Ok(()) => fail!(format!("Expected failure, got success")),
    }
}