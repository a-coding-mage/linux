// SPDX-License-Identifier: GPL-2.0-only
/*
 * Bit sliced AES using NEON instructions
 *
 * Copyright (C) 2017 Linaro Ltd <ard.biesheuvel@linaro.org>
 */

// Kernel headers and build-time module macros are supplied by the surrounding
// kernel Rust environment.

extern "C" {
    fn aesbs_convert_key(out: *mut u8, rk: *const u32, rounds: i32);
    fn aesbs_ecb_encrypt(out: *mut u8, input: *const u8, rk: *const u8, rounds: i32, blocks: i32);
    fn aesbs_ecb_decrypt(out: *mut u8, input: *const u8, rk: *const u8, rounds: i32, blocks: i32);
    fn aesbs_cbc_decrypt(out: *mut u8, input: *const u8, rk: *const u8, rounds: i32, blocks: i32, iv: *mut u8);
    fn aesbs_ctr_encrypt(out: *mut u8, input: *const u8, rk: *const u8, rounds: i32, blocks: i32, ctr: *mut u8);
    fn aesbs_xts_encrypt(out: *mut u8, input: *const u8, rk: *const u8, rounds: i32, blocks: i32, iv: *mut u8, arg: i32);
    fn aesbs_xts_decrypt(out: *mut u8, input: *const u8, rk: *const u8, rounds: i32, blocks: i32, iv: *mut u8, arg: i32);
}

#[repr(C)]
struct aesbs_ctx {
    rounds: i32,
    rk: [u8; 13 * (8 * AES_BLOCK_SIZE) + 32],
}

#[repr(C)]
struct aesbs_cbc_ctx {
    key: aesbs_ctx,
    fallback: aes_enckey,
}

#[repr(C)]
struct aesbs_xts_ctx {
    key: aesbs_ctx,
    fallback: aes_key,
    tweak_key: aes_enckey,
}

unsafe fn aesbs_setkey(tfm: *mut crypto_skcipher, in_key: *const u8, key_len: usize) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm) as *mut aesbs_ctx;
    let mut rk: crypto_aes_ctx = core::mem::zeroed();
    let err = aes_expandkey(&mut rk, in_key, key_len);
    if err != 0 { return err; }
    (*ctx).rounds = 6 + (key_len / 4) as i32;
    kernel_neon_begin();
    aesbs_convert_key((*ctx).rk.as_mut_ptr(), rk.key_enc.as_ptr(), (*ctx).rounds);
    kernel_neon_end();
    0
}

unsafe fn __ecb_crypt(req: *mut skcipher_request,
                      func: unsafe extern "C" fn(*mut u8, *const u8, *const u8, i32, i32)) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm) as *mut aesbs_ctx;
    let mut walk: skcipher_walk = core::mem::zeroed();
    let mut err = skcipher_walk_virt(&mut walk, req, false);
    while walk.nbytes >= AES_BLOCK_SIZE {
        let mut blocks = walk.nbytes / AES_BLOCK_SIZE;
        if walk.nbytes < walk.total { blocks = round_down(blocks, walk.stride / AES_BLOCK_SIZE); }
        kernel_neon_begin();
        func(walk.dst.virt.addr, walk.src.virt.addr, (*ctx).rk.as_ptr(), (*ctx).rounds, blocks as i32);
        kernel_neon_end();
        err = skcipher_walk_done(&mut walk, walk.nbytes - blocks * AES_BLOCK_SIZE);
    }
    err
}

unsafe fn ecb_encrypt(req: *mut skcipher_request) -> i32 { __ecb_crypt(req, aesbs_ecb_encrypt) }
unsafe fn ecb_decrypt(req: *mut skcipher_request) -> i32 { __ecb_crypt(req, aesbs_ecb_decrypt) }

unsafe fn aesbs_cbc_setkey(tfm: *mut crypto_skcipher, in_key: *const u8, key_len: usize) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm) as *mut aesbs_cbc_ctx;
    let err = aes_prepareenckey(&mut (*ctx).fallback, in_key, key_len);
    if err != 0 { return err; }
    (*ctx).key.rounds = 6 + (key_len / 4) as i32;
    kernel_neon_begin();
    aesbs_convert_key((*ctx).key.rk.as_mut_ptr(), (*ctx).fallback.k.rndkeys.as_ptr(), (*ctx).key.rounds);
    kernel_neon_end();
    0
}

