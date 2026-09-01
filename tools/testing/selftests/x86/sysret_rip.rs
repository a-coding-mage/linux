// SPDX-License-Identifier: GPL-2.0-only
/*
 * sigreturn.c - tests that x86 avoids Intel SYSRET pitfalls
 * Copyright (c) 2014-2016 Andrew Lutomirski
 */

/*
 * C source included:
 * _GNU_SOURCE, stdlib.h, unistd.h, stdio.h, string.h, inttypes.h,
 * sys/signal.h, sys/ucontext.h, sys/syscall.h, err.h, stddef.h, stdbool.h,
 * setjmp.h, sys/user.h, sys/mman.h, assert.h, and "helpers.h".
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem;
use core::ptr;

/*
 * These items are in clang_helpers_64.S, in order to avoid clang inline asm
 * limitations:
 */
unsafe extern "C" {
    fn test_syscall_ins();
    static test_page: [c_char; 0];

    fn printf(format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut c_void) -> c_int;
    fn _exit(status: c_int) -> !;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn raise(sig: c_int) -> c_int;
    fn mremap(
        old_address: *mut c_void,
        old_size: usize,
        new_size: usize,
        flags: c_int,
        new_address: *mut c_void,
    ) -> *mut c_void;
    fn err(eval: c_int, fmt: *const c_char, ...) -> !;
    fn errx(eval: c_int, fmt: *const c_char, ...) -> !;
    fn sigsetjmp(env: *mut libc::sigjmp_buf, savesigs: c_int) -> c_int;
    fn siglongjmp(env: *mut libc::sigjmp_buf, val: c_int) -> !;

    fn sethandler(
        sig: c_int,
        handler: unsafe extern "C" fn(c_int, *mut libc::siginfo_t, *mut c_void),
        flags: c_int,
    );
    fn clearhandler(sig: c_int);
}

static mut current_test_page_addr: *const c_void = unsafe { test_page.as_ptr() as *const c_void };

/* State used by our signal handlers. */
static mut initial_regs: libc::gregset_t = unsafe { mem::zeroed() };

static mut rip: c_ulong = 0;

unsafe fn read_rip() -> c_ulong {
    ptr::read_volatile(&raw const rip)
}

unsafe fn write_rip(value: c_ulong) {
    ptr::write_volatile(&raw mut rip, value);
}

unsafe extern "C" fn sigsegv_for_sigreturn_test(
    _sig: c_int,
    _info: *mut libc::siginfo_t,
    ctx_void: *mut c_void,
) {
    let ctx = ctx_void as *mut libc::ucontext_t;

    if read_rip() != (*ctx).uc_mcontext.gregs[libc::REG_RIP as usize] as c_ulong {
        printf(
            c"[FAIL]\tRequested RIP=0x%lx but got RIP=0x%lx\n".as_ptr(),
            read_rip(),
            (*ctx).uc_mcontext.gregs[libc::REG_RIP as usize] as c_ulong,
        );
        fflush(ptr::null_mut());
        _exit(1);
    }

    memcpy(
        (*ctx).uc_mcontext.gregs.as_mut_ptr() as *mut c_void,
        (&raw const initial_regs) as *const c_void,
        mem::size_of::<libc::gregset_t>(),
    );

    printf(c"[OK]\tGot SIGSEGV at RIP=0x%lx\n".as_ptr(), read_rip());
}

unsafe extern "C" fn sigusr1(_sig: c_int, _info: *mut libc::siginfo_t, ctx_void: *mut c_void) {
    let ctx = ctx_void as *mut libc::ucontext_t;

    memcpy(
        (&raw mut initial_regs) as *mut c_void,
        (*ctx).uc_mcontext.gregs.as_ptr() as *const c_void,
        mem::size_of::<libc::gregset_t>(),
    );

    /* Set IP and CX to match so that SYSRET can happen. */
    (*ctx).uc_mcontext.gregs[libc::REG_RIP as usize] = read_rip() as _;
    (*ctx).uc_mcontext.gregs[libc::REG_RCX as usize] = read_rip() as _;

    /* R11 and EFLAGS should already match. */
    assert!(
        (*ctx).uc_mcontext.gregs[libc::REG_EFL as usize]
            == (*ctx).uc_mcontext.gregs[libc::REG_R11 as usize]
    );

    sethandler(libc::SIGSEGV, sigsegv_for_sigreturn_test, libc::SA_RESETHAND);
}

