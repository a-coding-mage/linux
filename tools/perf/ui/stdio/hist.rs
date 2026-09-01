// SPDX-License-Identifier: GPL-2.0
// Rust translation of perf/ui/stdio/hist.c.
// C include dependencies are represented by the extern declarations and
// opaque C-compatible types below.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_float, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type u32 = u32;
type u64 = u64;

const USHRT_MAX: c_int = u16::MAX as c_int;
const PERF_CONTEXT_MAX: u64 = 0xffff_ffff_ffff_ffff;
const PERF_RECORD_HEADER_MAX: c_int = 256;
const HIERARCHY_INDENT: c_int = 2;
const HMD_FORCE_CHILD: c_int = 1;
const CCVAL_COUNT: c_int = 1;
const CHAIN_NONE: c_int = 0;
const CHAIN_GRAPH_ABS: c_int = 1;
const CHAIN_GRAPH_REL: c_int = 2;
const CHAIN_FLAT: c_int = 3;
const CHAIN_FOLDED: c_int = 4;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct symbol {
    pub name: [c_char; 0],
}

#[repr(C)]
pub struct map_symbol {
    pub map: *mut c_void,
    pub sym: *mut symbol,
}

#[repr(C)]
pub struct callchain_list {
    pub list: list_head,
    pub ip: u64,
    pub ms: map_symbol,
}

#[repr(C)]
pub struct callchain_node {
    pub rb_node: rb_node,
    pub rb_root: rb_root,
    pub parent: *mut callchain_node,
    pub val: list_head,
    pub children_hit: u64,
    pub children_count: c_int,
    pub hit: u64,
    pub count: c_int,
}

#[repr(C)]
pub struct callchain_param_t {
    pub mode: c_int,
    pub print_limit: u32,
    pub value: c_int,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub show_branchflag_count: bool,
    pub field_sep: *const c_char,
    pub cumulate_callchain: bool,
    pub exclude_other: bool,
    pub use_callchain: bool,
    pub report_hierarchy: bool,
    pub report_block: bool,
    pub report_individual_block: bool,
    pub col_width_list_str: *const c_char,
    pub skip_empty: bool,
}

#[repr(C)]
pub struct hist_stat {
    pub period: u64,
}

#[repr(C)]
pub struct perf_hpp {
    pub buf: *mut c_char,
    pub size: size_t,
    pub skip: bool,
}

#[repr(C)]
pub struct perf_hpp_fmt {
    pub color: Option<unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int>,
    pub entry: unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hist_entry) -> c_int,
    pub header: unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hists, c_int, *mut c_int),
    pub width: unsafe extern "C" fn(*mut perf_hpp_fmt, *mut perf_hpp, *mut hists) -> c_uint,
}

#[repr(C)]
pub struct perf_hpp_list {
    pub nr_header_lines: c_int,
}

#[repr(C)]
pub struct perf_hpp_list_node {
    pub list: list_head,
    pub hpp: perf_hpp_list,
}

#[repr(C)]
pub struct hists_stats {
    pub total_period: u64,
}

#[repr(C)]
pub struct hists {
    pub hpp_list: *mut perf_hpp_list,
    pub hpp_formats: list_head,
    pub nr_hpp_node: c_int,
    pub stats: hists_stats,
    pub entries: rb_root_cached,
}

#[repr(C)]
pub struct hist_entry {
    pub rb_node: rb_node,
    pub hists: *mut hists,
    pub sorted_chain: rb_root,
    pub stat: hist_stat,
    pub stat_acc: *mut hist_stat,
    pub parent: *mut hist_entry,
    pub hpp_list: *mut perf_hpp_list,
    pub leaf: bool,
    pub filtered: bool,
    pub depth: c_int,
    pub ms: map_symbol,
    pub thread: *mut c_void,
}

#[repr(C)]
pub struct block_hists {
    pub nr_entries: c_uint,
}

#[repr(C)]
pub struct block_hist {
    pub he: hist_entry,
    pub block_hists: block_hists,
    pub block_idx: c_uint,
}

#[repr(C)]
pub struct events_stats {
    pub nr_events: [u32; PERF_RECORD_HEADER_MAX as usize],
}

