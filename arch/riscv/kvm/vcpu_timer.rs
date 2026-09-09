// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2019 Western Digital Corporation or its affiliates.
 *
 * Authors:
 *     Atish Patra <atish.patra@wdc.com>
 */

// Dependencies are supplied by the surrounding kernel/Rust bindings.

unsafe fn kvm_riscv_current_cycles(gt: *mut kvm_guest_timer) -> u64 {
    get_cycles64().wrapping_add((*gt).time_delta)
}

unsafe fn kvm_riscv_delta_cycles2ns(
    cycles: u64,
    gt: *mut kvm_guest_timer,
    _t: *mut kvm_vcpu_timer,
) -> u64 {
    let mut flags: c_ulong = 0;
    let cycles_now: u64;
    let cycles_delta: u64;
    let delta_ns: u64;

    local_irq_save(&mut flags);
    cycles_now = kvm_riscv_current_cycles(gt);
    if cycles_now < cycles {
        cycles_delta = cycles - cycles_now;
    } else {
        cycles_delta = 0;
    }
    delta_ns = cycles_delta
        .wrapping_mul((*gt).nsec_mult)
        >> (*gt).nsec_shift;
    local_irq_restore(flags);

    delta_ns
}

unsafe extern "C" fn kvm_riscv_vcpu_hrtimer_expired(h: *mut hrtimer) -> hrtimer_restart {
    let delta_ns: u64;
    let t: *mut kvm_vcpu_timer = container_of!(h, kvm_vcpu_timer, hrt);
    let vcpu: *mut kvm_vcpu = container_of!(t, kvm_vcpu, arch.timer);
    let gt: *mut kvm_guest_timer = &mut (*(*vcpu).kvm).arch.timer;

    if kvm_riscv_current_cycles(gt) < (*t).next_cycles {
        delta_ns = kvm_riscv_delta_cycles2ns((*t).next_cycles, gt, t);
        hrtimer_forward_now(&mut (*t).hrt, ktime_set(0, delta_ns));
        return HRTIMER_RESTART;
    }

    (*t).next_set = false;
    kvm_riscv_vcpu_set_interrupt(vcpu, IRQ_VS_TIMER);

    HRTIMER_NORESTART
}

unsafe fn kvm_riscv_vcpu_timer_cancel(t: *mut kvm_vcpu_timer) -> c_int {
    if !(*t).init_done || !(*t).next_set {
        return -EINVAL;
    }

    hrtimer_cancel(&mut (*t).hrt);
    (*t).next_set = false;

    0
}

unsafe fn kvm_riscv_vcpu_update_vstimecmp(
    _vcpu: *mut kvm_vcpu,
    ncycles: u64,
) -> c_int {
    // CONFIG_32BIT selects the split CSR writes in the original source.
    #[cfg(target_pointer_width = "32")]
    {
        ncsr_write(CSR_VSTIMECMP, ULONG_MAX);
        ncsr_write(CSR_VSTIMECMPH, ncycles >> 32);
        ncsr_write(CSR_VSTIMECMP, ncycles as u32);
    }
    #[cfg(not(target_pointer_width = "32"))]
    {
        ncsr_write(CSR_VSTIMECMP, ncycles);
    }
    0
}

unsafe fn kvm_riscv_vcpu_update_hrtimer(vcpu: *mut kvm_vcpu, ncycles: u64) -> c_int {
    let t: *mut kvm_vcpu_timer = &mut (*vcpu).arch.timer;
    let gt: *mut kvm_guest_timer = &mut (*(*vcpu).kvm).arch.timer;

    if !(*t).init_done {
        return -EINVAL;
    }

    kvm_riscv_vcpu_unset_interrupt(vcpu, IRQ_VS_TIMER);

    let delta_ns = kvm_riscv_delta_cycles2ns(ncycles, gt, t);
    (*t).next_cycles = ncycles;
    hrtimer_start(&mut (*t).hrt, ktime_set(0, delta_ns), HRTIMER_MODE_REL);
    (*t).next_set = true;

    0
}

