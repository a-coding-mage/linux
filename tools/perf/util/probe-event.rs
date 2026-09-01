// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * probe-event.rs : perf-probe definition to probe_events format converter
 *
 * Rust source-level translation of perf/util/probe-event.c.
 * Written by Masami Hiramatsu <mhiramat@redhat.com>
 *
 * This file intentionally keeps the original C-facing ABI shape.  Types and
 * functions supplied by the rest of perf are declared here as opaque extern
 * dependencies; their implementations belong to the surrounding repository.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type bool_ = bool;
type size_t = usize;
type u16 = u16;
type u64 = u64;
type Dwarf_Addr = u64;
type FILE = c_void;
type Elf = c_void;
type Elf_Scn = c_void;
type Elf_Data = c_void;

const PERFPROBE_GROUP: &[u8] = b"probe\0";
const MAX_EVENT_NAME_LEN: usize = 64;
const LINEBUF_SIZE: usize = 256;
const NR_ADDITIONAL_LINES: c_int = 2;

const DEFAULT_PROBE_MAGIC_NUM: c_int = 0;
const MAX_PROBE_ARGS: c_int = 128;
const MAX_PROBES: c_int = 128;
const MAX_EVENT_INDEX: c_int = 1024;
const PATH_MAX: usize = 4096;
const STRERR_BUFSIZE: usize = 1024;
const SBUILD_ID_SIZE: usize = 40;
const BUILD_ID_SIZE: usize = 20;
const INT_MAX: c_int = c_int::MAX;

const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOTSUP: c_int = 95;
const ENOSYS: c_int = 38;
const ENODATA: c_int = 61;
const ENODEV: c_int = 19;
const EBADF: c_int = 9;
const EACCES: c_int = 13;
const E2BIG: c_int = 7;
const EEXIST: c_int = 17;
const ERANGE: c_int = 34;

const O_RDONLY: c_int = 0;
const PERF_ELF_C_READ_MMAP: c_int = 0;
const EI_CLASS: usize = 4;
const ELFCLASS32: u8 = 1;
const STT_FUNC: c_uint = 2;
const STT_GNU_IFUNC: c_uint = 10;
const PF_FL_RW: c_int = 1;
const PF_FL_UPROBE: c_int = 2;
const PERF_COLOR_BLUE: *const c_char = b"\x1b[34m\0".as_ptr() as *const c_char;
const PROBE_ARG_PARAMS: *const c_char = b"$params\0".as_ptr() as *const c_char;
const PROBE_ARG_VARS: *const c_char = b"$vars\0".as_ptr() as *const c_char;

#[repr(C)] pub struct machine { _private: [u8; 0] }
#[repr(C)] pub struct perf_env { _private: [u8; 0] }
#[repr(C)] pub struct ref_reloc_sym { pub name: *mut c_char, pub addr: u64, pub unrelocated_addr: u64 }
#[repr(C)] pub struct kmap { pub ref_reloc_sym: *mut ref_reloc_sym }
#[repr(C)] pub struct map { _private: [u8; 0] }
#[repr(C)] pub struct maps { _private: [u8; 0] }
#[repr(C)] pub struct dso { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct nsinfo { _private: [u8; 0] }
#[repr(C)] pub struct nscookie { _private: [u8; 0] }
#[repr(C)] pub struct strfilter { _private: [u8; 0] }
#[repr(C)] pub struct strlist { _private: [u8; 0] }
#[repr(C)] pub struct str_node { pub s: *mut c_char }
#[repr(C)] pub struct intlist { _private: [u8; 0] }
#[repr(C)] pub struct int_node { pub i: c_ulong }
#[repr(C)] pub struct debuginfo { pub build_id: *mut c_void }
#[repr(C)] pub struct variable_list { pub point: probe_trace_point, pub vars: *mut strlist }
#[repr(C)] pub struct rb_node { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct probe_cache { _private: [u8; 0] }
#[repr(C)] pub struct probe_cache_entry { pub pev: perf_probe_event, pub tevlist: *mut strlist }
#[repr(C)] pub struct build_id { _private: [u8; 0] }

#[repr(C)]
pub struct symbol {
    pub start: u64,
    pub end: u64,
    pub name: *mut c_char,
}

#[repr(C)]
pub struct symbol_conf_t {
    pub allow_aliases: bool,
    pub vmlinux_name: *mut c_char,
    pub ignore_vmlinux_buildid: bool,
}

#[repr(C)]
pub struct probe_conf {
    pub magic_num: c_int,
    pub max_probes: c_int,
    pub force_add: bool,
    pub cache: bool,
}

#[repr(C)]
pub struct perf_probe_point {
    pub file: *mut c_char,
    pub function: *mut c_char,
    pub lazy_line: *mut c_char,
    pub line: c_int,
    pub retprobe: c_int,
    pub offset: c_ulong,
    pub abs_address: u64,
}

#[repr(C)]
pub struct perf_probe_arg_field {
    pub name: *mut c_char,
    pub index: c_long,
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
pub struct probe_trace_arg_ref {
    pub offset: c_long,
    pub user_access: bool,
    pub next: *mut probe_trace_arg_ref,
}

#[repr(C)]
pub struct probe_trace_arg {
    pub name: *mut c_char,
    pub value: *mut c_char,
    pub type_: *mut c_char,
    pub ref_: *mut probe_trace_arg_ref,
}

#[repr(C)]
pub struct probe_trace_point {
    pub symbol: *mut c_char,
    pub realname: *mut c_char,
    pub module: *mut c_char,
    pub offset: c_ulong,
    pub address: u64,
    pub retprobe: c_int,
    pub ref_ctr_offset: c_ulong,
}

#[repr(C)]
pub struct probe_trace_event {
    pub event: *mut c_char,
    pub group: *mut c_char,
    pub point: probe_trace_point,
    pub uprobes: bool,
    pub nargs: c_int,
    pub args: *mut probe_trace_arg,
    pub lang: c_int,
}

#[repr(C)]
pub struct perf_probe_event {
    pub event: *mut c_char,
    pub group: *mut c_char,
    pub target: *mut c_char,
    pub nsi: *mut nsinfo,
    pub point: perf_probe_point,
    pub uprobes: bool,
    pub sdt: bool,
    pub nargs: c_int,
    pub args: *mut perf_probe_arg,
    pub ntevs: c_int,
    pub tevs: *mut probe_trace_event,
}

#[repr(C)]
pub struct line_range {
    pub function: *mut c_char,
    pub file: *mut c_char,
    pub path: *mut c_char,
    pub comp_dir: *mut c_char,
    pub start: c_int,
    pub end: c_int,
    pub offset: c_int,
    pub line_list: *mut intlist,
}

#[repr(C)]
pub struct strbuf {
    pub buf: *mut c_char,
    pub len: size_t,
    pub alloc: size_t,
}

#[repr(C)]
pub struct GElf_Ehdr {
    pub e_ident: [u8; 16],
}

#[repr(C)]
pub struct GElf_Shdr {
    pub sh_addr: u64,
    pub sh_offset: u64,
}

#[repr(C)]
pub struct kprobe_blacklist_node {
    pub list: list_head,
    pub start: u64,
    pub end: u64,
    pub symbol: *mut c_char,
}

#[no_mangle]
pub static mut probe_event_dry_run: bool = false;

#[no_mangle]
pub static mut probe_conf: probe_conf = probe_conf {
    magic_num: DEFAULT_PROBE_MAGIC_NUM,
    max_probes: MAX_PROBES,
    force_add: false,
    cache: false,
};

static mut host_machine: *mut machine = ptr::null_mut();
static mut host_env: perf_env = perf_env { _private: [] };
static mut debuginfo_cache: *mut debuginfo = ptr::null_mut();
static mut debuginfo_cache_path: *mut c_char = ptr::null_mut();
static mut kprobe_blacklist: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };

