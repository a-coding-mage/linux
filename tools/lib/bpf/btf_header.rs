/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */
/* Copyright (c) 2018 Facebook */
/*! \file */

use core::ffi::{c_char, c_int, c_void};

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;
pub type __s32 = i32;
pub type __s64 = i64;
pub type size_t = usize;
pub type va_list = *mut c_void;

pub const BTF_ELF_SEC: &[u8] = b".BTF\0";
pub const BTF_EXT_ELF_SEC: &[u8] = b".BTF.ext\0";
pub const BTF_BASE_ELF_SEC: &[u8] = b".BTF.base\0";
pub const MAPS_ELF_SEC: &[u8] = b".maps\0";

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_ext {
    _private: [u8; 0],
}

#[repr(C)]
pub union btf_type_size_type {
    pub size: __u32,
    pub type_: __u32,
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
    pub info: __u32,
    pub size_type: btf_type_size_type,
}

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum btf_endianness {
    BTF_LITTLE_ENDIAN = 0,
    BTF_BIG_ENDIAN = 1,
}

unsafe extern "C" {
    /**
     * @brief **btf__free()** frees all data of a BTF object
     * @param btf BTF object to free
     */
    pub fn btf__free(btf: *mut btf);

    /**
     * @brief **btf__new()** creates a new instance of a BTF object from the raw
     * bytes of an ELF's BTF section
     * @param data raw bytes
     * @param size number of bytes passed in `data`
     * @return new BTF object instance which has to be eventually freed with
     * **btf__free()**
     *
     * On error, error-code-encoded-as-pointer is returned, not a NULL. To extract
     * error code from such a pointer `libbpf_get_error()` should be used. If
     * `libbpf_set_strict_mode(LIBBPF_STRICT_CLEAN_PTRS)` is enabled, NULL is
     * returned on error instead. In both cases thread-local `errno` variable is
     * always set to error code as well.
     */
    pub fn btf__new(data: *const c_void, size: __u32) -> *mut btf;

    /**
     * @brief **btf__new_split()** create a new instance of a BTF object from the
     * provided raw data bytes. It takes another BTF instance, **base_btf**, which
     * serves as a base BTF, which is extended by types in a newly created BTF
     * instance
     * @param data raw bytes
     * @param size length of raw bytes
     * @param base_btf the base BTF object
     * @return new BTF object instance which has to be eventually freed with
     * **btf__free()**
     *
     * If *base_btf* is NULL, `btf__new_split()` is equivalent to `btf__new()` and
     * creates non-split BTF.
     *
     * On error, error-code-encoded-as-pointer is returned, not a NULL. To extract
     * error code from such a pointer `libbpf_get_error()` should be used. If
     * `libbpf_set_strict_mode(LIBBPF_STRICT_CLEAN_PTRS)` is enabled, NULL is
     * returned on error instead. In both cases thread-local `errno` variable is
     * always set to error code as well.
     */
    pub fn btf__new_split(data: *const c_void, size: __u32, base_btf: *mut btf) -> *mut btf;

    /**
     * @brief **btf__new_empty()** creates an empty BTF object.  Use
     * `btf__add_*()` to populate such BTF object.
     * @return new BTF object instance which has to be eventually freed with
     * **btf__free()**
     *
     * On error, error-code-encoded-as-pointer is returned, not a NULL. To extract
     * error code from such a pointer `libbpf_get_error()` should be used. If
     * `libbpf_set_strict_mode(LIBBPF_STRICT_CLEAN_PTRS)` is enabled, NULL is
     * returned on error instead. In both cases thread-local `errno` variable is
     * always set to error code as well.
     */
    pub fn btf__new_empty() -> *mut btf;

    /**
     * @brief **btf__new_empty_split()** creates an unpopulated BTF object from an
     * ELF BTF section except with a base BTF on top of which split BTF should be
     * based
     * @param base_btf base BTF object
     * @return new BTF object instance which has to be eventually freed with
     * **btf__free()**
     *
     * If *base_btf* is NULL, `btf__new_empty_split()` is equivalent to
     * `btf__new_empty()` and creates non-split BTF.
     *
     * On error, error-code-encoded-as-pointer is returned, not a NULL. To extract
     * error code from such a pointer `libbpf_get_error()` should be used. If
     * `libbpf_set_strict_mode(LIBBPF_STRICT_CLEAN_PTRS)` is enabled, NULL is
     * returned on error instead. In both cases thread-local `errno` variable is
     * always set to error code as well.
     */
    pub fn btf__new_empty_split(base_btf: *mut btf) -> *mut btf;
}

