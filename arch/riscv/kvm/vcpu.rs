// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Anup Patel <anup.patel@wdc.com>
 */

// Linux/RISC-V dependencies are supplied by the surrounding kernel bindings.

static mut KVM_FORMER_VCPU: *mut kvm_vcpu = core::ptr::null_mut();

pub unsafe fn kvm_riscv_clear_former_vcpu() {
    /* Clear the per-CPU former VCPU pointer because hypervisor CSR state will be lost. */
    KVM_FORMER_VCPU = core::ptr::null_mut();
}

pub static KVM_VCPU_STATS_DESC: [kvm_stats_desc; 14] = [
    KVM_GENERIC_VCPU_STATS!(),
    STATS_DESC_COUNTER!(VCPU, ecall_exit_stat),
    STATS_DESC_COUNTER!(VCPU, wfi_exit_stat),
    STATS_DESC_COUNTER!(VCPU, wrs_exit_stat),
    STATS_DESC_COUNTER!(VCPU, mmio_exit_user),
    STATS_DESC_COUNTER!(VCPU, mmio_exit_kernel),
    STATS_DESC_COUNTER!(VCPU, csr_exit_user),
    STATS_DESC_COUNTER!(VCPU, csr_exit_kernel),
    STATS_DESC_COUNTER!(VCPU, signal_exits),
    STATS_DESC_COUNTER!(VCPU, exits),
    STATS_DESC_COUNTER!(VCPU, instr_illegal_exits),
    STATS_DESC_COUNTER!(VCPU, load_misaligned_exits),
    STATS_DESC_COUNTER!(VCPU, store_misaligned_exits),
    STATS_DESC_COUNTER!(VCPU, load_access_exits),
    STATS_DESC_COUNTER!(VCPU, store_access_exits),
];

pub static KVM_VCPU_STATS_HEADER: kvm_stats_header = kvm_stats_header {
    name_size: KVM_STATS_NAME_SIZE,
    num_desc: KVM_VCPU_STATS_DESC.len(),
    id_offset: core::mem::size_of::<kvm_stats_header>(),
    desc_offset: core::mem::size_of::<kvm_stats_header>() + KVM_STATS_NAME_SIZE,
    data_offset: core::mem::size_of::<kvm_stats_header>() + KVM_STATS_NAME_SIZE
        + core::mem::size_of_val(&KVM_VCPU_STATS_DESC),
};

unsafe fn kvm_riscv_vcpu_context_reset(vcpu: *mut kvm_vcpu, kvm_sbi_reset: bool) {
    let csr = &mut (*vcpu).arch.guest_csr;
    let cntx = &mut (*vcpu).arch.guest_context;
    let vector_datap = cntx.vector.datap;
    core::ptr::write_bytes(cntx as *mut _, 0, 1);
    core::ptr::write_bytes(csr as *mut _, 0, 1);
    core::ptr::write_bytes(&mut (*vcpu).arch.smstateen_csr as *mut _, 0, 1);
    core::ptr::write_bytes(&mut (*vcpu).arch.zicfiss_csr as *mut _, 0, 1);
    cntx.vector.datap = vector_datap;
    if kvm_sbi_reset { kvm_riscv_vcpu_sbi_load_reset_state(vcpu); }
    cntx.sstatus = SR_SPP | SR_SPIE;
    cntx.hstatus |= HSTATUS_VTW;
    cntx.hstatus |= HSTATUS_SPVP;
    cntx.hstatus |= HSTATUS_SPV;
}

unsafe fn kvm_riscv_reset_vcpu(vcpu: *mut kvm_vcpu, kvm_sbi_reset: bool) {
    let loaded;
    get_cpu();
    loaded = (*vcpu).cpu != -1;
    if loaded { kvm_arch_vcpu_put(vcpu); }
    (*vcpu).arch.last_exit_cpu = -1;
    kvm_riscv_vcpu_context_reset(vcpu, kvm_sbi_reset);
    kvm_riscv_vcpu_fp_reset(vcpu);
    kvm_riscv_vcpu_vector_reset(vcpu);
    kvm_riscv_vcpu_timer_reset(vcpu);
    kvm_riscv_vcpu_aia_reset(vcpu);
    raw_spin_lock_irqsave(&mut (*vcpu).arch.irqs_pending_lock, &mut 0);
    bitmap_zero((*vcpu).arch.irqs_pending, KVM_RISCV_VCPU_NR_IRQS);
    bitmap_zero((*vcpu).arch.irqs_pending_mask, KVM_RISCV_VCPU_NR_IRQS);
    raw_spin_unlock_irqrestore(&mut (*vcpu).arch.irqs_pending_lock, 0);
    kvm_riscv_vcpu_pmu_reset(vcpu);
    (*vcpu).arch.hfence_head = 0;
    (*vcpu).arch.hfence_tail = 0;
    core::ptr::write_bytes((*vcpu).arch.hfence_queue, 0, 1);
    kvm_riscv_vcpu_sbi_reset(vcpu);
    if loaded { kvm_arch_vcpu_load(vcpu, smp_processor_id()); }
    put_cpu();
}

