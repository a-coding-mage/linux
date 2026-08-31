// SPDX-License-Identifier: GPL-2.0

// C header guard removed: _TOOLS_PERF_LINUX_BUG_H.

// Force a compilation error if condition is true, but also produce a
// result (of value 0 and type size_t), so the expression can be used
// e.g. in a structure initializer (or where-ever else comma expressions
// aren't permitted).
//
// Original C macro:
// #define BUILD_BUG_ON_ZERO(e) (sizeof(struct { int:-!!(e); }))
//
// Rust cannot directly accept an arbitrary expression in a declarative macro
// const generic position on all dependency contexts, so preserve the externally
// visible macro name and its compile-time assertion intent for constant boolean
// expressions.
#[macro_export]
macro_rules! BUILD_BUG_ON_ZERO {
    ($e:expr) => {{
        const _: [(); 1] = [(); (!$e) as usize];
        0usize
    }};
}
