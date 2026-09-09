/* SPDX-License-Identifier: GPL-2.0-or-later */
/* V4L2 device support header. */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::c_void;

pub struct device;
pub struct media_device;
pub struct v4l2_subdev;
pub struct v4l2_ctrl_handler;
pub struct module;
pub struct atomic_t;
pub struct list_head;
pub struct v4l2_prio_state;
pub struct kref;

#[repr(C)]
pub struct v4l2_device {
    pub dev: *mut device,
    pub mdev: *mut media_device,
    pub subdevs: list_head,
    pub lock: spinlock_t,
    pub name: [core::ffi::c_char; 36],
    pub notify: Option<unsafe extern "C" fn(*mut v4l2_subdev, u32, *mut c_void)>,
    pub ctrl_handler: *mut v4l2_ctrl_handler,
    pub prio: v4l2_prio_state,
    pub ref_: kref,
    pub release: Option<unsafe extern "C" fn(*mut v4l2_device)>,
}

pub struct spinlock_t;

extern "C" {
    pub fn kref_get(kref: *mut kref);
    pub fn v4l2_device_put(v4l2_dev: *mut v4l2_device) -> i32;
    pub fn v4l2_device_register(dev: *mut device, v4l2_dev: *mut v4l2_device) -> i32;
    pub fn v4l2_device_set_name(v4l2_dev: *mut v4l2_device, basename: *const core::ffi::c_char,
                                instance: *mut atomic_t) -> i32;
    pub fn v4l2_device_disconnect(v4l2_dev: *mut v4l2_device);
    pub fn v4l2_device_unregister(v4l2_dev: *mut v4l2_device);
    pub fn __v4l2_device_register_subdev(v4l2_dev: *mut v4l2_device,
                                         sd: *mut v4l2_subdev,
                                         module: *mut module) -> i32;
    pub fn v4l2_device_unregister_subdev(sd: *mut v4l2_subdev);
    pub fn __v4l2_device_register_subdev_nodes(v4l2_dev: *mut v4l2_device,
                                               read_only: bool) -> i32;
}

#[inline]
pub unsafe fn v4l2_device_get(v4l2_dev: *mut v4l2_device) {
    kref_get(&mut (*v4l2_dev).ref_);
}

#[inline]
pub unsafe fn v4l2_device_register_subdev_nodes(v4l2_dev: *mut v4l2_device) -> i32 {
    // CONFIG_VIDEO_V4L2_SUBDEV_API controls this build-time branch.
    #[cfg(CONFIG_VIDEO_V4L2_SUBDEV_API)]
    { __v4l2_device_register_subdev_nodes(v4l2_dev, false) }
    #[cfg(not(CONFIG_VIDEO_V4L2_SUBDEV_API))]
    { let _ = v4l2_dev; 0 }
}

#[inline]
pub unsafe fn v4l2_device_register_ro_subdev_nodes(v4l2_dev: *mut v4l2_device) -> i32 {
    #[cfg(CONFIG_VIDEO_V4L2_SUBDEV_API)]
    { __v4l2_device_register_subdev_nodes(v4l2_dev, true) }
    #[cfg(not(CONFIG_VIDEO_V4L2_SUBDEV_API))]
    { let _ = v4l2_dev; 0 }
}

#[inline]
pub unsafe fn v4l2_subdev_notify(sd: *mut v4l2_subdev, notification: u32, arg: *mut c_void) {
    if !sd.is_null() {
        // The v4l2_subdev layout and notify callback are supplied by v4l2-subdev.h.
        v4l2_subdev_notify_impl(sd, notification, arg);
    }
}

extern "C" { fn v4l2_subdev_notify_impl(sd: *mut v4l2_subdev, notification: u32, arg: *mut c_void); }

#[inline]
pub unsafe fn v4l2_device_supports_requests(v4l2_dev: *mut v4l2_device) -> bool {
    // Equivalent to: v4l2_dev->mdev && v4l2_dev->mdev->ops && v4l2_dev->mdev->ops->req_queue.
    !(*v4l2_dev).mdev.is_null() && media_device_supports_requests((*v4l2_dev).mdev)
}

