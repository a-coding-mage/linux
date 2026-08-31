// SPDX-License-Identifier: GPL-2.0
/*
 * fill_buf benchmark
 *
 * Copyright (C) 2018 Intel Corporation
 *
 * Authors:
 *    Sai Praneeth Prakhya <sai.praneeth.prakhya@intel.com>,
 *    Fenghua Yu <fenghua.yu@intel.com>
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_void};

// Dependencies supplied by resctrl.h / libc in the original C translation unit.
unsafe extern "C" {
    static mut value_sink: *mut c_int;
    static MINIMUM_SPAN: isize;

    fn get_cache_size(cpu_no: c_int, cache_type: *const c_char, cache_total_size: *mut c_ulong)
        -> c_int;
    fn posix_memalign(memptr: *mut *mut c_void, alignment: usize, size: usize) -> c_int;
    fn rand() -> c_int;
}

const CL_SIZE: usize = 64;
const PAGE_SIZE: usize = 4 * 1024;
const MB: usize = 1024 * 1024;

unsafe fn sb() {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        asm!("sfence", options(nostack, preserves_flags));
    }
}

unsafe fn cl_flush(p: *mut c_void) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        asm!("clflush ({0})", in(reg) p, options(nostack, preserves_flags));
    }

    #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
    {
        let _ = p;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mem_flush(buf: *mut u8, mut buf_size: usize) {
    let cp = buf;
    let mut i: usize = 0;

    buf_size = buf_size / CL_SIZE; /* mem size in cache lines */

    while i < buf_size {
        unsafe {
            cl_flush(cp.add(i * CL_SIZE) as *mut c_void);
        }
        i += 1;
    }

    unsafe {
        sb();
    }
}

/*
 * Buffer index step advance to workaround HW prefetching interfering with
 * the measurements.
 *
 * Must be a prime to step through all indexes of the buffer.
 *
 * Some primes work better than others on some architectures (from MBA/MBM
 * result stability point of view).
 */
const FILL_IDX_MULT: u32 = 23;

unsafe fn fill_one_span_read(buf: *mut u8, buf_size: usize) -> c_int {
    let size: u32 = (buf_size / (CL_SIZE / 2)) as u32;
    let mut i: u32;
    let mut idx: u32 = 0;
    let mut sum: u8 = 0;

    /*
     * Read the buffer in an order that is unexpected by HW prefetching
     * optimizations to prevent them interfering with the caching pattern.
     *
     * The read order is (in terms of halves of cachelines):
     *	i * FILL_IDX_MULT % size
     * The formula is open-coded below to avoiding modulo inside the loop
     * as it improves MBA/MBM result stability on some architectures.
     */
    i = 0;
    while i < size {
        unsafe {
            sum = sum.wrapping_add(*buf.add((idx as usize) * (CL_SIZE / 2)));
        }

        idx = idx.wrapping_add(FILL_IDX_MULT);
        while idx >= size {
            idx -= size;
        }

        i += 1;
    }

    sum as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn fill_cache_read(buf: *mut u8, buf_size: usize, once: bool) {
    let mut ret: c_int;

    loop {
        ret = unsafe { fill_one_span_read(buf, buf_size) };
        if once {
            break;
        }
    }

    /* Consume read result so that reading memory is not optimized out. */
    unsafe {
        *value_sink = ret;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn alloc_buffer(buf_size: usize, memflush: bool) -> *mut u8 {
    let mut buf: *mut c_void = core::ptr::null_mut();
    let mut p64: *mut u64;
    let mut s64: isize;
    let ret: c_int;

    ret = unsafe { posix_memalign(&mut buf, PAGE_SIZE, buf_size) };
    if ret < 0 {
        return core::ptr::null_mut();
    }

    /* Initialize the buffer */
    p64 = buf as *mut u64;
    s64 = (buf_size / core::mem::size_of::<u64>()) as isize;

    while s64 > 0 {
        unsafe {
            *p64 = rand() as u64;
            p64 = p64.add(CL_SIZE / core::mem::size_of::<u64>());
        }
        s64 -= (CL_SIZE / core::mem::size_of::<u64>()) as isize;
    }

    /* Flush the memory before using to avoid "cache hot pages" effect */
    if memflush {
        unsafe {
            mem_flush(buf as *mut u8, buf_size);
        }
    }

    buf as *mut u8
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_fill_buf_size(cpu_no: c_int, cache_type: *const c_char) -> isize {
    let mut cache_total_size: c_ulong = 0;
    let ret: c_int;

    ret = unsafe { get_cache_size(cpu_no, cache_type, &mut cache_total_size) };
    if ret != 0 {
        return ret as isize;
    }

    unsafe {
        if (cache_total_size as isize) * 4 > MINIMUM_SPAN {
            (cache_total_size as isize) * 4
        } else {
            MINIMUM_SPAN
        }
    }
}
