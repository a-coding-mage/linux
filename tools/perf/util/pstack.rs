// SPDX-License-Identifier: GPL-2.0
/*
 * Simple pointer stack
 *
 * (c) 2010 Arnaldo Carvalho de Melo <acme@redhat.com>
 */

// C dependencies: "pstack.h", "debug.h", <linux/kernel.h>,
// <linux/zalloc.h>, <stdlib.h>, <string.h>

use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct pstack {
    pub top: u16,
    pub max_nr_entries: u16,
    // Flexible array member in C:
    // void *entries[];
}

unsafe extern "C" {
    fn zalloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memmove(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn pr_err(fmt: *const c_char, ...);
}

unsafe fn pstack__entries(pstack: *mut pstack) -> *mut *mut c_void {
    unsafe { pstack.add(1) as *mut *mut c_void }
}

unsafe fn pstack__entries_const(pstack: *const pstack) -> *const *mut c_void {
    unsafe { pstack.add(1) as *const *mut c_void }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pstack__new(max_nr_entries: u16) -> *mut pstack {
    let pstack = unsafe {
        zalloc(
            core::mem::size_of::<pstack>()
                + (max_nr_entries as usize) * core::mem::size_of::<*mut c_void>(),
        ) as *mut pstack
    };

    if !pstack.is_null() {
        unsafe {
            (*pstack).max_nr_entries = max_nr_entries;
        }
    }

    pstack
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pstack__delete(pstack: *mut pstack) {
    unsafe {
        free(pstack as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pstack__empty(pstack: *const pstack) -> bool {
    unsafe { (*pstack).top == 0 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pstack__remove(pstack: *mut pstack, key: *mut c_void) {
    let mut i: u16 = unsafe { (*pstack).top };
    let last_index: u16 = unsafe { (*pstack).top.wrapping_sub(1) };

    while {
        let old_i = i;
        i = i.wrapping_sub(1);
        old_i != 0
    } {
        if unsafe { *pstack__entries(pstack).add(i as usize) == key } {
            if i < last_index {
                unsafe {
                    memmove(
                        pstack__entries(pstack).add(i as usize) as *mut c_void,
                        pstack__entries(pstack).add(i as usize + 1) as *const c_void,
                        ((last_index - i) as usize) * core::mem::size_of::<*mut c_void>(),
                    );
                }
            }
            unsafe {
                (*pstack).top = (*pstack).top.wrapping_sub(1);
            }
            return;
        }
    }

    unsafe {
        pr_err(
            c"%s: %p not on the pstack!\n".as_ptr(),
            c"pstack__remove".as_ptr(),
            key,
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pstack__push(pstack: *mut pstack, key: *mut c_void) {
    if unsafe { (*pstack).top == (*pstack).max_nr_entries } {
        unsafe {
            pr_err(
                c"%s: top=%d, overflow!\n".as_ptr(),
                c"pstack__push".as_ptr(),
                (*pstack).top as i32,
            );
        }
        return;
    }

    unsafe {
        let top = (*pstack).top;
        *pstack__entries(pstack).add(top as usize) = key;
        (*pstack).top = top.wrapping_add(1);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pstack__peek(pstack: *mut pstack) -> *mut c_void {
    if unsafe { (*pstack).top == 0 } {
        return core::ptr::null_mut();
    }

    unsafe { *pstack__entries_const(pstack).add(((*pstack).top - 1) as usize) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
