/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_int, c_uint};

extern "C" {
    pub fn ia32_classify_syscall(syscall: c_uint) -> c_int;

    pub static mut ia32_dir_class: [c_uint; 0];
    pub static mut ia32_write_class: [c_uint; 0];
    pub static mut ia32_read_class: [c_uint; 0];
    pub static mut ia32_chattr_class: [c_uint; 0];
    pub static mut ia32_signal_class: [c_uint; 0];
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
