/* Clocksource change test
 *		by: john stultz (johnstul@us.ibm.com)
 *		(C) Copyright IBM 2012
 *		Licensed under the GPLv2
 *
 *  NOTE: This is a meta-test which quickly changes the clocksource and
 *  then uses other tests to detect problems. Thus this test requires
 *  that the inconsistency-check and nanosleep tests be present in the
 *  same directory it is run from.
 *
 *  To build:
 *	$ gcc clocksource-switch.c -o clocksource-switch -lrt
 *
 *   This program is free software: you can redistribute it and/or modify
 *   it under the terms of the GNU General Public License as published by
 *   the Free Software Foundation, either version 2 of the License, or
 *   (at your option) any later version.
 *
 *   This program is distributed in the hope that it will be useful,
 *   but WITHOUT ANY WARRANTY; without even the implied warranty of
 *   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *   GNU General Public License for more details.
 */

// C includes translated to libc/kselftest external dependencies:
// <fcntl.h>, <stdio.h>, <stdlib.h>, <string.h>, <sys/stat.h>,
// <sys/time.h>, <sys/timex.h>, <sys/types.h>, <sys/wait.h>, <time.h>,
// <unistd.h>, and "kselftest.h".

use libc::{
    c_char, c_int, close, fork, getopt, open, pid_t, read, size_t, ssize_t, system, waitpid, write,
    O_RDONLY, O_WRONLY, WEXITSTATUS, WIFEXITED, WNOHANG,
};
use std::ffi::CStr;
use std::ptr;

