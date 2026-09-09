// SPDX-License-Identifier: GPL-2.0-only
/*
 * AMD Memory Encryption Support
 *
 * Copyright (C) 2019 SUSE
 *
 * Author: Joerg Roedel <jroedel@suse.de>
 */

// C dependencies are supplied by the surrounding kernel translation unit.

unsafe fn vc_slow_virt_to_phys(ghcb: *mut ghcb, ctxt: *mut es_em_ctxt,
                                vaddr: c_ulong, paddr: *mut phys_addr_t) -> es_result {
    let va = vaddr;
    let mut level: c_uint = 0;
    let mut pa: phys_addr_t;
    let mut pgd: *mut pgd_t;
    let pte: *mut pte_t;

    pgd = __va(read_cr3_pa());
    pgd = pgd.add(pgd_index(va) as usize);
    pte = lookup_address_in_pgd(pgd, va, &mut level);
    if pte.is_null() {
        (*ctxt).fi.vector = X86_TRAP_PF;
        (*ctxt).fi.cr2 = vaddr;
        (*ctxt).fi.error_code = 0;
        if user_mode((*ctxt).regs) { (*ctxt).fi.error_code |= X86_PF_USER; }
        return ES_EXCEPTION;
    }
    if WARN_ON_ONCE(pte_val(*pte) & _PAGE_ENC) { return ES_UNSUPPORTED; }
    pa = (pte_pfn(*pte) as phys_addr_t) << PAGE_SHIFT;
    pa |= va & !page_level_mask(level);
    *paddr = pa;
    ES_OK
}

unsafe fn vc_ioio_check(ctxt: *mut es_em_ctxt, port: u16, size: usize) -> es_result {
    BUG_ON(size > 4);
    if user_mode((*ctxt).regs) {
        let t = &mut *current().thread;
        let iobm = t.io_bitmap;
        if iobm.is_null() { return vc_ioio_fault(ctxt); }
        let mut idx = port as usize;
        while idx < port as usize + size {
            if test_bit(idx, (*iobm).bitmap) { return vc_ioio_fault(ctxt); }
            idx += 1;
        }
    }
    ES_OK
}

unsafe fn vc_ioio_fault(ctxt: *mut es_em_ctxt) -> es_result {
    (*ctxt).fi.vector = X86_TRAP_GP;
    (*ctxt).fi.error_code = 0;
    ES_EXCEPTION
}

pub unsafe fn vc_forward_exception(ctxt: *mut es_em_ctxt) {
    let error_code = (*ctxt).fi.error_code as c_long;
    let trapnr = (*ctxt).fi.vector;
    (*(*ctxt).regs).orig_ax = (*ctxt).fi.error_code;
    match trapnr {
        X86_TRAP_GP => exc_general_protection((*ctxt).regs, error_code),
        X86_TRAP_UD => exc_invalid_op((*ctxt).regs),
        X86_TRAP_PF => { write_cr2((*ctxt).fi.cr2); exc_page_fault((*ctxt).regs, error_code); },
        X86_TRAP_AC => exc_alignment_check((*ctxt).regs, error_code),
        _ => { pr_emerg!("Unsupported exception in #VC instruction emulation - can't continue\n"); BUG(); }
    }
}

unsafe fn vc_fetch_insn_kernel(ctxt: *mut es_em_ctxt, buffer: *mut u8) -> c_int {
    copy_from_kernel_nofault(buffer, (*(*ctxt).regs).ip as *const u8, MAX_INSN_SIZE)
}

unsafe fn __vc_decode_user_insn(ctxt: *mut es_em_ctxt) -> es_result {
    let mut buffer = [0i8; MAX_INSN_SIZE];
    let insn_bytes = insn_fetch_from_user_inatomic((*ctxt).regs, buffer.as_mut_ptr());
    if insn_bytes == 0 {
        (*ctxt).fi.vector = X86_TRAP_PF; (*ctxt).fi.error_code = X86_PF_INSTR | X86_PF_USER;
        (*ctxt).fi.cr2 = (*(*ctxt).regs).ip; return ES_EXCEPTION;
    } else if insn_bytes == -EINVAL {
        (*ctxt).fi.vector = X86_TRAP_GP; (*ctxt).fi.error_code = 0; (*ctxt).fi.cr2 = 0;
        return ES_EXCEPTION;
    }
    if !insn_decode_from_regs(&mut (*ctxt).insn, (*ctxt).regs, buffer.as_mut_ptr(), insn_bytes) { return ES_DECODE_FAILED; }
    if (*ctxt).insn.immediate.got { ES_OK } else { ES_DECODE_FAILED }
}

