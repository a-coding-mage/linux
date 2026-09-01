// SPDX-License-Identifier: GPL-2.0
// C dependencies: math.h, unistd.h, stdio.h, stdlib.h, sys/types.h,
// sys/stat.h, fcntl.h, sys/timeb.h, sched.h, errno.h

#![allow(non_camel_case_types)]

type c_char = i8;
type c_int = i32;
type c_long = i64;
type c_longlong = i64;
type c_void = core::ffi::c_void;
type size_t = usize;
type ssize_t = isize;
type off_t = i64;

const O_RDONLY: c_int = 0;

unsafe extern "C" {
    static mut errno: c_int;

    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn perror(s: *const c_char);
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;
    fn printf(format: *const c_char, ...) -> c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut cpu: c_int;
    let mut fd: c_int;
    let mut msr: c_longlong = 0;
    let mut msr_file_name: [c_char; 64] = [0; 64];

    if argc != 2 {
        return 1;
    }

    errno = 0;
    cpu = strtol(*argv.add(1), core::ptr::null_mut(), 10) as c_int;

    if errno != 0 {
        return 1;
    }

    sprintf(
        msr_file_name.as_mut_ptr(),
        c"/dev/cpu/%d/msr".as_ptr() as *const c_char,
        cpu,
    );
    fd = open(msr_file_name.as_ptr(), O_RDONLY);

    if fd == -1 {
        perror(c"Failed to open".as_ptr() as *const c_char);
        return 1;
    }

    pread(
        fd,
        &mut msr as *mut c_longlong as *mut c_void,
        core::mem::size_of_val(&msr),
        0x199,
    );

    printf(c"msr 0x199: 0x%llx\n".as_ptr() as *const c_char, msr);
    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
