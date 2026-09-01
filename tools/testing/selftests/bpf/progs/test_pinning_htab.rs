// SPDX-License-Identifier: GPL-2.0

// Original C dependencies:
// #include "vmlinux.h"
// #include <bpf/bpf_helpers.h>

extern "C" {
    pub type bpf_timer;
}

pub const BPF_MAP_TYPE_HASH: u32 = 1;
pub const BPF_F_NO_PREALLOC: u32 = 1;

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct timer_val {
    pub timer: bpf_timer,
}

#[repr(C)]
pub struct timer_prealloc_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut timer_prealloc: timer_prealloc_def = timer_prealloc_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<timer_val>() as u32,
    max_entries: 1,
};

#[repr(C)]
pub struct timer_no_prealloc_def {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
    pub map_flags: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut timer_no_prealloc: timer_no_prealloc_def = timer_no_prealloc_def {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<u32>() as u32,
    value_size: core::mem::size_of::<timer_val>() as u32,
    max_entries: 1,
    map_flags: BPF_F_NO_PREALLOC,
};

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
