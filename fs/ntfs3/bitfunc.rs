// SPDX-License-Identifier: GPL-2.0
/*
 *
 * Copyright (C) 2019-2021 Paragon Software GmbH, All rights reserved.
 *
 */

const BITS_IN_SIZE_T: usize = core::mem::size_of::<usize>() * 8;

/*
 * fill_mask[i] - first i bits are '1' , i = 0,1,2,3,4,5,6,7,8
 * fill_mask[i] = 0xFF >> (8-i)
 */
static FILL_MASK: [u8; 9] = [
    0x00, 0x01, 0x03, 0x07, 0x0F, 0x1F, 0x3F, 0x7F, 0xFF,
];

/*
 * zero_mask[i] - first i bits are '0' , i = 0,1,2,3,4,5,6,7,8
 * zero_mask[i] = 0xFF << i
 */
static ZERO_MASK: [u8; 9] = [
    0xFF, 0xFE, 0xFC, 0xF8, 0xF0, 0xE0, 0xC0, 0x80, 0x00,
];

/*
 * are_bits_clear
 *
 * Return: True if all bits [bit, bit+nbits) are zeros "0".
 */
pub unsafe fn are_bits_clear(lmap: *const core::ffi::c_void, mut bit: usize, mut nbits: usize) -> bool {
    let mut pos = bit & 7;
    let mut map = (lmap as *const u8).add(bit >> 3);

    if pos != 0 {
        if 8 - pos >= nbits {
            return nbits == 0 || (*map & FILL_MASK[pos + nbits] & ZERO_MASK[pos]) == 0;
        }

        if *map & ZERO_MASK[pos] != 0 {
            return false;
        }
        map = map.add(1);
        nbits -= 8 - pos;
    }

    pos = (map as usize) & (core::mem::size_of::<usize>() - 1);
    if pos != 0 {
        pos = core::mem::size_of::<usize>() - pos;
        if nbits >= pos * 8 {
            nbits -= pos * 8;
            while pos != 0 {
                if *map != 0 {
                    return false;
                }
                pos -= 1;
                map = map.add(1);
            }
        }
    }

    pos = nbits / BITS_IN_SIZE_T;
    while pos != 0 {
        if *(map as *const usize) != 0 {
            return false;
        }
        pos -= 1;
        map = map.add(core::mem::size_of::<usize>());
    }

    pos = (nbits % BITS_IN_SIZE_T) >> 3;
    while pos != 0 {
        if *map != 0 {
            return false;
        }
        pos -= 1;
        map = map.add(1);
    }

    pos = nbits & 7;
    if pos != 0 && (*map & FILL_MASK[pos]) != 0 {
        return false;
    }

    true
}

/*
 * are_bits_set
 *
 * Return: True if all bits [bit, bit+nbits) are ones "1".
 */
pub unsafe fn are_bits_set(lmap: *const core::ffi::c_void, bit: usize, mut nbits: usize) -> bool {
    let mut mask: u8;
    let mut pos = bit & 7;
    let mut map = (lmap as *const u8).add(bit >> 3);

    if pos != 0 {
        if 8 - pos >= nbits {
            mask = FILL_MASK[pos + nbits] & ZERO_MASK[pos];
            return nbits == 0 || (*map & mask) == mask;
        }

        mask = ZERO_MASK[pos];
        if (*map & mask) != mask {
            return false;
        }
        map = map.add(1);
        nbits -= 8 - pos;
    }

    pos = (map as usize) & (core::mem::size_of::<usize>() - 1);
    if pos != 0 {
        pos = core::mem::size_of::<usize>() - pos;
        if nbits >= pos * 8 {
            nbits -= pos * 8;
            while pos != 0 {
                if *map != 0xFF {
                    return false;
                }
                pos -= 1;
                map = map.add(1);
            }
        }
    }

    pos = nbits / BITS_IN_SIZE_T;
    while pos != 0 {
        if *(map as *const usize) != usize::MAX {
            return false;
        }
        pos -= 1;
        map = map.add(core::mem::size_of::<usize>());
    }

    pos = (nbits % BITS_IN_SIZE_T) >> 3;
    while pos != 0 {
        if *map != 0xFF {
            return false;
        }
        pos -= 1;
        map = map.add(1);
    }

    pos = nbits & 7;
    if pos != 0 {
        mask = FILL_MASK[pos];
        if (*map & mask) != mask {
            return false;
        }
    }

    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
