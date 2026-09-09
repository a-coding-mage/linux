// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2023 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct kvm_vcpu {
    _private: [u8; 0],
}

extern "C" {
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn invtlb_all(op: usize, info: usize, address: usize);
    fn invtlb(op: usize, gid: usize, address: usize);
    fn read_csr_gstat() -> usize;
    fn lockdep_assert_irqs_disabled();
}

// Constants supplied by the corresponding architecture headers.
extern "C" {
    static INVTLB_ALLGID: usize;
    static INVTLB_GID_ADDR: usize;
    static CSR_GSTAT_GID: usize;
    static PAGE_MASK: usize;
}

/*
 * kvm_flush_tlb_all() - Flush all root TLB entries for guests.
 *
 * Invalidate all entries including GVA-->GPA and GPA-->HPA mappings.
 */
pub unsafe fn kvm_flush_tlb_all() {
    let mut flags: usize = 0;

    local_irq_save(&mut flags as *mut usize);
    invtlb_all(INVTLB_ALLGID, 0, 0);
    local_irq_restore(flags);
}

pub unsafe fn kvm_flush_tlb_gpa(vcpu: *mut kvm_vcpu, mut gpa: usize) {
    let _ = vcpu;
    lockdep_assert_irqs_disabled();
    gpa &= PAGE_MASK << 1;
    invtlb(INVTLB_GID_ADDR, read_csr_gstat() & CSR_GSTAT_GID, gpa);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
