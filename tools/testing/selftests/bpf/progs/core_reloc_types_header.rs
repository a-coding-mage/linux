use core::ffi::{c_char, c_int, c_long, c_longlong, c_void};

pub unsafe extern "C" fn preserce_ptr_sz_fn(_x: c_long) {}

/* __bpf_aligned: C used __attribute__((aligned(8))). */

/*
 * KERNEL
 */

#[repr(C)]
pub struct core_reloc_kernel_output {
    pub valid: [c_int; 10],
    pub comm: [c_char; 11],
    pub comm_len: c_int,
    pub local_task_struct_matches: bool,
}

/*
 * MODULE
 */

#[repr(C)]
pub struct core_reloc_module_output {
    pub len: c_longlong,
    pub off: c_longlong,
    pub read_ctx_sz: c_int,
    pub read_ctx_exists: bool,
    pub buf_exists: bool,
    pub len_exists: bool,
    pub off_exists: bool,
    /* we have test_progs[-flavor], so cut flavor part */
    pub comm: [c_char; 11],
    pub comm_len: c_int,
}

/*
 * FLAVORS
 */
#[repr(C)]
pub struct core_reloc_flavors {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
}

/* this is not a flavor, as it doesn't have triple underscore */
#[repr(C)]
pub struct core_reloc_flavors__err_wrong_name {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
}

/*
 * NESTING
 */
/* original set up, used to record relocations in BPF program */
#[repr(C)]
pub struct core_reloc_nesting_substruct {
    pub a: c_int,
}

#[repr(C)]
pub union core_reloc_nesting_subunion {
    pub b: c_int,
}

#[repr(C)]
pub union core_reloc_nesting_a_union {
    pub a: core_reloc_nesting_substruct,
}

#[repr(C)]
pub struct core_reloc_nesting_b_struct {
    pub b: core_reloc_nesting_subunion,
}

#[repr(C)]
pub struct core_reloc_nesting {
    pub a: core_reloc_nesting_a_union,
    pub b: core_reloc_nesting_b_struct,
}

#[repr(C)]
pub struct core_reloc_nesting___anon_embed_a_a_struct {
    pub a: c_int,
}

#[repr(C)]
pub union core_reloc_nesting___anon_embed_a_union {
    pub a: core_reloc_nesting___anon_embed_a_a_struct,
}

#[repr(C)]
pub union core_reloc_nesting___anon_embed_b_b_union {
    pub b: c_int,
}

#[repr(C)]
pub struct core_reloc_nesting___anon_embed_b_struct {
    pub b: core_reloc_nesting___anon_embed_b_b_union,
}

/* inlined anonymous struct/union instead of named structs in original */
#[repr(C)]
pub struct core_reloc_nesting___anon_embed {
    pub __just_for_padding: c_int,
    pub a: core_reloc_nesting___anon_embed_a_union,
    pub b: core_reloc_nesting___anon_embed_b_struct,
}

#[repr(C)]
pub union core_reloc_nesting___struct_union_mixup_a_union {
    pub __a: c_char,
    pub a: c_int,
}

#[repr(C)]
pub struct core_reloc_nesting___struct_union_mixup_a_struct {
    pub __a: c_int,
    pub a: core_reloc_nesting___struct_union_mixup_a_union,
}

#[repr(C)]
pub union core_reloc_nesting___struct_union_mixup_b_b_union {
    pub __b: c_char,
    pub b: c_int,
}

#[repr(C)]
pub union core_reloc_nesting___struct_union_mixup_b_union {
    pub __b: c_int,
    pub b: core_reloc_nesting___struct_union_mixup_b_b_union,
}

/* different mix of nested structs/unions than in original */
#[repr(C)]
pub struct core_reloc_nesting___struct_union_mixup {
    pub __a: c_int,
    pub a: core_reloc_nesting___struct_union_mixup_a_struct,
    pub __b: c_int,
    pub b: core_reloc_nesting___struct_union_mixup_b_union,
}

/* extra anon structs/unions, but still valid a.a.a and b.b.b accessors */
#[repr(C)]
pub struct core_reloc_nesting___extra_nesting {
    pub __padding: c_int,
    /* anonymous C nesting collapsed to the observable named fields */
    pub a: core_reloc_nesting___dup_compat_types_a,
    pub __some_more: c_int,
    pub b: core_reloc_nesting___dup_compat_types_b,
}

#[repr(C)]
pub struct core_reloc_nesting___dup_compat_types_a_a {
    pub a: c_int,
}

#[repr(C)]
pub struct core_reloc_nesting___dup_compat_types_a {
    pub a: core_reloc_nesting___dup_compat_types_a_a,
}

#[repr(C)]
pub struct core_reloc_nesting___dup_compat_types_b_b {
    pub b: c_int,
}

#[repr(C)]
pub struct core_reloc_nesting___dup_compat_types_b {
    pub b: core_reloc_nesting___dup_compat_types_b_b,
}

/* three flavors of same struct with different structure but same layout for
 * a.a.a and b.b.b, thus successfully resolved and relocatable */
#[repr(C)]
pub struct core_reloc_nesting___dup_compat_types {
    pub __just_for_padding: c_char,
    /* 3 more bytes of padding */
    pub a: core_reloc_nesting___dup_compat_types_a,
    pub __more_padding: c_longlong,
    pub b: core_reloc_nesting___dup_compat_types_b,
}

#[repr(C)]
pub struct core_reloc_nesting___dup_compat_types__2_a_a {
    pub __some_more_noops: [c_char; 0],
    pub a: c_int,
}

#[repr(C)]
pub struct core_reloc_nesting___dup_compat_types__2_a {
    pub __trickier_noop: [c_int; 0],
    pub a: core_reloc_nesting___dup_compat_types__2_a_a,
}

#[repr(C)]
pub struct core_reloc_nesting___dup_compat_types__2_b_b_b {
    pub __critical_padding: c_int,
    pub b: c_int,
}

