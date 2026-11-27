// IMPORT FILE DARI PROJEK
use crate::MATRIX_ROTASI_3D::data_matrix_3d;
use crate::MATRIX_ROTASI_3D::operasi_rotasi_matrix;
use crate::STRUKTUR_VEKTOR_3D::VektorData3D;

// MEMBUAT CONST PI
const PI : f64 = 3.141592653589793238462;
const SUDUT_SETENGAH : f64 = 180.0;

// SEGMEN U DAN V
pub struct SegmenVariabelDonat {
    pub segmen_u : i64,
    pub segmen_v : i64,
    pub vertex : Vec<VektorData3D>
}

pub struct MembuatDonat3D {
    pub VektorVariabelDonat3D : SegmenVariabelDonat
}

impl MembuatDonat3D {
    pub fn konversi_theta(theta : f64, pemilihan : bool) -> f64 {
        let mut hasil_perhitungan : f64 = 0.0;

        // JIKA USER MEMILIH TRUE
        // MAKA KONVERSI DARI SUDUT KE RADIAN
        // RUMUS: RADIAN = SUDUT * PI / 180
        // JADI PENYEBUTNYA ADALAH 180 / PI
        if pemilihan == true {
            let penyebut_radian : f64 = SUDUT_SETENGAH / PI;

            if penyebut_radian == 0.0 {
                println!("TIDAK BISA DIBAGI DENGAN NOL!");
                return penyebut_radian;
            } else {
                hasil_perhitungan = theta / penyebut_radian;
            }

            // JIKA TIDAK MAKA
            // USER MEMILIH RADIAN KE SUDUT
            // RUMUS: SUDUT = RADIAN * 180 / PI
            // JADI PENYEBUTNYA ADALAH PI / 180
        } else {
            let penyebut_theta : f64 = PI / SUDUT_SETENGAH;

            if penyebut_theta == 0.0 {
                println!("TIDAK BISA DIBAGI DENGAN NOL!");
                return penyebut_theta;
            } else {
                hasil_perhitungan = theta / penyebut_theta;
            }
        }

        hasil_perhitungan
    }

    pub fn akses_indeks(&self, mut u : i64,
        mut v : i64) -> i64 {
        // SEGMEN TIDAK BOLEH NOL ATAU NEGATIF
        // KARENA DIPAKAI SEBAGAI PEMBAGI MODULO
        if self.VektorVariabelDonat3D.segmen_u <= 0 || self.VektorVariabelDonat3D.segmen_v <= 0 {
            println!("TIDAK BISA NEGATIF!");
            return 0;
        }

        if u < 0 || v < 0 {
            println!("TIDAK BISA NEGATIF!");
            return 0;
        }

        // INDEKS YANG DI-MODULO OLEH SEGMEN
        // AGAR INDEKS TERAKHIR KEMBALI KE 0
        u = u % self.VektorVariabelDonat3D.segmen_u;
        v = v % self.VektorVariabelDonat3D.segmen_v;

        return u * self.VektorVariabelDonat3D.segmen_v + v;
    }

    pub fn koordinat_donat_3d(R : f64, r : f64,
        u : f64, v : f64) -> VektorData3D {
        // PERSAMAAN PARAMETRIK TORUS
        // x = (R + r * cos v) * cos u
        // y = (R + r * cos v) * sin u
        // z = r * sin v
        VektorData3D {
            x : (R + r * v.cos()) * u.cos(),
            y : (R + r * v.cos()) * u.sin(),
            z : r * v.sin(),
        }
    }

    pub fn penggabungan_torus(&mut self, R : f64, r : f64,
        segmen_u : i64, segmen_v : i64) {
        // JADI KITA MEMANGGIL SUATU VEKTOR
        // BERTIPE DATA STRUCT DONAT VARIABEL
        // AGAR BISA DIMASUKKAN
        // HASIL PERHITUNGAN KOORDINAT
        // KE DALAM SATU VEKTOR VARIABEL
        let mut u : f64 = 0.0;
        let mut v : f64 = 0.0;

        for i in 0..segmen_u {
            for j in 0..segmen_v {

                if segmen_u == 0 {
                    break;
                }

                u = 2.0 * (i as f64) * PI / (segmen_u as f64);

                if segmen_v == 0 {
                    break;
                }

                v = 2.0 * (j as f64) * PI / (segmen_v as f64);

                let koordinat_vertex : VektorData3D = Self::koordinat_donat_3d(
                    R, r,
                    u, v);

                self.VektorVariabelDonat3D.vertex.push(koordinat_vertex);
            }
        }
    }

