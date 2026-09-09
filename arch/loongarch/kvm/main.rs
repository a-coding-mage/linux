// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2023 Loongson Technology Corporation Limited
 */

// C headers provide the external kernel types, constants, macros, and functions
// referenced below.

use core::ffi::c_void;

pub static mut vpid_mask: c_ulong = 0;
pub static mut kvm_loongarch_ops: *mut kvm_world_switch = core::ptr::null_mut();
static mut gcsr_flag: [c_int; CSR_MAX_NUMS as usize] = [0; CSR_MAX_NUMS as usize];
static mut vmcs: *mut kvm_context = core::ptr::null_mut();

pub unsafe fn get_gcsr_flag(csr: c_int) -> c_int {
    if csr < CSR_MAX_NUMS { gcsr_flag[csr as usize] } else { INVALID_GCSR }
}

#[inline]
unsafe fn set_gcsr_sw_flag(csr: c_int) { if csr < CSR_MAX_NUMS { gcsr_flag[csr as usize] |= SW_GCSR; } }

#[inline]
unsafe fn set_gcsr_hw_flag(csr: c_int) { if csr < CSR_MAX_NUMS { gcsr_flag[csr as usize] |= HW_GCSR; } }

/* The default value is 0; mark software and hardware guest CSRs as 1 and 2. */
unsafe fn kvm_init_gcsr_flag() {
    macro_rules! hw { ($($x:ident),* $(,)?) => { $(set_gcsr_hw_flag($x);)* }; }
    macro_rules! sw { ($($x:ident),* $(,)?) => { $(set_gcsr_sw_flag($x);)* }; }
    hw!(LOONGARCH_CSR_CRMD, LOONGARCH_CSR_PRMD, LOONGARCH_CSR_EUEN, LOONGARCH_CSR_MISC,
        LOONGARCH_CSR_ECFG, LOONGARCH_CSR_ESTAT, LOONGARCH_CSR_ERA, LOONGARCH_CSR_BADV,
        LOONGARCH_CSR_BADI, LOONGARCH_CSR_EENTRY, LOONGARCH_CSR_TLBIDX, LOONGARCH_CSR_TLBEHI,
        LOONGARCH_CSR_TLBELO0, LOONGARCH_CSR_TLBELO1, LOONGARCH_CSR_ASID, LOONGARCH_CSR_PGDL,
        LOONGARCH_CSR_PGDH, LOONGARCH_CSR_PGD, LOONGARCH_CSR_PWCTL0, LOONGARCH_CSR_PWCTL1,
        LOONGARCH_CSR_STLBPGSIZE, LOONGARCH_CSR_RVACFG, LOONGARCH_CSR_CPUID, LOONGARCH_CSR_PRCFG1,
        LOONGARCH_CSR_PRCFG2, LOONGARCH_CSR_PRCFG3, LOONGARCH_CSR_KS0, LOONGARCH_CSR_KS1,
        LOONGARCH_CSR_KS2, LOONGARCH_CSR_KS3, LOONGARCH_CSR_KS4, LOONGARCH_CSR_KS5,
        LOONGARCH_CSR_KS6, LOONGARCH_CSR_KS7, LOONGARCH_CSR_TMID, LOONGARCH_CSR_TCFG,
        LOONGARCH_CSR_TVAL, LOONGARCH_CSR_TINTCLR, LOONGARCH_CSR_CNTC, LOONGARCH_CSR_LLBCTL,
        LOONGARCH_CSR_TLBRENTRY, LOONGARCH_CSR_TLBRBADV, LOONGARCH_CSR_TLBRERA,
        LOONGARCH_CSR_TLBRSAVE, LOONGARCH_CSR_TLBRELO0, LOONGARCH_CSR_TLBRELO1,
        LOONGARCH_CSR_TLBREHI, LOONGARCH_CSR_TLBRPRMD, LOONGARCH_CSR_DMWIN0,
        LOONGARCH_CSR_DMWIN1, LOONGARCH_CSR_DMWIN2, LOONGARCH_CSR_DMWIN3);
    sw!(LOONGARCH_CSR_IMPCTL1, LOONGARCH_CSR_IMPCTL2, LOONGARCH_CSR_MERRCTL,
        LOONGARCH_CSR_MERRINFO1, LOONGARCH_CSR_MERRINFO2, LOONGARCH_CSR_MERRENTRY,
        LOONGARCH_CSR_MERRERA, LOONGARCH_CSR_MERRSAVE, LOONGARCH_CSR_CTAG,
        LOONGARCH_CSR_DEBUG, LOONGARCH_CSR_DERA, LOONGARCH_CSR_DESAVE,
        LOONGARCH_CSR_FWPC, LOONGARCH_CSR_FWPS, LOONGARCH_CSR_MWPC, LOONGARCH_CSR_MWPS);
    sw!(LOONGARCH_CSR_DB0ADDR, LOONGARCH_CSR_DB0MASK, LOONGARCH_CSR_DB0CTRL, LOONGARCH_CSR_DB0ASID,
        LOONGARCH_CSR_DB1ADDR, LOONGARCH_CSR_DB1MASK, LOONGARCH_CSR_DB1CTRL, LOONGARCH_CSR_DB1ASID,
        LOONGARCH_CSR_DB2ADDR, LOONGARCH_CSR_DB2MASK, LOONGARCH_CSR_DB2CTRL, LOONGARCH_CSR_DB2ASID,
        LOONGARCH_CSR_DB3ADDR, LOONGARCH_CSR_DB3MASK, LOONGARCH_CSR_DB3CTRL, LOONGARCH_CSR_DB3ASID,
        LOONGARCH_CSR_DB4ADDR, LOONGARCH_CSR_DB4MASK, LOONGARCH_CSR_DB4CTRL, LOONGARCH_CSR_DB4ASID,
        LOONGARCH_CSR_DB5ADDR, LOONGARCH_CSR_DB5MASK, LOONGARCH_CSR_DB5CTRL, LOONGARCH_CSR_DB5ASID,
        LOONGARCH_CSR_DB6ADDR, LOONGARCH_CSR_DB6MASK, LOONGARCH_CSR_DB6CTRL, LOONGARCH_CSR_DB6ASID,
        LOONGARCH_CSR_DB7ADDR, LOONGARCH_CSR_DB7MASK, LOONGARCH_CSR_DB7CTRL, LOONGARCH_CSR_DB7ASID,
        LOONGARCH_CSR_IB0ADDR, LOONGARCH_CSR_IB0MASK, LOONGARCH_CSR_IB0CTRL, LOONGARCH_CSR_IB0ASID,
        LOONGARCH_CSR_IB1ADDR, LOONGARCH_CSR_IB1MASK, LOONGARCH_CSR_IB1CTRL, LOONGARCH_CSR_IB1ASID,
        LOONGARCH_CSR_IB2ADDR, LOONGARCH_CSR_IB2MASK, LOONGARCH_CSR_IB2CTRL, LOONGARCH_CSR_IB2ASID,
        LOONGARCH_CSR_IB3ADDR, LOONGARCH_CSR_IB3MASK, LOONGARCH_CSR_IB3CTRL, LOONGARCH_CSR_IB3ASID,
        LOONGARCH_CSR_IB4ADDR, LOONGARCH_CSR_IB4MASK, LOONGARCH_CSR_IB4CTRL, LOONGARCH_CSR_IB4ASID,
        LOONGARCH_CSR_IB5ADDR, LOONGARCH_CSR_IB5MASK, LOONGARCH_CSR_IB5CTRL, LOONGARCH_CSR_IB5ASID,
        LOONGARCH_CSR_IB6ADDR, LOONGARCH_CSR_IB6MASK, LOONGARCH_CSR_IB6CTRL, LOONGARCH_CSR_IB6ASID,
        LOONGARCH_CSR_IB7ADDR, LOONGARCH_CSR_IB7MASK, LOONGARCH_CSR_IB7CTRL, LOONGARCH_CSR_IB7ASID,
        LOONGARCH_CSR_PERFCTRL0, LOONGARCH_CSR_PERFCNTR0, LOONGARCH_CSR_PERFCTRL1,
        LOONGARCH_CSR_PERFCNTR1, LOONGARCH_CSR_PERFCTRL2, LOONGARCH_CSR_PERFCNTR2,
        LOONGARCH_CSR_PERFCTRL3, LOONGARCH_CSR_PERFCNTR3);
    if cpu_has_msgint { hw!(LOONGARCH_CSR_IPR, LOONGARCH_CSR_ISR0, LOONGARCH_CSR_ISR1, LOONGARCH_CSR_ISR2, LOONGARCH_CSR_ISR3); }
}

