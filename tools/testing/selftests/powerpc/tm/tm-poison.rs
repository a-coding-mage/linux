// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2019, Gustavo Romero, Michael Neuling, IBM Corp.
 *
 * This test will spawn two processes. Both will be attached to the same
 * CPU (CPU 0). The child will be in a loop writing to FP register f31 and
 * VMX/VEC/Altivec register vr31 a known value, called poison, calling
 * sched_yield syscall after to allow the parent to switch on the CPU.
 * Parent will set f31 and vr31 to 1 and in a loop will check if f31 and
 * vr31 remain 1 as expected until a given timeout (2m). If the issue is
 * present child's poison will leak into parent's f31 or vr31 registers,
 * otherwise, poison will never leak into parent's f31 and vr31 registers.
 */

use core::arch::asm;
use core::ffi::{c_char, c_int};

const SIGKILL: c_int = 9;
const CPU_SETSIZE: usize = 1024;
const NCPUBITS: usize = 8 * core::mem::size_of::<usize>();

#[repr(C)]
struct cpu_set_t {
    __bits: [usize; CPU_SETSIZE / NCPUBITS],
}

unsafe extern "C" {
    fn have_htm() -> bool;
    fn htm_is_synthetic() -> bool;
    fn pick_online_cpu() -> c_int;
    fn sched_setaffinity(pid: c_int, cpusetsize: usize, mask: *const cpu_set_t) -> c_int;
    fn sched_yield() -> c_int;
    fn fork() -> c_int;
    fn kill(pid: c_int, sig: c_int) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn test_harness_set_timeout(timeout: c_int);
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;

    /*
     * From tm.h: these are C macros in the original source. Their control-flow
     * behavior is supplied externally for this translation unit.
     */
    fn SKIP_IF(condition: bool);
    fn FAIL_IF(condition: bool);
}

unsafe fn CPU_ZERO(set: *mut cpu_set_t) {
    unsafe {
        (*set).__bits.fill(0);
    }
}