extern "C" {
    static mut optarg: *mut c_char;

    fn atoi(nptr: *const c_char) -> c_int;
    fn exit(status: c_int) -> !;
    fn printf(format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;

    fn ksft_print_msg(format: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_test_result(pass: c_int, format: *const c_char, ...);
    fn ksft_exit(pass: c_int) -> !;
}

unsafe fn ptr_diff(a: *mut c_char, b: *mut c_char) -> isize {
    a.offset_from(b)
}

#[no_mangle]
pub static mut clocksource_list: [[c_char; 30]; 10] = [[0; 30]; 10];

#[no_mangle]
pub unsafe extern "C" fn get_clocksources(list: (*mut [c_char; 30])) -> c_int {
    let fd: c_int;
    let size: size_t;
    let mut buf: [c_char; 512] = [0; 512];
    let mut head: *mut c_char;
    let mut tmp: *mut c_char;
    let mut i: c_int;

    fd = open(
        b"/sys/devices/system/clocksource/clocksource0/available_clocksource\0".as_ptr()
            as *const c_char,
        O_RDONLY,
    );

    size = read(fd, buf.as_mut_ptr() as *mut libc::c_void, 512) as size_t;

    close(fd);

    i = 0;
    while i < 10 {
        (*list.add(i as usize))[0] = b'\0' as c_char;
        i += 1;
    }

    head = buf.as_mut_ptr();
    i = 0;
    while ptr_diff(head, buf.as_mut_ptr()) < size as isize {
        /* Find the next space */
        tmp = head;
        while *tmp != b' ' as c_char {
            if *tmp == b'\n' as c_char {
                break;
            }
            if *tmp == b'\0' as c_char {
                break;
            }
            tmp = tmp.add(1);
        }
        *tmp = b'\0' as c_char;
        strcpy((*list.add(i as usize)).as_mut_ptr(), head);
        head = tmp.add(1);
        i += 1;
    }

    i - 1
}

#[no_mangle]
pub unsafe extern "C" fn get_cur_clocksource(buf: *mut c_char, mut size: size_t) -> c_int {
    let fd: c_int;

    fd = open(
        b"/sys/devices/system/clocksource/clocksource0/current_clocksource\0".as_ptr()
            as *const c_char,
        O_RDONLY,
    );

    size = read(fd, buf as *mut libc::c_void, size) as size_t;
    let _ = size;

    0
}

#[no_mangle]
pub unsafe extern "C" fn change_clocksource(clocksource: *mut c_char) -> c_int {
    let fd: c_int;
    let size: ssize_t;

    fd = open(
        b"/sys/devices/system/clocksource/clocksource0/current_clocksource\0".as_ptr()
            as *const c_char,
        O_WRONLY,
    );

    if fd < 0 {
        return -1;
    }

    size = write(
        fd,
        clocksource as *const libc::c_void,
        strlen(clocksource as *const c_char),
    );

    if size < 0 {
        return -1;
    }

    close(fd);
    0
}

#[no_mangle]
pub unsafe extern "C" fn run_tests(secs: c_int) -> c_int {
    let mut ret: c_int;
    let mut buf: [c_char; 255] = [0; 255];

    sprintf(
        buf.as_mut_ptr(),
        b"./inconsistency-check -t %i\0".as_ptr() as *const c_char,
        secs,
    );
    ret = system(buf.as_ptr());
    if WIFEXITED(ret) && WEXITSTATUS(ret) != 0 {
        return WEXITSTATUS(ret);
    }
    ret = system(b"./nanosleep\0".as_ptr() as *const c_char);
    if WIFEXITED(ret) {
        WEXITSTATUS(ret)
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut orig_clk: [c_char; 512] = [0; 512];
    let mut count: c_int;
    let mut i: c_int;
    let mut status: c_int = 0;
    let mut opt: c_int;
    let mut do_sanity_check: c_int = 1;
    let mut runtime: c_int = 60;
    let pid: pid_t;

    /* Process arguments */
    loop {
        opt = getopt(argc, argv, b"st:\0".as_ptr() as *const c_char);
        if opt == -1 {
            break;
        }
        match opt {
            x if x == b's' as c_int => {
                do_sanity_check = 0;
            }
            x if x == b't' as c_int => {
                runtime = atoi(optarg);
            }
            _ => {
                printf(
                    b"Usage: %s [-s] [-t <secs>]\n\0".as_ptr() as *const c_char,
                    *argv.add(0),
                );
                printf(b"\t-s: skip sanity checks\n\0".as_ptr() as *const c_char);
                printf(b"\t-t: Number of seconds to run\n\0".as_ptr() as *const c_char);
                exit(-1);
            }
        }
    }

    get_cur_clocksource(orig_clk.as_mut_ptr(), 512);

    count = get_clocksources(clocksource_list.as_mut_ptr());

    if change_clocksource(clocksource_list[0].as_mut_ptr()) != 0 {
        printf(b"Error: You probably need to run this as root\n\0".as_ptr() as *const c_char);
        return -1;
    }

    /* Check everything is sane before we start switching asynchronously */
    if do_sanity_check != 0 {
        i = 0;
        while i < count {
            ksft_print_msg(
                b"Validating clocksource %s\n\0".as_ptr() as *const c_char,
                clocksource_list[i as usize].as_mut_ptr(),
            );
            if change_clocksource(clocksource_list[i as usize].as_mut_ptr()) != 0 {
                status = -1;
                break;
            }
            if run_tests(5) != 0 {
                status = -1;
                break;
            }
            i += 1;
        }
        if status == -1 {
            change_clocksource(orig_clk.as_mut_ptr());

            /* Print at the end to not mix output with child process */
            ksft_print_header();
            ksft_set_plan(1);
            ksft_test_result((status == 0) as c_int, b"clocksource-switch\n\0".as_ptr() as *const c_char);
            ksft_exit((status == 0) as c_int);
        }
    }

    ksft_print_msg(b"Running Asynchronous Switching Tests...\n\0".as_ptr() as *const c_char);
    pid = fork();
    if pid == 0 {
        return run_tests(runtime);
    }

    while pid != waitpid(pid, &mut status as *mut c_int, WNOHANG) {
        i = 0;
        while i < count {
            if change_clocksource(clocksource_list[i as usize].as_mut_ptr()) != 0 {
                status = -1;
                change_clocksource(orig_clk.as_mut_ptr());

                /* Print at the end to not mix output with child process */
                ksft_print_header();
                ksft_set_plan(1);
                ksft_test_result(
                    (status == 0) as c_int,
                    b"clocksource-switch\n\0".as_ptr() as *const c_char,
                );
                ksft_exit((status == 0) as c_int);
            }
            i += 1;
        }
    }

    change_clocksource(orig_clk.as_mut_ptr());

    /* Print at the end to not mix output with child process */
    ksft_print_header();
    ksft_set_plan(1);
    ksft_test_result(
        (status == 0) as c_int,
        b"clocksource-switch\n\0".as_ptr() as *const c_char,
    );
    ksft_exit((status == 0) as c_int);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