#[repr(C)]
pub struct core_reloc_nesting___dup_compat_types__2_b_b {
    pub b: core_reloc_nesting___dup_compat_types__2_b_b_b,
    pub __does_not_matter: c_int,
}

#[repr(C)]
pub struct core_reloc_nesting___dup_compat_types__2_b {
    pub anon: core_reloc_nesting___dup_compat_types__2_b_b,
}

#[repr(C)]
pub struct core_reloc_nesting___dup_compat_types__2 {
    pub __aligned_padding: c_int,
    pub a: core_reloc_nesting___dup_compat_types__2_a,
    pub __more_padding: c_int,
    pub b: core_reloc_nesting___dup_compat_types__2_b,
    pub __more_irrelevant_stuff: c_int,
}

#[repr(C)]
pub struct core_reloc_nesting___dup_compat_types__3 {
    pub __correct_padding: [c_char; 4],
    pub a: core_reloc_nesting___dup_compat_types_a,
    /* 8 byte padding due to next struct's alignment; C aligned b to 16. */
    pub b: core_reloc_nesting___dup_compat_types_b,
}

/* b.b.b field is missing */
#[repr(C)]
pub struct core_reloc_nesting___err_missing_field_b_b {
    pub x: c_int,
}

#[repr(C)]
pub struct core_reloc_nesting___err_missing_field_b {
    pub b: core_reloc_nesting___err_missing_field_b_b,
}

#[repr(C)]
pub struct core_reloc_nesting___err_missing_field {
    pub a: core_reloc_nesting___dup_compat_types_a,
    pub b: core_reloc_nesting___err_missing_field_b,
}

/* b.b.b field is an array of integers instead of plain int */
#[repr(C)]
pub struct core_reloc_nesting___err_array_field_b_b {
    pub b: [c_int; 1],
}

#[repr(C)]
pub struct core_reloc_nesting___err_array_field_b {
    pub b: core_reloc_nesting___err_array_field_b_b,
}

#[repr(C)]
pub struct core_reloc_nesting___err_array_field {
    pub a: core_reloc_nesting___dup_compat_types_a,
    pub b: core_reloc_nesting___err_array_field_b,
}

/* middle b container is missing */
#[repr(C)]
pub struct core_reloc_nesting___err_missing_container_b {
    pub x: c_int,
}

#[repr(C)]
pub struct core_reloc_nesting___err_missing_container {
    pub a: core_reloc_nesting___dup_compat_types_a,
    pub b: core_reloc_nesting___err_missing_container_b,
}

/* middle b container is referenced through pointer instead of being embedded */
#[repr(C)]
pub struct core_reloc_nesting___err_nonstruct_container_b_inner {
    pub b: c_int,
}

#[repr(C)]
pub struct core_reloc_nesting___err_nonstruct_container_b {
    pub b: *mut core_reloc_nesting___err_nonstruct_container_b_inner,
}

#[repr(C)]
pub struct core_reloc_nesting___err_nonstruct_container {
    pub a: core_reloc_nesting___dup_compat_types_a,
    pub b: core_reloc_nesting___err_nonstruct_container_b,
}

/* middle b container is an array of structs instead of plain struct */
#[repr(C)]
pub struct core_reloc_nesting___err_array_container_b {
    pub b: [core_reloc_nesting___dup_compat_types_b_b; 1],
}

#[repr(C)]
pub struct core_reloc_nesting___err_array_container {
    pub a: core_reloc_nesting___dup_compat_types_a,
    pub b: core_reloc_nesting___err_array_container_b,
}

/* two flavors of same struct with incompatible layout for b.b.b */
#[repr(C)]
pub struct core_reloc_nesting___err_dup_incompat_types__1 {
    pub a: core_reloc_nesting___dup_compat_types_a,
    pub b: core_reloc_nesting___dup_compat_types_b,
}

#[repr(C)]
pub struct core_reloc_nesting___err_dup_incompat_types__2 {
    pub a: core_reloc_nesting___dup_compat_types_a,
    pub __extra_padding: c_int,
    pub b: core_reloc_nesting___dup_compat_types_b,
}

/* two flavors of same struct having one of a.a.a and b.b.b, but not both */
#[repr(C)]
pub struct core_reloc_nesting___err_partial_match_dups__a {
    pub a: core_reloc_nesting___dup_compat_types_a,
}

#[repr(C)]
pub struct core_reloc_nesting___err_partial_match_dups__b {
    pub b: core_reloc_nesting___dup_compat_types_b,
}

#[repr(C)]
pub struct core_reloc_nesting___err_too_deep {
    pub a: core_reloc_nesting___dup_compat_types_a,
    /* 65 levels of nestedness for b.b.b; anonymous C levels are represented by this terminal field. */
    pub b: core_reloc_nesting___dup_compat_types_b,
}

/*
 * ARRAYS
 */
#[repr(C)]
pub struct core_reloc_arrays_output {
    pub a2: c_int,
    pub a3: c_int,
    pub b123: c_char,
    pub c1c: c_int,
    pub d00d: c_int,
    pub f10c: c_int,
}

#[repr(C)]
pub struct core_reloc_arrays_substruct {
    pub c: c_int,
    pub d: c_int,
}

#[repr(C)]
pub struct core_reloc_arrays {
    pub a: [c_int; 5],
    pub b: [[[c_char; 4]; 3]; 2],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [[core_reloc_arrays_substruct; 2]; 1],
    pub f: [[core_reloc_arrays_substruct; 2]; 0],
}

/* bigger array dimensions */
#[repr(C)]
pub struct core_reloc_arrays___diff_arr_dim {
    pub a: [c_int; 7],
    pub b: [[[c_char; 5]; 4]; 3],
    pub c: [core_reloc_arrays_substruct; 4],
    pub d: [[core_reloc_arrays_substruct; 3]; 2],
    pub f: [[core_reloc_arrays_substruct; 3]; 1],
}

#[repr(C)]
pub struct core_reloc_arrays___diff_arr_val_sz_elem_c {
    pub __padding1: c_int,
    pub c: c_int,
    pub __padding2: c_int,
}

