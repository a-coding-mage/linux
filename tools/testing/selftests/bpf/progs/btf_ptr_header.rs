/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2020, Oracle and/or its affiliates. */
/* "undefine" structs in vmlinux.h, because we "override" them below */
/* C header temporarily remaps btf_ptr and BTF_F_* names before including vmlinux.h,
 * then undefines them so the local definitions below can override those names.
 */

#[repr(C)]
pub struct btf_ptr {
    pub ptr: *mut core::ffi::c_void,
    pub type_id: __u32,
    pub flags: __u32,
}

pub const BTF_F_COMPACT: u64 = 1u64 << 0;
pub const BTF_F_NONAME: u64 = 1u64 << 1;
pub const BTF_F_PTR_RAW: u64 = 1u64 << 2;
pub const BTF_F_ZERO: u64 = 1u64 << 3;
