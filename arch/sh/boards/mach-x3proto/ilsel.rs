// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/mach-x3proto/ilsel.c
 *
 * Helper routines for SH-X3 proto board ILSEL.
 *
 * Copyright (C) 2007 - 2010  Paul Mundt
 */

// Kernel headers and pr_fmt() provide the external types, macros, and I/O
// declarations used below.

const ILSEL_BASE: usize = 0xb8100004;
const ILSEL_LEVELS: usize = 15;

static mut ilsel_level_map: usize = 0;

extern "C" {
    fn pr_notice(fmt: *const i8, ...);
    fn pr_debug(fmt: *const i8, ...);
    fn pr_err(fmt: *const i8, ...);
    fn __raw_readw(addr: usize) -> u16;
    fn __raw_writew(value: u16, addr: usize);
    fn find_first_zero_bit(addr: *const usize, size: usize) -> usize;
    fn test_and_set_bit(bit: usize, addr: *mut usize) -> usize;
    fn clear_bit(bit: usize, addr: *mut usize);
}

// ilsel_source_t is supplied by <mach/ilsel.h>.

#[inline]
unsafe fn ilsel_offset(bit: usize) -> usize {
    ILSEL_LEVELS - bit - 1
}

#[inline]
unsafe fn mk_ilsel_addr(bit: usize) -> usize {
    ILSEL_BASE + ((ilsel_offset(bit) >> 1) & !0x1)
}

#[inline]
unsafe fn mk_ilsel_shift(bit: usize) -> usize {
    (ilsel_offset(bit) & 0x3) << 2
}

unsafe fn __ilsel_enable(set: ilsel_source_t, bit: usize) {
    pr_notice(b"enabling ILSEL set %d\n\0".as_ptr() as *const i8, set);

    let addr = mk_ilsel_addr(bit);
    let shift = mk_ilsel_shift(bit);

    pr_debug(
        b"%s: bit#%d: addr - 0x%08lx (shift %d, set %d)\n\0".as_ptr() as *const i8,
        b"__ilsel_enable\0".as_ptr() as *const i8,
        bit,
        addr,
        shift,
        set,
    );

    let mut tmp = __raw_readw(addr) as u32;
    tmp &= !(0xf_u32 << shift);
    tmp |= (set as u32) << shift;
    __raw_writew(tmp as u16, addr);
}

/// Enable an ILSEL set.
///
/// Enables a given non-aliased ILSEL source at the highest available
/// interrupt level. The return value is an IRQ number for ilsel_disable().
pub unsafe fn ilsel_enable(set: ilsel_source_t) -> i32 {
    let bit: usize;

    if set > ILSEL_KEY {
        pr_err(b"Aliased sources must use ilsel_enable_fixed()\n\0".as_ptr() as *const i8);
        return -22; // -EINVAL
    }

    loop {
        bit = find_first_zero_bit(&ilsel_level_map, ILSEL_LEVELS);
        if test_and_set_bit(bit, &mut ilsel_level_map) == 0 {
            break;
        }
    }

    __ilsel_enable(set, bit);
    bit as i32
}

/// Enable an ILSEL set at a fixed interrupt level.
pub unsafe fn ilsel_enable_fixed(set: ilsel_source_t, level: usize) -> i32 {
    let bit = ilsel_offset(level - 1);

    if test_and_set_bit(bit, &mut ilsel_level_map) != 0 {
        return -16; // -EBUSY
    }

    __ilsel_enable(set, bit);
    bit as i32
}

/// Disable an ILSEL set.
pub unsafe fn ilsel_disable(irq: usize) {
    pr_notice(b"disabling ILSEL set %d\n\0".as_ptr() as *const i8, irq);

    let addr = mk_ilsel_addr(irq);
    let mut tmp = __raw_readw(addr) as u32;
    tmp &= !(0xf_u32 << mk_ilsel_shift(irq));
    __raw_writew(tmp as u16, addr);

    clear_bit(irq, &mut ilsel_level_map);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
