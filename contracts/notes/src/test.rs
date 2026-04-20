#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{testutils::Ledger, Env};

    #[test]
    fn test_create_and_get_log() {
        let env = Env::default();
        let contract_id = env.register_contract(None, WanderLogContract);
        let client = WanderLogContractClient::new(&env, &contract_id);

        // Buat travel log baru
        let id = client.create_log(
            &String::from_str(&env, "Bali, Indonesia"),
            &String::from_str(&env, "Pantainya luar biasa indah, sunset terbaik!"),
            &String::from_str(&env, "2024-12-25"),
            &Mood::Amazing,
            &5,
        );

        // Verifikasi log berhasil dibuat
        let logs = client.get_all_logs();
        assert_eq!(logs.len(), 1);
        assert_eq!(client.get_total_logs(), 1);

        // Cari berdasarkan ID
        let found = client.get_log_by_id(&id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().rating, 5);
    }

    #[test]
    fn test_update_mood() {
        let env = Env::default();
        let contract_id = env.register_contract(None, WanderLogContract);
        let client = WanderLogContractClient::new(&env, &contract_id);

        let id = client.create_log(
            &String::from_str(&env, "Jakarta"),
            &String::from_str(&env, "Macet parah tapi worth it"),
            &String::from_str(&env, "2024-11-10"),
            &Mood::Tired,
            &3,
        );

        let result = client.update_mood(&id, &Mood::Good);
        let log = client.get_log_by_id(&id).unwrap();
        assert_eq!(log.mood, Mood::Good);
    }

    #[test]
    fn test_delete_log() {
        let env = Env::default();
        let contract_id = env.register_contract(None, WanderLogContract);
        let client = WanderLogContractClient::new(&env, &contract_id);

        let id = client.create_log(
            &String::from_str(&env, "Yogyakarta"),
            &String::from_str(&env, "Borobudur di pagi hari, magical!"),
            &String::from_str(&env, "2024-10-05"),
            &Mood::Amazing,
            &5,
        );

        assert_eq!(client.get_total_logs(), 1);
        client.delete_log(&id);
        assert_eq!(client.get_total_logs(), 0);
        assert!(client.get_log_by_id(&id).is_none());
    }

    #[test]
    fn test_filter_by_mood() {
        let env = Env::default();
        let contract_id = env.register_contract(None, WanderLogContract);
        let client = WanderLogContractClient::new(&env, &contract_id);

        client.create_log(
            &String::from_str(&env, "Lombok"),
            &String::from_str(&env, "Snorkeling seru!"),
            &String::from_str(&env, "2024-09-01"),
            &Mood::Amazing,
            &5,
        );

        client.create_log(
            &String::from_str(&env, "Medan"),
            &String::from_str(&env, "Danau Toba bikin kagum"),
            &String::from_str(&env, "2024-08-15"),
            &Mood::Good,
            &4,
        );

        let amazing_logs = client.get_logs_by_mood(&Mood::Amazing);
        assert_eq!(amazing_logs.len(), 1);
    }

    #[test]
    #[should_panic]
    fn test_invalid_rating() {
        let env = Env::default();
        let contract_id = env.register_contract(None, WanderLogContract);
        let client = WanderLogContractClient::new(&env, &contract_id);

        // Rating 6 harus panic
        client.create_log(
            &String::from_str(&env, "Test"),
            &String::from_str(&env, "Test"),
            &String::from_str(&env, "2024-01-01"),
            &Mood::Neutral,
            &6,
        );
    }
}