unsafe extern "C" {
    static mut stderr: *mut FILE;
    static mut symbol_conf: symbol_conf_t;
    static mut callchain_param: callchain_param_t;
    static mut field_order: *const c_char;
    static mut sort_order: *const c_char;
    static mut verbose: c_int;
    static dots: *const c_char;
    static graph_dotted_line: *const c_char;

    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn fputs(s: *const c_char, fp: *mut FILE) -> c_int;
    fn fputc(c: c_int, fp: *mut FILE) -> c_int;
    fn putc(c: c_int, fp: *mut FILE) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn skip_spaces(str_: *const c_char) -> *const c_char;
    fn strim(str_: *mut c_char) -> *mut c_char;
    fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    fn zfree(ptr: *mut *mut symbol);

    fn rb_first(root: *mut rb_root) -> *mut rb_node;
    fn rb_next(node: *mut rb_node) -> *mut rb_node;
    fn rb_first_cached(root: *mut rb_root_cached) -> *mut rb_node;
    fn __rb_hierarchy_next(node: *mut rb_node, mode: c_int) -> *mut rb_node;

    fn callchain_cumul_hits(node: *mut callchain_node) -> u64;
    fn callchain_cumul_counts(node: *mut callchain_node) -> c_int;
    fn callchain_node__fprintf_value(node: *mut callchain_node, fp: *mut FILE, total_samples: u64) -> size_t;
    fn callchain_list__sym_name(chain: *mut callchain_list, bf: *mut c_char, size: size_t, show_dso: bool) -> *const c_char;
    fn callchain_list_counts__printf_value(chain: *mut callchain_list, fp: *mut FILE, buf: *mut c_char, size: size_t) -> size_t;
    fn pr_err(fmt: *const c_char, ...) -> c_int;

    fn perf_hpp__should_skip(fmt: *mut perf_hpp_fmt, hists: *mut hists) -> bool;
    fn perf_hpp__use_color() -> bool;
    fn perf_hpp__set_user_width(str_: *const c_char);
    fn perf_hpp__color_overhead() -> size_t;
    fn hist_entry__snprintf_alignment(he: *mut hist_entry, hpp: *mut perf_hpp, fmt: *mut perf_hpp_fmt, ret: c_int) -> c_int;
    fn advance_hpp(hpp: *mut perf_hpp, ret: c_int);
    fn hist_entry__has_callchains(he: *mut hist_entry) -> bool;
    fn hists__total_period(hists: *mut hists) -> u64;
    fn hists__reset_column_width(hists: *mut hists);
    fn hists__sort_list_width(hists: *mut hists) -> size_t;
    fn hists__overhead_width(hists: *mut hists) -> c_uint;
    fn hist_entry__get_percent_limit(he: *mut hist_entry) -> c_float;
    fn hist_entry__has_hierarchy_children(he: *mut hist_entry, min_pcnt: c_float) -> bool;
    fn block_info__total_cycles_percent(he: *mut hist_entry) -> c_float;
    fn maps__fprintf(maps: *mut c_void, fp: *mut FILE) -> size_t;
    fn thread__maps(thread: *mut c_void) -> *mut c_void;
    fn perf_event__name(id: c_int) -> *const c_char;
}

const fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn list_entry<T>(ptr: *mut list_head, offset: usize) -> *mut T {
    (ptr as *mut u8).sub(offset) as *mut T
}

unsafe fn list_first_entry_callchain(head: *mut list_head) -> *mut callchain_list {
    list_entry((*head).next, core::mem::offset_of!(callchain_list, list))
}

unsafe fn list_next_entry_callchain(pos: *mut callchain_list) -> *mut callchain_list {
    list_entry((*pos).list.next, core::mem::offset_of!(callchain_list, list))
}

unsafe fn list_first_entry_hpp_node(head: *mut list_head) -> *mut perf_hpp_list_node {
    list_entry((*head).next, core::mem::offset_of!(perf_hpp_list_node, list))
}

unsafe fn list_next_entry_hpp_node(pos: *mut perf_hpp_list_node) -> *mut perf_hpp_list_node {
    list_entry((*pos).list.next, core::mem::offset_of!(perf_hpp_list_node, list))
}

unsafe fn rb_entry_callchain(node: *mut rb_node) -> *mut callchain_node {
    (node as *mut u8).sub(core::mem::offset_of!(callchain_node, rb_node)) as *mut callchain_node
}

unsafe fn rb_entry_hist_entry(node: *mut rb_node) -> *mut hist_entry {
    (node as *mut u8).sub(core::mem::offset_of!(hist_entry, rb_node)) as *mut hist_entry
}

unsafe fn container_of_block_hist(he: *mut hist_entry) -> *mut block_hist {
    (he as *mut u8).sub(core::mem::offset_of!(block_hist, he)) as *mut block_hist
}

// External iterator macros such as perf_hpp_list__for_each_format() and
// hists__for_each_format() require repository list definitions not present in
// this isolated file. These helpers preserve the source-level loop sites.
unsafe fn perf_hpp_list_for_each_format(
    _hpp_list: *mut perf_hpp_list,
    _body: &mut dyn FnMut(*mut perf_hpp_fmt),
) {
    todo!("perf_hpp_list__for_each_format macro expansion is supplied by perf headers");
}

unsafe fn hists_for_each_format(
    _hists: *mut hists,
    _body: &mut dyn FnMut(*mut perf_hpp_fmt),
) {
    todo!("hists__for_each_format macro expansion is supplied by perf headers");
}

static mut rem_sq_bracket: *mut symbol = ptr::null_mut();
static mut rem_hits: callchain_list = callchain_list {
    list: list_head { next: ptr::null_mut(), prev: ptr::null_mut() },
    ip: 0,
    ms: map_symbol { map: ptr::null_mut(), sym: ptr::null_mut() },
};

unsafe fn callchain__fprintf_left_margin(fp: *mut FILE, mut left_margin: c_int) -> size_t {
    let mut i: c_int;
    let mut ret = fprintf(fp, cstr(b"            \0"));

    if left_margin > USHRT_MAX {
        left_margin = USHRT_MAX;
    }

    i = 0;
    while i < left_margin {
        ret += fprintf(fp, cstr(b" \0"));
        i += 1;
    }

    ret as size_t
}

unsafe fn ipchain__fprintf_graph_line(
    fp: *mut FILE,
    depth: c_int,
    depth_mask: c_int,
    left_margin: c_int,
) -> size_t {
    let mut i: c_int = 0;
    let mut ret = callchain__fprintf_left_margin(fp, left_margin);

    while i < depth {
        if depth_mask & (1 << i) != 0 {
            ret += fprintf(fp, cstr(b"|          \0")) as size_t;
        } else {
            ret += fprintf(fp, cstr(b"           \0")) as size_t;
        }
        i += 1;
    }

    ret += fprintf(fp, cstr(b"\n\0")) as size_t;
    ret
}

