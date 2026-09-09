// SPDX-License-Identifier: GPL-2.0
// Translated from proc-v7-bugs.c. Kernel dependencies are supplied externally.

#[cfg(CONFIG_ARM_PSCI)]
unsafe fn spectre_v2_get_cpu_fw_mitigation_state() -> i32 {
    let mut res: arm_smccc_res = core::mem::zeroed();
    arm_smccc_1_1_invoke(ARM_SMCCC_ARCH_FEATURES_FUNC_ID, ARM_SMCCC_ARCH_WORKAROUND_1, &mut res);
    match res.a0 as i32 {
        SMCCC_RET_SUCCESS => SPECTRE_MITIGATED,
        SMCCC_ARCH_WORKAROUND_RET_UNAFFECTED => SPECTRE_UNAFFECTED,
        _ => SPECTRE_VULNERABLE,
    }
}

#[cfg(not(CONFIG_ARM_PSCI))]
unsafe fn spectre_v2_get_cpu_fw_mitigation_state() -> i32 { SPECTRE_VULNERABLE }

#[cfg(CONFIG_HARDEN_BRANCH_PREDICTOR)]
static mut harden_branch_predictor_fn: harden_branch_predictor_fn_t = None;

#[cfg(CONFIG_HARDEN_BRANCH_PREDICTOR)]
unsafe fn harden_branch_predictor_bpiall() { write_sysreg(0, BPIALL); }

#[cfg(CONFIG_HARDEN_BRANCH_PREDICTOR)]
unsafe fn harden_branch_predictor_iciallu() { write_sysreg(0, ICIALLU); }

#[cfg(CONFIG_HARDEN_BRANCH_PREDICTOR)]
unsafe fn call_smc_arch_workaround_1() {
    arm_smccc_1_1_smc(ARM_SMCCC_ARCH_WORKAROUND_1, core::ptr::null_mut());
}

#[cfg(CONFIG_HARDEN_BRANCH_PREDICTOR)]
unsafe fn call_hvc_arch_workaround_1() {
    arm_smccc_1_1_hvc(ARM_SMCCC_ARCH_WORKAROUND_1, core::ptr::null_mut());
}

#[cfg(CONFIG_HARDEN_BRANCH_PREDICTOR)]
unsafe fn spectre_v2_install_workaround(method: u32) -> u32 {
    let mut spectre_v2_method: *const core::ffi::c_char = core::ptr::null();
    let cpu = smp_processor_id();
    if harden_branch_predictor_fn.is_some() { return SPECTRE_MITIGATED as u32; }
    match method {
        SPECTRE_V2_METHOD_BPIALL => { harden_branch_predictor_fn = Some(harden_branch_predictor_bpiall); spectre_v2_method = b"BPIALL\0".as_ptr() as _; }
        SPECTRE_V2_METHOD_ICIALLU => { harden_branch_predictor_fn = Some(harden_branch_predictor_iciallu); spectre_v2_method = b"ICIALLU\0".as_ptr() as _; }
        SPECTRE_V2_METHOD_HVC => { harden_branch_predictor_fn = Some(call_hvc_arch_workaround_1); cpu_do_switch_mm = Some(cpu_v7_hvc_switch_mm); spectre_v2_method = b"hypervisor\0".as_ptr() as _; }
        SPECTRE_V2_METHOD_SMC => { harden_branch_predictor_fn = Some(call_smc_arch_workaround_1); cpu_do_switch_mm = Some(cpu_v7_smc_switch_mm); spectre_v2_method = b"firmware\0".as_ptr() as _; }
        _ => {}
    }
    if !spectre_v2_method.is_null() { pr_info!("CPU%u: Spectre v2: using %s workaround\n", smp_processor_id(), spectre_v2_method); }
    SPECTRE_MITIGATED as u32
}

#[cfg(not(CONFIG_HARDEN_BRANCH_PREDICTOR))]
unsafe fn spectre_v2_install_workaround(_method: u32) -> u32 {
    pr_info_once!("Spectre V2: workarounds disabled by configuration\n");
    SPECTRE_VULNERABLE as u32
}

