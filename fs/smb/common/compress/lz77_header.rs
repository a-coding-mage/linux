/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024-2026, SUSE LLC
 *
 * Authors: Enzo Matsumiya <ematsumiya@suse.de>
 *
 * Implementation of the LZ77 "plain" compression algorithm, as per MS-XCA spec.
 */

use core::ffi::c_void;

/**
 * smb_lz77_compressed_alloc_size() - Compute compressed buffer size.
 * @size:\tuncompressed (src) size
 *
 * Compute allocation size for the compressed buffer based on uncompressed size.
 * Accounts for metadata and overprovision for the worst case scenario.
 *
 * LZ77 metadata is a 4-byte flag that is written:
 * - on dst begin (pos 0)
 * - every 32 literals or matches
 * - on end-of-stream (possibly, if last write was another flag)
 *
 * Worst case scenario is an all-literal compression, which means:
 * metadata bytes = 4 + ((@size / 32) * 4) + 4, or, simplified, (@size >> 3) + 8
 *
 * The worst case scenario rarely happens, but such overprovisioning also
 * allows smb_lz77_compress() main loop to run without ever bound checking dst,
 * which is a huge perf improvement, while also being safe when compression goes
 * bad.
 *
 * Return: required (*) allocation size for compressed buffer.
 *
 * (*) checked once in the beginning of smb_lz77_compress()
 */
#[inline(always)]
pub const fn smb_lz77_compressed_alloc_size(size: u32) -> u32 {
    size.wrapping_add(size >> 3).wrapping_add(8)
}

unsafe extern "C" {
    pub fn smb_lz77_compress(
        src: *const c_void,
        slen: u32,
        dst: *mut c_void,
        dlen: *mut u32,
    ) -> i32;

    pub fn smb_lz77_decompress(
        src: *const c_void,
        slen: u32,
        dst: *mut c_void,
        dlen: u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
