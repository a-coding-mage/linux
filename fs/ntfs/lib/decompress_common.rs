// SPDX-License-Identifier: MIT
/*
 * decompress_common.c - Code shared by the XPRESS and LZX decompressors
 *
 * This is a port of the upstream wimlib "decompress_common.c" which builds
 * subtable-based Huffman decode tables, as opposed to the older
 * binary-tree-based format previously used in this library.  The vectorized
 * (SSE2/AVX2) fill paths are omitted for portability in the kernel.
 *
 * Copyright (C) 2022 Eric Biggers
 */

/* Dependency declarations and the MAKE_DECODE_TABLE_ENTRY() helper are
 * supplied by the surrounding translated code. */

unsafe fn compute_subtable_bits(
    table_bits: u32,
    codeword_len: u32,
    len_counts: *mut u16,
) -> u32 {
    let mut subtable_bits = codeword_len - table_bits;
    let mut remainder = (1i32) << subtable_bits;

    loop {
        remainder -= *len_counts.add((table_bits + subtable_bits) as usize) as i32;
        if remainder <= 0 {
            break;
        }
        subtable_bits += 1;
        remainder <<= 1;
    }
    subtable_bits
}

unsafe fn build_subtables(
    decode_table: *mut u16,
    num_syms: u32,
    table_bits: u32,
    len_counts: *mut u16,
    sorted_syms: *const u16,
    mut sym_idx: u32,
    decode_table_pos: u32,
    decode_table_size: u32,
) -> i32 {
    let mut subtable_pos = 1u32 << table_bits;
    let mut subtable_bits = table_bits;
    let mut subtable_prefix = u32::MAX;
    let mut codeword_len = table_bits + 1;
    let mut codeword = decode_table_pos << 1;
    let mut prefix: u32;
    let mut entry: u16;
    let mut n: u32;

    while sym_idx < num_syms {
        while *len_counts.add(codeword_len as usize) == 0 {
            codeword_len += 1;
            codeword <<= 1;
        }

        prefix = codeword >> (codeword_len - table_bits);

        if prefix != subtable_prefix {
            subtable_prefix = prefix;
            subtable_bits = compute_subtable_bits(table_bits, codeword_len, len_counts);
            *decode_table.add(subtable_prefix as usize) =
                MAKE_DECODE_TABLE_ENTRY(subtable_pos, subtable_bits);
        }

        entry = MAKE_DECODE_TABLE_ENTRY(
            *sorted_syms.add(sym_idx as usize),
            codeword_len - table_bits,
        );
        n = 1u32 << (subtable_bits - (codeword_len - table_bits));

        /* Defensive bound check: 'lens' is derived from untrusted on-disk
         * compressed data, and subtable growth depends on its content. */
        if subtable_pos + n > decode_table_size {
            return -1;
        }

        loop {
            *decode_table.add(subtable_pos as usize) = entry;
            subtable_pos += 1;
            n -= 1;
            if n == 0 {
                break;
            }
        }

        *len_counts.add(codeword_len as usize) -= 1;
        codeword += 1;
        sym_idx += 1;
    }

    0
}

/* Build a table for quickly decoding symbols encoded with a canonical prefix
 * code. */
pub unsafe fn make_huffman_decode_table(
    decode_table: *mut u16,
    num_syms: u32,
    table_bits: u32,
    lens: *const u8,
    max_codeword_len: u32,
    working_space: *mut u16,
    decode_table_size: u32,
) -> i32 {
    let len_counts = working_space;
    let offsets = working_space.add((max_codeword_len + 1) as usize);
    let sorted_syms = working_space.add((2 * (max_codeword_len + 1)) as usize);
    let mut decode_table_pos = 0u32;
    let mut sym_idx: u32;
    let mut codeword_len: u32;
    let mut remainder = 1i32;
    let mut entry_ptr = decode_table;
    let mut len: u32;
    let mut sym: u32;

    for len in 0..=max_codeword_len {
        *len_counts.add(len as usize) = 0;
    }
    for sym in 0..num_syms {
        *len_counts.add(*lens.add(sym as usize) as usize) += 1;
    }

    for len in 1..=max_codeword_len {
        remainder = (remainder << 1) - *len_counts.add(len as usize) as i32;
        if remainder < 0 {
            return -1;
        }
    }

    if remainder != 0 {
        if remainder != (1i32 << max_codeword_len) {
            return -1;
        }
        core::ptr::write_bytes(decode_table, 0, 1usize << table_bits);
        return 0;
    }

    *offsets = 0;
    for len in 0..max_codeword_len {
        *offsets.add((len + 1) as usize) =
            *offsets.add(len as usize) + *len_counts.add(len as usize);
    }
    for sym in 0..num_syms {
        let index = *lens.add(sym as usize) as usize;
        let pos = *offsets.add(index) as usize;
        *sorted_syms.add(pos) = sym as u16;
        *offsets.add(index) += 1;
    }

    sym_idx = *offsets as u32;
    codeword_len = 1;
    while codeword_len <= table_bits {
        let stores_per_loop = 1u32 << (table_bits - codeword_len);
        let end_sym_idx = sym_idx + *len_counts.add(codeword_len as usize) as u32;
        while sym_idx < end_sym_idx {
            let v = MAKE_DECODE_TABLE_ENTRY(
                *sorted_syms.add(sym_idx as usize),
                codeword_len,
            );
            let mut n = stores_per_loop;
            let mut p = entry_ptr;
            loop {
                *p = v;
                p = p.add(1);
                n -= 1;
                if n == 0 {
                    break;
                }
            }
            entry_ptr = p;
            sym_idx += 1;
        }
        codeword_len += 1;
    }
    decode_table_pos = entry_ptr.offset_from(decode_table) as u32;

    if sym_idx == num_syms {
        return 0;
    }

    build_subtables(
        decode_table,
        num_syms,
        table_bits,
        len_counts,
        sorted_syms,
        sym_idx,
        decode_table_pos,
        decode_table_size,
    )
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
