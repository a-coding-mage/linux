/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes <asm/linkage.h>; its declarations are supplied by the
// architecture-specific Rust dependencies.

/// C equivalent: `(unsigned long)__builtin_return_address(0)`.
/// Rust has no file-local equivalent for obtaining the caller's return address.
#[macro_export]
macro_rules! _RET_IP_ {
    () => {{
        // TODO: provide the architecture/compiler-specific caller address.
        0usize
    }};
}

// The generic C `_THIS_IP_` implementation is considered broken by GCC and
// Clang; the architecture-specific fallback is therefore represented here.
#[allow(non_upper_case_globals)]
pub const HAS_BROKEN_THIS_IP: bool = true;

/// Generic fallback for C `_THIS_IP_`.
#[macro_export]
macro_rules! _THIS_IP_ {
    () => {{
        static __here: u8 = 0;
        (&__here as *const u8 as usize)
    }};
}

/*
 * `_CODE_LOCATION_` provides a unique identifier for the current code
 * location.  When `_THIS_IP_` is broken, use a static marker whose address is
 * constant at link time and avoids runtime overhead.
 */
#[macro_export]
macro_rules! _CODE_LOCATION_ {
    () => {{
        static __here: u8 = 0;
        (&__here as *const u8 as usize)
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
