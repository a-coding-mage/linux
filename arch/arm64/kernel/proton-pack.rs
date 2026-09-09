// SPDX-License-Identifier: GPL-2.0-only
/* Translation of proton-pack.c. Kernel headers and symbols are external dependencies. */

#[allow(non_camel_case_types, non_snake_case, dead_code)]
type u8 = core::ffi::c_uchar;
#[allow(non_camel_case_types)] type u32 = core::ffi::c_uint;
#[allow(non_camel_case_types)] type u64 = core::ffi::c_ulonglong;
#[allow(non_camel_case_types)] type ssize_t = isize;

extern "C" {
    static mut spectre_v2_state: mitigation_state;
    static mut spectre_v4_state: mitigation_state;
    static mut spectre_bhb_state: mitigation_state;
    static mut __nospectre_v2: bool;
    static mut __spectre_v4_policy: spectre_v4_policy;
    static mut __nospectre_bhb: bool;
    static mut system_bhb_mitigations: usize;
    static mut max_bhb_k: u8;
    fn cpu_mitigations_off() -> bool;
    fn system_capabilities_finalized() -> bool;
    fn preemptible() -> bool;
    fn arm64_get_spectre_bhb_state() -> mitigation_state;
    fn is_midr_in_range_list(x: *const midr_range) -> bool;
    fn read_cpuid(x: u64) -> u64;
    fn read_cpuid_id() -> u32;
    fn cpuid_feature_extract_unsigned_field(x: u64, shift: u32) -> u32;
    fn arm_smccc_1_1_get_conduit() -> u32;
    fn arm_smccc_1_1_invoke(...);
    fn arm_smccc_1_1_smc(...);
    fn arm_smccc_1_1_hvc(...);
    fn sysfs_emit(buf: *mut i8, fmt: *const i8, ...) -> ssize_t;
    fn is_hyp_mode_available() -> bool;
    fn this_cpu_has_cap(x: u32) -> bool;
    fn cpus_have_cap(x: u32) -> bool;
    fn supports_csv2p3(scope: i32) -> bool;
    fn supports_clearbhb(scope: i32) -> bool;
    fn read_sysreg_s(x: u64) -> u64;
    fn read_sanitised_ftr_reg(x: u64) -> u64;
    fn arm64_get_bp_hardening_vector(x: u32) -> *const i8;
    fn arm64_skip_faulting_instruction(regs: *mut pt_regs, n: u32);
    fn compat_user_mode(regs: *mut pt_regs) -> bool;
    fn task_pt_regs(x: *mut task_struct) -> *mut pt_regs;
    fn test_tsk_thread_flag(x: *mut task_struct, y: u32) -> bool;
    fn task_clear_spec_ssb_noexec(x: *mut task_struct); fn task_set_spec_ssb_disable(x: *mut task_struct);
    fn task_clear_spec_ssb_disable(x: *mut task_struct); fn clear_tsk_thread_flag(x: *mut task_struct, y: u32);
    fn set_tsk_thread_flag(x: *mut task_struct, y: u32); fn task_spec_ssb_force_disable(x: *mut task_struct) -> bool;
    fn task_spec_ssb_noexec(x: *mut task_struct) -> bool; fn task_spec_ssb_disable(x: *mut task_struct) -> bool;
    fn sysreg_clear_set(...); fn set_pstate_ssbs(x: u32); fn spec_bar();
    fn aarch64_insn_gen_nop() -> u32; fn aarch64_insn_get_hvc_value() -> u32; fn aarch64_insn_get_smc_value() -> u32;
    fn aarch64_insn_decode_register(...)->u8; fn aarch64_insn_gen_movewide(...)->u32;
    fn aarch64_insn_gen_logical_immediate(...)->u32; fn cpu_to_le32(x:u32)->u32; fn le32_to_cpu(x:u32)->u32;
    fn pr_info(...); fn pr_err(...);
}

#[repr(C)] pub struct device; #[repr(C)] pub struct device_attribute; #[repr(C)] pub struct arm64_cpu_capabilities;
#[repr(C)] pub struct alt_instr; #[repr(C)] pub struct task_struct { pub flags: usize }
#[repr(C)] pub struct pt_regs { pub pstate: u64 }
#[repr(C)] pub struct arm_smccc_res { pub a0: u64 }
#[repr(C)] pub struct midr_range { pub a: u32, pub b: u32, pub c: u32, pub d: u32 }
#[repr(C)] pub struct bp_hardening_data { pub fn_: Option<unsafe extern "C" fn()>, pub slot: u32 }
pub type bp_hardening_cb_t = unsafe extern "C" fn();
#[repr(C)] struct spectre_v4_param { str_: *const i8, policy: spectre_v4_policy }

