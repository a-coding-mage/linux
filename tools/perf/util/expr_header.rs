/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/util/expr.h.
// C header guards and include directives are intentionally omitted.

use core::ffi::{c_char, c_double, c_int};

#[repr(C)]
pub struct hashmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct metric_ref {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct expr_scanner_ctx {
    pub user_requested_cpu_list: *mut c_char,
    pub runtime: c_int,
    pub system_wide: bool,
    pub is_test: bool,
}

#[repr(C)]
pub struct expr_parse_ctx {
    pub ids: *mut hashmap,
    pub sctx: expr_scanner_ctx,
}

#[repr(C)]
pub struct expr_id_data {
    _unused: [u8; 0],
}

unsafe extern "C" {
    pub fn ids__new() -> *mut hashmap;
    pub fn ids__free(ids: *mut hashmap);
    pub fn ids__insert(ids: *mut hashmap, id: *const c_char) -> c_int;

    /*
     * Union two sets of ids (hashmaps) and construct a third, freeing ids1 and
     * ids2.
     */
    pub fn ids__union(ids1: *mut hashmap, ids2: *mut hashmap) -> *mut hashmap;

    pub fn expr__ctx_new() -> *mut expr_parse_ctx;
    pub fn expr__ctx_clear(ctx: *mut expr_parse_ctx);
    pub fn expr__ctx_free(ctx: *mut expr_parse_ctx);

    pub fn expr__del_id(ctx: *mut expr_parse_ctx, id: *const c_char);
    pub fn expr__add_id(ctx: *mut expr_parse_ctx, id: *const c_char) -> c_int;
    pub fn expr__add_id_val(
        ctx: *mut expr_parse_ctx,
        id: *const c_char,
        val: c_double,
    ) -> c_int;
    pub fn expr__add_id_val_source_count(
        ctx: *mut expr_parse_ctx,
        id: *const c_char,
        val: c_double,
        source_count: c_int,
    ) -> c_int;
    pub fn expr__add_id_val_source_count_aggr_nr(
        ctx: *mut expr_parse_ctx,
        id: *const c_char,
        val: c_double,
        source_count: c_int,
        aggr_nr: c_int,
    ) -> c_int;
    pub fn expr__add_ref(ctx: *mut expr_parse_ctx, ref_: *mut metric_ref) -> c_int;
    pub fn expr__get_id(
        ctx: *mut expr_parse_ctx,
        id: *const c_char,
        data: *mut *mut expr_id_data,
    ) -> c_int;
    pub fn expr__subset_of_ids(
        haystack: *mut expr_parse_ctx,
        needles: *mut expr_parse_ctx,
    ) -> bool;
    pub fn expr__resolve_id(
        ctx: *mut expr_parse_ctx,
        id: *const c_char,
        datap: *mut *mut expr_id_data,
    ) -> c_int;

    pub fn expr__parse(
        final_val: *mut c_double,
        ctx: *mut expr_parse_ctx,
        expr: *const c_char,
    ) -> c_int;

    pub fn expr__find_ids(
        expr: *const c_char,
        one: *const c_char,
        ids: *mut expr_parse_ctx,
    ) -> c_int;

    pub fn expr_id_data__value(data: *const expr_id_data) -> c_double;
    pub fn expr_id_data__source_count(data: *const expr_id_data) -> c_double;
    pub fn expr_id_data__aggr_nr(data: *const expr_id_data) -> c_double;

    pub fn expr__get_literal(literal: *const c_char, ctx: *const expr_scanner_ctx) -> c_double;
    pub fn expr__has_event(
        ctx: *const expr_parse_ctx,
        compute_ids: bool,
        id: *const c_char,
    ) -> c_double;
    pub fn expr__strcmp_cpuid_str(
        ctx: *const expr_parse_ctx,
        compute_ids: bool,
        id: *const c_char,
    ) -> c_double;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
