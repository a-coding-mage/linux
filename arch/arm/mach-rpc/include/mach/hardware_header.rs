/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/mach-rpc/include/mach/hardware.h
 *
 *  Copyright (C) 1996-1999 Russell King.
 *
 *  This file contains the hardware definitions of the RiscPC series machines.
 */

// Dependency supplied by mach/memory.h in the original header.

/*
 * What hardware must be present
 */
pub const HAS_IOMD: bool = true;
pub const HAS_VIDC20: bool = true;

/* Hardware addresses of major areas.
 *  *_START is the physical address
 *  *_SIZE  is the size of the region
 *  *_BASE  is the virtual address
 */
pub const RPC_RAM_SIZE: usize = 0x10000000;
pub const RPC_RAM_START: usize = 0x10000000;

pub const EASI_SIZE: usize = 0x08000000; /* EASI I/O */
pub const EASI_START: usize = 0x08000000;
pub const EASI_BASE: usize = IOMEM(0xe5000000);

pub const IO_START: usize = 0x03000000; /* I/O */
pub const IO_SIZE: usize = 0x01000000;
pub const IO_BASE: usize = IOMEM(0xe0000000);

pub const SCREEN_START: usize = 0x02000000; /* VRAM */
pub const SCREEN_END: usize = 0xdfc00000;
pub const SCREEN_BASE: usize = 0xdf800000;

pub const UNCACHEABLE_ADDR: usize = FLUSH_BASE + 0x10000;

/*
 * IO Addresses
 */
pub const ECARD_EASI_BASE: usize = EASI_BASE;
pub const VIDC_BASE: usize = IO_BASE + 0x00400000;
pub const EXPMASK_BASE: usize = IO_BASE + 0x00360000;
pub const ECARD_IOC4_BASE: usize = IO_BASE + 0x00270000;
pub const ECARD_IOC_BASE: usize = IO_BASE + 0x00240000;
pub const IOMD_BASE: usize = IO_BASE + 0x00200000;
pub const IOC_BASE: usize = IO_BASE + 0x00200000;
pub const ECARD_MEMC8_BASE: usize = IO_BASE + 0x0002b000;
pub const FLOPPYDMA_BASE: usize = IO_BASE + 0x0002a000;
pub const PCIO_BASE: usize = IO_BASE + 0x00010000;
pub const ECARD_MEMC_BASE: usize = IO_BASE + 0x00000000;

#[inline]
pub unsafe fn vidc_writel(val: u32) {
    __raw_writel(val, VIDC_BASE);
}

pub const NETSLOT_BASE: usize = 0x0302b000;
pub const NETSLOT_SIZE: usize = 0x00001000;

pub const PODSLOT_IOC0_BASE: usize = 0x03240000;
pub const PODSLOT_IOC4_BASE: usize = 0x03270000;
pub const PODSLOT_IOC_SIZE: usize = 1 << 14;
pub const PODSLOT_MEMC_BASE: usize = 0x03000000;
pub const PODSLOT_MEMC_SIZE: usize = 1 << 14;
pub const PODSLOT_EASI_BASE: usize = 0x08000000;
pub const PODSLOT_EASI_SIZE: usize = 1 << 24;

pub const EXPMASK_STATUS: usize = EXPMASK_BASE + 0x00;
pub const EXPMASK_ENABLE: usize = EXPMASK_BASE + 0x04;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
