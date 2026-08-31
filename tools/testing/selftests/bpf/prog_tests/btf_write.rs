// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */
/* Translated from testing/selftests/bpf/prog_tests/btf_write.c. */

use std::ffi::{c_char, c_int, c_uint, c_ulonglong};
use std::ptr;

#[repr(C)]
pub struct btf {
	_opaque: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
	pub name_off: c_uint,
	pub info: c_uint,
	pub size: c_uint,
	pub type_: c_uint,
}

#[repr(C)]
pub struct btf_member {
	pub name_off: c_uint,
	pub type_: c_uint,
	pub offset: c_uint,
}

#[repr(C)]
pub struct btf_enum {
	pub name_off: c_uint,
	pub val: c_int,
}

#[repr(C)]
pub struct btf_enum64 {
	pub name_off: c_uint,
	pub val_lo32: c_uint,
	pub val_hi32: c_uint,
}

#[repr(C)]
pub struct btf_param {
	pub name_off: c_uint,
	pub type_: c_uint,
}

#[repr(C)]
pub struct btf_var_secinfo {
	pub type_: c_uint,
	pub offset: c_uint,
	pub size: c_uint,
}

#[repr(C)]
pub struct btf_array_type {
	pub type_: c_uint,
	pub index_type: c_uint,
	pub nelems: c_uint,
}

#[repr(C)]
pub struct btf_var_type {
	pub linkage: c_uint,
}

#[repr(C)]
pub struct btf_decl_tag_type {
	pub component_idx: c_int,
}

const ENOENT: c_int = 2;

unsafe extern "C" {
	fn btf__find_str(btf: *mut btf, s: *const c_char) -> c_int;
	fn btf__add_str(btf: *mut btf, s: *const c_char) -> c_int;
	fn btf__add_int(btf: *mut btf, name: *const c_char, byte_sz: c_uint, encoding: c_uint) -> c_int;
	fn btf__type_by_id(btf: *mut btf, id: c_uint) -> *const btf_type;
	fn btf__str_by_offset(btf: *mut btf, offset: c_uint) -> *const c_char;
	fn btf_type_raw_dump(btf: *mut btf, id: c_uint) -> *const c_char;
	fn btf__add_ptr(btf: *mut btf, ref_type_id: c_uint) -> c_int;
	fn btf__add_const(btf: *mut btf, ref_type_id: c_uint) -> c_int;
	fn btf__add_volatile(btf: *mut btf, ref_type_id: c_uint) -> c_int;
	fn btf__add_restrict(btf: *mut btf, ref_type_id: c_uint) -> c_int;
	fn btf__add_array(btf: *mut btf, index_type_id: c_uint, elem_type_id: c_uint, nr_elems: c_uint) -> c_int;
	fn btf__add_field(
		btf: *mut btf,
		name: *const c_char,
		type_id: c_uint,
		bit_offset: c_uint,
		bitfield_size: c_uint,
	) -> c_int;
	fn btf__add_struct(btf: *mut btf, name: *const c_char, byte_sz: c_uint) -> c_int;
	fn btf__add_union(btf: *mut btf, name: *const c_char, byte_sz: c_uint) -> c_int;
	fn btf__add_enum(btf: *mut btf, name: *const c_char, byte_sz: c_uint) -> c_int;
	fn btf__add_enum_value(btf: *mut btf, name: *const c_char, value: c_int) -> c_int;
	fn btf__add_fwd(btf: *mut btf, name: *const c_char, fwd_kind: c_uint) -> c_int;
	fn btf__add_typedef(btf: *mut btf, name: *const c_char, ref_type_id: c_uint) -> c_int;
	fn btf__add_func(btf: *mut btf, name: *const c_char, linkage: c_uint, proto_type_id: c_uint) -> c_int;
	fn btf__add_func_proto(btf: *mut btf, ret_type_id: c_uint) -> c_int;
	fn btf__add_func_param(btf: *mut btf, name: *const c_char, type_id: c_uint) -> c_int;
	fn btf__add_var(btf: *mut btf, name: *const c_char, linkage: c_uint, type_id: c_uint) -> c_int;
	fn btf__add_datasec(btf: *mut btf, name: *const c_char, byte_sz: c_uint) -> c_int;
	fn btf__add_datasec_var_info(btf: *mut btf, type_id: c_uint, offset: c_uint, byte_sz: c_uint) -> c_int;
	fn btf__add_decl_tag(btf: *mut btf, value: *const c_char, ref_type_id: c_uint, component_idx: c_int) -> c_int;
	fn btf__add_type_tag(btf: *mut btf, value: *const c_char, ref_type_id: c_uint) -> c_int;
	fn btf__add_enum64(btf: *mut btf, name: *const c_char, byte_sz: c_uint, is_signed: bool) -> c_int;
	fn btf__add_enum64_value(btf: *mut btf, name: *const c_char, value: c_ulonglong) -> c_int;
	fn btf__new_empty() -> *mut btf;
	fn btf__new_empty_split(base: *mut btf) -> *mut btf;
	fn btf__free(btf: *mut btf);
	fn btf__add_btf(btf: *mut btf, src_btf: *mut btf) -> c_int;
	fn btf__type_cnt(btf: *mut btf) -> c_uint;
	fn btf__dedup(btf: *mut btf, opts: *mut core::ffi::c_void) -> c_int;
	fn test__start_subtest(name: *const c_char) -> bool;

	fn btf_kind(t: *const btf_type) -> c_uint;
	fn btf_int_encoding(t: *const btf_type) -> c_uint;
	fn btf_int_bits(t: *const btf_type) -> c_uint;
	fn btf_array(t: *const btf_type) -> *const btf_array_type;
	fn btf_members(t: *const btf_type) -> *const btf_member;
	fn btf_member_bit_offset(t: *const btf_type, idx: c_uint) -> c_uint;
	fn btf_member_bitfield_size(t: *const btf_type, idx: c_uint) -> c_uint;
	fn btf_vlen(t: *const btf_type) -> c_uint;
	fn btf_kflag(t: *const btf_type) -> c_uint;
	fn btf_enum(t: *const btf_type) -> *const btf_enum;
	fn btf_params(t: *const btf_type) -> *const btf_param;
	fn btf_var(t: *const btf_type) -> *const btf_var_type;
	fn btf_var_secinfos(t: *const btf_type) -> *const btf_var_secinfo;
	fn btf_decl_tag(t: *const btf_type) -> *const btf_decl_tag_type;
	fn btf_enum64(t: *const btf_type) -> *const btf_enum64;
}

