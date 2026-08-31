// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/* Copyright (c) 2021 Facebook */
/* Copyright (c) 2024, Oracle and/or its affiliates. */

/* Dependencies from the original C file:
 * - under __KERNEL__: <linux/bpf.h>, <linux/btf.h>
 * - otherwise: "btf.h", "libbpf_internal.h"
 * The __KERNEL__-only btf_var_secinfos(t) macro is not used in this file.
 */

use core::mem::size_of;
use core::ptr;

pub type __u32 = u32;

pub const EINVAL: i32 = 22;

pub const BTF_FIELD_DESC_MAX_T_OFFS: usize = 2;
pub const BTF_FIELD_DESC_MAX_M_OFFS: usize = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_field_desc {
    pub t_off_cnt: __u32,
    pub t_offs: [usize; BTF_FIELD_DESC_MAX_T_OFFS],
    pub m_sz: usize,
    pub m_off_cnt: __u32,
    pub m_offs: [usize; BTF_FIELD_DESC_MAX_M_OFFS],
}

impl Default for btf_field_desc {
    fn default() -> Self {
        Self {
            t_off_cnt: 0,
            t_offs: [0; BTF_FIELD_DESC_MAX_T_OFFS],
            m_sz: 0,
            m_off_cnt: 0,
            m_offs: [0; BTF_FIELD_DESC_MAX_M_OFFS],
        }
    }
}

#[repr(C)]
pub struct btf_field_iter {
    pub p: *mut __u32,
    pub m_idx: i32,
    pub off_idx: __u32,
    pub vlen: __u32,
    pub desc: btf_field_desc,
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
    pub info: __u32,
    pub size: __u32,
}

#[repr(C)]
pub struct btf_array {
    pub type_: __u32,
    pub index_type: __u32,
    pub nelems: __u32,
}

#[repr(C)]
pub struct btf_member {
    pub name_off: __u32,
    pub type_: __u32,
    pub offset: __u32,
}

#[repr(C)]
pub struct btf_param {
    pub name_off: __u32,
    pub type_: __u32,
}

#[repr(C)]
pub struct btf_var_secinfo {
    pub type_: __u32,
    pub offset: __u32,
    pub size: __u32,
}

#[repr(C)]
pub struct btf_enum {
    pub name_off: __u32,
    pub val: i32,
}

