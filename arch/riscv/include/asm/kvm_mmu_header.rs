/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2025 Ventana Micro Systems Inc.
 */

// Dependency supplied by the surrounding kernel translation:
// <asm/kvm_gstage.h>

/// int kvm_riscv_mmu_ioremap(struct kvm *kvm, gpa_t gpa, phys_addr_t hpa,
///                           unsigned long size, bool writable, bool in_atomic);
///
/// External C declarations are preserved here; the referenced types are
/// supplied by the surrounding translated kernel sources.
pub unsafe extern "C" {
    pub fn kvm_riscv_mmu_ioremap(
        kvm: *mut crate::kvm,
        gpa: crate::gpa_t,
        hpa: crate::phys_addr_t,
        size: core::ffi::c_ulong,
        writable: bool,
        in_atomic: bool,
    ) -> core::ffi::c_int;

    /// void kvm_riscv_mmu_iounmap(struct kvm *kvm, gpa_t gpa,
    ///                             unsigned long size);
    pub fn kvm_riscv_mmu_iounmap(
        kvm: *mut crate::kvm,
        gpa: crate::gpa_t,
        size: core::ffi::c_ulong,
    );

    /// int kvm_riscv_mmu_map(struct kvm_vcpu *vcpu,
    ///                       struct kvm_memory_slot *memslot, gpa_t gpa,
    ///                       unsigned long hva, bool is_write,
    ///                       struct kvm_gstage_mapping *out_map);
    pub fn kvm_riscv_mmu_map(
        vcpu: *mut crate::kvm_vcpu,
        memslot: *mut crate::kvm_memory_slot,
        gpa: crate::gpa_t,
        hva: core::ffi::c_ulong,
        is_write: bool,
        out_map: *mut crate::kvm_gstage_mapping,
    ) -> core::ffi::c_int;

    /// int kvm_riscv_mmu_alloc_pgd(struct kvm *kvm);
    pub fn kvm_riscv_mmu_alloc_pgd(kvm: *mut crate::kvm) -> core::ffi::c_int;

    /// void kvm_riscv_mmu_free_pgd(struct kvm *kvm);
    pub fn kvm_riscv_mmu_free_pgd(kvm: *mut crate::kvm);

    /// void kvm_riscv_mmu_update_hgatp(struct kvm_vcpu *vcpu);
    pub fn kvm_riscv_mmu_update_hgatp(vcpu: *mut crate::kvm_vcpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
