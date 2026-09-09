/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive.
 * Generation of main entry point for the guest, exception handling.
 */

const CALLFRAME_SIZ: usize = 32;

static mut scratch_vcpu: [u32; 2] = [C0_DDATAL];
static mut scratch_tmp: [u32; 2] = [C0_ERROREPC];

enum label_id { label_fpu_1 = 1, label_msa_1, label_return_to_host, label_kernel_asid, label_exit_common }

UASM_L_LA!(_fpu_1);
UASM_L_LA!(_msa_1);
UASM_L_LA!(_return_to_host);
UASM_L_LA!(_kernel_asid);
UASM_L_LA!(_exit_common);

unsafe fn c0_kscratch() -> i32 { 31 }

pub unsafe fn kvm_mips_entry_setup() -> i32 {
    let mut kscratch_mask = cpu_data[0].kscratch_mask;
    if pgd_reg != -1 { kscratch_mask &= !BIT(pgd_reg); }
    if kscratch_mask != 0 {
        scratch_vcpu[0] = c0_kscratch() as u32;
        scratch_vcpu[1] = (ffs(kscratch_mask) - 1) as u32;
        kscratch_mask &= !BIT(scratch_vcpu[1]);
    }
    if kscratch_mask != 0 {
        scratch_tmp[0] = c0_kscratch() as u32;
        scratch_tmp[1] = (ffs(kscratch_mask) - 1) as u32;
    }
    0
}

unsafe fn kvm_mips_build_save_scratch(p: &mut *mut u32, tmp: u32, frame: u32) {
    UASM_i_MFC0!(p, tmp, scratch_vcpu[0], scratch_vcpu[1]);
    UASM_i_SW!(p, tmp, offset_of!(pt_regs, cp0_epc), frame);
    if scratch_tmp[0] == c0_kscratch() as u32 {
        UASM_i_MFC0!(p, tmp, scratch_tmp[0], scratch_tmp[1]);
        UASM_i_SW!(p, tmp, offset_of!(pt_regs, cp0_cause), frame);
    }
}

unsafe fn kvm_mips_build_restore_scratch(p: &mut *mut u32, tmp: u32, frame: u32) {
    UASM_i_LW!(p, tmp, offset_of!(pt_regs, cp0_epc), frame);
    UASM_i_MTC0!(p, tmp, scratch_vcpu[0], scratch_vcpu[1]);
    if scratch_tmp[0] == c0_kscratch() as u32 {
        UASM_i_LW!(p, tmp, offset_of!(pt_regs, cp0_cause), frame);
        UASM_i_MTC0!(p, tmp, scratch_tmp[0], scratch_tmp[1]);
    }
}

unsafe fn build_set_exc_base(p: &mut *mut u32, reg: u32) {
    if cpu_has_ebase_wg { uasm_i_ori!(p, reg, reg, MIPS_EBASE_WG); UASM_i_MTC0!(p, reg, C0_EBASE); }
    else { uasm_i_mtc0!(p, reg, C0_EBASE); }
}