unsafe fn kvm_update_vpid(vcpu: *mut kvm_vcpu, cpu: c_int) {
    let context = per_cpu_ptr((*(*vcpu).kvm).arch.vmcs, cpu);
    let mut vpid = (*context).vpid_cache + 1;
    if vpid & vpid_mask == 0 { if vpid == 0 { vpid = vpid_mask + 1; } vpid += 1; kvm_flush_tlb_all(); }
    (*context).vpid_cache = vpid; (*vcpu).arch.vpid = vpid;
}

pub unsafe fn kvm_check_vpid(vcpu: *mut kvm_vcpu) {
    let cpu = smp_processor_id(); let migrated = (*vcpu).cpu != cpu;
    let context = per_cpu_ptr((*(*vcpu).kvm).arch.vmcs, cpu);
    let ver = (*vcpu).arch.vpid & !vpid_mask; let old = (*context).vpid_cache & !vpid_mask;
    if migrated || ver != old { kvm_update_vpid(vcpu, cpu); trace_kvm_vpid_change(vcpu, (*vcpu).arch.vpid); (*vcpu).cpu = cpu; kvm_clear_request(KVM_REQ_TLB_FLUSH_GPA, vcpu); set_gcsr_llbctl(CSR_LLBCTL_WCLLB); let vpid = ((*vcpu).arch.vpid & vpid_mask) << CSR_GSTAT_GID_SHIFT; change_csr_gstat(vpid_mask << CSR_GSTAT_GID_SHIFT, vpid); }
}

