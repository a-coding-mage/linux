// SPDX-License-Identifier: GPL-2.0-only
/*
 * Mostly IRQ support for Hexagon
 *
 * Copyright (c) 2010-2012, The Linux Foundation. All rights reserved.
 */

// Dependencies supplied by the surrounding kernel translation.

extern "C" {
    fn show_regs_print_info(loglevel: *const core::ffi::c_char);
    fn printk(fmt: *const core::ffi::c_char, ...);
    fn pt_elr(regs: *mut pt_regs) -> ::core::ffi::c_ulong;
    fn pt_cause(regs: *mut pt_regs) -> ::core::ffi::c_ulong;
    fn user_mode(regs: *mut pt_regs) -> ::core::ffi::c_int;
    fn pt_psp(regs: *mut pt_regs) -> ::core::ffi::c_ulong;
    fn pt_badva(regs: *mut pt_regs) -> ::core::ffi::c_ulong;
    fn ints_enabled(regs: *mut pt_regs) -> ::core::ffi::c_int;
    fn set_irq_regs(regs: *mut pt_regs) -> *mut pt_regs;
    fn irq_enter();
    fn generic_handle_irq(irq: ::core::ffi::c_int);
    fn irq_exit();
}

// `struct pt_regs` and its fields are supplied by the architecture register definitions.
pub use crate::pt_regs;

const KERN_EMERG: &[u8] = b"<0>\0";

pub unsafe fn show_regs(regs: *mut pt_regs) {
    show_regs_print_info(KERN_EMERG.as_ptr() as *const core::ffi::c_char);

    printk(b"<0>restart_r0: \t0x%08lx   syscall_nr: %ld\n\0".as_ptr() as _, (*regs).restart_r0, (*regs).syscall_nr);
    printk(b"<0>preds: \t\t0x%08lx\n\0".as_ptr() as _, (*regs).preds);
    printk(b"<0>lc0: \t0x%08lx   sa0: 0x%08lx   m0:  0x%08lx\n\0".as_ptr() as _, (*regs).lc0, (*regs).sa0, (*regs).m0);
    printk(b"<0>lc1: \t0x%08lx   sa1: 0x%08lx   m1:  0x%08lx\n\0".as_ptr() as _, (*regs).lc1, (*regs).sa1, (*regs).m1);
    printk(b"<0>gp: \t0x%08lx   ugp: 0x%08lx   usr: 0x%08lx\n\0".as_ptr() as _, (*regs).gp, (*regs).ugp, (*regs).usr);
    printk(b"<0>cs0: \t0x%08lx   cs1: 0x%08lx\n\0".as_ptr() as _, (*regs).cs0, (*regs).cs1);
    printk(b"<0>r0: \t0x%08lx %08lx %08lx %08lx\n\0".as_ptr() as _, (*regs).r00, (*regs).r01, (*regs).r02, (*regs).r03);
    printk(b"<0>r4:  \t0x%08lx %08lx %08lx %08lx\n\0".as_ptr() as _, (*regs).r04, (*regs).r05, (*regs).r06, (*regs).r07);
    printk(b"<0>r8:  \t0x%08lx %08lx %08lx %08lx\n\0".as_ptr() as _, (*regs).r08, (*regs).r09, (*regs).r10, (*regs).r11);
    printk(b"<0>r12: \t0x%08lx %08lx %08lx %08lx\n\0".as_ptr() as _, (*regs).r12, (*regs).r13, (*regs).r14, (*regs).r15);
    printk(b"<0>r16: \t0x%08lx %08lx %08lx %08lx\n\0".as_ptr() as _, (*regs).r16, (*regs).r17, (*regs).r18, (*regs).r19);
    printk(b"<0>r20: \t0x%08lx %08lx %08lx %08lx\n\0".as_ptr() as _, (*regs).r20, (*regs).r21, (*regs).r22, (*regs).r23);
    printk(b"<0>r24: \t0x%08lx %08lx %08lx %08lx\n\0".as_ptr() as _, (*regs).r24, (*regs).r25, (*regs).r26, (*regs).r27);
    printk(b"<0>r28: \t0x%08lx %08lx %08lx %08lx\n\0".as_ptr() as _, (*regs).r28, (*regs).r29, (*regs).r30, (*regs).r31);
    printk(b"<0>elr: \t0x%08lx   cause: 0x%08lx   user_mode: %d\n\0".as_ptr() as _, pt_elr(regs), pt_cause(regs), user_mode(regs));
    printk(b"<0>psp: \t0x%08lx   badva: 0x%08lx   int_enabled: %d\n\0".as_ptr() as _, pt_psp(regs), pt_badva(regs), ints_enabled(regs));
}

pub unsafe fn arch_do_IRQ(regs: *mut pt_regs) {
    let irq: ::core::ffi::c_int = pt_cause(regs) as ::core::ffi::c_int;
    let old_regs: *mut pt_regs = set_irq_regs(regs);

    irq_enter();
    generic_handle_irq(irq);
    irq_exit();
    set_irq_regs(old_regs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
