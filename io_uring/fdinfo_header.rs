// SPDX-License-Identifier: GPL-2.0

// Opaque types supplied by external dependencies.
#[repr(C)]
pub struct seq_file {
    _private: [u8; 0],
}

#[repr(C)]
pub struct file {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn io_uring_show_fdinfo(m: *mut seq_file, f: *mut file);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
