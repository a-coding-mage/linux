/* SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB */
/* Copyright (c) 2017, Mellanox Technologies inc.  All rights reserved. */

// Dependencies supplied by the surrounding translation unit:
// rdma/uverbs_types.h, linux/uaccess.h, rdma/rdma_user_ioctl.h,
// rdma/ib_user_ioctl_verbs.h, and rdma/ib_user_ioctl_cmds.h.

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum uverbs_attr_type {
    UVERBS_ATTR_TYPE_NA,
    UVERBS_ATTR_TYPE_PTR_IN,
    UVERBS_ATTR_TYPE_PTR_OUT,
    UVERBS_ATTR_TYPE_IDR,
    UVERBS_ATTR_TYPE_FD,
    UVERBS_ATTR_TYPE_RAW_FD,
    UVERBS_ATTR_TYPE_ENUM_IN,
    UVERBS_ATTR_TYPE_IDRS_ARRAY,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum uverbs_obj_access {
    UVERBS_ACCESS_READ,
    UVERBS_ACCESS_WRITE,
    UVERBS_ACCESS_NEW,
    UVERBS_ACCESS_DESTROY,
}

#[repr(C)]
pub union uverbs_attr_spec_u {
    pub ptr: uverbs_attr_spec_u_ptr,
    pub obj: uverbs_attr_spec_u_obj,
    pub enum_def: uverbs_attr_spec_u_enum_def,
}
#[repr(C)] pub struct uverbs_attr_spec_u_ptr { pub len: u16, pub min_len: u16 }
#[repr(C)] pub struct uverbs_attr_spec_u_obj { pub obj_type: u16, pub access: u8 }
#[repr(C)] pub struct uverbs_attr_spec_u_enum_def { pub num_elems: u8 }

#[repr(C)]
pub union uverbs_attr_spec_u2 {
    pub enum_def: uverbs_attr_spec_u2_enum_def,
    pub objs_arr: uverbs_attr_spec_u2_objs_arr,
}
#[repr(C)] pub struct uverbs_attr_spec_u2_enum_def { pub ids: *const uverbs_attr_spec }
#[repr(C)] pub struct uverbs_attr_spec_u2_objs_arr {
    pub obj_type: u16, pub min_len: u16, pub max_len: u16, pub access: u8,
}

