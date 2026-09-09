/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * arch/arm64/include/asm/arch_gicv3.h
 *
 * Copyright (C) 2015 ARM Ltd.
 */

// C dependency: <asm/sysreg.h>, <linux/irqchip/arm-gic-common.h>,
// <linux/stringify.h>, <asm/barrier.h>, and <asm/cacheflush.h>.

#[inline(always)]
pub unsafe fn read_gicreg(r: usize) -> u64 {
    read_sysreg_s(r)
}

#[inline(always)]
pub unsafe fn write_gicreg(v: u64, r: usize) {
    write_sysreg_s(v, r);
}

/* Low-level accessors. */

#[inline(always)]
pub unsafe fn gic_write_dir(irq: u32) {
    write_sysreg_s(irq as u64, SYS_ICC_DIR_EL1);
    isb();
}

#[inline]
pub unsafe fn gic_read_iar_common() -> u64 {
    let irqstat = read_sysreg_s(SYS_ICC_IAR1_EL1);
    dsb(SY);
    irqstat
}

/* Cavium ThunderX erratum 23154 and erratum 38545 workarounds. */
#[inline]
pub unsafe fn gic_read_iar_cavium_thunderx() -> u64 {
    let apr = read_sysreg_s(SYS_ICC_AP1R0_EL1);
    nops(8);
    let irqstat = read_sysreg_s(SYS_ICC_IAR1_EL1);
    nops(4);
    mb();

    /* Max priority groups implemented is only 32 */
    if likely(apr != read_sysreg_s(SYS_ICC_AP1R0_EL1)) {
        irqstat
    } else {
        0x3ff
    }
}

#[inline]
pub unsafe fn gic_read_iar() -> u64 {
    if alternative_has_cap_unlikely(ARM64_WORKAROUND_CAVIUM_23154) {
        gic_read_iar_cavium_thunderx()
    } else {
        gic_read_iar_common()
    }
}

#[inline]
pub unsafe fn gic_write_ctlr(val: u32) {
    write_sysreg_s(val as u64, SYS_ICC_CTLR_EL1);
    isb();
}

#[inline]
pub unsafe fn gic_read_ctlr() -> u32 {
    read_sysreg_s(SYS_ICC_CTLR_EL1) as u32
}

#[inline]
pub unsafe fn gic_write_grpen1(val: u32) {
    write_sysreg_s(val as u64, SYS_ICC_IGRPEN1_EL1);
    isb();
}

#[inline]
pub unsafe fn gic_write_sgi1r(val: u64) {
    write_sysreg_s(val, SYS_ICC_SGI1R_EL1);
}

#[inline]
pub unsafe fn gic_read_sre() -> u32 {
    read_sysreg_s(SYS_ICC_SRE_EL1) as u32
}

#[inline]
pub unsafe fn gic_write_sre(val: u32) {
    write_sysreg_s(val as u64, SYS_ICC_SRE_EL1);
    isb();
}

#[inline]
pub unsafe fn gic_write_bpr1(val: u32) {
    write_sysreg_s(val as u64, SYS_ICC_BPR1_EL1);
}

#[inline]
pub unsafe fn gic_read_pmr() -> u32 {
    read_sysreg_s(SYS_ICC_PMR_EL1) as u32
}

#[inline(always)]
pub unsafe fn gic_write_pmr(val: u32) {
    write_sysreg_s(val as u64, SYS_ICC_PMR_EL1);
}

#[inline]
pub unsafe fn gic_read_rpr() -> u32 {
    read_sysreg_s(SYS_ICC_RPR_EL1) as u32
}

#[inline]
pub unsafe fn gic_read_typer(c: *const u64) -> u64 { readq_relaxed(c) }
#[inline]
pub unsafe fn gic_write_irouter(v: u64, c: *mut u64) { writeq_relaxed(v, c) }
#[inline]
pub unsafe fn gic_read_lpir(c: *const u64) -> u64 { readq_relaxed(c) }
#[inline]
pub unsafe fn gic_write_lpir(v: u64, c: *mut u64) { writeq_relaxed(v, c) }

#[inline]
pub unsafe fn gic_flush_dcache_to_poc(a: *const core::ffi::c_void, l: usize) {
    dcache_clean_inval_poc(a as usize as u64, (a as usize + l) as u64);
}

#[inline]
pub unsafe fn gits_read_baser(c: *const u64) -> u64 { readq_relaxed(c) }
#[inline]
pub unsafe fn gits_write_baser(v: u64, c: *mut u64) { writeq_relaxed(v, c) }
#[inline]
pub unsafe fn gits_read_cbaser(c: *const u64) -> u64 { readq_relaxed(c) }
#[inline]
pub unsafe fn gits_write_cbaser(v: u64, c: *mut u64) { writeq_relaxed(v, c) }
#[inline]
pub unsafe fn gits_write_cwriter(v: u64, c: *mut u64) { writeq_relaxed(v, c) }
#[inline]
pub unsafe fn gicr_read_propbaser(c: *const u64) -> u64 { readq_relaxed(c) }
#[inline]
pub unsafe fn gicr_write_propbaser(v: u64, c: *mut u64) { writeq_relaxed(v, c) }
#[inline]
pub unsafe fn gicr_write_pendbaser(v: u64, c: *mut u64) { writeq_relaxed(v, c) }
#[inline]
pub unsafe fn gicr_read_pendbaser(c: *const u64) -> u64 { readq_relaxed(c) }
#[inline]
pub unsafe fn gicr_write_vpropbaser(v: u64, c: *mut u64) { writeq_relaxed(v, c) }
#[inline]
pub unsafe fn gicr_read_vpropbaser(c: *const u64) -> u64 { readq_relaxed(c) }
#[inline]
pub unsafe fn gicr_write_vpendbaser(v: u64, c: *mut u64) { writeq_relaxed(v, c) }
#[inline]
pub unsafe fn gicr_read_vpendbaser(c: *const u64) -> u64 { readq_relaxed(c) }

#[inline]
pub unsafe fn gic_prio_masking_enabled() -> bool { system_uses_irq_prio_masking() }

#[inline]
pub unsafe fn gic_pmr_mask_irqs() { gic_write_pmr(GIC_PRIO_IRQOFF); }

#[inline]
pub unsafe fn gic_unmask_pnmis() {
    if gic_prio_masking_enabled() {
        gic_pmr_mask_irqs();
        core::arch::asm!("msr daifclr, #3", options(nostack, preserves_flags));
    }
}

#[inline]
pub unsafe fn gic_has_relaxed_pmr_sync() -> bool {
    cpus_have_cap(ARM64_HAS_GIC_PRIO_RELAXED_SYNC)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
