// SPDX-License-Identifier: GPL-2.0-only
// Translated from book3s_hv_p9_entry.c; kernel dependencies are supplied externally.

unsafe fn load_spr_state(vcpu: *mut kvm_vcpu, host: *mut p9_host_os_sprs) {
    mtspr(SPRN_TAR, (*vcpu).arch.tar);
    #[cfg(CONFIG_ALTIVEC)]
    if cpu_has_feature(CPU_FTR_ALTIVEC) && current.thread.vrsave != (*vcpu).arch.vrsave { mtspr(SPRN_VRSAVE, (*vcpu).arch.vrsave); }
    if (*vcpu).arch.hfscr & HFSCR_EBB != 0 {
        if current.thread.ebbhr != (*vcpu).arch.ebbhr { mtspr(SPRN_EBBHR, (*vcpu).arch.ebbhr); }
        if current.thread.ebbrr != (*vcpu).arch.ebbrr { mtspr(SPRN_EBBRR, (*vcpu).arch.ebbrr); }
        if current.thread.bescr != (*vcpu).arch.bescr { mtspr(SPRN_BESCR, (*vcpu).arch.bescr); }
    }
    if cpu_has_feature(CPU_FTR_P9_TIDR) && current.thread.tidr != (*vcpu).arch.tid { mtspr(SPRN_TIDR, (*vcpu).arch.tid); }
    if (*host).iamr != (*vcpu).arch.iamr { mtspr(SPRN_IAMR, (*vcpu).arch.iamr); }
    if (*host).amr != (*vcpu).arch.amr { mtspr(SPRN_AMR, (*vcpu).arch.amr); }
    if (*vcpu).arch.uamor != 0 { mtspr(SPRN_UAMOR, (*vcpu).arch.uamor); }
    if current.thread.fscr != (*vcpu).arch.fscr { mtspr(SPRN_FSCR, (*vcpu).arch.fscr); }
    if current.thread.dscr != (*vcpu).arch.dscr { mtspr(SPRN_DSCR, (*vcpu).arch.dscr); }
    if (*vcpu).arch.pspb != 0 { mtspr(SPRN_PSPB, (*vcpu).arch.pspb); }
    if (*vcpu).arch.ctrl & 1 == 0 { mtspr(SPRN_CTRLT, 0); }
}

unsafe fn store_spr_state(vcpu: *mut kvm_vcpu) {
    (*vcpu).arch.tar = mfspr(SPRN_TAR);
    #[cfg(CONFIG_ALTIVEC)] if cpu_has_feature(CPU_FTR_ALTIVEC) { (*vcpu).arch.vrsave = mfspr(SPRN_VRSAVE); }
    if (*vcpu).arch.hfscr & HFSCR_EBB != 0 { (*vcpu).arch.ebbhr = mfspr(SPRN_EBBHR); (*vcpu).arch.ebbrr = mfspr(SPRN_EBBRR); (*vcpu).arch.bescr = mfspr(SPRN_BESCR); }
    if cpu_has_feature(CPU_FTR_P9_TIDR) { (*vcpu).arch.tid = mfspr(SPRN_TIDR); }
    (*vcpu).arch.iamr = mfspr(SPRN_IAMR); (*vcpu).arch.amr = mfspr(SPRN_AMR); (*vcpu).arch.uamor = mfspr(SPRN_UAMOR);
    (*vcpu).arch.fscr = mfspr(SPRN_FSCR); (*vcpu).arch.dscr = mfspr(SPRN_DSCR); (*vcpu).arch.pspb = mfspr(SPRN_PSPB); (*vcpu).arch.ctrl = mfspr(SPRN_CTRLF);
}

pub unsafe fn load_vcpu_state(vcpu: *mut kvm_vcpu, host: *mut p9_host_os_sprs) -> bool {
    let mut ret = false;
    #[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
    if cpu_has_feature(CPU_FTR_TM) || cpu_has_feature(CPU_FTR_P9_TM_HV_ASSIST) {
        let guest_msr = (*vcpu).arch.shregs.msr;
        if MSR_TM_ACTIVE(guest_msr) { kvmppc_restore_tm_hv(vcpu, guest_msr, true); ret = true; }
        else if (*vcpu).arch.hfscr & HFSCR_TM != 0 { mtspr(SPRN_TEXASR, (*vcpu).arch.texasr); mtspr(SPRN_TFHAR, (*vcpu).arch.tfhar); mtspr(SPRN_TFIAR, (*vcpu).arch.tfiar); }
    }
    load_spr_state(vcpu, host); load_fp_state(&mut (*vcpu).arch.fp);
    #[cfg(CONFIG_ALTIVEC)] load_vr_state(&mut (*vcpu).arch.vr);
    ret
}

