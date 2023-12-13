use std::io;
use std::fs::*;

struct Problem{
    ukuran : usize,
    koefisien : Vec<Vec<f64>>,
    konstan : Vec<f64>,
    solusi : Vec<f64>,
    unstable : bool
}

impl Problem {
    fn init() -> Problem {
        let file_koef = read_to_string("./koef.txt").unwrap();
        let file_konst = read_to_string("./konst.txt").unwrap();
        if file_koef.lines().count() != file_konst.lines().count(){
            panic!();
        }
        let mut buffer = Problem{
            ukuran : file_konst.lines().count(),
            koefisien : Vec::new(),
            konstan : Vec::new(),
            solusi : Vec::new(),
            unstable : true
        };
        let mut temp: Vec<&str>;
        let mut koef_buffer: Vec<f64>;
        for i in 0..buffer.ukuran{
            buffer.konstan.push(file_konst.lines().nth(i).unwrap().trim().parse().expect("Parsing failed"));
            temp = file_koef.lines().nth(i).unwrap().split(' ').collect();
            if temp.len() != buffer.ukuran{
                panic!("Not a square matrix")
            }
            koef_buffer = Vec::new();
            for value in temp{
                koef_buffer.push(value.trim().parse().expect("Parsing failed"))
            }
            buffer.koefisien.push(koef_buffer);
            buffer.solusi.push(0.0);
        }
        return buffer;
    }

    fn eval (&self, index: usize) -> f64 {
        let mut sum = 0.0;
        for i in 0..self.ukuran{
            sum += self.solusi[i] * self.koefisien[index][i];
        }
        return sum;
    }

    fn dominan_diagonal(&mut self) {
        let mut buffer = self.koefisien.clone();
        let mut temp = self.konstan.clone();
        let mut max: f64;
        for i in 0..self.ukuran {
            max = 0.0;
            for line in &buffer {
                if (line[i]).abs() > max.abs() {
                    max = line[i];
                };
            };
            let mut j = 0;
            while j < buffer.len() && buffer[j][i] != max {
                j += 1;
            }
            self.koefisien[i] = buffer[j].clone();
            self.konstan[i] = temp[j];
            buffer.remove(j);
            temp.remove(j);
        }
    }

    fn iteras_jacobi(&mut self, threshold: f64, accuracy: f64) {
        self.unstable = false;
        let buffer = self.solusi.clone();
        for i in 0..self.ukuran{
            self.solusi[i] = self.konstan[i];
            for j in 0..self.ukuran{
                if i != j {
                    self.solusi[i] -= self.koefisien[i][j]*buffer[j];
                }
            }
            self.solusi[i] /= self.koefisien[i][i]; 
            if (self.solusi[i] - buffer[i]).abs() > threshold || (self.eval(i) - self.konstan[i]).abs() > accuracy {
                self.unstable = true;
            }
        }
    }

    fn gauss_seidel(&mut self, threshold: f64, accuracy: f64) {
        self.unstable = false;
        let buffer = self.solusi.clone();
        for i in 0..self.ukuran{
            self.solusi[i] = self.konstan[i];
            for j in 0..self.ukuran{
                if i != j {
                    self.solusi[i] -= self.koefisien[i][j]*self.solusi[j];
                }
            }
            self.solusi[i] /= self.koefisien[i][i]; 
            if (self.solusi[i] - buffer[i]).abs() > threshold || (self.eval(i) - self.konstan[i]).abs() > accuracy {
                self.unstable = true;
            }
        }
    }
}

macro_rules! read {
    ($var_name : ident, $typ : ty, $msg : expr) =>  {
        println!();
        println!($msg);
        let mut $var_name = String::new();
        io::stdin()
            .read_line(&mut $var_name)
            .expect("Failed");
        let $var_name : $typ = $var_name.trim().parse().expect("Failed");
    };
}

macro_rules! readmut {
    ($var_name : ident, $typ : ty, $msg : expr) => {
        println!();
        println!($msg);
        let mut $var_name = String::new();
        io::stdin()
            .read_line(&mut $var_name)
            .expect("Failed");
        let mut $var_name : $typ = $var_name.trim().parse().expect("Failed");
    };
}

fn main() {
    read!(galat_iterasi, f64, "Galat iterasi = ");
    read!(galat_akurasi, f64, "Galat mutlak = "); 
    read!(max_iter, usize, "Batas iteras = ");

    println!("\n\nMetode Iterasi Jacobi:");
    let mut spl = Problem::init();
    spl.dominan_diagonal();
    let mut iterasi = 0;
    while spl.unstable && iterasi < max_iter{
        spl.iteras_jacobi(galat_iterasi, galat_akurasi);
        iterasi += 1;
    }
    println!();
    for i in 0..spl.ukuran {
        println!("x{} = {}", i+1, spl.solusi[i]);
    }
    println!();
    for i in 0..spl.ukuran {
        println!("Persamaan {} = {}\n Dengan galat: {}", i+1, spl.eval(i), (spl.eval(i) - spl.konstan[i]).abs());
    }
    println!("\n Jumlah total iteras: {iterasi}");

    println!("\n\nMetode Gauss-Seidel");
    spl = Problem::init();
    spl.dominan_diagonal();
    iterasi = 0;
    while spl.unstable && iterasi < max_iter{
        spl.gauss_seidel(galat_iterasi, galat_akurasi);
        iterasi += 1;
    }
    println!();
    for i in 0..spl.ukuran {
        println!("x{} = {}", i+1, spl.solusi[i]);
    }
    println!();
    for i in 0..spl.ukuran {
        println!("Persamaan {} = {}\n Dengan galat: {}", i+1, spl.eval(i), (spl.eval(i) - spl.konstan[i]).abs());
    }
    println!("\n Jumlah total iteras: {iterasi}");
}