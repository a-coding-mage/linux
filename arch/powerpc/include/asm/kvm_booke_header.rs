/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright SUSE Linux Products GmbH 2010
 *
 * Authors: Alexander Graf <agraf@suse.de>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub const KVMPPC_NR_LPIDS: usize = 64;

pub const KVMPPC_INST_EHPRIV: u32 = 0x7c00021c;
pub const EHPRIV_OC_SHIFT: u32 = 11;
/* "ehpriv 1" : ehpriv with OC = 1 is used for debug emulation */
pub const EHPRIV_OC_DEBUG: u32 = 1;

pub unsafe fn kvmppc_set_gpr(vcpu: *mut kvm_vcpu, num: i32, val: usize) {
    (*vcpu).arch.regs.gpr[num as usize] = val;
}

pub unsafe fn kvmppc_get_gpr(vcpu: *mut kvm_vcpu, num: i32) -> usize {
    (*vcpu).arch.regs.gpr[num as usize]
}

pub unsafe fn kvmppc_set_cr(vcpu: *mut kvm_vcpu, val: u32) {
    (*vcpu).arch.regs.ccr = val;
}

pub unsafe fn kvmppc_get_cr(vcpu: *mut kvm_vcpu) -> u32 {
    (*vcpu).arch.regs.ccr
}

pub unsafe fn kvmppc_set_xer(vcpu: *mut kvm_vcpu, val: usize) {
    (*vcpu).arch.regs.xer = val;
}

pub unsafe fn kvmppc_get_xer(vcpu: *mut kvm_vcpu) -> usize {
    (*vcpu).arch.regs.xer
}

pub unsafe fn kvmppc_need_byteswap(_vcpu: *mut kvm_vcpu) -> bool {
    /* XXX Would need to check TLB entry */
    false
}

pub unsafe fn kvmppc_set_ctr(vcpu: *mut kvm_vcpu, val: usize) {
    (*vcpu).arch.regs.ctr = val;
}

pub unsafe fn kvmppc_get_ctr(vcpu: *mut kvm_vcpu) -> usize {
    (*vcpu).arch.regs.ctr
}

pub unsafe fn kvmppc_set_lr(vcpu: *mut kvm_vcpu, val: usize) {
    (*vcpu).arch.regs.link = val;
}

pub unsafe fn kvmppc_get_lr(vcpu: *mut kvm_vcpu) -> usize {
    (*vcpu).arch.regs.link
}

pub unsafe fn kvmppc_set_pc(vcpu: *mut kvm_vcpu, val: usize) {
    (*vcpu).arch.regs.nip = val;
}

pub unsafe fn kvmppc_get_pc(vcpu: *mut kvm_vcpu) -> usize {
    (*vcpu).arch.regs.nip
}

pub unsafe fn kvmppc_set_fpr(vcpu: *mut kvm_vcpu, i: i32, val: u64) {
    (*vcpu).arch.fp.fpr[i as usize][TS_FPROFFSET] = val;
}

pub unsafe fn kvmppc_get_fpr(vcpu: *mut kvm_vcpu, i: i32) -> u64 {
    (*vcpu).arch.fp.fpr[i as usize][TS_FPROFFSET]
}

// Preserved from #ifdef CONFIG_BOOKE; the surrounding build selects whether
// this declaration is enabled.
#[cfg(feature = "CONFIG_BOOKE")]
pub unsafe fn kvmppc_get_fault_dar(vcpu: *mut kvm_vcpu) -> usize {
    (*vcpu).arch.fault_dear
}

pub unsafe fn kvmppc_supports_magic_page(_vcpu: *mut kvm_vcpu) -> bool {
    /* Magic page is only supported on e500v2 */
    // CONFIG_KVM_E500V2 is a build-time condition from the original header.
    #[cfg(feature = "CONFIG_KVM_E500V2")]
    {
        true
    }
    #[cfg(not(feature = "CONFIG_KVM_E500V2"))]
    {
        false
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
