// SPDX-License-Identifier: GPL-2.0-only
/* Copyright(c) 2026 Intel Corporation */

// Translated from qat_comp_zstd_utils.c.  Linux/ZSTD declarations are supplied
// by the surrounding translation unit.

const ML_BITS: u32 = 4;
const ML_MASK: u32 = (1u32 << ML_BITS) - 1;
const RUN_BITS: u32 = 8 - ML_BITS;
const RUN_MASK: u32 = (1u32 << RUN_BITS) - 1;
const LZ4S_MINMATCH: usize = 2;
const QAT_ZSTD_BLOCK_MAX: usize = ZSTD_BLOCKSIZE_MAX;

unsafe fn emit_delimiter(
    out_seqs: *mut ZSTD_Sequence,
    seqs_idx: *mut usize,
    out_seqs_capacity: usize,
    _lz4s_buff_size: u32,
) -> i32 {
    if *seqs_idx >= out_seqs_capacity - 1 {
        // pr_debug("QAT ZSTD: sequence overflow ...");
        return -EOVERFLOW;
    }

    let seq = out_seqs.add(*seqs_idx);
    (*seq).offset = 0;
    (*seq).litLength = 0;
    (*seq).matchLength = 0;
    *seqs_idx += 1;
    0
}

pub unsafe fn qat_alg_dec_lz4s(
    out_seqs: *mut ZSTD_Sequence,
    out_seqs_capacity: usize,
    lz4s_buff: *mut u8,
    lz4s_buff_size: u32,
    mut literals: *mut u8,
    lit_len: *mut u32,
) -> i32 {
    let end_ip = lz4s_buff.add(lz4s_buff_size as usize);
    let mut hist_literal_len: usize = 0;
    let mut ip = lz4s_buff;
    let mut block_decomp_size: usize = 0;
    let mut seqs_idx: usize = 0;

    *lit_len = 0;
    if lz4s_buff_size == 0 {
        return 0;
    }

    while ip < end_ip {
        let mut literal_len: usize = 0;
        let mut match_len: usize = 0;
        let token = *ip;
        ip = ip.add(1);
        let mut length: usize = (token as u32 >> ML_BITS) as usize;
        if length == RUN_MASK as usize {
            let mut s: u8;
            loop {
                s = *ip;
                ip = ip.add(1);
                length += s as usize;
                if s != 255 { break; }
            }
        }
        literal_len = length;
        let mut start = ip;
        let mut dest = literals;
        let dest_end = literals.add(length);
        while dest.add(QAT_ZSTD_LIT_COPY_LEN as usize) < dest_end {
            std::ptr::copy_nonoverlapping(start, dest, QAT_ZSTD_LIT_COPY_LEN as usize);
            dest = dest.add(QAT_ZSTD_LIT_COPY_LEN as usize);
            start = start.add(QAT_ZSTD_LIT_COPY_LEN as usize);
        }
        std::ptr::copy_nonoverlapping(start, dest, QAT_ZSTD_LIT_COPY_LEN as usize);
        literals = literals.add(length);
        *lit_len += length as u32;
        ip = ip.add(length);

        if ip == end_ip {
            literal_len += hist_literal_len;
            if block_decomp_size + literal_len > QAT_ZSTD_BLOCK_MAX {
                let ret = emit_delimiter(out_seqs, &mut seqs_idx, out_seqs_capacity, lz4s_buff_size);
                if ret != 0 { return ret; }
            }
            let seq = out_seqs.add(seqs_idx);
            (*seq).litLength = literal_len;
            (*seq).offset = 0;
            (*seq).matchLength = match_len;
            break;
        }

        let offset = u16::from_le_bytes([*ip, *ip.add(1)]) as usize;
        ip = ip.add(2);
        length = (token as u32 & ML_MASK) as usize;
        if length == ML_MASK as usize {
            loop {
                let s = *ip;
                ip = ip.add(1);
                length += s as usize;
                if s != 255 { break; }
            }
        }
        if length != 0 {
            length += LZ4S_MINMATCH;
            match_len = length as u16 as usize;
            literal_len += hist_literal_len;
            if block_decomp_size + literal_len + match_len > QAT_ZSTD_BLOCK_MAX {
                let ret = emit_delimiter(out_seqs, &mut seqs_idx, out_seqs_capacity, lz4s_buff_size);
                if ret != 0 { return ret; }
                block_decomp_size = 0;
            }
            let seq = out_seqs.add(seqs_idx);
            (*seq).offset = offset;
            (*seq).litLength = literal_len;
            (*seq).matchLength = match_len;
            hist_literal_len = 0;
            seqs_idx += 1;
            if seqs_idx >= out_seqs_capacity - 1 {
                // pr_debug("QAT ZSTD: sequence overflow ...");
                return -EOVERFLOW;
            }
            block_decomp_size += literal_len + match_len;
        } else if literal_len > 0 {
            hist_literal_len += literal_len;
        }
    }
    (seqs_idx + 1) as i32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
