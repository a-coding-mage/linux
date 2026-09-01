// SPDX-License-Identifier: GPL-2.0
//
// Translated from lib/perf/threadmap.c. C include dependencies:
// <perf/threadmap.h>, <stdlib.h>, <linux/refcount.h>,
// <internal/threadmap.h>, <string.h>, <asm/bug.h>, <stdio.h>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

pub type pid_t = c_int;

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_thread_map_entry {
    pub pid: pid_t,
    pub comm: *mut c_char,
}

#[repr(C)]
pub struct perf_thread_map {
    pub refcnt: refcount_t,
    pub nr: c_int,
    pub err_thread: c_int,
    pub map: [perf_thread_map_entry; 0],
}

unsafe extern "C" {
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;

    fn refcount_set(r: *mut refcount_t, n: c_int);
    fn refcount_read(r: *const refcount_t) -> c_int;
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;

    fn WARN_ONCE(condition: bool, format: *const c_char, ...) -> c_int;
}

unsafe fn perf_thread_map__entry(
    map: *mut perf_thread_map,
    idx: c_int,
) -> *mut perf_thread_map_entry {
    unsafe { (*map).map.as_mut_ptr().offset(idx as isize) }
}

unsafe fn perf_thread_map__reset(map: *mut perf_thread_map, start: c_int, nr: c_int) {
    let size: usize =
        ((nr - start) as usize).wrapping_mul(mem::size_of::<perf_thread_map_entry>());

    unsafe {
        memset(
            perf_thread_map__entry(map, start) as *mut c_void,
            0,
            size,
        );
        (*map).err_thread = -1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_thread_map__realloc(
    mut map: *mut perf_thread_map,
    nr: c_int,
) -> *mut perf_thread_map {
    let size: usize = mem::size_of::<perf_thread_map>()
        .wrapping_add(mem::size_of::<perf_thread_map_entry>().wrapping_mul(nr as usize));
    let start: c_int = if !map.is_null() {
        unsafe { (*map).nr }
    } else {
        0
    };

    unsafe {
        map = realloc(map as *mut c_void, size) as *mut perf_thread_map;
    }
    /*
     * We only realloc to add more items, let's reset new items.
     */
    if !map.is_null() {
        unsafe {
            perf_thread_map__reset(map, start, nr);
        }
    }

    map
}

unsafe fn thread_map__alloc(__nr: c_int) -> *mut perf_thread_map {
    unsafe { perf_thread_map__realloc(ptr::null_mut(), __nr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_thread_map__set_pid(
    map: *mut perf_thread_map,
    idx: c_int,
    pid: pid_t,
) {
    unsafe {
        (*perf_thread_map__entry(map, idx)).pid = pid;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_thread_map__comm(
    map: *mut perf_thread_map,
    idx: c_int,
) -> *mut c_char {
    unsafe { (*perf_thread_map__entry(map, idx)).comm }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_thread_map__new_array(
    nr_threads: c_int,
    array: *mut pid_t,
) -> *mut perf_thread_map {
    let threads: *mut perf_thread_map = unsafe { thread_map__alloc(nr_threads) };
    let mut i: c_int;

    if threads.is_null() {
        return ptr::null_mut();
    }

    i = 0;
    while i < nr_threads {
        unsafe {
            perf_thread_map__set_pid(
                threads,
                i,
                if !array.is_null() {
                    *array.offset(i as isize)
                } else {
                    -1
                },
            );
        }
        i += 1;
    }

    unsafe {
        (*threads).nr = nr_threads;
        refcount_set(&mut (*threads).refcnt, 1);
    }

    threads
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_thread_map__new_dummy() -> *mut perf_thread_map {
    unsafe { perf_thread_map__new_array(1, ptr::null_mut()) }
}

unsafe fn perf_thread_map__delete(threads: *mut perf_thread_map) {
    if !threads.is_null() {
        let mut i: c_int;

        unsafe {
            WARN_ONCE(
                refcount_read(&(*threads).refcnt) != 0,
                c"thread map refcnt unbalanced\n".as_ptr(),
            );
        }
        i = 0;
        while unsafe { i < (*threads).nr } {
            unsafe {
                free(perf_thread_map__comm(threads, i) as *mut c_void);
            }
            i += 1;
        }
        unsafe {
            free(threads as *mut c_void);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_thread_map__get(
    map: *mut perf_thread_map,
) -> *mut perf_thread_map {
    if !map.is_null() {
        unsafe {
            refcount_inc(&mut (*map).refcnt);
        }
    }
    map
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_thread_map__put(map: *mut perf_thread_map) {
    if !map.is_null() && unsafe { refcount_dec_and_test(&mut (*map).refcnt) } {
        unsafe {
            perf_thread_map__delete(map);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_thread_map__nr(threads: *mut perf_thread_map) -> c_int {
    if !threads.is_null() {
        unsafe { (*threads).nr }
    } else {
        1
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_thread_map__pid(
    map: *mut perf_thread_map,
    idx: c_int,
) -> pid_t {
    if map.is_null() {
        assert!(idx == 0);
        return -1;
    }

    unsafe { (*perf_thread_map__entry(map, idx)).pid }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_thread_map__idx(
    threads: *mut perf_thread_map,
    pid: pid_t,
) -> c_int {
    if threads.is_null() {
        return if pid == -1 { 0 } else { -1 };
    }

    let mut i: c_int = 0;
    while unsafe { i < (*threads).nr } {
        if unsafe { (*perf_thread_map__entry(threads, i)).pid == pid } {
            return i;
        }
        i += 1;
    }
    -1
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
