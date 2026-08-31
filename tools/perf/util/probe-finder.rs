// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * probe-finder.c : C expression to kprobe event converter
 *
 * Written by Masami Hiramatsu <mhiramat@redhat.com>
 *
 * Rust translation of the isolated C source. External perf/libdw/libelf
 * declarations are intentionally left as FFI dependencies.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type u64 = u64;
type uintmax_t = u64;
type Dwarf_Addr = u64;
type Dwarf_Off = u64;
type Dwarf_Word = u64;
type Dwarf_Sword = i64;

const MAX_BASIC_TYPE_BITS: c_int = 64;
const STRERR_BUFSIZE: usize = 128;
const MAX_PROBE_ARGS: usize = 128;
const BUILD_ID_SIZE: usize = 20;
const SBUILD_ID_SIZE: usize = 64;
const PATH_MAX: usize = 4096;

const ENOENT: c_int = 2;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENOTSUP: c_int = 95;
const ERANGE: c_int = 34;
const E2BIG: c_int = 7;
const EEXIST: c_int = 17;
const ENAMETOOLONG: c_int = 36;
const EROFS: c_int = 30;
const EFAULT: c_int = 14;

const R_OK: c_int = 4;
const SHT_PROGBITS: u32 = 1;

const DW_LANG_C89: c_int = 0x0001;
const DW_LANG_C: c_int = 0x0002;
const DW_LANG_C99: c_int = 0x000c;
const DW_LANG_C11: c_int = 0x001d;

const DW_AT_external: c_uint = 0x3f;
const DW_AT_const_value: c_uint = 0x1c;
const DW_AT_location: c_uint = 0x02;
const DW_AT_frame_base: c_uint = 0x40;
const DW_AT_abstract_origin: c_uint = 0x31;

const DW_TAG_variable: c_int = 0x34;
const DW_TAG_formal_parameter: c_int = 0x05;
const DW_TAG_pointer_type: c_int = 0x0f;
const DW_TAG_array_type: c_int = 0x01;
const DW_TAG_structure_type: c_int = 0x13;
const DW_TAG_union_type: c_int = 0x17;
const DW_TAG_subprogram: c_int = 0x2e;

const DW_OP_addr: c_uint = 0x03;
const DW_OP_reg0: c_uint = 0x50;
const DW_OP_reg31: c_uint = 0x6f;
const DW_OP_breg0: c_uint = 0x70;
const DW_OP_breg31: c_uint = 0x8f;
const DW_OP_regx: c_uint = 0x90;
const DW_OP_fbreg: c_uint = 0x91;
const DW_OP_bregx: c_uint = 0x92;
const DW_OP_call_frame_cfa: c_uint = 0x9c;

const DWARF_CB_OK: c_int = 0;
const DWARF_CB_ABORT: c_int = 1;
const DIE_FIND_CB_END: c_int = 0;
const DIE_FIND_CB_CONTINUE: c_int = 1;
const DIE_FIND_CB_SIBLING: c_int = 2;

const PROBE_TYPE_X: c_int = 0;

unsafe fn BYTES_TO_BITS(x: c_int) -> c_int {
    x * 8
}

#[repr(C)] pub struct Dwarf_Die { _priv: [u8; 0] }
#[repr(C)] pub struct Dwarf_Attribute { _priv: [u8; 0] }
#[repr(C)] pub struct Dwarf { _priv: [u8; 0] }
#[repr(C)] pub struct Dwarf_CFI { _priv: [u8; 0] }
#[repr(C)] pub struct Dwarf_Frame { _priv: [u8; 0] }
#[repr(C)] pub struct Dwfl_Module { _priv: [u8; 0] }
#[repr(C)] pub struct Elf { _priv: [u8; 0] }
#[repr(C)] pub struct FILE { _priv: [u8; 0] }
#[repr(C)] pub struct intlist { _priv: [u8; 0] }
#[repr(C)] pub struct strlist { _priv: [u8; 0] }

#[repr(C)]
pub struct Dwarf_Op {
    pub atom: c_uint,
    pub number: Dwarf_Word,
    pub number2: Dwarf_Word,
    pub offset: Dwarf_Off,
}

#[repr(C)] pub struct GElf_Sym { pub st_value: u64 }
#[repr(C)] pub struct GElf_Ehdr { pub e_machine: u16, pub e_flags: u32 }
#[repr(C)] pub struct GElf_Shdr { pub sh_type: u32 }
#[repr(C)] pub struct Dwarf_Global { pub die_offset: Dwarf_Off, pub cu_offset: Dwarf_Off }
#[repr(C)] pub struct build_id { _priv: [u8; 0] }

#[repr(C)]
pub struct probe_trace_arg_ref {
    pub next: *mut probe_trace_arg_ref,
    pub offset: c_long,
    pub user_access: bool,
}

#[repr(C)]
pub struct probe_trace_arg {
    pub name: *mut c_char,
    pub value: *mut c_char,
    pub type_: *mut c_char,
    pub ref_: *mut probe_trace_arg_ref,
}

#[repr(C)]
pub struct perf_probe_arg_field {
    pub name: *mut c_char,
    pub index: c_int,
    pub ref_: bool,
    pub next: *mut perf_probe_arg_field,
}

#[repr(C)]
pub struct perf_probe_arg {
    pub name: *mut c_char,
    pub var: *mut c_char,
    pub type_: *mut c_char,
    pub field: *mut perf_probe_arg_field,
    pub user_access: bool,
}

#[repr(C)]
pub struct perf_probe_point {
    pub function: *mut c_char,
    pub file: *mut c_char,
    pub line: c_int,
    pub retprobe: bool,
    pub lazy_line: *mut c_char,
    pub offset: u64,
    pub abs_address: u64,
}

#[repr(C)]
pub struct perf_probe_event {
    pub point: perf_probe_point,
    pub uprobes: bool,
    pub nargs: c_int,
    pub args: *mut perf_probe_arg,
}

#[repr(C)]
pub struct probe_trace_point {
    pub symbol: *mut c_char,
    pub realname: *mut c_char,
    pub offset: c_ulong,
    pub address: Dwarf_Addr,
    pub retprobe: bool,
}

#[repr(C)]
pub struct probe_trace_event {
    pub point: probe_trace_point,
    pub nargs: c_int,
    pub args: *mut probe_trace_arg,
    pub lang: c_int,
}

#[repr(C)]
pub struct variable_list {
    pub point: probe_trace_point,
    pub vars: *mut strlist,
}

#[repr(C)]
pub struct debuginfo {
    pub dbg: *mut Dwarf,
    pub mod_: *mut Dwfl_Module,
    pub build_id: *mut u8,
}

type probe_finder_cb = Option<unsafe extern "C" fn(*mut Dwarf_Die, *mut probe_finder) -> c_int>;

#[repr(C)]
pub struct probe_finder {
    pub pev: *mut perf_probe_event,
    pub dbg: *mut debuginfo,
    pub callback: probe_finder_cb,
    pub addr: Dwarf_Addr,
    pub fb_ops: *mut Dwarf_Op,
    pub sp_die: Dwarf_Die,
    pub cu_die: Dwarf_Die,
    pub cfi_eh: *mut Dwarf_CFI,
    pub cfi_dbg: *mut Dwarf_CFI,
    pub e_machine: u16,
    pub e_flags: u32,
    pub pvar: *mut perf_probe_arg,
    pub tvar: *mut probe_trace_arg,
    pub fname: *const c_char,
    pub lno: c_int,
    pub lcache: *mut intlist,
    pub abstrace_dieoffset: Dwarf_Off,
    pub skip_empty_arg: bool,
}

#[repr(C)]
pub struct trace_event_finder {
    pub pf: probe_finder,
    pub tevs: *mut probe_trace_event,
    pub ntevs: c_int,
    pub max_tevs: c_int,
    pub mod_: *mut Dwfl_Module,
}

#[repr(C)]
pub struct available_var_finder {
    pub pf: probe_finder,
    pub vls: *mut variable_list,
    pub nvls: c_int,
    pub max_vls: c_int,
    pub mod_: *mut Dwfl_Module,
    pub child: bool,
}

#[repr(C)]
pub struct strbuf {
    pub buf: *mut c_char,
    pub len: size_t,
    pub alloc: size_t,
}

#[repr(C)]
pub struct line_range {
    pub function: *mut c_char,
    pub file: *mut c_char,
    pub start: c_int,
    pub end: c_int,
    pub offset: c_int,
    pub path: *mut c_char,
    pub comp_dir: *mut c_char,
    pub line_list: *mut intlist,
}

#[repr(C)]
pub struct line_finder {
    pub lr: *mut line_range,
    pub cu_die: Dwarf_Die,
    pub sp_die: Dwarf_Die,
    pub fname: *const c_char,
    pub lno_s: c_int,
    pub lno_e: c_int,
    pub found: c_int,
}

#[repr(C)] pub struct probe_conf_t { pub show_location_range: bool, pub no_inlines: bool, pub max_probes: c_int, pub magic_num: c_ulong, pub show_ext_vars: bool }
#[repr(C)] pub struct symbol_conf_t { pub source_prefix: *const c_char }

