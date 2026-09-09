/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub union VmxExitReason {
    // C bit-fields are represented by their containing value; accessors for the
    // individual fields belong to the dependent translation.
    pub full: u32,
}

#[repr(C)]
pub struct VcpuVt {
    /* Posted interrupt descriptor */
    pub pi_desc: pi_desc,

    /* Used if this vCPU is waiting for PI notification wakeup. */
    pub pi_wakeup_list: list_head,

    pub exit_reason: VmxExitReason,

    pub exit_qualification: libc::c_ulong,
    pub exit_intr_info: u32,

    /*
     * If true, guest state has been loaded into hardware, and host state
     * saved into vcpu_{vt,vmx,tdx}.  If false, host state is loaded into
     * hardware.
     */
    pub guest_state_loaded: bool,
    pub emulation_required: bool,

    #[cfg(target_pointer_width = "64")]
    pub msr_host_kernel_gs_base: u64,
}

#[cfg(CONFIG_KVM_INTEL_TDX)]
#[inline(always)]
pub unsafe fn is_td(kvm: *mut kvm) -> bool {
    (*kvm).arch.vm_type == KVM_X86_TDX_VM
}

#[cfg(CONFIG_KVM_INTEL_TDX)]
#[inline(always)]
pub unsafe fn is_td_vcpu(vcpu: *mut kvm_vcpu) -> bool {
    is_td((*vcpu).kvm)
}

#[cfg(not(CONFIG_KVM_INTEL_TDX))]
#[inline(always)]
pub unsafe fn is_td(_kvm: *mut kvm) -> bool { false }

#[cfg(not(CONFIG_KVM_INTEL_TDX))]
#[inline(always)]
pub unsafe fn is_td_vcpu(_vcpu: *mut kvm_vcpu) -> bool { false }

#[inline]
pub unsafe fn vt_is_tdx_private_gpa(kvm: *mut kvm, gpa: gpa_t) -> bool {
    /* For TDX the direct mask is the shared mask. */
    !kvm_is_addr_direct(kvm, gpa)
}

#[inline]
pub unsafe fn __vmx_handle_ept_violation(
    vcpu: *mut kvm_vcpu,
    gpa: gpa_t,
    exit_qualification: libc::c_ulong,
) -> libc::c_int {
    let mut error_code: u64;

    /* Is it a write fault? */
    error_code = if exit_qualification & EPT_VIOLATION_ACC_WRITE != 0 {
        PFERR_WRITE_MASK
    } else { 0 };
    /* Is it a fetch fault? */
    error_code |= if exit_qualification & EPT_VIOLATION_ACC_INSTR != 0 {
        PFERR_FETCH_MASK
    } else { 0 };
    /* ept page table entry is present?  */
    error_code |= if exit_qualification
        & (EPT_VIOLATION_PROT_MASK & !EPT_VIOLATION_PROT_USER_EXEC) != 0
    {
        PFERR_PRESENT_MASK
    } else { 0 };

    if mmu_has_mbec((*vcpu).arch.mmu) {
        error_code |= if exit_qualification & EPT_VIOLATION_PROT_USER_EXEC != 0 {
            PFERR_PRESENT_MASK
        } else { 0 };
    }

    if exit_qualification & EPT_VIOLATION_GVA_IS_VALID != 0 {
        if exit_qualification & EPT_VIOLATION_GVA_TRANSLATED != 0 {
            error_code |= PFERR_GUEST_FINAL_MASK;
            if exit_qualification & EPT_VIOLATION_GVA_USER != 0 {
                error_code |= PFERR_USER_MASK;
            }
        } else {
            error_code |= PFERR_GUEST_PAGE_MASK;
        }
    }

    if vt_is_tdx_private_gpa((*vcpu).kvm, gpa) {
        error_code |= PFERR_PRIVATE_ACCESS;
    }

    kvm_mmu_page_fault(vcpu, gpa, error_code, core::ptr::null_mut(), 0)
}

#[inline]
pub unsafe fn kvm_vcpu_trigger_posted_interrupt(vcpu: *mut kvm_vcpu, pi_vec: libc::c_int) {
    #[cfg(CONFIG_SMP)]
    if (*vcpu).mode == IN_GUEST_MODE {
        if vcpu != kvm_get_running_vcpu() {
            __apic_send_IPI_mask(get_cpu_mask((*vcpu).cpu), pi_vec);
        }
        return;
    }

    kvm_vcpu_wake_up(vcpu);
}

/*
 * Post an interrupt to a vCPU's PIR and trigger the vCPU to process the
 * interrupt if necessary.
 */
#[inline]
pub unsafe fn __vmx_deliver_posted_interrupt(
    vcpu: *mut kvm_vcpu,
    pi_desc_ptr: *mut pi_desc,
    vector: libc::c_int,
) {
    if pi_test_and_set_pir(vector, pi_desc_ptr) != 0 {
        return;
    }

    /* If a previous notification has sent the IPI, nothing to do.  */
    if pi_test_and_set_on(pi_desc_ptr) != 0 {
        return;
    }

    /* The implied barrier in pi_test_and_set_on() pairs with the barrier after
     * setting vcpu->mode in vcpu_enter_guest(). */
    kvm_vcpu_trigger_posted_interrupt(vcpu, POSTED_INTR_VECTOR);
}

extern "C" {
    pub fn vmx_handle_nmi(vcpu: *mut kvm_vcpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
