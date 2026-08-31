// SPDX-License-Identifier: GPL-2.0

use std::ffi::{c_char, c_int, c_void};

pub type size_t = usize;
pub type gfp_t = u32;

unsafe extern "C" {
    static __GFP_DIRECT_RECLAIM: gfp_t;
    static __GFP_ZERO: gfp_t;

    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn printf(format: *const c_char, ...) -> c_int;

    fn uatomic_inc(addr: *mut c_int);
    fn uatomic_dec(addr: *mut c_int);
}

#[unsafe(no_mangle)]
pub static mut kmalloc_nr_allocated: c_int = 0;

#[unsafe(no_mangle)]
pub static mut kmalloc_verbose: c_int = 0;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmalloc(size: size_t, gfp: gfp_t) -> *mut c_void {
    let ret: *mut c_void;

    if (gfp & __GFP_DIRECT_RECLAIM) == 0 {
        return std::ptr::null_mut();
    }

    ret = malloc(size);
    uatomic_inc(&raw mut kmalloc_nr_allocated);
    if kmalloc_verbose != 0 {
        printf(c"Allocating %p from malloc\n".as_ptr(), ret);
    }
    if (gfp & __GFP_ZERO) != 0 {
        memset(ret, 0, size);
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kfree(p: *mut c_void) {
    if p.is_null() {
        return;
    }
    uatomic_dec(&raw mut kmalloc_nr_allocated);
    if kmalloc_verbose != 0 {
        printf(c"Freeing %p to malloc\n".as_ptr(), p);
    }
    free(p);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn kmalloc_array(n: size_t, size: size_t, gfp: gfp_t) -> *mut c_void {
    let ret: *mut c_void;

    if (gfp & __GFP_DIRECT_RECLAIM) == 0 {
        return std::ptr::null_mut();
    }

    ret = calloc(n, size);
    uatomic_inc(&raw mut kmalloc_nr_allocated);
    if kmalloc_verbose != 0 {
        printf(c"Allocating %p from calloc\n".as_ptr(), ret);
    }
    if (gfp & __GFP_ZERO) != 0 {
        memset(ret, 0, n.wrapping_mul(size));
    }
    ret
}
