// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2022 Red Hat, Inc.
// Author: Vladis Dronov <vdronoff@gmail.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8 = u8;
type u32 = u32;
type u64 = u64;

const EFAULT: c_int = 14;
const ENOMEM: c_int = 12;
const EAGAIN: c_int = 11;
const GFP_KERNEL: c_uint = 0;
const CRYPTO_TFM_REQ_MAY_BACKLOG: c_uint = 0;
const KERN_INFO: *const c_char = b"\0".as_ptr() as *const c_char;
const DUMP_PREFIX_OFFSET: c_int = 0;

static mut data_size: c_uint = 256;
static mut debug: c_uint = 0;

#[repr(C)]
struct scatterlist {
    _private: [u8; 0],
}

#[repr(C)]
struct crypto_skcipher {
    _private: [u8; 0],
}

#[repr(C)]
struct skcipher_request {
    cryptlen: c_uint,
}

#[repr(C)]
struct crypto_wait {
    _private: [u8; 0],
}

#[repr(C)]
struct chacha_state {
    _private: [u8; 0],
}

#[repr(C)]
struct crypto_alg {
    walksize: c_uint,
}

/* tie all skcipher structures together */
#[repr(C)]
struct skcipher_def {
    sginp: scatterlist,
    sgout: scatterlist,
    tfm: *mut crypto_skcipher,
    req: *mut skcipher_request,
    wait: crypto_wait,
}

extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn print_hex_dump(
        level: *const c_char,
        prefix_str: *const c_char,
        prefix_type: c_int,
        rowsize: c_int,
        groupsize: c_int,
        buf: *const c_void,
        len: usize,
        ascii: bool,
    );
    fn pr_info(fmt: *const c_char, ...);
    fn chacha_init(state: *mut chacha_state, key: *const u32, iv: *const u8);
    fn chacha_crypt_arch(
        state: *mut chacha_state,
        dst: *mut u8,
        src: *const u8,
        bytes: c_uint,
        nrounds: c_int,
    );
    fn ktime_get_ns() -> u64;
    fn crypto_wait_req(err: c_int, wait: *mut crypto_wait) -> c_int;
    fn crypto_skcipher_encrypt(req: *mut skcipher_request) -> c_int;
    fn crypto_skcipher_decrypt(req: *mut skcipher_request) -> c_int;
    fn crypto_alloc_skcipher(name: *const c_char, type_: u32, mask: u32) -> *mut crypto_skcipher;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn skcipher_request_alloc(
        tfm: *mut crypto_skcipher,
        gfp: c_uint,
    ) -> *mut skcipher_request;
    fn skcipher_request_set_callback(
        req: *mut skcipher_request,
        flags: c_uint,
        complete: unsafe extern "C" fn(*mut c_void, c_int),
        data: *mut c_void,
    );
    fn crypto_req_done(req: *mut c_void, err: c_int);
    fn crypto_skcipher_setkey(tfm: *mut crypto_skcipher, key: *const u8, keylen: c_uint)
        -> c_int;
    fn sg_init_one(sg: *mut scatterlist, buf: *const c_void, buflen: c_uint);
    fn skcipher_request_set_crypt(
        req: *mut skcipher_request,
        src: *mut scatterlist,
        dst: *mut scatterlist,
        cryptlen: c_uint,
        iv: *mut u8,
    );
    fn crypto_init_wait(wait: *mut crypto_wait);
    fn crypto_skcipher_blocksize(tfm: *mut crypto_skcipher) -> c_uint;
    fn crypto_skcipher_alg(tfm: *mut crypto_skcipher) -> *mut crypto_alg;
    fn crypto_skcipher_ivsize(tfm: *mut crypto_skcipher) -> c_uint;
    fn crypto_skcipher_alignmask(tfm: *mut crypto_skcipher) -> c_uint;
    fn crypto_free_skcipher(tfm: *mut crypto_skcipher);
    fn skcipher_request_free(req: *mut skcipher_request);
    fn vmalloc(size: c_uint) -> *mut c_void;
    fn vzalloc(size: c_uint) -> *mut c_void;
    fn vfree(addr: *const c_void);
    fn get_random_bytes(buf: *mut c_void, nbytes: c_uint);
}

unsafe extern "C" fn crypto_req_done_callback(req: *mut c_void, err: c_int) {
    crypto_req_done(req, err);
}

