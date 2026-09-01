/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Convert sample address to data type using DWARF debug info.
 *
 * Written by Namhyung Kim <namhyung@kernel.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_double, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

type u8 = u8;
type u32 = u32;
type u64 = u64;
type s64 = i64;
type size_t = usize;
type ptrdiff_t = isize;
type uintptr_t = usize;
type Dwarf_Word = c_ulong;
type Dwarf_Addr = u64;
type Dwarf_Off = u64;

const X86_REG_SP: c_int = 7;
const DWARF_REG_PC: c_int = -1;
const DWARF_REG_FB: c_int = -2;
const DW_AT_location: c_uint = 0;
const DW_AT_data_member_location: c_uint = 0;
const DW_AT_data_bit_offset: c_uint = 0;
const DW_AT_bit_size: c_uint = 0;
const DW_AT_frame_base: c_uint = 0;
const DW_TAG_member: c_int = 0;
const DW_TAG_typedef: c_int = 0;
const DW_TAG_structure_type: c_int = 0;
const DW_TAG_union_type: c_int = 0;
const DW_TAG_subprogram: c_int = 0;
const DW_TAG_inlined_subroutine: c_int = 0;
const DW_TAG_lexical_block: c_int = 0;
const DW_OP_reg0: c_uint = 0;
const DW_OP_reg31: c_uint = 31;
const DW_OP_breg0: c_uint = 0;
const DW_OP_breg31: c_uint = 31;
const DW_OP_regx: c_uint = 0;
const DW_OP_bregx: c_uint = 0;
const DW_OP_fbreg: c_uint = 0;
const DW_OP_addr: c_uint = 0;
const DW_OP_call_frame_cfa: c_uint = 0;
const DIE_FIND_CB_SIBLING: c_int = 0;
const DIE_FIND_CB_END: c_int = 1;
const INSN_SEG_X86_GS: c_int = 0;
const ENOMEM: c_int = 12;

#[repr(C)] pub struct rb_node { _priv: [u8; 0] }
#[repr(C)] pub struct rb_root { _priv: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct Dwarf { _priv: [u8; 0] }
#[repr(C)] #[derive(Copy, Clone)] pub struct Dwarf_Die { pub cu: *mut c_void, pub addr: *mut c_void }
#[repr(C)] pub struct Dwarf_Attribute { _priv: [u8; 0] }
#[repr(C)] pub struct Dwarf_Op { pub atom: c_uint, pub number: Dwarf_Word, pub number2: Dwarf_Word }
#[repr(C)] pub struct Dwarf_Block { pub length: size_t, pub data: *mut u8 }
#[repr(C)] pub struct strbuf { pub buf: *mut c_char }
#[repr(C)] pub struct arch { pub update_insn_state: Option<unsafe extern "C" fn(*mut type_state, *mut data_loc_info, *mut Dwarf_Die, *mut disasm_line)> }
#[repr(C)] pub struct debuginfo { pub dbg: *mut Dwarf }
#[repr(C)] pub struct map { _priv: [u8; 0] }
#[repr(C)] pub struct dso { _priv: [u8; 0] }
#[repr(C)] pub struct symbol { pub start: u64, pub name: *const c_char }
#[repr(C)] pub struct map_symbol { pub map: *mut map, pub sym: *mut symbol }
#[repr(C)] pub struct thread { _priv: [u8; 0] }
#[repr(C)] pub struct evlist { _priv: [u8; 0] }
#[repr(C)] pub struct evsel_core { pub idx: c_int }
#[repr(C)] pub struct evsel { pub evlist: *mut evlist, pub core: evsel_core, pub name: *const c_char }
#[repr(C)] pub struct hists { pub stats: hists_stats }
#[repr(C)] pub struct hists_stats { pub nr_samples: u64 }
#[repr(C)] pub struct addr_location { pub map: *mut map }
#[repr(C)] pub struct annotation { pub src: *mut annotation_source }
#[repr(C)] pub struct annotation_source { pub source: list_head }
#[repr(C)] pub struct disasm_line { pub al: disasm_line_al }
#[repr(C)] pub struct disasm_line_al { pub offset: u64, pub node: list_head }
#[repr(C)] pub struct annotated_basic_block { pub list: list_head, pub begin: *mut disasm_line, pub end: *mut disasm_line }
#[repr(C)] pub struct annotated_op_loc { pub reg1: c_int, pub reg2: c_int, pub offset: c_int, pub segment: c_int, pub imm: bool, pub multi_regs: bool }
#[repr(C)] pub struct data_loc_info {
    pub arch: *const arch, pub di: *mut debuginfo, pub ms: *mut map_symbol, pub thread: *mut thread,
    pub cpumode: c_int, pub ip: u64, pub var_addr: u64, pub op: *mut annotated_op_loc,
    pub type_offset: c_int, pub fbreg: c_int, pub fb_cfa: bool,
}
#[repr(C)] pub struct die_var_type {
    pub next: *mut die_var_type, pub die_off: Dwarf_Off, pub addr: u64, pub end: u64,
    pub has_range: bool, pub reg: c_int, pub offset: c_int, pub is_reg_var_addr: bool,
}
#[repr(C)] #[derive(Copy, Clone)] pub struct type_state_reg {
    pub type_: Dwarf_Die, pub ok: bool, pub caller_saved: bool, pub kind: u8, pub offset: c_int,
    pub copied_from: c_int, pub imm_value: u64, pub lifetime_active: bool, pub lifetime_end: u64,
}
#[repr(C)] pub struct type_state_stack {
    pub list: list_head, pub type_: Dwarf_Die, pub size: Dwarf_Word, pub offset: c_int,
    pub ptr_offset: c_int, pub kind: u8, pub compound: bool,
}
#[repr(C)] pub struct type_state { pub regs: [type_state_reg; 32], pub stack_vars: list_head, pub ret_reg: c_int, pub stack_reg: c_int }
#[repr(C)] pub struct annotated_member {
    pub node: list_head, pub children: list_head, pub type_name: *mut c_char, pub var_name: *mut c_char,
    pub size: Dwarf_Word, pub offset: Dwarf_Word,
}
#[repr(C)] pub struct type_hist_entry { pub nr_samples: c_int, pub period: u64 }
#[repr(C)] pub struct type_hist { pub nr_samples: c_int, pub period: u64, pub addr: [type_hist_entry; 0] }
#[repr(C)] pub struct annotated_data_type { pub node: rb_node, pub self_: annotated_member, pub histograms: *mut *mut type_hist, pub nr_histograms: c_int }
#[repr(C)] pub struct hist_entry {
    pub ms: map_symbol, pub stat: hist_entry_stat, pub pairs: hist_entry_pairs, pub mem_type: *mut annotated_data_type,
}
#[repr(C)] pub struct hist_entry_stat { pub nr_events: c_int }
#[repr(C)] pub struct hist_entry_pairs { pub head: list_head }
#[repr(C)] pub struct symbol_conf_t { pub annotate_data_member: bool, pub skip_empty: bool, pub show_total_period: bool, pub show_nr_samples: bool }
#[repr(C)] pub struct ann_data_stat_t { pub no_cuinfo: c_int, pub insn_track: c_int, pub no_typeinfo: c_int, pub invalid_size: c_int, pub bad_offset: c_int, pub no_var: c_int }

const TSR_KIND_INVALID: u8 = 0;
const TSR_KIND_TYPE: u8 = 1;
const TSR_KIND_PERCPU_BASE: u8 = 2;
const TSR_KIND_CONST: u8 = 3;
const TSR_KIND_POINTER: u8 = 4;
const TSR_KIND_PERCPU_POINTER: u8 = 5;
const TSR_KIND_CANARY: u8 = 6;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum type_match_result {
    PERF_TMR_UNKNOWN = 0,
    PERF_TMR_OK,
    PERF_TMR_NO_TYPE,
    PERF_TMR_NO_POINTER,
    PERF_TMR_NO_SIZE,
    PERF_TMR_BAD_OFFSET,
    PERF_TMR_BAIL_OUT,
}

#[repr(C)]
struct global_var_entry { node: rb_node, name: *mut c_char, start: u64, end: u64, die_offset: u64 }