unsafe fn cbc_encrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let ctx = crypto_skcipher_ctx(tfm) as *const aesbs_cbc_ctx;
    let mut walk: skcipher_walk = core::mem::zeroed();
    let mut err = skcipher_walk_virt(&mut walk, req, false);
    let mut nbytes;
    while { nbytes = walk.nbytes; nbytes >= AES_BLOCK_SIZE } {
        let mut src = walk.src.virt.addr;
        let mut dst = walk.dst.virt.addr;
        let mut prev = walk.iv;
        while nbytes >= AES_BLOCK_SIZE {
            crypto_xor_cpy(dst, src, prev, AES_BLOCK_SIZE);
            aes_encrypt(&(*ctx).fallback, dst, dst);
            prev = dst; src = src.add(AES_BLOCK_SIZE); dst = dst.add(AES_BLOCK_SIZE); nbytes -= AES_BLOCK_SIZE;
        }
        core::ptr::copy_nonoverlapping(prev, walk.iv, AES_BLOCK_SIZE);
        err = skcipher_walk_done(&mut walk, nbytes);
    }
    err
}

unsafe fn cbc_decrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(tfm) as *mut aesbs_cbc_ctx;
    let mut walk: skcipher_walk = core::mem::zeroed(); let mut err = skcipher_walk_virt(&mut walk, req, false);
    while walk.nbytes >= AES_BLOCK_SIZE {
        let mut blocks = walk.nbytes / AES_BLOCK_SIZE;
        if walk.nbytes < walk.total { blocks = round_down(blocks, walk.stride / AES_BLOCK_SIZE); }
        kernel_neon_begin(); aesbs_cbc_decrypt(walk.dst.virt.addr, walk.src.virt.addr, (*ctx).key.rk.as_ptr(), (*ctx).key.rounds, blocks as i32, walk.iv); kernel_neon_end();
        err = skcipher_walk_done(&mut walk, walk.nbytes - blocks * AES_BLOCK_SIZE);
    } err
}

unsafe fn ctr_encrypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(tfm) as *mut aesbs_ctx;
    let mut buf = [0u8; AES_BLOCK_SIZE]; let mut walk: skcipher_walk = core::mem::zeroed(); let mut err = skcipher_walk_virt(&mut walk, req, false);
    while walk.nbytes > 0 {
        let mut src = walk.src.virt.addr; let mut dst = walk.dst.virt.addr; let mut bytes = walk.nbytes;
        if bytes < AES_BLOCK_SIZE { src = buf.as_mut_ptr().add(buf.len() - bytes); dst = src; core::ptr::copy_nonoverlapping(walk.src.virt.addr, src, bytes); }
        else if walk.nbytes < walk.total { bytes &= !(8 * AES_BLOCK_SIZE - 1); }
        kernel_neon_begin(); aesbs_ctr_encrypt(dst, src, (*ctx).rk.as_ptr(), (*ctx).rounds, bytes as i32, walk.iv); kernel_neon_end();
        if bytes < AES_BLOCK_SIZE { core::ptr::copy_nonoverlapping(buf.as_ptr().add(buf.len() - bytes), walk.dst.virt.addr, bytes); }
        err = skcipher_walk_done(&mut walk, walk.nbytes - bytes);
    } err
}

unsafe fn aesbs_xts_setkey(tfm: *mut crypto_skcipher, in_key: *const u8, mut key_len: usize) -> i32 {
    let ctx = crypto_skcipher_ctx(tfm) as *mut aesbs_xts_ctx; let mut err = xts_verify_key(tfm, in_key, key_len); if err != 0 { return err; }
    key_len /= 2; err = aes_preparekey(&mut (*ctx).fallback, in_key, key_len); if err != 0 { return err; }
    err = aes_prepareenckey(&mut (*ctx).tweak_key, in_key.add(key_len), key_len); if err != 0 { return err; }
    aesbs_setkey(tfm, in_key, key_len)
}

