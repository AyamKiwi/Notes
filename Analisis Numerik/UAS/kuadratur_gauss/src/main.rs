use std::ops::Range;
use f64 as float;

fn v(t: float) -> float {
    return (7 as float)/(1 as float + (-0.02*(t - 90 as float)).exp());
}

fn kuadratur_gauss_2(range: Range<float>) -> float {
    let a = range.start;
    let b = range.end;

    let c = [
        1 as float,
        1 as float
    ];
    let x = [
        -0.577350269,
        0.577350269
    ];

    let mut buffer = 0 as float;
    for i in 0..2 {
        buffer += c[i] * (b-a) * 0.5 * v(0.5*((a + b) + (b - a)*x[i]));
    }

    println!("
        Kuadratur-Gauss 2 titik
        \t{buffer}
    ");
    return buffer;
}

fn kuadratur_gauss_3(range: Range<float>) -> float {
    let a = range.start;
    let b = range.end;

    let c = [
        0.555555556,
        0.888888889,
        0.555555556
    ];
    let x = [
        -0.774596669,
        0 as float,
        0.774596669
    ];

    let mut buffer = 0 as float;
    for i in 0..3 {
        buffer += c[i] * (b-a) * 0.5 * v(0.5*((a + b) + (b - a)*x[i]));
    }

    println!("
        Kuadratur-Gauss 3 titik
        \t{buffer}
    ");
    return buffer;
}

fn kuadratur_gauss_4(range: Range<float>) -> float {
    let a = range.start;
    let b = range.end;

    let c = [
        0.347854845,
        0.652145155,
        0.652145155,
        0.347854845
    ];
    let x = [
        -0.861136312,
        -0.339981044,
        0.339981044,
        0.861136312
    ];

    let mut buffer = 0 as float;
    for i in 0..4 {
        buffer += c[i] * (b-a) * 0.5 *v(0.5*((a + b) + (b - a)*x[i]));
    }

    println!("
        Kuadratur-Gauss 4 titik
        \t{buffer}
    ");
    return buffer;
}

fn kuadratur_gauss_5(range: Range<float>) -> float {
    let a = range.start;
    let b = range.end;

    let c = [
        0.236926885,
        0.478628670,
        0.568888889,
        0.478628670,
        0.236926885
    ];
    let x = [
        -0.906179846,
        -0.538469310,
        0 as float,
        0.538469310,
        0.906179846
    ];

    let mut buffer = 0 as float;
    for i in 0..5 {
        buffer += c[i] * (b-a) * 0.5 * v(0.5*((a + b) + (b - a)*x[i]));
    }

    println!("
        Kuadratur-Gauss 5 titik
        \t{buffer}
    ");
    return buffer;
}

fn main() {
    let range = 0 as float .. 200 as float;
    kuadratur_gauss_2(range.clone());
    kuadratur_gauss_3(range.clone());
    kuadratur_gauss_4(range.clone());
    kuadratur_gauss_5(range.clone());
}
