// MEMBUAT MATRIX ROTASI 3D
// BERGUNA UNTUK MELAKUKAN OPERASI
// PERHITUNGAN AGAR OBJEK BISA BERGERAK

pub struct data_matrix_3d {
    pub matrix_3d : [[f64; 3]; 3],
}

// KELAS OPERASI MATRIX BISA DIGUNAKAN
// SECARA MODULAR
pub struct operasi_rotasi_matrix;

impl operasi_rotasi_matrix {
    pub fn rotasi_x(theta : &f64) -> data_matrix_3d {
        data_matrix_3d {
            matrix_3d: [
                [1.0, 0.0, 0.0],
                [0.0, theta.cos(), -(theta.sin())],
                [0.0, theta.sin(), theta.cos()],
            ],
        }
    }


    pub fn rotasi_y(theta : &f64) -> data_matrix_3d {
        data_matrix_3d {
            matrix_3d: [
                [theta.cos(), 0.0, theta.sin()],
                [0.0, 1.0, 0.0],
                [-(theta.sin()), 0.0, theta.cos()],
            ],
        }
    }

    pub fn rotasi_z(theta : &f64) -> data_matrix_3d {
        data_matrix_3d {
            matrix_3d: [
                [theta.cos(), -(theta.sin()), 0.0],
                [theta.sin(), theta.cos(), 0.0],
                [0.0, 0.0, 1.0],
            ],
        }
    }
}

// FUNGSI UTILS
pub fn print_matrix_3d(data_matrix : &data_matrix_3d) {
    for i in data_matrix.matrix_3d {
        for (idx, j) in i.iter().enumerate() {
            print!("{}", j);
        }

        println!();
    }

    println!();
}

