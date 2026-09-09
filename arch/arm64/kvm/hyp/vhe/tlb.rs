// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

#[repr(C)]
struct tlb_inv_context {
    mmu: *mut kvm_s2_mmu,
    flags: c_ulong,
    tcr: u64,
    sctlr: u64,
}

unsafe fn enter_vmid_context(mmu: *mut kvm_s2_mmu, cxt: *mut tlb_inv_context) {
    let vcpu: *mut kvm_vcpu = kvm_get_running_vcpu();
    let mut val: u64;

    local_irq_save(&mut (*cxt).flags);

    if !vcpu.is_null() && mmu != (*vcpu).arch.hw_mmu {
        (*cxt).mmu = (*vcpu).arch.hw_mmu;
    } else {
        (*cxt).mmu = core::ptr::null_mut();
    }

    if cpus_have_final_cap(ARM64_WORKAROUND_SPECULATIVE_AT) {
        val = read_sysreg_el1(SYS_TCR);
        (*cxt).tcr = val;
        val |= TCR_EPD1_MASK | TCR_EPD0_MASK;
        write_sysreg_el1(val, SYS_TCR);
        val = read_sysreg_el1(SYS_SCTLR);
        (*cxt).sctlr = val;
        val |= SCTLR_ELx_M;
        write_sysreg_el1(val, SYS_SCTLR);
    }

    __load_stage2(mmu);
    val = read_sysreg(hcr_el2);
    val &= !HCR_TGE;
    write_sysreg_hcr(val);
    isb();
}

unsafe fn exit_vmid_context(cxt: *mut tlb_inv_context) {
    write_sysreg_hcr(HCR_HOST_VHE_FLAGS);
    isb();

    if !(*cxt).mmu.is_null() {
        __load_stage2((*cxt).mmu);
    }

    if cpus_have_final_cap(ARM64_WORKAROUND_SPECULATIVE_AT) {
        write_sysreg_el1((*cxt).tcr, SYS_TCR);
        write_sysreg_el1((*cxt).sctlr, SYS_SCTLR);
    }

    local_irq_restore((*cxt).flags);
}

pub unsafe fn __kvm_tlb_flush_vmid_ipa(
    mmu: *mut kvm_s2_mmu,
    mut ipa: phys_addr_t,
    level: c_int,
) {
    let mut cxt = core::mem::MaybeUninit::<tlb_inv_context>::uninit();

    dsb(ishst);
    enter_vmid_context(mmu, cxt.as_mut_ptr());
    __tlbi_level(ipas2e1is, ipa, level);
    dsb(ish);
    __tlbi(vmalle1is);
    __tlbi_sync_s1ish_hyp();
    isb();
    exit_vmid_context(cxt.as_mut_ptr());
}

pub unsafe fn __kvm_tlb_flush_vmid_ipa_nsh(
    mmu: *mut kvm_s2_mmu,
    ipa: phys_addr_t,
    level: c_int,
) {
    let mut cxt = core::mem::MaybeUninit::<tlb_inv_context>::uninit();

    dsb(nshst);
    enter_vmid_context(mmu, cxt.as_mut_ptr());
    __tlbi_level(ipas2e1, ipa, level);
    dsb(nsh);
    __tlbi(vmalle1);
    dsb(nsh);
    isb();
    exit_vmid_context(cxt.as_mut_ptr());
}

pub unsafe fn __kvm_tlb_flush_vmid_range(
    mmu: *mut kvm_s2_mmu,
    mut start: phys_addr_t,
    pages: c_ulong,
) {
    let mut cxt = core::mem::MaybeUninit::<tlb_inv_context>::uninit();
    let stride: c_ulong = PAGE_SIZE;

    start = round_down(start, stride);
    dsb(ishst);
    enter_vmid_context(mmu, cxt.as_mut_ptr());
    __flush_s2_tlb_range_op(ipas2e1is, start, pages, stride, TLBI_TTL_UNKNOWN);
    dsb(ish);
    __tlbi(vmalle1is);
    __tlbi_sync_s1ish_hyp();
    isb();
    exit_vmid_context(cxt.as_mut_ptr());
}

pub unsafe fn __kvm_tlb_flush_vmid(mmu: *mut kvm_s2_mmu) {
    let mut cxt = core::mem::MaybeUninit::<tlb_inv_context>::uninit();

    dsb(ishst);
    enter_vmid_context(mmu, cxt.as_mut_ptr());
    __tlbi(vmalls12e1is);
    __tlbi_sync_s1ish_hyp();
    isb();
    exit_vmid_context(cxt.as_mut_ptr());
}

pub unsafe fn __kvm_flush_cpu_context(mmu: *mut kvm_s2_mmu) {
    let mut cxt = core::mem::MaybeUninit::<tlb_inv_context>::uninit();

    enter_vmid_context(mmu, cxt.as_mut_ptr());
    __tlbi(vmalle1);
    core::arch::asm!("ic iallu");
    dsb(nsh);
    isb();
    exit_vmid_context(cxt.as_mut_ptr());
}

pub unsafe fn __kvm_flush_vm_context() {
    dsb(ishst);
    __tlbi(alle1is);
    __tlbi_sync_s1ish_hyp();
}