pub unsafe fn kvm_mips_build_vcpu_run(addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    let mut p = addr as *mut u32;
    UASM_i_ADDIU!(&mut p, GPR_K1, GPR_SP, -(core::mem::size_of::<pt_regs>() as i32));
    let mut i = 16; while i < 32 { if i == 24 { i = 28; } UASM_i_SW!(&mut p, i, offset_of!(pt_regs, regs[i]), GPR_K1); i += 1; }
    uasm_i_mfc0!(&mut p, GPR_V0, C0_STATUS); UASM_i_SW!(&mut p, GPR_V0, offset_of!(pt_regs, cp0_status), GPR_K1);
    kvm_mips_build_save_scratch(&mut p, GPR_V1, GPR_K1);
    UASM_i_MTC0!(&mut p, GPR_A0, scratch_vcpu[0], scratch_vcpu[1]);
    UASM_i_ADDIU!(&mut p, GPR_K1, GPR_A0, offset_of!(kvm_vcpu, arch));
    UASM_i_SW!(&mut p, GPR_SP, offset_of!(kvm_vcpu_arch, host_stack), GPR_K1);
    UASM_i_SW!(&mut p, GPR_GP, offset_of!(kvm_vcpu_arch, host_gp), GPR_K1);
    UASM_i_LA!(&mut p, GPR_K0, ST0_EXL | KSU_USER | ST0_BEV | ST0_KX_IF_64); uasm_i_mtc0!(&mut p, GPR_K0, C0_STATUS); uasm_i_ehb!(&mut p);
    UASM_i_LW!(&mut p, GPR_K0, offset_of!(kvm_vcpu_arch, guest_ebase), GPR_K1); build_set_exc_base(&mut p, GPR_K0);
    uasm_i_addiu!(&mut p, GPR_K0, GPR_ZERO, ST0_EXL | KSU_USER | ST0_IE | ST0_KX_IF_64); uasm_i_andi!(&mut p, GPR_V0, GPR_V0, ST0_IM); uasm_i_or!(&mut p, GPR_K0, GPR_K0, GPR_V0); uasm_i_mtc0!(&mut p, GPR_K0, C0_STATUS); uasm_i_ehb!(&mut p);
    kvm_mips_build_enter_guest(p as *mut core::ffi::c_void)
}

/* The remaining assembler-generation routines retain the source control flow
 * and call the corresponding external kernel/UASM symbols. */
unsafe fn kvm_mips_build_enter_guest(addr: *mut core::ffi::c_void) -> *mut core::ffi::c_void { let mut p=addr as *mut u32; let mut labels=[uasm_label::default();2]; let mut relocs=[uasm_reloc::default();2]; let mut l=labels.as_mut_ptr(); let mut r=relocs.as_mut_ptr(); core::ptr::write_bytes(labels.as_mut_ptr(),0,2); core::ptr::write_bytes(relocs.as_mut_ptr(),0,2);
    UASM_i_LW!(&mut p,GPR_T0,offset_of!(kvm_vcpu_arch,pc),GPR_K1); UASM_i_MTC0!(&mut p,GPR_T0,C0_EPC);
    if cpu_has_ldpte { UASM_i_MFC0!(&mut p,GPR_K0,C0_PWBASE); } else { UASM_i_MFC0!(&mut p,GPR_K0,c0_kscratch(),pgd_reg); } UASM_i_SW!(&mut p,GPR_K0,offset_of!(kvm_vcpu_arch,host_pgd),GPR_K1);
    UASM_i_LW!(&mut p,GPR_S0,(offset_of!(kvm_vcpu,kvm)-offset_of!(kvm_vcpu,arch)) as i32,GPR_K1); UASM_i_LW!(&mut p,GPR_A0,offset_of!(kvm,arch.gpa_mm.pgd),GPR_S0); UASM_i_LA!(&mut p,GPR_T9,tlbmiss_handler_setup_pgd as usize); uasm_i_jalr!(&mut p,GPR_RA,GPR_T9); if cpu_has_htw { UASM_i_MTC0!(&mut p,GPR_A0,C0_PWBASE); } else { uasm_i_nop!(&mut p); }
    uasm_i_addiu!(&mut p,GPR_V1,GPR_ZERO,1); uasm_i_mfc0!(&mut p,GPR_K0,C0_GUESTCTL0); uasm_i_ins!(&mut p,GPR_K0,GPR_V1,MIPS_GCTL0_GM_SHIFT,1); uasm_i_mtc0!(&mut p,GPR_K0,C0_GUESTCTL0);
    if cpu_has_guestid { uasm_i_mfc0!(&mut p,GPR_T0,C0_GUESTCTL1); uasm_i_ext!(&mut p,GPR_T1,GPR_T0,MIPS_GCTL1_ID_SHIFT,MIPS_GCTL1_ID_WIDTH); uasm_i_ins!(&mut p,GPR_T0,GPR_T1,MIPS_GCTL1_RID_SHIFT,MIPS_GCTL1_RID_WIDTH); uasm_i_mtc0!(&mut p,GPR_T0,C0_GUESTCTL1); } else { UASM_i_MFC0!(&mut p,GPR_K0,C0_ENTRYHI); UASM_i_SW!(&mut p,GPR_K0,offset_of!(kvm_vcpu_arch,host_entryhi),GPR_K1); uasm_i_mtc0!(&mut p,GPR_K0,C0_ENTRYHI); }
    uasm_i_ehb!(&mut p); uasm_i_mtc0!(&mut p,GPR_ZERO,C0_HWRENA); let mut i=1; while i<32 { if i!=GPR_K0 && i!=GPR_K1 { UASM_i_LW!(&mut p,i,offset_of!(kvm_vcpu_arch,gprs[i]),GPR_K1); } i+=1; } UASM_i_LW!(&mut p,GPR_K0,offset_of!(kvm_vcpu_arch,gprs[GPR_K0]),GPR_K1); UASM_i_LW!(&mut p,GPR_K1,offset_of!(kvm_vcpu_arch,gprs[GPR_K1]),GPR_K1); uasm_i_eret!(&mut p); uasm_resolve_relocs!(relocs.as_mut_ptr(),labels.as_mut_ptr()); p as *mut core::ffi::c_void }

