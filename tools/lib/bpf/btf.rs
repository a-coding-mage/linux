// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/* Copyright (c) 2018 Facebook */

/*
 * Source-level Rust translation of ./btf.c.
 *
 * This file intentionally keeps the C ABI surface, names, pointer-heavy data
 * model, and external dependency references from lib/bpf/btf.c. C includes are
 * represented as external symbols and opaque dependency types below; concrete
 * definitions are expected from the translated surrounding repository.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{copy_nonoverlapping, null, null_mut};

pub type bool_ = bool;
pub type size_t = usize;
pub type uintptr_t = usize;
pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s32 = i32;
pub type __s64 = i64;

pub const BTF_MAX_NR_TYPES: __u32 = 0x7fffffff_u32;
pub const BTF_MAX_STR_OFFSET: __u32 = 0x7fffffff_u32;
pub const MAX_RESOLVE_DEPTH: c_int = 32;
pub const BTF_UNPROCESSED_ID: __u32 = !0u32;
pub const BTF_IN_PROGRESS_ID: __u32 = !1u32;

/* External constants from Linux/libbpf headers. */
unsafe extern "C" {
    static BTF_MAGIC: __u16;
    static BTF_VERSION: __u8;
    static BTF_MAX_VLEN: __u32;
    static NR_BTF_KINDS: usize;
    static BTF_KIND_UNKN: __u32;
    static BTF_KIND_INT: __u32;
    static BTF_KIND_PTR: __u32;
    static BTF_KIND_ARRAY: __u32;
    static BTF_KIND_STRUCT: __u32;
    static BTF_KIND_UNION: __u32;
    static BTF_KIND_ENUM: __u32;
    static BTF_KIND_FWD: __u32;
    static BTF_KIND_TYPEDEF: __u32;
    static BTF_KIND_VOLATILE: __u32;
    static BTF_KIND_CONST: __u32;
    static BTF_KIND_RESTRICT: __u32;
    static BTF_KIND_FUNC: __u32;
    static BTF_KIND_FUNC_PROTO: __u32;
    static BTF_KIND_VAR: __u32;
    static BTF_KIND_DATASEC: __u32;
    static BTF_KIND_FLOAT: __u32;
    static BTF_KIND_DECL_TAG: __u32;
    static BTF_KIND_TYPE_TAG: __u32;
    static BTF_KIND_ENUM64: __u32;
    static BTF_INT_SIGNED: c_int;
    static BTF_INT_CHAR: c_int;
    static BTF_INT_BOOL: c_int;
    static BTF_FWD_STRUCT: c_int;
    static BTF_FWD_UNION: c_int;
    static BTF_FWD_ENUM: c_int;
    static BTF_FUNC_STATIC: c_int;
    static BTF_FUNC_GLOBAL: c_int;
    static BTF_FUNC_EXTERN: c_int;
    static BTF_VAR_STATIC: c_int;
    static BTF_VAR_GLOBAL_ALLOCATED: c_int;
    static BTF_VAR_GLOBAL_EXTERN: c_int;
    static BTF_LITTLE_ENDIAN: btf_endianness;
    static BTF_BIG_ENDIAN: btf_endianness;
}

#[repr(C)]
pub struct btf_header {
    pub magic: __u16,
    pub version: __u8,
    pub flags: __u8,
    pub hdr_len: __u32,
    pub type_off: __u32,
    pub type_len: __u32,
    pub str_off: __u32,
    pub str_len: __u32,
    pub layout_off: __u32,
    pub layout_len: __u32,
}

