/* SPDX-License-Identifier: GPL-2.0 */

/*
 * The C header supplies generated vdso offset declarations.  Those generated
 * dependencies are intentionally left external to this translation.
 */

#[cfg(feature = "CONFIG_64BIT")]
#[macro_export]
macro_rules! VDSO64_SYMBOL {
    ($tsk:expr, $offset:expr) => {
        ($tsk).mm.context.vdso_base + $offset
    };
}

#[cfg(any(not(feature = "CONFIG_64BIT"), feature = "CONFIG_COMPAT"))]
#[macro_export]
macro_rules! VDSO32_SYMBOL {
    ($tsk:expr, $offset:expr) => {
        ($tsk).mm.context.vdso_base + $offset
    };
}

#[cfg(all(feature = "CONFIG_64BIT", not(feature = "CONFIG_COMPAT")))]
#[macro_export]
macro_rules! VDSO32_SYMBOL {
    ($tsk:expr, $offset:expr) => {
        0u64
    };
}

/* Default link addresses for the vDSOs. */
pub const VDSO_LBASE: u64 = 0;

pub const VDSO_VERSION_STRING: &str = "LINUX_6.11";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
