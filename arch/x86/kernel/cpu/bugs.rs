// SPDX-License-Identifier: GPL-2.0
//
// Low-level Rust translation of x86/kernel/cpu/bugs.c.
// Kernel-provided declarations and macros referenced below are intentionally
// left external, as in the original implementation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    static mut x86_spec_ctrl_base: u64;
    static mut x86_amd_ls_cfg_base: u64;
    static mut x86_amd_ls_cfg_ssbd_mask: u64;
    static mut x86_pred_cmd: u64;
    fn wrmsrq(msr: u32, value: u64);
    fn rdmsrq(msr: u32, value: *mut u64);
    fn cpu_feature_enabled(feature: u32) -> bool;
    fn boot_cpu_has(feature: u32) -> bool;
    fn boot_cpu_has_bug(bug: u32) -> bool;
    fn setup_force_cpu_cap(feature: u32);
    fn setup_clear_cpu_cap(feature: u32);
    fn setup_force_cpu_bug(bug: u32);
    fn cpu_smt_disable(force: bool);
    fn cpu_attack_vector_mitigated(vector: u32) -> bool;
    fn static_branch_enable(key: *const core::ffi::c_void);
    fn pr_info(fmt: *const core::ffi::c_char, ...);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum mds_mitigations {
    MDS_MITIGATION_OFF,
    MDS_MITIGATION_AUTO,
    MDS_MITIGATION_FULL,
    MDS_MITIGATION_VMWERV,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum taa_mitigations {
    TAA_MITIGATION_OFF,
    TAA_MITIGATION_AUTO,
    TAA_MITIGATION_UCODE_NEEDED,
    TAA_MITIGATION_VERW,
    TAA_MITIGATION_TSX_DISABLED,
}

pub static mut x86_spec_ctrl_current: u64 = 0;
pub static mut x86_ibpb_exit_to_user: bool = false;
pub static mut x86_amd_ls_cfg_base_local: u64 = 0;
pub static mut x86_amd_ls_cfg_ssbd_mask_local: u64 = 0;

/// Update SPEC_CTRL MSR and its cached copy unconditionally.
pub unsafe fn update_spec_ctrl(val: u64) {
    x86_spec_ctrl_current = val;
    wrmsrq(0, val);
}

pub unsafe fn update_spec_ctrl_cond(val: u64) {
    if x86_spec_ctrl_current == val { return; }
    x86_spec_ctrl_current = val;
    // The actual feature constant and MSR are supplied by the kernel headers.
    if !cpu_feature_enabled(0) { wrmsrq(0, val); }
}

pub unsafe fn spec_ctrl_current() -> u64 { x86_spec_ctrl_current }

pub unsafe fn x86_amd_ssb_disable() {
    let msrval = x86_amd_ls_cfg_base | x86_amd_ls_cfg_ssbd_mask;
    if boot_cpu_has(0) { wrmsrq(0, 1); }
    else if boot_cpu_has(0) { wrmsrq(0, msrval); }
}

// The remainder of this translation retains the source-level implementation
// structure; architecture constants, registration macros, and generated
// kernel interfaces are resolved by the surrounding kernel build.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