pub unsafe fn kvm_riscv_vcpu_timer_next_event(vcpu: *mut kvm_vcpu, ncycles: u64) -> c_int {
    let t: *mut kvm_vcpu_timer = &mut (*vcpu).arch.timer;
    ((*t).timer_next_event)(vcpu, ncycles)
}

unsafe extern "C" fn kvm_riscv_vcpu_vstimer_expired(h: *mut hrtimer) -> hrtimer_restart {
    let t: *mut kvm_vcpu_timer = container_of!(h, kvm_vcpu_timer, hrt);
    let vcpu: *mut kvm_vcpu = container_of!(t, kvm_vcpu, arch.timer);
    let gt: *mut kvm_guest_timer = &mut (*(*vcpu).kvm).arch.timer;

    if kvm_riscv_current_cycles(gt) < (*t).next_cycles {
        let delta_ns = kvm_riscv_delta_cycles2ns((*t).next_cycles, gt, t);
        hrtimer_forward_now(&mut (*t).hrt, ktime_set(0, delta_ns));
        return HRTIMER_RESTART;
    }

    (*t).next_set = false;
    kvm_vcpu_kick(vcpu);

    HRTIMER_NORESTART
}

pub unsafe fn kvm_riscv_vcpu_timer_pending(vcpu: *mut kvm_vcpu) -> bool {
    let t: *mut kvm_vcpu_timer = &mut (*vcpu).arch.timer;
    let gt: *mut kvm_guest_timer = &mut (*(*vcpu).kvm).arch.timer;

    !kvm_riscv_delta_cycles2ns((*t).next_cycles, gt, t)
        || kvm_riscv_vcpu_has_interrupts(vcpu, 1UL << IRQ_VS_TIMER)
}

unsafe fn kvm_riscv_vcpu_timer_blocking(vcpu: *mut kvm_vcpu) {
    let t: *mut kvm_vcpu_timer = &mut (*vcpu).arch.timer;
    let gt: *mut kvm_guest_timer = &mut (*(*vcpu).kvm).arch.timer;

    if !(*t).init_done {
        return;
    }

    let delta_ns = kvm_riscv_delta_cycles2ns((*t).next_cycles, gt, t);
    hrtimer_start(&mut (*t).hrt, ktime_set(0, delta_ns), HRTIMER_MODE_REL);
    (*t).next_set = true;
}

unsafe fn kvm_riscv_vcpu_timer_unblocking(vcpu: *mut kvm_vcpu) {
    kvm_riscv_vcpu_timer_cancel(&mut (*vcpu).arch.timer);
}

pub unsafe fn kvm_riscv_vcpu_get_reg_timer(
    vcpu: *mut kvm_vcpu,
    reg: *const kvm_one_reg,
) -> c_int {
    let t: *mut kvm_vcpu_timer = &mut (*vcpu).arch.timer;
    let gt: *mut kvm_guest_timer = &mut (*(*vcpu).kvm).arch.timer;
    let uaddr = (*reg).addr as *mut u64;
    let reg_num = (*reg).id & !(KVM_REG_ARCH_MASK | KVM_REG_SIZE_MASK | KVM_REG_RISCV_TIMER);
    let reg_val: u64;

    if KVM_REG_SIZE((*reg).id) != core::mem::size_of::<u64>() {
        return -EINVAL;
    }
    if reg_num >= core::mem::size_of::<kvm_riscv_timer>() / core::mem::size_of::<u64>() {
        return -ENOENT;
    }

    match reg_num {
        KVM_REG_RISCV_TIMER_REG!(frequency) => reg_val = riscv_timebase,
        KVM_REG_RISCV_TIMER_REG!(time) => reg_val = kvm_riscv_current_cycles(gt),
        KVM_REG_RISCV_TIMER_REG!(compare) => reg_val = (*t).next_cycles,
        KVM_REG_RISCV_TIMER_REG!(state) => {
            reg_val = if (*t).next_set { KVM_RISCV_TIMER_STATE_ON } else { KVM_RISCV_TIMER_STATE_OFF };
        }
        _ => return -ENOENT,
    }

    if copy_to_user(uaddr, &reg_val, KVM_REG_SIZE((*reg).id)) != 0 {
        return -EFAULT;
    }
    0
}

