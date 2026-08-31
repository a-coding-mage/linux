/*
 * signal.c: Register a sigaltstack for objtool, to be able to
 *	     run a signal handler on a separate stack even if
 *	     the main process stack has overflown. Print out
 *	     stack overflow errors when this happens.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem;
use core::ptr;

// C dependencies:
// #include <stdio.h>
// #include <stdlib.h>
// #include <signal.h>
// #include <unistd.h>
// #include <sys/resource.h>
// #include <string.h>
// #include <objtool/objtool.h>
// #include <objtool/warn.h>

static mut stack_limit: c_ulong = 0;

unsafe extern "C" {
    static objname: *const c_char;

    fn ERROR_GLIBC(fmt: *const c_char, ...);
    fn ERROR(fmt: *const c_char, ...);
}

unsafe fn is_stack_overflow(fault_addr: *mut c_void) -> bool {
    let fault: c_ulong = fault_addr as c_ulong;

    /* Check if fault is in the guard page just below the limit. */
    fault < stack_limit && fault >= stack_limit.wrapping_sub(4096)
}

unsafe extern "C" fn signal_handler(
    sig_num: c_int,
    info: *mut libc::siginfo_t,
    _context: *mut c_void,
) {
    let mut sa_dfl: libc::sigaction = mem::zeroed();
    let sig_name: *const c_char;
    let mut msg: [c_char; 256] = [0; 256];
    let mut msg_len: c_int;

    match sig_num {
        libc::SIGSEGV => {
            sig_name = c"SIGSEGV".as_ptr();
        }
        libc::SIGBUS => {
            sig_name = c"SIGBUS".as_ptr();
        }
        libc::SIGILL => {
            sig_name = c"SIGILL".as_ptr();
        }
        libc::SIGABRT => {
            sig_name = c"SIGABRT".as_ptr();
        }
        _ => {
            sig_name = c"Unknown signal".as_ptr();
        }
    }

    if is_stack_overflow((*info).si_addr() as *mut c_void) {
        msg_len = libc::snprintf(
            msg.as_mut_ptr(),
            msg.len(),
            c"%s: error: %s: objtool stack overflow!\n".as_ptr(),
            objname,
            sig_name,
        );
    } else {
        msg_len = libc::snprintf(
            msg.as_mut_ptr(),
            msg.len(),
            c"%s: error: %s: objtool crash!\n".as_ptr(),
            objname,
            sig_name,
        );
    }

    msg_len = libc::write(libc::STDERR_FILENO, msg.as_ptr() as *const c_void, msg_len as usize) as c_int;

    /* Re-raise the signal to trigger the core dump */
    sa_dfl.sa_sigaction = libc::SIG_DFL;
    libc::sigaction(sig_num, &sa_dfl, ptr::null_mut());
    libc::raise(sig_num);
}

unsafe fn read_stack_limit() -> c_int {
    let mut stack_start: c_ulong = 0;
    let mut stack_end: c_ulong = 0;
    let mut rlim: libc::rlimit = mem::zeroed();
    let mut line: [c_char; 256] = [0; 256];
    let mut ret: c_int = 0;
    let mut found_stack: bool = false;
    let fp: *mut libc::FILE;

    if libc::getrlimit(libc::RLIMIT_STACK, &mut rlim) != 0 {
        ERROR_GLIBC(c"getrlimit".as_ptr());
        return -1;
    }

    fp = libc::fopen(c"/proc/self/maps".as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        ERROR_GLIBC(c"fopen".as_ptr());
        return -1;
    }

    while !libc::fgets(line.as_mut_ptr(), line.len() as c_int, fp).is_null() {
        if !libc::strstr(line.as_ptr(), c"[stack]".as_ptr()).is_null() {
            found_stack = true;
            if libc::sscanf(
                line.as_ptr(),
                c"%lx-%lx".as_ptr(),
                &mut stack_start as *mut c_ulong,
                &mut stack_end as *mut c_ulong,
            ) != 2
            {
                ERROR_GLIBC(c"sscanf".as_ptr());
                ret = -1;
                break;
            }
            stack_limit = stack_end.wrapping_sub(rlim.rlim_cur as c_ulong);
            break;
        }
    }

    if !found_stack {
        ret = -1;
        ERROR(c"/proc/self/maps: can't find [stack]".as_ptr());
    }

    libc::fclose(fp);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn init_signal_handler() -> c_int {
    let signals: [c_int; 4] = [libc::SIGSEGV, libc::SIGBUS, libc::SIGILL, libc::SIGABRT];
    let mut sa: libc::sigaction = mem::zeroed();
    let mut ss: libc::stack_t = mem::zeroed();

    if read_stack_limit() != 0 {
        return -1;
    }

    ss.ss_sp = libc::malloc(libc::SIGSTKSZ);
    if ss.ss_sp.is_null() {
        ERROR_GLIBC(c"malloc".as_ptr());
        return -1;
    }
    ss.ss_size = libc::SIGSTKSZ;
    ss.ss_flags = 0;

    if libc::sigaltstack(&ss, ptr::null_mut()) == -1 {
        ERROR_GLIBC(c"sigaltstack".as_ptr());
        return -1;
    }

    sa.sa_sigaction = signal_handler as usize;
    libc::sigemptyset(&mut sa.sa_mask);

    sa.sa_flags = libc::SA_ONSTACK | libc::SA_SIGINFO;

    for i in 0..signals.len() {
        if libc::sigaction(signals[i], &sa, ptr::null_mut()) == -1 {
            ERROR_GLIBC(c"sigaction".as_ptr());
            return -1;
        }
    }

    0
}
