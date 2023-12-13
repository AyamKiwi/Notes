use std::io;

// INPUT KE VARIABEL F64
fn read() -> f64 {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed");
    return buffer
        .trim()
        .parse()
        .expect("Failed");
}

// FUNGSI F(X)
fn f(x : f64) -> f64 {
    return (x * x) - 3.0 - x.ln();
}

fn main() {

    // INPUT BATAS BAWAH 
    println!();
    println!("Batas bawah =");
    let mut a = read();

    // INPUT BATAS ATAS
    println!();
    println!("Batas atas =");
    let mut b = read();

    // VALIDASI INTERVAL
    while f(a) * f(b) >= 0.0{
        println!();
        println!("Batas bawah =");
        a = read();

        println!();
        println!("Batas atas =");
        b = read();
    }

    // INPUT GALAT
    println!();
    println!("Batas Lebar Interval =");
    let galat = read();

    println!();
    println!("Galat Relatif =");
    let galat_rel = read();
    // LOOPING ALGORITMA
    let mut iter = 0;
    let mut c_lama = 0.0;
    let mut c = (a * f(b) - b * f(a))/(f(b) - f(a));
    while (f(c) != 0.0) 
        & ((a - b).abs() > galat) 
        & (((c - c_lama)/c).abs() > galat_rel){

        // OUTPUT SEMENTARA
        println!();
        println!();
        println!("Iterasi {iter}");
        println!();
        println!("a = {a}");
        println!("b = {b}");
        println!("c = {c}");
        println!();
        println!("f(c) = {}",f(c));
        println!();
        println!("f(a) * f(c) = {}",f(a)*f(c));
        println!("f(b) * f(c) = {}",f(b)*f(c));
        
        // PERBARUAN INTERVAL
        if f(a) * f(c) < 0.0 {
            b = c;
        }
        if f(b) * f(c) < 0.0 {
            a = c;
        }
        
        // PERBARUAN NILAI C
        c_lama = c;
        c = (a * f(b) - b * f(a))/(f(b) - f(a));

        iter = iter + 1;
    }

    // OUTPUT AKHIR
    println!();
    println!();
    println!("iterasi {iter}");
    println!();
    println!("a = {a}");
    println!("b = {b}");
    println!("c = {c}");
    println!();
    println!("f(c) = {}",f(c));
    println!();
    println!("f(a) * f(c) = {}",f(a)*f(c));
    println!("f(b) * f(c) = {}",f(b)*f(c));
    println!();
    println!("x = {c}");
}
