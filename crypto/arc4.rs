// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Cryptographic API
 *
 * ARC4 Cipher Algorithm
 *
 * Jon Oberheide <jon@oberheide.org>
 */

// Dependencies are supplied by the surrounding kernel translation unit:
// crypto/arc4.h, crypto/internal/skcipher.h, linux/init.h, linux/kernel.h,
// linux/module.h, and linux/sched.h.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type u8 = ::core::primitive::u8;
type u32 = ::core::primitive::u32;

#[repr(C)]
pub struct arc4_ctx {
    _private: [u8; 0],
}

#[repr(C)]
pub struct crypto_lskcipher {
    _private: [u8; 0],
}

#[repr(C)]
pub struct lskcipher_alg {
    _private: [u8; 0],
}

extern "C" {
    fn crypto_lskcipher_ctx(tfm: *mut crypto_lskcipher) -> *mut arc4_ctx;
    fn arc4_setkey(ctx: *mut arc4_ctx, in_key: *const u8, key_len: c_uint) -> c_int;
    fn arc4_crypt(ctx: *mut arc4_ctx, dst: *mut u8, src: *const u8, nbytes: c_uint);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn crypto_register_lskcipher(alg: *mut lskcipher_alg) -> c_int;
    fn crypto_unregister_lskcipher(alg: *mut lskcipher_alg);
    fn pr_warn_ratelimited(fmt: *const c_char, ...);
    static mut current: *mut task_struct;
    static THIS_MODULE: *mut c_void;
}

#[repr(C)]
struct task_struct {
    _private: [u8; 0],
}

// #define ARC4_ALIGN __alignof__(struct arc4_ctx)
const ARC4_ALIGN: usize = core::mem::align_of::<arc4_ctx>();

unsafe fn crypto_arc4_setkey(
    tfm: *mut crypto_lskcipher,
    in_key: *const u8,
    key_len: c_uint,
) -> c_int {
    let ctx = crypto_lskcipher_ctx(tfm);
    arc4_setkey(ctx, in_key, key_len)
}

unsafe fn crypto_arc4_crypt(
    tfm: *mut crypto_lskcipher,
    src: *const u8,
    dst: *mut u8,
    nbytes: c_uint,
    siv: *mut u8,
    flags: u32,
) -> c_int {
    let mut ctx = crypto_lskcipher_ctx(tfm);

    if (flags & CRYPTO_LSKCIPHER_FLAG_CONT) == 0 {
        memcpy(siv as *mut c_void, ctx as *const c_void, core::mem::size_of::<arc4_ctx>());
    }

    ctx = siv as *mut arc4_ctx;

    arc4_crypt(ctx, dst, src, nbytes);
    0
}

unsafe fn crypto_arc4_init(_tfm: *mut crypto_lskcipher) -> c_int {
    pr_warn_ratelimited(
        b"\"%s\" (%ld) uses obsolete ecb(arc4) skcipher\0".as_ptr() as *const c_char,
        (*current).comm,
        (*current).pid as c_ulong,
    );

    0
}

// The nested lskcipher_alg initializer is retained as the corresponding
// kernel ABI object supplied by the surrounding translation unit.
extern "C" {
    static mut arc4_alg: lskcipher_alg;
}

unsafe fn arc4_init() -> c_int {
    crypto_register_lskcipher(&mut arc4_alg)
}

unsafe fn arc4_exit() {
    crypto_unregister_lskcipher(&mut arc4_alg);
}

// module_init(arc4_init);
// module_exit(arc4_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("ARC4 Cipher Algorithm");
// MODULE_AUTHOR("Jon Oberheide <jon@oberheide.org>");
// MODULE_ALIAS_CRYPTO("ecb(arc4)");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
