// PANGGIL FILE MODULAR LAIN
mod MEMBUAT_DONAT;

// PANGGIL FUNGSI, KELAS, dan STRUCT
pub MEMBUAT_DONAT::MembuatDonat3D;
pub MEMBUAT_DONAT::VektorData3D;

// PEMANGGILAN STANDARD LIBARY
use std::cmp::{min, max};

pub const CHAR_ASCII : [8, char] = '/!%#+^>%';

pub struct Ascii_Variabel {
    hasil_dda : Vec<VektorData3D>,
}

pub struct render_ascii_donat;

impl render_ascii_donat {
    pub fn projeksi_algoritma(a : VektorData3D, mut f : f64) -> VektorData3D {
        // X screen harus menggunakan 
        // persamaan seperti ini
        // x_screen = f * x / z 
        // begitu juga dengan y_screen
        
        if a.z == 0 {
            println!("Z TIDAK BOLEH NOL!");
            return a;
        }

        let x_screen : f64 = f * a.x / a.z;
        let y_screen : f64 = f * a.y / a.z;

        return {
            x_screen,
            y_screen,
            a.z
        };
    }

    pub fn algoritma_dda(a : VektorData3D, 
        b : VektorData3D) -> Vec<VektorData3D> {
        let delta_x : f64 = b.x - a.x;
        let delta_y : f64 = b.y - b.y;

        let delta_x_abs : f64 = delta_x.abs();
        let delta_y_abs : f64 = delta_y.abs();

        let langkah_maksimum : f64 = max(a, b);

        if langkah_maksimum == 0.0 {
            hasil_dda.push(langkah_maksimum);
            return hasil_dda;
        } 

        let turunan_x : f64 = delta_x / langkah_maksimum;
        let turunan_y : f64 = delta_y / langkah_maksimum;

        let x : i64 = a.x;
        let y : i64 = a.y;

        // UBAH MENJADI MENJADI INTERGER POSITIF
        // AGAR BISA MEMBUAT GARIS
        for i in 0..=langkah_maksimum as i64 {
            hasil_dda.push(
                x, y,
                a.z + (b.z - a.z) *
                (i / langkah_maksimum));

            x += turunan_x;
            y += turunan_y;
        }
    }

    pub fn kedalaman_titik(titik : VektorData3D) -> f64 {
        return titik.z;
    }

    pub fn karakter_kedalaman(kedalaman : f64) {
        let nilai : f64 = (kedalaman + 10.0) / 20.0;

        nilai = max(0.0, min(1.0, nilai));

        let indeks : i64 = nilai * 8;

        return CHAR_ASCII[indeks];
    }

    pub fn gambar_titik(
        layar : Vec<Vec<char>>,
        titik : VektorData3D,
        panjang : i64,
        tinggi : i64) {

    }
}