/* Perform cipher operations with the chacha lib */
unsafe fn test_lib_chacha(revert: *mut u8, cipher: *mut u8, plain: *mut u8) -> c_int {
    let mut chacha_state: chacha_state = core::mem::zeroed();
    let mut iv: [u8; 16] = [0; 16];
    let mut key: [u8; 32] = [0; 32];
    let mut start: u64;
    let mut end: u64;

    memset(key.as_mut_ptr() as *mut c_void, b'X' as c_int, core::mem::size_of_val(&key));
    memset(iv.as_mut_ptr() as *mut c_void, b'I' as c_int, core::mem::size_of_val(&iv));

    if debug != 0 {
        print_hex_dump(KERN_INFO, c"key: ".as_ptr(), DUMP_PREFIX_OFFSET, 16, 1, key.as_ptr() as *const c_void, 32, true);

        print_hex_dump(KERN_INFO, c"iv:  ".as_ptr(), DUMP_PREFIX_OFFSET, 16, 1, iv.as_ptr() as *const c_void, 16, true);
    }

    /* Encrypt */
    chacha_init(&mut chacha_state, key.as_ptr() as *const u32, iv.as_ptr());

    start = ktime_get_ns();
    chacha_crypt_arch(&mut chacha_state, cipher, plain, data_size, 20);
    end = ktime_get_ns();

    if debug != 0 {
        print_hex_dump(KERN_INFO, c"encr:".as_ptr(), DUMP_PREFIX_OFFSET, 16, 1, cipher as *const c_void, if data_size > 64 { 64 } else { data_size } as usize, true);
    }

    pr_info(c"lib encryption took: %lld nsec".as_ptr(), end.wrapping_sub(start));

    /* Decrypt */
    chacha_init(&mut chacha_state, key.as_ptr() as *const u32, iv.as_ptr());

    start = ktime_get_ns();
    chacha_crypt_arch(&mut chacha_state, revert, cipher, data_size, 20);
    end = ktime_get_ns();

    if debug != 0 {
        print_hex_dump(KERN_INFO, c"decr:".as_ptr(), DUMP_PREFIX_OFFSET, 16, 1, revert as *const c_void, if data_size > 64 { 64 } else { data_size } as usize, true);
    }

    pr_info(c"lib decryption took: %lld nsec".as_ptr(), end.wrapping_sub(start));

    0
}

/* Perform cipher operations with skcipher */
unsafe fn test_skcipher_encdec(sk: *mut skcipher_def, enc: c_int) -> c_uint {
    let rc: c_int;

    if enc != 0 {
        rc = crypto_wait_req(crypto_skcipher_encrypt((*sk).req), &mut (*sk).wait);
        if rc != 0 {
            pr_info(c"skcipher encrypt returned with result%d\n".as_ptr(), rc);
        }
    } else {
        rc = crypto_wait_req(crypto_skcipher_decrypt((*sk).req), &mut (*sk).wait);
        if rc != 0 {
            pr_info(c"skcipher decrypt returned with result%d\n".as_ptr(), rc);
        }
    }

    rc as c_uint
}

