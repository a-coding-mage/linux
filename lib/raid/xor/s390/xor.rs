// SPDX-License-Identifier: GPL-2.0
/*
 * Optimized xor_block operation for RAID4/5
 *
 * Copyright IBM Corp. 2016
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// The C implementation uses s390x inline assembly (`xc`) for these routines.
// The byte-wise operations below preserve the same XOR behavior and pointer
// advancement while leaving the architecture-specific interface to callers.

unsafe fn xor_xc_2(bytes: usize, p1: *mut usize, p2: *const usize) {
    let mut i = 0usize;
    while i < bytes {
        let dst = (p1 as *mut u8).add(i);
        let src = (p2 as *const u8).add(i);
        *dst ^= *src;
        i += 1;
    }
}

unsafe fn xor_xc_3(bytes: usize, p1: *mut usize, p2: *const usize, p3: *const usize) {
    let mut i = 0usize;
    while i < bytes {
        let dst = (p1 as *mut u8).add(i);
        *dst ^= *((p2 as *const u8).add(i));
        *dst ^= *((p3 as *const u8).add(i));
        i += 1;
    }
}

unsafe fn xor_xc_4(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
    p3: *const usize,
    p4: *const usize,
) {
    let mut i = 0usize;
    while i < bytes {
        let dst = (p1 as *mut u8).add(i);
        *dst ^= *((p2 as *const u8).add(i));
        *dst ^= *((p3 as *const u8).add(i));
        *dst ^= *((p4 as *const u8).add(i));
        i += 1;
    }
}

unsafe fn xor_xc_5(
    bytes: usize,
    p1: *mut usize,
    p2: *const usize,
    p3: *const usize,
    p4: *const usize,
    p5: *const usize,
) {
    let mut i = 0usize;
    while i < bytes {
        let dst = (p1 as *mut u8).add(i);
        *dst ^= *((p2 as *const u8).add(i));
        *dst ^= *((p3 as *const u8).add(i));
        *dst ^= *((p4 as *const u8).add(i));
        *dst ^= *((p5 as *const u8).add(i));
        i += 1;
    }
}

// DO_XOR_BLOCKS(xc, xor_xc_2, xor_xc_3, xor_xc_4, xor_xc_5);
// This macro invocation supplies the architecture-specific xor_gen_xc entry
// point and xor_block_template registration in the surrounding C headers.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