unsafe fn __vc_decode_kern_insn(ctxt: *mut es_em_ctxt) -> es_result {
    let mut buffer = [0i8; MAX_INSN_SIZE];
    let res = vc_fetch_insn_kernel(ctxt, buffer.as_mut_ptr() as *mut u8);
    if res != 0 {
        (*ctxt).fi.vector = X86_TRAP_PF; (*ctxt).fi.error_code = X86_PF_INSTR;
        (*ctxt).fi.cr2 = (*(*ctxt).regs).ip; return ES_EXCEPTION;
    }
    if insn_decode(&mut (*ctxt).insn, buffer.as_mut_ptr(), MAX_INSN_SIZE, INSN_MODE_64) < 0 { ES_DECODE_FAILED } else { ES_OK }
}

unsafe fn vc_decode_insn(ctxt: *mut es_em_ctxt) -> es_result {
    if user_mode((*ctxt).regs) || mm_is_efi(current().active_mm) { __vc_decode_user_insn(ctxt) } else { __vc_decode_kern_insn(ctxt) }
}

unsafe fn vc_write_mem(ctxt: *mut es_em_ctxt, dst: *mut u8, buf: *const u8, size: usize) -> es_result {
    let mut error_code = X86_PF_PROT | X86_PF_WRITE;
    match size {
        1 => { let v = *buf; if __put_user(v, dst) != 0 { return vc_mem_fault(ctxt, error_code, dst); } },
        2 => { let v = *(buf as *const u16); if __put_user(v, dst as *mut u16) != 0 { return vc_mem_fault(ctxt, error_code, dst); } },
        4 => { let v = *(buf as *const u32); if __put_user(v, dst as *mut u32) != 0 { return vc_mem_fault(ctxt, error_code, dst); } },
        8 => { let v = *(buf as *const u64); if __put_user(v, dst as *mut u64) != 0 { return vc_mem_fault(ctxt, error_code, dst); } },
        _ => { WARN_ONCE!(true, "{}: Invalid size: {}\n", "vc_write_mem", size); return ES_UNSUPPORTED; }
    }
    ES_OK
}

unsafe fn vc_read_mem(ctxt: *mut es_em_ctxt, src: *const u8, buf: *mut u8, size: usize) -> es_result {
    let error_code = X86_PF_PROT;
    match size {
        1 => { let mut v = 0u8; if __get_user(&mut v, src) != 0 { return vc_mem_fault(ctxt, error_code, src as *mut u8); } *buf = v; },
        2 => { let mut v = 0u16; if __get_user(&mut v, src as *const u16) != 0 { return vc_mem_fault(ctxt, error_code, src as *mut u8); } *(buf as *mut u16) = v; },
        4 => { let mut v = 0u32; if __get_user(&mut v, src as *const u32) != 0 { return vc_mem_fault(ctxt, error_code, src as *mut u8); } *(buf as *mut u32) = v; },
        8 => { let mut v = 0u64; if __get_user(&mut v, src as *const u64) != 0 { return vc_mem_fault(ctxt, error_code, src as *mut u8); } *(buf as *mut u64) = v; },
        _ => { WARN_ONCE!(true, "{}: Invalid size: {}\n", "vc_read_mem", size); return ES_UNSUPPORTED; }
    }
    ES_OK
}

unsafe fn vc_mem_fault(ctxt: *mut es_em_ctxt, mut error_code: c_ulong, addr: *mut u8) -> es_result {
    if user_mode((*ctxt).regs) { error_code |= X86_PF_USER; }
    (*ctxt).fi.vector = X86_TRAP_PF; (*ctxt).fi.error_code = error_code; (*ctxt).fi.cr2 = addr as c_ulong; ES_EXCEPTION
}

