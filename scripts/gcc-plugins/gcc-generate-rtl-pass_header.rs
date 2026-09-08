/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Generator for RTL pass related boilerplate code/data.
 *
 * This header is a C preprocessor template.  The Rust macro below preserves
 * the same generated data, callbacks, constructor, and exported factory.
 * The GCC types and constants are supplied by the surrounding translation.
 */

/* The original template requires PASS_NAME to be defined before inclusion. */
macro_rules! generate_rtl_pass {
    (
        $pass_name:ident,
        required = $properties_required:expr,
        provided = $properties_provided:expr,
        destroyed = $properties_destroyed:expr,
        todo_start = $todo_flags_start:expr,
        todo_finish = $todo_flags_finish:expr,
        gate = $gate:expr,
        execute = $execute:expr
    ) => {
        const $pass_name##_pass_data: pass_data = pass_data {
            type_: RTL_PASS,
            name: stringify!($pass_name),
            optinfo_flags: OPTGROUP_NONE,
            tv_id: TV_NONE,
            properties_required: $properties_required,
            properties_provided: $properties_provided,
            properties_destroyed: $properties_destroyed,
            todo_flags_start: $todo_flags_start,
            todo_flags_finish: $todo_flags_finish,
        };

        struct $pass_name##_pass {
            base: rtl_opt_pass,
        }

        impl $pass_name##_pass {
            unsafe fn new() -> Self {
                Self {
                    base: rtl_opt_pass::new($pass_name##_pass_data, g),
                }
            }
        }

        impl $pass_name##_pass {
            unsafe fn gate(&mut self, function: *mut function) -> bool {
                let _ = function;
                ($gate)()
            }

            unsafe fn clone(&self) -> *mut opt_pass {
                Box::into_raw(Box::new(Self::new())) as *mut opt_pass
            }

            unsafe fn execute(&mut self, function: *mut function) -> u32 {
                let _ = function;
                ($execute)()
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn make_$pass_name##_pass() -> *mut opt_pass {
            Box::into_raw(Box::new($pass_name##_pass::new())) as *mut opt_pass
        }
    };
}

/*
 * NO_GATE and NO_EXECUTE omit the corresponding virtual callback in the C
 * template.  PROPERTIES_* and TODO_FLAGS_* default to zero when unspecified;
 * callers of this Rust form pass those defaults explicitly.
 */


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
