/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/kcsan-checks.h. */

/* Dependencies supplied by the surrounding kernel translation. */

pub const KCSAN_ACCESS_WRITE: i32 = 1 << 0;
pub const KCSAN_ACCESS_COMPOUND: i32 = 1 << 1;
pub const KCSAN_ACCESS_ATOMIC: i32 = 1 << 2;
pub const KCSAN_ACCESS_ASSERT: i32 = 1 << 3;
pub const KCSAN_ACCESS_SCOPED: i32 = 1 << 4;

#[cfg(feature = "CONFIG_KCSAN")]
extern "C" {
    pub fn __kcsan_check_access(ptr: *const core::ffi::c_void, size: usize, ty: i32);
    pub fn __kcsan_mb();
    pub fn __kcsan_wmb();
    pub fn __kcsan_rmb();
    pub fn __kcsan_release();
    pub fn kcsan_disable_current();
    pub fn kcsan_enable_current();
    pub fn kcsan_enable_current_nowarn();
    pub fn kcsan_nestable_atomic_begin();
    pub fn kcsan_nestable_atomic_end();
    pub fn kcsan_flat_atomic_begin();
    pub fn kcsan_flat_atomic_end();
    pub fn kcsan_atomic_next(n: i32);
    pub fn kcsan_set_access_mask(mask: usize);
}

#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline]
pub unsafe fn __kcsan_check_access(_ptr: *const core::ffi::c_void, _size: usize, _ty: i32) {}
#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline] pub unsafe fn __kcsan_mb() {}
#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline] pub unsafe fn __kcsan_wmb() {}
#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline] pub unsafe fn __kcsan_rmb() {}
#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline] pub unsafe fn __kcsan_release() {}
#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline] pub unsafe fn kcsan_disable_current() {}
#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline] pub unsafe fn kcsan_enable_current() {}
#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline] pub unsafe fn kcsan_enable_current_nowarn() {}
#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline] pub unsafe fn kcsan_nestable_atomic_begin() {}
#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline] pub unsafe fn kcsan_nestable_atomic_end() {}
#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline] pub unsafe fn kcsan_flat_atomic_begin() {}
#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline] pub unsafe fn kcsan_flat_atomic_end() {}
#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline] pub unsafe fn kcsan_atomic_next(_n: i32) {}
#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline] pub unsafe fn kcsan_set_access_mask(_mask: usize) {}

#[repr(C)]
pub union kcsan_scoped_access_union {
    pub list: crate::list_head,
    pub stack_depth: i32,
}

#[repr(C)]
pub struct kcsan_scoped_access {
    pub _union: kcsan_scoped_access_union,
    pub ptr: *const core::ffi::c_void,
    pub size: usize,
    pub ty: i32,
    pub ip: usize,
}

#[cfg(feature = "CONFIG_KCSAN")]
extern "C" {
    pub fn kcsan_begin_scoped_access(
        ptr: *const core::ffi::c_void, size: usize, ty: i32,
        sa: *mut kcsan_scoped_access,
    ) -> *mut kcsan_scoped_access;
    pub fn kcsan_end_scoped_access(sa: *mut kcsan_scoped_access);
}

#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline]
pub unsafe fn kcsan_begin_scoped_access(
    _ptr: *const core::ffi::c_void, _size: usize, _ty: i32,
    sa: *mut kcsan_scoped_access,
) -> *mut kcsan_scoped_access { sa }
#[cfg(not(feature = "CONFIG_KCSAN"))]
#[inline]
pub unsafe fn kcsan_end_scoped_access(_sa: *mut kcsan_scoped_access) {}

pub const __KCSAN_BARRIER_TO_SIGNAL_FENCE_MB: i32 = 5; // __ATOMIC_SEQ_CST
pub const __KCSAN_BARRIER_TO_SIGNAL_FENCE_WMB: i32 =  acq_rel_order();
pub const __KCSAN_BARRIER_TO_SIGNAL_FENCE_RMB: i32 = 2; // __ATOMIC_ACQUIRE
pub const __KCSAN_BARRIER_TO_SIGNAL_FENCE_RELEASE: i32 = 3; // __ATOMIC_RELEASE

const fn acq_rel_order() -> i32 { 4 } // __ATOMIC_ACQ_REL

#[cfg(feature = "__SANITIZE_THREAD__")]
#[inline]
pub unsafe fn kcsan_check_access(ptr: *const core::ffi::c_void, size: usize, ty: i32) {
    __kcsan_check_access(ptr, size, ty)
}
#[cfg(not(feature = "__SANITIZE_THREAD__"))]
#[inline] pub unsafe fn kcsan_check_access(_ptr: *const core::ffi::c_void, _size: usize, _ty: i32) {}

