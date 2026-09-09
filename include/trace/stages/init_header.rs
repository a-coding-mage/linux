// Translation of init.h.
//
// The original header relies on C preprocessor token pasting, stringification,
// section attributes, and externally supplied trace_eval_map definitions.

/// C equivalent of `__app__(x, y)` / `__app(x, y)`.
/// Rust's `concat_idents!` is not stable, so identifier token pasting remains
/// represented by this macro's string-level equivalent.
#[macro_export]
macro_rules! __app {
    ($x:ident, $y:ident) => {
        concat!(stringify!($x), stringify!($y))
    };
}

/// `TRACE_SYSTEM_STRING` is the stringified concatenation of the externally
/// supplied `TRACE_SYSTEM_VAR` and `__trace_system_name` symbols.
#[macro_export]
macro_rules! TRACE_SYSTEM_STRING {
    () => {
        $crate::__app!(TRACE_SYSTEM_VAR, __trace_system_name)
    };
}

/// Rust equivalent of `TRACE_MAKE_SYSTEM_STR()`.
#[macro_export]
macro_rules! TRACE_MAKE_SYSTEM_STR {
    () => {
        static TRACE_SYSTEM_STRING_VALUE: &[u8] =
            concat!(stringify!(TRACE_SYSTEM), "\0").as_bytes();
    };
}

// TRACE_MAKE_SYSTEM_STR();

// `TRACE_DEFINE_ENUM(a)` in the C header creates two static trace_eval_map
// objects, one in init data and one in the `_ftrace_eval_map` linker section.
// The trace_eval_map type and linker-section representation are supplied by
// other files; the C token-pasted object names cannot be formed by stable
// declarative Rust macros. Callers provide the object identifier explicitly.
#[macro_export]
macro_rules! TRACE_DEFINE_ENUM {
    ($object:ident, $a:expr) => {
        static $object: trace_eval_map = trace_eval_map {
            system: TRACE_SYSTEM_STRING_VALUE,
            eval_string: stringify!($a),
            eval_value: $a,
        };
        #[link_section = "_ftrace_eval_map"]
        static $object##_ftrace_eval_map: *const trace_eval_map = &$object;
    };
}

// `TRACE_DEFINE_SIZEOF(a)` has the same layout and linker-section behavior as
// TRACE_DEFINE_ENUM, but records the C `sizeof(a)` value and its spelling.
#[macro_export]
macro_rules! TRACE_DEFINE_SIZEOF {
    ($object:ident, $a:ty) => {
        static $object: trace_eval_map = trace_eval_map {
            system: TRACE_SYSTEM_STRING_VALUE,
            eval_string: concat!("sizeof(", stringify!($a), ")"),
            eval_value: core::mem::size_of::<$a>(),
        };
        #[link_section = "_ftrace_eval_map"]
        static $object##_ftrace_eval_map: *const trace_eval_map = &$object;
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
