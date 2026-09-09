// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// Dependency declarations supplied by the surrounding kernel translation unit.

/* VHE specific context */
#[allow(non_upper_case_globals)]
static mut kvm_host_data: PerCpu<kvm_host_data> = PerCpu::new();
#[allow(non_upper_case_globals)]
static mut kvm_hyp_ctxt: PerCpu<kvm_cpu_context> = PerCpu::new();
#[allow(non_upper_case_globals)]
static mut kvm_hyp_vector: PerCpu<c_ulong> = PerCpu::new();

const NV_HCR_GUEST_EXCLUDE: u64 = HCR_TGE | HCR_API | HCR_APK | HCR_FIEN;

unsafe fn __compute_hcr(vcpu: *mut kvm_vcpu) -> u64 {
    let mut guest_hcr: u64;
    let mut hcr = (*vcpu).arch.hcr_el2;
    if !vcpu_has_nv(vcpu) { return hcr; }
    if is_hyp_ctxt(vcpu) {
        host_data_set_flag(VCPU_IN_HYP_CONTEXT);
        hcr |= HCR_NV | HCR_NV2 | HCR_AT | HCR_TTLB;
        if !vcpu_el2_e2h_is_set(vcpu) { hcr |= HCR_NV1; }
        if cpus_have_final_cap(ARM64_HAS_NV3) && vcpu_el2_e2h_is_set(vcpu) {
            write_sysreg_s(__vcpu_sys_reg(vcpu, HCR_EL2), SYS_NVHCR_EL2);
        } else {
            __vcpu_assign_sys_reg(vcpu, NVHCR_EL2, __vcpu_sys_reg(vcpu, HCR_EL2));
        }
        guest_hcr = kvm_vcpu_apply_reg_masks(vcpu, HCR_EL2, 0);
        write_sysreg_s((*vcpu).arch.ctxt.vncr_array, SYS_VNCR_EL2);
    } else {
        host_data_clear_flag(VCPU_IN_HYP_CONTEXT);
        guest_hcr = __vcpu_sys_reg(vcpu, HCR_EL2);
        if guest_hcr & HCR_NV != 0 {
            let mut va = __fix_to_virt(vncr_fixmap(smp_processor_id()));
            va |= __vcpu_sys_reg(vcpu, VNCR_EL2) & GENMASK(PAGE_SHIFT - 1, 0);
            write_sysreg_s(va, SYS_VNCR_EL2);
            guest_hcr |= HCR_NV2;
        }
        if guest_hcr & HCR_TWE == 0 { guest_hcr &= !(HCR_EL2_TWEDEn | HCR_EL2_TWEDEL); }
    }
    BUG_ON(host_data_test_flag(VCPU_IN_HYP_CONTEXT) && host_data_test_flag(L1_VNCR_MAPPED));
    hcr | (guest_hcr & !NV_HCR_GUEST_EXCLUDE)
}

unsafe fn __activate_traps(vcpu: *mut kvm_vcpu) {
    let mut val: u64 = 0;
    ___activate_traps(vcpu, __compute_hcr(vcpu));
    if has_cntpoff() {
        let mut map = timer_map::default(); get_timer_map(vcpu, &mut map);
        if map.direct_ptimer == vcpu_ptimer(vcpu) { val = __vcpu_sys_reg(vcpu, CNTP_CVAL_EL0); }
        if map.direct_ptimer == vcpu_hptimer(vcpu) { val = __vcpu_sys_reg(vcpu, CNTHP_CVAL_EL2); }
        if map.direct_ptimer { write_sysreg_el0(val, SYS_CNTP_CVAL); isb(); }
    }
    __activate_cptr_traps(vcpu);
    write_sysreg(this_cpu_read(kvm_hyp_vector), vbar_el1);
}

unsafe fn __deactivate_traps(vcpu: *mut kvm_vcpu) {
    let mut host_vectors = vectors;
    ___deactivate_traps(vcpu); write_sysreg_hcr(HCR_HOST_VHE_FLAGS);
    if has_cntpoff() {
        let mut map = timer_map::default(); let mut val: u64; let offset: u64;
        get_timer_map(vcpu, &mut map); val = read_sysreg_el0(SYS_CNTP_CVAL);
        if map.direct_ptimer == vcpu_ptimer(vcpu) { __vcpu_assign_sys_reg(vcpu, CNTP_CVAL_EL0, val); }
        if map.direct_ptimer == vcpu_hptimer(vcpu) { __vcpu_assign_sys_reg(vcpu, CNTHP_CVAL_EL2, val); }
        offset = read_sysreg_s(SYS_CNTPOFF_EL2);
        if map.direct_ptimer && offset != 0 { write_sysreg_el0(val.wrapping_add(offset), SYS_CNTP_CVAL); isb(); }
    }
    asm!("nop");
    __deactivate_cptr_traps(vcpu);
    if !arm64_kernel_unmapped_at_el0() { host_vectors = this_cpu_read(this_cpu_vector); }
    write_sysreg(host_vectors, vbar_el1);
}

