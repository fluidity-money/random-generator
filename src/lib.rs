
use rand::Rng;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();


#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct Seed {
    val: i8,
}

#[near_bindgen]
impl Seed {
    pub fn get_num(&self) -> i8 {
        return self.val;
    }

    pub fn generate(&mut self) {
        let mut rng = rand::thread_rng();
        self.val = rng.gen_range(-100..100);
        let log_message = format!("Random number generated is {}", self.val);
        env::log(log_message.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext};

    // mock transaction
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice.testnet".to_string(),
            signer_account_id: "robert.testnet".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "jane.testnet".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    #[test]
    fn increment() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = Seed { val: 0 };
        contract.generate();
        println!("Random value generated is {}", contract.get_num());
    }
}