pub unsafe fn kvm_arch_vcpu_precreate(_kvm: *mut kvm, _id: u32) -> i32 { 0 }

pub unsafe fn kvm_arch_vcpu_create(vcpu: *mut kvm_vcpu) -> i32 {
    spin_lock_init(&mut (*vcpu).arch.mp_state_lock);
    (*vcpu).arch.ran_atleast_once = false;
    (*vcpu).arch.mmu_page_cache.gfp_zero = __GFP_ZERO;
    bitmap_zero((*vcpu).arch.isa, RISCV_ISA_EXT_MAX);
    kvm_riscv_vcpu_config_init(vcpu);
    kvm_riscv_vcpu_setup_isa(vcpu);
    (*vcpu).arch.mvendorid = sbi_get_mvendorid();
    (*vcpu).arch.marchid = sbi_get_marchid();
    (*vcpu).arch.mimpid = sbi_get_mimpid();
    spin_lock_init(&mut (*vcpu).arch.hfence_lock);
    raw_spin_lock_init(&mut (*vcpu).arch.irqs_pending_lock);
    spin_lock_init(&mut (*vcpu).arch.reset_state.lock);
    let rc = kvm_riscv_vcpu_alloc_vector_context(vcpu);
    if rc != 0 { return rc; }
    kvm_riscv_vcpu_timer_init(vcpu);
    kvm_riscv_vcpu_pmu_init(vcpu);
    kvm_riscv_vcpu_aia_init(vcpu);
    kvm_riscv_vcpu_sbi_init(vcpu);
    kvm_riscv_reset_vcpu(vcpu, false);
    0
}

pub unsafe fn kvm_arch_vcpu_postcreate(vcpu: *mut kvm_vcpu) {
    if (*vcpu).vcpu_idx != 0 { kvm_riscv_vcpu_power_off(vcpu); }
}

pub unsafe fn kvm_arch_vcpu_destroy(vcpu: *mut kvm_vcpu) {
    kvm_riscv_vcpu_sbi_deinit(vcpu);
    kvm_riscv_vcpu_aia_deinit(vcpu);
    kvm_riscv_vcpu_timer_deinit(vcpu);
    kvm_riscv_vcpu_pmu_deinit(vcpu);
    kvm_mmu_free_memory_cache(&mut (*vcpu).arch.mmu_page_cache);
    kvm_riscv_vcpu_free_vector_context(vcpu);
}

pub unsafe fn kvm_cpu_has_pending_timer(vcpu: *mut kvm_vcpu) -> i32 { kvm_riscv_vcpu_timer_pending(vcpu) }

pub unsafe fn kvm_arch_vcpu_runnable(vcpu: *mut kvm_vcpu) -> bool {
    kvm_riscv_vcpu_has_interrupts(vcpu, !0) && !kvm_riscv_vcpu_stopped(vcpu) && !(*vcpu).arch.pause
}

pub unsafe fn kvm_arch_vcpu_should_kick(vcpu: *mut kvm_vcpu) -> bool {
    kvm_vcpu_exiting_guest_mode(vcpu) == IN_GUEST_MODE
}

pub unsafe fn kvm_arch_vcpu_in_kernel(vcpu: *mut kvm_vcpu) -> bool {
    ((*vcpu).arch.guest_context.sstatus & SR_SPP) != 0
}

#[cfg(CONFIG_GUEST_PERF_EVENTS)]
pub unsafe fn kvm_arch_vcpu_get_ip(vcpu: *mut kvm_vcpu) -> usize { (*vcpu).arch.guest_context.sepc }

pub unsafe fn kvm_arch_vcpu_fault(_vcpu: *mut kvm_vcpu, _vmf: *mut vm_fault) -> vm_fault_t { VM_FAULT_SIGBUS }