unsafe fn gen_btf(btf: *mut btf) {
	let mut vi: *const btf_var_secinfo;
	let mut t: *const btf_type;
	let mut m: *const btf_member;
	let mut v64: *const btf_enum64;
	let mut v: *const btf_enum;
	let mut p: *const btf_param;
	let mut id: c_int;
	let mut err: c_int;
	let mut str_off: c_int;

	str_off = btf__find_str(btf, c"int".as_ptr());
	ASSERT_EQ!(str_off, -ENOENT, c"int_str_missing_off".as_ptr());

	str_off = btf__add_str(btf, c"int".as_ptr());
	ASSERT_EQ!(str_off, 1, c"int_str_off".as_ptr());

	str_off = btf__find_str(btf, c"int".as_ptr());
	ASSERT_EQ!(str_off, 1, c"int_str_found_off".as_ptr());

	/* BTF_KIND_INT */
	id = btf__add_int(btf, c"int".as_ptr(), 4, BTF_INT_SIGNED);
	ASSERT_EQ!(id, 1, c"int_id".as_ptr());

	t = btf__type_by_id(btf, 1);
	/* should re-use previously added "int" string */
	ASSERT_EQ!((*t).name_off, str_off as c_uint, c"int_name_off".as_ptr());
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"int".as_ptr(), c"int_name".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_INT, c"int_kind".as_ptr());
	ASSERT_EQ!((*t).size, 4, c"int_sz".as_ptr());
	ASSERT_EQ!(btf_int_encoding(t), BTF_INT_SIGNED, c"int_enc".as_ptr());
	ASSERT_EQ!(btf_int_bits(t), 32, c"int_bits".as_ptr());
	ASSERT_STREQ!(
		btf_type_raw_dump(btf, 1),
		c"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED".as_ptr(),
		c"raw_dump".as_ptr()
	);

	/* invalid int size */
	id = btf__add_int(btf, c"bad sz int".as_ptr(), 7, 0);
	ASSERT_ERR!(id, c"int_bad_sz".as_ptr());
	/* invalid encoding */
	id = btf__add_int(btf, c"bad enc int".as_ptr(), 4, 123);
	ASSERT_ERR!(id, c"int_bad_enc".as_ptr());
	/* NULL name */
	id = btf__add_int(btf, ptr::null(), 4, 0);
	ASSERT_ERR!(id, c"int_bad_null_name".as_ptr());
	/* empty name */
	id = btf__add_int(btf, c"".as_ptr(), 4, 0);
	ASSERT_ERR!(id, c"int_bad_empty_name".as_ptr());

	/* PTR/CONST/VOLATILE/RESTRICT */
	id = btf__add_ptr(btf, 1);
	ASSERT_EQ!(id, 2, c"ptr_id".as_ptr());
	t = btf__type_by_id(btf, 2);
	ASSERT_EQ!(btf_kind(t), BTF_KIND_PTR, c"ptr_kind".as_ptr());
	ASSERT_EQ!((*t).type_, 1, c"ptr_type".as_ptr());
	ASSERT_STREQ!(btf_type_raw_dump(btf, 2), c"[2] PTR '(anon)' type_id=1".as_ptr(), c"raw_dump".as_ptr());

	id = btf__add_const(btf, 5); /* points forward to restrict */
	ASSERT_EQ!(id, 3, c"const_id".as_ptr());
	t = btf__type_by_id(btf, 3);
	ASSERT_EQ!(btf_kind(t), BTF_KIND_CONST, c"const_kind".as_ptr());
	ASSERT_EQ!((*t).type_, 5, c"const_type".as_ptr());
	ASSERT_STREQ!(btf_type_raw_dump(btf, 3), c"[3] CONST '(anon)' type_id=5".as_ptr(), c"raw_dump".as_ptr());

	id = btf__add_volatile(btf, 3);
	ASSERT_EQ!(id, 4, c"volatile_id".as_ptr());
	t = btf__type_by_id(btf, 4);
	ASSERT_EQ!(btf_kind(t), BTF_KIND_VOLATILE, c"volatile_kind".as_ptr());
	ASSERT_EQ!((*t).type_, 3, c"volatile_type".as_ptr());
	ASSERT_STREQ!(btf_type_raw_dump(btf, 4), c"[4] VOLATILE '(anon)' type_id=3".as_ptr(), c"raw_dump".as_ptr());

	id = btf__add_restrict(btf, 4);
	ASSERT_EQ!(id, 5, c"restrict_id".as_ptr());
	t = btf__type_by_id(btf, 5);
	ASSERT_EQ!(btf_kind(t), BTF_KIND_RESTRICT, c"restrict_kind".as_ptr());
	ASSERT_EQ!((*t).type_, 4, c"restrict_type".as_ptr());
	ASSERT_STREQ!(btf_type_raw_dump(btf, 5), c"[5] RESTRICT '(anon)' type_id=4".as_ptr(), c"raw_dump".as_ptr());

	/* ARRAY */
	id = btf__add_array(btf, 1, 2, 10); /* int *[10] */
	ASSERT_EQ!(id, 6, c"array_id".as_ptr());
	t = btf__type_by_id(btf, 6);
	ASSERT_EQ!(btf_kind(t), BTF_KIND_ARRAY, c"array_kind".as_ptr());
	ASSERT_EQ!((*btf_array(t)).index_type, 1, c"array_index_type".as_ptr());
	ASSERT_EQ!((*btf_array(t)).type_, 2, c"array_elem_type".as_ptr());
	ASSERT_EQ!((*btf_array(t)).nelems, 10, c"array_nelems".as_ptr());
	ASSERT_STREQ!(
		btf_type_raw_dump(btf, 6),
		c"[6] ARRAY '(anon)' type_id=2 index_type_id=1 nr_elems=10".as_ptr(),
		c"raw_dump".as_ptr()
	);

	/* STRUCT */
	err = btf__add_field(btf, c"field".as_ptr(), 1, 0, 0);
	ASSERT_ERR!(err, c"no_struct_field".as_ptr());
	id = btf__add_struct(btf, c"s1".as_ptr(), 8);
	ASSERT_EQ!(id, 7, c"struct_id".as_ptr());
	err = btf__add_field(btf, c"f1".as_ptr(), 1, 0, 0);
	ASSERT_OK!(err, c"f1_res".as_ptr());
	err = btf__add_field(btf, c"f2".as_ptr(), 1, 32, 16);
	ASSERT_OK!(err, c"f2_res".as_ptr());

	t = btf__type_by_id(btf, 7);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"s1".as_ptr(), c"struct_name".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_STRUCT, c"struct_kind".as_ptr());
	ASSERT_EQ!(btf_vlen(t), 2, c"struct_vlen".as_ptr());
	ASSERT_EQ!(btf_kflag(t), true, c"struct_kflag".as_ptr());
	ASSERT_EQ!((*t).size, 8, c"struct_sz".as_ptr());
	m = btf_members(t).add(0);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*m).name_off), c"f1".as_ptr(), c"f1_name".as_ptr());
	ASSERT_EQ!((*m).type_, 1, c"f1_type".as_ptr());
	ASSERT_EQ!(btf_member_bit_offset(t, 0), 0, c"f1_bit_off".as_ptr());
	ASSERT_EQ!(btf_member_bitfield_size(t, 0), 0, c"f1_bit_sz".as_ptr());
	m = btf_members(t).add(1);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*m).name_off), c"f2".as_ptr(), c"f2_name".as_ptr());
	ASSERT_EQ!((*m).type_, 1, c"f2_type".as_ptr());
	ASSERT_EQ!(btf_member_bit_offset(t, 1), 32, c"f2_bit_off".as_ptr());
	ASSERT_EQ!(btf_member_bitfield_size(t, 1), 16, c"f2_bit_sz".as_ptr());
	ASSERT_STREQ!(
		btf_type_raw_dump(btf, 7),
		c"[7] STRUCT 's1' size=8 vlen=2\n\t'f1' type_id=1 bits_offset=0\n\t'f2' type_id=1 bits_offset=32 bitfield_size=16".as_ptr(),
		c"raw_dump".as_ptr()
	);

	/* UNION */
	id = btf__add_union(btf, c"u1".as_ptr(), 8);
	ASSERT_EQ!(id, 8, c"union_id".as_ptr());

	/* invalid, non-zero offset */
	err = btf__add_field(btf, c"field".as_ptr(), 1, 1, 0);
	ASSERT_ERR!(err, c"no_struct_field".as_ptr());

	err = btf__add_field(btf, c"f1".as_ptr(), 1, 0, 16);
	ASSERT_OK!(err, c"f1_res".as_ptr());

	t = btf__type_by_id(btf, 8);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"u1".as_ptr(), c"union_name".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_UNION, c"union_kind".as_ptr());
	ASSERT_EQ!(btf_vlen(t), 1, c"union_vlen".as_ptr());
	ASSERT_EQ!(btf_kflag(t), true, c"union_kflag".as_ptr());
	ASSERT_EQ!((*t).size, 8, c"union_sz".as_ptr());
	m = btf_members(t).add(0);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*m).name_off), c"f1".as_ptr(), c"f1_name".as_ptr());
	ASSERT_EQ!((*m).type_, 1, c"f1_type".as_ptr());
	ASSERT_EQ!(btf_member_bit_offset(t, 0), 0, c"f1_bit_off".as_ptr());
	ASSERT_EQ!(btf_member_bitfield_size(t, 0), 16, c"f1_bit_sz".as_ptr());
	ASSERT_STREQ!(
		btf_type_raw_dump(btf, 8),
		c"[8] UNION 'u1' size=8 vlen=1\n\t'f1' type_id=1 bits_offset=0 bitfield_size=16".as_ptr(),
		c"raw_dump".as_ptr()
	);

	/* ENUM */
	id = btf__add_enum(btf, c"e1".as_ptr(), 4);
	ASSERT_EQ!(id, 9, c"enum_id".as_ptr());
	err = btf__add_enum_value(btf, c"v1".as_ptr(), 1);
	ASSERT_OK!(err, c"v1_res".as_ptr());
	err = btf__add_enum_value(btf, c"v2".as_ptr(), 2);
	ASSERT_OK!(err, c"v2_res".as_ptr());

	t = btf__type_by_id(btf, 9);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"e1".as_ptr(), c"enum_name".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_ENUM, c"enum_kind".as_ptr());
	ASSERT_EQ!(btf_vlen(t), 2, c"enum_vlen".as_ptr());
	ASSERT_EQ!((*t).size, 4, c"enum_sz".as_ptr());
	v = btf_enum(t).add(0);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*v).name_off), c"v1".as_ptr(), c"v1_name".as_ptr());
	ASSERT_EQ!((*v).val, 1, c"v1_val".as_ptr());
	v = btf_enum(t).add(1);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*v).name_off), c"v2".as_ptr(), c"v2_name".as_ptr());
	ASSERT_EQ!((*v).val, 2, c"v2_val".as_ptr());
	ASSERT_STREQ!(
		btf_type_raw_dump(btf, 9),
		c"[9] ENUM 'e1' encoding=UNSIGNED size=4 vlen=2\n\t'v1' val=1\n\t'v2' val=2".as_ptr(),
		c"raw_dump".as_ptr()
	);

	/* FWDs */
	id = btf__add_fwd(btf, c"struct_fwd".as_ptr(), BTF_FWD_STRUCT);
	ASSERT_EQ!(id, 10, c"struct_fwd_id".as_ptr());
	t = btf__type_by_id(btf, 10);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"struct_fwd".as_ptr(), c"fwd_name".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_FWD, c"fwd_kind".as_ptr());
	ASSERT_EQ!(btf_kflag(t), 0, c"fwd_kflag".as_ptr());
	ASSERT_STREQ!(btf_type_raw_dump(btf, 10), c"[10] FWD 'struct_fwd' fwd_kind=struct".as_ptr(), c"raw_dump".as_ptr());

	id = btf__add_fwd(btf, c"union_fwd".as_ptr(), BTF_FWD_UNION);
	ASSERT_EQ!(id, 11, c"union_fwd_id".as_ptr());
	t = btf__type_by_id(btf, 11);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"union_fwd".as_ptr(), c"fwd_name".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_FWD, c"fwd_kind".as_ptr());
	ASSERT_EQ!(btf_kflag(t), 1, c"fwd_kflag".as_ptr());
	ASSERT_STREQ!(btf_type_raw_dump(btf, 11), c"[11] FWD 'union_fwd' fwd_kind=union".as_ptr(), c"raw_dump".as_ptr());

	id = btf__add_fwd(btf, c"enum_fwd".as_ptr(), BTF_FWD_ENUM);
	ASSERT_EQ!(id, 12, c"enum_fwd_id".as_ptr());
	t = btf__type_by_id(btf, 12);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"enum_fwd".as_ptr(), c"fwd_name".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_ENUM, c"enum_fwd_kind".as_ptr());
	ASSERT_EQ!(btf_vlen(t), 0, c"enum_fwd_kind".as_ptr());
	ASSERT_EQ!((*t).size, 4, c"enum_fwd_sz".as_ptr());
	ASSERT_STREQ!(
		btf_type_raw_dump(btf, 12),
		c"[12] ENUM 'enum_fwd' encoding=UNSIGNED size=4 vlen=0".as_ptr(),
		c"raw_dump".as_ptr()
	);

	/* TYPEDEF */
	id = btf__add_typedef(btf, c"typedef1".as_ptr(), 1);
	ASSERT_EQ!(id, 13, c"typedef_fwd_id".as_ptr());
	t = btf__type_by_id(btf, 13);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"typedef1".as_ptr(), c"typedef_name".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_TYPEDEF, c"typedef_kind".as_ptr());
	ASSERT_EQ!((*t).type_, 1, c"typedef_type".as_ptr());
	ASSERT_STREQ!(btf_type_raw_dump(btf, 13), c"[13] TYPEDEF 'typedef1' type_id=1".as_ptr(), c"raw_dump".as_ptr());

	/* FUNC & FUNC_PROTO */
	id = btf__add_func(btf, c"func1".as_ptr(), BTF_FUNC_GLOBAL, 15);
	ASSERT_EQ!(id, 14, c"func_id".as_ptr());
	t = btf__type_by_id(btf, 14);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"func1".as_ptr(), c"func_name".as_ptr());
	ASSERT_EQ!((*t).type_, 15, c"func_type".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_FUNC, c"func_kind".as_ptr());
	ASSERT_EQ!(btf_vlen(t), BTF_FUNC_GLOBAL, c"func_vlen".as_ptr());
	ASSERT_STREQ!(btf_type_raw_dump(btf, 14), c"[14] FUNC 'func1' type_id=15 linkage=global".as_ptr(), c"raw_dump".as_ptr());

	id = btf__add_func_proto(btf, 1);
	ASSERT_EQ!(id, 15, c"func_proto_id".as_ptr());
	err = btf__add_func_param(btf, c"p1".as_ptr(), 1);
	ASSERT_OK!(err, c"p1_res".as_ptr());
	err = btf__add_func_param(btf, c"p2".as_ptr(), 2);
	ASSERT_OK!(err, c"p2_res".as_ptr());

	t = btf__type_by_id(btf, 15);
	ASSERT_EQ!(btf_kind(t), BTF_KIND_FUNC_PROTO, c"func_proto_kind".as_ptr());
	ASSERT_EQ!(btf_vlen(t), 2, c"func_proto_vlen".as_ptr());
	ASSERT_EQ!((*t).type_, 1, c"func_proto_ret_type".as_ptr());
	p = btf_params(t).add(0);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*p).name_off), c"p1".as_ptr(), c"p1_name".as_ptr());
	ASSERT_EQ!((*p).type_, 1, c"p1_type".as_ptr());
	p = btf_params(t).add(1);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*p).name_off), c"p2".as_ptr(), c"p2_name".as_ptr());
	ASSERT_EQ!((*p).type_, 2, c"p2_type".as_ptr());
	ASSERT_STREQ!(
		btf_type_raw_dump(btf, 15),
		c"[15] FUNC_PROTO '(anon)' ret_type_id=1 vlen=2\n\t'p1' type_id=1\n\t'p2' type_id=2".as_ptr(),
		c"raw_dump".as_ptr()
	);

	/* VAR */
	id = btf__add_var(btf, c"var1".as_ptr(), BTF_VAR_GLOBAL_ALLOCATED, 1);
	ASSERT_EQ!(id, 16, c"var_id".as_ptr());
	t = btf__type_by_id(btf, 16);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"var1".as_ptr(), c"var_name".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_VAR, c"var_kind".as_ptr());
	ASSERT_EQ!((*t).type_, 1, c"var_type".as_ptr());
	ASSERT_EQ!((*btf_var(t)).linkage, BTF_VAR_GLOBAL_ALLOCATED, c"var_type".as_ptr());
	ASSERT_STREQ!(btf_type_raw_dump(btf, 16), c"[16] VAR 'var1' type_id=1, linkage=global-alloc".as_ptr(), c"raw_dump".as_ptr());

	/* DATASECT */
	id = btf__add_datasec(btf, c"datasec1".as_ptr(), 12);
	ASSERT_EQ!(id, 17, c"datasec_id".as_ptr());
	err = btf__add_datasec_var_info(btf, 1, 4, 8);
	ASSERT_OK!(err, c"v1_res".as_ptr());

	t = btf__type_by_id(btf, 17);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"datasec1".as_ptr(), c"datasec_name".as_ptr());
	ASSERT_EQ!((*t).size, 12, c"datasec_sz".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_DATASEC, c"datasec_kind".as_ptr());
	ASSERT_EQ!(btf_vlen(t), 1, c"datasec_vlen".as_ptr());
	vi = btf_var_secinfos(t).add(0);
	ASSERT_EQ!((*vi).type_, 1, c"v1_type".as_ptr());
	ASSERT_EQ!((*vi).offset, 4, c"v1_off".as_ptr());
	ASSERT_EQ!((*vi).size, 8, c"v1_sz".as_ptr());
	ASSERT_STREQ!(
		btf_type_raw_dump(btf, 17),
		c"[17] DATASEC 'datasec1' size=12 vlen=1\n\ttype_id=1 offset=4 size=8".as_ptr(),
		c"raw_dump".as_ptr()
	);

	/* DECL_TAG */
	id = btf__add_decl_tag(btf, c"tag1".as_ptr(), 16, -1);
	ASSERT_EQ!(id, 18, c"tag_id".as_ptr());
	t = btf__type_by_id(btf, 18);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"tag1".as_ptr(), c"tag_value".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_DECL_TAG, c"tag_kind".as_ptr());
	ASSERT_EQ!((*t).type_, 16, c"tag_type".as_ptr());
	ASSERT_EQ!((*btf_decl_tag(t)).component_idx, -1, c"tag_component_idx".as_ptr());
	ASSERT_STREQ!(btf_type_raw_dump(btf, 18), c"[18] DECL_TAG 'tag1' type_id=16 component_idx=-1".as_ptr(), c"raw_dump".as_ptr());

	id = btf__add_decl_tag(btf, c"tag2".as_ptr(), 14, 1);
	ASSERT_EQ!(id, 19, c"tag_id".as_ptr());
	t = btf__type_by_id(btf, 19);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"tag2".as_ptr(), c"tag_value".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_DECL_TAG, c"tag_kind".as_ptr());
	ASSERT_EQ!((*t).type_, 14, c"tag_type".as_ptr());
	ASSERT_EQ!((*btf_decl_tag(t)).component_idx, 1, c"tag_component_idx".as_ptr());
	ASSERT_STREQ!(btf_type_raw_dump(btf, 19), c"[19] DECL_TAG 'tag2' type_id=14 component_idx=1".as_ptr(), c"raw_dump".as_ptr());

	/* TYPE_TAG */
	id = btf__add_type_tag(btf, c"tag1".as_ptr(), 1);
	ASSERT_EQ!(id, 20, c"tag_id".as_ptr());
	t = btf__type_by_id(btf, 20);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"tag1".as_ptr(), c"tag_value".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_TYPE_TAG, c"tag_kind".as_ptr());
	ASSERT_EQ!((*t).type_, 1, c"tag_type".as_ptr());
	ASSERT_STREQ!(btf_type_raw_dump(btf, 20), c"[20] TYPE_TAG 'tag1' type_id=1".as_ptr(), c"raw_dump".as_ptr());

	/* ENUM64 */
	id = btf__add_enum64(btf, c"e1".as_ptr(), 8, true);
	ASSERT_EQ!(id, 21, c"enum64_id".as_ptr());
	err = btf__add_enum64_value(btf, c"v1".as_ptr(), -1i64 as c_ulonglong);
	ASSERT_OK!(err, c"v1_res".as_ptr());
	err = btf__add_enum64_value(btf, c"v2".as_ptr(), 0x123456789); /* 4886718345 */
	ASSERT_OK!(err, c"v2_res".as_ptr());
	t = btf__type_by_id(btf, 21);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"e1".as_ptr(), c"enum64_name".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_ENUM64, c"enum64_kind".as_ptr());
	ASSERT_EQ!(btf_vlen(t), 2, c"enum64_vlen".as_ptr());
	ASSERT_EQ!((*t).size, 8, c"enum64_sz".as_ptr());
	v64 = btf_enum64(t).add(0);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*v64).name_off), c"v1".as_ptr(), c"v1_name".as_ptr());
	ASSERT_EQ!((*v64).val_hi32, 0xffffffff, c"v1_val".as_ptr());
	ASSERT_EQ!((*v64).val_lo32, 0xffffffff, c"v1_val".as_ptr());
	v64 = btf_enum64(t).add(1);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*v64).name_off), c"v2".as_ptr(), c"v2_name".as_ptr());
	ASSERT_EQ!((*v64).val_hi32, 0x1, c"v2_val".as_ptr());
	ASSERT_EQ!((*v64).val_lo32, 0x23456789, c"v2_val".as_ptr());
	ASSERT_STREQ!(
		btf_type_raw_dump(btf, 21),
		c"[21] ENUM64 'e1' encoding=SIGNED size=8 vlen=2\n\t'v1' val=-1\n\t'v2' val=4886718345".as_ptr(),
		c"raw_dump".as_ptr()
	);

	id = btf__add_enum64(btf, c"e1".as_ptr(), 8, false);
	ASSERT_EQ!(id, 22, c"enum64_id".as_ptr());
	err = btf__add_enum64_value(btf, c"v1".as_ptr(), 0xffffffffFFFFFFFF); /* 18446744073709551615 */
	ASSERT_OK!(err, c"v1_res".as_ptr());
	t = btf__type_by_id(btf, 22);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*t).name_off), c"e1".as_ptr(), c"enum64_name".as_ptr());
	ASSERT_EQ!(btf_kind(t), BTF_KIND_ENUM64, c"enum64_kind".as_ptr());
	ASSERT_EQ!(btf_vlen(t), 1, c"enum64_vlen".as_ptr());
	ASSERT_EQ!((*t).size, 8, c"enum64_sz".as_ptr());
	v64 = btf_enum64(t).add(0);
	ASSERT_STREQ!(btf__str_by_offset(btf, (*v64).name_off), c"v1".as_ptr(), c"v1_name".as_ptr());
	ASSERT_EQ!((*v64).val_hi32, 0xffffffff, c"v1_val".as_ptr());
	ASSERT_EQ!((*v64).val_lo32, 0xffffffff, c"v1_val".as_ptr());
	ASSERT_STREQ!(
		btf_type_raw_dump(btf, 22),
		c"[22] ENUM64 'e1' encoding=UNSIGNED size=8 vlen=1\n\t'v1' val=18446744073709551615".as_ptr(),
		c"raw_dump".as_ptr()
	);
}

