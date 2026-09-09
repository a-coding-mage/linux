/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020 ARM Ltd.
 */

/* C header guard: __ASM_VDSO_PROCESSOR_H */

/* The original declaration is excluded for assembler builds. */

/*
 * Loongson-3's SFB (Store-Fill-Buffer) may buffer writes indefinitely when a
 * tight read loop is executed, because reads take priority over writes & the
 * hardware (incorrectly) doesn't ensure that writes will eventually occur.
 *
 * Since spin loops of any kind should have a cpu_relax() in them, force an SFB
 * flush from cpu_relax() such that any pending writes will become visible as
 * expected.
 *
 * CONFIG_CPU_LOONGSON64 is represented here as a build-time cfg feature.
 */
#[cfg(feature = "CONFIG_CPU_LOONGSON64")]
#[macro_export]
macro_rules! cpu_relax {
    () => {
        smp_mb()
    };
}

#[cfg(not(feature = "CONFIG_CPU_LOONGSON64"))]
#[macro_export]
macro_rules! cpu_relax {
    () => {
        barrier()
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
