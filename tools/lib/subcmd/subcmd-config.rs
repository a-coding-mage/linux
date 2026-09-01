// SPDX-License-Identifier: GPL-2.0
// C dependency intent: #include "subcmd-config.h"

use core::ffi::c_char;

const UNDEFINED: *const c_char = b"SUBCMD_HAS_NOT_BEEN_INITIALIZED\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct subcmd_config {
    pub exec_name: *const c_char,
    pub prefix: *const c_char,
    pub exec_path: *const c_char,
    pub exec_path_env: *const c_char,
    pub pager_env: *const c_char,
}

pub static mut subcmd_config: subcmd_config = subcmd_config {
    exec_name: UNDEFINED,
    prefix: UNDEFINED,
    exec_path: UNDEFINED,
    exec_path_env: UNDEFINED,
    pager_env: UNDEFINED,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
