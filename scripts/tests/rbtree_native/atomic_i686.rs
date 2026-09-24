// Test-only pointer API transport to unchanged C READ/WRITE_ONCE helper macros.
// Native tests use the actual kernel::sync::atomic implementation instead.
use core::ffi::c_void;
pub struct Relaxed;
pub struct Acquire;
pub struct Release;
pub trait LoadOrder { const ACQUIRE: bool; }
pub trait StoreOrder { const RELEASE: bool; }
impl LoadOrder for Relaxed { const ACQUIRE: bool = false; }
impl LoadOrder for Acquire { const ACQUIRE: bool = true; }
impl StoreOrder for Relaxed { const RELEASE: bool = false; }
impl StoreOrder for Release { const RELEASE: bool = true; }
unsafe extern "C" {
    fn rust_helper_atomic_ptr_read(slot: *mut *const c_void) -> *const c_void;
    fn rust_helper_atomic_ptr_read_acquire(slot: *mut *const c_void) -> *const c_void;
    fn rust_helper_atomic_ptr_set(slot: *mut *const c_void, value: *const c_void);
    fn rust_helper_atomic_ptr_set_release(slot: *mut *const c_void, value: *const c_void);
}
pub unsafe fn atomic_load<T, O: LoadOrder>(slot: *mut *mut T, _: O) -> *mut T {
    // SAFETY: same fixture pointer/lifetime contract; original C primitive reads.
    unsafe {
        if O::ACQUIRE { rust_helper_atomic_ptr_read_acquire(slot.cast()) }
        else { rust_helper_atomic_ptr_read(slot.cast()) }
    }.cast_mut().cast()
}
pub unsafe fn atomic_store<T, O: StoreOrder>(slot: *mut *mut T, value: *mut T, _: O) {
    // SAFETY: same fixture pointer/lifetime contract; original C primitive writes.
    unsafe {
        if O::RELEASE { rust_helper_atomic_ptr_set_release(slot.cast(), value.cast()) }
        else { rust_helper_atomic_ptr_set(slot.cast(), value.cast()) }
    }
}
