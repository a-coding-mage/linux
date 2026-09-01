// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// #include <vmlinux.h>
// #include <bpf/bpf_helpers.h>
// #include "bpf_misc.h"

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_skb_pull_data(sk: *mut __sk_buff, len: u32) -> i64;
    fn bpf_copy_from_user(dst: *mut core::ffi::c_void, size: u32, user_ptr: *const core::ffi::c_void) -> i64;
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn changes_pkt_data(sk: *mut __sk_buff) -> i64 {
    unsafe { bpf_skb_pull_data(sk, 0) }
}

#[unsafe(no_mangle)]
#[linkage = "weak"]
#[inline(never)]
pub unsafe extern "C" fn does_not_change_pkt_data(sk: *mut __sk_buff) -> i64 {
    0
}

// SEC("?tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_changes_with_subprogs(sk: *mut __sk_buff) -> i32 {
    unsafe {
        changes_pkt_data(sk);
        does_not_change_pkt_data(sk);
    }
    0
}

// SEC("?tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_changes(sk: *mut __sk_buff) -> i32 {
    unsafe {
        bpf_skb_pull_data(sk, 0);
    }
    0
}

// SEC("?tc")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_does_not_change(sk: *mut __sk_buff) -> i32 {
    0
}

#[unsafe(no_mangle)]
#[inline(never)]
pub unsafe extern "C" fn might_sleep(ctx: *mut pt_regs) -> i64 {
    let mut i: i32 = 0;

    unsafe {
        bpf_copy_from_user(
            &mut i as *mut i32 as *mut core::ffi::c_void,
            core::mem::size_of_val(&i) as u32,
            core::ptr::null(),
        );
    }
    i as i64
}

#[unsafe(no_mangle)]
#[linkage = "weak"]
#[inline(never)]
pub unsafe extern "C" fn does_not_sleep(ctx: *mut pt_regs) -> i64 {
    0
}

// SEC("?uprobe.s")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_might_sleep_with_subprogs(ctx: *mut pt_regs) -> i32 {
    unsafe {
        might_sleep(ctx);
        does_not_sleep(ctx);
    }
    0
}

// SEC("?uprobe.s")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_might_sleep(ctx: *mut pt_regs) -> i32 {
    let mut i: i32 = 0;

    unsafe {
        bpf_copy_from_user(
            &mut i as *mut i32 as *mut core::ffi::c_void,
            core::mem::size_of_val(&i) as u32,
            core::ptr::null(),
        );
    }
    i
}

// SEC("?uprobe.s")
#[unsafe(no_mangle)]
pub unsafe extern "C" fn main_does_not_sleep(ctx: *mut pt_regs) -> i32 {
    0
}

// SEC("license")
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