// The XTS routine and registration table retain the kernel ABI in the same
// form; dependent kernel types and helpers are declared by the build context.
unsafe fn __xts_crypt(req: *mut skcipher_request, encrypt: bool,
                      func: unsafe extern "C" fn(*mut u8, *const u8, *const u8, i32, i32, *mut u8, i32)) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req); let ctx = crypto_skcipher_ctx(tfm) as *mut aesbs_xts_ctx;
    let rounds = (*ctx).key.rounds; let tail = (*req).cryptlen % AES_BLOCK_SIZE;
    if (*req).cryptlen < AES_BLOCK_SIZE { return -EINVAL; }
    let mut walk: skcipher_walk = core::mem::zeroed(); let mut err = skcipher_walk_virt(&mut walk, req, true); if err != 0 { return err; }
    aes_encrypt(&(*ctx).tweak_key, walk.iv, walk.iv);
    while walk.nbytes >= AES_BLOCK_SIZE {
        let mut blocks = walk.nbytes / AES_BLOCK_SIZE; let mut reorder = (!encrypt && tail > 0) as i32;
        if walk.nbytes < walk.total { blocks = round_down(blocks, walk.stride / AES_BLOCK_SIZE); reorder = 0; }
        kernel_neon_begin(); func(walk.dst.virt.addr, walk.src.virt.addr, (*ctx).key.rk.as_ptr(), rounds, blocks as i32, walk.iv, reorder); kernel_neon_end();
        err = skcipher_walk_done(&mut walk, walk.nbytes - blocks * AES_BLOCK_SIZE);
    }
    if err != 0 || tail == 0 { return err; }
    let mut buf = [0u8; 2 * AES_BLOCK_SIZE];
    scatterwalk_map_and_copy(buf.as_mut_ptr(), (*req).dst, (*req).cryptlen - AES_BLOCK_SIZE, AES_BLOCK_SIZE, 0);
    core::ptr::copy_nonoverlapping(buf.as_ptr(), buf.as_mut_ptr().add(AES_BLOCK_SIZE), tail);
    scatterwalk_map_and_copy(buf.as_mut_ptr(), (*req).src, (*req).cryptlen, tail, 0);
    crypto_xor(buf.as_mut_ptr(), (*req).iv, AES_BLOCK_SIZE);
    if encrypt { aes_encrypt(&(*ctx).fallback, buf.as_mut_ptr(), buf.as_mut_ptr()); }
    else { aes_decrypt(&(*ctx).fallback, buf.as_mut_ptr(), buf.as_mut_ptr()); }
    crypto_xor(buf.as_mut_ptr(), (*req).iv, AES_BLOCK_SIZE);
    scatterwalk_map_and_copy(buf.as_mut_ptr(), (*req).dst, (*req).cryptlen - AES_BLOCK_SIZE, AES_BLOCK_SIZE + tail, 1);
    0
}

unsafe fn xts_encrypt(req: *mut skcipher_request) -> i32 { __xts_crypt(req, true, aesbs_xts_encrypt) }
unsafe fn xts_decrypt(req: *mut skcipher_request) -> i32 { __xts_crypt(req, false, aesbs_xts_decrypt) }

// The following algorithm descriptors, module metadata, and init/exit hooks
// correspond directly to the C registration table and kernel module macros.
#[allow(dead_code)]
static mut AES_ALGS: [skcipher_alg; 4] = [
    skcipher_alg::new("ecb(aes)", "ecb-aes-neonbs", AES_BLOCK_SIZE, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, 8 * AES_BLOCK_SIZE, aesbs_setkey, ecb_encrypt, ecb_decrypt),
    skcipher_alg::new("cbc(aes)", "cbc-aes-neonbs", AES_BLOCK_SIZE, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, 8 * AES_BLOCK_SIZE, aesbs_cbc_setkey, cbc_encrypt, cbc_decrypt),
    skcipher_alg::new("ctr(aes)", "ctr-aes-neonbs", 1, AES_MIN_KEY_SIZE, AES_MAX_KEY_SIZE, 8 * AES_BLOCK_SIZE, aesbs_setkey, ctr_encrypt, ctr_encrypt),
    skcipher_alg::new("xts(aes)", "xts-aes-neonbs", AES_BLOCK_SIZE, 2 * AES_MIN_KEY_SIZE, 2 * AES_MAX_KEY_SIZE, 8 * AES_BLOCK_SIZE, aesbs_xts_setkey, xts_encrypt, xts_decrypt),
];

unsafe fn aes_exit() { crypto_unregister_skciphers(AES_ALGS.as_mut_ptr(), AES_ALGS.len()); }

unsafe fn aes_init() -> i32 {
    if (elf_hwcap & HWCAP_NEON) == 0 { return -ENODEV; }
    crypto_register_skciphers(AES_ALGS.as_mut_ptr(), AES_ALGS.len())
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