unsafe fn __vcpu_load_activate_traps(vcpu: *mut kvm_vcpu) { let mut flags: c_ulong = 0; local_irq_save(&mut flags); __activate_traps_common(vcpu); local_irq_restore(flags); }
unsafe fn __vcpu_put_deactivate_traps(vcpu: *mut kvm_vcpu) { let mut flags: c_ulong = 0; local_irq_save(&mut flags); __deactivate_traps_common(vcpu); local_irq_restore(flags); }

pub unsafe fn kvm_vcpu_load_vhe(vcpu: *mut kvm_vcpu) { host_data_ptr(host_ctxt).__hyp_running_vcpu = vcpu; __vcpu_load_switch_sysregs(vcpu); __vcpu_load_activate_traps(vcpu); __load_stage2((*vcpu).arch.hw_mmu); }
pub unsafe fn kvm_vcpu_put_vhe(vcpu: *mut kvm_vcpu) { __vcpu_put_deactivate_traps(vcpu); __vcpu_put_switch_sysregs(vcpu); host_data_ptr(host_ctxt).__hyp_running_vcpu = core::ptr::null_mut(); }

unsafe fn compute_emulated_cntx_ctl_el0(vcpu: *mut kvm_vcpu, reg: vcpu_sysreg) -> c_ulong {
    let (cval, ctl, cnt) = match reg { CNTP_CTL_EL0 => (__vcpu_sys_reg(vcpu,CNTP_CVAL_EL0), __vcpu_sys_reg(vcpu,CNTP_CTL_EL0), compute_counter_value(vcpu_ptimer(vcpu))), CNTV_CTL_EL0 => (__vcpu_sys_reg(vcpu,CNTV_CVAL_EL0), __vcpu_sys_reg(vcpu,CNTV_CTL_EL0), compute_counter_value(vcpu_vtimer(vcpu))), _ => { BUG(); (0,0,0) } };
    let mut ctl = ctl; __assign_bit(__ffs(ARCH_TIMER_CTRL_IT_STAT), &mut ctl, cval <= cnt); ctl
}

unsafe fn kvm_hyp_handle_timer(vcpu: *mut kvm_vcpu, _exit_code: *mut u64) -> bool {
    if !is_hyp_ctxt(vcpu) { return false; } let esr = kvm_vcpu_get_esr(vcpu); if esr & ESR_ELx_SYS64_ISS_DIR_MASK != ESR_ELx_SYS64_ISS_DIR_READ { return false; }
    let val = match esr_sys64_to_sysreg(esr) {
        SYS_CNTP_CTL_EL02 => compute_emulated_cntx_ctl_el0(vcpu,CNTP_CTL_EL0) as u64,
        SYS_CNTP_CTL_EL0 => if vcpu_el2_e2h_is_set(vcpu) { read_sysreg_el0(SYS_CNTP_CTL) } else { compute_emulated_cntx_ctl_el0(vcpu,CNTP_CTL_EL0) as u64 },
        SYS_CNTP_CVAL_EL02 => __vcpu_sys_reg(vcpu,CNTP_CVAL_EL0),
        SYS_CNTP_CVAL_EL0 => if vcpu_el2_e2h_is_set(vcpu) { let mut x=read_sysreg_el0(SYS_CNTP_CVAL); if !has_cntpoff(){x=x.wrapping_sub(timer_get_offset(vcpu_hptimer(vcpu)));} x } else { __vcpu_sys_reg(vcpu,CNTP_CVAL_EL0) },
        SYS_CNTPCT_EL0 | SYS_CNTPCTSS_EL0 => compute_counter_value(vcpu_hptimer(vcpu)),
        SYS_CNTV_CTL_EL02 => compute_emulated_cntx_ctl_el0(vcpu,CNTV_CTL_EL0) as u64,
        SYS_CNTV_CTL_EL0 => if vcpu_el2_e2h_is_set(vcpu){read_sysreg_el0(SYS_CNTV_CTL)}else{compute_emulated_cntx_ctl_el0(vcpu,CNTV_CTL_EL0) as u64},
        SYS_CNTV_CVAL_EL02 => __vcpu_sys_reg(vcpu,CNTV_CVAL_EL0),
        SYS_CNTV_CVAL_EL0 => if vcpu_el2_e2h_is_set(vcpu){read_sysreg_el0(SYS_CNTV_CVAL)}else{__vcpu_sys_reg(vcpu,CNTV_CVAL_EL0)},
        SYS_CNTVCT_EL0 | SYS_CNTVCTSS_EL0 => compute_counter_value(vcpu_hvtimer(vcpu)), _ => return false,
    }; vcpu_set_reg(vcpu,kvm_vcpu_sys_get_rt(vcpu),val); __kvm_skip_instr(vcpu); true
}

