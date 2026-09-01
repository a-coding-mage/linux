// SPDX-License-Identifier: GPL-2.0

// Dependencies from "vmlinux.h" and <bpf/bpf_helpers.h> are expected to be
// supplied by the surrounding BPF build environment.

extern "C" {
    fn bpf_get_current_pid_tgid() -> u64;
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

extern "C" {
    static BPF_MAP_TYPE_HASH: u32;
    static BPF_NOEXIST: u64;
}

#[no_mangle]
pub static mut set_pid: u32 = 0;
#[no_mangle]
pub static mut set_key: u64 = 0;
#[no_mangle]
pub static mut set_value: u64 = 0;

#[repr(C)]
pub struct hash_map_def {
    pub type_: u32,
    pub max_entries: u32,
    pub key_size: u32,
    pub value_size: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut hash_map: hash_map_def = hash_map_def {
    type_: unsafe { BPF_MAP_TYPE_HASH },
    max_entries: 2,
    key_size: core::mem::size_of::<u64>() as u32,
    value_size: core::mem::size_of::<u64>() as u32,
};

#[no_mangle]
#[link_section = "tp/syscalls/sys_enter_getpgid"]
pub unsafe extern "C" fn bpf_lookup_and_delete_test(ctx: *const core::ffi::c_void) -> i32 {
    let _ = ctx;

    if set_pid == (bpf_get_current_pid_tgid() >> 32) as u32 {
        bpf_map_update_elem(
            core::ptr::addr_of_mut!(hash_map) as *mut core::ffi::c_void,
            core::ptr::addr_of!(set_key) as *const core::ffi::c_void,
            core::ptr::addr_of!(set_value) as *const core::ffi::c_void,
            BPF_NOEXIST,
        );
    }

    0
}

#[no_mangle]
#[link_section = "license"]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