#[repr(C)]
pub struct btf_new_opts {
    pub sz: size_t,
    pub base_btf: *mut btf, /* optional base BTF */
    pub add_layout: bool,  /* add BTF layout information */
}
pub const btf_new_opts__last_field: &str = "add_layout";

unsafe extern "C" {
    /**
     * @brief **btf__new_empty_opts()** creates an unpopulated BTF object with
     * optional *base_btf* and BTF kind layout description if *add_layout*
     * is set
     * @return new BTF object instance which has to be eventually freed with
     * **btf__free()**
     *
     * On error, NULL is returned and the thread-local `errno` variable is
     * set to the error code.
     */
    pub fn btf__new_empty_opts(opts: *mut btf_new_opts) -> *mut btf;

    /**
     * @brief **btf__distill_base()** creates new versions of the split BTF
     * *src_btf* and its base BTF. The new base BTF will only contain the types
     * needed to improve robustness of the split BTF to small changes in base BTF.
     * When that split BTF is loaded against a (possibly changed) base, this
     * distilled base BTF will help update references to that (possibly changed)
     * base BTF.
     * @param src_btf source split BTF object
     * @param new_base_btf pointer to where the new base BTF object pointer will be stored
     * @param new_split_btf pointer to where the new split BTF object pointer will be stored
     * @return 0 on success; negative error code, otherwise
     *
     * Both the new split and its associated new base BTF must be freed by
     * the caller.
     *
     * If successful, 0 is returned and **new_base_btf** and **new_split_btf**
     * will point at new base/split BTF. Both the new split and its associated
     * new base BTF must be freed by the caller.
     *
     * A negative value is returned on error and the thread-local `errno` variable
     * is set to the error code as well.
     */
    pub fn btf__distill_base(
        src_btf: *const btf,
        new_base_btf: *mut *mut btf,
        new_split_btf: *mut *mut btf,
    ) -> c_int;

    pub fn btf__parse(path: *const c_char, btf_ext: *mut *mut btf_ext) -> *mut btf;
    pub fn btf__parse_split(path: *const c_char, base_btf: *mut btf) -> *mut btf;
    pub fn btf__parse_elf(path: *const c_char, btf_ext: *mut *mut btf_ext) -> *mut btf;
    pub fn btf__parse_elf_split(path: *const c_char, base_btf: *mut btf) -> *mut btf;
    pub fn btf__parse_raw(path: *const c_char) -> *mut btf;
    pub fn btf__parse_raw_split(path: *const c_char, base_btf: *mut btf) -> *mut btf;

    pub fn btf__load_vmlinux_btf() -> *mut btf;
    pub fn btf__load_module_btf(module_name: *const c_char, vmlinux_btf: *mut btf) -> *mut btf;

    pub fn btf__load_from_kernel_by_id(id: __u32) -> *mut btf;
    pub fn btf__load_from_kernel_by_id_split(id: __u32, base_btf: *mut btf) -> *mut btf;

    pub fn btf__load_into_kernel(btf: *mut btf) -> c_int;
    pub fn btf__find_by_name(btf: *const btf, type_name: *const c_char) -> __s32;
    pub fn btf__find_by_name_kind(btf: *const btf, type_name: *const c_char, kind: __u32) -> __s32;
    pub fn btf__find_by_name_kind_own(
        btf: *const btf,
        type_name: *const c_char,
        kind: __u32,
    ) -> __s32;
    pub fn btf__type_cnt(btf: *const btf) -> __u32;
    pub fn btf__base_btf(btf: *const btf) -> *const btf;
    pub fn btf__type_by_id(btf: *const btf, id: __u32) -> *const btf_type;
    pub fn btf__pointer_size(btf: *const btf) -> size_t;
    pub fn btf__set_pointer_size(btf: *mut btf, ptr_sz: size_t) -> c_int;
    pub fn btf__endianness(btf: *const btf) -> btf_endianness;
    pub fn btf__set_endianness(btf: *mut btf, endian: btf_endianness) -> c_int;
    pub fn btf__resolve_size(btf: *const btf, type_id: __u32) -> __s64;
    pub fn btf__resolve_type(btf: *const btf, type_id: __u32) -> c_int;
    pub fn btf__align_of(btf: *const btf, id: __u32) -> c_int;
    pub fn btf__fd(btf: *const btf) -> c_int;
    pub fn btf__set_fd(btf: *mut btf, fd: c_int);
    pub fn btf__raw_data(btf: *const btf, size: *mut __u32) -> *const c_void;
    pub fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    pub fn btf__str_by_offset(btf: *const btf, offset: __u32) -> *const c_char;

    pub fn btf_ext__new(data: *const __u8, size: __u32) -> *mut btf_ext;
    pub fn btf_ext__free(btf_ext: *mut btf_ext);
    pub fn btf_ext__raw_data(btf_ext: *const btf_ext, size: *mut __u32) -> *const c_void;
    pub fn btf_ext__endianness(btf_ext: *const btf_ext) -> btf_endianness;
    pub fn btf_ext__set_endianness(btf_ext: *mut btf_ext, endian: btf_endianness) -> c_int;

    pub fn btf__find_str(btf: *mut btf, s: *const c_char) -> c_int;
    pub fn btf__add_str(btf: *mut btf, s: *const c_char) -> c_int;
    pub fn btf__add_type(btf: *mut btf, src_btf: *const btf, src_type: *const btf_type) -> c_int;
    /**
     * @brief **btf__add_btf()** appends all the BTF types from *src_btf* into *btf*
     * @param btf BTF object which all the BTF types and strings are added to
     * @param src_btf BTF object which all BTF types and referenced strings are copied from
     * @return BTF type ID of the first appended BTF type, or negative error code
     *
     * **btf__add_btf()** can be used to simply and efficiently append the entire
     * contents of one BTF object to another one. All the BTF type data is copied
     * over, all referenced type IDs are adjusted by adding a necessary ID offset.
     * Only strings referenced from BTF types are copied over and deduplicated, so
     * if there were some unused strings in *src_btf*, those won't be copied over,
     * which is consistent with the general string deduplication semantics of BTF
     * writing APIs.
     *
     * If any error is encountered during this process, the contents of *btf* is
     * left intact, which means that **btf__add_btf()** follows the transactional
     * semantics and the operation as a whole is all-or-nothing.
     *
     * *src_btf* has to be non-split BTF, as of now copying types from split BTF
     * is not supported and will result in -ENOTSUP error code returned.
     */
    pub fn btf__add_btf(btf: *mut btf, src_btf: *const btf) -> c_int;

    pub fn btf__add_int(btf: *mut btf, name: *const c_char, byte_sz: size_t, encoding: c_int) -> c_int;
    pub fn btf__add_float(btf: *mut btf, name: *const c_char, byte_sz: size_t) -> c_int;
    pub fn btf__add_ptr(btf: *mut btf, ref_type_id: c_int) -> c_int;
    pub fn btf__add_array(
        btf: *mut btf,
        index_type_id: c_int,
        elem_type_id: c_int,
        nr_elems: __u32,
    ) -> c_int;
    /* struct/union construction APIs */
    pub fn btf__add_struct(btf: *mut btf, name: *const c_char, sz: __u32) -> c_int;
    pub fn btf__add_union(btf: *mut btf, name: *const c_char, sz: __u32) -> c_int;
    pub fn btf__add_field(
        btf: *mut btf,
        name: *const c_char,
        field_type_id: c_int,
        bit_offset: __u32,
        bit_size: __u32,
    ) -> c_int;

    /* enum construction APIs */
    pub fn btf__add_enum(btf: *mut btf, name: *const c_char, bytes_sz: __u32) -> c_int;
    pub fn btf__add_enum_value(btf: *mut btf, name: *const c_char, value: __s64) -> c_int;
    pub fn btf__add_enum64(
        btf: *mut btf,
        name: *const c_char,
        bytes_sz: __u32,
        is_signed: bool,
    ) -> c_int;
    pub fn btf__add_enum64_value(btf: *mut btf, name: *const c_char, value: __u64) -> c_int;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum btf_fwd_kind {
    BTF_FWD_STRUCT = 0,
    BTF_FWD_UNION = 1,
    BTF_FWD_ENUM = 2,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum btf_func_linkage {
    BTF_FUNC_STATIC = 0,
    BTF_FUNC_GLOBAL = 1,
    BTF_FUNC_EXTERN = 2,
}

unsafe extern "C" {
    pub fn btf__add_fwd(btf: *mut btf, name: *const c_char, fwd_kind: btf_fwd_kind) -> c_int;
    pub fn btf__add_typedef(btf: *mut btf, name: *const c_char, ref_type_id: c_int) -> c_int;
    pub fn btf__add_volatile(btf: *mut btf, ref_type_id: c_int) -> c_int;
    pub fn btf__add_const(btf: *mut btf, ref_type_id: c_int) -> c_int;
    pub fn btf__add_restrict(btf: *mut btf, ref_type_id: c_int) -> c_int;
    pub fn btf__add_type_tag(btf: *mut btf, value: *const c_char, ref_type_id: c_int) -> c_int;
    pub fn btf__add_type_attr(btf: *mut btf, value: *const c_char, ref_type_id: c_int) -> c_int;

    /* func and func_proto construction APIs */
    pub fn btf__add_func(
        btf: *mut btf,
        name: *const c_char,
        linkage: btf_func_linkage,
        proto_type_id: c_int,
    ) -> c_int;
    pub fn btf__add_func_proto(btf: *mut btf, ret_type_id: c_int) -> c_int;
    pub fn btf__add_func_param(btf: *mut btf, name: *const c_char, type_id: c_int) -> c_int;

    /* var & datasec construction APIs */
    pub fn btf__add_var(btf: *mut btf, name: *const c_char, linkage: c_int, type_id: c_int) -> c_int;
    pub fn btf__add_datasec(btf: *mut btf, name: *const c_char, byte_sz: __u32) -> c_int;
    pub fn btf__add_datasec_var_info(
        btf: *mut btf,
        var_type_id: c_int,
        offset: __u32,
        byte_sz: __u32,
    ) -> c_int;

    /* tag construction API */
    pub fn btf__add_decl_tag(
        btf: *mut btf,
        value: *const c_char,
        ref_type_id: c_int,
        component_idx: c_int,
    ) -> c_int;
    pub fn btf__add_decl_attr(
        btf: *mut btf,
        value: *const c_char,
        ref_type_id: c_int,
        component_idx: c_int,
    ) -> c_int;
}

#[repr(C)]
pub struct btf_dedup_opts {
    pub sz: size_t,
    /* optional .BTF.ext info to dedup along the main BTF info */
    pub btf_ext: *mut btf_ext,
    /* force hash collisions (used for testing) */
    pub force_collisions: bool,
}
pub const btf_dedup_opts__last_field: &str = "force_collisions";

unsafe extern "C" {
    pub fn btf__dedup(btf: *mut btf, opts: *const btf_dedup_opts) -> c_int;

    /**
     * @brief **btf__relocate()** will check the split BTF *btf* for references
     * to base BTF kinds, and verify those references are compatible with
     * *base_btf*; if they are, *btf* is adjusted such that is re-parented to
     * *base_btf* and type ids and strings are adjusted to accommodate this.
     * @param btf split BTF object to relocate
     * @param base_btf base BTF object
     * @return 0 on success; negative error code, otherwise
     *
     * If successful, 0 is returned and **btf** now has **base_btf** as its
     * base.
     *
     * A negative value is returned on error and the thread-local `errno` variable
     * is set to the error code as well.
     */
    pub fn btf__relocate(btf: *mut btf, base_btf: *const btf) -> c_int;
}

#[repr(C)]
pub struct btf_permute_opts {
    pub sz: size_t,
    /* optional .BTF.ext info along the main BTF info */
    pub btf_ext: *mut btf_ext,
}
pub const btf_permute_opts__last_field: &str = "btf_ext";

unsafe extern "C" {
    /**
     * @brief **btf__permute()** rearranges BTF types in-place according to a specified ID mapping
     * @param btf BTF object to permute
     * @param id_map Array mapping original type IDs to new IDs
     * @param id_map_cnt Number of elements in @id_map
     * @param opts Optional parameters, including BTF extension data for reference updates
     * @return 0 on success, negative error code on failure
     *
     * **btf__permute()** reorders BTF types based on the provided @id_map array,
     * updating all internal type references to maintain consistency. The function
     * operates in-place, modifying the BTF object directly.
     *
     * For **base BTF**:
     * - @id_map must include all types from ID 0 to `btf__type_cnt(btf) - 1`
     * - @id_map_cnt must be `btf__type_cnt(btf)`
     * - Mapping is defined as `id_map[original_id] = new_id`
     * - `id_map[0]` must be 0 (void type cannot be moved)
     *
     * For **split BTF**:
     * - @id_map must include only split types (types added on top of the base BTF)
     * - @id_map_cnt must be `btf__type_cnt(btf) - btf__type_cnt(btf__base_btf(btf))`
     * - Mapping is defined as `id_map[original_id - start_id] = new_id`
     * - `start_id` equals `btf__type_cnt(btf__base_btf(btf))`
     *
     * After permutation, all type references within the BTF data and optional
     * BTF extension (if provided via @opts) are updated automatically.
     *
     * On error, returns a negative error code and sets errno:
     *   - `-EINVAL`: Invalid parameters or invalid ID mapping
     *   - `-ENOMEM`: Memory allocation failure
     */
    pub fn btf__permute(
        btf: *mut btf,
        id_map: *mut __u32,
        id_map_cnt: __u32,
        opts: *const btf_permute_opts,
    ) -> c_int;
}

#[repr(C)]
pub struct btf_dump {
    _private: [u8; 0],
}

#[repr(C)]
pub struct btf_dump_opts {
    pub sz: size_t,
}
pub const btf_dump_opts__last_field: &str = "sz";

pub type btf_dump_printf_fn_t =
    Option<unsafe extern "C" fn(ctx: *mut c_void, fmt: *const c_char, args: va_list)>;

unsafe extern "C" {
    pub fn btf_dump__new(
        btf: *const btf,
        printf_fn: btf_dump_printf_fn_t,
        ctx: *mut c_void,
        opts: *const btf_dump_opts,
    ) -> *mut btf_dump;

    pub fn btf_dump__free(d: *mut btf_dump);

    pub fn btf_dump__dump_type(d: *mut btf_dump, id: __u32) -> c_int;
}

#[repr(C)]
pub struct btf_dump_emit_type_decl_opts {
    /* size of this struct, for forward/backward compatibility */
    pub sz: size_t,
    /* optional field name for type declaration, e.g.:
     * - struct my_struct <FNAME>
     * - void (*<FNAME>)(int)
     * - char (*<FNAME>)[123]
     */
    pub field_name: *const c_char,
    /* extra indentation level (in number of tabs) to emit for multi-line
     * type declarations (e.g., anonymous struct); applies for lines
     * starting from the second one (first line is assumed to have
     * necessary indentation already
     */
    pub indent_level: c_int,
    /* strip all the const/volatile/restrict mods */
    pub strip_mods: bool,
}
pub const btf_dump_emit_type_decl_opts__last_field: &str = "strip_mods";

unsafe extern "C" {
    pub fn btf_dump__emit_type_decl(
        d: *mut btf_dump,
        id: __u32,
        opts: *const btf_dump_emit_type_decl_opts,
    ) -> c_int;
}

#[repr(C)]
pub struct btf_dump_type_data_opts {
    /* size of this struct, for forward/backward compatibility */
    pub sz: size_t,
    pub indent_str: *const c_char,
    pub indent_level: c_int,
    /* below match "show" flags for bpf_show_snprintf() */
    pub compact: bool,      /* no newlines/indentation */
    pub skip_names: bool,   /* skip member/type names */
    pub emit_zeroes: bool,  /* show 0-valued fields */
    pub emit_strings: bool, /* print char arrays as strings */
}
pub const btf_dump_type_data_opts__last_field: &str = "emit_strings";

unsafe extern "C" {
    pub fn btf_dump__dump_type_data(
        d: *mut btf_dump,
        id: __u32,
        data: *const c_void,
        data_sz: size_t,
        opts: *const btf_dump_type_data_opts,
    ) -> c_int;
}

/*
 * A set of helpers for easier BTF types handling.
 *
 * The inline functions below rely on constants from the kernel headers which
 * may not be available for applications including this header file. To avoid
 * compilation errors, we define all the constants here that were added after
 * the initial introduction of the BTF_KIND* constants.
 */
/* Original C conditionally defines BTF_KIND_FUNC and BTF_KIND_FUNC_PROTO if
 * unavailable from kernel headers.
 */
pub const BTF_KIND_FUNC: __u32 = 12; /* Function */
pub const BTF_KIND_FUNC_PROTO: __u32 = 13; /* Function Proto */
/* Original C conditionally defines BTF_KIND_VAR and BTF_KIND_DATASEC if
 * unavailable from kernel headers.
 */
pub const BTF_KIND_VAR: __u32 = 14; /* Variable */
pub const BTF_KIND_DATASEC: __u32 = 15; /* Section */
/* Original C conditionally defines BTF_KIND_FLOAT if unavailable from kernel headers. */
pub const BTF_KIND_FLOAT: __u32 = 16; /* Floating point */
/* The kernel header switched to enums, so the following were never #defined */
pub const BTF_KIND_DECL_TAG: __u32 = 17; /* Decl Tag */
pub const BTF_KIND_TYPE_TAG: __u32 = 18; /* Type Tag */
pub const BTF_KIND_ENUM64: __u32 = 19; /* Enum for up-to 64bit values */

unsafe extern "C" {
    pub fn BTF_INFO_KIND(info: __u32) -> __u16;
    pub fn BTF_INFO_VLEN(info: __u32) -> __u32;
    pub fn BTF_INFO_KFLAG(info: __u32) -> bool;
    pub fn BTF_INT_ENCODING(int_data: __u32) -> __u8;
    pub fn BTF_INT_OFFSET(int_data: __u32) -> __u8;
    pub fn BTF_INT_BITS(int_data: __u32) -> __u8;
    pub fn BTF_MEMBER_BIT_OFFSET(offset: __u32) -> __u32;
    pub fn BTF_MEMBER_BITFIELD_SIZE(offset: __u32) -> __u32;
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

#[inline]
pub unsafe fn btf_kind(t: *const btf_type) -> __u16 {
    unsafe { BTF_INFO_KIND((*t).info) }
}

#[inline]
pub unsafe fn btf_vlen(t: *const btf_type) -> __u32 {
    unsafe { BTF_INFO_VLEN((*t).info) }
}

#[inline]
pub unsafe fn btf_kflag(t: *const btf_type) -> bool {
    unsafe { BTF_INFO_KFLAG((*t).info) }
}

#[inline]
pub unsafe fn btf_is_void(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_UNKN }
}

#[inline]
pub unsafe fn btf_is_int(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_INT }
}

#[inline]
pub unsafe fn btf_is_ptr(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_PTR }
}

