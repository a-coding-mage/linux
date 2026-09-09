// SPDX-License-Identifier: GPL-2.0-or-later
/* Key to pathname encoder
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Dependencies supplied by the surrounding kernel translation unit are intentionally
// left as external names, corresponding to the C includes.

static CACHEFILES_CHARMAP: &[u8; 64] =
    b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ_-";

const fn make_filecharmap() -> [u8; 256] {
    let mut map = [0u8; 256];
    let mut i = 33usize;
    while i <= 46 {
        map[i] = 1;
        i += 1;
    }
    i = 48;
    while i <= 127 {
        map[i] = 1;
        i += 1;
    }
    map
}

static CACHEFILES_FILECHARMAP: [u8; 256] = make_filecharmap();

#[inline]
unsafe fn how_many_hex_digits(x: u32) -> u32 {
    if x != 0 {
        (((31 - x.leading_zeros()) + 1 + 3) & !3) / 4
    } else {
        0
    }
}

/*
 * turn the raw key into something cooked
 * - the key may be up to NAME_MAX in length (including the length word)
 *   - "base64" encode the strange keys, mapping 3 bytes of raw to four of
 *     cooked
 *   - need to cut the cooked key into 252 char lengths (189 raw bytes)
 */
pub unsafe fn cachefiles_cook_key(object: *mut cachefiles_object) -> bool {
    let cookie = (*object).cookie;
    let mut key = fscache_get_key(cookie);
    let keylen: u32 = (*cookie).key_len;
    let mut kend: *const u8;
    let mut ch: u8;
    let mut acc: u32;
    let mut i: u32;
    let mut n: u32;
    let mut nle: u32;
    let mut nbe: u32;
    let mut b64len: u32;
    let mut len: u32;
    let mut print: u8;
    let mut pad: u32;
    let name: *mut i8;
    let mut sep: i8;

    _enter!(",%u,%*phN", keylen, keylen, key);
    BUG_ON!(keylen > NAME_MAX - 3);

    print = 1;
    i = 0;
    while i < keylen {
        ch = *key.add(i as usize);
        print &= CACHEFILES_FILECHARMAP[ch as usize];
        i += 1;
    }

    if print != 0 {
        len = 1 + keylen;
        name = kmalloc((len + 1) as usize, GFP_KERNEL);
        if name.is_null() {
            return false;
        }
        *name = b'D' as i8;
        memcpy(name.add(1) as *mut u8, key, keylen as usize);
    } else {
        n = (keylen + 3) & !3;
        nbe = 0;
        nle = 0;
        i = 0;
        while i < n {
            let be = be32_to_cpu((*((key.add(i as usize)) as *const __be32)));
            let le = le32_to_cpu((*((key.add(i as usize)) as *const __le32)));
            nbe += 1 + how_many_hex_digits(be);
            nle += 1 + how_many_hex_digits(le);
            i += 4;
        }

        b64len = (keylen + 2) / 3;
        pad = b64len * 3 - keylen;
        b64len = 2 + b64len * 4;
        _debug!("len=%u nbe=%u nle=%u b64=%u", keylen, nbe, nle, b64len);
        if nbe < b64len || nle < b64len {
            let nlen = (if nbe <= nle { nbe } else { nle }) + 1;
            name = kmalloc(nlen as usize, GFP_KERNEL);
            if name.is_null() {
                return false;
            }
            sep = if nbe <= nle { b'S' as i8 } else { b'T' as i8 };
            len = 0;
            i = 0;
            while i < n {
                let x: u32 = if nbe <= nle {
                    be32_to_cpu((*((key.add(i as usize)) as *const __be32)))
                } else {
                    le32_to_cpu((*((key.add(i as usize)) as *const __le32)))
                };
                *name.add(len as usize) = sep;
                len += 1;
                if x != 0 {
                    len += snprintf(
                        name.add(len as usize),
                        (nlen - len) as usize,
                        b"%x\0".as_ptr() as *const i8,
                        x,
                    );
                }
                sep = b',' as i8;
                i += 4;
            }
        } else {
            name = kmalloc((b64len + 1) as usize, GFP_KERNEL);
            if name.is_null() {
                return false;
            }
            *name = b'E' as i8;
            *name.add(1) = (b'0' as u32 + pad) as i8;
            len = 2;
            kend = key.add(keylen as usize);
            loop {
                acc = *key as u32;
                key = key.add(1);
                if key < kend {
                    acc |= (*key as u32) << 8;
                    key = key.add(1);
                    if key < kend {
                        acc |= (*key as u32) << 16;
                        key = key.add(1);
                    }
                }
                *name.add(len as usize) = CACHEFILES_CHARMAP[(acc & 63) as usize] as i8;
                len += 1;
                acc >>= 6;
                *name.add(len as usize) = CACHEFILES_CHARMAP[(acc & 63) as usize] as i8;
                len += 1;
                acc >>= 6;
                *name.add(len as usize) = CACHEFILES_CHARMAP[(acc & 63) as usize] as i8;
                len += 1;
                acc >>= 6;
                *name.add(len as usize) = CACHEFILES_CHARMAP[(acc & 63) as usize] as i8;
                len += 1;
                if key >= kend {
                    break;
                }
            }
        }
    }

    *name.add(len as usize) = 0;
    (*object).d_name = name;
    _leave!(" = %s", (*object).d_name);
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
