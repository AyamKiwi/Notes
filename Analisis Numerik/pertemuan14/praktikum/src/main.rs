use f64 as float;
use i32 as int;

fn f(x: float, y: float) -> float {
    return x.powi(2)*y.powi(2);
}

fn galat(y: float) {
    println!("Galat = {}", 11.7233 - y);
}

fn orde1(x: float, x0: float, y0: float, n: int) -> float {
    let h = (x - x0)/n as float;
    let mut y = y0;
    let mut x = x0;
    let mut k = h*f(x, y);
    for _ in 0..n {
        y += k;
        x += h;
        k = h*f(x, y);
    }
    println!("\nOrde 1:");
    println!("h = {h} \ny = {y}");
    galat(y);
    return y;
}

fn orde2_heun (x: float, x0: float, y0: float, n: int) -> float {

    let a1 = 0.5;
    let a2 = 0.5;
    let p = 1 as float;
    let q = 1 as float;

    let h = (x - x0)/n as float;

    let mut y = y0;
    let mut x = x0;

    let mut k1 = h*f(x,y);
    let mut k2 = h*f(x + p*h, y + q*k1);

    for _ in 0..n {
        y += a1*k1 + a2*k2;
        x += h;
        k1 = h*f(x,y);
        k2 = h*f(x + p*h, y + q*k1);
    }
    println!("\nOrde 2 Heun:");
    println!("h = {h} \ny = {y}");
    galat(y);
    return y;
}

fn orde2_raltson (x: float, x0: float, y0: float, n: int) -> float {

    let a1 = 1.0/3.0;
    let a2 = 2.0/3.0;
    let p = 3.0/4.0;
    let q = 3.0/4.0;

    let h = (x - x0)/n as float;

    let mut y = y0;
    let mut x = x0;

    let mut k1 = h*f(x,y);
    let mut k2 = h*f(x + p*h, y + q*k1);

    for _ in 0..n {
        y += a1*k1 + a2*k2;
        x += h;
        k1 = h*f(x,y);
        k2 = h*f(x + p*h, y + q*k1);
    }
    println!("\nOrde 2 Raltson");
    println!("h = {h} \ny = {y}");
    galat(y);
    return y;
}

fn orde3(x: float, x0: float, y0: float, n: int) -> float {
    let a0 = 1.0/6.0;
    let a1 = 2.0/3.0;
    let a2 = 1.0/6.0;
    let p = (0.5, 1.0);
    let q = (0.5, (-1.0, 2.0));

    let h = (x - x0)/n as float;

    let mut y = y0;
    let mut x = x0;

    let mut k0 = h*f(x,y);
    let mut k1 = h*f(x + p.0*h, y + q.0*k0);
    let mut k2 = h*f(x + p.1*h, y + q.1.0*k0 + q.1.1*k1);

    for _ in 0..n {
        y += a0*k0 + a1*k1 + a2*k2;
        x += h;
        k0 = h*f(x,y);
        k1 = h*f(x + p.0*h, y + q.0*k0);
        k2 = h*f(x + p.1*h, y + q.1.0*k0 + q.1.1*k1);
    }
    println!("\nOrde 3:");
    println!("h = {h} \ny = {y}");
    galat(y);
    return y;
}

fn orde4(x: float, x0: float, y0: float, n: int) -> float {
    let a0 = 1.0/6.0;
    let a1 = 1.0/3.0;
    let a2 = 1.0/3.0;
    let a3 = 1.0/6.0;
    let p = (0.5, 0.5, 1.0);
    let q = (0.5, (0.0, 0.5), (0.0, 0.0, 1.0));
    
    let h = (x - x0)/n as float;

    let mut y = y0;
    let mut x = x0;

    let mut k0 = h*f(x,y);
    let mut k1 = h*f(x + p.0*h, y + q.0*k0);
    let mut k2 = h*f(x + p.1*h, y + q.1.0*k0 + q.1.1*k1);
    let mut k3 = h*f(x + p.2*h, y + q.2.0*k0 + q.2.1*k1 + q.2.2*k2);

    for _ in 0..n {
        y += a0*k0 + a1*k1 + a2*k2 + a3*k3;
        x += h;
        k0 = h*f(x,y);
        k1 = h*f(x + p.0*h, y + q.0*k0);
        k2 = h*f(x + p.1*h, y + q.1.0*k0 + q.1.1*k1);
        k3 = h*f(x + p.2*h, y + q.2.0*k0 + q.2.1*k1 + q.2.2*k2);
    }
    println!("\nOrde 4:");
    println!("h = {h} \ny = {y}");
    galat(y);
    return y;
}

fn main() {
    let x0 = 0 as float;
    let y0 = 1 as float;
    let x = 1.4;
    let n = 14;
    // orde1(x, x0, y0, n);
    orde2_heun(x, x0, y0, n);
    orde2_raltson(x, x0, y0, n);
    orde3(x, x0, y0, n);
    // orde4(x, x0, y0, n);
    println!();

    let n = 140;
    // orde1(x, x0, y0, n);
    orde2_heun(x, x0, y0, n);
    orde2_raltson(x, x0, y0, n);
    orde3(x, x0, y0, n);
    // orde4(x, x0, y0, n);
    println!();

    let n = 1400;
    // orde1(x, x0, y0, n);
    orde2_heun(x, x0, y0, n);
    orde2_raltson(x, x0, y0, n);
    orde3(x, x0, y0, n);
    // orde4(x, x0, y0, n);
    println!();
}
