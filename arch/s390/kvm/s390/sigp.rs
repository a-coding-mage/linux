// SPDX-License-Identifier: GPL-2.0
/*
 * handling interprocessor communication
 *
 * Copyright IBM Corp. 2008, 2013
 *
 *    Author(s): Carsten Otte <cotte@de.ibm.com>
 *               Christian Borntraeger <borntraeger@de.ibm.com>
 *               Christian Ehrhardt <ehrhardt@de.ibm.com>
 */

// Dependencies supplied by the surrounding kernel/KVM translation unit.

unsafe fn __sigp_sense(vcpu: *mut kvm_vcpu, dst_vcpu: *mut kvm_vcpu, reg: *mut u64) -> i32 {
    let stopped: bool = kvm_s390_test_cpuflags(dst_vcpu, CPUSTAT_STOPPED);
    let ext_call_pending: i32 = kvm_s390_ext_call_pending(dst_vcpu);
    let rc: i32;

    if !stopped && ext_call_pending == 0 {
        rc = SIGP_CC_ORDER_CODE_ACCEPTED;
    } else {
        *reg &= 0xffffffff00000000u64;
        if ext_call_pending != 0 {
            *reg |= SIGP_STATUS_EXT_CALL_PENDING;
        }
        if stopped {
            *reg |= SIGP_STATUS_STOPPED;
        }
        rc = SIGP_CC_STATUS_STORED;
    }

    VCPU_EVENT!(vcpu, 4, "sensed status of cpu %x rc %x", (*dst_vcpu).vcpu_id, rc);
    rc
}

unsafe fn __inject_sigp_emergency(vcpu: *mut kvm_vcpu, dst_vcpu: *mut kvm_vcpu) -> i32 {
    let irq = kvm_s390_irq {
        type_: KVM_S390_INT_EMERGENCY,
        u: kvm_s390_irq_union { emerg: kvm_s390_irq_emerg { code: (*vcpu).vcpu_id } },
    };
    let rc = kvm_s390_inject_vcpu(dst_vcpu, &irq);
    if rc == 0 {
        VCPU_EVENT!(vcpu, 4, "sent sigp emerg to cpu %x", (*dst_vcpu).vcpu_id);
    }
    if rc != 0 { rc } else { SIGP_CC_ORDER_CODE_ACCEPTED }
}

unsafe fn __sigp_emergency(vcpu: *mut kvm_vcpu, dst_vcpu: *mut kvm_vcpu) -> i32 {
    __inject_sigp_emergency(vcpu, dst_vcpu)
}

unsafe fn __sigp_conditional_emergency(vcpu: *mut kvm_vcpu, dst_vcpu: *mut kvm_vcpu, asn: u16, reg: *mut u64) -> i32 {
    let psw_int_mask = PSW_MASK_IO | PSW_MASK_EXT;
    let psw = &(*(*dst_vcpu).arch.sie_block).gpsw;
    let p_asn = (*dst_vcpu).arch.sie_block.as_ref().unwrap().gcr[4] & 0xffff;
    let s_asn = (*dst_vcpu).arch.sie_block.as_ref().unwrap().gcr[3] & 0xffff;
    let idle = is_vcpu_idle(vcpu);

    if !is_vcpu_stopped(vcpu) || (psw.mask & psw_int_mask) != psw_int_mask || (idle && psw.addr != 0) || (!idle && (asn == p_asn || asn == s_asn)) {
        __inject_sigp_emergency(vcpu, dst_vcpu)
    } else {
        *reg &= 0xffffffff00000000u64;
        *reg |= SIGP_STATUS_INCORRECT_STATE;
        SIGP_CC_STATUS_STORED
    }
}

