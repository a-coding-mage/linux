/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2024 Intel Corporation */

// Dependency: the original header includes <linux/cache.h>.
// Build-time CONFIG_64BIT and SMP_CACHE_BYTES conditions are preserved below
// as Rust macro intent; their definitions are supplied by the surrounding
// translation unit.

/**
 * libeth_cacheline_group_assert - make sure cacheline group size is expected
 * @type: type of the structure containing the group
 * @grp: group name inside the struct
 * @sz: expected group size
 */
#[cfg(all(CONFIG_64BIT, SMP_CACHE_BYTES_64))]
macro_rules! libeth_cacheline_group_assert {
    ($type:ty, $grp:ident, $sz:expr) => {
        const _: () = assert!(
            core::mem::offset_of!($type, __cacheline_group_end__$grp)
                - core::mem::offset_of!($type, __cacheline_group_begin__$grp)
                == $sz
        );
    };
}

#[cfg(not(all(CONFIG_64BIT, SMP_CACHE_BYTES_64)))]
macro_rules! libeth_cacheline_group_assert {
    ($type:ty, $grp:ident, $sz:expr) => {
        const _: () = assert!(
            core::mem::offset_of!($type, __cacheline_group_end__$grp)
                - core::mem::offset_of!($type, __cacheline_group_begin__$grp)
                <= $sz
        );
    };
}

#[cfg(all(CONFIG_64BIT, SMP_CACHE_BYTES_64))]
macro_rules! __libeth_cacheline_struct_assert {
    ($type:ty, $sz:expr) => {
        const _: () = assert!(core::mem::size_of::<$type>() == $sz);
    };
}

#[cfg(not(all(CONFIG_64BIT, SMP_CACHE_BYTES_64)))]
macro_rules! __libeth_cacheline_struct_assert {
    ($type:ty, $sz:expr) => {
        const _: () = assert!(core::mem::size_of::<$type>() <= $sz);
    };
}

macro_rules! __libeth_cls1 {
    ($sz1:expr) => { SMP_CACHE_ALIGN!($sz1) };
}

macro_rules! __libeth_cls2 {
    ($sz1:expr, $sz2:expr) => {
        (SMP_CACHE_ALIGN!($sz1) + SMP_CACHE_ALIGN!($sz2))
    };
}

macro_rules! __libeth_cls3 {
    ($sz1:expr, $sz2:expr, $sz3:expr) => {
        (SMP_CACHE_ALIGN!($sz1) + SMP_CACHE_ALIGN!($sz2) + SMP_CACHE_ALIGN!($sz3))
    };
}

macro_rules! __libeth_cls {
    ($sz1:expr) => { __libeth_cls1!($sz1) };
    ($sz1:expr, $sz2:expr) => { __libeth_cls2!($sz1, $sz2) };
    ($sz1:expr, $sz2:expr, $sz3:expr) => { __libeth_cls3!($sz1, $sz2, $sz3) };
}

/**
 * libeth_cacheline_struct_assert - make sure CL-based struct size is expected
 * @type: type of the struct
 * @...: from 1 to 3 CL group sizes (read-mostly, read-write, cold)
 *
 * When a struct contains several CL groups, it's difficult to predict its size
 * on different architectures. The macro instead takes sizes of all of the
 * groups the structure contains and generates the final struct size.
 */
macro_rules! libeth_cacheline_struct_assert {
    ($type:ty, $($sz:expr),+ $(,)?) => {
        __libeth_cacheline_struct_assert!($type, __libeth_cls!($($sz),+));
        const _: () = assert!(core::mem::align_of::<$type>() >= SMP_CACHE_BYTES);
    };
}

/**
 * libeth_cacheline_set_assert - make sure CL-based struct layout is expected
 * @type: type of the struct
 * @ro: expected size of the read-mostly group
 * @rw: expected size of the read-write group
 * @c: expected size of the cold group
 *
 * Check that each group size is expected and then do final struct size check.
 */
macro_rules! libeth_cacheline_set_assert {
    ($type:ty, $ro:expr, $rw:expr, $c:expr) => {
        libeth_cacheline_group_assert!($type, read_mostly, $ro);
        libeth_cacheline_group_assert!($type, read_write, $rw);
        libeth_cacheline_group_assert!($type, cold, $c);
        libeth_cacheline_struct_assert!($type, $ro, $rw, $c);
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
