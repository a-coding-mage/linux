// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2026 Xiaomi */

// C dependencies translated as external Rust dependencies:
// <test_progs.h>, <bpf/btf.h>, "btf_helpers.h"

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

type __u32 = u32;

#[repr(C)]
pub struct btf {
	_private: [u8; 0],
}

unsafe extern "C" {
	fn btf__new_empty() -> *mut btf;
	fn btf__new_empty_split(base_btf: *mut btf) -> *mut btf;
	fn btf__free(btf: *mut btf);
	fn btf__add_int(btf: *mut btf, name: *const c_char, sz: u32, encoding: u32) -> c_int;
	fn btf__add_ptr(btf: *mut btf, type_id: u32) -> c_int;
	fn btf__add_struct(btf: *mut btf, name: *const c_char, sz: u32) -> c_int;
	fn btf__add_field(
		btf: *mut btf,
		name: *const c_char,
		type_id: u32,
		bit_offset: u32,
		bit_size: u32,
	) -> c_int;
	fn btf__add_func_proto(btf: *mut btf, ret_type_id: u32) -> c_int;
	fn btf__add_func_param(btf: *mut btf, name: *const c_char, type_id: u32) -> c_int;
	fn btf__add_func(
		btf: *mut btf,
		name: *const c_char,
		linkage: u32,
		proto_type_id: u32,
	) -> c_int;
	fn btf__permute(
		btf: *mut btf,
		id_map: *mut __u32,
		id_map_cnt: usize,
		opts: *mut c_void,
	) -> c_int;
	fn btf__type_cnt(btf: *mut btf) -> c_int;
	fn test__start_subtest(name: *const c_char) -> bool;
}

const BTF_INT_SIGNED: u32 = 1;
const BTF_FUNC_STATIC: u32 = 0;

unsafe fn permute_base_check(btf: *mut btf) {
	VALIDATE_RAW_BTF!(
		btf,
		"[1] STRUCT 's2' size=4 vlen=1\n\t'm' type_id=4 bits_offset=0",
		"[2] FUNC 'f' type_id=6 linkage=static",
		"[3] PTR '(anon)' type_id=4",
		"[4] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED",
		"[5] STRUCT 's1' size=4 vlen=1\n\t'm' type_id=4 bits_offset=0",
		"[6] FUNC_PROTO '(anon)' ret_type_id=4 vlen=1\n\t'p' type_id=3"
	);
}