unsafe fn test_btf_add() {
	let btf: *mut btf;

	btf = btf__new_empty();
	if !ASSERT_OK_PTR!(btf, c"new_empty".as_ptr()) {
		return;
	}

	gen_btf(btf);

	VALIDATE_RAW_BTF!(
		btf,
		c"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED".as_ptr(),
		c"[2] PTR '(anon)' type_id=1".as_ptr(),
		c"[3] CONST '(anon)' type_id=5".as_ptr(),
		c"[4] VOLATILE '(anon)' type_id=3".as_ptr(),
		c"[5] RESTRICT '(anon)' type_id=4".as_ptr(),
		c"[6] ARRAY '(anon)' type_id=2 index_type_id=1 nr_elems=10".as_ptr(),
		c"[7] STRUCT 's1' size=8 vlen=2\n\t'f1' type_id=1 bits_offset=0\n\t'f2' type_id=1 bits_offset=32 bitfield_size=16".as_ptr(),
		c"[8] UNION 'u1' size=8 vlen=1\n\t'f1' type_id=1 bits_offset=0 bitfield_size=16".as_ptr(),
		c"[9] ENUM 'e1' encoding=UNSIGNED size=4 vlen=2\n\t'v1' val=1\n\t'v2' val=2".as_ptr(),
		c"[10] FWD 'struct_fwd' fwd_kind=struct".as_ptr(),
		c"[11] FWD 'union_fwd' fwd_kind=union".as_ptr(),
		c"[12] ENUM 'enum_fwd' encoding=UNSIGNED size=4 vlen=0".as_ptr(),
		c"[13] TYPEDEF 'typedef1' type_id=1".as_ptr(),
		c"[14] FUNC 'func1' type_id=15 linkage=global".as_ptr(),
		c"[15] FUNC_PROTO '(anon)' ret_type_id=1 vlen=2\n\t'p1' type_id=1\n\t'p2' type_id=2".as_ptr(),
		c"[16] VAR 'var1' type_id=1, linkage=global-alloc".as_ptr(),
		c"[17] DATASEC 'datasec1' size=12 vlen=1\n\ttype_id=1 offset=4 size=8".as_ptr(),
		c"[18] DECL_TAG 'tag1' type_id=16 component_idx=-1".as_ptr(),
		c"[19] DECL_TAG 'tag2' type_id=14 component_idx=1".as_ptr(),
		c"[20] TYPE_TAG 'tag1' type_id=1".as_ptr(),
		c"[21] ENUM64 'e1' encoding=SIGNED size=8 vlen=2\n\t'v1' val=-1\n\t'v2' val=4886718345".as_ptr(),
		c"[22] ENUM64 'e1' encoding=UNSIGNED size=8 vlen=1\n\t'v1' val=18446744073709551615".as_ptr()
	);

	btf__free(btf);
}

