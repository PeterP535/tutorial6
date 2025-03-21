# tutorial6



1. Dokumentasi `handle_connection` dalam Rust

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
