/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * Copyright (C) 2015, Dmitry Eremin-Solenikov
 */

pub unsafe extern "C" {
    pub fn sa11x0_gpio_set_wake(gpio: u32, on: u32) -> i32;
    pub fn sa11x0_sc_set_wake(irq: u32, on: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
