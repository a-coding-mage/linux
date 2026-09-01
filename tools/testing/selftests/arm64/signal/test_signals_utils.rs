// SPDX-License-Identifier: GPL-2.0
/* Copyright (C) 2019 ARM Limited */

/*
 * Translated from C implementation source.
 * Original includes:
 * <stdio.h>, <stdlib.h>, <signal.h>, <string.h>, <unistd.h>, <assert.h>,
 * <sys/auxv.h>, <linux/auxvec.h>, <ucontext.h>, <asm/unistd.h>,
 * <kselftest.h>, "test_signals.h", "test_signals_utils.h",
 * "testcases/testcases.h"
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type size_t = usize;
type bool_t = bool;

const SIGTRAP: c_int = 5;
const SIGALRM: c_int = 14;
const SIGSEGV: c_int = 11;
const SIGRTMIN: c_int = 34;
const SIGRTMAX: c_int = 64;
const SIG_UNBLOCK: c_int = 1;
const SA_SIGINFO: c_int = 4;
const SA_RESTART: c_int = 0x10000000;
const SEGV_ACCERR: c_int = 2;
const AT_HWCAP: c_ulong = 16;
const AT_HWCAP2: c_ulong = 26;
const AT_MINSIGSTKSZ: c_ulong = 51;
const MINSIGSTKSZ: c_ulong = 5120;

const KSFT_PASS: c_int = 0;
const KSFT_FAIL: c_int = 1;
const KSFT_SKIP: c_int = 4;

const FMAX_END: usize = 7;
const FEAT_SSBS: c_ulong = 1 << 0;
const FEAT_SVE: c_ulong = 1 << 1;
const FEAT_SME: c_ulong = 1 << 2;
const FEAT_SME_FA64: c_ulong = 1 << 3;
const FEAT_SME2: c_ulong = 1 << 4;
const FEAT_GCS: c_ulong = 1 << 5;
const FEAT_POE: c_ulong = 1 << 6;

const HWCAP_SSBS: c_ulong = 1 << 28;
const HWCAP_SVE: c_ulong = 1 << 22;
const HWCAP_GCS: c_ulong = 1 << 32;
const HWCAP2_SME: c_ulong = 1 << 23;
const HWCAP2_SME_FA64: c_ulong = 1 << 28;
const HWCAP2_SME2: c_ulong = 1 << 37;
const HWCAP2_POE: c_ulong = 1 << 42;

const EXTRA_MAGIC: u32 = 0x45585401;
const MAX_FEATS_SZ: usize = 128;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
pub struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
    pub _pad: [c_int; 29],
}

impl siginfo_t {
    unsafe fn si_addr(&self) -> *mut c_void {
        let p = self as *const siginfo_t as *const u8;
        *(p.add(16) as *const *mut c_void)
    }
}

#[repr(C)]
pub struct sigaction {
    pub sa_sigaction: extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    pub sa_mask: sigset_t,
    pub sa_flags: c_int,
    pub sa_restorer: Option<extern "C" fn()>,
}

#[repr(C)]
pub struct mcontext_t {
    pub fault_address: u64,
    pub regs: [u64; 31],
    pub sp: u64,
    pub pc: u64,
    pub pstate: u64,
    pub __reserved: [u8; 4096],
}

#[repr(C)]
pub struct ucontext_t {
    pub uc_flags: c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_sigmask: sigset_t,
    pub uc_mcontext: mcontext_t,
}

#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut c_void,
    pub ss_flags: c_int,
    pub ss_size: size_t,
}

#[repr(C)]
pub struct _aarch64_ctx {
    pub magic: u32,
    pub size: u32,
}

#[repr(C)]
pub struct extra_context {
    pub head: _aarch64_ctx,
    pub datap: u64,
    pub size: u32,
    pub __reserved: [u32; 3],
}

type SignalRun = unsafe extern "C" fn(*mut tdescr, *mut siginfo_t, *mut c_void) -> c_int;
type TestFn = unsafe extern "C" fn(*mut tdescr) -> c_int;

#[repr(C)]
pub struct tdescr {
    pub name: *const c_char,
    pub sig_trig: c_int,
    pub sig_ok: c_int,
    pub sig_unsupp: c_int,
    pub sig_ok_code: c_int,
    pub sa_flags: c_int,
    pub timeout: c_uint,
    pub initialized: c_int,
    pub triggered: c_int,
    pub pass: c_int,
    pub result: c_int,
    pub sanity_disabled: c_int,
    pub token: *mut c_void,
    pub live_uc: *mut ucontext_t,
    pub live_uc_valid: c_int,
    pub live_sz: size_t,
    pub minsigstksz: c_ulong,
    pub feats_required: c_ulong,
    pub feats_incompatible: c_ulong,
    pub feats_supported: c_ulong,
    pub run: Option<SignalRun>,
    pub init: Option<TestFn>,
    pub setup: Option<TestFn>,
    pub trigger: Option<TestFn>,
    pub check_result: Option<TestFn>,
    pub cleanup: Option<TestFn>,
}

type c_uint = u32;

unsafe extern "C" {
    static mut current: *mut tdescr;
    static mut stderr: *mut FILE;
    static mut stdout: *mut FILE;

    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn abort() -> !;
    fn strlen(s: *const c_char) -> size_t;
    fn strncat(dest: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaddset(set: *mut sigset_t, signum: c_int) -> c_int;
    fn sigprocmask(how: c_int, set: *const sigset_t, oldset: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn raise(sig: c_int) -> c_int;
    fn alarm(seconds: c_uint) -> c_uint;
    fn getauxval(type_: c_ulong) -> c_ulong;

    fn feats_ok(td: *mut tdescr) -> bool_t;
    fn get_header(
        head: *mut _aarch64_ctx,
        magic: u32,
        size: size_t,
        offset: *mut size_t,
    ) -> *mut _aarch64_ctx;
    fn ASSERT_GOOD_CONTEXT(uc: *mut ucontext_t);
}

static mut sig_copyctx: c_int = SIGTRAP;

static feats_names: [*const c_char; FMAX_END] = [
    b" SSBS \0".as_ptr() as *const c_char,
    b" SVE \0".as_ptr() as *const c_char,
    b" SME \0".as_ptr() as *const c_char,
    b" FA64 \0".as_ptr() as *const c_char,
    b" SME2 \0".as_ptr() as *const c_char,
    b" GCS \0".as_ptr() as *const c_char,
    b" POE \0".as_ptr() as *const c_char,
];

static mut feats_string: [c_char; MAX_FEATS_SZ] = [0; MAX_FEATS_SZ];

unsafe fn feats_to_string(feats: c_ulong) -> *mut c_char {
    let mut flen: size_t = MAX_FEATS_SZ - 1;

    feats_string[0] = b'\0' as c_char;

    for i in 0..FMAX_END {
        if feats & (1_u64 << i) as c_ulong != 0 {
            let tlen: size_t = strlen(feats_names[i]);

            assert!(flen > tlen);
            flen -= tlen;
            strncat(feats_string.as_mut_ptr(), feats_names[i], flen);
        }
    }

    feats_string.as_mut_ptr()
}

unsafe fn unblock_signal(signum: c_int) {
    let mut sset: sigset_t = core::mem::zeroed();

    sigemptyset(&mut sset);
    sigaddset(&mut sset, signum);
    sigprocmask(SIG_UNBLOCK, &sset, ptr::null_mut());
}

unsafe fn default_result(td: *mut tdescr, force_exit: bool) {
    if (*td).result == KSFT_SKIP {
        fprintf(stderr, c"==>> completed. SKIP.\n".as_ptr());
    } else if (*td).pass != 0 {
        fprintf(stderr, c"==>> completed. PASS(1)\n".as_ptr());
        (*td).result = KSFT_PASS;
    } else {
        fprintf(stdout, c"==>> completed. FAIL(0)\n".as_ptr());
        (*td).result = KSFT_FAIL;
    }

    if force_exit {
        exit((*td).result);
    }
}

/*
 * The following handle_signal_* helpers are used by main default_handler
 * and are meant to return true when signal is handled successfully:
 * when false is returned instead, it means that the signal was somehow
 * unexpected in that context and it was NOT handled; default_handler will
 * take care of such unexpected situations.
 */