unsafe fn cpu_v7_spectre_v2_init() {
    let mut state: u32;
    let mut method = 0;
    match read_cpuid_part() {
        ARM_CPU_PART_CORTEX_A8 | ARM_CPU_PART_CORTEX_A9 | ARM_CPU_PART_CORTEX_A12 |
        ARM_CPU_PART_CORTEX_A17 | ARM_CPU_PART_CORTEX_A73 | ARM_CPU_PART_CORTEX_A75 => { state = SPECTRE_MITIGATED; method = SPECTRE_V2_METHOD_BPIALL; }
        ARM_CPU_PART_CORTEX_A15 | ARM_CPU_PART_BRAHMA_B15 => { state = SPECTRE_MITIGATED; method = SPECTRE_V2_METHOD_ICIALLU; }
        ARM_CPU_PART_BRAHMA_B53 => { state = SPECTRE_UNAFFECTED; }
        ARM_CPU_PART_CORTEX_A57 | ARM_CPU_PART_CORTEX_A72 => { state = spectre_v2_get_cpu_fw_mitigation_state() as u32; if state == SPECTRE_MITIGATED { method = match arm_smccc_1_1_get_conduit() { SMCCC_CONDUIT_HVC => SPECTRE_V2_METHOD_HVC, SMCCC_CONDUIT_SMC => SPECTRE_V2_METHOD_SMC, _ => { state = SPECTRE_VULNERABLE; 0 } }; } }
        _ => { if read_cpuid_implementor() == ARM_CPU_IMP_ARM { state = SPECTRE_UNAFFECTED; } else { state = spectre_v2_get_cpu_fw_mitigation_state() as u32; } }
    }
    if state == SPECTRE_MITIGATED { state = spectre_v2_install_workaround(method); }
    spectre_v2_update_state(state as i32, method);
}

#[cfg(CONFIG_HARDEN_BRANCH_HISTORY)]
static mut spectre_bhb_method: i32 = 0;

#[cfg(CONFIG_HARDEN_BRANCH_HISTORY)]
unsafe fn spectre_bhb_method_name(method: i32) -> *const core::ffi::c_char {
    match method { SPECTRE_V2_METHOD_LOOP8 => b"loop\0".as_ptr() as _, SPECTRE_V2_METHOD_BPIALL => b"BPIALL\0".as_ptr() as _, _ => b"unknown\0".as_ptr() as _ }
}

#[cfg(CONFIG_HARDEN_BRANCH_HISTORY)]
unsafe fn spectre_bhb_install_workaround(method: i32) -> i32 {
    if spectre_bhb_method != method { if spectre_bhb_method != 0 { pr_err!("CPU%u: Spectre BHB: method disagreement, system vulnerable\n", smp_processor_id()); return SPECTRE_VULNERABLE; } if spectre_bhb_update_vectors(method) == SPECTRE_VULNERABLE { return SPECTRE_VULNERABLE; } spectre_bhb_method = method; pr_info!("CPU%u: Spectre BHB: enabling %s workaround for all CPUs\n", smp_processor_id(), spectre_bhb_method_name(method)); }
    SPECTRE_MITIGATED
}

#[cfg(not(CONFIG_HARDEN_BRANCH_HISTORY))]
unsafe fn spectre_bhb_install_workaround(_method: i32) -> i32 { SPECTRE_VULNERABLE }

unsafe fn cpu_v7_spectre_bhb_init() {
    let (mut state, mut method) = (SPECTRE_UNAFFECTED, 0);
    match read_cpuid_part() { ARM_CPU_PART_CORTEX_A15 | ARM_CPU_PART_BRAHMA_B15 | ARM_CPU_PART_CORTEX_A57 | ARM_CPU_PART_CORTEX_A72 => { state = SPECTRE_MITIGATED; method = SPECTRE_V2_METHOD_LOOP8; }, ARM_CPU_PART_CORTEX_A73 | ARM_CPU_PART_CORTEX_A75 => { state = SPECTRE_MITIGATED; method = SPECTRE_V2_METHOD_BPIALL; }, _ => {} }
    if state == SPECTRE_MITIGATED { state = spectre_bhb_install_workaround(method); }
    spectre_v2_update_state(state, method as u32);
}

unsafe fn cpu_v7_check_auxcr_set(warned: *mut bool, mask: u32, msg: *const core::ffi::c_char) -> bool {
    let aux_cr: u32; core::arch::asm!("mrc p15, 0, {0}, c1, c0, 1", out(reg) aux_cr);
    if (aux_cr & mask) != mask { if !*warned { pr_err!("CPU%u: %s", smp_processor_id(), msg); } *warned = true; return false; } true
}

static mut spectre_warned: bool = false;
unsafe fn check_spectre_auxcr(warned: *mut bool, bit: u32) -> bool { cfg!(CONFIG_HARDEN_BRANCH_PREDICTOR) && cpu_v7_check_auxcr_set(warned, bit, b"Spectre v2: firmware did not set auxiliary control register IBE bit, system vulnerable\n\0".as_ptr() as _) }
unsafe fn cpu_v7_ca8_ibe() { if check_spectre_auxcr(&mut spectre_warned, 1 << 6) { cpu_v7_spectre_v2_init(); } }
unsafe fn cpu_v7_ca15_ibe() { if check_spectre_auxcr(&mut spectre_warned, 1) { cpu_v7_spectre_v2_init(); } cpu_v7_spectre_bhb_init(); }
unsafe fn cpu_v7_bugs_init() { cpu_v7_spectre_v2_init(); cpu_v7_spectre_bhb_init(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
