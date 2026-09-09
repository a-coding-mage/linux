/* SPDX-License-Identifier: GPL-2.0 */
/* net/atm/resources.h - ATM-related resources */

/* Written 1995-1998 by Werner Almesberger, EPFL LRC/ICA */

/* C dependencies: linux/atmdev.h, linux/mutex.h, and (when enabled)
 * linux/proc_fs.h. Their Rust declarations are supplied by other files. */

extern "C" {
    pub static mut atm_devs: crate::list_head;
    pub static mut atm_dev_mutex: crate::mutex;

    pub fn atm_getnames(buf: *mut core::ffi::c_void, iobuf_len: *mut i32) -> i32;
    pub fn atm_dev_ioctl(
        cmd: u32,
        buf: *mut core::ffi::c_void,
        sioc_len: *mut i32,
        number: i32,
        compat: i32,
    ) -> i32;
}

/* CONFIG_PROC_FS conditional declarations. */
#[cfg(feature = "CONFIG_PROC_FS")]
extern "C" {
    pub fn atm_dev_seq_start(
        seq: *mut crate::seq_file,
        pos: *mut i64,
    ) -> *mut core::ffi::c_void;
    pub fn atm_dev_seq_stop(seq: *mut crate::seq_file, v: *mut core::ffi::c_void);
    pub fn atm_dev_seq_next(
        seq: *mut crate::seq_file,
        v: *mut core::ffi::c_void,
        pos: *mut i64,
    ) -> *mut core::ffi::c_void;

    pub fn atm_proc_dev_register(dev: *mut crate::atm_dev) -> i32;
    pub fn atm_proc_dev_deregister(dev: *mut crate::atm_dev);
}

#[cfg(not(feature = "CONFIG_PROC_FS"))]
#[inline]
pub fn atm_proc_dev_register(_dev: *mut crate::atm_dev) -> i32 {
    0
}

#[cfg(not(feature = "CONFIG_PROC_FS"))]
#[inline]
pub fn atm_proc_dev_deregister(_dev: *mut crate::atm_dev) {
    /* nothing */
}

extern "C" {
    pub fn atm_register_sysfs(
        adev: *mut crate::atm_dev,
        parent: *mut crate::device,
    ) -> i32;
    pub fn atm_unregister_sysfs(adev: *mut crate::atm_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