unsafe fn handle_signal_unsupported(td: *mut tdescr, _si: *mut siginfo_t, uc: *mut c_void) -> bool {
    if feats_ok(td) {
        return false;
    }

    /* Mangling PC to avoid loops on original SIGILL */
    (*(uc as *mut ucontext_t)).uc_mcontext.pc += 4;

    if (*td).initialized == 0 {
        fprintf(stderr, c"Got SIG_UNSUPP @test_init. Ignore.\n".as_ptr());
    } else {
        fprintf(
            stderr,
            c"-- RX SIG_UNSUPP on unsupported feat...OK\n".as_ptr(),
        );
        (*td).pass = 1;
        default_result(current, true);
    }

    true
}

unsafe fn handle_signal_trigger(td: *mut tdescr, si: *mut siginfo_t, uc: *mut c_void) -> bool {
    (*td).triggered = 1;
    /* ->run was asserted NON-NULL in test_setup() already */
    ((*td).run.unwrap())(td, si, uc);

    true
}

unsafe fn handle_signal_ok(td: *mut tdescr, si: *mut siginfo_t, uc: *mut c_void) -> bool {
    /*
     * it's a bug in the test code when this assert fail:
     * if sig_trig was defined, it must have been used before getting here.
     */
    assert!((*td).sig_trig == 0 || (*td).triggered != 0);
    fprintf(
        stderr,
        c"SIG_OK -- SP:0x%llX  si_addr@:%p  si_code:%d  token@:%p  offset:%ld\n".as_ptr(),
        (*(uc as *mut ucontext_t)).uc_mcontext.sp,
        (*si).si_addr(),
        (*si).si_code,
        (*td).token,
        ((*td).token as isize).wrapping_sub((*si).si_addr() as isize),
    );
    /*
     * fake_sigreturn tests, which have sanity_enabled=1, set, at the very
     * last time, the token field to the SP address used to place the fake
     * sigframe: so token==0 means we never made it to the end,
     * segfaulting well-before, and the test is possibly broken.
     */
    if (*td).sanity_disabled == 0 && (*td).token.is_null() {
        fprintf(
            stdout,
            c"current->token ZEROED...test is probably broken!\n".as_ptr(),
        );
        abort();
    }
    if (*td).sig_ok_code != 0 {
        if (*si).si_code != (*td).sig_ok_code {
            fprintf(
                stdout,
                c"si_code is %d not %d\n".as_ptr(),
                (*si).si_code,
                (*td).sig_ok_code,
            );
            abort();
        }
    } else {
        /*
         * Trying to narrow down the SEGV to the ones
         * generated by Kernel itself via
         * arm64_notify_segfault(). This is a best-effort
         * check anyway, and the si_code check may need to
         * change if this aspect of the kernel ABI changes.
         */
        if (*td).sig_ok == SIGSEGV && (*si).si_code != SEGV_ACCERR {
            fprintf(
                stdout,
                c"si_code != SEGV_ACCERR...test is probably broken!\n".as_ptr(),
            );
            abort();
        }
    }
    (*td).pass = 1;
    /*
     * Some tests can lead to SEGV loops: in such a case we want to
     * terminate immediately exiting straight away; some others are not
     * supposed to outlive the signal handler code, due to the content of
     * the fake sigframe which caused the signal itself.
     */
    default_result(current, true);

    true
}

