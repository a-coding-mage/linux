// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Cryptographic API.
 *
 * Single-block cipher operations.
 *
 * Copyright (c) 2002 James Morris <jmorris@intercode.com.au>
 * Copyright (c) 2005 Herbert Xu <herbert@gondor.apana.org.au>
 */

// Dependencies supplied by the corresponding kernel headers and "internal.h".

unsafe fn setkey_unaligned(
    tfm: *mut crypto_cipher,
    key: *const u8,
    keylen: u32,
) -> i32 {
    let cia = crypto_cipher_alg(tfm);
    let alignmask = crypto_cipher_alignmask(tfm) as usize;
    let absize = keylen as usize + alignmask;
    let buffer = kmalloc(absize, GFP_ATOMIC);
    if buffer.is_null() {
        return -ENOMEM;
    }

    let alignbuffer = (((buffer as usize) + alignmask) & !alignmask) as *mut u8;
    core::ptr::copy_nonoverlapping(key, alignbuffer, keylen as usize);
    let ret = ((*cia).cia_setkey)(crypto_cipher_tfm(tfm), alignbuffer, keylen);
    kfree_sensitive(buffer);
    ret
}

pub unsafe fn crypto_cipher_setkey(
    tfm: *mut crypto_cipher,
    key: *const u8,
    keylen: u32,
) -> i32 {
    let cia = crypto_cipher_alg(tfm);
    let alignmask = crypto_cipher_alignmask(tfm) as usize;

    if keylen < (*cia).cia_min_keysize || keylen > (*cia).cia_max_keysize {
        return -EINVAL;
    }

    if (key as usize & alignmask) != 0 {
        return setkey_unaligned(tfm, key, keylen);
    }

    ((*cia).cia_setkey)(crypto_cipher_tfm(tfm), key as *mut u8, keylen)
}

// EXPORT_SYMBOL_NS_GPL(crypto_cipher_setkey, "CRYPTO_INTERNAL");

#[inline]
unsafe fn cipher_crypt_one(
    tfm: *mut crypto_cipher,
    dst: *mut u8,
    src: *const u8,
    enc: bool,
) {
    let alignmask = crypto_cipher_alignmask(tfm) as usize;
    let cia = crypto_cipher_alg(tfm);
    let fn_ptr = if enc {
        (*cia).cia_encrypt
    } else {
        (*cia).cia_decrypt
    };

    if ((dst as usize | src as usize) & alignmask) != 0 {
        let bs = crypto_cipher_blocksize(tfm) as usize;
        let mut buffer = [0u8; MAX_CIPHER_BLOCKSIZE + MAX_CIPHER_ALIGNMASK];
        let tmp = ((buffer.as_mut_ptr() as usize + alignmask) & !alignmask) as *mut u8;

        core::ptr::copy_nonoverlapping(src, tmp, bs);
        fn_ptr(crypto_cipher_tfm(tfm), tmp, tmp);
        core::ptr::copy_nonoverlapping(tmp, dst, bs);
    } else {
        fn_ptr(crypto_cipher_tfm(tfm), dst, src);
    }
}

pub unsafe fn crypto_cipher_encrypt_one(
    tfm: *mut crypto_cipher,
    dst: *mut u8,
    src: *const u8,
) {
    cipher_crypt_one(tfm, dst, src, true);
}

// EXPORT_SYMBOL_NS_GPL(crypto_cipher_encrypt_one, "CRYPTO_INTERNAL");

pub unsafe fn crypto_cipher_decrypt_one(
    tfm: *mut crypto_cipher,
    dst: *mut u8,
    src: *const u8,
) {
    cipher_crypt_one(tfm, dst, src, false);
}

// EXPORT_SYMBOL_NS_GPL(crypto_cipher_decrypt_one, "CRYPTO_INTERNAL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
