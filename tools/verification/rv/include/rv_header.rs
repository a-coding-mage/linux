// SPDX-License-Identifier: GPL-2.0

pub const MAX_DESCRIPTION: usize = 1024;
pub const MAX_DA_NAME_LEN: usize = 32;

#[repr(C)]
pub struct monitor {
    pub name: [::std::os::raw::c_char; MAX_DA_NAME_LEN],
    pub desc: [::std::os::raw::c_char; MAX_DESCRIPTION],
    pub enabled: ::std::os::raw::c_int,
    pub nested: ::std::os::raw::c_int,
}

unsafe extern "C" {
    pub fn should_stop() -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