unsafe fn test_btf_add_btf() {
	let mut btf1: *mut btf = ptr::null_mut();
	let mut btf2: *mut btf = ptr::null_mut();
	let mut id: c_int;

	'cleanup: {
		btf1 = btf__new_empty();
		if !ASSERT_OK_PTR!(btf1, c"btf1".as_ptr()) {
			return;
		}

		btf2 = btf__new_empty();
		if !ASSERT_OK_PTR!(btf2, c"btf2".as_ptr()) {
			break 'cleanup;
		}

		gen_btf(btf1);
		gen_btf(btf2);

		id = btf__add_btf(btf1, btf2);
		if !ASSERT_EQ!(id, 23, c"id".as_ptr()) {
			break 'cleanup;
		}

		VALIDATE_RAW_BTF!(
			btf1,
			c"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED".as_ptr(),
			c"[2] PTR '(anon)' type_id=1".as_ptr(),
			c"[3] CONST '(anon)' type_id=5".as_ptr(),
			c"[4] VOLATILE '(anon)' type_id=3".as_ptr(),
			c"[5] RESTRICT '(anon)' type_id=4".as_ptr(),
			c"[6] ARRAY '(anon)' type_id=2 index_type_id=1 nr_elems=10".as_ptr(),
			c"[7] STRUCT 's1' size=8 vlen=2\n\t'f1' type_id=1 bits_offset=0\n\t'f2' type_id=1 bits_offset=32 bitfield_size=16".as_ptr(),
			c"[8] UNION 'u1' size=8 vlen=1\n\t'f1' type_id=1 bits_offset=0 bitfield_size=16".as_ptr(),
			c"[9] ENUM 'e1' encoding=UNSIGNED size=4 vlen=2\n\t'v1' val=1\n\t'v2' val=2".as_ptr(),
			c"[10] FWD 'struct_fwd' fwd_kind=struct".as_ptr(),
			c"[11] FWD 'union_fwd' fwd_kind=union".as_ptr(),
			c"[12] ENUM 'enum_fwd' encoding=UNSIGNED size=4 vlen=0".as_ptr(),
			c"[13] TYPEDEF 'typedef1' type_id=1".as_ptr(),
			c"[14] FUNC 'func1' type_id=15 linkage=global".as_ptr(),
			c"[15] FUNC_PROTO '(anon)' ret_type_id=1 vlen=2\n\t'p1' type_id=1\n\t'p2' type_id=2".as_ptr(),
			c"[16] VAR 'var1' type_id=1, linkage=global-alloc".as_ptr(),
			c"[17] DATASEC 'datasec1' size=12 vlen=1\n\ttype_id=1 offset=4 size=8".as_ptr(),
			c"[18] DECL_TAG 'tag1' type_id=16 component_idx=-1".as_ptr(),
			c"[19] DECL_TAG 'tag2' type_id=14 component_idx=1".as_ptr(),
			c"[20] TYPE_TAG 'tag1' type_id=1".as_ptr(),
			c"[21] ENUM64 'e1' encoding=SIGNED size=8 vlen=2\n\t'v1' val=-1\n\t'v2' val=4886718345".as_ptr(),
			c"[22] ENUM64 'e1' encoding=UNSIGNED size=8 vlen=1\n\t'v1' val=18446744073709551615".as_ptr(),

			/* types appended from the second BTF */
			c"[23] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED".as_ptr(),
			c"[24] PTR '(anon)' type_id=23".as_ptr(),
			c"[25] CONST '(anon)' type_id=27".as_ptr(),
			c"[26] VOLATILE '(anon)' type_id=25".as_ptr(),
			c"[27] RESTRICT '(anon)' type_id=26".as_ptr(),
			c"[28] ARRAY '(anon)' type_id=24 index_type_id=23 nr_elems=10".as_ptr(),
			c"[29] STRUCT 's1' size=8 vlen=2\n\t'f1' type_id=23 bits_offset=0\n\t'f2' type_id=23 bits_offset=32 bitfield_size=16".as_ptr(),
			c"[30] UNION 'u1' size=8 vlen=1\n\t'f1' type_id=23 bits_offset=0 bitfield_size=16".as_ptr(),
			c"[31] ENUM 'e1' encoding=UNSIGNED size=4 vlen=2\n\t'v1' val=1\n\t'v2' val=2".as_ptr(),
			c"[32] FWD 'struct_fwd' fwd_kind=struct".as_ptr(),
			c"[33] FWD 'union_fwd' fwd_kind=union".as_ptr(),
			c"[34] ENUM 'enum_fwd' encoding=UNSIGNED size=4 vlen=0".as_ptr(),
			c"[35] TYPEDEF 'typedef1' type_id=23".as_ptr(),
			c"[36] FUNC 'func1' type_id=37 linkage=global".as_ptr(),
			c"[37] FUNC_PROTO '(anon)' ret_type_id=23 vlen=2\n\t'p1' type_id=23\n\t'p2' type_id=24".as_ptr(),
			c"[38] VAR 'var1' type_id=23, linkage=global-alloc".as_ptr(),
			c"[39] DATASEC 'datasec1' size=12 vlen=1\n\ttype_id=23 offset=4 size=8".as_ptr(),
			c"[40] DECL_TAG 'tag1' type_id=38 component_idx=-1".as_ptr(),
			c"[41] DECL_TAG 'tag2' type_id=36 component_idx=1".as_ptr(),
			c"[42] TYPE_TAG 'tag1' type_id=23".as_ptr(),
			c"[43] ENUM64 'e1' encoding=SIGNED size=8 vlen=2\n\t'v1' val=-1\n\t'v2' val=4886718345".as_ptr(),
			c"[44] ENUM64 'e1' encoding=UNSIGNED size=8 vlen=1\n\t'v1' val=18446744073709551615".as_ptr()
		);
	}

	btf__free(btf1);
	btf__free(btf2);
}