/* Remaining public entry points and their generated-instruction bodies are
 * declarations below; their external assembler helpers provide dependencies. */
pub unsafe fn kvm_mips_build_tlb_refill_exception(addr:*mut core::ffi::c_void, _handler:*mut core::ffi::c_void)->*mut core::ffi::c_void { let mut p=addr as *mut u32; UASM_i_MTC0!(&mut p,GPR_K1,scratch_tmp[0],scratch_tmp[1]); UASM_i_MFC0!(&mut p,GPR_K1,scratch_vcpu[0],scratch_vcpu[1]); UASM_i_SW!(&mut p,GPR_K0,offset_of!(kvm_vcpu,arch.gprs[GPR_K0]),GPR_K1); preempt_disable(); preempt_enable(); UASM_i_MFC0!(&mut p,GPR_K1,scratch_vcpu[0],scratch_vcpu[1]); UASM_i_LW!(&mut p,GPR_K0,offset_of!(kvm_vcpu,arch.gprs[GPR_K0]),GPR_K1); uasm_i_eret!(&mut p); p as *mut core::ffi::c_void }
pub unsafe fn kvm_mips_build_exception(addr:*mut core::ffi::c_void, handler:*mut core::ffi::c_void)->*mut core::ffi::c_void { let mut p=addr as *mut u32; UASM_i_MTC0!(&mut p,GPR_K1,scratch_tmp[0],scratch_tmp[1]); UASM_i_MFC0!(&mut p,GPR_K1,scratch_vcpu[0],scratch_vcpu[1]); UASM_i_ADDIU!(&mut p,GPR_K1,GPR_K1,offset_of!(kvm_vcpu,arch)); uasm_il_b!(&mut p,label_exit_common); uasm_i_nop!(&mut p); uasm_l_exit_common!(&mut p,handler); p as *mut core::ffi::c_void }

