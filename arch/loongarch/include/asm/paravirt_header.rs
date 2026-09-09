/* SPDX-License-Identifier: GPL-2.0 */

/* CONFIG_PARAVIRT conditional from the original header. */
#[cfg(feature = "CONFIG_PARAVIRT")]
mod config_paravirt {
    use core::ffi::c_void;

    /* DECLARE_STATIC_KEY_FALSE(virt_preempt_key); */
    unsafe extern "C" {
        pub static virt_preempt_key: c_void;
        pub static virt_spin_lock_key: c_void;

        /* DECLARE_PER_CPU(struct kvm_steal_time, steal_time); */
        pub static steal_time: c_void;

        pub fn pv_ipi_init() -> core::ffi::c_int;
        pub fn pv_time_init() -> core::ffi::c_int;
        pub fn pv_spinlock_init() -> core::ffi::c_int;
    }
}

#[cfg(not(feature = "CONFIG_PARAVIRT"))]
#[inline]
pub const fn pv_ipi_init() -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_PARAVIRT"))]
#[inline]
pub const fn pv_time_init() -> core::ffi::c_int {
    0
}

#[cfg(not(feature = "CONFIG_PARAVIRT"))]
#[inline]
pub const fn pv_spinlock_init() -> core::ffi::c_int {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
