/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies represented by the original header includes:
// linux/build_bug.h, linux/stddef.h

/// Equivalent of C's `typeof_member(T, m)`.
///
/// Rust has no direct stable type operator corresponding to the C extension;
/// use the member's type through the containing type where required.

/// `container_of` - cast a member of a structure out to the containing structure.
///
/// WARNING: any const qualifier of `ptr` is lost.
/// Do not use `container_of!` in new code.
#[macro_export]
macro_rules! container_of {
    ($ptr:expr, $type:ty, $member:tt) => {{
        // The C implementation performs a compile-time pointer-type assertion
        // and subtracts offsetof($type, $member) from the member pointer.
        unsafe {
            (($ptr as *mut u8).sub(core::mem::offset_of!($type, $member))) as *mut $type
        }
    }};
}

/// `container_of_const` - cast a member of a structure out to the containing
/// structure and preserve the const-ness of the pointer.
///
/// Always prefer `container_of_const!` instead of `container_of!` in new code.
#[macro_export]
macro_rules! container_of_const {
    ($ptr:expr, $type:ty, $member:tt) => {{
        // C11 _Generic preserves const-qualified member pointers. Rust's cast
        // expression preserves the requested pointer mutability at the call site.
        container_of!($ptr, $type, $member)
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
