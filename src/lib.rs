#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {

    use super::add;
    
    #[test]
    fn add_test() {
        unsafe {
            assert_eq!(2 + 3, add(2, 3));
        }
    }
}