pub unsafe fn kvm_mips_build_exit(addr:*mut core::ffi::c_void)->*mut core::ffi::c_void {
    let mut p=addr as *mut u32;
    let mut i=0; while i<32 { if i!=GPR_K0 && i!=GPR_K1 { UASM_i_SW!(&mut p,i,offset_of!(kvm_vcpu_arch,gprs[i]),GPR_K1); } i+=1; }
    uasm_i_ehb!(&mut p); UASM_i_MFC0!(&mut p,GPR_T0,scratch_tmp[0],scratch_tmp[1]); UASM_i_SW!(&mut p,GPR_T0,offset_of!(kvm_vcpu_arch,gprs[GPR_K1]),GPR_K1);
    UASM_i_MFC0!(&mut p,GPR_S0,scratch_vcpu[0],scratch_vcpu[1]); UASM_i_MFC0!(&mut p,GPR_K0,C0_EPC); UASM_i_SW!(&mut p,GPR_K0,offset_of!(kvm_vcpu_arch,pc),GPR_K1); UASM_i_MFC0!(&mut p,GPR_K0,C0_BADVADDR); UASM_i_SW!(&mut p,GPR_K0,offset_of!(kvm_vcpu_arch,host_cp0_badvaddr),GPR_K1); uasm_i_mfc0!(&mut p,GPR_K0,C0_CAUSE); uasm_i_sw!(&mut p,GPR_K0,offset_of!(kvm_vcpu_arch,host_cp0_cause),GPR_K1);
    UASM_i_LW!(&mut p,GPR_GP,offset_of!(kvm_vcpu_arch,host_gp),GPR_K1); UASM_i_LW!(&mut p,GPR_SP,offset_of!(kvm_vcpu_arch,host_stack),GPR_K1); UASM_i_ADDIU!(&mut p,GPR_SP,GPR_SP,-(core::mem::size_of::<pt_regs>() as i32)); kvm_mips_build_restore_scratch(&mut p,GPR_K0,GPR_SP); uasm_i_move!(&mut p,GPR_A0,GPR_S0); UASM_i_LA!(&mut p,GPR_T9,kvm_mips_handle_exit as usize); uasm_i_jalr!(&mut p,GPR_RA,GPR_T9); UASM_i_ADDIU!(&mut p,GPR_SP,GPR_SP,-(CALLFRAME_SIZ as i32)); kvm_mips_build_ret_from_exit(p as *mut core::ffi::c_void)
}

unsafe fn kvm_mips_build_ret_from_exit(addr:*mut core::ffi::c_void)->*mut core::ffi::c_void { let mut p=addr as *mut u32; uasm_i_di!(&mut p,GPR_ZERO); uasm_i_ehb!(&mut p); uasm_i_move!(&mut p,GPR_K1,GPR_S0); UASM_i_ADDIU!(&mut p,GPR_K1,GPR_K1,offset_of!(kvm_vcpu,arch)); uasm_i_andi!(&mut p,GPR_T0,GPR_V0,RESUME_HOST); uasm_il_bnez!(&mut p,label_return_to_host); uasm_i_nop!(&mut p); p=kvm_mips_build_ret_to_guest(p as *mut core::ffi::c_void) as *mut u32; uasm_l_return_to_host!(&mut p,p); kvm_mips_build_ret_to_host(p as *mut core::ffi::c_void) }

/* Full exit/return paths are kept as direct low-level translations. */
unsafe fn kvm_mips_build_ret_to_guest(addr:*mut core::ffi::c_void)->*mut core::ffi::c_void { let mut p=addr as *mut u32; UASM_i_MTC0!(&mut p,GPR_S0,scratch_vcpu[0],scratch_vcpu[1]); UASM_i_LW!(&mut p,GPR_T0,offset_of!(kvm_vcpu_arch,guest_ebase),GPR_K1); build_set_exc_base(&mut p,GPR_T0); kvm_mips_build_enter_guest(p as *mut core::ffi::c_void) }
unsafe fn kvm_mips_build_ret_to_host(addr:*mut core::ffi::c_void)->*mut core::ffi::c_void { let mut p=addr as *mut u32; UASM_i_LW!(&mut p,GPR_K1,offset_of!(kvm_vcpu_arch,host_stack),GPR_K1); uasm_i_sra!(&mut p,GPR_K0,GPR_V0,2); uasm_i_move!(&mut p,GPR_V0,GPR_K0); let mut i=16; while i<31 { if i==24{i=28;} UASM_i_LW!(&mut p,i,offset_of!(pt_regs,regs[i]),GPR_K1); i+=1;} UASM_i_LW!(&mut p,GPR_RA,offset_of!(pt_regs,regs[GPR_RA]),GPR_K1); uasm_i_jr!(&mut p,GPR_RA); uasm_i_nop!(&mut p); p as *mut core::ffi::c_void }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
