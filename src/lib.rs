#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("./bindings.rs");

pub fn vimBufferGetAllText(buff: *mut file_buffer) -> String
{
    let mut text = String::new();
    unsafe {
        let line_count = vimBufferGetLineCount(buff);
        for line in 1 ..= line_count {
            let line_txt = vimBufferGetLine(buff, line as i64);
            let len = libc::strlen(line_txt as *mut i8);
            let line_string: String = String::from_raw_parts(line_txt, len, len);
            text.push_str(&line_string);
        }
    }
    text
}

pub fn vimPrintAllText(buff: *mut file_buffer)
{
    unsafe {
        let line_count = vimBufferGetLineCount(buff);
        for line in 1 ..= line_count {
            let line_txt = vimBufferGetLine(buff, line as i64) as *const i8;
            libc::puts(line_txt);
        }
    }
}

#[cfg(test)]
mod tests
{
    use crate::vimBufferOpen;
    use crate::vimInit;
    // use ::std::os::raw::c_char;
    #[test]
    fn test()
    {

        let argc = 1;
        let mut argv_string = String::from("vim");
        let mut argv2 = argv_string.as_mut_ptr() as *mut i8;
        let argv = &mut argv2;

        let mut file_name_string = String::from("file_name");
        let file_name = file_name_string.as_mut_ptr();

        unsafe {
            vimInit(argc, argv);
            let buff = vimBufferOpen(file_name, 0, 0);
            super::vimPrintAllText(buff);
        }
        
        assert_eq!(1, 1);
    }
}
