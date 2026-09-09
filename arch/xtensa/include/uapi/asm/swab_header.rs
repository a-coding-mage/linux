/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * include/asm-xtensa/swab.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// The C header depends on <linux/types.h> and <linux/compiler.h>.
// __SWAB_64_THRU_32__ is a build-visible marker.
pub const __SWAB_64_THRU_32__: () = ();

#[inline]
pub unsafe fn __arch_swab32(x: __u32) -> __u32 {
    let mut res: __u32;
    /* instruction sequence from Xtensa ISA release 2/2000 */
    core::arch::asm!(
        "ssai     8           ",
        "srli     {res}, {x}, 16  ",
        "src      {res}, {res}, {x}  ",
        "src      {res}, {res}, {res}  ",
        "src      {res}, {x}, {res}  ",
        res = lateout(reg) res,
        x = in(reg) x,
    );
    res
}

// #define __arch_swab32 __arch_swab32

#[inline]
pub unsafe fn __arch_swab16(x: __u16) -> __u16 {
    /* Given that 'short' values are signed (i.e., can be negative),
     * we cannot assume that the upper 16-bits of the register are
     * zero.  We are careful to mask values after shifting.
     */

    /* There exists an anomaly between xt-gcc and xt-xcc.  xt-gcc
     * inserts an extui instruction after putting this function inline
     * to ensure that it uses only the least-significant 16 bits of the
     * result.  xt-xcc doesn't use an extui, but assumes the
     * __asm__ macro follows convention that the upper 16 bits of an
     * 'unsigned short' result are still zero.  This macro doesn't
     * follow convention; indeed, it leaves garbage in the upport 16
     * bits of the register.

     * Declaring the temporary variables 'res' and 'tmp' to be 32-bit
     * types while the return type of the function is a 16-bit type
     * forces both compilers to insert exactly one extui instruction
     * (or equivalent) to mask off the upper 16 bits. */

    let mut res: __u32;
    let mut tmp: __u32;

    core::arch::asm!(
        "extui    {tmp}, {x}, 8, 8",
        "slli     {res}, {x}, 8   ",
        "or       {res}, {res}, {tmp}  ",
        res = lateout(reg) res,
        tmp = lateout(reg) tmp,
        x = in(reg) x,
    );

    res as __u16
}

// #define __arch_swab16 __arch_swab16

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
