/* SPDX-License-Identifier: GPL-2.0 */

/* Declarations from the Linux compiler, linkage, irqflags, reboot, and percpu
 * headers are supplied by the translated dependencies. */

#[allow(improper_ctypes)]
extern "C" {
    pub fn cpu_init();
    pub fn soft_restart(arg: ::core::ffi::c_ulong);
    pub static mut arm_pm_idle: Option<unsafe extern "C" fn()>;
}

#[cfg(feature = "CONFIG_HARDEN_BRANCH_PREDICTOR")]
pub type harden_branch_predictor_fn_t = unsafe extern "C" fn();

#[cfg(feature = "CONFIG_HARDEN_BRANCH_PREDICTOR")]
extern "C" {
    /* DECLARE_PER_CPU(harden_branch_predictor_fn_t, harden_branch_predictor_fn) */
    pub static mut harden_branch_predictor_fn: harden_branch_predictor_fn_t;
    pub fn smp_processor_id() -> ::core::ffi::c_int;
}

#[cfg(feature = "CONFIG_HARDEN_BRANCH_PREDICTOR")]
#[inline]
pub unsafe fn harden_branch_predictor() {
    /* per_cpu(harden_branch_predictor_fn, smp_processor_id()) */
    let fn_: harden_branch_predictor_fn_t = harden_branch_predictor_fn;
    fn_();
}

#[cfg(not(feature = "CONFIG_HARDEN_BRANCH_PREDICTOR"))]
#[inline]
pub fn harden_branch_predictor() {}

pub const UDBG_UNDEFINED: ::core::ffi::c_uint = 1 << 0;
pub const UDBG_SYSCALL: ::core::ffi::c_uint = 1 << 1;
pub const UDBG_BADABORT: ::core::ffi::c_uint = 1 << 2;
pub const UDBG_SEGV: ::core::ffi::c_uint = 1 << 3;
pub const UDBG_BUS: ::core::ffi::c_uint = 1 << 4;

extern "C" {
    pub static mut user_debug: ::core::ffi::c_uint;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
