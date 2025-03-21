# tutorial6



# Dokumentasi `handle_connection` dalam Rust

## Fungsi `handle_connection`

Fungsi `handle_connection` bertanggung jawab untuk menangani koneksi masuk dari klien yang terhubung ke server TCP. Fungsi ini membaca permintaan HTTP yang dikirim oleh klien dan mencetaknya ke terminal.

### Implementasi Fungsi

```rust
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    
    println!("Request: {:#?}", http_request);
}
```


# Commit 2
## Screenshot

![commit2](images/commit2.png)


# Commit 3 Reflection Notes

##  **Kode Program Setelah Refactoring**
Kode telah diperbaiki dengan memisahkan tanggung jawab **membangun respons HTTP** dalam fungsi terpisah, sehingga lebih rapi dan mudah dikelola.

```rust
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("Server berjalan di http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

// Fungsi utama untuk menangani koneksi HTTP
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = if request_line.contains("GET / ") {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let response = build_response(status_line, filename);
    stream.write_all(response.as_bytes()).unwrap();
}

// Fungsi untuk membangun respons HTTP
fn build_response(status_line: &str, filename: &str) -> String {
    let contents = fs::read_to_string(filename).unwrap_or_else(|_| {
        "<h1>404 - Halaman Tidak Ditemukan</h1><p>Silakan periksa kembali URL yang Anda masukkan.</p>".to_string()
    });

    let length = contents.len();
    format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}")
}

```
##  **Mengapa Perlu Refactoring?**
Refactoring dilakukan untuk **memisahkan tanggung jawab fungsi** agar kode lebih mudah dibaca dan dipelihara. Berikut adalah alasan utama mengapa refactoring diperlukan:

###  **1. Modularitas**
- Dengan **memisahkan fungsi `build_response()`**, kode menjadi lebih **terstruktur** dan **mudah dikembangkan**.
- Jika ingin menambahkan fitur baru, kita hanya perlu memperbarui fungsi yang relevan **tanpa mengubah keseluruhan kode**.

### **2. Pemeliharaan yang Lebih Mudah**
- Jika ada perubahan pada **format respons HTTP**, kita hanya perlu **mengedit satu fungsi**, yaitu `build_response()`, tanpa menyentuh `handle_connection()`.
- Dengan cara ini, **pemeliharaan kode lebih mudah** dan mengurangi risiko kesalahan.

### **3. Menghindari Duplikasi Kode**
- Tanpa refactoring, kita harus menulis ulang format respons di beberapa tempat.
- Dengan **memisahkan logika respons ke dalam fungsi `build_response()`**, kita **menghindari pengulangan kode** dan **menjaga efisiensi**.

![commit3](images/commit3.png)


# Commit 4 Reflection notes

## Mengapa Server Mengalami Delay Saat Mengakses `/sleep`

### 1. Penjelasan
Saat membuka dua jendela browser:
- **Jendela 1:** Akses `127.0.0.1/sleep`
- **Jendela 2:** Akses `127.0.0.1/`

Jendela pertama akan mengalami delay selama **10 detik** sebelum mendapatkan respons dari server. Sementara itu, jendela kedua juga akan mengalami keterlambatan dalam memuat halaman utama (`127.0.0.1`). Hal ini disebabkan oleh cara kerja server dalam menangani koneksi yang masuk.

### 2. Penyebab Delay
Server saat ini menangani koneksi secara **sekuensial (single-threaded)**. Ini berarti:
- Saat request ke `/sleep` diterima, server akan **menunda eksekusi selama 10 detik** menggunakan `thread::sleep(Duration::from_secs(10))`.
- Selama proses penundaan ini, server tidak bisa memproses request lain yang masuk.
- Akibatnya, request ke `127.0.0.1/` yang datang setelah `/sleep` harus menunggu hingga eksekusi sebelumnya selesai.

