// SPDX-License-Identifier: GPL-2.0
// Translated from C. Kernel headers and configuration symbols are supplied by
// the surrounding translation unit.

// #ifdef CONFIG_X86_VMX_FEATURE_NAMES
#[repr(C)]
enum vmx_feature_leafs {
    MISC_FEATURES = 0,
    PRIMARY_CTLS,
    SECONDARY_CTLS,
    TERTIARY_CTLS_LOW,
    TERTIARY_CTLS_HIGH,
    NR_VMX_FEATURE_WORDS,
}

macro_rules! VMX_F {
    (EPT_EXECUTE_ONLY) => { 1u32 << (VMX_FEATURE_EPT_EXECUTE_ONLY & 0x1f) };
    (EPT_AD) => { 1u32 << (VMX_FEATURE_EPT_AD & 0x1f) };
    (EPT_1GB) => { 1u32 << (VMX_FEATURE_EPT_1GB & 0x1f) };
    (EPT_5LEVEL) => { 1u32 << (VMX_FEATURE_EPT_5LEVEL & 0x1f) };
    (VIRTUAL_TPR) => { 1u32 << (VMX_FEATURE_VIRTUAL_TPR & 0x1f) };
    (VIRT_APIC_ACCESSES) => { 1u32 << (VMX_FEATURE_VIRT_APIC_ACCESSES & 0x1f) };
    (FLEXPRIORITY) => { 1u32 << (VMX_FEATURE_FLEXPRIORITY & 0x1f) };
    (APIC_REGISTER_VIRT) => { 1u32 << (VMX_FEATURE_APIC_REGISTER_VIRT & 0x1f) };
    (VIRT_INTR_DELIVERY) => { 1u32 << (VMX_FEATURE_VIRT_INTR_DELIVERY & 0x1f) };
    (POSTED_INTR) => { 1u32 << (VMX_FEATURE_POSTED_INTR & 0x1f) };
    (APICV) => { 1u32 << (VMX_FEATURE_APICV & 0x1f) };
    (VIRTUAL_NMIS) => { 1u32 << (VMX_FEATURE_VIRTUAL_NMIS & 0x1f) };
    (EPT) => { 1u32 << (VMX_FEATURE_EPT & 0x1f) };
    (VPID) => { 1u32 << (VMX_FEATURE_VPID & 0x1f) };
}

