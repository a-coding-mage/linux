/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * KVM L1 hypervisor optimizations on Hyper-V.
 */

/* The following declarations are enabled when CONFIG_HYPERV is enabled. */
#[cfg(feature = "CONFIG_HYPERV")]
extern "C" {
    pub fn hv_flush_remote_tlbs_range(
        kvm: *mut kvm,
        gfn: gfn_t,
        nr_pages: gfn_t,
    ) -> ::core::ffi::c_int;
    pub fn hv_flush_remote_tlbs(kvm: *mut kvm) -> ::core::ffi::c_int;
    pub fn hv_track_root_tdp(vcpu: *mut kvm_vcpu, root_tdp: hpa_t);
}

#[cfg(feature = "CONFIG_HYPERV")]
#[inline]
pub unsafe fn hv_get_partition_assist_page(vcpu: *mut kvm_vcpu) -> hpa_t {
    /*
     * Partition assist page is something which Hyper-V running in L0
     * requires from KVM running in L1 before direct TLB flush for L2
     * guests can be enabled. KVM doesn't currently use the page but to
     * comply with TLFS it still needs to be allocated. For now, this
     * is a single page shared among all vCPUs.
     */
    let p_hv_pa_pg: *mut *mut hv_partition_assist_pg =
        &mut (*(*vcpu).kvm).arch.hv_pa_pg;

    if (*p_hv_pa_pg).is_null() {
        *p_hv_pa_pg = kzalloc(PAGE_SIZE, GFP_KERNEL_ACCOUNT);
    }

    if (*p_hv_pa_pg).is_null() {
        return INVALID_PAGE;
    }

    __pa(*p_hv_pa_pg)
}

/* !CONFIG_HYPERV */
#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline]
pub unsafe fn hv_flush_remote_tlbs(kvm: *mut kvm) -> ::core::ffi::c_int {
    -EOPNOTSUPP
}

#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline]
pub unsafe fn hv_track_root_tdp(vcpu: *mut kvm_vcpu, root_tdp: hpa_t) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