#[macro_export]
macro_rules! __kcsan_check_read { ($ptr:expr, $size:expr) => { unsafe { $crate::__kcsan_check_access(($ptr) as *const _, $size, 0) } }; }
#[macro_export]
macro_rules! __kcsan_check_write { ($ptr:expr, $size:expr) => { unsafe { $crate::__kcsan_check_access(($ptr) as *const _, $size, $crate::KCSAN_ACCESS_WRITE) } }; }
#[macro_export]
macro_rules! __kcsan_check_read_write { ($ptr:expr, $size:expr) => { unsafe { $crate::__kcsan_check_access(($ptr) as *const _, $size, $crate::KCSAN_ACCESS_COMPOUND | $crate::KCSAN_ACCESS_WRITE) } }; }
#[macro_export]
macro_rules! kcsan_check_read { ($ptr:expr, $size:expr) => { unsafe { $crate::kcsan_check_access(($ptr) as *const _, $size, 0) } }; }
#[macro_export]
macro_rules! kcsan_check_write { ($ptr:expr, $size:expr) => { unsafe { $crate::kcsan_check_access(($ptr) as *const _, $size, $crate::KCSAN_ACCESS_WRITE) } }; }
#[macro_export]
macro_rules! kcsan_check_read_write { ($ptr:expr, $size:expr) => { unsafe { $crate::kcsan_check_access(($ptr) as *const _, $size, $crate::KCSAN_ACCESS_COMPOUND | $crate::KCSAN_ACCESS_WRITE) } }; }

#[cfg(not(feature = "CONFIG_KCSAN_IGNORE_ATOMICS"))]
#[macro_export] macro_rules! kcsan_check_atomic_read { ($ptr:expr, $size:expr) => { unsafe { $crate::kcsan_check_access(($ptr) as *const _, $size, $crate::KCSAN_ACCESS_ATOMIC) } }; }
#[cfg(feature = "CONFIG_KCSAN_IGNORE_ATOMICS")]
#[macro_export] macro_rules! kcsan_check_atomic_read { ($($args:tt)*) => {}; }
#[cfg(not(feature = "CONFIG_KCSAN_IGNORE_ATOMICS"))]
#[macro_export] macro_rules! kcsan_check_atomic_write { ($ptr:expr, $size:expr) => { unsafe { $crate::__kcsan_check_access(($ptr) as *const _, $size, $crate::KCSAN_ACCESS_ATOMIC | $crate::KCSAN_ACCESS_WRITE) } }; }
#[cfg(feature = "CONFIG_KCSAN_IGNORE_ATOMICS")]
#[macro_export] macro_rules! kcsan_check_atomic_write { ($($args:tt)*) => {}; }

#[macro_export]
macro_rules! ASSERT_EXCLUSIVE_WRITER { ($var:expr) => { unsafe { $crate::__kcsan_check_access(&($var) as *const _ as *const _, core::mem::size_of_val(&($var)), $crate::KCSAN_ACCESS_ASSERT) } }; }
#[macro_export]
macro_rules! ASSERT_EXCLUSIVE_ACCESS { ($var:expr) => { unsafe { $crate::__kcsan_check_access(&($var) as *const _ as *const _, core::mem::size_of_val(&($var)), $crate::KCSAN_ACCESS_WRITE | $crate::KCSAN_ACCESS_ASSERT) } }; }
#[macro_export]
macro_rules! ASSERT_EXCLUSIVE_BITS { ($var:expr, $mask:expr) => {{ unsafe { $crate::kcsan_set_access_mask($mask); $crate::__kcsan_check_access(&($var) as *const _ as *const _, core::mem::size_of_val(&($var)), $crate::KCSAN_ACCESS_ASSERT); $crate::kcsan_set_access_mask(0); $crate::kcsan_atomic_next(1); } }}; }

/* Scoped assertion helpers retain the source's scope/lifetime intent. */
#[macro_export]
macro_rules! ASSERT_EXCLUSIVE_WRITER_SCOPED { ($var:expr) => { $crate::ASSERT_EXCLUSIVE_WRITER!($var) }; }
#[macro_export]
macro_rules! ASSERT_EXCLUSIVE_ACCESS_SCOPED { ($var:expr) => { $crate::ASSERT_EXCLUSIVE_ACCESS!($var) }; }

#[inline]
pub unsafe fn __kcsan_disable_current() { kcsan_disable_current(); }
#[inline]
pub unsafe fn __kcsan_enable_current() { kcsan_enable_current_nowarn(); }

#[cfg(not(feature = "CONFIG_KCSAN_IGNORE_ATOMICS"))]
#[macro_export]
macro_rules! kcsan_check_atomic_read_write { ($ptr:expr, $size:expr) => { unsafe { $crate::kcsan_check_access(($ptr) as *const _, $size, $crate::KCSAN_ACCESS_ATOMIC | $crate::KCSAN_ACCESS_WRITE | $crate::KCSAN_ACCESS_COMPOUND) } }; }
#[cfg(feature = "CONFIG_KCSAN_IGNORE_ATOMICS")]
#[macro_export]
macro_rules! kcsan_check_atomic_read_write { ($($args:tt)*) => {}; }

#[macro_export]
macro_rules! kcsan_mb { () => { unsafe { $crate::__kcsan_mb() } }; }
#[macro_export]
macro_rules! kcsan_wmb { () => { unsafe { $crate::__kcsan_wmb() } }; }
#[macro_export]
macro_rules! kcsan_rmb { () => { unsafe { $crate::__kcsan_rmb() } }; }
#[macro_export]
macro_rules! kcsan_release { () => { unsafe { $crate::__kcsan_release() } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
