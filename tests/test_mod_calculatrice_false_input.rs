extern crate calculatrice_back;

use calculatrice_back::calculatrice_primordial_fonction::basics as basic;
// use calculatrice_back::calculatrice_primordial_fonction::extends as extend;

#[cfg(test)]
mod test_mod_calculatrice_false_input {
    #[test]
    #[should_panic]
    fn normal_input_add_test() {
        super::basic::div(10, 0);
    }
}
