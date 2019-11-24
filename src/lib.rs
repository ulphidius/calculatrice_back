
#[allow(dead_code)]
pub mod calculatrice_primordial_fonction {

    pub mod basics {
        pub fn add(first_number: u32, second_number: u32) -> u32 {
            first_number + second_number
        }
    
        pub fn sub(first_number: u32, second_number: u32) -> u32 {
            first_number - second_number
        }
    
        pub fn mul(first_number: u32, second_number: u32) -> u32 {
            first_number * second_number
        }
    
        pub fn div(first_number: u32, second_number: u32) -> u32 {
            first_number / second_number
        }
    }

    pub mod extends {
        pub fn modulo(first_number: u32, second_number: u32) -> u32 {
            first_number % second_number
        }

        pub fn pow(first_number: u32, second_number: u32) -> u32 {
            first_number ^ second_number
        }
    }
}