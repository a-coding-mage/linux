// SPDX-License-Identifier: GPL-2.0
/*
 * Stas Sergeev <stsp@users.sourceforge.net>
 *
 * test sigaltstack(SS_ONSTACK | SS_AUTODISARM)
 * If that succeeds, then swapcontext() can be used inside sighandler safely.
 *
 */

// C dependencies: _GNU_SOURCE, signal.h, stdio.h, stdlib.h, sys/mman.h,
// ucontext.h, alloca.h, string.h, assert.h, errno.h, sys/auxv.h,
// "kselftest.h", and "current_stack_pointer.h".

use libc::{
    c_char, c_int, c_ulong, c_void, raise, sigaction, sigaltstack, sigemptyset, stack_t, strerror,
    EINVAL, EXIT_FAILURE, MAP_ANONYMOUS, MAP_FAILED, MAP_PRIVATE, MAP_STACK, PROT_READ, PROT_WRITE,
    SA_ONSTACK, SA_SIGINFO, SIGSTKSZ, SIGUSR1, SIGUSR2, SS_DISABLE, SS_ONSTACK,
};

const SS_AUTODISARM: c_int = (1u32 << 31) as c_int;
const AT_MINSIGSTKSZ: c_ulong = 51;

static mut stack_size: libc::c_uint = 0;
static mut sstack: *mut c_void = core::ptr::null_mut();
static mut ustack: *mut c_void = core::ptr::null_mut();
static mut uc: libc::ucontext_t = unsafe { core::mem::zeroed() };
static mut sc: libc::ucontext_t = unsafe { core::mem::zeroed() };
static msg: &[u8] = b"[OK]\tStack preserved\0";
static msg2: &[u8] = b"[FAIL]\tStack corrupted\0";

#[repr(C)]
struct stk_data {
    msg: [c_char; 128],
    flag: c_int,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut sp: c_ulong;

    fn alloca(size: libc::size_t) -> *mut c_void;
    fn exit(status: c_int) -> !;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn getcontext(ucp: *mut libc::ucontext_t) -> c_int;
    fn makecontext(ucp: *mut libc::ucontext_t, func: extern "C" fn(), argc: c_int, ...);
    fn memmem(
        haystack: *const c_void,
        haystacklen: libc::size_t,
        needle: *const c_void,
        needlelen: libc::size_t,
    ) -> *mut c_void;
    fn setcontext(ucp: *const libc::ucontext_t) -> c_int;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> libc::size_t;
    fn swapcontext(oucp: *mut libc::ucontext_t, ucp: *const libc::ucontext_t) -> c_int;

    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_exit_pass() -> !;
    fn ksft_print_header();
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_set_plan(plan: c_int);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
}

unsafe extern "C" fn my_usr1(_sig: c_int, _si: *mut libc::siginfo_t, _u: *mut c_void) {
    let aa: *mut c_char;
    let mut err: c_int;
    let mut stk: stack_t = core::mem::zeroed();
    let p: *mut stk_data;

    if sp < sstack as c_ulong || sp >= sstack as c_ulong + stack_size as c_ulong {
        ksft_exit_fail_msg(c"SP is not on sigaltstack\n".as_ptr());
    }
    /* put some data on stack. other sighandler will try to overwrite it */
    aa = alloca(1024) as *mut c_char;
    assert!(!aa.is_null());
    p = aa.add(512) as *mut stk_data;
    strcpy((*p).msg.as_mut_ptr(), msg.as_ptr() as *const c_char);
    (*p).flag = 1;
    ksft_print_msg(c"[RUN]\tsignal USR1\n".as_ptr());
    err = sigaltstack(core::ptr::null(), &mut stk);
    if err != 0 {
        ksft_exit_fail_msg(c"sigaltstack() - %s\n".as_ptr(), strerror(errno));
        exit(EXIT_FAILURE);
    }
    if stk.ss_flags != SS_DISABLE {
        ksft_test_result_fail(
            c"tss_flags=%x, should be SS_DISABLE\n".as_ptr(),
            stk.ss_flags,
        );
    } else {
        ksft_test_result_pass(c"sigaltstack is disabled in sighandler\n".as_ptr());
    }
    swapcontext(&raw mut sc, &raw const uc);
    ksft_print_msg(c"%s\n".as_ptr(), (*p).msg.as_ptr());
    if (*p).flag == 0 {
        ksft_exit_fail_msg(c"[RUN]\tAborting\n".as_ptr());
        exit(EXIT_FAILURE);
    }
}

