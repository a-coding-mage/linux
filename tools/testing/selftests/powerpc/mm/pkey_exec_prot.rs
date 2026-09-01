// SPDX-License-Identifier: GPL-2.0+

/*
 * Copyright 2020, Sandipan Das, IBM Corp.
 *
 * Test if applying execute protection on pages using memory
 * protection keys works as expected.
 */

// C dependencies: stdio.h, stdlib.h, string.h, signal.h, unistd.h, "pkeys.h".

use core::arch::asm;
use core::ffi::{c_int, c_ulong, c_void};

const PPC_INST_NOP: u32 = 0x60000000;
const PPC_INST_TRAP: u32 = 0x7fe00008;
const PPC_INST_BLR: u32 = 0x4e800020;

static mut fault_pkey: libc::sig_atomic_t = 0;
static mut fault_code: libc::sig_atomic_t = 0;
static mut fault_type: libc::sig_atomic_t = 0;
static mut remaining_faults: libc::sig_atomic_t = 0;
static mut fault_addr: *mut u32 = core::ptr::null_mut();
static mut pgsize: c_ulong = 0;
static mut numinsns: c_ulong = 0;
static mut insns: *mut u32 = core::ptr::null_mut();

unsafe extern "C" {
    fn sigsafe_err(fmt: *const libc::c_char, ...);
    fn siginfo_pkey(sinfo: *mut libc::siginfo_t) -> c_int;
    fn pkey_set_rights(pkey: c_int, rights: c_ulong);
    fn pkeys_unsupported() -> c_int;
    fn sys_pkey_alloc(flags: c_ulong, rights: c_ulong) -> c_int;
    fn sys_pkey_free(pkey: c_int) -> c_int;
    fn sys_pkey_mprotect(addr: *mut c_void, len: c_ulong, prot: c_int, pkey: c_int) -> c_int;
    fn pkey_rights(rights: c_ulong) -> *const libc::c_char;
    fn next_pkey_rights(rights: c_ulong) -> c_ulong;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const libc::c_char) -> c_int;
}

// Constants supplied by "pkeys.h" in the original C translation unit.
extern "Rust" {
    static PKEY_DISABLE_ACCESS: c_ulong;
    static PKEY_DISABLE_EXECUTE: c_ulong;
    static PKEY_UNRESTRICTED: c_ulong;
}

// FAIL_IF is supplied by the selftest harness headers in the original C file.
macro_rules! FAIL_IF {
    ($cond:expr) => {
        if $cond {
            return 1;
        }
    };
}

unsafe extern "C" fn trap_handler(
    _signum: c_int,
    sinfo: *mut libc::siginfo_t,
    _ctx: *mut c_void,
) {
    /* Check if this fault originated from the expected address */
    if unsafe { (*sinfo).si_addr() } != unsafe { fault_addr as *mut c_void } {
        unsafe {
            sigsafe_err(c"got a fault for an unexpected address\n".as_ptr());
        }
    }

    unsafe {
        libc::_exit(1);
    }
}

unsafe extern "C" fn segv_handler(
    _signum: c_int,
    sinfo: *mut libc::siginfo_t,
    _ctx: *mut c_void,
) {
    let signal_pkey: c_int;

    unsafe {
        signal_pkey = siginfo_pkey(sinfo);
        fault_code = (*sinfo).si_code;
    }

    /* Check if this fault originated from the expected address */
    if unsafe { (*sinfo).si_addr() } != unsafe { fault_addr as *mut c_void } {
        unsafe {
            sigsafe_err(c"got a fault for an unexpected address\n".as_ptr());
            libc::_exit(1);
        }
    }

    /* Check if too many faults have occurred for a single test case */
    if unsafe { remaining_faults == 0 } {
        unsafe {
            sigsafe_err(c"got too many faults for the same address\n".as_ptr());
            libc::_exit(1);
        }
    }

    /* Restore permissions in order to continue */
    match unsafe { fault_code } {
        libc::SEGV_ACCERR => {
            if unsafe {
                libc::mprotect(
                    insns as *mut c_void,
                    pgsize as libc::size_t,
                    libc::PROT_READ | libc::PROT_WRITE,
                )
            } != 0
            {
                unsafe {
                    sigsafe_err(c"failed to set access permissions\n".as_ptr());
                    libc::_exit(1);
                }
            }
        }
        libc::SEGV_PKUERR => {
            if signal_pkey != unsafe { fault_pkey } {
                unsafe {
                    sigsafe_err(c"got a fault for an unexpected pkey\n".as_ptr());
                    libc::_exit(1);
                }
            }

            if unsafe { fault_type } == unsafe { PKEY_DISABLE_ACCESS as libc::sig_atomic_t } {
                unsafe {
                    pkey_set_rights(fault_pkey, PKEY_UNRESTRICTED);
                }
            } else if unsafe { fault_type } == unsafe { PKEY_DISABLE_EXECUTE as libc::sig_atomic_t } {
                /*
                 * Reassociate the exec-only pkey with the region
                 * to be able to continue. Unlike AMR, we cannot
                 * set IAMR directly from userspace to restore the
                 * permissions.
                 */
                if unsafe {
                    libc::mprotect(insns as *mut c_void, pgsize as libc::size_t, libc::PROT_EXEC)
                } != 0
                {
                    unsafe {
                        sigsafe_err(c"failed to set execute permissions\n".as_ptr());
                        libc::_exit(1);
                    }
                }
            } else {
                unsafe {
                    sigsafe_err(c"got a fault with an unexpected type\n".as_ptr());
                    libc::_exit(1);
                }
            }
        }
        _ => {
            unsafe {
                sigsafe_err(c"got a fault with an unexpected code\n".as_ptr());
                libc::_exit(1);
            }
        }
    }

    unsafe {
        remaining_faults -= 1;
    }
}