/* Ensure btf__permute works as expected in the base-BTF scenario */
unsafe fn test_permute_base() {
	let mut btf: *mut btf;
	let mut permute_ids: [__u32; 7] = [0; 7];
	let mut err: c_int;

	btf = btf__new_empty();
	if !ASSERT_OK_PTR!(btf, "empty_main_btf") {
		return;
	}

	btf__add_int(btf, c"int".as_ptr(), 4, BTF_INT_SIGNED);	/* [1] int */
	btf__add_ptr(btf, 1);					/* [2] ptr to int */
	btf__add_struct(btf, c"s1".as_ptr(), 4);		/* [3] struct s1 { */
	btf__add_field(btf, c"m".as_ptr(), 1, 0, 0);		/*       int m; */
								/* } */
	btf__add_struct(btf, c"s2".as_ptr(), 4);		/* [4] struct s2 { */
	btf__add_field(btf, c"m".as_ptr(), 1, 0, 0);		/*       int m; */
								/* } */
	btf__add_func_proto(btf, 1);				/* [5] int (*)(int *p); */
	btf__add_func_param(btf, c"p".as_ptr(), 2);
	btf__add_func(btf, c"f".as_ptr(), BTF_FUNC_STATIC, 5);	/* [6] int f(int *p); */

	VALIDATE_RAW_BTF!(
		btf,
		"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED",
		"[2] PTR '(anon)' type_id=1",
		"[3] STRUCT 's1' size=4 vlen=1\n\t'm' type_id=1 bits_offset=0",
		"[4] STRUCT 's2' size=4 vlen=1\n\t'm' type_id=1 bits_offset=0",
		"[5] FUNC_PROTO '(anon)' ret_type_id=1 vlen=1\n\t'p' type_id=2",
		"[6] FUNC 'f' type_id=5 linkage=static"
	);

	permute_ids[0] = 0; /* [0] -> [0] */
	permute_ids[1] = 4; /* [1] -> [4] */
	permute_ids[2] = 3; /* [2] -> [3] */
	permute_ids[3] = 5; /* [3] -> [5] */
	permute_ids[4] = 1; /* [4] -> [1] */
	permute_ids[5] = 6; /* [5] -> [6] */
	permute_ids[6] = 2; /* [6] -> [2] */
	err = btf__permute(btf, permute_ids.as_mut_ptr(), permute_ids.len(), ptr::null_mut());
	if !ASSERT_OK!(err, "btf__permute_base") {
		goto_done(btf);
		return;
	}
	permute_base_check(btf);

	/* ids[0] must be 0 for base BTF */
	permute_ids[0] = 4; /* [0] -> [0] */
	permute_ids[1] = 0; /* [1] -> [4] */
	permute_ids[2] = 3; /* [2] -> [3] */
	permute_ids[3] = 5; /* [3] -> [5] */
	permute_ids[4] = 1; /* [4] -> [1] */
	permute_ids[5] = 6; /* [5] -> [6] */
	permute_ids[6] = 2; /* [6] -> [2] */
	err = btf__permute(btf, permute_ids.as_mut_ptr(), permute_ids.len(), ptr::null_mut());
	if !ASSERT_ERR!(err, "btf__permute_base") {
		goto_done(btf);
		return;
	}
	/* BTF is not modified */
	permute_base_check(btf);

	/* id_map_cnt is invalid */
	permute_ids[0] = 0; /* [0] -> [0] */
	permute_ids[1] = 4; /* [1] -> [4] */
	permute_ids[2] = 3; /* [2] -> [3] */
	permute_ids[3] = 5; /* [3] -> [5] */
	permute_ids[4] = 1; /* [4] -> [1] */
	permute_ids[5] = 6; /* [5] -> [6] */
	permute_ids[6] = 2; /* [6] -> [2] */
	err = btf__permute(btf, permute_ids.as_mut_ptr(), permute_ids.len() - 1, ptr::null_mut());
	if !ASSERT_ERR!(err, "btf__permute_base") {
		goto_done(btf);
		return;
	}
	/* BTF is not modified */
	permute_base_check(btf);

	/* Multiple types can not be mapped to the same ID */
	permute_ids[0] = 0;
	permute_ids[1] = 4;
	permute_ids[2] = 4;
	permute_ids[3] = 5;
	permute_ids[4] = 1;
	permute_ids[5] = 6;
	permute_ids[6] = 2;
	err = btf__permute(btf, permute_ids.as_mut_ptr(), permute_ids.len(), ptr::null_mut());
	if !ASSERT_ERR!(err, "btf__permute_base") {
		goto_done(btf);
		return;
	}
	/* BTF is not modified */
	permute_base_check(btf);

	/* Type ID must be valid */
	permute_ids[0] = 0;
	permute_ids[1] = 4;
	permute_ids[2] = 3;
	permute_ids[3] = 5;
	permute_ids[4] = 1;
	permute_ids[5] = 7;
	permute_ids[6] = 2;
	err = btf__permute(btf, permute_ids.as_mut_ptr(), permute_ids.len(), ptr::null_mut());
	if !ASSERT_ERR!(err, "btf__permute_base") {
		goto_done(btf);
		return;
	}
	/* BTF is not modified */
	permute_base_check(btf);

	goto_done(btf);
}

unsafe fn goto_done(btf: *mut btf) {
	btf__free(btf);
}

unsafe fn permute_split_check(btf: *mut btf) {
	VALIDATE_RAW_BTF!(
		btf,
		"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED",
		"[2] PTR '(anon)' type_id=1",
		"[3] STRUCT 's2' size=4 vlen=1\n\t'm' type_id=1 bits_offset=0",
		"[4] FUNC 'f' type_id=5 linkage=static",
		"[5] FUNC_PROTO '(anon)' ret_type_id=1 vlen=1\n\t'p' type_id=2",
		"[6] STRUCT 's1' size=4 vlen=1\n\t'm' type_id=1 bits_offset=0"
	);
}

