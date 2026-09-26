// SPDX-License-Identifier: GPL-2.0
/*
 * A fast, small, non-recursive O(n log n) sort for the Linux kernel
 *
 * This performs n*log2(n) + 0.37*n + o(n) comparisons on average,
 * and 1.5*n*log2(n) + O(n) in the (very contrived) worst case.
 *
 * Quicksort manages n*log2(n) - 1.26*n for random inputs (1.63*n
 * better) at the expense of stack usage and much larger code to avoid
 * quicksort's O(n^2) worst case.
 */
//! Bottom-up heapsort, preserving the original callback and scheduling order.

#[path = "../include/linux/sort_header.rs"]
pub mod declarations;
#[path = "sort_sched.rs"]
mod scheduling;
use core::{ffi::c_void, mem::MaybeUninit, ptr};
use declarations::{SortCmp, SortPriv, SortRCmp, SortRSwap, SortSwap};

type Cmp = unsafe extern "C" fn(*const c_void, *const c_void) -> i32;
type CmpR = unsafe extern "C" fn(*const c_void, *const c_void, *const c_void) -> i32;
type Swap = unsafe extern "C" fn(*mut c_void, *mut c_void, i32);
type SwapR = unsafe extern "C" fn(*mut c_void, *mut c_void, i32, *const c_void);

#[repr(C)]
struct Wrapper {
    cmp: Option<Cmp>,
    swap: Option<Swap>,
}

#[derive(Clone, Copy)]
enum Swapper {
    Default,
    Words64,
    Words32,
    Bytes,
    Wrapper,
    Custom(SwapR),
}

#[inline(always)]
fn is_aligned(base: *const c_void, size: usize, align: u8) -> bool {
    let _ = base;
    #[cfg(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS)]
    let lsbits = size as u8;
    #[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
    let lsbits = size as u8 | base as usize as u8;
    lsbits & align.wrapping_sub(1) == 0
}

// MaybeUninit preserves arbitrary padding without constructing an invalid value.
// Unaligned accesses also implement the efficient-unaligned configuration.
unsafe fn exchange<T>(a: *mut u8, b: *mut u8, n: usize) {
    // SAFETY: Both byte ranges are accessible; MaybeUninit permits uninitialized
    // bytes. Reads precede writes, so even identical pointers retain C behavior.
    unsafe {
        let a = a.add(n).cast::<MaybeUninit<T>>();
        let b = b.add(n).cast::<MaybeUninit<T>>();
        let value = ptr::read_unaligned(a);
        ptr::write_unaligned(a, ptr::read_unaligned(b));
        ptr::write_unaligned(b, value);
    }
}

unsafe fn do_swap(a: *mut u8, b: *mut u8, size: usize, swap: Swapper, priv_: *const c_void) {
    // SAFETY: Sort's caller supplies valid objects and callbacks; selected word
    // sizes divide size. The wrapper exists for the entire invocation.
    unsafe {
        match swap {
            Swapper::Wrapper => ((*priv_.cast::<Wrapper>()).swap.unwrap_unchecked())(
                a.cast(),
                b.cast(),
                size as i32,
            ),
            Swapper::Custom(f) => f(a.cast(), b.cast(), size as i32, priv_),
            Swapper::Words64 => {
                let mut n = size;
                loop {
                    #[cfg(CONFIG_64BIT)]
                    {
                        n = n.wrapping_sub(8);
                        exchange::<u64>(a, b, n);
                    }
                    #[cfg(not(CONFIG_64BIT))]
                    {
                        n = n.wrapping_sub(4);
                        exchange::<u32>(a, b, n);
                        n = n.wrapping_sub(4);
                        exchange::<u32>(a, b, n);
                    }
                    if n == 0 {
                        break;
                    }
                }
            }
            Swapper::Words32 => {
                let mut n = size;
                loop {
                    n = n.wrapping_sub(4);
                    exchange::<u32>(a, b, n);
                    if n == 0 {
                        break;
                    }
                }
            }
            Swapper::Bytes => {
                let mut n = size;
                loop {
                    n = n.wrapping_sub(1);
                    exchange::<u8>(a, b, n);
                    if n == 0 {
                        break;
                    }
                }
            }
            Swapper::Default => core::hint::unreachable_unchecked(),
        }
    }
}

unsafe fn do_cmp(a: *const u8, b: *const u8, cmp: Option<CmpR>, priv_: *const c_void) -> i32 {
    // SAFETY: As in C, NULL selects the ordinary-callback wrapper. This is not
    // evaluated in the original no-work domains.
    unsafe {
        match cmp {
            Some(f) => f(a.cast(), b.cast(), priv_),
            None => ((*priv_.cast::<Wrapper>()).cmp.unwrap_unchecked())(a.cast(), b.cast()),
        }
    }
}

#[inline(always)]
fn parent(i: usize, lsbit: u32, size: usize) -> usize {
    let i = i.wrapping_sub(size);
    i.wrapping_sub(size & (i & lsbit as usize).wrapping_neg()) / 2
}

