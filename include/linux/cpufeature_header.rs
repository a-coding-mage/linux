/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2014 Linaro Ltd. <ard.biesheuvel@linaro.org>
 */

// The original header is active only when CONFIG_GENERIC_CPU_AUTOPROBE is
// enabled. The declarations below preserve that build-time condition.

/*
 * Dependencies supplied by the surrounding kernel translation:
 * - cpu_feature(x): ordinal value of feature called `x`
 * - cpu_have_feature(n): whether feature `n` is available
 * - MAX_CPU_FEATURES: upper bound for feature ordinal values
 * Optional CPU_FEATURE_TYPEFMT and CPU_FEATURE_TYPEVAL are architecture
 * supplied values; their defaults in the C header are "%s" and ELF_PLATFORM.
 */

#[cfg(feature = "CONFIG_GENERIC_CPU_AUTOPROBE")]
#[allow(unused_macros)]
macro_rules! CPU_FEATURE_TYPEFMT {
    () => { "%s" };
}

#[cfg(feature = "CONFIG_GENERIC_CPU_AUTOPROBE")]
#[allow(unused_macros)]
macro_rules! CPU_FEATURE_TYPEVAL {
    () => { ELF_PLATFORM };
}

/*
 * Declare that a module is probed when CPU feature `feature` is discovered
 * and cannot be loaded when that feature is absent.
 *
 * Rust macro_rules! cannot concatenate identifiers on its own. The generated
 * module retains the C declaration's data and initialization behavior while
 * keeping the feature and initializer supplied by the caller.
 */
#[cfg(feature = "CONFIG_GENERIC_CPU_AUTOPROBE")]
#[macro_export]
macro_rules! module_cpu_feature_match {
    ($x:ident, $initfunc:path) => {
        const _: () = {
            #[allow(non_upper_case_globals, dead_code)]
            static cpu_feature_match: [cpu_feature; 2] = [
                cpu_feature { feature: cpu_feature($x) },
                cpu_feature { ..unsafe { core::mem::zeroed() } },
            ];

            #[allow(dead_code)]
            unsafe fn cpu_feature_match_init() -> i32 {
                if !cpu_have_feature(cpu_feature($x)) {
                    return -ENODEV;
                }
                $initfunc()
            }

            let _ = cpu_feature_match;
            let _ = cpu_feature_match_init;
        };
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
