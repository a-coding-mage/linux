/* SPDX-License-Identifier: GPL-2.0 */
// Original header guard: _ASM_POWERPC_OHARE_H
// The following declarations are present under the C build condition __KERNEL__.

/*
 * ohare.h: definitions for using the "O'Hare" I/O controller chip.
 *
 * Copyright (C) 1997 Paul Mackerras.
 *
 * BenH: Changed to match those of heathrow (but not all of them). Please
 *       check if I didn't break anything (especially the media bay).
 */

/* offset from ohare base for feature control register */
pub const OHARE_MBCR: u32 = 0x34;
pub const OHARE_FCR: u32 = 0x38;

/*
 * Bits in feature control register.
 * These were mostly derived by experiment on a powerbook 3400
 * and may differ for other machines.
 */
pub const OH_SCC_RESET: u32 = 1;
pub const OH_BAY_POWER_N: u32 = 2; /* a guess */
pub const OH_BAY_PCI_ENABLE: u32 = 4; /* a guess */
pub const OH_BAY_IDE_ENABLE: u32 = 8;
pub const OH_BAY_FLOPPY_ENABLE: u32 = 0x10;
pub const OH_IDE0_ENABLE: u32 = 0x20;
pub const OH_IDE0_RESET_N: u32 = 0x40; /* a guess */
pub const OH_BAY_DEV_MASK: u32 = 0x1c;
pub const OH_BAY_RESET_N: u32 = 0x80;
pub const OH_IOBUS_ENABLE: u32 = 0x100; /* IOBUS seems to be IDE */
pub const OH_SCC_ENABLE: u32 = 0x200;
pub const OH_MESH_ENABLE: u32 = 0x400;
pub const OH_FLOPPY_ENABLE: u32 = 0x800;
pub const OH_SCCA_IO: u32 = 0x4000;
pub const OH_SCCB_IO: u32 = 0x8000;
pub const OH_VIA_ENABLE: u32 = 0x10000; /* Is apparently wrong, to be verified */
pub const OH_IDE1_RESET_N: u32 = 0x800000;

/*
 * Bits to set in the feature control register on PowerBooks.
 * OH_IDE_ENABLE is supplied by the surrounding translated dependencies,
 * as it was referenced by the original header but not defined here.
 */
pub const PBOOK_FEATURES: u32 =
    OH_IDE_ENABLE | OH_SCC_ENABLE | OH_MESH_ENABLE | OH_SCCA_IO | OH_SCCB_IO;

/*
 * A magic value to put into the feature control register of the
 * "ohare" I/O controller on Starmaxes to enable the IDE CD interface.
 * Contributed by Harry Eaton.
 */
pub const STARMAX_FEATURES: u32 = 0xbeff7a;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