unsafe fn CPU_SET(cpu: c_int, set: *mut cpu_set_t) {
    let cpu = cpu as usize;

    unsafe {
        (*set).__bits[cpu / NCPUBITS] |= 1usize << (cpu % NCPUBITS);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tm_poison_test() -> c_int {
    let mut cpu: c_int;
    let pid: c_int;
    let mut cpuset = cpu_set_t {
        __bits: [0; CPU_SETSIZE / NCPUBITS],
    };
    let poison: u64 = 0xdeadbeefc0dec0fe;
    let mut unknown: u64 = 0;
    let mut fail_fp: bool = false;
    let mut fail_vr: bool = false;

    unsafe {
        SKIP_IF(!have_htm());
        SKIP_IF(htm_is_synthetic());

        cpu = pick_online_cpu();
        FAIL_IF(cpu < 0);

        // Attach both Child and Parent to the same CPU
        CPU_ZERO(&mut cpuset);
        CPU_SET(cpu, &mut cpuset);
        FAIL_IF(sched_setaffinity(0, core::mem::size_of_val(&cpuset), &cpuset) != 0);

        pid = fork();
        if pid == 0 {
            /**
             * child
             */
            loop {
                sched_yield();
                asm!(
                    "mtvsrd 31, {poison}", // f31 = poison
                    "mtvsrd 63, {poison}", // vr31 = poison
                    poison = in(reg) poison,
                );
            }
        }

        /**
         * parent
         */
        asm!(
            /*
             * Set r3, r4, and f31 to known value 1 before entering
             * in transaction. They won't be written after that.
             */
            "       li      3, 0x1          ;",
            "       li      4, 0x1          ;",
            "       mtvsrd  31, 4           ;",

            /*
             * The Time Base (TB) is a 64-bit counter register that is
             * independent of the CPU clock and which is incremented
             * at a frequency of 512000000 Hz, so every 1.953125ns.
             * So it's necessary 120s/0.000000001953125s = 61440000000
             * increments to get a 2 minutes timeout. Below we set that
             * value in r5 and then use r6 to track initial TB value,
             * updating TB values in r7 at every iteration and comparing it
             * to r6. When r7 (current) - r6 (initial) > 61440000000 we bail
             * out since for sure we spent already 2 minutes in the loop.
             * SPR 268 is the TB register.
             */
            "       lis     5, 14           ;",
            "       ori     5, 5, 19996     ;",
            "       sldi    5, 5, 16        ;", // r5 = 61440000000

            "       mfspr   6, 268          ;", // r6 (TB initial)
            "1:     mfspr   7, 268          ;", // r7 (TB current)
            "       subf    7, 6, 7         ;", // r7 - r6 > 61440000000 ?
            "       cmpd    7, 5            ;",
            "       bgt     3f              ;", // yes, exit

            /*
             * Main loop to check f31
             */
            "       tbegin.                 ;", // no, try again
            "       beq     1b              ;", // restart if no timeout
            "       mfvsrd  3, 31           ;", // read f31
            "       cmpd    3, 4            ;", // f31 == 1 ?
            "       bne     2f              ;", // broken :-(
            "       tabort. 3               ;", // try another transaction
            "2:     tend.                   ;", // commit transaction
            "3:     mr    {unknown}, 3      ;", // record r3

            unknown = out(reg) unknown,
            out("r3") _,
            out("r4") _,
            out("r5") _,
            out("r6") _,
            out("r7") _,
        );

        /*
         * On leak 'unknown' will contain 'poison' value from child,
         * otherwise (no leak) 'unknown' will contain the same value
         * as r3 before entering in transactional mode, i.e. 0x1.
         */
        fail_fp = unknown != 0x1;
        if fail_fp {
            printf(
                c"Unknown value %#lx leaked into f31!\n".as_ptr(),
                unknown,
            );
        } else {
            printf(c"Good, no poison or leaked value into FP registers\n".as_ptr());
        }

        asm!(
            /*
             * Set r3, r4, and vr31 to known value 1 before entering
             * in transaction. They won't be written after that.
             */
            "       li      3, 0x1          ;",
            "       li      4, 0x1          ;",
            "       mtvsrd  63, 4           ;",

            "       lis     5, 14           ;",
            "       ori     5, 5, 19996     ;",
            "       sldi    5, 5, 16        ;", // r5 = 61440000000

            "       mfspr   6, 268          ;", // r6 (TB initial)
            "1:     mfspr   7, 268          ;", // r7 (TB current)
            "       subf    7, 6, 7         ;", // r7 - r6 > 61440000000 ?
            "       cmpd    7, 5            ;",
            "       bgt     3f              ;", // yes, exit

            /*
             * Main loop to check vr31
             */
            "       tbegin.                 ;", // no, try again
            "       beq     1b              ;", // restart if no timeout
            "       mfvsrd  3, 63           ;", // read vr31
            "       cmpd    3, 4            ;", // vr31 == 1 ?
            "       bne     2f              ;", // broken :-(
            "       tabort. 3               ;", // try another transaction
            "2:     tend.                   ;", // commit transaction
            "3:     mr    {unknown}, 3      ;", // record r3

            unknown = out(reg) unknown,
            out("r3") _,
            out("r4") _,
            out("r5") _,
            out("r6") _,
            out("r7") _,
        );

        /*
         * On leak 'unknown' will contain 'poison' value from child,
         * otherwise (no leak) 'unknown' will contain the same value
         * as r3 before entering in transactional mode, i.e. 0x1.
         */
        fail_vr = unknown != 0x1;
        if fail_vr {
            printf(
                c"Unknown value %#lx leaked into vr31!\n".as_ptr(),
                unknown,
            );
        } else {
            printf(c"Good, no poison or leaked value into VEC registers\n".as_ptr());
        }

        kill(pid, SIGKILL);
    }

    (fail_fp | fail_vr) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    unsafe {
        /* Test completes in about 4m */
        test_harness_set_timeout(250);
        test_harness(tm_poison_test, c"tm_poison_test".as_ptr())
    }
}
