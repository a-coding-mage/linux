/*
 * Copyright 2003 PathScale, Inc.
 *
 * Licensed under the GPL
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};

/* Declarations supplied by the surrounding kernel environment. */
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

#[repr(C)]
pub struct task_struct {
    pub comm: [c_char; 16],
}

#[repr(C)]
pub struct utsname {
    pub release: [c_char; 65],
}

extern "C" {
    fn printk(fmt: *const c_char, ...) -> c_int;
    fn print_modules();
    fn task_pid_nr(task: *mut task_struct) -> c_int;
    fn print_tainted() -> *const c_char;
    fn init_utsname() -> *mut utsname;

    /* PT_REGS_* are the architecture-specific register accessors/macros. */
    fn PT_REGS_CS(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_IP(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_SP(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_EFLAGS(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_AX(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_BX(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_CX(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_DX(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_SI(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_DI(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_BP(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_R8(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_R9(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_R10(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_R11(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_R12(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_R13(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_R14(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_R15(regs: *mut pt_regs) -> c_ulong;

    static mut current: *mut task_struct;
}

pub unsafe fn show_regs(regs: *mut pt_regs) {
    printk(b"\n\0".as_ptr() as *const c_char);
    print_modules();
    printk(
        b"Pid: %d, comm: %.20s %s %s\n\0".as_ptr() as *const c_char,
        task_pid_nr(current),
        (*current).comm.as_ptr(),
        print_tainted(),
        (*init_utsname()).release.as_ptr(),
    );
    printk(
        b"RIP: %04lx:%pS\n\0".as_ptr() as *const c_char,
        PT_REGS_CS(regs) & 0xffff,
        PT_REGS_IP(regs) as *const c_void,
    );
    printk(
        b"RSP: %016lx  EFLAGS: %08lx\n\0".as_ptr() as *const c_char,
        PT_REGS_SP(regs),
        PT_REGS_EFLAGS(regs),
    );
    printk(b"RAX: %016lx RBX: %016lx RCX: %016lx\n\0".as_ptr() as *const c_char,
        PT_REGS_AX(regs), PT_REGS_BX(regs), PT_REGS_CX(regs));
    printk(b"RDX: %016lx RSI: %016lx RDI: %016lx\n\0".as_ptr() as *const c_char,
        PT_REGS_DX(regs), PT_REGS_SI(regs), PT_REGS_DI(regs));
    printk(b"RBP: %016lx R08: %016lx R09: %016lx\n\0".as_ptr() as *const c_char,
        PT_REGS_BP(regs), PT_REGS_R8(regs), PT_REGS_R9(regs));
    printk(b"R10: %016lx R11: %016lx R12: %016lx\n\0".as_ptr() as *const c_char,
        PT_REGS_R10(regs), PT_REGS_R11(regs), PT_REGS_R12(regs));
    printk(b"R13: %016lx R14: %016lx R15: %016lx\n\0".as_ptr() as *const c_char,
        PT_REGS_R13(regs), PT_REGS_R14(regs), PT_REGS_R15(regs));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