pub unsafe fn kvm_arch_vcpu_unlocked_ioctl(filp: *mut file, ioctl: u32, arg: usize) -> isize {
    let vcpu = (*filp).private_data as *mut kvm_vcpu;
    let argp = arg as *mut core::ffi::c_void;
    if ioctl == KVM_INTERRUPT {
        let mut irq: kvm_interrupt = core::mem::zeroed();
        if copy_from_user(&mut irq, argp, core::mem::size_of::<kvm_interrupt>()) != 0 { return -EFAULT as isize; }
        return if irq.irq == KVM_INTERRUPT_SET { kvm_riscv_vcpu_set_interrupt(vcpu, IRQ_VS_EXT) as isize } else { kvm_riscv_vcpu_unset_interrupt(vcpu, IRQ_VS_EXT) as isize };
    }
    -ENOIOCTLCMD as isize
}

pub unsafe fn kvm_arch_vcpu_ioctl(filp: *mut file, ioctl: u32, arg: usize) -> isize {
    let vcpu = (*filp).private_data as *mut kvm_vcpu;
    let argp = arg as *mut core::ffi::c_void;
    let mut r: isize = -EINVAL as isize;
    match ioctl {
        KVM_SET_ONE_REG | KVM_GET_ONE_REG => {
            let mut reg: kvm_one_reg = core::mem::zeroed();
            if copy_from_user(&mut reg, argp, core::mem::size_of::<kvm_one_reg>()) != 0 { return -EFAULT as isize; }
            r = if ioctl == KVM_SET_ONE_REG { kvm_riscv_vcpu_set_reg(vcpu, &mut reg) as isize } else { kvm_riscv_vcpu_get_reg(vcpu, &mut reg) as isize };
        }
        KVM_GET_REG_LIST => {
            let user_list = argp as *mut kvm_reg_list;
            let mut reg_list: kvm_reg_list = core::mem::zeroed();
            if copy_from_user(&mut reg_list, user_list as *mut _, core::mem::size_of::<kvm_reg_list>()) != 0 { return -EFAULT as isize; }
            let n = reg_list.n;
            reg_list.n = kvm_riscv_vcpu_num_regs(vcpu);
            if copy_to_user(user_list as *mut _, &reg_list, core::mem::size_of::<kvm_reg_list>()) != 0 { return -EFAULT as isize; }
            if n < reg_list.n { return -E2BIG as isize; }
            r = kvm_riscv_vcpu_copy_reg_indices(vcpu, (*user_list).reg) as isize;
        }
        _ => {}
    }
    r
}

pub unsafe fn kvm_arch_vcpu_ioctl_get_sregs(_: *mut kvm_vcpu, _: *mut kvm_sregs) -> i32 { -EINVAL }
pub unsafe fn kvm_arch_vcpu_ioctl_set_sregs(_: *mut kvm_vcpu, _: *mut kvm_sregs) -> i32 { -EINVAL }
pub unsafe fn kvm_arch_vcpu_ioctl_get_fpu(_: *mut kvm_vcpu, _: *mut kvm_fpu) -> i32 { -EINVAL }
pub unsafe fn kvm_arch_vcpu_ioctl_set_fpu(_: *mut kvm_vcpu, _: *mut kvm_fpu) -> i32 { -EINVAL }
pub unsafe fn kvm_arch_vcpu_ioctl_translate(_: *mut kvm_vcpu, _: *mut kvm_translation) -> i32 { -EINVAL }
pub unsafe fn kvm_arch_vcpu_ioctl_get_regs(_: *mut kvm_vcpu, _: *mut kvm_regs) -> i32 { -EINVAL }
pub unsafe fn kvm_arch_vcpu_ioctl_set_regs(_: *mut kvm_vcpu, _: *mut kvm_regs) -> i32 { -EINVAL }

pub unsafe fn kvm_riscv_vcpu_flush_interrupts(vcpu: *mut kvm_vcpu) {
    let csr = &mut (*vcpu).arch.guest_csr;
    raw_spin_lock_irqsave(&mut (*vcpu).arch.irqs_pending_lock, &mut 0);
    let mask = (*vcpu).arch.irqs_pending_mask[0];
    if mask != 0 {
        (*vcpu).arch.irqs_pending_mask[0] = 0;
        let val = (*vcpu).arch.irqs_pending[0] & mask;
        csr.hvip &= !mask;
        csr.hvip |= val;
    }
    kvm_riscv_vcpu_aia_flush_interrupts(vcpu);
    raw_spin_unlock_irqrestore(&mut (*vcpu).arch.irqs_pending_lock, 0);
}

