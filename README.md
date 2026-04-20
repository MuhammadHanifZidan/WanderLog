# 🗺️ WanderLog — Decentralized Travel Journal

> *Your journeys, forever on-chain.*

WanderLog is a decentralized travel journaling application built on the **Stellar Soroban** smart contract platform. Every trip you take — the destinations, the stories, the feelings — is stored permanently and immutably on the blockchain. No servers. No middlemen. Just you and your memories, on-chain.

---

## ✨ Features

| Feature | Description |
|---|---|
| 📍 **Create Travel Log** | Record your journey with destination, story, date, mood & rating |
| 📖 **Read All Logs** | Fetch all travel entries stored on-chain |
| 🔍 **Find by ID** | Retrieve a specific log using its unique on-chain ID |
| 🎭 **Filter by Mood** | Query logs filtered by mood (Amazing, Good, Neutral, Tired, Difficult) |
| ✏️ **Update Mood** | Update the mood of an existing travel entry |
| 🗑️ **Delete Log** | Remove a specific travel log by ID |
| 📊 **Total Counter** | On-chain counter tracking total number of logs |
| 🧹 **Clear All** | Reset all travel data from storage |

---

## 🏗️ Application Description

WanderLog is a **dApp (decentralized application)** that leverages Soroban smart contracts on the Stellar testnet to store travel journal entries. Unlike traditional journaling apps that rely on centralized servers, WanderLog stores every entry directly on the blockchain — making your memories permanent, transparent, and tamper-proof.

The app consists of two parts:
- **Smart Contract** (`lib.rs`) — Written in Rust for the Soroban platform, handling all on-chain data storage and logic
- **Frontend** (`index.html`) — A beautiful, standalone HTML/CSS/JS interface to interact with your travel logs

### Data Structure

Each travel log stores:
```
TravelLog {
    id          : u64       — Unique on-chain identifier
    destination : String    — Name of the location visited
    story       : String    — Your personal travel story
    date        : String    — Date of travel (YYYY-MM-DD)
    mood        : Mood      — Amazing | Good | Neutral | Tired | Difficult
    rating      : u32       — Rating from 1 to 5
}
```

---

## 🔗 Smart Contract

### Contract ID (Stellar Testnet)

```
CDOFWH3M5LABWJQG7RVBSSKB5U7V2WDVD325TSKBSLMFWB2R5UTJPWLZ
```

### Network
- **Network:** Stellar Testnet
- **Platform:** Soroban Smart Contracts
- **Language:** Rust (`#![no_std]`)

### Available Functions

```rust
// Read functions
get_all_logs(env)                          → Vec<TravelLog>
get_log_by_id(env, id: u64)               → Option<TravelLog>
get_logs_by_mood(env, mood: Mood)         → Vec<TravelLog>
get_total_logs(env)                        → u64

// Write functions
create_log(env, destination, story, date, mood, rating) → u64
update_mood(env, id: u64, new_mood: Mood) → String
delete_log(env, id: u64)                  → String
clear_all_logs(env)                        → String
```

### Storage

WanderLog uses two on-chain storage keys:

```rust
const TRAVEL_DATA: Symbol = symbol_short!("TRVL_DATA"); // stores Vec<TravelLog>
const TOTAL_LOGS:  Symbol = symbol_short!("TOTAL");      // stores u64 counter
```

Both use `env.storage().instance()` — persistent storage that lives as long as the contract is active on-chain.

---

## 🖥️ Testnet Screenshot

<!-- Paste your Sorobix testnet screenshot below -->

> **`create_log` invocation:**

![Testnet Screenshot - create_log](<img width="1919" height="927" alt="image" src="https://github.com/user-attachments/assets/0e9d890c-4ba5-4ae7-8d75-cf6c8f164a7d" />
)

---

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| Smart Contract | Rust + Soroban SDK |
| Blockchain | Stellar Testnet |
| Frontend | HTML5 / CSS3 / Vanilla JS |
| Online IDE | Sorobix |
| Fonts | Playfair Display + DM Sans + DM Mono |

---

## 📁 Project Structure

```
wanderlog/
├── contracts/
│   └── wanderlog/
│       └── src/
│           ├── lib.rs       ← Smart contract (main logic)
│           └── test.rs      ← Unit tests (5 test cases)
└── frontend/
    └── index.html           ← Standalone frontend UI
```

---

## 🧪 Running Tests

```bash
cargo test
```

Test cases covered:
- ✅ Create and retrieve a travel log
- ✅ Update mood of an existing log
- ✅ Delete a log by ID
- ✅ Filter logs by mood
- ✅ Invalid rating (1–5 only) panics correctly

---

## 👤 Author

Built with ❤️ using AI assistance during the Soroban Workshop.

> *"The world is a book, and those who do not travel read only one page."*
