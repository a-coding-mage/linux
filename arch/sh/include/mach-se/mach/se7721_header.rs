/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2008 Renesas Solutions Corp.
 *
 * Hitachi UL SolutionEngine 7721 Support.
 */

// Dependencies supplied by the surrounding SH kernel translation:
// linux/sh_intc.h, asm/addrspace.h, and asm/io_generic.h.

/* Box specific addresses. */
pub const SE_AREA0_WIDTH: u32 = 2; // Area0: 32bit
pub const PA_ROM: u32 = 0xa0000000; // EPROM
pub const PA_ROM_SIZE: u32 = 0x00200000; // EPROM size 2M byte
pub const PA_FROM: u32 = 0xa1000000; // Flash-ROM
pub const PA_FROM_SIZE: u32 = 0x01000000; // Flash-ROM size 16M byte
pub const PA_EXT1: u32 = 0xa4000000;
pub const PA_EXT1_SIZE: u32 = 0x04000000;
pub const PA_SDRAM: u32 = 0xaC000000; // SDRAM(Area3) 64MB
pub const PA_SDRAM_SIZE: u32 = 0x04000000;

pub const PA_EXT4: u32 = 0xb0000000;
pub const PA_EXT4_SIZE: u32 = 0x04000000;

pub const PA_PERIPHERAL: u32 = 0xB8000000;

pub const PA_PCIC: u32 = PA_PERIPHERAL;
pub const PA_MRSHPC: u32 = PA_PERIPHERAL + 0x003fffe0;
pub const PA_MRSHPC_MW1: u32 = PA_PERIPHERAL + 0x00400000;
pub const PA_MRSHPC_MW2: u32 = PA_PERIPHERAL + 0x00500000;
pub const PA_MRSHPC_IO: u32 = PA_PERIPHERAL + 0x00600000;
pub const MRSHPC_OPTION: u32 = PA_MRSHPC + 6;
pub const MRSHPC_CSR: u32 = PA_MRSHPC + 8;
pub const MRSHPC_ISR: u32 = PA_MRSHPC + 10;
pub const MRSHPC_ICR: u32 = PA_MRSHPC + 12;
pub const MRSHPC_CPWCR: u32 = PA_MRSHPC + 14;
pub const MRSHPC_MW0CR1: u32 = PA_MRSHPC + 16;
pub const MRSHPC_MW1CR1: u32 = PA_MRSHPC + 18;
pub const MRSHPC_IOWCR1: u32 = PA_MRSHPC + 20;
pub const MRSHPC_MW0CR2: u32 = PA_MRSHPC + 22;
pub const MRSHPC_MW1CR2: u32 = PA_MRSHPC + 24;
pub const MRSHPC_IOWCR2: u32 = PA_MRSHPC + 26;
pub const MRSHPC_CDCR: u32 = PA_MRSHPC + 28;
pub const MRSHPC_PCIC_INFO: u32 = PA_MRSHPC + 30;

pub const PA_LED: u32 = 0xB6800000; // 8bit LED
pub const PA_FPGA: u32 = 0xB7000000; // FPGA base address

// The value is supplied by the external SH interrupt-address-space helper.
#[allow(improper_ctypes)]
unsafe extern "C" {
    pub fn evt2irq(event: u32) -> u32;
}

#[macro_export]
macro_rules! MRSHPC_IRQ0 {
    () => {{ unsafe { $crate::evt2irq(0x340) } }};
}

pub unsafe extern "C" {
    pub fn init_se7721_IRQ();
}

// __IO_PREFIX is se7721; the generic I/O declarations are supplied externally.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