/* Initialize and trigger cipher operations */
unsafe fn test_skcipher(name: *mut c_char, revert: *mut u8, cipher: *mut u8, plain: *mut u8) -> c_int {
    let mut sk: skcipher_def = core::mem::zeroed();
    let mut skcipher: *mut crypto_skcipher = core::ptr::null_mut();
    let mut req: *mut skcipher_request = core::ptr::null_mut();
    let mut iv: [u8; 16] = [0; 16];
    let mut key: [u8; 32] = [0; 32];
    let mut start: u64;
    let mut end: u64;
    let mut ret: c_int = -EFAULT;

    skcipher = crypto_alloc_skcipher(name, 0, 0);
    if IS_ERR(skcipher as *const c_void) {
        pr_info(c"could not allocate skcipher %s handle\n".as_ptr(), name);
        return PTR_ERR(skcipher as *const c_void);
    }

    req = skcipher_request_alloc(skcipher, GFP_KERNEL);
    if req.is_null() {
        pr_info(c"could not allocate skcipher request\n".as_ptr());
        ret = -ENOMEM;
        goto_out(&mut skcipher, &mut req);
        return ret;
    }

    skcipher_request_set_callback(req, CRYPTO_TFM_REQ_MAY_BACKLOG, crypto_req_done_callback, &mut sk.wait as *mut _ as *mut c_void);

    memset(key.as_mut_ptr() as *mut c_void, b'X' as c_int, core::mem::size_of_val(&key));
    memset(iv.as_mut_ptr() as *mut c_void, b'I' as c_int, core::mem::size_of_val(&iv));

    if crypto_skcipher_setkey(skcipher, key.as_ptr(), 32) != 0 {
        pr_info(c"key could not be set\n".as_ptr());
        ret = -EAGAIN;
        goto_out(&mut skcipher, &mut req);
        return ret;
    }

    if debug != 0 {
        print_hex_dump(KERN_INFO, c"key: ".as_ptr(), DUMP_PREFIX_OFFSET, 16, 1, key.as_ptr() as *const c_void, 32, true);

        print_hex_dump(KERN_INFO, c"iv:  ".as_ptr(), DUMP_PREFIX_OFFSET, 16, 1, iv.as_ptr() as *const c_void, 16, true);
    }

    sk.tfm = skcipher;
    sk.req = req;

    /* Encrypt in one pass */
    sg_init_one(&mut sk.sginp, plain as *const c_void, data_size);
    sg_init_one(&mut sk.sgout, cipher as *const c_void, data_size);
    skcipher_request_set_crypt(req, &mut sk.sginp, &mut sk.sgout, data_size, iv.as_mut_ptr());
    crypto_init_wait(&mut sk.wait);

    /* Encrypt data */
    start = ktime_get_ns();
    ret = test_skcipher_encdec(&mut sk, 1) as c_int;
    end = ktime_get_ns();

    if ret != 0 {
        goto_out(&mut skcipher, &mut req);
        return ret;
    }

    pr_info(c"%s tfm encryption successful, took %lld nsec\n".as_ptr(), name, end.wrapping_sub(start));

    if debug != 0 {
        print_hex_dump(KERN_INFO, c"encr:".as_ptr(), DUMP_PREFIX_OFFSET, 16, 1, cipher as *const c_void, if data_size > 64 { 64 } else { data_size } as usize, true);
    }

    /* Prepare for decryption */
    memset(iv.as_mut_ptr() as *mut c_void, b'I' as c_int, core::mem::size_of_val(&iv));

    sg_init_one(&mut sk.sginp, cipher as *const c_void, data_size);
    sg_init_one(&mut sk.sgout, revert as *const c_void, data_size);
    skcipher_request_set_crypt(req, &mut sk.sginp, &mut sk.sgout, data_size, iv.as_mut_ptr());
    crypto_init_wait(&mut sk.wait);

    /* Decrypt data */
    start = ktime_get_ns();
    ret = test_skcipher_encdec(&mut sk, 0) as c_int;
    end = ktime_get_ns();

    if ret != 0 {
        goto_out(&mut skcipher, &mut req);
        return ret;
    }

    pr_info(c"%s tfm decryption successful, took %lld nsec\n".as_ptr(), name, end.wrapping_sub(start));

    if debug != 0 {
        print_hex_dump(KERN_INFO, c"decr:".as_ptr(), DUMP_PREFIX_OFFSET, 16, 1, revert as *const c_void, if data_size > 64 { 64 } else { data_size } as usize, true);
    }

    /* Dump some internal skcipher data */
    if debug != 0 {
        pr_info(
            c"skcipher %s: cryptlen %d blksize %d stride %d ivsize %d alignmask 0x%x\n".as_ptr(),
            name,
            (*sk.req).cryptlen,
            crypto_skcipher_blocksize(sk.tfm),
            (*crypto_skcipher_alg(sk.tfm)).walksize,
            crypto_skcipher_ivsize(sk.tfm),
            crypto_skcipher_alignmask(sk.tfm),
        );
    }

    goto_out(&mut skcipher, &mut req);
    ret
}

unsafe fn goto_out(skcipher: &mut *mut crypto_skcipher, req: &mut *mut skcipher_request) {
    if !(*skcipher).is_null() {
        crypto_free_skcipher(*skcipher);
    }
    if !(*req).is_null() {
        skcipher_request_free(*req);
    }
}

