// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-spear13xx/platsmp.c
 *
 * based upon linux/arch/arm/mach-realview/platsmp.c
 *
 * Copyright (C) 2012 ST Microelectronics Ltd.
 * Shiraz Hashim <shiraz.linux.kernel@gmail.com>
 */

// Dependencies supplied by the surrounding kernel translation.

/* XXX spear_pen_release is cargo culted code - DO NOT COPY XXX */
static mut spear_pen_release: core::ffi::c_int = -1;

/*
 * XXX CARGO CULTED CODE - DO NOT COPY XXX
 *
 * Write spear_pen_release in a way that is guaranteed to be visible to
 * all observers, irrespective of whether they're taking part in coherency
 * or not.  This is necessary for the hotplug code to work reliably.
 */
unsafe fn spear_write_pen_release(val: core::ffi::c_int) {
    spear_pen_release = val;
    smp_wmb();
    sync_cache_w(&raw mut spear_pen_release);
}

static mut boot_lock: SpinLock = DEFINE_SPINLOCK!();

static mut scu_base: *mut core::ffi::c_void = IOMEM(VA_SCU_BASE);

unsafe fn spear13xx_secondary_init(_cpu: core::ffi::c_uint) {
    /*
     * let the primary processor know we're out of the
     * pen, then head off into the C entry point
     */
    spear_write_pen_release(-1);

    /*
     * Synchronise with the boot thread.
     */
    spin_lock(&raw mut boot_lock);
    spin_unlock(&raw mut boot_lock);
}

unsafe fn spear13xx_boot_secondary(
    cpu: core::ffi::c_uint,
    _idle: *mut task_struct,
) -> core::ffi::c_int {
    let mut timeout: core::ffi::c_ulong;

    /*
     * set synchronisation state between this boot processor
     * and the secondary one
     */
    spin_lock(&raw mut boot_lock);

    /*
     * The secondary processor is waiting to be released from
     * the holding pen - release it, then wait for it to flag
     * that it has been released by resetting spear_pen_release.
     *
     * Note that "spear_pen_release" is the hardware CPU ID, whereas
     * "cpu" is Linux's internal ID.
     */
    spear_write_pen_release(cpu as core::ffi::c_int);

    timeout = jiffies.wrapping_add(1 * HZ);
    while time_before(jiffies, timeout) {
        smp_rmb();
        if spear_pen_release == -1 {
            break;
        }

        udelay(10);
    }

    /*
     * now the secondary core is starting up let it run its
     * calibrations, then wait for it to finish
     */
    spin_unlock(&raw mut boot_lock);

    if spear_pen_release != -1 { -ENOSYS } else { 0 }
}

/*
 * Initialise the CPU possible map early - this describes the CPUs
 * which may be present or become present in the system.
 */
unsafe fn spear13xx_smp_init_cpus() {
    let mut ncores: core::ffi::c_uint = scu_get_core_count(scu_base);

    if ncores > nr_cpu_ids {
        pr_warn!("SMP: %u cores greater than maximum (%u), clipping\\n",
            ncores, nr_cpu_ids);
        ncores = nr_cpu_ids;
    }

    for i in 0..ncores {
        set_cpu_possible(i, true);
    }
}

unsafe fn spear13xx_smp_prepare_cpus(_max_cpus: core::ffi::c_uint) {
    scu_enable(scu_base);

    /*
     * Write the address of secondary startup into the system-wide location
     * (presently it is in SRAM). The BootMonitor waits until it receives a
     * soft interrupt, and then the secondary CPU branches to this address.
     */
    __raw_writel(__pa_symbol(spear13xx_secondary_startup), SYS_LOCATION);
}

pub static spear13xx_smp_ops: SmpOperations = SmpOperations {
    smp_init_cpus: Some(spear13xx_smp_init_cpus),
    smp_prepare_cpus: Some(spear13xx_smp_prepare_cpus),
    smp_secondary_init: Some(spear13xx_secondary_init),
    smp_boot_secondary: Some(spear13xx_boot_secondary),
    // .cpu_die = spear13xx_cpu_die when CONFIG_HOTPLUG_CPU is enabled.
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
