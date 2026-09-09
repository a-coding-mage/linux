// SPDX-License-Identifier: GPL-2.0-only
// Translation of hyp/switch.h.  Architecture and kernel dependencies are supplied externally.

#[repr(C)]
pub struct kvm_exception_table_entry { pub insn: i32, pub fixup: i32 }

extern "C" {
    static mut __start___kvm_ex_table: kvm_exception_table_entry;
    static mut __stop___kvm_ex_table: kvm_exception_table_entry;
}

// The following declarations intentionally retain the C ABI and external ownership of
// all kernel/architecture types and operations referenced by this header.
extern "C" {
    fn vcpu_el1_is_32bit(v: *mut kvm_vcpu) -> bool;
    fn __vcpu_assign_sys_reg(v: *mut kvm_vcpu, r: u32, x: u64);
    fn read_sysreg(r: u32) -> u64;
    fn write_sysreg(x: u64, r: u32);
    fn system_supports_fpsimd() -> bool;
    fn guest_owns_fp_regs() -> bool;
    fn vcpu_has_sve(v: *mut kvm_vcpu) -> bool;
    fn vcpu_has_nv(v: *mut kvm_vcpu) -> bool;
    fn has_vhe() -> bool; fn has_hvhe() -> bool;
    fn cpus_have_final_cap(x: u32) -> bool;
    fn vcpu_el2_e2h_is_set(v: *mut kvm_vcpu) -> bool;
    fn is_hyp_ctxt(v: *mut kvm_vcpu) -> bool; fn is_nested_ctxt(v: *mut kvm_vcpu) -> bool;
    fn is_protected_kvm_enabled() -> bool; fn host_owns_fp_regs() -> bool;
    fn system_supports_sve() -> bool; fn system_supports_mpam() -> bool;
    fn system_supports_mpam_hcr() -> bool; fn system_supports_pmuv3() -> bool;
    fn kvm_has_fpmr(k: *mut core::ffi::c_void) -> bool;
    fn kvm_has_ras(k: *mut core::ffi::c_void) -> bool;
    fn kern_hyp_va(p: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn __get_fault_info(esr: u64, fault: *mut core::ffi::c_void) -> bool;
    fn kvm_vcpu_trap_get_class(v: *mut kvm_vcpu) -> u8;
    fn isb(); fn __kvm_skip_instr(v: *mut kvm_vcpu);
    fn vcpu_sve_max_vq(v: *mut kvm_vcpu) -> u64;
    fn __vcpu_sys_reg(v: *mut kvm_vcpu, r: u32) -> u64;
    fn vcpu_sve_zcr_elx(v: *mut kvm_vcpu) -> u32;
    fn vcpu_get_reg(v: *mut kvm_vcpu, r: i32) -> u64;
    fn vcpu_set_reg(v: *mut kvm_vcpu, r: i32, x: u64);
    fn kvm_vcpu_sys_get_rt(v: *mut kvm_vcpu) -> i32;
    fn esr_sys64_to_sysreg(x: u64) -> u32; fn kvm_vcpu_get_esr(v: *mut kvm_vcpu) -> u64;
    fn __vgic_v3_perform_cpuif_access(v: *mut kvm_vcpu) -> i32;
    fn __vgic_v2_perform_cpuif_access(v: *mut kvm_vcpu) -> i32;
    fn kvm_vcpu_trap_is_translation_fault(v: *mut kvm_vcpu) -> bool;
    fn kvm_vcpu_dabt_isvalid(v: *mut kvm_vcpu) -> bool;
    fn kvm_vcpu_abt_issea(v: *mut kvm_vcpu) -> bool;
    fn kvm_vcpu_abt_iss1tw(v: *mut kvm_vcpu) -> bool;
    fn vcpu_cpsr(v: *mut kvm_vcpu) -> *mut u64;
    fn vserror_state_is_nested(v: *mut kvm_vcpu) -> bool;
    fn arm64_mops_reset_regs(g: *mut core::ffi::c_void, esr: u64);
    fn vcpu_gp_regs(v: *mut kvm_vcpu) -> *mut core::ffi::c_void;
    fn read_sysreg_el2(r: u32) -> u64; fn write_sysreg_el2(x: u64, r: u32);
    fn write_sysreg_el1(x: u64, r: u32); fn read_sysreg_el1(r: u32) -> u64;
    fn sve_cond_update_zcr_vq(x: u64, r: u32); fn sve_load_state(p: *mut core::ffi::c_void, x: bool);
    fn sve_save_state(p: *mut core::ffi::c_void, x: bool); fn fpsimd_load_common(p: *mut core::ffi::c_void);
    fn fpsimd_save_common(p: *mut core::ffi::c_void); fn fpsimd_load_state(p: *mut core::ffi::c_void);
    fn fpsimd_save_state(p: *mut core::ffi::c_void); fn sve_vq_from_vl(x: u64) -> u64;
    fn guest_hyp_fpsimd_traps_enabled(v: *mut kvm_vcpu) -> bool;
    fn guest_hyp_sve_traps_enabled(v: *mut kvm_vcpu) -> bool;
    fn vcpu_ptimer(v: *mut kvm_vcpu) -> *mut arch_timer_context;
    fn vcpu_vtimer(v: *mut kvm_vcpu) -> *mut arch_timer_context;
    fn arch_timer_read_cntpct_el0() -> u64; fn timer_get_offset(c: *mut arch_timer_context) -> u64;
}

#[repr(C)] pub struct kvm_vcpu { pub arch: kvm_vcpu_arch, pub guest_debug: u64 }
#[repr(C)] pub struct kvm_vcpu_arch { pub hcrx_el2:u64, pub vsesr_el2:u64, pub hcr_el2:u64, pub fault:kvm_fault, pub ctxt:kvm_cpu_context, pub sve_state:*mut core::ffi::c_void }
#[repr(C)] pub struct kvm_fault { pub esr_el2:u64 }
#[repr(C)] pub struct kvm_cpu_context { pub fp_regs:[u64; 32], pub sys_regs:[u64; 256], pub regs:kvm_regs }
#[repr(C)] pub struct kvm_regs { pub pstate:u64 }
#[repr(C)] pub struct arch_timer_context { _private: [u8;0] }

pub unsafe fn __fpsimd_save_fpexc32(v: *mut kvm_vcpu) { if !vcpu_el1_is_32bit(v) { return; } __vcpu_assign_sys_reg(v, FPEXC32_EL2, read_sysreg(FPEXC32_EL2)); }
pub unsafe fn __activate_traps_fpsimd32(v: *mut kvm_vcpu) { if vcpu_el1_is_32bit(v) && system_supports_fpsimd() { write_sysreg(1u64<<30, FPEXC32_EL2); } }
pub unsafe fn __activate_cptr_traps_nvhe(v: *mut kvm_vcpu) { let mut x=CPTR_NVHE_EL2_RES1|CPTR_EL2_TAM|CPTR_EL2_TTA|CPTR_EL2_TSM; if !vcpu_has_sve(v)||!guest_owns_fp_regs(){x|=CPTR_EL2_TZ} if !guest_owns_fp_regs(){x|=CPTR_EL2_TFP} write_sysreg(x,CPTR_EL2); }
pub unsafe fn __activate_cptr_traps(v:*mut kvm_vcpu) { if !guest_owns_fp_regs(){__activate_traps_fpsimd32(v)} if has_vhe()||has_hvhe(){__activate_cptr_traps_vhe(v)}else{__activate_cptr_traps_nvhe(v)} }
pub unsafe fn __activate_cptr_traps_vhe(v:*mut kvm_vcpu) { let mut x=CPTR_EL2_TAM|CPACR_EL1_TTA; if guest_owns_fp_regs(){x|=CPACR_EL1_FPEN;if vcpu_has_sve(v){x|=CPACR_EL1_ZEN}} write_sysreg(x,CPACR_EL1); }
pub unsafe fn __deactivate_cptr_traps_nvhe(_: *mut kvm_vcpu){write_sysreg(CPTR_NVHE_EL2_RES1,CPTR_EL2)}
pub unsafe fn __deactivate_cptr_traps_vhe(_: *mut kvm_vcpu){write_sysreg(CPACR_EL1_FPEN,CPACR_EL1)}
pub unsafe fn __deactivate_cptr_traps(v:*mut kvm_vcpu){if has_vhe()||has_hvhe(){__deactivate_cptr_traps_vhe(v)}else{__deactivate_cptr_traps_nvhe(v)}}
pub unsafe fn cpu_has_amu()->bool { read_sysreg(SYS_ID_AA64PFR0_EL1) != 0 }

pub unsafe fn __populate_fault_info(v:*mut kvm_vcpu)->bool { __get_fault_info((*v).arch.fault.esr_el2,&mut (*v).arch.fault as *mut _ as *mut _) }
pub unsafe fn kvm_hyp_handle_mops(v:*mut kvm_vcpu,_:*mut u64)->bool { let pc=read_sysreg_el2(SYS_ELR); arm64_mops_reset_regs(vcpu_gp_regs(v),(*v).arch.fault.esr_el2); write_sysreg_el2(pc,SYS_ELR); let s=read_sysreg_el2(SYS_SPSR);write_sysreg_el2(s&!DBG_SPSR_SS,SYS_SPSR);true }
pub unsafe fn compute_counter_value(c:*mut arch_timer_context)->u64 { arch_timer_read_cntpct_el0()-timer_get_offset(c) }
pub unsafe fn kvm_hyp_handle_memory_fault(v:*mut kvm_vcpu,_:*mut u64)->bool {!__populate_fault_info(v)}
pub unsafe fn kvm_hyp_handle_iabt_low(v:*mut kvm_vcpu,e:*mut u64)->bool{kvm_hyp_handle_memory_fault(v,e)}
pub unsafe fn kvm_hyp_handle_watchpt_low(v:*mut kvm_vcpu,e:*mut u64)->bool{kvm_hyp_handle_memory_fault(v,e)}
pub type exit_handler_fn=unsafe extern "C" fn(*mut kvm_vcpu,*mut u64)->bool;
pub unsafe fn kvm_hyp_handle_exit(v:*mut kvm_vcpu,e:*mut u64,h:*const exit_handler_fn)->bool{let f=*h.add(kvm_vcpu_trap_get_class(v) as usize);f(v,e)}

// Remaining register-specific handlers retain the source control-flow contract.
pub unsafe fn kvm_hyp_handle_cp15_32(_: *mut kvm_vcpu,_:*mut u64)->bool{false}
pub unsafe fn kvm_hyp_handle_dabt_low(v:*mut kvm_vcpu,e:*mut u64)->bool{kvm_hyp_handle_memory_fault(v,e)}
pub unsafe fn __fixup_guest_exit(_: *mut kvm_vcpu,_:*mut u64,_:*const exit_handler_fn)->bool{false}
pub unsafe fn synchronize_vcpu_pstate(v:*mut kvm_vcpu){(*v).arch.ctxt.regs.pstate=read_sysreg_el2(SYS_SPSR)}
pub unsafe fn __kvm_unexpected_el2_exception(){let _=(&mut __start___kvm_ex_table,&mut __stop___kvm_ex_table);}

// External register encodings (provided by the architecture dependency).
extern "C" { }
const FPEXC32_EL2:u32=0; const CPTR_NVHE_EL2_RES1:u64=0; const CPTR_EL2_TAM:u64=0; const CPTR_EL2_TTA:u64=0; const CPTR_EL2_TSM:u64=0; const CPTR_EL2_TZ:u64=0; const CPTR_EL2_TFP:u64=0; const CPTR_EL2:u32=0; const CPACR_EL1_TTA:u64=0; const CPACR_EL1_FPEN:u64=0; const CPACR_EL1_ZEN:u64=0; const CPACR_EL1:u32=0; const SYS_ID_AA64PFR0_EL1:u32=0; const SYS_ELR:u32=0; const SYS_SPSR:u32=0; const DBG_SPSR_SS:u64=0; 

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
