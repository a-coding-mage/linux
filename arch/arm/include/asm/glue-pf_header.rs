/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/glue-pf.h
 *
 *  Copyright (C) 1997-1999 Russell King
 *  Copyright (C) 2000-2002 Deep Blue Solutions Ltd.
 */

/*
 * Prefetch Abort Model
 * ====================
 *
 * We have the following to choose from:
 *   legacy - no IFSR, no IFAR
 *   v6     - ARMv6: IFSR, no IFAR
 *   v7     - ARMv7: IFSR and IFAR
 *
 * The original header derives CPU_PABORT_HANDLER and MULTI_PABORT from
 * build-time CONFIG_CPU_PABRT_* definitions.  Rust build configurations
 * corresponding to those definitions are represented by the feature names
 * below.
 */

#[cfg(all(
    feature = "CONFIG_CPU_PABRT_LEGACY",
    not(any(
        feature = "CONFIG_CPU_PABRT_V6",
        feature = "CONFIG_CPU_PABRT_V7"
    ))
))]
macro_rules! CPU_PABORT_HANDLER {
    () => { legacy_pabort };
}

#[cfg(all(
    feature = "CONFIG_CPU_PABRT_V6",
    not(any(
        feature = "CONFIG_CPU_PABRT_LEGACY",
        feature = "CONFIG_CPU_PABRT_V7"
    ))
))]
macro_rules! CPU_PABORT_HANDLER {
    () => { v6_pabort };
}

#[cfg(all(
    feature = "CONFIG_CPU_PABRT_V7",
    not(any(
        feature = "CONFIG_CPU_PABRT_LEGACY",
        feature = "CONFIG_CPU_PABRT_V6"
    ))
))]
macro_rules! CPU_PABORT_HANDLER {
    () => { v7_pabort };
}

/* Multiple configured prefetch-abort models select the multi-handler path. */
#[cfg(any(
    all(feature = "CONFIG_CPU_PABRT_LEGACY", feature = "CONFIG_CPU_PABRT_V6"),
    all(feature = "CONFIG_CPU_PABRT_LEGACY", feature = "CONFIG_CPU_PABRT_V7"),
    all(feature = "CONFIG_CPU_PABRT_V6", feature = "CONFIG_CPU_PABRT_V7")
))]
#[allow(non_upper_case_globals)]
pub const MULTI_PABORT: usize = 1;

#[cfg(not(any(
    feature = "CONFIG_CPU_PABRT_LEGACY",
    feature = "CONFIG_CPU_PABRT_V6",
    feature = "CONFIG_CPU_PABRT_V7"
)))]
compile_error!("Unknown prefetch abort handler type");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