#[inline]
pub unsafe fn btf_is_array(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_ARRAY }
}

#[inline]
pub unsafe fn btf_is_struct(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_STRUCT }
}

#[inline]
pub unsafe fn btf_is_union(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_UNION }
}

#[inline]
pub unsafe fn btf_is_composite(t: *const btf_type) -> bool {
    let kind: __u16 = unsafe { btf_kind(t) };

    kind as __u32 == BTF_KIND_STRUCT || kind as __u32 == BTF_KIND_UNION
}

#[inline]
pub unsafe fn btf_is_enum(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_ENUM }
}

#[inline]
pub unsafe fn btf_is_enum64(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_ENUM64 }
}

#[inline]
pub unsafe fn btf_is_fwd(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_FWD }
}

#[inline]
pub unsafe fn btf_is_typedef(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_TYPEDEF }
}

#[inline]
pub unsafe fn btf_is_volatile(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_VOLATILE }
}

#[inline]
pub unsafe fn btf_is_const(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_CONST }
}

#[inline]
pub unsafe fn btf_is_restrict(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_RESTRICT }
}

#[inline]
pub unsafe fn btf_is_mod(t: *const btf_type) -> bool {
    let kind: __u16 = unsafe { btf_kind(t) };

    kind as __u32 == BTF_KIND_VOLATILE
        || kind as __u32 == BTF_KIND_CONST
        || kind as __u32 == BTF_KIND_RESTRICT
        || kind as __u32 == BTF_KIND_TYPE_TAG
}

