// SPDX-License-Identifier: GPL-2.0

// Translated from drm_managed.h. C preprocessor includes and header guards
// are omitted; the referenced types and symbols are supplied by dependencies.

use core::ffi::c_char;

pub type size_t = usize;
pub type gfp_t = usize;

#[repr(C)]
pub struct drm_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

pub type drmres_release_t = Option<unsafe extern "C" fn(*mut drm_device, *mut core::ffi::c_void)>;

// drmm_add_action - add a managed release action to a drm_device.
#[macro_export]
macro_rules! drmm_add_action {
    ($dev:expr, $action:expr, $data:expr) => {
        unsafe { $crate::__drmm_add_action($dev, $action, $data, stringify!($action).as_ptr() as *const core::ffi::c_char) }
    };
}

unsafe extern "C" {
    pub fn __drmm_add_action(
        dev: *mut drm_device,
        action: drmres_release_t,
        data: *mut core::ffi::c_void,
        name: *const c_char,
    ) -> i32;
}

// drmm_add_action_or_reset - add a managed release action to a drm_device.
#[macro_export]
macro_rules! drmm_add_action_or_reset {
    ($dev:expr, $action:expr, $data:expr) => {
        unsafe { $crate::__drmm_add_action_or_reset($dev, $action, $data, stringify!($action).as_ptr() as *const core::ffi::c_char) }
    };
}

unsafe extern "C" {
    pub fn __drmm_add_action_or_reset(
        dev: *mut drm_device,
        action: drmres_release_t,
        data: *mut core::ffi::c_void,
        name: *const c_char,
    ) -> i32;

    pub fn drmm_release_action(
        dev: *mut drm_device,
        action: drmres_release_t,
        data: *mut core::ffi::c_void,
    );

    pub fn drmm_kmalloc(dev: *mut drm_device, size: size_t, gfp: gfp_t) -> *mut core::ffi::c_void;

    pub fn drmm_kstrdup(dev: *mut drm_device, s: *const c_char, gfp: gfp_t) -> *mut c_char;

    pub fn drmm_kfree(dev: *mut drm_device, data: *mut core::ffi::c_void);

    pub fn __drmm_mutex_release(dev: *mut drm_device, res: *mut core::ffi::c_void);

    pub fn __drmm_workqueue_release(device: *mut drm_device, wq: *mut core::ffi::c_void);
}

// drmm_kzalloc - drm_device managed kzalloc().
#[inline]
pub unsafe fn drmm_kzalloc(dev: *mut drm_device, size: size_t, gfp: gfp_t) -> *mut core::ffi::c_void {
    drmm_kmalloc(dev, size, gfp | __GFP_ZERO)
}

// drmm_kmalloc_array - drm_device managed kmalloc_array().
#[inline]
pub unsafe fn drmm_kmalloc_array(
    dev: *mut drm_device,
    n: size_t,
    size: size_t,
    flags: gfp_t,
) -> *mut core::ffi::c_void {
    match n.checked_mul(size) {
        Some(bytes) => drmm_kmalloc(dev, bytes, flags),
        None => core::ptr::null_mut(),
    }
}

// drmm_kcalloc - drm_device managed kcalloc().
#[inline]
pub unsafe fn drmm_kcalloc(
    dev: *mut drm_device,
    n: size_t,
    size: size_t,
    flags: gfp_t,
) -> *mut core::ffi::c_void {
    drmm_kmalloc_array(dev, n, size, flags | __GFP_ZERO)
}

#[macro_export]
macro_rules! drmm_mutex_init {
    ($dev:expr, $lock:expr) => {{
        unsafe { mutex_init($lock); }
        unsafe { $crate::drmm_add_action_or_reset!($dev, $crate::__drmm_mutex_release, $lock) }
    }};
}

unsafe extern "C" {
    pub fn mutex_init(lock: *mut mutex);
    pub fn alloc_ordered_workqueue(fmt: *const c_char, flags: u32, ...) -> *mut core::ffi::c_void;
}

pub const __GFP_ZERO: gfp_t = 0x8000;

// drmm_alloc_ordered_workqueue - drm_device managed alloc_ordered_workqueue().
// The C variadic macro's format arguments are retained through Rust's variadic
// macro forwarding; ERR_PTR and errno symbols are supplied by dependencies.
#[macro_export]
macro_rules! drmm_alloc_ordered_workqueue {
    ($dev:expr, $fmt:expr, $flags:expr $(, $args:expr)*) => {{
        let wq = unsafe { alloc_ordered_workqueue($fmt, $flags $(, $args)*) };
        if !wq.is_null() {
            let ret = unsafe { $crate::drmm_add_action_or_reset!($dev, $crate::__drmm_workqueue_release, wq) };
            if ret != 0 { ERR_PTR(ret) } else { wq }
        } else {
            ERR_PTR(-ENOMEM)
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
