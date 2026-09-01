// SPDX-License-Identifier: GPL-2.0

// Depends on dt-bindings/sound/fsl-imx-audmux.h for related constants.

unsafe extern "C" {
    pub fn imx_audmux_v1_configure_port(port: ::core::ffi::c_uint, pcr: ::core::ffi::c_uint) -> ::core::ffi::c_int;

    pub fn imx_audmux_v2_configure_port(
        port: ::core::ffi::c_uint,
        ptcr: ::core::ffi::c_uint,
        pdcr: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
