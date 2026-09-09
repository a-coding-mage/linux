/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Module interface for CPU features
 *
 * Copyright IBM Corp. 2015, 2022
 * Author(s): Hendrik Brueckner <brueckner@linux.vnet.ibm.com>
 */

// #include <asm/facility.h>

#[repr(u32)]
pub enum CpuFeature {
    S390CpuFeatureMsa,
    S390CpuFeatureVxrs,
    S390CpuFeatureUv,
    S390CpuFeatureD288,
    MaxCpuFeatures,
}

#[inline]
pub const fn cpu_feature(feature: u32) -> u32 {
    feature
}

extern "C" {
    pub fn cpu_have_feature(nr: u32) -> i32;
    pub fn test_facility(nr: u32) -> bool;
}

#[inline]
pub unsafe fn cpu_has_bear() -> bool {
    test_facility(193)
}

#[inline]
pub unsafe fn cpu_has_edat1() -> bool {
    test_facility(8)
}

#[inline]
pub unsafe fn cpu_has_edat2() -> bool {
    test_facility(78)
}

#[inline]
pub unsafe fn cpu_has_gs() -> bool {
    test_facility(133)
}

#[inline]
pub unsafe fn cpu_has_nx() -> bool {
    test_facility(130)
}

#[inline]
pub unsafe fn cpu_has_rdp() -> bool {
    test_facility(194)
}

#[inline]
pub unsafe fn cpu_has_seq_insn() -> bool {
    test_facility(85)
}

#[inline]
pub unsafe fn cpu_has_tlb_lc() -> bool {
    test_facility(51)
}

#[inline]
pub unsafe fn cpu_has_topology() -> bool {
    test_facility(11)
}

#[inline]
pub unsafe fn cpu_has_vx() -> bool {
    test_facility(129)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
