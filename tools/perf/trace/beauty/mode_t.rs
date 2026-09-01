// SPDX-License-Identifier: LGPL-2.1

// C dependencies removed from executable Rust:
// "trace/beauty/beauty.h", <sys/types.h>, <sys/stat.h>, <unistd.h>.
// The syscall_arg type, scnprintf function, and S_* mode constants are expected
// to be supplied by the surrounding translated repository.

use std::os::raw::{c_char, c_int};

extern "C" {
    fn scnprintf(bf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
}

extern "C" {
    static S_IRWXU: c_int;
    static S_IRWXG: c_int;
    static S_IRWXO: c_int;
    static S_ISUID: c_int;
    static S_ISGID: c_int;
    static S_ISVTX: c_int;
    static S_IRUSR: c_int;
    static S_IRGRP: c_int;
    static S_IROTH: c_int;
    static S_IWUSR: c_int;
    static S_IWGRP: c_int;
    static S_IWOTH: c_int;
    static S_IXUSR: c_int;
    static S_IXGRP: c_int;
    static S_IXOTH: c_int;
    static S_IFMT: c_int;
    static S_IFSOCK: c_int;
    static S_IFLNK: c_int;
    static S_IFREG: c_int;
    static S_IFBLK: c_int;
    static S_IFDIR: c_int;
    static S_IFCHR: c_int;
    static S_IFIFO: c_int;
}

#[repr(C)]
pub struct syscall_arg {
    pub show_string_prefix: bool,
    pub val: usize,
}

unsafe fn s_irwxugo() -> c_int {
    S_IRWXU | S_IRWXG | S_IRWXO
}

unsafe fn s_iallugo() -> c_int {
    S_ISUID | S_ISGID | S_ISVTX | s_irwxugo()
}

unsafe fn s_irugo() -> c_int {
    S_IRUSR | S_IRGRP | S_IROTH
}

unsafe fn s_iwugo() -> c_int {
    S_IWUSR | S_IWGRP | S_IWOTH
}

unsafe fn s_ixugo() -> c_int {
    S_IXUSR | S_IXGRP | S_IXOTH
}

#[no_mangle]
pub unsafe extern "C" fn syscall_arg__scnprintf_mode_t(
    bf: *mut c_char,
    size: usize,
    arg: *mut syscall_arg,
) -> usize {
    let show_prefix: bool = (*arg).show_string_prefix;
    let prefix: *const c_char = b"S_\0".as_ptr() as *const c_char;
    let mut printed: c_int = 0;
    let mut mode: c_int = (*arg).val as c_int;

    macro_rules! p_mode {
        ($name:ident, $value:expr) => {{
            let value: c_int = $value;
            if (mode & value) == value {
                printed += scnprintf(
                    bf.offset(printed as isize),
                    size.wrapping_sub(printed as usize),
                    b"%s%s%s\0".as_ptr() as *const c_char,
                    if printed != 0 {
                        b"|\0".as_ptr() as *const c_char
                    } else {
                        b"\0".as_ptr() as *const c_char
                    },
                    if show_prefix {
                        prefix
                    } else {
                        b"\0".as_ptr() as *const c_char
                    },
                    concat!(stringify!($name), "\0").as_ptr() as *const c_char,
                );
                mode &= !value;
            }
        }};
    }

    p_mode!(IALLUGO, s_iallugo());
    p_mode!(IRWXUGO, s_irwxugo());
    p_mode!(IRUGO, s_irugo());
    p_mode!(IWUGO, s_iwugo());
    p_mode!(IXUGO, s_ixugo());
    p_mode!(IFMT, S_IFMT);
    p_mode!(IFSOCK, S_IFSOCK);
    p_mode!(IFLNK, S_IFLNK);
    p_mode!(IFREG, S_IFREG);
    p_mode!(IFBLK, S_IFBLK);
    p_mode!(IFDIR, S_IFDIR);
    p_mode!(IFCHR, S_IFCHR);
    p_mode!(IFIFO, S_IFIFO);
    p_mode!(ISUID, S_ISUID);
    p_mode!(ISGID, S_ISGID);
    p_mode!(ISVTX, S_ISVTX);
    p_mode!(IRWXU, S_IRWXU);
    p_mode!(IRUSR, S_IRUSR);
    p_mode!(IWUSR, S_IWUSR);
    p_mode!(IXUSR, S_IXUSR);
    p_mode!(IRWXG, S_IRWXG);
    p_mode!(IRGRP, S_IRGRP);
    p_mode!(IWGRP, S_IWGRP);
    p_mode!(IXGRP, S_IXGRP);
    p_mode!(IRWXO, S_IRWXO);
    p_mode!(IROTH, S_IROTH);
    p_mode!(IWOTH, S_IWOTH);
    p_mode!(IXOTH, S_IXOTH);

    if mode != 0 {
        printed += scnprintf(
            bf.offset(printed as isize),
            size.wrapping_sub(printed as usize),
            b"%s%#x\0".as_ptr() as *const c_char,
            if printed != 0 {
                b"|\0".as_ptr() as *const c_char
            } else {
                b"\0".as_ptr() as *const c_char
            },
            mode,
        );
    }

    printed as usize
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
