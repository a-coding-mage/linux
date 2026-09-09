/* Direct source-level Rust translation of mips.c.  Kernel declarations and
 * configuration supplied by the surrounding tree remain external. */

const VECTORSPACING: usize = 0x100;

pub static mut kvm_trace_guest_mode_change: bool = false;

pub unsafe fn kvm_guest_mode_change_trace_reg() -> i32 {
    kvm_trace_guest_mode_change = true;
    0
}
pub unsafe fn kvm_guest_mode_change_trace_unreg() { kvm_trace_guest_mode_change = false; }

pub unsafe fn kvm_arch_vcpu_runnable(vcpu: *mut kvm_vcpu) -> i32 {
    (!(*vcpu).arch.pending_exceptions == 0) as i32
}
pub unsafe fn kvm_arch_vcpu_in_kernel(_vcpu: *mut kvm_vcpu) -> bool { false }
pub unsafe fn kvm_arch_vcpu_should_kick(_vcpu: *mut kvm_vcpu) -> i32 { 1 }
pub unsafe fn kvm_arch_enable_virtualization_cpu() -> i32 { (*kvm_mips_callbacks).enable_virtualization_cpu() }
pub unsafe fn kvm_arch_disable_virtualization_cpu() { (*kvm_mips_callbacks).disable_virtualization_cpu(); }

pub unsafe fn kvm_arch_init_vm(kvm: *mut kvm, ty: c_ulong) -> i32 {
    match ty { KVM_VM_MIPS_AUTO | KVM_VM_MIPS_VZ => (), _ => return -EINVAL }
    (*kvm).arch.gpa_mm.pgd = kvm_pgd_alloc();
    if (*kvm).arch.gpa_mm.pgd.is_null() { return -ENOMEM; }
    #[cfg(CONFIG_CPU_LOONGSON64)] { kvm_init_loongson_ipi(kvm); }
    0
}
unsafe fn kvm_mips_free_gpa_pt(kvm: *mut kvm) {
    WARN_ON(!kvm_mips_flush_gpa_pt(kvm, 0, !0));
    pgd_free(core::ptr::null_mut(), (*kvm).arch.gpa_mm.pgd);
}
pub unsafe fn kvm_arch_destroy_vm(kvm: *mut kvm) { kvm_destroy_vcpus(kvm); kvm_mips_free_gpa_pt(kvm); }
pub unsafe fn kvm_arch_dev_ioctl(_f: *mut file, _ioctl: c_uint, _arg: c_ulong) -> c_long { -ENOIOCTLCMD }
pub unsafe fn kvm_arch_flush_shadow_all(kvm: *mut kvm) { kvm_mips_flush_gpa_pt(kvm,0,!0); kvm_flush_remote_tlbs(kvm); }
pub unsafe fn kvm_arch_flush_shadow_memslot(kvm: *mut kvm, slot: *mut kvm_memory_slot) {
    spin_lock(&mut (*kvm).mmu_lock);
    kvm_mips_flush_gpa_pt(kvm, (*slot).base_gfn, (*slot).base_gfn + (*slot).npages - 1);
    kvm_flush_remote_tlbs_memslot(kvm, slot); spin_unlock(&mut (*kvm).mmu_lock);
}
pub unsafe fn kvm_arch_prepare_memory_region(_k: *mut kvm, _o: *const kvm_memory_slot, _n: *mut kvm_memory_slot, _c: kvm_mr_change) -> i32 { 0 }
pub unsafe fn kvm_arch_commit_memory_region(k: *mut kvm, old: *mut kvm_memory_slot, new: *const kvm_memory_slot, change: kvm_mr_change) {
    if change == KVM_MR_FLAGS_ONLY && ((*old).flags & KVM_MEM_LOG_DIRTY_PAGES) == 0 && ((*new).flags & KVM_MEM_LOG_DIRTY_PAGES) != 0 {
        spin_lock(&mut (*k).mmu_lock);
        let flush = kvm_mips_mkclean_gpa_pt(k, (*new).base_gfn, (*new).base_gfn + (*new).npages - 1);
        if flush != 0 { kvm_flush_remote_tlbs_memslot(k, new as *mut _); }
        spin_unlock(&mut (*k).mmu_lock);
    }
}

