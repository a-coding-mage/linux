// SPDX-License-Identifier: GPL-2.0
// Translated from perf/util/help-unknown-cmd.c.
// Original C dependencies: cache.h, config.h, poll.h, stdio.h, stdlib.h,
// subcmd/help.h, ../builtin.h, levenshtein.h, linux/zalloc.h.

use core::ffi::{c_char, c_int, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct cmdname {
    pub len: c_int,
    pub name: *const c_char,
}

#[repr(C)]
pub struct cmdnames {
    pub cnt: u32,
    pub alloc: u32,
    pub names: *mut *mut cmdname,
}

unsafe extern "C" {
    static mut stderr: *mut c_void;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn qsort(
        base: *mut c_void,
        nmemb: usize,
        size: usize,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    );
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn poll(fds: *mut c_void, nfds: usize, timeout: c_int) -> c_int;

    fn perf_config_int(dest: *mut c_int, var: *const c_char, value: *const c_char) -> c_int;
    fn perf_config(
        fn_: Option<unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;
    fn load_command_list(
        prefix: *const c_char,
        main_cmds: *mut cmdnames,
        other_cmds: *mut cmdnames,
    );
    fn cmdname_compare(p1: *const c_void, p2: *const c_void) -> c_int;
    fn uniq(cmds: *mut cmdnames);
    fn levenshtein(
        string1: *const c_char,
        string2: *const c_char,
        swap_penalty: c_int,
        substition_penality: c_int,
        insertion_penality: c_int,
        deletion_penalty: c_int,
    ) -> c_int;
    fn clean_cmdnames(cmds: *mut cmdnames);
    fn zfree(ptr: *mut *mut c_void);

    // External equivalent of the alloc_nr() helper macro used by the C source.
    fn alloc_nr(x: u32) -> u32;
}

static mut autocorrect: c_int = 0;

unsafe extern "C" fn perf_unknown_cmd_config(
    var: *const c_char,
    value: *const c_char,
    cb: *mut c_void,
) -> c_int {
    let _ = cb;

    if strcmp(var, c"help.autocorrect".as_ptr()) == 0 {
        return perf_config_int(&raw mut autocorrect, var, value);
    }

    0
}

unsafe extern "C" fn levenshtein_compare(p1: *const c_void, p2: *const c_void) -> c_int {
    let c1 = p1 as *const *const cmdname;
    let c2 = p2 as *const *const cmdname;
    let s1 = (**c1).name;
    let s2 = (**c2).name;
    let l1 = (**c1).len;
    let l2 = (**c2).len;

    if l1 != l2 {
        l1 - l2
    } else {
        strcmp(s1, s2)
    }
}

unsafe fn add_cmd_list(cmds: *mut cmdnames, old: *mut cmdnames) -> c_int {
    let mut i: u32;
    let nr: u32 = (*cmds).cnt + (*old).cnt;
    let tmp: *mut c_void;

    if nr > (*cmds).alloc {
        /* Choose bigger one to alloc */
        if alloc_nr((*cmds).alloc) < nr {
            (*cmds).alloc = nr;
        } else {
            (*cmds).alloc = alloc_nr((*cmds).alloc);
        }

        tmp = realloc(
            (*cmds).names as *mut c_void,
            ((*cmds).alloc as usize) * size_of::<*mut cmdname>(),
        );
        if tmp.is_null() {
            return -1;
        }
        (*cmds).names = tmp as *mut *mut cmdname;
    }

    i = 0;
    while i < (*old).cnt {
        *(*cmds).names.add((*cmds).cnt as usize) = *(*old).names.add(i as usize);
        (*cmds).cnt += 1;
        i += 1;
    }

    zfree(&raw mut (*old).names as *mut *mut cmdname as *mut *mut c_void);
    (*old).cnt = 0;
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn help_unknown_cmd(
    cmd: *const c_char,
    main_cmds: *mut cmdnames,
) -> *const c_char {
    let mut i: u32;
    let mut n: u32 = 0;
    let mut best_similarity: u32 = 0;
    let mut other_cmds: cmdnames = cmdnames {
        cnt: 0,
        alloc: 0,
        names: ptr::null_mut(),
    };

    memset(
        &mut other_cmds as *mut cmdnames as *mut c_void,
        0,
        size_of::<cmdnames>(),
    );

    perf_config(Some(perf_unknown_cmd_config), ptr::null_mut());

    load_command_list(c"perf-".as_ptr(), main_cmds, &mut other_cmds);

    if add_cmd_list(main_cmds, &mut other_cmds) < 0 {
        fprintf(
            stderr,
            c"ERROR: Failed to allocate command list for unknown command.\n".as_ptr(),
        );
        clean_cmdnames(&mut other_cmds);
        return ptr::null();
    }

    qsort(
        (*main_cmds).names as *mut c_void,
        (*main_cmds).cnt as usize,
        size_of::<*mut *mut cmdname>(),
        Some(cmdname_compare),
    );
    uniq(main_cmds);

    if (*main_cmds).cnt != 0 {
        /* This reuses cmdname->len for similarity index */
        i = 0;
        while i < (*main_cmds).cnt {
            (**(*main_cmds).names.add(i as usize)).len = levenshtein(
                cmd,
                (**(*main_cmds).names.add(i as usize)).name,
                0, /*swap_penalty=*/
                2, /*substition_penality=*/
                1, /*insertion_penality=*/
                1, /*deletion_penalty=*/
            );
            i += 1;
        }

        qsort(
            (*main_cmds).names as *mut c_void,
            (*main_cmds).cnt as usize,
            size_of::<*mut cmdname>(),
            Some(levenshtein_compare),
        );

        best_similarity = (**(*main_cmds).names.add(0)).len as u32;
        n = 1;
        while n < (*main_cmds).cnt && best_similarity == (**(*main_cmds).names.add(n as usize)).len as u32 {
            n += 1;
        }
    }

    if autocorrect != 0 && n == 1 {
        let assumed: *const c_char = (**(*main_cmds).names.add(0)).name;

        *(*main_cmds).names.add(0) = ptr::null_mut();
        clean_cmdnames(&mut other_cmds);
        fprintf(
            stderr,
            c"WARNING: You called a perf program named '%s', which does not exist.\nContinuing under the assumption that you meant '%s'\n".as_ptr(),
            cmd,
            assumed,
        );
        if autocorrect > 0 {
            fprintf(
                stderr,
                c"in %0.1f seconds automatically...\n".as_ptr(),
                (autocorrect as f32 / 10.0f32) as f64,
            );
            poll(ptr::null_mut(), 0, autocorrect * 100);
        }
        return assumed;
    }

    fprintf(
        stderr,
        c"perf: '%s' is not a perf-command. See 'perf --help'.\n".as_ptr(),
        cmd,
    );

    if (*main_cmds).cnt != 0 && best_similarity < 6 {
        fprintf(
            stderr,
            c"\nDid you mean %s?\n".as_ptr(),
            if n < 2 {
                c"this".as_ptr()
            } else {
                c"one of these".as_ptr()
            },
        );

        i = 0;
        while i < n {
            fprintf(
                stderr,
                c"\t%s\n".as_ptr(),
                (**(*main_cmds).names.add(i as usize)).name,
            );
            i += 1;
        }
    }

    clean_cmdnames(&mut other_cmds);
    ptr::null()
}
