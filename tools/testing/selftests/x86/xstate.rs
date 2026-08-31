// SPDX-License-Identifier: GPL-2.0

// Translated from C implementation source: testing/selftests/x86/xstate.c
// Original includes supplied declarations from elf.h, pthread.h, stdbool.h,
// asm/prctl.h, sys/ptrace.h, sys/syscall.h, sys/uio.h, sys/wait.h,
// helpers.h, and xstate.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

const SIGNAL_BUF_LEN: usize = 1000;

#[repr(C)]
pub struct xsave_buffer {
    pub header: [u8; 0],
    pub bytes: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xstate_info {
    pub name: *const c_char,
    pub mask: u64,
    pub size: c_uint,
    pub xbuf_offset: c_uint,
}

#[repr(C)]
pub struct _fpx_sw_bytes {
    pub magic1: u32,
    pub extended_size: u32,
    pub xfeatures: u64,
    pub xstate_size: u32,
    pub padding: [u32; 7],
}

#[repr(C)]
pub struct pthread_mutex_t {
    _private: [u8; 0],
}

pub type pthread_t = c_ulong;
pub type pid_t = c_int;
pub type size_t = usize;
pub type siginfo_t = c_void;

#[repr(C)]
pub struct cpu_set_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: size_t,
}

#[repr(C)]
pub struct mcontext_t {
    pub fpregs: *mut c_void,
}

#[repr(C)]
pub struct ucontext_t {
    pub uc_mcontext: mcontext_t,
}

#[repr(C)]
pub struct futex_info {
    pub iterations: c_uint,
    pub next: *mut futex_info,
    pub mutex: pthread_mutex_t,
    pub thread: pthread_t,
    pub valid: bool,
    pub nr: c_int,
}

extern "C" {
    static XFEATURE_YMM: c_int;
    static XFEATURE_OPMASK: c_int;
    static XFEATURE_ZMM_Hi256: c_int;
    static XFEATURE_Hi16_ZMM: c_int;
    static XFEATURE_XTILEDATA: c_int;
    static XFEATURE_APX: c_int;
    static FP_XSTATE_MAGIC1: u32;
    static FP_XSTATE_MAGIC2: u32;
    static PTRACE_GETREGSET: c_int;
    static PTRACE_SETREGSET: c_int;
    static PTRACE_TRACEME: c_int;
    static PTRACE_DETACH: c_int;
    static NT_X86_XSTATE: c_int;
    static SYS_arch_prctl: c_long;
    static ARCH_GET_XCOMP_SUPP: c_int;
    static SIGTRAP: c_int;
    static SIGUSR1: c_int;

    fn clear_xstate_header(xbuf: *mut xsave_buffer);
    fn set_xstatebv(xbuf: *mut xsave_buffer, mask: u64);
    fn set_rand_data(xstate: *mut xstate_info, xbuf: *mut xsave_buffer);
    fn xrstor(xbuf: *mut xsave_buffer, mask: u64);
    fn xsave(xbuf: *mut xsave_buffer, mask: u64);
    fn alloc_xbuf() -> *mut xsave_buffer;
    fn get_xbuf_size() -> u32;
    fn get_fpx_sw_bytes(xbuf: *mut c_void) -> *mut _fpx_sw_bytes;
    fn get_fpx_sw_bytes_features(xbuf: *mut c_void) -> u64;
    fn get_xstate_info(feature_num: u32) -> xstate_info;
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn sethandler(sig: c_int, handler: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void), flags: c_int);
    fn clearhandler(sig: c_int);

    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strncat(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn syscall(num: c_long, ...) -> c_long;
    fn fork() -> pid_t;
    fn wait(status: *mut c_int) -> pid_t;
    fn raise(sig: c_int) -> c_int;
    fn _exit(status: c_int) -> !;
    fn ptrace(request: c_int, ...) -> c_long;
    fn pthread_mutex_init(mutex: *mut pthread_mutex_t, attr: *const c_void) -> c_int;
    fn pthread_mutex_lock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_mutex_unlock(mutex: *mut pthread_mutex_t) -> c_int;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn sched_setaffinity(pid: pid_t, cpusetsize: size_t, mask: *const cpu_set_t) -> c_int;
    fn CPU_ZERO(set: *mut cpu_set_t);
    fn CPU_SET(cpu: c_int, set: *mut cpu_set_t);
    fn WSTOPSIG(status: c_int) -> c_int;
    fn WIFEXITED(status: c_int) -> c_int;
    fn WEXITSTATUS(status: c_int) -> c_int;
}

