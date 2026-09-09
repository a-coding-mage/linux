/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding translation unit:
// linux/perf_event.h and linux/types.h

#[repr(C)]
pub struct hv_perf_caps {
    pub version: u16,
    // C bit-fields: collect_privileged:1, ga:1, expanded:1, lab:1,
    // unused:12.  The flags occupy the second 16-bit storage unit.
    pub flags: u16,
}

extern "C" {
    pub fn hv_perf_caps_get(caps: *mut hv_perf_caps) -> core::ffi::c_ulong;
}

// EVENT_DEFINE_RANGE_FORMAT(name, attr_var, bit_start, bit_end)
// expands to PMU_FORMAT_ATTR(name, "attr_var:bit_start-bit_end") and
// EVENT_DEFINE_RANGE(name, attr_var, bit_start, bit_end).
#[macro_export]
macro_rules! EVENT_DEFINE_RANGE_FORMAT {
    ($name:ident, $attr_var:ident, $bit_start:expr, $bit_end:expr) => {
        PMU_FORMAT_ATTR!($name, concat!(stringify!($attr_var), ":", stringify!($bit_start), "-", stringify!($bit_end)));
        EVENT_DEFINE_RANGE!($name, $attr_var, $bit_start, $bit_end);
    };
}

/*
 * The EVENT_DEFINE_RANGE_FORMAT() macro above includes helper functions
 * for the fields (eg: event_get_starting_index()). For some fields
 * we need the bit-range definition, but not the helper functions. Define a
 * lite version of the above macro without the helpers and silence
 * compiler warnings unused static functions.
 */
#[macro_export]
macro_rules! EVENT_DEFINE_RANGE_FORMAT_LITE {
    ($name:ident, $attr_var:ident, $bit_start:expr, $bit_end:expr) => {
        PMU_FORMAT_ATTR!($name, concat!(stringify!($attr_var), ":", stringify!($bit_start), "-", stringify!($bit_end)));
    };
}

// Rust cannot concatenate identifiers in stable macro_rules! macros.  The
// generated helpers are therefore placed in a module named after the field;
// callers use `<name>::event_get` and `<name>::event_get_max`.
#[macro_export]
macro_rules! EVENT_DEFINE_RANGE {
    ($name:ident, $attr_var:ident, $bit_start:expr, $bit_end:expr) => {
        pub mod $name {
            #[inline]
            pub const fn event_get_max() -> u64 {
                assert!($bit_start <= $bit_end);
                assert!($bit_end < (core::mem::size_of::<u64>() * 8));
                (((1u64 << ($bit_end - $bit_start)) - 1) << 1) + 1
            }

            #[inline]
            pub unsafe fn event_get(event: *const perf_event) -> u64 {
                ((*event).attr.$attr_var >> ($bit_start)) & event_get_max()
            }
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
