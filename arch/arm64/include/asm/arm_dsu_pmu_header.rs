/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ARM DynamIQ Shared Unit (DSU) PMU Low level register access routines.
 *
 * Copyright (C) ARM Limited, 2017.
 *
 * Author: Suzuki K Poulose <suzuki.poulose@arm.com>
 */

// C header dependencies: linux/bitops.h, linux/build_bug.h,
// linux/compiler.h, linux/types.h, asm/barrier.h, and asm/sysreg.h.

extern "C" {
    fn read_sysreg_s(reg: u32) -> u64;
    fn write_sysreg_s(val: u64, reg: u32);
    fn isb();
}

const fn sys_reg(op0: u32, op1: u32, crn: u32, crm: u32, op2: u32) -> u32 {
    (op0 << 19) | (op1 << 16) | (crn << 12) | (crm << 8) | (op2 << 5)
}

pub const CLUSTERPMCR_EL1: u32 = sys_reg(3, 0, 15, 5, 0);
pub const CLUSTERPMCNTENSET_EL1: u32 = sys_reg(3, 0, 15, 5, 1);
pub const CLUSTERPMCNTENCLR_EL1: u32 = sys_reg(3, 0, 15, 5, 2);
pub const CLUSTERPMOVSSET_EL1: u32 = sys_reg(3, 0, 15, 5, 3);
pub const CLUSTERPMOVSCLR_EL1: u32 = sys_reg(3, 0, 15, 5, 4);
pub const CLUSTERPMSELR_EL1: u32 = sys_reg(3, 0, 15, 5, 5);
pub const CLUSTERPMINTENSET_EL1: u32 = sys_reg(3, 0, 15, 5, 6);
pub const CLUSTERPMINTENCLR_EL1: u32 = sys_reg(3, 0, 15, 5, 7);
pub const CLUSTERPMCCNTR_EL1: u32 = sys_reg(3, 0, 15, 6, 0);
pub const CLUSTERPMXEVTYPER_EL1: u32 = sys_reg(3, 0, 15, 6, 1);
pub const CLUSTERPMXEVCNTR_EL1: u32 = sys_reg(3, 0, 15, 6, 2);
pub const CLUSTERPMMDCR_EL1: u32 = sys_reg(3, 0, 15, 6, 3);
pub const CLUSTERPMCEID0_EL1: u32 = sys_reg(3, 0, 15, 6, 4);
pub const CLUSTERPMCEID1_EL1: u32 = sys_reg(3, 0, 15, 6, 5);

#[inline]
pub unsafe fn __dsu_pmu_read_pmcr() -> u32 { read_sysreg_s(CLUSTERPMCR_EL1) as u32 }

#[inline]
pub unsafe fn __dsu_pmu_write_pmcr(val: u32) {
    write_sysreg_s(val as u64, CLUSTERPMCR_EL1);
    isb();
}

#[inline]
pub unsafe fn __dsu_pmu_get_reset_overflow() -> u32 {
    let val = read_sysreg_s(CLUSTERPMOVSCLR_EL1) as u32;
    /* Clear the bit */
    write_sysreg_s(val as u64, CLUSTERPMOVSCLR_EL1);
    isb();
    val
}

#[inline]
pub unsafe fn __dsu_pmu_select_counter(counter: i32) {
    write_sysreg_s(counter as u64, CLUSTERPMSELR_EL1);
    isb();
}

#[inline]
pub unsafe fn __dsu_pmu_read_counter(counter: i32) -> u64 {
    __dsu_pmu_select_counter(counter);
    read_sysreg_s(CLUSTERPMXEVCNTR_EL1)
}

#[inline]
pub unsafe fn __dsu_pmu_write_counter(counter: i32, val: u64) {
    __dsu_pmu_select_counter(counter);
    write_sysreg_s(val, CLUSTERPMXEVCNTR_EL1);
    isb();
}

#[inline]
pub unsafe fn __dsu_pmu_set_event(counter: i32, event: u32) {
    __dsu_pmu_select_counter(counter);
    write_sysreg_s(event as u64, CLUSTERPMXEVTYPER_EL1);
    isb();
}

#[inline]
pub unsafe fn __dsu_pmu_read_pmccntr() -> u64 { read_sysreg_s(CLUSTERPMCCNTR_EL1) }

#[inline]
pub unsafe fn __dsu_pmu_write_pmccntr(val: u64) {
    write_sysreg_s(val, CLUSTERPMCCNTR_EL1);
    isb();
}

#[inline]
pub unsafe fn __dsu_pmu_disable_counter(counter: i32) {
    write_sysreg_s(1u64.wrapping_shl(counter as u32), CLUSTERPMCNTENCLR_EL1);
    isb();
}

#[inline]
pub unsafe fn __dsu_pmu_enable_counter(counter: i32) {
    write_sysreg_s(1u64.wrapping_shl(counter as u32), CLUSTERPMCNTENSET_EL1);
    isb();
}

#[inline]
pub unsafe fn __dsu_pmu_counter_interrupt_enable(counter: i32) {
    write_sysreg_s(1u64.wrapping_shl(counter as u32), CLUSTERPMINTENSET_EL1);
    isb();
}

#[inline]
pub unsafe fn __dsu_pmu_counter_interrupt_disable(counter: i32) {
    write_sysreg_s(1u64.wrapping_shl(counter as u32), CLUSTERPMINTENCLR_EL1);
    isb();
}

#[inline]
pub unsafe fn __dsu_pmu_read_pmceid(n: i32) -> u32 {
    match n {
        0 => read_sysreg_s(CLUSTERPMCEID0_EL1) as u32,
        1 => read_sysreg_s(CLUSTERPMCEID1_EL1) as u32,
        _ => core::hint::unreachable_unchecked(),
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
