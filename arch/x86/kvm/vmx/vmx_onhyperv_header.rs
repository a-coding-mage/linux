/* SPDX-License-Identifier: GPL-2.0-only */

// Dependencies supplied by the surrounding kernel translation:
// hyperv/hvhdk.h, asm/mshyperv.h, linux/jump_label.h, capabilities.h,
// hyperv_evmcs.h, and vmcs12.h.

// #define current_evmcs ((struct hv_enlightened_vmcs *)this_cpu_read(current_vmcs))
macro_rules! current_evmcs {
    () => {
        this_cpu_read(current_vmcs) as *mut hv_enlightened_vmcs
    };
}

// #if IS_ENABLED(CONFIG_HYPERV)

extern "C" {
    pub static mut __kvm_is_using_evmcs: static_key_false;
    pub fn static_branch_unlikely(key: *const static_key_false) -> bool;
    pub fn evmcs_field_offset(field: c_ulong, clean_field: *mut u16) -> c_int;
    pub fn hv_get_vp_assist_page(cpu: c_int) -> *mut hv_vp_assist_page;
    pub fn smp_processor_id() -> c_int;
    pub fn kvm_get_running_vcpu() -> *mut kvm_vcpu;
    pub fn KVM_BUG_ON(condition: bool, kvm: *mut kvm) -> bool;
    pub fn evmcs_sanitize_exec_ctrls(vmcs_conf: *mut vmcs_config);
}

#[inline(always)]
pub unsafe fn kvm_is_using_evmcs() -> bool {
    static_branch_unlikely(&raw const __kvm_is_using_evmcs)
}

#[inline(always)]
pub unsafe fn get_evmcs_offset(field: c_ulong, clean_field: *mut u16) -> c_int {
    let offset = evmcs_field_offset(field, clean_field);

    // WARN_ONCE(offset < 0, "accessing unsupported EVMCS field %lx\n", field);
    offset
}

#[inline(always)]
pub unsafe fn evmcs_write64(field: c_ulong, value: u64) {
    let mut clean_field: u16 = 0;
    let offset = get_evmcs_offset(field, &mut clean_field);

    if offset < 0 {
        return;
    }

    *((current_evmcs!() as *mut u8).offset(offset as isize) as *mut u64) = value;
    (*current_evmcs!()).hv_clean_fields &= !(clean_field as _);
}

#[inline(always)]
pub unsafe fn evmcs_write32(field: c_ulong, value: u32) {
    let mut clean_field: u16 = 0;
    let offset = get_evmcs_offset(field, &mut clean_field);

    if offset < 0 {
        return;
    }

    *((current_evmcs!() as *mut u8).offset(offset as isize) as *mut u32) = value;
    (*current_evmcs!()).hv_clean_fields &= !(clean_field as _);
}

#[inline(always)]
pub unsafe fn evmcs_write16(field: c_ulong, value: u16) {
    let mut clean_field: u16 = 0;
    let offset = get_evmcs_offset(field, &mut clean_field);

    if offset < 0 {
        return;
    }

    *((current_evmcs!() as *mut u8).offset(offset as isize) as *mut u16) = value;
    (*current_evmcs!()).hv_clean_fields &= !(clean_field as _);
}

#[inline(always)]
pub unsafe fn evmcs_read64(field: c_ulong) -> u64 {
    let offset = get_evmcs_offset(field, core::ptr::null_mut());
    if offset < 0 { return 0; }
    *((current_evmcs!() as *mut u8).offset(offset as isize) as *const u64)
}

#[inline(always)]
pub unsafe fn evmcs_read32(field: c_ulong) -> u32 {
    let offset = get_evmcs_offset(field, core::ptr::null_mut());
    if offset < 0 { return 0; }
    *((current_evmcs!() as *mut u8).offset(offset as isize) as *const u32)
}

#[inline(always)]
pub unsafe fn evmcs_read16(field: c_ulong) -> u16 {
    let offset = get_evmcs_offset(field, core::ptr::null_mut());
    if offset < 0 { return 0; }
    *((current_evmcs!() as *mut u8).offset(offset as isize) as *const u16)
}

#[inline]
pub unsafe fn evmcs_load(phys_addr: u64) {
    let vp_ap = hv_get_vp_assist_page(smp_processor_id());

    /*
     * When enabling eVMCS, KVM verifies that every CPU has a valid hv_vp_assist_page()
     * and aborts enabling the feature otherwise. CPU onlining path is also checked in
     * vmx_hardware_enable().
     */
    if KVM_BUG_ON(vp_ap.is_null(), (*kvm_get_running_vcpu()).kvm) {
        return;
    }

    if (*current_evmcs!()).hv_enlightenments_control.nested_flush_hypercall {
        (*vp_ap).nested_control.features.directhypercall = 1;
    }
    (*vp_ap).current_nested_vmcs = phys_addr;
    (*vp_ap).enlighten_vmentry = 1;
}

// #else /* !IS_ENABLED(CONFIG_HYPERV) */
// Configuration-disabled fallbacks.
#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline(always)]
pub fn kvm_is_using_evmcs() -> bool { false }
#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline(always)]
pub fn evmcs_write64(_field: c_ulong, _value: u64) {}
#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline(always)]
pub fn evmcs_write32(_field: c_ulong, _value: u32) {}
#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline(always)]
pub fn evmcs_write16(_field: c_ulong, _value: u16) {}
#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline(always)]
pub fn evmcs_read64(_field: c_ulong) -> u64 { 0 }
#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline(always)]
pub fn evmcs_read32(_field: c_ulong) -> u32 { 0 }
#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline(always)]
pub fn evmcs_read16(_field: c_ulong) -> u16 { 0 }
#[cfg(not(feature = "CONFIG_HYPERV"))]
#[inline]
pub fn evmcs_load(_phys_addr: u64) {}
// #endif /* IS_ENABLED(CONFIG_HYPERV) */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
