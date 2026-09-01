// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::c_void;

type __u8 = u8;
type size_t = usize;

#[repr(C)]
pub struct __sk_buff {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct prog_test_ref_kfunc {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_kfunc_trigger_ctx_check();
    fn bpf_kfunc_call_test_mem_len_pass1(mem: *mut c_void, len: size_t);
    fn bpf_kfunc_call_test_acquire(s: *mut u64) -> *mut prog_test_ref_kfunc;
    fn bpf_kfunc_call_test_get_rdonly_mem(
        pt: *mut prog_test_ref_kfunc,
        rdonly_buf_size: u32,
    ) -> *mut i32;
    fn bpf_kfunc_call_test_get_rdwr_mem(
        pt: *mut prog_test_ref_kfunc,
        rdwr_buf_size: u32,
    ) -> *mut i32;
    fn bpf_kfunc_call_test_release(pt: *mut prog_test_ref_kfunc);
    fn bpf_kfunc_call_test_acq_rdonly_mem(
        pt: *mut prog_test_ref_kfunc,
        rdonly_buf_size: u32,
    ) -> *mut i32;
    fn bpf_kfunc_call_int_mem_release(p: *mut i32);
    fn bpf_kfunc_call_test_pass_ctx(ctx: *mut c_void);
}

#[unsafe(link_section = ".data.A")]
static mut kfunc_call_lock: bpf_spin_lock = bpf_spin_lock { _private: [] };

#[repr(C)]
pub struct syscall_test_args {
    pub data: [__u8; 16],
    pub size: size_t,
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test_spin_lock_unsafe(skb: *mut __sk_buff) -> i32 {
    let _ = skb;

    unsafe {
        bpf_spin_lock(&raw mut kfunc_call_lock);
        bpf_kfunc_trigger_ctx_check();
        bpf_spin_unlock(&raw mut kfunc_call_lock);
    }

    0
}

#[unsafe(link_section = "?syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_syscall_test_fail(args: *mut syscall_test_args) -> i32 {
    unsafe {
        bpf_kfunc_call_test_mem_len_pass1(
            &raw mut (*args).data as *mut c_void,
            size_of::<syscall_test_args>() + 1,
        );
    }

    0
}

#[unsafe(link_section = "?syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_syscall_test_null_fail(args: *mut syscall_test_args) -> i32 {
    /*
     * Must be called with args as a NULL pointer
     * we do not check for it to have the verifier consider that
     * the pointer might not be null, and so we can load it.
     *
     * So the following can not be added:
     *
     * if (args)
     *      return -22;
     */

    unsafe {
        bpf_kfunc_call_test_mem_len_pass1(args as *mut c_void, size_of::<syscall_test_args>());
    }

