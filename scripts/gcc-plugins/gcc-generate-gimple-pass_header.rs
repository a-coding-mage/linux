/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Generator for GIMPLE pass related boilerplate code/data
 *
 * Supports gcc 4.5-6
 *
 * This header is intended to be included after defining PASS_NAME,
 * NO_GATE, NO_EXECUTE, PROPERTIES_*, and TODO_FLAGS_*.
 */

/* C preprocessor stringification and token-pasting are represented by the
 * caller's Rust identifiers and constants below. */

#[allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
pub mod gcc_generate_gimple_pass {
    use super::*;

    /* Required external GCC definitions. */
    extern "C" {
        static mut g: *mut gcc::context;
    }

    /* The concrete PASS_NAME expansion supplies these names and values. */
    pub const PROPERTIES_REQUIRED: u32 = 0;
    pub const PROPERTIES_PROVIDED: u32 = 0;
    pub const PROPERTIES_DESTROYED: u32 = 0;
    pub const TODO_FLAGS_START: u32 = 0;
    pub const TODO_FLAGS_FINISH: u32 = 0;

    pub static mut _PASS_NAME_PASS_DATA: gcc::pass_data = gcc::pass_data {
        type_: gcc::pass_type::GIMPLE_PASS,
        name: _PASS_NAME_NAME,
        optinfo_flags: gcc::optgroup_flags::OPTGROUP_NONE,
        tv_id: gcc::timevar_id::TV_NONE,
        properties_required: PROPERTIES_REQUIRED,
        properties_provided: PROPERTIES_PROVIDED,
        properties_destroyed: PROPERTIES_DESTROYED,
        todo_flags_start: TODO_FLAGS_START,
        todo_flags_finish: TODO_FLAGS_FINISH,
    };

    pub const _PASS_NAME_NAME: *const ::std::os::raw::c_char =
        b"PASS_NAME\0".as_ptr() as *const ::std::os::raw::c_char;

    pub struct _PASS_NAME_PASS {
        pub base: gcc::gimple_opt_pass,
    }

    impl _PASS_NAME_PASS {
        pub unsafe fn new() -> Self {
            Self {
                base: gcc::gimple_opt_pass::new(
                    &_PASS_NAME_PASS_DATA as *const gcc::pass_data,
                    g,
                ),
            }
        }

        #[cfg(not(feature = "NO_GATE"))]
        pub unsafe fn gate(&mut self, function: *mut gcc::function) -> bool {
            let _ = function;
            PASS_NAME_gate()
        }

        pub unsafe fn clone_pass(&self) -> *mut gcc::opt_pass {
            Box::into_raw(Box::new(Self::new())) as *mut gcc::opt_pass
        }

        #[cfg(not(feature = "NO_EXECUTE"))]
        pub unsafe fn execute(&mut self, function: *mut gcc::function) -> u32 {
            let _ = function;
            PASS_NAME_execute()
        }
    }

    #[cfg(not(feature = "PASS_NAME"))]
    pub unsafe fn make_PASS_NAME_pass() -> *mut gcc::opt_pass {
        Box::into_raw(Box::new(_PASS_NAME_PASS::new())) as *mut gcc::opt_pass
    }

    #[cfg(feature = "PASS_NAME")]
    pub unsafe fn make_PASS_NAME_pass() -> *mut gcc::opt_pass {
        &_PASS_NAME_PASS.pass as *const gcc::opt_pass as *mut gcc::opt_pass
    }
}

/* External names supplied by the including translation unit. */
extern "C" {
    fn PASS_NAME_gate() -> bool;
    fn PASS_NAME_execute() -> u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
