/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2012 ARM Ltd.
 */

// Dependency intent: symbols and macros supplied by asm/kvm_host.h,
// asm/cpufeature.h, and asm/sysreg.h remain external dependencies.

pub unsafe fn read_pmevcntrn(n: i32) -> u64 {
    PMEVN_SWITCH!(n, RETURN_READ_PMEVCNTRN);
    0
}

pub unsafe fn write_pmevcntrn(n: i32, val: u64) {
    PMEVN_SWITCH!(n, WRITE_PMEVCNTRN);
}

pub unsafe fn write_pmevtypern(n: i32, val: u64) {
    PMEVN_SWITCH!(n, WRITE_PMEVTYPERN);
}

pub unsafe fn read_pmevtypern(n: i32) -> u64 {
    PMEVN_SWITCH!(n, RETURN_READ_PMEVTYPERN);
    0
}

pub unsafe fn read_pmmir() -> u64 {
    read_cpuid!(PMMIR_EL1)
}

pub unsafe fn read_pmuver() -> u32 {
    let dfr0: u64 = read_sysreg!(id_aa64dfr0_el1);

    cpuid_feature_extract_unsigned_field!(dfr0, ID_AA64DFR0_EL1_PMUVer_SHIFT)
}

pub unsafe fn pmuv3_has_icntr() -> bool {
    let dfr1: u64 = read_sysreg!(id_aa64dfr1_el1);

    cpuid_feature_extract_unsigned_field!(dfr1, ID_AA64DFR1_EL1_PMICNTR_SHIFT) != 0
}

pub unsafe fn write_pmcr(val: u64) {
    write_sysreg!(val, pmcr_el0);
}

pub unsafe fn read_pmcr() -> u64 {
    read_sysreg!(pmcr_el0)
}

pub unsafe fn write_pmselr(val: u32) {
    write_sysreg!(val, pmselr_el0);
}

pub unsafe fn write_pmccntr(val: u64) {
    write_sysreg!(val, pmccntr_el0);
}

pub unsafe fn read_pmccntr() -> u64 {
    read_sysreg!(pmccntr_el0)
}

pub unsafe fn write_pmicntr(val: u64) {
    write_sysreg_s!(val, SYS_PMICNTR_EL0);
}

pub unsafe fn read_pmicntr() -> u64 {
    read_sysreg_s!(SYS_PMICNTR_EL0)
}

pub unsafe fn write_pmcntenset(val: u64) {
    write_sysreg!(val, pmcntenset_el0);
}

pub unsafe fn write_pmcntenclr(val: u64) {
    write_sysreg!(val, pmcntenclr_el0);
}

pub unsafe fn write_pmintenset(val: u64) {
    write_sysreg!(val, pmintenset_el1);
}

pub unsafe fn write_pmintenclr(val: u64) {
    write_sysreg!(val, pmintenclr_el1);
}

pub unsafe fn write_pmccfiltr(val: u64) {
    write_sysreg!(val, pmccfiltr_el0);
}

pub unsafe fn read_pmccfiltr() -> u64 {
    read_sysreg!(pmccfiltr_el0)
}

pub unsafe fn write_pmicfiltr(val: u64) {
    write_sysreg_s!(val, SYS_PMICFILTR_EL0);
}

pub unsafe fn read_pmicfiltr() -> u64 {
    read_sysreg_s!(SYS_PMICFILTR_EL0)
}

pub unsafe fn write_pmovsclr(val: u64) {
    write_sysreg!(val, pmovsclr_el0);
}

pub unsafe fn read_pmovsclr() -> u64 {
    read_sysreg!(pmovsclr_el0)
}

pub unsafe fn write_pmuserenr(val: u32) {
    write_sysreg!(val, pmuserenr_el0);
}

pub unsafe fn write_pmuacr(val: u64) {
    write_sysreg_s!(val, SYS_PMUACR_EL1);
}

pub unsafe fn read_pmceid0() -> u64 {
    read_sysreg!(pmceid0_el0)
}

pub unsafe fn read_pmceid1() -> u64 {
    read_sysreg!(pmceid1_el0)
}

pub fn pmuv3_implemented(pmuver: i32) -> bool {
    !(pmuver == ID_AA64DFR0_EL1_PMUVer_IMP_DEF || pmuver == ID_AA64DFR0_EL1_PMUVer_NI)
}

pub fn is_pmuv3p4(pmuver: i32) -> bool {
    pmuver >= ID_AA64DFR0_EL1_PMUVer_V3P4
}

pub fn is_pmuv3p5(pmuver: i32) -> bool {
    pmuver >= ID_AA64DFR0_EL1_PMUVer_V3P5
}

pub fn is_pmuv3p9(pmuver: i32) -> bool {
    pmuver >= ID_AA64DFR0_EL1_PMUVer_V3P9
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
