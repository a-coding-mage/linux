/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Generator for SIMPLE_IPA pass related boilerplate code/data.
 *
 * This header is parameterized by the build-time PASS_NAME, NO_GATE,
 * NO_EXECUTE, PROPERTIES_* and TODO_FLAGS_* definitions. Rust has no direct
 * equivalent of those token-pasting macros, so the generated pass items below
 * retain the same structure and are intended to be specialized by the
 * including translation unit.
 */

/* Required external GCC types and values are supplied by other bindings. */
extern "C" {
    pub static g: *mut gcc::context;
}

#[repr(C)]
pub struct pass_data {
    pub type_: gcc::pass_type,
    pub name: *const ::std::os::raw::c_char,
    pub optinfo_flags: ::std::os::raw::c_int,
    pub tv_id: ::std::os::raw::c_int,
    pub properties_required: ::std::os::raw::c_uint,
    pub properties_provided: ::std::os::raw::c_uint,
    pub properties_destroyed: ::std::os::raw::c_uint,
    pub todo_flags_start: ::std::os::raw::c_uint,
    pub todo_flags_finish: ::std::os::raw::c_uint,
}

/*
 * The following declarations correspond to the C++ template-like generated
 * names. Define PASS_NAME and the callback availability in the consuming
 * translation unit, then provide the corresponding generated symbols.
 */
extern "C" {
    pub fn PASS_NAME_gate(function: *mut gcc::function) -> bool;
    pub fn PASS_NAME_execute(function: *mut gcc::function) -> ::std::os::raw::c_uint;
}

pub struct simple_ipa_opt_pass {
    pub data: pass_data,
}

impl simple_ipa_opt_pass {
    pub const unsafe fn new(data: pass_data, _context: *mut gcc::context) -> Self {
        Self { data }
    }
}

/* Equivalent of the anonymous C++ pass class and its virtual callbacks. */
pub struct PASS_NAME_pass {
    pub base: simple_ipa_opt_pass,
}

impl PASS_NAME_pass {
    pub unsafe fn new(data: pass_data) -> Self {
        Self {
            base: simple_ipa_opt_pass::new(data, g),
        }
    }

    #[cfg(not(feature = "NO_GATE"))]
    pub unsafe fn gate(&mut self, function: *mut gcc::function) -> bool {
        PASS_NAME_gate(function)
    }

    pub unsafe fn clone_pass(&self, data: pass_data) -> *mut PASS_NAME_pass {
        Box::into_raw(Box::new(Self::new(data)))
    }

    #[cfg(not(feature = "NO_EXECUTE"))]
    pub unsafe fn execute(&mut self, function: *mut gcc::function) -> ::std::os::raw::c_uint {
        PASS_NAME_execute(function)
    }
}

/* Equivalent of: opt_pass *make_PASS_NAME_pass(void). */
#[no_mangle]
pub unsafe extern "C" fn make_PASS_NAME_pass(data: pass_data) -> *mut PASS_NAME_pass {
    Box::into_raw(Box::new(PASS_NAME_pass::new(data)))
}

/* Build-time defaults from the original header: all property and TODO flags default to 0. */

/* C++ include guards and macro cleanup are intentionally represented by the
 * absence of executable Rust equivalents; dependent names remain external. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