pub unsafe fn store_vcpu_state(vcpu: *mut kvm_vcpu) {
    store_spr_state(vcpu); store_fp_state(&mut (*vcpu).arch.fp);
    #[cfg(CONFIG_ALTIVEC)] store_vr_state(&mut (*vcpu).arch.vr);
    #[cfg(CONFIG_PPC_TRANSACTIONAL_MEM)]
    if cpu_has_feature(CPU_FTR_TM) || cpu_has_feature(CPU_FTR_P9_TM_HV_ASSIST) {
        let guest_msr = (*vcpu).arch.shregs.msr;
        if MSR_TM_ACTIVE(guest_msr) { kvmppc_save_tm_hv(vcpu, guest_msr, true); }
        else if (*vcpu).arch.hfscr & HFSCR_TM != 0 {
            (*vcpu).arch.texasr = mfspr(SPRN_TEXASR); (*vcpu).arch.tfhar = mfspr(SPRN_TFHAR); (*vcpu).arch.tfiar = mfspr(SPRN_TFIAR);
            if !(*vcpu).arch.nested { (*vcpu).arch.load_tm = (*vcpu).arch.load_tm.wrapping_add(1); if (*vcpu).arch.load_tm == 0 { (*vcpu).arch.hfscr &= !HFSCR_TM; } }
        }
    }
}

pub unsafe fn save_p9_host_os_sprs(host: *mut p9_host_os_sprs) { (*host).iamr = mfspr(SPRN_IAMR); (*host).amr = mfspr(SPRN_AMR); }

pub unsafe fn restore_p9_host_os_sprs(vcpu: *mut kvm_vcpu, host: *mut p9_host_os_sprs) {
    mtspr(SPRN_SPRG_VDSO_WRITE, local_paca.sprg_vdso);
    if cpu_has_feature(CPU_FTR_P9_TIDR) && current.thread.tidr != (*vcpu).arch.tid { mtspr(SPRN_TIDR, current.thread.tidr); }
    if (*host).iamr != (*vcpu).arch.iamr { mtspr(SPRN_IAMR, (*host).iamr); }
    if (*vcpu).arch.uamor != 0 { mtspr(SPRN_UAMOR, 0); }
    if (*host).amr != (*vcpu).arch.amr { mtspr(SPRN_AMR, (*host).amr); }
    if current.thread.fscr != (*vcpu).arch.fscr { mtspr(SPRN_FSCR, current.thread.fscr); }
    if current.thread.dscr != (*vcpu).arch.dscr { mtspr(SPRN_DSCR, current.thread.dscr); }
    if (*vcpu).arch.pspb != 0 { mtspr(SPRN_PSPB, 0); }
    if (*vcpu).arch.ctrl & 1 == 0 { mtspr(SPRN_CTRLT, 1); }
    #[cfg(CONFIG_ALTIVEC)] if cpu_has_feature(CPU_FTR_ALTIVEC) && (*vcpu).arch.vrsave != current.thread.vrsave { mtspr(SPRN_VRSAVE, current.thread.vrsave); }
    if (*vcpu).arch.hfscr & HFSCR_EBB != 0 {
        if (*vcpu).arch.bescr != current.thread.bescr { mtspr(SPRN_BESCR, current.thread.bescr); }
        if (*vcpu).arch.ebbhr != current.thread.ebbhr { mtspr(SPRN_EBBHR, current.thread.ebbhr); }
        if (*vcpu).arch.ebbrr != current.thread.ebbrr { mtspr(SPRN_EBBRR, current.thread.ebbrr); }
        if !(*vcpu).arch.nested { (*vcpu).arch.load_ebb = (*vcpu).arch.load_ebb.wrapping_add(1); if (*vcpu).arch.load_ebb == 0 { (*vcpu).arch.hfscr &= !HFSCR_EBB; } }
    }
    if (*vcpu).arch.tar != current.thread.tar { mtspr(SPRN_TAR, current.thread.tar); }
}

