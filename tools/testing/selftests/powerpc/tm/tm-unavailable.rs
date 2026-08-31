// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2017, Gustavo Romero, Breno Leitao, Cyril Bur, IBM Corp.
 *
 * Force FP, VEC and VSX unavailable exception during transaction in all
 * possible scenarios regarding the MSR.FP and MSR.VEC state, e.g. when FP
 * is enable and VEC is disable, when FP is disable and VEC is enable, and
 * so on. Then we check if the restored state is correctly set for the
 * FP and VEC registers to the previous state we set just before we entered
 * in TM, i.e. we check if it corrupts somehow the recheckpointed FP and
 * VEC/Altivec registers on abortion due to an unavailable exception in TM.
 * N.B. In this test we do not test all the FP/Altivec/VSX registers for
 * corruption, but only for registers vs0 and vs32, which are respectively
 * representatives of FP and VEC/Altivec reg sets.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_void};

const DEBUG: c_int = 0;

/* Unavailable exceptions to test in HTM */
const FP_UNA_EXCEPTION: c_int = 0;
const VEC_UNA_EXCEPTION: c_int = 1;
const VSX_UNA_EXCEPTION: c_int = 2;

const NUM_EXCEPTIONS: c_int = 3;

type uint64_t = u64;
type pthread_t = c_ulong;

#[repr(C)]
pub struct pthread_attr_t {
    __size: [u8; 56],
}

#[repr(C)]
pub struct cpu_set_t {
    __bits: [c_ulong; 16],
}

#[repr(C)]
struct Flags {
    touch_fp: c_int,
    touch_vec: c_int,
    result: c_int,
    exception: c_int,
}

static mut flags: Flags = Flags {
    touch_fp: 0,
    touch_vec: 0,
    result: 0,
    exception: 0,
};

