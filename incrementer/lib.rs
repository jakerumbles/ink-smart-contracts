#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod incrementer {
    use ink_storage::traits::SpreadAllocate;

    #[ink(storage)]
    #[derive(SpreadAllocate)]
    pub struct Incrementer {
        value: u32,
        // Store a mapping from AccountIds to a u32
        map: ink_storage::Mapping<AccountId, u32>,
    }

    impl Incrementer {
        #[ink(constructor)]
        pub fn new(init_value: u32) -> Self {
            // This call is required to correctly initialize the
            // Mapping of the contract
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.value = init_value;
                let caller = Self::env().caller();
                contract.map.insert(&caller, &0);
            })
        }

        #[ink(constructor)]
        pub fn default() -> Self {
            ink_lang::utils::initialize_contract(|contract: &mut Self| {
                contract.value = Default::default();
            })
        }

        #[ink(message)]
        pub fn get(&self) -> u32 {
            self.value
        }

        // Get the number associated with the caller's AccountId, if it exists
        #[ink(message)]
        pub fn get_mine(&self) -> u32 {
            let caller = Self::env().caller();
            self.map.get(&caller).unwrap_or_default()
        }

        #[ink(message)]
        pub fn inc(&mut self, by: u32) {
            self.value += by;
        }

        #[ink(message)]
        pub fn inc_mine(&mut self, by: u32) {
            let caller = self.env().caller();
            let my_value = self.get_mine();
            self.map.insert(caller, &(my_value + by));
        }

        #[ink(message)]
        pub fn remove_mine(&mut self) {
            let caller = self.env().caller();
            self.map.remove(caller);
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink_lang as ink;

        #[ink::test]
        fn default_works() {
            let contract = Incrementer::default();
            assert_eq!(contract.get(), 0);
        }

        #[ink::test]
        fn it_works() {
            let mut contract = Incrementer::new(42);
            assert_eq!(contract.get(), 42);
            contract.inc(1);
            assert_eq!(contract.get(), 43);
            contract.inc(11);
            assert_eq!(contract.get(), 54);
        }

        #[ink::test]
        fn my_value_works() {
            let contract = Incrementer::new(11);
            assert_eq!(contract.get(), 11);
            assert_eq!(contract.get_mine(), 0);
        }

        #[ink::test]
        fn inc_mine_works() {
            let mut contract = Incrementer::default();
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(1);
            assert_eq!(contract.get_mine(), 1);
            contract.inc_mine(100);
            assert_eq!(contract.get_mine(), 101);
        }

        #[ink::test]
        fn remove_mine_works() {
            let mut contract = Incrementer::default();
            assert_eq!(contract.get_mine(), 0);
            contract.inc_mine(25);
            assert_eq!(contract.get_mine(), 25);
            contract.remove_mine();
            assert_eq!(contract.get_mine(), 0);
        }
    }
}
