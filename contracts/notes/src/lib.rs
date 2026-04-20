#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data catatan/todo
#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    pub id: u64,
    pub content: String,
    pub completed: bool,
}

// Storage key
const NOTE_DATA: Symbol = symbol_short!("NOTE_DATA");
const NEXT_ID: Symbol = symbol_short!("NEXT_ID");

#[contract]
pub struct NotesContract;

#[contractimpl]
impl NotesContract {
    fn load_notes(env: &Env) -> Vec<Note> {
        env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(env))
    }

    fn save_notes(env: &Env, notes: &Vec<Note>) {
        env.storage().instance().set(&NOTE_DATA, notes);
    }

    fn load_next_id(env: &Env) -> u64 {
        env.storage().instance().get(&NEXT_ID).unwrap_or(1)
    }

    fn save_next_id(env: &Env, next_id: u64) {
        env.storage().instance().set(&NEXT_ID, &next_id);
    }

    // Ambil semua catatan
    pub fn get_notes(env: Env) -> Vec<Note> {
        Self::load_notes(&env)
    }

    // Tambah catatan
    pub fn add_note(env: Env, content: String) -> String {
        let mut notes: Vec<Note> = Self::load_notes(&env);
        let id = Self::load_next_id(&env);

        let note = Note {
            id,
            content,
            completed: false,
        };

        notes.push_back(note);
        Self::save_notes(&env, &notes);
        Self::save_next_id(&env, id + 1);

        String::from_str(&env, "Catatan berhasil ditambahkan")
    }

    // Hapus catatan
    pub fn delete_note(env: Env, id: u64) -> String {
        let mut notes: Vec<Note> = Self::load_notes(&env);

        for i in 0..notes.len() {
            if notes.get(i).unwrap().id == id {
                notes.remove(i);
                Self::save_notes(&env, &notes);
                return String::from_str(&env, "Catatan berhasil dihapus");
            }
        }

        String::from_str(&env, "Catatan tidak ditemukan")
    }

    // Toggle status todo (done <-> pending)
    pub fn toggle_note(env: Env, id: u64) -> String {
        let mut notes: Vec<Note> = Self::load_notes(&env);

        for i in 0..notes.len() {
            let mut note = notes.get(i).unwrap();

            if note.id == id {
                note.completed = !note.completed;
                notes.set(i, note);
                Self::save_notes(&env, &notes);
                return String::from_str(&env, "Status catatan berhasil diubah");
            }
        }

        String::from_str(&env, "Catatan tidak ditemukan")
    }
}

mod test;