unsafe fn __sigp_external_call(vcpu: *mut kvm_vcpu, dst_vcpu: *mut kvm_vcpu, reg: *mut u64) -> i32 {
    let irq = kvm_s390_irq {
        type_: KVM_S390_INT_EXTERNAL_CALL,
        u: kvm_s390_irq_union { extcall: kvm_s390_irq_extcall { code: (*vcpu).vcpu_id } },
    };
    let rc = kvm_s390_inject_vcpu(dst_vcpu, &irq);
    if rc == -EBUSY {
        *reg &= 0xffffffff00000000u64;
        *reg |= SIGP_STATUS_EXT_CALL_PENDING;
        return SIGP_CC_STATUS_STORED;
    } else if rc == 0 {
        VCPU_EVENT!(vcpu, 4, "sent sigp ext call to cpu %x", (*dst_vcpu).vcpu_id);
    }
    if rc != 0 { rc } else { SIGP_CC_ORDER_CODE_ACCEPTED }
}

unsafe fn __sigp_stop(vcpu: *mut kvm_vcpu, dst_vcpu: *mut kvm_vcpu) -> i32 {
    let irq = kvm_s390_irq { type_: KVM_S390_SIGP_STOP, u: kvm_s390_irq_union::default() };
    let mut rc = kvm_s390_inject_vcpu(dst_vcpu, &irq);
    if rc == -EBUSY { rc = SIGP_CC_BUSY; } else if rc == 0 { VCPU_EVENT!(vcpu, 4, "sent sigp stop to cpu %x", (*dst_vcpu).vcpu_id); }
    rc
}

unsafe fn __sigp_stop_and_store_status(vcpu: *mut kvm_vcpu, dst_vcpu: *mut kvm_vcpu, _reg: *mut u64) -> i32 {
    let irq = kvm_s390_irq { type_: KVM_S390_SIGP_STOP, u: kvm_s390_irq_union { stop: kvm_s390_irq_stop { flags: KVM_S390_STOP_FLAG_STORE_STATUS } } };
    let mut rc = kvm_s390_inject_vcpu(dst_vcpu, &irq);
    if rc == -EBUSY { rc = SIGP_CC_BUSY; } else if rc == 0 { VCPU_EVENT!(vcpu, 4, "sent sigp stop and store status to cpu %x", (*dst_vcpu).vcpu_id); }
    rc
}

unsafe fn __sigp_set_arch(_vcpu: *mut kvm_vcpu, _parameter: u32, status_reg: *mut u64) -> i32 {
    *status_reg &= 0xffffffff00000000u64;
    // Reject set arch order, with czam we're always in z/Arch mode.
    *status_reg |= SIGP_STATUS_INVALID_PARAMETER;
    SIGP_CC_STATUS_STORED
}

unsafe fn __sigp_set_prefix(vcpu: *mut kvm_vcpu, dst_vcpu: *mut kvm_vcpu, address: u32, reg: *mut u64) -> i32 {
    let irq = kvm_s390_irq { type_: KVM_S390_SIGP_SET_PREFIX, u: kvm_s390_irq_union { prefix: kvm_s390_irq_prefix { address: address & 0x7fffe000u32 } } };
    if !kvm_is_gpa_in_memslot((*vcpu).kvm, irq.u.prefix.address) {
        *reg &= 0xffffffff00000000u64; *reg |= SIGP_STATUS_INVALID_PARAMETER; return SIGP_CC_STATUS_STORED;
    }
    let rc = kvm_s390_inject_vcpu(dst_vcpu, &irq);
    if rc == -EBUSY { *reg &= 0xffffffff00000000u64; *reg |= SIGP_STATUS_INCORRECT_STATE; return SIGP_CC_STATUS_STORED; }
    rc
}

unsafe fn __sigp_store_status_at_addr(_vcpu: *mut kvm_vcpu, dst_vcpu: *mut kvm_vcpu, mut addr: u32, reg: *mut u64) -> i32 {
    if !kvm_s390_test_cpuflags(dst_vcpu, CPUSTAT_STOPPED) { *reg &= 0xffffffff00000000u64; *reg |= SIGP_STATUS_INCORRECT_STATE; return SIGP_CC_STATUS_STORED; }
    addr &= 0x7ffffe00;
    let mut rc = kvm_s390_store_status_unloaded(dst_vcpu, addr);
    if rc == -EFAULT { *reg &= 0xffffffff00000000u64; *reg |= SIGP_STATUS_INVALID_PARAMETER; rc = SIGP_CC_STATUS_STORED; }
    rc
}