unsafe fn handle_signal_copyctx(td: *mut tdescr, _si: *mut siginfo_t, uc_in: *mut c_void) -> bool {
    let uc: *mut ucontext_t = uc_in as *mut ucontext_t;
    let mut head: *mut _aarch64_ctx;
    let mut extra: *mut extra_context;
    let copied_extra: *mut extra_context;
    let mut offset: size_t = 0;
    let mut to_copy: size_t;

    ASSERT_GOOD_CONTEXT(uc);

    /* Mangling PC to avoid loops on original BRK instr */
    (*uc).uc_mcontext.pc += 4;

    /*
     * Check for an preserve any extra data too with fixups.
     */
    head = (*uc).uc_mcontext.__reserved.as_mut_ptr() as *mut _aarch64_ctx;
    head = get_header(head, EXTRA_MAGIC, (*td).live_sz, &mut offset);
    if !head.is_null() {
        extra = head as *mut extra_context;

        /*
         * The extra buffer must be immediately after the
         * extra_context and a 16 byte terminator. Include it
         * in the copy, this was previously validated in
         * ASSERT_GOOD_CONTEXT().
         */
        to_copy = offset_of!(ucontext_t, uc_mcontext) + offset_of!(mcontext_t, __reserved);
        to_copy += offset + size_of::<extra_context>() + 16;
        to_copy += (*extra).size as size_t;
        copied_extra =
            (*(*td).live_uc).uc_mcontext.__reserved.as_mut_ptr().add(offset) as *mut extra_context;
    } else {
        copied_extra = ptr::null_mut();
        to_copy = size_of::<ucontext_t>();
    }

    if to_copy > (*td).live_sz {
        fprintf(
            stderr,
            c"Not enough space to grab context, %lu/%lu bytes\n".as_ptr(),
            (*td).live_sz,
            to_copy,
        );
        return false;
    }

    ptr::copy_nonoverlapping(uc as *const u8, (*td).live_uc as *mut u8, to_copy);

    /*
     * If there was any EXTRA_CONTEXT fix up the size to be the
     * struct extra_context and the following terminator record,
     * this means that the rest of the code does not need to have
     * special handling for the record and we don't need to fix up
     * datap for the new location.
     */
    if !copied_extra.is_null() {
        (*copied_extra).head.size = (size_of::<extra_context>() + 16) as u32;
    }

    (*td).live_uc_valid = 1;
    fprintf(
        stderr,
        c"%lu byte GOOD CONTEXT grabbed from sig_copyctx handler\n".as_ptr(),
        to_copy,
    );

    true
}

