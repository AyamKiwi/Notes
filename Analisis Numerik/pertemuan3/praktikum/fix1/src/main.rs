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
    (x * x) - 3.0 - x.ln() 
}

// FUNGSI g(x)
fn g(x:f64) -> f64 {
    (x * x - 3.0).exp()
    // (3.0 + x.ln()).sqrt()
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
        {   // OUTPUT ITERASI
            println!();
            println!();
            println!("Iterasi {iter}");
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
    {   // OUTPUT ITERASI
        println!();
        println!();
        println!("Iterasi {iter}");
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
