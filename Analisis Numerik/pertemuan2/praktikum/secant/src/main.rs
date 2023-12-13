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

fn main() {

    // INPUT X0
    println!();
    println!("x0 =");
    let mut x0 : f64 = read().trim().parse().expect("Not a real number!");

    // INPUT X1
    println!();
    println!("x1 =");
    let mut x : f64 = read().trim().parse().expect("Not a real number!");

    // VALIDASI X0 X1
    while f(x) == f(x0) {
        println!();
        println!();
        println!("Invalid");
        println!();
        println!("x0 =");
        x0 = read().trim().parse().expect("Not a real number!");
        println!();
        println!("x1 =");
        x = read().trim().parse().expect("Not a real number!");
    }

    // INPUT GALAT
    println!();
    println!("Galat Mutlak =");
    let galat : f64 = read().trim().parse().expect("Not a real number!");

    // INPUT GALAT RELATIF
    println!();
    println!("Galat Relatif =");
    let galat_ : f64 = read().trim().parse().expect("Not a real number!");

    // LOOPING ALGORITMA
    let mut iter = 0;
    while (f(x) != 0.0) & (f(x) != f(x0)) & ((x - x0).abs() > galat) & (((x - x0) / x).abs() > galat_) {
        
        {   // OUTPUT ITERASI
            println!();
            println!();
            println!("Iterasi {iter}");
            println!();
            println!("x{iter} = {x0}");
            println!("x{} = {x}",iter + 1);
            println!();
            println!("f(x{}) = {}", iter + 1, f(x));
            println!();
            println!("|x{} - x{iter}| = {}", iter + 1, (x - x0).abs());
        }

        let buffer = x;
        x = x - (f(x) * (x - x0)) / (f(x) - f(x0));
        x0 = buffer;
        iter = iter + 1
    }
    {   // OUTPUT ITERASI
        println!();
        println!();
        println!("Iterasi {iter}");
        println!();
        println!("x{iter} = {x0}");
        println!("x{} = {x}",iter + 1);
        println!();
        println!("f(x{iter}) = {}", f(x));
        println!();
        println!("|x{} - x{iter}| = {}", iter + 1, (x - x0).abs());
        println!();
        println!();
        println!("Solusi x = {x}");
        println!();
    }
}
