/* SPDX-License-Identifier: GPL-2.0 */

// The C header guard is omitted in Rust; item/module inclusion provides the guard.

extern "C" {
    pub static mut __stack_chk_guard: kernel::ffi::c_ulong;
    pub static mut stack_protector_debug: kernel::ffi::c_int;

    pub fn __stack_protector_apply_early(kernel_start: kernel::ffi::c_ulong);
    pub fn __stack_protector_apply(
        start: *mut kernel::ffi::c_ulong,
        end: *mut kernel::ffi::c_ulong,
        kernel_start: kernel::ffi::c_ulong,
    ) -> kernel::ffi::c_int;
}

#[inline]
pub unsafe fn stack_protector_apply_early(kernel_start: kernel::ffi::c_ulong) {
    // Preserves IS_ENABLED(CONFIG_STACKPROTECTOR), whose build-time definition
    // is supplied by the surrounding kernel configuration.
    #[cfg(CONFIG_STACKPROTECTOR)]
    {
        __stack_protector_apply_early(kernel_start);
    }
}

#[inline]
pub unsafe fn stack_protector_apply(
    start: *mut kernel::ffi::c_ulong,
    end: *mut kernel::ffi::c_ulong,
) -> kernel::ffi::c_int {
    // Preserves IS_ENABLED(CONFIG_STACKPROTECTOR), whose build-time definition
    // is supplied by the surrounding kernel configuration.
    #[cfg(CONFIG_STACKPROTECTOR)]
    {
        return __stack_protector_apply(start, end, 0);
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
