// SPDX-License-Identifier: GPL-2.0
// C dependencies: "builtin.h", "color.h", "util/debug.h", "util/header.h",
// <tools/config.h>, <subcmd/parse-options.h>

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct version {
    pub build_options: bool,
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct feature {
    pub name: *const c_char,
}

unsafe extern "C" {
    static mut supported_features: [feature; 0];
    static perf_version_string: *const c_char;
    static verbose: c_int;

    fn parse_options(
        argc: c_int,
        argv: *const *const c_char,
        options: *mut option,
        usagestr: *const *const c_char,
        flags: c_uint,
    ) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn feature_status__printf(feature: *const feature);
}

unsafe extern "C" {
    fn OPT_BOOLEAN(
        short_name: c_int,
        long_name: *const c_char,
        value: *mut bool,
        help: *const c_char,
    ) -> option;
    fn OPT_END() -> option;
}

unsafe extern "C" {
    static PARSE_OPT_STOP_AT_NON_OPTION: c_uint;
}

static mut version: version = version {
    build_options: false,
};

static mut version_options: [option; 2] = unsafe {
    [
        OPT_BOOLEAN(
            0,
            c"build-options".as_ptr(),
            core::ptr::addr_of_mut!(version.build_options),
            c"display the build options".as_ptr(),
        ),
        OPT_END(),
    ]
};

static version_usage_0: &[u8] = b"perf version [<options>]\0";

static version_usage: [*const c_char; 2] = [
    version_usage_0.as_ptr() as *const c_char,
    core::ptr::null(),
];

unsafe fn library_status() {
    let mut i: usize = 0;

    while !(*supported_features.as_ptr().add(i)).name.is_null() {
        feature_status__printf(supported_features.as_ptr().add(i));
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cmd_version(mut argc: c_int, argv: *const *const c_char) -> c_int {
    argc = parse_options(
        argc,
        argv,
        version_options.as_mut_ptr(),
        version_usage.as_ptr(),
        PARSE_OPT_STOP_AT_NON_OPTION,
    );
    let _ = argc;

    printf(c"perf version %s\n".as_ptr(), perf_version_string);

    if version.build_options || verbose > 0 {
        library_status();
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
