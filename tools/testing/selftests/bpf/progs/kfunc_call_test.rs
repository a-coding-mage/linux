// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2021 Facebook */
/* Dependencies in the original C source:
 * #include <vmlinux.h>
 * #include <bpf/bpf_helpers.h>
 * #include "bpf_misc.h"
 * #include "../test_kmods/bpf_testmod_kfunc.h"
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type u8 = __u8;
type u16 = u16;
type u32 = __u32;
type size_t = usize;

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct __sk_buff {
    pub sk: *mut bpf_sock,
}

#[repr(C)]
pub struct bpf_sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct prog_test_ref_kfunc {
    pub a: c_int,
    pub b: c_int,
}

#[repr(C)]
pub struct prog_test_pass1 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct prog_test_pass2 {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_testmod_ctx {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_testmod_test_mod_kfunc(arg: c_int);
    fn bpf_sk_fullsock(sk: *mut bpf_sock) -> *mut bpf_sock;
    fn bpf_kfunc_call_test5(arg1: __u8, arg2: u16, arg3: __u32) -> c_int;
    fn bpf_get_prandom_u32() -> __u32;
    fn bpf_kfunc_call_test4(arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> c_long;
    fn bpf_kfunc_call_test2(sk: *mut sock, arg1: c_int, arg2: c_int) -> c_int;
    fn bpf_kfunc_call_test1(
        sk: *mut sock,
        arg1: c_int,
        arg2: __u64,
        arg3: c_int,
        arg4: __u64,
    ) -> __u64;
    fn bpf_kfunc_call_test_acquire(s: *mut c_ulong) -> *mut prog_test_ref_kfunc;
    fn bpf_kfunc_call_test_release(pt: *mut prog_test_ref_kfunc);
    fn bpf_kfunc_call_test_pass_ctx(skb: *mut __sk_buff);
    fn bpf_kfunc_call_test_pass1(p1: *mut prog_test_pass1);
    fn bpf_kfunc_call_test_pass2(p2: *mut prog_test_pass2);
    fn bpf_kfunc_call_test_mem_len_pass1(mem: *mut c_void, len: size_t);
    fn bpf_kfunc_call_test_mem_len_fail2(mem: *mut c_void, len: c_int);
    fn bpf_kfunc_call_test_get_rdwr_mem(
        pt: *mut prog_test_ref_kfunc,
        len: size_t,
    ) -> *mut c_int;
    fn bpf_kfunc_call_test_get_rdonly_mem(
        pt: *mut prog_test_ref_kfunc,
        len: size_t,
    ) -> *mut c_int;
    fn bpf_kfunc_call_test_static_unused_arg(expected: u32, unused: u32) -> u32;
    fn bpf_testmod_ctx_create(err: *mut c_int) -> *mut bpf_testmod_ctx;
    fn bpf_map_lookup_elem(map: *mut c_void, key: *const c_void) -> *mut c_void;
    fn bpf_kptr_xchg(kptr: *mut *mut bpf_testmod_ctx, ptr: *mut bpf_testmod_ctx)
        -> *mut bpf_testmod_ctx;
    fn bpf_testmod_ctx_release(ctx: *mut bpf_testmod_ctx);
}

/* static struct bpf_spin_lock kfunc_call_lock SEC(".data.A"); */
#[unsafe(link_section = ".data.A")]
static mut kfunc_call_lock: bpf_spin_lock = bpf_spin_lock { _private: [] };

