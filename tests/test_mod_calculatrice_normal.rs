extern crate calculatrice_back;

use calculatrice_back::calculatrice_primordial_fonction as basic;

#[cfg(test)]
mod test_mod_calculatrice_basic {
    #[test]
    fn normal_input_add_test() {
        assert_eq!(super::basic::add(2, 2), 4);
    }

    #[test]
    fn normal_input_sub_test() {
        assert_eq!(super::basic::sub(6, 2), 4);
    }

    #[test]
    fn normal_input_mul_test() {
        assert_eq!(super::basic::mul(2, 2), 4);
    }

    #[test]
    fn normal_input_div_test() {
        assert_eq!(super::basic::div(8, 2), 4);
    }
}
