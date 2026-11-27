use crate::MATRIX_ROTASI_3D;
use crate::MEMBUAT_DONAT;
use crate::STRUKTUR_VEKTOR_3D;

pub use crate::STRUKTUR_VEKTOR_3D::VektorData3D;
pub use crate::MEMBUAT_DONAT::MembuatDonat3D;

use std::cmp::{min, max};

// KARAKTER DIURUTKAN DARI YANG PALING DEKAT (TEBAL)
// SAMPAI YANG PALING JAUH (TIPIS)
pub const CHAR_ASCII: [char; 8] = ['@', '#', '%', '+', '=', '-', ':', '.'];

// JARAK KAMERA KE PUSAT DONAT
// HARUS LEBIH BESAR DARI R + r
// AGAR Z TIDAK PERNAH NOL ATAU NEGATIF
pub const JARAK_KAMERA: f64 = 6.0;

// BATAS KEDALAMAN Z DONAT (NILAI R + r)
// DIPAKAI UNTUK MEMILIH KARAKTER ASCII
pub const BATAS_KEDALAMAN: f64 = 3.0;

// KARAKTER TERMINAL ITU TINGGINYA KIRA-KIRA
// 2 KALI LEBARNYA, JADI SUMBU Y DIKECILKAN
// AGAR DONAT TIDAK TERLIHAT LONJONG
pub const RASIO_KARAKTER: f64 = 0.5;

pub struct Ascii_Variabel {
    pub hasil_dda: Vec<VektorData3D>,
    pub skala: f64,
}

pub struct render_ascii_donat {
    pub VariabelAscii: Ascii_Variabel,
}

impl render_ascii_donat {
    pub fn proyeksi_algoritma(a: VektorData3D, mut f: f64) -> VektorData3D {
        // X screen harus menggunakan
        // persamaan seperti ini
        // x_screen = f * x / z
        // begitu juga dengan y_screen

        // Z DITAMBAH JARAK KAMERA DULU
        // KARENA DONAT BERADA DI TITIK (0, 0, 0)
        let z_kamera: f64 = a.z + JARAK_KAMERA;

        if z_kamera <= 0.0 {
            println!("Z TIDAK BOLEH NOL!");
            return a;
        }

        let x_screen: f64 = f * a.x / z_kamera;
        let y_screen: f64 = f * a.y / z_kamera;

        return VektorData3D {
            x: x_screen,
            y: y_screen,
            z: a.z,
        };
    }

    pub fn algoritma_dda(
        a: VektorData3D,
        b: VektorData3D,
    ) -> Vec<VektorData3D> {
        let delta_x: f64 = b.x - a.x;
        let delta_y: f64 = b.y - a.y;

        let delta_x_abs: f64 = delta_x.abs();
        let delta_y_abs: f64 = delta_y.abs();

        let langkah_maksimum: f64 = delta_x_abs.max(delta_y_abs);

        let mut hasil_dda: Vec<VektorData3D> = Vec::new();

        if langkah_maksimum == 0.0 {
            hasil_dda.push(a);
            return hasil_dda;
        }

        let turunan_x: f64 = delta_x / langkah_maksimum;
        let turunan_y: f64 = delta_y / langkah_maksimum;

        let mut x: f64 = a.x;
        let mut y: f64 = a.y;

        // UBAH MENJADI MENJADI INTERGER POSITIF
        // AGAR BISA MEMBUAT GARIS
        for i in 0..=langkah_maksimum as i64 {
            hasil_dda.push(VektorData3D {
                x,
                y,
                z: a.z + (b.z - a.z) * (i as f64 / langkah_maksimum),
            });

            x += turunan_x;
            y += turunan_y;
        }

        return hasil_dda;
    }

    pub fn kedalaman_titik(titik: &VektorData3D) -> f64 {
        return titik.z;
    }

    pub fn karakter_kedalaman(kedalaman: f64) -> char {
        // UBAH KEDALAMAN DARI RENTANG
        // [-BATAS, +BATAS] MENJADI [0, 1]
        let mut nilai: f64 = (kedalaman + BATAS_KEDALAMAN) / (2.0 * BATAS_KEDALAMAN);

        nilai = nilai.max(0.0).min(1.0);

        let indeks: usize = (nilai * 7.0) as usize;

        return CHAR_ASCII[indeks];
    }

