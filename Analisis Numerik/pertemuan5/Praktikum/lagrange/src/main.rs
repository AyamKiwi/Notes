// Library yang digunakan:
//      std::io - Mengelola input-output dengan user
//      std::fs - Mengelola pembacaan file
use std::io;
use std::fs;

// Macro: Input user ke variable konstan
#[allow(unused_macros)]
macro_rules! read {
    ($var_name : ident, $typ : ty, $msg : expr) =>  {
        println!();
        println!($msg);
        let mut $var_name = String::new();
        io::stdin().read_line(&mut $var_name).expect("Failed");
        let $var_name : $typ = $var_name.trim().parse().expect("Failed");
    };
}

// Macro: Input user ke variable mutable
#[allow(unused_macros)]
macro_rules! readmut {
    ($var_name : ident, $typ : ty, $msg : expr) => {
        println!();
        println!($msg);
        let mut $var_name = String::new();
        io::stdin().read_line(&mut $var_name).expect("Failed");
        let mut $var_name : $typ = $var_name.trim().parse().expect("Failed");
    };
}

// Struktur data 'Tabel'
//      x - array of floating
//      y - array of floating
pub struct Tabel {
    pub x : Vec<f64>,
    pub y : Vec<f64>
}

// Implementasi prosedur dan fungsi bagi struktur data 'Tabel'
impl Tabel {
    
    // Prosedur membaca data dari data.txt ke variabel bertipe 'Tabel'
    fn read () -> Tabel {
        let mut buffer = Tabel{
            x : Vec::new(),
            y : Vec::new()
        };
        let text = fs::read_to_string("./data.txt").expect("Unable to read file");
        let mut temp : Vec<&str>;
        for line in text.lines(){
            temp = line.split(' ').collect();
            buffer.x.push(temp[0].trim().parse().expect(""));
            buffer.y.push(temp[1].trim().parse().expect(""));
        }
        return buffer;

    }

    // Fungsi mencari dan mengevaluasi nilai polinom lagrange di x bedasarakan variabel 'Tabel'
    fn lagrange (&self, x : f64) -> f64{
        let mut buffer : f64;
        let mut temp : f64;
        buffer = 0.0;
        for i in 0..self.x.len() {
            temp = 1.0;
            for j in 0..self.x.len(){
                if i != j {
                    temp = temp * (x - self.x[j])/(self.x[i] - self.x[j]);
                };
            };
            buffer = buffer + (self.y[i] * temp);
        };
        return buffer;
    }
}

// Titik entri
fn main() {
    let data = Tabel::read();            // Membaca data dari data.txt ke variabel "data"
    read!(x, f64, "x = ");                      // Membaca user input ke variabel "x"
    println!("f(x) = {}", data.lagrange(x));    // Output nilai polinom lagrange di "x"
}