### 3. Dampak Jika Banyak Pengguna Mengakses Secara Bersamaan
Jika banyak pengguna mencoba mengakses `/sleep`, maka semua permintaan lainnya juga akan mengalami antrean yang panjang karena server hanya dapat menangani **satu koneksi dalam satu waktu**.

Dampak yang dapat terjadi:
- **Latensi tinggi**: Pengguna harus menunggu lama untuk mendapatkan respons.
- **Pengalaman pengguna buruk**: Website terasa lambat dan tidak responsif.
- **Server menjadi bottleneck**: Semakin banyak request, semakin panjang antrean dan semakin lambat server merespons.

### 4. Solusi Potensial
Untuk meningkatkan performa dan menghindari antrean panjang, beberapa solusi yang dapat diterapkan:
- **Menggunakan multi-threading atau async programming**
  - Bisa menggunakan **thread pool** (contoh: `std::thread::spawn()` atau library seperti `tokio` untuk async processing)
  - Dengan ini, setiap koneksi bisa ditangani secara paralel tanpa menunggu request lain selesai
- **Menggunakan load balancer**
  - Jika server sudah digunakan dalam skala besar, load balancer dapat membantu mendistribusikan permintaan ke beberapa instance server
- **Membatasi akses ke endpoint `/sleep`**
  - Bisa menggunakan rate-limiting agar tidak ada penyalahgunaan yang menyebabkan server overload

### 5. Kesimpulan
Server saat ini menggunakan pendekatan single-threaded sehingga tidak dapat menangani request secara paralel. Akibatnya, request yang memerlukan waktu lama (seperti `/sleep`) akan menghambat request lain. Untuk mengatasi ini, pendekatan multi-threading atau asynchronous perlu diterapkan agar server tetap responsif dan dapat menangani banyak pengguna secara bersamaan.

# Commit 5 multithreaded server using threadpool

## **Mengapa Menggunakan Multithreading?**
Ketika sebuah server menerima banyak permintaan dari pengguna, server harus menangani setiap permintaan tanpa membuat pengguna lain menunggu. Jika server hanya berjalan dalam satu thread (**single-threaded**), maka:

- **Setiap permintaan diproses satu per satu**, sehingga permintaan yang lebih lambat (misalnya, akses ke `/sleep` yang butuh 10 detik) akan membuat permintaan lain ikut terhambat.
- **Tidak dapat menangani banyak pengguna sekaligus**, karena setiap koneksi harus menunggu hingga koneksi sebelumnya selesai.

Solusi dari masalah ini adalah **multithreading**, yang memungkinkan beberapa koneksi diproses secara bersamaan menggunakan beberapa thread.

## **Apa Itu Thread Pool?**
**Thread pool** adalah teknik dalam multithreading yang menggunakan **sejumlah thread tetap (worker threads)** untuk menangani tugas secara **bergiliran**. Ketika sebuah koneksi masuk:

1. Server akan menugaskan koneksi tersebut ke salah satu thread yang tersedia di dalam **thread pool**.
2. Jika semua thread sedang sibuk, koneksi akan **menunggu** hingga ada thread yang selesai dan siap menerima tugas baru.
3. Dengan cara ini, server tidak perlu **membuat thread baru setiap kali ada koneksi baru**, sehingga lebih **efisien** dalam penggunaan sumber daya.

## **Mengapa Menggunakan Thread Pool?**
Menggunakan thread pool memiliki beberapa keuntungan dibandingkan dengan hanya menggunakan multithreading biasa:

 **Menghemat sumber daya**: Thread dibuat sekali dan digunakan ulang, sehingga tidak ada overhead dalam pembuatan dan penghancuran thread.  
 **Lebih cepat**: Mengurangi waktu pemrosesan karena thread sudah siap digunakan.  
 **Lebih scalable**: Dapat menangani lebih banyak koneksi tanpa membebani sistem.  
 **Mencegah crash akibat terlalu banyak thread**: Dengan membatasi jumlah thread, server tidak akan kehabisan memori atau mengalami overhead CPU karena terlalu banyak thread aktif.
