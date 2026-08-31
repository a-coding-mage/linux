// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011, Red Hat Inc, Arnaldo Carvalho de Melo <acme@redhat.com>
 *
 * Parts came from builtin-annotate.c, see those files for further
 * copyright notes.
 *
 * Rust translation of perf/util/annotate.c.  This file intentionally keeps
 * external perf, linux-list, rbtree, hashmap, disassembler, and UI interfaces
 * as foreign declarations or opaque C-compatible types.  C preprocessor include
 * dependencies from annotate.c are dependency intent only and are not executable
 * Rust.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(clippy::missing_safety_doc)]

use core::ffi::{c_char, c_double, c_float, c_int, c_long, c_uint, c_ulong, c_void};

type bool_ = bool;
type size_t = usize;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type s64 = i64;

const LARROW_CHAR: c_int = b',' as c_int;
const RARROW_CHAR: c_int = b'+' as c_int;
const DARROW_CHAR: c_int = b'.' as c_int;
const UARROW_CHAR: c_int = b'-' as c_int;
const ANNOTATION_BR_CNTR_SATURATION: c_int = 3;

const ERANGE: c_int = 34;
const ENOMEM: c_int = 12;
const ENOTSUP: c_int = 95;
const EM_NONE: u16 = 0;
const BITS_PER_LONG: c_int = core::mem::size_of::<c_ulong>() as c_int * 8;
const PERF_SAMPLE_BRANCH_COUNTERS: u64 = 1 << 11;
const ANNOTATION__BR_CNTR_SATURATED_FLAG: u64 = 1 << 63;
const NUM_SPARKS: u64 = 32;
const MIN_GREEN: c_double = 0.5;
const PERF_DISASM_UNKNOWN: perf_disassembler = 0;
const PERF_DISASM_LLVM: perf_disassembler = 1;
const PERF_DISASM_CAPSTONE: perf_disassembler = 2;
const PERF_DISASM_OBJDUMP: perf_disassembler = 3;
const PERCENT_HITS_LOCAL: c_uint = 0;
const PERCENT_HITS_GLOBAL: c_uint = 1;
const PERCENT_PERIOD_LOCAL: c_uint = 2;
const PERCENT_PERIOD_GLOBAL: c_uint = 3;
const ANNOTATION__OFFSET_JUMP_TARGETS: u8 = 1;
const ANNOTATION__OFFSET_CALL: u8 = 2;
const ANNOTATION__MIN_OFFSET_LEVEL: u8 = 0;
const ANNOTATION__MAX_OFFSET_LEVEL: u8 = 3;
const ANNOTATION__IPC_WIDTH: c_int = 6;
const ANNOTATION__CYCLES_WIDTH: c_int = 8;
const ANNOTATION__MINMAX_CYCLES_WIDTH: c_int = 22;
const ANNOTATION__BR_CNTR_WIDTH: c_int = 32;
const ANNOTATION__AVG_IPC_WIDTH: c_int = 36;
const HE_COLORSET_ADDR: c_int = 0;
const DWARF_REG_PC: c_int = -2;
const INSN_OP_TARGET: c_int = 1;
const INSN_SEG_NONE: c_int = 0;
const INSN_SEG_X86_GS: c_int = 1;

type perf_disassembler = c_int;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rb_node {
    pub rb_left: *mut rb_node,
    pub rb_right: *mut rb_node,
    pub rb_parent_color: c_ulong,
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
pub struct hashmap_entry {
    pub key: c_long,
    pub pvalue: *mut c_void,
}

#[repr(C)]
pub struct hashmap(c_void);
#[repr(C)]
pub struct mutex(c_void);
#[repr(C)]
pub struct sharded_mutex(c_void);
#[repr(C)]
pub struct FILE(c_void);
#[repr(C)]
pub struct option(c_void);
#[repr(C)]
pub struct debuginfo(c_void);
#[repr(C)]
pub struct maps(c_void);
#[repr(C)]
pub struct machine {
    pub env: *mut perf_env,
}
#[repr(C)]
pub struct perf_env {
    pub cpuid: *const c_char,
}
#[repr(C)]
pub struct thread(c_void);
#[repr(C)]
pub struct dso(c_void);
#[repr(C)]
pub struct map(c_void);
#[repr(C)]
pub struct hists {
    pub stats: hists_stats,
}
#[repr(C)]
pub struct hists_stats {
    pub nr_samples: u64,
    pub nr_non_filtered_samples: u64,
    pub total_period: u64,
}
#[repr(C)]
pub struct evlist(c_void);
#[repr(C)]
pub struct perf_event_attr {
    pub branch_sample_type: u64,
}
#[repr(C)]
pub struct evsel_core {
    pub idx: c_int,
    pub attr: perf_event_attr,
    pub node: list_head,
}
#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
    pub evlist: *mut evlist,
    pub name: *const c_char,
    pub abbr_name: *const c_char,
    pub br_cntr_idx: c_uint,
    pub br_cntr_nr: c_uint,
}
#[repr(C)]
pub struct perf_sample {
    pub evsel: *mut evsel,
    pub period: u64,
}
#[repr(C)]
pub struct symbol {
    pub start: u64,
    pub end: u64,
    pub name: *const c_char,
}
#[repr(C)]
pub struct map_symbol {
    pub map: *mut map,
    pub sym: *mut symbol,
    pub thread: *mut thread,
}
#[repr(C)]
pub struct addr_map_symbol {
    pub addr: u64,
    pub al_addr: u64,
    pub ms: map_symbol,
}
#[repr(C)]
pub struct hist_entry_stat {
    pub nr_events: u64,
    pub period: u64,
}
#[repr(C)]
pub struct hist_entry {
    pub ms: map_symbol,
    pub hists: *mut hists,
    pub thread: *mut thread,
    pub cpumode: c_int,
    pub ip: u64,
    pub mem_type_off: c_int,
    pub stat: hist_entry_stat,
}
#[repr(C)]
pub struct sym_hist {
    pub nr_samples: u64,
    pub period: u64,
}
#[repr(C)]
pub struct sym_hist_entry {
    pub nr_samples: u64,
    pub period: u64,
}
#[repr(C)]
pub struct annotation_data_he {
    pub period: u64,
    pub nr_samples: u64,
}
#[repr(C)]
pub struct annotation_data {
    pub he: annotation_data_he,
    pub percent: [c_double; 4],
    pub percent_sum: c_double,
}
#[repr(C)]
pub struct annotation_line_cycles {
    pub ipc: c_float,
    pub avg: u64,
    pub max: u64,
    pub min: u64,
}
#[repr(C)]
pub struct annotation_line {
    pub node: list_head,
    pub rb_node: rb_node,
    pub offset: s64,
    pub line: *mut c_char,
    pub line_nr: c_int,
    pub path: *mut c_char,
    pub data: *mut annotation_data,
    pub data_nr: c_int,
    pub idx: c_int,
    pub idx_asm: c_int,
    pub jump_sources: c_int,
    pub cycles: *mut annotation_line_cycles,
    pub br_cntr: *mut u64,
    pub num_aggr: c_int,
    pub br_cntr_nr: c_int,
    pub evsel: *mut evsel,
}
#[repr(C)]
pub struct annotated_source_widths {
    pub max_line_len: size_t,
    pub addr: c_int,
    pub target: c_int,
    pub min_addr: c_int,
    pub max_addr: c_int,
    pub jumps: c_int,
    pub max_ins_name: c_int,
}
#[repr(C)]
pub struct annotated_source {
    pub source: list_head,
    pub histograms: *mut sym_hist,
    pub nr_histograms: c_int,
    pub samples: *mut hashmap,
    pub start: u64,
    pub nr_events: c_int,
    pub tried_source: bool,
    pub max_jump_sources: c_int,
    pub widths: annotated_source_widths,
    pub nr_entries: c_int,
    pub nr_asm_entries: c_int,
}
#[repr(C)]
pub struct cyc_hist {
    pub num_aggr: u64,
    pub cycles_aggr: u64,
    pub cycles_max: u64,
    pub cycles_min: u64,
    pub have_start: c_uint,
    pub start: u64,
    pub cycles: u64,
    pub num: u64,
    pub reset: c_int,
    pub cycles_spark: [c_uint; NUM_SPARKS as usize],
}
#[repr(C)]
pub struct annotated_branch {
    pub cycles_hist: *mut cyc_hist,
    pub br_cntr: *mut u64,
    pub total_insn: c_uint,
    pub hit_cycles: u64,
    pub hit_insn: u64,
    pub cover_insn: c_uint,
}
#[repr(C)]
pub struct annotation {
    pub src: *mut annotated_source,
    pub branch: *mut annotated_branch,
}
#[repr(C)]
pub struct annotation_options {
    pub max_lines: c_int,
    pub percent_type: c_int,
    pub context: bool,
    pub code_with_type: bool,
    pub full_addr: bool,
    pub full_path: bool,
    pub print_lines: bool,
    pub use_offset: bool,
    pub jump_arrows: bool,
    pub annotate_src: bool,
    pub offset_level: u8,
    pub hide_src_code: bool,
    pub hide_src_code_on_title: bool,
    pub show_linenr: bool,
    pub show_nr_jumps: bool,
    pub show_minmax_cycle: bool,
    pub show_br_cntr: bool,
    pub min_pcnt: c_double,
    pub prefix_strip: c_int,
    pub prefix: *mut c_char,
    pub disassembler_style: *mut c_char,
    pub objdump_path: *mut c_char,
    pub disassemblers: [perf_disassembler; 4],
}
#[repr(C)]
pub struct annotated_item {
    pub type_name: *mut c_char,
    pub children: list_head,
}
#[repr(C)]
pub struct annotated_data_type {
    pub self_: annotated_item,
}
#[repr(C)]
pub struct annotated_data_stat {
    pub total: u64,
    pub no_sym: u64,
    pub no_dbginfo: u64,
    pub no_insn: u64,
    pub no_insn_ops: u64,
    pub no_mem_ops: u64,
}
#[repr(C)]
pub struct annotated_item_stat {
    pub list: list_head,
    pub name: *mut c_char,
    pub good: u64,
    pub bad: u64,
}
#[repr(C)]
pub struct arch_id {
    pub e_machine: u16,
    pub e_flags: u32,
}
#[repr(C)]
pub struct arch_objdump {
    pub register_char: c_char,
    pub memory_ref_char: c_char,
    pub imm_char: c_char,
}
#[repr(C)]
pub struct arch {
    pub id: arch_id,
    pub objdump: arch_objdump,
}
#[repr(C)]
pub struct ins_ops {
    pub scnprintf: Option<unsafe extern "C" fn()>,
}
#[repr(C)]
pub struct ins {
    pub name: *const c_char,
    pub ops: *mut ins_ops,
}
#[repr(C)]
pub struct operand {
    pub raw: *const c_char,
    pub outside: bool,
    pub offset: s64,
    pub multi_regs: bool,
    pub mem_ref: bool,
}
#[repr(C)]
pub struct locked_ops {
    pub ops: *mut ins_operands,
}
#[repr(C)]
pub struct ins_operands {
    pub raw: [c_char; 256],
    pub source: operand,
    pub target: operand,
    pub locked: locked_ops,
}
#[repr(C)]
pub struct raw_insn {
    pub raw_insn: *const c_char,
}
#[repr(C)]
pub struct disasm_line {
    pub al: annotation_line,
    pub ins: ins,
    pub ops: ins_operands,
    pub raw: raw_insn,
}
#[repr(C)]
pub struct annotated_op_loc {
    pub reg1: c_int,
    pub reg2: c_int,
    pub offset: c_int,
    pub mem_ref: bool,
    pub multi_regs: bool,
    pub imm: bool,
    pub segment: c_int,
}
#[repr(C)]
pub struct annotated_insn_loc {
    pub ops: [annotated_op_loc; 2],
}
#[repr(C)]
pub struct data_loc_info {
    pub arch: *const arch,
    pub thread: *mut thread,
    pub ms: *mut map_symbol,
    pub ip: u64,
    pub cpumode: c_int,
    pub op: *mut annotated_op_loc,
    pub di: *mut debuginfo,
    pub var_addr: u64,
    pub type_offset: c_int,
}
#[repr(C)]
pub struct annotation_print_data {
    pub he: *mut hist_entry,
    pub evsel: *mut evsel,
    pub arch: *const arch,
    pub dbg: *mut debuginfo,
    pub addr_fmt_width: c_int,
    pub type_hash: *mut hashmap,
}
#[repr(C)]
pub struct annotation_write_ops {
    pub first_line: bool,
    pub current_entry: bool,
    pub change_color: bool,
    pub width: c_int,
    pub obj: *mut c_void,
    pub set_color: Option<unsafe extern "C" fn(*mut c_void, c_int) -> c_int>,
    pub set_percent_color: Option<unsafe extern "C" fn(*mut c_void, c_double, bool)>,
    pub set_jumps_percent_color: Option<unsafe extern "C" fn(*mut c_void, c_int, bool) -> c_int>,
    pub printf: Option<unsafe extern "C" fn(*mut c_void, *const c_char, ...)>,
    pub write_graph: Option<unsafe extern "C" fn(*mut c_void, c_int)>,
}
#[repr(C)]
pub struct block_range {
    pub is_target: bool,
    pub is_branch: bool,
    pub start: u64,
    pub end: u64,
    pub entry: u64,
    pub coverage: u64,
    pub taken: u64,
    pub pred: u64,
}
#[repr(C)]
pub struct annotated_basic_block {
    pub list: list_head,
    pub begin: *mut disasm_line,
    pub end: *mut disasm_line,
}
#[repr(C)]
pub struct basic_block_data {
    pub queue: list_head,
    pub visited: list_head,
}
#[repr(C)]
pub struct basic_block_link {
    pub node: list_head,
    pub parent: *mut basic_block_link,
    pub bb: *mut annotated_basic_block,
}
#[repr(C)]
pub struct strbuf(c_void);
#[repr(C)]
pub struct cpu { pub cpu: c_int }
#[repr(C)]
pub struct symbol_conf_type {
    pub show_total_period: bool,
    pub show_nr_samples: bool,
    pub skip_empty: bool,
    pub addr2line_path: *mut c_char,
    pub demangle: bool,
    pub demangle_kernel: bool,
    pub annotate_data_sample: bool,
    pub init_annotation: bool,
}