#[repr(C)]
pub struct core_reloc_arrays___diff_arr_val_sz_elem_d {
    pub __padding1: c_int,
    pub d: c_int,
    pub __padding2: c_int,
}

/* different size of array's value (struct) */
#[repr(C)]
pub struct core_reloc_arrays___diff_arr_val_sz {
    pub a: [c_int; 5],
    pub b: [[[c_char; 4]; 3]; 2],
    pub c: [core_reloc_arrays___diff_arr_val_sz_elem_c; 3],
    pub d: [[core_reloc_arrays___diff_arr_val_sz_elem_d; 2]; 1],
    pub f: [[core_reloc_arrays___diff_arr_val_sz_elem_c; 2]; 0],
}

#[repr(C)]
pub struct core_reloc_arrays___equiv_zero_sz_arr {
    pub a: [c_int; 5],
    pub b: [[[c_char; 4]; 3]; 2],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [[core_reloc_arrays_substruct; 2]; 1],
    /* equivalent to flexible array */
    pub f: [[core_reloc_arrays_substruct; 2]; 0],
}

#[repr(C)]
pub struct core_reloc_arrays___fixed_arr {
    pub a: [c_int; 5],
    pub b: [[[c_char; 4]; 3]; 2],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [[core_reloc_arrays_substruct; 2]; 1],
    /* not a flexible array anymore, but within access bounds */
    pub f: [[core_reloc_arrays_substruct; 2]; 1],
}

#[repr(C)]
pub struct core_reloc_arrays___err_too_small {
    pub a: [c_int; 2],
    pub b: [[[c_char; 4]; 3]; 2],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [[core_reloc_arrays_substruct; 2]; 1],
    pub f: [[core_reloc_arrays_substruct; 2]; 0],
}

#[repr(C)]
pub struct core_reloc_arrays___err_too_shallow {
    pub a: [c_int; 5],
    pub b: [[c_char; 3]; 2],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [[core_reloc_arrays_substruct; 2]; 1],
    pub f: [[core_reloc_arrays_substruct; 2]; 0],
}

#[repr(C)]
pub struct core_reloc_arrays___err_non_array {
    pub a: c_int,
    pub b: [[[c_char; 4]; 3]; 2],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [[core_reloc_arrays_substruct; 2]; 1],
    pub f: [[core_reloc_arrays_substruct; 2]; 0],
}

#[repr(C)]
pub struct core_reloc_arrays___err_wrong_val_type {
    pub a: [c_int; 5],
    pub b: [[[c_char; 4]; 3]; 2],
    pub c: [c_int; 3],
    pub d: [[core_reloc_arrays_substruct; 2]; 1],
    pub f: [[core_reloc_arrays_substruct; 2]; 0],
}

#[repr(C)]
pub struct core_reloc_arrays___err_bad_zero_sz_arr {
    /* zero-sized array, but not at the end */
    pub f: [[core_reloc_arrays_substruct; 2]; 0],
    pub a: [c_int; 5],
    pub b: [[[c_char; 4]; 3]; 2],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [[core_reloc_arrays_substruct; 2]; 1],
}

#[repr(C)]
pub struct core_reloc_arrays___err_bad_signed_arr_elem_sz {
    /* int -> short (signed!): not supported case */
    pub a: [i16; 5],
    pub b: [[[c_char; 4]; 3]; 2],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [[core_reloc_arrays_substruct; 2]; 1],
    pub f: [[core_reloc_arrays_substruct; 2]; 0],
}

/*
 * PRIMITIVES
 */
#[repr(C)]
pub enum core_reloc_primitives_enum {
    A = 0,
    B = 1,
}

#[repr(C, align(8))]
pub struct Aligned<T>(pub T);

pub type core_reloc_func = Option<unsafe extern "C" fn(*const c_char) -> c_int>;

#[repr(C)]
pub struct core_reloc_primitives {
    pub a: c_char,
    pub b: c_int,
    pub c: core_reloc_primitives_enum,
    pub d: Aligned<*mut c_void>,
    pub f: Aligned<core_reloc_func>,
}

#[repr(C)]
pub enum core_reloc_primitives___diff_enum_def_c {
    X = 100,
    Y = 200,
}

#[repr(C)]
pub struct core_reloc_primitives___diff_enum_def {
    pub a: c_char,
    pub b: c_int,
    pub d: Aligned<*mut c_void>,
    pub f: Aligned<core_reloc_func>,
    pub c: Aligned<core_reloc_primitives___diff_enum_def_c>,
}

#[repr(C)]
pub struct core_reloc_primitives___diff_func_proto {
    pub f: Aligned<Option<unsafe extern "C" fn(c_int)>>,
    pub d: Aligned<*mut c_void>,
    pub c: Aligned<core_reloc_primitives_enum>,
    pub b: c_int,
    pub a: c_char,
}

#[repr(C)]
pub struct core_reloc_primitives___diff_ptr_type {
    pub d: Aligned<*const c_char>,
    pub a: Aligned<c_char>,
    pub b: c_int,
    pub c: core_reloc_primitives_enum,
    pub f: Aligned<core_reloc_func>,
}

#[repr(C)]
pub struct core_reloc_primitives___err_non_enum {
    pub a: [c_char; 1],
    pub b: c_int,
    pub c: c_int,
    pub d: Aligned<*mut c_void>,
    pub f: Aligned<core_reloc_func>,
}

#[repr(C)]
pub struct core_reloc_primitives___err_non_int {
    pub a: [c_char; 1],
    pub b: Aligned<*mut c_int>,
    pub c: Aligned<core_reloc_primitives_enum>,
    pub d: Aligned<*mut c_void>,
    pub f: Aligned<core_reloc_func>,
}

#[repr(C)]
pub struct core_reloc_primitives___err_non_ptr {
    pub a: [c_char; 1],
    pub b: c_int,
    pub c: core_reloc_primitives_enum,
    pub d: c_int,
    pub f: Aligned<core_reloc_func>,
}

/*
 * MODS
 */