#[inline]
unsafe fn xgetbv(index: u32) -> u64 {
    let eax: u32;
    let edx: u32;

    core::arch::asm!("xgetbv", out("eax") eax, out("edx") edx, in("ecx") index, options(nostack));
    (eax as u64).wrapping_add((edx as u64) << 32)
}

#[inline]
unsafe fn get_xstatebv(xbuf: *mut xsave_buffer) -> u64 {
    *((&mut (*xbuf).header) as *mut [u8; 0] as *mut u64)
}

static mut XSTATE: xstate_info = xstate_info {
    name: core::ptr::null(),
    mask: 0,
    size: 0,
    xbuf_offset: 0,
};

#[inline]
unsafe fn xfeature_mask_test_supported() -> u64 {
    (1u64 << XFEATURE_YMM)
        | (1u64 << XFEATURE_OPMASK)
        | (1u64 << XFEATURE_ZMM_Hi256)
        | (1u64 << XFEATURE_Hi16_ZMM)
        | (1u64 << XFEATURE_XTILEDATA)
        | (1u64 << XFEATURE_APX)
}

#[inline]
unsafe fn load_rand_xstate(xstate: *mut xstate_info, xbuf: *mut xsave_buffer) {
    clear_xstate_header(xbuf);
    set_xstatebv(xbuf, (*xstate).mask);
    set_rand_data(xstate, xbuf);
    xrstor(xbuf, (*xstate).mask);
}

#[inline]
unsafe fn load_init_xstate(xstate: *mut xstate_info, xbuf: *mut xsave_buffer) {
    clear_xstate_header(xbuf);
    xrstor(xbuf, (*xstate).mask);
}

#[inline]
unsafe fn copy_xstate(xbuf_dst: *mut xsave_buffer, xbuf_src: *mut xsave_buffer) {
    memcpy(
        (*xbuf_dst).bytes.as_mut_ptr().add(XSTATE.xbuf_offset as usize) as *mut c_void,
        (*xbuf_src).bytes.as_ptr().add(XSTATE.xbuf_offset as usize) as *const c_void,
        XSTATE.size as size_t,
    );
}

#[inline]
unsafe fn validate_xstate_same(xbuf1: *mut xsave_buffer, xbuf2: *mut xsave_buffer) -> bool {
    let ret: c_int;

    ret = memcmp(
        (*xbuf1).bytes.as_ptr().add(XSTATE.xbuf_offset as usize) as *const c_void,
        (*xbuf2).bytes.as_ptr().add(XSTATE.xbuf_offset as usize) as *const c_void,
        XSTATE.size as size_t,
    );
    ret == 0
}

#[inline]
unsafe fn validate_xregs_same(xbuf1: *mut xsave_buffer) -> bool {
    let xbuf2: *mut xsave_buffer;
    let ret: bool;

    xbuf2 = alloc_xbuf();
    if xbuf2.is_null() {
        ksft_exit_fail_msg(c"failed to allocate XSAVE buffer\n".as_ptr());
    }

    xsave(xbuf2, XSTATE.mask);
    ret = validate_xstate_same(xbuf1, xbuf2);

    free(xbuf2 as *mut c_void);
    ret
}

/* Context switching test */

