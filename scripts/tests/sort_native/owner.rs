//! Private harness of the complete production candidate.
#![no_std]
#![feature(cfi_encoding)]
extern crate self as bindings;
extern crate self as ffi;
pub use core::ffi::c_void;
/// Genuine bindgen declarations plus the targeted sort adapters.
#[allow(missing_docs, non_camel_case_types, dead_code, unreachable_pub)]
mod generated {
    include!(env!("SORT_GENERATED"));
}
pub use generated::*;
include!(env!("SORT_ADAPTER_MODULE"));
unsafe extern "C" {
    /// Original nonpreempt scheduler dispatch supplied by the test driver.
    pub fn __cond_resched() -> i32;
}
#[path = "../../../lib/sort.rs"]
pub mod implementation;
unsafe extern "C" {
    fn fixture_panic() -> !;
}
#[panic_handler]
fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
    // SAFETY: Test driver terminates execution on unexpected panic.
    unsafe { fixture_panic() }
}

/// Invoke all four exports through the corrected Rust public declarations.
/// # Safety
/// Caller supplies the original sort contract.
#[no_mangle]
pub unsafe extern "C" fn rust_consumer(
    mode: u32,
    base: *mut c_void,
    num: usize,
    size: usize,
    cmp: bindings::cmp_func_t,
    swap: bindings::swap_func_t,
    cmpr: bindings::cmp_r_func_t,
    swapr: bindings::swap_r_func_t,
    priv_: *const c_void,
) {
    // Volatile loads retain protected outer indirect calls at every opt level.
    // SAFETY: These addresses contain genuine function pointers of their exact
    // public declaration types; array/callback requirements come from caller.
    unsafe {
        match mode {
            0 => {
                core::ptr::read_volatile(&(bindings::sort as unsafe extern "C" fn(_, _, _, _, _)))(
                    base,
                    num,
                    size,
                    bindings::SortCmp::from_option(cmp),
                    bindings::SortSwap::from_option(swap),
                )
            }
            1 => core::ptr::read_volatile(
                &(bindings::sort_nonatomic as unsafe extern "C" fn(_, _, _, _, _)),
            )(
                base,
                num,
                size,
                bindings::SortCmp::from_option(cmp),
                bindings::SortSwap::from_option(swap),
            ),
            2 => core::ptr::read_volatile(
                &(bindings::sort_r as unsafe extern "C" fn(_, _, _, _, _, _)),
            )(
                base,
                num,
                size,
                bindings::SortRCmp::from_option(cmpr),
                bindings::SortRSwap::from_option(swapr),
                bindings::SortPriv::from_ptr(priv_),
            ),
            _ => core::ptr::read_volatile(
                &(bindings::sort_r_nonatomic as unsafe extern "C" fn(_, _, _, _, _, _)),
            )(
                base,
                num,
                size,
                bindings::SortRCmp::from_option(cmpr),
                bindings::SortRSwap::from_option(swapr),
                bindings::SortPriv::from_ptr(priv_),
            ),
        }
    }
}

/// Verify full-width signed and unsigned comparison domains.
#[no_mangle]
pub extern "C" fn cmp_int_domains() -> i32 {
    use implementation::declarations::cmp_int;
    i32::from(
        cmp_int(u128::MAX, 1) == 1
            && cmp_int(i128::MIN, i128::MAX) == -1
            && cmp_int(u64::MAX, 0) == 1
            && cmp_int(-1i64, 0) == -1
            && cmp_int(7u8, 7) == 0,
    )
}

unsafe extern "C" {
    #[link_name = "sort"]
    fn raw_sort(
        base: *mut c_void,
        num: usize,
        size: usize,
        cmp: bindings::cmp_func_t,
        swap: bindings::swap_func_t,
    );
    #[link_name = "sort_r"]
    fn raw_sort_r(
        base: *mut c_void,
        num: usize,
        size: usize,
        cmp: bindings::cmp_r_func_t,
        swap: bindings::swap_r_func_t,
        priv_: *const c_void,
    );
    #[link_name = "sort_r"]
    fn wrong_context_sort_r(
        base: *mut c_void,
        num: usize,
        size: usize,
        cmp: bindings::SortRCmp,
        swap: bindings::SortRSwap,
        priv_: *const c_void,
    );
}

/// Negative control: genuine Option aliases have the wrong outer KCFI identity.
/// # Safety
/// Called only by the standalone deliberate rejection fixture.
#[no_mangle]
pub unsafe extern "C" fn rust_negative(
    mode: u32,
    base: *mut c_void,
    cmp: bindings::cmp_func_t,
    cmpr: bindings::cmp_r_func_t,
    priv_: *const c_void,
) {
    // SAFETY: Ordinary ABI is valid; protected builds deliberately trap here.
    unsafe {
        if mode == 0 {
            core::ptr::read_volatile(&(raw_sort as unsafe extern "C" fn(_, _, _, _, _)))(
                base,
                2usize,
                1usize,
                cmp,
                None::<unsafe extern "C" fn(*mut c_void, *mut c_void, i32)>,
            );
        } else if mode == 1 {
            core::ptr::read_volatile(&(raw_sort_r as unsafe extern "C" fn(_, _, _, _, _, _)))(
                base,
                2usize,
                1usize,
                cmpr,
                None::<unsafe extern "C" fn(*mut c_void, *mut c_void, i32, *const c_void)>,
                priv_,
            );
        } else {
            core::ptr::read_volatile(
                &(wrong_context_sort_r as unsafe extern "C" fn(_, _, _, _, _, _)),
            )(
                base,
                2usize,
                1usize,
                bindings::SortRCmp::from_option(cmpr),
                bindings::SortRSwap::from_option(None),
                priv_,
            );
        }
    }
}
