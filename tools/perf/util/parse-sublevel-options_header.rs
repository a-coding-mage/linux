#[repr(C)]
pub struct sublevel_option {
    pub name: *const ::std::os::raw::c_char,
    pub value_ptr: *mut ::std::os::raw::c_int,
}

unsafe extern "C" {
    pub fn perf_parse_sublevel_options(
        str: *const ::std::os::raw::c_char,
        opts: *mut sublevel_option,
    ) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
