/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Some ColdFire platforms do in fact have a PCI bus. So for those we want
 * to use the real IO access functions, don't fake them out or redirect them
 * for that case.
 */

/*
 * The following declarations apply when CONFIG_PCI is not enabled.  The
 * original header includes asm/io.h and asm/kmap.h here; those dependencies
 * are supplied by the surrounding translation unit.
 */

/*
 * FIXME
 * Ugh, we don't have PCI space, so map readb() and friends to use raw I/O
 * accessors, which are identical to the z_*() Zorro bus accessors.
 * This should make cirrusfb work again on Amiga
 */

/// Equivalent to the C `inb_p(port)` macro when PCI is unavailable.
#[inline(always)]
pub const fn inb_p<T>(_port: T) -> u8 {
    0
}

/// Equivalent to the C `inw_p(port)` macro when PCI is unavailable.
#[inline(always)]
pub const fn inw_p<T>(_port: T) -> u16 {
    0
}

/// Equivalent to the C no-op `outb_p(port, val)` macro.
#[inline(always)]
pub fn outb_p<T, U>(_port: T, _val: U) {}

/// Equivalent to the C no-op `outw(port, val)` macro.
#[inline(always)]
pub fn outw<T, U>(_port: T, _val: U) {}

/*
 * In the C header these are preprocessor aliases:
 *
 *     readb  -> __raw_readb
 *     writeb -> __raw_writeb
 *     writew -> __raw_writew
 *
 * The raw accessors are external dependencies supplied by asm/io.h.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