#[inline]
pub unsafe fn btf_is_func(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_FUNC }
}

#[inline]
pub unsafe fn btf_is_func_proto(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_FUNC_PROTO }
}

#[inline]
pub unsafe fn btf_is_var(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_VAR }
}

#[inline]
pub unsafe fn btf_is_datasec(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_DATASEC }
}

#[inline]
pub unsafe fn btf_is_float(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_FLOAT }
}

#[inline]
pub unsafe fn btf_is_decl_tag(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_DECL_TAG }
}

#[inline]
pub unsafe fn btf_is_type_tag(t: *const btf_type) -> bool {
    unsafe { btf_kind(t) as __u32 == BTF_KIND_TYPE_TAG }
}

#[inline]
pub unsafe fn btf_is_any_enum(t: *const btf_type) -> bool {
    unsafe { btf_is_enum(t) || btf_is_enum64(t) }
}

#[inline]
pub unsafe fn btf_kind_core_compat(t1: *const btf_type, t2: *const btf_type) -> bool {
    unsafe { btf_kind(t1) == btf_kind(t2) || (btf_is_any_enum(t1) && btf_is_any_enum(t2)) }
}

#[inline]
pub unsafe fn btf_int_encoding(t: *const btf_type) -> __u8 {
    unsafe { BTF_INT_ENCODING(*(t.add(1) as *const __u32)) }
}

