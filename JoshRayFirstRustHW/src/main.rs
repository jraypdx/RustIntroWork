#![allow(non_snake_case)]
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //assert!(args.len() == 4, "You must enter 3 arguments on command line");
    if args.len() != 4 {
        error()
    }

    let x = args[1].parse::<u64>().unwrap();
    let y = args[2].parse::<u64>().unwrap();
    let m = args[3].parse::<u64>().unwrap();

    //assert!(x >= 0, "X must be a positive number or 0");
    //assert!(y >= 0, "Y must be a positive number or 0");
    assert!(m >= 1, "Mod must be a positive number"); //Can't modulo with 0

    //println!("x-y{} y-m{} m-x{}", x-y, y-m, m-x);
    let fnoutput = modexp(x, y, m);
    //println!("( {} ^ {} ) mod {} = {}", x, y, m, fnoutput);
    println!("{}", fnoutput)
}

fn modexp(x: u64, y: u64, m: u64) -> u64 {
    if x == 0 {
        return 0;
    }
    if y == 0 {
        return 1;
    }
    let mut z = modexp(x, y / 2, m);
    z = (z * z) % m;
    if y % 2 == 1 {
        z = (z * x) % m;
    }
    z
}

//Error function from assignment description
fn error() -> ! {
    eprintln!("modexp: usage: modexp <x> <y> <m>  ex: (x^y)%m");
    std::process::exit(1);
}

#[test]
fn assignment_supplied_example() {
    assert!(modexp(2, 20, 17) == 16);
}

#[test]
fn another_example() {
    assert!(modexp(5, 4, 3) == 1);
}

#[test]
fn last_test() {
    assert!(modexp(12, 2, 44) == 12);
}
