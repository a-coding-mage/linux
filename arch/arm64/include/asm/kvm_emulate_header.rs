/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of the ARM64 KVM emulation header. */

pub const CURRENT_EL_SP_EL0_VECTOR: u64 = 0x0;
pub const CURRENT_EL_SP_ELX_VECTOR: u64 = 0x200;
pub const LOWER_EL_AARCH64_VECTOR: u64 = 0x400;
pub const LOWER_EL_AARCH32_VECTOR: u64 = 0x600;

#[repr(C)]
pub enum exception_type { except_type_sync = 0, except_type_irq = 0x80, except_type_fiq = 0x100, except_type_serror = 0x180 }

extern "C" {
    fn kvm_condition_valid32(vcpu: *const kvm_vcpu) -> bool;
    fn kvm_skip_instr32(vcpu: *mut kvm_vcpu);
    fn kvm_inject_undefined(vcpu: *mut kvm_vcpu);
    fn kvm_inject_sync(vcpu: *mut kvm_vcpu, esr: u64);
    fn kvm_inject_serror_esr(vcpu: *mut kvm_vcpu, esr: u64) -> i32;
    fn kvm_inject_sea(vcpu: *mut kvm_vcpu, iabt: bool, addr: u64) -> i32;
    fn kvm_inject_dabt_excl_atomic(vcpu: *mut kvm_vcpu, addr: u64) -> i32;
    fn kvm_inject_size_fault(vcpu: *mut kvm_vcpu);
    fn kvm_vcpu_wfi(vcpu: *mut kvm_vcpu);
    fn kvm_emulate_nested_eret(vcpu: *mut kvm_vcpu);
    fn kvm_inject_nested_sync(vcpu: *mut kvm_vcpu, esr: u64) -> i32;
    fn kvm_inject_nested_irq(vcpu: *mut kvm_vcpu) -> i32;
    fn kvm_inject_nested_sea(vcpu: *mut kvm_vcpu, iabt: bool, addr: u64) -> i32;
    fn kvm_inject_nested_serror(vcpu: *mut kvm_vcpu, esr: u64) -> i32;
}

/* Types and constants below are supplied by the translated KVM dependencies. */
#[allow(non_camel_case_types)] type phys_addr_t = u64;
#[allow(non_camel_case_types)] type kvm_vcpu = crate::kvm_vcpu;
#[allow(non_camel_case_types)] type kvm = crate::kvm;
#[allow(non_camel_case_types)] type kvm_cpu_context = crate::kvm_cpu_context;
#[allow(non_camel_case_types)] type vcpu_reset_state = crate::vcpu_reset_state;

#[inline] pub unsafe fn kvm_inject_sea_dabt(v: *mut kvm_vcpu, a: u64) -> i32 { kvm_inject_sea(v, false, a) }
#[inline] pub unsafe fn kvm_inject_sea_iabt(v: *mut kvm_vcpu, a: u64) -> i32 { kvm_inject_sea(v, true, a) }
#[inline] pub unsafe fn kvm_inject_serror(v: *mut kvm_vcpu) -> i32 { kvm_inject_serror_esr(v, ESR_ELx_ISV) }

#[inline] pub unsafe fn kvm_inject_nested_sve_trap(v: *mut kvm_vcpu) {
    let esr = FIELD_PREP(ESR_ELx_EC_MASK, ESR_ELx_EC_SVE) | ESR_ELx_IL;
    kvm_inject_nested_sync(v, esr);
}