unsafe fn kvm_mips_comparecount_wakeup(timer: *mut hrtimer) -> hrtimer_restart {
    let v = container_of(timer, kvm_vcpu, arch.comparecount_timer);
    (*kvm_mips_callbacks).queue_timer_int(v); (*v).arch.wait = 0; rcuwait_wake_up(&mut (*v).wait);
    kvm_mips_count_timeout(v)
}
pub unsafe fn kvm_arch_vcpu_precreate(_k: *mut kvm, _id: c_uint) -> i32 { 0 }

pub unsafe fn kvm_arch_vcpu_create(vcpu: *mut kvm_vcpu) -> i32 {
    let mut err = (*kvm_mips_callbacks).vcpu_init(vcpu); if err != 0 { return err; }
    hrtimer_setup(&mut (*vcpu).arch.comparecount_timer, kvm_mips_comparecount_wakeup, CLOCK_MONOTONIC, HRTIMER_MODE_REL);
    let size = if cpu_has_veic || cpu_has_vint { 0x200 + VECTORSPACING * 64 } else { 0x4000 };
    let base = kzalloc(ALIGN(size, PAGE_SIZE), GFP_KERNEL); if base.is_null() { err=-ENOMEM; goto out; }
    if !cpu_has_ebase_wg && virt_to_phys(base) >= 0x20000000 { err=-ENOMEM; kfree(base); goto out; }
    (*vcpu).arch.guest_ebase=base;
    let handler=(base as *mut u8).add(0x2000) as *mut core::ffi::c_void;
    let start=if cfg!(CONFIG_64BIT) {(base as *mut u8).add(0x80) as *mut _} else {base};
    kvm_mips_build_tlb_refill_exception(start,handler); kvm_mips_build_exception((base as *mut u8).add(0x180) as *mut _,handler);
    for i in 0..8 { kvm_mips_build_exception((base as *mut u8).add(0x200+i*VECTORSPACING) as *mut _,handler); }
    let mut p=kvm_mips_build_exit(handler); (*vcpu).arch.vcpu_run=p; p=kvm_mips_build_vcpu_run(p);
    flush_icache_range(base as usize, base as usize + ALIGN(size,PAGE_SIZE)); (*vcpu).arch.last_sched_cpu=-1; (*vcpu).arch.last_exec_cpu=-1;
    err=(*kvm_mips_callbacks).vcpu_setup(vcpu); if err==0 { return 0; } kfree(base);
out: (*kvm_mips_callbacks).vcpu_uninit(vcpu); err
}
pub unsafe fn kvm_arch_vcpu_destroy(v: *mut kvm_vcpu) { hrtimer_cancel(&mut (*v).arch.comparecount_timer); kvm_mips_dump_stats(v); kvm_mmu_free_memory_caches(v); kfree((*v).arch.guest_ebase); (*kvm_mips_callbacks).vcpu_uninit(v); }
pub unsafe fn kvm_arch_vcpu_ioctl_set_guest_debug(_v:*mut kvm_vcpu,_d:*mut kvm_guest_debug)->i32{-ENOIOCTLCMD}

pub unsafe fn kvm_arch_vcpu_ioctl_run(v:*mut kvm_vcpu)->i32 { let mut r=-EINTR; vcpu_load(v); kvm_sigset_activate(v); if !(*v).wants_to_run { kvm_sigset_deactivate(v); vcpu_put(v); return r; } lose_fpu(1); local_irq_disable(); guest_timing_enter_irqoff(); trace_kvm_enter(v); smp_store_mb((*v).mode,IN_GUEST_MODE); r=(*kvm_mips_callbacks).vcpu_run(v); local_irq_enable(); local_irq_disable(); trace_kvm_out(v); guest_timing_exit_irqoff(); local_irq_enable(); kvm_sigset_deactivate(v); vcpu_put(v); r }

