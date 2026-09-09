/* SPDX-License-Identifier: GPL-2.0 */

/*
 * 	NetWinder Button Driver-
 *	Copyright (C) Alex Holden <alex@linuxhacker.org> 1998, 1999.
 */

/*
 * These declarations are present when compiling the driver itself
 * (__NWBUTTON_C). Rust has no direct preprocessor equivalent here; the
 * driver-build conditional intent is preserved by retaining all declarations.
 */

/* Various defines: */

pub const NUM_PRESSES_REBOOT: ::core::ffi::c_int = 2; /* How many presses to activate shutdown */
pub const BUTTON_DELAY: ::core::ffi::c_int = 30; /* How many jiffies for sequence to end */
pub const VERSION: &str = "0.3"; /* Driver version number */

/* Structure definitions: */

#[repr(C)]
pub struct button_callback {
    pub callback: Option<unsafe extern "C" fn()>,
    pub count: ::core::ffi::c_int,
}

/* Function prototypes: */

unsafe extern "C" {
    fn button_sequence_finished(unused: *mut timer_list);
    fn button_handler(irq: ::core::ffi::c_int, dev_id: *mut ::core::ffi::c_void) -> irqreturn_t;
    fn button_init() -> ::core::ffi::c_int;
    fn button_add_callback(
        callback: Option<unsafe extern "C" fn()>,
        count: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn button_del_callback(callback: Option<unsafe extern "C" fn()>) -> ::core::ffi::c_int;
    fn button_consume_callbacks(bpcount: ::core::ffi::c_int);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
