fn main()
{
    let argc = 1;
    let mut argv_string = String::from("vim");
    let mut argv2 = argv_string.as_mut_ptr() as *mut i8;
    let argv = &mut argv2;
    let mut file_name_string = String::from("file_name");
    let file_name = file_name_string.as_mut_ptr();
    unsafe {
        rust_ffi::vimInit(argc, argv);
        let buff = rust_ffi::vimBufferOpen(file_name, 0, 0);
        rust_ffi::vimPrintAllText(buff);
    }
}