#[inline] pub unsafe fn vcpu_el1_is_32bit(v: *mut kvm_vcpu) -> bool { !( (*v).arch.hcr_el2 & HCR_RW != 0) }
#[inline] pub unsafe fn vcpu_reset_hcr(v: *mut kvm_vcpu) {
    if !vcpu_has_run_once(v) { (*v).arch.hcr_el2 = HCR_GUEST_FLAGS; }
    if !cpus_have_final_cap(ARM64_HAS_STAGE2_FWB) { (*v).arch.hcr_el2 |= HCR_TVM; }
}
#[inline] pub unsafe fn vcpu_hcr(v: *mut kvm_vcpu) -> *mut usize { &mut (*v).arch.hcr_el2 as *mut _ as *mut usize }
#[inline] pub unsafe fn vcpu_get_vsesr(v: *mut kvm_vcpu) -> u64 { (*v).arch.vsesr_el2 }
#[inline] pub unsafe fn vcpu_set_vsesr(v: *mut kvm_vcpu, x: u64) { (*v).arch.vsesr_el2 = x; }
#[inline] pub unsafe fn vcpu_pc(v: *const kvm_vcpu) -> *mut usize { &mut (*vcpu_gp_regs(v)).pc as *mut _ }
#[inline] pub unsafe fn vcpu_cpsr(v: *const kvm_vcpu) -> *mut usize { &mut (*vcpu_gp_regs(v)).pstate as *mut _ }
#[inline] pub unsafe fn vcpu_mode_is_32bit(v: *const kvm_vcpu) -> bool { *vcpu_cpsr(v) & PSR_MODE32_BIT != 0 }
#[inline] pub unsafe fn kvm_condition_valid(v: *const kvm_vcpu) -> bool { if vcpu_mode_is_32bit(v) { kvm_condition_valid32(v) } else { true } }
#[inline] pub unsafe fn vcpu_set_thumb(v: *mut kvm_vcpu) { *vcpu_cpsr(v) |= PSR_AA32_T_BIT; }
#[inline] pub unsafe fn vcpu_get_reg(v: *const kvm_vcpu, r: u8) -> usize { if r == 31 { 0 } else { (*vcpu_gp_regs(v)).regs[r as usize] } }
#[inline] pub unsafe fn vcpu_set_reg(v: *mut kvm_vcpu, r: u8, x: usize) { if r != 31 { (*vcpu_gp_regs(v)).regs[r as usize] = x; } }
#[inline] pub unsafe fn vcpu_is_el2_ctxt(c: *const kvm_cpu_context) -> bool { matches!((*c).regs.pstate & (PSR_MODE32_BIT | PSR_MODE_MASK), PSR_MODE_EL2h | PSR_MODE_EL2t) }
#[inline] pub unsafe fn vcpu_is_el2(v: *const kvm_vcpu) -> bool { vcpu_is_el2_ctxt(&(*v).arch.ctxt) }
#[inline] pub unsafe fn vcpu_el2_e2h_is_set(v: *const kvm_vcpu) -> bool { !cpus_have_final_cap(ARM64_HAS_HCR_NV1) || (__vcpu_sys_reg(v,HCR_EL2)&HCR_E2H)!=0 }
#[inline] pub unsafe fn vcpu_el2_tge_is_set(v: *const kvm_vcpu) -> bool { ctxt_sys_reg(&(*v).arch.ctxt,HCR_EL2)&HCR_TGE != 0 }
#[inline] pub unsafe fn vcpu_el2_amo_is_set(v: *const kvm_vcpu) -> bool { (vcpu_is_el2(v)&&vcpu_el2_e2h_is_set(v)&&!vcpu_el2_tge_is_set(v)) || ctxt_sys_reg(&(*v).arch.ctxt,HCR_EL2)&HCR_AMO != 0 }
#[inline] pub unsafe fn is_hyp_ctxt(v: *const kvm_vcpu) -> bool { if !vcpu_has_nv(v) { return false; } let h=__vcpu_sys_reg(v,HCR_EL2); vcpu_is_el2(v) || (h&HCR_E2H!=0 && h&HCR_TGE!=0) || h&HCR_TGE!=0 }
#[inline] pub unsafe fn vcpu_is_host_el0(v: *const kvm_vcpu) -> bool { is_hyp_ctxt(v)&&!vcpu_is_el2(v) }
#[inline] pub unsafe fn is_nested_ctxt(v: *mut kvm_vcpu) -> bool { vcpu_has_nv(v)&&!is_hyp_ctxt(v) }
#[inline] pub unsafe fn vserror_state_is_nested(v: *mut kvm_vcpu) -> bool { is_nested_ctxt(v)&&(vcpu_el2_amo_is_set(v)||__vcpu_sys_reg(v,HCRX_EL2)&HCRX_EL2_TMEA!=0) }
#[inline] pub unsafe fn kvm_has_nv2(k: *mut kvm) -> bool { cpus_have_final_cap(ARM64_HAS_NESTED_VIRT)&&kvm_has_feat(k,ID_AA64MMFR4_EL1,NV_frac,NV2_ONLY) }
#[inline] pub unsafe fn kvm_has_nv3(k: *mut kvm) -> bool { cpus_have_final_cap(ARM64_HAS_NV3)&&kvm_has_feat(k,ID_AA64MMFR4_EL1,NV_frac,NV3) }
#[inline] pub unsafe fn is_nested_nv3_ctxt(v: *mut kvm_vcpu) -> bool { has_vhe()&&kvm_has_nv3((*v).kvm)&&is_nested_ctxt(v)&&__vcpu_sys_reg(v,HCR_EL2)&HCR_EL2_NV!=0&&__vcpu_sys_reg(v,HCRX_EL2)&HCRX_EL2_NVTGE!=0 }

