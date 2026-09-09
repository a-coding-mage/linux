// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * CMAC: Cipher Block Mode for Authentication
 *
 * Copyright © 2013 Jussi Kivilinna <jussi.kivilinna@iki.fi>
 *
 * Based on work by:
 *  Copyright © 2013 Tom St Denis <tstdenis@elliptictech.com>
 * Based on crypto/xcbc.c:
 *  Copyright © 2006 USAGI/WIDE Project,
 *   Author: Kazunori Miyazawa <miyazawa@linux-ipv6.org>
 */

// C dependencies supplied by the surrounding kernel crypto implementation.

#[repr(C)]
pub struct cmac_tfm_ctx {
    pub child: *mut crypto_cipher,
    pub consts: [__be64; 0],
}

unsafe fn crypto_cmac_digest_setkey(
    parent: *mut crypto_shash,
    inkey: *const u8,
    keylen: u32,
) -> i32 {
    let ctx = crypto_shash_ctx(parent);
    let bs = crypto_shash_blocksize(parent);
    let consts = (*ctx).consts.as_mut_ptr();
    let mut _const = [0u64; 2];
    let mut err = 0i32;
    let mut i: i32;
    let mut msb_mask: u8;
    let gfmask: u8;

    err = crypto_cipher_setkey((*ctx).child, inkey, keylen);
    if err != 0 {
        return err;
    }

    /* encrypt the zero block */
    core::ptr::write_bytes(consts as *mut u8, 0, bs as usize);
    crypto_cipher_encrypt_one((*ctx).child, consts as *mut u8, consts as *mut u8);

    match bs {
        16 => {
            gfmask = 0x87;
            _const[0] = be64_to_cpu(*consts.add(1));
            _const[1] = be64_to_cpu(*consts.add(0));

            /* gf(2^128) multiply zero-ciphertext with u and u^2 */
            i = 0;
            while i < 4 {
                msb_mask = (((_const[1] as i64) >> 63) as u8) & gfmask;
                _const[1] = (_const[1] << 1) | (_const[0] >> 63);
                _const[0] = (_const[0] << 1) ^ msb_mask as u64;
                *consts.add((i + 0) as usize) = cpu_to_be64(_const[1]);
                *consts.add((i + 1) as usize) = cpu_to_be64(_const[0]);
                i += 2;
            }
        }
        8 => {
            gfmask = 0x1B;
            _const[0] = be64_to_cpu(*consts);

            /* gf(2^64) multiply zero-ciphertext with u and u^2 */
            i = 0;
            while i < 2 {
                msb_mask = (((_const[0] as i64) >> 63) as u8) & gfmask;
                _const[0] = (_const[0] << 1) ^ msb_mask as u64;
                *consts.add(i as usize) = cpu_to_be64(_const[0]);
                i += 1;
            }
        }
        _ => {}
    }

    0
}

unsafe fn crypto_cmac_digest_init(pdesc: *mut shash_desc) -> i32 {
    let bs = crypto_shash_blocksize((*pdesc).tfm);
    let prev = shash_desc_ctx(pdesc);
    core::ptr::write_bytes(prev, 0, bs as usize);
    0
}

unsafe fn crypto_cmac_digest_update(
    pdesc: *mut shash_desc,
    mut p: *const u8,
    mut len: u32,
) -> i32 {
    let parent = (*pdesc).tfm;
    let tctx = crypto_shash_ctx(parent);
    let tfm = (*tctx).child;
    let bs = crypto_shash_blocksize(parent) as usize;
    let prev = shash_desc_ctx(pdesc);

    loop {
        crypto_xor(prev, p, bs as u32);
        crypto_cipher_encrypt_one(tfm, prev, prev);
        p = p.add(bs);
        len -= bs as u32;
        if len < bs as u32 {
            break;
        }
    }
    len as i32
}

unsafe fn crypto_cmac_digest_finup(
    pdesc: *mut shash_desc,
    src: *const u8,
    len: u32,
    out: *mut u8,
) -> i32 {
    let parent = (*pdesc).tfm;
    let tctx = crypto_shash_ctx(parent);
    let tfm = (*tctx).child;
    let bs = crypto_shash_blocksize(parent) as usize;
    let prev = shash_desc_ctx(pdesc);
    let mut offset = 0usize;

    crypto_xor(prev, src, len);
    if len as usize != bs {
        *prev.add(len as usize) ^= 0x80;
        offset += bs;
    }
    crypto_xor(prev, ((*tctx).consts.as_ptr() as *const u8).add(offset), bs as u32);
    crypto_cipher_encrypt_one(tfm, out, prev);
    0
}

