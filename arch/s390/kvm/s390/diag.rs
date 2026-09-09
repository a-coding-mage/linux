// SPDX-License-Identifier: GPL-2.0
/*
 * handling diagnose instructions
 *
 * Copyright IBM Corp. 2008, 2020
 *
 *    Author(s): Carsten Otte <cotte@de.ibm.com>
 *               Christian Borntraeger <borntraeger@de.ibm.com>
 */

unsafe fn do_discard_gfn_range(vcpu: *mut kvm_vcpu, gfn_start: gfn_t, gfn_end: gfn_t) {
    let mut iter: kvm_memslot_iter = core::mem::zeroed();
    let mut slot: *mut kvm_memory_slot;
    let slots: *mut kvm_memslots;
    let start: c_ulong;
    let end: c_ulong;

    slots = kvm_vcpu_memslots(vcpu);

    kvm_for_each_memslot_in_gfn_range(&mut iter, slots, gfn_start, gfn_end) {
        slot = iter.slot;
        start = __gfn_to_hva_memslot(slot, core::cmp::max(gfn_start, (*slot).base_gfn));
        end = __gfn_to_hva_memslot(
            slot,
            core::cmp::min(gfn_end, (*slot).base_gfn + (*slot).npages),
        );
        gmap_helper_discard((*vcpu).kvm.mm, start, end);
    }
}

unsafe fn diag_release_pages(vcpu: *mut kvm_vcpu) -> c_int {
    let start: c_ulong;
    let end: c_ulong;
    let prefix: c_ulong = kvm_s390_get_prefix(vcpu);

    start = (*vcpu).run.s.regs.gprs[(((*vcpu).arch.sie_block).ipa & 0xf0) >> 4];
    end = (*vcpu).run.s.regs.gprs[(*vcpu).arch.sie_block.ipa & 0xf] + PAGE_SIZE;
    (*vcpu).stat.instruction_diagnose_10 += 1;

    if (start & !PAGE_MASK) != 0 || (end & !PAGE_MASK) != 0 || start >= end || start < 2 * PAGE_SIZE {
        return kvm_s390_inject_program_int(vcpu, PGM_SPECIFICATION);
    }

    VCPU_EVENT(vcpu, 5, "diag release pages %lX %lX", start, end);

    mmap_read_lock((*vcpu).kvm.mm);
    if end <= prefix || start >= prefix + 2 * PAGE_SIZE {
        do_discard_gfn_range(vcpu, gpa_to_gfn(start), gpa_to_gfn(end));
    } else {
        do_discard_gfn_range(vcpu, gpa_to_gfn(start), gpa_to_gfn(prefix));
        if start <= prefix {
            do_discard_gfn_range(vcpu, 0, 1);
        }
        if end > prefix + PAGE_SIZE {
            do_discard_gfn_range(vcpu, 1, 2);
        }
        do_discard_gfn_range(vcpu, gpa_to_gfn(prefix) + 2, gpa_to_gfn(end));
    }
    mmap_read_unlock((*vcpu).kvm.mm);
    0
}

#[repr(C)]
struct prs_parm {
    code: u16,
    subcode: u16,
    parm_len: u16,
    parm_version: u16,
    token_addr: u64,
    select_mask: u64,
    compare_mask: u64,
    zarch: u64,
}