unsafe extern "C" fn check_xstate(info: *mut c_void) -> *mut c_void {
    let finfo: *mut futex_info = info as *mut futex_info;
    let xbuf: *mut xsave_buffer;
    let mut i: c_int;

    xbuf = alloc_xbuf();
    if xbuf.is_null() {
        ksft_exit_fail_msg(c"unable to allocate XSAVE buffer\n".as_ptr());
    }

    /*
     * Load random data into 'xbuf' and then restore it to the xstate
     * registers.
     */
    load_rand_xstate(&mut XSTATE, xbuf);
    (*finfo).valid = true;

    i = 0;
    while i < (*finfo).iterations as c_int {
        pthread_mutex_lock(&mut (*finfo).mutex);

        /*
         * Ensure the register values have not diverged from the
         * record. Then reload a new random value.  If it failed
         * ever before, skip it.
         */
        if (*finfo).valid {
            (*finfo).valid = validate_xregs_same(xbuf);
            load_rand_xstate(&mut XSTATE, xbuf);
        }

        /*
         * The last thread's last unlock will be for thread 0's
         * mutex. However, thread 0 will have already exited the
         * loop and the mutex will already be unlocked.
         *
         * Because this is not an ERRORCHECK mutex, that
         * inconsistency will be silently ignored.
         */
        pthread_mutex_unlock(&mut (*(*finfo).next).mutex);
        i += 1;
    }

    free(xbuf as *mut c_void);
    finfo as *mut c_void
}

unsafe fn create_threads(num_threads: u32, iterations: u32, finfo: *mut futex_info) {
    let mut i: c_int;

    i = 0;
    while i < num_threads as c_int {
        let next_nr: c_int;

        (*finfo.add(i as usize)).nr = i;
        (*finfo.add(i as usize)).iterations = iterations;

        /*
         * Thread 'i' will wait on this mutex to be unlocked.
         * Lock it immediately after initialization:
         */
        pthread_mutex_init(&mut (*finfo.add(i as usize)).mutex, core::ptr::null());
        pthread_mutex_lock(&mut (*finfo.add(i as usize)).mutex);

        next_nr = (i + 1) % num_threads as c_int;
        (*finfo.add(i as usize)).next = finfo.add(next_nr as usize);

        if pthread_create(
            &mut (*finfo.add(i as usize)).thread,
            core::ptr::null(),
            check_xstate,
            finfo.add(i as usize) as *mut c_void,
        ) != 0
        {
            ksft_exit_fail_msg(c"pthread_create() failed\n".as_ptr());
        }
        i += 1;
    }
}

unsafe fn checkout_threads(num_threads: u32, finfo: *mut futex_info) -> bool {
    let mut thread_retval: *mut c_void = core::ptr::null_mut();
    let mut valid: bool = true;
    let err: c_int;
    let mut i: c_int;

    i = 0;
    while i < num_threads as c_int {
        err = pthread_join((*finfo.add(i as usize)).thread, &mut thread_retval);
        if err != 0 {
            ksft_exit_fail_msg(
                c"pthread_join() failed for thread %d err: %d\n".as_ptr(),
                i,
                err,
            );
        }

        if thread_retval != finfo.add(i as usize) as *mut c_void {
            ksft_exit_fail_msg(
                c"unexpected thread retval for thread %d: %p\n".as_ptr(),
                i,
                thread_retval,
            );
        }

        valid &= (*finfo.add(i as usize)).valid;
        i += 1;
    }

    valid
}

unsafe fn affinitize_cpu0() {
    let mut cpuset: cpu_set_t = core::mem::zeroed();

    CPU_ZERO(&mut cpuset);
    CPU_SET(0, &mut cpuset);

    if sched_setaffinity(0, core::mem::size_of_val(&cpuset), &cpuset) != 0 {
        ksft_exit_fail_msg(c"sched_setaffinity to CPU 0 failed\n".as_ptr());
    }
}