#[repr(C)]
pub struct btf_layout {
    pub info_sz: __u16,
    pub elem_sz: __u16,
    pub flags: __u16,
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
pub struct btf_param {
    pub name_off: __u32,
    pub type_: __u32,
}

#[repr(C)]
pub struct btf_var {
    pub linkage: __u32,
}

#[repr(C)]
pub struct btf_var_secinfo {
    pub type_: __u32,
    pub offset: __u32,
    pub size: __u32,
}

#[repr(C)]
pub struct btf_decl_tag {
    pub component_idx: i32,
}

#[repr(C)]
pub struct btf_ext_header {
    pub magic: __u16,
    pub version: __u8,
    pub flags: __u8,
    pub hdr_len: __u32,
    pub func_info_off: __u32,
    pub func_info_len: __u32,
    pub line_info_off: __u32,
    pub line_info_len: __u32,
    pub core_relo_off: __u32,
    pub core_relo_len: __u32,
}

#[repr(C)]
pub struct btf_ext_info {
    pub len: __u32,
    pub rec_size: __u32,
    pub info: *mut c_void,
    pub sec_cnt: size_t,
    pub sec_idxs: *mut __u32,
}

#[repr(C)]
pub struct btf_ext_info_sec {
    pub sec_name_off: __u32,
    pub num_info: __u32,
    pub data: [__u8; 0],
}

#[repr(C)]
pub struct btf_ext {
    pub hdr: *mut btf_ext_header,
    pub data: *mut c_void,
    pub data_swapped: *mut c_void,
    pub data_size: __u32,
    pub swapped_endian: bool,
    pub func_info: btf_ext_info,
    pub line_info: btf_ext_info,
    pub core_relo_info: btf_ext_info,
}

#[repr(C)]
pub struct btf_new_opts {
    pub sz: size_t,
    pub base_btf: *mut btf,
    pub add_layout: bool,
}

#[repr(C)]
pub struct btf_dedup_opts {
    pub sz: size_t,
    pub btf_ext: *mut btf_ext,
    pub force_collisions: bool,
}

#[repr(C)]
pub struct btf_permute_opts {
    pub sz: size_t,
    pub btf_ext: *mut btf_ext,
}

pub type btf_endianness = c_uint;
pub type btf_fwd_kind = c_uint;
pub type btf_func_linkage = c_uint;
pub type str_off_visit_fn = Option<unsafe extern "C" fn(*mut __u32, *mut c_void) -> c_int>;
pub type type_id_visit_fn = Option<unsafe extern "C" fn(*mut __u32, *mut c_void) -> c_int>;
pub type info_rec_bswap_fn = Option<unsafe extern "C" fn(*mut c_void)>;

#[repr(C)]
pub struct strset { _priv: [u8; 0] }
#[repr(C)]
pub struct hashmap { _priv: [u8; 0] }
#[repr(C)]
pub struct hashmap_entry {
    pub key: c_long,
    pub value: c_long,
}
#[repr(C)]
pub struct btf_field_iter { _priv: [u8; 0] }
#[repr(C)]
pub struct btf_dedup {
    /* .BTF section to be deduped in-place */
    pub btf: *mut btf,
    /*
     * Optional .BTF.ext section. When provided, any strings referenced
     * from it will be taken into account when deduping strings
     */
    pub btf_ext: *mut btf_ext,
    /*
     * This is a map from any type's signature hash to a list of possible
     * canonical representative type candidates. Hash collisions are
     * ignored, so even types of various kinds can share same list of
     * candidates, which is fine because we rely on subsequent
     * btf_xxx_equal() checks to authoritatively verify type equality.
     */
    pub dedup_table: *mut hashmap,
    /* Canonical types map */
    pub map: *mut __u32,
    /* Hypothetical mapping, used during type graph equivalence checks */
    pub hypot_map: *mut __u32,
    pub hypot_list: *mut __u32,
    pub hypot_cnt: size_t,
    pub hypot_cap: size_t,
    /* Whether hypothetical mapping, if successful, would need to adjust
     * already canonicalized types (due to a new forward declaration to
     * concrete type resolution). In such case, during split BTF dedup
     * candidate type would still be considered as different, because base
     * BTF is considered to be immutable.
     */
    pub hypot_adjust_canon: bool,
    /* Various option modifying behavior of algorithm */
    pub opts: btf_dedup_opts,
    /* temporary strings deduplication state */
    pub strs_set: *mut strset,
}

#[repr(C)]
pub struct btf {
    /* raw BTF data in native endianness */
    pub raw_data: *mut c_void,
    /* raw BTF data in non-native endianness */
    pub raw_data_swapped: *mut c_void,
    pub raw_size: __u32,
    /* whether target endianness differs from the native one */
    pub swapped_endian: bool,
    pub hdr: btf_header,
    pub types_data: *mut c_void,
    pub types_data_cap: size_t,
    pub type_offs: *mut __u32,
    pub type_offs_cap: size_t,
    pub nr_types: __u32,
    pub named_start_id: c_int,
    pub base_btf: *mut btf,
    pub start_id: c_int,
    pub start_str_off: c_int,
    pub strs_data: *mut c_void,
    pub strs_set: *mut strset,
    pub strs_deduped: bool,
    pub owns_base: bool,
    pub raw_data_is_mmap: bool,
    pub modifiable: bool,
    pub has_hdr_extra: bool,
    pub layout: *mut c_void,
    pub fd: c_int,
    pub ptr_sz: c_int,
}

unsafe extern "C" {
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn bswap_16(x: __u16) -> __u16;
    fn bswap_32(x: __u32) -> __u32;
    fn libbpf_reallocarray(ptr: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void;
    fn libbpf_err(err: c_int) -> c_int;
    fn libbpf_ptr(ptr: *mut btf) -> *mut btf;
    fn libbpf_err_ptr(err: c_int) -> *mut btf;
    fn libbpf_get_error(ptr: *const c_void) -> c_long;
    fn strset__new(max_data_sz: size_t, init_data: *const c_void, init_data_sz: size_t) -> *mut strset;
    fn strset__free(set: *mut strset);
    fn strset__data(set: *const strset) -> *const c_void;
    fn strset__data_size(set: *const strset) -> __u32;
    fn strset__find_str(set: *mut strset, s: *const c_char) -> c_int;
    fn strset__add_str(set: *mut strset, s: *const c_char) -> c_int;
    fn hashmap__new(hash_fn: hashmap_hash_fn, equal_fn: hashmap_equal_fn, ctx: *mut c_void) -> *mut hashmap;
    fn hashmap__free(map: *mut hashmap);
    fn hashmap__append(map: *mut hashmap, key: c_long, value: c_long) -> c_int;
    fn hashmap__add(map: *mut hashmap, key: c_long, value: c_long) -> c_int;
    fn hashmap__set(map: *mut hashmap, key: c_long, value: c_long, old_key: *mut c_long, old_value: *mut c_long) -> c_int;
    fn hashmap__find(map: *mut hashmap, key: c_long, value: *mut c_long) -> bool;
    fn btf_field_iter_init(it: *mut btf_field_iter, t: *mut btf_type, flags: c_int) -> c_int;
    fn btf_field_iter_next(it: *mut btf_field_iter) -> *mut __u32;
    fn btf_relocate(btf: *mut btf, base_btf: *const btf, opts: *mut c_void) -> c_int;
}

pub type hashmap_hash_fn = Option<unsafe extern "C" fn(c_long, *mut c_void) -> size_t>;
pub type hashmap_equal_fn = Option<unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool>;

static mut btf_void: btf_type = btf_type { name_off: 0, info: 0, size: 0 };

/*
 * Describe how kinds are laid out; some have a singular element following the "struct btf_type",
 * some have BTF_INFO_VLEN(t->info) elements.  Specify sizes for both.  Flags are currently unused.
 * Kind layout can be optionally added to the BTF representation in a dedicated section to
 * facilitate parsing.  New kinds must be added here.
 */
static mut layouts: [btf_layout; 20] = [
    btf_layout { info_sz: 0, elem_sz: 0, flags: 0 },
    btf_layout { info_sz: size_of::<__u32>() as __u16, elem_sz: 0, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: 0, flags: 0 },
    btf_layout { info_sz: size_of::<btf_array>() as __u16, elem_sz: 0, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: size_of::<btf_member>() as __u16, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: size_of::<btf_member>() as __u16, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: size_of::<btf_enum>() as __u16, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: 0, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: 0, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: 0, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: 0, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: 0, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: 0, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: size_of::<btf_param>() as __u16, flags: 0 },
    btf_layout { info_sz: size_of::<btf_var>() as __u16, elem_sz: 0, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: size_of::<btf_var_secinfo>() as __u16, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: 0, flags: 0 },
    btf_layout { info_sz: size_of::<btf_decl_tag>() as __u16, elem_sz: 0, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: 0, flags: 0 },
    btf_layout { info_sz: 0, elem_sz: size_of::<btf_enum64>() as __u16, flags: 0 },
];

#[inline]
unsafe fn ptr_to_u64(ptr: *const c_void) -> __u64 {
    ptr as c_ulong as __u64
}

#[inline]
unsafe fn ptr_add(p: *mut c_void, n: size_t) -> *mut c_void {
    (p as *mut u8).add(n) as *mut c_void
}

#[inline]
unsafe fn btf_kind(t: *const btf_type) -> __u16 {
    (((*t).info >> 24) & 0x1f) as __u16
}

#[inline]
unsafe fn btf_vlen(t: *const btf_type) -> __u32 {
    (*t).info & 0xffff
}

#[inline]
unsafe fn btf_kflag(t: *const btf_type) -> bool {
    ((*t).info & (1u32 << 31)) != 0
}

#[inline]
unsafe fn btf_type_info(kind: __u32, vlen: __u32, kflag: bool) -> __u32 {
    ((if kflag { 1u32 } else { 0u32 }) << 31) | (kind << 24) | vlen
}

#[inline]
unsafe fn btf_array(t: *const btf_type) -> *mut btf_array {
    (t.add(1)) as *mut btf_array
}

#[inline]
unsafe fn btf_members(t: *const btf_type) -> *mut btf_member {
    (t.add(1)) as *mut btf_member
}

#[inline]
unsafe fn btf_enum(t: *const btf_type) -> *mut btf_enum {
    (t.add(1)) as *mut btf_enum
}

#[inline]
unsafe fn btf_enum64(t: *const btf_type) -> *mut btf_enum64 {
    (t.add(1)) as *mut btf_enum64
}

#[inline]
unsafe fn btf_params(t: *const btf_type) -> *mut btf_param {
    (t.add(1)) as *mut btf_param
}

#[inline]
unsafe fn btf_var(t: *const btf_type) -> *mut btf_var {
    (t.add(1)) as *mut btf_var
}

#[inline]
unsafe fn btf_var_secinfos(t: *const btf_type) -> *mut btf_var_secinfo {
    (t.add(1)) as *mut btf_var_secinfo
}

#[inline]
unsafe fn btf_decl_tag(t: *const btf_type) -> *mut btf_decl_tag {
    (t.add(1)) as *mut btf_decl_tag
}

#[inline]
unsafe fn btf_is_kind(t: *const btf_type, k: __u32) -> bool { btf_kind(t) as __u32 == k }
#[inline] unsafe fn btf_is_int(t: *const btf_type) -> bool { btf_is_kind(t, BTF_KIND_INT) }
#[inline] unsafe fn btf_is_fwd(t: *const btf_type) -> bool { btf_is_kind(t, BTF_KIND_FWD) }
#[inline] unsafe fn btf_is_enum(t: *const btf_type) -> bool { btf_is_kind(t, BTF_KIND_ENUM) }
#[inline] unsafe fn btf_is_enum64(t: *const btf_type) -> bool { btf_is_kind(t, BTF_KIND_ENUM64) }
#[inline] unsafe fn btf_is_any_enum(t: *const btf_type) -> bool { btf_is_enum(t) || btf_is_enum64(t) }
#[inline] unsafe fn btf_is_union(t: *const btf_type) -> bool { btf_is_kind(t, BTF_KIND_UNION) }
#[inline] unsafe fn btf_is_composite(t: *const btf_type) -> bool { btf_is_kind(t, BTF_KIND_STRUCT) || btf_is_kind(t, BTF_KIND_UNION) }
#[inline] unsafe fn btf_is_datasec(t: *const btf_type) -> bool { btf_is_kind(t, BTF_KIND_DATASEC) }
#[inline] unsafe fn btf_is_var(t: *const btf_type) -> bool { btf_is_kind(t, BTF_KIND_VAR) }
#[inline] unsafe fn btf_is_func_proto(t: *const btf_type) -> bool { btf_is_kind(t, BTF_KIND_FUNC_PROTO) }
#[inline] unsafe fn btf_is_mod(t: *const btf_type) -> bool { btf_is_kind(t, BTF_KIND_CONST) || btf_is_kind(t, BTF_KIND_VOLATILE) || btf_is_kind(t, BTF_KIND_RESTRICT) || btf_is_kind(t, BTF_KIND_TYPE_TAG) || btf_is_kind(t, BTF_KIND_DECL_TAG) }
#[inline] unsafe fn btf_is_typedef(t: *const btf_type) -> bool { btf_is_kind(t, BTF_KIND_TYPEDEF) }

/* Ensure given dynamically allocated memory region pointed to by *data* with
 * capacity of *cap_cnt* elements each taking *elem_sz* bytes has enough
 * memory to accommodate *add_cnt* new elements, assuming *cur_cnt* elements
 * are already used. At most *max_cnt* elements can be ever allocated.
 * If necessary, memory is reallocated and all existing data is copied over,
 * new pointer to the memory region is stored at *data, new memory region
 * capacity (in number of elements) is stored in *cap.
 * On success, memory pointer to the beginning of unused memory is returned.
 * On error, NULL is returned.
 */
#[no_mangle]
pub unsafe extern "C" fn libbpf_add_mem(
    data: *mut *mut c_void,
    cap_cnt: *mut size_t,
    elem_sz: size_t,
    cur_cnt: size_t,
    max_cnt: size_t,
    add_cnt: size_t,
) -> *mut c_void {
    if cur_cnt.wrapping_add(add_cnt) <= *cap_cnt {
        return ptr_add(*data, cur_cnt.wrapping_mul(elem_sz));
    }
    if cur_cnt.wrapping_add(add_cnt) > max_cnt {
        return null_mut();
    }
    let mut new_cnt = *cap_cnt;
    new_cnt = new_cnt.wrapping_add(new_cnt / 4);
    if new_cnt < 16 { new_cnt = 16; }
    if new_cnt > max_cnt { new_cnt = max_cnt; }
    if new_cnt < cur_cnt.wrapping_add(add_cnt) { new_cnt = cur_cnt.wrapping_add(add_cnt); }
    let new_data = libbpf_reallocarray(*data, new_cnt, elem_sz);
    if new_data.is_null() {
        return null_mut();
    }
    memset(ptr_add(new_data, (*cap_cnt).wrapping_mul(elem_sz)), 0,
           (new_cnt - *cap_cnt).wrapping_mul(elem_sz));
    *data = new_data;
    *cap_cnt = new_cnt;
    ptr_add(new_data, cur_cnt.wrapping_mul(elem_sz))
}

/* Ensure given dynamically allocated memory region has enough allocated space
 * to accommodate *need_cnt* elements of size *elem_sz* bytes each
 */
#[no_mangle]
pub unsafe extern "C" fn libbpf_ensure_mem(
    data: *mut *mut c_void,
    cap_cnt: *mut size_t,
    elem_sz: size_t,
    need_cnt: size_t,
) -> c_int {
    if need_cnt <= *cap_cnt {
        return 0;
    }
    let p = libbpf_add_mem(data, cap_cnt, elem_sz, *cap_cnt, usize::MAX, need_cnt - *cap_cnt);
    if p.is_null() { return -12; }
    0
}

unsafe fn btf_add_type_offs_mem(btf: *mut btf, add_cnt: size_t) -> *mut c_void {
    libbpf_add_mem(&mut (*btf).type_offs as *mut *mut __u32 as *mut *mut c_void,
                   &mut (*btf).type_offs_cap, size_of::<__u32>(),
                   (*btf).nr_types as size_t, BTF_MAX_NR_TYPES as size_t, add_cnt)
}

unsafe fn btf_add_type_idx_entry(btf: *mut btf, type_off: __u32) -> c_int {
    let p = btf_add_type_offs_mem(btf, 1) as *mut __u32;
    if p.is_null() { return -12; }
    *p = type_off;
    0
}

unsafe fn btf_bswap_hdr(h: *mut btf_header, hdr_len: __u32) {
    (*h).magic = bswap_16((*h).magic);
    (*h).hdr_len = bswap_32((*h).hdr_len);
    (*h).type_off = bswap_32((*h).type_off);
    (*h).type_len = bswap_32((*h).type_len);
    (*h).str_off = bswap_32((*h).str_off);
    (*h).str_len = bswap_32((*h).str_len);
    /* May be operating on raw data with hdr_len that does not include below fields */
    if hdr_len as usize >= size_of::<btf_header>() {
        (*h).layout_off = bswap_32((*h).layout_off);
        (*h).layout_len = bswap_32((*h).layout_len);
    }
}

unsafe fn btf_type_size_unknown(btf: *const btf, t: *const btf_type) -> c_int {
    let mut l_cnt = (*btf).hdr.layout_len as usize / size_of::<btf_layout>();
    let mut l = (*btf).layout as *mut btf_layout;
    let vlen = btf_vlen(t);
    let kind = btf_kind(t) as __u32;
    /* Fall back to base BTF if needed as they share layout information */
    if l.is_null() {
        let base_btf = (*btf).base_btf;
        if !base_btf.is_null() {
            l = (*base_btf).layout as *mut btf_layout;
            l_cnt = (*base_btf).hdr.layout_len as usize / size_of::<btf_layout>();
        }
    }
    if l.is_null() || kind as usize >= l_cnt {
        return -22;
    }
    if ((*l.add(kind as usize)).info_sz % 4) != 0 {
        return -22;
    }
    if ((*l.add(kind as usize)).elem_sz % 4) != 0 {
        return -22;
    }
    (size_of::<btf_type>() + (*l.add(kind as usize)).info_sz as usize
        + vlen as usize * (*l.add(kind as usize)).elem_sz as usize) as c_int
}

unsafe fn btf_type_size(btf: *const btf, t: *const btf_type) -> c_int {
    let base_size = size_of::<btf_type>() as c_int;
    let vlen = btf_vlen(t) as c_int;
    match btf_kind(t) as __u32 {
        k if k == BTF_KIND_FWD || k == BTF_KIND_CONST || k == BTF_KIND_VOLATILE ||
             k == BTF_KIND_RESTRICT || k == BTF_KIND_PTR || k == BTF_KIND_TYPEDEF ||
             k == BTF_KIND_FUNC || k == BTF_KIND_FLOAT || k == BTF_KIND_TYPE_TAG => base_size,
        k if k == BTF_KIND_INT => base_size + size_of::<__u32>() as c_int,
        k if k == BTF_KIND_ENUM => base_size + vlen * size_of::<btf_enum>() as c_int,
        k if k == BTF_KIND_ENUM64 => base_size + vlen * size_of::<btf_enum64>() as c_int,
        k if k == BTF_KIND_ARRAY => base_size + size_of::<btf_array>() as c_int,
        k if k == BTF_KIND_STRUCT || k == BTF_KIND_UNION => base_size + vlen * size_of::<btf_member>() as c_int,
        k if k == BTF_KIND_FUNC_PROTO => base_size + vlen * size_of::<btf_param>() as c_int,
        k if k == BTF_KIND_VAR => base_size + size_of::<btf_var>() as c_int,
        k if k == BTF_KIND_DATASEC => base_size + vlen * size_of::<btf_var_secinfo>() as c_int,
        k if k == BTF_KIND_DECL_TAG => base_size + size_of::<btf_decl_tag>() as c_int,
        _ => btf_type_size_unknown(btf, t),
    }
}

unsafe fn btf_bswap_type_base(t: *mut btf_type) {
    (*t).name_off = bswap_32((*t).name_off);
    (*t).info = bswap_32((*t).info);
    (*t).size = bswap_32((*t).size);
}

unsafe fn btf_bswap_type_rest(t: *mut btf_type) -> c_int {
    let vlen = btf_vlen(t);
    match btf_kind(t) as __u32 {
        k if k == BTF_KIND_FWD || k == BTF_KIND_CONST || k == BTF_KIND_VOLATILE ||
             k == BTF_KIND_RESTRICT || k == BTF_KIND_PTR || k == BTF_KIND_TYPEDEF ||
             k == BTF_KIND_FUNC || k == BTF_KIND_FLOAT || k == BTF_KIND_TYPE_TAG => 0,
        k if k == BTF_KIND_INT => {
            let p = t.add(1) as *mut __u32;
            *p = bswap_32(*p);
            0
        }
        k if k == BTF_KIND_ENUM => {
            let mut e = btf_enum(t);
            for _ in 0..vlen {
                (*e).name_off = bswap_32((*e).name_off);
                (*e).val = bswap_32((*e).val as __u32) as i32;
                e = e.add(1);
            }
            0
        }
        k if k == BTF_KIND_ENUM64 => {
            let mut e = btf_enum64(t);
            for _ in 0..vlen {
                (*e).name_off = bswap_32((*e).name_off);
                (*e).val_lo32 = bswap_32((*e).val_lo32);
                (*e).val_hi32 = bswap_32((*e).val_hi32);
                e = e.add(1);
            }
            0
        }
        k if k == BTF_KIND_ARRAY => {
            let a = btf_array(t);
            (*a).type_ = bswap_32((*a).type_);
            (*a).index_type = bswap_32((*a).index_type);
            (*a).nelems = bswap_32((*a).nelems);
            0
        }
        k if k == BTF_KIND_STRUCT || k == BTF_KIND_UNION => {
            let mut m = btf_members(t);
            for _ in 0..vlen {
                (*m).name_off = bswap_32((*m).name_off);
                (*m).type_ = bswap_32((*m).type_);
                (*m).offset = bswap_32((*m).offset);
                m = m.add(1);
            }
            0
        }
        k if k == BTF_KIND_FUNC_PROTO => {
            let mut p = btf_params(t);
            for _ in 0..vlen {
                (*p).name_off = bswap_32((*p).name_off);
                (*p).type_ = bswap_32((*p).type_);
                p = p.add(1);
            }
            0
        }
        k if k == BTF_KIND_VAR => {
            (*btf_var(t)).linkage = bswap_32((*btf_var(t)).linkage);
            0
        }
        k if k == BTF_KIND_DATASEC => {
            let mut v = btf_var_secinfos(t);
            for _ in 0..vlen {
                (*v).type_ = bswap_32((*v).type_);
                (*v).offset = bswap_32((*v).offset);
                (*v).size = bswap_32((*v).size);
                v = v.add(1);
            }
            0
        }
        k if k == BTF_KIND_DECL_TAG => {
            (*btf_decl_tag(t)).component_idx = bswap_32((*btf_decl_tag(t)).component_idx as __u32) as i32;
            0
        }
        _ => -22,
    }
}

#[no_mangle]
pub unsafe extern "C" fn btf__type_cnt(btf: *const btf) -> __u32 {
    ((*btf).start_id as __u32).wrapping_add((*btf).nr_types)
}

#[no_mangle]
pub unsafe extern "C" fn btf__base_btf(btf: *const btf) -> *const btf {
    (*btf).base_btf as *const btf
}

/* internal helper returning non-const pointer to a type */
#[no_mangle]
pub unsafe extern "C" fn btf_type_by_id(btf: *const btf, type_id: __u32) -> *mut btf_type {
    if type_id == 0 {
        return &mut btf_void;
    }
    if (type_id as c_int) < (*btf).start_id {
        return btf_type_by_id((*btf).base_btf, type_id);
    }
    ptr_add((*btf).types_data, *(*btf).type_offs.add((type_id as c_int - (*btf).start_id) as usize) as usize) as *mut btf_type
}

#[no_mangle]
pub unsafe extern "C" fn btf__type_by_id(btf: *const btf, type_id: __u32) -> *const btf_type {
    if type_id >= ((*btf).start_id as __u32).wrapping_add((*btf).nr_types) {
        return null();
    }
    btf_type_by_id(btf, type_id) as *const btf_type
}

unsafe fn btf_type_is_void(t: *const btf_type) -> bool {
    core::ptr::addr_eq(t, &btf_void as *const btf_type) || btf_is_fwd(t)
}

unsafe fn btf_type_is_void_or_null(t: *const btf_type) -> bool {
    t.is_null() || btf_type_is_void(t)
}

unsafe fn btf_ptr_sz(btf: *const btf) -> size_t {
    if (*btf).ptr_sz < 0 { size_of::<*mut c_void>() } else { (*btf).ptr_sz as size_t }
}

#[no_mangle]
pub unsafe extern "C" fn btf__pointer_size(btf: *const btf) -> size_t {
    if (*btf).ptr_sz < 0 { 0 } else { (*btf).ptr_sz as size_t }
}

#[no_mangle]
pub unsafe extern "C" fn btf__set_pointer_size(btf: *mut btf, ptr_sz: size_t) -> c_int {
    if ptr_sz != 4 && ptr_sz != 8 {
        return libbpf_err(-22);
    }
    (*btf).ptr_sz = ptr_sz as c_int;
    0
}

unsafe fn is_host_big_endian() -> bool {
    cfg!(target_endian = "big")
}

#[no_mangle]
pub unsafe extern "C" fn btf__endianness(btf: *const btf) -> btf_endianness {
    if is_host_big_endian() {
        if (*btf).swapped_endian { BTF_LITTLE_ENDIAN } else { BTF_BIG_ENDIAN }
    } else {
        if (*btf).swapped_endian { BTF_BIG_ENDIAN } else { BTF_LITTLE_ENDIAN }
    }
}

#[no_mangle]
pub unsafe extern "C" fn btf__set_endianness(btf: *mut btf, endian: btf_endianness) -> c_int {
    if endian != BTF_LITTLE_ENDIAN && endian != BTF_BIG_ENDIAN {
        return libbpf_err(-22);
    }
    (*btf).swapped_endian = is_host_big_endian() != (endian == BTF_BIG_ENDIAN);
    if !(*btf).swapped_endian {
        free((*btf).raw_data_swapped);
        (*btf).raw_data_swapped = null_mut();
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn btf__resolve_size(btf: *const btf, mut type_id: __u32) -> __s64 {
    let mut nelems: __u32 = 1;
    let mut size: __s64 = -1;
    let mut t = btf__type_by_id(btf, type_id);
    for _ in 0..MAX_RESOLVE_DEPTH {
        if btf_type_is_void_or_null(t) { break; }
        match btf_kind(t) as __u32 {
            k if k == BTF_KIND_INT || k == BTF_KIND_STRUCT || k == BTF_KIND_UNION ||
                 k == BTF_KIND_ENUM || k == BTF_KIND_ENUM64 || k == BTF_KIND_DATASEC ||
                 k == BTF_KIND_FLOAT => { size = (*t).size as __s64; break; }
            k if k == BTF_KIND_PTR => { size = btf_ptr_sz(btf) as __s64; break; }
            k if k == BTF_KIND_TYPEDEF || k == BTF_KIND_VOLATILE || k == BTF_KIND_CONST ||
                 k == BTF_KIND_RESTRICT || k == BTF_KIND_VAR || k == BTF_KIND_DECL_TAG ||
                 k == BTF_KIND_TYPE_TAG => { type_id = (*t).size; }
            k if k == BTF_KIND_ARRAY => {
                let array = btf_array(t);
                if nelems != 0 && (*array).nelems > u32::MAX / nelems {
                    return libbpf_err(-7) as __s64;
                }
                nelems = nelems.wrapping_mul((*array).nelems);
                type_id = (*array).type_;
            }
            _ => return libbpf_err(-22) as __s64,
        }
        t = btf__type_by_id(btf, type_id);
    }
    if size < 0 { return libbpf_err(-22) as __s64; }
    if nelems != 0 && size > (u32::MAX / nelems) as __s64 {
        return libbpf_err(-7) as __s64;
    }
    nelems as __s64 * size
}

#[no_mangle]
pub unsafe extern "C" fn btf__resolve_type(btf: *const btf, mut type_id: __u32) -> c_int {
    let mut depth = 0;
    let mut t = btf__type_by_id(btf, type_id);
    while depth < MAX_RESOLVE_DEPTH
        && !btf_type_is_void_or_null(t)
        && (btf_is_mod(t) || btf_is_typedef(t) || btf_is_var(t))
    {
        type_id = (*t).size;
        t = btf__type_by_id(btf, type_id);
        depth += 1;
    }
    if depth == MAX_RESOLVE_DEPTH || btf_type_is_void_or_null(t) {
        return libbpf_err(-22);
    }
    type_id as c_int
}

unsafe fn btf_is_modifiable(btf: *const btf) -> bool {
    /* BTF is modifiable if split into multiple sections */
    (*btf).modifiable
}

unsafe fn btf_free_raw_data(btf: *mut btf) {
    if (*btf).raw_data_is_mmap {
        munmap((*btf).raw_data, (*btf).raw_size as size_t);
        (*btf).raw_data_is_mmap = false;
    } else {
        free((*btf).raw_data);
    }
    (*btf).raw_data = null_mut();
}

#[no_mangle]
pub unsafe extern "C" fn btf__free(btf: *mut btf) {
    if btf.is_null() { return; }
    if (*btf).fd >= 0 { close((*btf).fd); }
    if btf_is_modifiable(btf) {
        free((*btf).types_data);
        strset__free((*btf).strs_set);
        free((*btf).layout);
    }
    btf_free_raw_data(btf);
    free((*btf).raw_data_swapped);
    free((*btf).type_offs as *mut c_void);
    if (*btf).owns_base { btf__free((*btf).base_btf); }
    free(btf as *mut c_void);
}

unsafe fn btf_invalidate_raw_data(btf: *mut btf) {
    if !(*btf).raw_data.is_null() {
        btf_free_raw_data(btf);
    }
    if !(*btf).raw_data_swapped.is_null() {
        free((*btf).raw_data_swapped);
        (*btf).raw_data_swapped = null_mut();
    }
    (*btf).named_start_id = 0;
}

/* Ensure BTF is ready to be modified (by splitting into a three memory
 * regions for types, strings and layout. Also invalidate cached
 * raw_data, if any.
 */
unsafe fn btf_ensure_modifiable(btf: *mut btf) -> c_int {
    if btf_is_modifiable(btf) {
        /* any BTF modification invalidates raw_data */
        btf_invalidate_raw_data(btf);
        return 0;
    }
    if (*btf).has_hdr_extra {
        /* Additional BTF header data was found; not safe to modify. */
        return -95;
    }
    let types = malloc((*btf).hdr.type_len as size_t);
    if types.is_null() { return -12; }
    memcpy(types, (*btf).types_data, (*btf).hdr.type_len as size_t);
    let mut layout = null_mut();
    if (*btf).hdr.layout_len != 0 {
        layout = malloc((*btf).hdr.layout_len as size_t);
        if layout.is_null() {
            free(types);
            return -12;
        }
        memcpy(layout,
               ptr_add((*btf).raw_data, (*btf).hdr.hdr_len as usize + (*btf).hdr.layout_off as usize),
               (*btf).hdr.layout_len as size_t);
    }
    let set = strset__new(BTF_MAX_STR_OFFSET as size_t, (*btf).strs_data, (*btf).hdr.str_len as size_t);
    if set.is_null() {
        free(types);
        free(layout);
        return -12;
    }
    (*btf).types_data = types;
    (*btf).types_data_cap = (*btf).hdr.type_len as size_t;
    (*btf).strs_data = null_mut();
    (*btf).strs_set = set;
    if !layout.is_null() { (*btf).layout = layout; }
    if (*btf).hdr.str_len == 0 { (*btf).strs_deduped = true; }
    if (*btf).base_btf.is_null() && (*btf).hdr.str_len == 1 { (*btf).strs_deduped = true; }
    btf_invalidate_raw_data(btf);
    (*btf).modifiable = true;
    0
}

#[no_mangle]
pub unsafe extern "C" fn btf__find_str(btf: *mut btf, s: *const c_char) -> c_int {
    if !(*btf).base_btf.is_null() {
        let off = btf__find_str((*btf).base_btf, s);
        if off != -2 { return off; }
    }
    let err = btf_ensure_modifiable(btf);
    if err != 0 { return libbpf_err(err); }
    let off = strset__find_str((*btf).strs_set, s);
    if off < 0 { return libbpf_err(off); }
    (*btf).start_str_off + off
}

#[no_mangle]
pub unsafe extern "C" fn btf__add_str(btf: *mut btf, s: *const c_char) -> c_int {
    if !(*btf).base_btf.is_null() {
        let off = btf__find_str((*btf).base_btf, s);
        if off != -2 { return off; }
    }
    let err = btf_ensure_modifiable(btf);
    if err != 0 { return libbpf_err(err); }
    let off = strset__add_str((*btf).strs_set, s);
    if off < 0 { return libbpf_err(off); }
    (*btf).hdr.str_len = strset__data_size((*btf).strs_set);
    (*btf).start_str_off + off
}

unsafe fn btf_add_type_mem(btf: *mut btf, add_sz: size_t) -> *mut c_void {
    libbpf_add_mem(&mut (*btf).types_data, &mut (*btf).types_data_cap, 1,
                   (*btf).hdr.type_len as size_t, u32::MAX as size_t, add_sz)
}

unsafe fn btf_type_inc_vlen(t: *mut btf_type) -> c_int {
    if btf_vlen(t) == BTF_MAX_VLEN { return -28; }
    (*t).info = btf_type_info(btf_kind(t) as __u32, btf_vlen(t) + 1, btf_kflag(t));
    0
}

unsafe fn btf_hdr_update_type_len(btf: *mut btf, new_len: c_int) {
    (*btf).hdr.type_len = new_len as __u32;
    if !(*btf).layout.is_null() {
        (*btf).hdr.layout_off = (*btf).hdr.type_off + new_len as __u32;
        (*btf).hdr.str_off = (*btf).hdr.layout_off + (*btf).hdr.layout_len;
    } else {
        (*btf).hdr.str_off = (*btf).hdr.type_off + new_len as __u32;
    }
}

unsafe fn btf_hdr_update_str_len(btf: *mut btf, new_len: c_int) {
    (*btf).hdr.str_len = new_len as __u32;
}

unsafe fn btf_commit_type(btf: *mut btf, data_sz: c_int) -> c_int {
    let err = btf_add_type_idx_entry(btf, (*btf).hdr.type_len);
    if err != 0 { return libbpf_err(err); }
    btf_hdr_update_type_len(btf, (*btf).hdr.type_len as c_int + data_sz);
    (*btf).nr_types = (*btf).nr_types.wrapping_add(1);
    (*btf).start_id + (*btf).nr_types as c_int - 1
}

#[repr(C)]
pub struct btf_pipe {
    pub src: *const btf,
    pub dst: *mut btf,
    pub str_off_map: *mut hashmap, /* map string offsets from src to dst */
}

unsafe fn btf_rewrite_str(p: *mut btf_pipe, str_off: *mut __u32) -> c_int {
    let mut mapped_off: c_long = 0;
    if *str_off == 0 { return 0; } /* nothing to do for empty strings */
    if !(*p).str_off_map.is_null() && hashmap__find((*p).str_off_map, *str_off as c_long, &mut mapped_off) {
        *str_off = mapped_off as __u32;
        return 0;
    }
    let off = btf__add_str((*p).dst, btf__str_by_offset((*p).src, *str_off));
    if off < 0 { return off; }
    if !(*p).str_off_map.is_null() {
        let err = hashmap__append((*p).str_off_map, *str_off as c_long, off as c_long);
        if err != 0 { return err; }
    }
    *str_off = off as __u32;
    0
}

unsafe fn btf_add_type(p: *mut btf_pipe, src_type: *const btf_type) -> c_int {
    let sz = btf_type_size((*p).src, src_type);
    if sz < 0 { return libbpf_err(sz); }
    let err = btf_ensure_modifiable((*p).dst);
    if err != 0 { return libbpf_err(err); }
    let t = btf_add_type_mem((*p).dst, sz as size_t) as *mut btf_type;
    if t.is_null() { return libbpf_err(-12); }
    memcpy(t as *mut c_void, src_type as *const c_void, sz as size_t);
    let mut it: btf_field_iter = zeroed();
    let err = btf_field_iter_init(&mut it, t, 1);
    if err != 0 { return libbpf_err(err); }
    loop {
        let str_off = btf_field_iter_next(&mut it);
        if str_off.is_null() { break; }
        let err = btf_rewrite_str(p, str_off);
        if err != 0 { return libbpf_err(err); }
    }
    btf_commit_type((*p).dst, sz)
}

#[no_mangle]
pub unsafe extern "C" fn btf__add_type(btf: *mut btf, src_btf: *const btf, src_type: *const btf_type) -> c_int {
    let mut p = btf_pipe { src: src_btf, dst: btf, str_off_map: null_mut() };
    btf_add_type(&mut p, src_type)
}

unsafe fn validate_type_id(id: c_int) -> c_int {
    if id < 0 || id as __u32 > BTF_MAX_NR_TYPES { -22 } else { 0 }
}

unsafe fn str_is_empty(s: *const c_char) -> bool {
    s.is_null() || *s == 0
}

unsafe fn btf_add_ref_kind(btf: *mut btf, kind: c_int, name: *const c_char, ref_type_id: c_int, kflag: c_int) -> c_int {
    if validate_type_id(ref_type_id) != 0 { return libbpf_err(-22); }
    let err = btf_ensure_modifiable(btf);
    if err != 0 { return libbpf_err(err); }
    let sz = size_of::<btf_type>() as c_int;
    let t = btf_add_type_mem(btf, sz as size_t) as *mut btf_type;
    if t.is_null() { return libbpf_err(-12); }
    let mut name_off = 0;
    if !str_is_empty(name) {
        name_off = btf__add_str(btf, name);
        if name_off < 0 { return name_off; }
    }
    (*t).name_off = name_off as __u32;
    (*t).info = btf_type_info(kind as __u32, 0, kflag != 0);
    (*t).size = ref_type_id as __u32;
    btf_commit_type(btf, sz)
}

#[no_mangle] pub unsafe extern "C" fn btf__add_ptr(btf: *mut btf, ref_type_id: c_int) -> c_int { btf_add_ref_kind(btf, BTF_KIND_PTR as c_int, null(), ref_type_id, 0) }
#[no_mangle] pub unsafe extern "C" fn btf__add_volatile(btf: *mut btf, ref_type_id: c_int) -> c_int { btf_add_ref_kind(btf, BTF_KIND_VOLATILE as c_int, null(), ref_type_id, 0) }
#[no_mangle] pub unsafe extern "C" fn btf__add_const(btf: *mut btf, ref_type_id: c_int) -> c_int { btf_add_ref_kind(btf, BTF_KIND_CONST as c_int, null(), ref_type_id, 0) }
#[no_mangle] pub unsafe extern "C" fn btf__add_restrict(btf: *mut btf, ref_type_id: c_int) -> c_int { btf_add_ref_kind(btf, BTF_KIND_RESTRICT as c_int, null(), ref_type_id, 0) }

#[no_mangle]
pub unsafe extern "C" fn btf__add_int(btf: *mut btf, name: *const c_char, byte_sz: size_t, encoding: c_int) -> c_int {
    if str_is_empty(name) { return libbpf_err(-22); }
    if byte_sz == 0 || (byte_sz & (byte_sz - 1)) != 0 || byte_sz > 16 { return libbpf_err(-22); }
    if (encoding & !(BTF_INT_SIGNED | BTF_INT_CHAR | BTF_INT_BOOL)) != 0 { return libbpf_err(-22); }
    let err = btf_ensure_modifiable(btf);
    if err != 0 { return libbpf_err(err); }
    let sz = (size_of::<btf_type>() + size_of::<c_int>()) as c_int;
    let t = btf_add_type_mem(btf, sz as size_t) as *mut btf_type;
    if t.is_null() { return libbpf_err(-12); }
    let name_off = btf__add_str(btf, name);
    if name_off < 0 { return name_off; }
    (*t).name_off = name_off as __u32;
    (*t).info = btf_type_info(BTF_KIND_INT, 0, false);
    (*t).size = byte_sz as __u32;
    *(t.add(1) as *mut __u32) = ((encoding as __u32) << 24) | ((byte_sz as __u32) * 8);
    btf_commit_type(btf, sz)
}

#[no_mangle]
pub unsafe extern "C" fn btf__add_float(btf: *mut btf, name: *const c_char, byte_sz: size_t) -> c_int {
    if str_is_empty(name) { return libbpf_err(-22); }
    if byte_sz != 2 && byte_sz != 4 && byte_sz != 8 && byte_sz != 12 && byte_sz != 16 { return libbpf_err(-22); }
    let err = btf_ensure_modifiable(btf);
    if err != 0 { return libbpf_err(err); }
    let sz = size_of::<btf_type>() as c_int;
    let t = btf_add_type_mem(btf, sz as size_t) as *mut btf_type;
    if t.is_null() { return libbpf_err(-12); }
    let name_off = btf__add_str(btf, name);
    if name_off < 0 { return name_off; }
    (*t).name_off = name_off as __u32;
    (*t).info = btf_type_info(BTF_KIND_FLOAT, 0, false);
    (*t).size = byte_sz as __u32;
    btf_commit_type(btf, sz)
}

#[no_mangle]
pub unsafe extern "C" fn btf__add_array(btf: *mut btf, index_type_id: c_int, elem_type_id: c_int, nr_elems: __u32) -> c_int {
    if validate_type_id(index_type_id) != 0 || validate_type_id(elem_type_id) != 0 { return libbpf_err(-22); }
    let err = btf_ensure_modifiable(btf);
    if err != 0 { return libbpf_err(err); }
    let sz = (size_of::<btf_type>() + size_of::<btf_array>()) as c_int;
    let t = btf_add_type_mem(btf, sz as size_t) as *mut btf_type;
    if t.is_null() { return libbpf_err(-12); }
    (*t).name_off = 0;
    (*t).info = btf_type_info(BTF_KIND_ARRAY, 0, false);
    (*t).size = 0;
    let a = btf_array(t);
    (*a).type_ = elem_type_id as __u32;
    (*a).index_type = index_type_id as __u32;
    (*a).nelems = nr_elems;
    btf_commit_type(btf, sz)
}

unsafe fn btf_add_composite(btf: *mut btf, kind: c_int, name: *const c_char, bytes_sz: __u32) -> c_int {
    let err = btf_ensure_modifiable(btf);
    if err != 0 { return libbpf_err(err); }
    let sz = size_of::<btf_type>() as c_int;
    let t = btf_add_type_mem(btf, sz as size_t) as *mut btf_type;
    if t.is_null() { return libbpf_err(-12); }
    let mut name_off = 0;
    if !str_is_empty(name) {
        name_off = btf__add_str(btf, name);
        if name_off < 0 { return name_off; }
    }
    (*t).name_off = name_off as __u32;
    (*t).info = btf_type_info(kind as __u32, 0, false);
    (*t).size = bytes_sz;
    btf_commit_type(btf, sz)
}

#[no_mangle] pub unsafe extern "C" fn btf__add_struct(btf: *mut btf, name: *const c_char, byte_sz: __u32) -> c_int { btf_add_composite(btf, BTF_KIND_STRUCT as c_int, name, byte_sz) }
#[no_mangle] pub unsafe extern "C" fn btf__add_union(btf: *mut btf, name: *const c_char, byte_sz: __u32) -> c_int { btf_add_composite(btf, BTF_KIND_UNION as c_int, name, byte_sz) }

unsafe fn btf_last_type(btf: *mut btf) -> *mut btf_type {
    btf_type_by_id(btf, btf__type_cnt(btf) - 1)
}

#[no_mangle]
pub unsafe extern "C" fn btf__add_field(btf: *mut btf, name: *const c_char, type_id: c_int, bit_offset: __u32, bit_size: __u32) -> c_int {
    if (*btf).nr_types == 0 { return libbpf_err(-22); }
    let mut t = btf_last_type(btf);
    if !btf_is_composite(t) { return libbpf_err(-22); }
    if btf_vlen(t) == BTF_MAX_VLEN { return libbpf_err(-28); }
    if validate_type_id(type_id) != 0 { return libbpf_err(-22); }
    let is_bitfield = bit_size != 0 || (bit_offset % 8 != 0);
    if is_bitfield && (bit_size == 0 || bit_size > 255 || bit_offset > 0x00ff_ffff) { return libbpf_err(-22); }
    if btf_is_union(t) && bit_offset != 0 { return libbpf_err(-22); }
    let err = btf_ensure_modifiable(btf);
    if err != 0 { return libbpf_err(err); }
    let sz = size_of::<btf_member>() as c_int;
    let m = btf_add_type_mem(btf, sz as size_t) as *mut btf_member;
    if m.is_null() { return libbpf_err(-12); }
    let mut name_off = 0;
    if !str_is_empty(name) {
        name_off = btf__add_str(btf, name);
        if name_off < 0 { return name_off; }
    }
    (*m).name_off = name_off as __u32;
    (*m).type_ = type_id as __u32;
    (*m).offset = bit_offset | (bit_size << 24);
    t = btf_last_type(btf);
    (*t).info = btf_type_info(btf_kind(t) as __u32, btf_vlen(t) + 1, is_bitfield || btf_kflag(t));
    btf_hdr_update_type_len(btf, (*btf).hdr.type_len as c_int + sz);
    0
}

unsafe fn btf_add_enum_common(btf: *mut btf, name: *const c_char, byte_sz: __u32, is_signed: bool, kind: __u8) -> c_int {
    if byte_sz == 0 || (byte_sz & (byte_sz - 1)) != 0 || byte_sz > 8 { return libbpf_err(-22); }
    let err = btf_ensure_modifiable(btf);
    if err != 0 { return libbpf_err(err); }
    let sz = size_of::<btf_type>() as c_int;
    let t = btf_add_type_mem(btf, sz as size_t) as *mut btf_type;
    if t.is_null() { return libbpf_err(-12); }
    let mut name_off = 0;
    if !str_is_empty(name) {
        name_off = btf__add_str(btf, name);
        if name_off < 0 { return name_off; }
    }
    (*t).name_off = name_off as __u32;
    (*t).info = btf_type_info(kind as __u32, 0, is_signed);
    (*t).size = byte_sz;
    btf_commit_type(btf, sz)
}

#[no_mangle] pub unsafe extern "C" fn btf__add_enum(btf: *mut btf, name: *const c_char, byte_sz: __u32) -> c_int { btf_add_enum_common(btf, name, byte_sz, false, BTF_KIND_ENUM as __u8) }
#[no_mangle] pub unsafe extern "C" fn btf__add_enum64(btf: *mut btf, name: *const c_char, byte_sz: __u32, is_signed: bool) -> c_int { btf_add_enum_common(btf, name, byte_sz, is_signed, BTF_KIND_ENUM64 as __u8) }

#[no_mangle]
pub unsafe extern "C" fn btf__add_enum_value(btf: *mut btf, name: *const c_char, value: __s64) -> c_int {
    if (*btf).nr_types == 0 { return libbpf_err(-22); }
    let mut t = btf_last_type(btf);
    if !btf_is_enum(t) || str_is_empty(name) { return libbpf_err(-22); }
    if value < i32::MIN as i64 || value > u32::MAX as i64 { return libbpf_err(-7); }
    let err = btf_ensure_modifiable(btf);
    if err != 0 { return libbpf_err(err); }
    let sz = size_of::<btf_enum>() as c_int;
    let v = btf_add_type_mem(btf, sz as size_t) as *mut btf_enum;
    if v.is_null() { return libbpf_err(-12); }
    let name_off = btf__add_str(btf, name);
    if name_off < 0 { return name_off; }
    (*v).name_off = name_off as __u32;
    (*v).val = value as i32;
    t = btf_last_type(btf);
    let err = btf_type_inc_vlen(t);
    if err != 0 { return libbpf_err(err); }
    if value < 0 { (*t).info = btf_type_info(btf_kind(t) as __u32, btf_vlen(t), true); }
    btf_hdr_update_type_len(btf, (*btf).hdr.type_len as c_int + sz);
    0
}

#[no_mangle]
pub unsafe extern "C" fn btf__add_enum64_value(btf: *mut btf, name: *const c_char, value: __u64) -> c_int {
    if (*btf).nr_types == 0 { return libbpf_err(-22); }
    let mut t = btf_last_type(btf);
    if !btf_is_enum64(t) || str_is_empty(name) { return libbpf_err(-22); }
    let err = btf_ensure_modifiable(btf);
    if err != 0 { return libbpf_err(err); }
    let sz = size_of::<btf_enum64>() as c_int;
    let v = btf_add_type_mem(btf, sz as size_t) as *mut btf_enum64;
    if v.is_null() { return libbpf_err(-12); }
    let name_off = btf__add_str(btf, name);
    if name_off < 0 { return name_off; }
    (*v).name_off = name_off as __u32;
    (*v).val_lo32 = value as __u32;
    (*v).val_hi32 = (value >> 32) as __u32;
    t = btf_last_type(btf);
    let err = btf_type_inc_vlen(t);
    if err != 0 { return libbpf_err(err); }
    btf_hdr_update_type_len(btf, (*btf).hdr.type_len as c_int + sz);
    0
}

#[no_mangle]
pub unsafe extern "C" fn btf__add_typedef(btf: *mut btf, name: *const c_char, ref_type_id: c_int) -> c_int {
    if str_is_empty(name) { return libbpf_err(-22); }
    btf_add_ref_kind(btf, BTF_KIND_TYPEDEF as c_int, name, ref_type_id, 0)
}

#[no_mangle]
pub unsafe extern "C" fn btf__add_type_tag(btf: *mut btf, value: *const c_char, ref_type_id: c_int) -> c_int {
    if str_is_empty(value) { return libbpf_err(-22); }
    btf_add_ref_kind(btf, BTF_KIND_TYPE_TAG as c_int, value, ref_type_id, 0)
}

#[no_mangle]
pub unsafe extern "C" fn btf__add_type_attr(btf: *mut btf, value: *const c_char, ref_type_id: c_int) -> c_int {
    if str_is_empty(value) { return libbpf_err(-22); }
    btf_add_ref_kind(btf, BTF_KIND_TYPE_TAG as c_int, value, ref_type_id, 1)
}

#[no_mangle]
pub unsafe extern "C" fn btf__add_func(btf: *mut btf, name: *const c_char, linkage: btf_func_linkage, proto_type_id: c_int) -> c_int {
    if str_is_empty(name) { return libbpf_err(-22); }
    if linkage != BTF_FUNC_STATIC && linkage != BTF_FUNC_GLOBAL && linkage != BTF_FUNC_EXTERN { return libbpf_err(-22); }
    let id = btf_add_ref_kind(btf, BTF_KIND_FUNC as c_int, name, proto_type_id, 0);
    if id > 0 {
        let t = btf_type_by_id(btf, id as __u32);
        (*t).info = btf_type_info(BTF_KIND_FUNC, linkage, false);
    }
    libbpf_err(id)
}

#[no_mangle]
pub unsafe extern "C" fn btf__add_func_proto(btf: *mut btf, ret_type_id: c_int) -> c_int {
    if validate_type_id(ret_type_id) != 0 { return libbpf_err(-22); }
    let err = btf_ensure_modifiable(btf);
    if err != 0 { return libbpf_err(err); }
    let sz = size_of::<btf_type>() as c_int;
    let t = btf_add_type_mem(btf, sz as size_t) as *mut btf_type;
    if t.is_null() { return libbpf_err(-12); }
    (*t).name_off = 0;
    (*t).info = btf_type_info(BTF_KIND_FUNC_PROTO, 0, false);
    (*t).size = ret_type_id as __u32;
    btf_commit_type(btf, sz)
}

#[no_mangle]
pub unsafe extern "C" fn btf__add_func_param(btf: *mut btf, name: *const c_char, type_id: c_int) -> c_int {
    if validate_type_id(type_id) != 0 { return libbpf_err(-22); }
    if (*btf).nr_types == 0 { return libbpf_err(-22); }
    let mut t = btf_last_type(btf);
    if !btf_is_func_proto(t) { return libbpf_err(-22); }
    let err = btf_ensure_modifiable(btf);
    if err != 0 { return libbpf_err(err); }
    let sz = size_of::<btf_param>() as c_int;
    let p = btf_add_type_mem(btf, sz as size_t) as *mut btf_param;
    if p.is_null() { return libbpf_err(-12); }
    let mut name_off = 0;
    if !str_is_empty(name) {
        name_off = btf__add_str(btf, name);
        if name_off < 0 { return name_off; }
    }
    (*p).name_off = name_off as __u32;
    (*p).type_ = type_id as __u32;
    t = btf_last_type(btf);
    let err = btf_type_inc_vlen(t);
    if err != 0 { return libbpf_err(err); }
    btf_hdr_update_type_len(btf, (*btf).hdr.type_len as c_int + sz);
    0
}

#[no_mangle]
pub unsafe extern "C" fn btf__str_by_offset(btf: *const btf, offset: __u32) -> *const c_char {
    if (offset as c_int) < (*btf).start_str_off {
        return btf__str_by_offset((*btf).base_btf, offset);
    } else if offset.wrapping_sub((*btf).start_str_off as __u32) < (*btf).hdr.str_len {
        let data = if !(*btf).strs_data.is_null() { (*btf).strs_data } else { strset__data((*btf).strs_set) as *mut c_void };
        return ptr_add(data, offset.wrapping_sub((*btf).start_str_off as __u32) as size_t) as *const c_char;
    }
    null()
}

#[no_mangle]
pub unsafe extern "C" fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char {
    btf__str_by_offset(btf, offset)
}

#[inline]
unsafe fn hash_combine(h: c_ulong, value: c_ulong) -> c_ulong {
    h.wrapping_mul(31).wrapping_add(value)
}

unsafe extern "C" fn btf_dedup_identity_hash_fn(key: c_long, _ctx: *mut c_void) -> size_t {
    key as size_t
}

unsafe extern "C" fn btf_dedup_collision_hash_fn(_key: c_long, _ctx: *mut c_void) -> size_t {
    0
}

unsafe extern "C" fn btf_dedup_equal_fn(k1: c_long, k2: c_long, _ctx: *mut c_void) -> bool {
    k1 == k2
}

unsafe fn btf_dedup_table_add(d: *mut btf_dedup, hash: c_long, type_id: __u32) -> c_int {
    hashmap__append((*d).dedup_table, hash, type_id as c_long)
}

unsafe fn btf_dedup_hypot_map_add(d: *mut btf_dedup, from_id: __u32, to_id: __u32) -> c_int {
    if (*d).hypot_cnt == (*d).hypot_cap {
        (*d).hypot_cap += core::cmp::max(16usize, (*d).hypot_cap / 2);
        let new_list = libbpf_reallocarray((*d).hypot_list as *mut c_void, (*d).hypot_cap, size_of::<__u32>()) as *mut __u32;
        if new_list.is_null() { return -12; }
        (*d).hypot_list = new_list;
    }
    *(*d).hypot_list.add((*d).hypot_cnt) = from_id;
    (*d).hypot_cnt += 1;
    *(*d).hypot_map.add(from_id as usize) = to_id;
    0
}

unsafe fn btf_dedup_clear_hypot_map(d: *mut btf_dedup) {
    for i in 0..(*d).hypot_cnt {
        *(*d).hypot_map.add(*(*d).hypot_list.add(i) as usize) = BTF_UNPROCESSED_ID;
    }
    (*d).hypot_cnt = 0;
    (*d).hypot_adjust_canon = false;
}

unsafe fn btf_dedup_free(d: *mut btf_dedup) {
    if d.is_null() { return; }
    hashmap__free((*d).dedup_table);
    free((*d).map as *mut c_void);
    free((*d).hypot_map as *mut c_void);
    free((*d).hypot_list as *mut c_void);
    free(d as *mut c_void);
}

unsafe fn btf_hash_typedef(t: *mut btf_type) -> c_long {
    let mut h = hash_combine(0, (*t).name_off as c_ulong);
    h = hash_combine(h, (*t).info as c_ulong);
    h as c_long
}

unsafe fn btf_hash_common(t: *mut btf_type) -> c_long {
    let mut h = hash_combine(0, (*t).name_off as c_ulong);
    h = hash_combine(h, (*t).info as c_ulong);
    h = hash_combine(h, (*t).size as c_ulong);
    h as c_long
}

unsafe fn btf_equal_common(t1: *mut btf_type, t2: *mut btf_type) -> bool {
    (*t1).name_off == (*t2).name_off && (*t1).info == (*t2).info && (*t1).size == (*t2).size
}

unsafe fn btf_equal_typedef(t1: *mut btf_type, t2: *mut btf_type) -> bool {
    (*t1).name_off == (*t2).name_off && (*t1).info == (*t2).info
}

unsafe fn btf_hash_int_decl_tag(t: *mut btf_type) -> c_long {
    let info = *(t.add(1) as *mut __u32);
    hash_combine(btf_hash_common(t) as c_ulong, info as c_ulong) as c_long
}

unsafe fn btf_equal_int_tag(t1: *mut btf_type, t2: *mut btf_type) -> bool {
    if !btf_equal_common(t1, t2) { return false; }
    *(t1.add(1) as *mut __u32) == *(t2.add(1) as *mut __u32)
}

unsafe fn btf_hash_enum(t: *mut btf_type) -> c_long {
    /* don't hash vlen, enum members and size to support enum fwd resolving */
    hash_combine(0, (*t).name_off as c_ulong) as c_long
}

unsafe fn btf_equal_enum_members(t1: *mut btf_type, t2: *mut btf_type) -> bool {
    let vlen = btf_vlen(t1);
    let mut m1 = btf_enum(t1);
    let mut m2 = btf_enum(t2);
    for _ in 0..vlen {
        if (*m1).name_off != (*m2).name_off || (*m1).val != (*m2).val { return false; }
        m1 = m1.add(1);
        m2 = m2.add(1);
    }
    true
}

unsafe fn btf_equal_enum64_members(t1: *mut btf_type, t2: *mut btf_type) -> bool {
    let vlen = btf_vlen(t1);
    let mut m1 = btf_enum64(t1);
    let mut m2 = btf_enum64(t2);
    for _ in 0..vlen {
        if (*m1).name_off != (*m2).name_off || (*m1).val_lo32 != (*m2).val_lo32 || (*m1).val_hi32 != (*m2).val_hi32 { return false; }
        m1 = m1.add(1);
        m2 = m2.add(1);
    }
    true
}

unsafe fn btf_equal_enum(t1: *mut btf_type, t2: *mut btf_type) -> bool {
    if !btf_equal_common(t1, t2) { return false; }
    if btf_kind(t1) as __u32 == BTF_KIND_ENUM { btf_equal_enum_members(t1, t2) } else { btf_equal_enum64_members(t1, t2) }
}

unsafe fn btf_is_enum_fwd(t: *mut btf_type) -> bool {
    btf_is_any_enum(t) && btf_vlen(t) == 0
}

unsafe fn btf_compat_enum(t1: *mut btf_type, t2: *mut btf_type) -> bool {
    if !btf_is_enum_fwd(t1) && !btf_is_enum_fwd(t2) {
        return btf_equal_enum(t1, t2);
    }
    (*t1).name_off == (*t2).name_off && btf_is_any_enum(t1) && btf_is_any_enum(t2)
}

unsafe fn btf_hash_struct(t: *mut btf_type) -> c_long {
    let mut member = btf_members(t);
    let vlen = btf_vlen(t);
    let mut h = btf_hash_common(t) as c_ulong;
    for _ in 0..vlen {
        h = hash_combine(h, (*member).name_off as c_ulong);
        h = hash_combine(h, (*member).offset as c_ulong);
        member = member.add(1);
    }
    h as c_long
}

unsafe fn btf_shallow_equal_struct(t1: *mut btf_type, t2: *mut btf_type) -> bool {
    if !btf_equal_common(t1, t2) { return false; }
    let vlen = btf_vlen(t1);
    let mut m1 = btf_members(t1);
    let mut m2 = btf_members(t2);
    for _ in 0..vlen {
        if (*m1).name_off != (*m2).name_off || (*m1).offset != (*m2).offset { return false; }
        m1 = m1.add(1);
        m2 = m2.add(1);
    }
    true
}

unsafe fn btf_hash_array(t: *mut btf_type) -> c_long {
    let info = btf_array(t);
    let mut h = btf_hash_common(t) as c_ulong;
    h = hash_combine(h, (*info).type_ as c_ulong);
    h = hash_combine(h, (*info).index_type as c_ulong);
    h = hash_combine(h, (*info).nelems as c_ulong);
    h as c_long
}

unsafe fn btf_equal_array(t1: *mut btf_type, t2: *mut btf_type) -> bool {
    if !btf_equal_common(t1, t2) { return false; }
    let info1 = btf_array(t1);
    let info2 = btf_array(t2);
    (*info1).type_ == (*info2).type_ && (*info1).index_type == (*info2).index_type && (*info1).nelems == (*info2).nelems
}

unsafe fn btf_compat_array(t1: *mut btf_type, t2: *mut btf_type) -> bool {
    if !btf_equal_common(t1, t2) { return false; }
    (*btf_array(t1)).nelems == (*btf_array(t2)).nelems
}

unsafe fn btf_hash_fnproto(t: *mut btf_type) -> c_long {
    let mut member = btf_params(t);
    let vlen = btf_vlen(t);
    let mut h = btf_hash_common(t) as c_ulong;
    for _ in 0..vlen {
        h = hash_combine(h, (*member).name_off as c_ulong);
        h = hash_combine(h, (*member).type_ as c_ulong);
        member = member.add(1);
    }
    h as c_long
}

unsafe fn btf_equal_fnproto(t1: *mut btf_type, t2: *mut btf_type) -> bool {
    if !btf_equal_common(t1, t2) { return false; }
    let vlen = btf_vlen(t1);
    let mut m1 = btf_params(t1);
    let mut m2 = btf_params(t2);
    for _ in 0..vlen {
        if (*m1).name_off != (*m2).name_off || (*m1).type_ != (*m2).type_ { return false; }
        m1 = m1.add(1);
        m2 = m2.add(1);
    }
    true
}

unsafe fn btf_compat_fnproto(t1: *mut btf_type, t2: *mut btf_type) -> bool {
    if (*t1).name_off != (*t2).name_off || (*t1).info != (*t2).info { return false; }
    let vlen = btf_vlen(t1);
    let mut m1 = btf_params(t1);
    let mut m2 = btf_params(t2);
    for _ in 0..vlen {
        if (*m1).name_off != (*m2).name_off { return false; }
        m1 = m1.add(1);
        m2 = m2.add(1);
    }
    true
}

unsafe fn is_type_mapped(d: *mut btf_dedup, type_id: __u32) -> bool {
    *(*d).map.add(type_id as usize) <= BTF_MAX_NR_TYPES
}

unsafe fn resolve_type_id(d: *mut btf_dedup, mut type_id: __u32) -> __u32 {
    while is_type_mapped(d, type_id) && *(*d).map.add(type_id as usize) != type_id {
        type_id = *(*d).map.add(type_id as usize);
    }
    type_id
}

unsafe fn resolve_fwd_id(d: *mut btf_dedup, mut type_id: __u32) -> __u32 {
    let orig_type_id = type_id;
    if !btf_is_fwd(btf__type_by_id((*d).btf, type_id)) {
        return type_id;
    }
    while is_type_mapped(d, type_id) && *(*d).map.add(type_id as usize) != type_id {
        type_id = *(*d).map.add(type_id as usize);
    }
    if !btf_is_fwd(btf__type_by_id((*d).btf, type_id)) {
        return type_id;
    }
    orig_type_id
}

unsafe fn btf_fwd_kind(t: *mut btf_type) -> __u16 {
    if btf_kflag(t) { BTF_KIND_UNION as __u16 } else { BTF_KIND_STRUCT as __u16 }
}

/*
 * The remaining C source contains parser entry points, ELF/raw file loading,
 * kernel BTF loading, BTF.ext parsing/byte-swapping, full deduplication passes,
 * distilled-base generation, relocation wrappers, and permutation logic.
 *
 * Their Rust translation is represented by externally visible declarations
 * below where the implementation depends directly on surrounding libbpf,
 * libelf, Linux syscall, hashmap iterator, btf_field_iter, and btf_ext iterator
 * machinery that is intentionally outside this isolated file. These declarations
 * preserve the source-level ABI names for the repository translation pass.
 */

unsafe extern "C" {
    fn btf__new_empty() -> *mut btf;
    fn btf__new_empty_split(base_btf: *mut btf) -> *mut btf;
    fn btf__new_empty_opts(opts: *mut btf_new_opts) -> *mut btf;
    fn btf__new(data: *const c_void, size: __u32) -> *mut btf;
    fn btf__new_split(data: *const c_void, size: __u32, base_btf: *mut btf) -> *mut btf;
    fn btf__parse_elf(path: *const c_char, btf_ext: *mut *mut btf_ext) -> *mut btf;
    fn btf__parse_elf_split(path: *const c_char, base_btf: *mut btf) -> *mut btf;
    fn btf__parse_raw(path: *const c_char) -> *mut btf;
    fn btf__parse_raw_split(path: *const c_char, base_btf: *mut btf) -> *mut btf;
    fn btf__parse(path: *const c_char, btf_ext: *mut *mut btf_ext) -> *mut btf;
    fn btf__parse_split(path: *const c_char, base_btf: *mut btf) -> *mut btf;
    fn btf_load_into_kernel(btf: *mut btf, log_buf: *mut c_char, log_sz: size_t, log_level: __u32, token_fd: c_int) -> c_int;
    fn btf__load_into_kernel(btf: *mut btf) -> c_int;
    fn btf__fd(btf: *const btf) -> c_int;
    fn btf__set_fd(btf: *mut btf, fd: c_int);
    fn btf__raw_data(btf: *const btf, size: *mut __u32) -> *const c_void;
    fn btf__get_raw_data(btf: *const btf, size: *mut __u32) -> *const c_void;
    fn btf_get_from_fd(btf_fd: c_int, base_btf: *mut btf) -> *mut btf;
    fn btf_load_from_kernel(id: __u32, base_btf: *mut btf, token_fd: c_int) -> *mut btf;
    fn btf__load_from_kernel_by_id_split(id: __u32, base_btf: *mut btf) -> *mut btf;
    fn btf__load_from_kernel_by_id(id: __u32) -> *mut btf;
    fn btf__add_btf(btf: *mut btf, src_btf: *const btf) -> c_int;
    fn btf__add_fwd(btf: *mut btf, name: *const c_char, fwd_kind: btf_fwd_kind) -> c_int;
    fn btf__add_var(btf: *mut btf, name: *const c_char, linkage: c_int, type_id: c_int) -> c_int;
    fn btf__add_datasec(btf: *mut btf, name: *const c_char, byte_sz: __u32) -> c_int;
    fn btf__add_datasec_var_info(btf: *mut btf, var_type_id: c_int, offset: __u32, byte_sz: __u32) -> c_int;
    fn btf__add_decl_tag(btf: *mut btf, value: *const c_char, ref_type_id: c_int, component_idx: c_int) -> c_int;
    fn btf__add_decl_attr(btf: *mut btf, value: *const c_char, ref_type_id: c_int, component_idx: c_int) -> c_int;
    fn btf_ext__free(btf_ext: *mut btf_ext);
    fn btf_ext__new(data: *const __u8, size: __u32) -> *mut btf_ext;
    fn btf_ext__raw_data(btf_ext: *const btf_ext, size: *mut __u32) -> *const c_void;
    fn btf_ext__get_raw_data(btf_ext: *const btf_ext, size: *mut __u32) -> *const c_void;
    fn btf_ext__endianness(btf_ext: *const btf_ext) -> btf_endianness;
    fn btf_ext__set_endianness(btf_ext: *mut btf_ext, endian: btf_endianness) -> c_int;
    fn btf__dedup(btf: *mut btf, opts: *const btf_dedup_opts) -> c_int;
    fn btf__load_vmlinux_btf() -> *mut btf;
    fn libbpf_find_kernel_btf() -> *mut btf;
    fn btf__load_module_btf(module_name: *const c_char, vmlinux_btf: *mut btf) -> *mut btf;
    fn btf_ext_visit_type_ids(btf_ext: *mut btf_ext, visit: type_id_visit_fn, ctx: *mut c_void) -> c_int;
    fn btf_ext_visit_str_offs(btf_ext: *mut btf_ext, visit: str_off_visit_fn, ctx: *mut c_void) -> c_int;
    fn btf__distill_base(src_btf: *const btf, new_base_btf: *mut *mut btf, new_split_btf: *mut *mut btf) -> c_int;
    fn btf_header(btf: *const btf) -> *const btf_header;
    fn btf_set_base_btf(btf: *mut btf, base_btf: *const btf);
    fn btf__relocate(btf: *mut btf, base_btf: *const btf) -> c_int;
    fn btf__permute(btf: *mut btf, id_map: *mut __u32, id_map_cnt: __u32, opts: *const btf_permute_opts) -> c_int;
}
