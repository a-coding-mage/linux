/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *
 * Copyright (c) 2005 Linas Vepstas <linas@linas.org>
 */

/* The declarations in this header are kernel-only in the C source. */

/*
 * structure holding pci controller data that describes a
 * change in the isolation status of a PCI slot.  A pointer
 * to this struct is passed as the data pointer in a notify
 * callback.
 */
#[repr(C)]
pub struct eeh_event {
    pub list: list_head, /* to form event queue */
    pub pe: *mut eeh_pe, /* EEH PE */
}

/* Supplied by the surrounding kernel translation unit. */
extern "C" {
    pub fn eeh_event_init() -> ::core::ffi::c_int;
    pub fn eeh_send_failure_event(pe: *mut eeh_pe) -> ::core::ffi::c_int;
    pub fn __eeh_send_failure_event(pe: *mut eeh_pe) -> ::core::ffi::c_int;
    pub fn eeh_remove_event(pe: *mut eeh_pe, force: bool);
    pub fn eeh_handle_normal_event(pe: *mut eeh_pe);
    pub fn eeh_handle_special_event();
}

/* Types provided by other headers. */
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct eeh_pe {
    _private: [u8; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
