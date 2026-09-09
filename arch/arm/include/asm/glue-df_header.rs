/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  arch/arm/include/asm/glue-df.h
 *
 *  Copyright (C) 1997-1999 Russell King
 *  Copyright (C) 2000-2002 Deep Blue Solutions Ltd.
 */

// Dependency intent: names supplied by <asm/glue.h> remain external.

/*
 * Data Abort Model
 * =================
 *
 * We have the following to choose from:
 *   arm7       - ARM7 style
 *   v4_early   - ARMv4 without Thumb early abort handler
 *   v4t_late   - ARMv4 with Thumb late abort handler
 *   v4t_early  - ARMv4 with Thumb early abort handler
 *   v5t_early  - ARMv5 with Thumb early abort handler
 *   v5tj_early - ARMv5 with Thumb and Java early abort handler
 *   xscale     - ARMv5 with Thumb with Xscale extensions
 *   v6_early   - ARMv6 generic early abort handler
 *   v7_early   - ARMv7 generic early abort handler
 */

// The C preprocessor selected one handler and defined MULTI_DABORT when more
// than one CONFIG_CPU_ABRT_* option was enabled. Rust cfg names below preserve
// those build-time conditions.

#[cfg(any(
    CONFIG_CPU_ABRT_EV4,
    CONFIG_CPU_ABRT_LV4T,
    CONFIG_CPU_ABRT_EV4T,
    CONFIG_CPU_ABRT_EV5T,
    CONFIG_CPU_ABRT_EV5TJ,
    CONFIG_CPU_ABRT_EV6,
    CONFIG_CPU_ABRT_EV7,
    CONFIG_CPU_ABRT_NOMMU,
))]
#[macro_export]
macro_rules! CPU_DABORT_HANDLER {
    () => {
        $crate::cpu_dabort_handler!()
    };
}

#[cfg(CONFIG_CPU_ABRT_EV4)]
#[macro_export]
macro_rules! cpu_dabort_handler {
    () => { v4_early_abort };
}

#[cfg(all(not(CONFIG_CPU_ABRT_EV4), CONFIG_CPU_ABRT_LV4T))]
#[macro_export]
macro_rules! cpu_dabort_handler {
    () => { v4t_late_abort };
}

#[cfg(all(
    not(CONFIG_CPU_ABRT_EV4),
    not(CONFIG_CPU_ABRT_LV4T),
    CONFIG_CPU_ABRT_EV4T,
))]
#[macro_export]
macro_rules! cpu_dabort_handler {
    () => { v4t_early_abort };
}

#[cfg(all(
    not(CONFIG_CPU_ABRT_EV4),
    not(CONFIG_CPU_ABRT_LV4T),
    not(CONFIG_CPU_ABRT_EV4T),
    CONFIG_CPU_ABRT_EV5T,
))]
#[macro_export]
macro_rules! cpu_dabort_handler {
    () => { v5t_early_abort };
}

#[cfg(all(
    not(CONFIG_CPU_ABRT_EV4),
    not(CONFIG_CPU_ABRT_LV4T),
    not(CONFIG_CPU_ABRT_EV4T),
    not(CONFIG_CPU_ABRT_EV5T),
    CONFIG_CPU_ABRT_EV5TJ,
))]
#[macro_export]
macro_rules! cpu_dabort_handler {
    () => { v5tj_early_abort };
}

#[cfg(all(
    not(CONFIG_CPU_ABRT_EV4),
    not(CONFIG_CPU_ABRT_LV4T),
    not(CONFIG_CPU_ABRT_EV4T),
    not(CONFIG_CPU_ABRT_EV5T),
    not(CONFIG_CPU_ABRT_EV5TJ),
    CONFIG_CPU_ABRT_EV6,
))]
#[macro_export]
macro_rules! cpu_dabort_handler {
    () => { v6_early_abort };
}

#[cfg(all(
    not(CONFIG_CPU_ABRT_EV4),
    not(CONFIG_CPU_ABRT_LV4T),
    not(CONFIG_CPU_ABRT_EV4T),
    not(CONFIG_CPU_ABRT_EV5T),
    not(CONFIG_CPU_ABRT_EV5TJ),
    not(CONFIG_CPU_ABRT_EV6),
    CONFIG_CPU_ABRT_EV7,
))]
#[macro_export]
macro_rules! cpu_dabort_handler {
    () => { v7_early_abort };
}

#[cfg(all(
    not(CONFIG_CPU_ABRT_EV4),
    not(CONFIG_CPU_ABRT_LV4T),
    not(CONFIG_CPU_ABRT_EV4T),
    not(CONFIG_CPU_ABRT_EV5T),
    not(CONFIG_CPU_ABRT_EV5TJ),
    not(CONFIG_CPU_ABRT_EV6),
    not(CONFIG_CPU_ABRT_EV7),
    CONFIG_CPU_ABRT_NOMMU,
))]
#[macro_export]
macro_rules! cpu_dabort_handler {
    () => { nommu_early_abort };
}

