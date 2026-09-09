/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2026 Namjae Jeon <linkinjeon@kernel.org>
 */

/* Dependency: ../smb2pdu.h */

/*
 * SMB3_COMPRESS_NONE is valid only in chained payload headers. It is never
 * negotiated as a compression algorithm.
 */
#[inline(always)]
fn smb_compress_alg_valid(alg: __le16, valid_none: bool) -> bool {
    if alg == SMB3_COMPRESS_NONE {
        return valid_none;
    }

    alg == SMB3_COMPRESS_LZ77 || alg == SMB3_COMPRESS_PATTERN
}

unsafe extern "C" {
    fn smb_compression_decompress(
        alg: __le16,
        allow_chained: bool,
        allow_pattern: bool,
        src: *const core::ffi::c_void,
        slen: u32,
        dst: *mut core::ffi::c_void,
        dlen: u32,
    ) -> i32;

    fn smb_compression_compress_chained(
        alg: __le16,
        allow_pattern: bool,
        src: *const core::ffi::c_void,
        slen: u32,
        dst: *mut core::ffi::c_void,
        dlen: *mut u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
