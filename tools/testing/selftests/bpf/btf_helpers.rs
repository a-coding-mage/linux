// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2020 Facebook */

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

pub type __u8 = u8;
pub type __u16 = u16;
pub type __u32 = u32;
pub type __u64 = u64;

#[repr(C)]
pub struct FILE {
	_private: [u8; 0],
}

#[repr(C)]
pub struct btf {
	_private: [u8; 0],
}

#[repr(C)]
pub struct btf_dump {
	_private: [u8; 0],
}

#[repr(C)]
pub struct btf_type {
	pub name_off: __u32,
	pub info: __u32,
	pub size: __u32,
	pub type_: __u32,
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
	pub val: c_int,
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
	pub component_idx: c_int,
}

unsafe extern "C" {
	static mut stderr: *mut FILE;
	static mut errno: c_int;

	fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
	fn vfprintf(stream: *mut FILE, format: *const c_char, args: *mut c_void) -> c_int;
	fn fflush(stream: *mut FILE) -> c_int;
	fn fclose(stream: *mut FILE) -> c_int;
	fn fmemopen(buf: *mut c_void, size: usize, mode: *const c_char) -> *mut FILE;

	fn btf__str_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
	fn btf__type_by_id(btf: *const btf, id: __u32) -> *const btf_type;
	fn btf__type_cnt(btf: *const btf) -> c_int;
	fn btf_vlen(t: *const btf_type) -> __u32;
	fn btf_kind(t: *const btf_type) -> c_int;
	fn btf_int_offset(t: *const btf_type) -> __u32;
	fn btf_int_bits(t: *const btf_type) -> __u32;
	fn btf_int_encoding(t: *const btf_type) -> __u8;
	fn btf_array(t: *const btf_type) -> *const btf_array;
	fn btf_members(t: *const btf_type) -> *const btf_member;
	fn btf_member_bit_offset(t: *const btf_type, member_idx: c_int) -> __u32;
	fn btf_member_bitfield_size(t: *const btf_type, member_idx: c_int) -> __u32;
	fn btf_enum(t: *const btf_type) -> *const btf_enum;
	fn btf_enum64(t: *const btf_type) -> *const btf_enum64;
	fn btf_kflag(t: *const btf_type) -> bool;
	fn btf_params(t: *const btf_type) -> *const btf_param;
	fn btf_var(t: *const btf_type) -> *const btf_var;
	fn btf_var_secinfos(t: *const btf_type) -> *const btf_var_secinfo;
	fn btf_decl_tag(t: *const btf_type) -> *const btf_decl_tag;

	fn btf_dump__new(
		btf: *const btf,
		printf_fn: unsafe extern "C" fn(*mut c_void, *const c_char, *mut c_void),
		ctx: *mut c_void,
		opts: *mut c_void,
	) -> *mut btf_dump;
	fn btf_dump__dump_type(d: *mut btf_dump, id: c_int) -> c_int;
	fn btf_dump__free(d: *mut btf_dump);
	fn libbpf_get_error(ptr: *const c_void) -> c_long;

	fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
	fn ASSERT_STREQ(actual: *const c_char, expected: *const c_char, name: *const c_char) -> bool;
}

const EINVAL: c_int = 22;

const BTF_KIND_UNKN: c_int = 0;
const BTF_KIND_INT: c_int = 1;
const BTF_KIND_PTR: c_int = 2;
const BTF_KIND_ARRAY: c_int = 3;
const BTF_KIND_STRUCT: c_int = 4;
const BTF_KIND_UNION: c_int = 5;
const BTF_KIND_ENUM: c_int = 6;
const BTF_KIND_FWD: c_int = 7;
const BTF_KIND_TYPEDEF: c_int = 8;
const BTF_KIND_VOLATILE: c_int = 9;
const BTF_KIND_CONST: c_int = 10;
const BTF_KIND_RESTRICT: c_int = 11;
const BTF_KIND_FUNC: c_int = 12;
const BTF_KIND_FUNC_PROTO: c_int = 13;
const BTF_KIND_VAR: c_int = 14;
const BTF_KIND_DATASEC: c_int = 15;
const BTF_KIND_FLOAT: c_int = 16;
const BTF_KIND_DECL_TAG: c_int = 17;
const BTF_KIND_TYPE_TAG: c_int = 18;
const BTF_KIND_ENUM64: c_int = 19;

