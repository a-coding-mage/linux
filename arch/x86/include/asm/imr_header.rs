/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * imr.h: Isolated Memory Region API
 *
 * Copyright(c) 2013 Intel Corporation.
 * Copyright(c) 2015 Bryan O'Donoghue <pure.logic@nexus-software.ie>
 */

/*
 * IMR agent access mask bits
 * See section 12.7.4.7 from quark-x1000-datasheet.pdf for register
 * definitions.
 */
pub const IMR_ESRAM_FLUSH: u32 = 1u32 << 31;
pub const IMR_CPU_SNOOP: u32 = 1u32 << 30; /* Applicable only to write */
pub const IMR_RMU: u32 = 1u32 << 29;
pub const IMR_VC1_SAI_ID3: u32 = 1u32 << 15;
pub const IMR_VC1_SAI_ID2: u32 = 1u32 << 14;
pub const IMR_VC1_SAI_ID1: u32 = 1u32 << 13;
pub const IMR_VC1_SAI_ID0: u32 = 1u32 << 12;
pub const IMR_VC0_SAI_ID3: u32 = 1u32 << 11;
pub const IMR_VC0_SAI_ID2: u32 = 1u32 << 10;
pub const IMR_VC0_SAI_ID1: u32 = 1u32 << 9;
pub const IMR_VC0_SAI_ID0: u32 = 1u32 << 8;
pub const IMR_CPU_0: u32 = 1u32 << 1; /* SMM mode */
pub const IMR_CPU: u32 = 1u32 << 0; /* Non SMM mode */
pub const IMR_ACCESS_NONE: u32 = 0;

/*
 * Read/Write access-all bits here include some reserved bits
 * These are the values firmware uses and are accepted by hardware.
 * The kernel defines read/write access-all in the same way as firmware
 * in order to have a consistent and crisp definition across firmware,
 * bootloader and kernel.
 */
pub const IMR_READ_ACCESS_ALL: u32 = 0xBFFFFFFF;
pub const IMR_WRITE_ACCESS_ALL: u32 = 0xFFFFFFFF;

/* Number of IMRs provided by Quark X1000 SoC */
pub const QUARK_X1000_IMR_MAX: u32 = 0x08;
pub const QUARK_X1000_IMR_REGBASE: u32 = 0x40;

/* IMR alignment bits - only bits 31:10 are checked for IMR validity */
pub const IMR_ALIGN: u32 = 0x400;
pub const IMR_MASK: u32 = IMR_ALIGN - 1;

extern "C" {
    pub fn imr_add_range(
        base: phys_addr_t,
        size: size_t,
        rmask: c_uint,
        wmask: c_uint,
    ) -> c_int;

    pub fn imr_remove_range(base: phys_addr_t, size: size_t) -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