unsafe fn __sigp_sense_running(vcpu: *mut kvm_vcpu, dst_vcpu: *mut kvm_vcpu, reg: *mut u64) -> i32 {
    if !test_kvm_facility((*vcpu).kvm, 9) { *reg &= 0xffffffff00000000u64; *reg |= SIGP_STATUS_INVALID_ORDER; return SIGP_CC_STATUS_STORED; }
    let rc;
    if kvm_s390_test_cpuflags(dst_vcpu, CPUSTAT_RUNNING) { rc = SIGP_CC_ORDER_CODE_ACCEPTED; } else { *reg &= 0xffffffff00000000u64; *reg |= SIGP_STATUS_NOT_RUNNING; rc = SIGP_CC_STATUS_STORED; }
    VCPU_EVENT!(vcpu, 4, "sensed running status of cpu %x rc %x", (*dst_vcpu).vcpu_id, rc);
    rc
}

unsafe fn __prepare_sigp_re_start(_vcpu: *mut kvm_vcpu, dst_vcpu: *mut kvm_vcpu, _order_code: u8) -> i32 {
    let li = &mut (*dst_vcpu).arch.local_int;
    let mut rc = -EOPNOTSUPP;
    spin_lock(&mut li.lock);
    if kvm_s390_is_stop_irq_pending(dst_vcpu) { rc = SIGP_CC_BUSY; }
    spin_unlock(&mut li.lock);
    rc
}

unsafe fn __prepare_sigp_cpu_reset(_vcpu: *mut kvm_vcpu, _dst_vcpu: *mut kvm_vcpu, _order_code: u8) -> i32 { -EOPNOTSUPP }
unsafe fn __prepare_sigp_unknown(_vcpu: *mut kvm_vcpu, _dst_vcpu: *mut kvm_vcpu) -> i32 { -EOPNOTSUPP }

// The remaining dispatcher logic mirrors the C switch and delegates to the
// external KVM structures and constants supplied by the surrounding unit.
unsafe fn handle_sigp_dst(vcpu: *mut kvm_vcpu, order_code: u8, cpu_addr: u16, parameter: u32, status_reg: *mut u64) -> i32 {
    let dst_vcpu = kvm_get_vcpu_by_id((*vcpu).kvm, cpu_addr);
    if dst_vcpu.is_null() { return SIGP_CC_NOT_OPERATIONAL; }
    if order_code != SIGP_INITIAL_CPU_RESET && order_code != SIGP_CPU_RESET && (kvm_s390_is_stop_irq_pending(dst_vcpu) || kvm_s390_is_restart_irq_pending(dst_vcpu)) { return SIGP_CC_BUSY; }
    let rc = match order_code {
        SIGP_SENSE => { (*vcpu).stat.instruction_sigp_sense += 1; __sigp_sense(vcpu, dst_vcpu, status_reg) },
        SIGP_EXTERNAL_CALL => { (*vcpu).stat.instruction_sigp_external_call += 1; __sigp_external_call(vcpu, dst_vcpu, status_reg) },
        SIGP_EMERGENCY_SIGNAL => { (*vcpu).stat.instruction_sigp_emergency += 1; __sigp_emergency(vcpu, dst_vcpu) },
        SIGP_STOP => { (*vcpu).stat.instruction_sigp_stop += 1; __sigp_stop(vcpu, dst_vcpu) },
        SIGP_STOP_AND_STORE_STATUS => { (*vcpu).stat.instruction_sigp_stop_store_status += 1; __sigp_stop_and_store_status(vcpu, dst_vcpu, status_reg) },
        SIGP_STORE_STATUS_AT_ADDRESS => { (*vcpu).stat.instruction_sigp_store_status += 1; __sigp_store_status_at_addr(vcpu, dst_vcpu, parameter, status_reg) },
        SIGP_SET_PREFIX => { (*vcpu).stat.instruction_sigp_prefix += 1; __sigp_set_prefix(vcpu, dst_vcpu, parameter, status_reg) },
        SIGP_COND_EMERGENCY_SIGNAL => { (*vcpu).stat.instruction_sigp_cond_emergency += 1; __sigp_conditional_emergency(vcpu, dst_vcpu, parameter as u16, status_reg) },
        SIGP_SENSE_RUNNING => { (*vcpu).stat.instruction_sigp_sense_running += 1; __sigp_sense_running(vcpu, dst_vcpu, status_reg) },
        SIGP_START | SIGP_RESTART => __prepare_sigp_re_start(vcpu, dst_vcpu, order_code),
        SIGP_INITIAL_CPU_RESET | SIGP_CPU_RESET => __prepare_sigp_cpu_reset(vcpu, dst_vcpu, order_code),
        _ => __prepare_sigp_unknown(vcpu, dst_vcpu),
    };
    if rc == -EOPNOTSUPP { VCPU_EVENT!(vcpu, 4, "sigp order %u -> cpu %x: handled in user space", order_code, (*dst_vcpu).vcpu_id); }
    rc
}

