// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
pub struct subcmd_config {
    pub exec_name: *const ::std::os::raw::c_char,
    pub prefix: *const ::std::os::raw::c_char,
    pub exec_path: *const ::std::os::raw::c_char,
    pub exec_path_env: *const ::std::os::raw::c_char,
    pub pager_env: *const ::std::os::raw::c_char,
}

unsafe extern "C" {
    pub static mut subcmd_config: subcmd_config;
}