#[inline] pub fn host_spsr_to_spsr32(mut s: usize) -> usize { let overlap=(1usize<<24)|(1usize<<21); let dit=((s&PSR_AA32_DIT_BIT)!=0) as usize; s &= !overlap; s |= dit<<21; s }
#[inline] pub unsafe fn vcpu_mode_priv(v: *const kvm_vcpu) -> bool { if vcpu_mode_is_32bit(v) { return (*vcpu_cpsr(v)&PSR_AA32_MODE_MASK)>PSR_AA32_MODE_USR; } *vcpu_cpsr(v)&PSR_MODE_MASK != PSR_MODE_EL0t }
#[inline] pub unsafe fn kvm_vcpu_get_esr(v:*const kvm_vcpu)->u64 { (*v).arch.fault.esr_el2 }
#[inline] pub unsafe fn guest_hyp_wfx_traps_enabled(v:*const kvm_vcpu)->bool { let e=kvm_vcpu_get_esr(v); let w=e&ESR_ELx_WFx_ISS_WFE!=0; let h=__vcpu_sys_reg(v,HCR_EL2); vcpu_has_nv(v)&&!vcpu_is_el2(v)&&((w&&h&HCR_TWE!=0)||(!w&&h&HCR_TWI!=0)) }
#[inline] pub unsafe fn kvm_vcpu_get_condition(v:*const kvm_vcpu)->i32 { let e=kvm_vcpu_get_esr(v); if e&ESR_ELx_CV!=0 { ((e&ESR_ELx_COND_MASK)>>ESR_ELx_COND_SHIFT) as i32 } else {-1} }
#[inline] pub unsafe fn kvm_vcpu_get_hfar(v:*const kvm_vcpu)->usize { (*v).arch.fault.far_el2 }
#[inline] pub unsafe fn kvm_vcpu_get_fault_ipa(v:*const kvm_vcpu)->phys_addr_t { let h=(*v).arch.fault.hpfar_el2; if h&HPFAR_EL2_NS==0 { INVALID_GPA } else { FIELD_GET(HPFAR_EL2_FIPA,h)<<12 } }
#[inline] pub unsafe fn kvm_vcpu_get_disr(v:*const kvm_vcpu)->u64 { (*v).arch.fault.disr_el1 }
#[inline] pub unsafe fn kvm_vcpu_hvc_get_imm(v:*const kvm_vcpu)->u32 { (kvm_vcpu_get_esr(v)&ESR_ELx_xVC_IMM_MASK) as u32 }
#[inline] pub unsafe fn kvm_vcpu_dabt_isvalid(v:*const kvm_vcpu)->bool { kvm_vcpu_get_esr(v)&ESR_ELx_ISV!=0 }
#[inline] pub unsafe fn kvm_vcpu_dabt_iss_nisv_sanitized(v:*const kvm_vcpu)->usize { (kvm_vcpu_get_esr(v)&(ESR_ELx_CM|ESR_ELx_WNR|ESR_ELx_FSC)) as usize }
#[inline] pub unsafe fn kvm_vcpu_dabt_issext(v:*const kvm_vcpu)->bool { kvm_vcpu_get_esr(v)&ESR_ELx_SSE!=0 }
#[inline] pub unsafe fn kvm_vcpu_dabt_issf(v:*const kvm_vcpu)->bool { kvm_vcpu_get_esr(v)&ESR_ELx_SF!=0 }
#[inline] pub unsafe fn kvm_vcpu_dabt_get_rd(v:*const kvm_vcpu)->i32 { ((kvm_vcpu_get_esr(v)&ESR_ELx_SRT_MASK)>>ESR_ELx_SRT_SHIFT) as i32 }
#[inline] pub unsafe fn kvm_vcpu_abt_iss1tw(v:*const kvm_vcpu)->bool { kvm_vcpu_get_esr(v)&ESR_ELx_S1PTW!=0 }
#[inline] pub unsafe fn kvm_vcpu_dabt_iswrite(v:*const kvm_vcpu)->bool { kvm_vcpu_get_esr(v)&ESR_ELx_WNR!=0 }
#[inline] pub unsafe fn kvm_vcpu_dabt_is_cm(v:*const kvm_vcpu)->bool { kvm_vcpu_get_esr(v)&ESR_ELx_CM!=0 }
#[inline] pub unsafe fn kvm_vcpu_dabt_get_as(v:*const kvm_vcpu)->u32 { 1<<((kvm_vcpu_get_esr(v)&ESR_ELx_SAS)>>ESR_ELx_SAS_SHIFT) }
#[inline] pub unsafe fn kvm_vcpu_trap_il_is32bit(v:*const kvm_vcpu)->bool { kvm_vcpu_get_esr(v)&ESR_ELx_IL!=0 }
#[inline] pub unsafe fn kvm_vcpu_trap_get_class(v:*const kvm_vcpu)->u8 { ESR_ELx_EC(kvm_vcpu_get_esr(v)) }
#[inline] pub unsafe fn kvm_vcpu_trap_is_iabt(v:*const kvm_vcpu)->bool { kvm_vcpu_trap_get_class(v)==ESR_ELx_EC_IABT_LOW }
#[inline] pub unsafe fn kvm_vcpu_trap_is_exec_fault(v:*const kvm_vcpu)->bool { kvm_vcpu_trap_is_iabt(v)&&!kvm_vcpu_abt_iss1tw(v) }
#[inline] pub unsafe fn kvm_vcpu_trap_get_fault(v:*const kvm_vcpu)->u8 { (kvm_vcpu_get_esr(v)&ESR_ELx_FSC) as u8 }
#[inline] pub unsafe fn kvm_vcpu_trap_is_permission_fault(v:*const kvm_vcpu)->bool { esr_fsc_is_permission_fault(kvm_vcpu_get_esr(v)) }
#[inline] pub unsafe fn kvm_vcpu_trap_is_translation_fault(v:*const kvm_vcpu)->bool { esr_fsc_is_translation_fault(kvm_vcpu_get_esr(v)) }
#[inline] pub unsafe fn kvm_vcpu_trap_get_perm_fault_granule(v:*const kvm_vcpu)->u64 { BIT(ARM64_HW_PGTABLE_LEVEL_SHIFT(kvm_vcpu_get_esr(v)&ESR_ELx_FSC_LEVEL)) }
#[inline] pub unsafe fn kvm_vcpu_abt_issea(v:*const kvm_vcpu)->bool { let f=kvm_vcpu_trap_get_fault(v); f==ESR_ELx_FSC_EXTABT || (f>=ESR_ELx_FSC_SEA_TTW(-1)&&f<=ESR_ELx_FSC_SEA_TTW(3)) || f==ESR_ELx_FSC_SECC || (f>=ESR_ELx_FSC_SECC_TTW(-1)&&f<=ESR_ELx_FSC_SECC_TTW(3)) }
#[inline] pub unsafe fn kvm_vcpu_sys_get_rt(v:*mut kvm_vcpu)->i32 { ESR_ELx_SYS64_ISS_RT(kvm_vcpu_get_esr(v)) }
#[inline] pub unsafe fn kvm_is_write_fault(v:*mut kvm_vcpu)->bool { if kvm_vcpu_abt_iss1tw(v) { return kvm_vcpu_trap_is_permission_fault(v); } if kvm_vcpu_trap_is_iabt(v) { return false; } kvm_vcpu_dabt_iswrite(v) }
#[inline] pub unsafe fn kvm_vcpu_get_mpidr_aff(v:*mut kvm_vcpu)->usize { (__vcpu_sys_reg(v,MPIDR_EL1)&MPIDR_HWID_BITMASK) as usize }

