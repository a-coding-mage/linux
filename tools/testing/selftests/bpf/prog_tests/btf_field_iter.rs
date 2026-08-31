// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024, Oracle and/or its affiliates. */

/* Translated from:
 * testing/selftests/bpf/prog_tests/btf_field_iter.c
 *
 * C includes removed; the referenced BTF/libbpf/test symbols are expected to be
 * provided by the surrounding translated test harness.
 */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

type __u32 = u32;

#[repr(C)]
pub struct btf {
	_private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
	_private: [u8; 0],
}

#[repr(C)]
pub struct btf_field_iter {
	_private: [u8; 0],
}

#[repr(C)]
struct field_data {
	ids: [__u32; 5],
	strs: [*const c_char; 5],
}

const BTF_INT_SIGNED: u32 = 1;
const BTF_FWD_STRUCT: c_int = 0;
const BTF_FUNC_GLOBAL: c_int = 1;
const BTF_VAR_STATIC: c_int = 0;
const BTF_FIELD_ITER_STRS: c_int = 0;
const BTF_FIELD_ITER_IDS: c_int = 1;

unsafe extern "C" {
	fn btf__new_empty() -> *mut btf;
	fn btf__add_int(btf: *mut btf, name: *const c_char, sz: u32, encoding: u32) -> c_int;
	fn btf__add_ptr(btf: *mut btf, ref_type_id: c_int) -> c_int;
	fn btf__add_array(
		btf: *mut btf,
		index_type_id: c_int,
		elem_type_id: c_int,
		nr_elems: u32,
	) -> c_int;
	fn btf__add_struct(btf: *mut btf, name: *const c_char, sz: u32) -> c_int;
	fn btf__add_field(
		btf: *mut btf,
		name: *const c_char,
		type_id: c_int,
		bit_offset: u32,
		bit_size: u32,
	) -> c_int;
	fn btf__add_union(btf: *mut btf, name: *const c_char, sz: u32) -> c_int;
	fn btf__add_enum(btf: *mut btf, name: *const c_char, sz: u32) -> c_int;
	fn btf__add_enum_value(btf: *mut btf, name: *const c_char, value: c_int) -> c_int;
	fn btf__add_fwd(btf: *mut btf, name: *const c_char, kind: c_int) -> c_int;
	fn btf__add_typedef(btf: *mut btf, name: *const c_char, ref_type_id: c_int) -> c_int;
	fn btf__add_volatile(btf: *mut btf, ref_type_id: c_int) -> c_int;
	fn btf__add_const(btf: *mut btf, ref_type_id: c_int) -> c_int;
	fn btf__add_restrict(btf: *mut btf, ref_type_id: c_int) -> c_int;
	fn btf__add_func_proto(btf: *mut btf, ret_type_id: c_int) -> c_int;
	fn btf__add_func_param(btf: *mut btf, name: *const c_char, type_id: c_int) -> c_int;
	fn btf__add_func(
		btf: *mut btf,
		name: *const c_char,
		linkage: c_int,
		proto_type_id: c_int,
	) -> c_int;
	fn btf__add_var(btf: *mut btf, name: *const c_char, linkage: c_int, type_id: c_int) -> c_int;
	fn btf__add_float(btf: *mut btf, name: *const c_char, sz: u32) -> c_int;
	fn btf__add_decl_tag(
		btf: *mut btf,
		value: *const c_char,
		ref_type_id: c_int,
		component_idx: c_int,
	) -> c_int;
	fn btf__add_type_tag(btf: *mut btf, value: *const c_char, ref_type_id: c_int) -> c_int;
	fn btf__add_enum64(btf: *mut btf, name: *const c_char, sz: u32, is_signed: bool) -> c_int;
	fn btf__add_enum64_value(btf: *mut btf, name: *const c_char, value: u64) -> c_int;
	fn btf__add_datasec(btf: *mut btf, name: *const c_char, sz: u32) -> c_int;
	fn btf__add_datasec_var_info(
		btf: *mut btf,
		type_id: c_int,
		offset: u32,
		sz: u32,
	) -> c_int;
	fn btf__type_cnt(btf: *const btf) -> c_int;
	fn btf_type_by_id(btf: *const btf, type_id: c_int) -> *mut btf_type;
	fn btf_field_iter_init(it: *mut btf_field_iter, t: *mut btf_type, iter_kind: c_int) -> c_int;
	fn btf_field_iter_next(it: *mut btf_field_iter) -> *mut __u32;
	fn btf__str_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
	fn btf__free(btf: *mut btf);
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;

	fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
	fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
	fn ASSERT_EQ(actual: usize, expected: usize, name: *const c_char) -> bool;
}

