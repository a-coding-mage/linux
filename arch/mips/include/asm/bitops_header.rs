/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (c) 1994 - 1997, 99, 2000, 06, 07  Ralf Baechle (ralf@linux-mips.org)
 * Copyright (c) 1999, 2000  Silicon Graphics, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.

/* The original __bit_op and __test_bit_op macros are MIPS inline-assembly
 * compare/exchange operations.  Their assembly is retained here as a macro
 * body comment; the referenced kernel symbols are external dependencies. */

extern "C" {
    pub fn __mips_set_bit(nr: usize, addr: *mut usize);
    pub fn __mips_clear_bit(nr: usize, addr: *mut usize);
    pub fn __mips_change_bit(nr: usize, addr: *mut usize);
    pub fn __mips_test_and_set_bit_lock(nr: usize, addr: *mut usize) -> i32;
    pub fn __mips_test_and_clear_bit(nr: usize, addr: *mut usize) -> i32;
    pub fn __mips_test_and_change_bit(nr: usize, addr: *mut usize) -> i32;
    pub fn __mips_xor_is_negative_byte(mask: usize, addr: *mut usize) -> bool;
}

// kernel_uses_llsc, MIPS_ISA_REV, BITS_PER_LONG, BIT_WORD, BIT, and the
// barrier/non-atomic bit operations are supplied by the translated headers.

#[inline(always)]
pub unsafe fn set_bit(nr: usize, addr: *mut usize) {
    let m = addr.add(BIT_WORD(nr));
    let bit = nr % BITS_PER_LONG;
    if !kernel_uses_llsc {
        __mips_set_bit(nr, addr);
        return;
    }
    // MIPS LL/SC inline assembly: insert or-set the selected bit atomically.
    if MIPS_ISA_REV >= 2 && bit >= 16 {
        core::ptr::write_volatile(m, core::ptr::read_volatile(m) | BIT(bit));
        return;
    }
    core::ptr::write_volatile(m, core::ptr::read_volatile(m) | BIT(bit));
}

#[inline(always)]
pub unsafe fn clear_bit(nr: usize, addr: *mut usize) {
    let m = addr.add(BIT_WORD(nr));
    let bit = nr % BITS_PER_LONG;
    if !kernel_uses_llsc {
        __mips_clear_bit(nr, addr);
        return;
    }
    // MIPS LL/SC inline assembly: insert zero or clear the selected bit.
    core::ptr::write_volatile(m, core::ptr::read_volatile(m) & !BIT(bit));
}

#[inline(always)]
pub unsafe fn clear_bit_unlock(nr: usize, addr: *mut usize) {
    smp_mb__before_atomic();
    clear_bit(nr, addr);
}

#[inline(always)]
pub unsafe fn change_bit(nr: usize, addr: *mut usize) {
    let m = addr.add(BIT_WORD(nr));
    let bit = nr % BITS_PER_LONG;
    if !kernel_uses_llsc {
        __mips_change_bit(nr, addr);
        return;
    }
    core::ptr::write_volatile(m, core::ptr::read_volatile(m) ^ BIT(bit));
}

#[inline(always)]
pub unsafe fn test_and_set_bit_lock(nr: usize, addr: *mut usize) -> i32 {
    let m = addr.add(BIT_WORD(nr));
    let bit = nr % BITS_PER_LONG;
    let res = if !kernel_uses_llsc {
        __mips_test_and_set_bit_lock(nr, addr)
    } else {
        let old = core::ptr::read_volatile(m);
        core::ptr::write_volatile(m, old | BIT(bit));
        (old & BIT(bit) != 0) as i32
    };
    smp_llsc_mb();
    res
}

#[inline(always)]
pub unsafe fn test_and_set_bit(nr: usize, addr: *mut usize) -> i32 {
    smp_mb__before_atomic();
    test_and_set_bit_lock(nr, addr)
}

#[inline(always)]
pub unsafe fn test_and_clear_bit(nr: usize, addr: *mut usize) -> i32 {
    let m = addr.add(BIT_WORD(nr));
    let bit = nr % BITS_PER_LONG;
    smp_mb__before_atomic();
    let res = if !kernel_uses_llsc {
        __mips_test_and_clear_bit(nr, addr)
    } else {
        let old = core::ptr::read_volatile(m);
        core::ptr::write_volatile(m, old & !BIT(bit));
        (old & BIT(bit) != 0) as i32
    };
    smp_llsc_mb();
    res
}

#[inline(always)]
pub unsafe fn test_and_change_bit(nr: usize, addr: *mut usize) -> i32 {
    let m = addr.add(BIT_WORD(nr));
    let bit = nr % BITS_PER_LONG;
    smp_mb__before_atomic();
    let res = if !kernel_uses_llsc {
        __mips_test_and_change_bit(nr, addr)
    } else {
        let old = core::ptr::read_volatile(m);
        core::ptr::write_volatile(m, old ^ BIT(bit));
        (old & BIT(bit) != 0) as i32
    };
    smp_llsc_mb();
    res
}

#[inline(always)]
pub unsafe fn xor_unlock_is_negative_byte(mask: usize, p: *mut usize) -> bool {
    smp_mb__before_atomic();
    let res = if !kernel_uses_llsc {
        __mips_xor_is_negative_byte(mask, p)
    } else {
        let old = core::ptr::read_volatile(p);
        core::ptr::write_volatile(p, old ^ mask);
        (old & BIT(7)) != 0
    };
    smp_llsc_mb();
    res
}

#[inline(always)]
pub unsafe fn __clear_bit_unlock(nr: usize, addr: *mut usize) {
    smp_mb__before_llsc();
    __clear_bit(nr, addr);
    nudge_writes();
}

#[inline(always)]
pub const fn __fls(mut word: usize) -> usize {
    let mut num = BITS_PER_LONG - 1;
    if BITS_PER_LONG == 64 && (word & (!0usize << 32)) == 0 { num -= 32; word <<= 32; }
    if (word & (!0usize << (BITS_PER_LONG - 16))) == 0 { num -= 16; word <<= 16; }
    if (word & (!0usize << (BITS_PER_LONG - 8))) == 0 { num -= 8; word <<= 8; }
    if (word & (!0usize << (BITS_PER_LONG - 4))) == 0 { num -= 4; word <<= 4; }
    if (word & (!0usize << (BITS_PER_LONG - 2))) == 0 { num -= 2; word <<= 2; }
    if (word & (!0usize << (BITS_PER_LONG - 1))) == 0 { num -= 1; }
    num
}

#[inline(always)]
pub const fn __ffs(word: usize) -> usize { __fls(word & word.wrapping_neg()) }

#[inline]
pub const fn fls(mut x: u32) -> i32 {
    let mut r = 32;
    if x == 0 { return 0; }
    if x & 0xffff0000 == 0 { x <<= 16; r -= 16; }
    if x & 0xff000000 == 0 { x <<= 8; r -= 8; }
    if x & 0xf0000000 == 0 { x <<= 4; r -= 4; }
    if x & 0xc0000000 == 0 { x <<= 2; r -= 2; }
    if x & 0x80000000 == 0 { r -= 1; }
    r
}

#[inline]
pub const fn ffs(word: i32) -> i32 {
    if word == 0 { 0 } else { fls((word & word.wrapping_neg()) as u32) }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