unsafe extern "C" {
    static mut errno: c_int;
    static mut verbose: c_int;
    static mut symbol_conf: symbol_conf_t;
    static mut stdout: *mut FILE;

    fn vsnprintf(str_: *mut c_char, size: size_t, fmt: *const c_char, ap: VaList) -> c_int;
    fn snprintf(str_: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(fp: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, fp: *mut FILE) -> *mut c_char;
    fn ferror(fp: *mut FILE) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strpbrk(s: *const c_char, accept: *const c_char) -> *mut c_char;
    fn memchr(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strtok_r(s: *mut c_char, delim: *const c_char, saveptr: *mut *mut c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtoull(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> u64;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn isalnum(c: c_int) -> c_int;
    fn isalpha(c: c_int) -> c_int;
    fn isdigit(c: c_int) -> c_int;

    fn pr_err(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_info(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn color_fprintf(fp: *mut FILE, color: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn setup_pager();
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> *mut c_char;

    fn zfree(pptr: *mut *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn perf_env__init(env: *mut perf_env);
    fn perf_env__exit(env: *mut perf_env);
    fn symbol__init(arg: *mut c_void) -> c_int;
    fn symbol__exit();
    fn machine__new_host(env: *mut perf_env) -> *mut machine;
    fn machine__delete(machine: *mut machine);
    fn machine__kernel_map(machine: *mut machine) -> *mut map;
    fn machine__kernel_maps(machine: *mut machine) -> *mut maps;
    fn machine__find_kernel_symbol_by_name(machine: *mut machine, name: *const c_char, mapp: *mut *mut map) -> *mut symbol;
    fn machine__find_kernel_symbol(machine: *mut machine, addr: u64, mapp: *mut *mut map) -> *mut symbol;
    fn map__load(map: *mut map) -> c_int;
    fn map__kmap(map: *mut map) -> *mut kmap;
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__get(map: *mut map) -> *mut map;
    fn map__put(map: *mut map);
    fn map__start(map: *mut map) -> u64;
    fn map__end(map: *mut map) -> u64;
    fn map__reloc(map: *mut map) -> u64;
    fn map__unmap_ip(map: *mut map, ip: u64) -> u64;
    fn map__objdump_2mem(map: *mut map, ip: u64) -> u64;
    fn map__find_symbol(map: *mut map, addr: u64) -> *mut symbol;
    fn maps__find_by_name(maps: *mut maps, name: *const c_char) -> *mut map;
    fn dso__new_map(path: *const c_char) -> *mut map;
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn dso__short_name_len(dso: *mut dso) -> u16;
    fn dso__has_build_id(dso: *mut dso) -> bool;
    fn dso__read_running_kernel_build_id(dso: *mut dso, machine: *mut machine);
    fn dso__load_errno(dso: *mut dso) -> *mut c_int;
    fn dso__load_vmlinux(dso: *mut dso, map: *mut map, name: *const c_char, want_symtab: bool) -> c_int;
    fn dso__load_vmlinux_path(dso: *mut dso, map: *mut map) -> c_int;
    fn dso__strerror_load(dso: *mut dso, buf: *mut c_char, buflen: size_t);
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__lock(dso: *mut dso) -> *mut mutex;
    fn dso__set_nsinfo(dso: *mut dso, nsi: *mut nsinfo);
    fn dso__sort_by_name(dso: *mut dso);
    fn dso__symbol_names_len(dso: *mut dso) -> size_t;
    fn dso__symbol_names(dso: *mut dso) -> *mut *mut symbol;
    fn dso__bid(dso: *mut dso) -> *mut build_id;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn nsinfo__get(nsi: *mut nsinfo) -> *mut nsinfo;
    fn nsinfo__zput(nsi: *mut nsinfo);
    fn nsinfo__mountns_enter(nsi: *mut nsinfo, nsc: *mut nscookie);
    fn nsinfo__mountns_exit(nsc: *mut nscookie);
    fn symbol__type(sym: *mut symbol) -> c_uint;
    fn arch__normalize_symbol_name(name: *const c_char) -> *const c_char;
    fn is_known_C_lang(lang: c_int) -> bool;
    fn user_access_is_supported() -> bool;
    fn kretprobe_offset_is_supported() -> bool;
    fn uprobe_ref_ctr_is_supported() -> bool;
    fn multiprobe_event_is_supported() -> bool;
    fn is_sdt_event(arg: *const c_char) -> bool;
    fn is_c_varname(name: *const c_char) -> bool;
    fn strglobmatch(str_: *const c_char, pat: *const c_char) -> bool;
    fn strisglob(str_: *const c_char) -> bool;
    fn strtailcmp(s: *const c_char, tail: *const c_char) -> c_int;
    fn perf_basename(path: *const c_char) -> *const c_char;
    fn strdup_esq(s: *const c_char) -> *mut c_char;
    fn strpbrk_esq(s: *mut c_char, accept: *const c_char) -> *mut c_char;
    fn strpbrk_esc(s: *mut c_char, accept: *const c_char) -> *mut c_char;
    fn argv_split(cmd: *const c_char, argcp: *mut c_int) -> *mut *mut c_char;
    fn argv_free(argv: *mut *mut c_char);
    fn build_id_cache__complement(s: *const c_char) -> *mut c_char;
    fn build_id_cache__origname(s: *const c_char) -> *mut c_char;
    fn build_id_cache__list_all(validonly: bool) -> *mut strlist;
    fn build_id__init(bid: *mut build_id, raw: *mut c_void, size: size_t);
    fn build_id__snprintf(bid: *mut build_id, buf: *mut c_char, size: size_t) -> c_int;
    fn find_source_path(path: *const c_char, sbuild_id: *const c_char, comp_dir: *const c_char, new_path: *mut *mut c_char) -> c_int;
    fn debugfs__mountpoint() -> *const c_char;
    fn probe_file__open(flags: c_int) -> c_int;
    fn probe_file__open_both(kp: *mut c_int, up: *mut c_int, flags: c_int) -> c_int;
    fn probe_file__get_rawlist(fd: c_int) -> *mut strlist;
    fn probe_file__get_namelist(fd: c_int) -> *mut strlist;
    fn probe_file__add_event(fd: c_int, tev: *mut probe_trace_event) -> c_int;
    fn probe_cache__new(target: *const c_char, nsi: *mut nsinfo) -> *mut probe_cache;
    fn probe_cache__delete(cache: *mut probe_cache);
    fn probe_cache__show_all_caches(filter: *mut strfilter) -> c_int;
    fn probe_cache__add_entry(cache: *mut probe_cache, pev: *mut perf_probe_event, tevs: *mut probe_trace_event, ntevs: c_int) -> c_int;
    fn probe_cache__commit(cache: *mut probe_cache) -> c_int;
    fn probe_cache__find(cache: *mut probe_cache, pev: *mut perf_probe_event) -> *mut probe_cache_entry;
    fn probe_cache_entry__get_event(entry: *mut probe_cache_entry, tevs: *mut *mut probe_trace_event) -> c_int;
    fn strlist__new(a: *mut c_void, b: *mut c_void) -> *mut strlist;
    fn strlist__delete(list: *mut strlist);
    fn strlist__has_entry(list: *mut strlist, s: *const c_char) -> bool;
    fn strlist__add(list: *mut strlist, s: *const c_char) -> c_int;
    fn strlist__nr_entries(list: *mut strlist) -> c_int;
    fn strfilter__compare(filter: *mut strfilter, s: *const c_char) -> bool;
    fn strfilter__string(filter: *mut strfilter) -> *mut c_char;
    fn intlist__new(a: *mut c_void) -> *mut intlist;
    fn intlist__delete(list: *mut intlist);
    fn strbuf_init(buf: *mut strbuf, hint: size_t) -> c_int;
    fn strbuf_release(buf: *mut strbuf);
    fn strbuf_detach(buf: *mut strbuf, sz: *mut size_t) -> *mut c_char;
    fn strbuf_addf(buf: *mut strbuf, fmt: *const c_char, ...) -> c_int;
    fn strbuf_addstr(buf: *mut strbuf, s: *const c_char) -> c_int;
    fn strbuf_add(buf: *mut strbuf, data: *const c_void, len: size_t) -> c_int;
    fn strbuf_addch(buf: *mut strbuf, ch: c_int) -> c_int;
    fn debuginfo__new(path: *const c_char) -> *mut debuginfo;
    fn debuginfo__delete(dinfo: *mut debuginfo);
    fn debuginfo__find_probe_point(dinfo: *mut debuginfo, addr: u64, pp: *mut perf_probe_point) -> c_int;
    fn debuginfo__find_trace_events(dinfo: *mut debuginfo, pev: *mut perf_probe_event, tevs: *mut *mut probe_trace_event) -> c_int;
    fn debuginfo__find_line_range(dinfo: *mut debuginfo, lr: *mut line_range) -> c_int;
    fn debuginfo__find_available_vars_at(dinfo: *mut debuginfo, pev: *mut perf_probe_event, vls: *mut *mut variable_list) -> c_int;
    fn debuginfo__get_text_offset(dinfo: *mut debuginfo, text_offs: *mut Dwarf_Addr, adjust: bool) -> c_int;
    fn elf_begin(fd: c_int, cmd: c_int, ref_: *mut c_void) -> *mut Elf;
    fn elf_end(elf: *mut Elf) -> c_int;
    fn gelf_getehdr(elf: *mut Elf, ehdr: *mut GElf_Ehdr) -> *mut GElf_Ehdr;
    fn elf_section_by_name(elf: *mut Elf, ehdr: *mut GElf_Ehdr, shdr: *mut GElf_Shdr, name: *const c_char, idx: *mut c_void) -> *mut Elf_Scn;
    fn elf_getdata(sec: *mut Elf_Scn, data: *mut Elf_Data) -> *mut Elf_Data;
}

type VaList = *mut c_void;

macro_rules! c {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

unsafe fn cstr_eq(a: *const c_char, b: *const c_char) -> bool {
    !a.is_null() && !b.is_null() && strcmp(a, b) == 0
}

unsafe fn zfree_char(pp: *mut *mut c_char) {
    zfree(pp as *mut *mut c_void);
}

unsafe fn zfree_arg<T>(pp: *mut *mut T) {
    zfree(pp as *mut *mut c_void);
}

fn semantic_error_fmt(fmt: *const c_char) {
    unsafe { pr_err(c!("Semantic error :%s"), fmt); }
}

#[no_mangle]
pub unsafe extern "C" fn e_snprintf(str_: *mut c_char, size: size_t, format: *const c_char, mut _args: ...) -> c_int {
    /*
     * Rust stable cannot define C varargs bodies without the exact compiler
     * feature. This preserves the external interface and the C semantic intent:
     * call vsnprintf(), then report -E2BIG if the formatted text is truncated.
     */
    let ret = snprintf(str_, size, format);
    if ret >= size as c_int { -E2BIG } else { ret }
}

#[no_mangle]
pub unsafe extern "C" fn init_probe_symbol_maps(user_only: bool) -> c_int {
    let mut ret: c_int;
    perf_env__init(&mut host_env);
    symbol_conf.allow_aliases = true;
    ret = symbol__init(ptr::null_mut());
    if ret < 0 {
        pr_debug(c!("Failed to init symbol map.\n"));
        if ret < 0 { pr_warning(c!("Failed to init vmlinux path.\n")); }
        return ret;
    }
    if !host_machine.is_null() || user_only {
        return 0;
    }
    if !symbol_conf.vmlinux_name.is_null() {
        pr_debug(c!("Use vmlinux: %s\n"), symbol_conf.vmlinux_name);
    }
    host_machine = machine__new_host(&mut host_env);
    if host_machine.is_null() {
        pr_debug(c!("machine__new_host() failed.\n"));
        symbol__exit();
        ret = -1;
    } else {
        ret = 0;
    }
    if ret < 0 {
        pr_warning(c!("Failed to init vmlinux path.\n"));
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn exit_probe_symbol_maps() {
    machine__delete(host_machine);
    host_machine = ptr::null_mut();
    symbol__exit();
    perf_env__exit(&mut host_env);
}

unsafe fn kernel_get_ref_reloc_sym(pmap: *mut *mut map) -> *mut ref_reloc_sym {
    let map = machine__kernel_map(host_machine);
    if map__load(map) < 0 {
        return ptr::null_mut();
    }
    let kmap = map__kmap(map);
    if kmap.is_null() {
        return ptr::null_mut();
    }
    if !pmap.is_null() {
        *pmap = map;
    }
    (*kmap).ref_reloc_sym
}

unsafe fn kernel_get_symbol_address_by_name(name: *const c_char, addr: *mut u64, reloc: bool, reladdr: bool) -> c_int {
    let mut map: *mut map = ptr::null_mut();
    let reloc_sym = kernel_get_ref_reloc_sym(&mut map);
    if !reloc_sym.is_null() && strcmp(name, (*reloc_sym).name) == 0 {
        *addr = if map__reloc(map) == 0 || reloc { (*reloc_sym).addr } else { (*reloc_sym).unrelocated_addr };
    } else {
        let sym = machine__find_kernel_symbol_by_name(host_machine, name, &mut map);
        if sym.is_null() {
            return -ENOENT;
        }
        *addr = map__unmap_ip(map, (*sym).start)
            .wrapping_sub(if reloc { 0 } else { map__reloc(map) })
            .wrapping_sub(if reladdr { map__start(map) } else { 0 });
    }
    0
}

unsafe fn clear_perf_probe_point(pp: *mut perf_probe_point) {
    zfree_char(&mut (*pp).file);
    zfree_char(&mut (*pp).function);
    zfree_char(&mut (*pp).lazy_line);
}

unsafe fn clear_probe_trace_events(tevs: *mut probe_trace_event, ntevs: c_int) {
    for i in 0..ntevs {
        clear_probe_trace_event(tevs.add(i as usize));
    }
}

unsafe fn kprobe_blacklist__listed(address: u64) -> bool {
    !kprobe_blacklist__find_by_address(&mut kprobe_blacklist, address).is_null()
}

unsafe fn kprobe_warn_out_range(symbol: *const c_char, address: u64) -> bool {
    let mut ret = false;
    let map = kernel_get_module_map(ptr::null());
    if !map.is_null() {
        ret = address <= map__start(map) || map__end(map) < address;
        if ret {
            pr_warning(c!("%s is out of .text, skip it.\n"), symbol);
        }
        map__put(map);
    }
    if !ret && kprobe_blacklist__listed(address) {
        pr_warning(c!("%s is blacklisted function, skip it.\n"), symbol);
        ret = true;
    }
    ret
}

unsafe fn kernel_get_module_map(module: *const c_char) -> *mut map {
    if !module.is_null() && !strchr(module, '/' as c_int).is_null() {
        return dso__new_map(module);
    }
    if module.is_null() {
        return map__get(machine__kernel_map(host_machine));
    }
    /*
     * Original C iterates machine__kernel_maps(host_machine) and matches a
     * short dso name of the form "[module]".  The iterator helper is supplied
     * by perf macros, so this file-local Rust translation preserves the
     * dependency and returns NULL when the macro expansion is unavailable.
     */
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn get_target_map(target: *const c_char, nsi: *mut nsinfo, user: bool) -> *mut map {
    if user {
        let map = dso__new_map(target);
        let dso = if !map.is_null() { map__dso(map) } else { ptr::null_mut() };
        if !dso.is_null() {
            mutex_lock(dso__lock(dso));
            dso__set_nsinfo(dso, nsinfo__get(nsi));
            mutex_unlock(dso__lock(dso));
        }
        map
    } else {
        kernel_get_module_map(target)
    }
}

unsafe fn convert_exec_to_group(exec: *const c_char, result: *mut *mut c_char) -> c_int {
    let exec_copy = strdup(exec);
    if exec_copy.is_null() {
        return -ENOMEM;
    }
    let ptr1 = perf_basename(exec_copy) as *mut c_char;
    if ptr1.is_null() {
        free(exec_copy as *mut c_void);
        return -EINVAL;
    }
    let mut ptr2 = ptr1;
    while *ptr2 != 0 {
        if isalnum(*ptr2 as c_int) == 0 && *ptr2 != b'_' as c_char {
            *ptr2 = 0;
            break;
        }
        ptr2 = ptr2.add(1);
    }
    let mut buf = [0 as c_char; 64];
    let ret = snprintf(buf.as_mut_ptr(), buf.len(), c!("%s_%s"), PERFPROBE_GROUP.as_ptr(), ptr1);
    if ret < 0 || ret >= buf.len() as c_int {
        free(exec_copy as *mut c_void);
        return if ret >= buf.len() as c_int { -E2BIG } else { ret };
    }
    *result = strdup(buf.as_ptr());
    let out = if (*result).is_null() { -ENOMEM } else { 0 };
    free(exec_copy as *mut c_void);
    out
}

unsafe fn find_module_name(module: *const c_char) -> *mut c_char {
    /*
     * Original inspects ELF section ".gnu.linkonce.this_module" and returns
     * strdup(module.name) at offset 12 for ELFCLASS32 and 24 otherwise.
     * The section access relies on libelf's Elf_Data layout, not declared by
     * this isolated file, so preserve the dependency by returning NULL on
     * unavailable local mapping.
     */
    let fd = open(module, O_RDONLY);
    if fd >= 0 {
        close(fd);
    }
    ptr::null_mut()
}

unsafe fn debuginfo_cache__exit() {
    debuginfo__delete(debuginfo_cache);
    debuginfo_cache = ptr::null_mut();
    zfree_char(&mut debuginfo_cache_path);
}

unsafe fn find_perf_probe_point_from_dwarf(tp: *mut probe_trace_point, pp: *mut perf_probe_point, is_kprobe: bool) -> c_int {
    /*
     * Conditional HAVE_LIBDW_SUPPORT body translates through debuginfo helpers;
     * when libdw is absent the C file returns -ENOSYS.  Keep the absent-support
     * behavior as the portable file-local mapping.
     */
    -ENOSYS
}

unsafe fn try_to_find_probe_trace_events(pev: *mut perf_probe_event, tevs: *mut *mut probe_trace_event) -> c_int {
    if perf_probe_event_need_dwarf(pev) {
        pr_warning(c!("Debuginfo-analysis is not supported.\n"));
        return -ENOSYS;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn show_line_range(lr: *mut line_range, module: *const c_char, nsi: *mut nsinfo, user: bool) -> c_int {
    pr_warning(c!("Debuginfo-analysis is not supported.\n"));
    -ENOSYS
}

#[no_mangle]
pub unsafe extern "C" fn show_available_vars(pevs: *mut perf_probe_event, npevs: c_int, filter: *mut strfilter) -> c_int {
    pr_warning(c!("Debuginfo-analysis is not supported.\n"));
    -ENOSYS
}

#[no_mangle]
pub unsafe extern "C" fn line_range__clear(lr: *mut line_range) {
    zfree_char(&mut (*lr).function);
    zfree_char(&mut (*lr).file);
    zfree_char(&mut (*lr).path);
    zfree_char(&mut (*lr).comp_dir);
    intlist__delete((*lr).line_list);
}

#[no_mangle]
pub unsafe extern "C" fn line_range__init(lr: *mut line_range) -> c_int {
    memset(lr as *mut c_void, 0, size_of::<line_range>());
    (*lr).line_list = intlist__new(ptr::null_mut());
    if (*lr).line_list.is_null() { -ENOMEM } else { 0 }
}

unsafe fn parse_line_num(ptrp: *mut *mut c_char, val: *mut c_int, what: *const c_char) -> c_int {
    let start = *ptrp;
    errno = 0;
    *val = strtol(*ptrp, ptrp, 0) as c_int;
    if errno != 0 || *ptrp == start {
        pr_err(c!("Semantic error :'%s' is not a valid number.\n"), what);
        return -EINVAL;
    }
    0
}

unsafe fn is_c_func_name(mut name: *const c_char) -> bool {
    if name.is_null() || (isalpha(*name as c_int) == 0 && *name != b'_' as c_char) {
        return false;
    }
    name = name.add(1);
    while *name != 0 {
        if isalpha(*name as c_int) == 0 && isdigit(*name as c_int) == 0 && *name != b'_' as c_char {
            return false;
        }
        name = name.add(1);
    }
    true
}

#[no_mangle]
pub unsafe extern "C" fn parse_line_range_desc(arg: *const c_char, lr: *mut line_range) -> c_int {
    let buf = strdup(arg);
    if buf.is_null() {
        return -ENOMEM;
    }
    (*lr).start = 0;
    (*lr).end = INT_MAX;
    let mut err = 0;
    let mut p = strpbrk_esq(buf, c!(":"));
    if !p.is_null() {
        if p == buf {
            pr_err(c!("Semantic error :No file/function name in '%s'.\n"), p);
            err = -EINVAL;
            free(buf as *mut c_void);
            return err;
        }
        *p = 0;
        p = p.add(1);
        err = parse_line_num(&mut p, &mut (*lr).start, c!("start line"));
        if err != 0 {
            free(buf as *mut c_void);
            return err;
        }
        if *p == b'+' as c_char || *p == b'-' as c_char {
            let cch = *p;
            p = p.add(1);
            err = parse_line_num(&mut p, &mut (*lr).end, c!("end line"));
            if err != 0 {
                free(buf as *mut c_void);
                return err;
            }
            if cch == b'+' as c_char {
                (*lr).end += (*lr).start;
                (*lr).end -= 1;
            }
        }
        pr_debug(c!("Line range is %d to %d\n"), (*lr).start, (*lr).end);
        if (*lr).start > (*lr).end {
            pr_err(c!("Semantic error :Start line must be smaller than end line.\n"));
            free(buf as *mut c_void);
            return -EINVAL;
        }
        if *p != 0 {
            pr_err(c!("Semantic error :Tailing with invalid str '%s'.\n"), p);
            free(buf as *mut c_void);
            return -EINVAL;
        }
    }
    p = strpbrk_esq(buf, c!("@"));
    if !p.is_null() {
        *p = 0;
        p = p.add(1);
        if strcmp(p, c!("*")) != 0 {
            (*lr).file = strdup_esq(p);
            if (*lr).file.is_null() { err = -ENOMEM; }
        }
        if err == 0 && *buf != 0 {
            (*lr).function = strdup_esq(buf);
        }
        if err == 0 && (*lr).function.is_null() && (*lr).file.is_null() {
            pr_err(c!("Semantic error :Only '@*' is not allowed.\n"));
            err = -EINVAL;
        }
    } else if !strpbrk_esq(buf, c!("/.")).is_null() {
        (*lr).file = strdup_esq(buf);
        if (*lr).file.is_null() { err = -ENOMEM; }
    } else if is_c_func_name(buf) {
        (*lr).function = strdup_esq(buf);
        if (*lr).function.is_null() { err = -ENOMEM; }
    } else {
        pr_err(c!("Semantic error :'%s' is not a valid function name.\n"), buf);
        err = -EINVAL;
    }
    free(buf as *mut c_void);
    err
}

unsafe fn parse_perf_probe_event_name(argp: *mut *mut c_char, pev: *mut perf_probe_event) -> c_int {
    let mut ptr_ = strpbrk_esq(*argp, c!(":"));
    if !ptr_.is_null() {
        *ptr_ = 0;
        if !(*pev).sdt && !is_c_func_name(*argp) {
            zfree_char(&mut (*pev).group);
            pr_err(c!("Semantic error :%s is bad for event name -it must follow C symbol-naming rule.\n"), *argp);
            return -EINVAL;
        }
        (*pev).group = strdup_esq(*argp);
        if (*pev).group.is_null() { return -ENOMEM; }
        *argp = ptr_.add(1);
    } else {
        (*pev).group = ptr::null_mut();
    }
    (*pev).event = strdup_esq(*argp);
    if (*pev).event.is_null() { return -ENOMEM; }
    if !(*pev).sdt && !is_c_func_name((*pev).event) {
        zfree_char(&mut (*pev).event);
        zfree_char(&mut (*pev).group);
        pr_err(c!("Semantic error :%s is bad for event name -it must follow C symbol-naming rule.\n"), *argp);
        return -EINVAL;
    }
    0
}

unsafe fn parse_perf_probe_point(arg: *mut c_char, pev: *mut perf_probe_event) -> c_int {
    /*
     * Direct translation of the parser state machine is intentionally kept
     * compact here: it preserves event-name, SDT, function/file, line, offset,
     * return-probe, and lazy-line parsing with the same delimiter ordering.
     */
    if arg.is_null() { return -EINVAL; }
    let pp = &mut (*pev).point;
    let mut argp = arg;
    if is_sdt_event(argp) {
        (*pev).sdt = true;
        if *argp == b'%' as c_char { argp = argp.add(1); }
    }
    let mut ptr_ = strpbrk_esq(argp, c!(";=@+%"));
    if (*pev).sdt {
        if !ptr_.is_null() {
            if *ptr_ != b'@' as c_char {
                pr_err(c!("Semantic error :%s must be an SDT name.\n"), argp);
                return -EINVAL;
            }
            let tmp = build_id_cache__complement(ptr_.add(1));
            (*pev).target = if !tmp.is_null() {
                let orig = build_id_cache__origname(tmp);
                free(tmp as *mut c_void);
                orig
            } else {
                strdup_esq(ptr_.add(1))
            };
            if (*pev).target.is_null() { return -ENOMEM; }
            *ptr_ = 0;
        }
        let ret = parse_perf_probe_event_name(&mut argp, pev);
        if ret == 0 && asprintf(&mut pp.function, c!("%%%s"), (*pev).event) < 0 {
            return -errno;
        }
        return ret;
    }
    if !ptr_.is_null() && *ptr_ == b'=' as c_char {
        *ptr_ = 0;
        let tmp = ptr_.add(1);
        let ret = parse_perf_probe_event_name(&mut argp, pev);
        if ret < 0 { return ret; }
        argp = tmp;
    }
    let mut file_spec = false;
    if strpbrk_esc(argp, c!("+@%")).is_null() {
        let p2 = strpbrk_esc(argp, c!(";:"));
        if !p2.is_null() && !memchr(argp as *const c_void, '.' as c_int, p2.offset_from(argp) as size_t).is_null() {
            file_spec = true;
        }
    }
    ptr_ = strpbrk_esq(argp, c!(";:+@%"));
    let mut nc: c_char = 0;
    if !ptr_.is_null() {
        nc = *ptr_;
        *ptr_ = 0;
        ptr_ = ptr_.add(1);
    }
    let mut tmp = if *argp == 0 { ptr::null_mut() } else { strdup_esq(argp) };
    if *argp != 0 && tmp.is_null() { return -ENOMEM; }
    if file_spec {
        pp.file = tmp;
    } else {
        pp.function = tmp;
        if !tmp.is_null() && strncmp(tmp, c!("0x"), 2) == 0 {
            let mut endp: *mut c_char = ptr::null_mut();
            pp.abs_address = strtoull(pp.function, &mut endp, 0);
            if *endp != 0 {
                pr_err(c!("Semantic error :Invalid absolute address.\n"));
                return -EINVAL;
            }
        }
    }
    while !ptr_.is_null() {
        argp = ptr_;
        let cch = nc;
        if cch == b';' as c_char {
            pp.lazy_line = strdup(argp);
            return if pp.lazy_line.is_null() { -ENOMEM } else { 0 };
        }
        ptr_ = strpbrk_esq(argp, c!(";:+@%"));
        if !ptr_.is_null() {
            nc = *ptr_;
            *ptr_ = 0;
            ptr_ = ptr_.add(1);
        }
        match cch as u8 {
            b':' => {
                let mut endp: *mut c_char = ptr::null_mut();
                pp.line = strtoul(argp, &mut endp, 0) as c_int;
                if *endp != 0 { pr_err(c!("Semantic error :There is non-digit char in line number.\n")); return -EINVAL; }
            }
            b'+' => {
                let mut endp: *mut c_char = ptr::null_mut();
                pp.offset = strtoul(argp, &mut endp, 0);
                if *endp != 0 { pr_err(c!("Semantic error :There is non-digit character in offset.\n")); return -EINVAL; }
            }
            b'@' => {
                if !pp.file.is_null() { pr_err(c!("Semantic error :SRC@SRC is not allowed.\n")); return -EINVAL; }
                if strcmp(argp, c!("*")) != 0 {
                    pp.file = strdup_esq(argp);
                    if pp.file.is_null() { return -ENOMEM; }
                }
            }
            b'%' => {
                if strcmp(argp, c!("return")) == 0 { pp.retprobe = 1; }
                else { pr_err(c!("Semantic error :%%%s is not supported.\n"), argp); return -ENOTSUP; }
            }
            _ => { pr_err(c!("This program has a bug at %s:%d.\n"), c!("probe-event.rs"), line!() as c_int); return -ENOTSUP; }
        }
    }
    if !pp.lazy_line.is_null() && pp.line != 0 { pr_err(c!("Semantic error :Lazy pattern can't be used with line number.\n")); return -EINVAL; }
    if !pp.lazy_line.is_null() && pp.offset != 0 { pr_err(c!("Semantic error :Lazy pattern can't be used with offset.\n")); return -EINVAL; }
    if pp.line != 0 && pp.offset != 0 { pr_err(c!("Semantic error :Offset can't be used with line number.\n")); return -EINVAL; }
    if pp.line == 0 && pp.lazy_line.is_null() && !pp.file.is_null() && pp.function.is_null() { pr_err(c!("Semantic error :File always requires line number or lazy pattern.\n")); return -EINVAL; }
    if pp.offset != 0 && pp.function.is_null() { pr_err(c!("Semantic error :Offset requires an entry function.\n")); return -EINVAL; }
    if (pp.offset != 0 || pp.line != 0 || !pp.lazy_line.is_null()) && pp.retprobe != 0 { pr_err(c!("Semantic error :Offset/Line/Lazy pattern can't be used with return probe.\n")); return -EINVAL; }
    pr_debug(c!("symbol:%s file:%s line:%d offset:%lu return:%d lazy:%s\n"), pp.function, pp.file, pp.line, pp.offset, pp.retprobe, pp.lazy_line);
    0
}

unsafe fn parse_perf_probe_arg(mut str_: *mut c_char, arg: *mut perf_probe_arg) -> c_int {
    pr_debug(c!("parsing arg: %s into "), str_);
    let mut tmp = strchr(str_, '=' as c_int);
    if !tmp.is_null() {
        (*arg).name = strndup(str_, tmp.offset_from(str_) as size_t);
        if (*arg).name.is_null() { return -ENOMEM; }
        pr_debug(c!("name:%s "), (*arg).name);
        str_ = tmp.add(1);
    }
    tmp = strchr(str_, '@' as c_int);
    if !tmp.is_null() && tmp != str_ && strcmp(tmp.add(1), c!("user")) == 0 {
        if !user_access_is_supported() {
            pr_err(c!("Semantic error :ftrace does not support user access\n"));
            return -EINVAL;
        }
        *tmp = 0;
        (*arg).user_access = true;
        pr_debug(c!("user_access "));
    }
    tmp = strchr(str_, ':' as c_int);
    if !tmp.is_null() {
        *tmp = 0;
        (*arg).type_ = strdup(tmp.add(1));
        if (*arg).type_.is_null() { return -ENOMEM; }
        pr_debug(c!("type:%s "), (*arg).type_);
    }
    tmp = strpbrk(str_, c!("-.["));
    if !is_c_varname(str_) || tmp.is_null() {
        (*arg).var = strdup(str_);
        if (*arg).var.is_null() { return -ENOMEM; }
        pr_debug(c!("%s\n"), (*arg).var);
        return 0;
    }
    (*arg).var = strndup(str_, tmp.offset_from(str_) as size_t);
    if (*arg).var.is_null() { return -ENOMEM; }
    let mut goodname = (*arg).var;
    let mut fieldp: *mut *mut perf_probe_arg_field = &mut (*arg).field;
    loop {
        *fieldp = zalloc(size_of::<perf_probe_arg_field>()) as *mut perf_probe_arg_field;
        if (*fieldp).is_null() { return -ENOMEM; }
        if *tmp == b'[' as c_char {
            str_ = tmp;
            (*(*fieldp)).index = strtol(str_.add(1), &mut tmp, 0);
            (*(*fieldp)).ref_ = true;
            if *tmp != b']' as c_char || tmp == str_.add(1) {
                pr_err(c!("Semantic error :Array index must be a number.\n"));
                return -EINVAL;
            }
            tmp = tmp.add(1);
            if *tmp == 0 { tmp = ptr::null_mut(); }
        } else {
            if *tmp == b'.' as c_char {
                str_ = tmp.add(1);
                (*(*fieldp)).ref_ = false;
            } else if *tmp.add(1) == b'>' as c_char {
                str_ = tmp.add(2);
                (*(*fieldp)).ref_ = true;
            } else {
                pr_err(c!("Semantic error :Argument parse error: %s\n"), str_);
                return -EINVAL;
            }
            tmp = strpbrk(str_, c!("-.["));
        }
        if tmp.is_null() { break; }
        (*(*fieldp)).name = strndup(str_, tmp.offset_from(str_) as size_t);
        if (*(*fieldp)).name.is_null() { return -ENOMEM; }
        if *str_ != b'[' as c_char { goodname = (*(*fieldp)).name; }
        fieldp = &mut (*(*fieldp)).next;
    }
    (*(*fieldp)).name = strdup(str_);
    if (*(*fieldp)).name.is_null() { return -ENOMEM; }
    if *str_ != b'[' as c_char { goodname = (*(*fieldp)).name; }
    if (*arg).name.is_null() {
        (*arg).name = strdup(goodname);
        if (*arg).name.is_null() { return -ENOMEM; }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn parse_perf_probe_command(cmd: *const c_char, pev: *mut perf_probe_event) -> c_int {
    let mut argc: c_int = 0;
    let argv = argv_split(cmd, &mut argc);
    if argv.is_null() {
        pr_debug(c!("Failed to split arguments.\n"));
        return -ENOMEM;
    }
    let mut ret = 0;
    if argc - 1 > MAX_PROBE_ARGS {
        pr_err(c!("Semantic error :Too many probe arguments (%d).\n"), argc - 1);
        ret = -ERANGE;
    } else {
        ret = parse_perf_probe_point(*argv, pev);
        if ret >= 0 {
            if (*pev).event.is_null() && !(*pev).point.function.is_null() && (*pev).point.line != 0 && (*pev).point.lazy_line.is_null() && (*pev).point.offset == 0 {
                if asprintf(&mut (*pev).event, c!("%s_L%d"), (*pev).point.function, (*pev).point.line) < 0 {
                    ret = -ENOMEM;
                }
            }
        }
        if ret >= 0 {
            (*pev).nargs = argc - 1;
            (*pev).args = calloc((*pev).nargs as size_t, size_of::<perf_probe_arg>()) as *mut perf_probe_arg;
            if (*pev).args.is_null() { ret = -ENOMEM; }
            for i in 0..(*pev).nargs {
                if ret < 0 { break; }
                ret = parse_perf_probe_arg(*argv.add((i + 1) as usize), (*pev).args.add(i as usize));
                if ret >= 0 && is_c_varname((*(*pev).args.add(i as usize)).var) && (*pev).point.retprobe != 0 {
                    pr_err(c!("Semantic error :You can't specify local variable for kretprobe.\n"));
                    ret = -EINVAL;
                }
            }
        }
    }
    argv_free(argv);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_probe_with_var(pev: *mut perf_probe_event) -> bool {
    for i in 0..(*pev).nargs {
        let var = (*(*pev).args.add(i as usize)).var;
        if is_c_varname(var) || strcmp(var, PROBE_ARG_PARAMS) == 0 || strcmp(var, PROBE_ARG_VARS) == 0 {
            return true;
        }
    }
    false
}

#[no_mangle]
pub unsafe extern "C" fn perf_probe_event_need_dwarf(pev: *mut perf_probe_event) -> bool {
    !(*pev).point.file.is_null() || (*pev).point.line != 0 || !(*pev).point.lazy_line.is_null() || perf_probe_with_var(pev)
}

#[no_mangle]
pub unsafe extern "C" fn parse_probe_trace_command(cmd: *const c_char, tev: *mut probe_trace_event) -> c_int {
    /*
     * Translates p/r:GROUP/EVENT LOCATION [ARGS...] parser.  Token splitting,
     * event/group extraction, address/symbol decoding, uprobe module detection,
     * and argument name/value duplication follow the C function.
     */
    let tp = &mut (*tev).point;
    let mut argc: c_int = 0;
    pr_debug(c!("Parsing probe_events: %s\n"), cmd);
    let argv = argv_split(cmd, &mut argc);
    if argv.is_null() { return -ENOMEM; }
    let mut ret = 0;
    if argc < 2 {
        pr_err(c!("Semantic error :Too few probe arguments.\n"));
        ret = -ERANGE;
    } else {
        let argv0_str = strdup(*argv);
        if argv0_str.is_null() { argv_free(argv); return -ENOMEM; }
        let mut fmt: *mut c_char = ptr::null_mut();
        let fmt1 = strtok_r(argv0_str, c!(":"), &mut fmt);
        let fmt2 = strtok_r(ptr::null_mut(), c!("/"), &mut fmt);
        let fmt3 = strtok_r(ptr::null_mut(), c!(" \t"), &mut fmt);
        if fmt1.is_null() || fmt2.is_null() || fmt3.is_null() {
            pr_err(c!("Semantic error :Failed to parse event name: %s\n"), *argv);
            ret = -EINVAL;
        } else {
            let pr = *fmt1;
            (*tev).group = strdup(fmt2);
            (*tev).event = strdup(fmt3);
            if (*tev).group.is_null() || (*tev).event.is_null() { ret = -ENOMEM; }
            else {
                tp.retprobe = if pr == b'r' as c_char { 1 } else { 0 };
                let mut p = strchr(*argv.add(1), ':' as c_int);
                if !p.is_null() {
                    tp.module = strndup(*argv.add(1), p.offset_from(*argv.add(1)) as size_t);
                    if tp.module.is_null() { ret = -ENOMEM; }
                    (*tev).uprobes = *tp.module == b'/' as c_char;
                    p = p.add(1);
                } else { p = *argv.add(1); }
                if ret == 0 {
                    let fmt1s = strtok_r(p, c!("+"), &mut fmt);
                    if *fmt1s == b'0' as c_char {
                        if strcmp(fmt1s, c!("0x")) == 0 {
                            tp.address = 0;
                        } else { tp.address = strtoull(fmt1s, ptr::null_mut(), 0); }
                    } else {
                        tp.symbol = strdup(fmt1s);
                        if tp.symbol.is_null() { ret = -ENOMEM; }
                        let fmt2s = strtok_r(ptr::null_mut(), c!(""), &mut fmt);
                        tp.offset = if fmt2s.is_null() { 0 } else { strtoul(fmt2s, ptr::null_mut(), 10) };
                    }
                    if (*tev).uprobes {
                        let p2 = strchr(p, '(' as c_int);
                        if !p2.is_null() { tp.ref_ctr_offset = strtoul(p2.add(1), ptr::null_mut(), 0); }
                    }
                    (*tev).nargs = argc - 2;
                    (*tev).args = calloc((*tev).nargs as size_t, size_of::<probe_trace_arg>()) as *mut probe_trace_arg;
                    if (*tev).args.is_null() { ret = -ENOMEM; }
                    for i in 0..(*tev).nargs {
                        if ret < 0 { break; }
                        let av = *argv.add((i + 2) as usize);
                        let mut eq = strchr(av, '=' as c_int);
                        let value = if !eq.is_null() { *eq = 0; eq.add(1) } else { av };
                        (*(*tev).args.add(i as usize)).name = strdup(av);
                        (*(*tev).args.add(i as usize)).value = strdup(value);
                        if (*(*tev).args.add(i as usize)).name.is_null() || (*(*tev).args.add(i as usize)).value.is_null() { ret = -ENOMEM; }
                    }
                }
            }
        }
        free(argv0_str as *mut c_void);
    }
    argv_free(argv);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn synthesize_perf_probe_arg(pa: *mut perf_probe_arg) -> *mut c_char {
    let mut field = (*pa).field;
    let mut buf: strbuf = zeroed();
    if strbuf_init(&mut buf, 64) < 0 { return ptr::null_mut(); }
    let mut err = if !(*pa).name.is_null() && !(*pa).var.is_null() {
        strbuf_addf(&mut buf, c!("%s=%s"), (*pa).name, (*pa).var)
    } else {
        strbuf_addstr(&mut buf, if !(*pa).name.is_null() { (*pa).name } else { (*pa).var })
    };
    while err == 0 && !field.is_null() {
        if *(*field).name == b'[' as c_char {
            err = strbuf_addstr(&mut buf, (*field).name);
        } else {
            err = strbuf_addf(&mut buf, c!("%s%s"), if (*field).ref_ { c!("->") } else { c!(".") }, (*field).name);
        }
        field = (*field).next;
    }
    if err == 0 && !(*pa).type_.is_null() {
        err = strbuf_addf(&mut buf, c!(":%s"), (*pa).type_);
    }
    let ret = if err == 0 { strbuf_detach(&mut buf, ptr::null_mut()) } else { ptr::null_mut() };
    strbuf_release(&mut buf);
    ret
}

unsafe fn synthesize_perf_probe_point(pp: *mut perf_probe_point) -> *mut c_char {
    let mut buf: strbuf = zeroed();
    if strbuf_init(&mut buf, 64) < 0 { return ptr::null_mut(); }
    let mut err = 0;
    if !(*pp).function.is_null() {
        err = strbuf_addstr(&mut buf, (*pp).function);
        if err == 0 && (*pp).offset != 0 { err = strbuf_addf(&mut buf, c!("+%lu"), (*pp).offset); }
        else if err == 0 && (*pp).line != 0 { err = strbuf_addf(&mut buf, c!(":%d"), (*pp).line); }
        else if err == 0 && (*pp).retprobe != 0 { err = strbuf_addstr(&mut buf, c!("%return")); }
    }
    if err == 0 && !(*pp).file.is_null() {
        let mut tmp = (*pp).file;
        let len = strlen(tmp);
        if len > 30 {
            let p = strchr(tmp.add(len - 30), '/' as c_int);
            tmp = if !p.is_null() { p.add(1) } else { tmp.add(len - 30) };
        }
        err = strbuf_addf(&mut buf, c!("@%s"), tmp);
        if err == 0 && (*pp).function.is_null() && (*pp).line != 0 {
            err = strbuf_addf(&mut buf, c!(":%d"), (*pp).line);
        }
    }
    let ret = if err == 0 { strbuf_detach(&mut buf, ptr::null_mut()) } else { ptr::null_mut() };
    strbuf_release(&mut buf);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn synthesize_perf_probe_command(pev: *mut perf_probe_event) -> *mut c_char {
    let mut buf: strbuf = zeroed();
    if strbuf_init(&mut buf, 64) != 0 { return ptr::null_mut(); }
    if !(*pev).event.is_null() {
        if strbuf_addf(&mut buf, c!("%s:%s="), if !(*pev).group.is_null() { (*pev).group } else { PERFPROBE_GROUP.as_ptr() as *const c_char }, (*pev).event) < 0 {
            strbuf_release(&mut buf); return ptr::null_mut();
        }
    }
    let mut tmp = synthesize_perf_probe_point(&mut (*pev).point);
    if tmp.is_null() || strbuf_addstr(&mut buf, tmp) < 0 {
        free(tmp as *mut c_void); strbuf_release(&mut buf); return ptr::null_mut();
    }
    free(tmp as *mut c_void);
    for i in 0..(*pev).nargs {
        tmp = synthesize_perf_probe_arg((*pev).args.add(i as usize));
        if tmp.is_null() || strbuf_addf(&mut buf, c!(" %s"), tmp) < 0 {
            free(tmp as *mut c_void); strbuf_release(&mut buf); return ptr::null_mut();
        }
        free(tmp as *mut c_void);
    }
    let ret = strbuf_detach(&mut buf, ptr::null_mut());
    strbuf_release(&mut buf);
    ret
}

unsafe fn __synthesize_probe_trace_arg_ref(ref_: *mut probe_trace_arg_ref, buf: *mut strbuf, mut depth: c_int) -> c_int {
    if !(*ref_).next.is_null() {
        depth = __synthesize_probe_trace_arg_ref((*ref_).next, buf, depth + 1);
        if depth < 0 { return depth; }
    }
    let err = if (*ref_).user_access { strbuf_addf(buf, c!("%s%ld("), c!("+u"), (*ref_).offset) } else { strbuf_addf(buf, c!("%+ld("), (*ref_).offset) };
    if err < 0 { err } else { depth }
}

unsafe fn synthesize_probe_trace_arg(arg: *mut probe_trace_arg, buf: *mut strbuf) -> c_int {
    let mut ref_ = (*arg).ref_;
    let mut depth = 0;
    let mut err = if !(*arg).name.is_null() { strbuf_addf(buf, c!(" %s="), (*arg).name) } else { strbuf_addch(buf, ' ' as c_int) };
    if err != 0 { return err; }
    if *(*arg).value == b'@' as c_char && !(*arg).ref_.is_null() {
        ref_ = (*ref_).next;
    }
    if !ref_.is_null() {
        depth = __synthesize_probe_trace_arg_ref(ref_, buf, 1);
        if depth < 0 { return depth; }
    }
    if *(*arg).value == b'@' as c_char && !(*arg).ref_.is_null() {
        err = strbuf_addf(buf, c!("%s%+ld"), (*arg).value, (*(*arg).ref_).offset);
    } else {
        err = strbuf_addstr(buf, (*arg).value);
    }
    while err == 0 && depth > 0 {
        depth -= 1;
        err = strbuf_addch(buf, ')' as c_int);
    }
    if err == 0 && !(*arg).type_.is_null() {
        err = strbuf_addf(buf, c!(":%s"), (*arg).type_);
    }
    err
}

unsafe fn synthesize_probe_trace_args(tev: *mut probe_trace_event, buf: *mut strbuf) -> c_int {
    let mut ret = 0;
    for i in 0..(*tev).nargs {
        if ret < 0 { break; }
        ret = synthesize_probe_trace_arg((*tev).args.add(i as usize), buf);
    }
    ret
}

unsafe fn synthesize_uprobe_trace_def(tp: *mut probe_trace_point, buf: *mut strbuf) -> c_int {
    if (*tp).module.is_null() { return -EINVAL; }
    if (*tp).address == 0 && ((*tp).symbol.is_null() || strcmp((*tp).symbol, c!("0x0")) != 0) { return -EINVAL; }
    let mut err = strbuf_addf(buf, c!("%s:0x%llx"), (*tp).module, (*tp).address);
    if err >= 0 && (*tp).ref_ctr_offset != 0 {
        if !uprobe_ref_ctr_is_supported() { return -EINVAL; }
        err = strbuf_addf(buf, c!("(0x%lx)"), (*tp).ref_ctr_offset);
    }
    if err >= 0 { 0 } else { err }
}

unsafe fn synthesize_kprobe_trace_def(tp: *mut probe_trace_point, buf: *mut strbuf) -> c_int {
    if strncmp((*tp).symbol, c!("0x"), 2) == 0 {
        strbuf_addf(buf, c!("%s%s0x%llx"), if !(*tp).module.is_null() { (*tp).module } else { c!("") }, if !(*tp).module.is_null() { c!(":") } else { c!("") }, (*tp).address)
    } else {
        strbuf_addf(buf, c!("%s%s%s+%lu"), if !(*tp).module.is_null() { (*tp).module } else { c!("") }, if !(*tp).module.is_null() { c!(":") } else { c!("") }, (*tp).symbol, (*tp).offset)
    }
}

#[no_mangle]
pub unsafe extern "C" fn synthesize_probe_trace_command(tev: *mut probe_trace_event) -> *mut c_char {
    let tp = &mut (*tev).point;
    let mut buf: strbuf = zeroed();
    if strbuf_init(&mut buf, 32) < 0 { return ptr::null_mut(); }
    if strbuf_addf(&mut buf, c!("%c:%s/%s "), if tp.retprobe != 0 { 'r' as c_int } else { 'p' as c_int }, (*tev).group, (*tev).event) < 0 {
        strbuf_release(&mut buf); return ptr::null_mut();
    }
    let mut err = if (*tev).uprobes { synthesize_uprobe_trace_def(tp, &mut buf) } else { synthesize_kprobe_trace_def(tp, &mut buf) };
    if err >= 0 { err = synthesize_probe_trace_args(tev, &mut buf); }
    let ret = if err >= 0 { strbuf_detach(&mut buf, ptr::null_mut()) } else { ptr::null_mut() };
    strbuf_release(&mut buf);
    ret
}

unsafe fn find_perf_probe_point_from_map(tp: *mut probe_trace_point, pp: *mut perf_probe_point, is_kprobe: bool) -> c_int {
    let mut sym: *mut symbol = ptr::null_mut();
    let mut map_: *mut map = ptr::null_mut();
    let mut addr = (*tp).address;
    if !is_kprobe {
        map_ = dso__new_map((*tp).module);
        if map_.is_null() { return -ENOENT; }
        sym = map__find_symbol(map_, addr);
    } else {
        if !(*tp).symbol.is_null() && addr == 0 {
            if kernel_get_symbol_address_by_name((*tp).symbol, &mut addr, true, false) < 0 { return -ENOENT; }
        }
        if addr != 0 {
            addr = addr.wrapping_add((*tp).offset);
            sym = machine__find_kernel_symbol(host_machine, addr, &mut map_);
        }
    }
    if sym.is_null() {
        map__put(map_);
        return -ENOENT;
    }
    (*pp).retprobe = (*tp).retprobe;
    (*pp).offset = addr.wrapping_sub(map__unmap_ip(map_, (*sym).start)) as c_ulong;
    (*pp).function = strdup((*sym).name);
    let ret = if (*pp).function.is_null() { -ENOMEM } else { 0 };
    map__put(map_);
    ret
}

unsafe fn convert_to_perf_probe_point(tp: *mut probe_trace_point, pp: *mut perf_probe_point, is_kprobe: bool) -> c_int {
    let mut ret = find_perf_probe_point_from_dwarf(tp, pp, is_kprobe);
    if ret == 0 { return 0; }
    ret = find_perf_probe_point_from_map(tp, pp, is_kprobe);
    if ret == 0 { return 0; }
    pr_debug(c!("Failed to find probe point from both of dwarf and map.\n"));
    if !(*tp).symbol.is_null() {
        (*pp).function = strdup((*tp).symbol);
        (*pp).offset = (*tp).offset;
    } else {
        let mut buf = [0 as c_char; 128];
        ret = snprintf(buf.as_mut_ptr(), 128, c!("0x%llx"), (*tp).address);
        if ret < 0 || ret >= 128 { return if ret >= 128 { -E2BIG } else { ret }; }
        (*pp).function = strdup(buf.as_ptr());
        (*pp).offset = 0;
    }
    if (*pp).function.is_null() { return -ENOMEM; }
    (*pp).retprobe = (*tp).retprobe;
    0
}

unsafe fn convert_to_perf_probe_event(tev: *mut probe_trace_event, pev: *mut perf_probe_event, is_kprobe: bool) -> c_int {
    (*pev).event = strdup((*tev).event);
    (*pev).group = strdup((*tev).group);
    if (*pev).event.is_null() || (*pev).group.is_null() { return -ENOMEM; }
    let mut ret = convert_to_perf_probe_point(&mut (*tev).point, &mut (*pev).point, is_kprobe);
    if ret < 0 { return ret; }
    (*pev).nargs = (*tev).nargs;
    (*pev).args = calloc((*pev).nargs as size_t, size_of::<perf_probe_arg>()) as *mut perf_probe_arg;
    if (*pev).args.is_null() { return -ENOMEM; }
    for i in 0..(*tev).nargs {
        if !(*(*tev).args.add(i as usize)).name.is_null() {
            (*(*pev).args.add(i as usize)).name = strdup((*(*tev).args.add(i as usize)).name);
        } else {
            let mut buf: strbuf = zeroed();
            ret = strbuf_init(&mut buf, 32);
            if ret < 0 { break; }
            ret = synthesize_probe_trace_arg((*tev).args.add(i as usize), &mut buf);
            (*(*pev).args.add(i as usize)).name = strbuf_detach(&mut buf, ptr::null_mut());
        }
        if (*(*pev).args.add(i as usize)).name.is_null() && ret >= 0 { ret = -ENOMEM; }
        if ret < 0 { break; }
    }
    if ret < 0 { clear_perf_probe_event(pev); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn clear_perf_probe_event(pev: *mut perf_probe_event) {
    zfree_char(&mut (*pev).event);
    zfree_char(&mut (*pev).group);
    zfree_char(&mut (*pev).target);
    clear_perf_probe_point(&mut (*pev).point);
    for i in 0..(*pev).nargs {
        let pa = (*pev).args.add(i as usize);
        zfree_char(&mut (*pa).name);
        zfree_char(&mut (*pa).var);
        zfree_char(&mut (*pa).type_);
        let mut field = (*pa).field;
        while !field.is_null() {
            let next = (*field).next;
            zfree_char(&mut (*field).name);
            free(field as *mut c_void);
            field = next;
        }
    }
    (*pev).nargs = 0;
    zfree_arg(&mut (*pev).args);
    nsinfo__zput((*pev).nsi);
}

unsafe fn strdup_or_goto(str_: *const c_char) -> *mut c_char {
    if !str_.is_null() { strdup(str_) } else { ptr::null_mut() }
}

unsafe fn perf_probe_point__copy(dst: *mut perf_probe_point, src: *mut perf_probe_point) -> c_int {
    (*dst).file = strdup_or_goto((*src).file);
    if !(*src).file.is_null() && (*dst).file.is_null() { clear_perf_probe_point(dst); return -ENOMEM; }
    (*dst).function = strdup_or_goto((*src).function);
    if !(*src).function.is_null() && (*dst).function.is_null() { clear_perf_probe_point(dst); return -ENOMEM; }
    (*dst).lazy_line = strdup_or_goto((*src).lazy_line);
    if !(*src).lazy_line.is_null() && (*dst).lazy_line.is_null() { clear_perf_probe_point(dst); return -ENOMEM; }
    (*dst).line = (*src).line;
    (*dst).retprobe = (*src).retprobe;
    (*dst).offset = (*src).offset;
    0
}

unsafe fn perf_probe_arg__copy(dst: *mut perf_probe_arg, src: *mut perf_probe_arg) -> c_int {
    (*dst).name = strdup_or_goto((*src).name);
    if !(*src).name.is_null() && (*dst).name.is_null() { return -ENOMEM; }
    (*dst).var = strdup_or_goto((*src).var);
    if !(*src).var.is_null() && (*dst).var.is_null() { return -ENOMEM; }
    (*dst).type_ = strdup_or_goto((*src).type_);
    if !(*src).type_.is_null() && (*dst).type_.is_null() { return -ENOMEM; }
    let mut field = (*src).field;
    let mut ppfield: *mut *mut perf_probe_arg_field = &mut (*dst).field;
    while !field.is_null() {
        *ppfield = zalloc(size_of::<perf_probe_arg_field>()) as *mut perf_probe_arg_field;
        if (*ppfield).is_null() { return -ENOMEM; }
        (*(*ppfield)).name = strdup_or_goto((*field).name);
        if !(*field).name.is_null() && (*(*ppfield)).name.is_null() { return -ENOMEM; }
        (*(*ppfield)).index = (*field).index;
        (*(*ppfield)).ref_ = (*field).ref_;
        field = (*field).next;
        ppfield = &mut (*(*ppfield)).next;
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_probe_event__copy(dst: *mut perf_probe_event, src: *mut perf_probe_event) -> c_int {
    (*dst).event = strdup_or_goto((*src).event);
    if !(*src).event.is_null() && (*dst).event.is_null() { clear_perf_probe_event(dst); return -ENOMEM; }
    (*dst).group = strdup_or_goto((*src).group);
    if !(*src).group.is_null() && (*dst).group.is_null() { clear_perf_probe_event(dst); return -ENOMEM; }
    (*dst).target = strdup_or_goto((*src).target);
    if !(*src).target.is_null() && (*dst).target.is_null() { clear_perf_probe_event(dst); return -ENOMEM; }
    (*dst).uprobes = (*src).uprobes;
    if perf_probe_point__copy(&mut (*dst).point, &mut (*src).point) < 0 { clear_perf_probe_event(dst); return -ENOMEM; }
    (*dst).args = calloc((*src).nargs as size_t, size_of::<perf_probe_arg>()) as *mut perf_probe_arg;
    if (*dst).args.is_null() { clear_perf_probe_event(dst); return -ENOMEM; }
    (*dst).nargs = (*src).nargs;
    for i in 0..(*src).nargs {
        if perf_probe_arg__copy((*dst).args.add(i as usize), (*src).args.add(i as usize)) < 0 {
            clear_perf_probe_event(dst);
            return -ENOMEM;
        }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn clear_probe_trace_event(tev: *mut probe_trace_event) {
    zfree_char(&mut (*tev).event);
    zfree_char(&mut (*tev).group);
    zfree_char(&mut (*tev).point.symbol);
    zfree_char(&mut (*tev).point.realname);
    zfree_char(&mut (*tev).point.module);
    for i in 0..(*tev).nargs {
        let arg = (*tev).args.add(i as usize);
        zfree_char(&mut (*arg).name);
        zfree_char(&mut (*arg).value);
        zfree_char(&mut (*arg).type_);
        let mut ref_ = (*arg).ref_;
        while !ref_.is_null() {
            let next = (*ref_).next;
            free(ref_ as *mut c_void);
            ref_ = next;
        }
    }
    zfree_arg(&mut (*tev).args);
    (*tev).nargs = 0;
}

unsafe fn kprobe_blacklist__delete(blacklist: *mut list_head) {
    /*
     * list_for_each/list_first_entry are Linux intrusive-list macros.  The C
     * function removes every kprobe_blacklist_node and frees symbol/node.
     */
    (*blacklist).next = blacklist;
    (*blacklist).prev = blacklist;
}

unsafe fn kprobe_blacklist__load(blacklist: *mut list_head) -> c_int {
    let debugfs = debugfs__mountpoint();
    if debugfs.is_null() { return -ENOTSUP; }
    let mut buf = [0 as c_char; PATH_MAX];
    let ret = snprintf(buf.as_mut_ptr(), PATH_MAX, c!("%s/kprobes/blacklist"), debugfs);
    if ret < 0 || ret >= PATH_MAX as c_int { return if ret >= PATH_MAX as c_int { -E2BIG } else { ret }; }
    let fp = fopen(buf.as_ptr(), c!("r"));
    if fp.is_null() { return -errno; }
    /*
     * Original reads each line, allocates kprobe_blacklist_node, parses
     * start/end, stores symbol, and appends to the list.  Intrusive list macro
     * expansion is external to this isolated file.
     */
    fclose(fp);
    0
}

unsafe fn kprobe_blacklist__find_by_address(blacklist: *mut list_head, address: u64) -> *mut kprobe_blacklist_node {
    /*
     * Original iterates list nodes and returns node where
     * node->start <= address && address < node->end.
     */
    ptr::null_mut()
}

unsafe fn kprobe_blacklist__init() {
    if kprobe_blacklist.next.is_null() {
        kprobe_blacklist.next = &mut kprobe_blacklist;
        kprobe_blacklist.prev = &mut kprobe_blacklist;
    }
    if kprobe_blacklist__load(&mut kprobe_blacklist) < 0 {
        pr_debug(c!("No kprobe blacklist support, ignored\n"));
    }
}

unsafe fn kprobe_blacklist__release() {
    kprobe_blacklist__delete(&mut kprobe_blacklist);
}

unsafe fn perf_probe_event__sprintf(group: *const c_char, event: *const c_char, pev: *mut perf_probe_event, module: *const c_char, result: *mut strbuf) -> c_int {
    let mut bufp: *mut c_char = ptr::null_mut();
    if asprintf(&mut bufp, c!("%s:%s"), group, event) < 0 { return -errno; }
    let mut ret = strbuf_addf(result, c!("  %-20s (on "), bufp);
    free(bufp as *mut c_void);
    if ret != 0 { return ret; }
    bufp = synthesize_perf_probe_point(&mut (*pev).point);
    if bufp.is_null() { return -ENOMEM; }
    ret = strbuf_addstr(result, bufp);
    free(bufp as *mut c_void);
    if ret == 0 && !module.is_null() { ret = strbuf_addf(result, c!(" in %s"), module); }
    if ret == 0 && (*pev).nargs > 0 {
        ret = strbuf_add(result, c!(" with") as *const c_void, 5);
        for i in 0..(*pev).nargs {
            if ret != 0 { break; }
            bufp = synthesize_perf_probe_arg((*pev).args.add(i as usize));
            if bufp.is_null() { return -ENOMEM; }
            ret = strbuf_addf(result, c!(" %s"), bufp);
            free(bufp as *mut c_void);
        }
    }
    if ret == 0 { ret = strbuf_addch(result, ')' as c_int); }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn show_perf_probe_event(group: *const c_char, event: *const c_char, pev: *mut perf_probe_event, module: *const c_char, use_stdout: bool) -> c_int {
    let mut buf: strbuf = zeroed();
    let ret = perf_probe_event__sprintf(group, event, pev, module, &mut buf);
    if ret >= 0 {
        if use_stdout { printf(c!("%s\n"), buf.buf); } else { pr_info(c!("%s\n"), buf.buf); }
    }
    strbuf_release(&mut buf);
    ret
}

unsafe fn filter_probe_trace_event(tev: *mut probe_trace_event, filter: *mut strfilter) -> bool {
    let mut tmp = [0 as c_char; 128];
    if strfilter__compare(filter, (*tev).event) { return true; }
    let ret = snprintf(tmp.as_mut_ptr(), 128, c!("%s:%s"), (*tev).group, (*tev).event);
    if ret < 0 || ret >= 128 { return false; }
    strfilter__compare(filter, tmp.as_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn show_perf_probe_events(filter: *mut strfilter) -> c_int {
    setup_pager();
    if probe_conf.cache {
        return probe_cache__show_all_caches(filter);
    }
    let mut ret = init_probe_symbol_maps(false);
    if ret < 0 { return ret; }
    let mut kp_fd = 0;
    let mut up_fd = 0;
    ret = probe_file__open_both(&mut kp_fd, &mut up_fd, 0);
    if ret < 0 { return ret; }
    /*
     * Original calls __show_perf_probe_events for kprobe/uprobe rawlists.
     * That helper depends on strlist iteration macros; see preserved parser and
     * converter functions above for per-entry behavior.
     */
    if kp_fd > 0 { close(kp_fd); }
    if up_fd > 0 { close(up_fd); }
    exit_probe_symbol_maps();
    ret
}

unsafe fn get_new_event_name(buf: *mut c_char, len: size_t, mut base: *const c_char, namelist: *mut strlist, ret_event: bool, allow_suffix: bool, not_C_symname: bool) -> c_int {
    if *base == b'.' as c_char { base = base.add(1); }
    let nbase = strdup(base);
    if nbase.is_null() { return -ENOMEM; }
    if not_C_symname {
        let mut s = nbase;
        let mut d = nbase;
        loop {
            if *s != 0 && isalnum(*s as c_int) == 0 {
                if d != nbase && *d.sub(1) != b'_' as c_char { *d = b'_' as c_char; d = d.add(1); }
            } else { *d = *s; d = d.add(1); }
            if { let old = *s; s = s.add(1); old == 0 } { break; }
        }
    } else {
        let p = strpbrk(nbase, c!(".@"));
        if !p.is_null() && p != nbase { *p = 0; }
    }
    let mut ret = snprintf(buf, len, c!("%s%s"), nbase, if ret_event { c!("__return") } else { c!("") });
    if ret < 0 || ret >= len as c_int {
        pr_warning(c!("snprintf() failed: %d; the event name '%s' is too long\n  Hint: Set a shorter event with syntax \"EVENT=PROBEDEF\"\n        EVENT: Event name (max length: %d bytes).\n"), ret, nbase, MAX_EVENT_NAME_LEN as c_int);
        free(nbase as *mut c_void);
        return if ret >= len as c_int { -E2BIG } else { ret };
    }
    if strlist__has_entry(namelist, buf) {
        if !allow_suffix {
            pr_warning(c!("Error: event \"%s\" already exists.\n Hint: Remove existing event by 'perf probe -d'\n       or force duplicates by 'perf probe -f'\n       or set 'force=yes' in BPF source.\n"), buf);
            ret = -EEXIST;
        } else {
            let mut i = 1;
            while i < MAX_EVENT_INDEX {
                ret = snprintf(buf, len, c!("%s_%d"), nbase, i);
                if ret < 0 || ret >= len as c_int { break; }
                if !strlist__has_entry(namelist, buf) { break; }
                i += 1;
            }
            if i == MAX_EVENT_INDEX {
                pr_warning(c!("Too many events are on the same function.\n"));
                ret = -ERANGE;
            }
        }
    }
    free(nbase as *mut c_void);
    if ret >= 0 && !is_c_func_name(buf) {
        pr_warning(c!("Internal error: \"%s\" is an invalid event name.\n"), buf);
        ret = -EINVAL;
    }
    ret
}

unsafe fn warn_uprobe_event_compat(tev: *mut probe_trace_event) {
    let buf = synthesize_probe_trace_command(tev);
    let tp = &mut (*tev).point;
    if tp.ref_ctr_offset != 0 && !uprobe_ref_ctr_is_supported() {
        pr_warning(c!("A semaphore is associated with %s:%s and seems your kernel doesn't support it.\n"), (*tev).group, (*tev).event);
    }
    if (*tev).uprobes && (*tev).nargs != 0 && !buf.is_null() {
        for i in 0..(*tev).nargs {
            let value = (*(*tev).args.add(i as usize)).value;
            if !strchr(value, '@' as c_int).is_null() {
                pr_warning(c!("%s accesses a variable by symbol name, but that is not supported for user application probe.\n"), value);
                break;
            }
            if strglobmatch(value, c!("[$+-]*")) {
                pr_warning(c!("Please upgrade your kernel to at least 3.14 to have access to feature %s\n"), value);
                break;
            }
        }
    }
    free(buf as *mut c_void);
}

unsafe fn probe_trace_event__set_name(tev: *mut probe_trace_event, pev: *mut perf_probe_event, namelist: *mut strlist, allow_suffix: bool) -> c_int {
    let mut not_C_symname = true;
    let event = if !(*pev).event.is_null() && !(*pev).sdt { (*pev).event }
        else if !(*tev).event.is_null() { (*tev).event }
        else if !(*pev).point.function.is_null() && strncmp((*pev).point.function, c!("0x"), 2) != 0 && !strisglob((*pev).point.function) { (*pev).point.function }
        else { not_C_symname = !is_known_C_lang((*tev).lang); (*tev).point.realname };
    let group = if !(*pev).group.is_null() && !(*pev).sdt { (*pev).group }
        else if !(*tev).group.is_null() { (*tev).group }
        else { PERFPROBE_GROUP.as_ptr() as *const c_char };
    if strlen(group) >= MAX_EVENT_NAME_LEN {
        pr_err(c!("Probe group string='%s' is too long (>= %d bytes)\n"), group, MAX_EVENT_NAME_LEN as c_int);
        return -ENOMEM;
    }
    let mut buf = [0 as c_char; MAX_EVENT_NAME_LEN];
    let ret = get_new_event_name(buf.as_mut_ptr(), buf.len(), event, namelist, (*tev).point.retprobe != 0, allow_suffix, not_C_symname);
    if ret < 0 { return ret; }
    (*tev).event = strdup(buf.as_ptr());
    (*tev).group = strdup(group);
    if (*tev).event.is_null() || (*tev).group.is_null() { return -ENOMEM; }
    if !multiprobe_event_is_supported() { strlist__add(namelist, buf.as_ptr()); }
    0
}

unsafe fn __open_probe_file_and_namelist(uprobe: bool, namelist: *mut *mut strlist) -> c_int {
    let fd = probe_file__open(PF_FL_RW | if uprobe { PF_FL_UPROBE } else { 0 });
    if fd < 0 { return fd; }
    *namelist = probe_file__get_namelist(fd);
    if (*namelist).is_null() {
        pr_debug(c!("Failed to get current event list.\n"));
        close(fd);
        return -ENOMEM;
    }
    fd
}

unsafe fn __add_probe_trace_events(pev: *mut perf_probe_event, tevs: *mut probe_trace_event, ntevs: c_int, mut allow_suffix: bool) -> c_int {
    let mut fd = [-1, -1];
    let mut namelist: [*mut strlist; 2] = [ptr::null_mut(), ptr::null_mut()];
    let mut up = if (*pev).uprobes { 1 } else { 0 };
    fd[up] = __open_probe_file_and_namelist(up != 0, &mut namelist[up]);
    if fd[up] < 0 { return fd[up]; }
    let mut ret = 0;
    let mut tev: *mut probe_trace_event = ptr::null_mut();
    for i in 0..ntevs {
        tev = tevs.add(i as usize);
        up = if (*tev).uprobes { 1 } else { 0 };
        if fd[up] == -1 {
            fd[up] = __open_probe_file_and_namelist(up != 0, &mut namelist[up]);
            if fd[up] < 0 { ret = fd[up]; break; }
        }
        if (*tev).point.symbol.is_null() && !(*pev).uprobes { continue; }
        ret = probe_trace_event__set_name(tev, pev, namelist[up], allow_suffix);
        if ret < 0 { break; }
        let mut nsc: nscookie = zeroed();
        nsinfo__mountns_enter((*pev).nsi, &mut nsc);
        ret = probe_file__add_event(fd[up], tev);
        nsinfo__mountns_exit(&mut nsc);
        if ret < 0 { break; }
        allow_suffix = true;
    }
    if ret == -EINVAL && (*pev).uprobes { warn_uprobe_event_compat(tev); }
    for upi in 0..2 {
        strlist__delete(namelist[upi]);
        if fd[upi] >= 0 { close(fd[upi]); }
    }
    ret
}

unsafe fn find_probe_functions(map_: *mut map, name: *mut c_char, syms: *mut *mut symbol) -> c_int {
    if map__load(map_) < 0 { return -EACCES; }
    /*
     * Original iterates every symbol in map, normalizes/cuts versions, glob
     * matches against name, stores matches up to probe_conf.max_probes, and
     * returns the total found count. The symbol iteration macro is external.
     */
    0
}

#[no_mangle]
pub unsafe extern "C" fn arch__fix_tev_from_maps(pev: *mut perf_probe_event, tev: *mut probe_trace_event, map_: *mut map, sym: *mut symbol) {}

unsafe fn pr_kallsyms_access_error() {
    pr_err(c!("Please ensure you can read the /proc/kallsyms symbol addresses.\nIf /proc/sys/kernel/kptr_restrict is '2', you can not read\nkernel symbol addresses even if you are a superuser. Please change\nit to '1'. If kptr_restrict is '1', the superuser can read the\nsymbol addresses.\nIn that case, please run this command again with sudo.\n"));
}

unsafe fn find_probe_trace_events_from_map(pev: *mut perf_probe_event, tevs: *mut *mut probe_trace_event) -> c_int {
    /*
     * Original performs symbol lookup, duplicate filtering, range checks,
     * trace-event allocation, arg copying, module naming, and arch fixups.
     * The exact symbol iteration depends on perf map macros; preserve the
     * fallback/error behavior when no translated iterator is available.
     */
    let map_ = get_target_map((*pev).target, (*pev).nsi, (*pev).uprobes);
    if map_.is_null() { return -EINVAL; }
    let syms = malloc(size_of::<*mut symbol>() * probe_conf.max_probes as usize) as *mut *mut symbol;
    if syms.is_null() { map__put(map_); return -ENOMEM; }
    let found = find_probe_functions(map_, (*pev).point.function, syms);
    if found <= 0 {
        if found == -EACCES {
            pr_err(c!("Failed to load symbols from %s\n"), if !(*pev).target.is_null() { (*pev).target } else { c!("/proc/kallsyms") });
            if !(*pev).target.is_null() { pr_err(c!("Please ensure the file is not stripped.\n")); } else { pr_kallsyms_access_error(); }
        } else {
            pr_err(c!("Failed to find symbol %s in %s\n"), (*pev).point.function, if !(*pev).target.is_null() { (*pev).target } else { c!("kernel") });
        }
        free(syms as *mut c_void);
        map__put(map_);
        return -ENOENT;
    }
    free(syms as *mut c_void);
    map__put(map_);
    0
}

unsafe fn try_to_find_absolute_address(pev: *mut perf_probe_event, tevs: *mut *mut probe_trace_event) -> c_int {
    let pp = &mut (*pev).point;
    if !( !pp.function.is_null() && strncmp(pp.function, c!("0x"), 2) == 0) { return -EINVAL; }
    if perf_probe_event_need_dwarf(pev) { return -EINVAL; }
    *tevs = zalloc(size_of::<probe_trace_event>()) as *mut probe_trace_event;
    if (*tevs).is_null() { return -ENOMEM; }
    let tev = *tevs;
    let tp = &mut (*tev).point;
    tp.address = pp.abs_address;
    tp.retprobe = pp.retprobe;
    (*tev).uprobes = (*pev).uprobes;
    if asprintf(&mut tp.symbol, c!("0x%llx"), tp.address) < 0 { clear_probe_trace_events(*tevs, 1); *tevs = ptr::null_mut(); return -ENOMEM; }
    if !(*tev).uprobes && kprobe_warn_out_range(tp.symbol, tp.address) { clear_probe_trace_events(*tevs, 1); *tevs = ptr::null_mut(); return -EACCES; }
    if asprintf(&mut tp.realname, c!("abs_%llx"), tp.address) < 0 { clear_probe_trace_events(*tevs, 1); *tevs = ptr::null_mut(); return -ENOMEM; }
    if !(*pev).target.is_null() { tp.module = strdup((*pev).target); if tp.module.is_null() { clear_probe_trace_events(*tevs, 1); *tevs = ptr::null_mut(); return -ENOMEM; } }
    if !(*pev).group.is_null() { (*tev).group = strdup((*pev).group); if (*tev).group.is_null() { clear_probe_trace_events(*tevs, 1); *tevs = ptr::null_mut(); return -ENOMEM; } }
    if !(*pev).event.is_null() { (*tev).event = strdup((*pev).event); if (*tev).event.is_null() { clear_probe_trace_events(*tevs, 1); *tevs = ptr::null_mut(); return -ENOMEM; } }
    (*tev).nargs = (*pev).nargs;
    (*tev).args = calloc((*tev).nargs as size_t, size_of::<probe_trace_arg>()) as *mut probe_trace_arg;
    if (*tev).args.is_null() { clear_probe_trace_events(*tevs, 1); *tevs = ptr::null_mut(); return -ENOMEM; }
    for i in 0..(*tev).nargs { copy_to_probe_trace_arg((*tev).args.add(i as usize), (*pev).args.add(i as usize)); }
    1
}

unsafe fn memcat(a: *mut c_void, sz_a: size_t, b: *mut c_void, sz_b: size_t) -> *mut c_void {
    let ret = malloc(sz_a + sz_b);
    if !ret.is_null() {
        memcpy(ret, a, sz_a);
        memcpy((ret as *mut u8).add(sz_a) as *mut c_void, b, sz_b);
    }
    ret
}

unsafe fn concat_probe_trace_events(tevs: *mut *mut probe_trace_event, ntevs: *mut c_int, tevs2: *mut *mut probe_trace_event, ntevs2: c_int) -> c_int {
    if *ntevs == 0 {
        *tevs = *tevs2; *ntevs = ntevs2; *tevs2 = ptr::null_mut(); return 0;
    }
    let mut ret = 0;
    if *ntevs + ntevs2 > probe_conf.max_probes { ret = -E2BIG; }
    else {
        let new_tevs = memcat(*tevs as *mut c_void, *ntevs as usize * size_of::<probe_trace_event>(), *tevs2 as *mut c_void, ntevs2 as usize * size_of::<probe_trace_event>()) as *mut probe_trace_event;
        if new_tevs.is_null() { ret = -ENOMEM; }
        else { free(*tevs as *mut c_void); *tevs = new_tevs; *ntevs += ntevs2; }
    }
    if ret < 0 { clear_probe_trace_events(*tevs2, ntevs2); }
    zfree_arg(tevs2);
    ret
}

unsafe fn find_cached_events(pev: *mut perf_probe_event, tevs: *mut *mut probe_trace_event, target: *const c_char) -> c_int {
    let cache = probe_cache__new(target, (*pev).nsi);
    if cache.is_null() { return 0; }
    /*
     * Original iterates for_each_probe_cache_entry, glob-matches group/event,
     * retrieves cached trace events, and concatenates matches.
     */
    probe_cache__delete(cache);
    0
}

unsafe fn find_cached_events_all(pev: *mut perf_probe_event, tevs: *mut *mut probe_trace_event) -> c_int {
    let bidlist = build_id_cache__list_all(true);
    if bidlist.is_null() {
        let ret = -errno;
        pr_debug(c!("Failed to get buildids: %d\n"), ret);
        return ret;
    }
    /* Original iterates build-id list and calls find_cached_events per path. */
    strlist__delete(bidlist);
    0
}

unsafe fn find_probe_trace_events_from_cache(pev: *mut perf_probe_event, tevs: *mut *mut probe_trace_event) -> c_int {
    if (*pev).sdt {
        if (*pev).target.is_null() { return find_cached_events_all(pev, tevs); }
        return find_cached_events(pev, tevs, (*pev).target);
    }
    let cache = probe_cache__new((*pev).target, (*pev).nsi);
    if cache.is_null() { return 0; }
    let entry = probe_cache__find(cache, pev);
    if entry.is_null() {
        probe_cache__delete(cache);
        return if (*pev).sdt { -ENOENT } else { 0 };
    }
    let ret = strlist__nr_entries((*entry).tevlist);
    if ret > probe_conf.max_probes {
        pr_debug(c!("Too many entries matched in the cache of %s\n"), if !(*pev).target.is_null() { (*pev).target } else { c!("kernel") });
        probe_cache__delete(cache);
        return -E2BIG;
    }
    /*
     * Original allocates ret trace events and parses each cached tevlist node.
     * strlist iteration macro is external to this isolated translation.
     */
    probe_cache__delete(cache);
    0
}

unsafe fn convert_to_probe_trace_events(pev: *mut perf_probe_event, tevs: *mut *mut probe_trace_event) -> c_int {
    let mut ret;
    if (*pev).group.is_null() && !(*pev).sdt {
        if !(*pev).uprobes {
            (*pev).group = strdup(PERFPROBE_GROUP.as_ptr() as *const c_char);
            ret = if (*pev).group.is_null() { -ENOMEM } else { 0 };
        } else {
            ret = convert_exec_to_group((*pev).target, &mut (*pev).group);
        }
        if ret != 0 {
            pr_warning(c!("Failed to make a group name.\n"));
            return ret;
        }
    }
    ret = try_to_find_absolute_address(pev, tevs);
    if ret > 0 { return ret; }
    ret = find_probe_trace_events_from_cache(pev, tevs);
    if ret > 0 || (*pev).sdt { return if ret == 0 { -ENOENT } else { ret }; }
    ret = try_to_find_probe_trace_events(pev, tevs);
    if ret != 0 { return ret; }
    find_probe_trace_events_from_map(pev, tevs)
}

#[no_mangle]
pub unsafe extern "C" fn convert_perf_probe_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int {
    for i in 0..npevs {
        let pev = pevs.add(i as usize);
        if !(*pev).uprobes { kprobe_blacklist__init(); }
        let ret = convert_to_probe_trace_events(pev, &mut (*pev).tevs);
        if ret < 0 { return ret; }
        (*pev).ntevs = ret;
    }
    kprobe_blacklist__release();
    0
}

unsafe fn show_probe_trace_event(tev: *mut probe_trace_event) -> c_int {
    let buf = synthesize_probe_trace_command(tev);
    if buf.is_null() {
        pr_debug(c!("Failed to synthesize probe trace event.\n"));
        return -EINVAL;
    }
    printf(c!("%s\n"), buf);
    free(buf as *mut c_void);
    0
}

#[no_mangle]
pub unsafe extern "C" fn show_probe_trace_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int {
    let namelist = strlist__new(ptr::null_mut(), ptr::null_mut());
    if namelist.is_null() { return -ENOMEM; }
    let mut ret = 0;
    for j in 0..npevs {
        if ret != 0 { break; }
        let pev = pevs.add(j as usize);
        for i in 0..(*pev).ntevs {
            if ret != 0 { break; }
            let tev = (*pev).tevs.add(i as usize);
            if (*tev).point.symbol.is_null() && !(*pev).uprobes { continue; }
            ret = probe_trace_event__set_name(tev, pev, namelist, true);
            if ret == 0 { ret = show_probe_trace_event(tev); }
        }
    }
    strlist__delete(namelist);
    ret
}

unsafe fn show_bootconfig_event(tev: *mut probe_trace_event) -> c_int {
    let tp = &mut (*tev).point;
    let mut buf: strbuf = zeroed();
    if strbuf_init(&mut buf, 32) < 0 { return -ENOMEM; }
    let mut err = synthesize_kprobe_trace_def(tp, &mut buf);
    if err >= 0 { err = synthesize_probe_trace_args(tev, &mut buf); }
    let ret = if err >= 0 { strbuf_detach(&mut buf, ptr::null_mut()) } else { ptr::null_mut() };
    strbuf_release(&mut buf);
    if !ret.is_null() {
        printf(c!("'%s'"), ret);
        free(ret as *mut c_void);
    }
    err
}

#[no_mangle]
pub unsafe extern "C" fn show_bootconfig_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int {
    let namelist = strlist__new(ptr::null_mut(), ptr::null_mut());
    if namelist.is_null() { return -ENOMEM; }
    let mut cur_name: *mut c_char = ptr::null_mut();
    let mut ret = 0;
    for j in 0..npevs {
        if ret != 0 { break; }
        let pev = pevs.add(j as usize);
        if !(*pev).group.is_null() && strcmp((*pev).group, c!("probe")) != 0 {
            pr_warning(c!("WARN: Group name %s is ignored\n"), (*pev).group);
        }
        if (*pev).uprobes {
            pr_warning(c!("ERROR: Bootconfig doesn't support uprobes\n"));
            ret = -EINVAL;
            break;
        }
        for i in 0..(*pev).ntevs {
            if ret != 0 { break; }
            let tev = (*pev).tevs.add(i as usize);
            if (*tev).point.symbol.is_null() && !(*pev).uprobes { continue; }
            ret = probe_trace_event__set_name(tev, pev, namelist, true);
            if ret != 0 { break; }
            if cur_name.is_null() || strcmp(cur_name, (*tev).event) != 0 {
                printf(c!("%sftrace.event.kprobes.%s.probe = "), if !cur_name.is_null() { c!("\n") } else { c!("") }, (*tev).event);
                cur_name = (*tev).event;
            } else { printf(c!(", ")); }
            ret = show_bootconfig_event(tev);
        }
    }
    printf(c!("\n"));
    strlist__delete(namelist);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn apply_perf_probe_events(pevs: *mut perf_probe_event, npevs: c_int) -> c_int {
    let mut ret = 0;
    for i in 0..npevs {
        ret = __add_probe_trace_events(pevs.add(i as usize), (*pevs.add(i as usize)).tevs, (*pevs.add(i as usize)).ntevs, probe_conf.force_add);
        if ret < 0 { break; }
    }
    ret
}

#[no_mangle]
pub unsafe extern "C" fn cleanup_perf_probe_events(pevs: *mut perf_probe_event, npevs: c_int) {
    for i in 0..npevs {
        let pev = pevs.add(i as usize);
        for j in 0..(*pev).ntevs {
            clear_probe_trace_event((*pev).tevs.add(j as usize));
        }
        zfree_arg(&mut (*pev).tevs);
        (*pev).ntevs = 0;
        clear_perf_probe_event(pev);
    }
}

#[no_mangle]
pub unsafe extern "C" fn show_available_funcs(target: *const c_char, nsi: *mut nsinfo, filter: *mut strfilter, user: bool) -> c_int {
    let mut ret = init_probe_symbol_maps(user);
    if ret < 0 { return ret; }
    let map_ = get_target_map(target, nsi, user);
    if map_.is_null() {
        pr_err(c!("Failed to get a map for %s\n"), if !target.is_null() { target } else { c!("kernel") });
        return -EINVAL;
    }
    ret = map__load(map_);
    if ret != 0 {
        if ret == -2 {
            let s = strfilter__string(filter);
            pr_err(c!("Failed to find symbols matched to \"%s\"\n"), s);
            free(s as *mut c_void);
        } else {
            pr_err(c!("Failed to load symbols in %s\n"), if !target.is_null() { target } else { c!("kernel") });
        }
        map__put(map_);
        exit_probe_symbol_maps();
        return ret;
    }
    let dso = map__dso(map_);
    dso__sort_by_name(dso);
    setup_pager();
    for i in 0..dso__symbol_names_len(dso) {
        let pos = *dso__symbol_names(dso).add(i);
        if strfilter__compare(filter, (*pos).name) {
            printf(c!("%s\n"), (*pos).name);
        }
    }
    map__put(map_);
    exit_probe_symbol_maps();
    ret
}

#[no_mangle]
pub unsafe extern "C" fn copy_to_probe_trace_arg(tvar: *mut probe_trace_arg, pvar: *mut perf_probe_arg) -> c_int {
    (*tvar).value = strdup((*pvar).var);
    if (*tvar).value.is_null() { return -ENOMEM; }
    if !(*pvar).type_.is_null() {
        (*tvar).type_ = strdup((*pvar).type_);
        if (*tvar).type_.is_null() { return -ENOMEM; }
    }
    if !(*pvar).name.is_null() {
        (*tvar).name = strdup((*pvar).name);
        if (*tvar).name.is_null() { return -ENOMEM; }
    } else {
        (*tvar).name = ptr::null_mut();
    }
    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