unsafe fn test_btf_add_btf_split() {
	let mut base: *mut btf = ptr::null_mut();
	let mut split1: *mut btf = ptr::null_mut();
	let mut split2: *mut btf = ptr::null_mut();
	let mut combined: *mut btf = ptr::null_mut();
	let mut id: c_int;
	let mut err: c_int;

	'cleanup: {
		/* Create a base BTF with an INT and a PTR to it */
		base = btf__new_empty();
		if !ASSERT_OK_PTR!(base, c"base".as_ptr()) {
			return;
		}

		id = btf__add_int(base, c"int".as_ptr(), 4, BTF_INT_SIGNED);
		ASSERT_EQ!(id, 1, c"base_int_id".as_ptr());
		id = btf__add_ptr(base, 1);
		ASSERT_EQ!(id, 2, c"base_ptr_id".as_ptr());

		/* base has 2 types, type IDs 1..2 */
		ASSERT_EQ!(btf__type_cnt(base), 3, c"base_type_cnt".as_ptr());

		/* Create split1 on base: a STRUCT referencing base's int (ID 1) */
		split1 = btf__new_empty_split(base);
		if !ASSERT_OK_PTR!(split1, c"split1".as_ptr()) {
			break 'cleanup;
		}

		id = btf__add_struct(split1, c"s1".as_ptr(), 4);
		/* split types start at base_type_cnt = 3 */
		ASSERT_EQ!(id, 3, c"split1_struct_id".as_ptr());
		btf__add_field(split1, c"x".as_ptr(), 1, 0, 0); /* refers to base int */

		id = btf__add_ptr(split1, 3);
		ASSERT_EQ!(id, 4, c"split1_ptr_id".as_ptr()); /* ptr to the struct (split self-ref) */

		/* Add a typedef "int_alias" -> base int in split1, which will be
		 * duplicated in split2 to test that btf__dedup() merges them.
		 */
		id = btf__add_typedef(split1, c"int_alias".as_ptr(), 1);
		ASSERT_EQ!(id, 5, c"split1_typedef_id".as_ptr());

		/* Create split2 on base: a TYPEDEF referencing base's ptr (ID 2) */
		split2 = btf__new_empty_split(base);
		if !ASSERT_OK_PTR!(split2, c"split2".as_ptr()) {
			break 'cleanup;
		}

		id = btf__add_typedef(split2, c"int_ptr".as_ptr(), 2); /* refers to base ptr */
		ASSERT_EQ!(id, 3, c"split2_typedef_id".as_ptr());

		id = btf__add_struct(split2, c"s2".as_ptr(), 8);
		ASSERT_EQ!(id, 4, c"split2_struct_id".as_ptr());
		btf__add_field(split2, c"p".as_ptr(), 3, 0, 0); /* refers to split2's own typedef */

		/* Same "int_alias" typedef as split1 - should be deduped away */
		id = btf__add_typedef(split2, c"int_alias".as_ptr(), 1);
		ASSERT_EQ!(id, 5, c"split2_dup_typedef_id".as_ptr());

		/* Create combined split BTF on same base and merge both */
		combined = btf__new_empty_split(base);
		if !ASSERT_OK_PTR!(combined, c"combined".as_ptr()) {
			break 'cleanup;
		}

		/* Merge split1: its types (3,4,5) should land at IDs 3,4,5 */
		id = btf__add_btf(combined, split1);
		if !ASSERT_GE!(id, 0, c"add_split1".as_ptr()) {
			break 'cleanup;
		}
		ASSERT_EQ!(id, 3, c"split1_first_id".as_ptr());

		/* Merge split2: its types (3,4,5) should be remapped to 6,7,8 */
		id = btf__add_btf(combined, split2);
		if !ASSERT_GE!(id, 0, c"add_split2".as_ptr()) {
			break 'cleanup;
		}
		ASSERT_EQ!(id, 6, c"split2_first_id".as_ptr());

		/* Before dedup: base (2) + split1 (3) + split2 (3) = 8 types + void */
		ASSERT_EQ!(btf__type_cnt(combined), 9, c"pre_dedup_type_cnt".as_ptr());

		VALIDATE_RAW_BTF!(
			combined,
			/* base types (IDs 1-2) */
			c"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED".as_ptr(),
			c"[2] PTR '(anon)' type_id=1".as_ptr(),

			/* split1 types (IDs 3-5): base refs unchanged */
			c"[3] STRUCT 's1' size=4 vlen=1\n\t'x' type_id=1 bits_offset=0".as_ptr(), /* refers to base int=1 */
			c"[4] PTR '(anon)' type_id=3".as_ptr(),                                    /* refers to split1's struct=3 */
			c"[5] TYPEDEF 'int_alias' type_id=1".as_ptr(),                             /* refers to base int=1 */

			/* split2 types (IDs 6-8): remapped from 3,4,5 to 6,7,8 */
			c"[6] TYPEDEF 'int_ptr' type_id=2".as_ptr(),                               /* base ptr=2, unchanged */
			c"[7] STRUCT 's2' size=8 vlen=1\n\t'p' type_id=6 bits_offset=0".as_ptr(),   /* split2 typedef: 3->6 */
			c"[8] TYPEDEF 'int_alias' type_id=1".as_ptr()                              /* dup of [5] */
		);

		/* Dedup to mirror the bpftool merge flow; should remove the
		 * duplicate "int_alias" typedef.
		 */
		err = btf__dedup(combined, ptr::null_mut());
		if !ASSERT_OK!(err, c"dedup".as_ptr()) {
			break 'cleanup;
		}

		/* After dedup: one int_alias removed, so 7 types + void */
		ASSERT_EQ!(btf__type_cnt(combined), 8, c"dedup_type_cnt".as_ptr());
	}

	btf__free(combined);
	btf__free(split2);
	btf__free(split1);
	btf__free(base);
}

#[no_mangle]
pub unsafe extern "C" fn test_btf_write() {
	if test__start_subtest(c"btf_add".as_ptr()) {
		test_btf_add();
	}
	if test__start_subtest(c"btf_add_btf".as_ptr()) {
		test_btf_add_btf();
	}
	if test__start_subtest(c"btf_add_btf_split".as_ptr()) {
		test_btf_add_btf_split();
	}
}