const BTF_INT_SIGNED: __u8 = 1;
const BTF_INT_CHAR: __u8 = 2;
const BTF_INT_BOOL: __u8 = 4;

const BTF_VAR_STATIC: __u32 = 0;
const BTF_VAR_GLOBAL_ALLOCATED: __u32 = 1;

const BTF_FUNC_STATIC: __u32 = 0;
const BTF_FUNC_GLOBAL: __u32 = 1;
const BTF_FUNC_EXTERN: __u32 = 2;

static BTF_KIND_STR_MAPPING: [&[u8]; 20] = [
	b"UNKNOWN\0",
	b"INT\0",
	b"PTR\0",
	b"ARRAY\0",
	b"STRUCT\0",
	b"UNION\0",
	b"ENUM\0",
	b"FWD\0",
	b"TYPEDEF\0",
	b"VOLATILE\0",
	b"CONST\0",
	b"RESTRICT\0",
	b"FUNC\0",
	b"FUNC_PROTO\0",
	b"VAR\0",
	b"DATASEC\0",
	b"FLOAT\0",
	b"DECL_TAG\0",
	b"TYPE_TAG\0",
	b"ENUM64\0",
];

unsafe fn btf_kind_str(kind: __u16) -> *const c_char {
	if kind as c_int > BTF_KIND_ENUM64 {
		return c"UNKNOWN".as_ptr();
	}
	BTF_KIND_STR_MAPPING[kind as usize].as_ptr() as *const c_char
}

unsafe fn btf_int_enc_str(encoding: __u8) -> *const c_char {
	match encoding {
		0 => c"(none)".as_ptr(),
		BTF_INT_SIGNED => c"SIGNED".as_ptr(),
		BTF_INT_CHAR => c"CHAR".as_ptr(),
		BTF_INT_BOOL => c"BOOL".as_ptr(),
		_ => c"UNKN".as_ptr(),
	}
}

unsafe fn btf_var_linkage_str(linkage: __u32) -> *const c_char {
	match linkage {
		BTF_VAR_STATIC => c"static".as_ptr(),
		BTF_VAR_GLOBAL_ALLOCATED => c"global-alloc".as_ptr(),
		_ => c"(unknown)".as_ptr(),
	}
}

unsafe fn btf_func_linkage_str(t: *const btf_type) -> *const c_char {
	match btf_vlen(t) {
		BTF_FUNC_STATIC => c"static".as_ptr(),
		BTF_FUNC_GLOBAL => c"global".as_ptr(),
		BTF_FUNC_EXTERN => c"extern".as_ptr(),
		_ => c"(unknown)".as_ptr(),
	}
}

unsafe fn btf_str(btf: *const btf, off: __u32) -> *const c_char {
	if off == 0 {
		return c"(anon)".as_ptr();
	}
	let s = btf__str_by_offset(btf, off);
	if s.is_null() {
		c"(invalid)".as_ptr()
	} else {
		s
	}
}