/* SEC("tc") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test_spin_lock_safe(skb: *mut __sk_buff) -> c_int {
    let _ = skb;
    unsafe {
        bpf_spin_lock(&raw mut kfunc_call_lock);
        bpf_testmod_test_mod_kfunc(42);
        bpf_spin_unlock(&raw mut kfunc_call_lock);
    }

    0
}

/* SEC("tc") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test5(skb: *mut __sk_buff) -> c_int {
    let mut sk: *mut bpf_sock = unsafe { (*skb).sk };
    let mut ret: c_int;
    let val32: u32;
    let val16: u16;
    let val8: u8;

    if sk.is_null() {
        return -1;
    }

    sk = unsafe { bpf_sk_fullsock(sk) };
    if sk.is_null() {
        return -1;
    }

    /*
     * Test with constant values to verify zero-extension.
     * ISA-dependent BPF asm:
     *   With ALU32:    w1 = 0xFF; w2 = 0xFFFF; w3 = 0xFFFFffff
     *   Without ALU32: r1 = 0xFF; r2 = 0xFFFF; r3 = 0xFFFFffff
     * Both zero-extend to 64-bit before the kfunc call.
     */
    ret = unsafe { bpf_kfunc_call_test5(0xFF, 0xFFFF, 0xFFFFffffu64 as __u32) };
    if ret != 0 {
        return ret;
    }

    val32 = unsafe { bpf_get_prandom_u32() };
    val16 = (val32 & 0xFFFF) as u16;
    val8 = (val32 & 0xFF) as u8;
    ret = unsafe { bpf_kfunc_call_test5(val8, val16, val32) };
    if ret != 0 {
        return ret;
    }

    /*
     * Test multiplication with different operand sizes:
     *
     * val8 * 0xFF:
     *   - Both operands promote to int (32-bit signed)
     *   - Result: 32-bit multiplication, truncated to u8, then zero-extended
     *
     * val16 * 0xFFFF:
     *   - Both operands promote to int (32-bit signed)
     *   - Result: 32-bit multiplication, truncated to u16, then zero-extended
     *
     * val32 * 0xFFFFffffULL:
     *   - val32 (u32) promotes to unsigned long long (due to ULL suffix)
     *   - Result: 64-bit unsigned multiplication, truncated to u32, then zero-extended
     */
    ret = unsafe {
        bpf_kfunc_call_test5(
            ((val8 as c_int).wrapping_mul(0xFF) as u8),
            ((val16 as c_int).wrapping_mul(0xFFFF) as u16),
            ((val32 as u64).wrapping_mul(0xFFFFffffu64) as u32),
        )
    };
    if ret != 0 {
        return ret;
    }

    0
}

/*
 * Assembly version testing the multiplication edge case explicitly.
 * This ensures consistent testing across different ISA versions.
 */
/* SEC("tc") __naked */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test5_asm() -> c_int {
    unsafe {
        asm!(
            /* Get a random u32 value */
            "call {bpf_get_prandom_u32};",
            "r6 = r0;",
            /* Save val32 in r6 */
            /* Prepare first argument: val8 * 0xFF */
            "r1 = r6;",
            "r1 &= 0xFF;",
            /* val8 = val32 & 0xFF */
            "r7 = 0xFF;",
            "r1 *= r7;",
            /* 64-bit mult: r1 = r1 * r7 */
            /* Prepare second argument: val16 * 0xFFFF */
            "r2 = r6;",
            "r2 &= 0xFFFF;",
            /* val16 = val32 & 0xFFFF */
            "r7 = 0xFFFF;",
            "r2 *= r7;",
            /* 64-bit mult: r2 = r2 * r7 */
            /* Prepare third argument: val32 * 0xFFFFffff */
            "r3 = r6;",
            /* val32 */
            "r7 = 0xFFFFffff;",
            "r3 *= r7;",
            /* 64-bit mult: r3 = r3 * r7 */
            /* Call kfunc with multiplication results */
            "call bpf_kfunc_call_test5;",
            /* Check return value */
            "if r0 != 0 goto 2f;",
            "r0 = 0;",
            "2: exit;",
            bpf_get_prandom_u32 = sym bpf_get_prandom_u32,
            options(noreturn)
        );
    }
}

