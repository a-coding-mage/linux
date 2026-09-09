// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (c) 2017 Facebook
 */

use core::ffi::c_void;

// The C source includes Linux BPF and BPF helper declarations.
// Build-time architecture condition: these declarations/functions are omitted on aarch64.

#[cfg(not(target_arch = "aarch64"))]
#[repr(C)]
pub struct syscalls_enter_open_args {
    pub unused: u64,
    pub syscall_nr: i64,
    pub filename_ptr: i64,
    pub flags: i64,
    pub mode: i64,
}

#[repr(C)]
pub struct syscalls_exit_open_args {
    pub unused: u64,
    pub syscall_nr: i64,
    pub ret: i64,
}

#[repr(C)]
pub struct syscalls_enter_open_at_args {
    pub unused: u64,
    pub syscall_nr: i64,
    pub dfd: i64,
    pub filename_ptr: i64,
    pub flags: i64,
    pub mode: i64,
}

// Opaque representation of the map object emitted by the BPF map-definition macros.
#[repr(C)]
pub struct BpfMap {
    _private: [u8; 0],
}

#[link_section = ".maps"]
#[no_mangle]
pub static mut enter_open_map: BpfMap = BpfMap { _private: [] };

#[link_section = ".maps"]
#[no_mangle]
pub static mut exit_open_map: BpfMap = BpfMap { _private: [] };

extern "C" {
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const u32) -> *mut u32;
    fn bpf_map_update_elem(
        map: *mut c_void,
        key: *const u32,
        value: *const u32,
        flags: u64,
    ) -> i64;
}

const BPF_NOEXIST: u64 = 1;

#[inline(always)]
unsafe fn count(map: *mut c_void) {
    let key: u32 = 0;
    let mut init_val: u32 = 1;

    let value = bpf_map_lookup_elem(map, &key);
    if !value.is_null() {
        *value = (*value).wrapping_add(1);
    } else {
        bpf_map_update_elem(map, &key, &mut init_val, BPF_NOEXIST);
    }
}

// SEC("tracepoint/syscalls/sys_enter_open")
#[cfg(not(target_arch = "aarch64"))]
#[no_mangle]
pub unsafe extern "C" fn trace_enter_open(_ctx: *mut syscalls_enter_open_args) -> i32 {
    count(core::ptr::addr_of_mut!(enter_open_map).cast::<c_void>());
    0
}

// SEC("tracepoint/syscalls/sys_enter_openat")
#[no_mangle]
pub unsafe extern "C" fn trace_enter_open_at(_ctx: *mut syscalls_enter_open_at_args) -> i32 {
    count(core::ptr::addr_of_mut!(enter_open_map).cast::<c_void>());
    0
}

// SEC("tracepoint/syscalls/sys_enter_openat2")
#[no_mangle]
pub unsafe extern "C" fn trace_enter_open_at2(_ctx: *mut syscalls_enter_open_at_args) -> i32 {
    count(core::ptr::addr_of_mut!(enter_open_map).cast::<c_void>());
    0
}

// SEC("tracepoint/syscalls/sys_exit_open")
#[cfg(not(target_arch = "aarch64"))]
#[no_mangle]
pub unsafe extern "C" fn trace_enter_exit(_ctx: *mut syscalls_exit_open_args) -> i32 {
    count(core::ptr::addr_of_mut!(exit_open_map).cast::<c_void>());
    0
}

// SEC("tracepoint/syscalls/sys_exit_openat")
#[no_mangle]
pub unsafe extern "C" fn trace_enter_exit_at(_ctx: *mut syscalls_exit_open_args) -> i32 {
    count(core::ptr::addr_of_mut!(exit_open_map).cast::<c_void>());
    0
}

// SEC("tracepoint/syscalls/sys_exit_openat2")
#[no_mangle]
pub unsafe extern "C" fn trace_enter_exit_at2(_ctx: *mut syscalls_exit_open_args) -> i32 {
    count(core::ptr::addr_of_mut!(exit_open_map).cast::<c_void>());
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
