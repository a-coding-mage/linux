/* SPDX-License-Identifier: GPL-2.0 */

// The original header includes <linux/compiler.h> and <linux/types.h>.

/// Architecture-specific x86 TSC trace clock.
///
/// `notrace` is a compiler/build-system attribute in the C source and has no
/// direct Rust syntax here.
extern "C" {
    pub fn trace_clock_x86_tsc() -> u64;
}

// C source equivalent:
//   { trace_clock_x86_tsc, "x86-tsc", .in_ns = 0 },
// The containing initializer type is defined by the dependent trace-clock
// code, so preserve this as a locally expandable Rust macro.
#[macro_export]
macro_rules! ARCH_TRACE_CLOCKS {
    () => {
        {
            trace_clock_x86_tsc,
            "x86-tsc",
            in_ns: 0,
        }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