#[repr(C)]
pub struct core_reloc_mods_output {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
    pub d: c_int,
    pub e: c_int,
    pub f: c_int,
    pub g: c_int,
    pub h: c_int,
}

pub type int_t = c_int;
pub type char_ptr_t = Aligned<*const c_char>;
pub type arr_t = [c_int; 7];

#[repr(C)]
pub struct core_reloc_mods_substruct {
    pub x: c_int,
    pub y: c_int,
}

pub type core_reloc_mods_substruct_t = core_reloc_mods_substruct;

#[repr(C)]
pub struct core_reloc_mods {
    pub a: c_int,
    pub b: int_t,
    pub c: Aligned<*mut c_char>,
    pub d: char_ptr_t,
    pub e: Aligned<[c_int; 3]>,
    pub f: arr_t,
    pub g: core_reloc_mods_substruct,
    pub h: core_reloc_mods_substruct_t,
}

#[repr(C)]
pub struct core_reloc_mods___mod_swap_h {
    pub y: c_int,
    pub x: c_int,
}

/* a/b, c/d, e/f, and g/h pairs are swapped */
#[repr(C)]
pub struct core_reloc_mods___mod_swap {
    pub b: c_int,
    pub a: int_t,
    pub d: Aligned<*mut c_char>,
    pub c: char_ptr_t,
    pub f: Aligned<[c_int; 3]>,
    pub e: arr_t,
    pub h: core_reloc_mods___mod_swap_h,
    pub g: core_reloc_mods_substruct_t,
}

pub type int1_t = c_int;
pub type int2_t = int1_t;
pub type int3_t = int2_t;
pub type arr1_t = [c_int; 5];
pub type arr2_t = arr1_t;
pub type arr3_t = arr2_t;
pub type arr4_t = arr3_t;
pub type fancy_char_ptr_t = Aligned<*const c_char>;
pub type core_reloc_mods_substruct_tt = core_reloc_mods_substruct_t;

/* we need more typedefs */
#[repr(C)]
pub struct core_reloc_mods___typedefs {
    pub g: core_reloc_mods_substruct_tt,
    pub h: core_reloc_mods_substruct_tt,
    pub f: arr4_t,
    pub e: arr4_t,
    pub d: fancy_char_ptr_t,
    pub c: fancy_char_ptr_t,
    pub b: Aligned<int3_t>,
    pub a: int3_t,
}

/*
 * PTR_AS_ARR
 */
#[repr(C)]
pub struct core_reloc_ptr_as_arr {
    pub a: c_int,
}

#[repr(C)]
pub struct core_reloc_ptr_as_arr___diff_sz {
    /* int :32; padding */
    pub __bitfield_padding: c_int,
    pub __some_more_padding: c_char,
    pub a: c_int,
}

/*
 * INTS
 */
#[repr(C)]
pub struct core_reloc_ints {
    pub u8_field: u8,
    pub s8_field: i8,
    pub u16_field: u16,
    pub s16_field: i16,
    pub u32_field: u32,
    pub s32_field: i32,
    pub u64_field: u64,
    pub s64_field: i64,
}

/* signed/unsigned types swap */
#[repr(C)]
pub struct core_reloc_ints___reverse_sign {
    pub u8_field: i8,
    pub s8_field: u8,
    pub u16_field: i16,
    pub s16_field: u16,
    pub u32_field: i32,
    pub s32_field: u32,
    pub u64_field: i64,
    pub s64_field: u64,
}

#[repr(C)]
pub struct core_reloc_ints___bool {
    pub u8_field: bool,
    pub s8_field: i8,
    pub u16_field: u16,
    pub s16_field: i16,
    pub u32_field: u32,
    pub s32_field: i32,
    pub u64_field: u64,
    pub s64_field: i64,
}

/*
 * MISC
 */
#[repr(C)]
pub struct core_reloc_misc_output {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
}

#[repr(C)]
pub struct core_reloc_misc___a {
    pub a1: c_int,
    pub a2: c_int,
}

#[repr(C)]
pub struct core_reloc_misc___b {
    pub b1: c_int,
    pub b2: c_int,
}

/* this one extends core_reloc_misc_extensible struct from BPF prog */
#[repr(C)]
pub struct core_reloc_misc_extensible {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
    pub d: c_int,
}

/*
 * FIELD EXISTENCE
 */
#[repr(C)]
pub struct core_reloc_existence_output {
    pub a_exists: c_int,
    pub a_value: c_int,
    pub b_exists: c_int,
    pub b_value: c_int,
    pub c_exists: c_int,
    pub c_value: c_int,
    pub arr_exists: c_int,
    pub arr_value: c_int,
    pub s_exists: c_int,
    pub s_value: c_int,
}

#[repr(C)]
pub struct core_reloc_existence_s {
    pub x: c_int,
}

#[repr(C)]
pub struct core_reloc_existence {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
    pub arr: [c_int; 1],
    pub s: core_reloc_existence_s,
}

#[repr(C)]
pub struct core_reloc_existence___minimal {
    pub a: c_int,
}

#[repr(C)]
pub struct core_reloc_existence___wrong_field_defs_c {
    pub x: c_int,
}

#[repr(C)]
pub struct core_reloc_existence___wrong_field_defs {
    pub a: *mut c_void,
    pub b: [c_int; 1],
    pub c: core_reloc_existence___wrong_field_defs_c,
    pub arr: c_int,
    pub s: c_int,
}

/*
 * BITFIELDS
 */
/* bitfield read results, all as plain integers */
#[repr(C)]
pub struct core_reloc_bitfields_output {
    pub ub1: i64,
    pub ub2: i64,
    pub ub7: i64,
    pub sb4: i64,
    pub sb20: i64,
    pub u32: i64,
    pub s32: i64,
}

/* Rust has no native C-compatible bitfield declarations; storage fields preserve the source order and comments record widths. */
#[repr(C)]
pub struct core_reloc_bitfields {
    pub _bitfield_storage_0: u8,  /* uint8_t ub1:1; uint8_t ub2:2; */
    pub _bitfield_storage_1: u32, /* uint32_t ub7:7; */
    pub _bitfield_storage_2: i8,  /* int8_t sb4:4; */
    pub _bitfield_storage_3: i32, /* int32_t sb20:20; */
    pub u32: u32,
    pub s32: i32,
}