#[no_mangle]
pub unsafe extern "C" fn fprintf_btf_type_raw(out: *mut FILE, btf: *const btf, id: __u32) -> c_int {
	let t: *const btf_type;
	let kind: c_int;
	let mut i: c_int;
	let vlen: __u32;

	t = btf__type_by_id(btf, id);
	if t.is_null() {
		return -EINVAL;
	}

	vlen = btf_vlen(t);
	kind = btf_kind(t);

	fprintf(
		out,
		c"[%u] %s '%s'".as_ptr(),
		id,
		btf_kind_str(kind as __u16),
		btf_str(btf, (*t).name_off),
	);

	match kind {
		BTF_KIND_INT => {
			fprintf(
				out,
				c" size=%u bits_offset=%u nr_bits=%u encoding=%s".as_ptr(),
				(*t).size,
				btf_int_offset(t),
				btf_int_bits(t),
				btf_int_enc_str(btf_int_encoding(t)),
			);
		}
		BTF_KIND_PTR | BTF_KIND_CONST | BTF_KIND_VOLATILE | BTF_KIND_RESTRICT | BTF_KIND_TYPEDEF
		| BTF_KIND_TYPE_TAG => {
			fprintf(out, c" type_id=%u".as_ptr(), (*t).type_);
		}
		BTF_KIND_ARRAY => {
			let arr = btf_array(t);

			fprintf(
				out,
				c" type_id=%u index_type_id=%u nr_elems=%u".as_ptr(),
				(*arr).type_,
				(*arr).index_type,
				(*arr).nelems,
			);
		}
		BTF_KIND_STRUCT | BTF_KIND_UNION => {
			let mut m = btf_members(t);

			fprintf(out, c" size=%u vlen=%u".as_ptr(), (*t).size, vlen);
			i = 0;
			while i < vlen as c_int {
				let bit_off: __u32;
				let bit_sz: __u32;

				bit_off = btf_member_bit_offset(t, i);
				bit_sz = btf_member_bitfield_size(t, i);
				fprintf(
					out,
					c"\n\t'%s' type_id=%u bits_offset=%u".as_ptr(),
					btf_str(btf, (*m).name_off),
					(*m).type_,
					bit_off,
				);
				if bit_sz != 0 {
					fprintf(out, c" bitfield_size=%u".as_ptr(), bit_sz);
				}
				i += 1;
				m = m.add(1);
			}
		}
		BTF_KIND_ENUM => {
			let mut v = btf_enum(t);
			let fmt_str: *const c_char;

			fmt_str = if btf_kflag(t) {
				c"\n\t'%s' val=%d".as_ptr()
			} else {
				c"\n\t'%s' val=%u".as_ptr()
			};
			fprintf(
				out,
				c" encoding=%s size=%u vlen=%u".as_ptr(),
				if btf_kflag(t) {
					c"SIGNED".as_ptr()
				} else {
					c"UNSIGNED".as_ptr()
				},
				(*t).size,
				vlen,
			);
			i = 0;
			while i < vlen as c_int {
				fprintf(out, fmt_str, btf_str(btf, (*v).name_off), (*v).val);
				i += 1;
				v = v.add(1);
			}
		}
		BTF_KIND_ENUM64 => {
			let mut v = btf_enum64(t);
			let fmt_str: *const c_char;

			fmt_str = if btf_kflag(t) {
				c"\n\t'%s' val=%lld".as_ptr()
			} else {
				c"\n\t'%s' val=%llu".as_ptr()
			};

			fprintf(
				out,
				c" encoding=%s size=%u vlen=%u".as_ptr(),
				if btf_kflag(t) {
					c"SIGNED".as_ptr()
				} else {
					c"UNSIGNED".as_ptr()
				},
				(*t).size,
				vlen,
			);
			i = 0;
			while i < vlen as c_int {
				fprintf(
					out,
					fmt_str,
					btf_str(btf, (*v).name_off),
					(((*v).val_hi32 as __u64) << 32) | (*v).val_lo32 as __u64,
				);
				i += 1;
				v = v.add(1);
			}
		}
		BTF_KIND_FWD => {
			fprintf(
				out,
				c" fwd_kind=%s".as_ptr(),
				if btf_kflag(t) {
					c"union".as_ptr()
				} else {
					c"struct".as_ptr()
				},
			);
		}
		BTF_KIND_FUNC => {
			fprintf(
				out,
				c" type_id=%u linkage=%s".as_ptr(),
				(*t).type_,
				btf_func_linkage_str(t),
			);
		}
		BTF_KIND_FUNC_PROTO => {
			let mut p = btf_params(t);

			fprintf(out, c" ret_type_id=%u vlen=%u".as_ptr(), (*t).type_, vlen);
			i = 0;
			while i < vlen as c_int {
				fprintf(
					out,
					c"\n\t'%s' type_id=%u".as_ptr(),
					btf_str(btf, (*p).name_off),
					(*p).type_,
				);
				i += 1;
				p = p.add(1);
			}
		}
		BTF_KIND_VAR => {
			fprintf(
				out,
				c" type_id=%u, linkage=%s".as_ptr(),
				(*t).type_,
				btf_var_linkage_str((*btf_var(t)).linkage),
			);
		}
		BTF_KIND_DATASEC => {
			let mut v = btf_var_secinfos(t);

			fprintf(out, c" size=%u vlen=%u".as_ptr(), (*t).size, vlen);
			i = 0;
			while i < vlen as c_int {
				fprintf(
					out,
					c"\n\ttype_id=%u offset=%u size=%u".as_ptr(),
					(*v).type_,
					(*v).offset,
					(*v).size,
				);
				i += 1;
				v = v.add(1);
			}
		}
		BTF_KIND_FLOAT => {
			fprintf(out, c" size=%u".as_ptr(), (*t).size);
		}
		BTF_KIND_DECL_TAG => {
			fprintf(
				out,
				c" type_id=%u component_idx=%d".as_ptr(),
				(*t).type_,
				(*btf_decl_tag(t)).component_idx,
			);
		}
		_ => {}
	}

	0
}

