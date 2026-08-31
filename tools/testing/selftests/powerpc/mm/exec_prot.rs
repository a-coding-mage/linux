// SPDX-License-Identifier: GPL-2.0

/*
 * Copyright 2022, Nicholas Miehlbradt, IBM Corporation
 * based on pkey_exec_prot.c
 *
 * Test if applying execute protection on pages works as expected.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use core::arch::asm;
use core::ffi::c_void;
use core::ptr;

const PPC_INST_NOP: u32 = 0x60000000;
const PPC_INST_TRAP: u32 = 0x7fe00008;
const PPC_INST_BLR: u32 = 0x4e800020;

type sig_atomic_t = libc::c_int;

unsafe extern "C" {
    fn pkeys_unsupported() -> libc::c_int;
    fn have_hwcap2(feature: libc::c_ulong) -> bool;
    fn test_harness(test: unsafe extern "C" fn() -> libc::c_int, name: *const libc::c_char) -> libc::c_int;
    fn sigsafe_err(msg: *const libc::c_char);
}

static mut fault_code: sig_atomic_t = 0;
static mut remaining_faults: sig_atomic_t = 0;
static mut fault_addr: *mut u32 = ptr::null_mut();
static mut pgsize: libc::c_ulong = 0;
static mut numinsns: libc::c_ulong = 0;
static mut insns: *mut u32 = ptr::null_mut();
static mut pkeys_supported: bool = false;

unsafe fn FAIL_IF(cond: bool) -> libc::c_int {
    if cond {
        return 1;
    }
    0
}

unsafe fn SKIP_IF(cond: bool) -> libc::c_int {
    if cond {
        return 0;
    }
    0
}

unsafe fn is_fault_expected(fault_code_arg: libc::c_int) -> bool {
    if fault_code_arg == libc::SEGV_ACCERR {
        return true;
    }

    /* Assume any pkey error is fine since pkey_exec_prot test covers them */
    if fault_code_arg == libc::SEGV_PKUERR && pkeys_supported {
        return true;
    }

    false
}

unsafe extern "C" fn trap_handler(
    _signum: libc::c_int,
    sinfo: *mut libc::siginfo_t,
    _ctx: *mut c_void,
) {
    /* Check if this fault originated from the expected address */
    if unsafe { (*sinfo).si_addr() } != fault_addr as *mut c_void {
        unsafe { sigsafe_err(c"got a fault for an unexpected address\n".as_ptr()) };
    }

    unsafe { libc::_exit(1) };
}

unsafe extern "C" fn segv_handler(
    _signum: libc::c_int,
    sinfo: *mut libc::siginfo_t,
    _ctx: *mut c_void,
) {
    unsafe {
        fault_code = (*sinfo).si_code;

        /* Check if this fault originated from the expected address */
        if (*sinfo).si_addr() != fault_addr as *mut c_void {
            sigsafe_err(c"got a fault for an unexpected address\n".as_ptr());
            libc::_exit(1);
        }

        /* Check if too many faults have occurred for a single test case */
        if remaining_faults == 0 {
            sigsafe_err(c"got too many faults for the same address\n".as_ptr());
            libc::_exit(1);
        }

        /* Restore permissions in order to continue */
        if is_fault_expected(fault_code) {
            if libc::mprotect(
                insns as *mut c_void,
                pgsize as libc::size_t,
                libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            ) != 0
            {
                sigsafe_err(c"failed to set access permissions\n".as_ptr());
                libc::_exit(1);
            }
        } else {
            sigsafe_err(c"got a fault with an unexpected code\n".as_ptr());
            libc::_exit(1);
        }

        remaining_faults -= 1;
    }
}

unsafe fn check_exec_fault(rights: libc::c_int) -> libc::c_int {
    /*
     * Jump to the executable region.
     *
     * The first iteration also checks if the overwrite of the
     * first instruction word from a trap to a no-op succeeded.
     */
    unsafe {
        fault_code = -1;
        remaining_faults = 0;
        if rights & libc::PROT_EXEC == 0 {
            remaining_faults = 1;
        }

        if FAIL_IF(libc::mprotect(insns as *mut c_void, pgsize as libc::size_t, rights) != 0) != 0 {
            return 1;
        }
        asm!("mtctr {0}; bctrl", in(reg) insns);

        if FAIL_IF(remaining_faults != 0) != 0 {
            return 1;
        }
        if rights & libc::PROT_EXEC == 0 {
            if FAIL_IF(!is_fault_expected(fault_code)) != 0 {
                return 1;
            }
        }

        0
    }
}