#[cfg(CONFIG_KVM_BOOK3S_HV_P9_TIMING)]
pub unsafe fn accumulate_time(vcpu: *mut kvm_vcpu, next: *mut kvmhv_tb_accumulator) {
    let vc = (*vcpu).arch.vcore; let curr = (*vcpu).arch.cur_activity; let tb = mftb() - (*vc).tb_offset_applied; let prev = (*vcpu).arch.cur_tb_start;
    (*vcpu).arch.cur_activity = next; (*vcpu).arch.cur_tb_start = tb; if curr.is_null() { return; }
    let delta = tb - prev; let seq = (*curr).seqcount; (*curr).seqcount = seq + 1; smp_wmb(); (*curr).tb_total += delta;
    if seq == 0 || delta < (*curr).tb_min { (*curr).tb_min = delta; } if delta > (*curr).tb_max { (*curr).tb_max = delta; } smp_wmb(); (*curr).seqcount = seq + 2;
}

#[inline] unsafe fn mfslbv(idx: u32) -> u64 { let mut v; core::arch::asm!("slbmfev {0},{1}", out(reg) v, in(reg) idx); v }
#[inline] unsafe fn mfslbe(idx: u32) -> u64 { let mut v; core::arch::asm!("slbmfee {0},{1}", out(reg) v, in(reg) idx); v }
#[inline] unsafe fn mtslb(slbee: u64, slbev: u64) { core::arch::asm!("slbmte {0},{1}", in(reg) slbev, in(reg) slbee); }
#[inline] unsafe fn clear_slb_entry(idx: u32) { mtslb(idx as u64, 0); }
#[inline] unsafe fn slb_clear_invalidate_partition() { clear_slb_entry(0); core::arch::asm!(PPC_SLBIA!(6)); }
unsafe fn radix_clear_slb() { for i in 0..4 { clear_slb_entry(i); } }

unsafe fn switch_mmu_to_guest_radix(kvm: *mut kvm, vcpu: *mut kvm_vcpu, lpcr: u64) { let n = (*vcpu).arch.nested; let lpid = if !n.is_null() { (*n).shadow_lpid } else { (*kvm).arch.lpid }; let pid = kvmppc_get_pid(vcpu); core::arch::asm!("hwsync"); isync(); mtspr(SPRN_LPID,lpid); mtspr(SPRN_LPCR,lpcr); mtspr(SPRN_PID,pid); }
unsafe fn switch_mmu_to_guest_hpt(kvm: *mut kvm, vcpu: *mut kvm_vcpu, lpcr: u64) { let lpid=(*kvm).arch.lpid; let pid=kvmppc_get_pid(vcpu); core::arch::asm!("hwsync"); isync(); mtspr(SPRN_LPID,lpid); mtspr(SPRN_LPCR,lpcr); mtspr(SPRN_PID,pid); for i in 0..(*vcpu).arch.slb_max { mtslb((*vcpu).arch.slb[i].orige,(*vcpu).arch.slb[i].origv); } }
unsafe fn switch_mmu_to_host(kvm: *mut kvm, pid: u32) { core::arch::asm!("hwsync"); isync(); mtspr(SPRN_PID,pid); mtspr(SPRN_LPID,(*kvm).arch.host_lpid); mtspr(SPRN_LPCR,(*kvm).arch.host_lpcr); if !radix_enabled() { slb_restore_bolted_realmode(); } }
unsafe fn save_clear_host_mmu(_: *mut kvm) { if !radix_enabled() { slb_clear_invalidate_partition(); } }
unsafe fn save_clear_guest_mmu(kvm: *mut kvm, vcpu: *mut kvm_vcpu) { if kvm_is_radix(kvm) { radix_clear_slb(); } else { let mut nr=0; for i in 0..(*vcpu).arch.slb_nr { let e=mfslbe(i); if e & SLB_ESID_V != 0 { (*vcpu).arch.slb[nr].orige=e|i as u64; (*vcpu).arch.slb[nr].origv=mfslbv(i); nr+=1; } } (*vcpu).arch.slb_max=nr; slb_clear_invalidate_partition(); } }

