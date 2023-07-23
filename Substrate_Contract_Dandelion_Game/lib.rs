#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod guessgame {

    use ink_prelude::string::String;
    use ink_prelude::string::ToString;
    use ink_prelude::borrow::ToOwned;

    #[ink(storage)]
    pub struct GuessGame {
        value1: u32,
        value2: u32,
        win_string: String,
        lose_string: String,
        owner: AccountId,
        guess_num: u32,
        is_answer_correct: bool,
    }

    impl GuessGame {
        #[ink(constructor)]
        pub fn new(value1: u32, value2: u32) -> Self {
            Self {
                value1,
                value2,
                owner: Self::env().caller(),
                guess_num: 0,
                is_answer_correct: false,
                win_string: "Wow! You guess the answer accurately!".to_string(),
                lose_string: "Sorry! Please try again later.".to_string(),
            }
        }

        #[ink(message)]
        pub fn add(&self) -> u32 {
            self.value1 + self.value2
        }

        #[ink(message)]
        pub fn set_two_number_to_add(&mut self, value1: u32, value2: u32) -> String {
            if Self::env().caller() == self.owner {
                return "Owner cannot call this function".to_string();
            } else {
                self.value1 = value1;
                self.value2 = value2;
                return "Values set successfully".to_string();
            }
        }

        #[ink(message)]
        pub fn get_two_number_to_add(&self) -> (u32, u32) {
            (self.value1, self.value2)
        }

        #[ink(message)]
        pub fn set_guess_num(&mut self, guess_num: u32)-> String {

            if Self::env().caller() == self.owner {
                self.guess_num = guess_num;
                return "Guess number set".to_string();
                
             
            }
            return "Only Owner can call this function".to_string();
        }

        #[ink(message)]
        pub fn get_guess_num(&self) -> String {
            if Self::env().caller() == self.owner {
                return "Guess number is ".to_owned()+&self.guess_num.to_string();
                
             
            }
            return "Only Owner can call this function".to_string();
            
        }

        #[ink(message)]
        pub fn check_answer(&self) -> String {
            if Self::env().caller() == self.owner {
                return "Owner cannot call this function".to_string();
            } else if self.value1 + self.value2 ==  self.guess_num {
                return self.win_string.clone();
            } else {
                return self.lose_string.clone();
            }
        }

        #[ink(message)]
        pub fn get_owner(&self) -> AccountId {
            self.owner
        }

        #[ink(message)]
        pub fn get_caller(&self) -> AccountId {
            Self::env().caller()
        }
    }
}
