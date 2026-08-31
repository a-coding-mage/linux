/* SPDX-License-Identifier: GPL-2.0 */

/* Includes in the C header:
 * <stdbool.h>, <stddef.h>, <stdint.h>, <asm/types.h>, <asm/posix_types.h>
 *
 * __SANE_USERSPACE_TYPES__ is defined by the C header before including asm
 * types for PPC64 LL64 type selection.
 */

#[repr(C)]
pub struct page {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct kmem_cache {
    _unused: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum gfp_t {
    GFP_KERNEL,
    GFP_ATOMIC,
    __GFP_HIGHMEM,
    __GFP_HIGH,
}

/* C condition: #ifdef __SIZEOF_INT128__ */
#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct __s128(pub i128);

#[repr(align(16))]
#[derive(Copy, Clone)]
pub struct __u128(pub u128);

/*
 * We define u64 as uint64_t for every architecture
 * so that we can print it with "%"PRIx64 without getting warnings.
 *
 * typedef __u64 u64;
 * typedef __s64 s64;
 *
 * In Rust, the primitive names u64, i64, u32, i32, u16, i16, u8, and i8
 * already represent these typedefs.
 */

pub type ullong = ::core::ffi::c_ulonglong;

/* __bitwise is a sparse/checker annotation in C and has no Rust type effect.
 * __force, __user, __must_check, and __cold are likewise C annotations/macros
 * in this header.
 */

pub type __le16 = u16;
pub type __be16 = u16;
pub type __le32 = u32;
pub type __be32 = u32;
pub type __le64 = u64;
pub type __be64 = u64;

pub type __sum16 = u16;
pub type __wsum = u32;

/* C condition:
 *   #ifdef CONFIG_PHYS_ADDR_T_64BIT
 *   typedef u64 phys_addr_t;
 *   #else
 *   typedef u32 phys_addr_t;
 *   #endif
 */
#[cfg(CONFIG_PHYS_ADDR_T_64BIT)]
pub type phys_addr_t = u64;
#[cfg(not(CONFIG_PHYS_ADDR_T_64BIT))]
pub type phys_addr_t = u32;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct atomic_t {
    pub counter: ::core::ffi::c_int,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct atomic_long_t {
    pub counter: ::core::ffi::c_long,
}

#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct __aligned_u64(pub u64);

#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct __aligned_be64(pub __be64);

#[repr(C, align(8))]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct __aligned_le64(pub __le64);

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}

#[repr(C)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}
