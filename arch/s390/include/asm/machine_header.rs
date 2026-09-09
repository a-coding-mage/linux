/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 2024
 */

// Translated from the C header. The C include dependencies are supplied by
// the surrounding translation unit.

pub const MFEATURE_LOWCORE: u32 = 0;
pub const MFEATURE_PCI_MIO: u32 = 1;
pub const MFEATURE_SCC: u32 = 2;
pub const MFEATURE_TLB_GUEST: u32 = 3;
pub const MFEATURE_TX: u32 = 4;
pub const MFEATURE_ESOP: u32 = 5;
pub const MFEATURE_DIAG9C: u32 = 6;
pub const MFEATURE_VM: u32 = 7;
pub const MFEATURE_KVM: u32 = 8;
pub const MFEATURE_LPAR: u32 = 9;
pub const MFEATURE_DIAG288: u32 = 10;

// extern unsigned long machine_features[1];
extern "C" {
    pub static mut machine_features: [usize; 1];
}

pub const MAX_MFEATURE_BIT: usize = core::mem::size_of::<[usize; 1]>() * 8;

pub unsafe fn __set_machine_feature(nr: u32, mfeatures: *mut usize) {
    if nr as usize >= MAX_MFEATURE_BIT {
        return;
    }
    let word = (nr as usize) / usize::BITS as usize;
    let bit = (nr as usize) % usize::BITS as usize;
    *mfeatures.add(word) |= 1usize << bit;
}

pub unsafe fn set_machine_feature(nr: u32) {
    __set_machine_feature(nr, machine_features.as_mut_ptr());
}

pub unsafe fn __clear_machine_feature(nr: u32, mfeatures: *mut usize) {
    if nr as usize >= MAX_MFEATURE_BIT {
        return;
    }
    let word = (nr as usize) / usize::BITS as usize;
    let bit = (nr as usize) % usize::BITS as usize;
    *mfeatures.add(word) &= !(1usize << bit);
}

pub unsafe fn clear_machine_feature(nr: u32) {
    __clear_machine_feature(nr, machine_features.as_mut_ptr());
}

pub unsafe fn __test_machine_feature(nr: u32, mfeatures: *mut usize) -> bool {
    if nr as usize >= MAX_MFEATURE_BIT {
        return false;
    }
    let word = (nr as usize) / usize::BITS as usize;
    let bit = (nr as usize) % usize::BITS as usize;
    (*mfeatures.add(word) & (1usize << bit)) != 0
}

pub unsafe fn test_machine_feature(nr: u32) -> bool {
    __test_machine_feature(nr, machine_features.as_mut_ptr())
}

// The C implementation uses s390 alternative inline assembly to test a
// compile-time feature. Preserve the operation's result through the backing
// feature bitmap; the alternative-instruction selection is build-specific.
#[inline(always)]
pub unsafe fn __test_machine_feature_constant(nr: u32) -> bool {
    test_machine_feature(nr)
}

#[inline(always)]
pub unsafe fn machine_has_relocated_lowcore() -> bool {
    __test_machine_feature_constant(MFEATURE_LOWCORE)
}

#[inline(always)]
pub unsafe fn machine_has_scc() -> bool {
    __test_machine_feature_constant(MFEATURE_SCC)
}

#[inline(always)]
pub unsafe fn machine_has_tlb_guest() -> bool {
    __test_machine_feature_constant(MFEATURE_TLB_GUEST)
}

#[inline(always)]
pub unsafe fn machine_has_tx() -> bool {
    __test_machine_feature_constant(MFEATURE_TX)
}

#[inline(always)]
pub unsafe fn machine_has_esop() -> bool {
    __test_machine_feature_constant(MFEATURE_ESOP)
}

#[inline(always)]
pub unsafe fn machine_has_diag9c() -> bool {
    __test_machine_feature_constant(MFEATURE_DIAG9C)
}

#[inline(always)]
pub unsafe fn machine_has_vm() -> bool {
    __test_machine_feature_constant(MFEATURE_VM)
}

#[inline(always)]
pub unsafe fn machine_has_kvm() -> bool {
    __test_machine_feature_constant(MFEATURE_KVM)
}

#[inline(always)]
pub unsafe fn machine_has_lpar() -> bool {
    __test_machine_feature_constant(MFEATURE_LPAR)
}

pub use machine_has_vm as machine_is_vm;
pub use machine_has_kvm as machine_is_kvm;
pub use machine_has_lpar as machine_is_lpar;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
