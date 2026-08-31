// SPDX-License-Identifier: GPL-2.0-only
/*
 * syscall_nt.c - checks syscalls with NT set
 * Copyright (c) 2014-2015 Andrew Lutomirski
 *
 * Some obscure user-space code requires the ability to make system calls
 * with FLAGS.NT set.  Make sure it works.
 */

use core::ffi::{c_int, c_long, c_ulong, c_void};

// C dependencies:
// #include <stdio.h>
// #include <unistd.h>
// #include <string.h>
// #include <signal.h>
// #include <err.h>
// #include <sys/syscall.h>
// #include "helpers.h"

type SiginfoT = c_void;

unsafe extern "C" {
    fn printf(format: *const i8, ...) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;

    fn get_eflags() -> c_ulong;
    fn set_eflags(flags: c_ulong);
    fn sethandler(
        sig: c_int,
        handler: Option<unsafe extern "C" fn(c_int, *mut SiginfoT, *mut c_void)>,
        flags: c_int,
    );

    static SYS_getpid: c_long;
    static SIGTRAP: c_int;

    static X86_EFLAGS_IF: c_ulong;
    static X86_EFLAGS_FIXED: c_ulong;
    static X86_EFLAGS_NT: c_ulong;
    static X86_EFLAGS_AC: c_ulong;
    static X86_EFLAGS_TF: c_ulong;
    static X86_EFLAGS_DF: c_ulong;
}

static mut nerrs: c_uint = 0;

type c_uint = u32;

unsafe extern "C" fn sigtrap(_sig: c_int, _si: *mut SiginfoT, _ctx_void: *mut c_void) {}

unsafe fn do_it(extraflags: c_ulong) {
    let flags: c_ulong;

    unsafe {
        set_eflags(get_eflags() | extraflags);
        syscall(SYS_getpid);
        flags = get_eflags();
        set_eflags(X86_EFLAGS_IF | X86_EFLAGS_FIXED);
        if (flags & extraflags) == extraflags {
            printf(c"[OK]\tThe syscall worked and flags are still set\n".as_ptr());
        } else {
            printf(
                c"[FAIL]\tThe syscall worked but flags were cleared (flags = 0x%lx but expected 0x%lx set)\n"
                    .as_ptr(),
                flags,
                extraflags,
            );
            nerrs += 1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    unsafe {
        printf(c"[RUN]\tSet NT and issue a syscall\n".as_ptr());
        do_it(X86_EFLAGS_NT);

        printf(c"[RUN]\tSet AC and issue a syscall\n".as_ptr());
        do_it(X86_EFLAGS_AC);

        printf(c"[RUN]\tSet NT|AC and issue a syscall\n".as_ptr());
        do_it(X86_EFLAGS_NT | X86_EFLAGS_AC);

        /*
         * Now try it again with TF set -- TF forces returns via IRET in all
         * cases except non-ptregs-using 64-bit full fast path syscalls.
         */

        sethandler(SIGTRAP, Some(sigtrap), 0);

        printf(c"[RUN]\tSet TF and issue a syscall\n".as_ptr());
        do_it(X86_EFLAGS_TF);

        printf(c"[RUN]\tSet NT|TF and issue a syscall\n".as_ptr());
        do_it(X86_EFLAGS_NT | X86_EFLAGS_TF);

        printf(c"[RUN]\tSet AC|TF and issue a syscall\n".as_ptr());
        do_it(X86_EFLAGS_AC | X86_EFLAGS_TF);

        printf(c"[RUN]\tSet NT|AC|TF and issue a syscall\n".as_ptr());
        do_it(X86_EFLAGS_NT | X86_EFLAGS_AC | X86_EFLAGS_TF);

        /*
         * Now try DF.  This is evil and it's plausible that we will crash
         * glibc, but glibc would have to do something rather surprising
         * for this to happen.
         */
        printf(c"[RUN]\tSet DF and issue a syscall\n".as_ptr());
        do_it(X86_EFLAGS_DF);

        printf(c"[RUN]\tSet TF|DF and issue a syscall\n".as_ptr());
        do_it(X86_EFLAGS_TF | X86_EFLAGS_DF);

        if nerrs == 0 { 0 } else { 1 }
    }
}
