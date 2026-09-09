/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// C header guard: _UAPI_LINUX_STDDEF_H
// The kernel-only dependency on <linux/compiler_types.h> is supplied by the
// surrounding translation environment when building kernel code.

// C: #define __always_inline inline
// Rust has no direct file-local equivalent for this preprocessor macro;
// translated functions may use #[inline(always)] where applicable.

// C/C++ conditional behavior for __struct_group_tag(TAG):
// In C this expands to TAG; in C++ it expands to nothing.  Rust declarations
// should express the corresponding named type directly.

/**
 * C __struct_group() creates an anonymous union containing two structs with
 * identical layout and size, one anonymous and one named.  Rust equivalents
 * should use #[repr(C)] with an appropriately declared union and structs.
 */
// The variadic C macro cannot be represented as a general Rust macro without
// knowing the member declarations; its layout-preserving intent is retained
// above for each translated use.

// C++: __DECLARE_FLEX_ARRAY(T, member) expands to T member[0].
// C: __DECLARE_FLEX_ARRAY(TYPE, NAME) wraps an empty named struct and a
// flexible array in an anonymous struct so it can be used in a union.
// Rust equivalents should use raw pointers or zero-length arrays as required
// by the translated layout, for example: [T; 0].

// C annotation macros with no Rust storage or behavioral effect.
#[allow(unused_macros)]
macro_rules! __counted_by {
    ($m:ident) => {};
}

#[allow(unused_macros)]
macro_rules! __counted_by_le {
    ($m:ident) => {};
}

#[allow(unused_macros)]
macro_rules! __counted_by_be {
    ($m:ident) => {};
}

#[allow(unused_macros)]
macro_rules! __counted_by_ptr {
    ($m:ident) => {};
}

// C: __kernel_nonstring is __nonstring in kernel builds and empty otherwise.
// Rust has no corresponding attribute supplied by this header.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
