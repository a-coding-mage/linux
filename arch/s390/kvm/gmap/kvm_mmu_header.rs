/* SPDX-License-Identifier: GPL-2.0 */

// Translated from arch/s390/kvm/gmap/kvm_mmu.h.
// The declarations below are supplied by the Linux KVM dependencies.

#[repr(C)]
pub struct kvm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_dirty_log {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kvm_memory_slot {
    _private: [u8; 0],
}

#[repr(C)]
pub enum kvm_mr_change {
    _Unknown,
}

extern "C" {
    pub fn s390_kvm_mmu_get_dirty_log(
        kvm: *mut kvm,
        log: *mut kvm_dirty_log,
    ) -> ::core::ffi::c_int;

    pub fn s390_kvm_mmu_prepare_memory_region(
        kvm: *mut kvm,
        old: *const kvm_memory_slot,
        new: *mut kvm_memory_slot,
        change: kvm_mr_change,
    ) -> ::core::ffi::c_int;

    pub fn s390_kvm_mmu_commit_memory_region(
        kvm: *mut kvm,
        old: *mut kvm_memory_slot,
        new: *const kvm_memory_slot,
        change: kvm_mr_change,
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