/* Print raw BTF type dump into a local buffer and return string pointer back.
 * Buffer *will* be overwritten by subsequent btf_type_raw_dump() calls
 */
#[no_mangle]
pub unsafe extern "C" fn btf_type_raw_dump(btf: *const btf, type_id: c_int) -> *const c_char {
	static mut BUF: [c_char; 16 * 1024] = [0; 16 * 1024];
	let buf_file: *mut FILE;

	buf_file = fmemopen(BUF.as_mut_ptr() as *mut c_void, BUF.len() - 1, c"w".as_ptr());
	if buf_file.is_null() {
		fprintf(
			stderr,
			c"Failed to open memstream: %d\n".as_ptr(),
			errno,
		);
		return ptr::null();
	}

	fprintf_btf_type_raw(buf_file, btf, type_id as __u32);
	fflush(buf_file);
	fclose(buf_file);

	BUF.as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn btf_validate_raw(
	btf: *mut btf,
	nr_types: c_int,
	exp_types: *mut *const c_char,
) -> c_int {
	let mut i: c_int;
	let mut ok: bool = true;

	ASSERT_EQ(btf__type_cnt(btf) - 1, nr_types, c"btf_nr_types".as_ptr());

	i = 1;
	while i <= nr_types {
		if !ASSERT_STREQ(
			btf_type_raw_dump(btf, i),
			*exp_types.offset((i - 1) as isize),
			c"raw_dump".as_ptr(),
		) {
			ok = false;
		}
		i += 1;
	}

	ok as c_int
}

unsafe extern "C" fn btf_dump_printf(ctx: *mut c_void, fmt: *const c_char, args: *mut c_void) {
	vfprintf(ctx as *mut FILE, fmt, args);
}

/* Print BTF-to-C dump into a local buffer and return string pointer back.
 * Buffer *will* be overwritten by subsequent btf_type_raw_dump() calls
 */
#[no_mangle]
pub unsafe extern "C" fn btf_type_c_dump(btf: *const btf) -> *const c_char {
	static mut BUF: [c_char; 16 * 1024] = [0; 16 * 1024];
	let buf_file: *mut FILE;
	let mut d: *mut btf_dump = ptr::null_mut();
	let mut err: c_int;
	let mut i: c_int;

	buf_file = fmemopen(BUF.as_mut_ptr() as *mut c_void, BUF.len() - 1, c"w".as_ptr());
	if buf_file.is_null() {
		fprintf(
			stderr,
			c"Failed to open memstream: %d\n".as_ptr(),
			errno,
		);
		return ptr::null();
	}

	d = btf_dump__new(btf, btf_dump_printf, buf_file as *mut c_void, ptr::null_mut());
	if libbpf_get_error(d as *const c_void) != 0 {
		fprintf(
			stderr,
			c"Failed to create btf_dump instance: %ld\n".as_ptr(),
			libbpf_get_error(d as *const c_void),
		);
		goto_err_out(d, buf_file);
		return ptr::null();
	}

	i = 1;
	while i < btf__type_cnt(btf) {
		err = btf_dump__dump_type(d, i);
		if err != 0 {
			fprintf(
				stderr,
				c"Failed to dump type [%d]: %d\n".as_ptr(),
				i,
				err,
			);
			goto_err_out(d, buf_file);
			return ptr::null();
		}
		i += 1;
	}

	btf_dump__free(d);
	fflush(buf_file);
	fclose(buf_file);
	BUF.as_ptr()
}

unsafe fn goto_err_out(d: *mut btf_dump, buf_file: *mut FILE) {
	btf_dump__free(d);
	fclose(buf_file);
}
