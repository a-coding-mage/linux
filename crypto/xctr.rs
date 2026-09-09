// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * XCTR: XOR Counter mode - Adapted from ctr.c
 *
 * (C) Copyright IBM Corp. 2007 - Joy Latten <latten@us.ibm.com>
 * Copyright 2021 Google LLC
 */

/*
 * XCTR mode is a blockcipher mode of operation used to implement HCTR2. XCTR is
 * closely related to the CTR mode of operation; the main difference is that CTR
 * generates the keystream using E(CTR + IV) whereas XCTR generates the
 * keystream using E(CTR ^ IV). This allows implementations to avoid dealing
 * with multi-limb integers (as is required in CTR mode). XCTR is also specified
 * using little-endian arithmetic which makes it slightly faster on LE machines.
 *
 * See the HCTR2 paper for more details:
 *\tLength-preserving encryption with HCTR2
 *      (https://eprint.iacr.org/2021/1441.pdf)
 */

// For now this implementation is limited to 16-byte blocks for simplicity.
const XCTR_BLOCKSIZE: usize = 16;

unsafe fn crypto_xctr_crypt_final(
    walk: *mut skcipher_walk,
    tfm: *mut crypto_cipher,
    byte_ctr: u32,
) {
    let mut keystream = [0u8; XCTR_BLOCKSIZE];
    let src = (*walk).src.virt.addr;
    let dst = (*walk).dst.virt.addr;
    let nbytes = (*walk).nbytes;
    let ctr32 = (byte_ctr / XCTR_BLOCKSIZE as u32 + 1).to_le_bytes();

    crypto_xor((*walk).iv, ctr32.as_ptr(), core::mem::size_of::<u32>());
    crypto_cipher_encrypt_one(tfm, keystream.as_mut_ptr(), (*walk).iv);
    crypto_xor_cpy(dst, keystream.as_ptr(), src, nbytes);
    crypto_xor((*walk).iv, ctr32.as_ptr(), core::mem::size_of::<u32>());
}

unsafe fn crypto_xctr_crypt_segment(
    walk: *mut skcipher_walk,
    tfm: *mut crypto_cipher,
    byte_ctr: u32,
) -> i32 {
    let fn_: unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8) =
        (*crypto_cipher_alg(tfm)).cia_encrypt;
    let mut src = (*walk).src.virt.addr;
    let mut dst = (*walk).dst.virt.addr;
    let mut nbytes = (*walk).nbytes;
    let mut ctr32 = byte_ctr / XCTR_BLOCKSIZE as u32 + 1;

    loop {
        let ctr = ctr32.to_le_bytes();
        crypto_xor((*walk).iv, ctr.as_ptr(), core::mem::size_of::<u32>());
        fn_(crypto_cipher_tfm(tfm), dst, (*walk).iv);
        crypto_xor(dst, src, XCTR_BLOCKSIZE);
        crypto_xor((*walk).iv, ctr.as_ptr(), core::mem::size_of::<u32>());

        ctr32 = ctr32.wrapping_add(1);
        src = src.add(XCTR_BLOCKSIZE);
        dst = dst.add(XCTR_BLOCKSIZE);
        nbytes -= XCTR_BLOCKSIZE;
        if nbytes < XCTR_BLOCKSIZE {
            break;
        }
    }
    nbytes as i32
}

unsafe fn crypto_xctr_crypt_inplace(
    walk: *mut skcipher_walk,
    tfm: *mut crypto_cipher,
    byte_ctr: u32,
) -> i32 {
    let fn_: unsafe extern "C" fn(*mut crypto_tfm, *mut u8, *const u8) =
        (*crypto_cipher_alg(tfm)).cia_encrypt;
    let alignmask = crypto_cipher_alignmask(tfm);
    let mut nbytes = (*walk).nbytes;
    let mut data = (*walk).dst.virt.addr;
    let mut tmp = [0u8; XCTR_BLOCKSIZE + MAX_CIPHER_ALIGNMASK];
    let keystream = ((tmp.as_mut_ptr() as usize + alignmask) & !alignmask) as *mut u8;
    let mut ctr32 = byte_ctr / XCTR_BLOCKSIZE as u32 + 1;

    loop {
        let ctr = ctr32.to_le_bytes();
        crypto_xor((*walk).iv, ctr.as_ptr(), core::mem::size_of::<u32>());
        fn_(crypto_cipher_tfm(tfm), keystream, (*walk).iv);
        crypto_xor(data, keystream, XCTR_BLOCKSIZE);
        crypto_xor((*walk).iv, ctr.as_ptr(), core::mem::size_of::<u32>());

        ctr32 = ctr32.wrapping_add(1);
        data = data.add(XCTR_BLOCKSIZE);
        nbytes -= XCTR_BLOCKSIZE;
        if nbytes < XCTR_BLOCKSIZE {
            break;
        }
    }
    nbytes as i32
}

unsafe fn crypto_xctr_crypt(req: *mut skcipher_request) -> i32 {
    let tfm = crypto_skcipher_reqtfm(req);
    let cipher = skcipher_cipher_simple(tfm);
    let mut walk: skcipher_walk = core::mem::zeroed();
    let mut err = skcipher_walk_virt(&mut walk, req, false);
    let mut byte_ctr: u32 = 0;

    while walk.nbytes >= XCTR_BLOCKSIZE {
        let nbytes = if walk.src.virt.addr == walk.dst.virt.addr {
            crypto_xctr_crypt_inplace(&mut walk, cipher, byte_ctr) as usize
        } else {
            crypto_xctr_crypt_segment(&mut walk, cipher, byte_ctr) as usize
        };
        byte_ctr = byte_ctr.wrapping_add((walk.nbytes - nbytes) as u32);
        err = skcipher_walk_done(&mut walk, nbytes);
    }

    if walk.nbytes != 0 {
        crypto_xctr_crypt_final(&mut walk, cipher, byte_ctr);
        err = skcipher_walk_done(&mut walk, 0);
    }
    err
}

unsafe fn crypto_xctr_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> i32 {
    let inst = skcipher_alloc_instance_simple(tmpl, tb);
    if is_err(inst as *mut _) {
        return ptr_err(inst as *mut _);
    }
    let alg = skcipher_ialg_simple(inst);
    let mut err = -EINVAL;
    if (*alg).cra_blocksize != XCTR_BLOCKSIZE {
        (*inst).free(inst);
        return err;
    }
    (*inst).alg.base.cra_blocksize = 1;
    (*inst).alg.chunksize = (*alg).cra_blocksize;
    (*inst).alg.encrypt = Some(crypto_xctr_crypt);
    (*inst).alg.decrypt = Some(crypto_xctr_crypt);
    err = skcipher_register_instance(tmpl, inst);
    if err != 0 {
        (*inst).free(inst);
    }
    err
}

static mut crypto_xctr_tmpl: crypto_template = crypto_template {
    name: b"xctr\0".as_ptr() as *const i8,
    create: Some(crypto_xctr_create),
    module: THIS_MODULE,
};

unsafe extern "C" fn crypto_xctr_module_init() -> i32 {
    crypto_register_template(&mut crypto_xctr_tmpl)
}

unsafe extern "C" fn crypto_xctr_module_exit() {
    crypto_unregister_template(&mut crypto_xctr_tmpl);
}

// module_init(crypto_xctr_module_init);
// module_exit(crypto_xctr_module_exit);
// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("XCTR block cipher mode of operation");
// MODULE_ALIAS_CRYPTO("xctr");
// MODULE_IMPORT_NS("CRYPTO_INTERNAL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
