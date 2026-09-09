// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Poly1305 authenticator algorithm, RFC7539
 *
 * Copyright (C) 2015 Martin Willi
 *
 * Based on public domain code by Andrew Moon and Daniel J. Bernstein.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// left external, as in the original implementation.
use crate::{get_unaligned_le32, poly1305_desc_ctx, POLY1305_BLOCK_SIZE,
            POLY1305_KEY_SIZE, round_down};

extern "C" {
    fn poly1305_block_init_generic(state: *mut core::ffi::c_void, key: *const u8);
    fn poly1305_blocks_generic(state: *mut core::ffi::c_void, src: *const u8,
                               len: usize, padbit: u32);
    fn poly1305_emit_generic(h: *const core::ffi::c_void, dst: *mut u8,
                             nonce: *const u32);
}

// CONFIG_CRYPTO_LIB_POLY1305_ARCH selects architecture-specific definitions.
// The generic symbols are used here when no architecture implementation is
// supplied, matching the source-level aliases in the C file.

pub unsafe fn poly1305_init(desc: *mut poly1305_desc_ctx,
                             key: *const u8) {
    (*desc).s[0] = get_unaligned_le32(key.add(16));
    (*desc).s[1] = get_unaligned_le32(key.add(20));
    (*desc).s[2] = get_unaligned_le32(key.add(24));
    (*desc).s[3] = get_unaligned_le32(key.add(28));
    (*desc).buflen = 0;
    poly1305_block_init_generic(&mut (*desc).state as *mut _ as *mut core::ffi::c_void,
                                key);
}

pub unsafe fn poly1305_update(desc: *mut poly1305_desc_ctx,
                              mut src: *const u8, mut nbytes: u32) {
    if (*desc).buflen + nbytes >= POLY1305_BLOCK_SIZE {
        let bulk_len: u32;

        if (*desc).buflen != 0 {
            let l = POLY1305_BLOCK_SIZE - (*desc).buflen;
            core::ptr::copy_nonoverlapping(src, (*desc).buf.as_mut_ptr().add((*desc).buflen as usize), l as usize);
            src = src.add(l as usize);
            nbytes -= l;

            poly1305_blocks_generic(&mut (*desc).state as *mut _ as *mut core::ffi::c_void,
                                    (*desc).buf.as_ptr(), POLY1305_BLOCK_SIZE as usize, 1);
            (*desc).buflen = 0;
        }

        bulk_len = round_down(nbytes, POLY1305_BLOCK_SIZE);
        nbytes %= POLY1305_BLOCK_SIZE;

        if bulk_len != 0 {
            poly1305_blocks_generic(&mut (*desc).state as *mut _ as *mut core::ffi::c_void,
                                    src, bulk_len as usize, 1);
            src = src.add(bulk_len as usize);
        }
    }
    if nbytes != 0 {
        core::ptr::copy_nonoverlapping(src,
            (*desc).buf.as_mut_ptr().add((*desc).buflen as usize), nbytes as usize);
        (*desc).buflen += nbytes;
    }
}

pub unsafe fn poly1305_final(desc: *mut poly1305_desc_ctx, dst: *mut u8) {
    if (*desc).buflen != 0 {
        *(*desc).buf.as_mut_ptr().add((*desc).buflen as usize) = 1;
        (*desc).buflen += 1;
        core::ptr::write_bytes((*desc).buf.as_mut_ptr().add((*desc).buflen as usize), 0,
                               (POLY1305_BLOCK_SIZE - (*desc).buflen) as usize);
        poly1305_blocks_generic(&mut (*desc).state as *mut _ as *mut core::ffi::c_void,
                                (*desc).buf.as_ptr(), POLY1305_BLOCK_SIZE as usize, 0);
    }

    poly1305_emit_generic(&(*desc).state.h as *const _ as *const core::ffi::c_void,
                           dst, (*desc).s.as_ptr());
    *desc = core::mem::zeroed();
}

// EXPORT_SYMBOL(poly1305_init)
// EXPORT_SYMBOL(poly1305_update)
// EXPORT_SYMBOL(poly1305_final)
// Module metadata and optional architecture init/exit hooks are supplied by
// the surrounding kernel integration.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
