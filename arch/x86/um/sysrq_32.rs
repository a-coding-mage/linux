/*
 * Copyright (C) 2001 - 2003 Jeff Dike (jdike@addtoit.com)
 * Licensed under the GPL
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_ulong};

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn printk(fmt: *const c_char, ...);
    fn smp_processor_id() -> c_int;
    fn print_tainted() -> *const c_char;

    fn PT_REGS_CS(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_IP(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_SS(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_SP(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_EFLAGS(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_AX(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_BX(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_CX(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_DX(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_SI(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_DI(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_BP(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_DS(regs: *mut pt_regs) -> c_ulong;
    fn PT_REGS_ES(regs: *mut pt_regs) -> c_ulong;
}

/* This is declared by <linux/sched.h> */
#[no_mangle]
pub unsafe extern "C" fn show_regs(regs: *mut pt_regs) {
    printk(b"\n\0".as_ptr() as *const c_char);
    printk(
        b"EIP: %04lx:[<%08lx>] CPU: %d %s\0".as_ptr() as *const c_char,
        0xffff & PT_REGS_CS(regs),
        PT_REGS_IP(regs),
        smp_processor_id(),
        print_tainted(),
    );
    if PT_REGS_CS(regs) & 3 != 0 {
        printk(
            b" ESP: %04lx:%08lx\0".as_ptr() as *const c_char,
            0xffff & PT_REGS_SS(regs),
            PT_REGS_SP(regs),
        );
    }
    printk(
        b" EFLAGS: %08lx\n    %s\n\0".as_ptr() as *const c_char,
        PT_REGS_EFLAGS(regs),
        print_tainted(),
    );
    printk(
        b"EAX: %08lx EBX: %08lx ECX: %08lx EDX: %08lx\n\0".as_ptr() as *const c_char,
        PT_REGS_AX(regs),
        PT_REGS_BX(regs),
        PT_REGS_CX(regs),
        PT_REGS_DX(regs),
    );
    printk(
        b"ESI: %08lx EDI: %08lx EBP: %08lx\0".as_ptr() as *const c_char,
        PT_REGS_SI(regs),
        PT_REGS_DI(regs),
        PT_REGS_BP(regs),
    );
    printk(
        b" DS: %04lx ES: %04lx\n\0".as_ptr() as *const c_char,
        0xffff & PT_REGS_DS(regs),
        0xffff & PT_REGS_ES(regs),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