pub unsafe fn kvm_riscv_vcpu_set_reg_timer(
    vcpu: *mut kvm_vcpu,
    reg: *const kvm_one_reg,
) -> c_int {
    let t: *mut kvm_vcpu_timer = &mut (*vcpu).arch.timer;
    let gt: *mut kvm_guest_timer = &mut (*(*vcpu).kvm).arch.timer;
    let uaddr = (*reg).addr as *const u64;
    let reg_num = (*reg).id & !(KVM_REG_ARCH_MASK | KVM_REG_SIZE_MASK | KVM_REG_RISCV_TIMER);
    let mut reg_val = 0u64;
    let mut ret = 0;

    if KVM_REG_SIZE((*reg).id) != core::mem::size_of::<u64>() {
        return -EINVAL;
    }
    if reg_num >= core::mem::size_of::<kvm_riscv_timer>() / core::mem::size_of::<u64>() {
        return -ENOENT;
    }
    if copy_from_user(&mut reg_val, uaddr, KVM_REG_SIZE((*reg).id)) != 0 {
        return -EFAULT;
    }

    match reg_num {
        KVM_REG_RISCV_TIMER_REG!(frequency) => {
            if reg_val != riscv_timebase { return -EINVAL; }
        }
        KVM_REG_RISCV_TIMER_REG!(time) => (*gt).time_delta = reg_val.wrapping_sub(get_cycles64()),
        KVM_REG_RISCV_TIMER_REG!(compare) => (*t).next_cycles = reg_val,
        KVM_REG_RISCV_TIMER_REG!(state) => {
            if reg_val == KVM_RISCV_TIMER_STATE_ON {
                ret = kvm_riscv_vcpu_timer_next_event(vcpu, (*t).next_cycles);
            } else {
                ret = kvm_riscv_vcpu_timer_cancel(t);
            }
        }
        _ => ret = -ENOENT,
    }
    ret
}

pub unsafe fn kvm_riscv_vcpu_timer_init(vcpu: *mut kvm_vcpu) -> c_int {
    let t: *mut kvm_vcpu_timer = &mut (*vcpu).arch.timer;
    if (*t).init_done { return -EINVAL; }
    (*t).init_done = true;
    (*t).next_set = false;

    // Enable sstc for every vcpu if available in hardware.
    if !kvm_riscv_isa_check_host(SSTC) {
        (*t).sstc_enabled = true;
        hrtimer_setup(&mut (*t).hrt, kvm_riscv_vcpu_vstimer_expired, CLOCK_MONOTONIC, HRTIMER_MODE_REL);
        (*t).timer_next_event = kvm_riscv_vcpu_update_vstimecmp;
    } else {
        (*t).sstc_enabled = false;
        hrtimer_setup(&mut (*t).hrt, kvm_riscv_vcpu_hrtimer_expired, CLOCK_MONOTONIC, HRTIMER_MODE_REL);
        (*t).timer_next_event = kvm_riscv_vcpu_update_hrtimer;
    }
    0
}

pub unsafe fn kvm_riscv_vcpu_timer_deinit(vcpu: *mut kvm_vcpu) -> c_int {
    let ret = kvm_riscv_vcpu_timer_cancel(&mut (*vcpu).arch.timer);
    (*vcpu).arch.timer.init_done = false;
    ret
}

pub unsafe fn kvm_riscv_vcpu_timer_reset(vcpu: *mut kvm_vcpu) -> c_int {
    (*vcpu).arch.timer.next_cycles = u64::MAX;
    kvm_riscv_vcpu_timer_cancel(&mut (*vcpu).arch.timer)
}

