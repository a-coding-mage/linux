// SPDX-License-Identifier: GPL-2.0
/*
 * Intel Transactional Synchronization Extensions (TSX) control.
 *
 * Copyright (C) 2019-2021 Intel Corporation
 *
 * Author:
 *\tPawan Gupta <pawan.kumar.gupta@linux.intel.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

#[repr(C)]
enum tsx_ctrl_states {
    TSX_CTRL_AUTO,
    TSX_CTRL_ENABLE,
    TSX_CTRL_DISABLE,
    TSX_CTRL_RTM_ALWAYS_ABORT,
    TSX_CTRL_NOT_SUPPORTED,
}

static mut tsx_ctrl_state: tsx_ctrl_states = if cfg!(feature = "CONFIG_X86_INTEL_TSX_MODE_AUTO") {
    tsx_ctrl_states::TSX_CTRL_AUTO
} else if cfg!(feature = "CONFIG_X86_INTEL_TSX_MODE_OFF") {
    tsx_ctrl_states::TSX_CTRL_DISABLE
} else {
    tsx_ctrl_states::TSX_CTRL_ENABLE
};

unsafe fn tsx_disable() {
    let mut tsx: u64;

    rdmsrq(MSR_IA32_TSX_CTRL, &mut tsx);

    /* Force all transactions to immediately abort */
    tsx |= TSX_CTRL_RTM_DISABLE;

    /*
     * Ensure TSX support is not enumerated in CPUID.
     * This is visible to userspace and will ensure they
     * do not waste resources trying TSX transactions that
     * will always abort.
     */
    tsx |= TSX_CTRL_CPUID_CLEAR;

    wrmsrq(MSR_IA32_TSX_CTRL, tsx);
}

unsafe fn tsx_enable() {
    let mut tsx: u64;

    rdmsrq(MSR_IA32_TSX_CTRL, &mut tsx);

    /* Enable the RTM feature in the cpu */
    tsx &= !TSX_CTRL_RTM_DISABLE;

    /*
     * Ensure TSX support is enumerated in CPUID.
     * This is visible to userspace and will ensure they
     * can enumerate and use the TSX feature.
     */
    tsx &= !TSX_CTRL_CPUID_CLEAR;

    wrmsrq(MSR_IA32_TSX_CTRL, tsx);
}

unsafe fn x86_get_tsx_auto_mode() -> tsx_ctrl_states {
    if boot_cpu_has_bug(X86_BUG_TAA) {
        tsx_ctrl_states::TSX_CTRL_DISABLE
    } else {
        tsx_ctrl_states::TSX_CTRL_ENABLE
    }
}

/* See the original source for the full rationale regarding CPUID clearing. */
unsafe fn tsx_clear_cpuid() {
    let mut msr: u64;

    if boot_cpu_has(X86_FEATURE_RTM_ALWAYS_ABORT)
        && boot_cpu_has(X86_FEATURE_TSX_FORCE_ABORT)
    {
        rdmsrq(MSR_TSX_FORCE_ABORT, &mut msr);
        msr |= MSR_TFA_TSX_CPUID_CLEAR;
        wrmsrq(MSR_TSX_FORCE_ABORT, msr);
    } else if cpu_feature_enabled(X86_FEATURE_MSR_TSX_CTRL) {
        rdmsrq(MSR_IA32_TSX_CTRL, &mut msr);
        msr |= TSX_CTRL_CPUID_CLEAR;
        wrmsrq(MSR_IA32_TSX_CTRL, msr);
    }
}

/*
 * Disable TSX development mode
 *
 * When the microcode released in Feb 2022 is applied, TSX will be disabled by
 * default on some processors. MSR 0x122 (TSX_CTRL) and MSR 0x123
 * (IA32_MCU_OPT_CTRL) can be used to re-enable TSX for development, doing so is
 * not recommended for production deployments. In particular, applying MD_CLEAR
 * flows for mitigation of the Intel TSX Asynchronous Abort (TAA) transient
 * execution attack may not be effective on these processors when Intel TSX is
 * enabled with updated microcode.
 */
