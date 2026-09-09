// Translation of the Linux module dependency.
//
// #include <linux/module.h>

// The original source defines CREATE_TRACE_POINTS and includes "trace.h"
// unless __CHECKER__ is defined. The corresponding trace declarations and
// definitions are supplied by the surrounding Rust translation context.
#[cfg(not(__CHECKER__))]
pub const CREATE_TRACE_POINTS: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
