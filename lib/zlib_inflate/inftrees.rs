/* inftrees.c -- generate Huffman trees for efficient decoding
 * Copyright (C) 1995-2005 Mark Adler
 * For conditions of distribution and use, see copyright notice in zlib.h
 *
 * Rust translation of the frozen source at
 * 8e8505218ff400546323d71109207c781769e54e.
 */

use crate::bindings::{code, codetype, CODES, ENOUGH, LENS, MAXD};
use core::ffi::{c_int, c_uint};
use core::ptr;

const MAXBITS: usize = 15;

static LBASE: [u16; 31] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258, 0, 0,
];
static LEXT: [u16; 31] = [
    16, 16, 16, 16, 16, 16, 16, 16, 17, 17, 17, 17, 18, 18, 18, 18, 19, 19, 19, 19, 20, 20, 20, 20,
    21, 21, 21, 21, 16, 201, 196,
];
static DBASE: [u16; 32] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577, 0, 0,
];
static DEXT: [u16; 32] = [
    16, 16, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 24, 24, 25, 25, 26, 26,
    27, 27, 28, 28, 29, 29, 64, 64,
];

/// Build canonical Huffman decoding tables, returning 0, -1 for invalid
/// lengths, or 1 when the literal/length table exceeds the reserved space.
///
/// # Safety
/// The caller must provide the same storage and length preconditions as the
/// native zlib routine: every length is at most MAXBITS, `lens` contains
/// `codes` entries, `work` has sufficient writable entries, and `table` and
/// `bits` point to valid writable objects and sufficient decoding-table space.
#[no_mangle]
pub(crate) unsafe extern "C" fn zlib_inflate_table(
    kind: codetype,
    lens: *mut u16,
    codes: c_uint,
    table: *mut *mut code,
    bits: *mut c_uint,
    work: *mut u16,
) -> c_int {
    unsafe {
        let mut count = [0u16; MAXBITS + 1];
        let mut offs = [0u16; MAXBITS + 1];

        // Accumulate lengths; as in C, the caller guarantees 0..=MAXBITS.
        let mut sym: c_uint = 0;
        while sym < codes {
            let length = *lens.add(sym as usize) as usize;
            count[length] = count[length].wrapping_add(1);
            sym = sym.wrapping_add(1);
        }

        let mut root = *bits;
        let mut max = MAXBITS as c_uint;
        while max >= 1 && count[max as usize] == 0 {
            max = max.wrapping_sub(1);
        }
        if root > max {
            root = max;
        }
        if max == 0 {
            // With no symbols, make a two-entry table that forces a decoder
            // error, and leave reporting that error to the decoder.
            let entry = code {
                op: 64,
                bits: 1,
                val: 0,
            };
            ptr::write(*table, ptr::read(&entry));
            *table = (*table).add(1);
            ptr::write(*table, ptr::read(&entry));
            *table = (*table).add(1);
            *bits = 1;
            return 0;
        }
        let mut min: c_uint = 1;
        while min < MAXBITS as c_uint && count[min as usize] == 0 {
            min = min.wrapping_add(1);
        }
        if root < min {
            root = min;
        }

        // Reject oversubscribed and incomplete sets of lengths.
        let mut left: c_int = 1;
        let mut len: c_uint = 1;
        while len <= MAXBITS as c_uint {
            left = left.wrapping_shl(1);
            left = left.wrapping_sub(count[len as usize] as c_int);
            if left < 0 {
                return -1;
            }
            len = len.wrapping_add(1);
        }
        if left > 0 && (kind == CODES || max != 1) {
            return -1;
        }

        // Stable sort symbols by length using cumulative offsets.
        offs[1] = 0;
        len = 1;
        while len < MAXBITS as c_uint {
            offs[len.wrapping_add(1) as usize] =
                offs[len as usize].wrapping_add(count[len as usize]);
            len = len.wrapping_add(1);
        }
        sym = 0;
        while sym < codes {
            let length = *lens.add(sym as usize) as usize;
            if length != 0 {
                *work.add(offs[length] as usize) = sym as u16;
                offs[length] = offs[length].wrapping_add(1);
            }
            sym = sym.wrapping_add(1);
        }

        // C biases LBASE and LEXT pointers backwards by 257. Carry that bias
        // as an index instead, so no out-of-allocation pointer is formed.
        let (base, extra, base_symbol, end): (*const u16, *const u16, c_uint, c_int) =
            if kind == CODES {
                (work, work, 0, 19)
            } else if kind == LENS {
                (LBASE.as_ptr(), LEXT.as_ptr(), 257, 256)
            } else {
                (DBASE.as_ptr(), DEXT.as_ptr(), 0, -1)
            };

        let mut huff: c_uint = 0;
        sym = 0;
        len = min;
        let mut next = *table;
        let mut curr = root;
        let mut drop: c_uint = 0;
        let mut low = c_uint::MAX;
        let mut used = (1 as c_uint).wrapping_shl(root);
        let mask = used.wrapping_sub(1);
        if kind == LENS && used >= (ENOUGH as c_uint).wrapping_sub(MAXD as c_uint) {
            return 1;
        }

        loop {
            let symbol = *work.add(sym as usize);
            let (op, val) = if (symbol as c_int) < end {
                (0, symbol)
            } else if (symbol as c_int) > end {
                let index = (symbol as c_uint).wrapping_sub(base_symbol) as usize;
                (*extra.add(index) as u8, *base.add(index))
            } else {
                (32 + 64, 0)
            };
            let entry = code {
                op,
                bits: len.wrapping_sub(drop) as u8,
                val,
            };

            // Replicate the entry over all matching indices in this table.
            let mut incr = (1 as c_uint).wrapping_shl(len.wrapping_sub(drop));
            let mut fill = (1 as c_uint).wrapping_shl(curr);
            min = fill;
            loop {
                fill = fill.wrapping_sub(incr);
                ptr::write(
                    next.add((huff.wrapping_shr(drop)).wrapping_add(fill) as usize),
                    ptr::read(&entry),
                );
                if fill == 0 {
                    break;
                }
            }

            // Increment the backwards, len-bit Huffman code.
            incr = (1 as c_uint).wrapping_shl(len.wrapping_sub(1));
            while huff & incr != 0 {
                incr >>= 1;
            }
            if incr != 0 {
                huff &= incr.wrapping_sub(1);
                huff = huff.wrapping_add(incr);
            } else {
                huff = 0;
            }

            sym = sym.wrapping_add(1);
            count[len as usize] = count[len as usize].wrapping_sub(1);
            if count[len as usize] == 0 {
                if len == max {
                    break;
                }
                len = *lens.add(*work.add(sym as usize) as usize) as c_uint;
            }

            if len > root && (huff & mask) != low {
                if drop == 0 {
                    drop = root;
                }
                next = next.add(min as usize);
                curr = len.wrapping_sub(drop);
                left = (1 as c_int).wrapping_shl(curr);
                while curr.wrapping_add(drop) < max {
                    left = left.wrapping_sub(count[curr.wrapping_add(drop) as usize] as c_int);
                    if left <= 0 {
                        break;
                    }
                    curr = curr.wrapping_add(1);
                    left = left.wrapping_shl(1);
                }

                used = used.wrapping_add((1 as c_uint).wrapping_shl(curr));
                if kind == LENS && used >= (ENOUGH as c_uint).wrapping_sub(MAXD as c_uint) {
                    return 1;
                }

                low = huff & mask;
                ptr::write(
                    (*table).add(low as usize),
                    code {
                        op: curr as u8,
                        bits: root as u8,
                        val: next.offset_from(*table) as u16,
                    },
                );
            }
        }

        // Fill every unused code in an incomplete table with an invalid-code
        // marker, returning to the root when the current subtable is full.
        let mut entry = code {
            op: 64,
            bits: len.wrapping_sub(drop) as u8,
            val: 0,
        };
        while huff != 0 {
            if drop != 0 && (huff & mask) != low {
                drop = 0;
                len = root;
                next = *table;
                entry.bits = len as u8;
            }
            ptr::write(
                next.add(huff.wrapping_shr(drop) as usize),
                ptr::read(&entry),
            );

            let mut incr = (1 as c_uint).wrapping_shl(len.wrapping_sub(1));
            while huff & incr != 0 {
                incr >>= 1;
            }
            if incr != 0 {
                huff &= incr.wrapping_sub(1);
                huff = huff.wrapping_add(incr);
            } else {
                huff = 0;
            }
        }

        *table = (*table).add(used as usize);
        *bits = root;
        0
    }
}