unsafe extern "C" fn default_handler(signum: c_int, si: *mut siginfo_t, uc: *mut c_void) {
    if (*current).sig_unsupp != 0
        && signum == (*current).sig_unsupp
        && handle_signal_unsupported(current, si, uc)
    {
        fprintf(stderr, c"Handled SIG_UNSUPP\n".as_ptr());
    } else if (*current).sig_trig != 0
        && signum == (*current).sig_trig
        && handle_signal_trigger(current, si, uc)
    {
        fprintf(stderr, c"Handled SIG_TRIG\n".as_ptr());
    } else if (*current).sig_ok != 0
        && signum == (*current).sig_ok
        && handle_signal_ok(current, si, uc)
    {
        fprintf(stderr, c"Handled SIG_OK\n".as_ptr());
    } else if signum == sig_copyctx
        && !(*current).live_uc.is_null()
        && handle_signal_copyctx(current, si, uc)
    {
        fprintf(stderr, c"Handled SIG_COPYCTX\n".as_ptr());
    } else {
        if signum == SIGALRM && (*current).timeout != 0 {
            fprintf(stderr, c"-- Timeout !\n".as_ptr());
        } else {
            fprintf(
                stderr,
                c"-- RX UNEXPECTED SIGNAL: %d code %d address %p\n".as_ptr(),
                signum,
                (*si).si_code,
                (*si).si_addr(),
            );
        }
        default_result(current, true);
    }
}

unsafe fn default_setup(td: *mut tdescr) -> c_int {
    let mut sa: sigaction = core::mem::zeroed();

    sa.sa_sigaction = default_handler;
    sa.sa_flags = SA_SIGINFO | SA_RESTART;
    sa.sa_flags |= (*td).sa_flags;
    sigemptyset(&mut sa.sa_mask);
    /* uncatchable signals naturally skipped ... */
    for sig in 1..32 {
        sigaction(sig, &sa, ptr::null_mut());
    }
    /*
     * RT Signals default disposition is Term but they cannot be
     * generated by the Kernel in response to our tests; so just catch
     * them all and report them as UNEXPECTED signals.
     */
    let mut sig = SIGRTMIN;
    while sig <= SIGRTMAX {
        sigaction(sig, &sa, ptr::null_mut());
        sig += 1;
    }

    /* just in case...unblock explicitly all we need */
    if (*td).sig_trig != 0 {
        unblock_signal((*td).sig_trig);
    }
    if (*td).sig_ok != 0 {
        unblock_signal((*td).sig_ok);
    }
    if (*td).sig_unsupp != 0 {
        unblock_signal((*td).sig_unsupp);
    }

    if (*td).timeout != 0 {
        unblock_signal(SIGALRM);
        alarm((*td).timeout);
    }
    fprintf(stderr, c"Registered handlers for all signals.\n".as_ptr());

    1
}

