/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Generic IRQ handling for the SA11x0.
 *
 * Copyright (C) 2015 Dmitry Eremin-Solenikov
 * Copyright (C) 1999-2001 Nicolas Pitre
 */

// `__init` is a kernel build-time annotation with no direct Rust equivalent.
// `resource_size_t` is supplied by the surrounding kernel translation.
extern "C" {
    pub fn sa11x0_init_irq_nodt(irq_start: i32, io_start: resource_size_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
