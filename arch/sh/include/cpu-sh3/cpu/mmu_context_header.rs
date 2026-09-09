/* SPDX-License-Identifier: GPL-2.0
 *
 * include/asm-sh/cpu-sh3/mmu_context.h
 *
 * Copyright (C) 1999 Niibe Yutaka
 */

pub const MMU_PTEH: u32 = 0xFFFF_FFF0; /* Page table entry register HIGH */
pub const MMU_PTEL: u32 = 0xFFFF_FFF4; /* Page table entry register LOW */
pub const MMU_TTB: u32 = 0xFFFF_FFF8; /* Translation table base register */
pub const MMU_TEA: u32 = 0xFFFF_FFFC; /* TLB Exception Address */

pub const MMUCR: u32 = 0xFFFF_FFE0; /* MMU Control Register */
pub const MMUCR_TI: u32 = 1 << 2; /* TLB flush bit */

pub const MMU_TLB_ADDRESS_ARRAY: u32 = 0xF200_0000;
pub const MMU_PAGE_ASSOC_BIT: u32 = 0x80;

pub const MMU_NTLB_ENTRIES: u32 = 128; /* for 7708 */
pub const MMU_NTLB_WAYS: u32 = 4;
pub const MMU_CONTROL_INIT: u32 = 0x007; /* SV=0, TF=1, IX=1, AT=1 */

pub const TRA: u32 = 0xFFFF_FFD0;
pub const EXPEVT: u32 = 0xFFFF_FFD4;

/* C build condition:
 * CONFIG_CPU_SUBTYPE_SH7705 || CONFIG_CPU_SUBTYPE_SH7706 ||
 * CONFIG_CPU_SUBTYPE_SH7707 || CONFIG_CPU_SUBTYPE_SH7709 ||
 * CONFIG_CPU_SUBTYPE_SH7710 || CONFIG_CPU_SUBTYPE_SH7712 ||
 * CONFIG_CPU_SUBTYPE_SH7720 || CONFIG_CPU_SUBTYPE_SH7721
 */
#[cfg(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7705",
    feature = "CONFIG_CPU_SUBTYPE_SH7706",
    feature = "CONFIG_CPU_SUBTYPE_SH7707",
    feature = "CONFIG_CPU_SUBTYPE_SH7709",
    feature = "CONFIG_CPU_SUBTYPE_SH7710",
    feature = "CONFIG_CPU_SUBTYPE_SH7712",
    feature = "CONFIG_CPU_SUBTYPE_SH7720",
    feature = "CONFIG_CPU_SUBTYPE_SH7721",
))]
pub const INTEVT: u32 = 0xA400_0000; /* INTEVTE2(0xa4000000) */

#[cfg(not(any(
    feature = "CONFIG_CPU_SUBTYPE_SH7705",
    feature = "CONFIG_CPU_SUBTYPE_SH7706",
    feature = "CONFIG_CPU_SUBTYPE_SH7707",
    feature = "CONFIG_CPU_SUBTYPE_SH7709",
    feature = "CONFIG_CPU_SUBTYPE_SH7710",
    feature = "CONFIG_CPU_SUBTYPE_SH7712",
    feature = "CONFIG_CPU_SUBTYPE_SH7720",
    feature = "CONFIG_CPU_SUBTYPE_SH7721",
)))]
pub const INTEVT: u32 = 0xFFFF_FFD8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
