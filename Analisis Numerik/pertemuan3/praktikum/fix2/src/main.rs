
use std::io;

// INPUT KE VARIABEL
fn read() -> String {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed");
    return buffer;
}

// FUNGSI f(x)
fn f(x:f64) -> f64{
    x.powi(4) + 3.0 * x.powi(3) - x.powi(2) + 2.0 * x - 4.0
}

// FUNGSI g(x)
fn g(x:f64) -> f64 {
    // (4.0 + x.powi(2) - 3.0 * x.powi(3) - x.powi(4)) / 2.0 // g1(x)
    // (2.0 * x - 4.0 + 3.0 * x.powi(3) + x.powi(4)).sqrt() // g2(x)
    // ((4.0 - 2.0 * x + x.powi(2) - x.powi(4)) / 3.0).cbrt() // g3(x)
    (4.0 - 2.0 * x + x.powi(2) - 3.0 * x.powi(3)).powf(0.25) // g4(x)
}

// ENTRY POINT
fn main() {
    
    // INPUT X
    println!();
    println!("x = ");
    let mut x : f64 = read()
        .trim()
        .parse()
        .expect("Failed");

    // UJI PERKIRAAN AWAL
    if f(x) == 0.0 {
        println!();
        println!();
        println!("f(x) = {}", f(x));
        println!();
        println!("Solusi akhir = {x}");
        return ();
    }

    // INPUT GALAT
    println!();
    println!("Batas galat = ");
    let galat : f64 = read()
        .trim()
        .parse()
        .expect("Failed");

    // INPUT BATAS ITERASI
    println!();
    println!("Batas iterasi = ");
    let max : i32 = read()
        .trim()
        .parse()
        .expect("Failed");

    // LOOPING ALGORITMA
    let mut iter = 0;
    while   (true)
            & (f(g(x)) != 0.0) 
            & ((g(x) - x).abs() > galat) 
            & (iter < max){
        {
            // OUTPUT ITERASI
            println!();
            println!();
            println!("Iteras {iter}");
            println!();
            println!("x{iter} = {x}");
            println!("f(x{iter}) = {}",f(x));
            println!();
            println!("x{} = {}", iter + 1, g(x));
            println!("f(x{}) = {}", iter + 1, f(g(x)));
            println!();
            println!("galat = {}", ((g(x) - x) / g(x)).abs());
        }

        // PERHITUNGAN
        x = g(x);
        iter = iter + 1;
    }
    {
        // OUTPUT ITERASI
        println!();
        println!();
        println!("Iteras {iter}");
        println!();
        println!("x{iter} = {x}");
        println!("f(x{iter}) = {}",f(x));
        println!();
        println!("x{} = {}", iter + 1, g(x));
        println!("f(x{}) = {}", iter + 1, f(g(x)));
        println!();
        println!("galat = {}", ((g(x) - x) / g(x)).abs());
        println!();
        println!();
        println!("Solusi akhir = {x}");
    }
}