unsafe fn kvm_riscv_vcpu_update_timedelta(vcpu: *mut kvm_vcpu) {
    let gt: *mut kvm_guest_timer = &mut (*(*vcpu).kvm).arch.timer;
    // CONFIG_32BIT selects the split CSR writes in the original source.
    #[cfg(target_pointer_width = "32")]
    {
        ncsr_write(CSR_HTIMEDELTA, (*gt).time_delta as u32);
        ncsr_write(CSR_HTIMEDELTAH, ((*gt).time_delta >> 32) as u32);
    }
    #[cfg(not(target_pointer_width = "32"))]
    {
        ncsr_write(CSR_HTIMEDELTA, (*gt).time_delta);
    }
}

pub unsafe fn kvm_riscv_vcpu_timer_restore(vcpu: *mut kvm_vcpu) {
    let t: *mut kvm_vcpu_timer = &mut (*vcpu).arch.timer;
    kvm_riscv_vcpu_update_timedelta(vcpu);
    if !(*t).sstc_enabled { return; }

    #[cfg(target_pointer_width = "32")]
    {
        ncsr_write(CSR_VSTIMECMP, ULONG_MAX);
        ncsr_write(CSR_VSTIMECMPH, ((*t).next_cycles >> 32) as u32);
        ncsr_write(CSR_VSTIMECMP, (*t).next_cycles as u32);
    }
    #[cfg(not(target_pointer_width = "32"))]
    { ncsr_write(CSR_VSTIMECMP, (*t).next_cycles); }

    if unlikely!(!(*t).init_done) { return; }
    kvm_riscv_vcpu_timer_unblocking(vcpu);
}

pub unsafe fn kvm_riscv_vcpu_timer_sync(vcpu: *mut kvm_vcpu) {
    let t: *mut kvm_vcpu_timer = &mut (*vcpu).arch.timer;
    if !(*t).sstc_enabled { return; }
    #[cfg(target_pointer_width = "32")]
    {
        (*t).next_cycles = ncsr_read(CSR_VSTIMECMP);
        (*t).next_cycles |= (ncsr_read(CSR_VSTIMECMPH) as u64) << 32;
    }
    #[cfg(not(target_pointer_width = "32"))]
    { (*t).next_cycles = ncsr_read(CSR_VSTIMECMP); }
}

pub unsafe fn kvm_riscv_vcpu_timer_save(vcpu: *mut kvm_vcpu) {
    let t: *mut kvm_vcpu_timer = &mut (*vcpu).arch.timer;
    if !(*t).sstc_enabled { return; }

    /*
     * The vstimecmp CSRs are saved by kvm_riscv_vcpu_timer_sync()
     * upon every VM exit so no need to save here.
     *
     * If VS-timer expires when no VCPU running on a host CPU then WFI
     * executed by such host CPU will be effective NOP resulting in no
     * power savings. This is because WFI is required to resume execution
     * for locally enabled interrupts pending at any privilege level.
     *
     * To address the above issue, vstimecmp CSR must be set to -1UL
     * over here when VCPU is scheduled-out or exits to user space.
     */
    csr_write(CSR_VSTIMECMP, -1isize as usize);
    #[cfg(target_pointer_width = "32")]
    { csr_write(CSR_VSTIMECMPH, -1isize as usize); }

    if unlikely!(!(*t).init_done) { return; }
    if kvm_vcpu_is_blocking(vcpu) { kvm_riscv_vcpu_timer_blocking(vcpu); }
}

pub unsafe fn kvm_riscv_guest_timer_init(kvm: *mut kvm) {
    let gt: *mut kvm_guest_timer = &mut (*kvm).arch.timer;
    riscv_cs_get_mult_shift(&mut (*gt).nsec_mult, &mut (*gt).nsec_shift);
    (*gt).time_delta = (0u64).wrapping_sub(get_cycles64());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
