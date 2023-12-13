use std::ops::Range;

fn f(x: f64) -> f64 {
    // return x*(x.powi(2)).cos();
    return x.powi(2);
}

fn trapesium(range: Range<f64>) -> f64 {
    let a = range.start;
    let b = range.end;
    let h = b - a;
    println!("h = {h}");
    return (h/2 as f64)*(f(a) + f(b));
}

fn komposisi_trapesium(range: Range<f64>, n:usize) -> f64 {
    let mut buffer = 0 as f64;
    let a = range.start;
    let b = range.end;
    let h = (b - a)/n as f64;
    let mut temp: f64;
    for i in 0..n {
        temp = i as f64 * h + a;
        buffer += trapesium(temp..temp + h);
    }
    return buffer;
}

fn simpson_thirds(range: Range<f64>) -> f64 {
    let x0 = range.start;
    let x2 = range.end;
    let x1 = (x0 + x2)/2 as f64;
    let h = x1 - x0;
    println!("h = {h}");
    return (h/3 as f64)*(f(x0) + 4 as f64 * f(x1) + f(x2));
}

fn simpson_three_eighths(range: Range<f64>) -> f64 {
    let x0 = range.start;
    let x3 = range.end;
    let h = (x3 - x0)/3 as f64;
    let x1 = x0 + h;
    let x2 = x3 - h;
    println!("h = {h}");
    return (3 as f64 * h / 8 as f64)*(f(x0) + 3 as f64 * (f(x1) + f(x2)) + f(x3));
}

fn kuadratur_gauss_three(range: Range<f64>) -> f64 {
    let a = range.start;
    let b = range.end;
    let adder = (a + b)/2 as f64;
    let multiplier = (b - a)/2 as f64;
    let c = [5.0/9.0, 8.0/9.0, 5.0/9.0];
    let x = [-(3 as f64/5 as f64).sqrt(), 0.0, (3 as f64/5 as f64).sqrt()];
    return multiplier*(c[0]*f(adder + multiplier*x[0]) + c[1]*f(adder + multiplier*x[1]) + c[2]*f(adder + multiplier*x[2]));
}

fn komposisi_three_eighths(range: Range<f64>, n:usize) -> f64 {
    let a = range.start;
    let b = range.end;
    let h = (b - a)/n as f64;
    let mut buffer = 0 as f64;
    let mut temp: f64;
    for i in 0..n {
        temp = i as f64 * h + a;
        buffer += simpson_three_eighths(temp..temp+h);
    }
    return buffer;
}

fn main() {
    // println!("Kaidah Trapesium = {}\nGalat = {}", trapesium(1.5..2.5), (-0.405626206717739 - trapesium(1.5..2.5)).abs());
    // println!("Komposisi Trapesium = {}\n", komposisi_trapesium(1.5..2.5, 1000));
    // println!("Simpson 1/3 = {}\n", simpson_thirds(1.5..2.5));
    println!("Simpson 3/8 = {}\nGalat = {}", simpson_three_eighths(1.5..2.5), (-0.405626206717739 - simpson_three_eighths(1.5..2.5)).abs());
    println!("Kuadratur Gauss = {}\n", kuadratur_gauss_three(0.0..20.0));
    println!("Komposisi Simpson 3/8 = {}\nGalat = {}", komposisi_three_eighths(1.5..2.5, 3), (-0.405626206717739 - komposisi_three_eighths(1.5..2.5,3)).abs());
    
}