unsafe fn ipchain__fprintf_graph(
    fp: *mut FILE,
    node: *mut callchain_node,
    chain: *mut callchain_list,
    depth: c_int,
    depth_mask: c_int,
    period: c_int,
    total_samples: u64,
    left_margin: c_int,
) -> size_t {
    let mut i: c_int;
    let mut ret: size_t = 0;
    let mut bf = [0 as c_char; 1024];
    let mut alloc_str: *mut c_char = ptr::null_mut();
    let mut buf = [0 as c_char; 64];
    let mut str_: *const c_char;

    ret += callchain__fprintf_left_margin(fp, left_margin);
    i = 0;
    while i < depth {
        if depth_mask & (1 << i) != 0 {
            ret += fprintf(fp, cstr(b"|\0")) as size_t;
        } else {
            ret += fprintf(fp, cstr(b" \0")) as size_t;
        }
        if period == 0 && i == depth - 1 {
            ret += fprintf(fp, cstr(b"--\0")) as size_t;
            ret += callchain_node__fprintf_value(node, fp, total_samples);
            ret += fprintf(fp, cstr(b"--\0")) as size_t;
        } else {
            ret += fprintf(fp, cstr(b"%s\0"), cstr(b"          \0")) as size_t;
        }
        i += 1;
    }

    str_ = callchain_list__sym_name(chain, bf.as_mut_ptr(), bf.len(), false);

    if symbol_conf.show_branchflag_count {
        callchain_list_counts__printf_value(chain, ptr::null_mut(), buf.as_mut_ptr(), buf.len());

        if asprintf(&mut alloc_str, cstr(b"%s%s\0"), str_, buf.as_mut_ptr()) < 0 {
            str_ = cstr(b"Not enough memory!\0");
        } else {
            str_ = alloc_str;
        }
    }

    fputs(str_, fp);
    fputc('\n' as c_int, fp);
    free(alloc_str as *mut c_void);
    ret
}

unsafe fn init_rem_hits() {
    rem_sq_bracket = malloc(size_of::<symbol>() + 6) as *mut symbol;
    if rem_sq_bracket.is_null() {
        fprintf(stderr, cstr(b"Not enough memory to display remaining hits\n\0"));
        return;
    }

    strcpy((*rem_sq_bracket).name.as_mut_ptr(), cstr(b"[...]\0"));
    rem_hits.ms.sym = rem_sq_bracket;
}

unsafe fn __callchain__fprintf_graph(
    fp: *mut FILE,
    root: *mut rb_root,
    total_samples: u64,
    depth: c_int,
    depth_mask: c_int,
    left_margin: c_int,
) -> size_t {
    let mut node: *mut rb_node;
    let mut next: *mut rb_node;
    let mut child: *mut callchain_node = ptr::null_mut();
    let mut chain: *mut callchain_list;
    let mut new_depth_mask = depth_mask;
    let mut remaining = total_samples;
    let mut ret: size_t = 0;
    let mut entries_printed: c_uint = 0;
    let mut cumul_count: c_int = 0;

    node = rb_first(root);
    while !node.is_null() {
        let new_total: u64;
        let cumul: u64;

        child = rb_entry_callchain(node);
        cumul = callchain_cumul_hits(child);
        remaining = remaining.wrapping_sub(cumul);
        cumul_count += callchain_cumul_counts(child);

        /*
         * The depth mask manages the output of pipes that show
         * the depth. We don't want to keep the pipes of the current
         * level for the last child of this depth.
         * Except if we have remaining filtered hits. They will
         * supersede the last child
         */
        next = rb_next(node);
        if next.is_null() && (callchain_param.mode != CHAIN_GRAPH_REL || remaining == 0) {
            new_depth_mask &= !(1 << (depth - 1));
        }

        /*
         * But we keep the older depth mask for the line separator
         * to keep the level link until we reach the last child
         */
        ret += ipchain__fprintf_graph_line(fp, depth, depth_mask, left_margin);
        let mut i = 0;
        chain = list_first_entry_callchain(&mut (*child).val);
        while &mut (*chain).list as *mut list_head != &mut (*child).val as *mut list_head {
            ret += ipchain__fprintf_graph(
                fp,
                child,
                chain,
                depth,
                new_depth_mask,
                i,
                total_samples,
                left_margin,
            );
            i += 1;
            chain = list_next_entry_callchain(chain);
        }

        if callchain_param.mode == CHAIN_GRAPH_REL {
            new_total = (*child).children_hit;
        } else {
            new_total = total_samples;
        }

        ret += __callchain__fprintf_graph(
            fp,
            &mut (*child).rb_root,
            new_total,
            depth + 1,
            new_depth_mask | (1 << depth),
            left_margin,
        );
        node = next;
        entries_printed += 1;
        if entries_printed == callchain_param.print_limit {
            break;
        }
    }

    if callchain_param.mode == CHAIN_GRAPH_REL && remaining != 0 && remaining != total_samples {
        let mut rem_node = callchain_node {
            rb_node: rb_node { _private: [] },
            rb_root: rb_root { _private: [] },
            parent: ptr::null_mut(),
            val: list_head { next: ptr::null_mut(), prev: ptr::null_mut() },
            children_hit: 0,
            children_count: 0,
            hit: remaining,
            count: 0,
        };

        if rem_sq_bracket.is_null() {
            return ret;
        }

        if callchain_param.value == CCVAL_COUNT && !child.is_null() && !(*child).parent.is_null() {
            rem_node.count = (*(*child).parent).children_count - cumul_count;
            if rem_node.count <= 0 {
                return ret;
            }
        }

        new_depth_mask &= !(1 << (depth - 1));
        ret += ipchain__fprintf_graph(
            fp,
            &mut rem_node,
            &mut rem_hits,
            depth,
            new_depth_mask,
            0,
            total_samples,
            left_margin,
        );
    }

    ret
}

