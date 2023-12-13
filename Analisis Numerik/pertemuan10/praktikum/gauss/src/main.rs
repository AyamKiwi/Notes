// Library yang digunakan:
//      std::fs -> membaca file
use std::fs::*;

// Tipe data "Persamaan"
struct Persamaan{
    matrix : Vec<Vec<f64>>,
    vector : Vec<f64>,
    size : usize
}

// Fungsi terhadap tipe data "Persamaan"
impl Persamaan {

    // Membaca file "koef.txt" dan "konst.txt" ke variabel "Persamaan" baru
    fn read() -> Persamaan{
        let koef = read_to_string("./koef.txt").unwrap();
        let kosnt = read_to_string("./konst.txt").unwrap();
        if koef.lines().count() != kosnt.lines().count(){
            panic!("Difference in Matrix size and Vector size");
        }
        let mut buffer = Persamaan{
            matrix : Vec::new(),
            vector : Vec::new(),
            size : koef.lines().count()
        };
        let mut temp : Vec<&str>;
        let mut matrix_buffer : Vec<f64>;
        for i in 0..buffer.size{
            buffer.vector.push(kosnt.lines().nth(i).unwrap().trim().parse().expect("Failed to parse"));
            temp = koef.lines().nth(i).unwrap().split(' ').collect();
            if temp.len() != buffer.size{
                panic!("");
            }
            matrix_buffer = Vec::new();
            for value in temp{
                matrix_buffer.push(value.trim().parse().expect("Failed to parse"));
            }
            buffer.matrix.push(matrix_buffer);
        }
        return buffer;
    }

    // Eliminasi Gauss naif tanpa pivoting parsial:
    //      Mengubah matriks pada variabel "Persamaan"
    //      menjadi matriks segitiga atas
    fn gauss_naif(&mut self){
        let mut multiplier : f64;
        for i in 1..self.size{
            for j in i..self.size{
                multiplier = self.matrix[j][i-1]/self.matrix[i-1][i-1];
                self.vector[j] = self.vector[j] - self.vector[i-1] * multiplier;
                for k in 0..self.size{
                    self.matrix[j][k] = self.matrix[j][k] - self.matrix[i-1][k] * multiplier;
                }
            }
        }
    }

    // Fungsi pivoting pasrial
    fn pivot(&mut self, index: usize){
        for i in index+1..self.size{
            if self.matrix[i][index].abs() > self.matrix[index][index].abs(){
                self.matrix.swap(index, i);
                self.vector.swap(index, i);
            }
        }
    }

    // Elminiasi Gauss naif dengan pivoting parsial
    fn gauss_pivot(&mut self){
        let mut multiplier : f64;
        for i in 1..self.size{
            self.pivot(i-1);
            for j in i..self.size{
                multiplier = self.matrix[j][i-1]/self.matrix[i-1][i-1];
                self.vector[j] = self.vector[j] - self.vector[i-1] * multiplier;
                for k in 0..self.size{
                    self.matrix[j][k] = self.matrix[j][k] - self.matrix[i-1][k] * multiplier;
                }
            }
        }
    }

    // Perhitungan nilai solusi bedasarkan matriks segitiga atas
    fn solve(&self) -> Vec<f64> {
        let max_index = self.size-1;
        let mut buffer = Vec::new();
        buffer.push(self.vector[max_index]/self.matrix[max_index][max_index]);
        let mut sum : f64;
        for i in (0..max_index).rev(){
            sum = 0.0;
            for j in (i + 1 ..= max_index).rev() {
                sum = sum + (self.matrix[i][j] * buffer[max_index - j]);
            }
            buffer.push((self.vector[i] - sum) / self.matrix[i][i])
        }
        buffer.reverse();
        return buffer;
    }
}

// Fungsi validasi solusi
fn validate(hasil: Vec<f64>){
    println!("Dengan:");
    let mut eval : f64;
    let mut result = true;
    for i in 0..Persamaan::read().size{
        eval = 0.0;
        for j in 0..hasil.len(){
            eval = eval + Persamaan::read().matrix[i][j]*hasil[j];
        }
        println!("Persamaan {} = {eval}", i+1);
        result = eval == Persamaan::read().vector[i];
    }
    match result {
        true => println!("Kesimpulan: Pass!"),
        false => println!("Kesimpulan: Error!")
    }
}

// Entry-point
fn main() {

    // Perhitunagn menggunakan Gauss naif
    println!("\nGauss Naif:");
    let mut persamaan = Persamaan::read();
    persamaan.gauss_naif();
    let hasil_naif = persamaan.solve();
    let idk = ["a", "b", "c"];
    for i in 0..hasil_naif.len(){
        println!("{} = {}", idk[i], hasil_naif[i]);
    }
    // validate(hasil_naif);

    // Perhtingan menggunakan Gauss pivoting parsial
    println!("\nGauss Pivot Parsial");
    persamaan = Persamaan::read();
    persamaan.gauss_pivot();
    let hasil_pivot = persamaan.solve();
    for i in 0..hasil_pivot.len(){
        println!("{} = {}", idk[i], hasil_pivot[i]);
    }
    // validate(hasil_pivot);
}