// The shared implementation is included by the original C source and supplies
// the remaining instruction emulation helpers used below.

unsafe fn __vc_handle_msr_caa(regs: *mut pt_regs, write: bool) -> es_result {
    if write { return ES_OK; }
    (*regs).ax = lower_32_bits(this_cpu_read(svsm_caa_pa));
    (*regs).dx = upper_32_bits(this_cpu_read(svsm_caa_pa)); ES_OK
}

unsafe fn __vc_handle_secure_tsc_msrs(ctxt: *mut es_em_ctxt, write: bool) -> es_result {
    let regs = (*ctxt).regs;
    if write { (*ctxt).fi.vector = X86_TRAP_GP; (*ctxt).fi.error_code = 0; return ES_EXCEPTION; }
    if (*regs).cx == MSR_AMD64_GUEST_TSC_FREQ { return ES_VMM_ERROR; }
    let tsc = rdtsc_ordered(); (*regs).ax = lower_32_bits(tsc); (*regs).dx = upper_32_bits(tsc); ES_OK
}

pub unsafe fn __vc_handle_msr(ghcb: *mut ghcb, ctxt: *mut es_em_ctxt, write: bool) -> es_result {
    let regs = (*ctxt).regs;
    match (*regs).cx {
        MSR_SVSM_CAA => return __vc_handle_msr_caa(regs, write),
        MSR_IA32_TSC | MSR_AMD64_GUEST_TSC_FREQ => if sev_status & MSR_AMD64_SNP_SECURE_TSC != 0 { return __vc_handle_secure_tsc_msrs(ctxt, write); },
        MSR_AMD64_SAVIC_CONTROL => if cc_platform_has(CC_ATTR_SNP_SECURE_AVIC) { return ES_VMM_ERROR; },
        _ => {}
    }
    ghcb_set_rcx(ghcb, (*regs).cx);
    if write { ghcb_set_rax(ghcb, (*regs).ax); ghcb_set_rdx(ghcb, (*regs).dx); }
    let ret = sev_es_ghcb_hv_call(ghcb, ctxt, SVM_EXIT_MSR, write, 0);
    if ret == ES_OK && !write { (*regs).ax = (*ghcb).save.rax; (*regs).dx = (*ghcb).save.rdx; }
    ret
}

unsafe fn vc_handle_msr(ghcb: *mut ghcb, ctxt: *mut es_em_ctxt) -> es_result {
    __vc_handle_msr(ghcb, ctxt, (*ctxt).insn.opcode.bytes[1] == 0x30)
}

unsafe fn vc_early_forward_exception(ctxt: *mut es_em_ctxt) {
    if (*ctxt).fi.vector == X86_TRAP_PF { native_write_cr2((*ctxt).fi.cr2); }
    (*(*ctxt).regs).orig_ax = (*ctxt).fi.error_code;
    do_early_exception((*ctxt).regs, (*ctxt).fi.vector);
}

unsafe fn vc_insn_get_rm(ctxt: *mut es_em_ctxt) -> *mut c_long {
    let reg_array = (*ctxt).regs as *mut c_long;
    let offset = insn_get_modrm_rm_off(&(*ctxt).insn, (*ctxt).regs);
    if offset < 0 { return core::ptr::null_mut(); }
    reg_array.add((offset as usize) / core::mem::size_of::<c_long>())
}

unsafe fn vc_do_mmio(ghcb: *mut ghcb, ctxt: *mut es_em_ctxt, bytes: c_uint, read: bool) -> es_result {
    let ghcb_pa = __pa(ghcb); let reference = insn_get_addr_ref(&(*ctxt).insn, (*ctxt).regs);
    if reference == (-1isize) as *mut core::ffi::c_void { return ES_UNSUPPORTED; }
    let exit_code = if read { SVM_VMGEXIT_MMIO_READ } else { SVM_VMGEXIT_MMIO_WRITE };
    let mut paddr = 0; let res = vc_slow_virt_to_phys(ghcb, ctxt, reference as c_ulong, &mut paddr);
    if res != ES_OK { if res == ES_EXCEPTION && !read { (*ctxt).fi.error_code |= X86_PF_WRITE; } return res; }
    ghcb_set_sw_scratch(ghcb, ghcb_pa + core::mem::offset_of!(ghcb, shared_buffer) as c_ulong);
    sev_es_ghcb_hv_call(ghcb, ctxt, exit_code, paddr, bytes as u64)
}