#[inline]
pub unsafe fn btf_int_offset(t: *const btf_type) -> __u8 {
    unsafe { BTF_INT_OFFSET(*(t.add(1) as *const __u32)) }
}

#[inline]
pub unsafe fn btf_int_bits(t: *const btf_type) -> __u8 {
    unsafe { BTF_INT_BITS(*(t.add(1) as *const __u32)) }
}

#[repr(C)]
pub struct btf_array {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn btf_array(t: *const btf_type) -> *mut btf_array {
    unsafe { t.add(1) as *mut btf_array }
}

#[repr(C)]
pub struct btf_enum {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn btf_enum(t: *const btf_type) -> *mut btf_enum {
    unsafe { t.add(1) as *mut btf_enum }
}

#[repr(C)]
pub struct btf_enum64 {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn btf_enum64(t: *const btf_type) -> *mut btf_enum64 {
    unsafe { t.add(1) as *mut btf_enum64 }
}

#[inline]
pub unsafe fn btf_enum64_value(e: *const btf_enum64) -> __u64 {
    /* struct btf_enum64 is introduced in Linux 6.0, which is very
     * bleeding-edge. Here we are avoiding relying on struct btf_enum64
     * definition coming from kernel UAPI headers to support wider range
     * of system-wide kernel headers.
     *
     * Given this header can be also included from C++ applications, that
     * further restricts C tricks we can use (like using compatible
     * anonymous struct). So just treat struct btf_enum64 as
     * a three-element array of u32 and access second (lo32) and third
     * (hi32) elements directly.
     *
     * For reference, here is a struct btf_enum64 definition:
     *
     * const struct btf_enum64 {
     *      __u32   name_off;
     *      __u32   val_lo32;
     *      __u32   val_hi32;
     * };
     */
    let e64: *const __u32 = e as *const __u32;

    unsafe { ((*e64.add(2) as __u64) << 32) | (*e64.add(1) as __u64) }
}

#[repr(C)]
pub struct btf_member {
    pub name_off: __u32,
    pub type_: __u32,
    pub offset: __u32,
}

#[inline]
pub unsafe fn btf_members(t: *const btf_type) -> *mut btf_member {
    unsafe { t.add(1) as *mut btf_member }
}

/* Get bit offset of a member with specified index. */
#[inline]
pub unsafe fn btf_member_bit_offset(t: *const btf_type, member_idx: __u32) -> __u32 {
    let m: *const btf_member = unsafe { btf_members(t).add(member_idx as usize) };
    let kflag: bool = unsafe { btf_kflag(t) };

    if kflag {
        unsafe { BTF_MEMBER_BIT_OFFSET((*m).offset) }
    } else {
        unsafe { (*m).offset }
    }
}

/*
 * Get bitfield size of a member, assuming t is BTF_KIND_STRUCT or
 * BTF_KIND_UNION. If member is not a bitfield, zero is returned.
 */
#[inline]
pub unsafe fn btf_member_bitfield_size(t: *const btf_type, member_idx: __u32) -> __u32 {
    let m: *const btf_member = unsafe { btf_members(t).add(member_idx as usize) };
    let kflag: bool = unsafe { btf_kflag(t) };

    if kflag {
        unsafe { BTF_MEMBER_BITFIELD_SIZE((*m).offset) }
    } else {
        0
    }
}

#[repr(C)]
pub struct btf_param {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn btf_params(t: *const btf_type) -> *mut btf_param {
    unsafe { t.add(1) as *mut btf_param }
}

#[repr(C)]
pub struct btf_var {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn btf_var(t: *const btf_type) -> *mut btf_var {
    unsafe { t.add(1) as *mut btf_var }
}

#[repr(C)]
pub struct btf_var_secinfo {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn btf_var_secinfos(t: *const btf_type) -> *mut btf_var_secinfo {
    unsafe { t.add(1) as *mut btf_var_secinfo }
}

#[repr(C)]
pub struct btf_decl_tag {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn btf_decl_tag(t: *const btf_type) -> *mut btf_decl_tag {
    unsafe { t.add(1) as *mut btf_decl_tag }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
