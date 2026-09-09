/* SPDX-License-Identifier: GPL-2.0 */

// Translated from the C header. Include guards are not applicable in Rust.

use core::ffi::c_void;

pub struct trace_remote;
pub struct trace_event_fields;
pub struct trace_seq;

#[repr(C)]
pub struct remote_event_hdr {
    pub id: u16,
}

pub const REMOTE_EVENT_NAME_MAX: usize = 30;

#[repr(C)]
pub struct remote_event {
    pub name: [std::ffi::c_char; REMOTE_EVENT_NAME_MAX],
    pub id: u16,
    pub enabled: bool,
    pub remote: *mut trace_remote,
    pub fields: *mut trace_event_fields,
    pub print_fmt: *mut std::ffi::c_char,
    pub print: Option<unsafe extern "C" fn(evt: *mut c_void, seq: *mut trace_seq)>,
}

// C macro equivalent: RE_STRUCT(__args...) expands to its arguments.
#[macro_export]
macro_rules! RE_STRUCT {
    ($($args:tt)*) => { $($args)* };
}

// C macro equivalent: re_field(__type, __field) expands to a field declaration.
#[macro_export]
macro_rules! re_field {
    ($type:ty, $field:ident) => { pub $field: $type, };
}

// Rust cannot concatenate identifiers in a declarative macro on stable Rust;
// pass the complete generated structure name as the first argument.
#[macro_export]
macro_rules! REMOTE_EVENT_FORMAT {
    ($struct_name:ident, { $($fields:tt)* }) => {
        #[repr(C)]
        pub struct $struct_name {
            pub hdr: $crate::remote_event_hdr,
            $($fields)*
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