pub unsafe fn kvm_arch_vcpu_ioctl_get_mpstate(_v:*mut kvm_vcpu,_s:*mut kvm_mp_state)->i32{-ENOIOCTLCMD}
pub unsafe fn kvm_arch_vcpu_ioctl_set_mpstate(_v:*mut kvm_vcpu,_s:*mut kvm_mp_state)->i32{-ENOIOCTLCMD}
pub unsafe fn kvm_arch_sync_dirty_log(_k:*mut kvm,_s:*mut kvm_memory_slot) {}
pub unsafe fn kvm_arch_flush_remote_tlbs(k:*mut kvm)->i32 { (*kvm_mips_callbacks).prepare_flush_shadow(k); 1 }
pub unsafe fn kvm_arch_vm_ioctl(_f:*mut file,_i:c_uint,_a:c_ulong)->i32{-ENOIOCTLCMD}
pub unsafe fn kvm_arch_vcpu_ioctl_get_sregs(_v:*mut kvm_vcpu,_s:*mut kvm_sregs)->i32{-ENOIOCTLCMD}
pub unsafe fn kvm_arch_vcpu_ioctl_set_sregs(_v:*mut kvm_vcpu,_s:*mut kvm_sregs)->i32{-ENOIOCTLCMD}
pub unsafe fn kvm_arch_vcpu_postcreate(_v:*mut kvm_vcpu) {}
pub unsafe fn kvm_arch_vcpu_ioctl_get_fpu(_v:*mut kvm_vcpu,_f:*mut kvm_fpu)->i32{-ENOIOCTLCMD}
pub unsafe fn kvm_arch_vcpu_ioctl_set_fpu(_v:*mut kvm_vcpu,_f:*mut kvm_fpu)->i32{-ENOIOCTLCMD}
pub unsafe fn kvm_arch_vcpu_fault(_v:*mut kvm_vcpu,_m:*mut vm_fault)->vm_fault_t{VM_FAULT_SIGBUS}
pub unsafe fn kvm_arch_vcpu_ioctl_translate(_v:*mut kvm_vcpu,_t:*mut kvm_translation)->i32{0}

pub unsafe fn kvm_cpu_has_pending_timer(v:*mut kvm_vcpu)->i32 { (kvm_mips_pending_timer(v) != 0 || (kvm_read_c0_guest_cause(&mut (*v).arch.cop0) & C_TI)!=0) as i32 }
pub unsafe fn kvm_arch_vcpu_dump_regs(v:*mut kvm_vcpu)->i32 { if v.is_null(){return -1;} kvm_debug("VCPU Register Dump:\n"); 0 }
pub unsafe fn kvm_arch_vcpu_ioctl_set_regs(v:*mut kvm_vcpu,r:*mut kvm_regs)->i32 { vcpu_load(v); for i in 1..(*v).arch.gprs.len(){(*v).arch.gprs[i]=(*r).gpr[i];} (*v).arch.gprs[0]=0; (*v).arch.hi=(*r).hi; (*v).arch.lo=(*r).lo; (*v).arch.pc=(*r).pc; vcpu_put(v); 0 }
pub unsafe fn kvm_arch_vcpu_ioctl_get_regs(v:*mut kvm_vcpu,r:*mut kvm_regs)->i32 { vcpu_load(v); for i in 0..(*v).arch.gprs.len(){(*r).gpr[i]=(*v).arch.gprs[i];} (*r).hi=(*v).arch.hi; (*r).lo=(*v).arch.lo; (*r).pc=(*v).arch.pc; vcpu_put(v); 0 }

