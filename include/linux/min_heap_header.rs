/* SPDX-License-Identifier: GPL-2.0 */

/* Translated from linux/min_heap.h. */

#[repr(C)]
pub struct MinHeap<T, const N: usize> {
    pub nr: usize,
    pub size: usize,
    pub data: *mut T,
    pub preallocated: [T; N],
}

pub type MinHeapChar = MinHeap<u8, 0>;

#[repr(C)]
pub struct MinHeapCallbacks {
    pub less: Option<unsafe extern "C" fn(*const core::ffi::c_void, *const core::ffi::c_void, *mut core::ffi::c_void) -> bool>,
    pub swp: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void)>,
}

#[inline(always)]
pub unsafe fn is_aligned(base: *const core::ffi::c_void, size: usize, align: u8) -> bool {
    let mut lsbits = size as u8;
    // CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS controls this C conditional.
    lsbits |= base as usize as u8;
    (lsbits & (align.wrapping_sub(1))) == 0
}

#[inline(always)]
pub unsafe fn swap_words_32(mut a: *mut u8, mut b: *mut u8, mut n: usize) {
    loop {
        n = n.wrapping_sub(4);
        let t = (a.add(n) as *mut u32).read();
        (a.add(n) as *mut u32).write((b.add(n) as *mut u32).read());
        (b.add(n) as *mut u32).write(t);
        if n == 0 { break; }
    }
}

#[inline(always)]
pub unsafe fn swap_words_64(mut a: *mut u8, mut b: *mut u8, mut n: usize) {
    loop {
        n = n.wrapping_sub(8);
        let t = (a.add(n) as *mut u64).read();
        (a.add(n) as *mut u64).write((b.add(n) as *mut u64).read());
        (b.add(n) as *mut u64).write(t);
        if n == 0 { break; }
    }
}

#[inline(always)]
pub unsafe fn swap_bytes(mut a: *mut u8, mut b: *mut u8, mut n: usize) {
    loop {
        n = n.wrapping_sub(1);
        let t = a.add(n).read();
        a.add(n).write(b.add(n).read());
        b.add(n).write(t);
        if n == 0 { break; }
    }
}

pub const SWAP_WORDS_64: usize = 0;
pub const SWAP_WORDS_32: usize = 1;
pub const SWAP_BYTES: usize = 2;

#[inline(always)]
pub unsafe fn select_swap_func(base: *const core::ffi::c_void, size: usize) -> usize {
    if is_aligned(base, size, 8) { SWAP_WORDS_64 }
    else if is_aligned(base, size, 4) { SWAP_WORDS_32 }
    else { SWAP_BYTES }
}

#[inline(always)]
pub unsafe fn do_swap(a: *mut u8, b: *mut u8, size: usize,
    swap_func: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void)>,
    priv_: *mut core::ffi::c_void) {
    match swap_func.map(|_| 3) {
        Some(_) => swap_func.unwrap()(a as *mut _, b as *mut _, priv_),
        None => swap_bytes(a, b, size),
    }
}

#[inline(always)]
pub const fn parent(mut i: usize, lsbit: u32, size: usize) -> usize {
    i = i.wrapping_sub(size);
    i = i.wrapping_sub(size & (0usize.wrapping_sub(i & lsbit as usize)));
    i / 2
}

pub unsafe fn __min_heap_init_inline(heap: *mut MinHeapChar, data: *mut u8, size: usize) {
    (*heap).nr = 0;
    (*heap).size = size;
    (*heap).data = if !data.is_null() { data } else { (*heap).preallocated.as_mut_ptr() };
}

pub unsafe fn __min_heap_peek_inline(heap: *mut MinHeapChar) -> *mut u8 {
    if (*heap).nr != 0 { (*heap).data } else { core::ptr::null_mut() }
}

pub unsafe fn __min_heap_full_inline(heap: *mut MinHeapChar) -> bool { (*heap).nr == (*heap).size }

#[inline(always)]
pub unsafe fn __min_heap_sift_down_inline(heap: *mut MinHeapChar, pos: usize, elem_size: usize,
    func: *const MinHeapCallbacks, args: *mut core::ffi::c_void) {
    let data = (*heap).data;
    let less = (*func).less.unwrap();
    let mut a = pos * elem_size;
    let mut b;
    let mut c;
    let mut d;
    let n = (*heap).nr * elem_size;
    c = 0; d = 0;
    while { c = 2 * a + elem_size; d = c + elem_size; d < n } {
        b = if less(data.add(c) as *const _, data.add(d) as *const _, args) { c } else { d };
        a = b;
    }
    b = a;
    if d == n { b = c; }
    while b != pos * elem_size && less(data.add(pos * elem_size) as *const _, data.add(b) as *const _, args) {
        b = parent(b, (elem_size & elem_size.wrapping_neg()) as u32, elem_size);
    }
    c = b;
    while b != pos * elem_size {
        b = parent(b, (elem_size & elem_size.wrapping_neg()) as u32, elem_size);
        do_swap(data.add(b), data.add(c), elem_size, (*func).swp, args);
    }
}

