// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2010,2012 Freescale Semiconductor, Inc. All rights reserved.
 *
 * Author: Varun Sethi, <varun.sethi@freescale.com>
 *
 * Description:
 * This file is derived from arch/powerpc/kvm/e500.c,
 * by Yu Liu <yu.liu@freescale.com>.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// Dependencies supplied by the surrounding kernel translation.
use crate::*;

pub unsafe fn kvmppc_set_pending_interrupt(vcpu: *mut kvm_vcpu, ty: int_class) {
    let dbell_type: ppc_dbell;
    match ty {
        INT_CLASS_NONCRIT => dbell_type = PPC_G_DBELL,
        INT_CLASS_CRIT => dbell_type = PPC_G_DBELL_CRIT,
        INT_CLASS_MC => dbell_type = PPC_G_DBELL_MC,
        _ => { WARN_ONCE!(1, "%s: unknown int type %d\n", "kvmppc_set_pending_interrupt", ty); return; }
    }
    preempt_disable();
    let tag = PPC_DBELL_LPID(get_lpid(vcpu)) | (*vcpu).vcpu_id;
    mb();
    ppc_msgsnd(dbell_type, 0, tag);
    preempt_enable();
}

/* gtlbe must not be mapped by more than one host tlb entry */
pub unsafe fn kvmppc_e500_tlbil_one(vcpu_e500: *mut kvmppc_vcpu_e500, gtlbe: *mut kvm_book3e_206_tlb_entry) {
    let tid = get_tlb_tid(gtlbe); let ts = get_tlb_ts(gtlbe);
    let eaddr = get_tlb_eaddr(gtlbe); let mut val: u32 = (tid << 16) | ts; let mut flags = 0UL;
    local_irq_save(&mut flags);
    mtspr(SPRN_MAS6, val as _); mtspr(SPRN_MAS5, MAS5_SGS | get_lpid(&mut (*vcpu_e500).vcpu));
    asm!("tlbsx 0, {eaddr}", eaddr = in(reg) eaddr);
    val = mfspr(SPRN_MAS1) as u32;
    if val & MAS1_VALID != 0 { mtspr(SPRN_MAS1, (val & !MAS1_VALID) as _); asm!("tlbwe"); }
    mtspr(SPRN_MAS5, 0); mtspr(SPRN_MAS8, 0); isync(); local_irq_restore(flags);
}

pub unsafe fn kvmppc_e500_tlbil_all(vcpu_e500: *mut kvmppc_vcpu_e500) {
    let mut flags = 0UL; local_irq_save(&mut flags);
    mtspr(SPRN_MAS5, MAS5_SGS | get_lpid(&mut (*vcpu_e500).vcpu));
    // clang-17 and older could not assemble tlbilxlpid.
    asm!(PPC_TLBILX_LPID); mtspr(SPRN_MAS5, 0); local_irq_restore(flags);
}

pub unsafe fn kvmppc_set_pid(vcpu: *mut kvm_vcpu, pid: u32) { (*vcpu).arch.pid = pid; }
pub unsafe fn kvmppc_mmu_msr_notify(_vcpu: *mut kvm_vcpu, _old_msr: u32) {}

// CONFIG-dependent per-CPU storage.
static mut last_vcpu_of_lpid: [[*mut kvm_vcpu; KVMPPC_NR_LPIDS]; 1] = [[core::ptr::null_mut(); KVMPPC_NR_LPIDS]; 1];

unsafe fn kvmppc_core_vcpu_load_e500mc(vcpu: *mut kvm_vcpu, cpu: i32) {
    let e = to_e500(vcpu); kvmppc_booke_vcpu_load(vcpu, cpu);
    mtspr(SPRN_LPID, get_lpid(vcpu)); mtspr(SPRN_EPCR, (*vcpu).arch.shadow_epcr); mtspr(SPRN_GPIR, (*vcpu).vcpu_id); mtspr(SPRN_MSRP, (*vcpu).arch.shadow_msrp);
    (*vcpu).arch.eplc = EPC_EGS | (get_lpid(vcpu) << EPC_ELPID_SHIFT); (*vcpu).arch.epsc = (*vcpu).arch.eplc; mtspr(SPRN_EPLC, (*vcpu).arch.eplc); mtspr(SPRN_EPSC, (*vcpu).arch.epsc);
    mtspr(SPRN_GIVPR, (*vcpu).arch.ivpr); mtspr(SPRN_GIVOR2, (*vcpu).arch.ivor[BOOKE_IRQPRIO_DATA_STORAGE]); mtspr(SPRN_GIVOR8, (*vcpu).arch.ivor[BOOKE_IRQPRIO_SYSCALL]);
    mtspr(SPRN_GSPRG0, (*(*vcpu).arch.shared).sprg0 as _); mtspr(SPRN_GSPRG1, (*(*vcpu).arch.shared).sprg1 as _); mtspr(SPRN_GSPRG2, (*(*vcpu).arch.shared).sprg2 as _); mtspr(SPRN_GSPRG3, (*(*vcpu).arch.shared).sprg3 as _);
    mtspr(SPRN_GSRR0, (*(*vcpu).arch.shared).srr0); mtspr(SPRN_GSRR1, (*(*vcpu).arch.shared).srr1); mtspr(SPRN_GEPR, (*vcpu).arch.epr); mtspr(SPRN_GDEAR, (*(*vcpu).arch.shared).dar); mtspr(SPRN_GESR, (*(*vcpu).arch.shared).esr);
    if (*vcpu).arch.oldpir != mfspr(SPRN_PIR) || last_vcpu_of_lpid[0][get_lpid(vcpu) as usize] != vcpu { kvmppc_e500_tlbil_all(e); last_vcpu_of_lpid[0][get_lpid(vcpu) as usize] = vcpu; }
}