unsafe fn init_vmx_capabilities(c: *mut cpuinfo_x86) {
    let mut val: msr = core::mem::zeroed();
    let mut supported: u32;
    let mut funcs: u32;
    let mut ept: u32;
    let mut vpid: u32;

    // BUILD_BUG_ON(NVMXINTS != NR_VMX_FEATURE_WORDS);

    /*
     * The high bits contain the allowed-1 settings, i.e. features that can
     * be turned on.  The low bits contain the allowed-0 settings, i.e.
     * features that can be turned off.  Ignore the allowed-0 settings,
     * if a feature can be turned on then it's supported.
     *
     * Use raw rdmsr() for primary processor controls and pin controls MSRs
     * as they exist on any CPU that supports VMX, i.e. we want the WARN if
     * the RDMSR faults.
     */
    rdmsrq(MSR_IA32_VMX_PROCBASED_CTLS, &mut val.q);
    supported = val.h;
    (*c).vmx_capability[PRIMARY_CTLS as usize] = supported;

    rdmsrq_safe(MSR_IA32_VMX_PROCBASED_CTLS2, &mut val.q);
    supported = val.h;
    (*c).vmx_capability[SECONDARY_CTLS as usize] = supported;

    /* All 64 bits of tertiary controls MSR are allowed-1 settings. */
    rdmsrq_safe(MSR_IA32_VMX_PROCBASED_CTLS3, &mut val.q);
    (*c).vmx_capability[TERTIARY_CTLS_LOW as usize] = val.l;
    (*c).vmx_capability[TERTIARY_CTLS_HIGH as usize] = val.h;

    rdmsrq(MSR_IA32_VMX_PINBASED_CTLS, &mut val.q);
    supported = val.h;
    rdmsrq_safe(MSR_IA32_VMX_VMFUNC, &mut val.q);
    funcs = val.h;

    /* Except for EPT+VPID, which enumerates support for both in a single
     * MSR, low for EPT, high for VPID. */
    rdmsrq_safe(MSR_IA32_VMX_EPT_VPID_CAP, &mut val.q);
    ept = val.l;
    vpid = val.h;

    /* Pin, EPT, VPID and VM-Func are merged into a single word. */
    WARN_ON_ONCE(supported >> 16);
    WARN_ON_ONCE(funcs >> 4);
    (*c).vmx_capability[MISC_FEATURES as usize] = (supported & 0xffff)
        | ((vpid & 0x1) << 16)
        | ((funcs & 0xf) << 28);

    /* EPT bits are full on scattered and must be manually handled. */
    if ept & VMX_EPT_EXECUTE_ONLY_BIT != 0 { (*c).vmx_capability[MISC_FEATURES as usize] |= VMX_F!(EPT_EXECUTE_ONLY); }
    if ept & VMX_EPT_AD_BIT != 0 { (*c).vmx_capability[MISC_FEATURES as usize] |= VMX_F!(EPT_AD); }
    if ept & VMX_EPT_1GB_PAGE_BIT != 0 { (*c).vmx_capability[MISC_FEATURES as usize] |= VMX_F!(EPT_1GB); }
    if ept & VMX_EPT_PAGE_WALK_5_BIT != 0 { (*c).vmx_capability[MISC_FEATURES as usize] |= VMX_F!(EPT_5LEVEL); }

    /* Synthetic APIC features that are aggregates of multiple features. */
    if (*c).vmx_capability[PRIMARY_CTLS as usize] & VMX_F!(VIRTUAL_TPR) != 0
        && (*c).vmx_capability[SECONDARY_CTLS as usize] & VMX_F!(VIRT_APIC_ACCESSES) != 0
    { (*c).vmx_capability[MISC_FEATURES as usize] |= VMX_F!(FLEXPRIORITY); }

    if (*c).vmx_capability[PRIMARY_CTLS as usize] & VMX_F!(VIRTUAL_TPR) != 0
        && (*c).vmx_capability[SECONDARY_CTLS as usize] & VMX_F!(APIC_REGISTER_VIRT) != 0
        && (*c).vmx_capability[SECONDARY_CTLS as usize] & VMX_F!(VIRT_INTR_DELIVERY) != 0
        && (*c).vmx_capability[MISC_FEATURES as usize] & VMX_F!(POSTED_INTR) != 0
    { (*c).vmx_capability[MISC_FEATURES as usize] |= VMX_F!(APICV); }

    /* Set the synthetic cpufeatures to preserve /proc/cpuinfo's ABI. */
    if (*c).vmx_capability[PRIMARY_CTLS as usize] & VMX_F!(VIRTUAL_TPR) != 0 { set_cpu_cap(c, X86_FEATURE_TPR_SHADOW); }
    if (*c).vmx_capability[MISC_FEATURES as usize] & VMX_F!(FLEXPRIORITY) != 0 { set_cpu_cap(c, X86_FEATURE_FLEXPRIORITY); }
    if (*c).vmx_capability[MISC_FEATURES as usize] & VMX_F!(VIRTUAL_NMIS) != 0 { set_cpu_cap(c, X86_FEATURE_VNMI); }
    if (*c).vmx_capability[SECONDARY_CTLS as usize] & VMX_F!(EPT) != 0 { set_cpu_cap(c, X86_FEATURE_EPT); }
    if (*c).vmx_capability[MISC_FEATURES as usize] & VMX_F!(EPT_AD) != 0 { set_cpu_cap(c, X86_FEATURE_EPT_AD); }
    if (*c).vmx_capability[MISC_FEATURES as usize] & VMX_F!(VPID) != 0 { set_cpu_cap(c, X86_FEATURE_VPID); }
}
// #endif

