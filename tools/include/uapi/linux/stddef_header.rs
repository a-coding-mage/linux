/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * Rust translation of include/uapi/linux/stddef.h.
 * C header guards and include-time conditionals are omitted from executable
 * Rust; the local macro intent is preserved below.
 */

/*
 * C fallback:
 *
 * #ifndef __always_inline
 * #define __always_inline __inline__
 * #endif
 */
macro_rules! __always_inline {
    ($item:item) => {
        #[inline(always)]
        $item
    };
}

/*
 * Not all C++ standards support type declarations inside an anonymous union.
 *
 * C expands __struct_group_tag(TAG) to TAG; C++ expands it to nothing.
 * Rust has no C/C++ mode distinction here, so this preserves the C expansion.
 */
macro_rules! __struct_group_tag {
    ($TAG:ident) => {
        $TAG
    };
    () => {};
}

/**
 * __struct_group() - Create a mirrored named and anonyomous struct
 *
 * @TAG: The tag name for the named sub-struct (usually empty)
 * @NAME: The identifier name of the mirrored sub-struct
 * @ATTRS: Any struct attributes (usually empty)
 * @MEMBERS: The member declarations for the mirrored structs
 *
 * Used to create an anonymous union of two structs with identical layout
 * and size: one anonymous and one named. The former's members can be used
 * normally without sub-struct naming, and the latter can be used to
 * reason about the start, end, and size of the group of struct members.
 * The named struct can also be explicitly tagged for layer reuse (C only),
 * as well as both having struct attributes appended.
 *
 * Rust has no direct equivalent for C anonymous struct members in anonymous
 * unions. This macro preserves the source-level expansion shape for callers
 * that provide Rust items representing the mirrored structs.
 */
macro_rules! __struct_group {
    ($TAG:ident, $NAME:ident, [$($ATTRS:meta),* $(,)?], $($MEMBERS:item)*) => {
        #[repr(C)]
        $(#[$ATTRS])*
        union $NAME {
            anonymous: $TAG,
            named: $TAG,
        }
    };
}

/*
 * C++ branch:
 *
 * sizeof(struct{}) is 1 in C++, not 0, can't use C version of the macro.
 * #define __DECLARE_FLEX_ARRAY(T, member) T member[0]
 *
 * C branch:
 */
/**
 * __DECLARE_FLEX_ARRAY() - Declare a flexible array usable in a union
 *
 * @TYPE: The type of each flexible array element
 * @NAME: The name of the flexible array member
 *
 * In order to have a flexible array member in a union or alone in a
 * struct, it needs to be wrapped in an anonymous struct with at least 1
 * named member, but that member can be empty.
 *
 * Rust does not support C flexible array members directly. This macro expands
 * to a C-layout zero-sized wrapper followed by an unsized slice field, which
 * preserves the declaration intent for FFI struct tails.
 */
macro_rules! __DECLARE_FLEX_ARRAY {
    ($TYPE:ty, $NAME:ident) => {
        #[repr(C)]
        struct __DeclareFlexArray<T: ?Sized> {
            __empty: [u8; 0],
            $NAME: T,
        }

        __DeclareFlexArray<[$TYPE]>
    };
}

macro_rules! __counted_by {
    ($m:tt) => {};
}

macro_rules! __counted_by_le {
    ($m:tt) => {};
}

macro_rules! __counted_by_be {
    ($m:tt) => {};
}

macro_rules! __counted_by_ptr {
    ($m:tt) => {};
}

macro_rules! __kernel_nonstring {
    () => {};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
