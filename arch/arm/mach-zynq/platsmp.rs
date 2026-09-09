// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file contains Xilinx specific SMP code, used to start up
 * the second processor.
 *
 * Copyright (C) 2011-2013 Xilinx
 *
 * based on linux/arch/arm/mach-realview/platsmp.c
 *
 * Copyright (C) 2002 ARM Ltd.
 */

// C headers and symbols are supplied by the surrounding kernel translation.

extern "C" {
    static mut zynq_secondary_trampoline: u8;
    static mut zynq_secondary_trampoline_end: u8;
    static mut zynq_secondary_trampoline_jump: u8;
    static mut zynq_scu_base: *mut core::ffi::c_void;

    fn cpu_logical_map(cpu: i32) -> u32;
    fn zynq_slcr_cpu_stop(cpu: u32);
    fn zynq_slcr_cpu_start(cpu: u32);
    fn zynq_slcr_cpu_state_read(cpu: u32) -> bool;
    fn zynq_slcr_cpu_state_write(cpu: u32, state: bool);
    fn zynq_core_pm_init();
    fn __pa(addr: usize) -> usize;
    fn __pa_symbol(addr: usize) -> u32;
    fn ioremap(addr: usize, size: u32) -> *mut u8;
    fn iounmap(addr: *mut u8);
    fn memcpy_toio(dst: *mut u8, src: *const u8, size: u32);
    fn writel(value: u32, addr: *mut u8);
    fn flush_cache_all();
    fn outer_flush_range(start: u32, end: u32);
    fn smp_wmb();
    fn scu_get_core_count(base: *mut core::ffi::c_void) -> i32;
    fn scu_enable(base: *mut core::ffi::c_void);
    fn set_cpu_possible(cpu: i32, possible: bool);
    fn msecs_to_jiffies(ms: u32) -> u64;
    fn time_after(a: u64, b: u64) -> bool;
    fn cpu_do_idle() -> !;
    static mut jiffies: u64;
    static secondary_startup_arm: u8;
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

#[repr(C)]
pub struct smp_operations {
    pub smp_init_cpus: Option<unsafe extern "C" fn()>,
    pub smp_prepare_cpus: Option<unsafe extern "C" fn(u32)>,
    pub smp_boot_secondary: Option<unsafe extern "C" fn(u32, *mut task_struct) -> i32>,
    pub smp_secondary_init: Option<unsafe extern "C" fn(u32)>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_die: Option<unsafe extern "C" fn(u32)>,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_kill: Option<unsafe extern "C" fn(u32) -> i32>,
}

/* Store number of cores in the system. */
static mut ncores: i32 = 0;

pub unsafe extern "C" fn zynq_cpun_start(address: u32, cpu: i32) -> i32 {
    let trampoline_code_size = (&zynq_secondary_trampoline_end as *const u8 as usize)
        .wrapping_sub(&zynq_secondary_trampoline as *const u8 as usize) as u32;
    let phy_cpuid = cpu_logical_map(cpu);

    /* MS: Expectation that SLCR are directly map and accessible. */
    /* Not possible to jump to non aligned address. */
    if (address & 3) == 0 && (address == 0 || address >= trampoline_code_size) {
        static mut zero: *mut u8 = core::ptr::null_mut();
        let trampoline_size = (&zynq_secondary_trampoline_jump as *const u8 as usize)
            .wrapping_sub(&zynq_secondary_trampoline as *const u8 as usize) as u32;

        zynq_slcr_cpu_stop(phy_cpuid);
        if address != 0 {
            if __pa(0) != 0 {
                zero = ioremap(0, trampoline_code_size);
                if zero.is_null() {
                    return -1;
                }
            } else {
                zero = 0 as *mut u8;
            }

            /*
             * This is elegant way how to jump to any address
             * 0x0: Load address at 0x8 to r0
             * 0x4: Jump by mov instruction
             * 0x8: Jumping address
             */
            memcpy_toio(zero, &zynq_secondary_trampoline as *const u8, trampoline_size);
            writel(address, zero.add(trampoline_size as usize));

            flush_cache_all();
            outer_flush_range(0, trampoline_code_size);
            smp_wmb();

            if __pa(0) != 0 {
                iounmap(zero);
            }
        }
        zynq_slcr_cpu_start(phy_cpuid);
        return 0;
    }

    -1
}

unsafe extern "C" fn zynq_boot_secondary(cpu: u32, _idle: *mut task_struct) -> i32 {
    zynq_cpun_start(__pa_symbol(&secondary_startup_arm as *const u8 as usize), cpu as i32)
}

/* Initialise the CPU possible map early. */
unsafe extern "C" fn zynq_smp_init_cpus() {
    ncores = scu_get_core_count(zynq_scu_base);
    let mut i = 0;
    while i < ncores && i < CONFIG_NR_CPUS {
        set_cpu_possible(i, true);
        i += 1;
    }
}

unsafe extern "C" fn zynq_smp_prepare_cpus(_max_cpus: u32) {
    scu_enable(zynq_scu_base);
}

/* This function is in the hotplug path. Don't move it into the init section!! */
unsafe extern "C" fn zynq_secondary_init(_cpu: u32) {
    zynq_core_pm_init();
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe extern "C" fn zynq_cpu_kill(cpu: u32) -> i32 {
    let timeout = jiffies.wrapping_add(msecs_to_jiffies(50));
    while zynq_slcr_cpu_state_read(cpu) {
        if time_after(jiffies, timeout) {
            return 0;
        }
    }
    zynq_slcr_cpu_stop(cpu);
    1
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe extern "C" fn zynq_cpu_die(cpu: u32) -> ! {
    zynq_slcr_cpu_state_write(cpu, true);
    loop {
        cpu_do_idle();
    }
}

const CONFIG_NR_CPUS: i32 = 1;

#[no_mangle]
pub static zynq_smp_ops: smp_operations = smp_operations {
    smp_init_cpus: Some(zynq_smp_init_cpus),
    smp_prepare_cpus: Some(zynq_smp_prepare_cpus),
    smp_boot_secondary: Some(zynq_boot_secondary),
    smp_secondary_init: Some(zynq_secondary_init),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_die: Some(zynq_cpu_die),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    cpu_kill: Some(zynq_cpu_kill),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
