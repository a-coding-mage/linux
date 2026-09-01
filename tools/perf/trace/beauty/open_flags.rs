// SPDX-License-Identifier: LGPL-2.1

use core::ffi::{c_char, c_int, c_ulong};

pub type size_t = usize;

// Dependencies originally supplied by:
// #include "trace/beauty/beauty.h"
// #include <sys/types.h>
// #include <sys/stat.h>
// #include <fcntl.h>

unsafe extern "C" {
    fn scnprintf(bf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
}

#[repr(C)]
pub struct syscall_arg {
    pub val: c_ulong,
    pub mask: c_ulong,
    pub idx: c_int,
    pub show_string_prefix: bool,
}

pub const O_ACCMODE: c_ulong = 0o00000003;
pub const O_RDONLY: c_ulong = 0o00000000;
pub const O_WRONLY: c_ulong = 0o00000001;
pub const O_RDWR: c_ulong = 0o00000002;
pub const O_CREAT: c_ulong = 0o00000100;
pub const O_EXCL: c_ulong = 0o00000200;
pub const O_NOCTTY: c_ulong = 0o00000400;
pub const O_TRUNC: c_ulong = 0o00001000;
pub const O_APPEND: c_ulong = 0o00002000;
pub const O_NONBLOCK: c_ulong = 0o00004000;
pub const O_DSYNC: c_ulong = 0o00010000;
pub const O_ASYNC: c_ulong = 0o00020000;
pub const O_DIRECT: c_ulong = 0o00040000;
pub const O_LARGEFILE: c_ulong = 0o00100000;
pub const O_DIRECTORY: c_ulong = 0o00200000;
pub const O_NOFOLLOW: c_ulong = 0o00400000;
pub const O_NOATIME: c_ulong = 0o01000000;
pub const O_CLOEXEC: c_ulong = 0o02000000;
pub const O_SYNC: c_ulong = 0o04010000;
pub const O_PATH: c_ulong = 0o10000000;
pub const O_TMPFILE: c_ulong = 0o20000000;

unsafe fn p_flag(
    flags: *mut c_ulong,
    printed: *mut c_int,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
    flag: c_ulong,
    name: *const c_char,
) {
    if (*flags & flag) != 0 {
        *printed += scnprintf(
            bf.add(*printed as usize),
            size.wrapping_sub(*printed as usize),
            c"%s%s%s".as_ptr(),
            if *printed != 0 {
                c"|".as_ptr()
            } else {
                c"".as_ptr()
            },
            if show_prefix {
                c"O_".as_ptr()
            } else {
                c"".as_ptr()
            },
            name,
        );
        *flags &= !flag;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn open__scnprintf_flags(
    mut flags: c_ulong,
    bf: *mut c_char,
    size: size_t,
    show_prefix: bool,
) -> size_t {
    let prefix = c"O_".as_ptr();
    let mut printed: c_int = 0;

    if (flags & O_ACCMODE) == O_RDONLY {
        printed = scnprintf(
            bf,
            size,
            c"%s%s".as_ptr(),
            if show_prefix { prefix } else { c"".as_ptr() },
            c"RDONLY".as_ptr(),
        );
    }
    if flags == 0 {
        return printed as size_t;
    }

    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_RDWR, c"RDWR".as_ptr());
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_APPEND, c"APPEND".as_ptr());
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_ASYNC, c"ASYNC".as_ptr());
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_CLOEXEC, c"CLOEXEC".as_ptr());
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_CREAT, c"CREAT".as_ptr());
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_DIRECT, c"DIRECT".as_ptr());
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_DIRECTORY, c"DIRECTORY".as_ptr());
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_EXCL, c"EXCL".as_ptr());
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_LARGEFILE, c"LARGEFILE".as_ptr());
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_NOFOLLOW, c"NOFOLLOW".as_ptr());
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_TMPFILE, c"TMPFILE".as_ptr());
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_NOATIME, c"NOATIME".as_ptr());
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_NOCTTY, c"NOCTTY".as_ptr());

    // Original C condition: #ifdef O_NONBLOCK, otherwise #elif O_NDELAY.
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_NONBLOCK, c"NONBLOCK".as_ptr());

    // Original C condition: #ifdef O_PATH.
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_PATH, c"PATH".as_ptr());

    // Original C condition: #ifdef O_DSYNC, otherwise P_FLAG(SYNC).
    if (flags & O_SYNC) == O_SYNC {
        printed += scnprintf(
            bf.add(printed as usize),
            size.wrapping_sub(printed as usize),
            c"%s%s%s".as_ptr(),
            if printed != 0 {
                c"|".as_ptr()
            } else {
                c"".as_ptr()
            },
            if show_prefix {
                c"O_".as_ptr()
            } else {
                c"".as_ptr()
            },
            c"SYNC".as_ptr(),
        );
    } else {
        p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_DSYNC, c"DSYNC".as_ptr());
    }
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_TRUNC, c"TRUNC".as_ptr());
    p_flag(&mut flags, &mut printed, bf, size, show_prefix, O_WRONLY, c"WRONLY".as_ptr());

    if flags != 0 {
        printed += scnprintf(
            bf.add(printed as usize),
            size.wrapping_sub(printed as usize),
            c"%s%#x".as_ptr(),
            if printed != 0 {
                c"|".as_ptr()
            } else {
                c"".as_ptr()
            },
            flags as c_int,
        );
    }

    printed as size_t
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn syscall_arg__scnprintf_open_flags(
    bf: *mut c_char,
    size: size_t,
    arg: *mut syscall_arg,
) -> size_t {
    let flags: c_int = (*arg).val as c_int;

    if (flags & O_CREAT as c_int) == 0 {
        (*arg).mask |= 1 << ((*arg).idx + 1); /* Mask the mode parm */
    }

    open__scnprintf_flags(
        flags as c_ulong,
        bf,
        size,
        (*arg).show_string_prefix,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
