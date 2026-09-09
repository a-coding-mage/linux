/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Intel(R) Trace Hub data structures for implementing buffer sinks.
 *
 * Copyright (C) 2019 Intel Corporation.
 */

// C dependency: <linux/scatterlist.h>

/* MSC operating modes (MSC_MODE) */
pub const MSC_MODE_SINGLE: i32 = 0;
pub const MSC_MODE_MULTI: i32 = 1;
pub const MSC_MODE_EXI: i32 = 2;
pub const MSC_MODE_DEBUG: i32 = 3;

#[repr(C)]
pub struct msu_buffer {
    pub name: *const core::ffi::c_char,
    /*
     * ->assign() called when buffer 'mode' is set to this driver
     *   (aka mode_store())
     * @device: struct device * of the msc
     * @mode: allows the driver to set HW mode (see the enum above)
     * Returns: a pointer to a private structure associated with this
     * msc or NULL in case of error. This private structure
     * will then be passed into all other callbacks.
     */
    pub assign: Option<unsafe extern "C" fn(
        dev: *mut device,
        mode: *mut i32,
    ) -> *mut core::ffi::c_void>,
    /* ->unassign(): some other mode is selected, clean up */
    pub unassign: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void)>,
    /*
     * ->alloc_window(): allocate memory for the window of a given size
     * @sgt: pointer to sg_table, can be overridden by the buffer driver,
     * or kept intact
     * Returns: number of sg table entries <= number of pages;
     * 0 is treated as an allocation failure.
     */
    pub alloc_window: Option<unsafe extern "C" fn(
        priv_: *mut core::ffi::c_void,
        sgt: *mut *mut sg_table,
        size: usize,
    ) -> i32>,
    pub free_window: Option<unsafe extern "C" fn(
        priv_: *mut core::ffi::c_void,
        sgt: *mut sg_table,
    )>,
    /* ->activate(): trace has started */
    pub activate: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void)>,
    /* ->deactivate(): trace is about to stop */
    pub deactivate: Option<unsafe extern "C" fn(priv_: *mut core::ffi::c_void)>,
    /*
     * ->ready(): window @sgt is filled up to the last block OR tracing is
     * stopped by the user; this window contains @bytes data. The window in
     * question transitions into the "LOCKED" state, indicating that it can't
     * be used by hardware. To clear this state and make the window available
     * to the hardware again, call intel_th_msc_window_unlock().
     */
    pub ready: Option<unsafe extern "C" fn(
        priv_: *mut core::ffi::c_void,
        sgt: *mut sg_table,
        bytes: usize,
    ) -> i32>,
}

// Types supplied by the Linux kernel headers.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct sg_table {
    _private: [u8; 0],
}
#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn intel_th_msu_buffer_register(
        mbuf: *const msu_buffer,
        owner: *mut module,
    ) -> i32;
    pub fn intel_th_msu_buffer_unregister(mbuf: *const msu_buffer);
    pub fn intel_th_msc_window_unlock(dev: *mut device, sgt: *mut sg_table);
}

/*
 * C module_intel_th_msu_buffer() registration macro. The generated module
 * init/exit hooks are supplied by the kernel module infrastructure.
 */
#[macro_export]
macro_rules! module_intel_th_msu_buffer {
    ($buffer:ident, $init:ident, $exit:ident, $this_module:expr) => {
        fn $init() -> i32 {
            unsafe { $crate::intel_th_msu_buffer_register(&$buffer, $this_module) }
        }
        fn $exit() {
            unsafe { $crate::intel_th_msu_buffer_unregister(&$buffer) }
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
