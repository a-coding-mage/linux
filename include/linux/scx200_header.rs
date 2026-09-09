/* SPDX-License-Identifier: GPL-2.0 */
/* linux/include/linux/scx200.h

   Copyright (c) 2001,2002 Christer Weinigel <wingel@nano-system.com>

   Defines for the National Semiconductor SCx200 Processors
*/

/* Interesting stuff for the National Semiconductor SCx200 CPU */

unsafe extern "C" {
    pub static mut scx200_cb_base: ::core::ffi::c_uint;
}

#[inline]
pub unsafe fn scx200_cb_present() -> bool {
    unsafe { scx200_cb_base != 0 }
}

/* F0 PCI Header/Bridge Configuration Registers */
pub const SCX200_DOCCS_BASE: u32 = 0x78; /* DOCCS Base Address Register */
pub const SCX200_DOCCS_CTRL: u32 = 0x7c; /* DOCCS Control Register */

/* GPIO Register Block */
pub const SCX200_GPIO_SIZE: u32 = 0x2c; /* Size of GPIO register block */

/* General Configuration Block */
pub const SCX200_CB_BASE_FIXED: u32 = 0x9000; /* Base fixed at 0x9000 according to errata? */

/* Watchdog Timer */
pub const SCX200_WDT_OFFSET: u32 = 0x00; /* offset within configuration block */
pub const SCX200_WDT_SIZE: u32 = 0x05; /* size */

pub const SCX200_WDT_WDTO: u32 = 0x00; /* Time-Out Register */
pub const SCX200_WDT_WDCNFG: u32 = 0x02; /* Configuration Register */
pub const SCX200_WDT_WDSTS: u32 = 0x04; /* Status Register */
pub const SCX200_WDT_WDSTS_WDOVF: u32 = 1 << 0; /* Overflow bit */

/* High Resolution Timer */
pub const SCX200_TIMER_OFFSET: u32 = 0x08;
pub const SCX200_TIMER_SIZE: u32 = 0x06;

/* Clock Generators */
pub const SCX200_CLOCKGEN_OFFSET: u32 = 0x10;
pub const SCX200_CLOCKGEN_SIZE: u32 = 0x10;

/* Pin Multiplexing and Miscellaneous Configuration Registers */
pub const SCX200_MISC_OFFSET: u32 = 0x30;
pub const SCX200_MISC_SIZE: u32 = 0x10;

pub const SCX200_PMR: u32 = 0x30; /* Pin Multiplexing Register */
pub const SCX200_MCR: u32 = 0x34; /* Miscellaneous Configuration Register */
pub const SCX200_INTSEL: u32 = 0x38; /* Interrupt Selection Register */
pub const SCX200_IID: u32 = 0x3c; /* IA On a Chip Identification Number Reg */
pub const SCX200_REV: u32 = 0x3d; /* Revision Register */
pub const SCX200_CBA: u32 = 0x3e; /* Configuration Base Address Register */
pub const SCX200_CBA_SCRATCH: u32 = 0x64; /* Configuration Base Address Scratchpad */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