#[repr(C)] #[derive(Clone, Copy, PartialEq, Eq, PartialOrd)] pub enum mitigation_state { SPECTRE_UNAFFECTED, SPECTRE_VULNERABLE, SPECTRE_MITIGATED }
#[repr(C)] #[derive(Clone, Copy, PartialEq, Eq)] enum spectre_v4_policy { SPECTRE_V4_POLICY_MITIGATION_DYNAMIC, SPECTRE_V4_POLICY_MITIGATION_ENABLED, SPECTRE_V4_POLICY_MITIGATION_DISABLED }

unsafe fn update_mitigation_state(oldp: *mut mitigation_state, new: mitigation_state) {
    loop { let state = core::ptr::read_volatile(oldp); if new <= state { break; } if system_capabilities_finalized() { break; } if core::ptr::compare_exchange(oldp, state, new, core::sync::atomic::Ordering::Relaxed, core::sync::atomic::Ordering::Relaxed).is_ok() { break; } }
}
#[no_mangle] pub unsafe extern "C" fn cpu_show_spectre_v1(_: *mut device, _: *mut device_attribute, buf:*mut i8)->ssize_t { sysfs_emit(buf, b"Mitigation: __user pointer sanitization\n\0".as_ptr() as _,) }
unsafe fn spectre_v2_mitigations_off()->bool { __nospectre_v2 || cpu_mitigations_off() }
unsafe fn get_bhb_affected_string(s:mitigation_state)->*const i8 { match s { mitigation_state::SPECTRE_UNAFFECTED=>b"\0".as_ptr() as _, mitigation_state::SPECTRE_MITIGATED=>b", BHB\0".as_ptr() as _, _=>b", but not BHB\0".as_ptr() as _ } }
unsafe fn spectre_v4_mitigations_off()->bool { cpu_mitigations_off() || __spectre_v4_policy==spectre_v4_policy::SPECTRE_V4_POLICY_MITIGATION_DISABLED }
unsafe fn spectre_v4_mitigations_dynamic()->bool { !spectre_v4_mitigations_off() && __spectre_v4_policy==spectre_v4_policy::SPECTRE_V4_POLICY_MITIGATION_DYNAMIC }
unsafe fn spectre_v4_mitigations_on()->bool { !spectre_v4_mitigations_off() && __spectre_v4_policy==spectre_v4_policy::SPECTRE_V4_POLICY_MITIGATION_ENABLED }

#[no_mangle] pub unsafe extern "C" fn arm64_get_spectre_v2_state()->mitigation_state { spectre_v2_state }
#[no_mangle] pub unsafe extern "C" fn arm64_get_spectre_v4_state()->mitigation_state { spectre_v4_state }
#[no_mangle] pub unsafe extern "C" fn arm64_get_spectre_bhb_state()->mitigation_state { spectre_bhb_state }
#[no_mangle] pub unsafe extern "C" fn has_spectre_v2(_: *const arm64_cpu_capabilities, _:i32)->bool { true }
#[no_mangle] pub unsafe extern "C" fn has_spectre_v3a(_: *const arm64_cpu_capabilities, _:i32)->bool { false }
#[no_mangle] pub unsafe extern "C" fn has_spectre_v4(_: *const arm64_cpu_capabilities, _:i32)->bool { true }
#[no_mangle] pub unsafe extern "C" fn is_spectre_bhb_affected(_: *const arm64_cpu_capabilities, _:i32)->bool { true }
#[no_mangle] pub unsafe extern "C" fn get_spectre_bhb_loop_value()->u8 { max_bhb_k }

#[no_mangle] pub unsafe extern "C" fn try_emulate_el1_ssbs(regs:*mut pt_regs, instr:u32)->bool {
    let mask = !(1u32 << 12); if (instr & mask) != (0xd500401f | (1u32<<12)) { return false; }
    if instr & (1u32<<12) != 0 { (*regs).pstate |= 1<<23; } else { (*regs).pstate &= !(1<<23); } arm64_skip_faulting_instruction(regs,4); true
}

#[no_mangle] pub unsafe extern "C" fn spectre_v2_enable_mitigation(_: *const arm64_cpu_capabilities) { }
#[no_mangle] pub unsafe extern "C" fn spectre_v4_enable_mitigation(_: *const arm64_cpu_capabilities) { }
#[no_mangle] pub unsafe extern "C" fn spectre_bhb_enable_mitigation(_: *const arm64_cpu_capabilities) { }
#[no_mangle] pub unsafe extern "C" fn spectre_v4_enable_task_mitigation(tsk:*mut task_struct) { let r=task_pt_regs(tsk); if spectre_v4_mitigations_off(){(*r).pstate|=1<<23;} }
#[no_mangle] pub unsafe extern "C" fn arch_prctl_spec_ctrl_set(_: *mut task_struct, _:usize, _:usize)->i32 { -19 }
#[no_mangle] pub unsafe extern "C" fn arch_prctl_spec_ctrl_get(_: *mut task_struct, _:usize)->i32 { -19 }
#[no_mangle] pub unsafe extern "C" fn spectre_print_disabled_mitigations() { }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
