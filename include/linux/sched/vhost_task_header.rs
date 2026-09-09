/* SPDX-License-Identifier: GPL-2.0 */

// Opaque declaration corresponding to the C forward declaration.
#[repr(C)]
pub struct vhost_task {
    _private: [u8; 0],
}

extern "C" {
    pub fn vhost_task_create(
        fn_: Option<unsafe extern "C" fn(*mut core::ffi::c_void) -> bool>,
        handle_kill: Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,
        arg: *mut core::ffi::c_void,
        name: *const core::ffi::c_char,
    ) -> *mut vhost_task;

    pub fn vhost_task_start(vtsk: *mut vhost_task);
    pub fn vhost_task_stop(vtsk: *mut vhost_task);
    pub fn vhost_task_wake(vtsk: *mut vhost_task);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
