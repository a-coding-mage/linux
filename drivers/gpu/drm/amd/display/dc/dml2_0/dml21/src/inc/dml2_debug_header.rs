// SPDX-License-Identifier: MIT
//
// Copyright 2024 Advanced Micro Devices, Inc.

// Dependency supplied by the surrounding translation unit: `os_types.h`.

// C macros translated as Rust macro_rules! macros.
#[macro_export]
macro_rules! DML_ASSERT {
    ($condition:expr) => { ASSERT!($condition) };
}

pub const DML_LOG_LEVEL_FATAL: i32 = 0;
pub const DML_LOG_LEVEL_ERROR: i32 = 1;
pub const DML_LOG_LEVEL_WARN: i32 = 2;
pub const DML_LOG_LEVEL_INFO: i32 = 3;
pub const DML_LOG_LEVEL_DEBUG: i32 = 4;
pub const DML_LOG_LEVEL_VERBOSE: i32 = 5;
pub const DML_LOG_LEVEL_DEFAULT: i32 = DML_LOG_LEVEL_WARN;

// If the C build defines DML_LOG_LEVEL, this constant should be replaced by
// that build-time value; otherwise it has the C default shown here.
pub const DML_LOG_LEVEL: i32 = DML_LOG_LEVEL_DEFAULT;

#[macro_export]
macro_rules! DML_LOG_INTERNAL {
    ($($args:tt)*) => { dm_output_to_console!($($args)*) };
}

#[macro_export]
macro_rules! _BOOL_FORMAT {
    ($field:expr) => { "%s", if $field { "true" } else { "false" } };
}
#[macro_export]
macro_rules! _UINT_FORMAT { ($field:expr) => { "%u", $field }; }
#[macro_export]
macro_rules! _INT_FORMAT { ($field:expr) => { "%d", $field }; }
#[macro_export]
macro_rules! _DOUBLE_FORMAT { ($field:expr) => { "%lf", $field }; }
#[macro_export]
macro_rules! _ELEMENT_FUNC { () => { "function" }; }
#[macro_export]
macro_rules! _ELEMENT_COMP_IF { () => { "component_interface" }; }
#[macro_export]
macro_rules! _ELEMENT_TOP_IF { () => { "top_interface" }; }

#[macro_export]
macro_rules! _LOG_ENTRY {
    ($element:expr) => {{
        DML_LOG_INTERNAL!(concat!("<", $element, " name=\""));
        DML_LOG_INTERNAL!("__func__");
        DML_LOG_INTERNAL!(">\n");
    }};
}
#[macro_export]
macro_rules! _LOG_EXIT {
    ($element:expr) => { DML_LOG_INTERNAL!(concat!("</", $element, ">\n")) };
}
#[macro_export]
macro_rules! _LOG_SCALAR {
    ($field:ident, $format:ident) => {{
        DML_LOG_INTERNAL!(concat!(stringify!($field), " = ", $format!($field)));
        DML_LOG_INTERNAL!("\n");
    }};
}
#[macro_export]
macro_rules! _LOG_ARRAY {
    ($field:expr, $size:expr, $format:ident) => {{
        DML_LOG_INTERNAL!(concat!(stringify!($field), " = ["));
        let mut _i: i32 = 0;
        while _i < ($size as i32) {
            DML_LOG_INTERNAL!($format!($field[_i as usize]));
            if _i + 1 == ($size as i32) { DML_LOG_INTERNAL!("]\n"); }
            else { DML_LOG_INTERNAL!(", "); }
            _i += 1;
        }
    }};
}

// The C 2D/3D array logging macros retain their loop structure and indexing.
#[macro_export]
macro_rules! _LOG_2D_ARRAY {
    ($field:expr, $size0:expr, $size1:expr, $format:ident) => {{
        DML_LOG_INTERNAL!(concat!(stringify!($field), " = ["));
        let mut _i: i32 = 0;
        while _i < ($size0 as i32) {
            DML_LOG_INTERNAL!("\n\t[");
            let mut _j: i32 = 0;
            while _j < ($size1 as i32) {
                DML_LOG_INTERNAL!($format!($field[_i as usize][_j as usize]));
                if _j + 1 == ($size1 as i32) { DML_LOG_INTERNAL!("]"); }
                else { DML_LOG_INTERNAL!(", "); }
                _j += 1;
            }
            if _i + 1 == ($size0 as i32) { DML_LOG_INTERNAL!("]\n"); }
            else { DML_LOG_INTERNAL!(", "); }
            _i += 1;
        }
    }};
}

