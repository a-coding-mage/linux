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

use core::ffi::c_void;

type CmpFunc = unsafe extern "C" fn(*const c_void, *const c_void) -> i32;
type CmpRFunc = unsafe extern "C" fn(*const c_void, *const c_void, *const c_void) -> i32;
type SwapFunc = unsafe extern "C" fn(*mut c_void, *mut c_void, i32);
type SwapRFunc = unsafe extern "C" fn(*mut c_void, *mut c_void, i32, *const c_void);
type SwapRFuncSelector = usize;

#[repr(C)]
struct Wrapper {
    cmp: Option<CmpFunc>,
    swap: Option<SwapFunc>,
}

#[inline(always)]
fn is_aligned(base: *const c_void, size: usize, align: u8) -> bool {
    let mut lsbits = size as u8;
    #[cfg(not(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS))]
    {
        lsbits |= base as usize as u8;
    }
    (lsbits & (align - 1)) == 0
}

unsafe fn swap_words_32(a: *mut c_void, b: *mut c_void, mut n: usize) {
    loop {
        n -= 4;
        let t = *(a as *mut u8).add(n) as *mut u32;
        let t = *t;
        *(a as *mut u8).add(n) as *mut u32 = *(b as *mut u8).add(n) as *mut u32;
        *(b as *mut u8).add(n) as *mut u32 = t;
        if n == 0 { break; }
    }
}

unsafe fn swap_words_64(a: *mut c_void, b: *mut c_void, mut n: usize) {
    #[cfg(CONFIG_64BIT)]
    {
        loop {
            n -= 8;
            let t = *((a as *mut u8).add(n) as *mut u64);
            *((a as *mut u8).add(n) as *mut u64) = *((b as *mut u8).add(n) as *mut u64);
            *((b as *mut u8).add(n) as *mut u64) = t;
            if n == 0 { break; }
        }
    }
    #[cfg(not(CONFIG_64BIT))]
    {
        loop {
            n -= 4;
            let t = *((a as *mut u8).add(n) as *mut u32);
            *((a as *mut u8).add(n) as *mut u32) = *((b as *mut u8).add(n) as *mut u32);
            *((b as *mut u8).add(n) as *mut u32) = t;
            n -= 4;
            let t = *((a as *mut u8).add(n) as *mut u32);
            *((a as *mut u8).add(n) as *mut u32) = *((b as *mut u8).add(n) as *mut u32);
            *((b as *mut u8).add(n) as *mut u32) = t;
            if n == 0 { break; }
        }
    }
}

unsafe fn swap_bytes(a: *mut c_void, b: *mut c_void, mut n: usize) {
    loop {
        n -= 1;
        let t = *((a as *mut u8).add(n));
        *((a as *mut u8).add(n)) = *((b as *mut u8).add(n));
        *((b as *mut u8).add(n)) = t;
        if n == 0 { break; }
    }
}

const SWAP_WORDS_64: SwapRFuncSelector = 0;
const SWAP_WORDS_32: SwapRFuncSelector = 1;
const SWAP_BYTES: SwapRFuncSelector = 2;
const SWAP_WRAPPER: SwapRFuncSelector = 3;

unsafe fn do_swap(a: *mut c_void, b: *mut c_void, size: usize, swap_func: SwapRFuncSelector, priv_: *const c_void) {
    if swap_func == SWAP_WRAPPER {
        if let Some(swap) = (*(priv_ as *const Wrapper)).swap { swap(a, b, size as i32); }
        return;
    }
    if swap_func == SWAP_WORDS_64 { swap_words_64(a, b, size); }
    else if swap_func == SWAP_WORDS_32 { swap_words_32(a, b, size); }
    else if swap_func == SWAP_BYTES { swap_bytes(a, b, size); }
    else { core::mem::transmute::<usize, SwapRFunc>(swap_func)(a, b, size as i32, priv_); }
}

const CMP_WRAPPER: Option<CmpRFunc> = None;

unsafe fn do_cmp(a: *const c_void, b: *const c_void, cmp: Option<CmpRFunc>, priv_: *const c_void) -> i32 {
    if cmp.is_none() {
        return (*(priv_ as *const Wrapper)).cmp.unwrap()(a, b);
    }
    cmp.unwrap()(a, b, priv_)
}