pub unsafe fn kvm_riscv_vcpu_sync_interrupts(vcpu: *mut kvm_vcpu) {
    let v = &mut (*vcpu).arch;
    let csr = &mut v.guest_csr;
    csr.vsie = ncsr_read(CSR_VSIE);
    let hvip = ncsr_read(CSR_HVIP);
    raw_spin_lock_irqsave(&mut v.irqs_pending_lock, &mut 0);
    if ((csr.hvip ^ hvip) & (1usize << IRQ_VS_SOFT)) != 0 {
        if (hvip & (1usize << IRQ_VS_SOFT)) != 0 { if !test_and_set_bit(IRQ_VS_SOFT, v.irqs_pending_mask) { set_bit(IRQ_VS_SOFT, v.irqs_pending); } }
        else if !test_and_set_bit(IRQ_VS_SOFT, v.irqs_pending_mask) { clear_bit(IRQ_VS_SOFT, v.irqs_pending); }
    }
    if ((csr.hvip ^ hvip) & (1usize << IRQ_PMU_OVF)) != 0 && (hvip & (1usize << IRQ_PMU_OVF)) == 0 && !test_and_set_bit(IRQ_PMU_OVF, v.irqs_pending_mask) { clear_bit(IRQ_PMU_OVF, v.irqs_pending); }
    kvm_riscv_vcpu_aia_sync_interrupts(vcpu);
    raw_spin_unlock_irqrestore(&mut v.irqs_pending_lock, 0);
    kvm_riscv_vcpu_timer_sync(vcpu);
}

pub unsafe fn kvm_riscv_vcpu_set_interrupt(vcpu: *mut kvm_vcpu, irq: u32) -> i32 {
    if irq < IRQ_LOCAL_MAX && irq != IRQ_VS_SOFT && irq != IRQ_VS_TIMER && irq != IRQ_VS_EXT && irq != IRQ_PMU_OVF { return -EINVAL; }
    raw_spin_lock_irqsave(&mut (*vcpu).arch.irqs_pending_lock, &mut 0);
    set_bit(irq, (*vcpu).arch.irqs_pending); set_bit(irq, (*vcpu).arch.irqs_pending_mask);
    raw_spin_unlock_irqrestore(&mut (*vcpu).arch.irqs_pending_lock, 0);
    trace_kvm_vcpu_irq((*vcpu).vcpu_id, irq, 1); kvm_vcpu_kick(vcpu); 0
}

pub unsafe fn kvm_riscv_vcpu_unset_interrupt(vcpu: *mut kvm_vcpu, irq: u32) -> i32 {
    if irq < IRQ_LOCAL_MAX && irq != IRQ_VS_SOFT && irq != IRQ_VS_TIMER && irq != IRQ_VS_EXT && irq != IRQ_PMU_OVF { return -EINVAL; }
    raw_spin_lock_irqsave(&mut (*vcpu).arch.irqs_pending_lock, &mut 0);
    clear_bit(irq, (*vcpu).arch.irqs_pending); set_bit(irq, (*vcpu).arch.irqs_pending_mask);
    raw_spin_unlock_irqrestore(&mut (*vcpu).arch.irqs_pending_lock, 0);
    trace_kvm_vcpu_irq((*vcpu).vcpu_id, irq, 0); 0
}

pub unsafe fn kvm_riscv_vcpu_has_interrupts(vcpu: *mut kvm_vcpu, mask: u64) -> bool {
    raw_spin_lock_irqsave(&mut (*vcpu).arch.irqs_pending_lock, &mut 0);
    let ie = (((*vcpu).arch.guest_csr.vsie & VSIP_VALID_MASK) << VSIP_TO_HVIP_SHIFT) & mask as usize
        | ((*vcpu).arch.guest_csr.vsie & !IRQ_LOCAL_MASK) & mask as usize;
    let ret = ((*vcpu).arch.irqs_pending[0] & ie) != 0;
    raw_spin_unlock_irqrestore(&mut (*vcpu).arch.irqs_pending_lock, 0);
    ret || kvm_riscv_vcpu_aia_has_interrupts(vcpu, mask)
}

