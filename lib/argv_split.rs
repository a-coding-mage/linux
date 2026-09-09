// SPDX-License-Identifier: GPL-2.0
/*
 * Helper function for splitting a string into an argv-like array.
 */

use core::ffi::c_char;

pub type gfp_t = u32;

extern "C" {
    fn isspace(c: i32) -> i32;
    fn kstrndup(s: *const c_char, max: usize, gfp: gfp_t) -> *mut c_char;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kmalloc_array(n: usize, size: usize, gfp: gfp_t) -> *mut *mut c_char;
}

extern "C" {
    static KMALLOC_MAX_SIZE: usize;
}

unsafe fn count_argc(mut str_: *const c_char) -> i32 {
    let mut count: i32 = 0;
    let mut was_space: bool = true;

    while *str_ != 0 {
        if isspace(*str_ as i32) != 0 {
            was_space = true;
        } else if was_space {
            was_space = false;
            count += 1;
        }
        str_ = str_.add(1);
    }

    count
}

/**
 * argv_free - free an argv
 * @argv: the argument vector to be freed
 *
 * Frees an argv and the strings it points to.
 */
#[no_mangle]
pub unsafe extern "C" fn argv_free(argv: *mut *mut c_char) {
    let argv = argv.offset(-1);
    kfree((*argv).cast());
    kfree(argv.cast());
}

/**
 * argv_split - split a string at whitespace, returning an argv
 * @gfp: the GFP mask used to allocate memory
 * @str: the string to be split
 * @argcp: returned argument count
 *
 * Returns: an array of pointers to strings which are split out from
 * @str.  This is performed by strictly splitting on white-space; no
 * quote processing is performed.  Multiple whitespace characters are
 * considered to be a single argument separator.  The returned array
 * is always NULL-terminated.  Returns NULL on memory allocation
 * failure.
 *
 * The source string at `str' may be undergoing concurrent alteration via
 * userspace sysctl activity (at least).  The argv_split() implementation
 * attempts to handle this gracefully by taking a local copy to work on.
 */
#[no_mangle]
pub unsafe extern "C" fn argv_split(
    gfp: gfp_t,
    str_: *const c_char,
    argcp: *mut i32,
) -> *mut *mut c_char {
    let mut argv_str: *mut c_char;
    let mut was_space: bool;
    let mut argv: *mut *mut c_char;
    let argv_ret: *mut *mut c_char;
    let argc: i32;

    argv_str = kstrndup(str_, KMALLOC_MAX_SIZE - 1, gfp);
    if argv_str.is_null() {
        return core::ptr::null_mut();
    }

    argc = count_argc(argv_str);
    argv = kmalloc_array(
        (argc + 2) as usize,
        core::mem::size_of::<*mut c_char>(),
        gfp,
    );
    if argv.is_null() {
        kfree(argv_str.cast());
        return core::ptr::null_mut();
    }

    *argv = argv_str;
    argv = argv.add(1);
    argv_ret = argv;
    was_space = true;
    while *argv_str != 0 {
        if isspace(*argv_str as i32) != 0 {
            was_space = true;
            *argv_str = 0;
        } else if was_space {
            was_space = false;
            *argv = argv_str;
            argv = argv.add(1);
        }
        argv_str = argv_str.add(1);
    }
    *argv = core::ptr::null_mut();

    if !argcp.is_null() {
        *argcp = argc;
    }
    argv_ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
