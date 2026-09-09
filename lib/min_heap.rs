// SPDX-License-Identifier: GPL-2.0

use core::ffi::c_void;
use core::mem::MaybeUninit;

// Types and inline helpers are supplied by the Linux min_heap dependency.
#[repr(C)]
pub struct min_heap_char {
    _private: [MaybeUninit<u8>; 0],
}

#[repr(C)]
pub struct min_heap_callbacks {
    _private: [MaybeUninit<u8>; 0],
}

unsafe extern "C" {
    fn __min_heap_init_inline(heap: *mut min_heap_char, data: *mut c_void, size: usize);
    fn __min_heap_peek_inline(heap: *mut min_heap_char) -> *mut c_void;
    fn __min_heap_full_inline(heap: *mut min_heap_char) -> bool;
    fn __min_heap_sift_down_inline(
        heap: *mut min_heap_char,
        pos: usize,
        elem_size: usize,
        func: *const min_heap_callbacks,
        args: *mut c_void,
    );
    fn __min_heap_sift_up_inline(
        heap: *mut min_heap_char,
        elem_size: usize,
        idx: usize,
        func: *const min_heap_callbacks,
        args: *mut c_void,
    );
    fn __min_heapify_all_inline(
        heap: *mut min_heap_char,
        elem_size: usize,
        func: *const min_heap_callbacks,
        args: *mut c_void,
    );
    fn __min_heap_pop_inline(
        heap: *mut min_heap_char,
        elem_size: usize,
        func: *const min_heap_callbacks,
        args: *mut c_void,
    ) -> bool;
    fn __min_heap_pop_push_inline(
        heap: *mut min_heap_char,
        element: *const c_void,
        elem_size: usize,
        func: *const min_heap_callbacks,
        args: *mut c_void,
    );
    fn __min_heap_push_inline(
        heap: *mut min_heap_char,
        element: *const c_void,
        elem_size: usize,
        func: *const min_heap_callbacks,
        args: *mut c_void,
    ) -> bool;
    fn __min_heap_del_inline(
        heap: *mut min_heap_char,
        elem_size: usize,
        idx: usize,
        func: *const min_heap_callbacks,
        args: *mut c_void,
    ) -> bool;
}

pub unsafe fn __min_heap_init(heap: *mut min_heap_char, data: *mut c_void, size: usize) {
    unsafe { __min_heap_init_inline(heap, data, size) };
}

pub unsafe fn __min_heap_peek(heap: *mut min_heap_char) -> *mut c_void {
    unsafe { __min_heap_peek_inline(heap) }
}

pub unsafe fn __min_heap_full(heap: *mut min_heap_char) -> bool {
    unsafe { __min_heap_full_inline(heap) }
}

pub unsafe fn __min_heap_sift_down(
    heap: *mut min_heap_char,
    pos: usize,
    elem_size: usize,
    func: *const min_heap_callbacks,
    args: *mut c_void,
) {
    unsafe { __min_heap_sift_down_inline(heap, pos, elem_size, func, args) };
}

pub unsafe fn __min_heap_sift_up(
    heap: *mut min_heap_char,
    elem_size: usize,
    idx: usize,
    func: *const min_heap_callbacks,
    args: *mut c_void,
) {
    unsafe { __min_heap_sift_up_inline(heap, elem_size, idx, func, args) };
}

pub unsafe fn __min_heapify_all(
    heap: *mut min_heap_char,
    elem_size: usize,
    func: *const min_heap_callbacks,
    args: *mut c_void,
) {
    unsafe { __min_heapify_all_inline(heap, elem_size, func, args) };
}

pub unsafe fn __min_heap_pop(
    heap: *mut min_heap_char,
    elem_size: usize,
    func: *const min_heap_callbacks,
    args: *mut c_void,
) -> bool {
    unsafe { __min_heap_pop_inline(heap, elem_size, func, args) }
}

pub unsafe fn __min_heap_pop_push(
    heap: *mut min_heap_char,
    element: *const c_void,
    elem_size: usize,
    func: *const min_heap_callbacks,
    args: *mut c_void,
) {
    unsafe { __min_heap_pop_push_inline(heap, element, elem_size, func, args) };
}

pub unsafe fn __min_heap_push(
    heap: *mut min_heap_char,
    element: *const c_void,
    elem_size: usize,
    func: *const min_heap_callbacks,
    args: *mut c_void,
) -> bool {
    unsafe { __min_heap_push_inline(heap, element, elem_size, func, args) }
}

pub unsafe fn __min_heap_del(
    heap: *mut min_heap_char,
    elem_size: usize,
    idx: usize,
    func: *const min_heap_callbacks,
    args: *mut c_void,
) -> bool {
    unsafe { __min_heap_del_inline(heap, elem_size, idx, func, args) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