/*
 * If have one single callchain root, don't bother printing
 * its percentage (100 % in fractal mode and the same percentage
 * than the hist in graph mode). This also avoid one level of column.
 *
 * However when percent-limit applied, it's possible that single callchain
 * node have different (non-100% in fractal mode) percentage.
 */
unsafe fn need_percent_display(node: *mut rb_node, parent_samples: u64) -> bool {
    let cnode: *mut callchain_node;

    if !rb_next(node).is_null() {
        return true;
    }

    cnode = rb_entry_callchain(node);
    callchain_cumul_hits(cnode) != parent_samples
}

unsafe fn callchain__fprintf_graph(
    fp: *mut FILE,
    mut root: *mut rb_root,
    mut total_samples: u64,
    parent_samples: u64,
    mut left_margin: c_int,
) -> size_t {
    let mut cnode: *mut callchain_node;
    let mut chain: *mut callchain_list;
    let mut entries_printed: u32 = 0;
    let mut printed = false;
    let mut node: *mut rb_node;
    let mut i: c_int = 0;
    let mut ret: c_int = 0;
    let mut bf = [0 as c_char; 1024];

    node = rb_first(root);
    if !node.is_null() && !need_percent_display(node, parent_samples) {
        cnode = rb_entry_callchain(node);
        chain = list_first_entry_callchain(&mut (*cnode).val);
        while &mut (*chain).list as *mut list_head != &mut (*cnode).val as *mut list_head {
            /*
             * If we sort by symbol, the first entry is the same than
             * the symbol. No need to print it otherwise it appears as
             * displayed twice.
             */
            if i == 0
                && field_order.is_null()
                && !sort_order.is_null()
                && strstarts(sort_order, cstr(b"sym\0"))
            {
                i += 1;
                chain = list_next_entry_callchain(chain);
                continue;
            }
            i += 1;

            if !printed {
                ret += callchain__fprintf_left_margin(fp, left_margin) as c_int;
                ret += fprintf(fp, cstr(b"|\n\0"));
                ret += callchain__fprintf_left_margin(fp, left_margin) as c_int;
                ret += fprintf(fp, cstr(b"---\0"));
                left_margin += 3;
                printed = true;
            } else {
                ret += callchain__fprintf_left_margin(fp, left_margin) as c_int;
            }

            ret += fprintf(
                fp,
                cstr(b"%s\0"),
                callchain_list__sym_name(chain, bf.as_mut_ptr(), bf.len(), false),
            );

            if symbol_conf.show_branchflag_count {
                ret += callchain_list_counts__printf_value(chain, fp, ptr::null_mut(), 0) as c_int;
            }
            ret += fprintf(fp, cstr(b"\n\0"));

            entries_printed += 1;
            if entries_printed == callchain_param.print_limit {
                break;
            }
            chain = list_next_entry_callchain(chain);
        }
        root = &mut (*cnode).rb_root;
    }

    if callchain_param.mode == CHAIN_GRAPH_REL {
        total_samples = parent_samples;
    }

    ret += __callchain__fprintf_graph(fp, root, total_samples, 1, 1, left_margin) as c_int;
    if ret != 0 {
        /* do not add a blank line if it printed nothing */
        ret += fprintf(fp, cstr(b"\n\0"));
    }

    ret as size_t
}

unsafe fn __callchain__fprintf_flat(
    fp: *mut FILE,
    node: *mut callchain_node,
    total_samples: u64,
) -> size_t {
    let mut chain: *mut callchain_list;
    let mut ret: size_t = 0;
    let mut bf = [0 as c_char; 1024];

    if node.is_null() {
        return 0;
    }

    ret += __callchain__fprintf_flat(fp, (*node).parent, total_samples);

    chain = list_first_entry_callchain(&mut (*node).val);
    while &mut (*chain).list as *mut list_head != &mut (*node).val as *mut list_head {
        if (*chain).ip >= PERF_CONTEXT_MAX {
            chain = list_next_entry_callchain(chain);
            continue;
        }
        ret += fprintf(
            fp,
            cstr(b"                %s\n\0"),
            callchain_list__sym_name(chain, bf.as_mut_ptr(), bf.len(), false),
        ) as size_t;
        chain = list_next_entry_callchain(chain);
    }

    ret
}

unsafe fn callchain__fprintf_flat(
    fp: *mut FILE,
    tree: *mut rb_root,
    total_samples: u64,
) -> size_t {
    let mut ret: size_t = 0;
    let mut entries_printed: u32 = 0;
    let mut chain: *mut callchain_node;
    let mut rb_node = rb_first(tree);

    while !rb_node.is_null() {
        chain = rb_entry_callchain(rb_node);

        ret += fprintf(fp, cstr(b"           \0")) as size_t;
        ret += callchain_node__fprintf_value(chain, fp, total_samples);
        ret += fprintf(fp, cstr(b"\n\0")) as size_t;
        ret += __callchain__fprintf_flat(fp, chain, total_samples);
        ret += fprintf(fp, cstr(b"\n\0")) as size_t;
        entries_printed += 1;
        if entries_printed == callchain_param.print_limit {
            break;
        }

        rb_node = rb_next(rb_node);
    }

    ret
}

