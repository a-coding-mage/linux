// SPDX-License-Identifier: GPL-2.0
/*
 * in-kernel handling for sie intercepts
 *
 * Copyright IBM Corp. 2008, 2020
 *
 *    Author(s): Carsten Otte <cotte@de.ibm.com>
 *               Christian Borntraeger <borntraeger@de.ibm.com>
 */

// External kernel and s390 dependencies are supplied by the surrounding translation.

pub unsafe fn kvm_s390_get_ilen(vcpu: *mut kvm_vcpu) -> u8 {
    let sie_block = (*vcpu).arch.sie_block;
    let mut ilen: u8 = 0;
    match (*vcpu).arch.sie_block.icptcode {
        ICPT_INST | ICPT_INSTPROGI | ICPT_OPEREXC | ICPT_PARTEXEC | ICPT_IOINST => {
            // instruction only stored for these icptcodes
            ilen = insn_length((*vcpu).arch.sie_block.ipa >> 8);
            // Use the length of the EXECUTE instruction if necessary
            if sie_block.icptstatus & 1 != 0 {
                ilen = ((sie_block.icptstatus >> 4) & 0x6) as u8;
                if ilen == 0 { ilen = 4; }
            }
        }
        ICPT_PROGI => {
            // bit 1+2 of pgmilc are the ilc, so we directly get ilen
            ilen = ((*vcpu).arch.sie_block.pgmilc & 0x6) as u8;
        }
        _ => {}
    }
    ilen
}

unsafe fn handle_stop(vcpu: *mut kvm_vcpu) -> i32 {
    let li = &mut (*vcpu).arch.local_int;
    let mut rc = 0;
    (*vcpu).stat.exit_stop_request += 1;
    if kvm_s390_vcpu_has_irq(vcpu, 1) != 0 { return 0; }
    spin_lock(&mut li.lock);
    let flags = li.irq.stop.flags;
    let stop_pending = kvm_s390_is_stop_irq_pending(vcpu);
    spin_unlock(&mut li.lock);
    trace_kvm_s390_stop_request(stop_pending, flags);
    if stop_pending == 0 { return 0; }
    if flags & KVM_S390_STOP_FLAG_STORE_STATUS != 0 {
        rc = kvm_s390_vcpu_store_status(vcpu, KVM_S390_STORE_STATUS_NOADDR);
        if rc != 0 { return rc; }
    }
    // no need to check the return value of vcpu_stop as it can only have an error for protvirt
    if kvm_s390_user_cpu_state_ctrl((*vcpu).kvm) == 0 { kvm_s390_vcpu_stop(vcpu); }
    -EOPNOTSUPP
}

unsafe fn handle_validity(vcpu: *mut kvm_vcpu) -> i32 {
    let viwhy = ((*vcpu).arch.sie_block.ipb >> 16) as i32;
    (*vcpu).stat.exit_validity += 1;
    trace_kvm_s390_intercept_validity(vcpu, viwhy);
    // KVM_EVENT(3, "validity intercept 0x%x for pid %u (kvm 0x%p)", ...)
    WARN_ONCE(viwhy != 0x44, "kvm: unhandled validity intercept 0x%x\n", viwhy);
    -EINVAL
}

unsafe fn handle_instruction(vcpu: *mut kvm_vcpu) -> i32 {
    (*vcpu).stat.exit_instruction += 1;
    trace_kvm_s390_intercept_instruction(vcpu, (*vcpu).arch.sie_block.ipa, (*vcpu).arch.sie_block.ipb);
    match (*vcpu).arch.sie_block.ipa >> 8 {
        0x01 => kvm_s390_handle_01(vcpu), 0x82 => kvm_s390_handle_lpsw(vcpu),
        0x83 => kvm_s390_handle_diag(vcpu), 0xaa => kvm_s390_handle_aa(vcpu),
        0xae => kvm_s390_handle_sigp(vcpu), 0xb2 => kvm_s390_handle_b2(vcpu),
        0xb6 => kvm_s390_handle_stctl(vcpu), 0xb7 => kvm_s390_handle_lctl(vcpu),
        0xb9 => kvm_s390_handle_b9(vcpu), 0xe3 => kvm_s390_handle_e3(vcpu),
        0xe5 => kvm_s390_handle_e5(vcpu), 0xeb => kvm_s390_handle_eb(vcpu),
        _ => -EOPNOTSUPP,
    }
}

