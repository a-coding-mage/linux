/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2012 ARM Ltd. */

// Dependencies supplied by the architecture support layer: cp15 accessors,
// cputype definitions, PMEVN_SWITCH, read_sysreg/write_sysreg, and cpuid data.

/* Register access macros from the C header are represented as forwarding
 * macros so the supplied low-level access implementation remains external. */
macro_rules! PMCCNTR { () => { __ACCESS_CP15_64!(0, c9) }; }
macro_rules! PMCR { () => { __ACCESS_CP15!(c9, 0, c12, 0) }; }
macro_rules! PMCNTENSET { () => { __ACCESS_CP15!(c9, 0, c12, 1) }; }
macro_rules! PMCNTENCLR { () => { __ACCESS_CP15!(c9, 0, c12, 2) }; }
macro_rules! PMOVSR { () => { __ACCESS_CP15!(c9, 0, c12, 3) }; }
macro_rules! PMSELR { () => { __ACCESS_CP15!(c9, 0, c12, 5) }; }
macro_rules! PMCEID0 { () => { __ACCESS_CP15!(c9, 0, c12, 6) }; }
macro_rules! PMCEID1 { () => { __ACCESS_CP15!(c9, 0, c12, 7) }; }
macro_rules! PMXEVTYPER { () => { __ACCESS_CP15!(c9, 0, c13, 1) }; }
macro_rules! PMXEVCNTR { () => { __ACCESS_CP15!(c9, 0, c13, 2) }; }
macro_rules! PMUSERENR { () => { __ACCESS_CP15!(c9, 0, c14, 0) }; }
macro_rules! PMINTENSET { () => { __ACCESS_CP15!(c9, 0, c14, 1) }; }
macro_rules! PMINTENCLR { () => { __ACCESS_CP15!(c9, 0, c14, 2) }; }
macro_rules! PMCEID2 { () => { __ACCESS_CP15!(c9, 0, c14, 4) }; }
macro_rules! PMCEID3 { () => { __ACCESS_CP15!(c9, 0, c14, 5) }; }
macro_rules! PMMIR { () => { __ACCESS_CP15!(c9, 0, c14, 6) }; }
macro_rules! PMCCFILTR { () => { __ACCESS_CP15!(c14, 0, c15, 7) }; }

macro_rules! PMEVCNTR { ($n:expr) => { __ACCESS_CP15!(c14, 0, c8 + ($n / 8), $n % 8) }; }
macro_rules! PMEVTYPER { ($n:expr) => { __ACCESS_CP15!(c14, 0, c12 + ($n / 8), $n % 8) }; }

pub unsafe fn read_pmevcntrn(n: i32) -> usize {
    match n { 0..=30 => read_sysreg(PMEVCNTR!(n)), _ => 0 }
}

pub unsafe fn write_pmevcntrn(n: i32, val: usize) {
    if (0..=30).contains(&n) { write_sysreg(val, PMEVCNTR!(n)); }
}

pub unsafe fn write_pmevtypern(n: i32, val: usize) {
    if (0..=30).contains(&n) { write_sysreg(val, PMEVTYPER!(n)); }
}

pub unsafe fn read_pmmir() -> usize { read_sysreg(PMMIR!()) }

pub unsafe fn read_pmuver() -> u32 {
    /* PMUVers is not a signed field */
    let dfr0: u32 = read_cpuid_ext(CPUID_EXT_DFR0);
    (dfr0 >> 24) & 0xf
}

pub fn pmuv3_has_icntr() -> bool { /* FEAT_PMUv3_ICNTR not accessible for 32-bit */ false }
pub unsafe fn write_pmcr(val: u32) { write_sysreg(val, PMCR!()); }
pub unsafe fn read_pmcr() -> u32 { read_sysreg(PMCR!()) }
pub unsafe fn write_pmselr(val: u32) { write_sysreg(val, PMSELR!()); }
pub unsafe fn write_pmccntr(val: u64) { write_sysreg(val, PMCCNTR!()); }
pub unsafe fn read_pmccntr() -> u64 { read_sysreg(PMCCNTR!()) }
pub unsafe fn write_pmicntr(_val: u64) {}
pub unsafe fn read_pmicntr() -> u64 { 0 }
pub unsafe fn write_pmcntenset(val: u32) { write_sysreg(val, PMCNTENSET!()); }
pub unsafe fn write_pmcntenclr(val: u32) { write_sysreg(val, PMCNTENCLR!()); }
pub unsafe fn write_pmintenset(val: u32) { write_sysreg(val, PMINTENSET!()); }
pub unsafe fn write_pmintenclr(val: u32) { write_sysreg(val, PMINTENCLR!()); }
pub unsafe fn write_pmccfiltr(val: u32) { write_sysreg(val, PMCCFILTR!()); }
pub unsafe fn write_pmicfiltr(_val: u64) {}
pub unsafe fn read_pmicfiltr() -> u64 { 0 }
pub unsafe fn write_pmovsclr(val: u32) { write_sysreg(val, PMOVSR!()); }
pub unsafe fn read_pmovsclr() -> u32 { read_sysreg(PMOVSR!()) }
pub unsafe fn write_pmuserenr(val: u32) { write_sysreg(val, PMUSERENR!()); }
pub unsafe fn write_pmuacr(_val: u64) {}

pub unsafe fn kvm_set_pmu_events(_set: u32, _attr: *mut perf_event_attr) {}
pub unsafe fn kvm_clr_pmu_events(_clr: u32) {}
pub unsafe fn kvm_pmu_counter_deferred(_attr: *mut perf_event_attr) -> bool { false }
pub unsafe fn kvm_set_pmuserenr(_val: u64) -> bool { false }
pub unsafe fn kvm_vcpu_pmu_resync_el0() {}

/* PMU Version in DFR Register */
pub const ARMV8_PMU_DFR_VER_NI: i32 = 0;
pub const ARMV8_PMU_DFR_VER_V3P1: i32 = 0x4;
pub const ARMV8_PMU_DFR_VER_V3P4: i32 = 0x5;
pub const ARMV8_PMU_DFR_VER_V3P5: i32 = 0x6;
pub const ARMV8_PMU_DFR_VER_V3P9: i32 = 0x9;
pub const ARMV8_PMU_DFR_VER_IMP_DEF: i32 = 0xF;

pub fn pmuv3_implemented(pmuver: i32) -> bool { !(pmuver == ARMV8_PMU_DFR_VER_IMP_DEF || pmuver == ARMV8_PMU_DFR_VER_NI) }
pub fn is_pmuv3p4(pmuver: i32) -> bool { pmuver >= ARMV8_PMU_DFR_VER_V3P4 }
pub fn is_pmuv3p5(pmuver: i32) -> bool { pmuver >= ARMV8_PMU_DFR_VER_V3P5 }
pub fn is_pmuv3p9(pmuver: i32) -> bool { pmuver >= ARMV8_PMU_DFR_VER_V3P9 }

pub unsafe fn read_pmceid0() -> u64 {
    let mut val: u64 = read_sysreg(PMCEID0!());
    if read_pmuver() >= ARMV8_PMU_DFR_VER_V3P1 { val |= (read_sysreg(PMCEID2!()) as u64) << 32; }
    val
}
pub unsafe fn read_pmceid1() -> u64 {
    let mut val: u64 = read_sysreg(PMCEID1!());
    if read_pmuver() >= ARMV8_PMU_DFR_VER_V3P1 { val |= (read_sysreg(PMCEID3!()) as u64) << 32; }
    val
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
