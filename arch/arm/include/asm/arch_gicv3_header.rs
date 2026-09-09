/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm/include/asm/arch_gicv3.h
 *
 * Copyright (C) 2015 ARM Ltd.
 */

// C dependencies: linux/io.h, linux/io-64-nonatomic-lo-hi.h, asm/barrier.h,
// asm/cacheflush.h, and asm/cp15.h.

// The following register names are supplied by the CP15 implementation.
pub const ICC_EOIR1: u32 = __ACCESS_CP15!(c12, 0, c12, 1);
pub const ICC_DIR: u32 = __ACCESS_CP15!(c12, 0, c11, 1);
pub const ICC_IAR1: u32 = __ACCESS_CP15!(c12, 0, c12, 0);
pub const ICC_SGI1R: u32 = __ACCESS_CP15_64!(0, c12);
pub const ICC_PMR: u32 = __ACCESS_CP15!(c4, 0, c6, 0);
pub const ICC_CTLR: u32 = __ACCESS_CP15!(c12, 0, c12, 4);
pub const ICC_SRE: u32 = __ACCESS_CP15!(c12, 0, c12, 5);
pub const ICC_IGRPEN1: u32 = __ACCESS_CP15!(c12, 0, c12, 7);
pub const ICC_BPR1: u32 = __ACCESS_CP15!(c12, 0, c12, 3);
pub const ICC_RPR: u32 = __ACCESS_CP15!(c12, 0, c11, 3);

pub const ICC_AP0R0: u32 = __ACCESS_CP15!(c12, 0, c8, 4 | 0);
pub const ICC_AP0R1: u32 = __ACCESS_CP15!(c12, 0, c8, 4 | 1);
pub const ICC_AP0R2: u32 = __ACCESS_CP15!(c12, 0, c8, 4 | 2);
pub const ICC_AP0R3: u32 = __ACCESS_CP15!(c12, 0, c8, 4 | 3);
pub const ICC_AP1R0: u32 = __ACCESS_CP15!(c12, 0, c9, 0);
pub const ICC_AP1R1: u32 = __ACCESS_CP15!(c12, 0, c9, 1);
pub const ICC_AP1R2: u32 = __ACCESS_CP15!(c12, 0, c9, 2);
pub const ICC_AP1R3: u32 = __ACCESS_CP15!(c12, 0, c9, 3);

#[inline] pub unsafe fn write_ICC_EOIR1_EL1(val: u32) { write_sysreg!(val, ICC_EOIR1); }
#[inline] pub unsafe fn read_ICC_EOIR1_EL1() -> u32 { read_sysreg!(ICC_EOIR1) }
#[inline] pub unsafe fn write_ICC_PMR_EL1(val: u32) { write_sysreg!(val, ICC_PMR); }
#[inline] pub unsafe fn read_ICC_PMR_EL1() -> u32 { read_sysreg!(ICC_PMR) }
#[inline] pub unsafe fn write_ICC_AP0R0_EL1(val: u32) { write_sysreg!(val, ICC_AP0R0); }
#[inline] pub unsafe fn read_ICC_AP0R0_EL1() -> u32 { read_sysreg!(ICC_AP0R0) }
#[inline] pub unsafe fn write_ICC_AP0R1_EL1(val: u32) { write_sysreg!(val, ICC_AP0R1); }
#[inline] pub unsafe fn read_ICC_AP0R1_EL1() -> u32 { read_sysreg!(ICC_AP0R1) }
#[inline] pub unsafe fn write_ICC_AP0R2_EL1(val: u32) { write_sysreg!(val, ICC_AP0R2); }
#[inline] pub unsafe fn read_ICC_AP0R2_EL1() -> u32 { read_sysreg!(ICC_AP0R2) }
#[inline] pub unsafe fn write_ICC_AP0R3_EL1(val: u32) { write_sysreg!(val, ICC_AP0R3); }
#[inline] pub unsafe fn read_ICC_AP0R3_EL1() -> u32 { read_sysreg!(ICC_AP0R3) }
#[inline] pub unsafe fn write_ICC_AP1R0_EL1(val: u32) { write_sysreg!(val, ICC_AP1R0); }
#[inline] pub unsafe fn read_ICC_AP1R0_EL1() -> u32 { read_sysreg!(ICC_AP1R0) }
#[inline] pub unsafe fn write_ICC_AP1R1_EL1(val: u32) { write_sysreg!(val, ICC_AP1R1); }
#[inline] pub unsafe fn read_ICC_AP1R1_EL1() -> u32 { read_sysreg!(ICC_AP1R1) }
#[inline] pub unsafe fn write_ICC_AP1R2_EL1(val: u32) { write_sysreg!(val, ICC_AP1R2); }
#[inline] pub unsafe fn read_ICC_AP1R2_EL1() -> u32 { read_sysreg!(ICC_AP1R2) }
#[inline] pub unsafe fn write_ICC_AP1R3_EL1(val: u32) { write_sysreg!(val, ICC_AP1R3); }
#[inline] pub unsafe fn read_ICC_AP1R3_EL1() -> u32 { read_sysreg!(ICC_AP1R3) }