unsafe fn inject_prog_on_prog_intercept(vcpu: *mut kvm_vcpu) -> i32 {
    let mut pgm_info = kvm_s390_pgm_info { code: (*vcpu).arch.sie_block.iprcc, flags: KVM_S390_PGM_FLAGS_NO_REWIND, ..core::mem::zeroed() };
    match (*vcpu).arch.sie_block.iprcc & !PGM_PER {
        PGM_AFX_TRANSLATION | PGM_ASX_TRANSLATION | PGM_EX_TRANSLATION | PGM_LFX_TRANSLATION |
        PGM_LSTE_SEQUENCE | PGM_LSX_TRANSLATION | PGM_LX_TRANSLATION | PGM_PRIMARY_AUTHORITY |
        PGM_SECONDARY_AUTHORITY | PGM_SPACE_SWITCH => pgm_info.trans_exc_code = (*vcpu).arch.sie_block.tecmc,
        PGM_ALEN_TRANSLATION | PGM_ALE_SEQUENCE | PGM_ASTE_INSTANCE | PGM_ASTE_SEQUENCE |
        PGM_ASTE_VALIDITY | PGM_EXTENDED_AUTHORITY => pgm_info.exc_access_id = (*vcpu).arch.sie_block.eai,
        PGM_ASCE_TYPE | PGM_PAGE_TRANSLATION | PGM_REGION_FIRST_TRANS | PGM_REGION_SECOND_TRANS |
        PGM_REGION_THIRD_TRANS | PGM_SEGMENT_TRANSLATION => { pgm_info.trans_exc_code = (*vcpu).arch.sie_block.tecmc; pgm_info.exc_access_id = (*vcpu).arch.sie_block.eai; pgm_info.op_access_id = (*vcpu).arch.sie_block.oai; }
        PGM_MONITOR => { pgm_info.mon_class_nr = (*vcpu).arch.sie_block.mcn; pgm_info.mon_code = (*vcpu).arch.sie_block.tecmc; }
        PGM_VECTOR_PROCESSING | PGM_DATA => pgm_info.data_exc_code = (*vcpu).arch.sie_block.dxc,
        PGM_PROTECTION => { pgm_info.trans_exc_code = (*vcpu).arch.sie_block.tecmc; pgm_info.exc_access_id = (*vcpu).arch.sie_block.eai; }
        _ => {}
    }
    if (*vcpu).arch.sie_block.iprcc & PGM_PER != 0 { pgm_info.per_code = (*vcpu).arch.sie_block.perc; pgm_info.per_atmid = (*vcpu).arch.sie_block.peratmid; pgm_info.per_address = (*vcpu).arch.sie_block.peraddr; pgm_info.per_access_id = (*vcpu).arch.sie_block.peraid; }
    kvm_s390_inject_prog_irq(vcpu, &mut pgm_info)
}

unsafe fn handle_itdb(vcpu: *mut kvm_vcpu) -> i32 {
    if !IS_TE_ENABLED(vcpu) || !IS_ITDB_VALID(vcpu) || (*current()).thread.per_flags & PER_FLAG_NO_TE != 0 { return 0; }
    let itdb = phys_to_virt((*vcpu).arch.sie_block.itdba);
    let rc = write_guest_lc(vcpu, __LC_PGM_TDB, itdb, core::mem::size_of::<kvm_s390_itdb>());
    if rc != 0 { return rc; }
    core::ptr::write_bytes(itdb as *mut u8, 0, core::mem::size_of::<kvm_s390_itdb>());
    0
}

unsafe fn should_handle_per_event(vcpu: *const kvm_vcpu) -> bool {
    if !guestdbg_enabled(vcpu) || (*vcpu).arch.sie_block.iprcc & PGM_PER == 0 { return false; }
    if guestdbg_sstep_enabled(vcpu) && (*vcpu).arch.sie_block.iprcc != PGM_PER { return false; }
    true
}

unsafe fn handle_prog(vcpu: *mut kvm_vcpu) -> i32 {
    let mut psw: psw_t = core::mem::zeroed();
    (*vcpu).stat.exit_program_interruption += 1;
    if kvm_s390_pv_cpu_is_protected(vcpu) { return -EOPNOTSUPP; }
    if should_handle_per_event(vcpu) { let rc = kvm_s390_handle_per_event(vcpu); if rc != 0 { return rc; } if (*vcpu).arch.sie_block.iprcc == 0 { return 0; } }
    trace_kvm_s390_intercept_prog(vcpu, (*vcpu).arch.sie_block.iprcc);
    if (*vcpu).arch.sie_block.iprcc == PGM_SPECIFICATION { let rc = read_guest_lc(vcpu, __LC_PGM_NEW_PSW, &mut psw, core::mem::size_of::<psw_t>()); if rc != 0 { return rc; } if !is_valid_psw(&psw) { return -EOPNOTSUPP; } }
    let rc = handle_itdb(vcpu); if rc != 0 { return rc; }
    inject_prog_on_prog_intercept(vcpu)
}

