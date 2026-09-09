/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  Author: Nicolas Pitre
 *  Created: Jun 15, 2001
 *  Copyright: MontaVista Software Inc.
 */

/*
 * Workarounds for at least 2 errata so far require this.
 * The mapping is set in mach-pxa/generic.c.
 */
pub const UNCACHED_PHYS_0: u32 = 0xfe000000;
pub const UNCACHED_PHYS_0_SIZE: u32 = 0x00100000;

/*
 * Intel PXA2xx internal register mapping:
 *
 * 0x40000000 - 0x41ffffff <--> 0xf2000000 - 0xf3ffffff
 * 0x44000000 - 0x45ffffff <--> 0xf4000000 - 0xf5ffffff
 * 0x48000000 - 0x49ffffff <--> 0xf6000000 - 0xf7ffffff
 * 0x4c000000 - 0x4dffffff <--> 0xf8000000 - 0xf9ffffff
 * 0x50000000 - 0x51ffffff <--> 0xfa000000 - 0xfbffffff
 * 0x54000000 - 0x55ffffff <--> 0xfc000000 - 0xfdffffff
 * 0x58000000 - 0x59ffffff <--> 0xfe000000 - 0xffffffff
 *
 * Note that not all PXA2xx chips implement all those addresses, and the
 * kernel only maps the minimum needed range of this mapping.
 */
#[inline]
pub const fn io_v2p(x: u32) -> u32 {
    0x3c000000u32
        .wrapping_add(x & 0x01ffffff)
        .wrapping_add((x & 0x0e000000).wrapping_shl(1))
}

#[inline]
pub unsafe fn io_p2v(x: u32) -> u32 {
    // IOMEM is supplied by the platform's I/O mapping implementation.
    IOMEM(
        0xf2000000u32
            .wrapping_add(x & 0x01ffffff)
            .wrapping_add((x & 0x1c000000).wrapping_shr(1)),
    )
}

/* IOMEM is an external platform-provided mapping operation. */
unsafe extern "C" {
    fn IOMEM(x: u32) -> u32;
}

/* With indexed regs we don't want to feed the index through io_p2v()
   especially if it is a variable, otherwise horrible code will result. */
#[macro_export]
macro_rules! __REG {
    ($x:expr) => {
        *(($crate::io_p2v($x) as *mut u32))
    };
}

#[macro_export]
macro_rules! __REG2 {
    ($x:expr, $y:expr) => {
        *((($crate::io_p2v($x) as u32).wrapping_add($y as u32)) as *mut u32)
    };
}

#[macro_export]
macro_rules! __PREG {
    ($x:expr) => {
        $crate::io_v2p((&$x as *const _ as usize) as u32)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
