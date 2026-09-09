/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright IBM Corp. 2008
 *
 * Authors: Christian Ehrhardt <ehrhardt@linux.vnet.ibm.com>
 */

/* Dependency supplied by the surrounding kernel translation. */

#[cfg(CONFIG_KVM_EXIT_TIMING)]
extern "C" {
    pub fn kvmppc_init_timing_stats(vcpu: *mut crate::kvm_vcpu);
    pub fn kvmppc_update_timing_stats(vcpu: *mut crate::kvm_vcpu);
    pub fn kvmppc_create_vcpu_debugfs_e500(
        vcpu: *mut crate::kvm_vcpu,
        debugfs_dentry: *mut crate::dentry,
    ) -> ::std::os::raw::c_int;
}

#[cfg(CONFIG_KVM_EXIT_TIMING)]
#[inline]
pub unsafe fn kvmppc_set_exit_type(vcpu: *mut crate::kvm_vcpu, type_: ::std::os::raw::c_int) {
    (*vcpu).arch.last_exit_type = type_;
}

/* If exit timing is not configured there is no need to build the C file. */
#[cfg(not(CONFIG_KVM_EXIT_TIMING))]
#[inline]
pub unsafe fn kvmppc_init_timing_stats(_vcpu: *mut crate::kvm_vcpu) {}

#[cfg(not(CONFIG_KVM_EXIT_TIMING))]
#[inline]
pub unsafe fn kvmppc_update_timing_stats(_vcpu: *mut crate::kvm_vcpu) {}

#[cfg(not(CONFIG_KVM_EXIT_TIMING))]
#[inline]
pub unsafe fn kvmppc_create_vcpu_debugfs_e500(
    _vcpu: *mut crate::kvm_vcpu,
    _debugfs_dentry: *mut crate::dentry,
) -> ::std::os::raw::c_int {
    0
}

#[cfg(not(CONFIG_KVM_EXIT_TIMING))]
#[inline]
pub unsafe fn kvmppc_set_exit_type(
    _vcpu: *mut crate::kvm_vcpu,
    _type_: ::std::os::raw::c_int,
) {
}

/* Account the exit in kvm_stats. */
#[inline]
pub unsafe fn kvmppc_account_exit_stat(
    vcpu: *mut crate::kvm_vcpu,
    type_: ::std::os::raw::c_int,
) {
    /* type has to be known at build time for optimization. */
    match type_ {
        crate::EXT_INTR_EXITS => (*vcpu).stat.ext_intr_exits += 1,
        crate::DEC_EXITS => (*vcpu).stat.dec_exits += 1,
        crate::EMULATED_INST_EXITS => (*vcpu).stat.emulated_inst_exits += 1,
        crate::DSI_EXITS => (*vcpu).stat.dsi_exits += 1,
        crate::ISI_EXITS => (*vcpu).stat.isi_exits += 1,
        crate::SYSCALL_EXITS => (*vcpu).stat.syscall_exits += 1,
        crate::DTLB_REAL_MISS_EXITS => (*vcpu).stat.dtlb_real_miss_exits += 1,
        crate::DTLB_VIRT_MISS_EXITS => (*vcpu).stat.dtlb_virt_miss_exits += 1,
        crate::MMIO_EXITS => (*vcpu).stat.mmio_exits += 1,
        crate::ITLB_REAL_MISS_EXITS => (*vcpu).stat.itlb_real_miss_exits += 1,
        crate::ITLB_VIRT_MISS_EXITS => (*vcpu).stat.itlb_virt_miss_exits += 1,
        crate::SIGNAL_EXITS => (*vcpu).stat.signal_exits += 1,
        crate::DBELL_EXITS => (*vcpu).stat.dbell_exits += 1,
        crate::GDBELL_EXITS => (*vcpu).stat.gdbell_exits += 1,
        _ => {}
    }
}

/* Wrapper to set exit time and account for it in kvm_stats. */
#[inline]
pub unsafe fn kvmppc_account_exit(
    vcpu: *mut crate::kvm_vcpu,
    type_: ::std::os::raw::c_int,
) {
    kvmppc_set_exit_type(vcpu, type_);
    kvmppc_account_exit_stat(vcpu, type_);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