pub unsafe fn kvm_vcpu_ioctl_interrupt(v:*mut kvm_vcpu, irq:*mut kvm_mips_interrupt)->i32 {
    let intr=(*irq).irq as i32; let d=if (*irq).cpu == -1 {v} else {kvm_get_vcpu((*v).kvm,(*irq).cpu)};
    if intr==2||intr==3||intr==4||intr==6 { (*kvm_mips_callbacks).queue_io_int(d,irq); }
    else if intr==-2||intr==-3||intr==-4||intr==-6 { (*kvm_mips_callbacks).dequeue_io_int(d,irq); }
    else { return -EINVAL; }
    (*d).arch.wait=0; rcuwait_wake_up(&mut (*d).wait); 0
}

pub unsafe fn kvm_vm_ioctl_check_extension(k:*mut kvm, ext:c_long)->i32 {
    match ext {
        KVM_CAP_ONE_REG|KVM_CAP_ENABLE_CAP|KVM_CAP_READONLY_MEM|KVM_CAP_IMMEDIATE_EXIT=>1,
        KVM_CAP_NR_VCPUS=>min_t(num_online_cpus(),KVM_MAX_VCPUS) as i32,
        KVM_CAP_MAX_VCPUS=>KVM_MAX_VCPUS as i32,
        KVM_CAP_MAX_VCPU_ID=>KVM_MAX_VCPU_IDS as i32,
        KVM_CAP_MIPS_FPU=>(raw_cpu_has_fpu != 0) as i32,
        KVM_CAP_MIPS_MSA=>(cpu_has_msa && (boot_cpu_data.msa_id & MSA_IR_WRPF)==0) as i32,
        _=>(*kvm_mips_callbacks).check_extension(k,ext)
    }
}

pub unsafe fn kvm_own_fpu(v:*mut kvm_vcpu){ preempt_disable(); let sr=kvm_read_c0_guest_status(&mut (*v).arch.cop0); change_c0_status(ST0_CU1|ST0_FR,sr); enable_fpu_hazard(); if (*v).arch.aux_inuse&KVM_MIPS_AUX_FPU==0 {__kvm_restore_fpu(&mut (*v).arch); (*v).arch.aux_inuse|=KVM_MIPS_AUX_FPU;} preempt_enable(); }
pub unsafe fn kvm_drop_fpu(v:*mut kvm_vcpu){ preempt_disable(); if cpu_has_msa&&(*v).arch.aux_inuse&KVM_MIPS_AUX_MSA!=0 {disable_msa();(*v).arch.aux_inuse&=!KVM_MIPS_AUX_MSA;} if (*v).arch.aux_inuse&KVM_MIPS_AUX_FPU!=0 {clear_c0_status(ST0_CU1|ST0_FR);(*v).arch.aux_inuse&=!KVM_MIPS_AUX_FPU;} preempt_enable(); }
pub unsafe fn kvm_lose_fpu(v:*mut kvm_vcpu){ preempt_disable(); if cpu_has_msa&&(*v).arch.aux_inuse&KVM_MIPS_AUX_MSA!=0 {__kvm_save_msa(&mut (*v).arch);disable_msa();(*v).arch.aux_inuse&=!(KVM_MIPS_AUX_FPU|KVM_MIPS_AUX_MSA);} else if (*v).arch.aux_inuse&KVM_MIPS_AUX_FPU!=0 {__kvm_save_fpu(&mut (*v).arch);(*v).arch.aux_inuse&=!KVM_MIPS_AUX_FPU;clear_c0_status(ST0_CU1|ST0_FR);disable_fpu_hazard();} preempt_enable(); }

pub unsafe fn kvm_irq_to_priority(irq:u32)->u32 { for i in MIPS_EXC_INT_TIMER..MIPS_EXC_MAX { if (*kvm_priority_to_irq.add(i as usize))==(1u32<<(irq+8)){return i;} } MIPS_EXC_MAX }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
