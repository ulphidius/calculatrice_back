
#[allow(dead_code)]
pub mod calculatrice_primordial_fonction {

    pub mod basics {
        /// Add two number
        /// 
        /// # Examples
        /// 
        /// ```
        /// let first_number = 10;
        /// let second_number = 12;
        /// let result = calculatrice_back::calculatrice_primordial_fonction::basics::add(first_number, second_number);
        /// 
        /// assert_eq!(result, 22);
        /// ```
        pub fn add(first_number: i128, second_number: i128) -> i128 {
            first_number + second_number
        }
    
        /// Substract two number
        /// 
        /// # Examples
        /// 
        /// ```
        /// let first_number = 10;
        /// let second_number = 2;
        /// let result = calculatrice_back::calculatrice_primordial_fonction::basics::sub(first_number, second_number);
        /// 
        /// assert_eq!(result, 8);
        /// ```
        pub fn sub(first_number: i128, second_number: i128) -> i128 {
            first_number - second_number
        }
    
        /// Multiply two number
        /// 
        /// # Exemples
        /// 
        /// ```
        /// let first_number = 10;
        /// let second_number = 2;
        /// let result = calculatrice_back::calculatrice_primordial_fonction::basics::mul(first_number, second_number);
        /// 
        /// assert_eq!(result, 20);
        /// ```
        pub fn mul(first_number: i128, second_number: i128) -> i128 {
            first_number * second_number
        }
    
        /// Divide two number
        /// 
        /// # Examples
        /// 
        /// ```
        /// let first_number = 10;
        /// let second_number = 2;
        /// let result = calculatrice_back::calculatrice_primordial_fonction::basics::div(first_number, second_number);
        /// 
        /// assert_eq!(result, 5);
        /// ```
        pub fn div(first_number: i128, second_number: i128) -> i128 {
            if first_number == 0 || second_number == 0 {

            }
            first_number / second_number
        }
    }

    pub mod extends {
        pub fn modulo(first_number: i128, second_number: i128) -> i128 {
            first_number % second_number
        }

        pub fn pow(first_number: i128, second_number: i128) -> i128 {
            first_number ^ second_number
        }
    }
}