unsafe fn __callchain__fprintf_folded(fp: *mut FILE, node: *mut callchain_node) -> size_t {
    let sep = if !symbol_conf.field_sep.is_null() { symbol_conf.field_sep } else { cstr(b";\0") };
    let mut chain: *mut callchain_list;
    let mut ret: size_t = 0;
    let mut bf = [0 as c_char; 1024];
    let mut first: bool;

    if node.is_null() {
        return 0;
    }

    ret += __callchain__fprintf_folded(fp, (*node).parent);

    first = ret == 0;
    chain = list_first_entry_callchain(&mut (*node).val);
    while &mut (*chain).list as *mut list_head != &mut (*node).val as *mut list_head {
        if (*chain).ip >= PERF_CONTEXT_MAX {
            chain = list_next_entry_callchain(chain);
            continue;
        }
        ret += fprintf(
            fp,
            cstr(b"%s%s\0"),
            if first { cstr(b"\0") } else { sep },
            callchain_list__sym_name(chain, bf.as_mut_ptr(), bf.len(), false),
        ) as size_t;
        first = false;
        chain = list_next_entry_callchain(chain);
    }

    ret
}

unsafe fn callchain__fprintf_folded(
    fp: *mut FILE,
    tree: *mut rb_root,
    total_samples: u64,
) -> size_t {
    let mut ret: size_t = 0;
    let mut entries_printed: u32 = 0;
    let mut chain: *mut callchain_node;
    let mut rb_node = rb_first(tree);

    while !rb_node.is_null() {
        chain = rb_entry_callchain(rb_node);

        ret += callchain_node__fprintf_value(chain, fp, total_samples);
        ret += fprintf(fp, cstr(b" \0")) as size_t;
        ret += __callchain__fprintf_folded(fp, chain);
        ret += fprintf(fp, cstr(b"\n\0")) as size_t;
        entries_printed += 1;
        if entries_printed == callchain_param.print_limit {
            break;
        }

        rb_node = rb_next(rb_node);
    }

    ret
}

