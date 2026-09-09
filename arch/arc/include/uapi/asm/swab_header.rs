/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012 Synopsys, Inc. (www.synopsys.com)
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License version 2 as
 * published by the Free Software Foundation.
 *
 * vineetg: May 2011
 *  -Support single cycle endian-swap insn in ARC700 4.10
 *
 * vineetg: June 2009
 *  -Better htonl implementation (5 instead of 9 ALU instructions)
 *  -Hardware assisted single cycle bswap (Use Case of ARC custom instrn)
 */

// Dependency equivalent of <linux/types.h> is supplied externally.

/// Architecture-provided 32-bit endian swap.
#[inline]
pub unsafe fn __arch_swab32(x: u32) -> u32 {
    let mut tmp: u32 = x;
    core::arch::asm!(
        "swape {tmp}, {tmp}",
        tmp = inout(reg) tmp,
    );
    tmp
}

// C condition: !defined(__STRICT_ANSI__) || defined(__KERNEL__)
// When the corresponding build configuration is active, __SWAB_64_THRU_32__
// is defined.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