pub unsafe fn kvm_init_vmcs(kvm: *mut kvm) { (*kvm).arch.vmcs = vmcs; }
pub unsafe fn kvm_arch_dev_ioctl(_filp: *mut file, _ioctl: c_uint, _arg: c_ulong) -> c_long { -ENOIOCTLCMD }

pub unsafe fn kvm_arch_enable_virtualization_cpu() -> c_int {
    let env = read_csr_gcfg(); let mut gcfg = 0;
    write_csr_gcfg(0); write_csr_gstat(0); write_csr_gintc(0); clear_csr_gtlbc(CSR_GTLBC_USETGID | CSR_GTLBC_TOTI);
    if env & CSR_GCFG_GCIP_SECURE != 0 { gcfg |= CSR_GCFG_GCI_SECURE; } if env & CSR_GCFG_MATP_ROOT != 0 { gcfg |= CSR_GCFG_MATC_ROOT; }
    write_csr_gcfg(gcfg); kvm_flush_tlb_all(); set_csr_gtlbc(CSR_GTLBC_USETGID); this_cpu_ptr(vmcs).last_vcpu = core::ptr::null_mut(); 0
}

pub unsafe fn kvm_arch_disable_virtualization_cpu() { write_csr_gcfg(0); write_csr_gstat(0); write_csr_gintc(0); clear_csr_gtlbc(CSR_GTLBC_USETGID | CSR_GTLBC_TOTI); kvm_flush_tlb_all(); }

// The remaining module initialization and teardown are expressed through the
// corresponding kernel-provided Rust bindings/macros.
unsafe fn kvm_loongarch_env_init() -> c_int {
    let mut ret: c_int = 0;
    vmcs = alloc_percpu::<kvm_context>();
    if vmcs.is_null() { pr_err!("kvm: failed to allocate percpu kvm_context\n"); return -ENOMEM; }
    kvm_loongarch_ops = kzalloc_obj::<kvm_world_switch>();
    if kvm_loongarch_ops.is_null() { free_percpu(vmcs); vmcs = core::ptr::null_mut(); return -ENOMEM; }
    (*kvm_loongarch_ops).exc_entry = kvm_exc_entry as *mut c_void;
    (*kvm_loongarch_ops).enter_guest = kvm_enter_guest as *mut c_void;
    vpid_mask = (read_csr_gstat() & CSR_GSTAT_GIDBIT) >> CSR_GSTAT_GIDBIT_SHIFT;
    if vpid_mask != 0 { vpid_mask = GENMASK(vpid_mask - 1, 0); }
    for_each_possible_cpu!(cpu, {
        let context = per_cpu_ptr(vmcs, cpu);
        (*context).vpid_cache = vpid_mask + 1;
        (*context).last_vcpu = core::ptr::null_mut();
    });
    kvm_init_gcsr_flag(); kvm_register_perf_callbacks();
    ret = kvm_loongarch_register_ipi_device(); if ret != 0 { return ret; }
    ret = kvm_loongarch_register_eiointc_device(); if ret != 0 { return ret; }
    ret = kvm_loongarch_register_pch_pic_device(); if ret != 0 { return ret; }
    if cpu_has_msgint { ret = kvm_loongarch_register_dmsintc_device(); }
    ret
}
unsafe fn kvm_loongarch_env_exit() {
    if !vmcs.is_null() { free_percpu(vmcs); }
    if !kvm_loongarch_ops.is_null() { kfree(kvm_loongarch_ops); }
    kvm_unregister_perf_callbacks();
}
unsafe fn kvm_loongarch_init() -> c_int { if !cpu_has_lvz { return -ENODEV; } let r = kvm_loongarch_env_init(); if r != 0 { return r; } kvm_init(core::mem::size_of::<kvm_vcpu>(), 0, THIS_MODULE) }
unsafe fn kvm_loongarch_exit() { kvm_exit(); kvm_loongarch_env_exit(); }

#[cfg(feature = "module")]
static kvm_feature: [cpu_feature; 2] = [cpu_feature { feature: cpu_feature(LOONGARCH_LVZ) }, cpu_feature { feature: 0 }];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