/* Remaining endian conversion, trap-control, HCRX setup, and reset helpers retain the source interfaces. */
#[inline] pub unsafe fn kvm_incr_pc(v:*mut kvm_vcpu) { WARN_ON(vcpu_get_flag(v,PENDING_EXCEPTION)); vcpu_set_flag(v,INCREMENT_PC); }
#[inline] pub unsafe fn vcpu_sanitised_cptr_el2(v:*const kvm_vcpu)->u64 { let mut c=vcpu_read_sys_reg(v,CPTR_EL2); if !vcpu_el2_e2h_is_set(v) { c=translate_cptr_el2_to_cpacr_el1(c); } c }
#[inline] pub unsafe fn guest_hyp_fpsimd_traps_enabled(v:*const kvm_vcpu)->bool { __guest_hyp_cptr_xen_trap_enabled(v,FPEN) }
#[inline] pub unsafe fn guest_hyp_sve_traps_enabled(v:*const kvm_vcpu)->bool { __guest_hyp_cptr_xen_trap_enabled(v,ZEN) }
#[inline] pub unsafe fn ____cptr_xen_trap_enabled(v:*const kvm_vcpu,x:u32)->bool { match x {0|2=>true,1=>vcpu_el2_tge_is_set(v)&&!vcpu_is_el2(v),_=>false} }
#[inline] pub unsafe fn vcpu_data_guest_to_host(v:*mut kvm_vcpu,d:usize,n:u32)->usize { if kvm_vcpu_is_be(v){match n{1=>d&0xff,2=>be16_to_cpu(d&0xffff),4=>be32_to_cpu(d&0xffff_ffff),_=>be64_to_cpu(d)}}else{match n{1=>d&0xff,2=>le16_to_cpu(d&0xffff),4=>le32_to_cpu(d&0xffff_ffff),_=>le64_to_cpu(d)}} }
#[inline] pub unsafe fn vcpu_data_host_to_guest(v:*mut kvm_vcpu,d:usize,n:u32)->usize { if kvm_vcpu_is_be(v){match n{1=>d&0xff,2=>cpu_to_be16(d&0xffff),4=>cpu_to_be32(d&0xffff_ffff),_=>cpu_to_be64(d)}}else{match n{1=>d&0xff,2=>cpu_to_le16(d&0xffff),4=>cpu_to_le32(d&0xffff_ffff),_=>cpu_to_le64(d)}} }
#[inline] pub unsafe fn kvm_vcpu_set_be(v:*mut kvm_vcpu){if vcpu_mode_is_32bit(v){*vcpu_cpsr(v)|=PSR_AA32_E_BIT}else{let r=if vcpu_has_nv(v){SCTLR_EL2}else{SCTLR_EL1};let mut s=vcpu_read_sys_reg(v,r);s|=SCTLR_ELx_EE;vcpu_write_sys_reg(v,s,r)}}
#[inline] pub unsafe fn kvm_vcpu_is_be(v:*mut kvm_vcpu)->bool{if vcpu_mode_is_32bit(v){return *vcpu_cpsr(v)&PSR_AA32_E_BIT!=0}let r=if is_hyp_ctxt(v){SCTLR_EL2}else{SCTLR_EL1};let b=if vcpu_mode_priv(v){SCTLR_ELx_EE}else{SCTLR_EL1_E0E};vcpu_read_sys_reg(v,r)&b!=0}
#[inline] pub unsafe fn kvm_reset_vcpu_psci(v:*mut kvm_vcpu,s:*mut vcpu_reset_state){let mut pc=(*s).pc;if vcpu_mode_is_32bit(v)&&pc&1!=0{pc&=!1;vcpu_set_thumb(v)}if (*s).be{kvm_vcpu_set_be(v)}*vcpu_pc(v)=pc;vcpu_clear_flag(v,PENDING_EXCEPTION);vcpu_clear_flag(v,EXCEPT_MASK);vcpu_clear_flag(v,INCREMENT_PC);vcpu_set_reg(v,0,(*s).r0)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
