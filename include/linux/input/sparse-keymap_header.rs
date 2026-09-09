/* SPDX-License-Identifier: GPL-2.0-only */
/* _SPARSE_KEYMAP_H */

/*
 * Copyright (c) 2009 Dmitry Torokhov
 */

pub const KE_END: i32 = 0; /* Indicates end of keymap */
pub const KE_KEY: i32 = 1; /* Ordinary key/button */
pub const KE_SW: i32 = 2; /* Switch (predetermined value) */
pub const KE_VSW: i32 = 3; /* Switch (value supplied at runtime) */
pub const KE_IGNORE: i32 = 4; /* Known entry that should be ignored */
pub const KE_LAST: i32 = KE_IGNORE;

/**
 * struct key_entry - keymap entry for use in sparse keymap
 * @type: Type of the key entry (KE_KEY, KE_SW, KE_VSW, KE_END);
 *	drivers are allowed to extend the list with their own
 * 	private definitions.
 * @code: Device-specific data identifying the button/switch
 * @keycode: KEY_* code assigned to a key/button
 * @sw: struct with code/value used by KE_SW and KE_VSW
 * @sw.code: SW_* code assigned to a switch
 * @sw.value: Value that should be sent in an input even when KE_SW
 *	switch is toggled. KE_VSW switches ignore this field and
 *	expect driver to supply value for the event.
 *
 * This structure defines an entry in a sparse keymap used by some
 * input devices for which traditional table-based approach is not
 * suitable.
 */
#[repr(C)]
pub struct key_entry {
    pub type_: i32, /* See KE_* above */
    pub code: u32,
    pub __bindgen_anon_1: key_entry__bindgen_ty_1,
}

#[repr(C)]
pub union key_entry__bindgen_ty_1 {
    pub keycode: u16, /* For KE_KEY */
    pub sw: key_entry__bindgen_ty_1__bindgen_ty_1, /* For KE_SW, KE_VSW */
}

#[repr(C)]
pub struct key_entry__bindgen_ty_1__bindgen_ty_1 {
    pub code: u8,
    pub value: u8, /* For KE_SW, ignored by KE_VSW */
}

extern "C" {
    pub fn sparse_keymap_entry_from_scancode(
        dev: *mut input_dev,
        code: u32,
    ) -> *mut key_entry;
    pub fn sparse_keymap_entry_from_keycode(
        dev: *mut input_dev,
        code: u32,
    ) -> *mut key_entry;
    pub fn sparse_keymap_setup(
        dev: *mut input_dev,
        keymap: *const key_entry,
        setup: Option<unsafe extern "C" fn(*mut input_dev, *mut key_entry) -> i32>,
    ) -> i32;

    pub fn sparse_keymap_report_entry(
        dev: *mut input_dev,
        ke: *const key_entry,
        value: u32,
        autorelease: bool,
    );

    pub fn sparse_keymap_report_event(
        dev: *mut input_dev,
        code: u32,
        value: u32,
        autorelease: bool,
    ) -> bool;
}

/* Supplied by the input subsystem. */
pub enum input_dev {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
