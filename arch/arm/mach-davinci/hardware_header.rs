/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Hardware definitions common to all DaVinci family processors
 *
 * Author: Kevin Hilman, Deep Root Systems, LLC
 *
 * 2007 (c) Deep Root Systems, LLC.
 */

/*
 * Before you add anything to this file:
 *
 * This header is for defines common to ALL DaVinci family chips.
 * Anything that is chip specific should go in <chipname>.h,
 * and the chip/board init code should then explicitly include
 * <chipname>.h
 */

/*
 * I/O mapping
 */
pub const IO_PHYS: u32 = 0x01c0_0000;
pub const IO_OFFSET: u32 = 0xfd00_0000; /* Virtual IO = 0xfec00000 */
pub const IO_SIZE: u32 = 0x0040_0000;
pub const IO_VIRT: u32 = IO_PHYS.wrapping_add(IO_OFFSET);

#[inline]
pub const fn io_v2p(va: u32) -> u32 {
    va.wrapping_sub(IO_OFFSET)
}

#[inline]
pub const fn __io_address(x: u32) -> u32 {
    x.wrapping_add(IO_OFFSET)
}

/* IOMEM is supplied by the platform's I/O definitions. */
#[macro_export]
macro_rules! IO_ADDRESS {
    ($pa:expr) => {
        IOMEM($crate::__io_address($pa))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