unsafe fn __diag_page_ref_service(vcpu: *mut kvm_vcpu) -> c_int {
    let mut parm: prs_parm = core::mem::zeroed();
    let mut rc: c_int;
    let rx: u16 = (((*vcpu).arch.sie_block.ipa & 0xf0) >> 4) as u16;
    let ry: u16 = ((*vcpu).arch.sie_block.ipa & 0x0f) as u16;

    VCPU_EVENT(vcpu, 3, "diag page reference parameter block at 0x%llx", (*vcpu).run.s.regs.gprs[rx as usize]);
    (*vcpu).stat.instruction_diagnose_258 += 1;
    if (*vcpu).run.s.regs.gprs[rx as usize] & 7 != 0 {
        return kvm_s390_inject_program_int(vcpu, PGM_SPECIFICATION);
    }
    rc = read_guest_real(vcpu, (*vcpu).run.s.regs.gprs[rx as usize], &mut parm as *mut _ as *mut c_void, core::mem::size_of::<prs_parm>());
    if rc != 0 {
        return kvm_s390_inject_prog_cond(vcpu, rc);
    }
    if parm.parm_version != 2 || parm.parm_len < 5 || parm.code != 0x258 {
        return kvm_s390_inject_program_int(vcpu, PGM_SPECIFICATION);
    }

    match parm.subcode {
        0 => {
            VCPU_EVENT(vcpu, 3, "pageref token addr 0x%llx select mask 0x%llx compare mask 0x%llx", parm.token_addr, parm.select_mask, parm.compare_mask);
            if (*vcpu).arch.pfault_token != KVM_S390_PFAULT_TOKEN_INVALID {
                (*vcpu).run.s.regs.gprs[ry as usize] = 8;
                return 0;
            }
            if (parm.compare_mask & parm.select_mask) != parm.compare_mask || parm.token_addr & 7 != 0 || parm.zarch != 0x8000000000000000u64 {
                return kvm_s390_inject_program_int(vcpu, PGM_SPECIFICATION);
            }
            if !kvm_is_gpa_in_memslot((*vcpu).kvm, parm.token_addr) {
                return kvm_s390_inject_program_int(vcpu, PGM_ADDRESSING);
            }
            (*vcpu).arch.pfault_token = parm.token_addr;
            (*vcpu).arch.pfault_select = parm.select_mask;
            (*vcpu).arch.pfault_compare = parm.compare_mask;
            (*vcpu).run.s.regs.gprs[ry as usize] = 0;
            rc = 0;
        }
        1 => {
            VCPU_EVENT(vcpu, 3, "pageref cancel addr 0x%llx", parm.token_addr);
            if parm.token_addr != 0 || parm.select_mask != 0 || parm.compare_mask != 0 || parm.zarch != 0 {
                return kvm_s390_inject_program_int(vcpu, PGM_SPECIFICATION);
            }
            (*vcpu).run.s.regs.gprs[ry as usize] = 0;
            if (*vcpu).arch.pfault_token == KVM_S390_PFAULT_TOKEN_INVALID {
                (*vcpu).run.s.regs.gprs[ry as usize] = 4;
            } else {
                (*vcpu).arch.pfault_token = KVM_S390_PFAULT_TOKEN_INVALID;
            }
            rc = 0;
        }
        _ => rc = -EOPNOTSUPP,
    }
    rc
}

unsafe fn __diag_time_slice_end(vcpu: *mut kvm_vcpu) -> c_int {
    VCPU_EVENT(vcpu, 5, "%s", "diag time slice end");
    (*vcpu).stat.instruction_diagnose_44 += 1;
    kvm_vcpu_on_spin(vcpu, true);
    0
}

static mut forward_cnt: c_int = 0;
static mut cur_slice: c_ulong = 0;

unsafe fn diag9c_forwarding_overrun() -> c_int {
    if time_after(jiffies, cur_slice) {
        cur_slice = jiffies;
        forward_cnt = diag9c_forwarding_hz / HZ;
    }
    let old = forward_cnt;
    forward_cnt -= 1;
    if old <= 0 { 1 } else { 0 }
}

unsafe fn __diag_time_slice_end_directed(vcpu: *mut kvm_vcpu) -> c_int {
    let tcpu: *mut kvm_vcpu;
    let result: *const c_char;
    let mut tcpu_cpu: c_int = -1;
    let tid: c_int;

    tid = (*vcpu).run.s.regs.gprs[(((*vcpu).arch.sie_block).ipa & 0xf0) >> 4] as c_int;
    (*vcpu).stat.instruction_diagnose_9c += 1;
    if tid == (*vcpu).vcpu_id { goto no_yield; }
    tcpu = kvm_get_vcpu_by_id((*vcpu).kvm, tid);
    if tcpu.is_null() { goto no_yield; }
    tcpu_cpu = READ_ONCE((*tcpu).cpu);
    if tcpu_cpu >= 0 {
        if diag9c_forwarding_hz == 0 || diag9c_forwarding_overrun() != 0 { goto no_yield; }
        if !vcpu_is_preempted(tcpu_cpu) { goto no_yield; }
        smp_yield_cpu(tcpu_cpu);
        (*vcpu).stat.diag_9c_forward += 1;
        result = c"yield forwarded".as_ptr();
        goto out;
    }
    if kvm_vcpu_yield_to(tcpu) <= 0 { goto no_yield; }
    result = c"done".as_ptr();
    goto out;
no_yield:
    (*vcpu).stat.diag_9c_ignored += 1;
    result = c"ignored".as_ptr();
out:
    VCPU_EVENT(vcpu, 5, "diag time slice end directed to %d: %s", tid, result);
    trace_kvm_s390_diag_9c(vcpu, tid, tcpu_cpu, result);
    0
}

