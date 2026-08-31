// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as external Rust references:
// "util/debug.h", "util/event.h", <subcmd/parse-options.h>,
// "util/parse-branch-options.h", <stdlib.h>, <string.h>

use core::ffi::{c_char, c_int, c_void};

type __u64 = u64;

#[repr(C)]
pub struct option {
    pub value: *mut c_void,
}

#[repr(C)]
struct branch_mode {
    name: *const c_char,
    mode: c_int,
}

extern "C" {
    static PERF_SAMPLE_BRANCH_USER: __u64;
    static PERF_SAMPLE_BRANCH_KERNEL: __u64;
    static PERF_SAMPLE_BRANCH_HV: __u64;
    static PERF_SAMPLE_BRANCH_ANY: __u64;
    static PERF_SAMPLE_BRANCH_ANY_CALL: __u64;
    static PERF_SAMPLE_BRANCH_ANY_RETURN: __u64;
    static PERF_SAMPLE_BRANCH_IND_CALL: __u64;
    static PERF_SAMPLE_BRANCH_ABORT_TX: __u64;
    static PERF_SAMPLE_BRANCH_IN_TX: __u64;
    static PERF_SAMPLE_BRANCH_NO_TX: __u64;
    static PERF_SAMPLE_BRANCH_COND: __u64;
    static PERF_SAMPLE_BRANCH_IND_JUMP: __u64;
    static PERF_SAMPLE_BRANCH_CALL: __u64;
    static PERF_SAMPLE_BRANCH_NO_FLAGS: __u64;
    static PERF_SAMPLE_BRANCH_NO_CYCLES: __u64;
    static PERF_SAMPLE_BRANCH_TYPE_SAVE: __u64;
    static PERF_SAMPLE_BRANCH_CALL_STACK: __u64;
    static PERF_SAMPLE_BRANCH_HW_INDEX: __u64;
    static PERF_SAMPLE_BRANCH_PRIV_SAVE: __u64;
    static PERF_SAMPLE_BRANCH_COUNTERS: __u64;

    fn strdup(s: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn free(ptr: *mut c_void);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

macro_rules! branch_opt {
    ($n:expr, $m:expr) => {
        branch_mode {
            name: $n.as_ptr() as *const c_char,
            mode: $m as c_int,
        }
    };
}

macro_rules! branch_end {
    () => {
        branch_mode {
            name: core::ptr::null(),
            mode: 0,
        }
    };
}

unsafe fn branch_modes() -> [branch_mode; 21] {
    [
        branch_opt!(b"u\0", PERF_SAMPLE_BRANCH_USER),
        branch_opt!(b"k\0", PERF_SAMPLE_BRANCH_KERNEL),
        branch_opt!(b"hv\0", PERF_SAMPLE_BRANCH_HV),
        branch_opt!(b"any\0", PERF_SAMPLE_BRANCH_ANY),
        branch_opt!(b"any_call\0", PERF_SAMPLE_BRANCH_ANY_CALL),
        branch_opt!(b"any_ret\0", PERF_SAMPLE_BRANCH_ANY_RETURN),
        branch_opt!(b"ind_call\0", PERF_SAMPLE_BRANCH_IND_CALL),
        branch_opt!(b"abort_tx\0", PERF_SAMPLE_BRANCH_ABORT_TX),
        branch_opt!(b"in_tx\0", PERF_SAMPLE_BRANCH_IN_TX),
        branch_opt!(b"no_tx\0", PERF_SAMPLE_BRANCH_NO_TX),
        branch_opt!(b"cond\0", PERF_SAMPLE_BRANCH_COND),
        branch_opt!(b"ind_jmp\0", PERF_SAMPLE_BRANCH_IND_JUMP),
        branch_opt!(b"call\0", PERF_SAMPLE_BRANCH_CALL),
        branch_opt!(b"no_flags\0", PERF_SAMPLE_BRANCH_NO_FLAGS),
        branch_opt!(b"no_cycles\0", PERF_SAMPLE_BRANCH_NO_CYCLES),
        branch_opt!(b"save_type\0", PERF_SAMPLE_BRANCH_TYPE_SAVE),
        branch_opt!(b"stack\0", PERF_SAMPLE_BRANCH_CALL_STACK),
        branch_opt!(b"hw_index\0", PERF_SAMPLE_BRANCH_HW_INDEX),
        branch_opt!(b"priv\0", PERF_SAMPLE_BRANCH_PRIV_SAVE),
        branch_opt!(b"counter\0", PERF_SAMPLE_BRANCH_COUNTERS),
        branch_end!(),
    ]
}

pub unsafe extern "C" fn parse_branch_str(str_: *const c_char, mode: *mut __u64) -> c_int {
    let only_plm: __u64 =
        PERF_SAMPLE_BRANCH_USER | PERF_SAMPLE_BRANCH_KERNEL | PERF_SAMPLE_BRANCH_HV;

    let mut ret: c_int = 0;
    let mut p: *mut c_char;
    let mut s: *mut c_char;
    let os: *mut c_char;
    let mut br: *const branch_mode;

    if str_.is_null() {
        *mode = PERF_SAMPLE_BRANCH_ANY;
        return 0;
    }

    /* because str is read-only */
    os = strdup(str_);
    s = os;
    if s.is_null() {
        return -1;
    }

    loop {
        p = strchr(s, ',' as c_int);
        if !p.is_null() {
            *p = '\0' as c_char;
        }

        let branch_modes = branch_modes();
        br = branch_modes.as_ptr();
        while !(*br).name.is_null() {
            if strcasecmp(s, (*br).name) == 0 {
                break;
            }
            br = br.add(1);
        }
        if (*br).name.is_null() {
            ret = -1;
            pr_warning(
                b"unknown branch filter %s, check man page\n\0".as_ptr() as *const c_char,
                s,
            );
            return goto_error(os, ret);
        }

        *mode |= (*br).mode as __u64;

        if p.is_null() {
            break;
        }

        s = p.add(1);
    }

    /* default to any branch */
    if (*mode & !only_plm) == 0 {
        *mode = PERF_SAMPLE_BRANCH_ANY;
    }

    free(os as *mut c_void);
    ret
}

unsafe fn goto_error(os: *mut c_char, ret: c_int) -> c_int {
    free(os as *mut c_void);
    ret
}

pub unsafe extern "C" fn parse_branch_stack(
    opt: *const option,
    str_: *const c_char,
    unset: c_int,
) -> c_int {
    let mode: *mut __u64 = (*opt).value as *mut __u64;

    if unset != 0 {
        return 0;
    }

    /*
     * cannot set it twice, -b + --branch-filter for instance
     */
    if *mode != 0 {
        pr_err(
            b"Error: Can't use --branch-any (-b) with --branch-filter (-j).\n\0".as_ptr()
                as *const c_char,
        );
        return -1;
    }

    parse_branch_str(str_, mode)
}