unsafe fn flush_guest_tlb(kvm: *mut kvm) { let mut rb=PPC_BIT(52); if kvm_is_radix(kvm) { core::arch::asm!(PPC_TLBIEL!(rb,0,1,1,2)); for _ in 1..(*kvm).arch.tlb_sets { rb+=PPC_BIT(51); core::arch::asm!(PPC_TLBIEL!(rb,0,1,1,0)); } core::arch::asm!("ptesync"); core::arch::asm!(PPC_RADIX_INVALIDATE_ERAT_GUEST!()); } else { for _ in 0..(*kvm).arch.tlb_sets { core::arch::asm!(PPC_TLBIEL!(rb,0,0,0,0)); rb+=PPC_BIT(51); } core::arch::asm!("ptesync"); core::arch::asm!(PPC_ISA_3_0_INVALIDATE_ERAT!()); } }
unsafe fn check_need_tlb_flush(kvm:*mut kvm, pcpu:i32, nested:*mut kvm_nested_guest) { let mask=if !nested.is_null(){&mut (*nested).need_tlb_flush}else{&mut (*kvm).arch.need_tlb_flush}; if !cpumask_test_cpu(pcpu,mask){return;} let mut all=true; let mut i=cpu_first_tlb_thread_sibling(pcpu); while i<=cpu_last_tlb_thread_sibling(pcpu){if !cpumask_test_cpu(i,mask){all=false;break;} i+=cpu_tlb_thread_sibling_step();} if all{flush_guest_tlb(kvm)}else{core::arch::asm!("ptesync");} cpumask_clear_cpu(pcpu,mask); }

pub unsafe fn kvmppc_msr_hard_disable_set_facilities(vcpu:*mut kvm_vcpu, mut msr:usize)->usize { let mut needed=0; msr&=!MSR_EE; if IS_ENABLED(CONFIG_PPC_FPU){needed|=MSR_FP;} if cpu_has_feature(CPU_FTR_ALTIVEC){needed|=MSR_VEC;} if cpu_has_feature(CPU_FTR_VSX){needed|=MSR_VSX;} if (cpu_has_feature(CPU_FTR_TM)||cpu_has_feature(CPU_FTR_P9_TM_HV_ASSIST))&&((*vcpu).arch.hfscr&HFSCR_TM!=0){needed|=MSR_TM;} if msr&needed!=needed{msr|=needed;__mtmsrd(msr,0)}else{__hard_irq_disable();} local_paca.irq_happened|=PACA_IRQ_HARD_DIS; msr }

