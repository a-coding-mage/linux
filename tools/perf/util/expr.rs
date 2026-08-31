// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type u64 = u64;
type YY_BUFFER_STATE = *mut c_void;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;

#[repr(C)]
pub struct hashmap {
	_private: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
	pub key: c_long,
	pub value: *mut c_void,
	pub pkey: *mut c_char,
	pub pvalue: *mut c_void,
}

#[repr(C)]
pub struct expr_scanner_ctx {
	pub system_wide: bool,
	pub user_requested_cpu_list: *mut c_char,
}

#[repr(C)]
pub struct expr_parse_ctx {
	pub ids: *mut hashmap,
	pub sctx: expr_scanner_ctx,
}

#[repr(C)]
pub struct metric_ref {
	pub metric_name: *const c_char,
	pub metric_expr: *const c_char,
}

#[repr(C)]
pub struct evlist {
	_private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu {
	pub cpu: c_int,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tool_pmu_event {
	TOOL_PMU__EVENT_NONE = 0,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct expr_id_data_value {
	pub val: f64,
	pub source_count: c_int,
	pub aggr_nr: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct expr_id_data_ref {
	pub val: f64,
	pub metric_name: *const c_char,
	pub metric_expr: *const c_char,
}

#[repr(C)]
pub union expr_id_data_union {
	pub val: expr_id_data_value,
	pub ref_: expr_id_data_ref,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum expr_id_data_kind {
	/* Holding a double value. */
	EXPR_ID_DATA__VALUE,
	/* Reference to another metric. */
	EXPR_ID_DATA__REF,
	/* A reference but the value has been computed. */
	EXPR_ID_DATA__REF_VALUE,
}

#[repr(C)]
pub struct expr_id_data {
	pub data: expr_id_data_union,
	pub kind: expr_id_data_kind,
}

unsafe extern "C" {
	fn malloc(size: size_t) -> *mut c_void;
	fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
	fn free(ptr: *mut c_void);
	fn strdup(s: *const c_char) -> *mut c_char;
	fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
	fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
	fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;

	fn hashmap__new(
		hash_fn: Option<unsafe extern "C" fn(c_long, *mut c_void) -> size_t>,
		equal_fn: Option<unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool>,
		ctx: *mut c_void,
	) -> *mut hashmap;
	fn hashmap__free(map: *mut hashmap);
	fn hashmap__clear(map: *mut hashmap);
	fn hashmap__set(
		map: *mut hashmap,
		key: *const c_char,
		value: *mut expr_id_data,
		old_key: *mut *mut c_char,
		old_value: *mut *mut expr_id_data,
	) -> c_int;
	fn hashmap__find(map: *mut hashmap, key: *const c_char, value: *mut *mut expr_id_data) -> bool;
	fn hashmap__delete(
		map: *mut hashmap,
		key: *const c_char,
		old_key: *mut *mut c_char,
		old_value: *mut *mut expr_id_data,
	);
	fn hashmap__size(map: *mut hashmap) -> size_t;
	fn hashmap__for_each_entry(
		map: *mut hashmap,
		callback: unsafe extern "C" fn(*mut hashmap_entry, size_t, *mut c_void) -> c_int,
		ctx: *mut c_void,
	) -> c_int;

	fn IS_ERR(ptr: *const c_void) -> bool;

	fn expr_lex_init_extra(sctx: *mut expr_scanner_ctx, scanner: *mut *mut c_void) -> c_int;
	fn expr__scan_string(expr: *const c_char, scanner: *mut c_void) -> YY_BUFFER_STATE;
	fn expr_set_debug(debug: c_int, scanner: *mut c_void);
	fn expr_parse(
		val: *mut f64,
		ctx: *mut expr_parse_ctx,
		compute_ids: bool,
		scanner: *mut c_void,
	) -> c_int;
	fn expr__flush_buffer(buffer: YY_BUFFER_STATE, scanner: *mut c_void);
	fn expr__delete_buffer(buffer: YY_BUFFER_STATE, scanner: *mut c_void);
	fn expr_lex_destroy(scanner: *mut c_void) -> c_int;

	static mut expr_debug: c_int;

	fn tool_pmu__str_to_event(name: *const c_char) -> tool_pmu_event;
	fn tool_pmu__read_event(
		ev: tool_pmu_event,
		evsel: *mut c_void,
		system_wide: bool,
		user_requested_cpu_list: *mut c_char,
		count: *mut u64,
	) -> bool;
	fn evlist__new() -> *mut evlist;
	fn evlist__put(evlist: *mut evlist);
	fn parse_event(evlist: *mut evlist, str_: *const c_char) -> c_int;
	fn get_cpuid_allow_env_override(cpu: perf_cpu) -> *mut c_char;
	fn strcmp_cpuid_str(s1: *const c_char, s2: *const c_char) -> c_int;

	fn pr_debug(fmt: *const c_char, ...);
	fn pr_debug2(fmt: *const c_char, ...);
	fn pr_err(fmt: *const c_char, ...);
}

unsafe fn zfree<T>(ptrp: *mut *mut T) {
	if !(*ptrp).is_null() {
		free(*ptrp as *mut c_void);
		*ptrp = ptr::null_mut();
	}
}

unsafe extern "C" fn key_hash(key: c_long, _ctx: *mut c_void) -> size_t {
	let mut str_ = key as *const c_char;
	let mut hash: size_t = 0;

	while *str_ != 0 {
		hash = hash.wrapping_mul(31);
		hash = hash.wrapping_add(*str_ as size_t);
		str_ = str_.add(1);
	}
	hash
}

unsafe extern "C" fn key_equal(key1: c_long, key2: c_long, _ctx: *mut c_void) -> bool {
	strcmp(key1 as *const c_char, key2 as *const c_char) == 0
}

#[no_mangle]
pub unsafe extern "C" fn ids__new() -> *mut hashmap {
	let hash = hashmap__new(Some(key_hash), Some(key_equal), ptr::null_mut());
	if IS_ERR(hash as *const c_void) {
		return ptr::null_mut();
	}
	hash
}

unsafe extern "C" fn ids_free_entry(cur: *mut hashmap_entry, _bkt: size_t, _ctx: *mut c_void) -> c_int {
	zfree(&mut (*cur).pkey);
	zfree(&mut (*cur).pvalue);
	0
}

#[no_mangle]
pub unsafe extern "C" fn ids__free(ids: *mut hashmap) {
	if ids.is_null() {
		return;
	}

	hashmap__for_each_entry(ids, ids_free_entry, ptr::null_mut());
	hashmap__free(ids);
}

#[no_mangle]
pub unsafe extern "C" fn ids__insert(ids: *mut hashmap, id: *const c_char) -> c_int {
	let data_ptr: *mut expr_id_data = ptr::null_mut();
	let mut old_data: *mut expr_id_data = ptr::null_mut();
	let mut old_key: *mut c_char = ptr::null_mut();
	let ret = hashmap__set(ids, id, data_ptr, &mut old_key, &mut old_data);
	if ret != 0 {
		free(data_ptr as *mut c_void);
	}
	free(old_key as *mut c_void);
	free(old_data as *mut c_void);
	ret
}

unsafe extern "C" fn ids_union_entry(cur: *mut hashmap_entry, _bkt: size_t, ctx: *mut c_void) -> c_int {
	let ids1 = ctx as *mut hashmap;
	let mut old_data: *mut expr_id_data = ptr::null_mut();
	let mut old_key: *mut c_char = ptr::null_mut();
	let ret = hashmap__set(ids1, (*cur).key as *const c_char, (*cur).value as *mut expr_id_data, &mut old_key, &mut old_data);
	free(old_key as *mut c_void);
	free(old_data as *mut c_void);
	ret
}

#[no_mangle]
pub unsafe extern "C" fn ids__union(mut ids1: *mut hashmap, mut ids2: *mut hashmap) -> *mut hashmap {
	if ids1.is_null() {
		return ids2;
	}
	if ids2.is_null() {
		return ids1;
	}

	if hashmap__size(ids1) < hashmap__size(ids2) {
		mem::swap(&mut ids1, &mut ids2);
	}
	if hashmap__for_each_entry(ids2, ids_union_entry, ids1 as *mut c_void) != 0 {
		hashmap__free(ids1);
		hashmap__free(ids2);
		return ptr::null_mut();
	}
	hashmap__free(ids2);
	ids1
}

/* Caller must make sure id is allocated */
#[no_mangle]
pub unsafe extern "C" fn expr__add_id(ctx: *mut expr_parse_ctx, id: *const c_char) -> c_int {
	ids__insert((*ctx).ids, id)
}

/* Caller must make sure id is allocated */
#[no_mangle]
pub unsafe extern "C" fn expr__add_id_val(ctx: *mut expr_parse_ctx, id: *const c_char, val: f64) -> c_int {
	expr__add_id_val_source_count(ctx, id, val, 1)
}

/* Caller must make sure id is allocated */
#[no_mangle]
pub unsafe extern "C" fn expr__add_id_val_source_count_aggr_nr(
	ctx: *mut expr_parse_ctx,
	id: *const c_char,
	val: f64,
	source_count: c_int,
	aggr_nr: c_int,
) -> c_int {
	let mut data_ptr: *mut expr_id_data = ptr::null_mut();
	let mut old_data: *mut expr_id_data = ptr::null_mut();
	let mut old_key: *mut c_char = ptr::null_mut();

	data_ptr = malloc(mem::size_of::<expr_id_data>()) as *mut expr_id_data;
	if data_ptr.is_null() {
		return -ENOMEM;
	}
	(*data_ptr).data.val.val = val;
	(*data_ptr).data.val.source_count = source_count;
	(*data_ptr).data.val.aggr_nr = aggr_nr;
	(*data_ptr).kind = expr_id_data_kind::EXPR_ID_DATA__VALUE;

	let ret = hashmap__set((*ctx).ids, id, data_ptr, &mut old_key, &mut old_data);
	if ret != 0 {
		free(data_ptr as *mut c_void);
	} else if !old_data.is_null() {
		(*data_ptr).data.val.val += (*old_data).data.val.val;
		(*data_ptr).data.val.source_count += (*old_data).data.val.source_count;
		(*data_ptr).data.val.aggr_nr += (*old_data).data.val.aggr_nr;
	}
	free(old_key as *mut c_void);
	free(old_data as *mut c_void);
	ret
}

/* Caller must make sure id is allocated */
#[no_mangle]
pub unsafe extern "C" fn expr__add_id_val_source_count(
	ctx: *mut expr_parse_ctx,
	id: *const c_char,
	val: f64,
	source_count: c_int,
) -> c_int {
	expr__add_id_val_source_count_aggr_nr(ctx, id, val, source_count, 1)
}

#[no_mangle]
pub unsafe extern "C" fn expr__add_ref(ctx: *mut expr_parse_ctx, ref_: *mut metric_ref) -> c_int {
	let data_ptr = calloc(1, mem::size_of::<expr_id_data>()) as *mut expr_id_data;
	if data_ptr.is_null() {
		return -ENOMEM;
	}

	let name = strdup((*ref_).metric_name);
	if name.is_null() {
		free(data_ptr as *mut c_void);
		return -ENOMEM;
	}

	/*
	 * Intentionally passing just const char pointers,
	 * originally from 'struct pmu_event' object.
	 * We don't need to change them, so there's no
	 * need to create our own copy.
	 */
	(*data_ptr).data.ref_.metric_name = (*ref_).metric_name;
	(*data_ptr).data.ref_.metric_expr = (*ref_).metric_expr;
	(*data_ptr).kind = expr_id_data_kind::EXPR_ID_DATA__REF;

	let mut old_data: *mut expr_id_data = ptr::null_mut();
	let mut old_key: *mut c_char = ptr::null_mut();
	let ret = hashmap__set((*ctx).ids, name, data_ptr, &mut old_key, &mut old_data);
	if ret != 0 {
		free(data_ptr as *mut c_void);
	}

	pr_debug2(c"adding ref metric %s: %s\n".as_ptr(), (*ref_).metric_name, (*ref_).metric_expr);

	free(old_key as *mut c_void);
	free(old_data as *mut c_void);
	ret
}

#[no_mangle]
pub unsafe extern "C" fn expr__get_id(
	ctx: *mut expr_parse_ctx,
	id: *const c_char,
	data: *mut *mut expr_id_data,
) -> c_int {
	if ctx.is_null() || id.is_null() {
		return -1;
	}
	if hashmap__find((*ctx).ids, id, data) { 0 } else { -1 }
}

unsafe extern "C" fn subset_entry(cur: *mut hashmap_entry, _bkt: size_t, ctx: *mut c_void) -> c_int {
	let haystack = ctx as *mut expr_parse_ctx;
	let mut data: *mut expr_id_data = ptr::null_mut();
	if expr__get_id(haystack, (*cur).pkey, &mut data) != 0 {
		return 1;
	}
	0
}

#[no_mangle]
pub unsafe extern "C" fn expr__subset_of_ids(haystack: *mut expr_parse_ctx, needles: *mut expr_parse_ctx) -> bool {
	hashmap__for_each_entry((*needles).ids, subset_entry, haystack as *mut c_void) == 0
}

#[no_mangle]
pub unsafe extern "C" fn expr__resolve_id(
	ctx: *mut expr_parse_ctx,
	id: *const c_char,
	datap: *mut *mut expr_id_data,
) -> c_int {
	let mut data: *mut expr_id_data;

	if expr__get_id(ctx, id, datap) != 0 || (*datap).is_null() {
		pr_debug(c"%s not found\n".as_ptr(), id);
		return -1;
	}

	data = *datap;

	match (*data).kind {
		expr_id_data_kind::EXPR_ID_DATA__VALUE => {
			pr_debug2(c"lookup(%s): val %f\n".as_ptr(), id, (*data).data.val.val);
		}
		expr_id_data_kind::EXPR_ID_DATA__REF => {
			pr_debug2(c"lookup(%s): ref metric name %s\n".as_ptr(), id, (*data).data.ref_.metric_name);
			pr_debug(c"processing metric: %s ENTRY\n".as_ptr(), id);
			(*data).kind = expr_id_data_kind::EXPR_ID_DATA__REF_VALUE;
			if expr__parse(&mut (*data).data.ref_.val, ctx, (*data).data.ref_.metric_expr) != 0 {
				pr_debug(c"%s failed to count\n".as_ptr(), id);
				return -1;
			}
			pr_debug(c"processing metric: %s EXIT: %f\n".as_ptr(), id, (*data).data.ref_.val);
		}
		expr_id_data_kind::EXPR_ID_DATA__REF_VALUE => {
			pr_debug2(
				c"lookup(%s): ref val %f metric name %s\n".as_ptr(),
				id,
				(*data).data.ref_.val,
				(*data).data.ref_.metric_name,
			);
		}
	}

	0
}

#[no_mangle]
pub unsafe extern "C" fn expr__del_id(ctx: *mut expr_parse_ctx, id: *const c_char) {
	let mut old_val: *mut expr_id_data = ptr::null_mut();
	let mut old_key: *mut c_char = ptr::null_mut();

	hashmap__delete((*ctx).ids, id, &mut old_key, &mut old_val);
	free(old_key as *mut c_void);
	free(old_val as *mut c_void);
}

#[no_mangle]
pub unsafe extern "C" fn expr__ctx_new() -> *mut expr_parse_ctx {
	let ctx = calloc(1, mem::size_of::<expr_parse_ctx>()) as *mut expr_parse_ctx;
	if ctx.is_null() {
		return ptr::null_mut();
	}

	(*ctx).ids = hashmap__new(Some(key_hash), Some(key_equal), ptr::null_mut());
	if IS_ERR((*ctx).ids as *const c_void) {
		free(ctx as *mut c_void);
		return ptr::null_mut();
	}

	ctx
}

unsafe extern "C" fn ctx_clear_entry(cur: *mut hashmap_entry, _bkt: size_t, _ctx: *mut c_void) -> c_int {
	zfree(&mut (*cur).pkey);
	zfree(&mut (*cur).pvalue);
	0
}

#[no_mangle]
pub unsafe extern "C" fn expr__ctx_clear(ctx: *mut expr_parse_ctx) {
	hashmap__for_each_entry((*ctx).ids, ctx_clear_entry, ptr::null_mut());
	hashmap__clear((*ctx).ids);
}

unsafe extern "C" fn ctx_free_entry(cur: *mut hashmap_entry, _bkt: size_t, _ctx: *mut c_void) -> c_int {
	zfree(&mut (*cur).pkey);
	zfree(&mut (*cur).pvalue);
	0
}

#[no_mangle]
pub unsafe extern "C" fn expr__ctx_free(ctx: *mut expr_parse_ctx) {
	if ctx.is_null() {
		return;
	}

	zfree(&mut (*ctx).sctx.user_requested_cpu_list);
	hashmap__for_each_entry((*ctx).ids, ctx_free_entry, ptr::null_mut());
	hashmap__free((*ctx).ids);
	free(ctx as *mut c_void);
}

unsafe fn __expr__parse(
	val: *mut f64,
	ctx: *mut expr_parse_ctx,
	expr: *const c_char,
	compute_ids: bool,
) -> c_int {
	let mut buffer: YY_BUFFER_STATE;
	let mut scanner: *mut c_void = ptr::null_mut();

	pr_debug2(c"parsing metric: %s\n".as_ptr(), expr);

	let mut ret = expr_lex_init_extra(&mut (*ctx).sctx, &mut scanner);
	if ret != 0 {
		return ret;
	}

	buffer = expr__scan_string(expr, scanner);

	/* PARSER_DEBUG: enable expr_debug and scanner parser tracing when built with parser debug support. */

	ret = expr_parse(val, ctx, compute_ids, scanner);

	expr__flush_buffer(buffer, scanner);
	expr__delete_buffer(buffer, scanner);
	expr_lex_destroy(scanner);
	ret
}

#[no_mangle]
pub unsafe extern "C" fn expr__parse(final_val: *mut f64, ctx: *mut expr_parse_ctx, expr: *const c_char) -> c_int {
	if __expr__parse(final_val, ctx, expr, false) != 0 { -1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn expr__find_ids(
	expr: *const c_char,
	one: *const c_char,
	ctx: *mut expr_parse_ctx,
) -> c_int {
	let ret = __expr__parse(ptr::null_mut(), ctx, expr, true);

	if !one.is_null() {
		expr__del_id(ctx, one);
	}

	/* A positive value means syntax error, convert to -EINVAL */
	if ret > 0 { -EINVAL } else { ret }
}

#[no_mangle]
pub unsafe extern "C" fn expr_id_data__value(data: *const expr_id_data) -> f64 {
	if (*data).kind == expr_id_data_kind::EXPR_ID_DATA__VALUE {
		return (*data).data.val.val;
	}
	assert!((*data).kind == expr_id_data_kind::EXPR_ID_DATA__REF_VALUE);
	(*data).data.ref_.val
}

#[no_mangle]
pub unsafe extern "C" fn expr_id_data__source_count(data: *const expr_id_data) -> f64 {
	if (*data).kind == expr_id_data_kind::EXPR_ID_DATA__VALUE {
		return (*data).data.val.source_count as f64;
	}
	1.0
}

#[no_mangle]
pub unsafe extern "C" fn expr_id_data__aggr_nr(data: *const expr_id_data) -> f64 {
	if (*data).kind == expr_id_data_kind::EXPR_ID_DATA__VALUE {
		return (*data).data.val.aggr_nr as f64;
	}
	1.0
}

#[no_mangle]
pub unsafe extern "C" fn expr__get_literal(literal: *const c_char, ctx: *const expr_scanner_ctx) -> f64 {
	let mut result = f64::NAN;
	let ev = tool_pmu__str_to_event(literal.add(1));

	if ev != tool_pmu_event::TOOL_PMU__EVENT_NONE {
		let mut count: u64 = 0;

		if tool_pmu__read_event(
			ev,
			ptr::null_mut(),
			(*ctx).system_wide,
			(*ctx).user_requested_cpu_list,
			&mut count,
		) {
			result = count as f64;
		} else {
			pr_err(c"Failure to read '%s'\n".as_ptr(), literal);
		}
	} else {
		pr_err(c"Unrecognized literal '%s'\n".as_ptr(), literal);
	}

	pr_debug2(c"literal: %s = %f\n".as_ptr(), literal, result);
	result
}

/* Does the event 'id' parse? Determine via ctx->ids if possible. */
#[no_mangle]
pub unsafe extern "C" fn expr__has_event(
	ctx: *const expr_parse_ctx,
	compute_ids: bool,
	id: *const c_char,
) -> f64 {
	let mut tmp: *mut evlist;
	let ret: f64;

	if hashmap__find((*ctx).ids, id, ptr::null_mut()) {
		return 1.0;
	}

	if !compute_ids {
		return 0.0;
	}

	tmp = evlist__new();
	if tmp.is_null() {
		return f64::NAN;
	}

	if !strchr(id, '@' as c_int).is_null() {
		let tmp_id = strdup(id);
		if tmp_id.is_null() {
			ret = f64::NAN;
			evlist__put(tmp);
			return ret;
		}
		let mut p = strchr(tmp_id, '@' as c_int);
		*p = '/' as c_char;
		p = strrchr(tmp_id, '@' as c_int);
		*p = '/' as c_char;
		ret = if parse_event(tmp, tmp_id) != 0 { 0.0 } else { 1.0 };
		free(tmp_id as *mut c_void);
	} else {
		ret = if parse_event(tmp, id) != 0 { 0.0 } else { 1.0 };
	}

	evlist__put(tmp);
	ret
}

#[no_mangle]
pub unsafe extern "C" fn expr__strcmp_cpuid_str(
	_ctx: *const expr_parse_ctx,
	_compute_ids: bool,
	test_id: *const c_char,
) -> f64 {
	let ret: f64;
	let cpu = perf_cpu { cpu: -1 };
	let cpuid = get_cpuid_allow_env_override(cpu);

	if cpuid.is_null() {
		return f64::NAN;
	}

	ret = if strcmp_cpuid_str(test_id, cpuid) == 0 { 1.0 } else { 0.0 };

	free(cpuid as *mut c_void);
	ret
}
