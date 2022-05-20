
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