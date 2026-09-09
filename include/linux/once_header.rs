/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::c_ulong;

#[repr(C)]
pub struct static_key_true {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

extern "C" {
    pub fn __do_once_start(done: *mut bool, flags: *mut c_ulong) -> bool;
    pub fn __do_once_done(
        done: *mut bool,
        once_key: *mut static_key_true,
        flags: *mut c_ulong,
        module: *mut module,
    );

    pub fn __do_once_sleepable_start(done: *mut bool) -> bool;
    pub fn __do_once_sleepable_done(
        done: *mut bool,
        once_key: *mut static_key_true,
        module: *mut module,
    );
}

// Helpers used from arbitrary contexts. Hard irqs are blocked, be cautious.
// Variant for process contexts only.

// The kernel's static-branch machinery and THIS_MODULE are supplied by the
// surrounding translation. The local statics preserve DO_ONCE's per-call-site
// state; the zero initialization represents DEFINE_STATIC_KEY_TRUE here.
#[macro_export]
macro_rules! DO_ONCE {
    ($func:path $(, $arg:expr)*) => {{
        static mut ___DONE: bool = false;
        static mut ___ONCE_KEY: $crate::static_key_true = unsafe { core::mem::zeroed() };
        let mut ___ret: bool = false;
        if unsafe { $crate::static_branch_unlikely(&___ONCE_KEY) } {
            let mut ___flags: core::ffi::c_ulong = 0;
            ___ret = unsafe { $crate::__do_once_start(&mut ___DONE, &mut ___flags) };
            if ___ret {
                $func($($arg),*);
                unsafe {
                    $crate::__do_once_done(
                        &mut ___DONE,
                        &mut ___ONCE_KEY,
                        &mut ___flags,
                        $crate::THIS_MODULE,
                    );
                }
            }
        }
        ___ret
    }};
}

#[macro_export]
macro_rules! DO_ONCE_SLEEPABLE {
    ($func:path $(, $arg:expr)*) => {{
        static mut ___DONE: bool = false;
        static mut ___ONCE_KEY: $crate::static_key_true = unsafe { core::mem::zeroed() };
        let mut ___ret: bool = false;
        if unsafe { $crate::static_branch_unlikely(&___ONCE_KEY) } {
            ___ret = unsafe { $crate::__do_once_sleepable_start(&mut ___DONE) };
            if ___ret {
                $func($($arg),*);
                unsafe {
                    $crate::__do_once_sleepable_done(
                        &mut ___DONE,
                        &mut ___ONCE_KEY,
                        $crate::THIS_MODULE,
                    );
                }
            }
        }
        ___ret
    }};
}

#[macro_export]
macro_rules! get_random_once {
    ($buf:expr, $nbytes:expr) => {
        $crate::DO_ONCE!($crate::get_random_bytes, $buf, $nbytes)
    };
}

#[macro_export]
macro_rules! get_random_sleepable_once {
    ($buf:expr, $nbytes:expr) => {
        $crate::DO_ONCE_SLEEPABLE!($crate::get_random_bytes, $buf, $nbytes)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
