// SPDX-License-Identifier: GPL-2.0
/*
 * MSB0 numbered special bitops handling.
 *
 * The bits are numbered:
 *   |0..............63|64............127|128...........191|192...........255|
 *
 * The reason for this bit numbering is the fact that the hardware sets bits
 * in a bitmap starting at bit 0 (MSB) and we don't want to scan the bitmap
 * from the 'wrong end'.
 */

// BITS_PER_LONG is a build-time bitops constant (64 on s390).
const BITS_PER_LONG: usize = 64;

// Supplied by the surrounding bitops implementation.
extern "C" {
    fn __fls(word: usize) -> usize;
}

pub unsafe extern "C" fn find_first_bit_inv(addr: *const usize, size: usize) -> usize {
    let mut p = addr;
    let mut size = size;
    let mut result: usize = 0;
    let mut tmp: usize;

    while size & !(unsafe { BITS_PER_LONG } - 1) != 0 {
        tmp = *p;
        p = p.add(1);
        if tmp != 0 {
        return result + (unsafe { __fls(tmp) } ^ (BITS_PER_LONG - 1));
        }
        result += BITS_PER_LONG;
        size -= BITS_PER_LONG;
    }
    if size == 0 {
        return result;
    }
    tmp = *p & (!0usize << (BITS_PER_LONG - size));
    if tmp == 0 {
        return result + size;
    }
    result + (unsafe { __fls(tmp) } ^ (BITS_PER_LONG - 1))
}

pub unsafe extern "C" fn find_next_bit_inv(
    addr: *const usize,
    size: usize,
    offset: usize,
) -> usize {
    let mut p = addr.add(offset / BITS_PER_LONG);
    let mut result = offset & !(BITS_PER_LONG - 1);
    let mut size = size;
    let mut offset = offset;
    let mut tmp: usize;

    if offset >= size {
        return size;
    }
    size -= result;
    offset %= BITS_PER_LONG;
    if offset != 0 {
        tmp = *p;
        p = p.add(1);
        tmp &= !0usize >> offset;
        if size < BITS_PER_LONG {
            tmp &= !0usize << (BITS_PER_LONG - size);
            if tmp == 0 {
                return result + size;
            }
            return result + (unsafe { __fls(tmp) } ^ (BITS_PER_LONG - 1));
        }
        if tmp != 0 {
            return result + (unsafe { __fls(tmp) } ^ (BITS_PER_LONG - 1));
        }
        size -= BITS_PER_LONG;
        result += BITS_PER_LONG;
    }
    while size & !(BITS_PER_LONG - 1) != 0 {
        tmp = *p;
        p = p.add(1);
        if tmp != 0 {
            return result + (unsafe { __fls(tmp) } ^ (BITS_PER_LONG - 1));
        }
        result += BITS_PER_LONG;
        size -= BITS_PER_LONG;
    }
    if size == 0 {
        return result;
    }
    tmp = *p;
    tmp &= !0usize << (BITS_PER_LONG - size);
    if tmp == 0 {
        return result + size;
    }
    result + (unsafe { __fls(tmp) } ^ (BITS_PER_LONG - 1))
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