unsafe fn test_context_switch(num_threads: u32, iterations: u32) {
    let finfo: *mut futex_info;

    /* Affinitize to one CPU to force context switches */
    affinitize_cpu0();

    printf(
        c"[RUN]\t%s: check context switches, %d iterations, %d threads.\n".as_ptr(),
        XSTATE.name,
        iterations,
        num_threads,
    );

    finfo = malloc(core::mem::size_of::<futex_info>() * num_threads as usize) as *mut futex_info;
    if finfo.is_null() {
        ksft_exit_fail_msg(c"unable allocate memory\n".as_ptr());
    }

    create_threads(num_threads, iterations, finfo);

    /*
     * This thread wakes up thread 0
     * Thread 0 will wake up 1
     * Thread 1 will wake up 2
     * ...
     * The last thread will wake up 0
     *
     * This will repeat for the configured
     * number of iterations.
     */
    pthread_mutex_unlock(&mut (*finfo.add(0)).mutex);

    /* Wait for all the threads to finish: */
    if checkout_threads(num_threads, finfo) {
        printf(c"[OK]\tNo incorrect case was found.\n".as_ptr());
    } else {
        printf(c"[FAIL]\tFailed with context switching test.\n".as_ptr());
    }

    free(finfo as *mut c_void);
}

/*
 * Ptrace test for the ABI format as described in arch/x86/include/asm/user.h
 */

/*
 * Make sure the ptracee has the expanded kernel buffer on the first use.
 * Then, initialize the state before performing the state injection from
 * the ptracer. For non-dynamic states, this is benign.
 */
#[inline]
unsafe fn ptracee_touch_xstate() {
    let xbuf: *mut xsave_buffer;

    xbuf = alloc_xbuf();

    load_rand_xstate(&mut XSTATE, xbuf);
    load_init_xstate(&mut XSTATE, xbuf);

    free(xbuf as *mut c_void);
}

/*
 * Ptracer injects the randomized xstate data. It also reads before and
 * after that, which will execute the kernel's state copy functions.
 */
unsafe fn ptracer_inject_xstate(target: pid_t) {
    let xbuf_size: u32 = get_xbuf_size();
    let xbuf1: *mut xsave_buffer;
    let xbuf2: *mut xsave_buffer;
    let mut iov: iovec;

    /*
     * Allocate buffers to keep data while ptracer can write the
     * other buffer
     */
    xbuf1 = alloc_xbuf();
    xbuf2 = alloc_xbuf();
    if xbuf1.is_null() || xbuf2.is_null() {
        ksft_exit_fail_msg(c"unable to allocate XSAVE buffer\n".as_ptr());
    }

    iov = iovec {
        iov_base: xbuf1 as *mut c_void,
        iov_len: xbuf_size as size_t,
    };

    if ptrace(PTRACE_GETREGSET, target, NT_X86_XSTATE as u32, &mut iov) != 0 {
        ksft_exit_fail_msg(c"PTRACE_GETREGSET failed\n".as_ptr());
    }

    printf(c"[RUN]\t%s: inject xstate via ptrace().\n".as_ptr(), XSTATE.name);

    load_rand_xstate(&mut XSTATE, xbuf1);
    copy_xstate(xbuf2, xbuf1);

    if ptrace(PTRACE_SETREGSET, target, NT_X86_XSTATE as u32, &mut iov) != 0 {
        ksft_exit_fail_msg(c"PTRACE_SETREGSET failed\n".as_ptr());
    }

    if ptrace(PTRACE_GETREGSET, target, NT_X86_XSTATE as u32, &mut iov) != 0 {
        ksft_exit_fail_msg(c"PTRACE_GETREGSET failed\n".as_ptr());
    }

    if *(get_fpx_sw_bytes(xbuf1 as *mut c_void) as *mut u64) == xgetbv(0) {
        printf(c"[OK]\t'xfeatures' in SW reserved area was correctly written\n".as_ptr());
    } else {
        printf(c"[FAIL]\t'xfeatures' in SW reserved area was not correctly written\n".as_ptr());
    }

    if validate_xstate_same(xbuf2, xbuf1) {
        printf(c"[OK]\txstate was correctly updated.\n".as_ptr());
    } else {
        printf(c"[FAIL]\txstate was not correctly updated.\n".as_ptr());
    }

    free(xbuf1 as *mut c_void);
    free(xbuf2 as *mut c_void);
}

