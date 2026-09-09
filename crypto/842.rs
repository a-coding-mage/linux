// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Cryptographic API for the 842 software compression algorithm.
 *
 * Copyright (C) IBM Corporation, 2011-2015
 *
 * Original Authors: Robert Jennings <rcj@linux.vnet.ibm.com>
 *                   Seth Jennings <sjenning@linux.vnet.ibm.com>
 *
 * Rewrite: Dan Streetman <ddstreet@ieee.org>
 *
 * This is the software implementation of compression and decompression using
 * the 842 format.  This uses the software 842 library at lib/842/ which is
 * only a reference implementation, and is very, very slow as compared to other
 * software compressors.  You probably do not want to use this software
 * compression.  If you have access to the PowerPC 842 compression hardware, you
 * want to use the 842 hardware compression interface, which is at:
 * drivers/crypto/nx/nx-842-crypto.c
 */

use core::ffi::c_void;

extern "C" {
    static THIS_MODULE: *mut c_void;
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn sw842_compress(src: *const u8, slen: u32, dst: *mut u8,
                      dlen: *mut u32, ctx: *mut c_void) -> i32;
    fn sw842_decompress(src: *const u8, slen: u32, dst: *mut u8,
                        dlen: *mut u32) -> i32;
    fn crypto_register_scomp(alg: *mut ScompAlg) -> i32;
    fn crypto_unregister_scomp(alg: *mut ScompAlg);
}

const SW842_MEM_COMPRESS: usize = 0; // supplied by linux/sw842.h
const GFP_KERNEL: u32 = 0; // supplied by linux/gfp.h

#[repr(C)]
pub struct ScompStreams {
    pub alloc_ctx: Option<unsafe extern "C" fn() -> *mut c_void>,
    pub free_ctx: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct CryptoBase {
    pub cra_name: *const u8,
    pub cra_driver_name: *const u8,
    pub cra_priority: i32,
    pub cra_module: *mut c_void,
}

#[repr(C)]
pub struct ScompAlg {
    pub streams: ScompStreams,
    pub compress: Option<unsafe extern "C" fn(*mut c_void, *const u8, u32,
                                                *mut u8, *mut u32, *mut c_void) -> i32>,
    pub decompress: Option<unsafe extern "C" fn(*mut c_void, *const u8, u32,
                                                  *mut u8, *mut u32, *mut c_void) -> i32>,
    pub base: CryptoBase,
}

unsafe extern "C" fn crypto842_alloc_ctx() -> *mut c_void {
    let ctx = kmalloc(SW842_MEM_COMPRESS, GFP_KERNEL);
    if ctx.is_null() {
        return (-12isize) as *mut c_void; // ERR_PTR(-ENOMEM)
    }
    ctx
}

unsafe extern "C" fn crypto842_free_ctx(ctx: *mut c_void) {
    kfree(ctx);
}

unsafe extern "C" fn crypto842_scompress(
    _tfm: *mut c_void, src: *const u8, slen: u32,
    dst: *mut u8, dlen: *mut u32, ctx: *mut c_void,
) -> i32 {
    sw842_compress(src, slen, dst, dlen, ctx)
}

unsafe extern "C" fn crypto842_sdecompress(
    _tfm: *mut c_void, src: *const u8, slen: u32,
    dst: *mut u8, dlen: *mut u32, _ctx: *mut c_void,
) -> i32 {
    sw842_decompress(src, slen, dst, dlen)
}

static mut scomp: ScompAlg = ScompAlg {
    streams: ScompStreams {
        alloc_ctx: Some(crypto842_alloc_ctx),
        free_ctx: Some(crypto842_free_ctx),
    },
    compress: Some(crypto842_scompress),
    decompress: Some(crypto842_sdecompress),
    base: CryptoBase {
        cra_name: b"842\0".as_ptr(),
        cra_driver_name: b"842-scomp\0".as_ptr(),
        cra_priority: 100,
        cra_module: core::ptr::null_mut(),
    },
};

unsafe extern "C" fn crypto842_mod_init() -> i32 {
    scomp.base.cra_module = THIS_MODULE;
    crypto_register_scomp(&mut scomp)
}

unsafe extern "C" fn crypto842_mod_exit() {
    crypto_unregister_scomp(&mut scomp);
}

// module_init(crypto842_mod_init);
// module_exit(crypto842_mod_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("842 Software Compression Algorithm");
// MODULE_ALIAS_CRYPTO("842");
// MODULE_ALIAS_CRYPTO("842-generic");
// MODULE_AUTHOR("Dan Streetman <ddstreet@ieee.org>");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
