/*
 * Copyright (C) 2002 - 2007 Jeff Dike (jdike@{addtoit,linux.intel}.com)
 * Licensed under the GPL
 */

// Dependencies supplied by the surrounding UML sources:
// arch.h, signal.h, kern_util.h, longjmp.h, sysdep/ptrace.h,
// generated/asm-offsets.h

/* Set during early boot */
static mut host_has_cmov: i32 = 1;
static mut cmov_test_return: jmp_buf = unsafe { core::mem::zeroed() };

unsafe extern "C" {
    fn longjmp(env: *mut jmp_buf, value: i32) -> !;
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn sigemptyset(set: *mut sigset_t) -> i32;
    fn sigaction(signum: i32, act: *const sigaction, oldact: *mut sigaction) -> i32;
    fn setjmp(env: *mut jmp_buf) -> i32;
    fn get_current_pid() -> i32;
    fn copy_from_user_proc(to: *mut u8, from: *const core::ffi::c_void, n: usize) -> i32;
    fn UPT_IP(regs: *mut uml_pt_regs) -> usize;
}

unsafe fn cmov_sigill_test_handler(_sig: i32) {
    host_has_cmov = 0;
    longjmp(&mut cmov_test_return, 1);
}

pub unsafe fn arch_check_bugs() {
    let mut old: sigaction = core::mem::zeroed();
    let mut new: sigaction = core::mem::zeroed();

    printk(b"Checking for host processor cmov support...\0".as_ptr() as *const _);
    new.sa_handler = Some(cmov_sigill_test_handler);

    /* Make sure that SIGILL is enabled after the handler longjmps back */
    new.sa_flags = SA_NODEFER;
    sigemptyset(&mut new.sa_mask);
    sigaction(SIGILL, &new, &mut old);

    if setjmp(&mut cmov_test_return) == 0 {
        let mut foo: usize = 0;
        // C: __asm__ __volatile__("cmovz %0, %1" : "=r" (foo) : "0" (foo));
        core::arch::asm!("cmovz {0}, {0}", inout(reg) foo);
        printk(b"Yes\n\0".as_ptr() as *const _);
    } else {
        printk(b"No\n\0".as_ptr() as *const _);
    }

    sigaction(SIGILL, &old, &mut new);
}

pub unsafe fn arch_examine_signal(sig: i32, regs: *mut uml_pt_regs) {
    let mut tmp: [u8; 2] = [0; 2];

    /*
     * This is testing for a cmov (0x0f 0x4x) instruction causing a
     * SIGILL in init.
     */
    if (sig != SIGILL) || (get_current_pid() != 1) {
        return;
    }

    if copy_from_user_proc(tmp.as_mut_ptr(), UPT_IP(regs) as *const _, 2) != 0 {
        printk(b"SIGILL in init, could not read instructions!\n\0".as_ptr() as *const _);
        return;
    }

    if (tmp[0] != 0x0f) || ((tmp[1] & 0xf0) != 0x40) {
        return;
    }

    if host_has_cmov == 0 {
        printk(b"SIGILL caused by cmov, which this processor doesn't implement.  Boot a filesystem compiled for older processors\0".as_ptr() as *const _);
    } else if host_has_cmov == 1 {
        printk(b"SIGILL caused by cmov, which this processor claims to implement\0".as_ptr() as *const _);
    } else {
        printk(b"Bad value for host_has_cmov (%d)\0".as_ptr() as *const _, host_has_cmov);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
