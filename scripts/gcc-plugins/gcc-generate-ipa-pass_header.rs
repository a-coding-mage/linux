/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Generator for IPA pass related boilerplate code/data.
 *
 * This Rust macro is the direct declaration-level counterpart of the C
 * header.  The C preprocessor's PASS_NAME and NO_* defines are represented
 * by the macro arguments and optional callback expressions below.
 */

#[macro_export]
macro_rules! gcc_generate_ipa_pass {
    (
        $pass_name:ident,
        $pass_data:ident,
        $pass_type:ident,
        $make_pass:ident,
        $generate_summary:expr,
        $write_summary:expr,
        $read_summary:expr,
        $write_optimization_summary:expr,
        $read_optimization_summary:expr,
        $stmt_fixup:expr,
        $function_transform_todo_flags_start:expr,
        $function_transform:expr,
        $variable_transform:expr,
        $properties_required:expr,
        $properties_provided:expr,
        $properties_destroyed:expr,
        $todo_flags_start:expr,
        $todo_flags_finish:expr,
        $gate:expr,
        $execute:expr
    ) => {
        static $pass_data: pass_data = pass_data {
            type_: IPA_PASS,
            name: stringify!($pass_name),
            optinfo_flags: OPTGROUP_NONE,
            tv_id: TV_NONE,
            properties_required: $properties_required,
            properties_provided: $properties_provided,
            properties_destroyed: $properties_destroyed,
            todo_flags_start: $todo_flags_start,
            todo_flags_finish: $todo_flags_finish,
        };

        struct $pass_type;

        impl $pass_type {
            pub const fn new() -> Self { Self }

            pub unsafe fn gate(&self, function: *mut function) -> bool {
                ($gate)(function)
            }

            pub unsafe fn clone(&self) -> *mut opt_pass {
                Box::into_raw(Box::new(Self::new())) as *mut opt_pass
            }

            pub unsafe fn execute(&self, function: *mut function) -> ::core::ffi::c_uint {
                ($execute)(function)
            }
        }

        #[no_mangle]
        pub unsafe extern "C" fn $make_pass() -> *mut opt_pass {
            Box::into_raw(Box::new($pass_type::new())) as *mut opt_pass
        }

        const _: (&str, &str, &str, &str, &str, &str, &str, &str, &str,
                  &str, &str, &str, &str) = (
            stringify!($generate_summary),
            stringify!($write_summary),
            stringify!($read_summary),
            stringify!($write_optimization_summary),
            stringify!($read_optimization_summary),
            stringify!($stmt_fixup),
            stringify!($function_transform_todo_flags_start),
            stringify!($function_transform),
            stringify!($variable_transform),
            stringify!($properties_required),
            stringify!($properties_provided),
            stringify!($properties_destroyed),
            stringify!($todo_flags_finish),
        );
    };
}

/* C header cleanup: PASS_NAME, NO_*, PROPERTIES_*, TODO_FLAGS_*, and all
 * generated helper macros were undefined after inclusion. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