#[repr(C)]
pub struct core_reloc_bitfields___bit_sz_change {
    pub _bitfield_storage_0: u16, /* ub1:3 */
    pub _bitfield_storage_1: u32, /* ub2:20 */
    pub _bitfield_storage_2: u8,  /* ub7:1 */
    pub _bitfield_storage_3: i8,  /* sb4:1 */
    pub _bitfield_storage_4: i32, /* sb20:30 */
    pub u32: u16,
    pub s32: Aligned<i64>,
}

#[repr(C)]
pub struct core_reloc_bitfields___bitfield_vs_int {
    pub ub1: u64,
    pub ub2: u8,
    pub ub7: Aligned<i64>,
    pub sb4: Aligned<i64>,
    pub sb20: Aligned<u64>,
    pub _u32_bitfield_storage: i32, /* int32_t u32:20; */
    pub _s32_bitfield_storage: Aligned<u64>, /* uint64_t s32:60; */
}

#[repr(C, packed)]
pub struct core_reloc_bitfields___just_big_enough {
    pub _bitfield_storage_0: u64, /* ub1:4; ub2:60 packed tightly */
    pub ub7: u32,
    pub sb4: u32,
    pub sb20: u32,
    pub u32: u32,
    pub s32: u32,
}

#[repr(C, packed)]
pub struct core_reloc_bitfields___err_too_big_bitfield {
    pub _bitfield_storage_0: u64, /* ub1:4; ub2:61 packed tightly */
    pub _bitfield_overflow_marker: u8,
    pub ub7: u32,
    pub sb4: u32,
    pub sb20: u32,
    pub u32: u32,
    pub s32: u32,
}

/*
 * SIZE
 */
#[repr(C)]
pub struct core_reloc_size_output {
    pub int_sz: c_int,
    pub int_off: c_int,
    pub struct_sz: c_int,
    pub struct_off: c_int,
    pub union_sz: c_int,
    pub union_off: c_int,
    pub arr_sz: c_int,
    pub arr_off: c_int,
    pub arr_elem_sz: c_int,
    pub arr_elem_off: c_int,
    pub ptr_sz: c_int,
    pub ptr_off: c_int,
    pub enum_sz: c_int,
    pub enum_off: c_int,
    pub float_sz: c_int,
    pub float_off: c_int,
}

#[repr(C)]
pub struct core_reloc_size_struct_field {
    pub x: c_int,
}

#[repr(C)]
pub union core_reloc_size_union_field {
    pub x: c_int,
}

#[repr(C)]
pub enum core_reloc_size_enum_field {
    VALUE = 123,
}

#[repr(C)]
pub struct core_reloc_size {
    pub int_field: c_int,
    pub struct_field: core_reloc_size_struct_field,
    pub union_field: core_reloc_size_union_field,
    pub arr_field: [c_int; 4],
    pub ptr_field: *mut c_void,
    pub enum_field: core_reloc_size_enum_field,
    pub float_field: f32,
}

#[repr(C)]
pub struct core_reloc_size___diff_sz_struct_field {
    pub x: c_int,
    pub y: c_int,
    pub z: c_int,
}

#[repr(C)]
pub union core_reloc_size___diff_sz_union_field {
    pub x: c_int,
    pub bla: [c_char; 123],
}

#[repr(C)]
pub enum core_reloc_size___diff_sz_enum_field {
    OTHER_VALUE = -1,
}

#[repr(C)]
pub struct core_reloc_size___diff_sz {
    pub int_field: u64,
    pub struct_field: core_reloc_size___diff_sz_struct_field,
    pub union_field: core_reloc_size___diff_sz_union_field,
    pub arr_field: [c_char; 10],
    pub ptr_field: *mut c_void,
    pub enum_field: core_reloc_size___diff_sz_enum_field,
    pub float_field: f64,
}

#[repr(C)]
pub enum core_reloc_size___diff_offs_enum_field {
    YET_OTHER_VALUE = 123,
}

#[repr(C)]
pub struct core_reloc_size___diff_offs {
    pub float_field: f32,
    pub enum_field: core_reloc_size___diff_offs_enum_field,
    pub ptr_field: *mut c_void,
    pub arr_field: [c_int; 4],
    pub union_field: core_reloc_size_union_field,
    pub struct_field: core_reloc_size_struct_field,
    pub int_field: c_int,
}

#[repr(C)]
pub enum core_reloc_size___err_ambiguous1_enum_field {
    VALUE___1 = 123,
}

#[repr(C)]
pub struct core_reloc_size___err_ambiguous1 {
    pub int_field: c_int,
    pub struct_field: core_reloc_size_struct_field,
    pub union_field: core_reloc_size_union_field,
    pub arr_field: [c_int; 4],
    pub ptr_field: *mut c_void,
    pub enum_field: core_reloc_size___err_ambiguous1_enum_field,
    pub float_field: f32,
}

#[repr(C)]
pub enum core_reloc_size___err_ambiguous2_enum_field {
    VALUE___2 = 123,
}

#[repr(C)]
pub struct core_reloc_size___err_ambiguous2 {
    pub int_field: c_char,
    pub struct_field: core_reloc_size_struct_field,
    pub union_field: core_reloc_size_union_field,
    pub arr_field: [c_int; 4],
    pub ptr_field: *mut c_void,
    pub enum_field: core_reloc_size___err_ambiguous2_enum_field,
    pub float_field: f32,
}

/*
 * TYPE EXISTENCE, MATCH & SIZE
 */