pub unsafe fn __kvm_tlbi_s1e2(
    mmu: *mut kvm_s2_mmu,
    va: u64,
    sys_encoding: u64,
) -> c_int {
    let mut cxt = core::mem::MaybeUninit::<tlb_inv_context>::uninit();
    let mut ret: c_int = 0;

    if !mmu.is_null() {
        enter_vmid_context(mmu, cxt.as_mut_ptr());
    }

    match sys_encoding {
        OP_TLBI_ALLE2 | OP_TLBI_ALLE2IS | OP_TLBI_ALLE2OS |
        OP_TLBI_VMALLE1 | OP_TLBI_VMALLE1IS | OP_TLBI_VMALLE1OS |
        OP_TLBI_ALLE2NXS | OP_TLBI_ALLE2ISNXS | OP_TLBI_ALLE2OSNXS |
        OP_TLBI_VMALLE1NXS | OP_TLBI_VMALLE1ISNXS | OP_TLBI_VMALLE1OSNXS =>
            __tlbi(vmalle1is),
        OP_TLBI_VAE2 | OP_TLBI_VAE2IS | OP_TLBI_VAE2OS |
        OP_TLBI_VAE1 | OP_TLBI_VAE1IS | OP_TLBI_VAE1OS |
        OP_TLBI_VAE2NXS | OP_TLBI_VAE2ISNXS | OP_TLBI_VAE2OSNXS |
        OP_TLBI_VAE1NXS | OP_TLBI_VAE1ISNXS | OP_TLBI_VAE1OSNXS =>
            __tlbi(vae1is, va),
        OP_TLBI_VALE2 | OP_TLBI_VALE2IS | OP_TLBI_VALE2OS |
        OP_TLBI_VALE1 | OP_TLBI_VALE1IS | OP_TLBI_VALE1OS |
        OP_TLBI_VALE2NXS | OP_TLBI_VALE2ISNXS | OP_TLBI_VALE2OSNXS |
        OP_TLBI_VALE1NXS | OP_TLBI_VALE1ISNXS | OP_TLBI_VALE1OSNXS =>
            __tlbi(vale1is, va),
        OP_TLBI_ASIDE1 | OP_TLBI_ASIDE1IS | OP_TLBI_ASIDE1OS |
        OP_TLBI_ASIDE1NXS | OP_TLBI_ASIDE1ISNXS | OP_TLBI_ASIDE1OSNXS =>
            __tlbi(aside1is, va),
        OP_TLBI_VAAE1 | OP_TLBI_VAAE1IS | OP_TLBI_VAAE1OS |
        OP_TLBI_VAAE1NXS | OP_TLBI_VAAE1ISNXS | OP_TLBI_VAAE1OSNXS =>
            __tlbi(vaae1is, va),
        OP_TLBI_VAALE1 | OP_TLBI_VAALE1IS | OP_TLBI_VAALE1OS |
        OP_TLBI_VAALE1NXS | OP_TLBI_VAALE1ISNXS | OP_TLBI_VAALE1OSNXS =>
            __tlbi(vaale1is, va),
        OP_TLBI_RVAE2 | OP_TLBI_RVAE2IS | OP_TLBI_RVAE2OS |
        OP_TLBI_RVAE1 | OP_TLBI_RVAE1IS | OP_TLBI_RVAE1OS |
        OP_TLBI_RVAE2NXS | OP_TLBI_RVAE2ISNXS | OP_TLBI_RVAE2OSNXS |
        OP_TLBI_RVAE1NXS | OP_TLBI_RVAE1ISNXS | OP_TLBI_RVAE1OSNXS =>
            __tlbi(rvae1is, va),
        OP_TLBI_RVALE2 | OP_TLBI_RVALE2IS | OP_TLBI_RVALE2OS |
        OP_TLBI_RVALE1 | OP_TLBI_RVALE1IS | OP_TLBI_RVALE1OS |
        OP_TLBI_RVALE2NXS | OP_TLBI_RVALE2ISNXS | OP_TLBI_RVALE2OSNXS |
        OP_TLBI_RVALE1NXS | OP_TLBI_RVALE1ISNXS | OP_TLBI_RVALE1OSNXS =>
            __tlbi(rvale1is, va),
        OP_TLBI_RVAAE1 | OP_TLBI_RVAAE1IS | OP_TLBI_RVAAE1OS |
        OP_TLBI_RVAAE1NXS | OP_TLBI_RVAAE1ISNXS | OP_TLBI_RVAAE1OSNXS =>
            __tlbi(rvaae1is, va),
        OP_TLBI_RVAALE1 | OP_TLBI_RVAALE1IS | OP_TLBI_RVAALE1OS |
        OP_TLBI_RVAALE1NXS | OP_TLBI_RVAALE1ISNXS | OP_TLBI_RVAALE1OSNXS =>
            __tlbi(rvaale1is, va),
        _ => ret = -EINVAL,
    }

    __tlbi_sync_s1ish_hyp();
    isb();

    if !mmu.is_null() {
        exit_vmid_context(cxt.as_mut_ptr());
    }

    ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