unsafe extern "C" fn my_usr2(_sig: c_int, _si: *mut libc::siginfo_t, _u: *mut c_void) {
    let aa: *mut c_char;
    let p: *mut stk_data;

    ksft_print_msg(c"[RUN]\tsignal USR2\n".as_ptr());
    aa = alloca(1024) as *mut c_char;
    /* dont run valgrind on this */
    /* try to find the data stored by previous sighandler */
    p = memmem(
        aa as *const c_void,
        1024,
        msg.as_ptr() as *const c_void,
        strlen(msg.as_ptr() as *const c_char),
    ) as *mut stk_data;
    if !p.is_null() {
        ksft_test_result_fail(c"sigaltstack re-used\n".as_ptr());
        /* corrupt the data */
        strcpy((*p).msg.as_mut_ptr(), msg2.as_ptr() as *const c_char);
        /* tell other sighandler that his data is corrupted */
        (*p).flag = 0;
    }
}

extern "C" fn switch_fn() {
    unsafe {
        ksft_print_msg(c"[RUN]\tswitched to user ctx\n".as_ptr());
        raise(SIGUSR2);
        setcontext(&raw const sc);
    }
}

fn main() {
    unsafe {
        let mut act: sigaction = core::mem::zeroed();
        let mut stk: stack_t = core::mem::zeroed();
        let mut err: c_int;

        /* Make sure more than the required minimum. */
        stack_size = (getauxval(AT_MINSIGSTKSZ) + SIGSTKSZ as c_ulong) as libc::c_uint;
        ksft_print_msg(c"[NOTE]\tthe stack size is %u\n".as_ptr(), stack_size);

        ksft_print_header();
        ksft_set_plan(3);

        sigemptyset(&mut act.sa_mask);
        act.sa_flags = SA_ONSTACK | SA_SIGINFO;
        act.sa_sigaction = my_usr1 as usize;
        sigaction(SIGUSR1, &act, core::ptr::null_mut());
        act.sa_sigaction = my_usr2 as usize;
        sigaction(SIGUSR2, &act, core::ptr::null_mut());
        sstack = libc::mmap(
            core::ptr::null_mut(),
            stack_size as libc::size_t,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS | MAP_STACK,
            -1,
            0,
        );
        if sstack == MAP_FAILED {
            ksft_exit_fail_msg(c"mmap() - %s\n".as_ptr(), strerror(errno));
            std::process::exit(EXIT_FAILURE);
        }

        err = sigaltstack(core::ptr::null(), &mut stk);
        if err != 0 {
            ksft_exit_fail_msg(c"sigaltstack() - %s\n".as_ptr(), strerror(errno));
            exit(EXIT_FAILURE);
        }
        if stk.ss_flags == SS_DISABLE {
            ksft_test_result_pass(c"Initial sigaltstack state was SS_DISABLE\n".as_ptr());
        } else {
            ksft_exit_fail_msg(
                c"Initial sigaltstack state was %x; should have been SS_DISABLE\n".as_ptr(),
                stk.ss_flags,
            );
            std::process::exit(EXIT_FAILURE);
        }

        stk.ss_sp = sstack;
        stk.ss_size = stack_size as libc::size_t;
        stk.ss_flags = SS_ONSTACK | SS_AUTODISARM;
        err = sigaltstack(&stk, core::ptr::null_mut());
        if err != 0 {
            if errno == EINVAL {
                ksft_test_result_skip(
                    c"[NOTE]\tThe running kernel doesn't support SS_AUTODISARM\n".as_ptr(),
                );
                /*
                 * If test cases for the !SS_AUTODISARM variant were
                 * added, we could still run them.  We don't have any
                 * test cases like that yet, so just exit and report
                 * success.
                 */
                return;
            } else {
                ksft_exit_fail_msg(
                    c"sigaltstack(SS_ONSTACK | SS_AUTODISARM)  %s\n".as_ptr(),
                    strerror(errno),
                );
                std::process::exit(EXIT_FAILURE);
            }
        }

        ustack = libc::mmap(
            core::ptr::null_mut(),
            stack_size as libc::size_t,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS | MAP_STACK,
            -1,
            0,
        );
        if ustack == MAP_FAILED {
            ksft_exit_fail_msg(c"mmap() - %s\n".as_ptr(), strerror(errno));
            std::process::exit(EXIT_FAILURE);
        }
        getcontext(&raw mut uc);
        uc.uc_link = core::ptr::null_mut();
        uc.uc_stack.ss_sp = ustack;
        uc.uc_stack.ss_size = stack_size as libc::size_t;
        makecontext(&raw mut uc, switch_fn, 0);
        raise(SIGUSR1);

        err = sigaltstack(core::ptr::null(), &mut stk);
        if err != 0 {
            ksft_exit_fail_msg(c"sigaltstack() - %s\n".as_ptr(), strerror(errno));
            exit(EXIT_FAILURE);
        }
        if stk.ss_flags != SS_AUTODISARM {
            ksft_exit_fail_msg(
                c"ss_flags=%x, should be SS_AUTODISARM\n".as_ptr(),
                stk.ss_flags,
            );
            exit(EXIT_FAILURE);
        }
        ksft_test_result_pass(c"sigaltstack is still SS_AUTODISARM after signal\n".as_ptr());

        ksft_exit_pass();
    }
}