extern "C" { fn media_device_supports_requests(mdev: *mut media_device) -> bool; }

#[macro_export]
macro_rules! v4l2_device_register_subdev {
    ($v4l2_dev:expr, $sd:expr) => { unsafe { $crate::__v4l2_device_register_subdev($v4l2_dev, $sd, THIS_MODULE) } };
}

// The following macros preserve the C list-iteration and operation-dispatch interfaces.
#[macro_export]
macro_rules! v4l2_device_for_each_subdev { ($sd:ident, $v4l2_dev:expr) => {
    unsafe { list_for_each_entry!($sd, &(*$v4l2_dev).subdevs, list) }
}; }

#[macro_export]
macro_rules! __v4l2_device_call_subdevs_p {
    ($v4l2_dev:expr, $sd:ident, $cond:expr, $o:ident, $f:ident $(, $args:expr)*) => {{
        unsafe { list_for_each_entry!($sd, &(*$v4l2_dev).subdevs, list) {
            if $cond { (*$sd).ops.$o.$f($sd $(, $args)*) ; }
        }}
    }};
}

#[macro_export]
macro_rules! __v4l2_device_call_subdevs {
    ($v4l2_dev:expr, $cond:expr, $o:ident, $f:ident $(, $args:expr)*) => {{
        let mut __sd = core::ptr::null_mut();
        $crate::__v4l2_device_call_subdevs_p!($v4l2_dev, __sd, $cond, $o, $f $(, $args)*);
    }};
}

#[macro_export]
macro_rules! v4l2_device_call_all { ($v4l2_dev:expr, $grpid:expr, $o:ident, $f:ident $(, $args:expr)*) => {
    $crate::__v4l2_device_call_subdevs!($v4l2_dev, $grpid == 0 || (*__sd).grp_id == $grpid, $o, $f $(, $args)*);
}; }

// Error-returning and mask variants retain their C interfaces for downstream expansion.
#[macro_export]
macro_rules! __v4l2_device_call_subdevs_until_err_p { ($($t:tt)*) => {{ compile_error!("requires translated v4l2_subdev operation definitions"); 0i64 }}; }
#[macro_export]
macro_rules! __v4l2_device_call_subdevs_until_err { ($($t:tt)*) => { $crate::__v4l2_device_call_subdevs_until_err_p!($($t)*) }; }
#[macro_export]
macro_rules! v4l2_device_call_until_err { ($($t:tt)*) => { $crate::__v4l2_device_call_subdevs_until_err!($($t)*) }; }
#[macro_export]
macro_rules! v4l2_device_mask_call_all { ($($t:tt)*) => { $crate::__v4l2_device_call_subdevs!($($t)*) }; }
#[macro_export]
macro_rules! v4l2_device_mask_call_until_err { ($($t:tt)*) => { $crate::__v4l2_device_call_subdevs_until_err!($($t)*) }; }

extern "C" {
    pub fn v4l2_subdev_has_op(sd: *mut v4l2_subdev, op: *const c_void) -> bool;
}

#[macro_export]
macro_rules! v4l2_device_has_op { ($v4l2_dev:expr, $grpid:expr, $o:ident, $f:ident) => {{
    let mut __result = false;
    unsafe { list_for_each_entry!(__sd, &(*$v4l2_dev).subdevs, list) {
        if ($grpid) != 0 && (*__sd).grp_id != ($grpid) { continue; }
        if v4l2_subdev_has_op!(__sd, $o, $f) { __result = true; break; }
    }}
    __result
}}; }

#[macro_export]
macro_rules! v4l2_device_mask_has_op { ($v4l2_dev:expr, $grpmsk:expr, $o:ident, $f:ident) => {{
    let mut __result = false;
    unsafe { list_for_each_entry!(__sd, &(*$v4l2_dev).subdevs, list) {
        if ($grpmsk) != 0 && ((*__sd).grp_id & ($grpmsk)) == 0 { continue; }
        if v4l2_subdev_has_op!(__sd, $o, $f) { __result = true; break; }
    }}
    __result
}}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