unsafe fn hist_entry_callchain__fprintf(
    he: *mut hist_entry,
    total_samples: u64,
    left_margin: c_int,
    fp: *mut FILE,
) -> size_t {
    let mut parent_samples = (*he).stat.period;

    if symbol_conf.cumulate_callchain {
        parent_samples = (*(*he).stat_acc).period;
    }

    match callchain_param.mode {
        CHAIN_GRAPH_REL | CHAIN_GRAPH_ABS => callchain__fprintf_graph(
            fp,
            &mut (*he).sorted_chain,
            total_samples,
            parent_samples,
            left_margin,
        ),
        CHAIN_FLAT => callchain__fprintf_flat(fp, &mut (*he).sorted_chain, total_samples),
        CHAIN_FOLDED => callchain__fprintf_folded(fp, &mut (*he).sorted_chain, total_samples),
        CHAIN_NONE => 0,
        _ => {
            pr_err(cstr(b"Bad callchain mode\n\0"));
            0
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __hist_entry__snprintf(
    he: *mut hist_entry,
    hpp: *mut perf_hpp,
    hpp_list: *mut perf_hpp_list,
) -> c_int {
    let sep = symbol_conf.field_sep;
    let start = (*hpp).buf;
    let mut ret: c_int;
    let mut first = true;

    if symbol_conf.exclude_other && (*he).parent.is_null() {
        return 0;
    }

    perf_hpp_list_for_each_format(hpp_list, &mut |fmt| {
        unsafe {
            if perf_hpp__should_skip(fmt, (*he).hists) {
                return;
            }

            /*
             * If there's no field_sep, we still need
             * to display initial '  '.
             */
            if sep.is_null() || !first {
                ret = scnprintf(
                    (*hpp).buf,
                    (*hpp).size,
                    cstr(b"%s\0"),
                    if !sep.is_null() { sep } else { cstr(b"  \0") },
                );
                advance_hpp(hpp, ret);
            } else {
                first = false;
            }

            if perf_hpp__use_color() && (*fmt).color.is_some() {
                ret = ((*fmt).color.unwrap())(fmt, hpp, he);
            } else {
                ret = ((*fmt).entry)(fmt, hpp, he);
            }

            ret = hist_entry__snprintf_alignment(he, hpp, fmt, ret);
            advance_hpp(hpp, ret);
        }
    });

    (*hpp).buf.offset_from(start) as c_int
}

unsafe fn hist_entry__snprintf(he: *mut hist_entry, hpp: *mut perf_hpp) -> c_int {
    __hist_entry__snprintf(he, hpp, (*(*he).hists).hpp_list)
}

unsafe fn hist_entry__hierarchy_fprintf(
    he: *mut hist_entry,
    hpp: *mut perf_hpp,
    hists: *mut hists,
    fp: *mut FILE,
) -> c_int {
    let sep = symbol_conf.field_sep;
    let fmt_node: *mut perf_hpp_list_node;
    let buf = (*hpp).buf;
    let size = (*hpp).size;
    let mut ret: c_int = 0;
    let mut printed: c_int = 0;
    let mut first = true;

    if symbol_conf.exclude_other && (*he).parent.is_null() {
        return 0;
    }

    ret = scnprintf(
        (*hpp).buf,
        (*hpp).size,
        cstr(b"%*s\0"),
        (*he).depth * HIERARCHY_INDENT,
        cstr(b"\0"),
    );
    advance_hpp(hpp, ret);

    /* the first hpp_list_node is for overhead columns */
    fmt_node = list_first_entry_hpp_node(&mut (*hists).hpp_formats);
    perf_hpp_list_for_each_format(&mut (*fmt_node).hpp, &mut |fmt| {
        unsafe {
            /*
             * If there's no field_sep, we still need
             * to display initial '  '.
             */
            if sep.is_null() || !first {
                ret = scnprintf(
                    (*hpp).buf,
                    (*hpp).size,
                    cstr(b"%s\0"),
                    if !sep.is_null() { sep } else { cstr(b"  \0") },
                );
                advance_hpp(hpp, ret);
            } else {
                first = false;
            }

            if perf_hpp__use_color() && (*fmt).color.is_some() {
                ret = ((*fmt).color.unwrap())(fmt, hpp, he);
            } else {
                ret = ((*fmt).entry)(fmt, hpp, he);
            }

            ret = hist_entry__snprintf_alignment(he, hpp, fmt, ret);
            advance_hpp(hpp, ret);
        }
    });

    if sep.is_null() {
        ret = scnprintf(
            (*hpp).buf,
            (*hpp).size,
            cstr(b"%*s\0"),
            ((*hists).nr_hpp_node - 2) * HIERARCHY_INDENT,
            cstr(b"\0"),
        );
    }
    advance_hpp(hpp, ret);

    printed += fprintf(fp, cstr(b"%s\0"), buf);

    perf_hpp_list_for_each_format((*he).hpp_list, &mut |fmt| {
        unsafe {
            (*hpp).buf = buf;
            (*hpp).size = size;

            /*
             * No need to call hist_entry__snprintf_alignment() since this
             * fmt is always the last column in the hierarchy mode.
             */
            if perf_hpp__use_color() && (*fmt).color.is_some() {
                ((*fmt).color.unwrap())(fmt, hpp, he);
            } else {
                ((*fmt).entry)(fmt, hpp, he);
            }

            /*
             * dynamic entries are right-aligned but we want left-aligned
             * in the hierarchy mode
             */
            printed += fprintf(
                fp,
                cstr(b"%s%s\0"),
                if !sep.is_null() { sep } else { cstr(b"  \0") },
                skip_spaces(buf),
            );
        }
    });
    printed += putc('\n' as c_int, fp);

    if (*he).leaf && hist_entry__has_callchains(he) && symbol_conf.use_callchain {
        let total = hists__total_period(hists);
        printed += hist_entry_callchain__fprintf(he, total, 0, fp) as c_int;
    }

    printed
}

unsafe fn hist_entry__block_fprintf(
    he: *mut hist_entry,
    bf: *mut c_char,
    size: size_t,
    fp: *mut FILE,
) -> c_int {
    let bh = container_of_block_hist(he);
    let mut ret: c_int = 0;
    let mut i: c_uint = 0;

    while i < (*bh).block_hists.nr_entries {
        let mut hpp = perf_hpp {
            buf: bf,
            size,
            skip: false,
        };

        (*bh).block_idx = i;
        hist_entry__snprintf(he, &mut hpp);

        if !hpp.skip {
            ret += fprintf(fp, cstr(b"%s\n\0"), bf);
        }
        i += 1;
    }

    ret
}

unsafe fn hist_entry__individual_block_fprintf(
    he: *mut hist_entry,
    bf: *mut c_char,
    size: size_t,
    fp: *mut FILE,
) -> c_int {
    let mut ret: c_int = 0;
    let mut hpp = perf_hpp {
        buf: bf,
        size,
        skip: false,
    };

    hist_entry__snprintf(he, &mut hpp);
    if !hpp.skip {
        ret += fprintf(fp, cstr(b"%s\n\0"), bf);
    }

    ret
}

unsafe fn hist_entry__fprintf(
    he: *mut hist_entry,
    mut size: size_t,
    bf: *mut c_char,
    bfsz: size_t,
    fp: *mut FILE,
    ignore_callchains: bool,
) -> c_int {
    let mut callchain_ret: c_int = 0;
    let mut hpp = perf_hpp {
        buf: bf,
        size,
        skip: false,
    };
    let hists = (*he).hists;
    let total_period = (*hists).stats.total_period;

    if size == 0 || size > bfsz {
        size = bfsz;
        hpp.size = bfsz;
    }

    if symbol_conf.report_hierarchy {
        return hist_entry__hierarchy_fprintf(he, &mut hpp, hists, fp);
    }

    if symbol_conf.report_block {
        return hist_entry__block_fprintf(he, bf, size, fp);
    }

    if symbol_conf.report_individual_block {
        return hist_entry__individual_block_fprintf(he, bf, size, fp);
    }

    hist_entry__snprintf(he, &mut hpp);

    let mut ret = fprintf(fp, cstr(b"%s\n\0"), bf);

    if hist_entry__has_callchains(he) && !ignore_callchains {
        callchain_ret = hist_entry_callchain__fprintf(he, total_period, 0, fp) as c_int;
    }

    ret += callchain_ret;
    ret
}

unsafe fn print_hierarchy_indent(
    sep: *const c_char,
    indent: c_int,
    line: *const c_char,
    fp: *mut FILE,
) -> c_int {
    let width: c_int;

    if !sep.is_null() || indent < 2 {
        return 0;
    }

    width = (indent - 2) * HIERARCHY_INDENT;
    fprintf(fp, cstr(b"%-*.*s\0"), width, width, line)
}

unsafe fn hists__fprintf_hierarchy_headers(
    hists: *mut hists,
    hpp: *mut perf_hpp,
    fp: *mut FILE,
) -> c_int {
    let mut first_node: bool;
    let mut first_col: bool;
    let indent: c_int;
    let mut depth: c_int;
    let mut width: c_uint = 0;
    let mut header_width: c_uint = 0;
    let mut fmt_node: *mut perf_hpp_list_node;
    let hpp_list = (*hists).hpp_list;
    let sep = symbol_conf.field_sep;

    indent = (*hists).nr_hpp_node;

    /* the first hpp_list_node is for overhead columns */
    fmt_node = list_first_entry_hpp_node(&mut (*hists).hpp_formats);

    let mut line = 0;
    while line < (*hpp_list).nr_header_lines {
        /* first # is displayed one level up */
        if line != 0 {
            fprintf(fp, cstr(b"# \0"));
        }

        /* preserve max indent depth for column headers */
        print_hierarchy_indent(sep, indent, cstr(b" \0"), fp);

        perf_hpp_list_for_each_format(&mut (*fmt_node).hpp, &mut |fmt| unsafe {
            ((*fmt).header)(fmt, hpp, hists, line, ptr::null_mut());
            fprintf(
                fp,
                cstr(b"%s%s\0"),
                (*hpp).buf,
                if !sep.is_null() { sep } else { cstr(b"  \0") },
            );
        });

        if line < (*hpp_list).nr_header_lines - 1 {
            fprintf(fp, cstr(b"\n\0"));
            line += 1;
            continue;
        }

        /* combine sort headers with ' / ' */
        first_node = true;
        fmt_node = list_next_entry_hpp_node(fmt_node);
        while &mut (*fmt_node).list as *mut list_head != &mut (*hists).hpp_formats as *mut list_head {
            if !first_node {
                header_width += fprintf(fp, cstr(b" / \0")) as c_uint;
            }
            first_node = false;

            first_col = true;
            perf_hpp_list_for_each_format(&mut (*fmt_node).hpp, &mut |fmt| unsafe {
                if perf_hpp__should_skip(fmt, hists) {
                    return;
                }

                if !first_col {
                    header_width += fprintf(fp, cstr(b"+\0")) as c_uint;
                }
                first_col = false;

                ((*fmt).header)(fmt, hpp, hists, line, ptr::null_mut());
                header_width += fprintf(fp, cstr(b"%s\0"), strim((*hpp).buf)) as c_uint;
            });
            fmt_node = list_next_entry_hpp_node(fmt_node);
        }

        fprintf(fp, cstr(b"\n\0"));
        line += 1;
    }

    fprintf(fp, cstr(b"# \0"));

    /* preserve max indent depth for initial dots */
    print_hierarchy_indent(sep, indent, dots, fp);

    /* the first hpp_list_node is for overhead columns */
    fmt_node = list_first_entry_hpp_node(&mut (*hists).hpp_formats);

    first_col = true;
    perf_hpp_list_for_each_format(&mut (*fmt_node).hpp, &mut |fmt| unsafe {
        if !first_col {
            fprintf(fp, cstr(b"%s\0"), if !sep.is_null() { sep } else { cstr(b"..\0") });
        }
        first_col = false;

        width = ((*fmt).width)(fmt, hpp, hists);
        fprintf(fp, cstr(b"%.*s\0"), width, dots);
    });

    depth = 0;
    fmt_node = list_next_entry_hpp_node(fmt_node);
    while &mut (*fmt_node).list as *mut list_head != &mut (*hists).hpp_formats as *mut list_head {
        first_col = true;
        width = (depth * HIERARCHY_INDENT) as c_uint;

        perf_hpp_list_for_each_format(&mut (*fmt_node).hpp, &mut |fmt| unsafe {
            if perf_hpp__should_skip(fmt, hists) {
                return;
            }

            if !first_col {
                width += 1; /* for '+' sign between column header */
            }
            first_col = false;

            width += ((*fmt).width)(fmt, hpp, hists);
        });

        if width > header_width {
            header_width = width;
        }

        depth += 1;
        fmt_node = list_next_entry_hpp_node(fmt_node);
    }

    fprintf(
        fp,
        cstr(b"%s%-.*s\0"),
        if !sep.is_null() { sep } else { cstr(b"  \0") },
        header_width,
        dots,
    );

    fprintf(fp, cstr(b"\n#\n\0"));
    2
}

unsafe fn fprintf_line(hists: *mut hists, hpp: *mut perf_hpp, line: c_int, fp: *mut FILE) {
    let sep = symbol_conf.field_sep;
    let mut first = true;
    let mut span: c_int = 0;

    hists_for_each_format(hists, &mut |fmt| unsafe {
        if perf_hpp__should_skip(fmt, hists) {
            return;
        }

        if !first && span == 0 {
            fprintf(fp, cstr(b"%s\0"), if !sep.is_null() { sep } else { cstr(b"  \0") });
        } else {
            first = false;
        }

        ((*fmt).header)(fmt, hpp, hists, line, &mut span);

        if span == 0 {
            fprintf(fp, cstr(b"%s\0"), (*hpp).buf);
        }
    });
}

unsafe fn hists__fprintf_standard_headers(
    hists: *mut hists,
    hpp: *mut perf_hpp,
    fp: *mut FILE,
) -> c_int {
    let hpp_list = (*hists).hpp_list;
    let mut width: c_uint;
    let sep = symbol_conf.field_sep;
    let mut first = true;
    let mut line: c_int;

    line = 0;
    while line < (*hpp_list).nr_header_lines {
        /* first # is displayed one level up */
        if line != 0 {
            fprintf(fp, cstr(b"# \0"));
        }
        fprintf_line(hists, hpp, line, fp);
        fprintf(fp, cstr(b"\n\0"));
        line += 1;
    }

    if !sep.is_null() {
        return (*hpp_list).nr_header_lines;
    }

    first = true;
    fprintf(fp, cstr(b"# \0"));

    hists_for_each_format(hists, &mut |fmt| unsafe {
        if perf_hpp__should_skip(fmt, hists) {
            return;
        }

        if !first {
            fprintf(fp, cstr(b"%s\0"), if !sep.is_null() { sep } else { cstr(b"  \0") });
        } else {
            first = false;
        }

        width = ((*fmt).width)(fmt, hpp, hists);
        let mut i: c_uint = 0;
        while i < width {
            fprintf(fp, cstr(b".\0"));
            i += 1;
        }
    });

    fprintf(fp, cstr(b"\n\0"));
    fprintf(fp, cstr(b"#\n\0"));
    (*hpp_list).nr_header_lines + 2
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hists__fprintf_headers(hists: *mut hists, fp: *mut FILE) -> c_int {
    let mut bf = [0 as c_char; 1024];
    let mut dummy_hpp = perf_hpp {
        buf: bf.as_mut_ptr(),
        size: bf.len(),
        skip: false,
    };

    fprintf(fp, cstr(b"# \0"));

    if symbol_conf.report_hierarchy {
        hists__fprintf_hierarchy_headers(hists, &mut dummy_hpp, fp)
    } else {
        hists__fprintf_standard_headers(hists, &mut dummy_hpp, fp)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn hists__fprintf(
    hists: *mut hists,
    show_header: bool,
    max_rows: c_int,
    max_cols: c_int,
    min_pcnt: c_float,
    fp: *mut FILE,
    ignore_callchains: bool,
) -> size_t {
    let mut nd: *mut rb_node;
    let mut ret: size_t = 0;
    let sep = symbol_conf.field_sep;
    let mut nr_rows: c_int = 0;
    let mut linesz: size_t;
    let mut line: *mut c_char = ptr::null_mut();
    let indent: c_uint;

    init_rem_hits();
    hists__reset_column_width(hists);

    if !symbol_conf.col_width_list_str.is_null() {
        perf_hpp__set_user_width(symbol_conf.col_width_list_str);
    }

    if show_header {
        nr_rows += hists__fprintf_headers(hists, fp);
    }

    if max_rows != 0 && nr_rows >= max_rows {
        zfree(&mut rem_sq_bracket);
        return ret;
    }

    linesz = hists__sort_list_width(hists) + 3 + 1;
    linesz += perf_hpp__color_overhead();
    line = malloc(linesz) as *mut c_char;
    if line.is_null() {
        ret = usize::MAX;
        zfree(&mut rem_sq_bracket);
        return ret;
    }

    indent = hists__overhead_width(hists) + 4;

    nd = rb_first_cached(&mut (*hists).entries);
    while !nd.is_null() {
        let h = rb_entry_hist_entry(nd);
        let percent: c_float;

        if (*h).filtered {
            nd = __rb_hierarchy_next(nd, HMD_FORCE_CHILD);
            continue;
        }

        if symbol_conf.report_individual_block {
            percent = block_info__total_cycles_percent(h);
        } else {
            percent = hist_entry__get_percent_limit(h);
        }

        if percent < min_pcnt {
            nd = __rb_hierarchy_next(nd, HMD_FORCE_CHILD);
            continue;
        }

        ret += hist_entry__fprintf(h, max_cols as size_t, line, linesz, fp, ignore_callchains) as size_t;

        if max_rows != 0 {
            nr_rows += 1;
            if nr_rows >= max_rows {
                break;
            }
        }

        /*
         * If all children are filtered out or percent-limited,
         * display "no entry >= x.xx%" message.
         */
        if !(*h).leaf && !hist_entry__has_hierarchy_children(h, min_pcnt) {
            let depth = (*hists).nr_hpp_node + (*h).depth + 1;

            print_hierarchy_indent(sep, depth, cstr(b" \0"), fp);
            fprintf(fp, cstr(b"%*sno entry >= %.2f%%\n\0"), indent, cstr(b"\0"), min_pcnt as f64);

            if max_rows != 0 {
                nr_rows += 1;
                if nr_rows >= max_rows {
                    break;
                }
            }
        }

        if (*h).ms.map.is_null() && verbose > 1 {
            maps__fprintf(thread__maps((*h).thread), fp);
            fprintf(fp, cstr(b"%.10s end\n\0"), graph_dotted_line);
        }

        nd = __rb_hierarchy_next(nd, HMD_FORCE_CHILD);
    }

    free(line as *mut c_void);
    zfree(&mut rem_sq_bracket);
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn events_stats__fprintf(stats: *mut events_stats, fp: *mut FILE) -> size_t {
    let mut i: c_int;
    let mut ret: size_t = 0;
    let total: u32 = (*stats).nr_events[0];

    i = 0;
    while i < PERF_RECORD_HEADER_MAX {
        let name: *const c_char;

        name = perf_event__name(i);
        if strcmp(name, cstr(b"UNKNOWN\0")) == 0 {
            i += 1;
            continue;
        }
        if symbol_conf.skip_empty && (*stats).nr_events[i as usize] == 0 {
            i += 1;
            continue;
        }

        if i != 0 && total != 0 {
            ret += fprintf(
                fp,
                cstr(b"%20s events: %10d  (%4.1f%%)\n\0"),
                name,
                (*stats).nr_events[i as usize],
                100.0f64 * (*stats).nr_events[i as usize] as f64 / total as f64,
            ) as size_t;
        } else {
            ret += fprintf(
                fp,
                cstr(b"%20s events: %10d\n\0"),
                name,
                (*stats).nr_events[i as usize],
            ) as size_t;
        }
        i += 1;
    }

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