#[repr(C)]
pub struct btf_enum64 {
    pub name_off: __u32,
    pub val_lo32: __u32,
    pub val_hi32: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btf_field_iter_kind {
    BTF_FIELD_ITER_IDS,
    BTF_FIELD_ITER_STRS,
}

pub const BTF_KIND_UNKN: __u32 = 0;
pub const BTF_KIND_INT: __u32 = 1;
pub const BTF_KIND_PTR: __u32 = 2;
pub const BTF_KIND_ARRAY: __u32 = 3;
pub const BTF_KIND_STRUCT: __u32 = 4;
pub const BTF_KIND_UNION: __u32 = 5;
pub const BTF_KIND_ENUM: __u32 = 6;
pub const BTF_KIND_FWD: __u32 = 7;
pub const BTF_KIND_TYPEDEF: __u32 = 8;
pub const BTF_KIND_VOLATILE: __u32 = 9;
pub const BTF_KIND_CONST: __u32 = 10;
pub const BTF_KIND_RESTRICT: __u32 = 11;
pub const BTF_KIND_FUNC: __u32 = 12;
pub const BTF_KIND_FUNC_PROTO: __u32 = 13;
pub const BTF_KIND_VAR: __u32 = 14;
pub const BTF_KIND_DATASEC: __u32 = 15;
pub const BTF_KIND_FLOAT: __u32 = 16;
pub const BTF_KIND_DECL_TAG: __u32 = 17;
pub const BTF_KIND_TYPE_TAG: __u32 = 18;
pub const BTF_KIND_ENUM64: __u32 = 19;

extern "C" {
    pub fn btf_kind(t: *const btf_type) -> __u32;
    pub fn btf_vlen(t: *const btf_type) -> __u32;
}

macro_rules! offset_of {
    ($ty:ty, $field:tt) => {{
        let uninit = core::mem::MaybeUninit::<$ty>::uninit();
        let base = uninit.as_ptr();
        unsafe { (&(*base).$field as *const _ as usize) - (base as usize) }
    }};
}

#[no_mangle]
pub unsafe extern "C" fn btf_field_iter_init(
    it: *mut btf_field_iter,
    t: *mut btf_type,
    iter_kind: btf_field_iter_kind,
) -> i32 {
    (*it).p = ptr::null_mut();
    (*it).m_idx = -1;
    (*it).off_idx = 0;
    (*it).vlen = 0;

    match iter_kind {
        btf_field_iter_kind::BTF_FIELD_ITER_IDS => {
            match btf_kind(t) {
                BTF_KIND_UNKN
                | BTF_KIND_INT
                | BTF_KIND_FLOAT
                | BTF_KIND_ENUM
                | BTF_KIND_ENUM64 => {
                    (*it).desc = btf_field_desc::default();
                }
                BTF_KIND_FWD
                | BTF_KIND_CONST
                | BTF_KIND_VOLATILE
                | BTF_KIND_RESTRICT
                | BTF_KIND_PTR
                | BTF_KIND_TYPEDEF
                | BTF_KIND_FUNC
                | BTF_KIND_VAR
                | BTF_KIND_DECL_TAG
                | BTF_KIND_TYPE_TAG => {
                    (*it).desc = btf_field_desc {
                        t_off_cnt: 1,
                        t_offs: [offset_of!(btf_type, size), 0],
                        ..Default::default()
                    };
                }
                BTF_KIND_ARRAY => {
                    (*it).desc = btf_field_desc {
                        t_off_cnt: 2,
                        t_offs: [
                            size_of::<btf_type>() + offset_of!(btf_array, type_),
                            size_of::<btf_type>() + offset_of!(btf_array, index_type),
                        ],
                        ..Default::default()
                    };
                }
                BTF_KIND_STRUCT | BTF_KIND_UNION => {
                    (*it).desc = btf_field_desc {
                        t_off_cnt: 0,
                        t_offs: [0; BTF_FIELD_DESC_MAX_T_OFFS],
                        m_sz: size_of::<btf_member>(),
                        m_off_cnt: 1,
                        m_offs: [offset_of!(btf_member, type_)],
                    };
                }
                BTF_KIND_FUNC_PROTO => {
                    (*it).desc = btf_field_desc {
                        t_off_cnt: 1,
                        t_offs: [offset_of!(btf_type, size), 0],
                        m_sz: size_of::<btf_param>(),
                        m_off_cnt: 1,
                        m_offs: [offset_of!(btf_param, type_)],
                    };
                }
                BTF_KIND_DATASEC => {
                    (*it).desc = btf_field_desc {
                        t_off_cnt: 0,
                        t_offs: [0; BTF_FIELD_DESC_MAX_T_OFFS],
                        m_sz: size_of::<btf_var_secinfo>(),
                        m_off_cnt: 1,
                        m_offs: [offset_of!(btf_var_secinfo, type_)],
                    };
                }
                _ => return -EINVAL,
            }
        }
        btf_field_iter_kind::BTF_FIELD_ITER_STRS => {
            match btf_kind(t) {
                BTF_KIND_UNKN => {
                    (*it).desc = btf_field_desc::default();
                }
                BTF_KIND_INT
                | BTF_KIND_FLOAT
                | BTF_KIND_FWD
                | BTF_KIND_ARRAY
                | BTF_KIND_CONST
                | BTF_KIND_VOLATILE
                | BTF_KIND_RESTRICT
                | BTF_KIND_PTR
                | BTF_KIND_TYPEDEF
                | BTF_KIND_FUNC
                | BTF_KIND_VAR
                | BTF_KIND_DECL_TAG
                | BTF_KIND_TYPE_TAG
                | BTF_KIND_DATASEC => {
                    (*it).desc = btf_field_desc {
                        t_off_cnt: 1,
                        t_offs: [offset_of!(btf_type, name_off), 0],
                        ..Default::default()
                    };
                }
                BTF_KIND_ENUM => {
                    (*it).desc = btf_field_desc {
                        t_off_cnt: 1,
                        t_offs: [offset_of!(btf_type, name_off), 0],
                        m_sz: size_of::<btf_enum>(),
                        m_off_cnt: 1,
                        m_offs: [offset_of!(btf_enum, name_off)],
                    };
                }
                BTF_KIND_ENUM64 => {
                    (*it).desc = btf_field_desc {
                        t_off_cnt: 1,
                        t_offs: [offset_of!(btf_type, name_off), 0],
                        m_sz: size_of::<btf_enum64>(),
                        m_off_cnt: 1,
                        m_offs: [offset_of!(btf_enum64, name_off)],
                    };
                }
                BTF_KIND_STRUCT | BTF_KIND_UNION => {
                    (*it).desc = btf_field_desc {
                        t_off_cnt: 1,
                        t_offs: [offset_of!(btf_type, name_off), 0],
                        m_sz: size_of::<btf_member>(),
                        m_off_cnt: 1,
                        m_offs: [offset_of!(btf_member, name_off)],
                    };
                }
                BTF_KIND_FUNC_PROTO => {
                    (*it).desc = btf_field_desc {
                        t_off_cnt: 1,
                        t_offs: [offset_of!(btf_type, name_off), 0],
                        m_sz: size_of::<btf_param>(),
                        m_off_cnt: 1,
                        m_offs: [offset_of!(btf_param, name_off)],
                    };
                }
                _ => return -EINVAL,
            }
        }
    }

    if (*it).desc.m_sz != 0 {
        (*it).vlen = btf_vlen(t);
    }

    (*it).p = t as *mut __u32;
    0
}

#[no_mangle]
pub unsafe extern "C" fn btf_field_iter_next(it: *mut btf_field_iter) -> *mut __u32 {
    if (*it).p.is_null() {
        return ptr::null_mut();
    }

    if (*it).m_idx < 0 {
        if (*it).off_idx < (*it).desc.t_off_cnt {
            let off_idx = (*it).off_idx as usize;
            (*it).off_idx = (*it).off_idx.wrapping_add(1);
            return ((*it).p as *mut u8).add((*it).desc.t_offs[off_idx]) as *mut __u32;
        }
        /* move to per-member iteration */
        (*it).m_idx = 0;
        (*it).p = ((*it).p as *mut u8).add(size_of::<btf_type>()) as *mut __u32;
        (*it).off_idx = 0;
    }

    /* if type doesn't have members, stop */
    if (*it).desc.m_sz == 0 {
        (*it).p = ptr::null_mut();
        return ptr::null_mut();
    }

    if (*it).off_idx >= (*it).desc.m_off_cnt {
        /* exhausted this member's fields, go to the next member */
        (*it).m_idx += 1;
        (*it).p = ((*it).p as *mut u8).add((*it).desc.m_sz) as *mut __u32;
        (*it).off_idx = 0;
    }

    if (*it).m_idx < (*it).vlen as i32 {
        let off_idx = (*it).off_idx as usize;
        (*it).off_idx = (*it).off_idx.wrapping_add(1);
        return ((*it).p as *mut u8).add((*it).desc.m_offs[off_idx]) as *mut __u32;
    }

    (*it).p = ptr::null_mut();
    ptr::null_mut()
}