unsafe extern "C" fn test() -> libc::c_int {
    unsafe {
        let mut segv_act: libc::sigaction = core::mem::zeroed();
        let mut trap_act: libc::sigaction = core::mem::zeroed();
        let mut i: libc::c_int;

        /* Skip the test if the CPU doesn't support Radix */
        if SKIP_IF(!have_hwcap2(libc::PPC_FEATURE2_ARCH_3_00 as libc::c_ulong)) != 0 {
            return 0;
        }

        /* Check if pkeys are supported */
        pkeys_supported = pkeys_unsupported() == 0;

        /* Setup SIGSEGV handler */
        segv_act.sa_sigaction = segv_handler as usize;
        if FAIL_IF(libc::sigprocmask(libc::SIG_SETMASK, ptr::null(), &mut segv_act.sa_mask) != 0) != 0 {
            return 1;
        }
        segv_act.sa_flags = libc::SA_SIGINFO;
        if FAIL_IF(libc::sigaction(libc::SIGSEGV, &segv_act, ptr::null_mut()) != 0) != 0 {
            return 1;
        }

        /* Setup SIGTRAP handler */
        trap_act.sa_sigaction = trap_handler as usize;
        if FAIL_IF(libc::sigprocmask(libc::SIG_SETMASK, ptr::null(), &mut trap_act.sa_mask) != 0) != 0 {
            return 1;
        }
        trap_act.sa_flags = libc::SA_SIGINFO;
        if FAIL_IF(libc::sigaction(libc::SIGTRAP, &trap_act, ptr::null_mut()) != 0) != 0 {
            return 1;
        }

        /* Setup executable region */
        pgsize = libc::getpagesize() as libc::c_ulong;
        numinsns = pgsize / core::mem::size_of::<u32>() as libc::c_ulong;
        insns = libc::mmap(
            ptr::null_mut(),
            pgsize as libc::size_t,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        ) as *mut u32;
        if FAIL_IF(insns == libc::MAP_FAILED as *mut u32) != 0 {
            return 1;
        }

        /* Write the instruction words */
        i = 1;
        while (i as libc::c_ulong) < numinsns - 1 {
            *insns.add(i as usize) = PPC_INST_NOP;
            i += 1;
        }

        /*
         * Set the first instruction as an unconditional trap. If
         * the last write to this address succeeds, this should
         * get overwritten by a no-op.
         */
        *insns.add(0) = PPC_INST_TRAP;

        /*
         * Later, to jump to the executable region, we use a branch
         * and link instruction (bctrl) which sets the return address
         * automatically in LR. Use that to return back.
         */
        *insns.add((numinsns - 1) as usize) = PPC_INST_BLR;

        /*
         * Pick the first instruction's address from the executable
         * region.
         */
        fault_addr = insns;

        /*
         * Read an instruction word from the address when the page
         * is execute only. This should generate an access fault.
         */
        fault_code = -1;
        remaining_faults = 1;
        libc::printf(c"Testing read on --x, should fault...".as_ptr());
        if FAIL_IF(libc::mprotect(insns as *mut c_void, pgsize as libc::size_t, libc::PROT_EXEC) != 0) != 0 {
            return 1;
        }
        i = *(fault_addr as *mut libc::c_int);
        if FAIL_IF(remaining_faults != 0 || !is_fault_expected(fault_code)) != 0 {
            return 1;
        }
        libc::printf(c"ok!\n".as_ptr());

        /*
         * Write an instruction word to the address when the page
         * execute only. This should also generate an access fault.
         */
        fault_code = -1;
        remaining_faults = 1;
        libc::printf(c"Testing write on --x, should fault...".as_ptr());
        if FAIL_IF(libc::mprotect(insns as *mut c_void, pgsize as libc::size_t, libc::PROT_EXEC) != 0) != 0 {
            return 1;
        }
        *fault_addr = PPC_INST_NOP;
        if FAIL_IF(remaining_faults != 0 || !is_fault_expected(fault_code)) != 0 {
            return 1;
        }
        libc::printf(c"ok!\n".as_ptr());

        libc::printf(c"Testing exec on ---, should fault...".as_ptr());
        if FAIL_IF(check_exec_fault(libc::PROT_NONE) != 0) != 0 {
            return 1;
        }
        libc::printf(c"ok!\n".as_ptr());

        libc::printf(c"Testing exec on r--, should fault...".as_ptr());
        if FAIL_IF(check_exec_fault(libc::PROT_READ) != 0) != 0 {
            return 1;
        }
        libc::printf(c"ok!\n".as_ptr());

        libc::printf(c"Testing exec on -w-, should fault...".as_ptr());
        if FAIL_IF(check_exec_fault(libc::PROT_WRITE) != 0) != 0 {
            return 1;
        }
        libc::printf(c"ok!\n".as_ptr());

        libc::printf(c"Testing exec on rw-, should fault...".as_ptr());
        if FAIL_IF(check_exec_fault(libc::PROT_READ | libc::PROT_WRITE) != 0) != 0 {
            return 1;
        }
        libc::printf(c"ok!\n".as_ptr());

        libc::printf(c"Testing exec on --x, should succeed...".as_ptr());
        if FAIL_IF(check_exec_fault(libc::PROT_EXEC) != 0) != 0 {
            return 1;
        }
        libc::printf(c"ok!\n".as_ptr());

        libc::printf(c"Testing exec on r-x, should succeed...".as_ptr());
        if FAIL_IF(check_exec_fault(libc::PROT_READ | libc::PROT_EXEC) != 0) != 0 {
            return 1;
        }
        libc::printf(c"ok!\n".as_ptr());

        libc::printf(c"Testing exec on -wx, should succeed...".as_ptr());
        if FAIL_IF(check_exec_fault(libc::PROT_WRITE | libc::PROT_EXEC) != 0) != 0 {
            return 1;
        }
        libc::printf(c"ok!\n".as_ptr());

        libc::printf(c"Testing exec on rwx, should succeed...".as_ptr());
        if FAIL_IF(check_exec_fault(libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC) != 0) != 0 {
            return 1;
        }
        libc::printf(c"ok!\n".as_ptr());

        /* Cleanup */
        if FAIL_IF(libc::munmap(insns as *mut c_void, pgsize as libc::size_t) != 0) != 0 {
            return 1;
        }

        0
    }
}

fn main() {
    unsafe {
        std::process::exit(test_harness(test, c"exec_prot".as_ptr()));
    }
}