#[repr(C)]
pub struct core_reloc_type_based_output {
    pub struct_exists: bool,
    pub complex_struct_exists: bool,
    pub union_exists: bool,
    pub enum_exists: bool,
    pub typedef_named_struct_exists: bool,
    pub typedef_anon_struct_exists: bool,
    pub typedef_struct_ptr_exists: bool,
    pub typedef_int_exists: bool,
    pub typedef_enum_exists: bool,
    pub typedef_void_ptr_exists: bool,
    pub typedef_restrict_ptr_exists: bool,
    pub typedef_func_proto_exists: bool,
    pub typedef_arr_exists: bool,
    pub struct_matches: bool,
    pub complex_struct_matches: bool,
    pub union_matches: bool,
    pub enum_matches: bool,
    pub typedef_named_struct_matches: bool,
    pub typedef_anon_struct_matches: bool,
    pub typedef_struct_ptr_matches: bool,
    pub typedef_int_matches: bool,
    pub typedef_enum_matches: bool,
    pub typedef_void_ptr_matches: bool,
    pub typedef_restrict_ptr_matches: bool,
    pub typedef_func_proto_matches: bool,
    pub typedef_arr_matches: bool,
    pub struct_sz: c_int,
    pub union_sz: c_int,
    pub enum_sz: c_int,
    pub typedef_named_struct_sz: c_int,
    pub typedef_anon_struct_sz: c_int,
    pub typedef_struct_ptr_sz: c_int,
    pub typedef_int_sz: c_int,
    pub typedef_enum_sz: c_int,
    pub typedef_void_ptr_sz: c_int,
    pub typedef_func_proto_sz: c_int,
    pub typedef_arr_sz: c_int,
}

#[repr(C)]
pub struct a_struct { pub x: c_int }

#[repr(C)]
pub union a_complex_struct_x {
    pub a: *mut a_struct,
    pub b: *mut c_void,
}

#[repr(C)]
pub struct a_complex_struct {
    pub x: a_complex_struct_x,
    pub y: c_long,
}

#[repr(C)]
pub union a_union {
    pub y: c_int,
    pub z: c_int,
}

pub type named_struct_typedef = a_struct;

#[repr(C)]
pub struct anon_struct_typedef {
    pub x: c_int,
    pub y: c_int,
    pub z: c_int,
}

#[repr(C)]
pub struct struct_ptr_typedef_target {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
}
pub type struct_ptr_typedef = *mut struct_ptr_typedef_target;

#[repr(C)]
pub enum an_enum {
    AN_ENUM_VAL1 = 1,
    AN_ENUM_VAL2 = 2,
    AN_ENUM_VAL3 = 3,
}

pub type int_typedef = c_int;

#[repr(C)]
pub enum enum_typedef {
    TYPEDEF_ENUM_VAL1 = 0,
    TYPEDEF_ENUM_VAL2 = 1,
}

pub type void_ptr_typedef = *mut c_void;
pub type restrict_ptr_typedef = *mut c_int;
pub type func_proto_typedef = Option<unsafe extern "C" fn(c_long) -> c_int>;
pub type arr_typedef = [c_char; 20];

#[repr(C)]
pub struct core_reloc_type_based {
    pub f1: a_struct,
    pub f2: a_complex_struct,
    pub f3: a_union,
    pub f4: an_enum,
    pub f5: named_struct_typedef,
    pub f6: anon_struct_typedef,
    pub f7: struct_ptr_typedef,
    pub f8: int_typedef,
    pub f9: enum_typedef,
    pub f10: void_ptr_typedef,
    pub f11: restrict_ptr_typedef,
    pub f12: func_proto_typedef,
    pub f13: arr_typedef,
}

