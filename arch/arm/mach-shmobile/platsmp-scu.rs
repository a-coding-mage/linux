// SPDX-License-Identifier: GPL-2.0
/*
 * SMP support for SoCs with SCU covered by mach-shmobile
 *
 * Copyright (C) 2013  Magnus Damm
 */

// Dependencies supplied by the surrounding kernel translation.
extern "C" {
    static mut shmobile_boot_scu: u8;
    static mut shmobile_smp_boot: u8;
    static mut shmobile_boot_fn: usize;

    fn shmobile_smp_hook(cpu: ::core::ffi::c_uint, boot_vector: usize, scu_base_phys: phys_addr_t);
    fn ioremap(phys_addr: phys_addr_t, size: usize) -> *mut ::core::ffi::c_void;
    fn scu_enable(base: *mut ::core::ffi::c_void);
    fn scu_power_mode(base: *mut ::core::ffi::c_void, mode: ::core::ffi::c_uint);
    fn cpuhp_setup_state_nocalls(
        state: ::core::ffi::c_uint,
        name: *const ::core::ffi::c_char,
        startup: unsafe extern "C" fn(::core::ffi::c_uint) -> ::core::ffi::c_int,
        teardown: Option<unsafe extern "C" fn(::core::ffi::c_uint) -> ::core::ffi::c_int>,
    );
    fn dsb();
    fn flush_cache_all();
    fn shmobile_smp_sleep();
    fn readl(addr: *mut u8) -> ::core::ffi::c_uint;
    fn mdelay(milliseconds: ::core::ffi::c_uint);
}

type phys_addr_t = usize;

const PAGE_SIZE: usize = 4096;
const SCU_PM_NORMAL: ::core::ffi::c_uint = 0;
const SCU_PM_POWEROFF: ::core::ffi::c_uint = 3;
const CPUHP_ARM_SHMOBILE_SCU_PREPARE: ::core::ffi::c_uint = 0;

static mut shmobile_scu_base_phys: phys_addr_t = 0;
static mut shmobile_scu_base: *mut ::core::ffi::c_void = ::core::ptr::null_mut();

unsafe extern "C" fn shmobile_scu_cpu_prepare(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    /* For this particular CPU register SCU SMP boot vector */
    shmobile_smp_hook(
        cpu,
        (&raw mut shmobile_boot_scu) as usize,
        shmobile_scu_base_phys,
    );
    0
}

pub unsafe extern "C" fn shmobile_smp_scu_prepare_cpus(
    scu_base_phys: phys_addr_t,
    _max_cpus: ::core::ffi::c_uint,
) {
    /* install boot code shared by all CPUs */
    shmobile_boot_fn = (&raw mut shmobile_smp_boot) as usize;

    /* enable SCU and cache coherency on booting CPU */
    shmobile_scu_base_phys = scu_base_phys;
    shmobile_scu_base = ioremap(scu_base_phys, PAGE_SIZE);
    scu_enable(shmobile_scu_base);
    scu_power_mode(shmobile_scu_base, SCU_PM_NORMAL);

    /* Use CPU notifier for reset vector control */
    cpuhp_setup_state_nocalls(
        CPUHP_ARM_SHMOBILE_SCU_PREPARE,
        b"arm/shmobile-scu:prepare\0".as_ptr() as *const ::core::ffi::c_char,
        shmobile_scu_cpu_prepare,
        None,
    );
}

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe extern "C" fn shmobile_smp_scu_cpu_die(cpu: ::core::ffi::c_uint) {
    /* For this particular CPU deregister boot vector */
    shmobile_smp_hook(cpu, 0, 0);

    dsb();
    flush_cache_all();

    /* disable cache coherency */
    scu_power_mode(shmobile_scu_base, SCU_PM_POWEROFF);

    /* jump to shared mach-shmobile sleep / reset code */
    shmobile_smp_sleep();
}

#[cfg(CONFIG_HOTPLUG_CPU)]
unsafe fn shmobile_smp_scu_psr_core_disabled(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int {
    let mask: ::core::ffi::c_ulong = (SCU_PM_POWEROFF as ::core::ffi::c_ulong)
        .wrapping_shl((cpu * 8) as u32);

    if ((readl((shmobile_scu_base as *mut u8).add(8)) as ::core::ffi::c_ulong) & mask) == mask {
        return 1;
    }

    0
}

#[cfg(CONFIG_HOTPLUG_CPU)]
pub unsafe extern "C" fn shmobile_smp_scu_cpu_kill(cpu: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    let mut k: ::core::ffi::c_int = 0;

    /* this function is running on another CPU than the offline target,
     * here we need wait for shutdown code in platform_cpu_die() to
     * finish before asking SoC-specific code to power off the CPU core.
     */
    while k < 1000 {
        if shmobile_smp_scu_psr_core_disabled(cpu as ::core::ffi::c_int) != 0 {
            return 1;
        }

        mdelay(1);
        k += 1;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
