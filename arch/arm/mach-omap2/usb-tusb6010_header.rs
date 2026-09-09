/* SPDX-License-Identifier: GPL-2.0 */

// C header guard: __USB_TUSB6010_H

// C __init section annotation is preserved as intent; its platform-specific
// Rust equivalent is supplied by the surrounding kernel bindings.
extern "C" {
    pub fn tusb6010_setup_interface(
        data: *mut musb_hdrc_platform_data,
        ps_refclk: ::core::ffi::c_uint,
        waitpin: ::core::ffi::c_uint,
        async_cs: ::core::ffi::c_uint,
        sync_cs: ::core::ffi::c_uint,
        dmachan: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