unsafe fn test_sigreturn_to(ip: c_ulong) {
    write_rip(ip);
    printf(c"[RUN]\tsigreturn to 0x%lx\n".as_ptr(), ip);
    raise(libc::SIGUSR1);
}

static mut jmpbuf: libc::sigjmp_buf = unsafe { mem::zeroed() };

unsafe extern "C" fn sigsegv_for_fallthrough(
    _sig: c_int,
    _info: *mut libc::siginfo_t,
    ctx_void: *mut c_void,
) {
    let ctx = ctx_void as *mut libc::ucontext_t;

    if read_rip() != (*ctx).uc_mcontext.gregs[libc::REG_RIP as usize] as c_ulong {
        printf(
            c"[FAIL]\tExpected SIGSEGV at 0x%lx but got RIP=0x%lx\n".as_ptr(),
            read_rip(),
            (*ctx).uc_mcontext.gregs[libc::REG_RIP as usize] as c_ulong,
        );
        fflush(ptr::null_mut());
        _exit(1);
    }

    siglongjmp((&raw mut jmpbuf), 1);
}

unsafe fn test_syscall_fallthrough_to(ip: c_ulong) {
    let new_address = ip.wrapping_sub(4096) as *mut c_void;
    let ret: *mut c_void;

    printf(
        c"[RUN]\tTrying a SYSCALL that falls through to 0x%lx\n".as_ptr(),
        ip,
    );

    ret = mremap(
        current_test_page_addr as *mut c_void,
        4096,
        4096,
        libc::MREMAP_MAYMOVE | libc::MREMAP_FIXED,
        new_address,
    );
    if ret == libc::MAP_FAILED {
        if ip <= (1_u64 << 47).wrapping_sub(libc::PAGE_SIZE as u64) as c_ulong {
            err(1, c"mremap to %p".as_ptr(), new_address);
        } else {
            printf(c"[OK]\tmremap to %p failed\n".as_ptr(), new_address);
            return;
        }
    }

    if ret != new_address {
        errx(
            1,
            c"mremap malfunctioned: asked for %p but got %p\n".as_ptr(),
            new_address,
            ret,
        );
    }

    current_test_page_addr = new_address;
    write_rip(ip);

    if sigsetjmp((&raw mut jmpbuf), 1) == 0 {
        asm!(
            "call *{syscall_insn}",
            in("rax") libc::SYS_getpid,
            syscall_insn = in(reg) ip.wrapping_sub(2),
        );
        errx(1, c"[FAIL]\tSyscall trampoline returned".as_ptr());
    }

    printf(c"[OK]\tWe survived\n".as_ptr());
}

fn main() {
    unsafe {
        /*
         * When the kernel returns from a slow-path syscall, it will
         * detect whether SYSRET is appropriate.  If it incorrectly
         * thinks that SYSRET is appropriate when RIP is noncanonical,
         * it'll crash on Intel CPUs.
         */
        sethandler(libc::SIGUSR1, sigusr1, 0);
        for i in 47..64 {
            test_sigreturn_to(1_u64.wrapping_shl(i) as c_ulong);
        }

        clearhandler(libc::SIGUSR1);

        sethandler(libc::SIGSEGV, sigsegv_for_fallthrough, 0);

        /* One extra test to check that we didn't screw up the mremap logic. */
        test_syscall_fallthrough_to(
            (1_u64 << 47).wrapping_sub(2_u64.wrapping_mul(libc::PAGE_SIZE as u64)) as c_ulong,
        );

        /* These are the interesting cases. */
        for i in 47..64 {
            test_syscall_fallthrough_to(
                (1_u64.wrapping_shl(i)).wrapping_sub(libc::PAGE_SIZE as u64) as c_ulong,
            );
            test_syscall_fallthrough_to(1_u64.wrapping_shl(i) as c_ulong);
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
