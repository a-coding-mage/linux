// SPDX-License-Identifier: GPL-2.0-only
/*
 * AES-128-CMAC with TLen 16 for IEEE 802.11w BIP
 * Copyright 2008, Jouni Malinen <j@w1.fi>
 * Copyright (C) 2020 Intel Corporation
 */

// Dependencies supplied by the surrounding translation unit/crate:
// linux/kernel.h, linux/types.h, linux/export.h, linux/err.h,
// crypto/aes-cbc-macs.h, net/mac80211.h, key.h, and aes_cmac.h.
use crate::{
    aes_cmac_ctx, aes_cmac_final, aes_cmac_init, aes_cmac_key, aes_cmac_update,
    ieee80211_is_beacon, AES_BLOCK_SIZE, IEEE80211_CMAC_256_MIC_LEN,
};

const AAD_LEN: usize = 20;

static ZERO: [u8; IEEE80211_CMAC_256_MIC_LEN] = [0; IEEE80211_CMAC_256_MIC_LEN];

pub unsafe fn ieee80211_aes_cmac(
    key: *const aes_cmac_key,
    aad: *const u8,
    data: *const u8,
    data_len: usize,
    mic: *mut u8,
    mic_len: u32,
) {
    let mut ctx: aes_cmac_ctx = core::mem::zeroed();
    let mut out: [u8; AES_BLOCK_SIZE] = [0; AES_BLOCK_SIZE];
    let fc: *const u16;

    aes_cmac_init(&mut ctx, key);
    aes_cmac_update(&mut ctx, aad, AAD_LEN);
    fc = aad as *const u16;
    if ieee80211_is_beacon(*fc) {
        /* mask Timestamp field to zero */
        aes_cmac_update(&mut ctx, ZERO.as_ptr(), 8);
        aes_cmac_update(
            &mut ctx,
            data.add(8),
            data_len - 8 - mic_len as usize,
        );
    } else {
        aes_cmac_update(&mut ctx, data, data_len - mic_len as usize);
    }
    aes_cmac_update(&mut ctx, ZERO.as_ptr(), mic_len as usize);
    aes_cmac_final(&mut ctx, out.as_mut_ptr());
    core::ptr::copy_nonoverlapping(out.as_ptr(), mic, mic_len as usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