unsafe fn handle_external_interrupt(vcpu: *mut kvm_vcpu) -> i32 {
    let eic = (*vcpu).arch.sie_block.eic;
    let mut irq: kvm_s390_irq = core::mem::zeroed();
    let mut newpsw: psw_t = core::mem::zeroed();
    (*vcpu).stat.exit_external_interrupt += 1;
    if kvm_s390_pv_cpu_is_protected(vcpu) { newpsw = (*vcpu).arch.sie_block.gpsw; }
    else { let rc = read_guest_lc(vcpu, __LC_EXT_NEW_PSW, &mut newpsw, core::mem::size_of::<psw_t>()); if rc != 0 { return rc; } }
    if (eic == EXT_IRQ_CLK_COMP || eic == EXT_IRQ_CPU_TIMER) && newpsw.mask & PSW_MASK_EXT != 0 { return -EOPNOTSUPP; }
    match eic { EXT_IRQ_CLK_COMP => irq.type_ = KVM_S390_INT_CLOCK_COMP, EXT_IRQ_CPU_TIMER => irq.type_ = KVM_S390_INT_CPU_TIMER,
        EXT_IRQ_EXTERNAL_CALL => { irq.type_ = KVM_S390_INT_EXTERNAL_CALL; irq.u.extcall.code = (*vcpu).arch.sie_block.extcpuaddr; let rc = kvm_s390_inject_vcpu(vcpu, &mut irq); if rc == -EBUSY { return 0; } return rc; },
        _ => return -EOPNOTSUPP }
    kvm_s390_inject_vcpu(vcpu, &mut irq)
}

unsafe fn handle_mvpg_pei(vcpu: *mut kvm_vcpu) -> i32 {
    let (mut reg1, mut reg2) = (0, 0); kvm_s390_get_regs_rre(vcpu, &mut reg1, &mut reg2);
    let mut srcaddr = 0; let mut dstaddr = 0;
    let mut rc = guest_translate_address_with_key(vcpu, (*vcpu).run.s.regs.gprs[reg2], reg2, &mut srcaddr, GACC_FETCH, 0);
    if rc != 0 { return kvm_s390_inject_prog_cond(vcpu, rc); }
    loop { rc = kvm_s390_faultin_gfn_simple(vcpu, core::ptr::null_mut(), gpa_to_gfn(srcaddr), false); if rc != -EAGAIN { break; } }
    if rc != 0 { return rc; }
    rc = guest_translate_address_with_key(vcpu, (*vcpu).run.s.regs.gprs[reg1], reg1, &mut dstaddr, GACC_STORE, 0);
    if rc != 0 { return kvm_s390_inject_prog_cond(vcpu, rc); }
    loop { rc = kvm_s390_faultin_gfn_simple(vcpu, core::ptr::null_mut(), gpa_to_gfn(dstaddr), true); if rc != -EAGAIN { break; } }
    if rc != 0 { return rc; } kvm_s390_retry_instr(vcpu); 0
}

unsafe fn handle_partial_execution(vcpu: *mut kvm_vcpu) -> i32 {
    (*vcpu).stat.exit_pei += 1;
    if (*vcpu).arch.sie_block.ipa == 0xb254 { return handle_mvpg_pei(vcpu); }
    if (*vcpu).arch.sie_block.ipa >> 8 == 0xae { return kvm_s390_handle_sigp_pei(vcpu); }
    -EOPNOTSUPP
}