#[cfg(any(
    all(CONFIG_CPU_ABRT_EV4, any(
        CONFIG_CPU_ABRT_LV4T, CONFIG_CPU_ABRT_EV4T, CONFIG_CPU_ABRT_EV5T,
        CONFIG_CPU_ABRT_EV5TJ, CONFIG_CPU_ABRT_EV6, CONFIG_CPU_ABRT_EV7,
        CONFIG_CPU_ABRT_NOMMU,
    )),
    all(CONFIG_CPU_ABRT_LV4T, any(
        CONFIG_CPU_ABRT_EV4T, CONFIG_CPU_ABRT_EV5T, CONFIG_CPU_ABRT_EV5TJ,
        CONFIG_CPU_ABRT_EV6, CONFIG_CPU_ABRT_EV7, CONFIG_CPU_ABRT_NOMMU,
    )),
    all(CONFIG_CPU_ABRT_EV4T, any(
        CONFIG_CPU_ABRT_EV5T, CONFIG_CPU_ABRT_EV5TJ, CONFIG_CPU_ABRT_EV6,
        CONFIG_CPU_ABRT_EV7, CONFIG_CPU_ABRT_NOMMU,
    )),
    all(CONFIG_CPU_ABRT_EV5T, any(
        CONFIG_CPU_ABRT_EV5TJ, CONFIG_CPU_ABRT_EV6, CONFIG_CPU_ABRT_EV7,
        CONFIG_CPU_ABRT_NOMMU,
    )),
    all(CONFIG_CPU_ABRT_EV5TJ, any(
        CONFIG_CPU_ABRT_EV6, CONFIG_CPU_ABRT_EV7, CONFIG_CPU_ABRT_NOMMU,
    )),
    all(CONFIG_CPU_ABRT_EV6, any(CONFIG_CPU_ABRT_EV7, CONFIG_CPU_ABRT_NOMMU)),
    all(CONFIG_CPU_ABRT_EV7, CONFIG_CPU_ABRT_NOMMU),
))]
pub const MULTI_DABORT: bool = true;

#[cfg(not(any(
    all(CONFIG_CPU_ABRT_EV4, any(CONFIG_CPU_ABRT_LV4T, CONFIG_CPU_ABRT_EV4T, CONFIG_CPU_ABRT_EV5T, CONFIG_CPU_ABRT_EV5TJ, CONFIG_CPU_ABRT_EV6, CONFIG_CPU_ABRT_EV7, CONFIG_CPU_ABRT_NOMMU)),
    all(CONFIG_CPU_ABRT_LV4T, any(CONFIG_CPU_ABRT_EV4T, CONFIG_CPU_ABRT_EV5T, CONFIG_CPU_ABRT_EV5TJ, CONFIG_CPU_ABRT_EV6, CONFIG_CPU_ABRT_EV7, CONFIG_CPU_ABRT_NOMMU)),
    all(CONFIG_CPU_ABRT_EV4T, any(CONFIG_CPU_ABRT_EV5T, CONFIG_CPU_ABRT_EV5TJ, CONFIG_CPU_ABRT_EV6, CONFIG_CPU_ABRT_EV7, CONFIG_CPU_ABRT_NOMMU)),
    all(CONFIG_CPU_ABRT_EV5T, any(CONFIG_CPU_ABRT_EV5TJ, CONFIG_CPU_ABRT_EV6, CONFIG_CPU_ABRT_EV7, CONFIG_CPU_ABRT_NOMMU)),
    all(CONFIG_CPU_ABRT_EV5TJ, any(CONFIG_CPU_ABRT_EV6, CONFIG_CPU_ABRT_EV7, CONFIG_CPU_ABRT_NOMMU)),
    all(CONFIG_CPU_ABRT_EV6, any(CONFIG_CPU_ABRT_EV7, CONFIG_CPU_ABRT_NOMMU)),
    all(CONFIG_CPU_ABRT_EV7, CONFIG_CPU_ABRT_NOMMU),
))]
pub const MULTI_DABORT: bool = false;

#[cfg(not(any(
    CONFIG_CPU_ABRT_EV4, CONFIG_CPU_ABRT_LV4T, CONFIG_CPU_ABRT_EV4T,
    CONFIG_CPU_ABRT_EV5T, CONFIG_CPU_ABRT_EV5TJ, CONFIG_CPU_ABRT_EV6,
    CONFIG_CPU_ABRT_EV7, CONFIG_CPU_ABRT_NOMMU,
)))]
compile_error!("Unknown data abort handler type");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
