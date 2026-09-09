/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Poor man's closures; I wish we could've done them sanely polymorphic,
 * but...
 */

use core::ffi::c_void;

#[repr(C)]
pub struct delayed_call {
    pub fn_: Option<unsafe extern "C" fn(*mut c_void)>,
    pub arg: *mut c_void,
}

#[macro_export]
macro_rules! DEFINE_DELAYED_CALL {
    ($name:ident) => {
        let mut $name: $crate::delayed_call = $crate::delayed_call {
            fn_: None,
            arg: core::ptr::null_mut(),
        };
    };
}

/* I really wish we had closures with sane typechecking... */
#[inline]
pub unsafe fn set_delayed_call(
    call: *mut delayed_call,
    fn_: Option<unsafe extern "C" fn(*mut c_void)>,
    arg: *mut c_void,
) {
    (*call).fn_ = fn_;
    (*call).arg = arg;
}

#[inline]
pub unsafe fn do_delayed_call(call: *mut delayed_call) {
    if let Some(fn_) = (*call).fn_ {
        fn_((*call).arg);
    }
}

#[inline]
pub unsafe fn clear_delayed_call(call: *mut delayed_call) {
    (*call).fn_ = None;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
