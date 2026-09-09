/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2004, 2007-2010, 2011-2012, 2019-20 Synopsys, Inc. (www.synopsys.com)
 *
 * MMUv3 (arc700) / MMUv4 (archs) are software page walked and software managed.
 * This file contains the TLB access registers and commands
 */

/* Dependency: <soc/arc/arc_aux.h> */

/* TLB Management regs */
pub const ARC_REG_MMU_BCR: u32 = 0x06f;

/* CONFIG_ARC_MMU_V3 selects the first register layout. */
#[cfg(feature = "CONFIG_ARC_MMU_V3")]
pub const ARC_REG_TLBPD0: u32 = 0x405;
#[cfg(not(feature = "CONFIG_ARC_MMU_V3"))]
pub const ARC_REG_TLBPD0: u32 = 0x460;
#[cfg(feature = "CONFIG_ARC_MMU_V3")]
pub const ARC_REG_TLBPD1: u32 = 0x406;
#[cfg(not(feature = "CONFIG_ARC_MMU_V3"))]
pub const ARC_REG_TLBPD1: u32 = 0x461;
#[cfg(feature = "CONFIG_ARC_MMU_V3")]
pub const ARC_REG_TLBPD1HI: u32 = 0;
#[cfg(not(feature = "CONFIG_ARC_MMU_V3"))]
pub const ARC_REG_TLBPD1HI: u32 = 0x463;
#[cfg(feature = "CONFIG_ARC_MMU_V3")]
pub const ARC_REG_TLBINDEX: u32 = 0x407;
#[cfg(not(feature = "CONFIG_ARC_MMU_V3"))]
pub const ARC_REG_TLBINDEX: u32 = 0x464;
#[cfg(feature = "CONFIG_ARC_MMU_V3")]
pub const ARC_REG_TLBCOMMAND: u32 = 0x408;
#[cfg(not(feature = "CONFIG_ARC_MMU_V3"))]
pub const ARC_REG_TLBCOMMAND: u32 = 0x465;
#[cfg(feature = "CONFIG_ARC_MMU_V3")]
pub const ARC_REG_PID: u32 = 0x409;
#[cfg(not(feature = "CONFIG_ARC_MMU_V3"))]
pub const ARC_REG_PID: u32 = 0x468;
#[cfg(feature = "CONFIG_ARC_MMU_V3")]
pub const ARC_REG_SCRATCH_DATA0: u32 = 0x418;
#[cfg(not(feature = "CONFIG_ARC_MMU_V3"))]
pub const ARC_REG_SCRATCH_DATA0: u32 = 0x46c;

/* Bits in MMU PID reg */
pub const __TLB_ENABLE: u32 = 1 << 31;
pub const __PROG_ENABLE: u32 = 1 << 30;
pub const MMU_ENABLE: u32 = __TLB_ENABLE | __PROG_ENABLE;

/* Bits in TLB Index reg */
pub const TLB_LKUP_ERR: u32 = 0x80000000;
#[cfg(feature = "CONFIG_ARC_MMU_V3")]
pub const TLB_DUP_ERR: u32 = TLB_LKUP_ERR | 0x00000001;
#[cfg(not(feature = "CONFIG_ARC_MMU_V3"))]
pub const TLB_DUP_ERR: u32 = TLB_LKUP_ERR | 0x40000000;

/* TLB Commands */
pub const TLBWrite: u32 = 0x1;
pub const TLBRead: u32 = 0x2;
pub const TLBGetIndex: u32 = 0x3;
pub const TLBProbe: u32 = 0x4;
pub const TLBWriteNI: u32 = 0x5; /* write JTLB without inv uTLBs */
pub const TLBIVUTLB: u32 = 0x6; /* explicitly inv uTLBs */
#[cfg(feature = "CONFIG_ARC_MMU_V4")]
pub const TLBInsertEntry: u32 = 0x7;
#[cfg(feature = "CONFIG_ARC_MMU_V4")]
pub const TLBDeleteEntry: u32 = 0x8;

/* Masks for actual TLB "PD"s; supplied by dependent headers. */
pub const PTE_BITS_IN_PD0: u32 = _PAGE_GLOBAL | _PAGE_PRESENT | _PAGE_HW_SZ;
pub const PTE_BITS_RWX: u32 = _PAGE_EXECUTE | _PAGE_WRITE | _PAGE_READ;
pub const PTE_BITS_NON_RWX_IN_PD1: u32 = PAGE_MASK_PHYS | _PAGE_CACHEABLE;

/* External declarations supplied by dependent code. */
pub struct mm_struct;
extern "C" {
    pub fn pae40_exist_but_not_enab() -> ::core::ffi::c_int;
    pub fn write_aux_reg(reg: u32, value: ::core::ffi::c_ulong);
}

pub unsafe fn is_pae40_enabled() -> ::core::ffi::c_int {
    if cfg!(feature = "CONFIG_ARC_HAS_PAE40") { 1 } else { 0 }
}

pub unsafe fn mmu_setup_asid(_mm: *mut mm_struct, asid: ::core::ffi::c_ulong) {
    write_aux_reg(ARC_REG_PID, asid | MMU_ENABLE as ::core::ffi::c_ulong);
}

pub unsafe fn mmu_setup_pgd(_mm: *mut mm_struct, pgd: *mut ::core::ffi::c_void) {
    /* PGD cached in MMU reg to avoid 3 mem lookups: task->mm->pgd */
    #[cfg(feature = "CONFIG_ISA_ARCV2")]
    write_aux_reg(ARC_REG_SCRATCH_DATA0, pgd as u32 as ::core::ffi::c_ulong);
}

/* The assembler-only ARC_MMU_REENABLE macro is preserved as an interface note:
 * lr reg, [ARC_REG_PID]; or reg, reg, MMU_ENABLE; sr reg, [ARC_REG_PID].
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