unsafe fn test_ptrace() {
    let child: pid_t;
    let mut status: c_int = 0;

    child = fork();
    if child < 0 {
        ksft_exit_fail_msg(c"fork() failed\n".as_ptr());
    } else if child == 0 {
        if ptrace(
            PTRACE_TRACEME,
            0,
            core::ptr::null_mut::<c_void>(),
            core::ptr::null_mut::<c_void>(),
        ) != 0
        {
            ksft_exit_fail_msg(c"PTRACE_TRACEME failed\n".as_ptr());
        }

        ptracee_touch_xstate();

        raise(SIGTRAP);
        _exit(0);
    }

    loop {
        wait(&mut status);
        if WSTOPSIG(status) == SIGTRAP {
            break;
        }
    }

    ptracer_inject_xstate(child);

    ptrace(
        PTRACE_DETACH,
        child,
        core::ptr::null_mut::<c_void>(),
        core::ptr::null_mut::<c_void>(),
    );
    wait(&mut status);
    if WIFEXITED(status) == 0 || WEXITSTATUS(status) != 0 {
        ksft_exit_fail_msg(c"ptracee exit error\n".as_ptr());
    }
}

/*
 * Test signal delivery for the ABI compatibility.
 * See the ABI format: arch/x86/include/uapi/asm/sigcontext.h
 */

/*
 * Avoid using printf() in signal handlers as it is not
 * async-signal-safe.
 */
static mut SIGNAL_MESSAGE_BUFFER: [c_char; SIGNAL_BUF_LEN] = [0; SIGNAL_BUF_LEN];

unsafe fn sig_print(msg: *mut c_char) {
    let left: c_int =
        SIGNAL_BUF_LEN as c_int - strlen(SIGNAL_MESSAGE_BUFFER.as_ptr()) as c_int - 1;

    strncat(SIGNAL_MESSAGE_BUFFER.as_mut_ptr(), msg, left as size_t);
}

static mut STASHED_XBUF: *mut xsave_buffer = core::ptr::null_mut();

unsafe extern "C" fn validate_sigfpstate(_sig: c_int, _si: *mut siginfo_t, ctx_void: *mut c_void) {
    let ctx: *mut ucontext_t = ctx_void as *mut ucontext_t;
    let xbuf: *mut c_void = (*ctx).uc_mcontext.fpregs;
    let sw_bytes: *mut _fpx_sw_bytes;
    let magic2: u32;

    /* Reset the signal message buffer: */
    SIGNAL_MESSAGE_BUFFER[0] = b'\0' as c_char;

    sw_bytes = get_fpx_sw_bytes(xbuf);
    if (*sw_bytes).magic1 == FP_XSTATE_MAGIC1 {
        sig_print(c"[OK]\t'magic1' is valid\n".as_ptr() as *mut c_char);
    } else {
        sig_print(c"[FAIL]\t'magic1' is not valid\n".as_ptr() as *mut c_char);
    }

    if (get_fpx_sw_bytes_features(xbuf) & XSTATE.mask) != 0 {
        sig_print(c"[OK]\t'xfeatures' in SW reserved area is valid\n".as_ptr() as *mut c_char);
    } else {
        sig_print(c"[FAIL]\t'xfeatures' in SW reserved area is not valid\n".as_ptr() as *mut c_char);
    }

    if (get_xstatebv(xbuf as *mut xsave_buffer) & XSTATE.mask) != 0 {
        sig_print(c"[OK]\t'xfeatures' in XSAVE header is valid\n".as_ptr() as *mut c_char);
    } else {
        sig_print(c"[FAIL]\t'xfeatures' in XSAVE header is not valid\n".as_ptr() as *mut c_char);
    }

    if validate_xstate_same(STASHED_XBUF, xbuf as *mut xsave_buffer) {
        sig_print(c"[OK]\txstate delivery was successful\n".as_ptr() as *mut c_char);
    } else {
        sig_print(c"[FAIL]\txstate delivery was not successful\n".as_ptr() as *mut c_char);
    }

    magic2 = *((xbuf as *mut u8).add((*sw_bytes).xstate_size as usize) as *mut u32);
    if magic2 == FP_XSTATE_MAGIC2 {
        sig_print(c"[OK]\t'magic2' is valid\n".as_ptr() as *mut c_char);
    } else {
        sig_print(c"[FAIL]\t'magic2' is not valid\n".as_ptr() as *mut c_char);
    }

    set_rand_data(&mut XSTATE, xbuf as *mut xsave_buffer);
    copy_xstate(STASHED_XBUF, xbuf as *mut xsave_buffer);
}