unsafe fn default_trigger(td: *mut tdescr) -> c_int {
    (raise((*td).sig_trig) == 0) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn test_init(td: *mut tdescr) -> c_int {
    if (*td).sig_trig == sig_copyctx {
        fprintf(
            stdout,
            c"Signal %d is RESERVED, cannot be used as a trigger. Aborting\n".as_ptr(),
            sig_copyctx,
        );
        return 0;
    }
    /* just in case */
    unblock_signal(sig_copyctx);

    (*td).minsigstksz = getauxval(AT_MINSIGSTKSZ);
    if (*td).minsigstksz == 0 {
        (*td).minsigstksz = MINSIGSTKSZ;
    }
    fprintf(
        stderr,
        c"Detected MINSTKSIGSZ:%d\n".as_ptr(),
        (*td).minsigstksz as c_int,
    );

    if (*td).feats_required != 0 || (*td).feats_incompatible != 0 {
        (*td).feats_supported = 0;
        /*
         * Checking for CPU required features using both the
         * auxval and the arm64 MRS Emulation to read sysregs.
         */
        if getauxval(AT_HWCAP) & HWCAP_SSBS != 0 {
            (*td).feats_supported |= FEAT_SSBS;
        }
        if getauxval(AT_HWCAP) & HWCAP_SVE != 0 {
            (*td).feats_supported |= FEAT_SVE;
        }
        if getauxval(AT_HWCAP2) & HWCAP2_SME != 0 {
            (*td).feats_supported |= FEAT_SME;
        }
        if getauxval(AT_HWCAP2) & HWCAP2_SME_FA64 != 0 {
            (*td).feats_supported |= FEAT_SME_FA64;
        }
        if getauxval(AT_HWCAP2) & HWCAP2_SME2 != 0 {
            (*td).feats_supported |= FEAT_SME2;
        }
        if getauxval(AT_HWCAP) & HWCAP_GCS != 0 {
            (*td).feats_supported |= FEAT_GCS;
        }
        if getauxval(AT_HWCAP2) & HWCAP2_POE != 0 {
            (*td).feats_supported |= FEAT_POE;
        }
        if feats_ok(td) {
            if (*td).feats_required & (*td).feats_supported != 0 {
                fprintf(
                    stderr,
                    c"Required Features: [%s] supported\n".as_ptr(),
                    feats_to_string((*td).feats_required & (*td).feats_supported),
                );
            }
            if ((*td).feats_incompatible & (*td).feats_supported) == 0 {
                fprintf(
                    stderr,
                    c"Incompatible Features: [%s] absent\n".as_ptr(),
                    feats_to_string((*td).feats_incompatible),
                );
            }
        } else {
            if ((*td).feats_required & (*td).feats_supported) != (*td).feats_supported {
                fprintf(
                    stderr,
                    c"Required Features: [%s] NOT supported\n".as_ptr(),
                    feats_to_string((*td).feats_required & !(*td).feats_supported),
                );
            }
            if (*td).feats_incompatible & (*td).feats_supported != 0 {
                fprintf(
                    stderr,
                    c"Incompatible Features: [%s] supported\n".as_ptr(),
                    feats_to_string((*td).feats_incompatible & !(*td).feats_supported),
                );
            }

            (*td).result = KSFT_SKIP;
            return 0;
        }
    }

    /* Perform test specific additional initialization */
    if (*td).init.is_some() && ((*td).init.unwrap())(td) == 0 {
        fprintf(stderr, c"FAILED Testcase initialization.\n".as_ptr());
        return 0;
    }
    (*td).initialized = 1;
    fprintf(stderr, c"Testcase initialized.\n".as_ptr());

    1
}

#[no_mangle]
pub unsafe extern "C" fn test_setup(td: *mut tdescr) -> c_int {
    /* assert core invariants symptom of a rotten testcase */
    assert!(!current.is_null());
    assert!(!td.is_null());
    assert!(!(*td).name.is_null());
    assert!((*td).run.is_some());

    /* Default result is FAIL if test setup fails */
    (*td).result = KSFT_FAIL;
    if (*td).setup.is_some() {
        ((*td).setup.unwrap())(td)
    } else {
        default_setup(td)
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_run(td: *mut tdescr) -> c_int {
    if (*td).trigger.is_some() {
        ((*td).trigger.unwrap())(td)
    } else if (*td).sig_trig != 0 {
        default_trigger(td)
    } else {
        ((*td).run.unwrap())(td, ptr::null_mut(), ptr::null_mut())
    }
}

#[no_mangle]
pub unsafe extern "C" fn test_result(td: *mut tdescr) {
    if (*td).initialized != 0 && (*td).result != KSFT_SKIP && (*td).check_result.is_some() {
        ((*td).check_result.unwrap())(td);
    }
    default_result(td, false);
}

#[no_mangle]
pub unsafe extern "C" fn test_cleanup(td: *mut tdescr) {
    if (*td).cleanup.is_some() {
        ((*td).cleanup.unwrap())(td);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
