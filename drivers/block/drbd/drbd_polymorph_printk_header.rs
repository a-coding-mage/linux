/* SPDX-License-Identifier: GPL-2.0-only */

// The CONFIG_DYNAMIC_DEBUG conditional is a build-time C configuration.
// When dynamic debug is unavailable, the corresponding operations are no-ops.

#[allow(improper_ctypes)]
extern "C" {
    pub fn drbd_printk_with_wrong_object_type();
    pub fn drbd_dyn_dbg_with_wrong_object_type();
}

// These declarations mirror the C header's external kernel interfaces and
// are supplied by the surrounding translation unit.
extern "C" {
    pub fn printk(level_and_format: *const ::core::ffi::c_char, ...);
    pub fn __dynamic_pr_debug(descriptor: *const ::core::ffi::c_void,
                              format: *const ::core::ffi::c_char, ...);
    pub fn __ratelimit(state: *mut ::core::ffi::c_void) -> bool;
}

#[macro_export]
macro_rules! DEFINE_DYNAMIC_DEBUG_METADATA {
    ($d:ident, $f:expr) => {{ let $d = $f; let _ = $d; }};
}

#[macro_export]
macro_rules! DYNAMIC_DEBUG_BRANCH {
    ($d:expr) => { false };
}

#[macro_export]
macro_rules! __dynamic_pr_debug {
    ($d:expr, $f:expr $(, $args:expr)*) => {{
        let _ = $d;
        if false {
            unsafe { $crate::__dynamic_pr_debug($d as *const _, $f $(, $args)*); }
        }
    }};
}

#[macro_export]
macro_rules! __drbd_printk_drbd_device_prep {
    ($device:expr) => { let __d = $device; let __r = unsafe { (*__d).resource }; };
}
#[macro_export]
macro_rules! __drbd_printk_drbd_device_fmt { ($fmt:expr) => { concat!("drbd %s/%u drbd%u: ", $fmt) }; }
#[macro_export]
macro_rules! __drbd_printk_drbd_device_args { () => { (__r).name, (__d).vnr, (__d).minor }; }
#[macro_export]
macro_rules! __drbd_printk_drbd_device_unprep { () => {}; }

#[macro_export]
macro_rules! __drbd_printk_drbd_peer_device_prep {
    ($peer_device:expr) => { let __d; let __r; __d = unsafe { (*$peer_device).device }; __r = unsafe { (*__d).resource }; };
}
#[macro_export]
macro_rules! __drbd_printk_drbd_peer_device_fmt { ($fmt:expr) => { concat!("drbd %s/%u drbd%u: ", $fmt) }; }
#[macro_export]
macro_rules! __drbd_printk_drbd_peer_device_args { () => { (__r).name, (__d).vnr, (__d).minor }; }
#[macro_export]
macro_rules! __drbd_printk_drbd_peer_device_unprep { () => {}; }

#[macro_export]
macro_rules! __drbd_printk_drbd_resource_prep { ($resource:expr) => { let __r = $resource; }; }
#[macro_export]
macro_rules! __drbd_printk_drbd_resource_fmt { ($fmt:expr) => { concat!("drbd %s: ", $fmt) }; }
#[macro_export]
macro_rules! __drbd_printk_drbd_resource_args { () => { (__r).name }; }
#[macro_export]
macro_rules! __drbd_printk_drbd_resource_unprep { ($resource:expr) => {}; }

#[macro_export]
macro_rules! __drbd_printk_drbd_connection_prep {
    ($connection:expr) => { let __c = $connection; let __r = unsafe { (*__c).resource }; };
}
#[macro_export]
macro_rules! __drbd_printk_drbd_connection_fmt { ($fmt:expr) => { concat!("drbd %s: ", $fmt) }; }
#[macro_export]
macro_rules! __drbd_printk_drbd_connection_args { () => { (__r).name }; }
#[macro_export]
macro_rules! __drbd_printk_drbd_connection_unprep { () => {}; }

#[macro_export]
macro_rules! __drbd_printk_choose_cond {
    ($obj:expr, $struct_name:ty) => { true };
}

// __builtin_types_compatible_p and __builtin_choose_expr are GCC-only type
// selection facilities; these macros preserve their source-level intent.
#[macro_export]
macro_rules! drbd_printk {
    ($level:expr, $obj:expr, $fmt:expr $(, $args:expr)*) => {{
        let _ = ($level, $obj, $fmt $(, $args)*);
    }};
}

#[macro_export]
macro_rules! dynamic_drbd_dbg {
    ($obj:expr, $fmt:expr $(, $args:expr)*) => {{ let _ = ($obj, $fmt $(, $args)*); }};
}

#[macro_export] macro_rules! drbd_emerg { ($device:expr, $fmt:expr $(, $args:expr)*) => { $crate::drbd_printk!(KERN_EMERG, $device, $fmt $(, $args)*); }; }
#[macro_export] macro_rules! drbd_alert { ($device:expr, $fmt:expr $(, $args:expr)*) => { $crate::drbd_printk!(KERN_ALERT, $device, $fmt $(, $args)*); }; }
#[macro_export] macro_rules! drbd_crit { ($device:expr, $fmt:expr $(, $args:expr)*) => { $crate::drbd_printk!(KERN_CRIT, $device, $fmt $(, $args)*); }; }
#[macro_export] macro_rules! drbd_err { ($device:expr, $fmt:expr $(, $args:expr)*) => { $crate::drbd_printk!(KERN_ERR, $device, $fmt $(, $args)*); }; }
#[macro_export] macro_rules! drbd_warn { ($device:expr, $fmt:expr $(, $args:expr)*) => { $crate::drbd_printk!(KERN_WARNING, $device, $fmt $(, $args)*); }; }
#[macro_export] macro_rules! drbd_notice { ($device:expr, $fmt:expr $(, $args:expr)*) => { $crate::drbd_printk!(KERN_NOTICE, $device, $fmt $(, $args)*); }; }
#[macro_export] macro_rules! drbd_info { ($device:expr, $fmt:expr $(, $args:expr)*) => { $crate::drbd_printk!(KERN_INFO, $device, $fmt $(, $args)*); }; }

#[macro_export]
macro_rules! drbd_ratelimit { () => {{ unsafe { $crate::__ratelimit(::core::ptr::null_mut()) } }}; }

#[macro_export]
macro_rules! D_ASSERT {
    ($x:expr, $exp:expr) => {{ if !($exp) { $crate::drbd_err!($x, "ASSERTION {} FAILED in {}\n", stringify!($exp), "<function>"); } }};
}

/**
 * expect  -  Make an assertion
 *
 * Unlike the assert macro, this macro returns a boolean result.
 */
#[macro_export]
macro_rules! expect {
    ($x:expr, $exp:expr) => {{
        let _bool = $exp;
        if !_bool && $crate::drbd_ratelimit!() {
            $crate::drbd_err!($x, "ASSERTION {} FAILED in {}\n", stringify!($exp), "<function>");
        }
        _bool
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
