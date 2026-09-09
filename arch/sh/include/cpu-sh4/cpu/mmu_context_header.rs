/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/cpu-sh4/mmu_context.h
 *
 * Copyright (C) 1999 Niibe Yutaka
 */

pub const MMU_PTEH: u32 = 0xFF00_0000; /* Page table entry register HIGH */
pub const MMU_PTEL: u32 = 0xFF00_0004; /* Page table entry register LOW */
pub const MMU_TTB: u32 = 0xFF00_0008; /* Translation table base register */
pub const MMU_TEA: u32 = 0xFF00_000C; /* TLB Exception Address */
pub const MMU_PTEA: u32 = 0xFF00_0034; /* PTE assistance register */
pub const MMU_PTEAEX: u32 = 0xFF00_007C; /* PTE ASID extension register */

pub const MMUCR: u32 = 0xFF00_0010; /* MMU Control Register */

pub const MMU_TLB_ENTRY_SHIFT: u32 = 8;

pub const MMU_ITLB_ADDRESS_ARRAY: u32 = 0xF200_0000;
pub const MMU_ITLB_ADDRESS_ARRAY2: u32 = 0xF280_0000;
pub const MMU_ITLB_DATA_ARRAY: u32 = 0xF300_0000;
pub const MMU_ITLB_DATA_ARRAY2: u32 = 0xF380_0000;

pub const MMU_UTLB_ADDRESS_ARRAY: u32 = 0xF600_0000;
pub const MMU_UTLB_ADDRESS_ARRAY2: u32 = 0xF680_0000;
pub const MMU_UTLB_DATA_ARRAY: u32 = 0xF700_0000;
pub const MMU_UTLB_DATA_ARRAY2: u32 = 0xF780_0000;
pub const MMU_PAGE_ASSOC_BIT: u32 = 0x80;

/* CONFIG_MMU selects the target build configuration. */
#[cfg(CONFIG_MMU)]
pub const MMUCR_AT: u32 = 1 << 0;
#[cfg(not(CONFIG_MMU))]
pub const MMUCR_AT: u32 = 0;

pub const MMUCR_TI: u32 = 1 << 2;

pub const MMUCR_URB: u32 = 0x00FC_0000;
pub const MMUCR_URB_SHIFT: u32 = 18;
pub const MMUCR_URB_NENTRIES: u32 = 64;
pub const MMUCR_URC: u32 = 0x0000_FC00;
pub const MMUCR_URC_SHIFT: u32 = 10;

/* MMUCR_SE is enabled only for CONFIG_32BIT and CONFIG_CPU_SUBTYPE_ST40. */
#[cfg(all(CONFIG_32BIT, CONFIG_CPU_SUBTYPE_ST40))]
pub const MMUCR_SE: u32 = 1 << 4;
#[cfg(not(all(CONFIG_32BIT, CONFIG_CPU_SUBTYPE_ST40)))]
pub const MMUCR_SE: u32 = 0;

/* CONFIG_CPU_HAS_PTEAEX selects the target build configuration. */
#[cfg(CONFIG_CPU_HAS_PTEAEX)]
pub const MMUCR_AEX: u32 = 1 << 6;
#[cfg(not(CONFIG_CPU_HAS_PTEAEX))]
pub const MMUCR_AEX: u32 = 0;

/* CONFIG_X2TLB selects the target build configuration. */
#[cfg(CONFIG_X2TLB)]
pub const MMUCR_ME: u32 = 1 << 7;
#[cfg(not(CONFIG_X2TLB))]
pub const MMUCR_ME: u32 = 0;

/* CONFIG_SH_STORE_QUEUES selects the target build configuration. */
#[cfg(CONFIG_SH_STORE_QUEUES)]
pub const MMUCR_SQMD: u32 = 1 << 9;
#[cfg(not(CONFIG_SH_STORE_QUEUES))]
pub const MMUCR_SQMD: u32 = 0;

pub const MMU_NTLB_ENTRIES: u32 = 64;
pub const MMU_CONTROL_INIT: u32 = MMUCR_AT | MMUCR_TI | MMUCR_SQMD |
    MMUCR_ME | MMUCR_SE | MMUCR_AEX;

pub const TRA: u32 = 0xff00_0020;
pub const EXPEVT: u32 = 0xff00_0024;
pub const INTEVT: u32 = 0xff00_0028;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
