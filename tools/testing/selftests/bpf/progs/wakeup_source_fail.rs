// SPDX-License-Identifier: GPL-2.0
/* Copyright 2026 Google LLC */

// C dependencies: <vmlinux.h>, <bpf/bpf_helpers.h>, "bpf_misc.h"

#[repr(C)]
pub struct bpf_ws_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

unsafe extern "C" {
    #[link_name = "bpf_wakeup_sources_read_lock"]
    fn bpf_wakeup_sources_read_lock() -> *mut bpf_ws_lock;
    #[link_name = "bpf_wakeup_sources_read_unlock"]
    fn bpf_wakeup_sources_read_unlock(lock: *mut bpf_ws_lock);
    #[link_name = "bpf_wakeup_sources_get_head"]
    fn bpf_wakeup_sources_get_head() -> *mut core::ffi::c_void;
}

// SEC("syscall")
// __failure __msg("BPF_EXIT instruction in main prog would lead to reference leak")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wakeup_source_lock_no_unlock(ctx: *mut core::ffi::c_void) -> i32 {
    let lock: *mut bpf_ws_lock;

    lock = unsafe { bpf_wakeup_sources_read_lock() };
    if lock.is_null() {
        return 0;
    }

    return 0;
}

// SEC("syscall")
// __failure __msg("access beyond struct")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wakeup_source_access_lock_fields(ctx: *mut core::ffi::c_void) -> i32 {
    let lock: *mut bpf_ws_lock;
    let val: i32;

    lock = unsafe { bpf_wakeup_sources_read_lock() };
    if lock.is_null() {
        return 0;
    }

    val = unsafe { *(lock as *mut i32) };

    unsafe { bpf_wakeup_sources_read_unlock(lock) };
    return val;
}

// SEC("syscall")
// __failure __msg("release kfunc bpf_wakeup_sources_read_unlock expects referenced PTR_TO_BTF_ID passed to R1")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wakeup_source_unlock_no_lock(ctx: *mut core::ffi::c_void) -> i32 {
    let lock: *mut bpf_ws_lock = 0x1 as *mut core::ffi::c_void as *mut bpf_ws_lock;

    unsafe { bpf_wakeup_sources_read_unlock(lock) };

    return 0;
}

// SEC("syscall")
// __failure __msg("Possibly NULL pointer passed to trusted")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wakeup_source_unlock_null(ctx: *mut core::ffi::c_void) -> i32 {
    unsafe { bpf_wakeup_sources_read_unlock(core::ptr::null_mut()) };

    return 0;
}

// SEC("syscall")
// __failure __msg("R0 invalid mem access 'scalar'")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn wakeup_source_unsafe_dereference(ctx: *mut core::ffi::c_void) -> i32 {
    let head: *mut list_head = unsafe { bpf_wakeup_sources_get_head() } as *mut list_head;

    if unsafe { !(*head).next.is_null() } {
        return 1;
    }

    return 0;
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";
