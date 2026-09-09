/* SPDX-License-Identifier: GPL-2.0 */
// Source header guard: _LINUX_DIRENT_H

#[repr(C)]
pub struct linux_dirent64 {
    pub d_ino: u64,
    pub d_off: s64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [::core::ffi::c_char; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
