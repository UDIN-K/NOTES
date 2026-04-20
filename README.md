# Stellar Notes (Soroban Smart Contract)

#contract ID
CDJZHI2FM6TUJEZHRXW3BJANGXD74OLAR6ZXFBSUNINXBTZA7TI6W7FL

Smart contract notes/todo sederhana di Stellar Soroban, ditulis dengan Rust.

## Ringkasan

Contract ini menyimpan data catatan/todo di on-chain storage dan menyediakan operasi utama:

- Menambah catatan
- Mengambil semua catatan
- Mengubah status selesai/belum selesai
- Menghapus catatan

Setiap catatan memiliki `id`, `content`, dan status `completed`.

## Struktur Proyek

```text
.
├── Cargo.toml
└── contracts
    └── notes
        ├── Cargo.toml
        ├── Makefile
        └── src
            ├── lib.rs
            └── test.rs
```

## Fitur Contract

1. `add_note(env, content) -> String`
   Menambah catatan baru dengan ID auto-increment (`1, 2, 3, ...`) dan status awal belum selesai.

2. `get_notes(env) -> Vec<Note>`
   Mengambil semua catatan dari storage.

3. `toggle_note(env, id) -> String`
   Mengubah status `completed` dari `false` ke `true` (atau sebaliknya).

4. `delete_note(env, id) -> String`
   Menghapus catatan berdasarkan ID.

## Struktur Data

```rust
pub struct Note {
    pub id: u64,
   pub content: String,
   pub completed: bool,
}
```

## Prasyarat

- Rust toolchain
- `cargo`
- Stellar CLI (`stellar`)

## Menjalankan Project

Di root repository:

```bash
make -C contracts/notes build
```

Menjalankan test:

```bash
make -C contracts/notes test
```

Format code:

```bash
make -C contracts/notes fmt
```

## Catatan Implementasi

- Data catatan disimpan dengan key instance storage: `NOTE_DATA`.
- Counter ID disimpan dengan key `NEXT_ID` agar ID konsisten dan tidak random.
- Saat catatan dihapus, ID tidak didaur ulang (tetap monotonic).

## Rencana Lanjutan (Opsional)

- Menambahkan validasi input (`content` tidak kosong)
- Menambahkan event log untuk setiap aksi
- Menambahkan test unit yang lengkap di `test.rs`