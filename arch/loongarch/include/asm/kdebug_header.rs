/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependency intent: <linux/notifier.h>

#[repr(i32)]
pub enum die_val {
    DIE_OOPS = 1,
    DIE_RI,
    DIE_FP,
    DIE_SIMD,
    DIE_TRAP,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