/* Ensure btf__permute works as expected in the split-BTF scenario */
unsafe fn test_permute_split() {
	let mut split_btf: *mut btf = ptr::null_mut();
	let mut base_btf: *mut btf = ptr::null_mut();
	let mut permute_ids: [__u32; 4] = [0; 4];
	let mut err: c_int;
	let start_id: c_int;

	base_btf = btf__new_empty();
	if !ASSERT_OK_PTR!(base_btf, "empty_main_btf") {
		return;
	}

	btf__add_int(base_btf, c"int".as_ptr(), 4, BTF_INT_SIGNED);	/* [1] int */
	btf__add_ptr(base_btf, 1);					/* [2] ptr to int */
	VALIDATE_RAW_BTF!(
		base_btf,
		"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED",
		"[2] PTR '(anon)' type_id=1"
	);
	split_btf = btf__new_empty_split(base_btf);
	if !ASSERT_OK_PTR!(split_btf, "empty_split_btf") {
		goto_cleanup(split_btf, base_btf);
		return;
	}
	btf__add_struct(split_btf, c"s1".as_ptr(), 4);		/* [3] struct s1 { */
	btf__add_field(split_btf, c"m".as_ptr(), 1, 0, 0);	/*   int m; */
									/* } */
	btf__add_struct(split_btf, c"s2".as_ptr(), 4);		/* [4] struct s2 { */
	btf__add_field(split_btf, c"m".as_ptr(), 1, 0, 0);	/*   int m; */
									/* } */
	btf__add_func_proto(split_btf, 1);			/* [5] int (*)(int p); */
	btf__add_func_param(split_btf, c"p".as_ptr(), 2);
	btf__add_func(split_btf, c"f".as_ptr(), BTF_FUNC_STATIC, 5);	/* [6] int f(int *p); */

	VALIDATE_RAW_BTF!(
		split_btf,
		"[1] INT 'int' size=4 bits_offset=0 nr_bits=32 encoding=SIGNED",
		"[2] PTR '(anon)' type_id=1",
		"[3] STRUCT 's1' size=4 vlen=1\n\t'm' type_id=1 bits_offset=0",
		"[4] STRUCT 's2' size=4 vlen=1\n\t'm' type_id=1 bits_offset=0",
		"[5] FUNC_PROTO '(anon)' ret_type_id=1 vlen=1\n\t'p' type_id=2",
		"[6] FUNC 'f' type_id=5 linkage=static"
	);

	start_id = btf__type_cnt(base_btf);
	permute_ids[(3 - start_id) as usize] = 6; /* [3] -> [6] */
	permute_ids[(4 - start_id) as usize] = 3; /* [4] -> [3] */
	permute_ids[(5 - start_id) as usize] = 5; /* [5] -> [5] */
	permute_ids[(6 - start_id) as usize] = 4; /* [6] -> [4] */
	err = btf__permute(split_btf, permute_ids.as_mut_ptr(), permute_ids.len(), ptr::null_mut());
	if !ASSERT_OK!(err, "btf__permute_split") {
		goto_cleanup(split_btf, base_btf);
		return;
	}
	permute_split_check(split_btf);

	/*
	 * For split BTF, id_map_cnt must equal to the number of types
	 * added on top of base BTF
	 */
	permute_ids[(3 - start_id) as usize] = 4;
	permute_ids[(4 - start_id) as usize] = 3;
	permute_ids[(5 - start_id) as usize] = 5;
	permute_ids[(6 - start_id) as usize] = 6;
	err = btf__permute(split_btf, permute_ids.as_mut_ptr(), permute_ids.len() - 1, ptr::null_mut());
	if !ASSERT_ERR!(err, "btf__permute_split") {
		goto_cleanup(split_btf, base_btf);
		return;
	}
	/* BTF is not modified */
	permute_split_check(split_btf);

	/* Multiple types can not be mapped to the same ID */
	permute_ids[(3 - start_id) as usize] = 4;
	permute_ids[(4 - start_id) as usize] = 3;
	permute_ids[(5 - start_id) as usize] = 3;
	permute_ids[(6 - start_id) as usize] = 6;
	err = btf__permute(split_btf, permute_ids.as_mut_ptr(), permute_ids.len(), ptr::null_mut());
	if !ASSERT_ERR!(err, "btf__permute_split") {
		goto_cleanup(split_btf, base_btf);
		return;
	}
	/* BTF is not modified */
	permute_split_check(split_btf);

	/* Can not map to base ID */
	permute_ids[(3 - start_id) as usize] = 4;
	permute_ids[(4 - start_id) as usize] = 2;
	permute_ids[(5 - start_id) as usize] = 5;
	permute_ids[(6 - start_id) as usize] = 6;
	err = btf__permute(split_btf, permute_ids.as_mut_ptr(), permute_ids.len(), ptr::null_mut());
	if !ASSERT_ERR!(err, "btf__permute_split") {
		goto_cleanup(split_btf, base_btf);
		return;
	}
	/* BTF is not modified */
	permute_split_check(split_btf);

	goto_cleanup(split_btf, base_btf);
}

unsafe fn goto_cleanup(split_btf: *mut btf, base_btf: *mut btf) {
	btf__free(split_btf);
	btf__free(base_btf);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_btf_permute() {
	if test__start_subtest(c"permute_base".as_ptr()) {
		test_permute_base();
	}
	if test__start_subtest(c"permute_split".as_ptr()) {
		test_permute_split();
	}
}
