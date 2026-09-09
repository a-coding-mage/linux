/*
 * linux/arch/mips/tx4938/common/irq.c
 *
 * Common tx4938 irq handler
 * Copyright (C) 2000-2001 Toshiba Corporation
 *
 * 2003-2005 (c) MontaVista Software, Inc. This file is licensed under the
 * terms of the GNU General Public License version 2. This program is
 * licensed "as is" without any warranty of any kind, whether express or
 * implied.
 *
 * Support for TX4938 in 2.6 - Manish Lachwani (mlachwani@mvista.com)
 */

// Dependencies supplied by the surrounding kernel translation.
unsafe extern "C" {
    fn mips_cpu_irq_init();
    fn txx9_irq_init(base: u64);
    fn irq_set_chained_handler(irq: i32, handler: unsafe extern "C" fn());
    fn handle_simple_irq();
    fn txx9_irq_set_pri(irq: i32, priority: i32);
    fn TX4938_IR_TMR(index: i32) -> i32;
    fn TX4938_IR_SIO(index: i32) -> i32;
}

// Constants supplied by the TX4938 headers.
extern "C" {
    static TX4938_IRC_REG: u64;
    static MIPS_CPU_IRQ_BASE: i32;
    static TX4938_IRC_INT: i32;
    static TX4938_IR_ECCERR: i32;
    static TX4938_IR_WTOERR: i32;
    static TX4938_IR_PCIERR: i32;
    static TX4938_IR_PCIPME: i32;
    static TX4938_NUM_IR_TMR: i32;
    static TX4938_NUM_IR_SIO: i32;
}

// __init
pub unsafe fn tx4938_irq_init() {
    let mut i: i32;

    mips_cpu_irq_init();
    txx9_irq_init(TX4938_IRC_REG & 0xfffffffff_u64);
    irq_set_chained_handler(
        MIPS_CPU_IRQ_BASE + TX4938_IRC_INT,
        handle_simple_irq,
    );
    /* raise priority for errors, timers, SIO */
    txx9_irq_set_pri(TX4938_IR_ECCERR, 7);
    txx9_irq_set_pri(TX4938_IR_WTOERR, 7);
    txx9_irq_set_pri(TX4938_IR_PCIERR, 7);
    txx9_irq_set_pri(TX4938_IR_PCIPME, 7);
    i = 0;
    while i < TX4938_NUM_IR_TMR {
        txx9_irq_set_pri(TX4938_IR_TMR(i), 6);
        i += 1;
    }
    i = 0;
    while i < TX4938_NUM_IR_SIO {
        txx9_irq_set_pri(TX4938_IR_SIO(i), 5);
        i += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
