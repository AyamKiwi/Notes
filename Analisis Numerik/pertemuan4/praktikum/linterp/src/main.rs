use std::io;

macro_rules! trimparse

// READ USER INPUT TO VARIABLE
fn readtovar(name : &str) -> String {
    println!();
    println!("{name} =");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed");
    return buffer
}

fn linterp(){
    println!();
    let a : f64 = readtovar("x0").trim().parse().expect("");
    let fa : f64 = readtovar("f(x0)").trim().parse().expect("");
    
    println!();
    let b : f64 = readtovar("x1").trim().parse().expect("");
    let fb : f64 = readtovar("f(x1)").trim().parse().expect("");

    println!();
    let c : f64 = readtovar("x").trim().parse().expect("");

    println!();
    println!();
    println!("Hasil = {}",fa + ((fb - fa) / (b - a)) * (c - a));
}

fn quanterp(){
    println!();
    let a : f64 = readtovar("x0").trim().parse().expect("");
    let fa : f64 = readtovar("f(x0)").trim().parse().expect("");
    
    println!();
    let b : f64 = readtovar("x1").trim().parse().expect("");
    let fb : f64 = readtovar("f(x1)").trim().parse().expect("");

    println!();
    let c : f64 = readtovar("x2").trim().parse().expect("");
    let fc : f64 = readtovar("f(x1)").trim().parse().expect("")

    let x = a - b ;
    let x2 = a.powi(2) - b.powi(2);

    let y = b - c;
    let y2 = b.powi(2) - c.powi(2);

    let z1 = fa - fb;
    let z2 = fb - fc;
    
}
// ENTRY POINT
fn main() {
    // linterp();
}