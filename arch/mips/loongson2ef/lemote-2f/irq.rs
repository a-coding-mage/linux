// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2007 Lemote Inc.
 * Author: Fuxin Zhang, zhangfx@lemote.com
 */

// Dependencies supplied by the surrounding kernel sources are intentionally
// referenced here without reproducing their implementations.

const LOONGSON_TIMER_IRQ: i32 = MIPS_CPU_IRQ_BASE + 7; // cpu timer
const LOONGSON_NORTH_BRIDGE_IRQ: i32 = MIPS_CPU_IRQ_BASE + 6; // bonito
const LOONGSON_UART_IRQ: i32 = MIPS_CPU_IRQ_BASE + 3; // cpu serial port
const LOONGSON_SOUTH_BRIDGE_IRQ: i32 = MIPS_CPU_IRQ_BASE + 2; // i8259

const LOONGSON_INT_BIT_INT0: i32 = 1 << 11;
const LOONGSON_INT_BIT_INT1: i32 = 1 << 12;

/*
 * The generic i8259_irq() make the kernel hang on booting. Since we cannot
 * get the irq via the IRR directly, we access the ISR instead.
 */
pub unsafe fn mach_i8259_irq() -> i32 {
    let mut irq: i32 = -1;
    let mut isr: i32;

    if ((LOONGSON_INTISR & LOONGSON_INTEN) & LOONGSON_INT_BIT_INT0) != 0 {
        raw_spin_lock(&mut i8259A_lock);
        isr = (inb(PIC_MASTER_CMD) as i32)
            & !(inb(PIC_MASTER_IMR) as i32)
            & !(1 << PIC_CASCADE_IR);
        if isr == 0 {
            isr = ((inb(PIC_SLAVE_CMD) as i32) & !(inb(PIC_SLAVE_IMR) as i32)) << 8;
        }
        irq = ffs(isr) - 1;
        if irq == 7 {
            /*
             * This may be a spurious interrupt.
             *
             * Read the interrupt status register (ISR). If the most
             * significant bit is not set then there is no valid interrupt.
             */
            outb(0x0B, PIC_MASTER_ISR); // ISR register
            if !(inb(PIC_MASTER_ISR) as i32) & 0x80 != 0 {
                irq = -1;
            }
        }
        raw_spin_unlock(&mut i8259A_lock);
    }

    irq
}

static unsafe fn i8259_irqdispatch() {
    let irq: i32 = mach_i8259_irq();
    if irq >= 0 {
        do_IRQ(irq);
    } else {
        spurious_interrupt();
    }
}

pub unsafe fn mach_irq_dispatch(pending: u32) {
    if pending & CAUSEF_IP7 != 0 {
        do_IRQ(LOONGSON_TIMER_IRQ);
    } else if pending & CAUSEF_IP6 != 0 {
        // North Bridge, Perf counter
        bonito_irqdispatch();
    } else if pending & CAUSEF_IP3 != 0 {
        // CPU UART
        do_IRQ(LOONGSON_UART_IRQ);
    } else if pending & CAUSEF_IP2 != 0 {
        // South Bridge
        i8259_irqdispatch();
    } else {
        spurious_interrupt();
    }
}

unsafe fn ip6_action(_cpl: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    IRQ_HANDLED
}

pub unsafe fn mach_init_irq() {
    /* init all controller
     *   0-15      ------> i8259 interrupt
     *   16-23     ------> mips cpu interrupt
     *   32-63     ------> bonito irq
     */

    // setup cs5536 as high level trigger
    LOONGSON_INTPOL = LOONGSON_INT_BIT_INT0 | LOONGSON_INT_BIT_INT1;
    LOONGSON_INTEDGE &= !(LOONGSON_INT_BIT_INT0 | LOONGSON_INT_BIT_INT1);

    // Sets the first-level interrupt dispatcher.
    mips_cpu_irq_init();
    init_i8259_irqs();
    bonito_irq_init();

    // setup north bridge irq (bonito)
    if request_irq(
        LOONGSON_NORTH_BRIDGE_IRQ,
        ip6_action,
        IRQF_SHARED | IRQF_NO_THREAD,
        "cascade",
        ip6_action as *mut core::ffi::c_void,
    ) != 0 {
        pr_err!("Failed to register north bridge cascade interrupt\n");
    }
    // setup source bridge irq (i8259)
    if request_irq(
        LOONGSON_SOUTH_BRIDGE_IRQ,
        no_action,
        IRQF_NO_THREAD | IRQF_NO_SUSPEND,
        "cascade",
        core::ptr::null_mut(),
    ) != 0 {
        pr_err!("Failed to register south bridge cascade interrupt\n");
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
