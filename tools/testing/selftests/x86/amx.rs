// SPDX-License-Identifier: GPL-2.0

// C dependencies removed from executable Rust:
// err.h, errno.h, setjmp.h, stdio.h, string.h, stdbool.h, unistd.h, x86intrin.h,
// sys/auxv.h, sys/mman.h, sys/shm.h, sys/syscall.h, sys/wait.h, helpers.h,
// xstate.h. This test is 64-bit only in the original source.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem;
use core::ptr;

type PidT = libc::pid_t;

#[repr(C)]
pub struct xstate_info {
    pub size: c_int,
    pub xbuf_offset: c_int,
}

#[repr(C)]
pub struct xsave_buffer {
    pub bytes: [u8; 0],
}

#[repr(C)]
pub struct _fpx_sw_bytes {
    pub magic1: u32,
    pub extended_size: u32,
    pub xfeatures: u64,
    pub xstate_size: u32,
    pub padding: [u32; 7],
}

unsafe extern "C" {
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> usize;
    fn strncat(dst: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: libc::off_t,
    ) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn sigaltstack(ss: *const libc::stack_t, old_ss: *mut libc::stack_t) -> c_int;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn fork() -> PidT;
    fn wait(wstatus: *mut c_int) -> PidT;
    fn _exit(status: c_int) -> !;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;

    fn alloc_xbuf() -> *mut xsave_buffer;
    fn clear_xstate_header(xbuf: *mut xsave_buffer);
    fn xrstor(xbuf: *mut xsave_buffer, mask: u64);
    fn xsave(xbuf: *mut xsave_buffer, mask: u64);
    fn set_xstatebv(xbuf: *mut xsave_buffer, bv: u64);
    fn set_rand_data(info: *mut xstate_info, xbuf: *mut xsave_buffer);
    fn get_fpx_sw_bytes(xbuf: *mut c_void) -> *mut _fpx_sw_bytes;
    fn get_fpx_sw_bytes_features(xbuf: *mut c_void) -> u64;
    fn get_xstate_info(feature: c_int) -> xstate_info;
    fn ksft_print_msg(fmt: *const c_char, ...) -> c_int;
    fn sethandler(
        sig: c_int,
        handler: unsafe extern "C" fn(c_int, *mut libc::siginfo_t, *mut c_void),
        flags: c_int,
    );
    fn clearhandler(sig: c_int);
    fn test_xstate(feature: c_int);
}