unsafe fn handle_sigp_order_in_user_space(vcpu: *mut kvm_vcpu, order_code: u8, cpu_addr: u16) -> i32 {
    if !(*(*vcpu).kvm).arch.user_sigp { return 0; }
    match order_code { SIGP_SENSE | SIGP_EXTERNAL_CALL | SIGP_EMERGENCY_SIGNAL | SIGP_COND_EMERGENCY_SIGNAL | SIGP_SENSE_RUNNING => return 0, _ => {} }
    VCPU_EVENT!(vcpu, 3, "SIGP: order %u for CPU %d handled in userspace", order_code, cpu_addr);
    1
}

pub unsafe fn kvm_s390_handle_sigp(vcpu: *mut kvm_vcpu) -> i32 {
    let r1 = (((*vcpu).arch.sie_block.as_ref().unwrap().ipa & 0x00f0) >> 4) as usize;
    let r3 = ((*vcpu).arch.sie_block.as_ref().unwrap().ipa & 0x000f) as usize;
    let cpu_addr = (*vcpu).run.s.regs.gprs[r3] as u16;
    if (*vcpu).arch.sie_block.as_ref().unwrap().gpsw.mask & PSW_MASK_PSTATE != 0 { return kvm_s390_inject_program_int(vcpu, PGM_PRIVILEGED_OP); }
    let order_code = kvm_s390_get_base_disp_rs(vcpu, core::ptr::null_mut());
    if handle_sigp_order_in_user_space(vcpu, order_code, cpu_addr) != 0 { return -EOPNOTSUPP; }
    let parameter = if r1 % 2 != 0 { (*vcpu).run.s.regs.gprs[r1] } else { (*vcpu).run.s.regs.gprs[r1 + 1] } as u32;
    trace_kvm_s390_handle_sigp(vcpu, order_code, cpu_addr, parameter);
    let rc = if order_code == SIGP_SET_ARCHITECTURE { (*vcpu).stat.instruction_sigp_arch += 1; __sigp_set_arch(vcpu, parameter, &mut (*vcpu).run.s.regs.gprs[r1]) } else { handle_sigp_dst(vcpu, order_code, cpu_addr, parameter, &mut (*vcpu).run.s.regs.gprs[r1]) };
    if rc < 0 { return rc; }
    kvm_s390_set_psw_cc(vcpu, rc); 0
}

/* Handle SIGP partial execution interception. */
pub unsafe fn kvm_s390_handle_sigp_pei(vcpu: *mut kvm_vcpu) -> i32 {
    let r3 = ((*vcpu).arch.sie_block.as_ref().unwrap().ipa & 0x000f) as usize;
    let cpu_addr = (*vcpu).run.s.regs.gprs[r3] as u16;
    let order_code = kvm_s390_get_base_disp_rs(vcpu, core::ptr::null_mut());
    if order_code == SIGP_EXTERNAL_CALL {
        trace_kvm_s390_handle_sigp_pei(vcpu, order_code, cpu_addr);
        let dest_vcpu = kvm_get_vcpu_by_id((*vcpu).kvm, cpu_addr);
        BUG_ON!(dest_vcpu.is_null());
        kvm_s390_vcpu_wakeup(dest_vcpu);
        kvm_s390_set_psw_cc(vcpu, SIGP_CC_ORDER_CODE_ACCEPTED);
        return 0;
    }
    -EOPNOTSUPP
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
