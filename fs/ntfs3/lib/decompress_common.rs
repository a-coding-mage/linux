// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * decompress_common.c - Code shared by the XPRESS and LZX decompressors
 *
 * Copyright (C) 2015 Eric Biggers
 */

/* Translated from decompress_common.c. */

/// Build a decoding table for a canonical prefix code.
pub unsafe fn make_huffman_decode_table(
    decode_table: *mut u16,
    num_syms: u32,
    table_bits: u32,
    lens: *const u8,
    max_codeword_len: u32,
    working_space: *mut u16,
) -> i32 {
    let table_num_entries: usize = 1usize << table_bits;
    let len_counts = working_space;
    let offsets = working_space.add((max_codeword_len + 1) as usize);
    let sorted_syms = working_space.add((2 * (max_codeword_len + 1)) as usize);
    let mut left: i32;
    let mut sym_idx: u32;
    let mut codeword_len: u32;
    let mut stores_per_loop: u32;
    let mut decode_table_pos: usize;
    let mut len: u32;
    let mut sym: u32;

    for len in 0..=max_codeword_len {
        *len_counts.add(len as usize) = 0;
    }
    for sym in 0..num_syms {
        let l = *lens.add(sym as usize) as usize;
        *len_counts.add(l) = (*len_counts.add(l)).wrapping_add(1);
    }

    left = 1;
    for len in 1..=max_codeword_len {
        left <<= 1;
        left -= *len_counts.add(len as usize) as i32;
        if left < 0 {
            return -1;
        }
    }

    if left != 0 {
        if left == (1i32 << max_codeword_len) {
            core::ptr::write_bytes(decode_table, 0, table_num_entries);
            return 0;
        }
        return -1;
    }

    *offsets.add(1) = 0;
    for len in 1..max_codeword_len {
        let value = (*offsets.add(len as usize) as u32)
            + (*len_counts.add(len as usize) as u32);
        *offsets.add((len + 1) as usize) = value as u16;
    }

    for sym in 0..num_syms {
        let l = *lens.add(sym as usize);
        if l != 0 {
            let p = offsets.add(l as usize);
            *sorted_syms.add(*p as usize) = sym as u16;
            *p = (*p).wrapping_add(1);
        }
    }

    sym_idx = 0;
    decode_table_pos = 0;
    codeword_len = 1;
    stores_per_loop = 1u32 << (table_bits - codeword_len);
    while stores_per_loop != 0 {
        let end_sym_idx = sym_idx + *len_counts.add(codeword_len as usize) as u32;
        while sym_idx < end_sym_idx {
            let entry = ((codeword_len << 11) as u16) | *sorted_syms.add(sym_idx as usize);
            let mut p = decode_table.add(decode_table_pos);
            let mut n = stores_per_loop;
            while n != 0 {
                *p = entry;
                p = p.add(1);
                n -= 1;
            }
            decode_table_pos += stores_per_loop as usize;
            sym_idx += 1;
        }
        codeword_len += 1;
        stores_per_loop >>= 1;
    }

    if decode_table_pos != table_num_entries {
        let mut j = decode_table_pos;
        while j != table_num_entries {
            *decode_table.add(j) = 0;
            j += 1;
        }

        let mut next_free_tree_slot = table_num_entries;
        let mut cur_codeword = (decode_table_pos as u32) << 1;
        while codeword_len <= max_codeword_len {
            let end_sym_idx = sym_idx + *len_counts.add(codeword_len as usize) as u32;
            while sym_idx < end_sym_idx {
                let sorted_sym = *sorted_syms.add(sym_idx as usize) as u32;
                let mut extra_bits = codeword_len - table_bits;
                let mut node_idx = (cur_codeword >> extra_bits) as usize;
                loop {
                    if *decode_table.add(node_idx) == 0 {
                        *decode_table.add(node_idx) = (next_free_tree_slot as u16) | 0xC000;
                        *decode_table.add(next_free_tree_slot) = 0;
                        next_free_tree_slot += 1;
                        *decode_table.add(next_free_tree_slot) = 0;
                        next_free_tree_slot += 1;
                    }
                    node_idx = (*decode_table.add(node_idx) as usize & 0x3FFF);
                    extra_bits -= 1;
                    node_idx += ((cur_codeword >> extra_bits) & 1) as usize;
                    if extra_bits == 0 { break; }
                }
                *decode_table.add(node_idx) = sorted_sym as u16;
                sym_idx += 1;
                cur_codeword += 1;
            }
            codeword_len += 1;
            cur_codeword <<= 1;
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
