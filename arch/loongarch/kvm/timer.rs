// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2023 Loongson Technology Corporation Limited
 */

/* Dependencies are supplied by the surrounding kernel/KVM translation. */

#[inline]
unsafe fn ktime_to_tick(vcpu: *mut kvm_vcpu, now: ktime_t) -> u64 {
    let delta: u64;

    delta = ktime_to_ns(now);
    div_u64(delta.wrapping_mul((*vcpu).arch.timer_mhz), MNSEC_PER_SEC)
}

#[inline]
unsafe fn tick_to_ns(vcpu: *mut kvm_vcpu, tick: u64) -> u64 {
    div_u64(tick.wrapping_mul(MNSEC_PER_SEC), (*vcpu).arch.timer_mhz)
}

/* Low level hrtimer wake routine */
pub unsafe fn kvm_swtimer_wakeup(timer: *mut hrtimer) -> hrtimer_restart {
    let vcpu: *mut kvm_vcpu;

    vcpu = container_of!(timer, kvm_vcpu, arch.swtimer);
    kvm_vcpu_wake_up(vcpu);

    HRTIMER_NORESTART
}

/*
 * Initialise the timer to the specified frequency, zero it
 */
pub unsafe fn kvm_init_timer(vcpu: *mut kvm_vcpu, timer_hz: c_ulong) {
    (*vcpu).arch.timer_mhz = timer_hz >> 20;

    /* Starting at 0 */
    kvm_write_sw_gcsr((*vcpu).arch.csr, LOONGARCH_CSR_TVAL, 0);
}

/*
 * Restore soft timer state from saved context.
 */
pub unsafe fn kvm_restore_timer(vcpu: *mut kvm_vcpu) {
    let mut cfg: c_ulong;
    let mut estat: c_ulong;
    let mut ticks: c_ulong;
    let mut delta: c_ulong;
    let mut period: c_ulong;
    let mut expire: ktime_t;
    let mut now: ktime_t;
    let csr: *mut loongarch_csrs = (*vcpu).arch.csr;

    cfg = kvm_read_sw_gcsr(csr, LOONGARCH_CSR_TCFG);

    write_gcsr_timercfg(0);
    kvm_restore_hw_gcsr(csr, LOONGARCH_CSR_ESTAT);
    kvm_restore_hw_gcsr(csr, LOONGARCH_CSR_TCFG);
    if (cfg & CSR_TCFG_EN) == 0 {
        kvm_restore_hw_gcsr(csr, LOONGARCH_CSR_TVAL);
        return;
    }

    if kvm_vcpu_is_blocking(vcpu) {
        hrtimer_cancel(&mut (*vcpu).arch.swtimer);
    }

    ticks = kvm_read_sw_gcsr(csr, LOONGARCH_CSR_TVAL);
    estat = kvm_read_sw_gcsr(csr, LOONGARCH_CSR_ESTAT);
    if (cfg & CSR_TCFG_PERIOD) == 0 && ticks > cfg {
        write_gcsr_timertick(0);

        if (estat & CPU_TIMER) == 0 {
            __delay(2);
            estat = kvm_read_hw_gcsr(LOONGARCH_CSR_ESTAT);
            if (estat & CPU_TIMER) == 0 {
                write_gcsr_timertick(CSR_TCFG_VAL);
            }
            gcsr_write(CSR_TINTCLR_TI, LOONGARCH_CSR_TINTCLR);
        }
        return;
    }

    delta = 0;
    now = ktime_get();
    expire = (*vcpu).arch.expire;
    if expire == 0 {
        if ticks < cfg {
            delta = tick_to_ns(vcpu, ticks as u64) as c_ulong;
        }
        expire = ktime_add_ns(now, delta as u64);
    }

    if ktime_before(now, expire) {
        delta = ktime_to_tick(vcpu, ktime_sub(expire, now)) as c_ulong;
    } else if (cfg & CSR_TCFG_PERIOD) != 0 {
        period = cfg & CSR_TCFG_VAL;
        if period == 0 {
            period = 1;
        }
        delta = ktime_to_tick(vcpu, ktime_sub(now, expire)) as c_ulong;
        delta = period - (delta % period);
        kvm_queue_irq(vcpu, INT_TI);
    }

    write_gcsr_timertick(delta);
}

/*
 * Save guest timer state and switch to software emulation of guest
 * timer. The hard timer must already be in use, so preemption should be
 * disabled.
 */
unsafe fn _kvm_save_timer(vcpu: *mut kvm_vcpu) {
    let ticks: c_ulong;
    let mut delta: c_ulong;
    let cfg: c_ulong;
    let expire: ktime_t;
    let csr: *mut loongarch_csrs = (*vcpu).arch.csr;

    cfg = kvm_read_sw_gcsr(csr, LOONGARCH_CSR_TCFG);
    ticks = kvm_read_sw_gcsr(csr, LOONGARCH_CSR_TVAL);

    if ticks < cfg {
        delta = tick_to_ns(vcpu, ticks as u64) as c_ulong;
    } else {
        delta = 0;
    }

    expire = ktime_add_ns(ktime_get(), delta as u64);
    (*vcpu).arch.expire = expire;
    if kvm_vcpu_is_blocking(vcpu) {
        hrtimer_start(&mut (*vcpu).arch.swtimer, expire, HRTIMER_MODE_ABS_PINNED_HARD);
    }
}

/*
 * Save guest timer state and switch to soft guest timer if hard timer was
 * in use.
 */
pub unsafe fn kvm_save_timer(vcpu: *mut kvm_vcpu) {
    let csr: *mut loongarch_csrs = (*vcpu).arch.csr;

    preempt_disable();

    kvm_save_hw_gcsr(csr, LOONGARCH_CSR_TCFG);
    kvm_save_hw_gcsr(csr, LOONGARCH_CSR_TVAL);
    if (kvm_read_sw_gcsr(csr, LOONGARCH_CSR_TCFG) & CSR_TCFG_EN) != 0 {
        _kvm_save_timer(vcpu);
    }

    kvm_save_hw_gcsr(csr, LOONGARCH_CSR_ESTAT);
    preempt_enable();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