unsafe fn kvmppc_core_vcpu_put_e500mc(vcpu: *mut kvm_vcpu) {
    (*vcpu).arch.eplc=mfspr(SPRN_EPLC); (*vcpu).arch.epsc=mfspr(SPRN_EPSC); (*(*vcpu).arch.shared).sprg0=mfspr(SPRN_GSPRG0); (*(*vcpu).arch.shared).sprg1=mfspr(SPRN_GSPRG1); (*(*vcpu).arch.shared).sprg2=mfspr(SPRN_GSPRG2); (*(*vcpu).arch.shared).sprg3=mfspr(SPRN_GSPRG3); (*(*vcpu).arch.shared).srr0=mfspr(SPRN_GSRR0); (*(*vcpu).arch.shared).srr1=mfspr(SPRN_GSRR1); (*vcpu).arch.epr=mfspr(SPRN_GEPR); (*(*vcpu).arch.shared).dar=mfspr(SPRN_GDEAR); (*(*vcpu).arch.shared).esr=mfspr(SPRN_GESR); (*vcpu).arch.oldpir=mfspr(SPRN_PIR); kvmppc_booke_vcpu_put(vcpu);
}

unsafe fn kvmppc_e500mc_check_processor_compat() -> i32 {
    if strcmp(cur_cpu_spec.cpu_name, "e500mc") == 0 || strcmp(cur_cpu_spec.cpu_name, "e5500") == 0 { 0 } else { -ENOTSUPP }
}

pub unsafe fn kvmppc_core_vcpu_setup(vcpu: *mut kvm_vcpu) -> i32 {
    let e=to_e500(vcpu); (*vcpu).arch.shadow_epcr=SPRN_EPCR_DSIGS|SPRN_EPCR_DGTMI|SPRN_EPCR_DUVD; (*vcpu).arch.shadow_msrp=MSRP_UCLEP|MSRP_PMMP; (*vcpu).arch.pvr=mfspr(SPRN_PVR); (*e).svr=mfspr(SPRN_SVR); (*vcpu).arch.cpu_type=KVM_CPU_E500MC; 0
}

unsafe fn kvmppc_core_get_sregs_e500mc(vcpu:*mut kvm_vcpu,sregs:*mut kvm_sregs)->i32 { let e=to_e500(vcpu); (*sregs).u.e.features|=KVM_SREGS_E_ARCH206_MMU|KVM_SREGS_E_PM|KVM_SREGS_E_PC; (*sregs).u.e.impl_id=KVM_SREGS_E_IMPL_FSL; (*sregs).u.e.impl.fsl.features=0; (*sregs).u.e.impl.fsl.svr=(*e).svr; (*sregs).u.e.impl.fsl.hid0=(*e).hid0; (*sregs).u.e.impl.fsl.mcar=(*e).mcar; kvmppc_get_sregs_e500_tlb(vcpu,sregs); (*sregs).u.e.ivor_high[3]=(*vcpu).arch.ivor[BOOKE_IRQPRIO_PERFORMANCE_MONITOR]; (*sregs).u.e.ivor_high[4]=(*vcpu).arch.ivor[BOOKE_IRQPRIO_DBELL]; (*sregs).u.e.ivor_high[5]=(*vcpu).arch.ivor[BOOKE_IRQPRIO_DBELL_CRIT]; kvmppc_get_sregs_ivor(vcpu,sregs) }