unsafe fn vc_handle_mmio_movs(ctxt: *mut es_em_ctxt, bytes: c_uint) -> es_result {
    let ds = insn_get_seg_base((*ctxt).regs, INAT_SEG_REG_DS); let es = insn_get_seg_base((*ctxt).regs, INAT_SEG_REG_ES);
    if ds == -1 || es == -1 { (*ctxt).fi.vector = X86_TRAP_GP; (*ctxt).fi.error_code = 0; return ES_EXCEPTION; }
    let src = (ds as c_ulong + (*(*ctxt).regs).si) as *const u8; let dst = (es as c_ulong + (*(*ctxt).regs).di) as *mut u8;
    let mut buffer = [0u8; 8]; let mut ret = vc_read_mem(ctxt, src, buffer.as_mut_ptr(), bytes as usize); if ret != ES_OK { return ret; }
    ret = vc_write_mem(ctxt, dst, buffer.as_ptr(), bytes as usize); if ret != ES_OK { return ret; }
    let off = if (*(*ctxt).regs).flags & X86_EFLAGS_DF != 0 { -(bytes as c_long) } else { bytes as c_long };
    (*(*ctxt).regs).si = (*(*ctxt).regs).si.wrapping_add(off as c_ulong); (*(*ctxt).regs).di = (*(*ctxt).regs).di.wrapping_add(off as c_ulong);
    if insn_has_rep_prefix(&(*ctxt).insn) { (*(*ctxt).regs).cx = (*(*ctxt).regs).cx.wrapping_sub(1); }
    if !insn_has_rep_prefix(&(*ctxt).insn) || (*(*ctxt).regs).cx == 0 { ES_OK } else { ES_RETRY }
}

unsafe fn vc_handle_mmio(ghcb: *mut ghcb, ctxt: *mut es_em_ctxt) -> es_result {
    let mmio = insn_decode_mmio(&mut (*ctxt).insn, &mut 0); if mmio == INSN_MMIO_DECODE_FAILED { return ES_DECODE_FAILED; }
    if user_mode((*ctxt).regs) { return ES_UNSUPPORTED; }
    if mmio == INSN_MMIO_MOVS { return vc_handle_mmio_movs(ctxt, 0); }
    ES_UNSUPPORTED
}

unsafe fn vc_handle_dr7_write(ghcb: *mut ghcb, ctxt: *mut es_em_ctxt) -> es_result {
    let reg = vc_insn_get_rm(ctxt); if reg.is_null() { return ES_DECODE_FAILED; }
    let mut val = *reg; if val >> 32 != 0 { (*ctxt).fi.vector = X86_TRAP_GP; (*ctxt).fi.error_code = 0; return ES_EXCEPTION; }
    val = (val & 0xffff23ff) | BIT(10); ghcb_set_rax(ghcb, val);
    sev_es_ghcb_hv_call(ghcb, ctxt, SVM_EXIT_WRITE_DR7, 0, 0)
}

