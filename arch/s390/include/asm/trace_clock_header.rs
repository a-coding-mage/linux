/* SPDX-License-Identifier: GPL-2.0 */

// C dependencies:
// #include <linux/compiler.h>
// #include <linux/types.h>

extern "C" {
    pub fn trace_clock_s390_tod() -> u64;
}

// Equivalent to:
// { trace_clock_s390_tod, "s390-tod", .in_ns = 0 },
// This is an initializer fragment whose containing type is supplied by the
// including code.
#[macro_export]
macro_rules! ARCH_TRACE_CLOCKS {
    () => {
        { trace_clock_s390_tod, "s390-tod", in_ns: 0 }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
