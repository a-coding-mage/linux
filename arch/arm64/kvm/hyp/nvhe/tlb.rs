// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

struct TlbInvContext {
    mmu: *mut kvm_s2_mmu,
    tcr: u64,
    sctlr: u64,
}

unsafe fn enter_vmid_context(mmu: *mut kvm_s2_mmu, cxt: *mut TlbInvContext, nsh: bool) {
    let host_s2_mmu: *mut kvm_s2_mmu = &mut host_mmu.arch.mmu;
    let host_ctxt: *mut kvm_cpu_context;
    let vcpu: *mut kvm_vcpu;

    host_ctxt = &mut this_cpu_ptr(&mut kvm_host_data).host_ctxt;
    vcpu = (*host_ctxt).__hyp_running_vcpu;
    (*cxt).mmu = core::ptr::null_mut();

    if nsh {
        dsb(nsh);
    } else {
        dsb(ish);
    }

    if !vcpu.is_null() {
        if mmu == (*vcpu).arch.hw_mmu || WARN_ON(mmu != host_s2_mmu) {
            return;
        }

        (*cxt).mmu = (*vcpu).arch.hw_mmu;
    } else {
        if mmu == host_s2_mmu {
            return;
        }

        (*cxt).mmu = host_s2_mmu;
    }

    if cpus_have_final_cap(ARM64_WORKAROUND_SPECULATIVE_AT) {
        let mut val: u64;

        val = read_sysreg_el1(SYS_TCR);
        (*cxt).tcr = val;
        val |= TCR_EPD1_MASK | TCR_EPD0_MASK;
        write_sysreg_el1(val, SYS_TCR);
        isb();

        if !vcpu.is_null() {
            val = read_sysreg_el1(SYS_SCTLR);
            (*cxt).sctlr = val;
            if (val & SCTLR_ELx_M) == 0 {
                val |= SCTLR_ELx_M;
                write_sysreg_el1(val, SYS_SCTLR);
                isb();
            }
        } else {
            (*cxt).sctlr = SCTLR_ELx_M;
        }
    }

    if !vcpu.is_null() {
        __load_host_stage2();
    } else {
        __load_stage2(mmu);
    }

    asm!("isb");
}

unsafe fn exit_vmid_context(cxt: *mut TlbInvContext) {
    let mmu = (*cxt).mmu;
    let host_ctxt: *mut kvm_cpu_context;
    let vcpu: *mut kvm_vcpu;

    host_ctxt = &mut this_cpu_ptr(&mut kvm_host_data).host_ctxt;
    vcpu = (*host_ctxt).__hyp_running_vcpu;

    if mmu.is_null() {
        return;
    }

    if !vcpu.is_null() {
        __load_stage2(mmu);
    } else {
        __load_host_stage2();
    }

    isb();

    if cpus_have_final_cap(ARM64_WORKAROUND_SPECULATIVE_AT) {
        if ((*cxt).sctlr & SCTLR_ELx_M) == 0 {
            write_sysreg_el1((*cxt).sctlr, SYS_SCTLR);
            isb();
        }

        write_sysreg_el1((*cxt).tcr, SYS_TCR);
    }
}

pub unsafe fn __kvm_tlb_flush_vmid_ipa(mmu: *mut kvm_s2_mmu, ipa: phys_addr_t, level: i32) {
    let mut cxt = TlbInvContext { mmu: core::ptr::null_mut(), tcr: 0, sctlr: 0 };
    enter_vmid_context(mmu, &mut cxt, false);
    __tlbi_level!(ipas2e1is, ipa, level);
    dsb(ish);
    __tlbi!(vmalle1is);
    __tlbi_sync_s1ish_hyp();
    isb();
    exit_vmid_context(&mut cxt);
}

pub unsafe fn __kvm_tlb_flush_vmid_ipa_nsh(mmu: *mut kvm_s2_mmu, ipa: phys_addr_t, level: i32) {
    let mut cxt = TlbInvContext { mmu: core::ptr::null_mut(), tcr: 0, sctlr: 0 };
    enter_vmid_context(mmu, &mut cxt, true);
    __tlbi_level!(ipas2e1, ipa, level);
    dsb(nsh);
    __tlbi!(vmalle1);
    dsb(nsh);
    isb();
    exit_vmid_context(&mut cxt);
}

pub unsafe fn __kvm_tlb_flush_vmid_range(
    mmu: *mut kvm_s2_mmu,
    mut start: phys_addr_t,
    pages: c_ulong,
) {
    let mut cxt = TlbInvContext { mmu: core::ptr::null_mut(), tcr: 0, sctlr: 0 };
    let stride = PAGE_SIZE;
    start = round_down(start, stride);
    enter_vmid_context(mmu, &mut cxt, false);
    __flush_s2_tlb_range_op!(ipas2e1is, start, pages, stride, TLBI_TTL_UNKNOWN);
    dsb(ish);
    __tlbi!(vmalle1is);
    __tlbi_sync_s1ish_hyp();
    isb();
    exit_vmid_context(&mut cxt);
}

pub unsafe fn __kvm_tlb_flush_vmid(mmu: *mut kvm_s2_mmu) {
    let mut cxt = TlbInvContext { mmu: core::ptr::null_mut(), tcr: 0, sctlr: 0 };
    enter_vmid_context(mmu, &mut cxt, false);
    __tlbi!(vmalls12e1is);
    __tlbi_sync_s1ish_hyp();
    isb();
    exit_vmid_context(&mut cxt);
}

pub unsafe fn __kvm_flush_cpu_context(mmu: *mut kvm_s2_mmu) {
    let mut cxt = TlbInvContext { mmu: core::ptr::null_mut(), tcr: 0, sctlr: 0 };
    enter_vmid_context(mmu, &mut cxt, false);
    __tlbi!(vmalle1);
    asm!("ic iallu");
    dsb(nsh);
    isb();
    exit_vmid_context(&mut cxt);
}

pub unsafe fn __kvm_flush_vm_context() {
    dsb(ish);
    __tlbi!(alle1is);
    __tlbi_sync_s1ish_hyp();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
