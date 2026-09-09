// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/boards/titan/setup.c - Setup for Titan
 *
 *  Copyright (C) 2006  Jamie Lenehan
 */

// C dependencies supplied by other translation units:
// #include <linux/init.h>
// #include <linux/irq.h>
// #include <mach/titan.h>
// #include <asm/io.h>

unsafe extern "C" {
    fn plat_irq_setup_pins(mode: i32);
}

unsafe fn init_titan_irq() {
    /* enable individual interrupt mode for externals */
    unsafe {
        plat_irq_setup_pins(IRQ_MODE_IRQ);
    }
}

#[allow(non_upper_case_globals)]
static mut mv_titan: sh_machine_vector = sh_machine_vector {
    mv_name: "Titan",
    mv_init_irq: Some(init_titan_irq),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