unsafe extern "C" fn test() -> c_int {
    let mut segv_act: libc::sigaction = unsafe { core::mem::zeroed() };
    let mut trap_act: libc::sigaction = unsafe { core::mem::zeroed() };
    let mut rights: c_ulong;
    let mut pkey: c_int;
    let ret: c_int;
    let mut i: c_int;

    ret = unsafe { pkeys_unsupported() };
    if ret != 0 {
        return ret;
    }

    /* Setup SIGSEGV handler */
    segv_act.sa_sigaction = segv_handler as usize;
    FAIL_IF!(unsafe { libc::sigprocmask(libc::SIG_SETMASK, core::ptr::null(), &mut segv_act.sa_mask) } != 0);
    segv_act.sa_flags = libc::SA_SIGINFO;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        segv_act.sa_restorer = None;
    }
    FAIL_IF!(unsafe { libc::sigaction(libc::SIGSEGV, &segv_act, core::ptr::null_mut()) } != 0);

    /* Setup SIGTRAP handler */
    trap_act.sa_sigaction = trap_handler as usize;
    FAIL_IF!(unsafe { libc::sigprocmask(libc::SIG_SETMASK, core::ptr::null(), &mut trap_act.sa_mask) } != 0);
    trap_act.sa_flags = libc::SA_SIGINFO;
    #[cfg(any(target_os = "linux", target_os = "android"))]
    {
        trap_act.sa_restorer = None;
    }
    FAIL_IF!(unsafe { libc::sigaction(libc::SIGTRAP, &trap_act, core::ptr::null_mut()) } != 0);

    /* Setup executable region */
    unsafe {
        pgsize = libc::getpagesize() as c_ulong;
        numinsns = pgsize / core::mem::size_of::<u32>() as c_ulong;
        insns = libc::mmap(
            core::ptr::null_mut(),
            pgsize as libc::size_t,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        ) as *mut u32;
    }
    FAIL_IF!(unsafe { insns == libc::MAP_FAILED as *mut u32 });

    /* Write the instruction words */
    i = 1;
    while (i as c_ulong) < unsafe { numinsns - 1 } {
        unsafe {
            *insns.add(i as usize) = PPC_INST_NOP;
        }
        i += 1;
    }

    /*
     * Set the first instruction as an unconditional trap. If
     * the last write to this address succeeds, this should
     * get overwritten by a no-op.
     */
    unsafe {
        *insns.add(0) = PPC_INST_TRAP;
    }

    /*
     * Later, to jump to the executable region, we use a branch
     * and link instruction (bctrl) which sets the return address
     * automatically in LR. Use that to return back.
     */
    unsafe {
        *insns.add((numinsns - 1) as usize) = PPC_INST_BLR;
    }

    /* Allocate a pkey that restricts execution */
    rights = unsafe { PKEY_DISABLE_EXECUTE };
    pkey = unsafe { sys_pkey_alloc(0, rights) };
    FAIL_IF!(pkey < 0);

    /*
     * Pick the first instruction's address from the executable
     * region.
     */
    unsafe {
        fault_addr = insns;
    }

    /* The following two cases will avoid SEGV_PKUERR */
    unsafe {
        fault_type = -1;
        fault_pkey = -1;
    }

    /*
     * Read an instruction word from the address when AMR bits
     * are not set i.e. the pkey permits both read and write
     * access.
     *
     * This should not generate a fault as having PROT_EXEC
     * implies PROT_READ on GNU systems. The pkey currently
     * restricts execution only based on the IAMR bits. The
     * AMR bits are cleared.
     */
    unsafe {
        remaining_faults = 0;
    }
    FAIL_IF!(unsafe { sys_pkey_mprotect(insns as *mut c_void, pgsize, libc::PROT_EXEC, pkey) } != 0);
    unsafe {
        libc::printf(
            c"read from %p, pkey permissions are %s\n".as_ptr(),
            fault_addr,
            pkey_rights(rights),
        );
        i = *fault_addr as c_int;
    }
    let _ = i;
    FAIL_IF!(unsafe { remaining_faults != 0 });

    /*
     * Write an instruction word to the address when AMR bits
     * are not set i.e. the pkey permits both read and write
     * access.
     *
     * This should generate an access fault as having just
     * PROT_EXEC also restricts writes. The pkey currently
     * restricts execution only based on the IAMR bits. The
     * AMR bits are cleared.
     */
    unsafe {
        remaining_faults = 1;
    }
    FAIL_IF!(unsafe { sys_pkey_mprotect(insns as *mut c_void, pgsize, libc::PROT_EXEC, pkey) } != 0);
    unsafe {
        libc::printf(
            c"write to %p, pkey permissions are %s\n".as_ptr(),
            fault_addr,
            pkey_rights(rights),
        );
        *fault_addr = PPC_INST_TRAP;
    }
    FAIL_IF!(unsafe { remaining_faults != 0 || fault_code != libc::SEGV_ACCERR });

    /* The following three cases will generate SEGV_PKUERR */
    rights |= unsafe { PKEY_DISABLE_ACCESS };
    unsafe {
        fault_type = PKEY_DISABLE_ACCESS as libc::sig_atomic_t;
        fault_pkey = pkey;
    }

    /*
     * Read an instruction word from the address when AMR bits
     * are set i.e. the pkey permits neither read nor write
     * access.
     *
     * This should generate a pkey fault based on AMR bits only
     * as having PROT_EXEC implicitly allows reads.
     */
    unsafe {
        remaining_faults = 1;
    }
    FAIL_IF!(unsafe { sys_pkey_mprotect(insns as *mut c_void, pgsize, libc::PROT_EXEC, pkey) } != 0);
    unsafe {
        pkey_set_rights(pkey, rights);
        libc::printf(
            c"read from %p, pkey permissions are %s\n".as_ptr(),
            fault_addr,
            pkey_rights(rights),
        );
        i = *fault_addr as c_int;
    }
    let _ = i;
    FAIL_IF!(unsafe { remaining_faults != 0 || fault_code != libc::SEGV_PKUERR });

    /*
     * Write an instruction word to the address when AMR bits
     * are set i.e. the pkey permits neither read nor write
     * access.
     *
     * This should generate two faults. First, a pkey fault
     * based on AMR bits and then an access fault since
     * PROT_EXEC does not allow writes.
     */
    unsafe {
        remaining_faults = 2;
    }
    FAIL_IF!(unsafe { sys_pkey_mprotect(insns as *mut c_void, pgsize, libc::PROT_EXEC, pkey) } != 0);
    unsafe {
        pkey_set_rights(pkey, rights);
        libc::printf(
            c"write to %p, pkey permissions are %s\n".as_ptr(),
            fault_addr,
            pkey_rights(rights),
        );
        *fault_addr = PPC_INST_NOP;
    }
    FAIL_IF!(unsafe { remaining_faults != 0 || fault_code != libc::SEGV_ACCERR });

    /* Free the current pkey */
    unsafe {
        sys_pkey_free(pkey);
    }

    rights = 0;
    loop {
        /*
         * Allocate pkeys with all valid combinations of read,
         * write and execute restrictions.
         */
        pkey = unsafe { sys_pkey_alloc(0, rights) };
        FAIL_IF!(pkey < 0);

        /*
         * Jump to the executable region. AMR bits may or may not
         * be set but they should not affect execution.
         *
         * This should generate pkey faults based on IAMR bits which
         * may be set to restrict execution.
         *
         * The first iteration also checks if the overwrite of the
         * first instruction word from a trap to a no-op succeeded.
         */
        unsafe {
            fault_pkey = pkey;
            fault_type = -1;
            remaining_faults = 0;
            if rights & PKEY_DISABLE_EXECUTE != 0 {
                fault_type = PKEY_DISABLE_EXECUTE as libc::sig_atomic_t;
                remaining_faults = 1;
            }
        }

        FAIL_IF!(unsafe { sys_pkey_mprotect(insns as *mut c_void, pgsize, libc::PROT_EXEC, pkey) } != 0);
        unsafe {
            libc::printf(
                c"execute at %p, pkey permissions are %s\n".as_ptr(),
                fault_addr,
                pkey_rights(rights),
            );
            asm!("mtctr {0}; bctrl", in(reg) insns);
        }
        FAIL_IF!(unsafe { remaining_faults != 0 });
        if rights & unsafe { PKEY_DISABLE_EXECUTE } != 0 {
            FAIL_IF!(unsafe { fault_code != libc::SEGV_PKUERR });
        }

        /* Free the current pkey */
        unsafe {
            sys_pkey_free(pkey);
        }

        /* Find next valid combination of pkey rights */
        rights = unsafe { next_pkey_rights(rights) };
        if rights == 0 {
            break;
        }
    }

    /* Cleanup */
    unsafe {
        libc::munmap(insns as *mut c_void, pgsize as libc::size_t);
    }

    0
}

fn main() {
    unsafe {
        std::process::exit(test_harness(test, c"pkey_exec_prot".as_ptr()));
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