unsafe extern "C" {
    fn printf(format: *const c_char, ...) -> c_int;
    fn putchar(c: c_int) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn exit(status: c_int) -> !;
    fn sched_yield() -> c_int;

    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const pthread_attr_t,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn pthread_setname_np(thread: pthread_t, name: *const c_char) -> c_int;
    fn pthread_attr_init(attr: *mut pthread_attr_t) -> c_int;
    fn pthread_attr_setaffinity_np(
        attr: *mut pthread_attr_t,
        cpusetsize: usize,
        cpuset: *const cpu_set_t,
    ) -> c_int;

    fn error_at_line(
        status: c_int,
        errnum: c_int,
        filename: *const c_char,
        linenum: c_uint,
        format: *const c_char,
        ...
    );

    fn have_htm() -> c_int;
    fn htm_is_synthetic() -> c_int;
    fn pick_online_cpu() -> c_int;
    fn failure_is_reschedule() -> bool;
    fn failure_is_unavailable() -> bool;
    fn failure_code() -> c_ulong;
    fn test_harness_set_timeout(seconds: c_int);
    fn test_harness(test: extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

type c_uint = u32;

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    let set = unsafe { &mut *set };
    for bit in set.__bits.iter_mut() {
        *bit = 0;
    }
}

unsafe fn CPU_SET(cpu: c_int, set: *mut cpu_set_t) {
    let bits_per_word = 8 * core::mem::size_of::<c_ulong>();
    let set = unsafe { &mut *set };
    set.__bits[cpu as usize / bits_per_word] |= 1 as c_ulong << (cpu as usize % bits_per_word);
}

unsafe fn pr_warn(code: c_int, format: *const c_char) {
    unsafe {
        error_at_line(
            0,
            code,
            c"./tm-unavailable.rs".as_ptr(),
            line!(),
            format,
        );
    }
}

unsafe fn pr_err(code: c_int, format: *const c_char) {
    unsafe {
        error_at_line(
            1,
            code,
            c"./tm-unavailable.rs".as_ptr(),
            line!(),
            format,
        );
    }
}

unsafe fn SKIP_IF(cond: bool) {
    if cond {
        unsafe {
            exit(0);
        }
    }
}

unsafe fn FAIL_IF(cond: bool) {
    if cond {
        unsafe {
            exit(1);
        }
    }
}

fn expecting_failure() -> bool {
    unsafe {
        if flags.touch_fp != 0 && flags.exception == FP_UNA_EXCEPTION {
            return false;
        }

        if flags.touch_vec != 0 && flags.exception == VEC_UNA_EXCEPTION {
            return false;
        }

        /*
         * If both FP and VEC are touched it does not mean that touching VSX
         * won't raise an exception. However since FP and VEC state are already
         * correctly loaded, the transaction is not aborted (i.e.
         * treclaimed/trecheckpointed) and MSR.VSX is just set as 1, so a TM
         * failure is not expected also in this case.
         */
        if (flags.touch_fp != 0 && flags.touch_vec != 0) && flags.exception == VSX_UNA_EXCEPTION {
            return false;
        }
    }

    true
}

/* Check if failure occurred whilst in transaction. */
fn is_failure(condition_reg: uint64_t) -> bool {
    /*
     * When failure handling occurs, CR0 is set to 0b1010 (0xa). Otherwise
     * transaction completes without failure and hence reaches out 'tend.'
     * that sets CR0 to 0b0100 (0x4).
     */
    ((condition_reg >> 28) & 0xa) == 0xa
}

unsafe extern "C" fn tm_una_ping(_input: *mut c_void) -> *mut c_void {
    /*
     * Expected values for vs0 and vs32 after a TM failure. They must never
     * change, otherwise they got corrupted.
     */
    let mut high_vs0: uint64_t = 0x5555555555555555;
    let mut low_vs0: uint64_t = 0xffffffffffffffff;
    let mut high_vs32: uint64_t = 0x5555555555555555;
    let mut low_vs32: uint64_t = 0xffffffffffffffff;

    /* Counter for busy wait */
    let counter: uint64_t = 0x1ff000000;

    /*
     * Variable to keep a copy of CR register content taken just after we
     * leave the transactional state.
     */
    let mut cr_: uint64_t = 0;

    /*
     * Wait a bit so thread can get its name "ping". This is not important
     * to reproduce the issue but it's nice to have for systemtap debugging.
     */
    if DEBUG != 0 {
        unsafe {
            sleep(1);
        }
    }

    unsafe {
        printf(
            c"If MSR.FP=%d MSR.VEC=%d: ".as_ptr(),
            flags.touch_fp,
            flags.touch_vec,
        );
    }

    unsafe {
        if flags.exception != FP_UNA_EXCEPTION
            && flags.exception != VEC_UNA_EXCEPTION
            && flags.exception != VSX_UNA_EXCEPTION
        {
            printf(c"No valid exception specified to test.\n".as_ptr());
            return core::ptr::null_mut();
        }
    }

    unsafe {
        asm!(
            /* Prepare to merge low and high. */
            "mtvsrd		33, {high_vs0}",
            "mtvsrd		34, {low_vs0}",
            /*
             * Adjust VS0 expected value after an TM failure,
             * i.e. vs0 = 0x5555555555555555555FFFFFFFFFFFFFFFF
             */
            "xxmrghd		0, 33, 34",
            /*
             * Adjust VS32 expected value after an TM failure,
             * i.e. vs32 = 0x5555555555555555555FFFFFFFFFFFFFFFF
             */
            "xxmrghd		32, 33, 34",
            /*
             * Wait an amount of context switches so load_fp and load_vec
             * overflow and MSR.FP, MSR.VEC, and MSR.VSX become zero (off).
             */
            "mtctr		{counter}",
            /* Decrement CTR branch if CTR non zero. */
            "1:	bdnz 1b",
            /*
             * Check if we want to touch FP prior to the test in order
             * to set MSR.FP = 1 before provoking an unavailable
             * exception in TM.
             */
            "cmpldi		{touch_fp}, 0",
            "beq		no_fp",
            "fadd		10, 10, 10",
            "no_fp:",
            /*
             * Check if we want to touch VEC prior to the test in order
             * to set MSR.VEC = 1 before provoking an unavailable
             * exception in TM.
             */
            "cmpldi		{touch_vec}, 0",
            "beq		no_vec",
            "vaddcuw		10, 10, 10",
            "no_vec:",
            /*
             * Perhaps it would be a better idea to do the
             * compares outside transactional context and simply
             * duplicate code.
             */
            "tbegin.",
            "beq		trans_fail",
            /* Do we do FP Unavailable? */
            "cmpldi		{exception}, {ex_fp}",
            "bne		2f",
            "fadd		10, 10, 10",
            "b		done",
            /* Do we do VEC Unavailable? */
            "2:	cmpldi		{exception}, {ex_vec}",
            "bne		3f",
            "vaddcuw		10, 10, 10",
            "b		done",
            /*
             * Not FP or VEC, therefore VSX. Ensure this
             * instruction always generates a VSX Unavailable.
             * ISA 3.0 is tricky here.
             * (xxmrghd will on ISA 2.07 and ISA 3.0)
             */
            "3:	xxmrghd		10, 10, 10",
            "done:	tend.",
            "trans_fail:",
            /* Give values back to C. */
            "mfvsrd		{high_vs0}, 0",
            "xxsldwi		3, 0, 0, 2",
            "mfvsrd		{low_vs0}, 3",
            "mfvsrd		{high_vs32}, 32",
            "xxsldwi		3, 32, 32, 2",
            "mfvsrd		{low_vs32}, 3",
            /* Give CR back to C so that it can check what happened. */
            "mfcr		{cr_}",
            high_vs0 = inout(reg) high_vs0,
            low_vs0 = inout(reg) low_vs0,
            high_vs32 = out(reg) high_vs32,
            low_vs32 = out(reg) low_vs32,
            cr_ = inout(reg) cr_,
            touch_fp = in(reg) flags.touch_fp,
            touch_vec = in(reg) flags.touch_vec,
            exception = in(reg) flags.exception,
            ex_fp = const FP_UNA_EXCEPTION,
            ex_vec = const VEC_UNA_EXCEPTION,
            counter = in(reg) counter,
        );
    }

    /*
     * Check if we were expecting a failure and it did not occur by checking
     * CR0 state just after we leave the transaction. Either way we check if
     * vs0 or vs32 got corrupted.
     */
    if expecting_failure() && !is_failure(cr_) {
        unsafe {
            printf(
                c"\n\tExpecting the transaction to fail, %s".as_ptr(),
                c"but it didn't\n\t".as_ptr(),
            );
            flags.result += 1;
        }
    }

    /* Check if we were not expecting a failure and a it occurred. */
    if !expecting_failure()
        && is_failure(cr_)
        && unsafe { !failure_is_reschedule() }
    {
        unsafe {
            printf(
                c"\n\tUnexpected transaction failure 0x%02lx\n\t".as_ptr(),
                failure_code(),
            );
        }
        return (-1isize) as *mut c_void;
    }

    /*
     * Check if TM failed due to the cause we were expecting. 0xda is a
     * TM_CAUSE_FAC_UNAV cause, otherwise it's an unexpected cause, unless
     * it was caused by a reschedule.
     */
    if is_failure(cr_)
        && unsafe { !failure_is_unavailable() }
        && unsafe { !failure_is_reschedule() }
    {
        unsafe {
            printf(
                c"\n\tUnexpected failure cause 0x%02lx\n\t".as_ptr(),
                failure_code(),
            );
        }
        return (-1isize) as *mut c_void;
    }

    /* 0x4 is a success and 0xa is a fail. See comment in is_failure(). */
    if DEBUG != 0 {
        unsafe {
            printf(c"CR0: 0x%1lx ".as_ptr(), cr_ >> 28);
        }
    }

    /* Check FP (vs0) for the expected value. */
    if high_vs0 != 0x5555555555555555 || low_vs0 != 0xFFFFFFFFFFFFFFFF {
        unsafe {
            printf(c"FP corrupted!".as_ptr());
            printf(
                c"  high = %#16lx  low = %#16lx ".as_ptr(),
                high_vs0,
                low_vs0,
            );
            flags.result += 1;
        }
    } else {
        unsafe {
            printf(c"FP ok ".as_ptr());
        }
    }

    /* Check VEC (vs32) for the expected value. */
    if high_vs32 != 0x5555555555555555 || low_vs32 != 0xFFFFFFFFFFFFFFFF {
        unsafe {
            printf(c"VEC corrupted!".as_ptr());
            printf(
                c"  high = %#16lx  low = %#16lx".as_ptr(),
                high_vs32,
                low_vs32,
            );
            flags.result += 1;
        }
    } else {
        unsafe {
            printf(c"VEC ok".as_ptr());
        }
    }

    unsafe {
        putchar('\n' as c_int);
    }

    core::ptr::null_mut()
}

/* Thread to force context switch */
unsafe extern "C" fn tm_una_pong(_not_used: *mut c_void) -> *mut c_void {
    /* Wait thread get its name "pong". */
    if DEBUG != 0 {
        unsafe {
            sleep(1);
        }
    }

    /* Classed as an interactive-like thread. */
    loop {
        unsafe {
            sched_yield();
        }
    }
}

/* Function that creates a thread and launches the "ping" task. */
unsafe fn test_fp_vec(fp: c_int, vec: c_int, attr: *mut pthread_attr_t) {
    let mut retries: c_int = 2;
    let mut ret_value: *mut c_void = core::ptr::null_mut();
    let mut t0: pthread_t = 0;

    unsafe {
        flags.touch_fp = fp;
        flags.touch_vec = vec;
    }

    /*
     * Without luck it's possible that the transaction is aborted not due to
     * the unavailable exception caught in the middle as we expect but also,
     * for instance, due to a context switch or due to a KVM reschedule (if
     * it's running on a VM). Thus we try a few times before giving up,
     * checking if the failure cause is the one we expect.
     */
    while {
        let mut rc: c_int;

        /* Bind to CPU 0, as specified in 'attr'. */
        rc = unsafe {
            pthread_create(
                &mut t0,
                attr,
                tm_una_ping,
                (&raw mut flags).cast::<c_void>(),
            )
        };
        if rc != 0 {
            unsafe {
                pr_err(rc, c"pthread_create()".as_ptr());
            }
        }
        rc = unsafe { pthread_setname_np(t0, c"tm_una_ping".as_ptr()) };
        if rc != 0 {
            unsafe {
                pr_warn(rc, c"pthread_setname_np".as_ptr());
            }
        }
        rc = unsafe { pthread_join(t0, &mut ret_value) };
        if rc != 0 {
            unsafe {
                pr_err(rc, c"pthread_join".as_ptr());
            }
        }

        retries -= 1;
        !ret_value.is_null() && retries != 0
    } {}

    if retries == 0 {
        unsafe {
            flags.result = 1;
        }
        if DEBUG != 0 {
            unsafe {
                printf(c"All transactions failed unexpectedly\n".as_ptr());
            }
        }
    }
}

extern "C" fn tm_unavailable_test() -> c_int {
    let mut cpu: c_int;
    let mut rc: c_int;
    let mut exception: c_int; /* FP = 0, VEC = 1, VSX = 2 */
    let mut t1: pthread_t = 0;
    let mut attr: pthread_attr_t = unsafe { core::mem::zeroed() };
    let mut cpuset: cpu_set_t = unsafe { core::mem::zeroed() };

    unsafe {
        SKIP_IF(have_htm() == 0);
        SKIP_IF(htm_is_synthetic() != 0);

        cpu = pick_online_cpu();
        FAIL_IF(cpu < 0);

        // Set only one CPU in the mask. Both threads will be bound to that CPU.
        CPU_ZERO(&mut cpuset);
        CPU_SET(cpu, &mut cpuset);

        /* Init pthread attribute. */
        rc = pthread_attr_init(&mut attr);
        if rc != 0 {
            pr_err(rc, c"pthread_attr_init()".as_ptr());
        }

        /* Set CPU 0 mask into the pthread attribute. */
        rc = pthread_attr_setaffinity_np(&mut attr, core::mem::size_of::<cpu_set_t>(), &cpuset);
        if rc != 0 {
            pr_err(rc, c"pthread_attr_setaffinity_np()".as_ptr());
        }

        rc = pthread_create(&mut t1, &attr /* Bind to CPU 0 */, tm_una_pong, core::ptr::null_mut());
        if rc != 0 {
            pr_err(rc, c"pthread_create()".as_ptr());
        }

        /* Name it for systemtap convenience */
        rc = pthread_setname_np(t1, c"tm_una_pong".as_ptr());
        if rc != 0 {
            pr_warn(rc, c"pthread_create()".as_ptr());
        }

        flags.result = 0;

        exception = 0;
        while exception < NUM_EXCEPTIONS {
            printf(c"Checking if FP/VEC registers are sane after".as_ptr());

            if exception == FP_UNA_EXCEPTION {
                printf(c" a FP unavailable exception...\n".as_ptr());
            } else if exception == VEC_UNA_EXCEPTION {
                printf(c" a VEC unavailable exception...\n".as_ptr());
            } else {
                printf(c" a VSX unavailable exception...\n".as_ptr());
            }

            flags.exception = exception;

            test_fp_vec(0, 0, &mut attr);
            test_fp_vec(1, 0, &mut attr);
            test_fp_vec(0, 1, &mut attr);
            test_fp_vec(1, 1, &mut attr);

            exception += 1;
        }

        if flags.result > 0 {
            printf(c"result: failed!\n".as_ptr());
            exit(1);
        } else {
            printf(c"result: success\n".as_ptr());
            exit(0);
        }
    }
}

fn main() {
    unsafe {
        test_harness_set_timeout(220);
        std::process::exit(test_harness(
            tm_unavailable_test,
            c"tm_unavailable_test".as_ptr(),
        ));
    }
}
