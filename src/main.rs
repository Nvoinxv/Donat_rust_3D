// IMPORT FILE PENTING 
mod MATRIX_ROTASI_3D;
mod MEMBUAT_DONAT;
mod RENDER_ASCII;
mod STRUKTUR_VEKTOR_3D;

// PANGGIL KELAS DAN STRUKTUR DATA
use MATRIX_ROTASI_3D::operasi_rotasi_matrix;
use MATRIX_ROTASI_3D::data_matrix_3d;

use MEMBUAT_DONAT::MembuatDonat3D;
use MEMBUAT_DONAT::SegmenVariabelDonat;
use STRUKTUR_VEKTOR_3D::VektorData3D;

use RENDER_ASCII::render_ascii_donat;
use RENDER_ASCII::Ascii_Variabel;

// PANGGIL FUNGSI
use MATRIX_ROTASI_3D::print_matrix_3d;

// IMPORT STD 
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    // TESTING MATRIX ROTASI
    /*
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
    */

    // MENAMPILKAN HASIL
    /*
    println!("HASIL ROTASI MATRIX X: ");
    print_matrix_3d(&hasil_x);

    println!("HASIL ROTASI MATRIX Y: ");
    print_matrix_3d(&hasil_y);

    println!("HASIL ROTASI MATRIX Z: ");
    print_matrix_3d(&hasil_z);
    */

    // MEMULAI ROTASI DONAT
    println!("MEMULAI ROTASI DONAT 3D");
    println!("MASUKKAN INPUT ROTASI: ");

    let mut input_rotasi =  String::new();

    io::stdin().read_line(&mut input_rotasi).expect("INPUT HARUS TERBACA");
    let theta : f64 = input_rotasi.trim().parse().expect("INPUT HARUS NUMERIC FLOAT");
    
    println!("PEMULIHAN KONVERSI SUDUT");
    println!("true : sudut ke radian");
    println!("false : radian ke sudut");
    println!("MASUKKAN PEMILIHAN KONVERSI SUDUT (true/false): ");
    let mut input_konversi = String::new();

    io::stdin().read_line(&mut input_konversi).expect("INPUT HARUS TERBACA");
    let konversi : bool = input_konversi.trim().parse().expect("INPUT HARUS BOOLEAN true/false");

    println!("PEMILIHAN ROTASI OBJEK");
    println!("1 : ROTASI X");
    println!("2 : ROTASI Y");
    println!("3 : ROTASI Z");
    println!("MASUKKAN PEMILIHAN ROTASI: ");
    let mut input_pemilihan_rotasi = String::new();

    io::stdin().read_line(&mut input_pemilihan_rotasi).expect("INPUT HARUS TERBACA");
    let pemilihan_rotasi : i64 = input_pemilihan_rotasi.trim().parse().expect("INPUT HARUS INTERGER");

    println!("MASUKKAN INPUT PANJANG LAYAR: ");
    let mut input_panjang_layar = String::new();

    io::stdin().read_line(&mut input_panjang_layar).expect("INPUT HARUS TERBACA");
    let panjang : i64 = input_panjang_layar.trim().parse().expect("INPUT HARUS INTERGER");

    println!("MASUKKAN INPUT TINGGI LAYAR: ");
    let mut input_tinggi_layar = String::new();

    io::stdin().read_line(&mut input_tinggi_layar).expect("INPUT HARUS TERBACA");
    let tinggi : i64 = input_tinggi_layar.trim().parse().expect("INPUT HARUS INTERGER");

    // MEMBUAT OBJEK DONAT
    // (NILAI SEGMEN, R, r, DAN SKALA SILAKAN DIUBAH)
    let segmen_u : i64 = 30;
    let segmen_v : i64 = 15;

    let mut objek_donat : MembuatDonat3D = MembuatDonat3D {
        VektorVariabelDonat3D : SegmenVariabelDonat {
            segmen_u : segmen_u,
            segmen_v : segmen_v,
            vertex : Vec::new(),
        },
    };

    // ISI VERTEX DULU, KALAU TIDAK VEKTOR KOSONG
    objek_donat.penggabungan_torus(2.0, 1.0, segmen_u, segmen_v);

    // MEMBUAT OBJEK RENDER
    let objek_render : render_ascii_donat = render_ascii_donat {
        VariabelAscii : Ascii_Variabel {
            hasil_dda : Vec::new(),
            // SKALA MENGIKUTI PANJANG LAYAR
            // AGAR DONAT MEMENUHI LAYAR
            skala : (panjang as f64) * 0.6,
        },
    };

    // BERSIHKAN LAYAR CUKUP SEKALI SAJA DI AWAL
    // DAN SEMBUNYIKAN KURSOR AGAR TIDAK BERKEDIP
    print!("\x1b[2J\x1b[?25l");

    while (true) {
        // JANGAN BERSIHKAN LAYAR (\x1b[2J) SETIAP FRAME
        // KARENA LAYAR SEMPAT KOSONG SEBELUM FRAME BARU MUNCUL
        // CUKUP KEMBALIKAN KURSOR KE POJOK KIRI ATAS
        // LALU TIMPA FRAME LAMA DENGAN FRAME BARU
        print!("\x1b[H");

        // BAGIAN KODE ANSI (KALAU BISA RGB BIAR HASIL BAGUS)
        print!("\x1b[38;2;255;0;0m");

        objek_donat.rotasi_geometris_donat(theta,
            konversi,
            pemilihan_rotasi);

        objek_render.render_layar(&objek_donat,
            panjang, tinggi);

        print!("\x1b[0m\n");

        // MENAMBAHKAN BAGIAN CHRONO
        // AGAR BISA RUNNING DENGAN
        // OTOMATIS YANG MEMBUAT OBJEK
        // BERGERAK DENGAN LANCAR
        thread::sleep(Duration::from_millis(80));
    }
}