unsafe fn test_signal() {
    let valid_xstate: bool;

    /*
     * The signal handler will access this to verify xstate context
     * preservation.
     */
    STASHED_XBUF = alloc_xbuf();
    if STASHED_XBUF.is_null() {
        ksft_exit_fail_msg(c"unable to allocate XSAVE buffer\n".as_ptr());
    }

    printf(c"[RUN]\t%s: load xstate and raise SIGUSR1\n".as_ptr(), XSTATE.name);

    sethandler(SIGUSR1, validate_sigfpstate, 0);

    load_rand_xstate(&mut XSTATE, STASHED_XBUF);

    raise(SIGUSR1);

    /*
     * Immediately record the test result, deferring printf() to
     * prevent unintended state contamination by that.
     */
    valid_xstate = validate_xregs_same(STASHED_XBUF);
    printf(c"%s".as_ptr(), SIGNAL_MESSAGE_BUFFER.as_ptr());

    printf(
        c"[RUN]\t%s: load new xstate from sighandler and check it after sigreturn\n".as_ptr(),
        XSTATE.name,
    );

    if valid_xstate {
        printf(c"[OK]\txstate was restored correctly\n".as_ptr());
    } else {
        printf(c"[FAIL]\txstate restoration failed\n".as_ptr());
    }

    clearhandler(SIGUSR1);
    free(STASHED_XBUF as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn test_xstate(feature_num: u32) {
    let ctxtsw_num_threads: c_uint = 5;
    let ctxtsw_iterations: c_uint = 10;
    let mut features: c_ulong = 0;
    let rc: c_long;

    if (xfeature_mask_test_supported() & (1u64 << feature_num)) == 0 {
        ksft_print_msg(
            c"The xstate test does not fully support the component %u, yet.\n".as_ptr(),
            feature_num,
        );
        return;
    }

    rc = syscall(SYS_arch_prctl, ARCH_GET_XCOMP_SUPP, &mut features);
    if rc != 0 || (features & (1u64 << feature_num) as c_ulong) == 0 {
        ksft_print_msg(
            c"The kernel does not support feature number: %u\n".as_ptr(),
            feature_num,
        );
        return;
    }

    XSTATE = get_xstate_info(feature_num);
    if XSTATE.size == 0 || XSTATE.xbuf_offset == 0 {
        ksft_exit_fail_msg(
            c"invalid state size/offset (%d/%d)\n".as_ptr(),
            XSTATE.size,
            XSTATE.xbuf_offset,
        );
    }

    test_context_switch(ctxtsw_num_threads, ctxtsw_iterations);
    test_ptrace();
    test_signal();
}