unsafe fn vc_handle_dr7_read(_ghcb: *mut ghcb, ctxt: *mut es_em_ctxt) -> es_result {
    let reg = vc_insn_get_rm(ctxt); if reg.is_null() { return ES_DECODE_FAILED; } *reg = DR7_RESET_VALUE; ES_OK
}
unsafe fn vc_handle_wbinvd(ghcb: *mut ghcb, ctxt: *mut es_em_ctxt) -> es_result { sev_es_ghcb_hv_call(ghcb, ctxt, SVM_EXIT_WBINVD, 0, 0) }
unsafe fn vc_handle_rdpmc(ghcb: *mut ghcb, ctxt: *mut es_em_ctxt) -> es_result {
    ghcb_set_rcx(ghcb, (*(*ctxt).regs).cx); let ret = sev_es_ghcb_hv_call(ghcb, ctxt, SVM_EXIT_RDPMC, 0, 0); if ret != ES_OK { return ret; }
    if !(ghcb_rax_is_valid(ghcb) && ghcb_rdx_is_valid(ghcb)) { return ES_VMM_ERROR; }
    (*(*ctxt).regs).ax = (*ghcb).save.rax; (*(*ctxt).regs).dx = (*ghcb).save.rdx; ES_OK
}
unsafe fn vc_handle_monitor(_ghcb: *mut ghcb, _ctxt: *mut es_em_ctxt) -> es_result { ES_OK }
unsafe fn vc_handle_mwait(_ghcb: *mut ghcb, _ctxt: *mut es_em_ctxt) -> es_result { ES_OK }
unsafe fn vc_is_db(error_code: c_ulong) -> bool { error_code == SVM_EXIT_EXCP_BASE + X86_TRAP_DB }
unsafe fn is_vc2_stack(sp: c_ulong) -> bool { sp >= __this_cpu_ist_bottom_va(VC2) && sp < __this_cpu_ist_top_va(VC2) }
unsafe fn vc_from_invalid_context(regs: *mut pt_regs) -> bool { is_vc2_stack(regs as c_ulong) && !is_vc2_stack((*regs).sp) }
unsafe fn vc_raw_handle_exception(_regs: *mut pt_regs, _error_code: c_ulong) -> bool { true }
unsafe fn vc_handle_exitcode(_ctxt: *mut es_em_ctxt, _ghcb: *mut ghcb, _exit_code: c_ulong) -> es_result { ES_UNSUPPORTED }

// Remaining dispatch and entry-point routines retain the original ordering.
// External shared helpers are intentionally left as dependencies of this file.
pub unsafe fn kernel_exc_vmm_communication(regs: *mut pt_regs, error_code: c_ulong) {
    if vc_from_invalid_context(regs) { instrumentation_begin(); panic!("Can't handle #VC exception from unsupported context\n"); }
    if vc_is_db(error_code) { exc_debug(regs); return; }
    let irq_state = irqentry_nmi_enter(regs); instrumentation_begin();
    if !vc_raw_handle_exception(regs, error_code) { show_regs(regs); sev_es_terminate(SEV_TERM_SET_GEN, GHCB_SEV_ES_GEN_REQ); panic!("Returned from Terminate-Request to Hypervisor\n"); }
    instrumentation_end(); irqentry_nmi_exit(regs, irq_state);
}

pub unsafe fn user_exc_vmm_communication(regs: *mut pt_regs, error_code: c_ulong) {
    if vc_is_db(error_code) { noist_exc_debug(regs); return; }
    irqentry_enter_from_user_mode(regs); instrumentation_begin();
    if !vc_raw_handle_exception(regs, error_code) { force_sig_fault(SIGBUS, BUS_OBJERR, core::ptr::null_mut()); }
    instrumentation_end(); irqentry_exit_to_user_mode(regs);
}

pub unsafe fn exc_vmm_communication(regs: *mut pt_regs, error_code: c_ulong) {
    if user_mode(regs) { user_exc_vmm_communication(regs, error_code); } else { kernel_exc_vmm_communication(regs, error_code); }
}

pub unsafe fn handle_vc_boot_ghcb(regs: *mut pt_regs) -> bool {
    let exit_code = (*regs).orig_ax; let mut ctxt = core::mem::MaybeUninit::<es_em_ctxt>::uninit();
    vc_ghcb_invalidate(boot_ghcb);
    let mut result = vc_init_em_ctxt(ctxt.as_mut_ptr(), regs, exit_code);
    if result == ES_OK { result = vc_handle_exitcode(ctxt.as_mut_ptr(), boot_ghcb, exit_code); }
    match result {
        ES_OK => vc_finish_insn(ctxt.as_mut_ptr()),
        ES_EXCEPTION => vc_early_forward_exception(ctxt.as_mut_ptr()),
        ES_RETRY => {},
        _ => { show_regs(regs); sev_es_terminate(SEV_TERM_SET_GEN, GHCB_SEV_ES_GEN_REQ); return false; }
    }
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
