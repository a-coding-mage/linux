/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependency supplied by the surrounding kernel translation:
// #include <linux/atomic.h>

unsafe extern "C" {
    pub static mut irq_err_count: atomic_t;
}

pub const ARCH_IRQ_INIT_FLAGS: _ = IRQ_NOPROBE;

/*
 * interrupt-retrigger: NOP for now. This may not be appropriate for all
 * machines, we'll see ...
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
