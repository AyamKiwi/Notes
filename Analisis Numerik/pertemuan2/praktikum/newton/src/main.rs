use std::io;

// INPUT KE VARIABEL F64
fn read() -> String {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed");
    return buffer;
}

// FUNGSI F(X)
fn f(x : f64) -> f64 {
    return 3.0 + x.ln() - (x * x);
}

// FUNGSI F'(X)
fn f_(x : f64) -> f64 {
    return (1.0/x) - (2.0 * x)
}

fn main() {

    // INPUT X0
    println!("x0 =");
    let mut x : f64 = read().trim().parse().expect("Not a real number!");
    while f_(x) == 0.0 {
        println!();
        println!("Invalid value");
        println!("x0 =");
        x = read().trim().parse().expect("Not a real number!");
    }

    // INPUT GALAT
    println!();
    println!("Galat Mutlak =");
    let galat : f64 = read().trim().parse().expect("Not a real number!");

    // INPUT GALAT RELATIF
    println!();
    println!("Galat Relatif");
    let galat_ : f64 = read().trim().parse().expect("Not a real number!");
    
    // lOOPING ALGORITMA
    let mut iter = 0;
    while (f(x) != 0.0) & ((f(x)/f_(x)).abs() > galat) & ((f(x)/(f_(x)*(x - (f(x)/f_(x))))).abs() > galat_) {

        {   // OUTPUT ITERASI
            println!();
            println!();
            println!("Iteras {iter}");
            println!();
            println!("x0       = {x}");
            println!("f(x0)    = {}", f(x));
            println!();
            println!("x        = {}", x - (f(x)/f_(x)));
            println!("|x - x0| = {}", (f(x)/f_(x)).abs());
        }

        // PERHITUNGAN ITERASI
        x = x - (f(x)/f_(x));
        iter = iter + 1;
    }

    {   // OUTPUT HASIL
        println!();
        println!();
        println!("Iteras {iter}");
        println!();
        println!("x0       = {x}");
        println!("f(x0)    = {}", f(x));
        println!();
        println!("x        = {}", x - (f(x)/f_(x)));
        println!("|x - x0| = {}", (f(x)/f_(x)).abs());
        println!();
        println!();
        println!("Solusi x = {x}");
    }
}
