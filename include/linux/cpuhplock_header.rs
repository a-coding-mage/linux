/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/linux/cpuhplock.h - CPU hotplug locking
 *
 * Locking functions for CPU hotplug.
 */

/* linux/cleanup.h and linux/errno.h are provided by the surrounding tree. */

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_HOTPLUG_CPU")]
extern "C" {
    pub fn cpus_write_lock();
    pub fn cpus_write_unlock();
    pub fn cpus_read_lock();
    pub fn cpus_read_unlock();
    pub fn cpus_read_trylock() -> i32;
    pub fn lockdep_assert_cpus_held();
    pub fn lockdep_is_cpus_held() -> i32;
    pub fn lockdep_is_cpus_write_held() -> i32;
    pub fn cpu_hotplug_disable_offlining();
    pub fn cpu_hotplug_disable();
    pub fn cpu_hotplug_enable();
    pub fn clear_tasks_mm_cpumask(cpu: i32);
    pub fn remove_cpu(cpu: u32) -> i32;
    pub fn cpu_device_down(dev: *mut device) -> i32;
    pub fn smp_shutdown_nonboot_cpus(primary_cpu: u32);
}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub fn cpus_write_lock() {}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub fn cpus_write_unlock() {}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub fn cpus_read_lock() {}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub fn cpus_read_unlock() {}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub fn cpus_read_trylock() -> i32 {
    1
}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub fn lockdep_assert_cpus_held() {}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub fn lockdep_is_cpus_held() -> i32 {
    1
}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub fn lockdep_is_cpus_write_held() -> i32 {
    1
}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub fn cpu_hotplug_disable_offlining() {}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub fn cpu_hotplug_disable() {}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub fn cpu_hotplug_enable() {}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub fn remove_cpu(_cpu: u32) -> i32 {
    -EPERM
}

#[cfg(not(feature = "CONFIG_HOTPLUG_CPU"))]
#[inline]
pub fn smp_shutdown_nonboot_cpus(_primary_cpu: u32) {}

/* DEFINE_LOCK_GUARD_0(cpus_read_lock, cpus_read_lock(), cpus_read_unlock()) */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