unsafe extern "C" {
    static mut debug_type_profile: bool;
    static mut verbose: c_int;
    static mut symbol_conf: symbol_conf_t;
    static mut ann_data_stat: ann_data_stat_t;
    fn pr_info(fmt: *const c_char, ...);
    fn pr_debug3(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn asprintf(s: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn free(p: *mut c_void);
    fn malloc(sz: size_t) -> *mut c_void;
    fn calloc(n: size_t, sz: size_t) -> *mut c_void;
    fn zalloc(sz: size_t) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strbuf_init(sb: *mut strbuf, hint: size_t);
    fn strbuf_detach(sb: *mut strbuf, sz: *mut size_t) -> *mut c_char;
    fn strbuf_add(sb: *mut strbuf, s: *const c_char, len: size_t);
    fn strbuf_release(sb: *mut strbuf);
    fn die_get_typename_from_type(die: *mut Dwarf_Die, sb: *mut strbuf) -> c_int;
    fn die_get_typename(die: *mut Dwarf_Die, sb: *mut strbuf) -> c_int;
    fn __die_get_real_type(die: *mut Dwarf_Die, type_die: *mut Dwarf_Die) -> *mut Dwarf_Die;
    fn die_get_real_type(die: *mut Dwarf_Die, type_die: *mut Dwarf_Die) -> *mut Dwarf_Die;
    fn die_get_pointer_type(die: *mut Dwarf_Die, type_die: *mut Dwarf_Die) -> *mut Dwarf_Die;
    fn die_get_data_member_location(die: *mut Dwarf_Die, loc: *mut Dwarf_Word) -> c_int;
    fn die_find_child(die: *mut Dwarf_Die, cb: unsafe extern "C" fn(*mut Dwarf_Die, *mut c_void) -> c_int, arg: *mut c_void, die_mem: *mut Dwarf_Die) -> c_int;
    fn die_name(die: *mut Dwarf_Die) -> *const c_char;
    fn die_collect_global_vars(cu: *mut Dwarf_Die, vars: *mut *mut die_var_type);
    fn die_collect_vars(scope: *mut Dwarf_Die, vars: *mut *mut die_var_type);
    fn die_find_variable_by_addr(scope: *mut Dwarf_Die, addr: u64, var_die: *mut Dwarf_Die, type_die: *mut Dwarf_Die, offset: *mut c_int) -> bool;
    fn die_find_variable_at(scope: *mut Dwarf_Die, name: *const c_char, pc: u64, var_die: *mut Dwarf_Die) -> bool;
    fn die_find_variable_by_reg(scope: *mut Dwarf_Die, pc: u64, reg: c_int, type_die: *mut Dwarf_Die, offset: *mut c_int, is_fbreg: bool, var_die: *mut Dwarf_Die) -> bool;
    fn die_get_scopes(cu: *mut Dwarf_Die, pc: u64, scopes: *mut *mut Dwarf_Die) -> c_int;
    fn die_get_cfa(dbg: *mut Dwarf, pc: u64, reg: *mut c_int, off: *mut c_int) -> c_int;
    fn dwarf_aggregate_size(die: *mut Dwarf_Die, size: *mut Dwarf_Word) -> c_int;
    fn dwarf_dieoffset(die: *mut Dwarf_Die) -> Dwarf_Off;
    fn dwarf_tag(die: *mut Dwarf_Die) -> c_int;
    fn dwarf_diename(die: *mut Dwarf_Die) -> *const c_char;
    fn dwarf_attr(die: *mut Dwarf_Die, attr: c_uint, result: *mut Dwarf_Attribute) -> *mut Dwarf_Attribute;
    fn dwarf_attr_integrate(die: *mut Dwarf_Die, attr: c_uint, result: *mut Dwarf_Attribute) -> *mut Dwarf_Attribute;
    fn dwarf_formudata(attr: *mut Dwarf_Attribute, value: *mut Dwarf_Word) -> c_int;
    fn dwarf_formblock(attr: *mut Dwarf_Attribute, block: *mut Dwarf_Block) -> c_int;
    fn dwarf_getlocations(attr: *mut Dwarf_Attribute, off: ptrdiff_t, base: *mut Dwarf_Addr, start: *mut Dwarf_Addr, end: *mut Dwarf_Addr, ops: *mut *mut Dwarf_Op, nops: *mut size_t) -> ptrdiff_t;
    fn dwarf_addrdie(dbg: *mut Dwarf, pc: u64, die: *mut Dwarf_Die) -> *mut Dwarf_Die;
    fn dwarf_nextcu(dbg: *mut Dwarf, off: Dwarf_Off, next: *mut Dwarf_Off, header: *mut size_t, a: *mut c_void, b: *mut c_void, c: *mut c_void) -> c_int;
    fn dwarf_offdie(dbg: *mut Dwarf, off: Dwarf_Off, die: *mut Dwarf_Die) -> *mut Dwarf_Die;
    fn dwarf_haspc(die: *mut Dwarf_Die, pc: u64) -> bool;
    fn dwarf_ranges(die: *mut Dwarf_Die, idx: size_t, base: *mut Dwarf_Addr, start: *mut Dwarf_Addr, end: *mut Dwarf_Addr) -> c_int;
    fn dwarf_hasattr(die: *mut Dwarf_Die, attr: c_uint) -> bool;
    fn rb_find(key: *const c_void, root: *mut rb_root, cmp: unsafe extern "C" fn(*const c_void, *const rb_node) -> c_int) -> *mut rb_node;
    fn rb_add(node: *mut rb_node, root: *mut rb_root, less: unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool);
    fn rb_first(root: *mut rb_root) -> *mut rb_node;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn dso__data_types(dso: *mut dso) -> *mut rb_root;
    fn dso__global_vars(dso: *mut dso) -> *mut rb_root;
    fn dso__kernel(dso: *mut dso) -> bool;
    fn dso__name(dso: *mut dso) -> *const c_char;
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__reloc(map: *mut map) -> u64;
    fn map__unmap_ip(map: *mut map, ip: u64) -> u64;
    fn map__rip_2objdump(map: *mut map, ip: u64) -> u64;
    fn map__objdump_2rip(map: *mut map, ip: u64) -> u64;
    fn thread__find_symbol_fb(thread: *mut thread, cpumode: c_int, addr: u64, al: *mut addr_location) -> *mut symbol;
    fn addr_location__init(al: *mut addr_location);
    fn addr_location__exit(al: *mut addr_location);
    fn symbol__annotation(sym: *mut symbol) -> *mut annotation;
    fn annotate_get_basic_blocks(sym: *mut symbol, src: u64, dst: u64, list: *mut list_head) -> c_int;
    fn arch__is_x86(arch: *const arch) -> bool;
    fn arch__is_powerpc(arch: *const arch) -> bool;
    fn evlist__nr_entries(evlist: *mut evlist) -> c_int;
    fn evsel__is_group_event(evsel: *mut evsel) -> bool;
    fn evsel__hists(evsel: *mut evsel) -> *mut hists;
    fn get_percent_color(percent: c_double) -> *const c_char;
    fn color_fprintf(file: *mut c_void, color: *const c_char, fmt: *const c_char, ...) -> c_int;
    static mut stdout: *mut c_void;
}

macro_rules! c { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
unsafe fn list_init(h: *mut list_head) { (*h).next = h; (*h).prev = h; }
unsafe fn list_empty(h: *const list_head) -> bool { (*h).next == h as *mut list_head }
unsafe fn list_add_tail(n: *mut list_head, h: *mut list_head) { (*n).prev = (*h).prev; (*n).next = h; (*(*h).prev).next = n; (*h).prev = n; }
unsafe fn list_add(n: *mut list_head, h: *mut list_head) { (*n).next = (*h).next; (*n).prev = h; (*(*h).next).prev = n; (*h).next = n; }
unsafe fn list_del(e: *mut list_head) { (*(*e).next).prev = (*e).prev; (*(*e).prev).next = (*e).next; }
unsafe fn zfree<T>(p: *mut *mut T) { if !(*p).is_null() { free(*p as *mut c_void); *p = null_mut(); } }
unsafe fn RB_EMPTY_ROOT(root: *mut rb_root) -> bool { rb_first(root).is_null() }
unsafe fn container_of<T>(p: *mut c_void) -> *mut T { p as *mut T }

unsafe fn pr_debug_dtp0(fmt: *const c_char) { if debug_type_profile { pr_info(fmt); } else { pr_debug3(fmt); } }

#[no_mangle]
pub unsafe extern "C" fn pr_debug_type_name(die: *mut Dwarf_Die, kind: u8) {
    let mut sb: strbuf = zeroed();
    let mut size: Dwarf_Word = 0;
    if !debug_type_profile && verbose < 3 { return; }
    match kind {
        TSR_KIND_INVALID => { pr_info(c!("\n")); return; }
        TSR_KIND_PERCPU_BASE => { pr_info(c!(" percpu base\n")); return; }
        TSR_KIND_CONST => { pr_info(c!(" constant\n")); return; }
        TSR_KIND_PERCPU_POINTER => pr_info(c!(" percpu pointer")),
        TSR_KIND_POINTER => pr_info(c!(" pointer")),
        TSR_KIND_CANARY => { pr_info(c!(" stack canary\n")); return; }
        _ => {}
    }
    if dwarf_aggregate_size(die, &mut size) != 0 { size = 0; }
    strbuf_init(&mut sb, 32);
    die_get_typename_from_type(die, &mut sb);
    let str_ = strbuf_detach(&mut sb, null_mut());
    pr_info(c!(" type='%s' size=%#lx (die:%#lx)\n"), str_, size as c_long, dwarf_dieoffset(die) as c_long);
    free(str_ as *mut c_void);
}

unsafe extern "C" fn pr_debug_location(die: *mut Dwarf_Die, pc: u64, reg: c_int) {
    let mut off: ptrdiff_t = 0;
    let mut attr: Dwarf_Attribute = zeroed();
    let (mut base, mut start, mut end) = (0, 0, 0);
    let mut ops: *mut Dwarf_Op = null_mut();
    let mut nops: size_t = 0;
    if !debug_type_profile && verbose < 3 { return; }
    if dwarf_attr(die, DW_AT_location, &mut attr).is_null() { return; }
    loop {
        off = dwarf_getlocations(&mut attr, off, &mut base, &mut start, &mut end, &mut ops, &mut nops);
        if off <= 0 { break; }
        if reg != DWARF_REG_PC && end <= pc { continue; }
        if reg != DWARF_REG_PC && start > pc { break; }
        pr_info(c!(" variable location: "));
        let op = &*ops;
        if op.atom >= DW_OP_reg0 && op.atom <= DW_OP_reg31 {
            pr_info(c!("reg%d\n"), op.atom - DW_OP_reg0);
        } else if op.atom >= DW_OP_breg0 && op.atom <= DW_OP_breg31 {
            pr_info(c!("base=reg%d, offset=%#lx\n"), op.atom - DW_OP_breg0, op.number as c_long);
        } else if op.atom == DW_OP_regx {
            pr_info(c!("reg%ld\n"), op.number as c_long);
        } else if op.atom == DW_OP_bregx {
            pr_info(c!("base=reg%ld, offset=%#lx\n"), op.number as c_long, op.number2 as c_long);
        } else if op.atom == DW_OP_fbreg {
            pr_info(c!("use frame base, offset=%#lx\n"), op.number as c_long);
        } else if op.atom == DW_OP_addr {
            pr_info(c!("address=%#lx\n"), op.number as c_long);
        } else {
            pr_info(c!("unknown: code=%#x, number=%#lx\n"), op.atom, op.number as c_long);
        }
        break;
    }
}

unsafe extern "C" fn pr_debug_scope(scope_die: *mut Dwarf_Die) {
    if !debug_type_profile && verbose < 3 { return; }
    pr_info(c!("(die:%lx) "), dwarf_dieoffset(scope_die) as c_long);
    let tag = dwarf_tag(scope_die);
    if tag == DW_TAG_subprogram { pr_info(c!("[function] %s\n"), die_name(scope_die)); }
    else if tag == DW_TAG_inlined_subroutine { pr_info(c!("[inlined] %s\n"), die_name(scope_die)); }
    else if tag == DW_TAG_lexical_block { pr_info(c!("[block]\n")); }
    else { pr_info(c!("[unknown] tag=%x\n"), tag); }
}

#[no_mangle]
pub unsafe extern "C" fn has_reg_type(state: *mut type_state, reg: c_int) -> bool {
    (reg as c_uint) < (*state).regs.len() as c_uint
}

unsafe extern "C" fn init_type_state(state: *mut type_state, arch: *const arch) {
    memset(state as *mut c_void, 0, size_of::<type_state>());
    list_init(&mut (*state).stack_vars);
    if arch__is_x86(arch) {
        for r in [0usize, 1, 2, 4, 5, 8, 9, 10, 11] { (*state).regs[r].caller_saved = true; }
        (*state).ret_reg = 0;
        (*state).stack_reg = X86_REG_SP;
    }
}

unsafe extern "C" fn exit_type_state(state: *mut type_state) {
    while !list_empty(&(*state).stack_vars) {
        let stack = (*state).stack_vars.next as *mut type_state_stack;
        list_del(&mut (*stack).list);
        free(stack as *mut c_void);
    }
}

unsafe extern "C" fn data_type_cmp(_key: *const c_void, node: *const rb_node) -> c_int {
    let key = _key as *const annotated_data_type;
    let type_ = node as *mut annotated_data_type;
    if (*key).self_.size != (*type_).self_.size { return ((*key).self_.size as c_long - (*type_).self_.size as c_long) as c_int; }
    strcmp((*key).self_.type_name, (*type_).self_.type_name)
}

unsafe extern "C" fn data_type_less(node_a: *mut rb_node, node_b: *const rb_node) -> bool {
    let a = node_a as *mut annotated_data_type;
    let b = node_b as *mut annotated_data_type;
    if (*a).self_.size != (*b).self_.size { return (*a).self_.size < (*b).self_.size; }
    strcmp((*a).self_.type_name, (*b).self_.type_name) < 0
}

/* Recursively add new members for struct/union */
unsafe extern "C" fn __add_member_cb(die: *mut Dwarf_Die, arg: *mut c_void) -> c_int {
    let parent = arg as *mut annotated_member;
    let mut member_type: Dwarf_Die = zeroed();
    let mut die_mem: Dwarf_Die = zeroed();
    let mut size: Dwarf_Word = 0;
    let mut loc: Dwarf_Word = 0;
    let mut bit_size: Dwarf_Word = 0;
    let mut attr: Dwarf_Attribute = zeroed();
    let mut sb: strbuf = zeroed();
    if dwarf_tag(die) != DW_TAG_member { return DIE_FIND_CB_SIBLING; }
    let member = zalloc(size_of::<annotated_member>()) as *mut annotated_member;
    if member.is_null() { return DIE_FIND_CB_END; }
    strbuf_init(&mut sb, 32);
    die_get_typename(die, &mut sb);
    __die_get_real_type(die, &mut member_type);
    if dwarf_tag(&mut member_type) == DW_TAG_typedef { die_get_real_type(&mut member_type, &mut die_mem); } else { die_mem = member_type; }
    if dwarf_aggregate_size(&mut die_mem, &mut size) < 0 { size = 0; }
    if !dwarf_attr_integrate(die, DW_AT_data_member_location, &mut attr).is_null() {
        if dwarf_formudata(&mut attr, &mut loc) != 0 && die_get_data_member_location(die, &mut loc) != 0 { loc = 0; }
    } else {
        if !dwarf_attr_integrate(die, DW_AT_data_bit_offset, &mut attr).is_null() && dwarf_formudata(&mut attr, &mut loc) == 0 { loc /= 8; } else { loc = 0; }
        if !dwarf_attr_integrate(die, DW_AT_bit_size, &mut attr).is_null() && dwarf_formudata(&mut attr, &mut bit_size) == 0 { size = (bit_size + 7) / 8; }
    }
    (*member).type_name = strbuf_detach(&mut sb, null_mut());
    if !dwarf_diename(die).is_null() {
        if bit_size != 0 {
            if asprintf(&mut (*member).var_name, c!("%s:%ld"), dwarf_diename(die), bit_size as c_long) < 0 { (*member).var_name = null_mut(); }
        } else {
            let name = dwarf_diename(die);
            (*member).var_name = if !name.is_null() { strdup(name) } else { null_mut() };
        }
        if (*member).var_name.is_null() { free(member as *mut c_void); return DIE_FIND_CB_END; }
    }
    (*member).size = size;
    (*member).offset = loc + (*parent).offset;
    list_init(&mut (*member).children);
    list_add_tail(&mut (*member).node, &mut (*parent).children);
    let tag = dwarf_tag(&mut die_mem);
    if tag == DW_TAG_structure_type || tag == DW_TAG_union_type { die_find_child(&mut die_mem, __add_member_cb, member as *mut c_void, &mut die_mem); }
    DIE_FIND_CB_SIBLING
}

unsafe extern "C" fn add_member_types(parent: *mut annotated_data_type, type_: *mut Dwarf_Die) {
    let mut die_mem: Dwarf_Die = zeroed();
    die_find_child(type_, __add_member_cb, &mut (*parent).self_ as *mut _ as *mut c_void, &mut die_mem);
}

unsafe extern "C" fn delete_members(member: *mut annotated_member) {
    while !list_empty(&(*member).children) {
        let child = (*member).children.next as *mut annotated_member;
        list_del(&mut (*child).node);
        delete_members(child);
        zfree(&mut (*child).type_name);
        zfree(&mut (*child).var_name);
        free(child as *mut c_void);
    }
}

unsafe extern "C" fn fill_member_name(buf: *mut c_char, sz: size_t, m: *mut annotated_member, offset: c_int, mut first: bool) -> c_int {
    if list_empty(&(*m).children) { return 0; }
    let mut node = (*m).children.next;
    while node != &mut (*m).children {
        let child = node as *mut annotated_member;
        if offset >= (*child).offset as c_int && offset < ((*child).offset + (*child).size) as c_int {
            let len = if !(*child).var_name.is_null() {
                let n = scnprintf(buf, sz, c!("%s%s"), if first { c!("") } else { c!(".") }, (*child).var_name);
                first = false;
                n
            } else { 0 };
            return fill_member_name(buf.add(len as usize), sz - len as usize, child, offset, first) + len;
        }
        node = (*node).next;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn annotated_data_type__get_member_name(adt: *mut annotated_data_type, buf: *mut c_char, sz: size_t, member_offset: c_int) -> c_int {
    fill_member_name(buf, sz, &mut (*adt).self_, member_offset, true)
}

unsafe extern "C" fn dso__findnew_data_type(dso_: *mut dso, type_die: *mut Dwarf_Die) -> *mut annotated_data_type {
    let mut key: annotated_data_type = zeroed();
    let mut sb: strbuf = zeroed();
    let mut size: Dwarf_Word = 0;
    strbuf_init(&mut sb, 32);
    if die_get_typename_from_type(type_die, &mut sb) < 0 { strbuf_add(&mut sb, c!("(unknown type)"), 14); }
    let type_name = strbuf_detach(&mut sb, null_mut());
    if dwarf_tag(type_die) == DW_TAG_typedef { die_get_real_type(type_die, type_die); }
    if dwarf_aggregate_size(type_die, &mut size) != 0 { size = 0; }
    key.self_.type_name = type_name;
    key.self_.size = size;
    let node = rb_find(&key as *const _ as *const c_void, dso__data_types(dso_), data_type_cmp);
    if !node.is_null() { free(type_name as *mut c_void); return node as *mut annotated_data_type; }
    let result = zalloc(size_of::<annotated_data_type>()) as *mut annotated_data_type;
    if result.is_null() { free(type_name as *mut c_void); return null_mut(); }
    (*result).self_.type_name = type_name;
    (*result).self_.size = size;
    list_init(&mut (*result).self_.children);
    if symbol_conf.annotate_data_member { add_member_types(result, type_die); }
    rb_add(&mut (*result).node, dso__data_types(dso_), data_type_less);
    result
}

unsafe extern "C" fn find_cu_die(di: *mut debuginfo, pc: u64, cu_die: *mut Dwarf_Die) -> bool {
    let (mut off, mut next_off, mut header_size) = (0, 0, 0usize);
    if !dwarf_addrdie((*di).dbg, pc, cu_die).is_null() { return true; }
    while dwarf_nextcu((*di).dbg, off, &mut next_off, &mut header_size, null_mut(), null_mut(), null_mut()) == 0 {
        if !dwarf_offdie((*di).dbg, off + header_size as u64, cu_die).is_null() && dwarf_haspc(cu_die, pc) { return true; }
        off = next_off;
    }
    false
}

unsafe extern "C" fn match_result_str(tmr: type_match_result) -> *const c_char {
    match tmr {
        type_match_result::PERF_TMR_OK => c!("Good!"),
        type_match_result::PERF_TMR_NO_TYPE => c!("no type information"),
        type_match_result::PERF_TMR_NO_POINTER => c!("no/void pointer"),
        type_match_result::PERF_TMR_NO_SIZE => c!("type size is unknown"),
        type_match_result::PERF_TMR_BAD_OFFSET => c!("offset bigger than size"),
        _ => c!("invalid state"),
    }
}

unsafe extern "C" fn is_compound_type(type_die: *mut Dwarf_Die) -> bool {
    let tag = dwarf_tag(type_die);
    tag == DW_TAG_structure_type || tag == DW_TAG_union_type
}

unsafe extern "C" fn is_better_type(type_a: *mut Dwarf_Die, type_b: *mut Dwarf_Die) -> bool {
    let (mut size_a, mut size_b) = (0, 0);
    let (mut die_a, mut die_b, mut ptr_a, mut ptr_b): (Dwarf_Die, Dwarf_Die, Dwarf_Die, Dwarf_Die) = (zeroed(), zeroed(), zeroed(), zeroed());
    let ptr_type_a = die_get_pointer_type(type_a, &mut ptr_a);
    let ptr_type_b = die_get_pointer_type(type_b, &mut ptr_b);
    if ptr_type_a.is_null() != ptr_type_b.is_null() { return !ptr_type_b.is_null(); }
    let mut ta = type_a;
    let mut tb = type_b;
    if !ptr_type_b.is_null() {
        if die_get_real_type(ptr_type_a, &mut die_a).is_null() { return true; }
        if die_get_real_type(ptr_type_b, &mut die_b).is_null() { return false; }
        ta = &mut die_a; tb = &mut die_b;
    }
    if dwarf_aggregate_size(ta, &mut size_a) < 0 || dwarf_aggregate_size(tb, &mut size_b) < 0 { return false; }
    if size_a != size_b { return size_a < size_b; }
    if is_compound_type(ta) != is_compound_type(tb) { return is_compound_type(tb); }
    dwarf_tag(type_b) == DW_TAG_typedef
}

unsafe extern "C" fn check_variable(dloc: *mut data_loc_info, var_die: *mut Dwarf_Die, type_die: *mut Dwarf_Die, reg: c_int, offset: c_int, is_fbreg: bool) -> type_match_result {
    let mut size = 0;
    let mut sized_type: Dwarf_Die = zeroed();
    let mut needs_pointer = true;
    if reg == DWARF_REG_PC || reg == (*dloc).fbreg || is_fbreg || (arch__is_x86((*dloc).arch) && reg == X86_REG_SP) { needs_pointer = false; }
    if __die_get_real_type(var_die, type_die).is_null() { return type_match_result::PERF_TMR_NO_TYPE; }
    if needs_pointer && (die_get_pointer_type(type_die, type_die).is_null() || __die_get_real_type(type_die, type_die).is_null()) { return type_match_result::PERF_TMR_NO_POINTER; }
    if dwarf_tag(type_die) == DW_TAG_typedef { die_get_real_type(type_die, &mut sized_type); } else { sized_type = *type_die; }
    if dwarf_aggregate_size(&mut sized_type, &mut size) < 0 { return type_match_result::PERF_TMR_NO_SIZE; }
    if (offset as c_uint) >= size as c_uint { return type_match_result::PERF_TMR_BAD_OFFSET; }
    type_match_result::PERF_TMR_OK
}

#[no_mangle]
pub unsafe extern "C" fn find_stack_state(state: *mut type_state, offset: c_int) -> *mut type_state_stack {
    let mut node = (*state).stack_vars.next;
    while node != &mut (*state).stack_vars {
        let stack = node as *mut type_state_stack;
        if offset == (*stack).offset { return stack; }
        if (*stack).compound && (*stack).offset < offset && offset < (*stack).offset + (*stack).size as c_int { return stack; }
        node = (*node).next;
    }
    null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn set_stack_state(stack: *mut type_state_stack, offset: c_int, kind: u8, type_die: *mut Dwarf_Die, ptr_offset: c_int) {
    let mut size: Dwarf_Word = 0;
    if kind == TSR_KIND_POINTER { size = size_of::<*mut c_void>() as Dwarf_Word; }
    else if dwarf_aggregate_size(type_die, &mut size) < 0 { size = 0; }
    (*stack).type_ = *type_die; (*stack).size = size; (*stack).offset = offset; (*stack).ptr_offset = ptr_offset; (*stack).kind = kind;
    if kind == TSR_KIND_POINTER { (*stack).compound = false; return; }
    let tag = dwarf_tag(type_die);
    (*stack).compound = (tag == DW_TAG_structure_type || tag == DW_TAG_union_type) && kind != TSR_KIND_PERCPU_POINTER;
}

#[no_mangle]
pub unsafe extern "C" fn findnew_stack_state(state: *mut type_state, offset: c_int, kind: u8, type_die: *mut Dwarf_Die, ptr_offset: c_int) -> *mut type_state_stack {
    let stack = find_stack_state(state, offset);
    if !stack.is_null() { set_stack_state(stack, offset, kind, type_die, ptr_offset); return stack; }
    let stack = malloc(size_of::<type_state_stack>()) as *mut type_state_stack;
    if !stack.is_null() { set_stack_state(stack, offset, kind, type_die, ptr_offset); list_add(&mut (*stack).list, &mut (*state).stack_vars); }
    stack
}

unsafe extern "C" fn global_var_cmp(_key: *const c_void, node: *const rb_node) -> c_int {
    let addr = _key as uintptr_t as u64;
    let gvar = node as *mut global_var_entry;
    if (*gvar).start <= addr && addr < (*gvar).end { return 0; }
    if (*gvar).start > addr { -1 } else { 1 }
}

unsafe extern "C" fn global_var_less(node_a: *mut rb_node, node_b: *const rb_node) -> bool {
    (*(node_a as *mut global_var_entry)).start < (*(node_b as *mut global_var_entry)).start
}

unsafe extern "C" fn global_var__find(dloc: *mut data_loc_info, addr: u64) -> *mut global_var_entry {
    let dso_ = map__dso((*(*dloc).ms).map);
    let node = rb_find(addr as uintptr_t as *const c_void, dso__global_vars(dso_), global_var_cmp);
    if node.is_null() { null_mut() } else { node as *mut global_var_entry }
}

unsafe extern "C" fn global_var__add(dloc: *mut data_loc_info, addr: u64, name: *const c_char, type_die: *mut Dwarf_Die) -> bool {
    let dso_ = map__dso((*(*dloc).ms).map);
    let mut size = 0;
    if dwarf_aggregate_size(type_die, &mut size) < 0 { return false; }
    let gvar = malloc(size_of::<global_var_entry>()) as *mut global_var_entry;
    if gvar.is_null() { return false; }
    (*gvar).name = if !name.is_null() { strdup(name) } else { null_mut() };
    if !name.is_null() && (*gvar).name.is_null() { free(gvar as *mut c_void); return false; }
    (*gvar).start = addr; (*gvar).end = addr + size; (*gvar).die_offset = dwarf_dieoffset(type_die);
    rb_add(&mut (*gvar).node, dso__global_vars(dso_), global_var_less);
    true
}

#[no_mangle]
pub unsafe extern "C" fn global_var_type__tree_delete(root: *mut rb_root) {
    while !RB_EMPTY_ROOT(root) {
        let node = rb_first(root);
        rb_erase(node, root);
        let gvar = node as *mut global_var_entry;
        zfree(&mut (*gvar).name);
        free(gvar as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_global_var_info(dloc: *mut data_loc_info, addr: u64, var_name: *mut *const c_char, var_offset: *mut c_int) -> bool {
    let mut al: addr_location = zeroed();
    let mem_addr = addr + map__reloc((*(*dloc).ms).map);
    addr_location__init(&mut al);
    let sym = thread__find_symbol_fb((*dloc).thread, (*dloc).cpumode, mem_addr, &mut al);
    if !sym.is_null() {
        *var_name = (*sym).name;
        *var_offset = (mem_addr - map__unmap_ip(al.map, (*sym).start)) as c_int;
    } else { *var_name = null(); }
    addr_location__exit(&mut al);
    !(*var_name).is_null()
}

unsafe extern "C" fn global_var__collect(dloc: *mut data_loc_info) {
    let dwarf = (*(*dloc).di).dbg;
    let (mut off, mut next_off, mut header_size) = (0, 0, 0usize);
    let (mut cu_die, mut type_die): (Dwarf_Die, Dwarf_Die) = (zeroed(), zeroed());
    while dwarf_nextcu(dwarf, off, &mut next_off, &mut header_size, null_mut(), null_mut(), null_mut()) == 0 {
        let mut var_types: *mut die_var_type = null_mut();
        if dwarf_offdie(dwarf, off + header_size as u64, &mut cu_die).is_null() { off = next_off; continue; }
        die_collect_global_vars(&mut cu_die, &mut var_types);
        let mut pos = var_types;
        while !pos.is_null() {
            let mut var_name: *const c_char = null();
            let mut var_offset = 0;
            if (*pos).reg == -1 && !dwarf_offdie(dwarf, (*pos).die_off, &mut type_die).is_null() {
                get_global_var_info(dloc, (*pos).addr, &mut var_name, &mut var_offset);
                global_var__add(dloc, (*pos).addr, var_name, &mut type_die);
            }
            pos = (*pos).next;
        }
        delete_var_types(var_types);
        off = next_off;
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_global_var_type(cu_die: *mut Dwarf_Die, dloc: *mut data_loc_info, ip: u64, var_addr: u64, var_offset: *mut c_int, type_die: *mut Dwarf_Die) -> bool {
    let dso_ = map__dso((*(*dloc).ms).map);
    let mut var_name: *const c_char = null();
    let mut var_die: Dwarf_Die = zeroed();
    if RB_EMPTY_ROOT(dso__global_vars(dso_)) { global_var__collect(dloc); }
    let gvar = global_var__find(dloc, var_addr);
    if !gvar.is_null() {
        if dwarf_offdie((*(*dloc).di).dbg, (*gvar).die_offset, type_die).is_null() { return false; }
        *var_offset = (var_addr - (*gvar).start) as c_int;
        return true;
    }
    if die_find_variable_by_addr(cu_die, var_addr, &mut var_die, type_die, var_offset) {
        var_name = dwarf_diename(&mut var_die);
    } else if !get_global_var_info(dloc, var_addr, &mut var_name, var_offset) { return false; }
    else {
        let pc = map__rip_2objdump((*(*dloc).ms).map, ip);
        if !(die_find_variable_at(cu_die, var_name, pc, &mut var_die) &&
             check_variable(dloc, &mut var_die, type_die, DWARF_REG_PC, *var_offset, false) == type_match_result::PERF_TMR_OK) { return false; }
    }
    global_var__add(dloc, var_addr - *var_offset as u64, var_name, type_die);
    true
}

unsafe extern "C" fn die_is_same(a: *mut Dwarf_Die, b: *mut Dwarf_Die) -> bool { (*a).cu == (*b).cu && (*a).addr == (*b).addr }

unsafe extern "C" fn tsr_set_lifetime(tsr: *mut type_state_reg, var: *const die_var_type) {
    if !var.is_null() && (*var).has_range && (*var).end > (*var).addr {
        (*tsr).lifetime_active = true; (*tsr).lifetime_end = (*var).end;
    } else { (*tsr).lifetime_active = false; (*tsr).lifetime_end = 0; }
}

unsafe extern "C" fn update_var_state(state: *mut type_state, dloc: *mut data_loc_info, addr: u64, _insn_offset: u64, var_types: *mut die_var_type) {
    let mut fbreg = (*dloc).fbreg;
    let mut fb_offset = 0;
    if (*dloc).fb_cfa && die_get_cfa((*(*dloc).di).dbg, addr, &mut fbreg, &mut fb_offset) < 0 { fbreg = -1; }
    let mut var = var_types;
    while !var.is_null() {
        if ((*var).has_range && (addr < (*var).addr || ((*var).end != 0 && addr >= (*var).end))) ||
           (!(*var).has_range && addr != (*var).addr) { var = (*var).next; continue; }
        let mut mem_die: Dwarf_Die = zeroed();
        if dwarf_offdie((*(*dloc).di).dbg, (*var).die_off, &mut mem_die).is_null() { var = (*var).next; continue; }
        if (*var).reg == DWARF_REG_FB || (*var).reg == fbreg || (*var).reg == (*state).stack_reg {
            let mut ptr_die: Dwarf_Die = zeroed();
            let ptr_type = die_get_pointer_type(&mut mem_die, &mut ptr_die);
            let mut offset = (*var).offset;
            if !(*var).is_reg_var_addr && !ptr_type.is_null() && __die_get_real_type(ptr_type, &mut mem_die).is_null() { var = (*var).next; continue; }
            if (*var).reg != DWARF_REG_FB { offset -= fb_offset; }
            let stack = find_stack_state(state, offset);
            if !( !stack.is_null() && (*stack).kind == TSR_KIND_TYPE && !is_better_type(&mut (*stack).type_, &mut mem_die)) {
                findnew_stack_state(state, offset, TSR_KIND_TYPE, &mut mem_die, 0);
            }
        } else if has_reg_type(state, (*var).reg) {
            let regp = &mut (*state).regs[(*var).reg as usize] as *mut type_state_reg;
            if (*regp).ok && (*regp).kind == TSR_KIND_TYPE && (!is_better_type(&mut (*regp).type_, &mut mem_die) || (*var).is_reg_var_addr) { var = (*var).next; continue; }
            if (*var).is_reg_var_addr {
                if (*regp).ok && (*regp).kind == TSR_KIND_POINTER && !is_better_type(&mut (*regp).type_, &mut mem_die) { var = (*var).next; continue; }
                (*regp).offset = -(*var).offset; (*regp).type_ = mem_die; (*regp).kind = TSR_KIND_POINTER; (*regp).ok = true; tsr_set_lifetime(regp, var);
            } else {
                let mut orig_type = (*regp).type_;
                (*regp).offset = -(*var).offset; (*regp).type_ = mem_die; (*regp).kind = TSR_KIND_TYPE; (*regp).ok = true; tsr_set_lifetime(regp, var);
                if has_reg_type(state, (*regp).copied_from) {
                    let copy = &mut (*state).regs[(*regp).copied_from as usize] as *mut type_state_reg;
                    if (*copy).ok && (*copy).kind == TSR_KIND_TYPE && die_is_same(&mut (*copy).type_, &mut orig_type) && is_better_type(&mut (*copy).type_, &mut mem_die) {
                        (*copy).type_ = mem_die;
                    }
                }
            }
        }
        var = (*var).next;
    }
}

unsafe extern "C" fn update_insn_state(state: *mut type_state, dloc: *mut data_loc_info, cu_die: *mut Dwarf_Die, dl: *mut disasm_line) {
    if let Some(f) = (*(*dloc).arch).update_insn_state { f(state, dloc, cu_die, dl); }
}

unsafe extern "C" fn prepend_basic_blocks(this_blocks: *mut list_head, full_blocks: *mut list_head) {
    /* Source-level translation note: list_last_entry/list_first_entry/list_splice are kernel list helpers. */
    if list_empty(full_blocks) {
        if !list_empty(this_blocks) {
            (*(*this_blocks).prev).next = (*full_blocks).next;
            (*(*full_blocks).next).prev = (*this_blocks).prev;
            (*(*this_blocks).next).prev = full_blocks;
            (*full_blocks).next = (*this_blocks).next;
        }
        return;
    }
    if !list_empty(this_blocks) {
        (*(*this_blocks).prev).next = (*full_blocks).next;
        (*(*full_blocks).next).prev = (*this_blocks).prev;
        (*(*this_blocks).next).prev = full_blocks;
        (*full_blocks).next = (*this_blocks).next;
    }
}

unsafe extern "C" fn delete_basic_blocks(basic_blocks: *mut list_head) {
    while !list_empty(basic_blocks) {
        let bb = (*basic_blocks).next as *mut annotated_basic_block;
        list_del(&mut (*bb).list);
        free(bb as *mut c_void);
    }
}

unsafe extern "C" fn fixup_var_address(mut var_types: *mut die_var_type, addr: u64) {
    while !var_types.is_null() {
        if (*var_types).addr == 0 { (*var_types).addr = addr; }
        var_types = (*var_types).next;
    }
}

unsafe extern "C" fn delete_var_types(mut var_types: *mut die_var_type) {
    while !var_types.is_null() {
        let next = (*var_types).next;
        free(var_types as *mut c_void);
        var_types = next;
    }
}

unsafe extern "C" fn setup_stack_canary(dloc: *mut data_loc_info) {
    if arch__is_x86((*dloc).arch) {
        (*(*dloc).op).segment = INSN_SEG_X86_GS;
        (*(*dloc).op).imm = true;
        (*(*dloc).op).offset = 40;
    }
}

unsafe extern "C" fn check_matching_type(state: *mut type_state, dloc: *mut data_loc_info, cu_die: *mut Dwarf_Die, dl: *mut disasm_line, type_die: *mut Dwarf_Die) -> type_match_result {
    let reg = (*(*dloc).op).reg1 as usize;
    let mut retry = true;
    loop {
    if !(*state).regs[reg].ok { /* fall through */ } else if (*state).regs[reg].kind == TSR_KIND_TYPE {
        let mut ptr_die: Dwarf_Die = zeroed();
        let ptr_type = die_get_pointer_type(&mut (*state).regs[reg].type_, &mut ptr_die);
        if ptr_type.is_null() {
            if (*(*dloc).op).offset < 0 && reg as c_int != (*state).stack_reg { /* kernel path below */ } else { return type_match_result::PERF_TMR_NO_POINTER; }
        } else {
            let mut size = 0;
            let mut sized_type: Dwarf_Die = zeroed();
            if __die_get_real_type(ptr_type, type_die).is_null() { return type_match_result::PERF_TMR_NO_POINTER; }
            (*dloc).type_offset = (*(*dloc).op).offset + (*state).regs[reg].offset;
            if dwarf_tag(type_die) == DW_TAG_typedef { die_get_real_type(type_die, &mut sized_type); } else { sized_type = *type_die; }
            if dwarf_aggregate_size(&mut sized_type, &mut size) < 0 || ((*dloc).type_offset as c_uint) >= size as c_uint { return type_match_result::PERF_TMR_BAD_OFFSET; }
            return type_match_result::PERF_TMR_OK;
        }
    } else if (*state).regs[reg].kind == TSR_KIND_POINTER || (*state).regs[reg].kind == TSR_KIND_PERCPU_POINTER {
        let mut size = 0;
        *type_die = (*state).regs[reg].type_;
        (*dloc).type_offset = if (*state).regs[reg].kind == TSR_KIND_PERCPU_POINTER { (*(*dloc).op).offset } else { (*(*dloc).op).offset + (*state).regs[reg].offset };
        if dwarf_aggregate_size(type_die, &mut size) < 0 || ((*dloc).type_offset as c_uint) >= size as c_uint {
            return if (*state).regs[reg].kind == TSR_KIND_PERCPU_POINTER { type_match_result::PERF_TMR_BAIL_OUT } else { type_match_result::PERF_TMR_BAD_OFFSET };
        }
        return type_match_result::PERF_TMR_OK;
    } else if (*state).regs[reg].kind == TSR_KIND_CANARY {
        setup_stack_canary(dloc);
        return type_match_result::PERF_TMR_BAIL_OUT;
    } else if (*state).regs[reg].kind == TSR_KIND_PERCPU_BASE {
        let mut var_addr = (*(*dloc).op).offset as u64;
        let mut var_offset = 0;
        if (*(*dloc).op).multi_regs {
            let mut reg2 = (*(*dloc).op).reg2;
            if reg2 as usize == reg { reg2 = (*(*dloc).op).reg1; }
            if has_reg_type(state, reg2) && (*state).regs[reg2 as usize].ok && (*state).regs[reg2 as usize].kind == TSR_KIND_CONST { var_addr += (*state).regs[reg2 as usize].imm_value; }
        }
        if get_global_var_type(cu_die, dloc, (*dloc).ip, var_addr, &mut var_offset, type_die) { (*dloc).type_offset = var_offset; return type_match_result::PERF_TMR_OK; }
        return type_match_result::PERF_TMR_BAIL_OUT;
    }
    if reg as c_int == (*dloc).fbreg || reg as c_int == (*state).stack_reg {
        let stack = find_stack_state(state, (*dloc).type_offset);
        if stack.is_null() {
            if retry { retry = false; update_insn_state(state, dloc, cu_die, dl); continue; }
            return type_match_result::PERF_TMR_NO_TYPE;
        }
        if (*stack).kind == TSR_KIND_CANARY { setup_stack_canary(dloc); return type_match_result::PERF_TMR_BAIL_OUT; }
        if (*stack).kind != TSR_KIND_TYPE { return type_match_result::PERF_TMR_NO_TYPE; }
        *type_die = (*stack).type_;
        (*dloc).type_offset -= (*stack).offset;
        return type_match_result::PERF_TMR_OK;
    }
    if dso__kernel(map__dso((*(*dloc).ms).map)) {
        let mut offset = 0;
        if (*(*dloc).op).segment == INSN_SEG_X86_GS && (*(*dloc).op).imm && arch__is_x86((*dloc).arch) {
            if get_global_var_type(cu_die, dloc, (*dloc).ip, (*(*dloc).op).offset as u64, &mut offset, type_die) { (*dloc).type_offset = offset; return type_match_result::PERF_TMR_OK; }
            return type_match_result::PERF_TMR_BAIL_OUT;
        }
        if (*(*dloc).op).offset < 0 && reg as c_int != (*state).stack_reg {
            if get_global_var_type(cu_die, dloc, (*dloc).ip, (*(*dloc).op).offset as s64 as u64, &mut offset, type_die) { (*dloc).type_offset = offset; return type_match_result::PERF_TMR_OK; }
            return type_match_result::PERF_TMR_BAIL_OUT;
        }
    }
    return type_match_result::PERF_TMR_UNKNOWN;
    }
}

unsafe extern "C" fn find_data_type_insn(dloc: *mut data_loc_info, basic_blocks: *mut list_head, var_types: *mut die_var_type, cu_die: *mut Dwarf_Die, type_die: *mut Dwarf_Die) -> type_match_result {
    let mut state: type_state = zeroed();
    let mut ret = type_match_result::PERF_TMR_UNKNOWN;
    init_type_state(&mut state, (*dloc).arch);
    let mut bb_node = (*basic_blocks).next;
    while bb_node != basic_blocks {
        let bb = bb_node as *mut annotated_basic_block;
        let mut dl = (*bb).begin;
        while !dl.is_null() {
            let sym = (*(*dloc).ms).sym;
            let this_ip = (*sym).start + (*dl).al.offset;
            let addr = map__rip_2objdump((*(*dloc).ms).map, this_ip);
            if (*dl).al.offset != u64::MAX {
                update_var_state(&mut state, dloc, addr, (*dl).al.offset, var_types);
                if this_ip == (*dloc).ip { ret = check_matching_type(&mut state, dloc, cu_die, dl, type_die); break; }
                update_insn_state(&mut state, dloc, cu_die, dl);
                if dl == (*bb).end { break; }
            }
            dl = (*dl).al.node.next as *mut disasm_line;
        }
        if ret != type_match_result::PERF_TMR_UNKNOWN { break; }
        bb_node = (*bb_node).next;
    }
    exit_type_state(&mut state);
    ret
}

unsafe extern "C" fn arch_supports_insn_tracking(dloc: *mut data_loc_info) -> c_int {
    if arch__is_x86((*dloc).arch) || arch__is_powerpc((*dloc).arch) { 1 } else { 0 }
}

unsafe extern "C" fn find_data_type_block(dloc: *mut data_loc_info, cu_die: *mut Dwarf_Die, scopes: *mut Dwarf_Die, nr_scopes: c_int, type_die: *mut Dwarf_Die) -> type_match_result {
    let mut basic_blocks: list_head = zeroed();
    let mut var_types: *mut die_var_type = null_mut();
    let mut ret = type_match_result::PERF_TMR_UNKNOWN;
    if arch_supports_insn_tracking(dloc) == 0 { return type_match_result::PERF_TMR_BAIL_OUT; }
    list_init(&mut basic_blocks);
    let mut dst_ip = (*dloc).ip;
    let mut i = nr_scopes - 1;
    while i >= 0 {
        let (mut base, mut start, mut end) = (0, 0, 0);
        let mut this_blocks: list_head = zeroed();
        list_init(&mut this_blocks);
        if dwarf_ranges(scopes.add(i as usize), 0, &mut base, &mut start, &mut end) < 0 { break; }
        let src_ip = map__objdump_2rip((*(*dloc).ms).map, start);
        if annotate_get_basic_blocks((*(*dloc).ms).sym, src_ip, dst_ip, &mut this_blocks) >= 0 {
            prepend_basic_blocks(&mut this_blocks, &mut basic_blocks);
            die_collect_vars(scopes.add(i as usize), &mut var_types);
            fixup_var_address(var_types, start);
            ret = find_data_type_insn(dloc, &mut basic_blocks, var_types, cu_die, type_die);
            if ret == type_match_result::PERF_TMR_OK || ret == type_match_result::PERF_TMR_BAIL_OUT { break; }
            dst_ip = src_ip;
        }
        i -= 1;
    }
    delete_basic_blocks(&mut basic_blocks);
    delete_var_types(var_types);
    ret
}

unsafe extern "C" fn find_data_type_die(dloc: *mut data_loc_info, type_die: *mut Dwarf_Die) -> c_int {
    let loc = (*dloc).op;
    let (mut cu_die, mut var_die): (Dwarf_Die, Dwarf_Die) = (zeroed(), zeroed());
    let mut scopes: *mut Dwarf_Die = null_mut();
    let mut ret = -1;
    let mut fbreg = -1;
    let mut fb_offset = 0;
    let mut found = false;
    let mut result = type_match_result::PERF_TMR_UNKNOWN;
    let pc = map__rip_2objdump((*(*dloc).ms).map, (*dloc).ip);
    if !find_cu_die((*dloc).di, pc, &mut cu_die) { ann_data_stat.no_cuinfo += 1; return -1; }
    let mut reg = (*loc).reg1;
    let mut offset = (*loc).offset;
    if reg == DWARF_REG_PC && get_global_var_type(&mut cu_die, dloc, (*dloc).ip, (*dloc).var_addr, &mut offset, type_die) {
        (*dloc).type_offset = offset; found = true;
    }
    let nr_scopes = die_get_scopes(&mut cu_die, pc, &mut scopes);
    if !found && reg != DWARF_REG_PC && nr_scopes > 0 && dwarf_hasattr(scopes, DW_AT_frame_base) {
        let mut attr: Dwarf_Attribute = zeroed();
        let mut block: Dwarf_Block = zeroed();
        if !dwarf_attr(scopes, DW_AT_frame_base, &mut attr).is_null() && dwarf_formblock(&mut attr, &mut block) == 0 && block.length == 1 {
            let op = *block.data;
            if op as c_uint >= DW_OP_reg0 && op as c_uint <= DW_OP_reg31 { fbreg = op as c_int - DW_OP_reg0 as c_int; (*dloc).fbreg = fbreg; }
            else if op as c_uint == DW_OP_call_frame_cfa { (*dloc).fb_cfa = true; if die_get_cfa((*(*dloc).di).dbg, pc, &mut fbreg, &mut fb_offset) < 0 { fbreg = -1; } }
        }
    }
    if !found {
        let mut tried_second = false;
        loop {
            let is_fbreg = reg == fbreg;
            if is_fbreg { offset = (*loc).offset - fb_offset; }
            let mut i = nr_scopes - 1;
            while i >= 0 {
                let mut mem_die: Dwarf_Die = zeroed();
                let mut type_offset = offset;
                let ok = if reg == DWARF_REG_PC {
                    die_find_variable_by_addr(scopes.add(i as usize), (*dloc).var_addr, &mut var_die, &mut mem_die, &mut type_offset)
                } else {
                    die_find_variable_by_reg(scopes.add(i as usize), pc, reg, &mut mem_die, &mut type_offset, is_fbreg, &mut var_die)
                };
                if ok && (!found || (*dloc).type_offset < type_offset || ((*dloc).type_offset == type_offset && !is_better_type(&mut mem_die, type_die))) {
                    *type_die = mem_die; (*dloc).type_offset = type_offset; found = true;
                }
                i -= 1;
            }
            if found || !(*loc).multi_regs || tried_second || reg != (*loc).reg1 || (*loc).reg1 == (*loc).reg2 { break; }
            reg = (*loc).reg2; tried_second = true;
        }
    }
    if !found && reg != DWARF_REG_PC {
        result = find_data_type_block(dloc, &mut cu_die, scopes, nr_scopes, type_die);
        if result == type_match_result::PERF_TMR_OK { ann_data_stat.insn_track += 1; found = true; }
    }
    if found { ret = 0; } else {
        match result {
            type_match_result::PERF_TMR_NO_TYPE | type_match_result::PERF_TMR_NO_POINTER => ann_data_stat.no_typeinfo += 1,
            type_match_result::PERF_TMR_NO_SIZE => ann_data_stat.invalid_size += 1,
            type_match_result::PERF_TMR_BAD_OFFSET => ann_data_stat.bad_offset += 1,
            _ => ann_data_stat.no_var += 1,
        }
    }
    free(scopes as *mut c_void);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn find_data_type(dloc: *mut data_loc_info) -> *mut annotated_data_type {
    let dso_ = map__dso((*(*dloc).ms).map);
    let mut type_die: Dwarf_Die = zeroed();
    (*dloc).type_offset = (*(*dloc).op).offset;
    (*dloc).fbreg = -1;
    if find_data_type_die(dloc, &mut type_die) < 0 { return null_mut(); }
    dso__findnew_data_type(dso_, &mut type_die)
}

unsafe extern "C" fn alloc_data_type_histograms(adt: *mut annotated_data_type, nr_entries: c_int) -> c_int {
    let sz = size_of::<type_hist>() + size_of::<type_hist_entry>() * (*adt).self_.size as usize;
    (*adt).histograms = calloc(nr_entries as size_t, size_of::<*mut type_hist>()) as *mut *mut type_hist;
    if (*adt).histograms.is_null() { return -ENOMEM; }
    let mut i = 0;
    while i < nr_entries {
        *(*adt).histograms.add(i as usize) = zalloc(sz) as *mut type_hist;
        if (*(*adt).histograms.add(i as usize)).is_null() {
            while i > 0 { i -= 1; zfree(&mut *(*adt).histograms.add(i as usize)); }
            zfree(&mut (*adt).histograms);
            return -ENOMEM;
        }
        i += 1;
    }
    (*adt).nr_histograms = nr_entries;
    0
}

unsafe extern "C" fn delete_data_type_histograms(adt: *mut annotated_data_type) {
    let mut i = 0;
    while i < (*adt).nr_histograms {
        zfree(&mut *(*adt).histograms.add(i as usize));
        i += 1;
    }
    zfree(&mut (*adt).histograms);
    (*adt).nr_histograms = 0;
}

#[no_mangle]
pub unsafe extern "C" fn annotated_data_type__tree_delete(root: *mut rb_root) {
    while !RB_EMPTY_ROOT(root) {
        let node = rb_first(root);
        rb_erase(node, root);
        let pos = node as *mut annotated_data_type;
        delete_members(&mut (*pos).self_);
        delete_data_type_histograms(pos);
        zfree(&mut (*pos).self_.type_name);
        free(pos as *mut c_void);
    }
}

#[no_mangle]
pub unsafe extern "C" fn annotated_data_type__update_samples(adt: *mut annotated_data_type, evsel: *mut evsel, offset: c_int, nr_samples: c_int, period: u64) -> c_int {
    if adt.is_null() { return 0; }
    if (*adt).histograms.is_null() {
        let nr = evlist__nr_entries((*evsel).evlist);
        if alloc_data_type_histograms(adt, nr) < 0 { return -1; }
    }
    if offset < 0 || offset >= (*adt).self_.size as c_int { return -1; }
    let h = *(*adt).histograms.add((*evsel).core.idx as usize);
    (*h).nr_samples += nr_samples;
    let addr = (*h).addr.as_ptr().add(offset as usize) as *mut type_hist_entry;
    (*addr).nr_samples += nr_samples;
    (*h).period += period;
    (*addr).period += period;
    0
}

unsafe extern "C" fn print_annotated_data_header(he: *mut hist_entry, evsel: *mut evsel) {
    let dso_ = map__dso((*he).ms.map);
    let nr_samples = (*he).stat.nr_events;
    let mut width = 7;
    let mut val_hdr = c!("Percent");
    printf(c!("Annotate type: '%s' in %s (%d samples):\n"), (*(*he).mem_type).self_.type_name, dso__name(dso_), nr_samples);
    if symbol_conf.show_total_period { width = 11; val_hdr = c!("Period"); }
    else if symbol_conf.show_nr_samples { width = 7; val_hdr = c!("Samples"); }
    printf(c!("============================================================================\n"));
    printf(c!("%*s %10s %10s  %s\n"), width + 1, val_hdr, c!("offset"), c!("size"), c!("field"));
}

unsafe extern "C" fn print_annotated_data_value(h: *mut type_hist, period: u64, nr_samples: c_int) {
    let percent = if (*h).period != 0 { 100.0 * period as c_double / (*h).period as c_double } else { 0.0 };
    let color = get_percent_color(percent);
    if symbol_conf.show_total_period { color_fprintf(stdout, color, c!(" %11lu"), period); }
    else if symbol_conf.show_nr_samples { color_fprintf(stdout, color, c!(" %7d"), nr_samples); }
    else { color_fprintf(stdout, color, c!(" %7.2f"), percent); }
}

unsafe extern "C" fn print_annotated_data_type(mem_type: *mut annotated_data_type, member: *mut annotated_member, evsel: *mut evsel, indent: c_int) {
    let h = *(*mem_type).histograms.add((*evsel).core.idx as usize);
    let mut samples = 0;
    let mut period = 0;
    let width = if symbol_conf.show_total_period { 11 } else { 7 };
    let mut i = 0;
    while i < (*member).size as c_int {
        let e = (*h).addr.as_ptr().add(((*member).offset as c_int + i) as usize);
        samples += (*e).nr_samples;
        period += (*e).period;
        i += 1;
    }
    print_annotated_data_value(h, period, samples);
    printf(c!(" %#10x %#10x  %*s%s\t%s"), (*member).offset as c_int, (*member).size as c_int, indent, c!(""), (*member).type_name, if !(*member).var_name.is_null() { (*member).var_name } else { c!("") as *mut c_char });
    if !list_empty(&(*member).children) { printf(c!(" {\n")); }
    let mut node = (*member).children.next;
    while node != &mut (*member).children {
        let child = node as *mut annotated_member;
        print_annotated_data_type(mem_type, child, evsel, indent + 4);
        node = (*node).next;
    }
    if !list_empty(&(*member).children) { printf(c!("%*s}"), (width + 1) + 24 + indent, c!("")); }
    printf(c!(";\n"));
}

#[no_mangle]
pub unsafe extern "C" fn hist_entry__annotate_data_tty(he: *mut hist_entry, evsel: *mut evsel) -> c_int {
    print_annotated_data_header(he, evsel);
    print_annotated_data_type((*he).mem_type, &mut (*(*he).mem_type).self_, evsel, 0);
    printf(c!("\n"));
    '>' as c_int
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