unsafe fn tsx_dev_mode_disable() {
    let mut mcu_opt_ctrl: u64;

    /* Check if RTM_ALLOW exists */
    if !boot_cpu_has_bug(X86_BUG_TAA)
        || !cpu_feature_enabled(X86_FEATURE_MSR_TSX_CTRL)
        || !cpu_feature_enabled(X86_FEATURE_SRBDS_CTRL)
    {
        return;
    }

    rdmsrq(MSR_IA32_MCU_OPT_CTRL, &mut mcu_opt_ctrl);

    if mcu_opt_ctrl & RTM_ALLOW != 0 {
        mcu_opt_ctrl &= !RTM_ALLOW;
        wrmsrq(MSR_IA32_MCU_OPT_CTRL, mcu_opt_ctrl);
        setup_force_cpu_cap(X86_FEATURE_RTM_ALWAYS_ABORT);
    }
}

unsafe fn tsx_parse_cmdline(str_: *mut core::ffi::c_char) -> i32 {
    if str_.is_null() {
        return -EINVAL;
    }

    if strcmp(str_, c"on".as_ptr()) == 0 {
        tsx_ctrl_state = tsx_ctrl_states::TSX_CTRL_ENABLE;
    } else if strcmp(str_, c"off".as_ptr()) == 0 {
        tsx_ctrl_state = tsx_ctrl_states::TSX_CTRL_DISABLE;
    } else if strcmp(str_, c"auto".as_ptr()) == 0 {
        tsx_ctrl_state = tsx_ctrl_states::TSX_CTRL_AUTO;
    } else {
        tsx_ctrl_state = tsx_ctrl_states::TSX_CTRL_DISABLE;
        pr_err!("invalid option, defaulting to off\n");
    }

    0
}

// early_param("tsx", tsx_parse_cmdline);

pub unsafe fn tsx_init() {
    tsx_dev_mode_disable();

    if boot_cpu_has(X86_FEATURE_RTM_ALWAYS_ABORT) {
        tsx_ctrl_state = tsx_ctrl_states::TSX_CTRL_RTM_ALWAYS_ABORT;
        tsx_clear_cpuid();
        setup_clear_cpu_cap(X86_FEATURE_RTM);
        setup_clear_cpu_cap(X86_FEATURE_HLE);
        return;
    }

    if x86_read_arch_cap_msr() & ARCH_CAP_TSX_CTRL_MSR != 0 {
        setup_force_cpu_cap(X86_FEATURE_MSR_TSX_CTRL);
    } else {
        tsx_ctrl_state = tsx_ctrl_states::TSX_CTRL_NOT_SUPPORTED;
        return;
    }

    if matches!(tsx_ctrl_state, tsx_ctrl_states::TSX_CTRL_AUTO) {
        tsx_ctrl_state = x86_get_tsx_auto_mode();
    }

    if matches!(tsx_ctrl_state, tsx_ctrl_states::TSX_CTRL_DISABLE) {
        tsx_disable();
        setup_clear_cpu_cap(X86_FEATURE_RTM);
        setup_clear_cpu_cap(X86_FEATURE_HLE);
    } else if matches!(tsx_ctrl_state, tsx_ctrl_states::TSX_CTRL_ENABLE) {
        tsx_enable();
        setup_force_cpu_cap(X86_FEATURE_RTM);
        setup_force_cpu_cap(X86_FEATURE_HLE);
    }
}

pub unsafe fn tsx_ap_init() {
    tsx_dev_mode_disable();

    if matches!(tsx_ctrl_state, tsx_ctrl_states::TSX_CTRL_ENABLE) {
        tsx_enable();
    } else if matches!(tsx_ctrl_state, tsx_ctrl_states::TSX_CTRL_DISABLE) {
        tsx_disable();
    } else if matches!(tsx_ctrl_state, tsx_ctrl_states::TSX_CTRL_RTM_ALWAYS_ABORT) {
        /* See comment over that function for more details. */
        tsx_clear_cpuid();
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