// The remaining operations retain the C ABI-facing callback structure and external helper calls.
unsafe fn kvmppc_core_set_sregs_e500mc(vcpu:*mut kvm_vcpu,sregs:*mut kvm_sregs)->i32 { let e=to_e500(vcpu); if (*sregs).u.e.impl_id==KVM_SREGS_E_IMPL_FSL { (*e).svr=(*sregs).u.e.impl.fsl.svr; (*e).hid0=(*sregs).u.e.impl.fsl.hid0; (*e).mcar=(*sregs).u.e.impl.fsl.mcar; } let r=kvmppc_set_sregs_e500_tlb(vcpu,sregs); if r<0{return r} if (*sregs).u.e.features&KVM_SREGS_E_IVOR==0{return 0} if (*sregs).u.e.features&KVM_SREGS_E_PM!=0 {(*vcpu).arch.ivor[BOOKE_IRQPRIO_PERFORMANCE_MONITOR]=(*sregs).u.e.ivor_high[3]} if (*sregs).u.e.features&KVM_SREGS_E_PC!=0 {(*vcpu).arch.ivor[BOOKE_IRQPRIO_DBELL]=(*sregs).u.e.ivor_high[4];(*vcpu).arch.ivor[BOOKE_IRQPRIO_DBELL_CRIT]=(*sregs).u.e.ivor_high[5]} kvmppc_set_sregs_ivor(vcpu,sregs) }

unsafe fn kvmppc_get_one_reg_e500mc(v:*mut kvm_vcpu,id:u64,val:*mut kvmppc_one_reg)->i32 { if id==KVM_REG_PPC_SPRG9 {*val=get_reg_val(id,(*v).arch.sprg9);0} else {kvmppc_get_one_reg_e500_tlb(v,id,val)} }
unsafe fn kvmppc_set_one_reg_e500mc(v:*mut kvm_vcpu,id:u64,val:*mut kvmppc_one_reg)->i32 { if id==KVM_REG_PPC_SPRG9 {(*v).arch.sprg9=set_reg_val(id,*val);0} else {kvmppc_set_one_reg_e500_tlb(v,id,val)} }

// VM and VCPU lifecycle callbacks are direct translations of the corresponding C callbacks.
unsafe fn kvmppc_core_vcpu_create_e500mc(v:*mut kvm_vcpu)->i32 { let e=to_e500(v); (*v).arch.oldpir=0xffffffff; let r=kvmppc_e500_tlb_init(e); if r!=0{return r} (*v).arch.shared=__get_free_page(GFP_KERNEL|__GFP_ZERO) as _; if (*v).arch.shared.is_null(){kvmppc_e500_tlb_uninit(e);return -ENOMEM} 0 }
unsafe fn kvmppc_core_vcpu_free_e500mc(v:*mut kvm_vcpu){let e=to_e500(v);free_page((*v).arch.shared as _);kvmppc_e500_tlb_uninit(e)}
unsafe fn kvmppc_core_init_vm_e500mc(k:*mut kvm)->i32{let mut l=kvmppc_alloc_lpid();if l<0{return l}if threads_per_core==2{l<<=1}(*k).arch.lpid=l;0}
unsafe fn kvmppc_core_destroy_vm_e500mc(k:*mut kvm){let mut l=(*k).arch.lpid;if threads_per_core==2{l>>=1}kvmppc_free_lpid(l)}

static mut kvm_ops_e500mc: kvmppc_ops = kvmppc_ops {
    get_sregs: Some(kvmppc_core_get_sregs_e500mc), set_sregs: Some(kvmppc_core_set_sregs_e500mc),
    get_one_reg: Some(kvmppc_get_one_reg_e500mc), set_one_reg: Some(kvmppc_set_one_reg_e500mc),
    vcpu_load: Some(kvmppc_core_vcpu_load_e500mc), vcpu_put: Some(kvmppc_core_vcpu_put_e500mc),
    vcpu_create: Some(kvmppc_core_vcpu_create_e500mc), vcpu_free: Some(kvmppc_core_vcpu_free_e500mc),
    init_vm: Some(kvmppc_core_init_vm_e500mc), destroy_vm: Some(kvmppc_core_destroy_vm_e500mc),
    emulate_op: Some(kvmppc_core_emulate_op_e500), emulate_mtspr: Some(kvmppc_core_emulate_mtspr_e500),
    emulate_mfspr: Some(kvmppc_core_emulate_mfspr_e500), create_vcpu_debugfs: Some(kvmppc_create_vcpu_debugfs_e500),
    owner: core::ptr::null_mut(),
};

#[no_mangle] pub unsafe extern "C" fn kvmppc_e500mc_init()->i32 {let r=kvmppc_e500mc_check_processor_compat();if r!=0{return r}let r=kvmppc_booke_init();if r!=0{return r}kvmppc_init_lpid(KVMPPC_NR_LPIDS/threads_per_core);kvm_init(core::mem::size_of::<kvmppc_vcpu_e500>(),0,THIS_MODULE)}
#[no_mangle] pub unsafe extern "C" fn kvmppc_e500mc_exit(){kvmppc_pr_ops=core::ptr::null_mut();kvmppc_booke_exit()}

// Equivalent module registration metadata: module_init, module_exit,
// MODULE_ALIAS_MISCDEV(KVM_MINOR), and MODULE_ALIAS("devname:kvm").

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