/* no types in target */
#[repr(C)]
pub struct core_reloc_type_based___all_missing {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct a_struct___diff { pub x: c_int, pub a: c_int }
pub enum a_struct___forward {}

#[repr(C)]
pub union a_complex_struct___diff_x {
    pub a: *mut a_struct___forward,
    pub b: *mut c_void,
}

#[repr(C)]
pub struct a_complex_struct___diff {
    pub x: a_complex_struct___diff_x,
    pub y: c_long,
}

#[repr(C)]
pub union a_union___diff { pub z: c_int, pub y: c_int }
pub type named_struct_typedef___diff = a_struct___diff;

#[repr(C)]
pub struct anon_struct_typedef___diff { pub z: c_int, pub x: c_int, pub y: c_int }

#[repr(C)]
pub struct struct_ptr_typedef___diff_target { pub c: c_int, pub b: c_int, pub a: c_int }
pub type struct_ptr_typedef___diff = *mut struct_ptr_typedef___diff_target;

#[repr(C)]
pub enum an_enum___diff {
    AN_ENUM_VAL2___diff = 0,
    AN_ENUM_VAL1___diff = 42,
    AN_ENUM_VAL3___diff = 1,
}

pub type int_typedef___diff = c_uint;
use core::ffi::c_uint;

#[repr(C)]
pub enum enum_typedef___diff {
    TYPEDEF_ENUM_VAL2___diff = 0,
    TYPEDEF_ENUM_VAL1___diff = 50,
}

pub type void_ptr_typedef___diff = *const c_void;
pub type func_proto_typedef___diff = Option<unsafe extern "C" fn(c_long) -> int_typedef___diff>;
pub type arr_typedef___diff = [c_char; 3];

#[repr(C)]
pub struct core_reloc_type_based___diff {
    pub f1: a_struct___diff,
    pub f2: a_complex_struct___diff,
    pub f3: a_union___diff,
    pub f4: an_enum___diff,
    pub f5: named_struct_typedef___diff,
    pub f6: anon_struct_typedef___diff,
    pub f7: struct_ptr_typedef___diff,
    pub f8: int_typedef___diff,
    pub f9: enum_typedef___diff,
    pub f10: void_ptr_typedef___diff,
    pub f11: func_proto_typedef___diff,
    pub f12: arr_typedef___diff,
}

#[repr(C)]
pub struct a_struct___diff_sz { pub x: c_long, pub y: c_int, pub z: c_char }

#[repr(C)]
pub union a_union___diff_sz { pub yy: c_char, pub zz: c_char }
pub type named_struct_typedef___diff_sz = a_struct___diff_sz;

#[repr(C)]
pub struct anon_struct_typedef___diff_sz { pub xx: c_long, pub yy: c_long, pub zzz: c_long }

#[repr(C)]
pub struct struct_ptr_typedef___diff_sz_target {
    pub aa: [c_char; 1],
    pub bb: [c_char; 2],
    pub cc: [c_char; 3],
}
pub type struct_ptr_typedef___diff_sz = *mut struct_ptr_typedef___diff_sz_target;

#[repr(C)]
pub enum an_enum___diff_sz {
    AN_ENUM_VAL1___diff_sz = 0x123412341234,
    AN_ENUM_VAL2___diff_sz = 2,
}

pub type int_typedef___diff_sz = c_ulong;
use core::ffi::c_ulong;
pub type enum_typedef___diff_sz = an_enum___diff_sz;
pub type void_ptr_typedef___diff_sz = *const c_void;
pub type func_proto_typedef___diff_sz = Option<unsafe extern "C" fn(c_char) -> int_typedef___diff_sz>;
pub type arr_typedef___diff_sz = [c_int; 2];

#[repr(C)]
pub struct core_reloc_type_based___diff_sz {
    pub f1: a_struct___diff_sz,
    pub f2: a_union___diff_sz,
    pub f3: an_enum___diff_sz,
    pub f4: named_struct_typedef___diff_sz,
    pub f5: anon_struct_typedef___diff_sz,
    pub f6: struct_ptr_typedef___diff_sz,
    pub f7: int_typedef___diff_sz,
    pub f8: enum_typedef___diff_sz,
    pub f9: void_ptr_typedef___diff_sz,
    pub f10: func_proto_typedef___diff_sz,
    pub f11: arr_typedef___diff_sz,
}

/* incompatibilities between target and local types */
#[repr(C)]
pub union a_struct___incompat { pub x: c_int }

#[repr(C)]
pub struct a_union___incompat { pub y: c_int, pub z: c_int }

pub type named_struct_typedef___incompat = a_struct___incompat;
pub type anon_struct_typedef___incompat = *mut c_void;

#[repr(C)]
pub struct struct_ptr_typedef___incompat_target { pub a: c_int, pub b: c_int, pub c: c_int }
pub type struct_ptr_typedef___incompat = *mut *mut struct_ptr_typedef___incompat_target;

#[repr(C)]
pub struct int_typedef___incompat { pub x: c_int }
pub type enum_typedef___incompat = Option<unsafe extern "C" fn() -> c_int>;
pub type void_ptr_typedef___incompat = *mut c_char;
pub type func_proto_typedef___incompat = Option<unsafe extern "C" fn(c_long)>;
pub type arr_typedef___incompat = [[c_int; 2]; 20];

#[repr(C)]
pub struct core_reloc_type_based___incompat {
    pub f1: a_struct___incompat,
    pub f2: a_union___incompat,
    pub f3: an_enum,
    pub f4: named_struct_typedef___incompat,
    pub f5: anon_struct_typedef___incompat,
    pub f6: struct_ptr_typedef___incompat,
    pub f7: int_typedef___incompat,
    pub f8: enum_typedef___incompat,
    pub f9: void_ptr_typedef___incompat,
    pub f10: func_proto_typedef___incompat,
    pub f11: arr_typedef___incompat,
}

pub type func_proto_typedef___fn_wrong_ret1 = Option<unsafe extern "C" fn(c_long)>;
pub type func_proto_typedef___fn_wrong_ret2 = Option<unsafe extern "C" fn(c_long) -> *mut c_int>;
#[repr(C)]
pub struct int_struct_typedef { pub x: c_int }
pub type func_proto_typedef___fn_wrong_ret3 = Option<unsafe extern "C" fn(c_long) -> int_struct_typedef>;
pub type func_proto_typedef___fn_wrong_arg = Option<unsafe extern "C" fn(*mut c_void) -> c_int>;
pub type func_proto_typedef___fn_wrong_arg_cnt1 = Option<unsafe extern "C" fn(c_long, c_long) -> c_int>;
pub type func_proto_typedef___fn_wrong_arg_cnt2 = Option<unsafe extern "C" fn() -> c_int>;

#[repr(C)]
pub struct core_reloc_type_based___fn_wrong_args {
    pub f1: a_struct,
    pub f2: func_proto_typedef___fn_wrong_ret1,
    pub f3: func_proto_typedef___fn_wrong_ret2,
    pub f4: func_proto_typedef___fn_wrong_ret3,
    pub f5: func_proto_typedef___fn_wrong_arg,
    pub f6: func_proto_typedef___fn_wrong_arg_cnt1,
    pub f7: func_proto_typedef___fn_wrong_arg_cnt2,
}

/*
 * TYPE ID MAPPING (LOCAL AND TARGET)
 */
#[repr(C)]
pub struct core_reloc_type_id_output {
    pub local_anon_struct: c_int,
    pub local_anon_union: c_int,
    pub local_anon_enum: c_int,
    pub local_anon_func_proto_ptr: c_int,
    pub local_anon_void_ptr: c_int,
    pub local_anon_arr: c_int,
    pub local_struct: c_int,
    pub local_union: c_int,
    pub local_enum: c_int,
    pub local_int: c_int,
    pub local_struct_typedef: c_int,
    pub local_func_proto_typedef: c_int,
    pub local_arr_typedef: c_int,
    pub targ_struct: c_int,
    pub targ_union: c_int,
    pub targ_enum: c_int,
    pub targ_int: c_int,
    pub targ_struct_typedef: c_int,
    pub targ_func_proto_typedef: c_int,
    pub targ_arr_typedef: c_int,
}

#[repr(C)]
pub struct core_reloc_type_id {
    pub f1: a_struct,
    pub f2: a_union,
    pub f3: an_enum,
    pub f4: named_struct_typedef,
    pub f5: func_proto_typedef,
    pub f6: arr_typedef,
}

#[repr(C)]
pub struct core_reloc_type_id___missing_targets {
    _unused: [u8; 0],
}

/*
 * ENUMERATOR VALUE EXISTENCE AND VALUE RELOCATION
 */
#[repr(C)]
pub struct core_reloc_enumval_output {
    pub named_val1_exists: bool,
    pub named_val2_exists: bool,
    pub named_val3_exists: bool,
    pub anon_val1_exists: bool,
    pub anon_val2_exists: bool,
    pub anon_val3_exists: bool,
    pub named_val1: c_int,
    pub named_val2: c_int,
    pub anon_val1: c_int,
    pub anon_val2: c_int,
}

#[repr(C)]
pub struct core_reloc_enum64val_output {
    pub unsigned_val1_exists: bool,
    pub unsigned_val2_exists: bool,
    pub unsigned_val3_exists: bool,
    pub signed_val1_exists: bool,
    pub signed_val2_exists: bool,
    pub signed_val3_exists: bool,
    pub unsigned_val1: c_long,
    pub unsigned_val2: c_long,
    pub signed_val1: c_long,
    pub signed_val2: c_long,
}

#[repr(C)]
pub enum named_enum { NAMED_ENUM_VAL1 = 1, NAMED_ENUM_VAL2 = 2, NAMED_ENUM_VAL3 = 3 }

#[repr(C)]
pub enum anon_enum { ANON_ENUM_VAL1 = 0x10, ANON_ENUM_VAL2 = 0x20, ANON_ENUM_VAL3 = 0x30 }

#[repr(C)]
pub struct core_reloc_enumval {
    pub f1: named_enum,
    pub f2: anon_enum,
}

#[repr(u64)]
pub enum named_unsigned_enum64 {
    UNSIGNED_ENUM64_VAL1 = 0x1ffffffff,
    UNSIGNED_ENUM64_VAL2 = 0x2,
    UNSIGNED_ENUM64_VAL3 = 0x3ffffffff,
}

#[repr(i64)]
pub enum named_signed_enum64 {
    SIGNED_ENUM64_VAL1 = 0x1ffffffff,
    SIGNED_ENUM64_VAL2 = -2,
    SIGNED_ENUM64_VAL3 = 0x3ffffffff,
}

#[repr(C)]
pub struct core_reloc_enum64val {
    pub f1: named_unsigned_enum64,
    pub f2: named_signed_enum64,
}

#[repr(C)]
pub enum named_enum___diff {
    NAMED_ENUM_VAL1___diff = 101,
    NAMED_ENUM_VAL2___diff = 202,
    NAMED_ENUM_VAL3___diff = 303,
}

#[repr(C)]
pub enum anon_enum___diff {
    ANON_ENUM_VAL1___diff = 0x11,
    ANON_ENUM_VAL2___diff = 0x22,
    ANON_ENUM_VAL3___diff = 0x33,
}

#[repr(C)]
pub struct core_reloc_enumval___diff { pub f1: named_enum___diff, pub f2: anon_enum___diff }

#[repr(u64)]
pub enum named_unsigned_enum64___diff {
    UNSIGNED_ENUM64_VAL1___diff = 0x101ffffffff,
    UNSIGNED_ENUM64_VAL2___diff = 0x202ffffffff,
    UNSIGNED_ENUM64_VAL3___diff = 0x303ffffffff,
}

#[repr(i64)]
pub enum named_signed_enum64___diff {
    SIGNED_ENUM64_VAL1___diff = -101,
    SIGNED_ENUM64_VAL2___diff = -202,
    SIGNED_ENUM64_VAL3___diff = -303,
}

#[repr(C)]
pub struct core_reloc_enum64val___diff {
    pub f1: named_unsigned_enum64___diff,
    pub f2: named_signed_enum64___diff,
}

#[repr(C)]
pub enum named_enum___val3_missing {
    NAMED_ENUM_VAL1___val3_missing = 111,
    NAMED_ENUM_VAL2___val3_missing = 222,
}

#[repr(C)]
pub enum anon_enum___val3_missing {
    ANON_ENUM_VAL1___val3_missing = 0x111,
    ANON_ENUM_VAL2___val3_missing = 0x222,
}

#[repr(C)]
pub struct core_reloc_enumval___val3_missing {
    pub f1: named_enum___val3_missing,
    pub f2: anon_enum___val3_missing,
}

#[repr(u64)]
pub enum named_unsigned_enum64___val3_missing {
    UNSIGNED_ENUM64_VAL1___val3_missing = 0x111ffffffff,
    UNSIGNED_ENUM64_VAL2___val3_missing = 0x222,
}

#[repr(i64)]
pub enum named_signed_enum64___val3_missing {
    SIGNED_ENUM64_VAL1___val3_missing = 0x111ffffffff,
    SIGNED_ENUM64_VAL2___val3_missing = -222,
}

#[repr(C)]
pub struct core_reloc_enum64val___val3_missing {
    pub f1: named_unsigned_enum64___val3_missing,
    pub f2: named_signed_enum64___val3_missing,
}

#[repr(C)]
pub enum named_enum___err_missing {
    NAMED_ENUM_VAL1___err_missing = 1,
    NAMED_ENUM_VAL3___err_missing = 3,
}

#[repr(C)]
pub enum anon_enum___err_missing {
    ANON_ENUM_VAL1___err_missing = 0x111,
    ANON_ENUM_VAL3___err_missing = 0x222,
}

#[repr(C)]
pub struct core_reloc_enumval___err_missing {
    pub f1: named_enum___err_missing,
    pub f2: anon_enum___err_missing,
}

#[repr(u64)]
pub enum named_unsigned_enum64___err_missing {
    UNSIGNED_ENUM64_VAL1___err_missing = 0x1ffffffff,
    UNSIGNED_ENUM64_VAL3___err_missing = 0x3ffffffff,
}

#[repr(i64)]
pub enum named_signed_enum64___err_missing {
    SIGNED_ENUM64_VAL1___err_missing = 0x1ffffffff,
    SIGNED_ENUM64_VAL3___err_missing = -3,
}

#[repr(C)]
pub struct core_reloc_enum64val___err_missing {
    pub f1: named_unsigned_enum64___err_missing,
    pub f2: named_signed_enum64___err_missing,
}
