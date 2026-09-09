/* SPDX-License-Identifier: GPL-2.0 */
/*
 * KCSAN watchpoint encoding.
 *
 * Copyright (C) 2019, Google LLC.
 */

// Dependencies supplied by the surrounding kernel translation:
// PAGE_SIZE, KCSAN_CHECK_ADJACENT, BITS_PER_LONG, and
// CONFIG_KCSAN_NUM_WATCHPOINTS.

pub const SLOT_RANGE: usize = PAGE_SIZE;

pub const INVALID_WATCHPOINT: i32 = 0;
pub const CONSUMED_WATCHPOINT: i32 = 1;

/*
 * The maximum useful size of accesses for which we set up watchpoints is the
 * max range of slots we check on an access.
 */
pub const MAX_ENCODABLE_SIZE: usize = SLOT_RANGE * (1 + KCSAN_CHECK_ADJACENT);

/* Number of bits we use to store size info. */
pub const WATCHPOINT_SIZE_BITS: usize =
    (usize::BITS - (MAX_ENCODABLE_SIZE as usize).leading_zeros()) as usize;

/*
 * This encoding for addresses discards the upper (1 for is-write + SIZE_BITS);
 * however, most 64-bit architectures do not use the full 64-bit address space.
 * Also, in order for a false positive to be observable 2 things need to happen:
 *
 *     1. different addresses but with the same encoded address race;
 *     2. and both map onto the same watchpoint slots;
 *
 * Both these are assumed to be very unlikely. However, in case it still
 * happens, the report logic will filter out the false positive (see report.c).
 */
pub const WATCHPOINT_ADDR_BITS: usize = BITS_PER_LONG - 1 - WATCHPOINT_SIZE_BITS;

/* Bitmasks for the encoded watchpoint access information. */
pub const WATCHPOINT_WRITE_MASK: usize = 1usize << (BITS_PER_LONG - 1);
pub const WATCHPOINT_SIZE_MASK: usize =
    (((1usize << (BITS_PER_LONG - 1)) - 1) & !((1usize << WATCHPOINT_ADDR_BITS) - 1));
pub const WATCHPOINT_ADDR_MASK: usize = (1usize << WATCHPOINT_ADDR_BITS) - 1;

pub fn check_encodable(addr: usize, size: usize) -> bool {
    /*
     * While we can encode addrs<PAGE_SIZE, avoid crashing with a NULL
     * pointer deref inside KCSAN.
     */
    addr >= PAGE_SIZE && size <= MAX_ENCODABLE_SIZE
}

pub fn encode_watchpoint(addr: usize, size: usize, is_write: bool) -> isize {
    (if is_write { WATCHPOINT_WRITE_MASK } else { 0 }
        | (size << WATCHPOINT_ADDR_BITS)
        | (addr & WATCHPOINT_ADDR_MASK)) as isize
}

pub unsafe fn decode_watchpoint(
    watchpoint: isize,
    addr_masked: *mut usize,
    size: *mut usize,
    is_write: *mut bool,
) -> bool {
    if watchpoint == INVALID_WATCHPOINT as isize || watchpoint == CONSUMED_WATCHPOINT as isize {
        return false;
    }

    *addr_masked = watchpoint as usize & WATCHPOINT_ADDR_MASK;
    *size = ((watchpoint as usize & WATCHPOINT_SIZE_MASK) >> WATCHPOINT_ADDR_BITS) as usize;
    *is_write = (watchpoint as usize & WATCHPOINT_WRITE_MASK) != 0;

    true
}

/* Return watchpoint slot for an address. */
pub fn watchpoint_slot(addr: usize) -> i32 {
    ((addr / PAGE_SIZE) % CONFIG_KCSAN_NUM_WATCHPOINTS) as i32
}

pub fn matching_access(addr1: usize, size1: usize, addr2: usize, size2: usize) -> bool {
    let end_range1 = addr1.wrapping_add(size1).wrapping_sub(1);
    let end_range2 = addr2.wrapping_add(size2).wrapping_sub(1);

    addr1 <= end_range2 && addr2 <= end_range1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
