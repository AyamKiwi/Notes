use f64 as real;
use i32 as int;
const EXACT: real = 11.71875;

fn f(x: real, y: real) -> real {
    return x.powi(2)*y.powi(2);
}

fn euler(n: int, x: real, x_0: real, y_0: real) -> real {
    let h = (x - x_0)/(n as real);
    println!("\nMETODE EULER:\nh = {h}");
    let mut x_buffer = x_0;
    let mut y_buffer = y_0;
    for _ in 0..n {
        y_buffer += h*f(x_buffer,y_buffer);
        x_buffer += h;
    }
    println!("y = {y_buffer}");
    println!("galat = |11,71875 - {y_buffer}| = {}\n", (EXACT - y_buffer).abs());
    return y_buffer;
}

fn heun(n: int, x: real, x_0: real, y_0: real) -> real {
    let h = (x - x_0)/(n as real);
    println!("\nMETODE HEUN:\nh = {h}");
    let mut x_buffer = x_0;
    let mut y_buffer = y_0;
    let mut y_buffer_0: real;
    for _ in 0..n {
        y_buffer_0 = y_buffer;
        y_buffer += h*f(x_buffer,y_buffer);
        y_buffer = y_buffer_0 + (h/2.0)*(f(x_buffer,y_buffer_0) + f(x_buffer+h,y_buffer));
        x_buffer += h;
    }
    println!("y = {y_buffer}");
    println!("galat = |11,71875 - {y_buffer}| = {}\n", (EXACT - y_buffer).abs());
    return y_buffer;
}

fn main() {
    let x_0 = 0 as real;
    let y_0 = 1 as real;
    let x = 1.4;

    println!("\n\nPERCOBAAN 1:");
    let n = 28;
    euler(n, x, x_0, y_0);
    heun(n, x, x_0, y_0);

    println!("\n\nPERCOBAAN 2:");
    let n = 100;
    euler(n, x, x_0, y_0);
    heun(n, x, x_0, y_0);

    println!("\n\nPERCOBAAN 3:");
    let n = 1000;
    euler(n, x, x_0, y_0);
    heun(n, x, x_0, y_0);
}
