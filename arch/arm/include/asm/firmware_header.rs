/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Samsung Electronics.
 * Kyungmin Park <kyungmin.park@samsung.com>
 * Tomasz Figa <t.figa@samsung.com>
 */

/* The C header includes linux/bug.h for BUG_ON. */

/*
 * struct firmware_ops
 *
 * A structure to specify available firmware operations.
 *
 * A filled up structure can be registered with register_firmware_ops().
 */
#[repr(C)]
pub struct firmware_ops {
    /* Inform the firmware we intend to enter CPU idle mode */
    pub prepare_idle: Option<unsafe extern "C" fn(mode: ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    /* Enters CPU idle mode */
    pub do_idle: Option<unsafe extern "C" fn(mode: ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    /* Sets boot address of specified physical CPU */
    pub set_cpu_boot_addr: Option<unsafe extern "C" fn(cpu: ::core::ffi::c_int, boot_addr: ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    /* Gets boot address of specified physical CPU */
    pub get_cpu_boot_addr: Option<unsafe extern "C" fn(cpu: ::core::ffi::c_int, boot_addr: *mut ::core::ffi::c_ulong) -> ::core::ffi::c_int>,
    /* Boots specified physical CPU */
    pub cpu_boot: Option<unsafe extern "C" fn(cpu: ::core::ffi::c_int) -> ::core::ffi::c_int>,
    /* Initializes L2 cache */
    pub l2x0_init: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    /* Enter system-wide suspend. */
    pub suspend: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
    /* Restore state of privileged hardware after system-wide suspend. */
    pub resume: Option<unsafe extern "C" fn() -> ::core::ffi::c_int>,
}

/* Global pointer for current firmware_ops structure, can't be NULL. */
extern "C" {
    pub static mut firmware_ops: *const firmware_ops;
}

/*
 * call_firmware_op(op, ...)
 *
 * Checks if firmware operation is present and calls it,
 * otherwise returns -ENOSYS.
 */
#[macro_export]
macro_rules! call_firmware_op {
    ($op:ident $(, $arg:expr)*) => {{
        unsafe {
            match (*firmware_ops).$op {
                Some(func) => func($($arg),*),
                None => -ENOSYS,
            }
        }
    }};
}

/*
 * register_firmware_ops(ops)
 *
 * A function to register platform firmware_ops struct.
 */
#[inline]
pub unsafe fn register_firmware_ops(ops: *const firmware_ops) {
    /* Equivalent of BUG_ON(!ops); supplied by linux/bug.h in C. */
    if ops.is_null() {
        core::intrinsics::abort();
    }

    firmware_ops = ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
