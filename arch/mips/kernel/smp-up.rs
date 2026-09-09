/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2006, 07 by Ralf Baechle (ralf@linux-mips.org)
 *
 * Symmetric Uniprocessor (TM) Support
 */

// C dependencies: <linux/kernel.h>, <linux/sched.h>

#[repr(C)]
pub struct cpumask {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn panic(format: *const core::ffi::c_char, ...);
    fn BUG();
}

/*
 * Send inter-processor interrupt
 */
unsafe fn up_send_ipi_single(cpu: core::ffi::c_int, action: core::ffi::c_uint) {
    let _ = (cpu, action);
    panic(c"%s called".as_ptr(), c"up_send_ipi_single".as_ptr());
}

unsafe fn up_send_ipi_mask(mask: *const cpumask, action: core::ffi::c_uint) {
    let _ = (mask, action);
    panic(c"%s called".as_ptr(), c"up_send_ipi_mask".as_ptr());
}

/*
 *  After we've done initial boot, this function is called to allow the
 *  board code to clean up state, if needed
 */
unsafe fn up_init_secondary() {}

unsafe fn up_smp_finish() {}

/*
 * Firmware CPU startup hook
 */
unsafe fn up_boot_secondary(cpu: core::ffi::c_int, idle: *mut task_struct) -> core::ffi::c_int {
    let _ = (cpu, idle);
    0
}

unsafe fn up_smp_setup() {}

unsafe fn up_prepare_cpus(max_cpus: core::ffi::c_uint) {
    let _ = max_cpus;
}

/* CONFIG_HOTPLUG_CPU conditionally includes the following declarations. */
#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn up_cpu_disable() -> core::ffi::c_int {
    -38 /* -ENOSYS */
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
unsafe fn up_cpu_die(cpu: core::ffi::c_uint) {
    let _ = cpu;
    BUG();
}

#[repr(C)]
pub struct plat_smp_ops {
    pub send_ipi_single:
        unsafe fn(core::ffi::c_int, core::ffi::c_uint),
    pub send_ipi_mask:
        unsafe fn(*const cpumask, core::ffi::c_uint),
    pub init_secondary: unsafe fn(),
    pub smp_finish: unsafe fn(),
    pub boot_secondary:
        unsafe fn(core::ffi::c_int, *mut task_struct) -> core::ffi::c_int,
    pub smp_setup: unsafe fn(),
    pub prepare_cpus: unsafe fn(core::ffi::c_uint),
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_disable: unsafe fn() -> core::ffi::c_int,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    pub cpu_die: unsafe fn(core::ffi::c_uint),
}

pub static up_smp_ops: plat_smp_ops = plat_smp_ops {
    .send_ipi_single: up_send_ipi_single,
    .send_ipi_mask: up_send_ipi_mask,
    .init_secondary: up_init_secondary,
    .smp_finish: up_smp_finish,
    .boot_secondary: up_boot_secondary,
    .smp_setup: up_smp_setup,
    .prepare_cpus: up_prepare_cpus,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    .cpu_disable: up_cpu_disable,
    #[cfg(feature = "CONFIG_HOTPLUG_CPU")]
    .cpu_die: up_cpu_die,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
