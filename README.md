# Stellar Notes (Soroban Smart Contract)

# Contract ID
CDJZHI2FM6TUJEZHRXW3BJANGXD74OLAR6ZXFBSUNINXBTZA7TI6W7FL

A simple notes/todo smart contract on Stellar Soroban, written in Rust.

## Overview

This contract stores note/todo data in on-chain storage and provides the following main operations:

- Add a note
- Get all notes
- Toggle completion status
- Delete a note

Each note has an `id`, `content`, and a `completed` status.

## Project Structure

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

## Contract Features

1. `add_note(env, content) -> String`
   Adds a new note with an auto-incrementing ID (`1, 2, 3, ...`) and an initial incomplete status.

2. `get_notes(env) -> Vec<Note>`
   Retrieves all notes from storage.

3. `toggle_note(env, id) -> String`
   Toggles the `completed` status from `false` to `true` (or vice versa).

4. `delete_note(env, id) -> String`
   Deletes a note by its ID.

## Data Structure

```rust
pub struct Note {
    pub id: u64,
    pub content: String,
    pub completed: bool,
}
```

## Prerequisites

- Rust toolchain
- `cargo`
- Stellar CLI (`stellar`)

## Running the Project

In the repository root:

```bash
make -C contracts/notes build
```

Run tests:

```bash
make -C contracts/notes test
```

Format code:

```bash
make -C contracts/notes fmt
```

## Implementation Notes

- Note data is stored using the instance storage key: `NOTE_DATA`.
- The ID counter is stored with the key `NEXT_ID` to keep IDs consistent and not random.
- When a note is deleted, its ID is not recycled (remains monotonic).

## Future Plans (Optional)

- Add input validation (`content` cannot be empty)
- Add event logs for every action
- Add comprehensive unit tests in `test.rs`
- 
