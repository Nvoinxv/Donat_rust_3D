// IMPORT FILE PENTING 
mod MATRIX_ROTASI_3D;

// PANGGIL KELAS DAN STRUKTUR DATA
use MATRIX_ROTASI_3D::operasi_rotasi_matrix;
use MATRIX_ROTASI_3D::data_matrix_3d;

// PANGGIL FUNGSI
use MATRIX_ROTASI_3D::print_matrix_3d;

// IMPORT STD 
use std::io;

fn main() {
    // TESTING MATRIX ROTASI
    println!("MASUKKAN INPUT SUDUT: ");
    
    let mut input_theta = String::new();

    io::stdin().read_line(&mut input_theta).expect("INPUT HARUS TERBACA!");

    let theta : f64 = input_theta.trim().parse().expect("INPUT HARUS TIPE FLOAT NUMERIC!");
    
    // ROTASI X
    let hasil_x : data_matrix_3d = operasi_rotasi_matrix::rotasi_x(&theta);
    // ROTASI Y
    let hasil_y : data_matrix_3d = operasi_rotasi_matrix::rotasi_y(&theta);
    // ROTASI Z
    let hasil_z : data_matrix_3d = operasi_rotasi_matrix::rotasi_z(&theta);

    // MENAMPILKAN HASIL
    println!("HASIL ROTASI MATRIX X: ");
    print_matrix_3d(&hasil_x);

    println!("HASIL ROTASI MATRIX Y: ");
    print_matrix_3d(&hasil_y);

    println!("HASIL ROTASI MATRIX Z: ");
    print_matrix_3d(&hasil_z);
}
