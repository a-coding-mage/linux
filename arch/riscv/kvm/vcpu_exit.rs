// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Anup Patel <anup.patel@wdc.com>
 */

// Dependencies supplied by the surrounding kernel/Rust translation.

unsafe fn gstage_page_fault(
    vcpu: *mut kvm_vcpu,
    run: *mut kvm_run,
    trap: *mut kvm_cpu_trap,
) -> c_int {
    let mut host_map: kvm_gstage_mapping = core::mem::zeroed();
    let mut writable: bool = false;
    let fault_addr: gpa_t = ((*trap).htval << 2) | ((*trap).stval & 0x3);
    let gfn: gfn_t = fault_addr >> PAGE_SHIFT;
    let memslot = gfn_to_memslot((*vcpu).kvm, gfn);
    let hva = gfn_to_hva_memslot_prot(memslot, gfn, &mut writable);

    if kvm_is_error_hva(hva)
        || ((*trap).scause == EXC_STORE_GUEST_PAGE_FAULT && !writable)
    {
        match (*trap).scause {
            EXC_LOAD_GUEST_PAGE_FAULT => {
                return kvm_riscv_vcpu_mmio_load(vcpu, run, fault_addr, (*trap).htinst);
            }
            EXC_STORE_GUEST_PAGE_FAULT => {
                return kvm_riscv_vcpu_mmio_store(vcpu, run, fault_addr, (*trap).htinst);
            }
            EXC_INST_GUEST_PAGE_FAULT => {
                let mut inst_trap = kvm_cpu_trap {
                    sepc: (*trap).sepc,
                    scause: EXC_INST_ACCESS,
                    stval: (*trap).stval,
                    htval: 0,
                    htinst: 0,
                };
                kvm_riscv_vcpu_trap_redirect(vcpu, &mut inst_trap);
                return 1;
            }
            _ => return -EOPNOTSUPP,
        }
    }

    let ret = kvm_riscv_mmu_map(
        vcpu,
        memslot,
        fault_addr,
        hva,
        (*trap).scause == EXC_STORE_GUEST_PAGE_FAULT,
        &mut host_map,
    );
    if ret < 0 { return ret; }
    1
}

pub unsafe fn kvm_riscv_vcpu_unpriv_read(
    vcpu: *mut kvm_vcpu,
    read_insn: bool,
    mut guest_addr: c_ulong,
    trap: *mut kvm_cpu_trap,
) -> c_ulong {
    let mut taddr = trap as c_ulong;
    let mut ttmp: c_ulong;
    let mut val: c_ulong;
    let mut tmp: c_ulong;
    let mut flags: c_ulong;
    local_irq_save(&mut flags);
    let old_hstatus = csr_swap(CSR_HSTATUS, (*vcpu).arch.guest_context.hstatus);
    let old_stvec = csr_swap(CSR_STVEC, (&__kvm_riscv_unpriv_trap as *const _) as c_ulong);

    // The C HLV/HLVX macro expansion is architecture/build-config supplied.
    // Preserve the required volatile instruction sequence at this translation boundary.
    if read_insn {
        unsafe { core::arch::asm!("// HLVX_HU macro sequence", out("a0") val, out("a1") tmp, inout("a0") taddr, lateout("a1") ttmp, inout("a2") guest_addr, options(nostack)); }
        if (*trap).scause == EXC_LOAD_PAGE_FAULT { (*trap).scause = EXC_INST_PAGE_FAULT; }
    } else {
        unsafe { core::arch::asm!("// HLV_D/HLV_W macro sequence", out("a0") val, inout("a0") taddr, lateout("a1") ttmp, in("a2") guest_addr, options(nostack)); }
    }

    csr_write(CSR_STVEC, old_stvec);
    csr_write(CSR_HSTATUS, old_hstatus);
    local_irq_restore(flags);
    val
}

pub unsafe fn kvm_riscv_vcpu_trap_redirect(vcpu: *mut kvm_vcpu, trap: *mut kvm_cpu_trap) {
    let mut vsstatus = ncsr_read(CSR_VSSTATUS);
    vsstatus &= !SR_SPP;
    if (*vcpu).arch.guest_context.sstatus & SR_SPP != 0 { vsstatus |= SR_SPP; }
    vsstatus &= !SR_SPIE;
    if vsstatus & SR_SIE != 0 { vsstatus |= SR_SPIE; }
    vsstatus &= !SR_SIE;
    if (*vcpu).arch.cfg.henvcfg & ENVCFG_LPE != 0 {
        vsstatus &= !SR_SPELP;
        vsstatus |= (*vcpu).arch.guest_context.sstatus & SR_SPELP;
        (*vcpu).arch.guest_context.sstatus &= !SR_SPELP;
    }
    ncsr_write(CSR_VSSTATUS, vsstatus);
    ncsr_write(CSR_VSCAUSE, (*trap).scause);
    ncsr_write(CSR_VSTVAL, (*trap).stval);
    ncsr_write(CSR_VSEPC, (*trap).sepc);
    (*vcpu).arch.guest_context.sepc = ncsr_read(CSR_VSTVEC);
    (*vcpu).arch.guest_context.sstatus |= SR_SPP;
}