unsafe fn cmac_init_tfm(tfm: *mut crypto_shash) -> i32 {
    let inst = shash_alg_instance(tfm);
    let ctx = crypto_shash_ctx(tfm);
    let spawn = shash_instance_ctx(inst);
    let cipher = crypto_spawn_cipher(spawn);
    if is_err(cipher) {
        return ptr_err(cipher);
    }
    (*ctx).child = cipher;
    0
}

unsafe fn cmac_exit_tfm(tfm: *mut crypto_shash) {
    let ctx = crypto_shash_ctx(tfm);
    crypto_free_cipher((*ctx).child);
}

unsafe fn cmac_create(tmpl: *mut crypto_template, tb: *mut *mut rtattr) -> i32 {
    let mut inst: *mut shash_instance;
    let spawn: *mut crypto_cipher_spawn;
    let alg: *mut crypto_alg;
    let mut mask = 0u32;
    let mut err: i32;

    err = crypto_check_attr_type(tb, CRYPTO_ALG_TYPE_SHASH, &mut mask);
    if err != 0 {
        return err;
    }

    inst = kzalloc(
        core::mem::size_of::<shash_instance>() + core::mem::size_of::<crypto_cipher_spawn>(),
        GFP_KERNEL,
    );
    if inst.is_null() {
        return -ENOMEM;
    }
    spawn = shash_instance_ctx(inst);

    err = crypto_grab_cipher(
        spawn,
        shash_crypto_instance(inst),
        crypto_attr_alg_name(*tb.add(1)),
        0,
        mask,
    );
    if err != 0 {
        shash_free_singlespawn_instance(inst);
        return err;
    }
    alg = crypto_spawn_cipher_alg(spawn);

    match (*alg).cra_blocksize {
        16 | 8 => {}
        _ => {
            err = -EINVAL;
            shash_free_singlespawn_instance(inst);
            return err;
        }
    }

    err = crypto_inst_setname(shash_crypto_instance(inst), (*tmpl).name, alg);
    if err != 0 {
        shash_free_singlespawn_instance(inst);
        return err;
    }

    (*inst).alg.base.cra_priority = (*alg).cra_priority;
    (*inst).alg.base.cra_blocksize = (*alg).cra_blocksize;
    (*inst).alg.base.cra_ctxsize = core::mem::size_of::<cmac_tfm_ctx>()
        + (*alg).cra_blocksize as usize * 2;
    (*inst).alg.base.cra_flags = CRYPTO_AHASH_ALG_BLOCK_ONLY
        | CRYPTO_AHASH_ALG_FINAL_NONZERO;
    (*inst).alg.digestsize = (*alg).cra_blocksize;
    (*inst).alg.descsize = (*alg).cra_blocksize;
    (*inst).alg.init = Some(crypto_cmac_digest_init);
    (*inst).alg.update = Some(crypto_cmac_digest_update);
    (*inst).alg.finup = Some(crypto_cmac_digest_finup);
    (*inst).alg.setkey = Some(crypto_cmac_digest_setkey);
    (*inst).alg.init_tfm = Some(cmac_init_tfm);
    (*inst).alg.exit_tfm = Some(cmac_exit_tfm);
    (*inst).free = Some(shash_free_singlespawn_instance);

    err = shash_register_instance(tmpl, inst);
    if err != 0 {
        shash_free_singlespawn_instance(inst);
    }
    err
}

static mut crypto_cmac_tmpl: crypto_template = crypto_template {
    name: "cmac",
    create: Some(cmac_create),
    module: THIS_MODULE,
};

unsafe fn crypto_cmac_module_init() -> i32 {
    crypto_register_template(&mut crypto_cmac_tmpl)
}

unsafe fn crypto_cmac_module_exit() {
    crypto_unregister_template(&mut crypto_cmac_tmpl);
}

// module_init(crypto_cmac_module_init);
// module_exit(crypto_cmac_module_exit);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