unsafe extern "C" {
    static mut probe_conf: probe_conf_t;
    static mut symbol_conf: symbol_conf_t;
    static mut errno: c_int;
    static PROBE_ARG_VARS: *const c_char;
    static PROBE_ARG_PARAMS: *const c_char;

    fn zalloc(size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn ferror(stream: *mut FILE) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn access(path: *const c_char, mode: c_int) -> c_int;

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> *mut c_char;
    fn immediate_value_is_supported() -> bool;
    fn probe_type_is_available(t: c_int) -> bool;

    fn dwarf_attr(die: *mut Dwarf_Die, attr: c_uint, result: *mut Dwarf_Attribute) -> *mut Dwarf_Attribute;
    fn dwarf_formsdata(attr: *mut Dwarf_Attribute, return_sval: *mut Dwarf_Sword) -> c_int;
    fn dwarf_getlocation_addr(attr: *mut Dwarf_Attribute, address: Dwarf_Addr, expr: *mut *mut Dwarf_Op, exprlen: *mut size_t, max: size_t) -> isize;
    fn dwarf_entrypc(die: *mut Dwarf_Die, return_addr: *mut Dwarf_Addr) -> c_int;
    fn dwarf_highpc(die: *mut Dwarf_Die, return_addr: *mut Dwarf_Addr) -> c_int;
    fn dwarf_tag(die: *mut Dwarf_Die) -> c_int;
    fn dwarf_diename(die: *mut Dwarf_Die) -> *const c_char;
    fn dwarf_bitsize(die: *mut Dwarf_Die) -> c_int;
    fn dwarf_bitoffset(die: *mut Dwarf_Die) -> c_int;
    fn dwarf_bytesize(die: *mut Dwarf_Die) -> c_int;
    fn dwarf_dieoffset(die: *mut Dwarf_Die) -> Dwarf_Off;
    fn dwarf_haspc(die: *mut Dwarf_Die, addr: Dwarf_Addr) -> bool;
    fn dwarf_decl_line(die: *mut Dwarf_Die, linep: *mut c_int) -> c_int;
    fn dwarf_cfi_addrframe(cache: *mut Dwarf_CFI, address: Dwarf_Addr, frame: *mut *mut Dwarf_Frame) -> c_int;
    fn dwarf_frame_cfa(frame: *mut Dwarf_Frame, ops: *mut *mut Dwarf_Op, nops: *mut size_t) -> c_int;
    fn dwarf_getfuncs(cu: *mut Dwarf_Die, cb: unsafe extern "C" fn(*mut Dwarf_Die, *mut c_void) -> c_int, arg: *mut c_void, offset: c_long) -> isize;
    fn dwarf_offdie(dbg: *mut Dwarf, offset: Dwarf_Off, result: *mut Dwarf_Die) -> *mut Dwarf_Die;
    fn dwarf_getpubnames(dbg: *mut Dwarf, cb: unsafe extern "C" fn(*mut Dwarf, *mut Dwarf_Global, *mut c_void) -> c_int, arg: *mut c_void, offset: c_long) -> isize;
    fn dwarf_nextcu(dbg: *mut Dwarf, off: Dwarf_Off, noff: *mut Dwarf_Off, cuhl: *mut size_t, a: *mut c_void, b: *mut c_void, c: *mut c_void) -> c_int;
    fn dwarf_getelf(dbg: *mut Dwarf) -> *mut Elf;
    fn dwarf_getcfi_elf(elf: *mut Elf) -> *mut Dwarf_CFI;
    fn dwarf_getcfi(dbg: *mut Dwarf) -> *mut Dwarf_CFI;
    fn dwarf_cfi_end(cfi: *mut Dwarf_CFI);
    fn dwarf_diecu(die: *mut Dwarf_Die, result: *mut Dwarf_Die, a: *mut c_void, b: *mut c_void) -> *mut Dwarf_Die;
    fn dwarf_srclang(cu: *mut Dwarf_Die) -> c_int;
    fn dwarf_formref_die(attr: *mut Dwarf_Attribute, result: *mut Dwarf_Die) -> *mut Dwarf_Die;
    fn dwarf_addrdie(dbg: *mut Dwarf, addr: Dwarf_Addr, result: *mut Dwarf_Die) -> *mut Dwarf_Die;

    fn gelf_getehdr(elf: *mut Elf, dst: *mut GElf_Ehdr) -> *mut GElf_Ehdr;
    fn elf_section_by_name(elf: *mut Elf, ehdr: *mut GElf_Ehdr, shdr: *mut GElf_Shdr, name: *const c_char, idx: *mut c_void) -> bool;
    fn dwfl_module_addrsym(mod_: *mut Dwfl_Module, addr: Dwarf_Addr, sym: *mut GElf_Sym, shndxp: *mut c_void) -> *const c_char;

    fn get_dwarf_regstr(n: c_uint, machine: u16, flags: u32) -> *const c_char;
    fn die_get_real_type(die: *mut Dwarf_Die, result: *mut Dwarf_Die) -> *mut Dwarf_Die;
    fn die_name(die: *mut Dwarf_Die) -> *const c_char;
    fn die_compare_name(die: *mut Dwarf_Die, name: *const c_char) -> bool;
    fn die_is_signed_type(die: *mut Dwarf_Die) -> bool;
    fn die_find_member(type_: *mut Dwarf_Die, name: *const c_char, result: *mut Dwarf_Die) -> *mut Dwarf_Die;
    fn die_get_data_member_location(die: *mut Dwarf_Die, offs: *mut Dwarf_Word) -> c_int;
    fn die_find_variable_at(sc: *mut Dwarf_Die, name: *const c_char, addr: Dwarf_Addr, result: *mut Dwarf_Die) -> bool;
    fn die_is_func_def(die: *mut Dwarf_Die) -> bool;
    fn die_find_realfunc(cu: *mut Dwarf_Die, addr: Dwarf_Addr, result: *mut Dwarf_Die) -> bool;
    fn die_find_tailfunc(cu: *mut Dwarf_Die, addr: Dwarf_Addr, result: *mut Dwarf_Die) -> bool;
    fn die_get_decl_file(die: *mut Dwarf_Die) -> *const c_char;
    fn die_match_name(die: *mut Dwarf_Die, name: *const c_char) -> bool;
    fn cu_walk_functions_at(cu: *mut Dwarf_Die, addr: Dwarf_Addr, cb: unsafe extern "C" fn(*mut Dwarf_Die, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn cu_find_lineinfo(cu: *mut Dwarf_Die, addr: Dwarf_Addr, fname: *mut *const c_char, lineno: *mut c_int) -> c_int;
    fn die_walk_lines(die: *mut Dwarf_Die, cb: unsafe extern "C" fn(*const c_char, c_int, Dwarf_Addr, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn cu_get_comp_dir(cu: *mut Dwarf_Die) -> *const c_char;
    fn die_is_optimized_target(cu: *mut Dwarf_Die) -> bool;
    fn perf_probe_with_var(pev: *mut perf_probe_event) -> bool;
    fn die_skip_prologue(sp: *mut Dwarf_Die, cu: *mut Dwarf_Die, addr: *mut Dwarf_Addr);
    fn die_entrypc(die: *mut Dwarf_Die, addr: *mut Dwarf_Addr) -> c_int;
    fn die_is_func_instance(die: *mut Dwarf_Die) -> bool;
    fn die_walk_instances(die: *mut Dwarf_Die, cb: unsafe extern "C" fn(*mut Dwarf_Die, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn cu_find_realpath(cu: *mut Dwarf_Die, file: *const c_char) -> *const c_char;
    fn die_find_child(die: *mut Dwarf_Die, cb: unsafe extern "C" fn(*mut Dwarf_Die, *mut c_void) -> c_int, data: *mut c_void, result: *mut Dwarf_Die) -> c_int;
    fn die_get_varname(die: *mut Dwarf_Die, buf: *mut strbuf) -> c_int;
    fn die_get_var_range(sp: *mut Dwarf_Die, die: *mut Dwarf_Die, buf: *mut strbuf) -> c_int;
    fn die_get_linkage_name(die: *mut Dwarf_Die) -> *const c_char;
    fn die_find_top_inlinefunc(sp: *mut Dwarf_Die, addr: Dwarf_Addr, result: *mut Dwarf_Die) -> bool;
    fn die_get_call_lineno(die: *mut Dwarf_Die) -> c_int;
    fn die_get_call_file(die: *mut Dwarf_Die) -> *const c_char;

    fn is_c_varname(s: *const c_char) -> bool;
    fn copy_to_probe_trace_arg(tvar: *mut probe_trace_arg, pvar: *mut perf_probe_arg) -> c_int;
    fn synthesize_perf_probe_arg(pvar: *mut perf_probe_arg) -> *mut c_char;
    fn strtailcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlazymatch(line: *const c_char, pat: *const c_char) -> bool;
    fn strisglob(s: *const c_char) -> bool;
    fn intlist__new(a: *mut c_void) -> *mut intlist;
    fn intlist__delete(list: *mut intlist);
    fn intlist__empty(list: *mut intlist) -> bool;
    fn intlist__add(list: *mut intlist, i: c_int) -> c_int;
    fn intlist__has_entry(list: *mut intlist, i: c_int) -> bool;
    fn build_id__init(bid: *mut build_id, data: *mut u8, size: size_t);
    fn build_id__snprintf(bid: *mut build_id, buf: *mut c_char, size: size_t) -> c_int;
    fn debuginfo__get_text_offset(dbg: *mut debuginfo, base: *mut Dwarf_Addr, adjust: bool) -> c_int;
    fn path__join(buf: *mut c_char, size: size_t, a: *const c_char, b: *const c_char) -> *mut c_char;
    fn get_source_from_debuginfod(path: *mut c_char, build_id: *const c_char, new_path: *mut *mut c_char) -> c_int;
    fn clear_probe_trace_event(tev: *mut probe_trace_event);
    fn strbuf_init(buf: *mut strbuf, hint: size_t) -> c_int;
    fn strbuf_add(buf: *mut strbuf, data: *const c_char, len: size_t) -> c_int;
    fn strbuf_addch(buf: *mut strbuf, c: c_int) -> c_int;
    fn strbuf_detach(buf: *mut strbuf, sz: *mut size_t) -> *mut c_char;
    fn strbuf_release(buf: *mut strbuf);
    fn strlist__new(a: *mut c_void, b: *mut c_void) -> *mut strlist;
    fn strlist__add(list: *mut strlist, s: *mut c_char) -> c_int;
    fn strlist__empty(list: *mut strlist) -> bool;
    fn strlist__delete(list: *mut strlist);
}

unsafe fn zfree<T>(p: *mut *mut T) {
    if !(*p).is_null() {
        free(*p as *mut c_void);
        *p = ptr::null_mut();
    }
}

unsafe fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

pub unsafe extern "C" fn is_known_C_lang(lang: c_int) -> bool {
    match lang {
        DW_LANG_C89 | DW_LANG_C | DW_LANG_C99 | DW_LANG_C11 => true,
        _ => false,
    }
}

unsafe fn alloc_trace_arg_ref(offs: c_long) -> *mut probe_trace_arg_ref {
    let ref_ = zalloc(size_of::<probe_trace_arg_ref>()) as *mut probe_trace_arg_ref;
    if !ref_.is_null() {
        (*ref_).offset = offs;
    }
    ref_
}

unsafe fn convert_variable_location(vr_die: *mut Dwarf_Die, mut addr: Dwarf_Addr, fb_ops: *mut Dwarf_Op, sp_die: *mut Dwarf_Die, pf: *const probe_finder, tvar: *mut probe_trace_arg) -> c_int {
    let mut attr: Dwarf_Attribute = core::mem::zeroed();
    let mut tmp: Dwarf_Addr = 0;
    let mut op: *mut Dwarf_Op = ptr::null_mut();
    let mut nops: size_t = 0;
    let mut regn: c_uint;
    let mut offs: Dwarf_Word = 0;
    let mut ref_ = false;
    let regs: *const c_char;
    let name: *const c_char;
    let mut ret: c_int;
    let mut ret2: c_int = 0;

    if !dwarf_attr(vr_die, DW_AT_external, &mut attr).is_null() {
        goto_static_var(vr_die, tvar, offs, ret2);
        return STATIC_VAR_RET;
    }

    if !dwarf_attr(vr_die, DW_AT_const_value, &mut attr).is_null() && immediate_value_is_supported() {
        let mut snum: Dwarf_Sword = 0;
        if tvar.is_null() { return 0; }
        if dwarf_formsdata(&mut attr, &mut snum) != 0 { return -ENOENT; }
        ret = asprintf(&mut (*tvar).value, cstr(b"\\%ld\0"), snum as c_long);
        return if ret < 0 { -ENOMEM } else { 0 };
    }

    if dwarf_attr(vr_die, DW_AT_location, &mut attr).is_null() { return -EINVAL; }
    if dwarf_getlocation_addr(&mut attr, addr, &mut op, &mut nops, 1) <= 0 {
        if dwarf_entrypc(sp_die, &mut tmp) != 0 { return -ENOENT; }
        if probe_conf.show_location_range && dwarf_tag(vr_die) == DW_TAG_variable {
            ret2 = -ERANGE;
        } else if addr != tmp || dwarf_tag(vr_die) != DW_TAG_formal_parameter {
            return -ENOENT;
        }
        if dwarf_highpc(sp_die, &mut tmp) != 0 { return -ENOENT; }
        addr += 1;
        while addr <= tmp {
            if dwarf_getlocation_addr(&mut attr, addr, &mut op, &mut nops, 1) > 0 { break; }
            addr += 1;
        }
        if addr > tmp { return -ENOENT; }
    }

    if nops == 0 { return -ENOENT; }

    if (*op).atom == DW_OP_addr {
        if tvar.is_null() { return ret2; }
        name = dwarf_diename(vr_die);
        if name.is_null() { return -ENOENT; }
        ret = strlen(name) as c_int;
        (*tvar).value = zalloc((ret + 2) as size_t) as *mut c_char;
        if (*tvar).value.is_null() { return -ENOMEM; }
        snprintf((*tvar).value, (ret + 2) as size_t, cstr(b"@%s\0"), name);
        (*tvar).ref_ = alloc_trace_arg_ref(offs as c_long);
        if (*tvar).ref_.is_null() { return -ENOMEM; }
        return ret2;
    }

    if (*op).atom == DW_OP_fbreg {
        if fb_ops.is_null() { return -ENOTSUP; }
        ref_ = true;
        offs = (*op).number;
        op = fb_ops;
    }

    if (*op).atom >= DW_OP_breg0 && (*op).atom <= DW_OP_breg31 {
        regn = (*op).atom - DW_OP_breg0;
        offs = offs.wrapping_add((*op).number);
        ref_ = true;
    } else if (*op).atom >= DW_OP_reg0 && (*op).atom <= DW_OP_reg31 {
        regn = (*op).atom - DW_OP_reg0;
    } else if (*op).atom == DW_OP_bregx {
        regn = (*op).number as c_uint;
        offs = offs.wrapping_add((*op).number2);
        ref_ = true;
    } else if (*op).atom == DW_OP_regx {
        regn = (*op).number as c_uint;
    } else {
        pr_debug(cstr(b"DW_OP %x is not supported.\n\0"), (*op).atom);
        return -ENOTSUP;
    }

    if tvar.is_null() { return ret2; }
    regs = get_dwarf_regstr(regn, (*pf).e_machine, (*pf).e_flags);
    if regs.is_null() {
        pr_warning(cstr(b"Mapping for the register number %u missing on this architecture.\n\0"), regn);
        return -ENOTSUP;
    }
    (*tvar).value = strdup(regs);
    if (*tvar).value.is_null() { return -ENOMEM; }
    if ref_ {
        (*tvar).ref_ = alloc_trace_arg_ref(offs as c_long);
        if (*tvar).ref_.is_null() { return -ENOMEM; }
    }
    ret2
}

static mut STATIC_VAR_RET: c_int = 0;
unsafe fn goto_static_var(vr_die: *mut Dwarf_Die, tvar: *mut probe_trace_arg, offs: Dwarf_Word, ret2: c_int) {
    STATIC_VAR_RET = ret2;
    if tvar.is_null() { return; }
    let name = dwarf_diename(vr_die);
    if name.is_null() { STATIC_VAR_RET = -ENOENT; return; }
    let ret = strlen(name) as c_int;
    (*tvar).value = zalloc((ret + 2) as size_t) as *mut c_char;
    if (*tvar).value.is_null() { STATIC_VAR_RET = -ENOMEM; return; }
    snprintf((*tvar).value, (ret + 2) as size_t, cstr(b"@%s\0"), name);
    (*tvar).ref_ = alloc_trace_arg_ref(offs as c_long);
    if (*tvar).ref_.is_null() { STATIC_VAR_RET = -ENOMEM; }
}

unsafe fn convert_variable_type(vr_die: *mut Dwarf_Die, tvar: *mut probe_trace_arg, cast: *const c_char, user_access: bool) -> c_int {
    let mut ref_ptr: *mut *mut probe_trace_arg_ref = &mut (*tvar).ref_;
    let mut type_die: Dwarf_Die = core::mem::zeroed();
    let mut buf = [0i8; 16];
    let mut sbuf = [0i8; STRERR_BUFSIZE];
    let mut ret: c_int;
    let prefix: c_char;

    if !cast.is_null() && strcmp(cast, cstr(b"string\0")) != 0 && strcmp(cast, cstr(b"ustring\0")) != 0 &&
        strcmp(cast, cstr(b"x\0")) != 0 && strcmp(cast, cstr(b"s\0")) != 0 && strcmp(cast, cstr(b"u\0")) != 0 {
        (*tvar).type_ = strdup(cast);
        return if (*tvar).type_.is_null() { -ENOMEM } else { 0 };
    }

    let bsize = dwarf_bitsize(vr_die);
    if bsize > 0 {
        let boffs = dwarf_bitoffset(vr_die);
        let total = dwarf_bytesize(vr_die);
        if boffs < 0 || total < 0 { return -ENOENT; }
        ret = snprintf(buf.as_mut_ptr(), 16, cstr(b"b%d@%d/%d\0"), bsize, boffs, BYTES_TO_BITS(total));
    } else {
        if die_get_real_type(vr_die, &mut type_die).is_null() {
            let name = dwarf_diename(vr_die);
            pr_warning(cstr(b"Failed to get a type information of %s.\n\0"), if name.is_null() { cstr(b"<unknown>\0") } else { name });
            return -ENOENT;
        }
        pr_debug(cstr(b"%s type is %s.\n\0"), die_name(vr_die), die_name(&mut type_die));
        if !cast.is_null() && (strcmp(cast, cstr(b"string\0")) == 0 || strcmp(cast, cstr(b"ustring\0")) == 0) {
            ret = dwarf_tag(&mut type_die);
            if ret != DW_TAG_pointer_type && ret != DW_TAG_array_type {
                pr_warning(cstr(b"Failed to cast into string: %s(%s) is not a pointer nor array.\n\0"), die_name(vr_die), die_name(&mut type_die));
                return -EINVAL;
            }
            if die_get_real_type(&mut type_die, &mut type_die).is_null() {
                pr_warning(cstr(b"Failed to get a type information.\n\0"));
                return -ENOENT;
            }
            if ret == DW_TAG_pointer_type {
                while !(*ref_ptr).is_null() { ref_ptr = &mut (**ref_ptr).next; }
                *ref_ptr = zalloc(size_of::<probe_trace_arg_ref>()) as *mut probe_trace_arg_ref;
                if (*ref_ptr).is_null() {
                    pr_warning(cstr(b"Out of memory error\n\0"));
                    return -ENOMEM;
                }
                (**ref_ptr).user_access = user_access;
            }
            if !die_compare_name(&mut type_die, cstr(b"char\0")) && !die_compare_name(&mut type_die, cstr(b"unsigned char\0")) {
                pr_warning(cstr(b"Failed to cast into string: %s is not (unsigned) char *.\n\0"), die_name(vr_die));
                return -EINVAL;
            }
            (*tvar).type_ = strdup(cast);
            return if (*tvar).type_.is_null() { -ENOMEM } else { 0 };
        }
        prefix = if !cast.is_null() && strcmp(cast, cstr(b"u\0")) == 0 { b'u' as c_char }
        else if !cast.is_null() && strcmp(cast, cstr(b"s\0")) == 0 { b's' as c_char }
        else if !cast.is_null() && strcmp(cast, cstr(b"x\0")) == 0 && probe_type_is_available(PROBE_TYPE_X) { b'x' as c_char }
        else if die_is_signed_type(&mut type_die) { b's' as c_char }
        else if probe_type_is_available(PROBE_TYPE_X) { b'x' as c_char } else { b'u' as c_char };
        ret = dwarf_bytesize(&mut type_die);
        if ret <= 0 { return 0; }
        ret = BYTES_TO_BITS(ret);
        if ret > MAX_BASIC_TYPE_BITS {
            pr_info(cstr(b"%s exceeds max-bitwidth. Cut down to %d bits.\n\0"), die_name(&mut type_die), MAX_BASIC_TYPE_BITS);
            ret = MAX_BASIC_TYPE_BITS;
        }
        ret = snprintf(buf.as_mut_ptr(), 16, cstr(b"%c%d\0"), prefix as c_int, ret);
    }
    if ret < 0 || ret >= 16 {
        if ret >= 16 { ret = -E2BIG; }
        pr_warning(cstr(b"Failed to convert variable type: %s\n\0"), str_error_r(-ret, sbuf.as_mut_ptr(), sbuf.len()));
        return ret;
    }
    (*tvar).type_ = strdup(buf.as_ptr());
    if (*tvar).type_.is_null() { return -ENOMEM; }
    0
}

unsafe fn convert_variable_fields(vr_die: *mut Dwarf_Die, varname: *const c_char, field: *mut perf_probe_arg_field, ref_ptr: *mut *mut probe_trace_arg_ref, die_mem: *mut Dwarf_Die, user_access: bool) -> c_int {
    let mut ref_ = *ref_ptr;
    let mut type_die: Dwarf_Die = core::mem::zeroed();
    let mut offs: Dwarf_Word = 0;
    let mut ret: c_int;
    pr_debug(cstr(b"converting %s in %s\n\0"), (*field).name, varname);
    if die_get_real_type(vr_die, &mut type_die).is_null() {
        pr_warning(cstr(b"Failed to get the type of %s.\n\0"), varname);
        return -ENOENT;
    }
    pr_debug2(cstr(b"Var real type: %s (%x)\n\0"), die_name(&mut type_die), dwarf_dieoffset(&mut type_die) as c_uint);
    let mut tag = dwarf_tag(&mut type_die);

    if *(*field).name == b'[' as c_char && (tag == DW_TAG_array_type || tag == DW_TAG_pointer_type) {
        memcpy(die_mem as *mut c_void, &type_die as *const _ as *const c_void, size_of::<Dwarf_Die>());
        if die_get_real_type(&mut type_die, &mut type_die).is_null() {
            pr_warning(cstr(b"Failed to get the type of %s.\n\0"), varname);
            return -ENOENT;
        }
        pr_debug2(cstr(b"Array real type: %s (%x)\n\0"), die_name(&mut type_die), dwarf_dieoffset(&mut type_die) as c_uint);
        if tag == DW_TAG_pointer_type {
            ref_ = zalloc(size_of::<probe_trace_arg_ref>()) as *mut probe_trace_arg_ref;
            if ref_.is_null() { return -ENOMEM; }
            if !(*ref_ptr).is_null() { (**ref_ptr).next = ref_; } else { *ref_ptr = ref_; }
        }
        let bsize = dwarf_bytesize(&mut type_die);
        if bsize < 0 { return -EINVAL; }
        if ref_.is_null() {
            pr_warning(cstr(b"Array indexing not supported for variables in registers.\n\0"));
            return -ENOTSUP;
        }
        (*ref_).offset += (bsize * (*field).index) as c_long;
        (*ref_).user_access = user_access;
    } else {
        if tag == DW_TAG_pointer_type {
            if !(*field).ref_ {
                pr_err(cstr(b"Semantic error: %s must be referred by '->'\n\0"), (*field).name);
                return -EINVAL;
            }
            if die_get_real_type(&mut type_die, &mut type_die).is_null() {
                pr_warning(cstr(b"Failed to get the type of %s.\n\0"), varname);
                return -ENOENT;
            }
            tag = dwarf_tag(&mut type_die);
            if tag != DW_TAG_structure_type && tag != DW_TAG_union_type {
                pr_warning(cstr(b"%s is not a data structure nor a union.\n\0"), varname);
                return -EINVAL;
            }
            ref_ = zalloc(size_of::<probe_trace_arg_ref>()) as *mut probe_trace_arg_ref;
            if ref_.is_null() { return -ENOMEM; }
            if !(*ref_ptr).is_null() { (**ref_ptr).next = ref_; } else { *ref_ptr = ref_; }
        } else {
            if tag != DW_TAG_structure_type && tag != DW_TAG_union_type {
                pr_warning(cstr(b"%s is not a data structure nor a union.\n\0"), varname);
                return -EINVAL;
            }
            if *(*field).name == b'[' as c_char {
                pr_err(cstr(b"Semantic error: %s is not a pointer nor array.\n\0"), varname);
                return -EINVAL;
            }
            if (*field).ref_ && !dwarf_diename(vr_die).is_null() {
                pr_err(cstr(b"Semantic error: %s must be referred by '.'\n\0"), (*field).name);
                return -EINVAL;
            }
            if ref_.is_null() {
                pr_warning(cstr(b"Structure on a register is not supported yet.\n\0"));
                return -ENOTSUP;
            }
        }
        if die_find_member(&mut type_die, (*field).name, die_mem).is_null() {
            pr_warning(cstr(b"%s(type:%s) has no member %s.\n\0"), varname, die_name(&mut type_die), (*field).name);
            return -EINVAL;
        }
        if tag == DW_TAG_union_type { offs = 0; } else {
            ret = die_get_data_member_location(die_mem, &mut offs);
            if ret < 0 {
                pr_warning(cstr(b"Failed to get the offset of %s.\n\0"), (*field).name);
                return ret;
            }
        }
        (*ref_).offset += offs as c_long;
        (*ref_).user_access = user_access;
        if dwarf_diename(die_mem).is_null() {
            return convert_variable_fields(die_mem, varname, field, &mut ref_, die_mem, user_access);
        }
    }
    if !(*field).next.is_null() {
        convert_variable_fields(die_mem, (*field).name, (*field).next, &mut ref_, die_mem, user_access)
    } else { 0 }
}

unsafe fn print_var_not_found(varname: *const c_char) {
    pr_err(cstr(b"Failed to find the location of the '%s' variable at this address.\n Perhaps it has been optimized out.\n Use -V with the --range option to show '%s' location range.\n\0"), varname, varname);
}

unsafe fn convert_variable(vr_die: *mut Dwarf_Die, pf: *mut probe_finder) -> c_int {
    let mut die_mem: Dwarf_Die = core::mem::zeroed();
    pr_debug(cstr(b"Converting variable %s into trace event.\n\0"), die_name(vr_die));
    let mut ret = convert_variable_location(vr_die, (*pf).addr, (*pf).fb_ops, &mut (*pf).sp_die, pf, (*pf).tvar);
    if ret == -ENOENT && (*pf).skip_empty_arg { return 0; }
    if ret == -ENOENT || ret == -EINVAL { print_var_not_found((*(*pf).pvar).var); }
    else if ret == -ENOTSUP { pr_err(cstr(b"Sorry, we don't support this variable location yet.\n\0")); }
    else if ret == 0 && !(*(*pf).pvar).field.is_null() {
        ret = convert_variable_fields(vr_die, (*(*pf).pvar).var, (*(*pf).pvar).field, &mut (*(*pf).tvar).ref_, &mut die_mem, (*(*pf).pvar).user_access);
        vr_die = &mut die_mem;
    }
    if ret == 0 { ret = convert_variable_type(vr_die, (*pf).tvar, (*(*pf).pvar).type_, (*(*pf).pvar).user_access); }
    ret
}

unsafe fn find_variable(sc_die: *mut Dwarf_Die, pf: *mut probe_finder) -> c_int {
    let mut vr_die: Dwarf_Die = core::mem::zeroed();
    let mut ret = 0;
    if !is_c_varname((*(*pf).pvar).var) { return copy_to_probe_trace_arg((*pf).tvar, (*pf).pvar); }
    if !(*(*pf).pvar).name.is_null() { (*(*pf).tvar).name = strdup((*(*pf).pvar).name); }
    else {
        let buf = synthesize_perf_probe_arg((*pf).pvar);
        if buf.is_null() { return -ENOMEM; }
        let ptr_ = strchr(buf, b':' as c_int);
        if !ptr_.is_null() { *ptr_ = b'_' as c_char; }
        (*(*pf).tvar).name = buf;
    }
    if (*(*pf).tvar).name.is_null() { return -ENOMEM; }
    pr_debug(cstr(b"Searching '%s' variable in context.\n\0"), (*(*pf).pvar).var);
    if !die_find_variable_at(sc_die, (*(*pf).pvar).var, (*pf).addr, &mut vr_die) {
        if !die_find_variable_at(&mut (*pf).cu_die, (*(*pf).pvar).var, 0, &mut vr_die) {
            if (*pf).skip_empty_arg { return 0; }
            pr_warning(cstr(b"Failed to find '%s' in this function.\n\0"), (*(*pf).pvar).var);
            ret = -ENOENT;
        }
    }
    if ret >= 0 { ret = convert_variable(&mut vr_die, pf); }
    ret
}

unsafe fn convert_to_trace_point(sp_die: *mut Dwarf_Die, mod_: *mut Dwfl_Module, paddr: Dwarf_Addr, retprobe: bool, function: *const c_char, tp: *mut probe_trace_point) -> c_int {
    let mut eaddr: Dwarf_Addr = 0;
    let mut sym: GElf_Sym = core::mem::zeroed();
    let mut symbol: *const c_char;
    if !dwarf_haspc(sp_die, paddr) {
        pr_warning(cstr(b"Specified offset is out of %s\n\0"), die_name(sp_die));
        return -EINVAL;
    }
    if dwarf_entrypc(sp_die, &mut eaddr) == 0 { symbol = dwarf_diename(sp_die); }
    else {
        symbol = dwfl_module_addrsym(mod_, paddr, &mut sym, ptr::null_mut());
        eaddr = sym.st_value;
    }
    if symbol.is_null() {
        pr_warning(cstr(b"Failed to find symbol at 0x%lx\n\0"), paddr as c_ulong);
        return -ENOENT;
    }
    (*tp).offset = (paddr - eaddr) as c_ulong;
    (*tp).address = paddr;
    (*tp).symbol = strdup(symbol);
    if (*tp).symbol.is_null() { return -ENOMEM; }
    if retprobe {
        if eaddr != paddr {
            pr_warning(cstr(b"Failed to find \"%s%%return\",\n because %s is an inlined function and has no return point.\n\0"), function, function);
            return -EINVAL;
        }
        (*tp).retprobe = true;
    }
    0
}

unsafe fn call_probe_finder(sc_die: *mut Dwarf_Die, pf: *mut probe_finder) -> c_int {
    let mut fb_attr: Dwarf_Attribute = core::mem::zeroed();
    let mut frame: *mut Dwarf_Frame = ptr::null_mut();
    let mut nops: size_t = 0;
    if sc_die.is_null() {
        pr_err(cstr(b"Caller must pass a scope DIE. Program error.\n\0"));
        return -EINVAL;
    }
    if !die_is_func_def(sc_die) {
        if !die_find_realfunc(&mut (*pf).cu_die, (*pf).addr, &mut (*pf).sp_die) {
            if die_find_tailfunc(&mut (*pf).cu_die, (*pf).addr, &mut (*pf).sp_die) {
                pr_warning(cstr(b"Ignoring tail call from %s\n\0"), die_name(&mut (*pf).sp_die));
                return 0;
            } else {
                pr_warning(cstr(b"Failed to find probe point in any functions.\n\0"));
                return -ENOENT;
            }
        }
    } else {
        memcpy(&mut (*pf).sp_die as *mut _ as *mut c_void, sc_die as *const c_void, size_of::<Dwarf_Die>());
    }
    if dwarf_attr(&mut (*pf).sp_die, DW_AT_frame_base, &mut fb_attr).is_null() { (*pf).fb_ops = ptr::null_mut(); }
    else {
        let ret = dwarf_getlocation_addr(&mut fb_attr, (*pf).addr, &mut (*pf).fb_ops, &mut nops, 1);
        if ret <= 0 || nops == 0 { (*pf).fb_ops = ptr::null_mut(); }
    }
    if !(*pf).fb_ops.is_null() && nops == 1 && (*(*pf).fb_ops).atom == DW_OP_call_frame_cfa && (!(*pf).cfi_eh.is_null() || !(*pf).cfi_dbg.is_null()) {
        if (dwarf_cfi_addrframe((*pf).cfi_eh, (*pf).addr, &mut frame) != 0 && dwarf_cfi_addrframe((*pf).cfi_dbg, (*pf).addr, &mut frame) != 0) ||
            dwarf_frame_cfa(frame, &mut (*pf).fb_ops, &mut nops) != 0 {
            pr_warning(cstr(b"Failed to get call frame on 0x%jx\n\0"), (*pf).addr as uintmax_t);
            free(frame as *mut c_void);
            return -ENOENT;
        }
    }
    let ret = ((*pf).callback.unwrap())(sc_die, pf);
    free(frame as *mut c_void);
    (*pf).fb_ops = ptr::null_mut();
    ret
}

#[repr(C)] struct find_scope_param { function: *const c_char, file: *const c_char, line: c_int, diff: c_int, die_mem: *mut Dwarf_Die, found: bool }

unsafe extern "C" fn find_best_scope_cb(fn_die: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let fsp = data as *mut find_scope_param;
    let mut lno = 0;
    if !(*fsp).file.is_null() {
        let file = die_get_decl_file(fn_die);
        if file.is_null() || strcmp((*fsp).file, file) != 0 { return 0; }
    }
    if !(*fsp).function.is_null() {
        if die_match_name(fn_die, (*fsp).function) {
            memcpy((*fsp).die_mem as *mut c_void, fn_die as *const c_void, size_of::<Dwarf_Die>());
            (*fsp).found = true;
            return 1;
        }
    } else if dwarf_decl_line(fn_die, &mut lno) == 0 && lno < (*fsp).line && (*fsp).diff > (*fsp).line - lno {
        (*fsp).diff = (*fsp).line - lno;
        memcpy((*fsp).die_mem as *mut c_void, fn_die as *const c_void, size_of::<Dwarf_Die>());
        (*fsp).found = true;
    }
    0
}

unsafe extern "C" fn find_inner_scope_cb(fn_die: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let fsp = data as *mut find_scope_param;
    memcpy((*fsp).die_mem as *mut c_void, fn_die as *const c_void, size_of::<Dwarf_Die>());
    (*fsp).found = true;
    1
}

unsafe fn find_best_scope(pf: *mut probe_finder, die_mem: *mut Dwarf_Die) -> *mut Dwarf_Die {
    let mut fsp = find_scope_param { function: (*(*pf).pev).point.function, file: (*pf).fname, line: (*pf).lno, diff: c_int::MAX, die_mem, found: false };
    let ret = cu_walk_functions_at(&mut (*pf).cu_die, (*pf).addr, find_best_scope_cb, &mut fsp as *mut _ as *mut c_void);
    if ret == 0 && !fsp.found {
        cu_walk_functions_at(&mut (*pf).cu_die, (*pf).addr, find_inner_scope_cb, &mut fsp as *mut _ as *mut c_void);
    }
    if fsp.found { die_mem } else { ptr::null_mut() }
}

/* The remaining routines are translated as direct C-ABI Rust functions. */

unsafe fn verify_representive_line(pf: *mut probe_finder, fname: *const c_char, mut lineno: c_int, addr: Dwarf_Addr) -> c_int {
    let mut __fname: *const c_char = ptr::null();
    let mut __func: *const c_char = ptr::null();
    let mut die_mem: Dwarf_Die = core::mem::zeroed();
    let mut __lineno = 0;
    if cu_find_lineinfo(&mut (*pf).cu_die, addr, &mut __fname, &mut __lineno) < 0 { return 0; }
    pr_debug2(cstr(b"Reversed line: %s:%d\n\0"), __fname, __lineno);
    if strcmp(fname, __fname) != 0 || lineno == __lineno { return 0; }
    pr_warning(cstr(b"This line is sharing the address with other lines.\n\0"));
    if !(*(*pf).pev).point.function.is_null() {
        (*pf).addr = addr;
        if !find_best_scope(pf, &mut die_mem).is_null() && die_match_name(&mut die_mem, (*(*pf).pev).point.function) && dwarf_decl_line(&mut die_mem, &mut lineno) == 0 {
            __func = dwarf_diename(&mut die_mem);
            __lineno -= lineno;
        }
    }
    pr_warning(cstr(b"Please try to probe at %s:%d instead.\n\0"), if !__func.is_null() { __func } else { __fname }, __lineno);
    -ENOENT
}

unsafe extern "C" fn probe_point_line_walker(fname: *const c_char, lineno: c_int, addr: Dwarf_Addr, data: *mut c_void) -> c_int {
    let pf = data as *mut probe_finder;
    let mut die_mem: Dwarf_Die = core::mem::zeroed();
    if lineno != (*pf).lno || strtailcmp(fname, (*pf).fname) != 0 { return 0; }
    if verify_representive_line(pf, fname, lineno, addr) != 0 { return -ENOENT; }
    (*pf).addr = addr;
    let sc_die = find_best_scope(pf, &mut die_mem);
    if sc_die.is_null() {
        pr_warning(cstr(b"Failed to find scope of probe point.\n\0"));
        return -ENOENT;
    }
    let ret = call_probe_finder(sc_die, pf);
    if ret < 0 { ret } else { 0 }
}

unsafe fn find_probe_point_by_line(pf: *mut probe_finder) -> c_int {
    die_walk_lines(&mut (*pf).cu_die, probe_point_line_walker, pf as *mut c_void)
}

unsafe fn find_lazy_match_lines(list: *mut intlist, fname: *const c_char, pat: *const c_char) -> c_int {
    let mut line: *mut c_char = ptr::null_mut();
    let mut line_len: size_t = 0;
    let mut count = 0;
    let mut linenum = 1;
    let mut sbuf = [0i8; STRERR_BUFSIZE];
    let fp = fopen(fname, cstr(b"r\0"));
    if fp.is_null() {
        pr_warning(cstr(b"Failed to open %s: %s\n\0"), fname, str_error_r(errno, sbuf.as_mut_ptr(), sbuf.len()));
        return -errno;
    }
    loop {
        let len = getline(&mut line, &mut line_len, fp);
        if len <= 0 { break; }
        if *line.offset(len - 1) == b'\n' as c_char { *line.offset(len - 1) = 0; }
        if strlazymatch(line, pat) {
            intlist__add(list, linenum);
            count += 1;
        }
        linenum += 1;
    }
    if ferror(fp) != 0 { count = -errno; }
    free(line as *mut c_void);
    fclose(fp);
    if count == 0 { pr_debug(cstr(b"No matched lines found in %s.\n\0"), fname); }
    count
}

unsafe extern "C" fn probe_point_lazy_walker(fname: *const c_char, lineno: c_int, addr: Dwarf_Addr, data: *mut c_void) -> c_int {
    let pf = data as *mut probe_finder;
    let mut die_mem: Dwarf_Die = core::mem::zeroed();
    if !intlist__has_entry((*pf).lcache, lineno) || strtailcmp(fname, (*pf).fname) != 0 { return 0; }
    pr_debug(cstr(b"Probe line found: line:%d addr:0x%llx\n\0"), lineno, addr as u64);
    (*pf).addr = addr;
    (*pf).lno = lineno;
    let sc_die = find_best_scope(pf, &mut die_mem);
    if sc_die.is_null() {
        pr_warning(cstr(b"Failed to find scope of probe point.\n\0"));
        return -ENOENT;
    }
    let ret = call_probe_finder(sc_die, pf);
    if ret < 0 { ret } else { 0 }
}

unsafe fn find_probe_point_lazy(sp_die: *mut Dwarf_Die, pf: *mut probe_finder) -> c_int {
    let mut sbuild_id = [0i8; SBUILD_ID_SIZE];
    let mut ret = 0;
    let mut fpath: *mut c_char = ptr::null_mut();
    if intlist__empty((*pf).lcache) {
        let comp_dir = cu_get_comp_dir(&mut (*pf).cu_die);
        if !(*(*pf).dbg).build_id.is_null() {
            let mut bid: build_id = core::mem::zeroed();
            build_id__init(&mut bid, (*(*pf).dbg).build_id, BUILD_ID_SIZE);
            build_id__snprintf(&mut bid, sbuild_id.as_mut_ptr(), sbuild_id.len());
        }
        ret = find_source_path((*pf).fname, sbuild_id.as_ptr(), comp_dir, &mut fpath);
        if ret < 0 {
            pr_warning(cstr(b"Failed to find source file path.\n\0"));
            return ret;
        }
        ret = find_lazy_match_lines((*pf).lcache, fpath, (*(*pf).pev).point.lazy_line);
        free(fpath as *mut c_void);
        if ret <= 0 { return ret; }
    }
    die_walk_lines(sp_die, probe_point_lazy_walker, pf as *mut c_void)
}

unsafe fn skip_prologue(sp_die: *mut Dwarf_Die, pf: *mut probe_finder) {
    let pp = &mut (*(*pf).pev).point as *mut perf_probe_point;
    if !(*(*pf).pev).uprobes { return; }
    if die_is_optimized_target(&mut (*pf).cu_die) { return; }
    if (*pf).addr == 0 { return; }
    if (*pp).function.is_null() || (*pp).line != 0 || (*pp).retprobe || !(*pp).lazy_line.is_null() || (*pp).offset != 0 || (*pp).abs_address != 0 { return; }
    if !perf_probe_with_var((*pf).pev) { return; }
    pr_info(cstr(b"Target program is compiled without optimization. Skipping prologue.\nProbe on address 0x%lx to force probing at the function entry.\n\n\0"), (*pf).addr);
    die_skip_prologue(sp_die, &mut (*pf).cu_die, &mut (*pf).addr);
}

unsafe extern "C" fn probe_point_inline_cb(in_die: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let pf = data as *mut probe_finder;
    let pp = &mut (*(*pf).pev).point as *mut perf_probe_point;
    let mut addr = 0;
    if !(*pp).lazy_line.is_null() { return find_probe_point_lazy(in_die, pf); }
    if die_entrypc(in_die, &mut addr) != 0 {
        pr_warning(cstr(b"Failed to get entry address of %s.\n\0"), die_name(in_die));
        return -ENOENT;
    }
    if addr == 0 {
        pr_debug(cstr(b"%s has no valid entry address. skipped.\n\0"), die_name(in_die));
        return -ENOENT;
    }
    (*pf).addr = addr + (*pp).offset;
    pr_debug(cstr(b"found inline addr: 0x%jx\n\0"), (*pf).addr as uintmax_t);
    call_probe_finder(in_die, pf)
}

#[repr(C)] struct dwarf_callback_param { data: *mut c_void, retval: c_int }

unsafe extern "C" fn probe_point_search_cb(sp_die: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let param = data as *mut dwarf_callback_param;
    let pf = (*param).data as *mut probe_finder;
    let pp = &mut (*(*pf).pev).point as *mut perf_probe_point;
    if !die_is_func_def(sp_die) || !die_match_name(sp_die, (*pp).function) { return DWARF_CB_OK; }
    let fname = die_get_decl_file(sp_die);
    if fname.is_null() {
        pr_warning(cstr(b"A function DIE doesn't have decl_line. Maybe broken DWARF?\n\0"));
        return DWARF_CB_OK;
    }
    if !(*pp).file.is_null() && strtailcmp((*pp).file, fname) != 0 { return DWARF_CB_OK; }
    pr_debug(cstr(b"Matched function: %s [%lx]\n\0"), die_name(sp_die), dwarf_dieoffset(sp_die) as c_ulong);
    (*pf).fname = fname;
    (*pf).abstrace_dieoffset = dwarf_dieoffset(sp_die);
    if (*pp).line != 0 {
        if dwarf_decl_line(sp_die, &mut (*pf).lno) != 0 { return DWARF_CB_OK; }
        (*pf).lno += (*pp).line;
        (*param).retval = find_probe_point_by_line(pf);
    } else if die_is_func_instance(sp_die) {
        die_entrypc(sp_die, &mut (*pf).addr);
        if (*pf).addr == 0 {
            pr_debug(cstr(b"%s has no entry PC. Skipped\n\0"), die_name(sp_die));
            (*param).retval = 0;
        } else if !(*pp).lazy_line.is_null() { (*param).retval = find_probe_point_lazy(sp_die, pf); }
        else {
            skip_prologue(sp_die, pf);
            (*pf).addr += (*pp).offset;
            (*param).retval = call_probe_finder(sp_die, pf);
        }
    } else if !probe_conf.no_inlines {
        (*param).retval = die_walk_instances(sp_die, probe_point_inline_cb, pf as *mut c_void);
        if (*param).retval == -ENOENT { (*param).retval = 0; }
    }
    if strisglob((*pp).function) && (*param).retval >= 0 {
        (*param).retval = 0;
        return DWARF_CB_OK;
    }
    DWARF_CB_ABORT
}

unsafe fn find_probe_point_by_func(pf: *mut probe_finder) -> c_int {
    let mut param = dwarf_callback_param { data: pf as *mut c_void, retval: 0 };
    if dwarf_getfuncs(&mut (*pf).cu_die, probe_point_search_cb, &mut param as *mut _ as *mut c_void, 0) < 0 {
        pr_debug(cstr(b"Failed to get functions from CU\n\0"));
    }
    param.retval
}

#[repr(C)] struct pubname_callback_param { function: *mut c_char, file: *mut c_char, cu_die: *mut Dwarf_Die, sp_die: *mut Dwarf_Die, found: c_int }

unsafe extern "C" fn pubname_search_cb(dbg: *mut Dwarf, gl: *mut Dwarf_Global, data: *mut c_void) -> c_int {
    let param = data as *mut pubname_callback_param;
    if !dwarf_offdie(dbg, (*gl).die_offset, (*param).sp_die).is_null() {
        if dwarf_tag((*param).sp_die) != DW_TAG_subprogram { return DWARF_CB_OK; }
        if die_match_name((*param).sp_die, (*param).function) {
            if dwarf_offdie(dbg, (*gl).cu_offset, (*param).cu_die).is_null() { return DWARF_CB_OK; }
            if !(*param).file.is_null() {
                let fname = die_get_decl_file((*param).sp_die);
                if fname.is_null() || strtailcmp((*param).file, fname) != 0 { return DWARF_CB_OK; }
            }
            (*param).found = 1;
            return DWARF_CB_ABORT;
        }
    }
    DWARF_CB_OK
}

unsafe fn debuginfo__find_probe_location(dbg: *mut debuginfo, pf: *mut probe_finder) -> c_int {
    let pp = &mut (*(*pf).pev).point as *mut perf_probe_point;
    let mut off: Dwarf_Off = 0;
    let mut noff: Dwarf_Off = 0;
    let mut cuhl: size_t = 0;
    let mut ret = 0;
    (*pf).lcache = intlist__new(ptr::null_mut());
    if (*pf).lcache.is_null() { return -ENOMEM; }
    if !(*pp).function.is_null() && !strisglob((*pp).function) {
        let mut pubname_param = pubname_callback_param { function: (*pp).function, file: (*pp).file, cu_die: &mut (*pf).cu_die, sp_die: &mut (*pf).sp_die, found: 0 };
        let mut probe_param = dwarf_callback_param { data: pf as *mut c_void, retval: 0 };
        dwarf_getpubnames((*dbg).dbg, pubname_search_cb, &mut pubname_param as *mut _ as *mut c_void, 0);
        if pubname_param.found != 0 {
            ret = probe_point_search_cb(&mut (*pf).sp_die, &mut probe_param as *mut _ as *mut c_void);
            if ret != 0 { goto_found_probe_location(pf); return ret; }
        }
    }
    while dwarf_nextcu((*dbg).dbg, off, &mut noff, &mut cuhl, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()) == 0 {
        let diep = dwarf_offdie((*dbg).dbg, off + cuhl as u64, &mut (*pf).cu_die);
        if diep.is_null() { off = noff; continue; }
        (*pf).fname = if !(*pp).file.is_null() { cu_find_realpath(&mut (*pf).cu_die, (*pp).file) } else { ptr::null() };
        if (*pp).file.is_null() || !(*pf).fname.is_null() {
            if !(*pp).function.is_null() { ret = find_probe_point_by_func(pf); }
            else if !(*pp).lazy_line.is_null() { ret = find_probe_point_lazy(&mut (*pf).cu_die, pf); }
            else { (*pf).lno = (*pp).line; ret = find_probe_point_by_line(pf); }
            if ret < 0 { break; }
        }
        off = noff;
    }
    intlist__delete((*pf).lcache);
    (*pf).lcache = ptr::null_mut();
    ret
}

unsafe fn goto_found_probe_location(pf: *mut probe_finder) {
    intlist__delete((*pf).lcache);
    (*pf).lcache = ptr::null_mut();
}

unsafe fn debuginfo__find_probes(dbg: *mut debuginfo, pf: *mut probe_finder) -> c_int {
    if !(*pf).cfi_eh.is_null() || !(*pf).cfi_dbg.is_null() { return debuginfo__find_probe_location(dbg, pf); }
    let elf = dwarf_getelf((*dbg).dbg);
    if elf.is_null() { return -EINVAL; }
    let mut ehdr: GElf_Ehdr = core::mem::zeroed();
    if gelf_getehdr(elf, &mut ehdr).is_null() { return -EINVAL; }
    (*pf).e_machine = ehdr.e_machine;
    (*pf).e_flags = ehdr.e_flags;
    let mut shdr: GElf_Shdr = core::mem::zeroed();
    if elf_section_by_name(elf, &mut ehdr, &mut shdr, cstr(b".eh_frame\0"), ptr::null_mut()) && shdr.sh_type == SHT_PROGBITS {
        (*pf).cfi_eh = dwarf_getcfi_elf(elf);
    }
    (*pf).cfi_dbg = dwarf_getcfi((*dbg).dbg);
    debuginfo__find_probe_location(dbg, pf)
}

#[repr(C)] struct local_vars_finder { pf: *mut probe_finder, args: *mut perf_probe_arg, vars: bool, max_args: c_int, nargs: c_int, ret: c_int }

unsafe extern "C" fn copy_variables_cb(die_mem: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let vf = data as *mut local_vars_finder;
    let pf = (*vf).pf;
    let tag = dwarf_tag(die_mem);
    let mut attr: Dwarf_Attribute = core::mem::zeroed();
    let mut var_die: Dwarf_Die = core::mem::zeroed();
    if tag == DW_TAG_formal_parameter || (tag == DW_TAG_variable && (*vf).vars) {
        if convert_variable_location(die_mem, (*pf).addr, (*pf).fb_ops, &mut (*pf).sp_die, pf, ptr::null_mut()) == 0 {
            (*(*vf).args.add((*vf).nargs as usize)).var = dwarf_diename(die_mem) as *mut c_char;
            if (*(*vf).args.add((*vf).nargs as usize)).var.is_null() {
                (*vf).ret = -ENOMEM;
                return DIE_FIND_CB_END;
            }
            pr_debug(cstr(b" %s\0"), (*(*vf).args.add((*vf).nargs as usize)).var);
            (*vf).nargs += 1;
        }
    }
    if dwarf_haspc(die_mem, (*pf).addr) {
        if !dwarf_attr(die_mem, DW_AT_abstract_origin, &mut attr).is_null() {
            if dwarf_formref_die(&mut attr, &mut var_die).is_null() { return DIE_FIND_CB_SIBLING; }
            if (*pf).abstrace_dieoffset != dwarf_dieoffset(&mut var_die) { return DIE_FIND_CB_SIBLING; }
        }
        return DIE_FIND_CB_CONTINUE;
    }
    DIE_FIND_CB_SIBLING
}

unsafe fn expand_probe_args(sc_die: *mut Dwarf_Die, pf: *mut probe_finder, args: *mut perf_probe_arg) -> c_int {
    let mut die_mem: Dwarf_Die = core::mem::zeroed();
    let mut n = 0;
    let mut vf = local_vars_finder { pf, args, vars: false, max_args: MAX_PROBE_ARGS as c_int, nargs: 0, ret: 0 };
    for i in 0..(*(*pf).pev).nargs {
        let src = (*(*(*pf).pev).args.add(i as usize)).var;
        if strcmp(src, PROBE_ARG_VARS) == 0 { vf.vars = true; }
        else if strcmp(src, PROBE_ARG_PARAMS) != 0 {
            ptr::copy_nonoverlapping((*(*pf).pev).args.add(i as usize), args.add(n as usize), 1);
            n += 1;
            continue;
        }
        pr_debug(cstr(b"Expanding %s into:\0"), src);
        vf.nargs = n;
        die_find_child(sc_die, copy_variables_cb, &mut vf as *mut _ as *mut c_void, &mut die_mem);
        pr_debug(cstr(b" (%d)\n\0"), vf.nargs - n);
        if vf.ret < 0 { return vf.ret; }
        n = vf.nargs;
    }
    n
}

unsafe fn trace_event_finder_overlap(tf: *mut trace_event_finder) -> bool {
    for i in 0..(*tf).ntevs {
        if (*tf).pf.addr == (*(*tf).tevs.add(i as usize)).point.address { return true; }
    }
    false
}

unsafe extern "C" fn add_probe_trace_event(sc_die: *mut Dwarf_Die, pf: *mut probe_finder) -> c_int {
    let tf = pf as *mut trace_event_finder;
    let pp = &mut (*(*pf).pev).point as *mut perf_probe_point;
    let mut args: *mut perf_probe_arg = ptr::null_mut();
    if trace_event_finder_overlap(tf) { return 0; }
    if (*tf).ntevs == (*tf).max_tevs {
        pr_warning(cstr(b"Too many( > %d) probe point found.\n\0"), (*tf).max_tevs);
        return -ERANGE;
    }
    let tev = (*tf).tevs.add((*tf).ntevs as usize);
    (*tf).ntevs += 1;
    let mut ret = convert_to_trace_point(&mut (*pf).sp_die, (*tf).mod_, (*pf).addr, (*pp).retprobe, (*pp).function, &mut (*tev).point);
    if ret >= 0 {
        (*tev).point.realname = strdup(die_name(sc_die));
        if (*tev).point.realname.is_null() { ret = -ENOMEM; }
    }
    if ret >= 0 {
        (*tev).lang = if !dwarf_diecu(sc_die, &mut (*pf).cu_die, ptr::null_mut(), ptr::null_mut()).is_null() { dwarf_srclang(&mut (*pf).cu_die) } else { DW_LANG_C };
        pr_debug(cstr(b"Probe point found: %s+%lu\n\0"), (*tev).point.symbol, (*tev).point.offset);
        args = calloc(MAX_PROBE_ARGS, size_of::<perf_probe_arg>()) as *mut perf_probe_arg;
        if args.is_null() { ret = -ENOMEM; }
    }
    if ret >= 0 {
        ret = expand_probe_args(sc_die, pf, args);
        if ret >= 0 {
            (*tev).nargs = ret;
            (*tev).args = calloc((*tev).nargs as size_t, size_of::<probe_trace_arg>()) as *mut probe_trace_arg;
            if (*tev).args.is_null() { ret = -ENOMEM; }
        }
    }
    if ret >= 0 {
        for i in 0..(*tev).nargs {
            (*pf).pvar = args.add(i as usize);
            (*pf).tvar = (*tev).args.add(i as usize);
            ret = find_variable(sc_die, pf);
            if ret != 0 { break; }
        }
    }
    if ret != 0 {
        clear_probe_trace_event(tev);
        (*tf).ntevs -= 1;
    }
    free(args as *mut c_void);
    ret
}

unsafe fn fill_empty_trace_arg(pev: *mut perf_probe_event, tevs: *mut probe_trace_event, ntevs: c_int) -> c_int {
    if ntevs == 0 { return -ENOENT; }
    for i in 0..(*pev).nargs {
        let mut type_: *mut c_char = ptr::null_mut();
        let mut j = 0;
        while j < ntevs {
            if !(*(*tevs.add(j as usize)).args.add(i as usize)).value.is_null() {
                type_ = (*(*tevs.add(j as usize)).args.add(i as usize)).type_;
                break;
            }
            j += 1;
        }
        if j == ntevs {
            print_var_not_found((*(*pev).args.add(i as usize)).var);
            return -ENOENT;
        }
        for j2 in 0..ntevs {
            let valp = &mut (*(*tevs.add(j2 as usize)).args.add(i as usize)).value as *mut *mut c_char;
            if !(*valp).is_null() { continue; }
            let ret = asprintf(valp, cstr(b"\\%lx\0"), probe_conf.magic_num);
            if ret < 0 { return -ENOMEM; }
            if !type_.is_null() {
                (*(*tevs.add(j2 as usize)).args.add(i as usize)).type_ = strdup(type_);
                if (*(*tevs.add(j2 as usize)).args.add(i as usize)).type_.is_null() { return -ENOMEM; }
            }
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn debuginfo__find_trace_events(dbg: *mut debuginfo, pev: *mut perf_probe_event, tevs: *mut *mut probe_trace_event) -> c_int {
    let mut tf: trace_event_finder = core::mem::zeroed();
    tf.pf.pev = pev; tf.pf.dbg = dbg; tf.pf.callback = Some(add_probe_trace_event);
    tf.max_tevs = probe_conf.max_probes; tf.mod_ = (*dbg).mod_;
    *tevs = calloc(tf.max_tevs as size_t, size_of::<probe_trace_event>()) as *mut probe_trace_event;
    if (*tevs).is_null() { return -ENOMEM; }
    tf.tevs = *tevs;
    if (*pev).nargs != 0 && immediate_value_is_supported() { tf.pf.skip_empty_arg = true; }
    let mut ret = debuginfo__find_probes(dbg, &mut tf.pf);
    if ret >= 0 && tf.pf.skip_empty_arg { ret = fill_empty_trace_arg(pev, tf.tevs, tf.ntevs); }
    dwarf_cfi_end(tf.pf.cfi_eh);
    if ret < 0 || tf.ntevs == 0 {
        for i in 0..tf.ntevs { clear_probe_trace_event(tf.tevs.add(i as usize)); }
        zfree(tevs);
        return ret;
    }
    if ret < 0 { ret } else { tf.ntevs }
}

unsafe extern "C" fn collect_variables_cb(die_mem: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let af = data as *mut available_var_finder;
    let vl = (*af).vls.add(((*af).nvls - 1) as usize);
    let mut buf = strbuf { buf: ptr::null_mut(), len: 0, alloc: 0 };
    let tag = dwarf_tag(die_mem);
    if tag == DW_TAG_formal_parameter || tag == DW_TAG_variable {
        let ret = convert_variable_location(die_mem, (*af).pf.addr, (*af).pf.fb_ops, &mut (*af).pf.sp_die, &mut (*af).pf, ptr::null_mut());
        if ret == 0 || ret == -ERANGE {
            let externs = !(*af).child;
            if strbuf_init(&mut buf, 64) < 0 { goto_collect_error(&mut buf); return DIE_FIND_CB_END; }
            if probe_conf.show_location_range {
                let ret2 = if !externs { strbuf_add(&mut buf, if ret != 0 { cstr(b"[INV]\t\0") } else { cstr(b"[VAL]\t\0") }, 6) } else { strbuf_add(&mut buf, cstr(b"[EXT]\t\0"), 6) };
                if ret2 != 0 { goto_collect_error(&mut buf); return DIE_FIND_CB_END; }
            }
            let mut ret2 = die_get_varname(die_mem, &mut buf);
            if ret2 == 0 && probe_conf.show_location_range && !externs {
                if strbuf_addch(&mut buf, b'\t' as c_int) < 0 { goto_collect_error(&mut buf); return DIE_FIND_CB_END; }
                ret2 = die_get_var_range(&mut (*af).pf.sp_die, die_mem, &mut buf);
            }
            pr_debug(cstr(b"Add new var: %s\n\0"), buf.buf);
            if ret2 == 0 { strlist__add((*vl).vars, strbuf_detach(&mut buf, ptr::null_mut())); }
            strbuf_release(&mut buf);
        }
    }
    if (*af).child && dwarf_haspc(die_mem, (*af).pf.addr) { DIE_FIND_CB_CONTINUE } else { DIE_FIND_CB_SIBLING }
}

unsafe fn goto_collect_error(buf: *mut strbuf) {
    strbuf_release(buf);
    pr_debug(cstr(b"Error in strbuf\n\0"));
}

unsafe fn available_var_finder_overlap(af: *mut available_var_finder) -> bool {
    for i in 0..(*af).nvls {
        if (*af).pf.addr == (*(*af).vls.add(i as usize)).point.address { return true; }
    }
    false
}

unsafe extern "C" fn add_available_vars(sc_die: *mut Dwarf_Die, pf: *mut probe_finder) -> c_int {
    let af = pf as *mut available_var_finder;
    let pp = &mut (*(*pf).pev).point as *mut perf_probe_point;
    let mut die_mem: Dwarf_Die = core::mem::zeroed();
    if available_var_finder_overlap(af) { return 0; }
    if (*af).nvls == (*af).max_vls {
        pr_warning(cstr(b"Too many( > %d) probe point found.\n\0"), (*af).max_vls);
        return -ERANGE;
    }
    let vl = (*af).vls.add((*af).nvls as usize);
    (*af).nvls += 1;
    let ret = convert_to_trace_point(&mut (*pf).sp_die, (*af).mod_, (*pf).addr, (*pp).retprobe, (*pp).function, &mut (*vl).point);
    if ret < 0 { return ret; }
    pr_debug(cstr(b"Probe point found: %s+%lu\n\0"), (*vl).point.symbol, (*vl).point.offset);
    (*vl).vars = strlist__new(ptr::null_mut(), ptr::null_mut());
    if (*vl).vars.is_null() { return -ENOMEM; }
    (*af).child = true;
    die_find_child(sc_die, collect_variables_cb, af as *mut c_void, &mut die_mem);
    if !probe_conf.show_ext_vars { } else {
        (*af).child = false;
        die_find_child(&mut (*pf).cu_die, collect_variables_cb, af as *mut c_void, &mut die_mem);
    }
    if strlist__empty((*vl).vars) {
        strlist__delete((*vl).vars);
        (*vl).vars = ptr::null_mut();
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn debuginfo__find_available_vars_at(dbg: *mut debuginfo, pev: *mut perf_probe_event, vls: *mut *mut variable_list) -> c_int {
    let mut af: available_var_finder = core::mem::zeroed();
    af.pf.pev = pev; af.pf.dbg = dbg; af.pf.callback = Some(add_available_vars);
    af.mod_ = (*dbg).mod_; af.max_vls = probe_conf.max_probes;
    *vls = calloc(af.max_vls as size_t, size_of::<variable_list>()) as *mut variable_list;
    if (*vls).is_null() { return -ENOMEM; }
    af.vls = *vls;
    let ret = debuginfo__find_probes(dbg, &mut af.pf);
    if ret < 0 {
        while af.nvls != 0 {
            af.nvls -= 1;
            zfree(&mut (*af.vls.add(af.nvls as usize)).point.symbol);
            strlist__delete((*af.vls.add(af.nvls as usize)).vars);
        }
        zfree(vls);
        return ret;
    }
    if ret < 0 { ret } else { af.nvls }
}

#[no_mangle]
pub unsafe extern "C" fn debuginfo__find_probe_point(dbg: *mut debuginfo, mut addr: u64, ppt: *mut perf_probe_point) -> c_int {
    let mut cudie: Dwarf_Die = core::mem::zeroed();
    let mut spdie: Dwarf_Die = core::mem::zeroed();
    let mut indie: Dwarf_Die = core::mem::zeroed();
    let mut _addr: Dwarf_Addr = 0;
    let mut baseaddr: Dwarf_Addr = 0;
    let mut fname: *const c_char = ptr::null();
    let mut func: *const c_char = ptr::null();
    let mut basefunc: *const c_char = ptr::null();
    let mut baseline = 0;
    let mut lineno = 0;
    let mut ret = 0;
    if debuginfo__get_text_offset(dbg, &mut baseaddr, false) == 0 { addr += baseaddr; }
    if dwarf_addrdie((*dbg).dbg, addr as Dwarf_Addr, &mut cudie).is_null() {
        pr_warning(cstr(b"Failed to find debug information for address %#lx\n\0"), addr);
        ret = -EINVAL;
    } else {
        cu_find_lineinfo(&mut cudie, addr as Dwarf_Addr, &mut fname, &mut lineno);
        if die_find_realfunc(&mut cudie, addr as Dwarf_Addr, &mut spdie) {
            func = die_get_linkage_name(&mut spdie); basefunc = func;
            if func.is_null() { func = dwarf_diename(&mut spdie); basefunc = func; }
            if func.is_null() || die_entrypc(&mut spdie, &mut baseaddr) != 0 || dwarf_decl_line(&mut spdie, &mut baseline) != 0 { lineno = 0; }
            else {
                fname = die_get_decl_file(&mut spdie);
                if addr == baseaddr {
                    lineno = baseline;
                } else {
                    while die_find_top_inlinefunc(&mut spdie, addr as Dwarf_Addr, &mut indie) {
                        if die_entrypc(&mut indie, &mut _addr) == 0 && _addr == addr {
                            lineno = die_get_call_lineno(&mut indie);
                            fname = die_get_call_file(&mut indie);
                            break;
                        } else {
                            let tmp = dwarf_diename(&mut indie);
                            if tmp.is_null() || dwarf_decl_line(&mut indie, &mut baseline) != 0 { break; }
                            func = tmp;
                            memcpy(&mut spdie as *mut _ as *mut c_void, &indie as *const _ as *const c_void, size_of::<Dwarf_Die>());
                        }
                    }
                    let tmp = die_get_decl_file(&mut spdie);
                    if tmp.is_null() || (!fname.is_null() && strcmp(tmp, fname) != 0) { lineno = 0; }
                }
            }
        }
        if lineno != 0 { (*ppt).line = lineno - baseline; }
        else if !basefunc.is_null() { (*ppt).offset = addr - baseaddr; func = basefunc; }
        if !func.is_null() {
            (*ppt).function = strdup(func);
            if (*ppt).function.is_null() { ret = -ENOMEM; }
        }
        if ret == 0 && !fname.is_null() {
            (*ppt).file = strdup(fname);
            if (*ppt).file.is_null() {
                zfree(&mut (*ppt).function);
                ret = -ENOMEM;
            }
        }
    }
    if ret == 0 && (!fname.is_null() || !func.is_null()) { ret = 1; }
    ret
}

unsafe fn line_range_add_line(src: *const c_char, lineno: c_uint, lr: *mut line_range) -> c_int {
    if (*lr).path.is_null() {
        (*lr).path = strdup(src);
        if (*lr).path.is_null() { return -ENOMEM; }
    }
    intlist__add((*lr).line_list, lineno as c_int)
}

unsafe extern "C" fn line_range_walk_cb(fname: *const c_char, lineno: c_int, addr: Dwarf_Addr, data: *mut c_void) -> c_int {
    let lf = data as *mut line_finder;
    let mut __fname: *const c_char = ptr::null();
    let mut __lineno = 0;
    if strtailcmp(fname, (*lf).fname) != 0 || ((*lf).lno_s > lineno || (*lf).lno_e < lineno) { return 0; }
    if cu_find_lineinfo(&mut (*lf).cu_die, addr, &mut __fname, &mut __lineno) > 0 && (lineno != __lineno || strcmp(fname, __fname) != 0) { return 0; }
    let err = line_range_add_line(fname, lineno as c_uint, (*lf).lr);
    if err < 0 && err != -EEXIST { return err; }
    0
}

unsafe fn find_line_range_by_line(sp_die: *mut Dwarf_Die, lf: *mut line_finder) -> c_int {
    let target = if sp_die.is_null() { &mut (*lf).cu_die } else { sp_die };
    let mut ret = die_walk_lines(target, line_range_walk_cb, lf as *mut c_void);
    if ret >= 0 {
        if !intlist__empty((*(*lf).lr).line_list) { (*lf).found = 1; ret = 1; } else { ret = 0; }
    } else {
        zfree(&mut (*(*lf).lr).path);
    }
    ret
}

unsafe extern "C" fn line_range_inline_cb(in_die: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let ret = find_line_range_by_line(in_die, data as *mut line_finder);
    if ret < 0 { ret } else { 0 }
}

unsafe extern "C" fn line_range_search_cb(sp_die: *mut Dwarf_Die, data: *mut c_void) -> c_int {
    let param = data as *mut dwarf_callback_param;
    let lf = (*param).data as *mut line_finder;
    let lr = (*lf).lr;
    if !(*lr).file.is_null() {
        let fname = die_get_decl_file(sp_die);
        if fname.is_null() || strtailcmp((*lr).file, fname) != 0 { return DWARF_CB_OK; }
    }
    if die_match_name(sp_die, (*lr).function) && die_is_func_def(sp_die) {
        (*lf).fname = die_get_decl_file(sp_die);
        if dwarf_decl_line(sp_die, &mut (*lr).offset) != 0 { (*lr).offset = 0; }
        pr_debug(cstr(b"fname: %s, lineno:%d\n\0"), (*lf).fname, (*lr).offset);
        (*lf).lno_s = (*lr).offset + (*lr).start;
        if (*lf).lno_s < 0 { (*lf).lno_s = c_int::MAX; }
        (*lf).lno_e = (*lr).offset + (*lr).end;
        if (*lf).lno_e < 0 { (*lf).lno_e = c_int::MAX; }
        pr_debug(cstr(b"New line range: %d to %d\n\0"), (*lf).lno_s, (*lf).lno_e);
        (*lr).start = (*lf).lno_s; (*lr).end = (*lf).lno_e;
        (*param).retval = if !die_is_func_instance(sp_die) { die_walk_instances(sp_die, line_range_inline_cb, lf as *mut c_void) } else { find_line_range_by_line(sp_die, lf) };
        return DWARF_CB_ABORT;
    }
    DWARF_CB_OK
}

unsafe fn find_line_range_by_func(lf: *mut line_finder) -> c_int {
    let mut param = dwarf_callback_param { data: lf as *mut c_void, retval: 0 };
    if dwarf_getfuncs(&mut (*lf).cu_die, line_range_search_cb, &mut param as *mut _ as *mut c_void, 0) < 0 {
        pr_debug(cstr(b"Failed to get functions from CU\n\0"));
    }
    param.retval
}

#[no_mangle]
pub unsafe extern "C" fn debuginfo__find_line_range(dbg: *mut debuginfo, lr: *mut line_range) -> c_int {
    let mut lf: line_finder = core::mem::zeroed();
    lf.lr = lr;
    let mut ret = 0;
    let mut off: Dwarf_Off = 0;
    let mut noff: Dwarf_Off = 0;
    let mut cuhl: size_t = 0;
    if !(*lr).function.is_null() {
        let mut pubname_param = pubname_callback_param { function: (*lr).function, file: (*lr).file, cu_die: &mut lf.cu_die, sp_die: &mut lf.sp_die, found: 0 };
        let mut line_range_param = dwarf_callback_param { data: &mut lf as *mut _ as *mut c_void, retval: 0 };
        dwarf_getpubnames((*dbg).dbg, pubname_search_cb, &mut pubname_param as *mut _ as *mut c_void, 0);
        if pubname_param.found != 0 {
            line_range_search_cb(&mut lf.sp_die, &mut line_range_param as *mut _ as *mut c_void);
        }
    }
    while lf.found == 0 && ret >= 0 {
        if dwarf_nextcu((*dbg).dbg, off, &mut noff, &mut cuhl, ptr::null_mut(), ptr::null_mut(), ptr::null_mut()) != 0 { break; }
        let diep = dwarf_offdie((*dbg).dbg, off + cuhl as u64, &mut lf.cu_die);
        if diep.is_null() { off = noff; continue; }
        lf.fname = if !(*lr).file.is_null() { cu_find_realpath(&mut lf.cu_die, (*lr).file) } else { ptr::null() };
        if (*lr).file.is_null() || !lf.fname.is_null() {
            if !(*lr).function.is_null() { ret = find_line_range_by_func(&mut lf); }
            else { lf.lno_s = (*lr).start; lf.lno_e = (*lr).end; ret = find_line_range_by_line(ptr::null_mut(), &mut lf); }
        }
        off = noff;
    }
    if lf.found != 0 {
        let comp_dir = cu_get_comp_dir(&mut lf.cu_die);
        if !comp_dir.is_null() {
            (*lr).comp_dir = strdup(comp_dir);
            if (*lr).comp_dir.is_null() { ret = -ENOMEM; }
        }
    }
    pr_debug(cstr(b"path: %s\n\0"), (*lr).path);
    if ret < 0 { ret } else { lf.found }
}

#[no_mangle]
pub unsafe extern "C" fn find_source_path(raw_path_in: *const c_char, sbuild_id: *const c_char, comp_dir: *const c_char, new_path: *mut *mut c_char) -> c_int {
    let mut raw_path = raw_path_in;
    let mut prefix = symbol_conf.source_prefix;
    if !sbuild_id.is_null() && prefix.is_null() {
        let mut prefixed_raw_path = [0i8; PATH_MAX];
        path__join(prefixed_raw_path.as_mut_ptr(), prefixed_raw_path.len(), comp_dir, raw_path);
        if get_source_from_debuginfod(prefixed_raw_path.as_mut_ptr(), sbuild_id, new_path) == 0 { return 0; }
    }
    if prefix.is_null() {
        if *raw_path != b'/' as c_char && !comp_dir.is_null() {
            prefix = comp_dir;
        } else if access(raw_path, R_OK) == 0 {
            *new_path = strdup(raw_path);
            return if !(*new_path).is_null() { 0 } else { -ENOMEM };
        } else {
            return -errno;
        }
    }
    *new_path = malloc(strlen(prefix) + strlen(raw_path) + 2) as *mut c_char;
    if (*new_path).is_null() { return -ENOMEM; }
    loop {
        sprintf(*new_path, cstr(b"%s/%s\0"), prefix, raw_path);
        if access(*new_path, R_OK) == 0 { return 0; }
        if symbol_conf.source_prefix.is_null() {
            zfree(new_path);
            return -errno;
        }
        match errno {
            ENAMETOOLONG | ENOENT | EROFS | EFAULT => {
                raw_path = strchr(raw_path.add(1), b'/' as c_int);
                if raw_path.is_null() {
                    zfree(new_path);
                    return -ENOENT;
                }
            }
            _ => {
                zfree(new_path);
                return -errno;
            }
        }
    }
}