unsafe fn chacha_s390_test_init() -> c_int {
    let mut plain: *mut u8 = core::ptr::null_mut();
    let mut revert: *mut u8 = core::ptr::null_mut();
    let mut cipher_generic: *mut u8 = core::ptr::null_mut();
    let mut cipher_s390: *mut u8 = core::ptr::null_mut();
    let mut ret: c_int = -1;

    pr_info(c"s390 ChaCha20 test module: size=%d debug=%d\n".as_ptr(), data_size, debug);

    /* Allocate and fill buffers */
    plain = vmalloc(data_size) as *mut u8;
    if plain.is_null() {
        pr_info(c"could not allocate plain buffer\n".as_ptr());
        ret = -2;
        chacha_s390_test_out(plain, cipher_generic, cipher_s390, revert);
        return -1;
    }
    memset(plain as *mut c_void, b'a' as c_int, data_size as usize);
    get_random_bytes(plain as *mut c_void, if data_size > 256 { 256 } else { data_size });

    cipher_generic = vzalloc(data_size) as *mut u8;
    if cipher_generic.is_null() {
        pr_info(c"could not allocate cipher_generic buffer\n".as_ptr());
        ret = -2;
        chacha_s390_test_out(plain, cipher_generic, cipher_s390, revert);
        return -1;
    }

    cipher_s390 = vzalloc(data_size) as *mut u8;
    if cipher_s390.is_null() {
        pr_info(c"could not allocate cipher_s390 buffer\n".as_ptr());
        ret = -2;
        chacha_s390_test_out(plain, cipher_generic, cipher_s390, revert);
        return -1;
    }

    revert = vzalloc(data_size) as *mut u8;
    if revert.is_null() {
        pr_info(c"could not allocate revert buffer\n".as_ptr());
        ret = -2;
        chacha_s390_test_out(plain, cipher_generic, cipher_s390, revert);
        return -1;
    }

    if debug != 0 {
        print_hex_dump(KERN_INFO, c"src: ".as_ptr(), DUMP_PREFIX_OFFSET, 16, 1, plain as *const c_void, if data_size > 64 { 64 } else { data_size } as usize, true);
    }

    /* Use chacha20 generic */
    ret = test_skcipher(c"chacha20-generic".as_ptr() as *mut c_char, revert, cipher_generic, plain);
    if ret != 0 {
        chacha_s390_test_out(plain, cipher_generic, cipher_s390, revert);
        return -1;
    }

    if memcmp(plain as *const c_void, revert as *const c_void, data_size as usize) != 0 {
        pr_info(c"generic en/decryption check FAILED\n".as_ptr());
        ret = -2;
        chacha_s390_test_out(plain, cipher_generic, cipher_s390, revert);
        return -1;
    } else {
        pr_info(c"generic en/decryption check OK\n".as_ptr());
    }

    memset(revert as *mut c_void, 0, data_size as usize);

    /* Use chacha20 s390 */
    ret = test_skcipher(c"chacha20-s390".as_ptr() as *mut c_char, revert, cipher_s390, plain);
    if ret != 0 {
        chacha_s390_test_out(plain, cipher_generic, cipher_s390, revert);
        return -1;
    }

    if memcmp(plain as *const c_void, revert as *const c_void, data_size as usize) != 0 {
        pr_info(c"s390 en/decryption check FAILED\n".as_ptr());
        ret = -2;
        chacha_s390_test_out(plain, cipher_generic, cipher_s390, revert);
        return -1;
    } else {
        pr_info(c"s390 en/decryption check OK\n".as_ptr());
    }

    if memcmp(cipher_generic as *const c_void, cipher_s390 as *const c_void, data_size as usize) != 0 {
        pr_info(c"s390 vs generic check FAILED\n".as_ptr());
        ret = -2;
        chacha_s390_test_out(plain, cipher_generic, cipher_s390, revert);
        return -1;
    } else {
        pr_info(c"s390 vs generic check OK\n".as_ptr());
    }

    memset(cipher_s390 as *mut c_void, 0, data_size as usize);
    memset(revert as *mut c_void, 0, data_size as usize);

    /* Use chacha20 lib */
    test_lib_chacha(revert, cipher_s390, plain);

    if memcmp(plain as *const c_void, revert as *const c_void, data_size as usize) != 0 {
        pr_info(c"lib en/decryption check FAILED\n".as_ptr());
        ret = -2;
        chacha_s390_test_out(plain, cipher_generic, cipher_s390, revert);
        return -1;
    } else {
        pr_info(c"lib en/decryption check OK\n".as_ptr());
    }

    if memcmp(cipher_generic as *const c_void, cipher_s390 as *const c_void, data_size as usize) != 0 {
        pr_info(c"lib vs generic check FAILED\n".as_ptr());
        ret = -2;
        chacha_s390_test_out(plain, cipher_generic, cipher_s390, revert);
        return -1;
    } else {
        pr_info(c"lib vs generic check OK\n".as_ptr());
    }

    pr_info(c"--- chacha20 s390 test end ---\n".as_ptr());

    chacha_s390_test_out(plain, cipher_generic, cipher_s390, revert);

    -1
}

unsafe fn chacha_s390_test_out(
    plain: *mut u8,
    cipher_generic: *mut u8,
    cipher_s390: *mut u8,
    revert: *mut u8,
) {
    if !plain.is_null() {
        vfree(plain as *const c_void);
    }
    if !cipher_generic.is_null() {
        vfree(cipher_generic as *const c_void);
    }
    if !cipher_s390.is_null() {
        vfree(cipher_s390 as *const c_void);
    }
    if !revert.is_null() {
        vfree(revert as *const c_void);
    }
}

unsafe fn chacha_s390_test_exit() {
    pr_info(c"s390 ChaCha20 test module exit\n".as_ptr());
}

// module_param_named(size, data_size, uint, 0660);
// module_param(debug, int, 0660);
// MODULE_PARM_DESC(size, "Size of a plaintext");
// MODULE_PARM_DESC(debug, "Debug level (0=off,1=on)");
//
// module_init(chacha_s390_test_init);
// module_exit(chacha_s390_test_exit);
//
// MODULE_DESCRIPTION("s390 ChaCha20 self-test");
// MODULE_AUTHOR("Vladis Dronov <vdronoff@gmail.com>");
// MODULE_LICENSE("GPL v2");
