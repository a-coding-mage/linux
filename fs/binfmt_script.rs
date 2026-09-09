// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/fs/binfmt_script.c
 *
 *  Copyright (C) 1996  Martin von Löwis
 *  original #!-checking implemented by tytso.
 */

// Dependencies supplied by the kernel environment:
// linux/module.h, linux/string.h, linux/stat.h, linux/binfmts.h,
// linux/init.h, linux/file.h, linux/err.h, linux/fs.h

#[inline]
unsafe fn spacetab(c: i8) -> bool {
    c == b' ' as i8 || c == b'\t' as i8
}

#[inline]
unsafe fn next_non_spacetab(first: *const i8, last: *const i8) -> *const i8 {
    let mut current = first;
    while current <= last {
        if !spacetab(*current) {
            return current;
        }
        current = current.add(1);
    }
    core::ptr::null()
}

#[inline]
unsafe fn next_terminator(first: *const i8, last: *const i8) -> *const i8 {
    let mut current = first;
    while current <= last {
        if spacetab(*current) || *current == 0 {
            return current;
        }
        current = current.add(1);
    }
    core::ptr::null()
}

unsafe fn load_script(bprm: *mut linux_binprm) -> c_int {
    let mut i_name: *const i8;
    let mut i_sep: *const i8;
    let mut i_arg: *const i8;
    let mut i_end: *const i8;
    let buf_end: *const i8;
    let mut file: *mut file;
    let mut retval: c_int;

    /* Not ours to exec if we don't start with "#!". */
    if ((*bprm).buf[0] != b'#' as i8) || ((*bprm).buf[1] != b'!' as i8) {
        return -ENOEXEC;
    }

    /*
     * This section handles parsing the #! line into separate
     * interpreter path and argument strings. We must be careful
     * because bprm->buf is not yet guaranteed to be NUL-terminated
     * (though the buffer will have trailing NUL padding when the
     * file size was smaller than the buffer size).
     *
     * We do not want to exec a truncated interpreter path, so either
     * we find a newline (which indicates nothing is truncated), or
     * we find a space/tab/NUL after the interpreter path (which
     * itself may be preceded by spaces/tabs). Truncating the
     * arguments is fine: the interpreter can re-read the script to
     * parse them on its own.
     */
    buf_end = (*bprm).buf.as_ptr().add(core::mem::size_of_val(&(*bprm).buf) - 1);
    i_end = strnchr((*bprm).buf.as_ptr(), core::mem::size_of_val(&(*bprm).buf), b'\n' as i8);
    if i_end.is_null() {
        i_end = next_non_spacetab((*bprm).buf.as_ptr().add(2), buf_end);
        if i_end.is_null() {
            return -ENOEXEC; /* Entire buf is spaces/tabs */
        }
        /* If there is no later space/tab/NUL we must assume the interpreter path is truncated. */
        if next_terminator(i_end, buf_end).is_null() {
            return -ENOEXEC;
        }
        i_end = buf_end;
    }
    /* Trim any trailing spaces/tabs from i_end */
    while spacetab(*i_end.sub(1)) {
        i_end = i_end.sub(1);
    }

    /* Skip over leading spaces/tabs */
    i_name = next_non_spacetab((*bprm).buf.as_ptr().add(2), i_end);
    if i_name.is_null() || i_name == i_end {
        return -ENOEXEC; /* No interpreter name found */
    }

    /* Is there an optional argument? */
    i_arg = core::ptr::null();
    i_sep = next_terminator(i_name, i_end);
    if !i_sep.is_null() && *i_sep != 0 {
        i_arg = next_non_spacetab(i_sep, i_end);
    }

    if (*bprm).interp_flags & BINPRM_FLAGS_PATH_INACCESSIBLE != 0 {
        return -ENOENT;
    }

    retval = remove_arg_zero(bprm);
    if retval != 0 { return retval; }
    retval = copy_string_kernel((*bprm).interp, bprm);
    if retval < 0 { return retval; }
    (*bprm).argc += 1;
    *(i_end as *mut i8) = 0;
    if !i_arg.is_null() {
        *(i_sep as *mut i8) = 0;
        retval = copy_string_kernel(i_arg, bprm);
        if retval < 0 { return retval; }
        (*bprm).argc += 1;
    }
    retval = copy_string_kernel(i_name, bprm);
    if retval != 0 { return retval; }
    (*bprm).argc += 1;
    retval = bprm_change_interp(i_name, bprm);
    if retval < 0 { return retval; }

    file = open_exec(i_name);
    if IS_ERR(file) {
        return PTR_ERR(file);
    }
    (*bprm).interpreter = file;
    0
}

static mut script_format: linux_binfmt = linux_binfmt {
    module: THIS_MODULE,
    load_binary: Some(load_script),
};

unsafe fn init_script_binfmt() -> c_int {
    register_binfmt(&mut script_format);
    0
}

unsafe fn exit_script_binfmt() {
    unregister_binfmt(&mut script_format);
}

// core_initcall(init_script_binfmt);
// module_exit(exit_script_binfmt);
// MODULE_DESCRIPTION("Kernel support for scripts starting with #!");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
