use std::fs::File;                       // import tipe File untuk membuka file
use std::io::{self, Read};               // import modul io (untuk io::Error) dan trait Read (untuk read_to_string)

fn main () {                             // entry point program, dieksekusi pertama kali saat program jalan
    fn read_username_from_file() -> Result<String, io::Error> {  // fungsi tanpa parameter, mengembalikan username atau error I/O
        let username_file_result = File::open("hello.txt");      // coba buka hello.txt, hasilnya Result<File, io::Error>

        let mut username_file = match username_file_result {     // cocokkan hasil membuka file, mut dibutuhkan untuk membaca
            Ok(file) => file,                                    // berhasil: ambil File-nya
            Err(e) => return Err(e),   // teruskan error ke pemanggil  // gagal: hentikan fungsi dan kembalikan error
        };                                                       // akhir let dan match

        let mut username = String::new();                        // buat String kosong yang bisa diisi isi file

        match username_file.read_to_string(&mut username) {      // baca seluruh isi file (UTF-8) ke username, hasilnya Result<usize, io::Error>
            Ok(_) => Ok(username),                               // berhasil: abaikan jumlah byte, kembalikan username
            Err(e) => Err(e),                                    // gagal membaca: kembalikan error ke pemanggil
        }                                                        // nilai match ini adalah nilai kembalian fungsi
    }                                                            // akhir fungsi pertama

    fn read_username_from_file() -> Result<String, io::Error> {  // fungsi kedua dengan nama sama (ini error: nama duplikat dalam scope yang sama)
        let mut username_file = File::open("hello.txt")?;        // buka hello.txt, jika gagal langsung return error lewat operator ?
        let mut username = String::new();                        // buat String kosong untuk menampung isi file
        username_file.read_to_string(&mut username)?;            // baca isi file ke username, jika gagal langsung return error lewat ?
        Ok(username)                                             // berhasil: kembalikan username dalam Ok
    }                                                            // akhir fungsi kedua
}                                                                // akhir fungsi main
