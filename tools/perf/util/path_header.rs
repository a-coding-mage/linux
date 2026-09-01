/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: <stddef.h>, <stdbool.h>

#[repr(C)]
pub struct dirent {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn path__join(
        bf: *mut ::std::os::raw::c_char,
        size: usize,
        path1: *const ::std::os::raw::c_char,
        path2: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn path__join3(
        bf: *mut ::std::os::raw::c_char,
        size: usize,
        path1: *const ::std::os::raw::c_char,
        path2: *const ::std::os::raw::c_char,
        path3: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;

    pub fn is_regular_file(file: *const ::std::os::raw::c_char) -> bool;
    pub fn is_directory(
        base_path: *const ::std::os::raw::c_char,
        dent: *const dirent,
    ) -> bool;
    pub fn is_directory_at(
        dir_fd: ::std::os::raw::c_int,
        path: *const ::std::os::raw::c_char,
    ) -> bool;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
