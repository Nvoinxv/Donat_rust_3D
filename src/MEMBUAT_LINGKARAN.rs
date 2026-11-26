mod MATRIX_ROTASI_3D;

use MATRIX_ROTASI_3D::data_matrix_3d;

// MEMBUAT CONST PI
const PI : f64 = 3.141592653589793238462;
const SUDUT_SETENGAH : f64 = 180.0;

// MEMBUAT STRUCT VEKTOR
pub struct VektorData3D {
    x : f64,
    y : f64,
    z : f64,
}

// MEMBUAT STRUCT VARIABEL DONAT
pub struct DonatVariabel3D {
    VektorData3D {
        R : f64,
        r : f64,
        u : f64,
        v : f64
    }
}

// SEGMEN U DAN V
pub struct SegmenVariabelDonat {
    let segmen_u : i64 = 30,
    let segmen_v : i64 = 15,
}

pub struct MembuatDonat3D;

impl MembuatDonat3D {
    pub fn konversi_theta(theta : f64, pemilihan : bool) -> f64 {
        let mut hasil_perhitungan : f64 = 0.0;

        // JIKA USER MEMILIH TRUE
        // MAKA KONVERSI DARI SUDUT KE RADIAN
        if pemilihan == true {
            let penyebut_radian : f64 = PI / SUDUT_SETENGAH;

            if penyebut_radian == 0.0 {
                println!("TIDAK BISA DIBAGI DENGAN NOL!");
                return penyebut_radian;
            } else {
                hasil_perhitungan = theta / penyebut_radian;
            }

            // JIKA TIDAK MAKA
            // USER MEMILIH RADIAN KE SUDUT
        } else {
            let penyebut_theta : f64 = SUDUT_SETENGAH / PI;

            if penyebut_theta == 0.0 {
                println!("TIDAK BISA DIBAGI DENGAN NOL!");
                return penyebut_theta;
            } else {
                hasil_perhitungan = theta / penyebut_theta;
            }
        }

        return hasil_perhitungan;
    }

    pub fn akses_indeks(u : i64,
        v : i64) -> i64 {
        if segmen_u < 0  && segmen_v < 0 {
            println!("TIDAK BISA NEGATIF!");
            return 0;
        }

        if u < 0 && v < 0 {
            println!("TIDAK BISA NEGATIF!");
            return 0;
        }

        u = segmen_u % u;
        v = segmen_v % v;

        return u * segmen_v + v;
    }

    pub fn koordinat_donat_3d(R : f64, r : f64,
        u : f64, v : f64) -> VektorData3D {
        VektorData3D {
            x : (R + r * v.cos()) + u.cos(),
            y : (R + r * v.cos()) + u.sin(),
            z : r * v.sin(),
        }
    }

    pub fn penggabungan_torus(R : f64, r : f64, 
        segmen_u : i64, segmen_v : i64) {
        // JADI KITA MEMANGGIL SUATU VEKTOR 
        // BERTIPE DATA STRUCT DONAT VARIABEL
        // AGAR BISA DIMASUKKAN
        // HASIL PERHITUNGAN KOORDINAT
        // KE DALAM SATU VEKTOR VARIABEL
    }

    pub fn transformasi_linear(m : data_matrix_3d, 
        v : VektorData3D) -> VektorData3D {
        return {
            [m.matrix_3d[0][0] * v.x + m.matrix_3d[0][1] * v.y + m.matrix_3d[0][2] * v.z],
            [m.matrix_3d[1][0] * v.x + m.matrix_3d[1][1] * v.y + m.matrix_3d[1][2] * v.z],
            [m.matrix_3d[2][0] * v.x + m.matrix_3d[2][1] * v.y + m.matrix_3d[2][2] * v.z],
        }
    }

    pub fn rotasi_geometris_donat(theta : f64, 
        pemilihan_rotasi : bool) {
        theta = konversi_theta(theta, pemilihan_rotasi);
    }
}