unsafe fn vcpu_redirect(vcpu: *mut kvm_vcpu, trap: *mut kvm_cpu_trap) -> c_int {
    let mut ret = -EFAULT;
    if (*vcpu).arch.guest_context.hstatus & HSTATUS_SPV != 0 {
        kvm_riscv_vcpu_trap_redirect(vcpu, trap);
        ret = 1;
    }
    ret
}

pub unsafe fn kvm_riscv_vcpu_exit(
    vcpu: *mut kvm_vcpu, run: *mut kvm_run, trap: *mut kvm_cpu_trap,
) -> c_int {
    if (*trap).scause & CAUSE_IRQ_FLAG != 0 { return 1; }
    trace_kvm_vcpu_exit((*vcpu).vcpu_id, (*trap).sepc, (*trap).scause, (*trap).stval, (*trap).htval, (*trap).htinst);
    let mut ret = -EFAULT;
    (*run).exit_reason = KVM_EXIT_UNKNOWN;
    match (*trap).scause {
        EXC_INST_ILLEGAL => { kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_ILLEGAL_INSN); (*vcpu).stat.instr_illegal_exits += 1; ret = vcpu_redirect(vcpu, trap); }
        EXC_LOAD_MISALIGNED => { kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_MISALIGNED_LOAD); (*vcpu).stat.load_misaligned_exits += 1; ret = vcpu_redirect(vcpu, trap); }
        EXC_STORE_MISALIGNED => { kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_MISALIGNED_STORE); (*vcpu).stat.store_misaligned_exits += 1; ret = vcpu_redirect(vcpu, trap); }
        EXC_LOAD_ACCESS => { kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_ACCESS_LOAD); (*vcpu).stat.load_access_exits += 1; ret = vcpu_redirect(vcpu, trap); }
        EXC_STORE_ACCESS => { kvm_riscv_vcpu_pmu_incr_fw(vcpu, SBI_PMU_FW_ACCESS_STORE); (*vcpu).stat.store_access_exits += 1; ret = vcpu_redirect(vcpu, trap); }
        EXC_INST_ACCESS => ret = vcpu_redirect(vcpu, trap),
        EXC_VIRTUAL_INST_FAULT if (*vcpu).arch.guest_context.hstatus & HSTATUS_SPV != 0 => ret = kvm_riscv_vcpu_virtual_insn(vcpu, run, trap),
        EXC_INST_GUEST_PAGE_FAULT | EXC_LOAD_GUEST_PAGE_FAULT | EXC_STORE_GUEST_PAGE_FAULT if (*vcpu).arch.guest_context.hstatus & HSTATUS_SPV != 0 => ret = gstage_page_fault(vcpu, run, trap),
        EXC_SUPERVISOR_SYSCALL if (*vcpu).arch.guest_context.hstatus & HSTATUS_SPV != 0 => ret = kvm_riscv_vcpu_sbi_ecall(vcpu, run),
        EXC_BREAKPOINT => { (*run).exit_reason = KVM_EXIT_DEBUG; ret = 0; }
        EXC_SOFTWARE_CHECK if (*vcpu).arch.cfg.henvcfg & (ENVCFG_LPE | ENVCFG_SSE) != 0 => ret = vcpu_redirect(vcpu, trap),
        _ => {}
    }
    if ret < 0 {
        kvm_err!("VCPU exit error {}\n", ret);
        kvm_err!("SEPC=0x{:x} SSTATUS=0x{:x} HSTATUS=0x{:x}\n", (*vcpu).arch.guest_context.sepc, (*vcpu).arch.guest_context.sstatus, (*vcpu).arch.guest_context.hstatus);
        kvm_err!("SCAUSE=0x{:x} STVAL=0x{:x} HTVAL=0x{:x} HTINST=0x{:x}\n", (*trap).scause, (*trap).stval, (*trap).htval, (*trap).htinst);
    }
    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
