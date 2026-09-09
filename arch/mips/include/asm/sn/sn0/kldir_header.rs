/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Derived from IRIX <sys/SN/kldir.h>, revision 1.21.
 *
 * Copyright (C) 1992 - 1997, 1999, 2000 Silicon Graphics, Inc.
 * Copyright (C) 1999, 2000 by Ralf Baechle
 */

/*
 * The kldir memory area resides at a fixed place in each node's memory and
 * provides pointers to most other IP27 memory areas.  This allows us to
 * resize and/or relocate memory areas at a later time without breaking all
 * firmware and kernels that use them. Indices in the array are permanently
 * dedicated to areas listed below. Some memory areas reside at a permanently
 * fixed location, but are included in the directory for completeness.
 */

/* The upper portion of the memory map applies during boot only and is
 * overwritten by IRIX/SYMMON. The lower portion contains permanent data used
 * by the IP27PROM, IO6PROM and IRIX. */

/*
 * This is defined here because IP27_SYMMON_STK_SIZE must be at least what
 * we define here. Since it's set up in the prom, we can't redefine it later
 * and expect more space to be allocated. The true size of the symmon stacks
 * is found by dividing SYMMON_STK_SIZE by SYMMON_STK_STRIDE for a node.
 */
pub const SYMMON_STACK_SIZE: u32 = 0x8000;

/* PROM-only definitions; retain the source build-time condition. */
#[cfg(feature = "PROM")]
pub const IP27_LAUNCH_OFFSET: u32 = 0x2400;
#[cfg(feature = "PROM")]
pub const IP27_LAUNCH_SIZE: u32 = 0x400;
#[cfg(feature = "PROM")]
pub const IP27_LAUNCH_COUNT: u32 = 2;
#[cfg(feature = "PROM")]
pub const IP27_LAUNCH_STRIDE: u32 = 0x200;

#[cfg(feature = "PROM")]
pub const IP27_KLCONFIG_OFFSET: u32 = 0x4000;
#[cfg(feature = "PROM")]
pub const IP27_KLCONFIG_SIZE: u32 = 0xc000;
#[cfg(feature = "PROM")]
pub const IP27_KLCONFIG_COUNT: u32 = 1;
#[cfg(feature = "PROM")]
pub const IP27_KLCONFIG_STRIDE: u32 = 0;

#[cfg(feature = "PROM")]
pub const IP27_NMI_OFFSET: u32 = 0x3000;
#[cfg(feature = "PROM")]
pub const IP27_NMI_SIZE: u32 = 0x40;
#[cfg(feature = "PROM")]
pub const IP27_NMI_COUNT: u32 = 2;
#[cfg(feature = "PROM")]
pub const IP27_NMI_STRIDE: u32 = 0x40;

#[cfg(feature = "PROM")]
pub const IP27_PI_ERROR_OFFSET: u32 = 0x12000;
#[cfg(feature = "PROM")]
pub const IP27_PI_ERROR_SIZE: u32 = 0x4000;
#[cfg(feature = "PROM")]
pub const IP27_PI_ERROR_COUNT: u32 = 1;
#[cfg(feature = "PROM")]
pub const IP27_PI_ERROR_STRIDE: u32 = 0;

#[cfg(feature = "PROM")]
pub const IP27_SYMMON_STK_OFFSET: u32 = 0x25000;
#[cfg(feature = "PROM")]
pub const IP27_SYMMON_STK_SIZE: u32 = 0xe000;
#[cfg(feature = "PROM")]
pub const IP27_SYMMON_STK_COUNT: u32 = 2;
/* IP27_SYMMON_STK_STRIDE must be >= SYMMON_STACK_SIZE. */
#[cfg(feature = "PROM")]
pub const IP27_SYMMON_STK_STRIDE: u32 = 0x7000;

#[cfg(feature = "PROM")]
pub const IP27_FREEMEM_OFFSET: u32 = 0x19000;
#[cfg(feature = "PROM")]
pub const IP27_FREEMEM_SIZE: i32 = -1;
#[cfg(feature = "PROM")]
pub const IP27_FREEMEM_COUNT: u32 = 1;
#[cfg(feature = "PROM")]
pub const IP27_FREEMEM_STRIDE: u32 = 0;

/* There will be only one of these in a partition so the IO6 must set it up. */
pub const IO6_GDA_OFFSET: u32 = 0x11000;
pub const IO6_GDA_SIZE: u32 = 0x400;
pub const IO6_GDA_COUNT: u32 = 1;
pub const IO6_GDA_STRIDE: u32 = 0;

/* Save area of kernel NMI regs in the prom format. */
pub const IP27_NMI_KREGS_OFFSET: u32 = 0x11400;
pub const IP27_NMI_KREGS_CPU_SIZE: u32 = 0x200;

/* Save area of kernel NMI regs in eframe format. */
pub const IP27_NMI_EFRAME_OFFSET: u32 = 0x11800;
pub const IP27_NMI_EFRAME_SIZE: u32 = 0x200;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