pub unsafe fn __kvm_riscv_vcpu_power_off(vcpu: *mut kvm_vcpu) { WRITE_ONCE!((*vcpu).arch.mp_state.mp_state, KVM_MP_STATE_STOPPED); kvm_make_request(KVM_REQ_SLEEP, vcpu); kvm_vcpu_kick(vcpu); }
pub unsafe fn kvm_riscv_vcpu_power_off(vcpu: *mut kvm_vcpu) { spin_lock(&mut (*vcpu).arch.mp_state_lock); __kvm_riscv_vcpu_power_off(vcpu); spin_unlock(&mut (*vcpu).arch.mp_state_lock); }
pub unsafe fn __kvm_riscv_vcpu_power_on(vcpu: *mut kvm_vcpu) { WRITE_ONCE!((*vcpu).arch.mp_state.mp_state, KVM_MP_STATE_RUNNABLE); kvm_vcpu_wake_up(vcpu); }
pub unsafe fn kvm_riscv_vcpu_power_on(vcpu: *mut kvm_vcpu) { spin_lock(&mut (*vcpu).arch.mp_state_lock); __kvm_riscv_vcpu_power_on(vcpu); spin_unlock(&mut (*vcpu).arch.mp_state_lock); }
pub unsafe fn kvm_riscv_vcpu_stopped(vcpu: *mut kvm_vcpu) -> bool { READ_ONCE!((*vcpu).arch.mp_state.mp_state) == KVM_MP_STATE_STOPPED }

pub unsafe fn kvm_arch_vcpu_ioctl_get_mpstate(vcpu: *mut kvm_vcpu, mp: *mut kvm_mp_state) -> i32 { *mp = READ_ONCE!((*vcpu).arch.mp_state); 0 }
pub unsafe fn kvm_arch_vcpu_ioctl_set_mpstate(vcpu: *mut kvm_vcpu, mp: *mut kvm_mp_state) -> i32 {
    let mut ret = 0; spin_lock(&mut (*vcpu).arch.mp_state_lock);
    match (*mp).mp_state { KVM_MP_STATE_RUNNABLE => WRITE_ONCE!((*vcpu).arch.mp_state, *mp), KVM_MP_STATE_STOPPED => __kvm_riscv_vcpu_power_off(vcpu), KVM_MP_STATE_INIT_RECEIVED => if (*vcpu).kvm.arch.mp_state_reset { kvm_riscv_reset_vcpu(vcpu, false) } else { ret = -EINVAL }, _ => ret = -EINVAL }
    spin_unlock(&mut (*vcpu).arch.mp_state_lock); ret
}

pub unsafe fn kvm_arch_vcpu_ioctl_set_guest_debug(vcpu: *mut kvm_vcpu, dbg: *mut kvm_guest_debug) -> i32 { (*vcpu).guest_debug = if (*dbg).control & KVM_GUESTDBG_ENABLE != 0 { (*dbg).control } else { 0 }; kvm_riscv_vcpu_config_guest_debug(vcpu); 0 }

pub unsafe fn kvm_arch_vcpu_load(vcpu: *mut kvm_vcpu, cpu: i32) {
    let csr = &(*vcpu).arch.guest_csr;
    if vcpu != KVM_FORMER_VCPU { KVM_FORMER_VCPU = vcpu; }
    else if (*vcpu).arch.last_exit_cpu == cpu && !(*vcpu).arch.csr_dirty { goto_csr_restore_done(); return; }
    (*vcpu).arch.csr_dirty = false;
    kvm_riscv_vcpu_config_load(vcpu);
    if kvm_riscv_nacl_sync_csr_available() { let nsh = nacl_shmem(); nacl_csr_write(nsh, CSR_VSSTATUS, csr.vsstatus); nacl_csr_write(nsh, CSR_VSIE, csr.vsie); nacl_csr_write(nsh, CSR_VSTVEC, csr.vstvec); nacl_csr_write(nsh, CSR_VSSCRATCH, csr.vsscratch); nacl_csr_write(nsh, CSR_VSEPC, csr.vsepc); nacl_csr_write(nsh, CSR_VSCAUSE, csr.vscause); nacl_csr_write(nsh, CSR_VSTVAL, csr.vstval); nacl_csr_write(nsh, CSR_HVIP, csr.hvip); nacl_csr_write(nsh, CSR_VSATP, csr.vsatp); }
    else { csr_write(CSR_VSSTATUS, csr.vsstatus); csr_write(CSR_VSIE, csr.vsie); csr_write(CSR_VSTVEC, csr.vstvec); csr_write(CSR_VSSCRATCH, csr.vsscratch); csr_write(CSR_VSEPC, csr.vsepc); csr_write(CSR_VSCAUSE, csr.vscause); csr_write(CSR_VSTVAL, csr.vstval); csr_write(CSR_HVIP, csr.hvip); csr_write(CSR_VSATP, csr.vsatp); }
    kvm_riscv_mmu_update_hgatp(vcpu); kvm_riscv_vcpu_aia_load(vcpu, cpu);
goto_csr_restore_done();
}

