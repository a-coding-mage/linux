// SPDX-License-Identifier: GPL-2.0-only

// C dependencies translated from:
// signal.h, stdio.h, stdbool.h, string.h, err.h, errno.h, limits.h,
// sys/mman.h, sys/auxv.h, sys/prctl.h, sys/resource.h, setjmp.h,
// and "helpers.h".

use libc::{
    c_char, c_int, c_ulong, c_void, siginfo_t, size_t, stack_t, MAP_ANONYMOUS, MAP_FAILED,
    MAP_PRIVATE, MAP_STACK, PROT_READ, PROT_WRITE, SA_ONSTACK, SIGALRM, SIGSEGV, SIGSTKSZ,
};

/* sigaltstack()-enforced minimum stack */
const ENFORCED_MINSIGSTKSZ: c_ulong = 2048;

// #ifndef AT_MINSIGSTKSZ
// #  define AT_MINSIGSTKSZ 51
// #endif
const AT_MINSIGSTKSZ: c_ulong = 51;

#[repr(C)]
struct jmp_buf {
    // Opaque storage for the C jmp_buf object used by sigsetjmp/siglongjmp.
    // Exact platform layout is supplied by the C library headers in the
    // original source; this preserves file-local use as raw storage.
    __opaque: [libc::c_long; 32],
}

extern "C" {
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn sigaltstack(ss: *const stack_t, old_ss: *mut stack_t) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: libc::off_t,
    ) -> *mut c_void;
    fn raise(sig: c_int) -> c_int;
    fn sigsetjmp(env: *mut jmp_buf, savemask: c_int) -> c_int;
    fn siglongjmp(env: *mut jmp_buf, val: c_int) -> !;

    fn sethandler(
        sig: c_int,
        handler: extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
        flags: c_int,
    );
    fn clearhandler(sig: c_int);
}

static mut nerrs: c_int = 0;

static mut sigalrm_expected: bool = false;

static mut at_minstack_size: c_ulong = 0;

unsafe fn setup_altstack(start: *mut c_void, size: c_ulong) -> c_int {
    let mut ss: stack_t = core::mem::zeroed();

    memset(
        &mut ss as *mut stack_t as *mut c_void,
        0,
        core::mem::size_of::<stack_t>(),
    );
    ss.ss_size = size as size_t;
    ss.ss_sp = start;

    sigaltstack(&ss as *const stack_t, core::ptr::null_mut())
}

static mut jmpbuf: jmp_buf = jmp_buf { __opaque: [0; 32] };

extern "C" fn sigsegv(_sig: c_int, _info: *mut siginfo_t, _ctx_void: *mut c_void) {
    unsafe {
        if sigalrm_expected {
            printf(
                b"[FAIL]\tWrong signal delivered: SIGSEGV (expected SIGALRM).\0".as_ptr()
                    as *const c_char,
            );
            nerrs += 1;
        } else {
            printf(b"[OK]\tSIGSEGV signal delivered.\n\0".as_ptr() as *const c_char);
        }

        siglongjmp(&mut jmpbuf as *mut jmp_buf, 1);
    }
}

extern "C" fn sigalrm(_sig: c_int, _info: *mut siginfo_t, _ctx_void: *mut c_void) {
    unsafe {
        if !sigalrm_expected {
            printf(
                b"[FAIL]\tWrong signal delivered: SIGALRM (expected SIGSEGV).\0".as_ptr()
                    as *const c_char,
            );
            nerrs += 1;
        } else {
            printf(b"[OK]\tSIGALRM signal delivered.\n\0".as_ptr() as *const c_char);
        }
    }
}

unsafe fn test_sigaltstack(altstack: *mut c_void, size: c_ulong) {
    if setup_altstack(altstack, size) != 0 {
        err(1, b"sigaltstack()\0".as_ptr() as *const c_char);
    }

    sigalrm_expected = if size > at_minstack_size { true } else { false };

    sethandler(SIGSEGV, sigsegv, 0);
    sethandler(SIGALRM, sigalrm, SA_ONSTACK);

    if sigsetjmp(&mut jmpbuf as *mut jmp_buf, 1) == 0 {
        printf(
            b"[RUN]\tTest an alternate signal stack of %ssufficient size.\n\0".as_ptr()
                as *const c_char,
            if sigalrm_expected {
                b"\0".as_ptr() as *const c_char
            } else {
                b"in\0".as_ptr() as *const c_char
            },
        );
        printf(
            b"\tRaise SIGALRM. %s is expected to be delivered.\n\0".as_ptr() as *const c_char,
            if sigalrm_expected {
                b"It\0".as_ptr() as *const c_char
            } else {
                b"SIGSEGV\0".as_ptr() as *const c_char
            },
        );
        raise(SIGALRM);
    }

    clearhandler(SIGALRM);
    clearhandler(SIGSEGV);
}

fn main() -> c_int {
    unsafe {
        let altstack: *mut c_void;

        at_minstack_size = getauxval(AT_MINSIGSTKSZ);

        altstack = mmap(
            core::ptr::null_mut(),
            (at_minstack_size + SIGSTKSZ as c_ulong) as size_t,
            PROT_READ | PROT_WRITE,
            MAP_PRIVATE | MAP_ANONYMOUS | MAP_STACK,
            -1,
            0,
        );
        if altstack == MAP_FAILED {
            err(1, b"mmap()\0".as_ptr() as *const c_char);
        }

        if (ENFORCED_MINSIGSTKSZ + 1) < at_minstack_size {
            test_sigaltstack(altstack, ENFORCED_MINSIGSTKSZ + 1);
        }

        test_sigaltstack(altstack, at_minstack_size + SIGSTKSZ as c_ulong);

        if nerrs == 0 {
            0
        } else {
            1
        }
    }
}