// The remaining entry routine is kept as a direct unsafe translation of the kernel ABI.
// Its register-save, MMU-switch, trap-dispatch, and restore sequence is expressed below.
pub unsafe fn kvmhv_vcpu_entry_p9(vcpu:*mut kvm_vcpu, time_limit:u64, lpcr:usize, tb:*mut u64)->i32 {
    let hdec=time_limit-*tb; if (hdec as i64)<0{return BOOK3S_INTERRUPT_HV_DECREMENTER;}
    let kvm=(*vcpu).kvm; let nested=(*vcpu).arch.nested; let vc=(*vcpu).arch.vcore; let mut host=p9_host_os_sprs{iamr:0,amr:0};
    (*vcpu).arch.ceded=0; let mut msr=mfmsr()&!MSR_EE; let host_hfscr=mfspr(SPRN_HFSCR); let host_ciabr=mfspr(SPRN_CIABR); let host_psscr=mfspr(SPRN_PSSCR_PR); let host_pidr=mfspr(SPRN_PID);
    let host_dawr0=if dawr_enabled(){mfspr(SPRN_DAWR0)}else{0}; let host_dawrx0=if dawr_enabled(){mfspr(SPRN_DAWRX0)}else{0}; save_p9_host_os_sprs(&mut host);
    msr=kvmppc_msr_hard_disable_set_facilities(vcpu,msr); if lazy_irq_pending(){return 0;} if load_vcpu_state(vcpu,&mut host){msr=mfmsr();}
    mtspr(SPRN_VTB,(*vc).vtb); mtspr(SPRN_PURR,(*vcpu).arch.purr); mtspr(SPRN_SPURR,(*vcpu).arch.spurr); mtspr(SPRN_HFSCR,(*vcpu).arch.hfscr); mtspr(SPRN_HSRR0,(*vcpu).arch.regs.nip); mtspr(SPRN_HSRR1,((*vcpu).arch.shregs.msr&!MSR_HV)|MSR_ME);
    WRITE_ONCE(local_paca.kvm_hstate.in_guest,KVM_GUEST_MODE_HV_P9); barrier(); if !radix_enabled()||!kvm_is_radix(kvm)||cpu_has_feature(CPU_FTR_P9_RADIX_PREFETCH_BUG){__mtmsrd(msr&!(MSR_IR|MSR_DR|MSR_RI),0);} save_clear_host_mmu(kvm); if kvm_is_radix(kvm){switch_mmu_to_guest_radix(kvm,vcpu,lpcr as u64)}else{switch_mmu_to_guest_hpt(kvm,vcpu,lpcr as u64)} check_need_tlb_flush(kvm,(*vc).pcpu,nested); mtspr(SPRN_HDEC,hdec); mtspr(SPRN_DEC,(*vcpu).arch.dec_expires-*tb); mtspr(SPRN_DAR,(*vcpu).arch.shregs.dar); mtspr(SPRN_DSISR,(*vcpu).arch.shregs.dsisr); mtspr(SPRN_SRR0,(*vcpu).arch.shregs.srr0); mtspr(SPRN_SRR1,(*vcpu).arch.shregs.srr1);
    switch_pmu_to_guest(vcpu,&mut host); accumulate_time(vcpu,&mut (*vcpu).arch.in_guest); kvmppc_p9_enter_guest(vcpu); accumulate_time(vcpu,&mut (*vcpu).arch.guest_exit); switch_pmu_to_host(vcpu,&mut host);
    let trap=local_paca.kvm_hstate.scratch0&!2; let exsave=if trap>BOOK3S_INTERRUPT_MACHINE_CHECK{local_paca.exgen}else if trap==BOOK3S_INTERRUPT_SYSTEM_RESET{local_paca.exnmi}else{local_paca.exmc}; (*vcpu).arch.regs.gpr[1]=local_paca.kvm_hstate.scratch1; (*vcpu).arch.regs.gpr[3]=local_paca.kvm_hstate.scratch2; (*vcpu).arch.regs.gpr[9]=exsave[EX_R9/8]; (*vcpu).arch.regs.gpr[10]=exsave[EX_R10/8]; (*vcpu).arch.regs.gpr[11]=exsave[EX_R11/8]; (*vcpu).arch.regs.gpr[12]=exsave[EX_R12/8]; (*vcpu).arch.regs.gpr[13]=exsave[EX_R13/8]; (*vcpu).arch.ppr=exsave[EX_PPR/8]; (*vcpu).arch.cfar=exsave[EX_CFAR/8]; (*vcpu).arch.regs.ctr=exsave[EX_CTR/8];
    if trap==BOOK3S_INTERRUPT_MACHINE_CHECK{(*vcpu).arch.fault_dar=exsave[EX_DAR/8];(*vcpu).arch.fault_dsisr=exsave[EX_DSISR/8];kvmppc_realmode_machine_check(vcpu)}else if trap==BOOK3S_INTERRUPT_HMI{kvmppc_p9_realmode_hmi_handler(vcpu)}else if trap==BOOK3S_INTERRUPT_H_EMUL_ASSIST{(*vcpu).arch.emul_inst=mfspr(SPRN_HEIR)}else if trap==BOOK3S_INTERRUPT_H_DATA_STORAGE{(*vcpu).arch.fault_dar=exsave[EX_DAR/8];(*vcpu).arch.fault_dsisr=exsave[EX_DSISR/8];(*vcpu).arch.fault_gpa=mfspr(SPRN_ASDR)}else if trap==BOOK3S_INTERRUPT_H_INST_STORAGE{(*vcpu).arch.fault_gpa=mfspr(SPRN_ASDR)}else if trap==BOOK3S_INTERRUPT_H_FAC_UNAVAIL{(*vcpu).arch.hfscr=mfspr(SPRN_HFSCR)}
    (*vcpu).arch.purr=mfspr(SPRN_PURR);(*vcpu).arch.spurr=mfspr(SPRN_SPURR);(*vcpu).arch.ic=mfspr(SPRN_IC);(*vcpu).arch.pid=mfspr(SPRN_PID);(*vcpu).arch.psscr=mfspr(SPRN_PSSCR_PR); save_clear_guest_mmu(kvm,vcpu); switch_mmu_to_host(kvm,host_pidr); __mtmsrd(msr,0); store_vcpu_state(vcpu); mtspr(SPRN_PURR,local_paca.kvm_hstate.host_purr);mtspr(SPRN_SPURR,local_paca.kvm_hstate.host_spurr);mtspr(SPRN_HFSCR,host_hfscr);if (*vcpu).arch.ciabr!=host_ciabr{mtspr(SPRN_CIABR,host_ciabr)} if dawr_enabled(){mtspr(SPRN_DAWR0,host_dawr0);mtspr(SPRN_DAWRX0,host_dawrx0)} timer_rearm_host_dec(*tb); restore_p9_host_os_sprs(vcpu,&mut host); barrier();WRITE_ONCE(local_paca.kvm_hstate.in_guest,KVM_GUEST_MODE_NONE); trap
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