macro_rules! c_str {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

/* err() exits and will not return */
macro_rules! fatal_error {
    ($msg:literal $(, $arg:expr)* $(,)?) => {{
        unsafe { err(1, concat!("[FAIL]\t", $msg, "\0").as_ptr() as *const c_char $(, $arg)*); }
    }};
}

const XFEATURE_XTILECFG: c_int = 17;
const XFEATURE_XTILEDATA: c_int = 18;

const XFEATURE_MASK_XTILECFG: u64 = 1u64 << XFEATURE_XTILECFG;
const XFEATURE_MASK_XTILEDATA: u64 = 1u64 << XFEATURE_XTILEDATA;
const XFEATURE_MASK_XTILE: u64 = XFEATURE_MASK_XTILECFG | XFEATURE_MASK_XTILEDATA;

static mut xtiledata: xstate_info = xstate_info {
    size: 0,
    xbuf_offset: 0,
};

/* The helpers for managing XSAVE buffer and tile states: */

static mut stashed_xsave: *mut xsave_buffer = ptr::null_mut();

unsafe fn init_stashed_xsave() {
    unsafe {
        stashed_xsave = alloc_xbuf();
        if stashed_xsave.is_null() {
            fatal_error!("failed to allocate stashed_xsave\n");
        }
        clear_xstate_header(stashed_xsave);
    }
}

unsafe fn free_stashed_xsave() {
    unsafe {
        free(stashed_xsave as *mut c_void);
    }
}

/* Work around printf() being unsafe in signals: */
const SIGNAL_BUF_LEN: usize = 1000;
static mut signal_message_buffer: [c_char; SIGNAL_BUF_LEN] = [0; SIGNAL_BUF_LEN];

unsafe fn sig_print(msg: *const c_char) {
    unsafe {
        let left = SIGNAL_BUF_LEN - strlen(core::ptr::addr_of!(signal_message_buffer) as *const c_char) - 1;

        strncat(
            core::ptr::addr_of_mut!(signal_message_buffer) as *mut c_char,
            msg,
            left,
        );
    }
}

static mut noperm_signaled: bool = false;
static mut noperm_errs: c_int = 0;
/*
 * Signal handler for when AMX is used but
 * permission has not been obtained.
 */
unsafe extern "C" fn handle_noperm(sig: c_int, si: *mut libc::siginfo_t, ctx_void: *mut c_void) {
    let ctx = ctx_void as *mut libc::ucontext_t;
    let xbuf: *mut c_void;
    let sw_bytes: *mut _fpx_sw_bytes;
    let features: u64;

    unsafe {
        let _ = sig;
        xbuf = (*ctx).uc_mcontext.fpregs as *mut c_void;

        /* Reset the signal message buffer: */
        signal_message_buffer[0] = 0;
        sig_print(c_str!("\tAt SIGILL handler,\n"));

        if (*si).si_code != libc::ILL_ILLOPC {
            noperm_errs += 1;
            sig_print(c_str!("[FAIL]\tInvalid signal code.\n"));
        } else {
            sig_print(c_str!("[OK]\tValid signal code (ILL_ILLOPC).\n"));
        }

        sw_bytes = get_fpx_sw_bytes(xbuf);
        /*
         * Without permission, the signal XSAVE buffer should not
         * have room for AMX register state (aka. xtiledata).
         * Check that the size does not overlap with where xtiledata
         * will reside.
         *
         * This also implies that no state components *PAST*
         * XTILEDATA (features >=19) can be present in the buffer.
         */
        if (*sw_bytes).xstate_size <= xtiledata.xbuf_offset as u32 {
            sig_print(c_str!("[OK]\tValid xstate size\n"));
        } else {
            noperm_errs += 1;
            sig_print(c_str!("[FAIL]\tInvalid xstate size\n"));
        }

        features = get_fpx_sw_bytes_features(xbuf);
        /*
         * Without permission, the XTILEDATA feature
         * bit should not be set.
         */
        if (features & XFEATURE_MASK_XTILEDATA) == 0 {
            sig_print(c_str!("[OK]\tValid xstate mask\n"));
        } else {
            noperm_errs += 1;
            sig_print(c_str!("[FAIL]\tInvalid xstate mask\n"));
        }

        noperm_signaled = true;
        (*ctx).uc_mcontext.gregs[libc::REG_RIP as usize] += 3; /* Skip the faulting XRSTOR */
    }
}

/* Return true if XRSTOR is successful; otherwise, false. */
unsafe fn xrstor_safe(xbuf: *mut xsave_buffer, mask: u64) -> bool {
    unsafe {
        noperm_signaled = false;
        xrstor(xbuf, mask);

        /* Print any messages produced by the signal code: */
        printf(c_str!("%s"), core::ptr::addr_of!(signal_message_buffer) as *const c_char);
        /*
         * Reset the buffer to make sure any future printing
         * only outputs new messages:
         */
        signal_message_buffer[0] = 0;

        if noperm_errs != 0 {
            fatal_error!("saw %d errors in noperm signal handler\n", noperm_errs);
        }

        !noperm_signaled
    }
}

/*
 * Use XRSTOR to populate the XTILEDATA registers with
 * random data.
 *
 * Return true if successful; otherwise, false.
 */
unsafe fn load_rand_tiledata(xbuf: *mut xsave_buffer) -> bool {
    unsafe {
        clear_xstate_header(xbuf);
        set_xstatebv(xbuf, XFEATURE_MASK_XTILEDATA);
        set_rand_data(core::ptr::addr_of_mut!(xtiledata), xbuf);
        xrstor_safe(xbuf, XFEATURE_MASK_XTILEDATA)
    }
}

#[repr(C)]
#[derive(PartialEq, Eq, Copy, Clone)]
enum expected_result {
    FAIL_EXPECTED,
    SUCCESS_EXPECTED,
}

/* arch_prctl() and sigaltstack() test */

const ARCH_GET_XCOMP_SUPP: c_ulong = 0x1021;
const ARCH_GET_XCOMP_PERM: c_ulong = 0x1022;
const ARCH_REQ_XCOMP_PERM: c_ulong = 0x1023;

unsafe fn req_xtiledata_perm() {
    unsafe {
        syscall(
            libc::SYS_arch_prctl as c_long,
            ARCH_REQ_XCOMP_PERM,
            XFEATURE_XTILEDATA,
        );
    }
}

unsafe fn validate_req_xcomp_perm(exp: expected_result) {
    let mut bitmask: c_ulong = 0;
    let expected_bitmask: c_ulong;
    let mut rc: c_long;

    unsafe {
        rc = syscall(
            libc::SYS_arch_prctl as c_long,
            ARCH_GET_XCOMP_PERM,
            &mut bitmask as *mut c_ulong,
        );
        if rc != 0 {
            fatal_error!("prctl(ARCH_GET_XCOMP_PERM) error: %ld", rc);
        } else if (bitmask & XFEATURE_MASK_XTILECFG as c_ulong) == 0 {
            fatal_error!("ARCH_GET_XCOMP_PERM returns XFEATURE_XTILECFG off.");
        }

        rc = syscall(
            libc::SYS_arch_prctl as c_long,
            ARCH_REQ_XCOMP_PERM,
            XFEATURE_XTILEDATA,
        );
        if exp == expected_result::FAIL_EXPECTED {
            if rc != 0 {
                printf(c_str!("[OK]\tARCH_REQ_XCOMP_PERM saw expected failure..\n"));
                return;
            }

            fatal_error!("ARCH_REQ_XCOMP_PERM saw unexpected success.\n");
        } else if rc != 0 {
            fatal_error!("ARCH_REQ_XCOMP_PERM saw unexpected failure.\n");
        }

        expected_bitmask = bitmask | XFEATURE_MASK_XTILEDATA as c_ulong;

        rc = syscall(
            libc::SYS_arch_prctl as c_long,
            ARCH_GET_XCOMP_PERM,
            &mut bitmask as *mut c_ulong,
        );
        if rc != 0 {
            fatal_error!("prctl(ARCH_GET_XCOMP_PERM) error: %ld", rc);
        } else if bitmask != expected_bitmask {
            fatal_error!(
                "ARCH_REQ_XCOMP_PERM set a wrong bitmask: %lx, expected: %lx.\n",
                bitmask,
                expected_bitmask
            );
        } else {
            printf(c_str!("\tARCH_REQ_XCOMP_PERM is successful.\n"));
        }
    }
}

unsafe fn validate_xcomp_perm(exp: expected_result) {
    unsafe {
        let load_success = load_rand_tiledata(stashed_xsave);

        if exp == expected_result::FAIL_EXPECTED {
            if load_success {
                noperm_errs += 1;
                printf(c_str!("[FAIL]\tLoad tiledata succeeded.\n"));
            } else {
                printf(c_str!("[OK]\tLoad tiledata failed.\n"));
            }
        } else if exp == expected_result::SUCCESS_EXPECTED {
            if load_success {
                printf(c_str!("[OK]\tLoad tiledata succeeded.\n"));
            } else {
                noperm_errs += 1;
                printf(c_str!("[FAIL]\tLoad tiledata failed.\n"));
            }
        }
    }
}

const AT_MINSIGSTKSZ: c_ulong = 51;

unsafe fn alloc_altstack(size: c_uint) -> *mut c_void {
    let altstack: *mut c_void;

    unsafe {
        altstack = mmap(
            ptr::null_mut(),
            size as usize,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_STACK,
            -1,
            0,
        );

        if altstack == libc::MAP_FAILED {
            fatal_error!("mmap() for altstack");
        }

        altstack
    }
}

type c_uint = libc::c_uint;

unsafe fn setup_altstack(addr: *mut c_void, size: c_ulong, exp: expected_result) {
    let mut ss: libc::stack_t;
    let rc: c_int;

    unsafe {
        ss = mem::zeroed();
        memset(
            &mut ss as *mut libc::stack_t as *mut c_void,
            0,
            mem::size_of_val(&ss),
        );
        ss.ss_size = size as usize;
        ss.ss_sp = addr;

        rc = sigaltstack(&ss as *const libc::stack_t, ptr::null_mut());

        if exp == expected_result::FAIL_EXPECTED {
            if rc != 0 {
                printf(c_str!("[OK]\tsigaltstack() failed.\n"));
            } else {
                fatal_error!("sigaltstack() succeeded unexpectedly.\n");
            }
        } else if rc != 0 {
            fatal_error!("sigaltstack()");
        }
    }
}

unsafe fn test_dynamic_sigaltstack() {
    let small_size: c_uint;
    let enough_size: c_uint;
    let minsigstksz: c_ulong;
    let altstack: *mut c_void;

    unsafe {
        minsigstksz = getauxval(AT_MINSIGSTKSZ);
        printf(c_str!("\tAT_MINSIGSTKSZ = %lu\n"), minsigstksz);
        /*
         * getauxval() itself can return 0 for failure or
         * success.  But, in this case, AT_MINSIGSTKSZ
         * will always return a >=0 value if implemented.
         * Just check for 0.
         */
        if minsigstksz == 0 {
            printf(c_str!("no support for AT_MINSIGSTKSZ, skipping sigaltstack tests\n"));
            return;
        }

        enough_size = (minsigstksz * 2) as c_uint;

        altstack = alloc_altstack(enough_size);
        printf(c_str!("\tAllocate memory for altstack (%u bytes).\n"), enough_size);

        /*
         * Try setup_altstack() with a size which can not fit
         * XTILEDATA.  ARCH_REQ_XCOMP_PERM should fail.
         */
        small_size = (minsigstksz - xtiledata.size as c_ulong) as c_uint;
        printf(c_str!("\tAfter sigaltstack() with small size (%u bytes).\n"), small_size);
        setup_altstack(altstack, small_size as c_ulong, expected_result::SUCCESS_EXPECTED);
        validate_req_xcomp_perm(expected_result::FAIL_EXPECTED);

        /*
         * Try setup_altstack() with a size derived from
         * AT_MINSIGSTKSZ.  It should be more than large enough
         * and thus ARCH_REQ_XCOMP_PERM should succeed.
         */
        printf(c_str!("\tAfter sigaltstack() with enough size (%u bytes).\n"), enough_size);
        setup_altstack(altstack, enough_size as c_ulong, expected_result::SUCCESS_EXPECTED);
        validate_req_xcomp_perm(expected_result::SUCCESS_EXPECTED);

        /*
         * Try to coerce setup_altstack() to again accept a
         * too-small altstack.  This ensures that big-enough
         * sigaltstacks can not shrink to a too-small value
         * once XTILEDATA permission is established.
         */
        printf(c_str!("\tThen, sigaltstack() with small size (%u bytes).\n"), small_size);
        setup_altstack(altstack, small_size as c_ulong, expected_result::FAIL_EXPECTED);
    }
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn test_dynamic_state() {
    let parent: PidT;
    let child: PidT;
    let grandchild: PidT;

    unsafe {
        parent = fork();
        if parent < 0 {
            /* fork() failed */
            fatal_error!("fork");
        } else if parent > 0 {
            let mut status: c_int = 0;
            /* fork() succeeded.  Now in the parent. */

            wait(&mut status as *mut c_int);
            if !wifexited(status) || wexitstatus(status) != 0 {
                fatal_error!("arch_prctl test parent exit");
            }
            return;
        }
        /* fork() succeeded.  Now in the child . */

        printf(c_str!("[RUN]\tCheck ARCH_REQ_XCOMP_PERM around process fork() and sigaltack() test.\n"));

        printf(c_str!("\tFork a child.\n"));
        child = fork();
        if child < 0 {
            fatal_error!("fork");
        } else if child > 0 {
            let mut status: c_int = 0;

            wait(&mut status as *mut c_int);
            if !wifexited(status) || wexitstatus(status) != 0 {
                fatal_error!("arch_prctl test child exit");
            }
            _exit(0);
        }

        /*
         * The permission request should fail without an
         * XTILEDATA-compatible signal stack
         */
        printf(c_str!("\tTest XCOMP_PERM at child.\n"));
        validate_xcomp_perm(expected_result::FAIL_EXPECTED);

        /*
         * Set up an XTILEDATA-compatible signal stack and
         * also obtain permission to populate XTILEDATA.
         */
        printf(c_str!("\tTest dynamic sigaltstack at child:\n"));
        test_dynamic_sigaltstack();

        /* Ensure that XTILEDATA can be populated. */
        printf(c_str!("\tTest XCOMP_PERM again at child.\n"));
        validate_xcomp_perm(expected_result::SUCCESS_EXPECTED);

        printf(c_str!("\tFork a grandchild.\n"));
        grandchild = fork();
        if grandchild < 0 {
            /* fork() failed */
            fatal_error!("fork");
        } else if grandchild == 0 {
            /* fork() succeeded.  Now in the (grand)child. */
            printf(c_str!("\tTest XCOMP_PERM at grandchild.\n"));

            /*
             * Ensure that the grandchild inherited
             * permission and a compatible sigaltstack:
             */
            validate_xcomp_perm(expected_result::SUCCESS_EXPECTED);
        } else {
            let mut status: c_int = 0;
            /* fork() succeeded.  Now in the parent. */

            wait(&mut status as *mut c_int);
            if !wifexited(status) || wexitstatus(status) != 0 {
                fatal_error!("fork test grandchild");
            }
        }

        _exit(0);
    }
}

unsafe fn __compare_tiledata_state(xbuf1: *mut xsave_buffer, xbuf2: *mut xsave_buffer) -> c_int {
    unsafe {
        memcmp(
            ((*xbuf1).bytes.as_ptr()).add(xtiledata.xbuf_offset as usize) as *const c_void,
            ((*xbuf2).bytes.as_ptr()).add(xtiledata.xbuf_offset as usize) as *const c_void,
            xtiledata.size as usize,
        )
    }
}

/*
 * Save current register state and compare it to @xbuf1.'
 *
 * Returns false if @xbuf1 matches the registers.
 * Returns true  if @xbuf1 differs from the registers.
 */
unsafe fn __validate_tiledata_regs(xbuf1: *mut xsave_buffer) -> bool {
    let xbuf2: *mut xsave_buffer;
    let ret: c_int;

    unsafe {
        xbuf2 = alloc_xbuf();
        if xbuf2.is_null() {
            fatal_error!("failed to allocate XSAVE buffer\n");
        }

        xsave(xbuf2, XFEATURE_MASK_XTILEDATA);
        ret = __compare_tiledata_state(xbuf1, xbuf2);

        free(xbuf2 as *mut c_void);

        if ret == 0 {
            return false;
        }
        true
    }
}

unsafe fn validate_tiledata_regs_changed(xbuf: *mut xsave_buffer) {
    unsafe {
        let ret = __validate_tiledata_regs(xbuf);

        if ret == false {
            fatal_error!("TILEDATA registers did not change");
        }
    }
}

/* tiledata inheritance test */

unsafe fn test_fork() {
    let child: PidT;
    let grandchild: PidT;

    unsafe {
        child = fork();
        if child < 0 {
            /* fork() failed */
            fatal_error!("fork");
        } else if child > 0 {
            /* fork() succeeded.  Now in the parent. */
            let mut status: c_int = 0;

            wait(&mut status as *mut c_int);
            if !wifexited(status) || wexitstatus(status) != 0 {
                fatal_error!("fork test child");
            }
            return;
        }
        /* fork() succeeded.  Now in the child. */
        printf(c_str!("[RUN]\tCheck tile data inheritance.\n\tBefore fork(), load tiledata\n"));

        load_rand_tiledata(stashed_xsave);

        grandchild = fork();
        if grandchild < 0 {
            /* fork() failed */
            fatal_error!("fork");
        } else if grandchild > 0 {
            /* fork() succeeded.  Still in the first child. */
            let mut status: c_int = 0;

            wait(&mut status as *mut c_int);
            if !wifexited(status) || wexitstatus(status) != 0 {
                fatal_error!("fork test grand child");
            }
            _exit(0);
        }
        /* fork() succeeded.  Now in the (grand)child. */

        /*
         * TILEDATA registers are not preserved across fork().
         * Ensure that their value has changed:
         */
        validate_tiledata_regs_changed(stashed_xsave);

        _exit(0);
    }
}

const KSFT_SKIP: c_int = 4;

pub unsafe fn main() -> c_int {
    let mut features: c_ulong = 0;
    let rc: c_long;

    unsafe {
        rc = syscall(
            libc::SYS_arch_prctl as c_long,
            ARCH_GET_XCOMP_SUPP,
            &mut features as *mut c_ulong,
        );
        if rc != 0 || (features & XFEATURE_MASK_XTILE as c_ulong) != XFEATURE_MASK_XTILE as c_ulong {
            ksft_print_msg(c_str!("no AMX support\n"));
            return KSFT_SKIP;
        }

        xtiledata = get_xstate_info(XFEATURE_XTILEDATA);
        if xtiledata.size == 0 || xtiledata.xbuf_offset == 0 {
            fatal_error!(
                "xstate cpuid: invalid tile data size/offset: %d/%d",
                xtiledata.size,
                xtiledata.xbuf_offset
            );
        }

        init_stashed_xsave();
        sethandler(libc::SIGILL, handle_noperm, 0);

        test_dynamic_state();

        /* Request permission for the following tests */
        req_xtiledata_perm();

        test_fork();

        /*
         * Perform generic xstate tests for context switching, ptrace,
         * and signal.
         */
        test_xstate(XFEATURE_XTILEDATA);

        clearhandler(libc::SIGILL);
        free_stashed_xsave();

        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
