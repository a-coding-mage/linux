/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies from the original C header:
 * <linux/list.h>, <linux/rbtree.h>, "map_symbol.h", "branch.h"
 */

use core::ffi::{c_char, c_int, c_void};

pub const HELP_PAD: &str = "\t\t\t\t";

pub const CALLCHAIN_HELP: &str = "setup and enables call-graph (stack chain/backtrace):\n\n";

pub const RECORD_MODE_HELP: &str =
    "\t\t\t\trecord_mode:\tcall graph recording mode (fp|dwarf|lbr)\n";

pub const RECORD_SIZE_HELP: &str =
    "\t\t\t\trecord_size:\tif record_mode is 'dwarf', max size of stack recording (<bytes>)\n\
\t\t\t\t\t\tdefault: 8192 (bytes)\n";

pub const CALLCHAIN_RECORD_HELP: &str =
    "setup and enables call-graph (stack chain/backtrace):\n\n\
\t\t\t\trecord_mode:\tcall graph recording mode (fp|dwarf|lbr)\n\
\t\t\t\trecord_size:\tif record_mode is 'dwarf', max size of stack recording (<bytes>)\n\
\t\t\t\t\t\tdefault: 8192 (bytes)\n";

pub const CALLCHAIN_REPORT_HELP: &str =
    "\t\t\t\tprint_type:\tcall graph printing style (graph|flat|fractal|folded|none)\n\
\t\t\t\tthreshold:\tminimum call graph inclusion threshold (<percent>)\n\
\t\t\t\tprint_limit:\tmaximum number of call graph entry (<number>)\n\
\t\t\t\torder:\t\tcall graph order (caller|callee)\n\
\t\t\t\tsort_key:\tcall graph sort key (function|address)\n\
\t\t\t\tbranch:\t\tinclude last branch info to call graph (branch)\n\
\t\t\t\tvalue:\t\tcall graph value (percent|period|count)\n";

pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;
pub type s64 = i64;
pub type size_t = usize;
pub type uint16_t = u16;