#[repr(C)]
pub struct uverbs_attr_spec {
    pub type_: u8,
    pub zero_trailing: u8,
    pub alloc_and_copy: u8,
    pub mandatory: u8,
    pub is_udata: u8,
    pub u: uverbs_attr_spec_u,
    pub u2: uverbs_attr_spec_u2,
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum uapi_radix_data {
    UVERBS_API_NS_FLAG = 1u32 << UVERBS_ID_NS_SHIFT,
    UVERBS_API_ATTR_KEY_BITS = 6,
    UVERBS_API_ATTR_KEY_MASK = GENMASK(UVERBS_API_ATTR_KEY_BITS - 1, 0),
    UVERBS_API_ATTR_BKEY_LEN = (1 << UVERBS_API_ATTR_KEY_BITS) - 1,
    UVERBS_API_WRITE_KEY_NUM = 1 << UVERBS_API_ATTR_KEY_BITS,
    UVERBS_API_METHOD_KEY_BITS = 5,
    UVERBS_API_METHOD_KEY_SHIFT = UVERBS_API_ATTR_KEY_BITS,
    UVERBS_API_METHOD_KEY_NUM_CORE = 22,
    UVERBS_API_METHOD_IS_WRITE = 30 << UVERBS_API_METHOD_KEY_SHIFT,
    UVERBS_API_METHOD_IS_WRITE_EX = 31 << UVERBS_API_METHOD_KEY_SHIFT,
    UVERBS_API_METHOD_KEY_NUM_DRIVER =
        (UVERBS_API_METHOD_IS_WRITE >> UVERBS_API_METHOD_KEY_SHIFT) - UVERBS_API_METHOD_KEY_NUM_CORE,
    UVERBS_API_METHOD_KEY_MASK = GENMASK(
        UVERBS_API_METHOD_KEY_BITS + UVERBS_API_METHOD_KEY_SHIFT - 1,
        UVERBS_API_METHOD_KEY_SHIFT),
    UVERBS_API_OBJ_KEY_BITS = 5,
    UVERBS_API_OBJ_KEY_SHIFT = UVERBS_API_METHOD_KEY_BITS + UVERBS_API_METHOD_KEY_SHIFT,
    UVERBS_API_OBJ_KEY_NUM_CORE = 20,
    UVERBS_API_OBJ_KEY_NUM_DRIVER = (1 << UVERBS_API_OBJ_KEY_BITS) - UVERBS_API_OBJ_KEY_NUM_CORE,
    UVERBS_API_OBJ_KEY_MASK = GENMASK(31, UVERBS_API_OBJ_KEY_SHIFT),
    UVERBS_API_KEY_ERR = 0xFFFFFFFF,
}

#[inline] pub const fn uapi_key_obj(mut id: u32) -> u32 {
    if id & UVERBS_API_NS_FLAG != 0 { id &= !UVERBS_API_NS_FLAG; if id >= UVERBS_API_OBJ_KEY_NUM_DRIVER { return UVERBS_API_KEY_ERR; } id += UVERBS_API_OBJ_KEY_NUM_CORE; }
    else if id >= UVERBS_API_OBJ_KEY_NUM_CORE { return UVERBS_API_KEY_ERR; }
    id << UVERBS_API_OBJ_KEY_SHIFT
}
#[inline] pub const fn uapi_key_is_object(key: u32) -> bool { (key & !UVERBS_API_OBJ_KEY_MASK) == 0 }
#[inline] pub const fn uapi_key_ioctl_method(mut id: u32) -> u32 {
    if id & UVERBS_API_NS_FLAG != 0 { id &= !UVERBS_API_NS_FLAG; if id >= UVERBS_API_METHOD_KEY_NUM_DRIVER { return UVERBS_API_KEY_ERR; } id += UVERBS_API_METHOD_KEY_NUM_CORE; }
    else { id += 1; if id >= UVERBS_API_METHOD_KEY_NUM_CORE { return UVERBS_API_KEY_ERR; } }
    id << UVERBS_API_METHOD_KEY_SHIFT
}
#[inline] pub const fn uapi_key_write_method(id: u32) -> u32 { if id >= UVERBS_API_WRITE_KEY_NUM { UVERBS_API_KEY_ERR } else { UVERBS_API_METHOD_IS_WRITE | id } }
#[inline] pub const fn uapi_key_write_ex_method(id: u32) -> u32 { if id >= UVERBS_API_WRITE_KEY_NUM { UVERBS_API_KEY_ERR } else { UVERBS_API_METHOD_IS_WRITE_EX | id } }
#[inline] pub const fn uapi_key_attr_to_ioctl_method(attr_key: u32) -> u32 { attr_key & (UVERBS_API_OBJ_KEY_MASK | UVERBS_API_METHOD_KEY_MASK) }
#[inline] pub const fn uapi_key_is_ioctl_method(key: u32) -> bool { let method = key & UVERBS_API_METHOD_KEY_MASK; method != 0 && method < UVERBS_API_METHOD_IS_WRITE && key & UVERBS_API_ATTR_KEY_MASK == 0 }
#[inline] pub const fn uapi_key_is_write_method(key: u32) -> bool { key & UVERBS_API_METHOD_KEY_MASK == UVERBS_API_METHOD_IS_WRITE }
#[inline] pub const fn uapi_key_is_write_ex_method(key: u32) -> bool { key & UVERBS_API_METHOD_KEY_MASK == UVERBS_API_METHOD_IS_WRITE_EX }
#[inline] pub const fn uapi_key_attrs_start(key: u32) -> u32 { key + 1 }
#[inline] pub const fn uapi_key_attr(mut id: u32) -> u32 {
    if id & UVERBS_API_NS_FLAG != 0 { id &= !UVERBS_API_NS_FLAG; id += 1; if id >= 1 << (UVERBS_API_ATTR_KEY_BITS - 1) { return UVERBS_API_KEY_ERR; } id = (id << 1) | 0; }
    else { if id >= 1 << (UVERBS_API_ATTR_KEY_BITS - 1) { return UVERBS_API_KEY_ERR; } id = (id << 1) | 1; } id
}
#[inline] pub const fn uapi_key_is_attr(key: u32) -> bool { let method = key & UVERBS_API_METHOD_KEY_MASK; method != 0 && method < UVERBS_API_METHOD_IS_WRITE && key & UVERBS_API_ATTR_KEY_MASK != 0 }
#[inline] pub const fn uapi_bkey_attr(attr_key: u32) -> u32 { attr_key - 1 }
#[inline] pub const fn uapi_bkey_to_key_attr(attr_bkey: u32) -> u32 { attr_bkey + 1 }

#[repr(C)] pub struct uverbs_attr_def { pub id: u16, pub attr: uverbs_attr_spec }
#[repr(C)] pub struct uverbs_method_def {
    pub id: u16, pub flags: u32, pub num_attrs: usize,
    pub attrs: *const *const uverbs_attr_def,
    pub handler: Option<unsafe extern "C" fn(*mut uverbs_attr_bundle) -> i32>,
}
#[repr(C)] pub struct uverbs_object_def {
    pub id: u16, pub type_attrs: *const uverbs_obj_type, pub num_methods: usize,
    pub methods: *const *const uverbs_method_def,
}

