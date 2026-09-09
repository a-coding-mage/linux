/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: _BPF_PRELOAD_H

use core::ffi::c_int;

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct r#module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_preload_info {
    pub link_name: [i8; 16],
    pub link: *mut bpf_link,
}

#[repr(C)]
pub struct bpf_preload_ops {
    pub preload: Option<unsafe extern "C" fn(*mut bpf_preload_info) -> c_int>,
    pub owner: *mut r#module,
}

extern "C" {
    pub static mut bpf_preload_ops: *mut bpf_preload_ops;
}

pub const BPF_PRELOAD_LINKS: c_int = 2;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