#[inline(always)]
pub unsafe fn __min_heap_sift_up_inline(heap: *mut MinHeapChar, elem_size: usize, idx: usize,
    func: *const MinHeapCallbacks, args: *mut core::ffi::c_void) {
    let data = (*heap).data;
    let less = (*func).less.unwrap();
    let mut a = idx * elem_size;
    while a != 0 {
        let b = parent(a, (elem_size & elem_size.wrapping_neg()) as u32, elem_size);
        if less(data.add(b) as *const _, data.add(a) as *const _, args) { break; }
        do_swap(data.add(a), data.add(b), elem_size, (*func).swp, args);
        a = b;
    }
}

pub unsafe fn __min_heapify_all_inline(heap: *mut MinHeapChar, elem_size: usize, func: *const MinHeapCallbacks, args: *mut core::ffi::c_void) {
    let mut i = ((*heap).nr / 2) as isize - 1;
    while i >= 0 { __min_heap_sift_down_inline(heap, i as usize, elem_size, func, args); i -= 1; }
}

pub unsafe fn __min_heap_pop_inline(heap: *mut MinHeapChar, elem_size: usize, func: *const MinHeapCallbacks, args: *mut core::ffi::c_void) -> bool {
    if (*heap).nr == 0 { return false; }
    (*heap).nr -= 1;
    core::ptr::copy((*heap).data.add((*heap).nr * elem_size), (*heap).data, elem_size);
    __min_heap_sift_down_inline(heap, 0, elem_size, func, args);
    true
}

pub unsafe fn __min_heap_pop_push_inline(heap: *mut MinHeapChar, element: *const core::ffi::c_void, elem_size: usize, func: *const MinHeapCallbacks, args: *mut core::ffi::c_void) {
    core::ptr::copy_nonoverlapping(element as *const u8, (*heap).data, elem_size);
    __min_heap_sift_down_inline(heap, 0, elem_size, func, args);
}

pub unsafe fn __min_heap_push_inline(heap: *mut MinHeapChar, element: *const core::ffi::c_void, elem_size: usize, func: *const MinHeapCallbacks, args: *mut core::ffi::c_void) -> bool {
    if (*heap).nr >= (*heap).size { return false; }
    let pos = (*heap).nr;
    core::ptr::copy_nonoverlapping(element as *const u8, (*heap).data.add(pos * elem_size), elem_size);
    (*heap).nr += 1;
    __min_heap_sift_up_inline(heap, elem_size, pos, func, args);
    true
}

pub unsafe fn __min_heap_del_inline(heap: *mut MinHeapChar, elem_size: usize, idx: usize, func: *const MinHeapCallbacks, args: *mut core::ffi::c_void) -> bool {
    if (*heap).nr == 0 { return false; }
    (*heap).nr -= 1;
    if idx == (*heap).nr { return true; }
    do_swap((*heap).data.add(idx * elem_size), (*heap).data.add((*heap).nr * elem_size), elem_size, (*func).swp, args);
    __min_heap_sift_up_inline(heap, elem_size, idx, func, args);
    __min_heap_sift_down_inline(heap, idx, elem_size, func, args);
    true
}

extern "C" {
    pub fn __min_heap_init(heap: *mut MinHeapChar, data: *mut u8, size: usize);
    pub fn __min_heap_peek(heap: *mut MinHeapChar) -> *mut core::ffi::c_void;
    pub fn __min_heap_full(heap: *mut MinHeapChar) -> bool;
    pub fn __min_heap_sift_down(heap: *mut MinHeapChar, pos: usize, elem_size: usize, func: *const MinHeapCallbacks, args: *mut core::ffi::c_void);
    pub fn __min_heap_sift_up(heap: *mut MinHeapChar, elem_size: usize, idx: usize, func: *const MinHeapCallbacks, args: *mut core::ffi::c_void);
    pub fn __min_heapify_all(heap: *mut MinHeapChar, elem_size: usize, func: *const MinHeapCallbacks, args: *mut core::ffi::c_void);
    pub fn __min_heap_pop(heap: *mut MinHeapChar, elem_size: usize, func: *const MinHeapCallbacks, args: *mut core::ffi::c_void) -> bool;
    pub fn __min_heap_pop_push(heap: *mut MinHeapChar, element: *const core::ffi::c_void, elem_size: usize, func: *const MinHeapCallbacks, args: *mut core::ffi::c_void);
    pub fn __min_heap_push(heap: *mut MinHeapChar, element: *const core::ffi::c_void, elem_size: usize, func: *const MinHeapCallbacks, args: *mut core::ffi::c_void) -> bool;
    pub fn __min_heap_del(heap: *mut MinHeapChar, elem_size: usize, idx: usize, func: *const MinHeapCallbacks, args: *mut core::ffi::c_void) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