unsafe fn kvm_hyp_handle_eret(vcpu: *mut kvm_vcpu, _exit_code: *mut u64) -> bool {
    let esr=kvm_vcpu_get_esr(vcpu); if cpus_have_final_cap(ARM64_HAS_NV3)&&vcpu_el2_e2h_is_set(vcpu)||is_nested_ctxt(vcpu){return false;}
    let mut spsr=read_sysreg_el1(SYS_SPSR); let mut mode=spsr&(PSR_MODE_MASK|PSR_MODE32_BIT);
    match mode { PSR_MODE_EL0t=>if !(vcpu_el2_e2h_is_set(vcpu)&&vcpu_el2_tge_is_set(vcpu)){return false;}, PSR_MODE_EL2t=>mode=PSR_MODE_EL1t, PSR_MODE_EL2h=>mode=PSR_MODE_EL1h, _=>return false }
    let elr=if esr_iss_is_eretax(esr){let mut x=0;if !(vcpu_has_ptrauth(vcpu)&&kvm_auth_eretax(vcpu,&mut x)){return false;}x}else{read_sysreg_el1(SYS_ELR)};
    spsr=(spsr&!(PSR_MODE_MASK|PSR_MODE32_BIT))|mode; write_sysreg_el2(spsr,SYS_SPSR); write_sysreg_el2(elr,SYS_ELR); true
}

unsafe fn kvm_hyp_handle_tlbi_el2(vcpu:*mut kvm_vcpu,_:*mut u64)->bool { if !is_hyp_ctxt(vcpu){return false;} let instr=esr_sys64_to_sysreg(kvm_vcpu_get_esr(vcpu));let val=vcpu_get_reg(vcpu,kvm_vcpu_sys_get_rt(vcpu));let ok=(kvm_supported_tlbi_s1e1_op(vcpu,instr)&&vcpu_el2_e2h_is_set(vcpu)&&vcpu_el2_tge_is_set(vcpu))||kvm_supported_tlbi_s1e2_op(vcpu,instr);if !ok||__kvm_tlbi_s1e2(core::ptr::null_mut(),val,instr)!=0{return false;}if vcpu_el2_e2h_is_set(vcpu)&&vcpu_el2_tge_is_set(vcpu)&&atomic_read(&(*(*vcpu).kvm).arch.vncr_tlb_count)!=0{return false;}__kvm_skip_instr(vcpu);true }

unsafe fn kvm_hyp_handle_cpacr_el1(vcpu:*mut kvm_vcpu,_:*mut u64)->bool { let esr=kvm_vcpu_get_esr(vcpu);if cpus_have_final_cap(ARM64_HAS_NV2P1)||!is_hyp_ctxt(vcpu)||esr_sys64_to_sysreg(esr)!=SYS_CPACR_EL1{return false;}let rt=kvm_vcpu_sys_get_rt(vcpu);if esr&ESR_ELx_SYS64_ISS_DIR_MASK==ESR_ELx_SYS64_ISS_DIR_READ{vcpu_set_reg(vcpu,rt,__vcpu_sys_reg(vcpu,CPTR_EL2));}else{vcpu_write_sys_reg(vcpu,vcpu_get_reg(vcpu,rt),CPTR_EL2);__activate_cptr_traps(vcpu);}__kvm_skip_instr(vcpu);true }

unsafe fn kvm_hyp_handle_zcr_el2(vcpu:*mut kvm_vcpu,exit_code:*mut u64)->bool {if !vcpu_has_nv(vcpu)||esr_sys64_to_sysreg(kvm_vcpu_get_esr(vcpu))!=SYS_ZCR_EL2||guest_owns_fp_regs(){return false;}kvm_hyp_handle_fpsimd(vcpu,exit_code);false}
unsafe fn kvm_hyp_handle_sysreg_vhe(vcpu:*mut kvm_vcpu,e:*mut u64)->bool {kvm_hyp_handle_tlbi_el2(vcpu,e)||kvm_hyp_handle_timer(vcpu,e)||kvm_hyp_handle_cpacr_el1(vcpu,e)||kvm_hyp_handle_zcr_el2(vcpu,e)||kvm_hyp_handle_sysreg(vcpu,e)}

