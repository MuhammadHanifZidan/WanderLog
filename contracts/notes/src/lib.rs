#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Enum untuk mood perjalanan
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Mood {
    Amazing,
    Good,
    Neutral,
    Tired,
    Difficult,
}

// Struct utama untuk menyimpan data log perjalanan
#[contracttype]
#[derive(Clone, Debug)]
pub struct TravelLog {
    pub id: u64,
    pub destination: String,   // Nama lokasi/destinasi
    pub story: String,         // Cerita/pengalaman perjalanan
    pub date: String,          // Tanggal perjalanan (format: YYYY-MM-DD)
    pub mood: Mood,            // Mood saat perjalanan
    pub rating: u32,           // Rating 1-5
}

// Storage keys
const TRAVEL_DATA: Symbol = symbol_short!("TRVL_DATA");
const TOTAL_LOGS: Symbol = symbol_short!("TOTAL");

#[contract]
pub struct WanderLogContract;

#[contractimpl]
impl WanderLogContract {
    
    // Ambil semua travel logs
    pub fn get_all_logs(env: Env) -> Vec<TravelLog> {
        env.storage()
            .instance()
            .get(&TRAVEL_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // Ambil travel log berdasarkan ID
    pub fn get_log_by_id(env: Env, id: u64) -> Option<TravelLog> {
        let logs: Vec<TravelLog> = env
            .storage()
            .instance()
            .get(&TRAVEL_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..logs.len() {
            let log = logs.get(i).unwrap();
            if log.id == id {
                return Some(log);
            }
        }
        None
    }

    // Buat travel log baru
    pub fn create_log(
        env: Env,
        destination: String,
        story: String,
        date: String,
        mood: Mood,
        rating: u32,
    ) -> u64 {
        // Validasi rating harus antara 1-5
        if rating < 1 || rating > 5 {
            panic!("Rating harus antara 1 sampai 5");
        }

        // Ambil data yang sudah ada
        let mut logs: Vec<TravelLog> = env
            .storage()
            .instance()
            .get(&TRAVEL_DATA)
            .unwrap_or(Vec::new(&env));

        // Generate ID unik menggunakan timestamp + random
        let new_id = env.prng().gen::<u64>();

        // Buat travel log baru
        let travel_log = TravelLog {
            id: new_id,
            destination,
            story,
            date,
            mood,
            rating,
        };

        // Tambahkan ke list
        logs.push_back(travel_log);

        // Simpan ke storage
        env.storage().instance().set(&TRAVEL_DATA, &logs);

        // Update counter total logs
        let total: u64 = env
            .storage()
            .instance()
            .get(&TOTAL_LOGS)
            .unwrap_or(0u64);
        env.storage()
            .instance()
            .set(&TOTAL_LOGS, &(total + 1));

        // Kembalikan ID yang baru dibuat
        new_id
    }

    // Update mood dari travel log yang sudah ada
    pub fn update_mood(env: Env, id: u64, new_mood: Mood) -> String {
        let mut logs: Vec<TravelLog> = env
            .storage()
            .instance()
            .get(&TRAVEL_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..logs.len() {
            let mut log = logs.get(i).unwrap();
            if log.id == id {
                log.mood = new_mood;
                logs.set(i, log);
                env.storage().instance().set(&TRAVEL_DATA, &logs);
                return String::from_str(&env, "Mood berhasil diupdate");
            }
        }

        String::from_str(&env, "Travel log tidak ditemukan")
    }

    // Hapus travel log berdasarkan ID
    pub fn delete_log(env: Env, id: u64) -> String {
        let mut logs: Vec<TravelLog> = env
            .storage()
            .instance()
            .get(&TRAVEL_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..logs.len() {
            if logs.get(i).unwrap().id == id {
                logs.remove(i);
                env.storage().instance().set(&TRAVEL_DATA, &logs);

                // Update counter
                let total: u64 = env
                    .storage()
                    .instance()
                    .get(&TOTAL_LOGS)
                    .unwrap_or(1u64);
                env.storage()
                    .instance()
                    .set(&TOTAL_LOGS, &(total - 1));

                return String::from_str(&env, "Travel log berhasil dihapus");
            }
        }

        String::from_str(&env, "Travel log tidak ditemukan")
    }

    // Ambil total jumlah travel logs
    pub fn get_total_logs(env: Env) -> u64 {
        env.storage()
            .instance()
            .get(&TOTAL_LOGS)
            .unwrap_or(0u64)
    }

    // Ambil logs berdasarkan mood tertentu
    pub fn get_logs_by_mood(env: Env, mood: Mood) -> Vec<TravelLog> {
        let logs: Vec<TravelLog> = env
            .storage()
            .instance()
            .get(&TRAVEL_DATA)
            .unwrap_or(Vec::new(&env));

        let mut filtered = Vec::new(&env);
        for i in 0..logs.len() {
            let log = logs.get(i).unwrap();
            if log.mood == mood {
                filtered.push_back(log);
            }
        }
        filtered
    }

    // Hapus semua travel logs (reset)
    pub fn clear_all_logs(env: Env) -> String {
        env.storage().instance().remove(&TRAVEL_DATA);
        env.storage().instance().set(&TOTAL_LOGS, &0u64);
        String::from_str(&env, "Semua travel log berhasil dihapus")
    }
}

mod test;