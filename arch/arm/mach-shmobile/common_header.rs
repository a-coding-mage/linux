/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard is omitted; Rust items are already scoped by the module.

extern "C" {
    pub fn shmobile_init_delay();
    pub fn shmobile_boot_vector();
    pub static mut shmobile_boot_fn: core::ffi::c_ulong;
    pub static mut shmobile_boot_size: core::ffi::c_ulong;
    pub fn shmobile_boot_vector_gen2();
    pub static mut shmobile_boot_fn_gen2: core::ffi::c_ulong;
    pub static mut shmobile_boot_cpu_gen2: core::ffi::c_ulong;
    pub static mut shmobile_boot_size_gen2: core::ffi::c_ulong;
    pub fn shmobile_smp_boot();
    pub fn shmobile_smp_sleep();
    pub fn shmobile_smp_hook(cpu: core::ffi::c_uint, fn_: core::ffi::c_ulong,
                             arg: core::ffi::c_ulong);
    pub fn shmobile_smp_cpu_can_disable(cpu: core::ffi::c_uint) -> bool;
    pub fn shmobile_boot_apmu();
    pub fn shmobile_boot_scu();
    pub fn shmobile_smp_scu_prepare_cpus(scu_base_phys: phys_addr_t,
                                         max_cpus: core::ffi::c_uint);
    pub fn shmobile_smp_scu_cpu_die(cpu: core::ffi::c_uint);
    pub fn shmobile_smp_scu_cpu_kill(cpu: core::ffi::c_uint) -> core::ffi::c_int;
    pub static mut shmobile_suspend_ops: platform_suspend_ops;
}

// CONFIG_SUSPEND controls which C implementation is used at build time.
#[cfg(CONFIG_SUSPEND)]
extern "C" {
    pub fn shmobile_suspend_init() -> core::ffi::c_int;
}

#[cfg(CONFIG_SUSPEND)]
extern "C" {
    pub fn shmobile_smp_apmu_suspend_init();
}

#[cfg(not(CONFIG_SUSPEND))]
pub const unsafe fn shmobile_suspend_init() -> core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_SUSPEND))]
pub const unsafe fn shmobile_smp_apmu_suspend_init() {}

// C __init annotation has no direct Rust equivalent.
pub unsafe fn shmobile_init_late() {
    shmobile_suspend_init();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
