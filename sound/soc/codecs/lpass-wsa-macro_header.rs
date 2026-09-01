// SPDX-License-Identifier: GPL-2.0

/*
 * Selects compander and smart boost settings
 * for a given speaker mode
 */
pub const WSA_MACRO_SPKR_MODE_DEFAULT: ::core::ffi::c_uint = 0;
pub const WSA_MACRO_SPKR_MODE_1: ::core::ffi::c_uint = 1; /* COMP Gain = 12dB, Smartboost Max = 5.5V */

unsafe extern "C" {
    pub fn wsa_macro_set_spkr_mode(
        component: *mut snd_soc_component,
        mode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