    pub fn transformasi_linear(m : &data_matrix_3d,
        v : VektorData3D) -> VektorData3D {
        VektorData3D {
            x: m.matrix_3d[0][0] * v.x + m.matrix_3d[0][1] * v.y + m.matrix_3d[0][2] * v.z,
            y: m.matrix_3d[1][0] * v.x + m.matrix_3d[1][1] * v.y + m.matrix_3d[1][2] * v.z,
            z: m.matrix_3d[2][0] * v.x + m.matrix_3d[2][1] * v.y + m.matrix_3d[2][2] * v.z,
        }
    }

    pub fn rotasi_geometris_donat(&mut self, mut theta : f64,
        pemilihan_rotasi : bool,
        mode_rotasi_koordinat : i64) {
        theta = Self::konversi_theta(theta, pemilihan_rotasi);

        // TAMBAHKAN GABUNGAN TORUS
        // LALU, KITA WAJIB MELAKUKAN LOOP
        // TERUTAMA VARIABEL GABUNGAN TORUS
        // AGAR BISA DIGUNAKAN DALAM TRANSFORMASI LINEAR

        // PILIH ROTASI, KITA AKAN MENGGUNAKAN ROTASI
        // JIKA MEMILIH 1 MAKA MAIN ROTASI X
        // JIKA MEMILIH 2 MAKA MAIN ROTASI Y
        // JIKA MEMILIH 3 MAKA MAIN ROTASI Z

        // HASIL TRANSFORMASI WAJIB DISIMPAN KEMBALI
        // KE DALAM VERTEX (iter_mut), KALAU TIDAK
        // DONAT TIDAK AKAN PERNAH BERPUTAR
        // THETA DI SINI ADALAH SUDUT PER FRAME
        // JADI ROTASI AKAN TERAKUMULASI SETIAP LOOP

        if (mode_rotasi_koordinat == 1) {
            let rotasi_posisi_x : data_matrix_3d = operasi_rotasi_matrix::rotasi_x(&theta);

            for (idx, titik) in (self.VektorVariabelDonat3D.vertex.iter_mut().enumerate()) {
                let hasil_x : VektorData3D  = Self::transformasi_linear(
                    &rotasi_posisi_x, titik.clone());

                *titik = hasil_x;
            }
        } else if (mode_rotasi_koordinat == 2) {
            let rotasi_posisi_y : data_matrix_3d = operasi_rotasi_matrix::rotasi_y(&theta);

            for (idx, titik) in (self.VektorVariabelDonat3D.vertex.iter_mut().enumerate()) {
                let hasil_y : VektorData3D = Self::transformasi_linear(
                    &rotasi_posisi_y, titik.clone());

                *titik = hasil_y;
            }
        } else if (mode_rotasi_koordinat == 3) {
            let rotasi_posisi_z : data_matrix_3d = operasi_rotasi_matrix::rotasi_z(&theta);

            for (idx, titik) in (self.VektorVariabelDonat3D.vertex.iter_mut().enumerate()) {
                 let hasil_z : VektorData3D = Self::transformasi_linear(
                      &rotasi_posisi_z, titik.clone());

                 *titik = hasil_z;
            }
        }
    }

    // MENDAPATKAN NILAI DARI VARIABEL
    // BERGUNA UNTUK IMPLEMENTASI
    // DI BAGIAN RENDERING ASCII
    pub fn mendapatkan_nilai_vertices(&self) -> Vec<VektorData3D> {
        return self.VektorVariabelDonat3D.vertex.clone();
    }

    pub fn mendapatkan_nilai_segmen_u(&self) -> i64 {
        return self.VektorVariabelDonat3D.segmen_u;
    }

    pub fn mendapatkan_nilai_segmen_v(&self) -> i64 {
        return self.VektorVariabelDonat3D.segmen_v;
    }
}