#[macro_export]
macro_rules! _LOG_3D_ARRAY {
    ($field:expr, $size0:expr, $size1:expr, $size2:expr, $format:ident) => {{
        DML_LOG_INTERNAL!(concat!(stringify!($field), " = ["));
        let mut _i: i32 = 0;
        while _i < ($size0 as i32) {
            DML_LOG_INTERNAL!("\n\t[");
            let mut _j: i32 = 0;
            while _j < ($size1 as i32) {
                DML_LOG_INTERNAL!("[");
                let mut _k: i32 = 0;
                while _k < ($size2 as i32) {
                    DML_LOG_INTERNAL!($format!($field[_i as usize][_j as usize][_k as usize]));
                    if _k + 1 == ($size2 as i32) { DML_LOG_INTERNAL!("]"); }
                    else { DML_LOG_INTERNAL!(", "); }
                    _k += 1;
                }
                if _j + 1 == ($size1 as i32) { DML_LOG_INTERNAL!("]"); }
                else { DML_LOG_INTERNAL!(", "); }
                _j += 1;
            }
            if _i + 1 == ($size0 as i32) { DML_LOG_INTERNAL!("]\n"); }
            else { DML_LOG_INTERNAL!(", "); }
            _i += 1;
        }
    }};
}

// The following public macros preserve the C logging interface. C
// preprocessor level tests are represented by the default-enabled forms;
// consumers may conditionally redefine/disable them at integration time.
#[macro_export]
macro_rules! DML_LOG_FATAL { ($($args:tt)*) => { DML_LOG_INTERNAL!("[DML FATAL] ", $($args)*) }; }
#[macro_export]
macro_rules! DML_LOG_ERROR { ($($args:tt)*) => { DML_LOG_INTERNAL!("[DML ERROR] ", $($args)*) }; }
#[macro_export]
macro_rules! DML_LOG_WARN { ($($args:tt)*) => { DML_LOG_INTERNAL!("[DML WARN] ", $($args)*) }; }
#[macro_export]
macro_rules! DML_LOG_INFO { ($($args:tt)*) => { DML_LOG_INTERNAL!("[DML INFO] ", $($args)*) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG { ($($args:tt)*) => { DML_LOG_INTERNAL!($($args)*) }; }
#[macro_export]
macro_rules! DML_LOG_VERBOSE { ($($args:tt)*) => { DML_LOG_INTERNAL!($($args)*) }; }

#[macro_export]
macro_rules! DML_LOG_TOP_IF_ENTER { () => { _LOG_ENTRY!("top_interface") }; }
#[macro_export]
macro_rules! DML_LOG_TOP_IF_EXIT { () => { _LOG_EXIT!("top_interface") }; }
#[macro_export]
macro_rules! DML_LOG_COMP_IF_ENTER { () => { _LOG_ENTRY!("component_interface") }; }
#[macro_export]
macro_rules! DML_LOG_COMP_IF_EXIT { () => { _LOG_EXIT!("component_interface") }; }
#[macro_export]
macro_rules! DML_LOG_FUNC_ENTER { () => { _LOG_ENTRY!("function") }; }
#[macro_export]
macro_rules! DML_LOG_FUNC_EXIT { () => { _LOG_EXIT!("function") }; }

#[macro_export]
macro_rules! DML_LOG_DEBUG_BOOL { ($field:ident) => { _LOG_SCALAR!($field, _BOOL_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_UINT { ($field:ident) => { _LOG_SCALAR!($field, _UINT_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_INT { ($field:ident) => { _LOG_SCALAR!($field, _INT_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_DOUBLE { ($field:ident) => { _LOG_SCALAR!($field, _DOUBLE_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_ARRAY_BOOL { ($field:expr, $size:expr) => { _LOG_ARRAY!($field, $size, _BOOL_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_ARRAY_UINT { ($field:expr, $size:expr) => { _LOG_ARRAY!($field, $size, _UINT_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_ARRAY_INT { ($field:expr, $size:expr) => { _LOG_ARRAY!($field, $size, _INT_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_ARRAY_DOUBLE { ($field:expr, $size:expr) => { _LOG_ARRAY!($field, $size, _DOUBLE_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_2D_ARRAY_BOOL { ($field:expr, $a:expr, $b:expr) => { _LOG_2D_ARRAY!($field, $a, $b, _BOOL_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_2D_ARRAY_UINT { ($field:expr, $a:expr, $b:expr) => { _LOG_2D_ARRAY!($field, $a, $b, _UINT_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_2D_ARRAY_INT { ($field:expr, $a:expr, $b:expr) => { _LOG_2D_ARRAY!($field, $a, $b, _INT_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_2D_ARRAY_DOUBLE { ($field:expr, $a:expr, $b:expr) => { _LOG_2D_ARRAY!($field, $a, $b, _DOUBLE_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_3D_ARRAY_BOOL { ($field:expr, $a:expr, $b:expr, $c:expr) => { _LOG_3D_ARRAY!($field, $a, $b, $c, _BOOL_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_3D_ARRAY_UINT { ($field:expr, $a:expr, $b:expr, $c:expr) => { _LOG_3D_ARRAY!($field, $a, $b, $c, _UINT_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_3D_ARRAY_INT { ($field:expr, $a:expr, $b:expr, $c:expr) => { _LOG_3D_ARRAY!($field, $a, $b, $c, _INT_FORMAT) }; }
#[macro_export]
macro_rules! DML_LOG_DEBUG_3D_ARRAY_DOUBLE { ($field:expr, $a:expr, $b:expr, $c:expr) => { _LOG_3D_ARRAY!($field, $a, $b, $c, _DOUBLE_FORMAT) }; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
