// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 * Copyright (C) 2015 Nikolay Martynov <mar.kolya@gmail.com>
 * Copyright (C) 2015 John Crispin <john@phrozen.org>
 */

// Dependencies supplied by the surrounding kernel build.
unsafe extern "C" {
    fn gic_get_c0_perfcount_int() -> i32;
    fn irqchip_init();
}

#[no_mangle]
pub extern "C" fn get_c0_perfcount_int() -> i32 {
    unsafe { gic_get_c0_perfcount_int() }
}

// __init
#[no_mangle]
pub extern "C" fn arch_init_irq() {
    unsafe { irqchip_init() }
}

// EXPORT_SYMBOL_GPL(get_c0_perfcount_int);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