    pub fn gambar_titik(
        layar: &mut Vec<Vec<char>>,
        layar_kedalaman: &mut Vec<Vec<f64>>,
        titik: VektorData3D,
        panjang: i64,
        tinggi: i64,
    ) {
        // DIBULATKAN (round) AGAR TITIK
        // TIDAK MENUMPUK DI TENGAH LAYAR
        let nilai_x: i64 = panjang / 2 + (titik.x.round() as i64);
        let nilai_y: i64 = tinggi / 2 + ((titik.y * RASIO_KARAKTER).round() as i64);

        let nilai_kedalaman_z: f64 = Self::kedalaman_titik(&titik);

        if nilai_x >= 0
            && nilai_x < panjang
            && nilai_y >= 0
            && nilai_y < tinggi
        {
            // Z-BUFFER: HANYA GAMBAR JIKA TITIK INI
            // LEBIH DEKAT DARI TITIK YANG SUDAH ADA
            // AGAR BAGIAN BELAKANG TIDAK MENIMPA DEPAN
            if nilai_kedalaman_z < layar_kedalaman[nilai_y as usize][nilai_x as usize] {
                layar_kedalaman[nilai_y as usize][nilai_x as usize] = nilai_kedalaman_z;

                layar[nilai_y as usize][nilai_x as usize] =
                    Self::karakter_kedalaman(nilai_kedalaman_z);
            }
        }
    }

    pub fn gambar_garis(
        layar: &mut Vec<Vec<char>>,
        layar_kedalaman: &mut Vec<Vec<f64>>,
        a: VektorData3D,
        b: VektorData3D,
        panjang: i64,
        tinggi: i64,
    ) {
        let titik_garis: Vec<VektorData3D> = Self::algoritma_dda(a, b);

        for titik_data in titik_garis {
            Self::gambar_titik(
                layar,
                layar_kedalaman,
                titik_data,
                panjang,
                tinggi,
            );
        }
    }

    pub fn render_layar(
        &self,
        objek: &MembuatDonat3D,
        panjang: i64,
        tinggi: i64,
    ) {
        let mut layar: Vec<Vec<char>> = vec![
            vec![' '; panjang as usize];
            tinggi as usize
        ];

        // LAYAR KEDALAMAN (Z-BUFFER)
        // AWALNYA DIISI TAK HINGGA
        let mut layar_kedalaman: Vec<Vec<f64>> = vec![
            vec![f64::INFINITY; panjang as usize];
            tinggi as usize
        ];

        let nilai_titik = objek.mendapatkan_nilai_vertices();
        let nilai_segmen_u = objek.mendapatkan_nilai_segmen_u();
        let nilai_segmen_v = objek.mendapatkan_nilai_segmen_v();

        let mut titik_sekarang: i64 = 0;
        let mut titik_v: i64 = 0;
        let mut titik_u: i64 = 0;

        for i in 0..nilai_segmen_u {
            for j in 0..nilai_segmen_v {
                titik_sekarang = i * nilai_segmen_v + j;

                // TETANGGA KE ARAH V (LINGKARAN KECIL)
                titik_v = i * nilai_segmen_v
                    + ((j + 1) % nilai_segmen_v);

                // TETANGGA KE ARAH U (LINGKARAN BESAR)
                titik_u = ((i + 1) % nilai_segmen_u)
                    * nilai_segmen_v + j;

                let a: VektorData3D = Self::proyeksi_algoritma(
                    <VektorData3D as Clone>::clone(&nilai_titik[titik_sekarang as usize]),
                    self.VariabelAscii.skala,
                );

                let b: VektorData3D = Self::proyeksi_algoritma(
                    <VektorData3D as Clone>::clone(&nilai_titik[titik_u as usize]),
                    self.VariabelAscii.skala,
                );

                let c: VektorData3D = Self::proyeksi_algoritma(
                    <VektorData3D as Clone>::clone(&nilai_titik[titik_v as usize]),
                    self.VariabelAscii.skala,
                );

                Self::gambar_garis(
                    &mut layar,
                    &mut layar_kedalaman,
                    a.clone(),
                    b,
                    panjang,
                    tinggi,
                );

                Self::gambar_garis(
                    &mut layar,
                    &mut layar_kedalaman,
                    a,
                    c,
                    panjang,
                    tinggi,
                );
            }
        }

        // KUMPULKAN SATU FRAME PENUH KE DALAM SATU STRING
        // LALU CETAK SEKALIGUS. KALAU DICETAK PER BARIS
        // (println!), TERMINAL MENAMPILKAN FRAME SETENGAH JADI
        // SEHINGGA GAMBAR TERLIHAT BERKEDIP DAN PATAH-PATAH
        let mut hasil_frame: String = String::with_capacity(
            ((panjang + 1) * tinggi) as usize
        );

        for baris_data in layar {
            for karakter in baris_data {
                hasil_frame.push(karakter);
            }

            hasil_frame.push('\n');
        }

        print!("{}", hasil_frame);
        std::io::Write::flush(&mut std::io::stdout()).expect("OUTPUT HARUS TERKIRIM");
    }
}
