/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// The original declaration is excluded when assembling (__ASSEMBLER__).
// In Rust this translation is intended for the non-assembler context.
// `barrier` is supplied by the surrounding dependencies.
#[macro_export]
macro_rules! cpu_relax {
    () => {
        barrier()
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