#[repr(u8)] pub enum uapi_definition_kind { UAPI_DEF_END = 0, UAPI_DEF_OBJECT_START, UAPI_DEF_WRITE, UAPI_DEF_CHAIN_OBJ_TREE, UAPI_DEF_CHAIN, UAPI_DEF_IS_SUPPORTED_FUNC, UAPI_DEF_IS_SUPPORTED_DEV_FN }
#[repr(u8)] pub enum uapi_definition_scope { UAPI_SCOPE_OBJECT = 1, UAPI_SCOPE_METHOD = 2 }
#[repr(C)] pub union uapi_definition_func { pub func_is_supported: Option<unsafe extern "C" fn(*mut ib_device) -> bool>, pub func_write: Option<unsafe extern "C" fn(*mut uverbs_attr_bundle) -> i32>, pub chain: *const uapi_definition, pub chain_obj_tree: *const uverbs_object_def, pub needs_fn_offset: usize }
#[repr(C)] pub struct uapi_definition_object_start { pub object_id: u16 }
#[repr(C)] pub struct uapi_definition_write { pub command_num: u16, pub is_ex: u8, pub has_udata: u8, pub has_resp: u8, pub req_size: u8, pub resp_size: u8 }
#[repr(C)] pub union uapi_definition_data { pub object_start: uapi_definition_object_start, pub write: uapi_definition_write }
#[repr(C)] pub struct uapi_definition { pub kind: u8, pub scope: u8, pub data: uapi_definition_data, pub func: uapi_definition_func }

// Declaration helpers corresponding to the C variadic initializer macros.
#[macro_export] macro_rules! DECLARE_UVERBS_OBJECT { ($object_id:expr $(, $rest:tt)*) => { uapi_definition { kind: UAPI_DEF_OBJECT_START as u8, scope: 0, data: uapi_definition_data { object_start: uapi_definition_object_start { object_id: $object_id } }, func: uapi_definition_func { needs_fn_offset: 0 } } $(, $rest)* }; }
#[macro_export] macro_rules! DECLARE_UVERBS_WRITE { ($command_num:expr, $func:expr, $desc:expr $(, $rest:tt)*) => { uapi_definition { kind: UAPI_DEF_WRITE as u8, scope: UAPI_SCOPE_OBJECT as u8, data: uapi_definition_data { write: uapi_definition_write { command_num: $command_num, is_ex: 0, has_udata: 0, has_resp: 0, req_size: 0, resp_size: 0 } }, func: uapi_definition_func { func_write: Some($func) } } $(, $rest)* }; }
#[macro_export] macro_rules! DECLARE_UVERBS_WRITE_EX { ($command_num:expr, $func:expr, $desc:expr $(, $rest:tt)*) => { uapi_definition { kind: UAPI_DEF_WRITE as u8, scope: UAPI_SCOPE_OBJECT as u8, data: uapi_definition_data { write: uapi_definition_write { command_num: $command_num, is_ex: 1, has_udata: 0, has_resp: 0, req_size: 0, resp_size: 0 } }, func: uapi_definition_func { func_write: Some($func) } } $(, $rest)* }; }
#[macro_export] macro_rules! UAPI_DEF_CHAIN { ($def_var:expr) => { uapi_definition { kind: UAPI_DEF_CHAIN as u8, scope: 0, data: uapi_definition_data { object_start: uapi_definition_object_start { object_id: 0 } }, func: uapi_definition_func { chain: $def_var } } }; }
#[macro_export] macro_rules! UAPI_DEF_CHAIN_OBJ_TREE { ($object_enum:expr, $object_ptr:expr $(, $rest:tt)*) => { uapi_definition { kind: UAPI_DEF_CHAIN_OBJ_TREE as u8, scope: 0, data: uapi_definition_data { object_start: uapi_definition_object_start { object_id: $object_enum } }, func: uapi_definition_func { chain_obj_tree: $object_ptr } } $(, $rest)* }; }
#[macro_export] macro_rules! UVERBS_ATTR_SIZE { ($min_len:expr, $len:expr) => { .u.ptr.min_len = $min_len, .u.ptr.len = $len }; }
#[macro_export] macro_rules! UVERBS_ATTR_NO_DATA { () => { UVERBS_ATTR_SIZE!(0, 0) }; }
#[macro_export] macro_rules! UVERBS_ATTR_TYPE { ($type:ty) => { .u.ptr.min_len = core::mem::size_of::<$type>(), .u.ptr.len = core::mem::size_of::<$type>() }; }
#[macro_export] macro_rules! UVERBS_ATTR_STRUCT { ($type:ty, $last:tt) => { .zero_trailing = 1, UVERBS_ATTR_SIZE!(core::mem::offset_of!($type, $last) + core::mem::size_of_val(&(*(core::ptr::null::<$type>())).$last), core::mem::size_of::<$type>()) }; }
#[macro_export] macro_rules! UVERBS_ATTR_MIN_SIZE { ($min_len:expr) => { UVERBS_ATTR_SIZE!($min_len, u16::MAX) }; }
#[macro_export] macro_rules! UA_ALLOC_AND_COPY { () => { .alloc_and_copy = 1 }; }
#[macro_export] macro_rules! UA_MANDATORY { () => { .mandatory = 1 }; }
#[macro_export] macro_rules! UA_OPTIONAL { () => { .mandatory = 0 }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
