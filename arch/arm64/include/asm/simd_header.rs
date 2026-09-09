/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2017 Linaro Ltd. <ard.biesheuvel@linaro.org>
 */

/* C header dependencies are supplied by other translated units. */

#[cfg(feature = "CONFIG_KERNEL_MODE_NEON")]
extern "C" {
    fn system_capabilities_finalized() -> bool;
    fn system_supports_fpsimd() -> bool;
    fn in_hardirq() -> bool;
    fn in_nmi() -> bool;
    fn warn_on(condition: bool) -> bool;
}

#[cfg(feature = "CONFIG_KERNEL_MODE_NEON")]
/*
 * may_use_simd - whether it is allowable at this time to issue SIMD
 *                instructions or access the SIMD register file
 *
 * Callers must not assume that the result remains true beyond the next
 * preempt_enable() or return from softirq context.
 */
#[inline]
pub unsafe fn may_use_simd() -> bool {
    /*
     * We must make sure that the SVE has been initialized properly
     * before using the SIMD in kernel.
     */
    !warn_on(!system_capabilities_finalized())
        && system_supports_fpsimd()
        && !in_hardirq()
        && !in_nmi()
}

#[cfg(not(feature = "CONFIG_KERNEL_MODE_NEON"))]
#[inline]
pub fn may_use_simd() -> bool {
    false
}

/* DEFINE_LOCK_GUARD_1(ksimd, struct user_fpsimd_state, ...). */
#[repr(C)]
pub struct KsimdGuard<'a> {
    pub lock: &'a mut user_fpsimd_state,
}

extern "C" {
    pub fn kernel_neon_begin(state: *mut user_fpsimd_state);
    pub fn kernel_neon_end(state: *mut user_fpsimd_state);
}

/* The following type is provided by the translated Linux type definitions. */
#[allow(non_camel_case_types)]
pub enum user_fpsimd_state {}

impl<'a> KsimdGuard<'a> {
    #[inline]
    pub unsafe fn new(lock: &'a mut user_fpsimd_state) -> Self {
        kernel_neon_begin(lock as *mut user_fpsimd_state);
        Self { lock }
    }
}

impl Drop for KsimdGuard<'_> {
    #[inline]
    fn drop(&mut self) {
        unsafe { kernel_neon_end(self.lock as *mut user_fpsimd_state) }
    }
}

/* __scoped_ksimd(_label) and scoped_ksimd() are C scoped-cleanup macros. */
#[macro_export]
macro_rules! scoped_ksimd {
    ($state:expr) => {
        $crate::KsimdGuard::new($state)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
