/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * DMI defines for use by IPMI
 */

// Dependency supplied by the translated ipmi_si interface:
// use crate::ipmi_si::si_type;

// Corresponds to CONFIG_IPMI_DMI_DECODE.
#[cfg(feature = "CONFIG_IPMI_DMI_DECODE")]
unsafe extern "C" {
    pub fn ipmi_dmi_get_slave_addr(
        si_type: si_type,
        space: ::core::ffi::c_uint,
        base_addr: ::core::ffi::c_ulong,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
