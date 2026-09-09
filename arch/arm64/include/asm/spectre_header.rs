/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Interface for managing mitigations for Spectre vulnerabilities.
 *
 * Copyright (C) 2020 Google LLC
 * Author: Will Deacon <will@kernel.org>
 */

pub const BP_HARDEN_EL2_SLOTS: usize = 4;
pub const __BP_HARDEN_HYP_VECS_SZ: usize = (BP_HARDEN_EL2_SLOTS - 1) * SZ_2K;

/* External kernel dependencies supplied by other translated files. */
extern "C" {
    fn alternative_has_cap_unlikely(cap: i32) -> bool;
    fn this_cpu_ptr(data: *mut bp_hardening_data) -> *mut bp_hardening_data;
}

/* Watch out, ordering is important here. */
#[repr(C)]
pub enum mitigation_state {
    SPECTRE_UNAFFECTED,
    SPECTRE_MITIGATED,
    SPECTRE_VULNERABLE,
}

pub struct pt_regs;
pub struct task_struct;

/*
 * Note: the order of this enum corresponds to __bp_harden_hyp_vecs and
 * we rely on having the direct vectors first.
 */
#[repr(C)]
pub enum arm64_hyp_spectre_vector {
    /* Take exceptions directly to __kvm_hyp_vector. */
    HYP_VECTOR_DIRECT,
    /* Bounce via a slot containing an SMC call. */
    HYP_VECTOR_SPECTRE_DIRECT,
    /* Bounce via a slot next to the idmap page. */
    HYP_VECTOR_INDIRECT,
    /* Bounce via a slot next to the idmap page containing an SMC call. */
    HYP_VECTOR_SPECTRE_INDIRECT,
}

pub type bp_hardening_cb_t = unsafe extern "C" fn();

#[repr(C)]
pub struct bp_hardening_data {
    pub slot: arm64_hyp_spectre_vector,
    pub fn_: Option<bp_hardening_cb_t>,
}

/* DECLARE_PER_CPU_READ_MOSTLY(struct bp_hardening_data, bp_hardening_data); */
extern "C" {
    pub static mut bp_hardening_data: bp_hardening_data;
}

/* Called during entry so must be __always_inline. */
#[inline(always)]
pub unsafe fn arm64_apply_bp_hardening() {
    if !alternative_has_cap_unlikely(ARM64_SPECTRE_V2) {
        return;
    }

    let d = this_cpu_ptr(&mut bp_hardening_data);
    if let Some(func) = (*d).fn_ {
        func();
    }
}

extern "C" {
    pub fn arm64_get_spectre_v2_state() -> mitigation_state;
    pub fn has_spectre_v2(cap: *const arm64_cpu_capabilities, scope: i32) -> bool;
    pub fn spectre_v2_enable_mitigation(unused: *const arm64_cpu_capabilities);

    pub fn has_spectre_v3a(cap: *const arm64_cpu_capabilities, scope: i32) -> bool;
    pub fn spectre_v3a_enable_mitigation(unused: *const arm64_cpu_capabilities);

    pub fn arm64_get_spectre_v4_state() -> mitigation_state;
    pub fn has_spectre_v4(cap: *const arm64_cpu_capabilities, scope: i32) -> bool;
    pub fn spectre_v4_enable_mitigation(unused: *const arm64_cpu_capabilities);
    pub fn spectre_v4_enable_task_mitigation(tsk: *mut task_struct);

    pub fn arm64_get_meltdown_state() -> mitigation_state;

    pub fn arm64_get_spectre_bhb_state() -> mitigation_state;
    pub fn is_spectre_bhb_affected(entry: *const arm64_cpu_capabilities, scope: i32) -> bool;
    pub static mut __nospectre_bhb: bool;
    pub fn get_spectre_bhb_loop_value() -> u8;
    pub fn is_spectre_bhb_fw_mitigated() -> bool;
    pub fn spectre_bhb_enable_mitigation(unused: *const arm64_cpu_capabilities);
    pub fn try_emulate_el1_ssbs(regs: *mut pt_regs, instr: u32) -> bool;

    pub fn spectre_v4_patch_fw_mitigation_enable(alt: *mut alt_instr, origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32);
    pub fn smccc_patch_fw_mitigation_conduit(alt: *mut alt_instr, origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32);
    pub fn spectre_bhb_patch_loop_mitigation_enable(alt: *mut alt_instr, origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32);
    pub fn spectre_bhb_patch_fw_mitigation_enabled(alt: *mut alt_instr, origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32);
    pub fn spectre_bhb_patch_loop_iter(alt: *mut alt_instr, origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32);
    pub fn spectre_bhb_patch_wa3(alt: *mut alt_instr, origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32);
    pub fn spectre_bhb_patch_clearbhb(alt: *mut alt_instr, origptr: *mut __le32, updptr: *mut __le32, nr_inst: i32);
    pub fn spectre_print_disabled_mitigations();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
