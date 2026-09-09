/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2026 Intel Corporation */

// Dependency supplied by the Linux zstd library.

pub const QAT_ZSTD_LIT_COPY_LEN: usize = 8;

unsafe extern "C" {
    pub fn qat_alg_dec_lz4s(
        out_seqs: *mut ZSTD_Sequence,
        out_seqs_capacity: usize,
        lz4s_buff: *mut u8,
        lz4s_buff_size: u32,
        literals: *mut u8,
        lit_len: *mut u32,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
