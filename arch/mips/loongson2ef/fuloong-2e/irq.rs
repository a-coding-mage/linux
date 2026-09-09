// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007 Lemote Inc. & Institute of Computing Technology
 * Author: Fuxin Zhang, zhangfx@lemote.com
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn i8259_irqdispatch() {
    let irq: ::core::ffi::c_int;

    irq = i8259_irq();
    if irq >= 0 {
        do_IRQ(irq);
    } else {
        spurious_interrupt();
    }
}

pub unsafe extern "C" fn mach_irq_dispatch(pending: ::core::ffi::c_uint) {
    if pending & CAUSEF_IP7 != 0 {
        do_IRQ(MIPS_CPU_IRQ_BASE + 7);
    } else if pending & CAUSEF_IP6 != 0 {
        /* perf counter loverflow */
        return;
    } else if pending & CAUSEF_IP5 != 0 {
        i8259_irqdispatch();
    } else if pending & CAUSEF_IP2 != 0 {
        bonito_irqdispatch();
    } else {
        spurious_interrupt();
    }
}

pub unsafe extern "C" fn mach_init_irq() {
    let mut irq: ::core::ffi::c_int;

    /* init all controller
     *   0-15       ------> i8259 interrupt
     *   16-23      ------> mips cpu interrupt
     *   32-63      ------> bonito irq
     */

    /* most bonito irq should be level triggered */
    LOONGSON_INTEDGE = LOONGSON_ICU_SYSTEMERR
        | LOONGSON_ICU_MASTERERR
        | LOONGSON_ICU_RETRYERR
        | LOONGSON_ICU_MBOXES;

    /* Sets the first-level interrupt dispatcher. */
    mips_cpu_irq_init();
    init_i8259_irqs();
    bonito_irq_init();

    /* bonito irq at IP2 */
    irq = MIPS_CPU_IRQ_BASE + 2;
    if request_irq(
        irq,
        no_action,
        IRQF_NO_THREAD,
        b"cascade\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut(),
    ) != 0 {
        pr_err(b"Failed to request irq %d (cascade)\n\0".as_ptr(), irq);
    }
    /* 8259 irq at IP5 */
    irq = MIPS_CPU_IRQ_BASE + 5;
    if request_irq(
        irq,
        no_action,
        IRQF_NO_THREAD,
        b"cascade\0".as_ptr() as *const ::core::ffi::c_char,
        ::core::ptr::null_mut(),
    ) != 0 {
        pr_err(b"Failed to request irq %d (cascade)\n\0".as_ptr(), irq);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
