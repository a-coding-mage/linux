/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2006 Atmark Techno, Inc.
 */

/* Virtual address corresponding to last available physical memory address. */
/*
 * C preprocessor macro translated as a Rust expression macro so that the
 * externally supplied CONFIG_KERNEL_START and memory_size symbols retain
 * their original meaning and evaluation behavior.
 */
#[macro_export]
macro_rules! MAX_DMA_ADDRESS {
    () => {
        CONFIG_KERNEL_START + memory_size - 1
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