/* SEC("tc") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test4(skb: *mut __sk_buff) -> c_int {
    let mut sk: *mut bpf_sock = unsafe { (*skb).sk };
    let tmp: c_long;

    if sk.is_null() {
        return -1;
    }

    sk = unsafe { bpf_sk_fullsock(sk) };
    if sk.is_null() {
        return -1;
    }

    tmp = unsafe { bpf_kfunc_call_test4(-3, -30, -200, -1000) };
    ((tmp >> 32).wrapping_add(tmp)) as c_int
}

/* SEC("tc") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test2(skb: *mut __sk_buff) -> c_int {
    let mut sk: *mut bpf_sock = unsafe { (*skb).sk };

    if sk.is_null() {
        return -1;
    }

    sk = unsafe { bpf_sk_fullsock(sk) };
    if sk.is_null() {
        return -1;
    }

    unsafe { bpf_kfunc_call_test2(sk as *mut sock, 1, 2) }
}

/* SEC("tc") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test1(skb: *mut __sk_buff) -> c_int {
    let mut sk: *mut bpf_sock = unsafe { (*skb).sk };
    let mut a: __u64 = 1u64 << 32;
    let mut ret: __u32;

    if sk.is_null() {
        return -1;
    }

    sk = unsafe { bpf_sk_fullsock(sk) };
    if sk.is_null() {
        return -1;
    }

    a = unsafe { bpf_kfunc_call_test1(sk as *mut sock, 1, a | 2, 3, a | 4) };
    ret = (a >> 32) as __u32; /* ret should be 2 */
    ret = ret.wrapping_add(a as __u32); /* ret should be 12 */

    ret as c_int
}

/* SEC("tc") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test_ref_btf_id(skb: *mut __sk_buff) -> c_int {
    let _ = skb;
    let mut pt: *mut prog_test_ref_kfunc;
    let mut s: c_ulong = 0;
    let mut ret: c_int = 0;

    pt = unsafe { bpf_kfunc_call_test_acquire(&mut s) };
    if !pt.is_null() {
        if unsafe { (*pt).a != 42 || (*pt).b != 108 } {
            ret = -1;
        }
        unsafe { bpf_kfunc_call_test_release(pt) };
    }
    ret
}

/* SEC("tc") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test_pass(skb: *mut __sk_buff) -> c_int {
    let mut p1: prog_test_pass1 = unsafe { core::mem::zeroed() };
    let mut p2: prog_test_pass2 = unsafe { core::mem::zeroed() };
    let mut a: i16 = 0;
    let mut b: __u64 = 0;
    let mut c: c_long = 0;
    let mut d: c_char = 0;
    let mut e: c_int = 0;

    unsafe {
        bpf_kfunc_call_test_pass_ctx(skb);
        bpf_kfunc_call_test_pass1(&mut p1);
        bpf_kfunc_call_test_pass2(&mut p2);

        bpf_kfunc_call_test_mem_len_pass1(
            &mut a as *mut i16 as *mut c_void,
            core::mem::size_of_val(&a),
        );
        bpf_kfunc_call_test_mem_len_pass1(
            &mut b as *mut __u64 as *mut c_void,
            core::mem::size_of_val(&b),
        );
        bpf_kfunc_call_test_mem_len_pass1(
            &mut c as *mut c_long as *mut c_void,
            core::mem::size_of_val(&c),
        );
        bpf_kfunc_call_test_mem_len_pass1(
            &mut d as *mut c_char as *mut c_void,
            core::mem::size_of_val(&d),
        );
        bpf_kfunc_call_test_mem_len_pass1(
            &mut e as *mut c_int as *mut c_void,
            core::mem::size_of_val(&e),
        );
        bpf_kfunc_call_test_mem_len_fail2(&mut b as *mut __u64 as *mut c_void, -1);
    }

    0
}

#[repr(C)]
pub struct syscall_test_args {
    pub data: [__u8; 16],
    pub size: size_t,
}

/* SEC("syscall") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_syscall_test(args: *mut syscall_test_args) -> c_int {
    let size: c_long = unsafe { (*args).size as c_long };

    if size as usize > core::mem::size_of_val(unsafe { &(*args).data }) {
        return -7; /* -E2BIG */
    }

    unsafe {
        bpf_kfunc_call_test_mem_len_pass1(
            &mut (*args).data as *mut [__u8; 16] as *mut c_void,
            core::mem::size_of_val(&(*args).data),
        );
        bpf_kfunc_call_test_mem_len_pass1(
            &mut (*args).data as *mut [__u8; 16] as *mut c_void,
            core::mem::size_of::<syscall_test_args>(),
        );
        bpf_kfunc_call_test_mem_len_pass1(
            &mut (*args).data as *mut [__u8; 16] as *mut c_void,
            size as size_t,
        );
    }

    0
}