unsafe extern "C" fn nosgx(_str: *mut core::ffi::c_char) -> i32 {
    setup_clear_cpu_cap(X86_FEATURE_SGX);
    0
}

// early_param("nosgx", nosgx);

pub unsafe fn init_ia32_feat_ctl(c: *mut cpuinfo_x86) {
    let mut enable_sgx_kvm = false;
    let mut enable_sgx_driver = false;
    let tboot = tboot_enabled();
    let mut enable_vmx: bool;
    let mut msr: u64;

    if rdmsrq_safe(MSR_IA32_FEAT_CTL, &mut msr) != 0 {
        clear_cpu_cap(c, X86_FEATURE_VMX);
        clear_cpu_cap(c, X86_FEATURE_SGX);
        return;
    }

    enable_vmx = cpu_has(c, X86_FEATURE_VMX) && IS_ENABLED(CONFIG_KVM_INTEL);
    if cpu_has(c, X86_FEATURE_SGX) && IS_ENABLED(CONFIG_X86_SGX) {
        enable_sgx_driver = cpu_has(c, X86_FEATURE_SGX_LC);
        enable_sgx_kvm = enable_vmx && IS_ENABLED(CONFIG_X86_SGX_KVM);
    }

    if msr & FEAT_CTL_LOCKED != 0 { // goto update_caps
    } else {
        msr = FEAT_CTL_LOCKED;
        if enable_vmx {
            msr |= FEAT_CTL_VMX_ENABLED_OUTSIDE_SMX;
            if tboot { msr |= FEAT_CTL_VMX_ENABLED_INSIDE_SMX; }
        }
        if enable_sgx_kvm || enable_sgx_driver {
            msr |= FEAT_CTL_SGX_ENABLED;
            if enable_sgx_driver { msr |= FEAT_CTL_SGX_LC_ENABLED; }
        }
        wrmsrq(MSR_IA32_FEAT_CTL, msr);
    }

    set_cpu_cap(c, X86_FEATURE_MSR_IA32_FEAT_CTL);
    if !cpu_has(c, X86_FEATURE_VMX) { // goto update_sgx
    } else if (tboot && msr & FEAT_CTL_VMX_ENABLED_INSIDE_SMX == 0)
        || (!tboot && msr & FEAT_CTL_VMX_ENABLED_OUTSIDE_SMX == 0) {
        if IS_ENABLED(CONFIG_KVM_INTEL) { pr_err_once!("VMX (%s TXT) disabled by BIOS\\n", if tboot { "inside" } else { "outside" }); }
        clear_cpu_cap(c, X86_FEATURE_VMX);
    } else {
        // #ifdef CONFIG_X86_VMX_FEATURE_NAMES
        init_vmx_capabilities(c);
        // #endif
    }

    if msr & FEAT_CTL_SGX_ENABLED == 0 {
        if enable_sgx_kvm || enable_sgx_driver { pr_err_once!("SGX disabled or unsupported by BIOS.\\n"); }
        clear_cpu_cap(c, X86_FEATURE_SGX);
        return;
    }
    if !cpu_has(c, X86_FEATURE_VMX) && enable_sgx_kvm {
        pr_err_once!("SGX virtualization disabled due to lack of VMX.\\n");
        enable_sgx_kvm = false;
    }
    if msr & FEAT_CTL_SGX_LC_ENABLED == 0 && enable_sgx_driver {
        if !enable_sgx_kvm {
            pr_err_once!("SGX Launch Control is locked. Disable SGX.\\n");
            clear_cpu_cap(c, X86_FEATURE_SGX);
        } else {
            pr_err_once!("SGX Launch Control is locked. Support SGX virtualization only.\\n");
            clear_cpu_cap(c, X86_FEATURE_SGX_LC);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