    0
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test_get_mem_fail_rdonly(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    let mut pt: *mut prog_test_ref_kfunc;
    let mut s: u64 = 0;
    let mut p: *mut i32 = core::ptr::null_mut();
    let mut ret: i32 = 0;

    unsafe {
        pt = bpf_kfunc_call_test_acquire(&mut s);
        if !pt.is_null() {
            p = bpf_kfunc_call_test_get_rdonly_mem(pt, (2 * size_of::<i32>()) as u32);
            if !p.is_null() {
                *p.add(0) = 42; /* this is a read-only buffer, so -EACCES */
            } else {
                ret = -1;
            }

            bpf_kfunc_call_test_release(pt);
        }
    }
    ret
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test_get_mem_fail_use_after_free(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    let mut pt: *mut prog_test_ref_kfunc;
    let mut s: u64 = 0;
    let mut p: *mut i32 = core::ptr::null_mut();
    let mut ret: i32 = 0;

    unsafe {
        pt = bpf_kfunc_call_test_acquire(&mut s);
        if !pt.is_null() {
            p = bpf_kfunc_call_test_get_rdwr_mem(pt, (2 * size_of::<i32>()) as u32);
            if !p.is_null() {
                *p.add(0) = 42;
                ret = *p.add(1); /* 108 */
            } else {
                ret = -1;
            }

            bpf_kfunc_call_test_release(pt);
        }
        if !p.is_null() {
            ret = *p.add(0); /* p is not valid anymore */
        }
    }

    ret
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test_get_mem_fail_oob(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    let mut pt: *mut prog_test_ref_kfunc;
    let mut s: u64 = 0;
    let mut p: *mut i32 = core::ptr::null_mut();
    let mut ret: i32 = 0;

    unsafe {
        pt = bpf_kfunc_call_test_acquire(&mut s);
        if !pt.is_null() {
            p = bpf_kfunc_call_test_get_rdonly_mem(pt, (2 * size_of::<i32>()) as u32);
            if !p.is_null() {
                ret = *p.add(2 * size_of::<i32>()); /* oob access, so -EACCES */
            } else {
                ret = -1;
            }

            bpf_kfunc_call_test_release(pt);
        }
    }
    ret
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test_get_mem_fail_zero_size(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    let mut pt: *mut prog_test_ref_kfunc;
    let mut s: u64 = 0;
    let mut p: *mut i32 = core::ptr::null_mut();
    let mut ret: i32 = 0;

    unsafe {
        pt = bpf_kfunc_call_test_acquire(&mut s);
        if !pt.is_null() {
            /*
             * An explicit rdwr_buf_size of 0 gives R0 a zero-sized buffer,
             * so any access is out of bounds, hence -EACCES. Previously the
             * verifier treated a zero size as "no size argument" and sized
             * R0 after the pointed-to return type, wrongly allowing the read.
             */
            p = bpf_kfunc_call_test_get_rdwr_mem(pt, 0);
            if !p.is_null() {
                ret = *p.add(0);
            } else {
                ret = -1;
            }

            bpf_kfunc_call_test_release(pt);
        }
    }
    ret
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test_get_mem_fail_oversized(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    let mut pt: *mut prog_test_ref_kfunc;
    let mut s: u64 = 0;
    let mut p: *mut i32 = core::ptr::null_mut();
    let ret: i32 = 0;

    unsafe {
        pt = bpf_kfunc_call_test_acquire(&mut s);
        if !pt.is_null() {
            /*
             * rdwr_buf_size is a const int, so a C literal is narrowed to
             * 32 bits before the call. Force the full 64-bit value 2^64 - 192
             * (0xffffffffffffff40, > U32_MAX) into the argument register with
             * a 64-bit immediate load. The verifier records r0_size from the
             * full register value and must reject it before that value is
             * truncated into R0's u32 mem_size.
             */
            asm!(
                "r1 = {pt};",
                "r2 = {oversized} ll;",
                "call {get_rdwr_mem};",
                "{p} = r0;",
                pt = in(reg) pt,
                oversized = const 0xffffffffffffff40_u64,
                get_rdwr_mem = const bpf_kfunc_call_test_get_rdwr_mem,
                p = lateout(reg) p,
                out("r0") _,
                out("r1") _,
                out("r2") _,
                out("r3") _,
                out("r4") _,
                out("r5") _,
            );
            bpf_kfunc_call_test_release(pt);
        }
    }
    ret
}

#[unsafe(no_mangle)]
pub static mut not_const_size: i32 = (2 * size_of::<i32>()) as i32;

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test_get_mem_fail_not_const(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    let mut pt: *mut prog_test_ref_kfunc;
    let mut s: u64 = 0;
    let mut p: *mut i32 = core::ptr::null_mut();
    let mut ret: i32 = 0;

    unsafe {
        pt = bpf_kfunc_call_test_acquire(&mut s);
        if !pt.is_null() {
            p = bpf_kfunc_call_test_get_rdonly_mem(
                pt,
                not_const_size as u32,
            ); /* non const size, -EINVAL */
            if !p.is_null() {
                ret = *p.add(0);
            } else {
                ret = -1;
            }

            bpf_kfunc_call_test_release(pt);
        }
    }
    ret
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test_mem_acquire_fail(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    let mut pt: *mut prog_test_ref_kfunc;
    let mut s: u64 = 0;
    let mut p: *mut i32 = core::ptr::null_mut();
    let mut ret: i32 = 0;

    unsafe {
        pt = bpf_kfunc_call_test_acquire(&mut s);
        if !pt.is_null() {
            /* we are failing on this one, because we are not acquiring a PTR_TO_BTF_ID (a struct ptr) */
            p = bpf_kfunc_call_test_acq_rdonly_mem(pt, (2 * size_of::<i32>()) as u32);
            if !p.is_null() {
                ret = *p.add(0);
            } else {
                ret = -1;
            }

            bpf_kfunc_call_int_mem_release(p);

            bpf_kfunc_call_test_release(pt);
        }
    }
    ret
}

#[unsafe(link_section = "?tc")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test_pointer_arg_type_mismatch(skb: *mut __sk_buff) -> i32 {
    let _ = skb;
    unsafe {
        bpf_kfunc_call_test_pass_ctx(10 as *mut c_void);
    }
    0
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static mut _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