/* SEC("syscall") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_syscall_test_null(args: *mut syscall_test_args) -> c_int {
    /* Must be called with args as a NULL pointer
     * we do not check for it to have the verifier consider that
     * the pointer might not be null, and so we can load it.
     *
     * So the following can not be added:
     *
     * if (args)
     *      return -22;
     */

    unsafe { bpf_kfunc_call_test_mem_len_pass1(args as *mut c_void, 0) };

    0
}

/* SEC("tc") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test_get_mem(skb: *mut __sk_buff) -> c_int {
    let _ = skb;
    let mut pt: *mut prog_test_ref_kfunc;
    let mut s: c_ulong = 0;
    let mut p: *mut c_int = core::ptr::null_mut();
    let mut ret: c_int = 0;

    pt = unsafe { bpf_kfunc_call_test_acquire(&mut s) };
    if !pt.is_null() {
        p = unsafe {
            bpf_kfunc_call_test_get_rdwr_mem(pt, 2usize.wrapping_mul(core::mem::size_of::<c_int>()))
        };
        if !p.is_null() {
            unsafe {
                *p.add(0) = 42;
                ret = *p.add(1); /* 108 */
            }
        } else {
            ret = -1;
        }

        if ret >= 0 {
            p = unsafe {
                bpf_kfunc_call_test_get_rdonly_mem(
                    pt,
                    2usize.wrapping_mul(core::mem::size_of::<c_int>()),
                )
            };
            if !p.is_null() {
                ret = unsafe { *p.add(0) }; /* 42 */
            } else {
                ret = -1;
            }
        }

        unsafe { bpf_kfunc_call_test_release(pt) };
    }
    ret
}

/* SEC("tc") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_test_static_unused_arg(skb: *mut __sk_buff) -> c_int {
    let _ = skb;

    let expected: u32 = 5;
    let actual: u32;

    actual = unsafe { bpf_kfunc_call_test_static_unused_arg(expected, 0xdeadbeef) };
    if actual != expected { -1 } else { 0 }
}

#[repr(C)]
pub struct ctx_val {
    /* struct bpf_testmod_ctx __kptr *ctx; */
    pub ctx: *mut bpf_testmod_ctx,
}

#[repr(C)]
pub struct ctx_map_def {
    /* __uint(type, BPF_MAP_TYPE_ARRAY);
     * __uint(max_entries, 1);
     * __type(key, int);
     * __type(value, struct ctx_val);
     */
    _private: [u8; 0],
}

/* } ctx_map SEC(".maps"); */
#[unsafe(link_section = ".maps")]
static mut ctx_map: ctx_map_def = ctx_map_def { _private: [] };

/* SEC("tc") */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfunc_call_ctx(skb: *mut __sk_buff) -> c_int {
    let _ = skb;
    let mut ctx: *mut bpf_testmod_ctx;
    let mut err: c_int = 0;

    ctx = unsafe { bpf_testmod_ctx_create(&mut err) };
    if ctx.is_null() && err == 0 {
        err = -1;
    }
    if !ctx.is_null() {
        let key: c_int = 0;
        let ctx_val: *mut ctx_val = unsafe {
            bpf_map_lookup_elem(
                &raw mut ctx_map as *mut c_void,
                &key as *const c_int as *const c_void,
            ) as *mut ctx_val
        };

        /* Transfer ctx to map to be freed via implicit dtor call
         * on cleanup.
         */
        if !ctx_val.is_null() {
            ctx = unsafe { bpf_kptr_xchg(&mut (*ctx_val).ctx, ctx) };
        }
        if !ctx.is_null() {
            unsafe { bpf_testmod_ctx_release(ctx) };
            err = -1;
        }
    }
    err
}

/* char _license[] SEC("license") = "GPL"; */
#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