unsafe fn __diag_ipl_functions(vcpu: *mut kvm_vcpu) -> c_int {
    let reg = ((*vcpu).arch.sie_block.ipa & 0xf) as usize;
    let subcode = (*vcpu).run.s.regs.gprs[reg] & 0xffff;
    VCPU_EVENT(vcpu, 3, "diag ipl functions, subcode %lx", subcode);
    (*vcpu).stat.instruction_diagnose_308 += 1;
    match subcode {
        3 => (*vcpu).run.s390_reset_flags = KVM_S390_RESET_CLEAR,
        4 => (*vcpu).run.s390_reset_flags = 0,
        _ => return -EOPNOTSUPP,
    }
    if !kvm_s390_user_cpu_state_ctrl((*vcpu).kvm) { kvm_s390_vcpu_stop(vcpu); }
    (*vcpu).run.s390_reset_flags |= KVM_S390_RESET_SUBSYSTEM | KVM_S390_RESET_IPL | KVM_S390_RESET_CPU_INIT;
    (*vcpu).run.exit_reason = KVM_EXIT_S390_RESET;
    VCPU_EVENT(vcpu, 3, "requesting userspace resets %llx", (*vcpu).run.s390_reset_flags);
    trace_kvm_s390_request_resets((*vcpu).run.s390_reset_flags);
    -EREMOTE
}

unsafe fn __diag_virtio_hypercall(vcpu: *mut kvm_vcpu) -> c_int {
    (*vcpu).stat.instruction_diagnose_500 += 1;
    if !(*vcpu).kvm.arch.css_support || (*vcpu).run.s.regs.gprs[1] != KVM_S390_VIRTIO_CCW_NOTIFY { return -EOPNOTSUPP; }
    VCPU_EVENT(vcpu, 4, "diag 0x500 schid 0x%8.8x queue 0x%x cookie 0x%llx", (*vcpu).run.s.regs.gprs[2] as u32, (*vcpu).run.s.regs.gprs[3] as u32, (*vcpu).run.s.regs.gprs[4]);
    let ret = kvm_io_bus_write_cookie(vcpu, KVM_VIRTIO_CCW_NOTIFY_BUS, (*vcpu).run.s.regs.gprs[2] & 0xffffffff, 8, &(*vcpu).run.s.regs.gprs[3], (*vcpu).run.s.regs.gprs[4]);
    if ret != -EOPNOTSUPP { (*vcpu).run.s.regs.gprs[2] = ret as u64; }
    if ret < 0 { ret } else { 0 }
}

pub unsafe fn kvm_s390_handle_diag(vcpu: *mut kvm_vcpu) -> c_int {
    let code = (kvm_s390_get_base_disp_rs(vcpu, core::ptr::null_mut()) & 0xffff) as c_int;
    if (*vcpu).arch.sie_block.gpsw.mask & PSW_MASK_PSTATE != 0 { return kvm_s390_inject_program_int(vcpu, PGM_PRIVILEGED_OP); }
    trace_kvm_s390_handle_diag(vcpu, code);
    match code {
        0x10 => diag_release_pages(vcpu),
        0x44 => __diag_time_slice_end(vcpu),
        0x9c => __diag_time_slice_end_directed(vcpu),
        0x258 => __diag_page_ref_service(vcpu),
        0x308 => __diag_ipl_functions(vcpu),
        0x500 => __diag_virtio_hypercall(vcpu),
        _ => { (*vcpu).stat.instruction_diagnose_other += 1; -EOPNOTSUPP }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