#[inline(always)]
fn parent(mut i: usize, lsbit: u32, size: usize) -> usize {
    i -= size;
    i -= size & (!i + 1) & ((i as u32 & lsbit) as usize);
    i / 2
}

unsafe fn __sort_r(base: *mut c_void, num: usize, size: usize, cmp_func: Option<CmpRFunc>, mut swap_func: SwapRFuncSelector, priv_: *const c_void, may_schedule: bool) {
    let mut n = num * size;
    let mut a = (num / 2) * size;
    let lsbit = (size & size.wrapping_neg()) as u32;
    let mut shift: usize = 0;
    if a == 0 { return; }
    if swap_func == SWAP_WRAPPER && (*(priv_ as *const Wrapper)).swap.is_none() { swap_func = usize::MAX; }
    if swap_func == usize::MAX {
        if is_aligned(base, size, 8) { swap_func = SWAP_WORDS_64; }
        else if is_aligned(base, size, 4) { swap_func = SWAP_WORDS_32; }
        else { swap_func = SWAP_BYTES; }
    }
    loop {
        let (mut b, mut c, mut d);
        if a != 0 { a -= size << shift; }
        else if n > 3 * size {
            n -= size; do_swap(base, (base as *mut u8).add(n) as *mut c_void, size, swap_func, priv_);
            shift = (do_cmp((base as *mut u8).add(size) as *const c_void, (base as *mut u8).add(2 * size) as *const c_void, cmp_func, priv_) <= 0) as usize;
            a = size << shift; n -= size;
            do_swap((base as *mut u8).add(a) as *mut c_void, (base as *mut u8).add(n) as *mut c_void, size, swap_func, priv_);
        } else { break; }
        b = a;
        loop { c = 2 * b + size; d = c + size; if d >= n { break; } b = if do_cmp((base as *mut u8).add(c) as *const c_void, (base as *mut u8).add(d) as *const c_void, cmp_func, priv_) > 0 { c } else { d }; }
        if d == n { b = c; }
        while b != a && do_cmp((base as *mut u8).add(a) as *const c_void, (base as *mut u8).add(b) as *const c_void, cmp_func, priv_) >= 0 { b = parent(b, lsbit, size); }
        c = b;
        while b != a { b = parent(b, lsbit, size); do_swap((base as *mut u8).add(b) as *mut c_void, (base as *mut u8).add(c) as *mut c_void, size, swap_func, priv_); }
        if may_schedule { cond_resched(); }
    }
    n -= size; do_swap(base, (base as *mut u8).add(n) as *mut c_void, size, swap_func, priv_);
    if n == size * 2 && do_cmp(base, (base as *mut u8).add(size) as *const c_void, cmp_func, priv_) > 0 { do_swap(base, (base as *mut u8).add(size) as *mut c_void, size, swap_func, priv_); }
}

unsafe extern "C" { fn cond_resched(); }

pub unsafe extern "C" fn sort_r(base: *mut c_void, num: usize, size: usize, cmp_func: Option<CmpRFunc>, swap_func: SwapRFuncSelector, priv_: *const c_void) { __sort_r(base, num, size, cmp_func, swap_func, priv_, false); }
pub unsafe extern "C" fn sort_r_nonatomic(base: *mut c_void, num: usize, size: usize, cmp_func: Option<CmpRFunc>, swap_func: SwapRFuncSelector, priv_: *const c_void) { __sort_r(base, num, size, cmp_func, swap_func, priv_, true); }
pub unsafe extern "C" fn sort(base: *mut c_void, num: usize, size: usize, cmp_func: Option<CmpFunc>, swap_func: Option<SwapFunc>) { let w = Wrapper { cmp: cmp_func, swap: swap_func }; __sort_r(base, num, size, CMP_WRAPPER, SWAP_WRAPPER, &w as *const _ as *const c_void, false); }
pub unsafe extern "C" fn sort_nonatomic(base: *mut c_void, num: usize, size: usize, cmp_func: Option<CmpFunc>, swap_func: Option<SwapFunc>) { let w = Wrapper { cmp: cmp_func, swap: swap_func }; __sort_r(base, num, size, CMP_WRAPPER, SWAP_WRAPPER, &w as *const _ as *const c_void, true); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