unsafe fn sort_impl(
    base: *mut c_void,
    num: usize,
    size: usize,
    cmp: Option<CmpR>,
    mut swap: Swapper,
    priv_: *const c_void,
    may_schedule: bool,
) {
    // SAFETY: The public API's array and callback contracts apply throughout.
    unsafe {
        let base = base.cast::<u8>();
        let mut n = num.wrapping_mul(size);
        let mut a = (num / 2).wrapping_mul(size);
        let lsbit = (size & size.wrapping_neg()) as u32;
        let mut shift = 0;
        if a == 0 {
            return;
        }
        if matches!(swap, Swapper::Wrapper) && (*priv_.cast::<Wrapper>()).swap.is_none() {
            swap = Swapper::Default;
        }
        if matches!(swap, Swapper::Default) {
            swap = if is_aligned(base.cast(), size, 8) {
                Swapper::Words64
            } else if is_aligned(base.cast(), size, 4) {
                Swapper::Words32
            } else {
                Swapper::Bytes
            };
        }
        loop {
            if a != 0 {
                a = a.wrapping_sub(size.wrapping_shl(shift));
            } else if n > size.wrapping_mul(3) {
                n = n.wrapping_sub(size);
                do_swap(base, base.add(n), size, swap, priv_);
                shift = u32::from(
                    do_cmp(base.add(size), base.add(size.wrapping_mul(2)), cmp, priv_) <= 0,
                );
                a = size.wrapping_shl(shift);
                n = n.wrapping_sub(size);
                do_swap(base.add(a), base.add(n), size, swap, priv_);
            } else {
                break;
            }
            let mut b = a;
            let (mut c, mut d);
            loop {
                c = b.wrapping_mul(2).wrapping_add(size);
                d = c.wrapping_add(size);
                if d >= n {
                    break;
                }
                b = if do_cmp(base.add(c), base.add(d), cmp, priv_) > 0 {
                    c
                } else {
                    d
                };
            }
            if d == n {
                b = c;
            }
            while b != a && do_cmp(base.add(a), base.add(b), cmp, priv_) >= 0 {
                b = parent(b, lsbit, size);
            }
            c = b;
            while b != a {
                b = parent(b, lsbit, size);
                do_swap(base.add(b), base.add(c), size, swap, priv_);
            }
            if may_schedule {
                scheduling::cond_resched(concat!(file!(), "\0").as_ptr().cast(), line!() as i32);
            }
        }
        n = n.wrapping_sub(size);
        do_swap(base, base.add(n), size, swap, priv_);
        if n == size.wrapping_mul(2) && do_cmp(base, base.add(size), cmp, priv_) > 0 {
            do_swap(base, base.add(size), size, swap, priv_);
        }
    }
}

/// Sorts an array with context-aware callbacks.
///
/// # Safety
/// The caller provides exclusive access to num objects of size bytes and valid
/// callbacks/context whenever invoked by the original C algorithm.
#[no_mangle]
pub unsafe extern "C" fn sort_r(
    base: *mut c_void,
    num: usize,
    size: usize,
    cmp: SortRCmp,
    swap: SortRSwap,
    priv_: SortPriv,
) {
    // SAFETY: Forward the caller's contract without strengthening nullability.
    unsafe {
        sort_impl(
            base,
            num,
            size,
            cmp.into_option(),
            swap.into_option().map_or(Swapper::Default, Swapper::Custom),
            priv_.as_ptr(),
            false,
        );
    }
}

/// Sorts an array with context-aware callbacks and periodic rescheduling.
///
/// # Safety
/// Same as sort_r; the calling context must also permit cond_resched().
#[no_mangle]
pub unsafe extern "C" fn sort_r_nonatomic(
    base: *mut c_void,
    num: usize,
    size: usize,
    cmp: SortRCmp,
    swap: SortRSwap,
    priv_: SortPriv,
) {
    // SAFETY: Forward the caller's array, callback and scheduling contracts.
    unsafe {
        sort_impl(
            base,
            num,
            size,
            cmp.into_option(),
            swap.into_option().map_or(Swapper::Default, Swapper::Custom),
            priv_.as_ptr(),
            true,
        );
    }
}

/// Sorts an array, selecting the original built-in swap when swap is NULL.
///
/// # Safety
/// Same array and callback requirements as sort_r.
#[no_mangle]
pub unsafe extern "C" fn sort(
    base: *mut c_void,
    num: usize,
    size: usize,
    cmp: SortCmp,
    swap: SortSwap,
) {
    let w = Wrapper {
        cmp: cmp.into_option(),
        swap: swap.into_option(),
    };
    // SAFETY: The wrapper is live until sort_impl returns.
    unsafe {
        sort_impl(
            base,
            num,
            size,
            None,
            Swapper::Wrapper,
            ptr::addr_of!(w).cast(),
            false,
        );
    }
}

/// Sorts an array with periodic rescheduling.
///
/// # Safety
/// Same as sort; the calling context must also permit cond_resched().
#[no_mangle]
pub unsafe extern "C" fn sort_nonatomic(
    base: *mut c_void,
    num: usize,
    size: usize,
    cmp: SortCmp,
    swap: SortSwap,
) {
    let w = Wrapper {
        cmp: cmp.into_option(),
        swap: swap.into_option(),
    };
    // SAFETY: The wrapper is live until sort_impl returns.
    unsafe {
        sort_impl(
            base,
            num,
            size,
            None,
            Swapper::Wrapper,
            ptr::addr_of!(w).cast(),
            true,
        );
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
