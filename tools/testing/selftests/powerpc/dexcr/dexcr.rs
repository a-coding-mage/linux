// SPDX-License-Identifier: GPL-2.0+

// C dependencies removed from executable Rust:
// <errno.h>, <setjmp.h>, <signal.h>, <sys/prctl.h>, <sys/types.h>,
// <sys/wait.h>, "dexcr.h", "reg.h", "utils.h".

use core::ffi::{c_int, c_ulong, c_void};

type pid_t = i32;

// External C/library and selftest dependencies supplied by the surrounding tree.
type jmp_buf = [c_int; 1];

#[repr(C)]
pub struct siginfo_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigaction {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn longjmp(env: *mut jmp_buf, val: c_int) -> !;
    fn setjmp(env: *mut jmp_buf) -> c_int;
    fn prctl(option: c_int, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;

    fn push_signal_handler(
        signum: c_int,
        handler: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    ) -> sigaction;
    fn pop_signal_handler(signum: c_int, old: sigaction);

    fn mfspr(sprn: c_ulong) -> u32;
    fn do_bad_hashchk();
    fn FAIL_IF_EXIT_MSG(condition: bool, msg: *const u8) -> !;
}

unsafe extern "C" {
    static SIGILL: c_int;
    static ENODEV: c_int;
    static PR_PPC_GET_DEXCR: c_int;
    static PR_PPC_SET_DEXCR: c_int;
    static PR_PPC_DEXCR_SBHE: c_ulong;
    static PR_PPC_DEXCR_IBRTPD: c_ulong;
    static PR_PPC_DEXCR_SRAPD: c_ulong;
    static PR_PPC_DEXCR_NPHIE: c_ulong;
    static PR_PPC_DEXCR_CTRL_EDITABLE: c_int;
    static SPRN_DEXCR_RO: c_ulong;
    static SPRN_HDEXCR_RO: c_ulong;
    static DEXCR_PR_SBHE: u32;
    static DEXCR_PR_IBRTPD: u32;
    static DEXCR_PR_SRAPD: u32;
    static DEXCR_PR_NPHIE: u32;
}

static mut generic_signal_jump_buf: jmp_buf = [0; 1];

unsafe extern "C" fn generic_signal_handler(
    _signum: c_int,
    _info: *mut siginfo_t,
    _context: *mut c_void,
) {
    unsafe {
        longjmp(&raw mut generic_signal_jump_buf, 0);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dexcr_exists() -> bool {
    let old: sigaction;
    let mut exists: bool;

    unsafe {
        old = push_signal_handler(SIGILL, generic_signal_handler);
        if setjmp(&raw mut generic_signal_jump_buf) != 0 {
            pop_signal_handler(SIGILL, old);
            return exists;
        }

        /*
         * If the SPR is not recognised by the hardware it triggers
         * a hypervisor emulation interrupt. If the kernel does not
         * recognise/try to emulate it, we receive a SIGILL signal.
         *
         * If we do not receive a signal, assume we have the SPR or the
         * kernel is trying to emulate it correctly.
         */
        exists = false;
        mfspr(SPRN_DEXCR_RO);
        exists = true;

        pop_signal_handler(SIGILL, old);
        exists
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pr_which_to_aspect(which: c_ulong) -> u32 {
    unsafe {
        if which == PR_PPC_DEXCR_SBHE {
            return DEXCR_PR_SBHE;
        }
        if which == PR_PPC_DEXCR_IBRTPD {
            return DEXCR_PR_IBRTPD;
        }
        if which == PR_PPC_DEXCR_SRAPD {
            return DEXCR_PR_SRAPD;
        }
        if which == PR_PPC_DEXCR_NPHIE {
            return DEXCR_PR_NPHIE;
        }
        FAIL_IF_EXIT_MSG(true, c"unknown PR aspect".as_ptr() as *const u8);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pr_get_dexcr(which: c_ulong) -> c_int {
    unsafe { prctl(PR_PPC_GET_DEXCR, which, 0, 0, 0) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pr_set_dexcr(which: c_ulong, ctrl: c_ulong) -> c_int {
    unsafe { prctl(PR_PPC_SET_DEXCR, which, ctrl, 0, 0) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pr_dexcr_aspect_supported(which: c_ulong) -> bool {
    unsafe {
        if pr_get_dexcr(which) == -1 {
            return errno == ENODEV;
        }

        true
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn pr_dexcr_aspect_editable(which: c_ulong) -> bool {
    unsafe { (pr_get_dexcr(which) & PR_PPC_DEXCR_CTRL_EDITABLE) != 0 }
}

/*
 * Just test if a bad hashchk triggers a signal, without checking
 * for support or if the NPHIE aspect is enabled.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hashchk_triggers() -> bool {
    let old: sigaction;
    let mut triggers: bool;

    unsafe {
        old = push_signal_handler(SIGILL, generic_signal_handler);
        if setjmp(&raw mut generic_signal_jump_buf) != 0 {
            pop_signal_handler(SIGILL, old);
            return triggers;
        }

        triggers = true;
        do_bad_hashchk();
        triggers = false;

        pop_signal_handler(SIGILL, old);
        triggers
    }
}

#[repr(C)]
pub enum dexcr_source {
    DEXCR,
    HDEXCR,
    EFFECTIVE,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_dexcr(source: dexcr_source) -> u32 {
    unsafe {
        match source {
            dexcr_source::DEXCR => mfspr(SPRN_DEXCR_RO),
            dexcr_source::HDEXCR => mfspr(SPRN_HDEXCR_RO),
            dexcr_source::EFFECTIVE => mfspr(SPRN_DEXCR_RO) | mfspr(SPRN_HDEXCR_RO),
        }
    }
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn await_child_success(pid: pid_t) {
    let mut wstatus: c_int = 0;

    unsafe {
        if pid == -1 {
            FAIL_IF_EXIT_MSG(true, c"fork failed".as_ptr() as *const u8);
        }
        if waitpid(pid, &mut wstatus, 0) == -1 {
            FAIL_IF_EXIT_MSG(true, c"wait failed".as_ptr() as *const u8);
        }
        if !WIFEXITED(wstatus) {
            FAIL_IF_EXIT_MSG(true, c"child did not exit cleanly".as_ptr() as *const u8);
        }
        if WEXITSTATUS(wstatus) != 0 {
            FAIL_IF_EXIT_MSG(true, c"child exit error".as_ptr() as *const u8);
        }
    }
}

/*
 * Perform a hashst instruction. The following components determine the result
 *
 * 1. The LR value (any register technically)
 * 2. The SP value (also any register, but it must be a valid address)
 * 3. A secret key managed by the kernel
 *
 * The result is stored to the address held in SP.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hashst(lr: c_ulong, sp: *mut c_void) {
    unsafe {
        core::arch::asm!(
            "addi 31, {0}, 0",       /* set r31 (pretend LR) to lr */
            "addi 30, {1}, 8",       /* set r30 (pretend SP) to sp + 8 */
            ".long 0x7c0005e4",      /* PPC_RAW_HASHST(31, -8, 30): compute hash into stack location */
            in(reg) lr,
            in(reg) sp,
            lateout("r31") _,
            lateout("r30") _,
            options(nostack, preserves_flags),
        );
    }
}

/*
 * Perform a hashchk instruction. A hash is computed as per hashst(),
 * however the result is not stored to memory. Instead the existing
 * value is read and compared against the computed hash.
 *
 * If they match, execution continues.
 * If they differ, an interrupt triggers.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn hashchk(lr: c_ulong, sp: *mut c_void) {
    unsafe {
        core::arch::asm!(
            "addi 31, {0}, 0",       /* set r31 (pretend LR) to lr */
            "addi 30, {1}, 8",       /* set r30 (pretend SP) to sp + 8 */
            ".long 0x7c0005e4",      /* PPC_RAW_HASHCHK(31, -8, 30): check hash at stack location */
            in(reg) lr,
            in(reg) sp,
            lateout("r31") _,
            lateout("r30") _,
            options(nostack, preserves_flags),
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_bad_hashchk() {
    let mut hash: c_ulong = 0;

    unsafe {
        hashst(0, (&mut hash as *mut c_ulong).cast::<c_void>());
        hash = hash.wrapping_add(1);
        hashchk(0, (&mut hash as *mut c_ulong).cast::<c_void>());
    }
}