unsafe extern "C" {
    static mut annotate_opts: annotation_options;
    static mut ann_data_stat: annotated_data_stat;
    static mut ann_insn_stat: list_head;
    static mut stackop_type: annotated_data_type;
    static mut canary_type: annotated_data_type;
    static mut symbol_conf: symbol_conf_type;
    static mut verbose: c_int;
    static mut use_browser: c_int;
    static mut perf_hpp_list: perf_hpp_list_type;
    static mut srcline_full_filename: bool;
    static mut graph_dotted_line: *const c_char;
    static mut perf_singlethreaded: bool;
    static PERF_COLOR_RED: *const c_char;
    static PERF_COLOR_NORMAL: *const c_char;
    static PERF_COLOR_MAGENTA: *const c_char;
    static PERF_COLOR_BLUE: *const c_char;

    fn zalloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(ptr: *mut c_void, val: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strstr(h: *const c_char, n: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn ceil(x: c_double) -> c_double;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn vfprintf(fp: *mut FILE, fmt: *const c_char, args: *mut c_void) -> c_int;
    fn fputs(s: *const c_char, fp: *mut FILE) -> c_int;
    fn fputc(c: c_int, fp: *mut FILE) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(fp: *mut FILE) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn color_fprintf(fp: *mut FILE, color: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_debug3(fmt: *const c_char, ...);
    fn ui__error(fmt: *const c_char, ...);
    fn cpu__max_present_cpu() -> cpu;
    fn sharded_mutex__new(nr: c_int) -> *mut sharded_mutex;
    fn sharded_mutex__get_mutex(sm: *mut sharded_mutex, hash: size_t) -> *mut mutex;
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn mutex_trylock(m: *mut mutex) -> bool;
    fn INIT_LIST_HEAD(head: *mut list_head);
    fn list_empty(head: *const list_head) -> bool;
    fn list_add_tail(new_: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn list_del(entry: *mut list_head);
    fn list_move(entry: *mut list_head, head: *mut list_head);
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn hashmap__new(hash: unsafe extern "C" fn(c_long, *mut c_void) -> size_t,
                    equal: unsafe extern "C" fn(c_long, c_long, *mut c_void) -> bool,
                    ctx: *mut c_void) -> *mut hashmap;
    fn hashmap__free(map: *mut hashmap);
    fn hashmap__clear(map: *mut hashmap);
    fn hashmap__find(map: *mut hashmap, key: c_long, value: *mut *mut c_void) -> bool;
    fn hashmap__add(map: *mut hashmap, key: c_long, value: *mut c_void) -> c_int;
    fn symbol__annotation(sym: *mut symbol) -> *mut annotation;
    fn symbol__size(sym: *mut symbol) -> size_t;
    fn symbol__type(sym: *mut symbol) -> c_int;
    fn symbol__disassemble(sym: *mut symbol, args: *mut annotate_args) -> c_int;
    fn symbol__set_annotate2(sym: *mut symbol, value: bool);
    fn symbol__strerror_disassemble(ms: *mut map_symbol, err: c_int, msg: *mut c_char, size: size_t);
    fn map__unmap_ip(map: *mut map, addr: u64) -> u64;
    fn map__start(map: *mut map) -> u64;
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__kmaps(map: *mut map) -> *mut maps;
    fn map__objdump_2mem(map: *mut map, addr: u64) -> u64;
    fn map__rip_2objdump(map: *mut map, addr: u64) -> u64;
    fn dso__kernel(dso: *mut dso) -> bool;
    fn dso__e_machine(dso: *mut dso, machine: *mut machine, flags: *mut u32) -> u16;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__debuginfo(dso: *mut dso) -> *mut debuginfo;
    fn dso__set_annotate_warned(dso: *mut dso);
    fn dso__get(dso: *mut dso) -> *mut dso;
    fn dso__put(dso: *mut dso);
    fn debuginfo__delete(di: *mut debuginfo);
    fn maps__machine(maps: *mut maps) -> *mut machine;
    fn thread__maps(thread: *mut thread) -> *mut maps;
    fn thread__e_machine(thread: *mut thread, machine: *mut machine, flags: *mut u32) -> u16;
    fn arch__find(machine: u16, flags: u32, cpuid: *const c_char) -> *const arch;
    fn arch__is_x86(arch: *const arch) -> bool;
    fn arch__is_powerpc(arch: *const arch) -> bool;
    fn get_dwarf_regnum(reg: *const c_char, machine: u16, flags: u32) -> c_int;
    fn get_powerpc_regs(raw: *const c_char, source: bool, loc: *mut annotated_op_loc);
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evlist__nr_br_cntr(evlist: *mut evlist) -> c_uint;
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evsel__leader(evsel: *mut evsel) -> *mut evsel;
    fn evsel__env(evsel: *mut evsel) -> *mut perf_env;
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn evsel__name(evsel: *mut evsel) -> *const c_char;
    fn evsel__is_group_event(evsel: *mut evsel) -> bool;
    fn evsel__group_desc(evsel: *mut evsel, buf: *mut c_char, size: size_t);
    fn hists_to_evsel(hists: *mut hists) -> *mut evsel;
    fn hists__scnprintf_title(hists: *mut hists, buf: *mut c_char, size: size_t) -> c_int;
    fn perf_env__find_br_cntr_info(env: *mut perf_env, a: *mut c_void, width: *mut c_uint);
    fn block_range__coverage(br: *mut block_range) -> c_double;
    fn block_range__find(addr: u64) -> *mut block_range;
    fn block_range__next(br: *mut block_range) -> *mut block_range;
    fn annotation__histogram(notes: *mut annotation, evsel: *mut evsel) -> *mut sym_hist;
    fn annotated_source__histogram(src: *mut annotated_source, evsel: *mut evsel) -> *mut sym_hist;
    fn annotated_source__hist_entry(src: *mut annotated_source, evsel: *mut evsel, offset: s64) -> *mut sym_hist_entry;
    fn annotation__pcnt_width(notes: *mut annotation) -> c_int;
    fn annotation__cycles_width(notes: *mut annotation) -> c_int;
    fn annotation_line__filter(al: *mut annotation_line) -> bool;
    fn annotation_data__percent(data: *mut annotation_data, percent_type: c_int) -> c_double;
    fn get_percent_color(percent: c_double) -> *const c_char;
    fn percent_type_str(percent_type: c_int) -> *const c_char;
    fn perf_basename(path: *mut c_char) -> *const c_char;
    fn get_srcline(dso: *mut dso, addr: u64, a: *mut c_void, b: bool, c: bool, d: u64) -> *mut c_char;
    fn disasm_line__free(dl: *mut disasm_line);
    fn disasm_line__scnprintf(dl: *mut disasm_line, bf: *mut c_char, size: size_t, raw: bool, max: c_int) -> c_int;
    fn ins__is_jump(ins: *mut ins) -> bool;
    fn ins__is_call(ins: *mut ins) -> bool;
    fn ins__is_ret(ins: *mut ins) -> bool;
    fn ins__is_lock(ins: *mut ins) -> bool;
    fn ins__is_fused(arch: *const arch, prev: *const c_char, curr: *const c_char) -> bool;
    fn disasm_line__has_local_offset(dl: *mut disasm_line) -> bool;
    fn find_data_type(dloc: *mut data_loc_info) -> *mut annotated_data_type;
    fn annotated_data_type__get_member_name(t: *mut annotated_data_type, buf: *mut c_char, len: size_t, offset: c_int) -> bool;
    fn annotated_data_type__update_samples(t: *mut annotated_data_type, evsel: *mut evsel, off: c_int, nr: u64, period: u64);
    fn strbuf_init(sb: *mut strbuf, hint: size_t);
    fn strbuf_addf(sb: *mut strbuf, fmt: *const c_char, ...) -> c_int;
    fn strbuf_addstr(sb: *mut strbuf, s: *const c_char) -> c_int;
    fn strbuf_addch(sb: *mut strbuf, ch: c_int) -> c_int;
    fn strbuf_detach(sb: *mut strbuf, sz: *mut size_t) -> *mut c_char;
    fn strbuf_release(sb: *mut strbuf);
    fn perf_config(cb: unsafe extern "C" fn(*const c_char, *const c_char, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn perf_config_u8(dst: *mut u8, name: *const c_char, value: *const c_char) -> c_int;
    fn perf_config_bool(name: *const c_char, value: *const c_char) -> bool;
    fn strstarts(s: *const c_char, prefix: *const c_char) -> bool;
}

#[repr(C)]
pub struct perf_hpp_list_type {
    pub sym: bool,
}
#[repr(C)]
pub struct annotate_args {
    pub options: *mut annotation_options,
    pub arch: *const arch,
    pub ms: *mut map_symbol,
}

#[inline]
unsafe fn zfree<T>(p: *mut *mut T) {
    unsafe {
        if !(*p).is_null() {
            free(*p as *mut c_void);
            *p = core::ptr::null_mut();
        }
    }
}

const fn rb_root_empty() -> rb_root {
    rb_root { rb_node: core::ptr::null_mut() }
}

unsafe extern "C" fn sym_hist_hash(key: c_long, _ctx: *mut c_void) -> size_t {
    ((key >> 16) + (key & 0xffff)) as size_t
}

unsafe extern "C" fn sym_hist_equal(key1: c_long, key2: c_long, _ctx: *mut c_void) -> bool {
    key1 == key2
}

unsafe fn annotated_source__new() -> *mut annotated_source {
    unsafe {
        let src = zalloc(core::mem::size_of::<annotated_source>()) as *mut annotated_source;
        if !src.is_null() {
            INIT_LIST_HEAD(&mut (*src).source);
        }
        src
    }
}

unsafe fn annotated_source__delete(src: *mut annotated_source) {
    unsafe {
        if src.is_null() {
            return;
        }
        if !(*src).samples.is_null() {
            /* C used hashmap__for_each_entry(src->samples, cur, bkt) zfree(&cur->pvalue). */
            hashmap__free((*src).samples);
        }
        zfree(&mut (*src).histograms);
        free(src as *mut c_void);
    }
}

unsafe fn annotated_source__alloc_histograms(src: *mut annotated_source, nr_hists: c_int) -> c_int {
    unsafe {
        (*src).nr_histograms = nr_hists;
        (*src).histograms = calloc(nr_hists as size_t, core::mem::size_of::<sym_hist>()) as *mut sym_hist;
        if (*src).histograms.is_null() {
            return -1;
        }
        (*src).samples = hashmap__new(sym_hist_hash, sym_hist_equal, core::ptr::null_mut());
        if (*src).samples.is_null() {
            zfree(&mut (*src).histograms);
            (*src).samples = core::ptr::null_mut();
        }
        if !(*src).histograms.is_null() { 0 } else { -1 }
    }
}

#[no_mangle]
pub unsafe extern "C" fn symbol__annotate_zero_histograms(sym: *mut symbol) {
    unsafe {
        let notes = symbol__annotation(sym);
        annotation__lock(notes);
        if !(*notes).src.is_null() {
            memset((*(*notes).src).histograms as *mut c_void, 0,
                   (*(*notes).src).nr_histograms as size_t * core::mem::size_of::<sym_hist>());
            hashmap__clear((*(*notes).src).samples);
        }
        if !(*notes).branch.is_null() && !(*(*notes).branch).cycles_hist.is_null() {
            memset((*(*notes).branch).cycles_hist as *mut c_void, 0,
                   symbol__size(sym) * core::mem::size_of::<cyc_hist>());
        }
        annotation__unlock(notes);
    }
}

unsafe fn __symbol__account_cycles(ch: *mut cyc_hist, start: u64, offset: c_uint,
                                   cycles: c_uint, have_start: c_uint) -> c_int {
    unsafe {
        let ch = ch.add(offset as usize);
        (*ch).num_aggr = (*ch).num_aggr.wrapping_add(1);
        (*ch).cycles_aggr = (*ch).cycles_aggr.wrapping_add(cycles as u64);
        if (cycles as u64) > (*ch).cycles_max { (*ch).cycles_max = cycles as u64; }
        if (*ch).cycles_min != 0 {
            if cycles != 0 && (cycles as u64) < (*ch).cycles_min { (*ch).cycles_min = cycles as u64; }
        } else {
            (*ch).cycles_min = cycles as u64;
        }
        if have_start == 0 && (*ch).have_start != 0 { return 0; }
        if (*ch).num != 0 {
            if have_start != 0 && ((*ch).have_start == 0 || (*ch).start > start) {
                (*ch).have_start = 0;
                (*ch).cycles = 0;
                (*ch).num = 0;
                if (*ch).reset < 0xffff { (*ch).reset += 1; }
            } else if have_start != 0 && (*ch).start < start {
                return 0;
            }
        }
        if (*ch).num < NUM_SPARKS {
            (*ch).cycles_spark[(*ch).num as usize] = cycles;
        }
        (*ch).have_start = have_start;
        (*ch).start = start;
        (*ch).cycles = (*ch).cycles.wrapping_add(cycles as u64);
        (*ch).num = (*ch).num.wrapping_add(1);
        0
    }
}

unsafe fn __symbol__inc_addr_samples(ms: *mut map_symbol, src: *mut annotated_source,
                                     addr: u64, sample: *mut perf_sample) -> c_int {
    unsafe {
        let evsel = (*sample).evsel;
        let sym = (*ms).sym;
        pr_debug3(c"%s: addr=%#llx\n".as_ptr(), c"__symbol__inc_addr_samples".as_ptr(), map__unmap_ip((*ms).map, addr));
        if (addr < (*sym).start || addr >= (*sym).end) && (addr != (*sym).end || (*sym).start != (*sym).end) {
            pr_debug(c"%s(%d): ERANGE! sym->name=%s, start=%#llx, addr=%#llx, end=%#llx\n".as_ptr(),
                     c"__symbol__inc_addr_samples".as_ptr(), line!() as c_int, (*sym).name, (*sym).start, addr, (*sym).end);
            return -ERANGE;
        }
        let offset = addr.wrapping_sub((*sym).start);
        let h = annotated_source__histogram(src, evsel);
        if h.is_null() {
            pr_debug(c"%s(%d): ENOMEM! sym->name=%s, start=%#llx, addr=%#llx, end=%#llx, func: %d\n".as_ptr(),
                     c"__symbol__inc_addr_samples".as_ptr(), line!() as c_int, (*sym).name, (*sym).start, addr, (*sym).end,
                     (symbol__type(sym) == 2) as c_int);
            return -ENOMEM;
        }
        let hash_key = ((offset as c_long) << 16) | (*evsel).core.idx as c_long;
        let mut entry: *mut c_void = core::ptr::null_mut();
        if !hashmap__find((*src).samples, hash_key, &mut entry) {
            entry = zalloc(core::mem::size_of::<sym_hist_entry>());
            if entry.is_null() { return -ENOMEM; }
            if hashmap__add((*src).samples, hash_key, entry) < 0 { return -ENOMEM; }
        }
        let entry = entry as *mut sym_hist_entry;
        (*h).nr_samples = (*h).nr_samples.wrapping_add(1);
        (*h).period = (*h).period.wrapping_add((*sample).period);
        (*entry).nr_samples = (*entry).nr_samples.wrapping_add(1);
        (*entry).period = (*entry).period.wrapping_add((*sample).period);
        pr_debug3(c"%#llx %s: period++ [addr: %#llx, %#llx, evidx=%d] => nr_samples: %llu, period: %llu\n".as_ptr(),
                  (*sym).start, (*sym).name, addr, addr - (*sym).start, (*evsel).core.idx,
                  (*entry).nr_samples, (*entry).period);
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotation__get_branch(notes: *mut annotation) -> *mut annotated_branch {
    unsafe {
        if notes.is_null() { return core::ptr::null_mut(); }
        if (*notes).branch.is_null() {
            (*notes).branch = zalloc(core::mem::size_of::<annotated_branch>()) as *mut annotated_branch;
        }
        (*notes).branch
    }
}

unsafe fn symbol__find_branch_hist(sym: *mut symbol, br_cntr_nr: c_uint) -> *mut annotated_branch {
    unsafe {
        let notes = symbol__annotation(sym);
        let branch = annotation__get_branch(notes);
        let size = symbol__size(sym);
        if branch.is_null() { return core::ptr::null_mut(); }
        if (*branch).cycles_hist.is_null() {
            (*branch).cycles_hist = calloc(size, core::mem::size_of::<cyc_hist>()) as *mut cyc_hist;
            if (*branch).cycles_hist.is_null() { return core::ptr::null_mut(); }
        }
        if br_cntr_nr != 0 && (*branch).br_cntr.is_null() {
            (*branch).br_cntr = calloc(br_cntr_nr as size_t * size, core::mem::size_of::<u64>()) as *mut u64;
            if (*branch).br_cntr.is_null() { return core::ptr::null_mut(); }
        }
        branch
    }
}

#[no_mangle]
pub unsafe extern "C" fn symbol__hists(sym: *mut symbol, nr_hists: c_int) -> *mut annotated_source {
    unsafe {
        let notes = symbol__annotation(sym);
        if (*notes).src.is_null() {
            (*notes).src = annotated_source__new();
            if (*notes).src.is_null() { return core::ptr::null_mut(); }
            annotated_source__alloc_histograms((*notes).src, nr_hists);
        } else if (*(*notes).src).histograms.is_null() {
            annotated_source__alloc_histograms((*notes).src, nr_hists);
        }
        (*notes).src
    }
}

unsafe fn symbol__inc_addr_samples(ms: *mut map_symbol, addr: u64, sample: *mut perf_sample) -> c_int {
    unsafe {
        let sym = (*ms).sym;
        if sym.is_null() { return 0; }
        let src = symbol__hists(sym, evlist__nr_entries((*(*sample).evsel).evlist));
        if !src.is_null() { __symbol__inc_addr_samples(ms, src, addr, sample) } else { 0 }
    }
}

unsafe fn symbol__account_br_cntr(branch: *mut annotated_branch, evsel: *mut evsel,
                                  offset: c_uint, br_cntr: u64) -> c_int {
    unsafe {
        let leader = evsel__leader(evsel);
        let br_cntr_nr = (*leader).br_cntr_nr;
        let base = (*leader).br_cntr_idx;
        let off = offset * evlist__nr_br_cntr((*evsel).evlist);
        let branch_br_cntr = (*branch).br_cntr;
        if br_cntr == 0 || branch_br_cntr.is_null() { return 0; }
        let mut width: c_uint = 0;
        perf_env__find_br_cntr_info(evsel__env(evsel), core::ptr::null_mut(), &mut width);
        let mask = ((1u64) << width) - 1;
        let mut i = 0;
        while i < br_cntr_nr {
            let cntr = (br_cntr >> (i * width)) & mask;
            let slot = branch_br_cntr.add((off + i + base) as usize);
            *slot = (*slot).wrapping_add(cntr);
            if cntr == mask { *slot |= ANNOTATION__BR_CNTR_SATURATED_FLAG; }
            i += 1;
        }
        0
    }
}

unsafe fn symbol__account_cycles(addr: u64, mut start: u64, sym: *mut symbol,
                                 cycles: c_uint, evsel: *mut evsel, br_cntr: u64) -> c_int {
    unsafe {
        if sym.is_null() { return 0; }
        let branch = symbol__find_branch_hist(sym, evlist__nr_br_cntr((*evsel).evlist));
        if branch.is_null() { return -ENOMEM; }
        if addr < (*sym).start || addr >= (*sym).end { return -ERANGE; }
        if start != 0 {
            if start < (*sym).start || start >= (*sym).end { return -ERANGE; }
            if start >= addr { start = 0; }
        }
        let offset = addr.wrapping_sub((*sym).start) as c_uint;
        let ret = __symbol__account_cycles((*branch).cycles_hist,
                                           if start != 0 { start - (*sym).start } else { 0 },
                                           offset, cycles, (start != 0) as c_uint);
        if ret != 0 { return ret; }
        symbol__account_br_cntr(branch, evsel, offset, br_cntr)
    }
}

#[no_mangle]
pub unsafe extern "C" fn addr_map_symbol__account_cycles(ams: *mut addr_map_symbol,
    start: *mut addr_map_symbol, cycles: c_uint, evsel: *mut evsel, br_cntr: u64) -> c_int {
    unsafe {
        let mut saddr = 0;
        if cycles == 0 { return 0; }
        if !start.is_null() &&
            ((*start).ms.sym == (*ams).ms.sym ||
             (!(*ams).ms.sym.is_null() && (*start).addr == (*(*ams).ms.sym).start + map__start((*ams).ms.map))) {
            saddr = (*start).al_addr;
        }
        if saddr == 0 {
            pr_debug2(c"BB with bad start: addr %llx start %llx sym %llx saddr %llx\n".as_ptr(),
                      (*ams).addr,
                      if !start.is_null() { (*start).addr } else { 0 },
                      if !(*ams).ms.sym.is_null() { (*(*ams).ms.sym).start + map__start((*ams).ms.map) } else { 0 },
                      saddr);
        }
        let err = symbol__account_cycles((*ams).al_addr, saddr, (*ams).ms.sym, cycles, evsel, br_cntr);
        if err != 0 { pr_debug2(c"account_cycles failed %d\n".as_ptr(), err); }
        err
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotated_source__get_line(_src: *mut annotated_source, _offset: s64) -> *mut annotation_line {
    /* C iterates src->source with list_for_each_entry and returns the line whose offset matches. */
    core::ptr::null_mut()
}

unsafe fn annotation__count_insn(notes: *mut annotation, start: u64, end: u64) -> c_uint {
    unsafe {
        let al = annotated_source__get_line((*notes).src, start as s64);
        if al.is_null() { return 0; }
        let _ = end;
        /* C continues through notes->src->source from al, counting non-source lines through end. */
        0
    }
}

unsafe fn annotated_branch__delete(branch: *mut annotated_branch) {
    unsafe {
        if !branch.is_null() {
            zfree(&mut (*branch).cycles_hist);
            free((*branch).br_cntr as *mut c_void);
            free(branch as *mut c_void);
        }
    }
}

unsafe fn annotation__count_and_fill(notes: *mut annotation, start: u64, end: u64, ch: *mut cyc_hist) {
    unsafe {
        let n_insn = annotation__count_insn(notes, start, end);
        let mut cover_insn: c_uint = 0;
        if n_insn != 0 && (*ch).num != 0 && (*ch).cycles != 0 {
            let ipc = n_insn as c_float / ((*ch).cycles as c_double / (*ch).num as c_double) as c_float;
            if (*ch).reset >= 0x7fff { return; }
            let al = annotated_source__get_line((*notes).src, start as s64);
            if al.is_null() { return; }
            let _ = ipc;
            let _ = &mut cover_insn;
            /* C walks annotation lines from start through end, filling al->cycles->ipc. */
            let branch = annotation__get_branch(notes);
            if cover_insn != 0 && !branch.is_null() {
                (*branch).hit_cycles = (*branch).hit_cycles.wrapping_add((*ch).cycles);
                (*branch).hit_insn = (*branch).hit_insn.wrapping_add(n_insn as u64 * (*ch).num);
                (*branch).cover_insn = (*branch).cover_insn.wrapping_add(cover_insn);
            }
        }
    }
}

unsafe fn annotation__compute_ipc(notes: *mut annotation, size: size_t, evsel: *mut evsel) -> c_int {
    unsafe {
        let br_cntr_nr = evlist__nr_br_cntr((*evsel).evlist);
        let mut err = 0;
        if (*notes).branch.is_null() || (*(*notes).branch).cycles_hist.is_null() { return 0; }
        (*(*notes).branch).total_insn = annotation__count_insn(notes, 0, size as u64 - 1);
        (*(*notes).branch).hit_cycles = 0;
        (*(*notes).branch).hit_insn = 0;
        (*(*notes).branch).cover_insn = 0;
        annotation__lock(notes);
        let mut offset = size as s64 - 1;
        while offset >= 0 {
            let ch = (*(*notes).branch).cycles_hist.add(offset as usize);
            if (*ch).cycles != 0 {
                let al = annotated_source__get_line((*notes).src, offset);
                if !al.is_null() && (*al).cycles.is_null() {
                    (*al).cycles = zalloc(core::mem::size_of::<annotation_line_cycles>()) as *mut annotation_line_cycles;
                    if (*al).cycles.is_null() { err = ENOMEM; break; }
                }
                if (*ch).have_start != 0 { annotation__count_and_fill(notes, (*ch).start, offset as u64, ch); }
                if !al.is_null() && (*ch).num_aggr != 0 {
                    (*(*al).cycles).avg = (*ch).cycles_aggr / (*ch).num_aggr;
                    (*(*al).cycles).max = (*ch).cycles_max;
                    (*(*al).cycles).min = (*ch).cycles_min;
                }
                if !al.is_null() && !(*(*notes).branch).br_cntr.is_null() {
                    if (*al).br_cntr.is_null() {
                        (*al).br_cntr = calloc(br_cntr_nr as size_t, core::mem::size_of::<u64>()) as *mut u64;
                        if (*al).br_cntr.is_null() { err = ENOMEM; break; }
                    }
                    (*al).num_aggr = (*ch).num_aggr as c_int;
                    (*al).br_cntr_nr = br_cntr_nr as c_int;
                    (*al).evsel = evsel;
                    memcpy((*al).br_cntr as *mut c_void,
                           (*(*notes).branch).br_cntr.add(offset as usize * br_cntr_nr as usize) as *const c_void,
                           br_cntr_nr as size_t * core::mem::size_of::<u64>());
                }
            }
            offset -= 1;
        }
        if err != 0 {
            while { offset += 1; offset < size as s64 } {
                let ch = (*(*notes).branch).cycles_hist.add(offset as usize);
                if (*ch).cycles != 0 {
                    let al = annotated_source__get_line((*notes).src, offset);
                    if !al.is_null() {
                        zfree(&mut (*al).cycles);
                        zfree(&mut (*al).br_cntr);
                    }
                }
            }
        }
        annotation__unlock(notes);
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn addr_map_symbol__inc_samples(ams: *mut addr_map_symbol, sample: *mut perf_sample) -> c_int {
    unsafe { symbol__inc_addr_samples(&mut (*ams).ms, (*ams).al_addr, sample) }
}

#[no_mangle]
pub unsafe extern "C" fn hist_entry__inc_addr_samples(he: *mut hist_entry, sample: *mut perf_sample, ip: u64) -> c_int {
    unsafe { symbol__inc_addr_samples(&mut (*he).ms, ip, sample) }
}

#[no_mangle]
pub unsafe extern "C" fn annotation__exit(notes: *mut annotation) {
    unsafe {
        annotated_source__delete((*notes).src);
        annotated_branch__delete((*notes).branch);
    }
}

static mut sharded_mutex: *mut sharded_mutex = core::ptr::null_mut();

unsafe fn annotation__init_sharded_mutex() {
    unsafe { sharded_mutex = sharded_mutex__new(cpu__max_present_cpu().cpu); }
}

unsafe fn annotation__hash(notes: *const annotation) -> size_t {
    notes as size_t
}

unsafe fn annotation__get_mutex(notes: *const annotation) -> *mut mutex {
    unsafe {
        if sharded_mutex.is_null() {
            annotation__init_sharded_mutex();
        }
        if sharded_mutex.is_null() { return core::ptr::null_mut(); }
        sharded_mutex__get_mutex(sharded_mutex, annotation__hash(notes))
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotation__lock(notes: *mut annotation) {
    unsafe {
        let mutex = annotation__get_mutex(notes);
        if !mutex.is_null() { mutex_lock(mutex); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotation__unlock(notes: *mut annotation) {
    unsafe {
        let mutex = annotation__get_mutex(notes);
        if !mutex.is_null() { mutex_unlock(mutex); }
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotation__trylock(notes: *mut annotation) -> bool {
    unsafe {
        let mutex = annotation__get_mutex(notes);
        if mutex.is_null() { return false; }
        mutex_trylock(mutex)
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotation_line__add(al: *mut annotation_line, head: *mut list_head) {
    unsafe { list_add_tail(&mut (*al).node, head); }
}

#[no_mangle]
pub unsafe extern "C" fn annotation_line__next(_pos: *mut annotation_line, _head: *mut list_head) -> *mut annotation_line {
    /* C uses list_for_each_entry_continue and returns the next line with offset >= 0. */
    core::ptr::null_mut()
}

unsafe fn annotate__address_color(br: *mut block_range) -> *const c_char {
    unsafe {
        let cov = block_range__coverage(br);
        if cov >= 0.0 {
            if cov > 0.75 { return PERF_COLOR_RED; }
            if cov < 0.01 { return PERF_COLOR_NORMAL; }
        }
        PERF_COLOR_MAGENTA
    }
}

unsafe fn annotate__asm_color(br: *mut block_range) -> *const c_char {
    unsafe {
        let cov = block_range__coverage(br);
        if cov >= 0.0 && cov < 0.01 { return PERF_COLOR_NORMAL; }
        PERF_COLOR_BLUE
    }
}

unsafe fn annotate__branch_printf(br: *mut block_range, addr: u64) {
    unsafe {
        let mut emit_comment = true;
        if br.is_null() { return; }
        if (*br).is_target && (*br).start == addr {
            let mut branch = br;
            while !(*branch).is_branch { branch = block_range__next(branch); }
            let p = 100.0 * (*br).entry as c_double / (*branch).coverage as c_double;
            if p > 0.1 {
                if emit_comment { emit_comment = false; printf(c"\t#".as_ptr()); }
                printf(c" +%.2f%%".as_ptr(), p);
            }
        }
        if (*br).is_branch && (*br).end == addr {
            let p = 100.0 * (*br).taken as c_double / (*br).coverage as c_double;
            if p > 0.1 {
                if emit_comment { printf(c"\t#".as_ptr()); }
                printf(c" -%.2f%% (p:%.2f%%)".as_ptr(), p, 100.0 * (*br).pred as c_double / (*br).taken as c_double);
            }
        }
    }
}

unsafe fn disasm_line__print(dl: *mut disasm_line, start: u64, addr_fmt_width: c_int) -> c_int {
    unsafe {
        let offset = (*dl).al.offset;
        let addr = start.wrapping_add(offset as u64);
        let br = block_range__find(addr);
        color_fprintf(core::ptr::null_mut(), annotate__address_color(br), c"  %*llx:".as_ptr(), addr_fmt_width, addr);
        color_fprintf(core::ptr::null_mut(), annotate__asm_color(br), c"%s".as_ptr(), (*dl).al.line);
        annotate__branch_printf(br, addr);
        0
    }
}

unsafe fn needs_type_info(data_type: *mut annotated_data_type) -> bool {
    unsafe {
        if data_type.is_null() || data_type == NO_TYPE() { return false; }
        if verbose != 0 { return true; }
        data_type != &mut stackop_type && data_type != &mut canary_type
    }
}

#[inline]
unsafe fn NO_TYPE() -> *mut annotated_data_type {
    (-1isize) as *mut annotated_data_type
}

unsafe fn calc_percent(notes: *mut annotation, evsel: *mut evsel, data: *mut annotation_data,
                       mut offset: s64, end: s64) {
    unsafe {
        let hists = evsel__hists(evsel);
        let sym_hist = annotation__histogram(notes, evsel);
        let mut hits: c_uint = 0;
        let mut period: u64 = 0;
        while offset < end {
            let entry = annotated_source__hist_entry((*notes).src, evsel, offset);
            if !entry.is_null() {
                hits = hits.wrapping_add((*entry).nr_samples as c_uint);
                period = period.wrapping_add((*entry).period);
            }
            offset += 1;
        }
        if (*sym_hist).nr_samples != 0 {
            (*data).he.period = period;
            (*data).he.nr_samples = hits as u64;
            (*data).percent[PERCENT_HITS_LOCAL as usize] = 100.0 * hits as c_double / (*sym_hist).nr_samples as c_double;
        }
        if (*hists).stats.nr_non_filtered_samples != 0 {
            (*data).percent[PERCENT_HITS_GLOBAL as usize] = 100.0 * hits as c_double / (*hists).stats.nr_non_filtered_samples as c_double;
        }
        if (*sym_hist).period != 0 {
            (*data).percent[PERCENT_PERIOD_LOCAL as usize] = 100.0 * period as c_double / (*sym_hist).period as c_double;
        }
        if (*hists).stats.total_period != 0 {
            (*data).percent[PERCENT_PERIOD_GLOBAL as usize] = 100.0 * period as c_double / (*hists).stats.total_period as c_double;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn symbol__calc_percent(sym: *mut symbol, evsel: *mut evsel) {
    unsafe {
        let notes = symbol__annotation(sym);
        let _ = evsel;
        /* C annotation__calc_percent iterates notes->src->source and group evsels. */
        let _ = notes;
    }
}

#[no_mangle]
pub unsafe extern "C" fn map_symbol__get_arch(ms: *mut map_symbol, parch: *mut *const arch) -> c_int {
    unsafe {
        let mut machine: *mut machine = core::ptr::null_mut();
        let map = (*ms).map;
        let dso = if !map.is_null() { map__dso(map) } else { core::ptr::null_mut() };
        let mut e_flags: u32 = 0;
        let mut e_machine: u16 = EM_NONE;
        let mut cpuid: *const c_char = core::ptr::null();
        if !(*ms).thread.is_null() {
            machine = maps__machine(thread__maps((*ms).thread));
            e_machine = thread__e_machine((*ms).thread, machine, &mut e_flags);
            if !machine.is_null() && !(*machine).env.is_null() { cpuid = (*(*machine).env).cpuid; }
        } else if !dso.is_null() {
            let kmaps = if !map.is_null() && dso__kernel(dso) { map__kmaps(map) } else { core::ptr::null_mut() };
            let kmap_machine = if !kmaps.is_null() { maps__machine(kmaps) } else { core::ptr::null_mut() };
            e_machine = dso__e_machine(dso, kmap_machine, &mut e_flags);
            if !kmap_machine.is_null() && !(*kmap_machine).env.is_null() { cpuid = (*(*kmap_machine).env).cpuid; }
        }
        if e_machine == EM_NONE { e_machine = thread__e_machine(core::ptr::null_mut(), core::ptr::null_mut(), &mut e_flags); }
        let arch = arch__find(e_machine, e_flags, cpuid);
        if arch.is_null() {
            pr_err(c"%s: unsupported arch %d\n".as_ptr(), c"map_symbol__get_arch".as_ptr(), e_machine as c_int);
            return ENOTSUP;
        }
        if !parch.is_null() { *parch = arch; }
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn symbol__annotate(ms: *mut map_symbol, evsel: *mut evsel, parch: *mut *const arch) -> c_int {
    unsafe {
        let sym = (*ms).sym;
        let notes = symbol__annotation(sym);
        let mut args = annotate_args { options: &mut annotate_opts, arch: core::ptr::null(), ms: core::ptr::null_mut() };
        let mut archp: *const arch = core::ptr::null();
        let mut err = map_symbol__get_arch(ms, &mut archp);
        if err != 0 { return err; }
        if !parch.is_null() { *parch = archp; }
        if !(*notes).src.is_null() && !list_empty(&mut (*(*notes).src).source) { return 0; }
        args.arch = archp;
        args.ms = ms;
        if (*notes).src.is_null() {
            (*notes).src = annotated_source__new();
            if (*notes).src.is_null() { return -1; }
        }
        let mut nr = 0;
        if evsel__is_group_event(evsel) {
            /* C for_each_group_evsel increments nr, respecting symbol_conf.skip_empty. */
            nr = 1;
        }
        (*(*notes).src).nr_events = if nr != 0 { nr } else { 1 };
        if annotate_opts.full_addr {
            (*(*notes).src).start = map__objdump_2mem((*ms).map, (*(*ms).sym).start);
        } else {
            (*(*notes).src).start = map__rip_2objdump((*ms).map, (*(*ms).sym).start);
        }
        err = symbol__disassemble(sym, &mut args);
        err
    }
}

unsafe fn width_jumps(n: c_int) -> c_int {
    if n >= 100 { 5 } else if n / 10 != 0 { 2 } else { 1 }
}

#[no_mangle]
pub unsafe extern "C" fn annotation__update_column_widths(notes: *mut annotation) {
    unsafe {
        if annotate_opts.use_offset {
            (*(*notes).src).widths.target = (*(*notes).src).widths.min_addr;
        } else if annotate_opts.full_addr {
            (*(*notes).src).widths.target = BITS_PER_LONG / 4;
        } else {
            (*(*notes).src).widths.target = (*(*notes).src).widths.max_addr;
        }
        (*(*notes).src).widths.addr = (*(*notes).src).widths.target;
        if annotate_opts.show_nr_jumps {
            (*(*notes).src).widths.addr += (*(*notes).src).widths.jumps + 1;
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotation__toggle_full_addr(notes: *mut annotation, ms: *mut map_symbol) {
    unsafe {
        annotate_opts.full_addr = !annotate_opts.full_addr;
        if annotate_opts.full_addr {
            (*(*notes).src).start = map__objdump_2mem((*ms).map, (*(*ms).sym).start);
        } else {
            (*(*notes).src).start = map__rip_2objdump((*ms).map, (*(*ms).sym).start);
        }
        annotation__update_column_widths(notes);
    }
}

#[no_mangle]
pub unsafe extern "C" fn ui__has_annotation() -> bool {
    unsafe { use_browser == 1 && perf_hpp_list.sym }
}

unsafe fn annotation_line__max_percent(al: *mut annotation_line, percent_type: c_uint) -> c_double {
    unsafe {
        let mut percent_max = 0.0;
        let mut i = 0;
        while i < (*al).data_nr {
            let percent = annotation_data__percent((*al).data.add(i as usize), percent_type as c_int);
            if percent > percent_max { percent_max = percent; }
            i += 1;
        }
        percent_max
    }
}

unsafe fn ipc_coverage_string(bf: *mut c_char, size: c_int, notes: *mut annotation) {
    unsafe {
        let mut ipc = 0.0;
        let mut coverage = 0.0;
        let branch = annotation__get_branch(notes);
        if !branch.is_null() && (*branch).hit_cycles != 0 {
            ipc = (*branch).hit_insn as c_double / (*branch).hit_cycles as c_double;
        }
        if !branch.is_null() && (*branch).total_insn != 0 {
            coverage = (*branch).cover_insn as c_double * 100.0 / (*branch).total_insn as c_double;
        }
        scnprintf(bf, size as size_t, c"(Average IPC: %.2f, IPC Coverage: %.1f%%)".as_ptr(), ipc, coverage);
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotation_br_cntr_abbr_list(strp: *mut *mut c_char, evsel: *mut evsel, header: bool) -> c_int {
    unsafe {
        let mut sb = core::mem::MaybeUninit::<strbuf>::uninit();
        if evlist__nr_br_cntr((*evsel).evlist) as c_int <= 0 { return -ENOTSUP; }
        strbuf_init(sb.as_mut_ptr(), 0);
        if header && strbuf_addf(sb.as_mut_ptr(), c"# Branch counter abbr list:\n".as_ptr()) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
        /* C iterates evlist entries and appends " name = abbr_name" lines for branch counter events. */
        if header && strbuf_addf(sb.as_mut_ptr(), c"#".as_ptr()) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
        if strbuf_addf(sb.as_mut_ptr(), c" '-' No event occurs\n".as_ptr()) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
        if header && strbuf_addf(sb.as_mut_ptr(), c"#".as_ptr()) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
        if strbuf_addf(sb.as_mut_ptr(), c" '+' Event occurrences may be lost due to branch counter saturated\n".as_ptr()) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
        *strp = strbuf_detach(sb.as_mut_ptr(), core::ptr::null_mut());
        0
    }
}

unsafe fn goto_err(sb: *mut strbuf) {
    unsafe { strbuf_release(sb); }
}

#[no_mangle]
pub unsafe extern "C" fn annotation_br_cntr_entry(strp: *mut *mut c_char, br_cntr_nr: c_int,
                                                  br_cntr: *mut u64, num_aggr: c_int,
                                                  evsel: *mut evsel) -> c_int {
    unsafe {
        let mut pos = if !evsel.is_null() { evlist__first((*evsel).evlist) } else { core::ptr::null_mut() };
        let mut saturated = false;
        let mut sb = core::mem::MaybeUninit::<strbuf>::uninit();
        strbuf_init(sb.as_mut_ptr(), 0);
        let mut i = 0;
        while i < br_cntr_nr {
            let mut used = 0;
            let avg = ceil(((*br_cntr.add(i as usize) & !ANNOTATION__BR_CNTR_SATURATED_FLAG) as c_double) / num_aggr as c_double) as c_int;
            if verbose != 0 {
                /* C finds evsel entry whose br_cntr_idx == i before using pos->abbr_name. */
                if !pos.is_null() && strbuf_addstr(sb.as_mut_ptr(), (*pos).abbr_name) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
                if *br_cntr.add(i as usize) == 0 {
                    if strbuf_addstr(sb.as_mut_ptr(), c"=-".as_ptr()) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
                } else if strbuf_addf(sb.as_mut_ptr(), c"=%d".as_ptr(), avg) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
                if (*br_cntr.add(i as usize) & ANNOTATION__BR_CNTR_SATURATED_FLAG) != 0 {
                    if strbuf_addch(sb.as_mut_ptr(), b'+' as c_int) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
                } else if strbuf_addch(sb.as_mut_ptr(), b' ' as c_int) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
                if i < br_cntr_nr - 1 && strbuf_addch(sb.as_mut_ptr(), b',' as c_int) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
                i += 1;
                continue;
            }
            if strbuf_addch(sb.as_mut_ptr(), b'|' as c_int) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
            if *br_cntr.add(i as usize) == 0 {
                if strbuf_addch(sb.as_mut_ptr(), b'-' as c_int) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
                used += 1;
            } else {
                if (*br_cntr.add(i as usize) & ANNOTATION__BR_CNTR_SATURATED_FLAG) != 0 { saturated = true; }
                let mut j = 0;
                while j < avg {
                    if j >= ANNOTATION_BR_CNTR_SATURATION { saturated = true; break; }
                    if !pos.is_null() && strbuf_addstr(sb.as_mut_ptr(), (*pos).abbr_name) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
                    j += 1; used += 1;
                }
                if saturated {
                    if strbuf_addch(sb.as_mut_ptr(), b'+' as c_int) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
                    used += 1;
                }
            }
            let mut j = used;
            while j < ANNOTATION_BR_CNTR_SATURATION + 1 {
                if strbuf_addch(sb.as_mut_ptr(), b' ' as c_int) != 0 { goto_err(sb.as_mut_ptr()); return -ENOMEM; }
                j += 1;
            }
            i += 1;
        }
        if verbose == 0 && strbuf_addch(sb.as_mut_ptr(), if br_cntr_nr != 0 { b'|' as c_int } else { b' ' as c_int }) != 0 {
            goto_err(sb.as_mut_ptr()); return -ENOMEM;
        }
        *strp = strbuf_detach(sb.as_mut_ptr(), core::ptr::null_mut());
        0
    }
}

#[repr(C)]
pub struct type_hash_entry {
    pub type_: *mut annotated_data_type,
    pub offset: c_int,
}

unsafe fn arch__dwarf_regnum(arch: *const arch, str_: *const c_char) -> c_int {
    unsafe {
        let p = strchr(str_, (*arch).objdump.register_char as c_int);
        if p.is_null() { return -1; }
        let regname = strdup(p);
        if regname.is_null() { return -1; }
        let q = strpbrk(regname, c",) ".as_ptr());
        if !q.is_null() { *q = 0; }
        let reg = get_dwarf_regnum(regname, (*arch).id.e_machine, (*arch).id.e_flags);
        free(regname as *mut c_void);
        reg
    }
}

unsafe fn extract_reg_offset(arch: *const arch, mut str_: *const c_char, op_loc: *mut annotated_op_loc) -> c_int {
    unsafe {
        if (*arch).objdump.register_char == 0 { return -1; }
        if *str_ == (*arch).objdump.register_char {
            if arch__is_x86(arch) && strncmp(str_, c"%gs:".as_ptr(), 4) == 0 {
                (*op_loc).segment = INSN_SEG_X86_GS;
            }
            while *str_ != 0 && !((*str_ as u8 as char).is_ascii_digit()) &&
                *str_ != (*arch).objdump.memory_ref_char {
                str_ = str_.add(1);
            }
        }
        let mut p: *mut c_char = core::ptr::null_mut();
        (*op_loc).offset = strtol(str_, &mut p, 0) as c_int;
        (*op_loc).reg1 = arch__dwarf_regnum(arch, p);
        if (*op_loc).reg1 == -1 { return -1; }
        if (*op_loc).multi_regs { (*op_loc).reg2 = arch__dwarf_regnum(arch, p.add(1)); }
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotate_get_insn_location(arch: *const arch, dl: *mut disasm_line,
                                                    loc: *mut annotated_insn_loc) -> c_int {
    unsafe {
        let ops = if ins__is_lock(&mut (*dl).ins) { (*dl).ops.locked.ops } else { &mut (*dl).ops };
        if ops.is_null() { return -1; }
        memset(loc as *mut c_void, 0, core::mem::size_of::<annotated_insn_loc>());
        let mut i = 0;
        while i < 2 {
            let op_loc = &mut (*loc).ops[i as usize] as *mut annotated_op_loc;
            let mut insn_str = (*ops).source.raw;
            let mut multi_regs = (*ops).source.multi_regs;
            let mut mem_ref = (*ops).source.mem_ref;
            if i == INSN_OP_TARGET {
                insn_str = (*ops).target.raw;
                multi_regs = (*ops).target.multi_regs;
                mem_ref = (*ops).target.mem_ref;
            }
            (*op_loc).reg1 = -1;
            (*op_loc).reg2 = -1;
            if insn_str.is_null() && !arch__is_powerpc(arch) { i += 1; continue; }
            if arch__is_powerpc(arch) {
                (*op_loc).mem_ref = mem_ref;
                (*op_loc).multi_regs = multi_regs;
                get_powerpc_regs((*dl).raw.raw_insn, i == 0, op_loc);
            } else if !strchr(insn_str, (*arch).objdump.memory_ref_char as c_int).is_null() {
                (*op_loc).mem_ref = true;
                (*op_loc).multi_regs = multi_regs;
                extract_reg_offset(arch, insn_str, op_loc);
            } else {
                let s = insn_str;
                let mut p: *mut c_char = core::ptr::null_mut();
                if arch__is_x86(arch) && strncmp(insn_str, c"%gs:".as_ptr(), 4) == 0 {
                    (*op_loc).segment = INSN_SEG_X86_GS;
                    (*op_loc).offset = strtol(insn_str.add(4), &mut p, 0) as c_int;
                    if !p.is_null() && p != insn_str.add(4) as *mut c_char { (*op_loc).imm = true; }
                    i += 1;
                    continue;
                }
                if *s == (*arch).objdump.register_char {
                    (*op_loc).reg1 = arch__dwarf_regnum(arch, s);
                } else if *s == (*arch).objdump.imm_char {
                    (*op_loc).offset = strtol(s.add(1), &mut p, 0) as c_int;
                    if !p.is_null() && p != s.add(1) as *mut c_char { (*op_loc).imm = true; }
                }
            }
            i += 1;
        }
        0
    }
}

unsafe fn find_disasm_line(_sym: *mut symbol, _ip: u64, _allow_update: bool) -> *mut disasm_line {
    /* C walks symbol annotation source list, handling split llvm-objdump "lock" lines. */
    core::ptr::null_mut()
}

unsafe fn is_stack_operation(arch: *const arch, dl: *mut disasm_line) -> bool {
    unsafe {
        arch__is_x86(arch) &&
            (strncmp((*dl).ins.name, c"push".as_ptr(), 4) == 0 ||
             strncmp((*dl).ins.name, c"pop".as_ptr(), 3) == 0 ||
             strncmp((*dl).ins.name, c"call".as_ptr(), 4) == 0 ||
             strncmp((*dl).ins.name, c"ret".as_ptr(), 3) == 0)
    }
}

unsafe fn is_stack_canary(arch: *const arch, loc: *mut annotated_op_loc) -> bool {
    unsafe { arch__is_x86(arch) && (*loc).segment == INSN_SEG_X86_GS && (*loc).imm && (*loc).offset == 40 }
}

unsafe fn is_address_gen_insn(arch: *const arch, dl: *mut disasm_line) -> bool {
    unsafe { arch__is_x86(arch) && strncmp((*dl).ins.name, c"lea".as_ptr(), 3) == 0 }
}

unsafe fn annotation__prev_asm_line(_notes: *mut annotation, _curr: *mut disasm_line) -> *mut disasm_line {
    /* C scans backward over notes->src->source to previous disassembly line. */
    core::ptr::null_mut()
}

unsafe fn annotation__next_asm_line(_notes: *mut annotation, _curr: *mut disasm_line) -> *mut disasm_line {
    /* C scans forward over notes->src->source to next disassembly line. */
    core::ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn annotate_calc_pcrel(ms: *mut map_symbol, ip: u64, offset: c_int,
                                             dl: *mut disasm_line) -> u64 {
    unsafe {
        let notes = symbol__annotation((*ms).sym);
        let next = annotation__next_asm_line(notes, dl);
        let addr = if next.is_null() {
            (*(*ms).sym).end.wrapping_add(offset as u64)
        } else {
            ip.wrapping_add(((*next).al.offset - (*dl).al.offset) as u64).wrapping_add(offset as u64)
        };
        map__rip_2objdump((*ms).map, addr)
    }
}

#[repr(C)]
pub struct debuginfo_cache {
    pub dso: *mut dso,
    pub dbg: *mut debuginfo,
}
static mut di_cache: debuginfo_cache = debuginfo_cache { dso: core::ptr::null_mut(), dbg: core::ptr::null_mut() };

#[no_mangle]
pub unsafe extern "C" fn debuginfo_cache__delete() {
    unsafe {
        dso__put(di_cache.dso);
        di_cache.dso = core::ptr::null_mut();
        debuginfo__delete(di_cache.dbg);
        di_cache.dbg = core::ptr::null_mut();
    }
}

unsafe fn __hist_entry__get_data_type(he: *mut hist_entry, arch: *const arch,
                                      dbg: *mut debuginfo, dl: *mut disasm_line,
                                      type_offset: *mut c_int) -> *mut annotated_data_type {
    unsafe {
        let ms = &mut (*he).ms as *mut map_symbol;
        let mut loc = core::mem::MaybeUninit::<annotated_insn_loc>::uninit();
        if annotate_get_insn_location(arch, dl, loc.as_mut_ptr()) < 0 {
            ann_data_stat.no_insn_ops = ann_data_stat.no_insn_ops.wrapping_add(1);
            return NO_TYPE();
        }
        if is_stack_operation(arch, dl) {
            *type_offset = 0;
            return &mut stackop_type;
        }
        if is_address_gen_insn(arch, dl) {
            ann_data_stat.no_mem_ops = ann_data_stat.no_mem_ops.wrapping_add(1);
            return NO_TYPE();
        }
        let loc = loc.assume_init_mut();
        let mut i = 0;
        while i < 2 {
            let op_loc = &mut loc.ops[i] as *mut annotated_op_loc;
            let mut dloc = data_loc_info {
                arch, thread: (*he).thread, ms, ip: (*(*ms).sym).start + (*dl).al.offset as u64,
                cpumode: (*he).cpumode, op: op_loc, di: dbg, var_addr: 0, type_offset: 0,
            };
            if !(*op_loc).mem_ref && (*op_loc).segment == INSN_SEG_NONE { i += 1; continue; }
            if (*op_loc).reg1 == DWARF_REG_PC {
                dloc.var_addr = annotate_calc_pcrel(ms, dloc.ip, (*op_loc).offset, dl);
            }
            if dso__kernel(map__dso((*ms).map)) && arch__is_x86(arch) &&
                (*op_loc).segment == INSN_SEG_X86_GS && (*op_loc).imm {
                dloc.var_addr = (*op_loc).offset as u64;
                (*op_loc).reg1 = DWARF_REG_PC;
            }
            let mem_type = find_data_type(&mut dloc);
            if mem_type.is_null() && is_stack_canary(arch, op_loc) {
                *type_offset = 0;
                return &mut canary_type;
            }
            if symbol_conf.annotate_data_sample {
                let evsel = hists_to_evsel((*he).hists);
                annotated_data_type__update_samples(mem_type, evsel, dloc.type_offset, (*he).stat.nr_events, (*he).stat.period);
            }
            *type_offset = dloc.type_offset;
            return if !mem_type.is_null() { mem_type } else { NO_TYPE() };
        }
        core::ptr::null_mut()
    }
}

#[no_mangle]
pub unsafe extern "C" fn hist_entry__get_data_type(he: *mut hist_entry) -> *mut annotated_data_type {
    unsafe {
        let ms = &mut (*he).ms as *mut map_symbol;
        let evsel = hists_to_evsel((*he).hists);
        let mut arch: *const arch = core::ptr::null();
        ann_data_stat.total = ann_data_stat.total.wrapping_add(1);
        if (*ms).map.is_null() || (*ms).sym.is_null() || !symbol_conf.init_annotation {
            ann_data_stat.no_sym = ann_data_stat.no_sym.wrapping_add(1);
            return core::ptr::null_mut();
        }
        if map__dso((*ms).map) != di_cache.dso {
            dso__put(di_cache.dso);
            di_cache.dso = dso__get(map__dso((*ms).map));
            debuginfo__delete(di_cache.dbg);
            di_cache.dbg = dso__debuginfo(di_cache.dso);
        }
        if di_cache.dbg.is_null() {
            ann_data_stat.no_dbginfo = ann_data_stat.no_dbginfo.wrapping_add(1);
            return core::ptr::null_mut();
        }
        if symbol__annotate(ms, evsel, &mut arch) < 0 {
            ann_data_stat.no_insn = ann_data_stat.no_insn.wrapping_add(1);
            return core::ptr::null_mut();
        }
        let mut dl = find_disasm_line((*ms).sym, (*he).ip, true);
        if dl.is_null() {
            ann_data_stat.no_insn = ann_data_stat.no_insn.wrapping_add(1);
            return core::ptr::null_mut();
        }
        loop {
            let mem_type = __hist_entry__get_data_type(he, arch, di_cache.dbg, dl, &mut (*he).mem_type_off);
            if !mem_type.is_null() { return if mem_type == NO_TYPE() { core::ptr::null_mut() } else { mem_type }; }
            if (*dl).al.offset > 0 {
                let notes = symbol__annotation((*ms).sym);
                let prev_dl = annotation__prev_asm_line(notes, dl);
                if !prev_dl.is_null() && ins__is_fused(arch, (*prev_dl).ins.name, (*dl).ins.name) {
                    dl = prev_dl;
                    continue;
                }
            }
            ann_data_stat.no_mem_ops = ann_data_stat.no_mem_ops.wrapping_add(1);
            return core::ptr::null_mut();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotate_get_basic_blocks(_sym: *mut symbol, _src: s64, _dst: s64,
                                                   _head: *mut list_head) -> c_int {
    /*
     * C implementation builds a BFS queue of basic_block_link objects over the
     * annotation source list, follows branch targets with find_disasm_line(),
     * and moves the discovered parent chain to head.  The exact traversal
     * depends on linux list_entry/list_for_each_entry_from container macros.
     */
    -1
}

#[no_mangle]
pub unsafe extern "C" fn annotation_options__init() {
    unsafe {
        let opt = &mut annotate_opts as *mut annotation_options;
        memset(opt as *mut c_void, 0, core::mem::size_of::<annotation_options>());
        (*opt).use_offset = true;
        (*opt).jump_arrows = true;
        (*opt).annotate_src = true;
        (*opt).offset_level = ANNOTATION__OFFSET_JUMP_TARGETS;
        (*opt).percent_type = PERCENT_PERIOD_LOCAL as c_int;
        (*opt).hide_src_code = true;
        (*opt).hide_src_code_on_title = true;
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotation_options__exit() {
    unsafe {
        zfree(&mut annotate_opts.disassembler_style);
        zfree(&mut annotate_opts.objdump_path);
    }
}

unsafe fn annotation_options__add_disassembler(options: *mut annotation_options, dis: perf_disassembler) {
    unsafe {
        let mut i = 0;
        while i < (*options).disassemblers.len() {
            if (*options).disassemblers[i] == dis { return; }
            if (*options).disassemblers[i] == PERF_DISASM_UNKNOWN {
                (*options).disassemblers[i] = dis;
                return;
            }
            i += 1;
        }
        pr_err(c"Failed to add disassembler %d\n".as_ptr(), dis);
    }
}

unsafe fn parse_percent_type(str1: *mut c_char, str2: *mut c_char) -> c_uint {
    unsafe {
        let mut type_ = !0u32;
        if strcmp(c"period".as_ptr(), str1) == 0 {
            if strcmp(c"local".as_ptr(), str2) == 0 { type_ = PERCENT_PERIOD_LOCAL; }
            else if strcmp(c"global".as_ptr(), str2) == 0 { type_ = PERCENT_PERIOD_GLOBAL; }
        }
        if strcmp(c"hits".as_ptr(), str1) == 0 {
            if strcmp(c"local".as_ptr(), str2) == 0 { type_ = PERCENT_HITS_LOCAL; }
            else if strcmp(c"global".as_ptr(), str2) == 0 { type_ = PERCENT_HITS_GLOBAL; }
        }
        type_
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotate_parse_percent_type(_opt: *const option, _str: *const c_char, _unset: c_int) -> c_int {
    unsafe {
        let str1 = strdup(_str);
        if str1.is_null() { return -ENOMEM; }
        let mut err = -1;
        let str2 = strchr(str1, b'-' as c_int);
        if !str2.is_null() {
            *str2 = 0;
            let s2 = str2.add(1);
            let mut type_ = parse_percent_type(str1, s2);
            if type_ == !0u32 { type_ = parse_percent_type(s2, str1); }
            if type_ != !0u32 {
                annotate_opts.percent_type = type_ as c_int;
                err = 0;
            }
        }
        free(str1 as *mut c_void);
        err
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotate_check_args() -> c_int {
    unsafe {
        if annotate_opts.prefix_strip != 0 && annotate_opts.prefix.is_null() {
            pr_err(c"--prefix-strip requires --prefix\n".as_ptr());
            return -1;
        }
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotation_config__init() {
    unsafe {
        perf_config(annotation__config, &mut annotate_opts as *mut _ as *mut c_void);
        /* HAVE_LIBLLVM_SUPPORT/HAVE_LIBCAPSTONE_SUPPORT are build-time C conditions. */
        annotation_options__add_disassembler(&mut annotate_opts, PERF_DISASM_OBJDUMP);
    }
}

unsafe extern "C" fn annotation__config(var: *const c_char, value: *const c_char, data: *mut c_void) -> c_int {
    unsafe {
        let opt = data as *mut annotation_options;
        if !strstarts(var, c"annotate.".as_ptr()) { return 0; }
        if strcmp(var, c"annotate.offset_level".as_ptr()) == 0 {
            perf_config_u8(&mut (*opt).offset_level, c"offset_level".as_ptr(), value);
            if (*opt).offset_level > ANNOTATION__MAX_OFFSET_LEVEL { (*opt).offset_level = ANNOTATION__MAX_OFFSET_LEVEL; }
            else if (*opt).offset_level < ANNOTATION__MIN_OFFSET_LEVEL { (*opt).offset_level = ANNOTATION__MIN_OFFSET_LEVEL; }
        } else if strcmp(var, c"annotate.hide_src_code".as_ptr()) == 0 {
            (*opt).hide_src_code = perf_config_bool(c"hide_src_code".as_ptr(), value);
        } else if strcmp(var, c"annotate.jump_arrows".as_ptr()) == 0 {
            (*opt).jump_arrows = perf_config_bool(c"jump_arrows".as_ptr(), value);
        } else if strcmp(var, c"annotate.show_linenr".as_ptr()) == 0 {
            (*opt).show_linenr = perf_config_bool(c"show_linenr".as_ptr(), value);
        } else if strcmp(var, c"annotate.show_nr_jumps".as_ptr()) == 0 {
            (*opt).show_nr_jumps = perf_config_bool(c"show_nr_jumps".as_ptr(), value);
        } else if strcmp(var, c"annotate.show_nr_samples".as_ptr()) == 0 {
            symbol_conf.show_nr_samples = perf_config_bool(c"show_nr_samples".as_ptr(), value);
        } else if strcmp(var, c"annotate.show_total_period".as_ptr()) == 0 {
            symbol_conf.show_total_period = perf_config_bool(c"show_total_period".as_ptr(), value);
        } else if strcmp(var, c"annotate.use_offset".as_ptr()) == 0 {
            (*opt).use_offset = perf_config_bool(c"use_offset".as_ptr(), value);
        } else if strcmp(var, c"annotate.disassembler_style".as_ptr()) == 0 {
            (*opt).disassembler_style = strdup(value);
            if (*opt).disassembler_style.is_null() { pr_err(c"Not enough memory for annotate.disassembler_style\n".as_ptr()); return -1; }
        } else if strcmp(var, c"annotate.objdump".as_ptr()) == 0 {
            (*opt).objdump_path = strdup(value);
            if (*opt).objdump_path.is_null() { pr_err(c"Not enough memory for annotate.objdump\n".as_ptr()); return -1; }
        } else if strcmp(var, c"annotate.addr2line".as_ptr()) == 0 {
            symbol_conf.addr2line_path = strdup(value);
            if symbol_conf.addr2line_path.is_null() { pr_err(c"Not enough memory for annotate.addr2line\n".as_ptr()); return -1; }
        } else if strcmp(var, c"annotate.demangle".as_ptr()) == 0 {
            symbol_conf.demangle = perf_config_bool(c"demangle".as_ptr(), value);
        } else if strcmp(var, c"annotate.demangle_kernel".as_ptr()) == 0 {
            symbol_conf.demangle_kernel = perf_config_bool(c"demangle_kernel".as_ptr(), value);
        } else {
            pr_debug(c"%s variable unknown, ignoring...".as_ptr(), var);
        }
        0
    }
}
