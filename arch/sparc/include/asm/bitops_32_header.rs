/* SPDX-License-Identifier: GPL-2.0 */
/*
 * bitops.h: Bit string operations on the Sparc.
 *
 * Copyright 1995 David S. Miller (davem@caip.rutgers.org)
 * Copyright 1996 Eddie C. Dost   (ecd@skynet.be)
 * Copyright 2001 Anton Blanchard (anton@samba.org)
 */

// Original dependencies: <linux/compiler.h>, <asm/byteorder.h>.
// The following declarations are kernel-only in the original header.

/// Set a bit using the architecture-specific implementation.
unsafe extern "C" {
    pub fn sp32___set_bit(addr: *mut u32, mask: u32) -> u32;
    pub fn sp32___clear_bit(addr: *mut u32, mask: u32) -> u32;
    pub fn sp32___change_bit(addr: *mut u32, mask: u32) -> u32;
}

/*
 * Set bit 'nr' in 32-bit quantity at address 'addr' where bit '0'
 * is in the highest of the four bytes and bit '31' is the high bit
 * within the first byte. Sparc is BIG-Endian. Unless noted otherwise
 * all bit-ops return 0 if bit was previously clear and != 0 otherwise.
 */
pub unsafe fn test_and_set_bit(nr: u32, addr: *mut u32) -> i32 {
    let address = addr.add((nr >> 5) as usize);
    let mask = 1u32 << (nr & 31);
    (sp32___set_bit(address, mask) != 0) as i32
}

pub unsafe fn set_bit(nr: u32, addr: *mut u32) {
    let address = addr.add((nr >> 5) as usize);
    let mask = 1u32 << (nr & 31);
    let _ = sp32___set_bit(address, mask);
}

pub unsafe fn test_and_clear_bit(nr: u32, addr: *mut u32) -> i32 {
    let address = addr.add((nr >> 5) as usize);
    let mask = 1u32 << (nr & 31);
    (sp32___clear_bit(address, mask) != 0) as i32
}

pub unsafe fn clear_bit(nr: u32, addr: *mut u32) {
    let address = addr.add((nr >> 5) as usize);
    let mask = 1u32 << (nr & 31);
    let _ = sp32___clear_bit(address, mask);
}

pub unsafe fn test_and_change_bit(nr: u32, addr: *mut u32) -> i32 {
    let address = addr.add((nr >> 5) as usize);
    let mask = 1u32 << (nr & 31);
    (sp32___change_bit(address, mask) != 0) as i32
}

pub unsafe fn change_bit(nr: u32, addr: *mut u32) {
    let address = addr.add((nr >> 5) as usize);
    let mask = 1u32 << (nr & 31);
    let _ = sp32___change_bit(address, mask);
}

// Original generic dependencies:
// <asm-generic/bitops/non-atomic.h>, ffz.h, __ffs.h, sched.h, ffs.h,
// fls.h, __fls.h, fls64.h, hweight.h, lock.h, le.h, ext2-atomic.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