#[repr(C)]
pub struct addr_location {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hist_entry {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hists {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ip_callchain {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_sample {
    _private: [u8; 0],
}

#[repr(C)]
pub struct record_opts {
    _private: [u8; 0],
}

#[repr(C)]
pub struct thread {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rb_node {
    pub __rb_parent_color: usize,
    pub rb_right: *mut rb_node,
    pub rb_left: *mut rb_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

pub const RB_ROOT: rb_root = rb_root {
    rb_node: core::ptr::null_mut(),
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_symbol {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct branch_flags {
    _private: [u8; 0],
}

#[repr(C)]
pub struct branch_type_stat {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum perf_call_graph_mode {
    CALLCHAIN_NONE,
    CALLCHAIN_FP,
    CALLCHAIN_DWARF,
    CALLCHAIN_LBR,
    CALLCHAIN_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum chain_mode {
    CHAIN_NONE,
    CHAIN_FLAT,
    CHAIN_GRAPH_ABS,
    CHAIN_GRAPH_REL,
    CHAIN_FOLDED,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum chain_order {
    ORDER_CALLER,
    ORDER_CALLEE,
}

#[repr(C)]
pub struct callchain_node {
    pub parent: *mut callchain_node,
    pub val: list_head,
    pub parent_val: list_head,
    pub rb_node_in: rb_node, /* to insert nodes in an rbtree */
    pub rb_node: rb_node,    /* to sort nodes in an output tree */
    pub rb_root_in: rb_root, /* input tree of children */
    pub rb_root: rb_root,    /* sorted output tree of children */
    pub val_nr: c_uint,
    pub count: c_uint,
    pub children_count: c_uint,
    pub hit: u64,
    pub children_hit: u64,
}

pub type c_uint = u32;

#[repr(C)]
pub struct callchain_root {
    pub max_depth: u64,
    pub node: callchain_node,
}

#[repr(C)]
pub struct callchain_param {
    pub enabled: bool,
    pub defer: bool,
    pub record_mode: perf_call_graph_mode,
    pub dump_size: u32,
    pub mode: chain_mode,
    pub max_stack: u16,
    pub print_limit: u32,
    pub min_percent: f64,
    pub sort: sort_chain_func_t,
    pub order: chain_order,
    pub order_set: bool,
    pub key: chain_key,
    pub branch_callstack: bool,
    pub value: chain_value,
}

pub type sort_chain_func_t =
    Option<unsafe extern "C" fn(*mut rb_root, *mut callchain_root, u64, *mut callchain_param)>;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum chain_key {
    CCKEY_FUNCTION,
    CCKEY_ADDRESS,
    CCKEY_SRCLINE,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum chain_value {
    CCVAL_PERCENT,
    CCVAL_PERIOD,
    CCVAL_COUNT,
}

unsafe extern "C" {
    pub static mut dwarf_callchain_users: bool;
    pub static mut callchain_param: callchain_param;
    pub static mut callchain_param_default: callchain_param;
}

#[repr(C)]
pub struct callchain_list_tui {
    pub unfolded: bool,
    pub has_children: bool,
}

#[repr(C)]
pub struct callchain_list {
    pub list: list_head,
    pub ip: u64,
    pub ms: map_symbol,
    pub srcline: *const c_char,
    pub branch_count: u64,
    pub from_count: u64,
    pub cycles_count: u64,
    pub iter_count: u64,
    pub iter_cycles: u64,
    pub brtype_stat: *mut branch_type_stat,
    pub predicted_count: u64,
    pub abort_count: u64,
    /* for TUI */
    pub tui: callchain_list_tui,
}

/*
 * A callchain cursor is a single linked list that
 * let one feed a callchain progressively.
 * It keeps persistent allocated entries to minimize
 * allocations.
 */
#[repr(C)]
pub struct callchain_cursor_node {
    pub ip: u64,
    pub ms: map_symbol,
    pub srcline: *const c_char,
    /* Indicate valid cursor node for LBR stitch */
    pub valid: bool,

    pub branch: bool,
    pub branch_flags: branch_flags,
    pub branch_from: u64,
    pub nr_loop_iter: c_int,
    pub iter_cycles: u64,
    pub next: *mut callchain_cursor_node,
}

#[repr(C)]
pub struct stitch_list {
    pub node: list_head,
    pub cursor: callchain_cursor_node,
}

#[repr(C)]
pub struct callchain_cursor {
    pub nr: u64,
    pub first: *mut callchain_cursor_node,
    pub last: *mut *mut callchain_cursor_node,
    pub pos: u64,
    pub curr: *mut callchain_cursor_node,
}

#[inline]
pub unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

#[inline]
pub unsafe fn callchain_init(root: *mut callchain_root) {
    unsafe {
        INIT_LIST_HEAD(core::ptr::addr_of_mut!((*root).node.val));
        INIT_LIST_HEAD(core::ptr::addr_of_mut!((*root).node.parent_val));

        (*root).node.parent = core::ptr::null_mut();
        (*root).node.hit = 0;
        (*root).node.children_hit = 0;
        (*root).node.rb_root_in = RB_ROOT;
        (*root).max_depth = 0;
    }
}

#[inline]
pub unsafe fn callchain_cumul_hits(node: *mut callchain_node) -> u64 {
    unsafe { (*node).hit + (*node).children_hit }
}

#[inline]
pub unsafe fn callchain_cumul_counts(node: *mut callchain_node) -> c_uint {
    unsafe { (*node).count + (*node).children_count }
}

unsafe extern "C" {
    pub fn callchain_register_param(param: *mut callchain_param) -> c_int;
    pub fn callchain_append(
        root: *mut callchain_root,
        cursor: *mut callchain_cursor,
        period: u64,
    ) -> c_int;

    pub fn callchain_merge(
        cursor: *mut callchain_cursor,
        dst: *mut callchain_root,
        src: *mut callchain_root,
    ) -> c_int;

    pub fn callchain_cursor_reset(cursor: *mut callchain_cursor);

    pub fn callchain_cursor_append(
        cursor: *mut callchain_cursor,
        ip: u64,
        ms: *mut map_symbol,
        branch: bool,
        flags: *mut branch_flags,
        nr_loop_iter: c_int,
        iter_cycles: u64,
        branch_from: u64,
        srcline: *const c_char,
    ) -> c_int;
}

/* Close a cursor writing session. Initialize for the reader */
#[inline]
pub unsafe fn callchain_cursor_commit(cursor: *mut callchain_cursor) {
    unsafe {
        if cursor.is_null() {
            return;
        }
        (*cursor).curr = (*cursor).first;
        (*cursor).pos = 0;
    }
}

/* Cursor reading iteration helpers */
#[inline]
pub unsafe fn callchain_cursor_current(
    cursor: *mut callchain_cursor,
) -> *mut callchain_cursor_node {
    unsafe {
        if cursor.is_null() || (*cursor).pos == (*cursor).nr {
            return core::ptr::null_mut();
        }

        (*cursor).curr
    }
}

#[inline]
pub unsafe fn callchain_cursor_advance(cursor: *mut callchain_cursor) {
    unsafe {
        (*cursor).curr = (*(*cursor).curr).next;
        (*cursor).pos += 1;
    }
}

unsafe extern "C" {
    pub fn get_tls_callchain_cursor() -> *mut callchain_cursor;

    pub fn callchain_cursor__copy(
        dst: *mut callchain_cursor,
        src: *mut callchain_cursor,
    ) -> c_int;

    pub fn record_opts__parse_callchain(
        record: *mut record_opts,
        callchain: *mut callchain_param,
        arg: *const c_char,
        unset: bool,
    ) -> c_int;

    pub fn sample__resolve_callchain(
        sample: *mut perf_sample,
        cursor: *mut callchain_cursor,
        parent: *mut *mut symbol,
        al: *mut addr_location,
        max_stack: c_int,
    ) -> c_int;
    pub fn hist_entry__append_callchain(
        he: *mut hist_entry,
        sample: *mut perf_sample,
    ) -> c_int;
    pub fn fill_callchain_info(
        al: *mut addr_location,
        node: *mut callchain_cursor_node,
        hide_unresolved: bool,
    ) -> c_int;

    pub static record_callchain_help: [c_char; 0];
    pub fn parse_callchain_record(arg: *const c_char, param: *mut callchain_param) -> c_int;
    pub fn parse_callchain_record_opt(arg: *const c_char, param: *mut callchain_param) -> c_int;
    pub fn parse_callchain_report_opt(arg: *const c_char) -> c_int;
    pub fn parse_callchain_top_opt(arg: *const c_char) -> c_int;
    pub fn perf_callchain_config(var: *const c_char, value: *const c_char) -> c_int;
}

#[inline]
pub unsafe fn callchain_cursor_snapshot(
    dest: *mut callchain_cursor,
    src: *mut callchain_cursor,
) {
    unsafe {
        core::ptr::copy_nonoverlapping(src, dest, 1);

        (*dest).first = (*src).curr;
        (*dest).nr -= (*src).pos;
    }
}

/* Original C conditional:
 * #ifdef HAVE_SKIP_CALLCHAIN_IDX declares arch_skip_callchain_idx externally.
 * #else provides this inline fallback returning -1.
 */
#[cfg(HAVE_SKIP_CALLCHAIN_IDX)]
unsafe extern "C" {
    pub fn arch_skip_callchain_idx(thread: *mut thread, chain: *mut ip_callchain) -> c_int;
}

#[cfg(not(HAVE_SKIP_CALLCHAIN_IDX))]
#[inline]
pub unsafe fn arch_skip_callchain_idx(
    _thread: *mut thread,
    _chain: *mut ip_callchain,
) -> c_int {
    -1
}

unsafe extern "C" {
    pub fn callchain_list__sym_name(
        cl: *mut callchain_list,
        bf: *mut c_char,
        bfsize: size_t,
        show_dso: bool,
    ) -> *mut c_char;
    pub fn callchain_node__scnprintf_value(
        node: *mut callchain_node,
        bf: *mut c_char,
        bfsize: size_t,
        total: u64,
    ) -> *mut c_char;
    pub fn callchain_node__fprintf_value(
        node: *mut callchain_node,
        fp: *mut FILE,
        total: u64,
    ) -> c_int;

    pub fn callchain_list_counts__printf_value(
        clist: *mut callchain_list,
        fp: *mut FILE,
        bf: *mut c_char,
        bfsize: c_int,
    ) -> c_int;

    pub fn free_callchain(root: *mut callchain_root);
    pub fn callchain_cursor_cleanup(cursor: *mut callchain_cursor);
    pub fn decay_callchain(root: *mut callchain_root);
    pub fn callchain_node__make_parent_list(node: *mut callchain_node) -> c_int;

    pub fn callchain_branch_counts(
        root: *mut callchain_root,
        branch_count: *mut u64,
        predicted_count: *mut u64,
        abort_count: *mut u64,
        cycles_count: *mut u64,
    ) -> c_int;

    pub fn callchain_param_setup(sample_type: u64, e_machine: uint16_t);

    pub fn callchain_cnode_matched(
        base_cnode: *mut callchain_node,
        pair_cnode: *mut callchain_node,
    ) -> bool;

    pub fn callchain_total_hits(hists: *mut hists) -> u64;

    pub fn callchain_avg_cycles(cnode: *mut callchain_node) -> s64;
}

pub type callchain_iter_fn =
    Option<unsafe extern "C" fn(node: *mut callchain_cursor_node, data: *mut c_void) -> c_int>;

unsafe extern "C" {
    pub fn sample__for_each_callchain_node(
        thread: *mut thread,
        sample: *mut perf_sample,
        max_stack: c_int,
        symbols: bool,
        cb: callchain_iter_fn,
        data: *mut c_void,
    ) -> c_int;

    pub fn sample__merge_deferred_callchain(
        sample_orig: *mut perf_sample,
        sample_callchain: *mut perf_sample,
    ) -> c_int;
}