unsafe fn goto_csr_restore_done() { }

pub unsafe fn kvm_arch_vcpu_put(vcpu: *mut kvm_vcpu) {
    let csr = &mut (*vcpu).arch.guest_csr; (*vcpu).cpu = -1; kvm_riscv_vcpu_aia_put(vcpu);
    kvm_riscv_vcpu_guest_fp_save(&mut (*vcpu).arch.guest_context, (*vcpu).arch.isa); kvm_riscv_vcpu_host_fp_restore(&mut (*vcpu).arch.host_context); kvm_riscv_vcpu_timer_save(vcpu);
    if kvm_riscv_nacl_available() { let nsh = nacl_shmem(); csr.vsstatus = nacl_csr_read(nsh, CSR_VSSTATUS); csr.vsie = nacl_csr_read(nsh, CSR_VSIE); csr.vstvec = nacl_csr_read(nsh, CSR_VSTVEC); csr.vsscratch = nacl_csr_read(nsh, CSR_VSSCRATCH); csr.vsepc = nacl_csr_read(nsh, CSR_VSEPC); csr.vscause = nacl_csr_read(nsh, CSR_VSCAUSE); csr.vstval = nacl_csr_read(nsh, CSR_VSTVAL); csr.hvip = nacl_csr_read(nsh, CSR_HVIP); csr.vsatp = nacl_csr_read(nsh, CSR_VSATP); }
    else { csr.vsstatus = csr_read(CSR_VSSTATUS); csr.vsie = csr_read(CSR_VSIE); csr.vstvec = csr_read(CSR_VSTVEC); csr.vsscratch = csr_read(CSR_VSSCRATCH); csr.vsepc = csr_read(CSR_VSEPC); csr.vscause = csr_read(CSR_VSCAUSE); csr.vstval = csr_read(CSR_VSTVAL); csr.hvip = csr_read(CSR_HVIP); csr.vsatp = csr_read(CSR_VSATP); }
}

unsafe fn kvm_riscv_check_vcpu_requests(vcpu: *mut kvm_vcpu) -> i32 {
    let wait = kvm_arch_vcpu_get_wait(vcpu);
    if kvm_request_pending(vcpu) {
        if kvm_check_request(KVM_REQ_SLEEP, vcpu) {
            kvm_vcpu_srcu_read_unlock(vcpu);
            rcuwait_wait_event(wait, !kvm_riscv_vcpu_stopped(vcpu) && !(*vcpu).arch.pause, TASK_INTERRUPTIBLE);
            kvm_vcpu_srcu_read_lock(vcpu);
            if kvm_riscv_vcpu_stopped(vcpu) || (*vcpu).arch.pause { kvm_make_request(KVM_REQ_SLEEP, vcpu); }
        }
        if kvm_check_request(KVM_REQ_VCPU_RESET, vcpu) { kvm_riscv_reset_vcpu(vcpu, true); }
        if kvm_check_request(KVM_REQ_UPDATE_HGATP, vcpu) { kvm_riscv_mmu_update_hgatp(vcpu); }
        if kvm_check_request(KVM_REQ_FENCE_I, vcpu) { kvm_riscv_fence_i_process(vcpu); }
        if kvm_check_request(KVM_REQ_TLB_FLUSH, vcpu) { kvm_riscv_tlb_flush_process(vcpu); }
        if kvm_check_request(KVM_REQ_HFENCE_VVMA_ALL, vcpu) { kvm_riscv_hfence_vvma_all_process(vcpu); }
        if kvm_check_request(KVM_REQ_HFENCE, vcpu) { kvm_riscv_hfence_process(vcpu); }
        if kvm_check_request(KVM_REQ_STEAL_UPDATE, vcpu) { kvm_riscv_vcpu_record_steal_time(vcpu); }
        if kvm_dirty_ring_check_request(vcpu) { return 0; }
    }
    1
}

