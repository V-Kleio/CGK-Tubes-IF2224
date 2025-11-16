# CGK-Tubes-IF2224

# A Pascal-S Compiler

### Identitas Kelompok

| Nama                 | NIM      |
| -------------------- | -------- |
| Andi Farhan Hidayat  | 13523128 |
| Andri Nurdianto      | 13523145 |
| Rafael Marchel D. W. | 13523146 |
| Muhammad Kinan A.    | 13523152 |

### Deskripsi

Lexical Analyzer untuk bahasa Pascal-S. Lexical analyzer memanfaatkan DFA dalam perancangannya dan bahasa Rust untuk implementasinya. Output program yang dijalankan adalah source code yang diubah menjadi token-token dan disimpan dalam format .txt

### Requirements

- Rust

### Cara Instalasi dan Penggunaaan Program

#### Instalasi

Windows

- download rustup.exe

UNIX

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Direkomendasikan untuk run di sistem UNIX

#### Compile & Run

```
cargo run input.pas output.txt
```

Ubah `input.pas` dan `output.txt` sesuai kebutuhan

Contoh penggunaan:

```
cargo run test/milestone-1/input-1.pas test/milestone-1/output-1.txt
```

### Pembagian Tugas M1

| NIM      | TUGAS                                             |
| -------- | ------------------------------------------------- |
| 13523128 | Diagram, Laporan                                  |
| 13523145 | Diagram, main, dfa_rules, input & output, Laporan |
| 13523146 | dfa_rules, dfa, lexer, token, main, Laporan       |
| 13523152 | Diagram, Laporan                                  |

### Pembagian Tugas M2

| NIM      | TUGAS                                             |
| -------- | ------------------------------------------------- |
| 13523128 | Parser dan laporan                                |
| 13523145 | Parser, test, dan laporan                         |
| 13523146 | Laporan                                           |
| 13523152 | Parser, test, dan laporan                         |