#[inline] pub unsafe fn gic_write_dir(val: u32) { write_sysreg!(val, ICC_DIR); isb!(); }
#[inline] pub unsafe fn gic_read_iar() -> u32 { let irqstat = read_sysreg!(ICC_IAR1); dsb!(sy); irqstat }
#[inline] pub unsafe fn gic_write_ctlr(val: u32) { write_sysreg!(val, ICC_CTLR); isb!(); }
#[inline] pub unsafe fn gic_read_ctlr() -> u32 { read_sysreg!(ICC_CTLR) }
#[inline] pub unsafe fn gic_write_grpen1(val: u32) { write_sysreg!(val, ICC_IGRPEN1); isb!(); }
#[inline] pub unsafe fn gic_write_sgi1r(val: u64) { write_sysreg!(val, ICC_SGI1R); }
#[inline] pub unsafe fn gic_read_sre() -> u32 { read_sysreg!(ICC_SRE) }
#[inline] pub unsafe fn gic_write_sre(val: u32) { write_sysreg!(val, ICC_SRE); isb!(); }
#[inline] pub unsafe fn gic_write_bpr1(val: u32) { write_sysreg!(val, ICC_BPR1); }
#[inline] pub unsafe fn gic_read_pmr() -> u32 { read_sysreg!(ICC_PMR) }
#[inline] pub unsafe fn gic_write_pmr(val: u32) { write_sysreg!(val, ICC_PMR); }
#[inline] pub unsafe fn gic_read_rpr() -> u32 { read_sysreg!(ICC_RPR) }

#[inline] pub unsafe fn __gic_writeq_nonatomic(val: u64, addr: *mut core::ffi::c_void) {
    writel_relaxed!(val as u32, addr);
    writel_relaxed!((val >> 32) as u32, addr.add(4));
}

#[inline] pub unsafe fn __gic_readq_nonatomic(addr: *const core::ffi::c_void) -> u64 {
    let mut val = readl_relaxed!(addr) as u64;
    val |= (readl_relaxed!(addr.add(4)) as u64) << 32;
    val
}

#[inline] pub unsafe fn gicr_write_vpendbaser(val: u64, addr: *mut core::ffi::c_void) {
    let mut tmp = readl_relaxed!(addr.add(4));
    if tmp & ((GICR_VPENDBASER_Valid >> 32) as u32) != 0 {
        tmp &= !((GICR_VPENDBASER_Valid >> 32) as u32);
        writel_relaxed!(tmp, addr.add(4));
    }
    __gic_writeq_nonatomic(val, addr);
}

#[inline] pub fn gic_prio_masking_enabled() -> bool { false }
#[inline] pub unsafe fn gic_pmr_mask_irqs() { WARN_ON_ONCE!(true); }
#[inline] pub fn gic_unmask_pnmis() {}
#[inline] pub fn gic_has_relaxed_pmr_sync() -> bool { false }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
