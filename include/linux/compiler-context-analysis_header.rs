/* SPDX-License-Identifier: GPL-2.0 */
/* Macros and attributes for compiler-based static context analysis. */

/* C preprocessor attributes have no direct Rust equivalent here.  These
 * macros preserve the source-level interfaces and their intent. */

#[macro_export]
macro_rules! __ctx_lock_type { ($name:ident) => {}; }
#[macro_export]
macro_rules! __reentrant_ctx_lock { () => {}; }
#[macro_export]
macro_rules! __acquires_ctx_lock { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! __acquires_shared_ctx_lock { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! __try_acquires_ctx_lock { ($ret:expr, $var:expr) => {}; }
#[macro_export]
macro_rules! __try_acquires_shared_ctx_lock { ($ret:expr, $var:expr) => {}; }
#[macro_export]
macro_rules! __releases_ctx_lock { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! __releases_shared_ctx_lock { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! __returns_ctx_lock { ($var:expr) => {}; }
#[macro_export]
macro_rules! __excludes_ctx_lock { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! __requires_ctx_lock { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! __requires_shared_ctx_lock { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! __assumes_ctx_lock { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! __assumes_shared_ctx_lock { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! __guarded_by { ($($arg:tt)*) => {}; }
#[macro_export]
macro_rules! __pt_guarded_by { ($($arg:tt)*) => {}; }

/* context_lock_struct(name, ...) declares a C context-lock struct and its
 * acquire/release helpers.  Rust has no corresponding attribute machinery;
 * the macro retains the type declaration and helper behavior. */
#[macro_export]
macro_rules! context_lock_struct {
    ($name:ident { $($body:tt)* }) => {
        #[repr(C)]
        pub struct $name { $($body)* }
        #[inline(always)] pub unsafe fn __acquire_ctx_lock(_var: *const $name) {}
        #[inline(always)] pub unsafe fn __acquire_shared_ctx_lock(_var: *const $name) {}
        #[inline(always)] pub unsafe fn __try_acquire_ctx_lock(_var: *const $name, ret: bool) -> bool { ret }
        #[inline(always)] pub unsafe fn __try_acquire_shared_ctx_lock(_var: *const $name, ret: bool) -> bool { ret }
        #[inline(always)] pub unsafe fn __release_ctx_lock(_var: *const $name) {}
        #[inline(always)] pub unsafe fn __release_shared_ctx_lock(_var: *const $name) {}
        #[inline(always)] pub unsafe fn __assume_ctx_lock(_var: *const $name) {}
        #[inline(always)] pub unsafe fn __assume_shared_ctx_lock(_var: *const $name) {}
    };
    ($name:ident) => {
        #[repr(C)] pub struct $name;
    };
}

#[macro_export] macro_rules! disable_context_analysis { () => {}; }
#[macro_export] macro_rules! enable_context_analysis { () => {}; }
#[macro_export] macro_rules! __no_context_analysis { () => {}; }

#[macro_export]
macro_rules! context_unsafe {
    ($($expr:expr),+ $(,)?) => {{ $($expr);+ }};
}
#[macro_export]
macro_rules! __context_unsafe { ($comment:expr) => {}; }

/* Context lock alias barrier. */
#[inline(always)]
pub unsafe fn _context_unsafe_alias(p: *mut *mut core::ffi::c_void) {
    let _ = p;
}
#[macro_export]
macro_rules! context_unsafe_alias {
    ($p:expr) => {{
        unsafe { $crate::_context_unsafe_alias((&mut ($p) as *mut _).cast()) }
    }};
}

/* Abstract global context-lock declarations. */
#[macro_export]
macro_rules! token_context_lock {
    ($name:ident $(, $body:tt)*) => {
        #[repr(C)] pub struct __ctx_lock_$name;
        pub static $name: *const __ctx_lock_$name = core::ptr::null();
    };
}
#[macro_export]
macro_rules! token_context_lock_instance {
    ($ctx:ident, $name:ident) => {
        pub static $name: *const __ctx_lock_$ctx = core::ptr::null();
    };
}

#[macro_export] macro_rules! __must_hold { ($($arg:tt)*) => { $crate::__requires_ctx_lock!($($arg)*) }; }
#[macro_export] macro_rules! __must_not_hold { ($($arg:tt)*) => { $crate::__excludes_ctx_lock!($($arg)*) }; }
#[macro_export] macro_rules! __acquires { ($($arg:tt)*) => { $crate::__acquires_ctx_lock!($($arg)*) }; }
#[macro_export] macro_rules! __releases { ($($arg:tt)*) => { $crate::__releases_ctx_lock!($($arg)*) }; }
#[macro_export] macro_rules! __acquire { ($x:expr) => {{ unsafe { $crate::__acquire_ctx_lock($x) } }}; }
#[macro_export] macro_rules! __release { ($x:expr) => {{ unsafe { $crate::__release_ctx_lock($x) } }}; }
#[macro_export] macro_rules! __must_hold_shared { ($($arg:tt)*) => { $crate::__requires_shared_ctx_lock!($($arg)*) }; }
#[macro_export] macro_rules! __acquires_shared { ($($arg:tt)*) => { $crate::__acquires_shared_ctx_lock!($($arg)*) }; }
#[macro_export] macro_rules! __releases_shared { ($($arg:tt)*) => { $crate::__releases_shared_ctx_lock!($($arg)*) }; }
#[macro_export] macro_rules! __acquire_shared { ($x:expr) => {{ unsafe { $crate::__acquire_shared_ctx_lock($x) } }}; }
#[macro_export] macro_rules! __release_shared { ($x:expr) => {{ unsafe { $crate::__release_shared_ctx_lock($x) } }}; }

#[macro_export]
macro_rules! __cond_acquires { ($ret:tt, $x:expr) => {}; }
#[macro_export]
macro_rules! __cond_releases { ($ret:tt, $x:expr) => {}; }
#[macro_export]
macro_rules! __cond_acquires_shared { ($ret:tt, $x:expr) => {}; }

#[macro_export]
macro_rules! __acquire_ret {
    ($call:expr, $ret_expr:expr) => {{ let __ret = $call; $crate::__acquire!($ret_expr); __ret }};
}
#[macro_export]
macro_rules! __acquire_shared_ret {
    ($call:expr, $ret_expr:expr) => {{ let __ret = $call; $crate::__acquire_shared!($ret_expr); __ret }};
}

#[macro_export] macro_rules! __acquires_ret { () => {}; }
#[macro_export] macro_rules! __acquires_shared_ret { () => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