unsafe fn kvm_riscv_update_hvip(vcpu: *mut kvm_vcpu) {
    ncsr_write(CSR_HVIP, (*vcpu).arch.guest_csr.hvip);
    kvm_riscv_vcpu_aia_update_hvip(vcpu);
}

#[inline(always)]
unsafe fn kvm_riscv_vcpu_swap_in_guest_state(vcpu: *mut kvm_vcpu) {
    let csr = &mut (*vcpu).arch.guest_csr;
    (*vcpu).arch.host_scounteren = csr_swap(CSR_SCOUNTEREN, csr.scounteren);
    (*vcpu).arch.host_senvcfg = csr_swap(CSR_SENVCFG, csr.senvcfg);
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_SMSTATEEN) { (*vcpu).arch.host_sstateen0 = csr_swap(CSR_SSTATEEN0, (*vcpu).arch.smstateen_csr.sstateen0); }
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_ZICFISS) { csr_write(CSR_SSP, (*vcpu).arch.zicfiss_csr.ssp); }
}

#[inline(always)]
unsafe fn kvm_riscv_vcpu_swap_in_host_state(vcpu: *mut kvm_vcpu) {
    let csr = &mut (*vcpu).arch.guest_csr;
    csr.scounteren = csr_swap(CSR_SCOUNTEREN, (*vcpu).arch.host_scounteren);
    csr.senvcfg = csr_swap(CSR_SENVCFG, (*vcpu).arch.host_senvcfg);
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_SMSTATEEN) { (*vcpu).arch.smstateen_csr.sstateen0 = csr_swap(CSR_SSTATEEN0, (*vcpu).arch.host_sstateen0); }
    if riscv_has_extension_unlikely(RISCV_ISA_EXT_ZICFISS) { (*vcpu).arch.zicfiss_csr.ssp = csr_swap(CSR_SSP, 0); }
}

unsafe fn kvm_riscv_vcpu_enter_exit(vcpu: *mut kvm_vcpu, trap: *mut kvm_cpu_trap) {
    let gcntx = &mut (*vcpu).arch.guest_context;
    let hcntx = &mut (*vcpu).arch.host_context;
    kvm_riscv_vcpu_swap_in_guest_state(vcpu); guest_state_enter_irqoff();
    if current_thread_riscv_v_flags() & RISCV_V_VCPU_NEED_RESTORE != 0 {
        current_thread_riscv_clear_v_flag(RISCV_V_VCPU_NEED_RESTORE);
        current_thread_riscv_set_v_flag(RISCV_V_VCPU_CTX);
        __kvm_riscv_vector_restore(gcntx);
        gcntx.sstatus = (gcntx.sstatus & !SR_VS) | SR_VS_CLEAN;
    }
    if kvm_riscv_nacl_sync_sret_available() {
        let nsh = nacl_shmem();
        if kvm_riscv_nacl_autoswap_csr_available() {
            hcntx.hstatus = nacl_csr_read(nsh, CSR_HSTATUS);
            nacl_scratch_write_long(nsh, SBI_NACL_SHMEM_AUTOSWAP_OFFSET + SBI_NACL_SHMEM_AUTOSWAP_HSTATUS, gcntx.hstatus);
            nacl_scratch_write_long(nsh, SBI_NACL_SHMEM_AUTOSWAP_OFFSET, SBI_NACL_SHMEM_AUTOSWAP_FLAG_HSTATUS);
        } else if kvm_riscv_nacl_sync_csr_available() { hcntx.hstatus = nacl_csr_swap(nsh, CSR_HSTATUS, gcntx.hstatus); }
        else { hcntx.hstatus = csr_swap(CSR_HSTATUS, gcntx.hstatus); }
        nacl_scratch_write_longs(nsh, SBI_NACL_SHMEM_SRET_OFFSET + SBI_NACL_SHMEM_SRET_X(1), &gcntx.ra, SBI_NACL_SHMEM_SRET_X_LAST);
        __kvm_riscv_nacl_switch_to(&mut (*vcpu).arch, SBI_EXT_NACL, SBI_EXT_NACL_SYNC_SRET);
        if kvm_riscv_nacl_autoswap_csr_available() { nacl_scratch_write_long(nsh, SBI_NACL_SHMEM_AUTOSWAP_OFFSET, 0); gcntx.hstatus = nacl_scratch_read_long(nsh, SBI_NACL_SHMEM_AUTOSWAP_OFFSET + SBI_NACL_SHMEM_AUTOSWAP_HSTATUS); }
        else { gcntx.hstatus = csr_swap(CSR_HSTATUS, hcntx.hstatus); }
        (*trap).htval = nacl_csr_read(nsh, CSR_HTVAL); (*trap).htinst = nacl_csr_read(nsh, CSR_HTINST);
    } else {
        hcntx.hstatus = csr_swap(CSR_HSTATUS, gcntx.hstatus); __kvm_riscv_switch_to(&mut (*vcpu).arch); gcntx.hstatus = csr_swap(CSR_HSTATUS, hcntx.hstatus);
        (*trap).htval = csr_read(CSR_HTVAL); (*trap).htinst = csr_read(CSR_HTINST);
    }
    (*trap).sepc = gcntx.sepc; (*trap).scause = csr_read(CSR_SCAUSE); (*trap).stval = csr_read(CSR_STVAL);
    (*vcpu).arch.last_exit_cpu = (*vcpu).cpu; guest_state_exit_irqoff(); kvm_riscv_vcpu_swap_in_host_state(vcpu);
}

