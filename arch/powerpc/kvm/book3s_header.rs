/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright IBM Corporation, 2013
 * Author Aneesh Kumar K.V <aneesh.kumar@linux.vnet.ibm.com>
 */

// Declarations supplied by the surrounding kernel translation.
pub enum kvm {}
pub enum kvm_memory_slot {}
pub enum kvm_gfn_range {}
pub enum kvm_vcpu {}

extern "C" {
    pub fn kvmppc_core_flush_memslot_hv(
        kvm: *mut kvm,
        memslot: *mut kvm_memory_slot,
    );
    pub fn kvm_unmap_gfn_range_hv(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;
    pub fn kvm_age_gfn_hv(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;
    pub fn kvm_test_age_gfn_hv(kvm: *mut kvm, range: *mut kvm_gfn_range) -> bool;

    pub fn kvmppc_mmu_init_pr(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvmppc_mmu_destroy_pr(vcpu: *mut kvm_vcpu);
    pub fn kvmppc_core_emulate_op_pr(
        vcpu: *mut kvm_vcpu,
        inst: u32,
        advance: *mut i32,
    ) -> i32;
    pub fn kvmppc_core_emulate_mtspr_pr(
        vcpu: *mut kvm_vcpu,
        sprn: i32,
        spr_val: u64,
    ) -> i32;
    pub fn kvmppc_core_emulate_mfspr_pr(
        vcpu: *mut kvm_vcpu,
        sprn: i32,
        spr_val: *mut u64,
    ) -> i32;
    pub fn kvmppc_book3s_init_pr() -> i32;
    pub fn kvmppc_book3s_exit_pr();
    pub fn kvmppc_handle_exit_pr(vcpu: *mut kvm_vcpu, exit_nr: u32) -> i32;

    // CONFIG_PPC_TRANSACTIONAL_MEM
    #[cfg(feature = "CONFIG_PPC_TRANSACTIONAL_MEM")]
    pub fn kvmppc_emulate_tabort(vcpu: *mut kvm_vcpu, ra_val: i32);

    pub fn kvmppc_set_msr_hv(vcpu: *mut kvm_vcpu, msr: u64);
    pub fn kvmppc_inject_interrupt_hv(vcpu: *mut kvm_vcpu, vec: i32, srr1_flags: u64);
}

// When CONFIG_PPC_TRANSACTIONAL_MEM is disabled, the C header provides an
// empty inline implementation instead of the external declaration above.
#[cfg(not(feature = "CONFIG_PPC_TRANSACTIONAL_MEM"))]
#[inline]
pub unsafe fn kvmppc_emulate_tabort(_vcpu: *mut kvm_vcpu, _ra_val: i32) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