static mut fields: [field_data; 22] = [
	field_data { ids: [0, 0, 0, 0, 0], strs: [ptr::null(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [0, 0, 0, 0, 0], strs: [c"int".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [0, 0, 0, 0, 0], strs: [c"int64".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [1, 0, 0, 0, 0], strs: [c"".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [2, 1, 0, 0, 0], strs: [c"".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [3, 1, 0, 0, 0], strs: [c"s1".as_ptr(), c"f1".as_ptr(), c"f2".as_ptr(), ptr::null(), ptr::null()] },
	field_data { ids: [1, 5, 0, 0, 0], strs: [c"u1".as_ptr(), c"f1".as_ptr(), c"f2".as_ptr(), ptr::null(), ptr::null()] },
	field_data { ids: [0, 0, 0, 0, 0], strs: [c"e1".as_ptr(), c"v1".as_ptr(), c"v2".as_ptr(), ptr::null(), ptr::null()] },
	field_data { ids: [0, 0, 0, 0, 0], strs: [c"fw1".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [1, 0, 0, 0, 0], strs: [c"t".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [2, 0, 0, 0, 0], strs: [c"".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [1, 0, 0, 0, 0], strs: [c"".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [3, 0, 0, 0, 0], strs: [c"".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [1, 1, 3, 0, 0], strs: [c"".as_ptr(), c"p1".as_ptr(), c"p2".as_ptr(), ptr::null(), ptr::null()] },
	field_data { ids: [13, 0, 0, 0, 0], strs: [c"func".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [1, 0, 0, 0, 0], strs: [c"var1".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [3, 0, 0, 0, 0], strs: [c"var2".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [0, 0, 0, 0, 0], strs: [c"float".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [11, 0, 0, 0, 0], strs: [c"decltag".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [6, 0, 0, 0, 0], strs: [c"typetag".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
	field_data { ids: [0, 0, 0, 0, 0], strs: [c"e64".as_ptr(), c"eval1".as_ptr(), c"eval2".as_ptr(), c"eval3".as_ptr(), ptr::null()] },
	field_data { ids: [15, 16, 0, 0, 0], strs: [c"datasec1".as_ptr(), ptr::null(), ptr::null(), ptr::null(), ptr::null()] },
];

unsafe fn VALIDATE_RAW_BTF(
	_btf: *mut btf,
	_s1: *const c_char,
	_s2: *const c_char,
	_s3: *const c_char,
	_s4: *const c_char,
	_s5: *const c_char,
	_s6: *const c_char,
	_s7: *const c_char,
	_s8: *const c_char,
	_s9: *const c_char,
	_s10: *const c_char,
	_s11: *const c_char,
	_s12: *const c_char,
	_s13: *const c_char,
	_s14: *const c_char,
	_s15: *const c_char,
	_s16: *const c_char,
	_s17: *const c_char,
	_s18: *const c_char,
	_s19: *const c_char,
	_s20: *const c_char,
	_s21: *const c_char,
) {
	/* C macro supplied by btf_helpers.h; expected to be handled by the surrounding harness. */
}

/* Fabricate BTF with various types and check BTF field iteration finds types,
 * strings expected.
 */
pub unsafe extern "C" fn test_btf_field_iter() {
	let mut btf: *mut btf = ptr::null_mut();
	let mut id: c_int;

	btf = btf__new_empty();
	if !ASSERT_OK_PTR(btf as *const c_void, c"empty_btf".as_ptr()) {
		return;
	}

	btf__add_int(btf, c"int".as_ptr(), 4, BTF_INT_SIGNED); /* [1] int */
	btf__add_int(btf, c"int64".as_ptr(), 8, BTF_INT_SIGNED); /* [2] int64 */
	btf__add_ptr(btf, 1); /* [3] int * */
	btf__add_array(btf, 1, 2, 3); /* [4] int64[3] */
	btf__add_struct(btf, c"s1".as_ptr(), 12); /* [5] struct s1 { */
	btf__add_field(btf, c"f1".as_ptr(), 3, 0, 0); /*      int *f1; */
	btf__add_field(btf, c"f2".as_ptr(), 1, 0, 0); /*	int f2; */
	/* } */
	btf__add_union(btf, c"u1".as_ptr(), 12); /* [6] union u1 { */
	btf__add_field(btf, c"f1".as_ptr(), 1, 0, 0); /*	int f1; */
	btf__add_field(btf, c"f2".as_ptr(), 5, 0, 0); /*	struct s1 f2; */
	/* } */
	btf__add_enum(btf, c"e1".as_ptr(), 4); /* [7] enum e1 { */
	btf__add_enum_value(btf, c"v1".as_ptr(), 1); /*	v1 = 1; */
	btf__add_enum_value(btf, c"v2".as_ptr(), 2); /*	v2 = 2; */
	/* } */

	btf__add_fwd(btf, c"fw1".as_ptr(), BTF_FWD_STRUCT); /* [8] struct fw1; */
	btf__add_typedef(btf, c"t".as_ptr(), 1); /* [9] typedef int t; */
	btf__add_volatile(btf, 2); /* [10] volatile int64; */
	btf__add_const(btf, 1); /* [11] const int; */
	btf__add_restrict(btf, 3); /* [12] restrict int *; */
	btf__add_func_proto(btf, 1); /* [13] int (*)(int p1, int *p2); */
	btf__add_func_param(btf, c"p1".as_ptr(), 1);
	btf__add_func_param(btf, c"p2".as_ptr(), 3);

	btf__add_func(btf, c"func".as_ptr(), BTF_FUNC_GLOBAL, 13); /* [14] int func(int p1, int *p2); */
	btf__add_var(btf, c"var1".as_ptr(), BTF_VAR_STATIC, 1); /* [15] static int var1; */
	btf__add_var(btf, c"var2".as_ptr(), BTF_VAR_STATIC, 3); /* [16] static int *var2; */
	btf__add_float(btf, c"float".as_ptr(), 4); /* [17] float; */
	btf__add_decl_tag(btf, c"decltag".as_ptr(), 11, -1); /* [18] decltag const int; */
	btf__add_type_tag(btf, c"typetag".as_ptr(), 6); /* [19] typetag union u1; */
	btf__add_enum64(btf, c"e64".as_ptr(), 8, true); /* [20] enum { */
	btf__add_enum64_value(btf, c"eval1".as_ptr(), 1000); /*	 eval1 = 1000, */
	btf__add_enum64_value(btf, c"eval2".as_ptr(), 2000); /*	 eval2 = 2000, */
	btf__add_enum64_value(btf, c"eval3".as_ptr(), 3000); /*	 eval3 = 3000 */
	/* } */
	btf__add_datasec(btf, c"datasec1".as_ptr(), 12); /* [21] datasec datasec1 */
	btf__add_datasec_var_info(btf, 15, 0, 4);
	btf__add_datasec_var_info(btf, 16, 4, 8);

	VALIDATE_RAW_BTF(
		btf,
		c"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED".as_ptr(),
		c"[2] INT 'int64' size=8 bits_offset=0 nr_bits=64 encoding=SIGNED".as_ptr(),
		c"[3] PTR '(anon)' type_id=1".as_ptr(),
		c"[4] ARRAY '(anon)' type_id=2 index_type_id=1 nr_elems=3".as_ptr(),
		c"[5] STRUCT 's1' size=12 vlen=2\n\t'f1' type_id=3 bits_offset=0\n\t'f2' type_id=1 bits_offset=0".as_ptr(),
		c"[6] UNION 'u1' size=12 vlen=2\n\t'f1' type_id=1 bits_offset=0\n\t'f2' type_id=5 bits_offset=0".as_ptr(),
		c"[7] ENUM 'e1' encoding=UNSIGNED size=4 vlen=2\n\t'v1' val=1\n\t'v2' val=2".as_ptr(),
		c"[8] FWD 'fw1' fwd_kind=struct".as_ptr(),
		c"[9] TYPEDEF 't' type_id=1".as_ptr(),
		c"[10] VOLATILE '(anon)' type_id=2".as_ptr(),
		c"[11] CONST '(anon)' type_id=1".as_ptr(),
		c"[12] RESTRICT '(anon)' type_id=3".as_ptr(),
		c"[13] FUNC_PROTO '(anon)' ret_type_id=1 vlen=2\n\t'p1' type_id=1\n\t'p2' type_id=3".as_ptr(),
		c"[14] FUNC 'func' type_id=13 linkage=global".as_ptr(),
		c"[15] VAR 'var1' type_id=1, linkage=static".as_ptr(),
		c"[16] VAR 'var2' type_id=3, linkage=static".as_ptr(),
		c"[17] FLOAT 'float' size=4".as_ptr(),
		c"[18] DECL_TAG 'decltag' type_id=11 component_idx=-1".as_ptr(),
		c"[19] TYPE_TAG 'typetag' type_id=6".as_ptr(),
		c"[20] ENUM64 'e64' encoding=SIGNED size=8 vlen=3\n\t'eval1' val=1000\n\t'eval2' val=2000\n\t'eval3' val=3000".as_ptr(),
		c"[21] DATASEC 'datasec1' size=12 vlen=2\n\ttype_id=15 offset=0 size=4\n\ttype_id=16 offset=4 size=8".as_ptr(),
	);

	id = 1;
	while id < btf__type_cnt(btf) {
		let t: *mut btf_type = btf_type_by_id(btf, id);
		let mut it_strs: btf_field_iter = core::mem::zeroed();
		let mut it_ids: btf_field_iter = core::mem::zeroed();
		let mut str_idx: c_int = 0;
		let mut id_idx: c_int = 0;
		let mut next_str: *mut __u32;
		let mut next_id: *mut __u32;

		if !ASSERT_OK_PTR(t as *const c_void, c"btf_type_by_id".as_ptr()) {
			break;
		}
		if !ASSERT_OK(
			btf_field_iter_init(&mut it_strs, t, BTF_FIELD_ITER_STRS),
			c"iter_init_strs".as_ptr(),
		) {
			break;
		}
		if !ASSERT_OK(
			btf_field_iter_init(&mut it_ids, t, BTF_FIELD_ITER_IDS),
			c"iter_init_ids".as_ptr(),
		) {
			break;
		}
		loop {
			next_str = btf_field_iter_next(&mut it_strs);
			if next_str.is_null() {
				break;
			}
			let str_: *const c_char = btf__str_by_offset(btf, *next_str);

			if !ASSERT_OK(
				strcmp(fields[id as usize].strs[str_idx as usize], str_),
				c"field_str_match".as_ptr(),
			) {
				break;
			}
			str_idx += 1;
		}
		/* ensure no more strings are expected */
		ASSERT_EQ(
			fields[id as usize].strs[str_idx as usize] as usize,
			ptr::null::<c_char>() as usize,
			c"field_str_cnt".as_ptr(),
		);

		loop {
			next_id = btf_field_iter_next(&mut it_ids);
			if next_id.is_null() {
				break;
			}
			if !ASSERT_EQ(
				*next_id as usize,
				fields[id as usize].ids[id_idx as usize] as usize,
				c"field_id_match".as_ptr(),
			) {
				break;
			}
			id_idx += 1;
		}
		/* ensure no more ids are expected */
		ASSERT_EQ(
			fields[id as usize].ids[id_idx as usize] as usize,
			0,
			c"field_id_cnt".as_ptr(),
		);

		id += 1;
	}
	btf__free(btf);
}