// The remaining request/entry machinery is a literal low-level translation of the source.
pub unsafe fn kvm_arch_vcpu_ioctl_run(vcpu: *mut kvm_vcpu) -> i32 {
    let run = (*vcpu).run;
    if !(*vcpu).arch.ran_atleast_once { kvm_riscv_vcpu_config_ran_once(vcpu); }
    (*vcpu).arch.ran_atleast_once = true; kvm_vcpu_srcu_read_lock(vcpu);
    let mut ret = match (*run).exit_reason { KVM_EXIT_MMIO => kvm_riscv_vcpu_mmio_return(vcpu, run), KVM_EXIT_RISCV_SBI => kvm_riscv_vcpu_sbi_return(vcpu, run), KVM_EXIT_RISCV_CSR => kvm_riscv_vcpu_csr_return(vcpu, run), _ => 0 };
    if ret != 0 { kvm_vcpu_srcu_read_unlock(vcpu); return ret; }
    if !(*vcpu).wants_to_run { kvm_vcpu_srcu_read_unlock(vcpu); return -EINTR; }
    vcpu_load(vcpu); kvm_sigset_activate(vcpu); (*run).exit_reason = KVM_EXIT_UNKNOWN;
    while ret > 0 {
        ret = kvm_xfer_to_guest_mode_handle_work(vcpu); if ret != 0 { continue; }
        ret = kvm_riscv_check_vcpu_requests(vcpu); if ret <= 0 { continue; }
        preempt_disable(); ret = kvm_riscv_vcpu_aia_update(vcpu); if ret <= 0 { preempt_enable(); continue; }
        local_irq_disable(); (*vcpu).mode = IN_GUEST_MODE; kvm_vcpu_srcu_read_unlock(vcpu); smp_mb__after_srcu_read_unlock();
        kvm_riscv_vcpu_flush_interrupts(vcpu); kvm_riscv_update_hvip(vcpu);
        if kvm_riscv_gstage_vmid_ver_changed(&(*vcpu).kvm.arch.vmid) || kvm_request_pending(vcpu) || xfer_to_guest_mode_work_pending() { (*vcpu).mode = OUTSIDE_GUEST_MODE; local_irq_enable(); preempt_enable(); kvm_vcpu_srcu_read_lock(vcpu); continue; }
        kvm_riscv_local_tlb_sanitize(vcpu); trace_kvm_entry(vcpu); guest_timing_enter_irqoff();
        let mut trap: kvm_cpu_trap = core::mem::zeroed(); kvm_riscv_vcpu_enter_exit(vcpu, &mut trap);
        (*vcpu).mode = OUTSIDE_GUEST_MODE; (*vcpu).stat.exits += 1; kvm_riscv_vcpu_sync_interrupts(vcpu);
        local_irq_enable(); local_irq_disable(); guest_timing_exit_irqoff(); local_irq_enable(); trace_kvm_exit(&trap); preempt_enable(); kvm_vcpu_srcu_read_lock(vcpu); ret = kvm_riscv_vcpu_exit(vcpu, run, &mut trap);
    }
    kvm_sigset_deactivate(vcpu); vcpu_put(vcpu); kvm_vcpu_srcu_read_unlock(vcpu); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
