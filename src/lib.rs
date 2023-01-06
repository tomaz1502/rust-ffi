#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
include!("./bindings.rs");

#[cfg(test)]
mod tests
{
    use crate::vimBufferOpen;
    use crate::vimInit;
    use crate::char_u;
    use ::std::os::raw::c_char;
    #[test]
    fn test()
    {
        let mut bla: char_u = 'a' as char_u;
        let v: *mut char_u = &mut bla;
        let mut c = '\0' as c_char;
        let mut bla : *mut c_char = &mut c;
        let bla2 = &mut bla;
        unsafe {
            vimInit(1, bla2);
            vimBufferOpen(v, 0, 0);
        }
        assert_eq!(1, 1);
    }
//     use super::{add, sub, wot, S};
    
//     #[test]
//     fn add_test()
//     {
//         unsafe {
//             assert_eq!(2 + 3, add(2, 3));
//         }
//     }

//     #[test]
//     fn sub_test()
//     {
//         unsafe {
//             assert_eq!(2 - 3, sub(2, 3));
//         }
//     }

//     #[test]
//     fn wot_test()
//     {
//         unsafe {
//             assert_eq!(42, wot(S{}));
//         }
//     }
}