pub unsafe fn handle_sthyi(vcpu: *mut kvm_vcpu) -> i32 {
    let (mut reg1, mut reg2) = (0, 0); let mut cc = 0; let mut r = 0; let mut rc: u64 = 0;
    if !test_kvm_facility((*vcpu).kvm, 74) { return kvm_s390_inject_program_int(vcpu, PGM_OPERATION); }
    kvm_s390_get_regs_rre(vcpu, &mut reg1, &mut reg2); let code = (*vcpu).run.s.regs.gprs[reg1]; let addr = (*vcpu).run.s.regs.gprs[reg2];
    (*vcpu).stat.instruction_sthyi += 1; trace_kvm_s390_handle_sthyi(vcpu, code, addr);
    if reg1 == reg2 || reg1 & 1 != 0 || reg2 & 1 != 0 { return kvm_s390_inject_program_int(vcpu, PGM_SPECIFICATION); }
    if code & 0xffff != 0 { cc = 3; rc = 4; }
    else { if !kvm_s390_pv_cpu_is_protected(vcpu) && addr & !PAGE_MASK != 0 { return kvm_s390_inject_program_int(vcpu, PGM_SPECIFICATION); }
        let sctns = get_zeroed_page(GFP_KERNEL_ACCOUNT); if sctns.is_null() { return -ENOMEM; }
        cc = sthyi_fill(sctns, &mut rc); if cc < 0 { free_page(sctns as usize); return cc; }
        if cc == 0 { if kvm_s390_pv_cpu_is_protected(vcpu) { memcpy(sida_addr((*vcpu).arch.sie_block), sctns, PAGE_SIZE); } else { r = write_guest(vcpu, addr, reg2, sctns, PAGE_SIZE); if r != 0 { free_page(sctns as usize); return kvm_s390_inject_prog_cond(vcpu, r); } } }
        free_page(sctns as usize);
    }
    (*vcpu).run.s.regs.gprs[reg2 + 1] = rc; kvm_s390_set_psw_cc(vcpu, cc); r
}

unsafe fn handle_operexc(vcpu: *mut kvm_vcpu) -> i32 { (*vcpu).stat.exit_operation_exception += 1; if (*vcpu).arch.sie_block.ipa == 0xb256 { return handle_sthyi(vcpu); } if (*vcpu).kvm.arch.user_operexec != 0 || ((*vcpu).arch.sie_block.ipa == 0 && (*vcpu).kvm.arch.user_instr0 != 0) { return -EOPNOTSUPP; } kvm_s390_inject_program_int(vcpu, PGM_OPERATION) }
unsafe fn handle_pv_notification(vcpu: *mut kvm_vcpu) -> i32 { if (*vcpu).arch.sie_block.ipa >> 8 == 0xae { let r = kvm_s390_handle_sigp_pei(vcpu); if r == 0 { return r; } } handle_instruction(vcpu) }
unsafe fn should_handle_per_ifetch(vcpu: *const kvm_vcpu, rc: i32) -> bool { (*vcpu).arch.sie_block.icptstatus & 2 != 0 && (rc == 0 || rc == -EOPNOTSUPP) && !(guestdbg_sstep_enabled(vcpu) && (*vcpu).arch.local_int.pending_irqs != 0) }

// The remaining interception handlers retain their C control flow and call the supplied kernel interface.
pub unsafe fn kvm_handle_sie_intercept(vcpu: *mut kvm_vcpu) -> i32 {
    let mut rc: i32;
    let mut per_rc = 0;
    if kvm_is_ucontrol((*vcpu).kvm) { return -EOPNOTSUPP; }
    match (*vcpu).arch.sie_block.icptcode {
        ICPT_EXTREQ => { (*vcpu).stat.exit_external_request += 1; return 0; }
        ICPT_IOREQ => { (*vcpu).stat.exit_io_request += 1; return 0; }
        ICPT_INST | ICPT_PV_INSTR => rc = handle_instruction(vcpu),
        ICPT_PROGI => return handle_prog(vcpu),
        ICPT_VALIDITY => return handle_validity(vcpu),
        ICPT_STOP => return handle_stop(vcpu),
        ICPT_EXTINT => return handle_external_interrupt(vcpu),
        ICPT_WAIT => return kvm_s390_handle_wait(vcpu),
        ICPT_OPEREXC => rc = handle_operexc(vcpu),
        ICPT_PARTEXEC => rc = handle_partial_execution(vcpu),
        ICPT_KSS => return kvm_s390_skey_check_enable(vcpu),
        ICPT_MCHKREQ | ICPT_INT_ENABLE => rc = 0,
        ICPT_PV_NOTIFY => rc = handle_pv_notification(vcpu),
        ICPT_PV_PREF => { rc = 0; kvm_s390_pv_convert_to_secure((*vcpu).kvm, kvm_s390_get_prefix(vcpu)); kvm_s390_pv_convert_to_secure((*vcpu).kvm, kvm_s390_get_prefix(vcpu) + PAGE_SIZE); }
        _ => return -EOPNOTSUPP,
    }
    if should_handle_per_ifetch(vcpu, rc) { per_rc = kvm_s390_handle_per_ifetch_icpt(vcpu); }
    if per_rc != 0 { per_rc } else { rc }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