unsafe fn kvm_hyp_handle_impdef(vcpu:*mut kvm_vcpu,_:*mut u64)->bool {if !cpus_have_final_cap(ARM64_WORKAROUND_PMUV3_IMPDEF_TRAPS){return false;}let iss=ESR_ELx_ISS(read_sysreg_s(SYS_AFSR1_EL2));(*vcpu).arch.fault.esr_el2=FIELD_PREP(ESR_ELx_EC_MASK,ESR_ELx_EC_SYS64)|FIELD_PREP(ESR_ELx_ISS_MASK,iss)|ESR_ELx_IL;false}

// Indexed C handler table; supplied handler type and symbols are external dependencies.
static hyp_exit_handlers: [Option<exit_handler_fn>; ESR_ELx_EC_MAX as usize + 1] = [None; ESR_ELx_EC_MAX as usize + 1];

unsafe fn fixup_nv_guest_exit(vcpu:*mut kvm_vcpu) {if unlikely(host_data_test_flag(VCPU_IN_HYP_CONTEXT)){let mut mode=*vcpu_cpsr(vcpu)&(PSR_MODE_MASK|PSR_MODE32_BIT);match mode{PSR_MODE_EL1t=>mode=PSR_MODE_EL2t,PSR_MODE_EL1h=>mode=PSR_MODE_EL2h,_=>{}};*vcpu_cpsr(vcpu)&=!(PSR_MODE_MASK|PSR_MODE32_BIT);*vcpu_cpsr(vcpu)|=mode;let hcr=if cpus_have_final_cap(ARM64_HAS_NV3)&&vcpu_el2_e2h_is_set(vcpu){read_sysreg_s(SYS_NVHCR_EL2)}else{__vcpu_sys_reg(vcpu,NVHCR_EL2)};__vcpu_assign_sys_reg(vcpu,HCR_EL2,hcr);}BUG_ON((host_data_test_flag(VCPU_IN_HYP_CONTEXT)!=0)!=is_hyp_ctxt(vcpu));}
unsafe fn fixup_guest_exit(vcpu:*mut kvm_vcpu,e:*mut u64)->bool{synchronize_vcpu_pstate(vcpu);if vcpu_has_nv(vcpu){fixup_nv_guest_exit(vcpu);}__fixup_guest_exit(vcpu,e,hyp_exit_handlers.as_ptr())}

unsafe fn __kvm_vcpu_run_vhe(vcpu:*mut kvm_vcpu)->u64 {let host_ctxt=host_data_ptr(host_ctxt);let guest_ctxt=&mut (*vcpu).arch.ctxt;let mut exit_code;fpsimd_lazy_switch_to_guest(vcpu);sysreg_save_host_state_vhe(host_ctxt);__activate_traps(vcpu);__kvm_adjust_pc(vcpu);sysreg_restore_guest_state_vhe(guest_ctxt);__debug_switch_to_guest(vcpu);loop{exit_code=__guest_enter(vcpu);if !fixup_guest_exit(vcpu,&mut exit_code){break;}}sysreg_save_guest_state_vhe(guest_ctxt);__deactivate_traps(vcpu);sysreg_restore_host_state_vhe(host_ctxt);__debug_switch_to_host(vcpu);isb();fpsimd_lazy_switch_to_host(vcpu);if guest_owns_fp_regs(){__fpsimd_save_fpexc32(vcpu);}exit_code}
pub unsafe fn __kvm_vcpu_run(vcpu:*mut kvm_vcpu)->c_int{local_daif_mask();pmr_sync();let ret=__kvm_vcpu_run_vhe(vcpu);local_daif_restore(DAIF_PROCCTX_NOIRQ);ret as c_int}
unsafe fn __hyp_call_panic(spsr:u64,elr:u64,par:u64)->!{let host_ctxt=host_data_ptr(host_ctxt);let vcpu=host_ctxt.__hyp_running_vcpu;if !vcpu.is_null(){__deactivate_traps(vcpu);}sysreg_restore_host_state_vhe(host_ctxt);panic!("HYP panic: spsr={:08x} pc={:016x} esr={:016x} far={:016x} hpfar={:016x} par={:016x} vcpu={:p}",spsr,elr,read_sysreg_el2(SYS_ESR),read_sysreg_el2(SYS_FAR),read_sysreg(hpfar_el2),par,vcpu)}
pub unsafe fn hyp_panic()->!{__hyp_call_panic(read_sysreg_el2(SYS_SPSR),read_sysreg_el2(SYS_ELR),read_sysreg_par())}
pub unsafe fn kvm_unexpected_el2_exception(){__kvm_unexpected_el2_exception();}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
