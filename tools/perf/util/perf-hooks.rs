// SPDX-License-Identifier: GPL-2.0
/*
 * perf_hooks.c
 *
 * Copyright (C) 2016 Wang Nan <wangnan0@huawei.com>
 * Copyright (C) 2016 Huawei Inc.
 */

// C dependencies removed from executable Rust:
// <errno.h>, <stdlib.h>, <string.h>, <setjmp.h>, <linux/err.h>,
// <linux/kernel.h>, "util/debug.h", "util/perf-hooks.h".

use core::ffi::{c_char, c_int, c_void};

pub type perf_hook_func_t = Option<unsafe extern "C" fn(*mut c_void)>;

#[repr(C)]
pub struct perf_hook_desc {
    pub hook_name: *const c_char,
    pub p_hook_func: *mut perf_hook_func_t,
    pub hook_ctx: *mut c_void,
}

#[allow(non_camel_case_types)]
type sigjmp_buf = [c_int; 1];

const ENOENT: c_int = 2;

unsafe extern "C" {
    fn sigsetjmp(env: *mut sigjmp_buf, savesigs: c_int) -> c_int;
    fn siglongjmp(env: *mut sigjmp_buf, val: c_int) -> !;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn pr_warning(fmt: *const c_char, ...);
    fn ERR_PTR(error: isize) -> perf_hook_func_t;
}

static mut jmpbuf: sigjmp_buf = [0; 1];
static mut current_perf_hook: *const perf_hook_desc = core::ptr::null();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_hooks__invoke(desc: *const perf_hook_desc) {
    if !(unsafe {
        !desc.is_null()
            && !(*desc).p_hook_func.is_null()
            && (*(*desc).p_hook_func).is_some()
    }) {
        return;
    }

    if unsafe { sigsetjmp(&raw mut jmpbuf, 1) } != 0 {
        unsafe {
            pr_warning(
                c"Fatal error (SEGFAULT) in perf hook '%s'\n".as_ptr(),
                (*desc).hook_name,
            );
            *(*current_perf_hook).p_hook_func = None;
        }
    } else {
        unsafe {
            current_perf_hook = desc;
            ((*(*desc).p_hook_func).unwrap())((*desc).hook_ctx);
        }
    }
    unsafe {
        current_perf_hook = core::ptr::null();
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_hooks__recover() {
    if unsafe { !current_perf_hook.is_null() } {
        unsafe {
            siglongjmp(&raw mut jmpbuf, 1);
        }
    }
}

/*
 * C source:
 *
 * #define PERF_HOOK(name)                                 \
 * perf_hook_func_t __perf_hook_func_##name = NULL;        \
 * struct perf_hook_desc __perf_hook_desc_##name =         \
 *      {.hook_name = #name,                               \
 *       .p_hook_func = &__perf_hook_func_##name,          \
 *       .hook_ctx = NULL};
 * #include "perf-hooks-list.h"
 * #undef PERF_HOOK
 *
 * The isolated file does not contain perf-hooks-list.h, so the macro-generated
 * definitions are preserved as dependency intent rather than expanded here.
 */

/*
 * C source:
 *
 * #define PERF_HOOK(name)         \
 *      &__perf_hook_desc_##name,
 *
 * static struct perf_hook_desc *perf_hooks[] = {
 * #include "perf-hooks-list.h"
 * };
 * #undef PERF_HOOK
 *
 * The concrete entries are supplied by perf-hooks-list.h in the original build.
 */
static mut perf_hooks: [*mut perf_hook_desc; 0] = [];

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_hooks__set_hook(
    hook_name: *const c_char,
    hook_func: perf_hook_func_t,
    hook_ctx: *mut c_void,
) -> c_int {
    let mut i: u32;

    i = 0;
    while (i as usize) < unsafe { perf_hooks.len() } {
        if unsafe { strcmp(hook_name, (*perf_hooks[i as usize]).hook_name) } != 0 {
            i = i.wrapping_add(1);
            continue;
        }

        if unsafe { (*(*perf_hooks[i as usize]).p_hook_func).is_some() } {
            unsafe {
                pr_warning(c"Overwrite existing hook: %s\n".as_ptr(), hook_name);
            }
        }
        unsafe {
            *(*perf_hooks[i as usize]).p_hook_func = hook_func;
            (*perf_hooks[i as usize]).hook_ctx = hook_ctx;
        }
        return 0;
    }
    -ENOENT
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn perf_hooks__get_hook(hook_name: *const c_char) -> perf_hook_func_t {
    let mut i: u32;

    i = 0;
    while (i as usize) < unsafe { perf_hooks.len() } {
        if unsafe { strcmp(hook_name, (*perf_hooks[i as usize]).hook_name) } != 0 {
            i = i.wrapping_add(1);
            continue;
        }

        return unsafe { *(*perf_hooks[i as usize]).p_hook_func };
    }
    unsafe { ERR_PTR(-ENOENT as isize) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
