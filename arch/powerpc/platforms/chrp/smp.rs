// SPDX-License-Identifier: GPL-2.0
/*
 * Smp support for CHRP machines.
 *
 * Written by Cort Dougan (cort@cs.nmt.edu) borrowing a great
 * deal of code from the sparc and intel versions.
 *
 * Copyright (C) 1999 Cort Dougan <cort@cs.nmt.edu>
 *
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/kernel.h, linux/sched.h, linux/smp.h, linux/interrupt.h,
// linux/kernel_stat.h, linux/delay.h, linux/spinlock.h, linux/pgtable.h,
// asm/ptrace.h, linux/atomic.h, asm/irq.h, asm/page.h, asm/sections.h,
// asm/io.h, asm/smp.h, asm/machdep.h, asm/mpic.h, asm/rtas.h

extern "C" {
    fn mpic_setup_this_cpu();
    fn smp_mpic_message_pass();
    fn smp_mpic_probe();
    fn rtas_give_timebase();
    fn rtas_take_timebase();
}

// Supplied by the surrounding kernel translation.
extern "C" {
    static KERNELBASE: usize;
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct smp_ops_t {
    pub cause_nmi_ipi: Option<unsafe extern "C" fn()>,
    pub message_pass: unsafe extern "C" fn(),
    pub probe: unsafe extern "C" fn(),
    pub kick_cpu: unsafe extern "C" fn(i32) -> i32,
    pub setup_cpu: unsafe extern "C" fn(i32),
    pub give_timebase: unsafe extern "C" fn(),
    pub take_timebase: unsafe extern "C" fn(),
}

unsafe fn smp_chrp_kick_cpu(nr: i32) -> i32 {
    *(KERNELBASE as *mut usize) = nr as usize;

    // The C source uses: asm volatile("dcbf 0,%0"::"r"(KERNELBASE):"memory");
    core::arch::asm!("dcbf 0, {addr}", addr = in(reg) KERNELBASE, options(nostack));

    0
}

unsafe fn smp_chrp_setup_cpu(_cpu_nr: i32) {
    mpic_setup_this_cpu();
}

/* CHRP with openpic */
#[no_mangle]
pub static mut chrp_smp_ops: smp_ops_t = smp_ops_t {
    cause_nmi_ipi: None,
    message_pass: smp_mpic_message_pass,
    probe: smp_mpic_probe,
    kick_cpu: smp_chrp_kick_cpu,
    setup_cpu: smp_chrp_setup_cpu,
    give_timebase: rtas_give_timebase,
    take_timebase: rtas_take_timebase,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
