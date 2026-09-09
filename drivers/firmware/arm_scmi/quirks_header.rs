/* SPDX-License-Identifier: GPL-2.0 */
/*
 * System Control and Management Interface (SCMI) Message Protocol Quirks
 *
 * Copyright (C) 2025 ARM Ltd.
 */

// The C header includes linux/static_key.h and linux/types.h.

#[cfg(feature = "CONFIG_ARM_SCMI_QUIRKS")]
#[macro_export]
macro_rules! DECLARE_SCMI_QUIRK {
    ($key:ident) => {
        #[allow(non_upper_case_globals)]
        static mut $key: bool = false;
    };
}

/*
 * A helper to associate the actual code snippet to use as a quirk
 * named as _qn.
 *
 * Rust macro_rules! cannot perform C-style token pasting; callers pass the
 * complete static-key identifier as the first argument.
 */
#[cfg(feature = "CONFIG_ARM_SCMI_QUIRKS")]
#[macro_export]
macro_rules! SCMI_QUIRK {
    ($key:ident, $blk:block) => {{
        // Equivalent to static_branch_unlikely(&scmi_quirk_##_qn).
        if unsafe { $key } {
            $blk
        }
    }};
}

#[cfg(feature = "CONFIG_ARM_SCMI_QUIRKS")]
extern "C" {
    pub fn scmi_quirks_initialize();
    pub fn scmi_quirks_enable(
        dev: *mut core::ffi::c_void,
        vend: *const core::ffi::c_char,
        subv: *const core::ffi::c_char,
        impl_: u32,
    );
}

#[cfg(not(feature = "CONFIG_ARM_SCMI_QUIRKS"))]
#[macro_export]
macro_rules! DECLARE_SCMI_QUIRK {
    ($key:ident) => {};
}

/* Force quirks compilation even when SCMI Quirks are disabled */
#[cfg(not(feature = "CONFIG_ARM_SCMI_QUIRKS"))]
#[macro_export]
macro_rules! SCMI_QUIRK {
    ($key:ident, $blk:block) => {{
        if false {
            $blk
        }
    }};
}

#[cfg(not(feature = "CONFIG_ARM_SCMI_QUIRKS"))]
#[inline]
pub unsafe fn scmi_quirks_initialize() {}

#[cfg(not(feature = "CONFIG_ARM_SCMI_QUIRKS"))]
#[inline]
pub unsafe fn scmi_quirks_enable(
    _dev: *mut core::ffi::c_void,
    _vend: *const core::ffi::c_char,
    _sub_vend: *const core::ffi::c_char,
    _impl: u32,
) {
}

/* Quirk declarations */
#[cfg(feature = "CONFIG_ARM_SCMI_QUIRKS")]
DECLARE_SCMI_QUIRK!(scmi_quirk_clock_rates_triplet_out_of_spec);
#[cfg(feature = "CONFIG_ARM_SCMI_QUIRKS")]
DECLARE_SCMI_QUIRK!(scmi_quirk_perf_level_get_fc_force);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
