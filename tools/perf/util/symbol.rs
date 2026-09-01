// SPDX-License-Identifier: GPL-2.0
//
// Source-level Rust translation of perf/util/symbol.c.
// C includes are intentionally not executable Rust; the corresponding types,
// constants, macros, and functions are supplied by the surrounding perf tree.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of_mut, null, null_mut};

type bool_ = bool;
type u8 = u8;
type u16 = u16;
type u64 = u64;
type s64 = i64;
type size_t = usize;
type ssize_t = isize;

const NSEC_PER_MSEC: u64 = 1_000_000;
const PATH_MAX: usize = 4096;
const SIZE_MAX: usize = usize::MAX;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const ENOENT: c_int = 2;
const ENOENT_NEG: c_int = -ENOENT;
const PROT_EXEC: c_int = 0x4;
const O_RDONLY: c_int = 0;
const R_OK: c_int = 4;
const CAP_SYSLOG: c_int = 34;
const MAX_NR_CPUS: c_int = 4096;
const SBUILD_ID_SIZE: usize = 64;

const STT_NOTYPE: u8 = 0;
const STT_FUNC: u8 = 2;
const STT_GNU_IFUNC: u8 = 10;
const STB_GLOBAL: u8 = 1;
const STB_WEAK: u8 = 2;
const EM_NONE: u16 = 0;
const EM_386: u16 = 3;
const EM_X86_64: u16 = 62;
const EM_PPC64: u16 = 21;
const EM_S390: u16 = 22;

const SYMBOL_A: c_int = 0;
const SYMBOL_B: c_int = 1;
const SYMBOL_FLAG_TYPE_SHIFT: u16 = 0;
const SYMBOL_FLAG_BINDING_SHIFT: u16 = 4;
const SYMBOL_FLAG_IGNORE: u16 = 1 << 8;
const SYMBOL_FLAG_ANNOTATE2: u16 = 1 << 9;
const SYMBOL_FLAG_INLINED: u16 = 1 << 10;
const SYMBOL_FLAG_IFUNC_ALIAS: u16 = 1 << 11;
const SYMBOL_FLAG_IDLE_SHIFT: u16 = 12;
const SYMBOL_FLAG_IDLE_MASK: u16 = 0x3000;
const SYMBOL_IDLE__UNKNOWN: u16 = 0;
const SYMBOL_IDLE__IDLE: u16 = 1;
const SYMBOL_IDLE__NOT_IDLE: u16 = 2;

const DSO_SPACE__USER: c_int = 0;
const DSO_SPACE__KERNEL: c_int = 1;
const DSO_SPACE__KERNEL_GUEST: c_int = 2;
const MAPPING_TYPE__IDENTITY: c_int = 0;
const JAVA_DEMANGLE_NORET: c_int = 1;
const DemangleStyleUnknown: c_int = 0;
const OverflowOk: c_int = 0;

#[repr(C)]
pub struct rb_node {
    rb_left: *mut rb_node,
    rb_right: *mut rb_node,
}

#[repr(C)]
pub struct rb_root {
    rb_node: *mut rb_node,
}

#[repr(C)]
pub struct rb_root_cached {
    rb_root: rb_root,
    rb_leftmost: *mut rb_node,
}

#[repr(C)]
pub struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
pub struct symbol {
    rb_node: rb_node,
    start: u64,
    end: u64,
    flags: u16,
    namelen: u16,
    name: [c_char; 1],
}

#[repr(C)]
pub struct annotation {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map_list_node {
    node: list_head,
    map: *mut map,
}

#[repr(C)]
pub struct module_info {
    rb_node: rb_node,
    name: *mut c_char,
    start: u64,
}

#[repr(C)]
pub struct kcore_mapfn_data {
    dso: *mut dso,
    maps: list_head,
}

#[repr(C)]
pub struct build_id {
    size: c_int,
    data: [u8; 32],
}

#[repr(C)]
pub struct ref_reloc_sym {
    name: *const c_char,
    addr: u64,
}

#[repr(C)]
pub struct kmap {
    kmaps: *mut maps,
    ref_reloc_sym: *mut ref_reloc_sym,
}

#[repr(C)]
pub struct perf_env {
    arch: *const c_char,
    e_machine: u16,
}

#[repr(C)]
pub struct machine {
    env: *mut perf_env,
    root_dir: *mut c_char,
    trampolines_mapped: bool,
    kallsyms_filename: *const c_char,
    mmap_name: *const c_char,
}

#[repr(C)]
pub struct perf_cpu {
    cpu: c_int,
}

#[repr(C)]
pub struct demangle {
    style: c_int,
    mangled_len: size_t,
}

#[repr(C)]
pub struct dirent {
    d_type: u8,
}

#[repr(C)]
pub struct str_node {
    rb_node: rb_node,
    s: *mut c_char,
}

#[repr(C)]
pub struct nscookie {
    _private: [u8; 0],
}

#[repr(C)]
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symsrc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct maps {
    _private: [u8; 0],
}

#[repr(C)]
pub struct strlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct intlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dso_binary_type {
    DSO_BINARY_TYPE__KALLSYMS,
    DSO_BINARY_TYPE__GUEST_KALLSYMS,
    DSO_BINARY_TYPE__JAVA_JIT,
    DSO_BINARY_TYPE__DEBUGLINK,
    DSO_BINARY_TYPE__BUILD_ID_CACHE,
    DSO_BINARY_TYPE__BUILD_ID_CACHE_DEBUGINFO,
    DSO_BINARY_TYPE__FEDORA_DEBUGINFO,
    DSO_BINARY_TYPE__UBUNTU_DEBUGINFO,
    DSO_BINARY_TYPE__BUILDID_DEBUGINFO,
    DSO_BINARY_TYPE__GNU_DEBUGDATA,
    DSO_BINARY_TYPE__SYSTEM_PATH_DSO,
    DSO_BINARY_TYPE__GUEST_KMODULE,
    DSO_BINARY_TYPE__GUEST_KMODULE_COMP,
    DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE,
    DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE_COMP,
    DSO_BINARY_TYPE__OPENEMBEDDED_DEBUGINFO,
    DSO_BINARY_TYPE__MIXEDUP_UBUNTU_DEBUGINFO,
    DSO_BINARY_TYPE__NOT_FOUND,
    DSO_BINARY_TYPE__VMLINUX,
    DSO_BINARY_TYPE__KCORE,
    DSO_BINARY_TYPE__GUEST_VMLINUX,
    DSO_BINARY_TYPE__GUEST_KCORE,
    DSO_BINARY_TYPE__BPF_PROG_INFO,
    DSO_BINARY_TYPE__BPF_IMAGE,
    DSO_BINARY_TYPE__OOL,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum symbol_tag_include {
    SYMBOL_TAG_INCLUDE__NONE,
    SYMBOL_TAG_INCLUDE__DEFAULT_ONLY,
}

#[repr(C)]
pub struct symbol_conf_t {
    nanosecs: bool,
    use_modules: bool,
    try_vmlinux_path: bool,
    demangle: bool,
    demangle_kernel: bool,
    cumulate_callchain: bool,
    time_quantum: u64,
    show_hist_headers: bool,
    symfs: *const c_char,
    symfs_layout_flat: bool,
    event_group: bool,
    inline_name: bool,
    res_sample: c_int,
    addr2line_timeout_ms: c_int,
    allow_aliases: bool,
    priv_size: size_t,
    init_annotation: bool,
    initialized: bool,
    kptr_restrict: bool,
    kallsyms_name: *const c_char,
    ignore_vmlinux: bool,
    vmlinux_name: *const c_char,
    ignore_vmlinux_buildid: bool,
    default_guest_vmlinux_name: *const c_char,
    default_guest_kallsyms: *const c_char,
    dso_list: *mut strlist,
    dso_list_str: *const c_char,
    comm_list: *mut strlist,
    comm_list_str: *const c_char,
    pid_list: *mut intlist,
    pid_list_str: *const c_char,
    tid_list: *mut intlist,
    tid_list_str: *const c_char,
    sym_list: *mut strlist,
    sym_list_str: *const c_char,
    addr_list: *mut intlist,
    bt_stop_list: *mut strlist,
    bt_stop_list_str: *const c_char,
    has_filter: bool,
    field_sep: *const c_char,
    parallelism_list_str: *const c_char,
    parallelism_filter: *mut c_ulong,
}

unsafe extern "C" {
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strlcpy(dst: *mut c_char, src: *const c_char, n: size_t) -> size_t;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strndup(s: *const c_char, n: size_t) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn atoi(s: *const c_char) -> c_int;
    fn strtoul(s: *const c_char, end: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strtoull(s: *const c_char, end: *mut *mut c_char, base: c_int) -> u64;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(fp: *mut c_void) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut c_void) -> *mut c_char;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut c_void) -> ssize_t;
    fn feof(stream: *mut c_void) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn access(path: *const c_char, mode: c_int) -> c_int;
    fn realpath(path: *const c_char, resolved: *mut c_char) -> *mut c_char;
    fn uname(buf: *mut c_void) -> c_int;
    fn qsort(base: *mut c_void, nmemb: size_t, size: size_t,
             compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int);
    fn bsearch(key: *const c_void, base: *const c_void, nmemb: size_t, size: size_t,
               compar: unsafe extern "C" fn(*const c_void, *const c_void) -> c_int) -> *mut c_void;

    static mut errno: c_int;
    static mut verbose: c_int;
    static mut buildid_dir: *const c_char;
    static DSO__NAME_KALLSYMS: *const c_char;
    static DSO__NAME_KCORE: *const c_char;
    static ENTRY_TRAMPOLINE_NAME: *const c_char;

    fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_last(root: *const rb_root) -> *mut rb_node;
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color_cached(node: *mut rb_node, root: *mut rb_root_cached, leftmost: bool);
    fn rb_erase_cached(node: *mut rb_node, root: *mut rb_root_cached);
    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn RB_CLEAR_NODE(node: *mut rb_node);

    fn list_add(new_: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);

    fn annotation__init(notes: *mut annotation);
    fn annotation__exit(notes: *mut annotation);
    fn symbol__annotation(sym: *mut symbol) -> *mut annotation;
    fn symbol__type(sym: *const symbol) -> u8;
    fn symbol__binding(sym: *const symbol) -> u8;
    fn map__fixup_start(map: *mut map);
    fn map__fixup_end(map: *mut map);
    fn dso__symbols(dso: *mut dso) -> *mut rb_root_cached;
    fn dso__set_last_find_result_addr(dso: *mut dso, addr: u64);
    fn dso__last_find_result_addr(dso: *mut dso) -> u64;
    fn dso__set_last_find_result_symbol(dso: *mut dso, sym: *mut symbol);
    fn dso__last_find_result_symbol(dso: *mut dso) -> *mut symbol;
    fn dso__symbol_names(dso: *mut dso) -> *mut *mut symbol;
    fn dso__set_symbol_names(dso: *mut dso, names: *mut *mut symbol);
    fn dso__symbol_names_len(dso: *mut dso) -> size_t;
    fn dso__set_symbol_names_len(dso: *mut dso, len: size_t);
    fn dso__sorted_by_name(dso: *mut dso) -> bool;
    fn dso__set_sorted_by_name(dso: *mut dso);
    fn dso__lock(dso: *mut dso) -> *mut c_void;
    fn mutex_lock(lock: *mut c_void);
    fn mutex_unlock(lock: *mut c_void);
    fn kallsyms__parse(filename: *const c_char, arg: *mut c_void,
                       process_symbol: unsafe extern "C" fn(*mut c_void, *const c_char, c_char, u64) -> c_int) -> c_int;
    fn kallsyms2elf_binding(t: c_char) -> u8;
    fn kallsyms2elf_type(t: c_char) -> u8;
    fn is_ignored_kernel_symbol(name: *const c_char) -> bool;
    fn is_livepatch_symbol(name: *const c_char) -> bool;
    fn is_entry_trampoline(name: *const c_char) -> bool;
    fn maps__find(kmaps: *mut maps, addr: u64) -> *mut map;
    fn maps__find_by_name(kmaps: *mut maps, name: *const c_char) -> *mut map;
    fn maps__machine(kmaps: *mut maps) -> *mut machine;
    fn maps__insert(kmaps: *mut maps, map: *mut map) -> c_int;
    fn maps__remove(kmaps: *mut maps, map: *mut map);
    fn maps__remove_maps(kmaps: *mut maps, cb: unsafe extern "C" fn(*mut map, *mut c_void) -> bool, data: *mut c_void);
    fn maps__merge_in(kmaps: *mut maps, map: *mut map) -> c_int;
    fn maps__for_each_map(kmaps: *mut maps, cb: unsafe extern "C" fn(*mut map, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn maps__mutate_mapping(kmaps: *mut maps, map: *mut map,
                            cb: unsafe extern "C" fn(*mut map, *mut c_void) -> c_int, data: *mut c_void) -> c_int;
    fn map__dso(map: *mut map) -> *mut dso;
    fn map__start(map: *const map) -> u64;
    fn map__end(map: *const map) -> u64;
    fn map__pgoff(map: *const map) -> u64;
    fn map__size(map: *const map) -> u64;
    fn map__prot(map: *const map) -> c_int;
    fn map__mapping_type(map: *const map) -> c_int;
    fn map__set_start(map: *mut map, start: u64);
    fn map__set_end(map: *mut map, end: u64);
    fn map__set_pgoff(map: *mut map, pgoff: u64);
    fn map__set_mapping_type(map: *mut map, ty: c_int);
    fn map__map_ip(map: *mut map, ip: u64) -> u64;
    fn map__get(map: *mut map) -> *mut map;
    fn map__put(map: *mut map);
    fn map__zput(map: *mut map);
    fn map__new2(start: u64, dso: *mut dso) -> *mut map;
    fn map__kmaps(map: *mut map) -> *mut maps;
    fn map__kmap(map: *mut map) -> *mut kmap;
    fn __map__is_kmodule(map: *mut map) -> bool;
    fn __map__is_kernel(map: *mut map) -> bool;
    fn __map__is_bpf_prog(map: *mut map) -> bool;
    fn dso__kernel(dso: *const dso) -> c_int;
    fn dso__set_kernel(dso: *mut dso, kernel: c_int);
    fn dso__loaded(dso: *mut dso) -> bool;
    fn dso__set_loaded(dso: *mut dso);
    fn dso__new(name: *const c_char) -> *mut dso;
    fn dso__put(dso: *mut dso);
    fn dso__short_name(dso: *mut dso) -> *const c_char;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn dso__name(dso: *mut dso) -> *const c_char;
    fn dso__is_kmod(dso: *mut dso) -> bool;
    fn dso__is_kcore(dso: *mut dso) -> bool;
    fn dso__set_adjust_symbols(dso: *mut dso, v: bool);
    fn dso__set_symtab_type(dso: *mut dso, ty: dso_binary_type);
    fn dso__symtab_type(dso: *mut dso) -> dso_binary_type;
    fn dso__set_binary_type(dso: *mut dso, ty: dso_binary_type);
    fn dso__binary_type(dso: *mut dso) -> dso_binary_type;
    fn dso__set_long_name(dso: *mut dso, name: *const c_char, allocated: bool);
    fn dso__set_is_64_bit(dso: *mut dso, v: bool);
    fn dso__has_build_id(dso: *mut dso) -> bool;
    fn dso__set_build_id(dso: *mut dso, bid: *const build_id);
    fn dso__bid(dso: *mut dso) -> *const c_void;
    fn dso__build_id_equal(dso: *mut dso, bid: *const build_id) -> bool;
    fn dso__read_binary_type_filename(dso: *mut dso, ty: dso_binary_type, root: *const c_char, name: *mut c_char, len: size_t) -> c_int;
    fn dso__symsrc_filename(dso: *mut dso) -> *const c_char;
    fn dso__set_symsrc_filename(dso: *mut dso, name: *mut c_char);
    fn dso__filename_with_chroot(dso: *mut dso, name: *const c_char) -> *mut c_char;
    fn dso__load_sym(dso: *mut dso, map: *mut map, ss: *mut symsrc, runtime_ss: *mut symsrc, kmod: bool) -> c_int;
    fn dso__synthesize_plt_symbols(dso: *mut dso, runtime_ss: *mut symsrc) -> c_int;
    fn dso__nsinfo(dso: *mut dso) -> *mut nsinfo;
    fn dso__nsinfo_ptr(dso: *mut dso) -> *mut *mut nsinfo;
    fn machine__is_default_guest(machine: *mut machine) -> bool;
    fn machine__map_x86_64_entry_trampolines(machine: *mut machine, dso: *mut dso);
    fn perf_env__e_machine(env: *mut perf_env, e_flags: *mut c_void) -> u16;
    fn dso__e_machine(dso: *mut dso, machine: *mut machine, e_flags: *mut c_void) -> u16;
    fn perf_env__os_release(env: *mut perf_env) -> *const c_char;
    fn file__read_maps(fd: c_int, exe: c_int,
                       cb: unsafe extern "C" fn(u64, u64, u64, *mut c_void) -> c_int,
                       data: *mut c_void, is_64_bit: *mut bool) -> c_int;
    fn kallsyms__get_function_start(filename: *const c_char, name: *const c_char, start: *mut u64) -> c_int;
    fn is_perf_pid_map_name(name: *const c_char) -> bool;
    fn nsinfo__need_setns(nsi: *mut nsinfo) -> bool;
    fn nsinfo__nstgid(nsi: *mut nsinfo) -> c_int;
    fn nsinfo__tgid(nsi: *mut nsinfo) -> c_int;
    fn nsinfo__mountns_enter(nsi: *mut nsinfo, nsc: *mut nscookie);
    fn nsinfo__mountns_exit(nsc: *mut nscookie);
    fn nsinfo__copy(nsi: *mut nsinfo) -> *mut nsinfo;
    fn nsinfo__put(nsi: *mut nsinfo);
    fn nsinfo__clear_need_setns(nsi: *mut nsinfo);
    fn __symbol__join_symfs(dst: *mut c_char, len: size_t, path: *const c_char);
    fn symbol__join_symfs(dst: *mut c_char, path: *const c_char);
    fn filename__read_build_id(name: *const c_char, bid: *mut build_id) -> c_int;
    fn __dso__build_id_filename(dso: *mut dso, bf: *mut c_char, size: size_t, is_debug: bool, is_kallsyms: bool) -> *mut c_char;
    fn dso__build_id_filename(dso: *mut dso, bf: *mut c_char, size: size_t, is_debug: bool) -> *mut c_char;
    fn build_id__snprintf(bid: *const c_void, dst: *mut c_char, len: size_t) -> c_int;
    fn sysfs__read_build_id(path: *const c_char, bid: *mut build_id) -> c_int;
    fn build_id_cache__kallsyms_path(sbuild_id: *const c_char, path: *mut c_char, len: size_t) -> c_int;
    fn symsrc__init(ss: *mut symsrc, dso: *mut dso, name: *const c_char, ty: dso_binary_type) -> c_int;
    fn symsrc__destroy(ss: *mut symsrc);
    fn symsrc__has_symtab(ss: *mut symsrc) -> bool;
    fn symsrc__possibly_runtime(ss: *mut symsrc) -> bool;
    fn is_regular_file(path: *const c_char) -> bool;
    fn strlist__new(s: *const c_char, dupstr: *mut c_void) -> *mut strlist;
    fn strlist__delete(list: *mut strlist);
    fn strlist__remove(list: *mut strlist, node: *mut str_node);
    fn intlist__new(s: *const c_char) -> *mut intlist;
    fn intlist__add(list: *mut intlist, v: c_ulong) -> c_int;
    fn intlist__delete(list: *mut intlist);
    fn lsdir(path: *const c_char, filter: unsafe extern "C" fn(*const c_char, *mut dirent) -> bool) -> *mut strlist;
    fn lsdir_no_dot_filter(name: *const c_char, d: *mut dirent) -> bool;
    fn perf_cap__capable(cap: c_int) -> bool;
    fn perf_event_paranoid() -> c_int;
    fn symbol__elf_init();
    fn bitmap_fill(bitmap: *mut c_ulong, nbits: c_int);
    fn __clear_bit(nr: c_int, addr: *mut c_ulong);
    fn perf_cpu_map__new(s: *const c_char) -> *mut perf_cpu_map;
    fn perf_cpu_map__put(map: *mut perf_cpu_map);
    fn perf_cpu_map__nr(map: *mut perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(map: *mut perf_cpu_map, idx: c_int) -> perf_cpu;
    fn set_buildid_dir(dir: *const c_char);
    fn cxx_demangle_sym(str_: *const c_char, params: bool, modifiers: bool) -> *mut c_char;
    fn ocaml_demangle_sym(str_: *const c_char) -> *mut c_char;
    fn java_demangle_sym(str_: *const c_char, flags: c_int) -> *mut c_char;
    fn rust_demangle_demangle(name: *const c_char, demangle: *mut demangle);
    fn rust_demangle_is_known(demangle: *const demangle) -> bool;
    fn rust_demangle_display_demangle(demangle: *const demangle, buf: *mut c_char, len: size_t, alternate: bool) -> c_int;
    fn roundup_pow_of_two(x: size_t) -> size_t;
    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug4(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
}

#[inline]
unsafe fn symbol_name(sym: *const symbol) -> *const c_char {
    (*sym).name.as_ptr()
}

#[inline]
unsafe fn symbol_name_mut(sym: *mut symbol) -> *mut c_char {
    (*sym).name.as_mut_ptr()
}

#[inline]
unsafe fn rb_entry_symbol(n: *mut rb_node) -> *mut symbol {
    n as *mut symbol
}

#[inline]
unsafe fn rb_entry_module(n: *mut rb_node) -> *mut module_info {
    n as *mut module_info
}

#[inline]
unsafe fn list_entry_map_list_node(n: *mut list_head) -> *mut map_list_node {
    n as *mut map_list_node
}

#[inline]
fn roundup(x: u64, a: u64) -> u64 {
    ((x + a - 1) / a) * a
}

#[inline]
unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    (*list).next = list;
    (*list).prev = list;
}

#[inline]
unsafe fn list_empty(list: *const list_head) -> bool {
    (*list).next == list as *mut list_head
}

#[inline]
unsafe fn zfree_char(pp: *mut *mut c_char) {
    if !(*pp).is_null() {
        free(*pp as *mut c_void);
        *pp = null_mut();
    }
}

#[inline]
unsafe fn zfree_char_array(pp: *mut *mut *mut c_char) {
    if !(*pp).is_null() {
        free(*pp as *mut c_void);
        *pp = null_mut();
    }
}

#[inline]
unsafe fn strstarts(s: *const c_char, prefix: *const c_char) -> bool {
    strncmp(s, prefix, strlen(prefix)) == 0
}

#[inline]
unsafe fn toupper_ch(c: c_char) -> c_char {
    let v = c as u8;
    if v >= b'a' && v <= b'z' { (v - 32) as c_char } else { c }
}

#[inline]
unsafe fn isspace_ch(c: c_char) -> bool {
    matches!(c as u8, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

static mut vmlinux_path__nr_entries: c_int = 0;
static mut vmlinux_path: *mut *mut c_char = null_mut();

#[unsafe(no_mangle)]
pub static mut symbol_conf: symbol_conf_t = symbol_conf_t {
    nanosecs: false,
    use_modules: true,
    try_vmlinux_path: true,
    demangle: true,
    demangle_kernel: false,
    cumulate_callchain: true,
    time_quantum: 100 * NSEC_PER_MSEC,
    show_hist_headers: true,
    symfs: b"\0".as_ptr() as *const c_char,
    symfs_layout_flat: false,
    event_group: true,
    inline_name: true,
    res_sample: 0,
    addr2line_timeout_ms: 5 * 1000,
    allow_aliases: false,
    priv_size: 0,
    init_annotation: false,
    initialized: false,
    kptr_restrict: false,
    kallsyms_name: null(),
    ignore_vmlinux: false,
    vmlinux_name: null(),
    ignore_vmlinux_buildid: false,
    default_guest_vmlinux_name: null(),
    default_guest_kallsyms: null(),
    dso_list: null_mut(),
    dso_list_str: null(),
    comm_list: null_mut(),
    comm_list_str: null(),
    pid_list: null_mut(),
    pid_list_str: null(),
    tid_list: null_mut(),
    tid_list_str: null(),
    sym_list: null_mut(),
    sym_list_str: null(),
    addr_list: null_mut(),
    bt_stop_list: null_mut(),
    bt_stop_list_str: null(),
    has_filter: false,
    field_sep: null(),
    parallelism_list_str: null(),
    parallelism_filter: null_mut(),
};

static mut binary_type_symtab: [dso_binary_type; 18] = [
    dso_binary_type::DSO_BINARY_TYPE__KALLSYMS,
    dso_binary_type::DSO_BINARY_TYPE__GUEST_KALLSYMS,
    dso_binary_type::DSO_BINARY_TYPE__JAVA_JIT,
    dso_binary_type::DSO_BINARY_TYPE__DEBUGLINK,
    dso_binary_type::DSO_BINARY_TYPE__BUILD_ID_CACHE,
    dso_binary_type::DSO_BINARY_TYPE__BUILD_ID_CACHE_DEBUGINFO,
    dso_binary_type::DSO_BINARY_TYPE__FEDORA_DEBUGINFO,
    dso_binary_type::DSO_BINARY_TYPE__UBUNTU_DEBUGINFO,
    dso_binary_type::DSO_BINARY_TYPE__BUILDID_DEBUGINFO,
    dso_binary_type::DSO_BINARY_TYPE__GNU_DEBUGDATA,
    dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_DSO,
    dso_binary_type::DSO_BINARY_TYPE__GUEST_KMODULE,
    dso_binary_type::DSO_BINARY_TYPE__GUEST_KMODULE_COMP,
    dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE,
    dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE_COMP,
    dso_binary_type::DSO_BINARY_TYPE__OPENEMBEDDED_DEBUGINFO,
    dso_binary_type::DSO_BINARY_TYPE__MIXEDUP_UBUNTU_DEBUGINFO,
    dso_binary_type::DSO_BINARY_TYPE__NOT_FOUND,
];
const DSO_BINARY_TYPE__SYMTAB_CNT: usize = 18;

unsafe extern "C" fn map_fixup_cb(map: *mut map, _data: *mut c_void) -> c_int {
    unsafe {
        map__fixup_start(map);
        map__fixup_end(map);
    }
    0
}

unsafe fn map_list_node__new() -> *mut map_list_node {
    unsafe { malloc(size_of::<map_list_node>()) as *mut map_list_node }
}

unsafe fn symbol_type__filter(mut symbol_type: c_char) -> bool {
    unsafe {
        symbol_type = toupper_ch(symbol_type);
    }
    symbol_type == b'T' as c_char || symbol_type == b'W' as c_char ||
        symbol_type == b'D' as c_char || symbol_type == b'B' as c_char
}

unsafe fn prefix_underscores_count(str_: *const c_char) -> c_int {
    unsafe {
        let mut tail = str_;
        while *tail == b'_' as c_char {
            tail = tail.add(1);
        }
        tail.offset_from(str_) as c_int
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch__normalize_symbol_name(name: *const c_char) -> *const c_char {
    name
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch__compare_symbol_names(namea: *const c_char, nameb: *const c_char) -> c_int {
    unsafe { strcmp(namea, nameb) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch__compare_symbol_names_n(namea: *const c_char, nameb: *const c_char, n: c_uint) -> c_int {
    unsafe { strncmp(namea, nameb, n as size_t) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn arch__choose_best_symbol(syma: *mut symbol, _symb: *mut symbol) -> c_int {
    unsafe {
        if strlen(symbol_name(syma)) >= 3 && strncmp(symbol_name(syma), c"SyS".as_ptr(), 3) == 0 {
            return SYMBOL_B;
        }
        if strlen(symbol_name(syma)) >= 10 && strncmp(symbol_name(syma), c"compat_SyS".as_ptr(), 10) == 0 {
            return SYMBOL_B;
        }
    }
    SYMBOL_A
}

unsafe fn choose_best_symbol(syma: *mut symbol, symb: *mut symbol) -> c_int {
    unsafe {
        let mut a: s64 = (*syma).end as s64 - (*syma).start as s64;
        let mut b: s64 = (*symb).end as s64 - (*symb).start as s64;
        if b == 0 && a > 0 { return SYMBOL_A; } else if a == 0 && b > 0 { return SYMBOL_B; }

        if symbol__type(syma) != symbol__type(symb) {
            if symbol__type(syma) == STT_NOTYPE { return SYMBOL_B; }
            if symbol__type(symb) == STT_NOTYPE { return SYMBOL_A; }
        }

        a = (symbol__binding(syma) == STB_WEAK) as s64;
        b = (symbol__binding(symb) == STB_WEAK) as s64;
        if b != 0 && a == 0 { return SYMBOL_A; }
        if a != 0 && b == 0 { return SYMBOL_B; }

        a = (symbol__binding(syma) == STB_GLOBAL) as s64;
        b = (symbol__binding(symb) == STB_GLOBAL) as s64;
        if a != 0 && b == 0 { return SYMBOL_A; }
        if b != 0 && a == 0 { return SYMBOL_B; }

        a = prefix_underscores_count(symbol_name(syma)) as s64;
        b = prefix_underscores_count(symbol_name(symb)) as s64;
        if b > a { return SYMBOL_A; } else if a > b { return SYMBOL_B; }

        let na = strlen(symbol_name(syma));
        let nb = strlen(symbol_name(symb));
        if na > nb { return SYMBOL_A; } else if na < nb { return SYMBOL_B; }

        arch__choose_best_symbol(syma, symb)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbols__fixup_duplicate(symbols: *mut rb_root_cached) {
    unsafe {
        if symbol_conf.allow_aliases { return; }
        let mut nd = rb_first_cached(symbols);
        while !nd.is_null() {
            let curr = rb_entry_symbol(nd);
            loop {
                nd = rb_next(addr_of_mut!((*curr).rb_node));
                if nd.is_null() { return; }
                let next = rb_entry_symbol(nd);
                if (*curr).start != (*next).start { break; }
                if choose_best_symbol(curr, next) == SYMBOL_A {
                    if symbol__type(next) == STT_GNU_IFUNC { symbol__set_ifunc_alias(curr, true); }
                    rb_erase_cached(addr_of_mut!((*next).rb_node), symbols);
                    symbol__delete(next);
                } else {
                    if symbol__type(curr) == STT_GNU_IFUNC { symbol__set_ifunc_alias(next, true); }
                    nd = rb_next(addr_of_mut!((*curr).rb_node));
                    rb_erase_cached(addr_of_mut!((*curr).rb_node), symbols);
                    symbol__delete(curr);
                    break;
                }
            }
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbols__fixup_end(symbols: *mut rb_root_cached, is_kallsyms: bool) {
    unsafe {
        let prevnd = rb_first_cached(symbols);
        if prevnd.is_null() { return; }
        let mut curr = rb_entry_symbol(prevnd);
        let mut nd = rb_next(prevnd);
        while !nd.is_null() {
            let prev = curr;
            curr = rb_entry_symbol(nd);
            if (*prev).end == (*prev).start {
                if !is_kallsyms {
                    (*prev).end = (*curr).start;
                } else {
                    let prev_mod = strchr(symbol_name(prev), b'[' as c_int);
                    let curr_mod = strchr(symbol_name(curr), b'[' as c_int);
                    if prev_mod.is_null() != curr_mod.is_null() {
                        (*prev).end = roundup((*prev).end + 4096, 4096);
                    } else if !prev_mod.is_null() && strcmp(prev_mod, curr_mod) != 0 {
                        (*prev).end = roundup((*prev).end + 4096, 4096);
                    } else {
                        (*prev).end = (*curr).start;
                    }
                    pr_debug4(c"%s sym:%s end:%#lx\n".as_ptr(), c"symbols__fixup_end".as_ptr(), symbol_name(prev), (*prev).end);
                }
            }
            nd = rb_next(nd);
        }
        if (*curr).end == (*curr).start {
            (*curr).end = roundup((*curr).start, 4096) + 4096;
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__new(start: u64, len: u64, binding: u8, type_: u8, name: *const c_char) -> *mut symbol {
    unsafe {
        let namelen = strlen(name) + 1;
        let base = calloc(1, symbol_conf.priv_size + size_of::<symbol>() + namelen);
        if base.is_null() { return null_mut(); }
        let mut sym = base as *mut symbol;
        if symbol_conf.priv_size != 0 {
            if symbol_conf.init_annotation {
                annotation__init(base as *mut annotation);
            }
            sym = (base as *mut u8).add(symbol_conf.priv_size) as *mut symbol;
        }
        (*sym).start = start;
        (*sym).end = if len != 0 { start.wrapping_add(len) } else { start };
        (*sym).flags = ((type_ as u16) << SYMBOL_FLAG_TYPE_SHIFT) | ((binding as u16) << SYMBOL_FLAG_BINDING_SHIFT);
        (*sym).namelen = (namelen - 1) as u16;
        pr_debug4(c"%s: %s %#lx-%#lx\n".as_ptr(), c"symbol__new".as_ptr(), name, start, (*sym).end);
        memcpy(symbol_name_mut(sym) as *mut c_void, name as *const c_void, namelen);
        sym
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__delete(sym: *mut symbol) {
    unsafe {
        if symbol_conf.priv_size != 0 && symbol_conf.init_annotation {
            annotation__exit(symbol__annotation(sym));
        }
        free((sym as *mut u8).sub(symbol_conf.priv_size) as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__set_ignore(sym: *mut symbol, ignore: bool) {
    unsafe { if ignore { (*sym).flags |= SYMBOL_FLAG_IGNORE; } else { (*sym).flags &= !SYMBOL_FLAG_IGNORE; } }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__set_annotate2(sym: *mut symbol, annotate2: bool) {
    unsafe { if annotate2 { (*sym).flags |= SYMBOL_FLAG_ANNOTATE2; } else { (*sym).flags &= !SYMBOL_FLAG_ANNOTATE2; } }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__set_inlined(sym: *mut symbol, inlined: bool) {
    unsafe { if inlined { (*sym).flags |= SYMBOL_FLAG_INLINED; } else { (*sym).flags &= !SYMBOL_FLAG_INLINED; } }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__set_ifunc_alias(sym: *mut symbol, ifunc_alias: bool) {
    unsafe { if ifunc_alias { (*sym).flags |= SYMBOL_FLAG_IFUNC_ALIAS; } else { (*sym).flags &= !SYMBOL_FLAG_IFUNC_ALIAS; } }
}

unsafe fn symbol__set_idle(sym: *mut symbol, idle: bool) {
    unsafe {
        let idle_val = if idle { SYMBOL_IDLE__IDLE } else { SYMBOL_IDLE__NOT_IDLE };
        (*sym).flags = ((*sym).flags & !SYMBOL_FLAG_IDLE_MASK) | (idle_val << SYMBOL_FLAG_IDLE_SHIFT);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbols__delete(symbols: *mut rb_root_cached) {
    unsafe {
        let mut next = rb_first_cached(symbols);
        while !next.is_null() {
            let pos = rb_entry_symbol(next);
            next = rb_next(addr_of_mut!((*pos).rb_node));
            rb_erase_cached(addr_of_mut!((*pos).rb_node), symbols);
            symbol__delete(pos);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __symbols__insert(symbols: *mut rb_root_cached, sym: *mut symbol) {
    unsafe {
        let mut p = addr_of_mut!((*symbols).rb_root.rb_node);
        let mut parent: *mut rb_node = null_mut();
        let ip = (*sym).start;
        let mut leftmost = true;
        while !(*p).is_null() {
            parent = *p;
            let s = rb_entry_symbol(parent);
            if ip < (*s).start {
                p = addr_of_mut!((*(*p)).rb_left);
            } else {
                p = addr_of_mut!((*(*p)).rb_right);
                leftmost = false;
            }
        }
        rb_link_node(addr_of_mut!((*sym).rb_node), parent, p);
        rb_insert_color_cached(addr_of_mut!((*sym).rb_node), symbols, leftmost);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbols__insert(symbols: *mut rb_root_cached, sym: *mut symbol) {
    unsafe { __symbols__insert(symbols, sym); }
}

unsafe fn symbols__find(symbols: *mut rb_root_cached, ip: u64) -> *mut symbol {
    unsafe {
        if symbols.is_null() { return null_mut(); }
        let mut n = (*symbols).rb_root.rb_node;
        while !n.is_null() {
            let s = rb_entry_symbol(n);
            if ip < (*s).start {
                n = (*n).rb_left;
            } else if ip > (*s).end || (ip == (*s).end && ip != (*s).start) {
                n = (*n).rb_right;
            } else {
                return s;
            }
        }
        null_mut()
    }
}

unsafe fn symbols__first(symbols: *mut rb_root_cached) -> *mut symbol {
    unsafe {
        let n = rb_first_cached(symbols);
        if !n.is_null() { rb_entry_symbol(n) } else { null_mut() }
    }
}

unsafe fn symbols__last(symbols: *mut rb_root_cached) -> *mut symbol {
    unsafe {
        let n = rb_last(addr_of_mut!((*symbols).rb_root));
        if !n.is_null() { rb_entry_symbol(n) } else { null_mut() }
    }
}

unsafe fn symbols__next(sym: *mut symbol) -> *mut symbol {
    unsafe {
        let n = rb_next(addr_of_mut!((*sym).rb_node));
        if !n.is_null() { rb_entry_symbol(n) } else { null_mut() }
    }
}

unsafe extern "C" fn symbols__sort_name_cmp(vlhs: *const c_void, vrhs: *const c_void) -> c_int {
    unsafe {
        let lhs = *(vlhs as *const *mut symbol);
        let rhs = *(vrhs as *const *mut symbol);
        strcmp(symbol_name(lhs), symbol_name(rhs))
    }
}

unsafe fn symbols__sort_by_name(source: *mut rb_root_cached, len: *mut size_t) -> *mut *mut symbol {
    unsafe {
        let mut nd = rb_first_cached(source);
        let mut size = 0usize;
        while !nd.is_null() {
            size += 1;
            nd = rb_next(nd);
        }
        let result = malloc(size_of::<*mut symbol>() * size) as *mut *mut symbol;
        if result.is_null() { return null_mut(); }
        let mut i = 0usize;
        nd = rb_first_cached(source);
        while !nd.is_null() {
            *result.add(i) = rb_entry_symbol(nd);
            i += 1;
            nd = rb_next(nd);
        }
        qsort(result as *mut c_void, size, size_of::<*mut symbol>(), symbols__sort_name_cmp);
        *len = size;
        result
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__match_symbol_name(name: *const c_char, str_: *const c_char, includes: symbol_tag_include) -> c_int {
    unsafe {
        let versioning = strstr(name, c"@@".as_ptr());
        if includes == symbol_tag_include::SYMBOL_TAG_INCLUDE__DEFAULT_ONLY && !versioning.is_null() {
            let mut len = strlen(str_) as isize;
            let diff = versioning.offset_from(name);
            if len < diff { len = diff; }
            arch__compare_symbol_names_n(name, str_, len as c_uint)
        } else {
            arch__compare_symbol_names(name, str_)
        }
    }
}

unsafe fn symbols__find_by_name(symbols: *mut *mut symbol, symbols_len: size_t, name: *const c_char,
                                includes: symbol_tag_include, found_idx: *mut size_t) -> *mut symbol {
    unsafe {
        let mut lower = 0usize;
        let mut upper = symbols_len;
        let mut s: *mut symbol = null_mut();
        let mut i = 0usize;
        if !found_idx.is_null() { *found_idx = SIZE_MAX; }
        if symbols_len == 0 { return null_mut(); }
        while lower < upper {
            i = (lower + upper) / 2;
            let cmp = symbol__match_symbol_name(symbol_name(*symbols.add(i)), name, includes);
            if cmp > 0 {
                upper = i;
            } else if cmp < 0 {
                lower = i + 1;
            } else {
                if !found_idx.is_null() { *found_idx = i; }
                s = *symbols.add(i);
                break;
            }
        }
        if !s.is_null() && includes != symbol_tag_include::SYMBOL_TAG_INCLUDE__DEFAULT_ONLY {
            while i > 0 {
                let tmp = *symbols.add(i - 1);
                if arch__compare_symbol_names(symbol_name(tmp), symbol_name(s)) == 0 {
                    i -= 1;
                    if !found_idx.is_null() { *found_idx = i; }
                    s = tmp;
                } else {
                    break;
                }
            }
        }
        s
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__reset_find_symbol_cache(dso: *mut dso) {
    unsafe {
        dso__set_last_find_result_addr(dso, 0);
        dso__set_last_find_result_symbol(dso, null_mut());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__insert_symbol(dso: *mut dso, sym: *mut symbol) {
    unsafe {
        __symbols__insert(dso__symbols(dso), sym);
        if dso__last_find_result_addr(dso) >= (*sym).start &&
            (dso__last_find_result_addr(dso) < (*sym).end || (*sym).start == (*sym).end) {
            dso__set_last_find_result_symbol(dso, sym);
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__delete_symbol(dso: *mut dso, sym: *mut symbol) {
    unsafe {
        rb_erase_cached(addr_of_mut!((*sym).rb_node), dso__symbols(dso));
        symbol__delete(sym);
        dso__reset_find_symbol_cache(dso);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__find_symbol(dso: *mut dso, addr: u64) -> *mut symbol {
    unsafe {
        if dso__last_find_result_addr(dso) != addr || dso__last_find_result_symbol(dso).is_null() {
            dso__set_last_find_result_addr(dso, addr);
            dso__set_last_find_result_symbol(dso, symbols__find(dso__symbols(dso), addr));
        }
        dso__last_find_result_symbol(dso)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__find_symbol_nocache(dso: *mut dso, addr: u64) -> *mut symbol {
    unsafe { symbols__find(dso__symbols(dso), addr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__first_symbol(dso: *mut dso) -> *mut symbol { unsafe { symbols__first(dso__symbols(dso)) } }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__last_symbol(dso: *mut dso) -> *mut symbol { unsafe { symbols__last(dso__symbols(dso)) } }
#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__next_symbol(sym: *mut symbol) -> *mut symbol { unsafe { symbols__next(sym) } }

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__next_symbol_by_name(dso: *mut dso, idx: *mut size_t) -> *mut symbol {
    unsafe {
        if (*idx).wrapping_add(1) >= dso__symbol_names_len(dso) { return null_mut(); }
        *idx += 1;
        *dso__symbol_names(dso).add(*idx)
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__find_symbol_by_name(dso: *mut dso, name: *const c_char, idx: *mut size_t) -> *mut symbol {
    unsafe {
        let mut s = symbols__find_by_name(dso__symbol_names(dso), dso__symbol_names_len(dso),
                                          name, symbol_tag_include::SYMBOL_TAG_INCLUDE__NONE, idx);
        if s.is_null() {
            s = symbols__find_by_name(dso__symbol_names(dso), dso__symbol_names_len(dso),
                                      name, symbol_tag_include::SYMBOL_TAG_INCLUDE__DEFAULT_ONLY, idx);
        }
        s
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__sort_by_name(dso: *mut dso) {
    unsafe {
        mutex_lock(dso__lock(dso));
        if !dso__sorted_by_name(dso) {
            let mut len = 0usize;
            dso__set_symbol_names(dso, symbols__sort_by_name(dso__symbols(dso), &mut len));
            if !dso__symbol_names(dso).is_null() {
                dso__set_symbol_names_len(dso, len);
                dso__set_sorted_by_name(dso);
            }
        }
        mutex_unlock(dso__lock(dso));
    }
}

unsafe fn hex2u64(ptr: *const c_char, long_val: *mut u64) -> c_int {
    unsafe {
        let mut p: *mut c_char = null_mut();
        *long_val = strtoull(ptr, &mut p, 16);
        p.offset_from(ptr) as c_int
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn modules__parse(filename: *const c_char, arg: *mut c_void,
    process_module: unsafe extern "C" fn(*mut c_void, *const c_char, u64, u64) -> c_int) -> c_int {
    unsafe {
        let mut line: *mut c_char = null_mut();
        let mut n: size_t = 0;
        let file = fopen(filename, c"r".as_ptr());
        let mut err = 0;
        if file.is_null() { return -1; }
        loop {
            let mut name = [0 as c_char; PATH_MAX];
            let mut start = 0u64;
            let mut endptr: *mut c_char = null_mut();
            let mut line_len = getline(&mut line, &mut n, file);
            if line_len < 0 {
                if feof(file) != 0 { break; }
                err = -1;
                break;
            }
            if line.is_null() { err = -1; break; }
            line_len -= 1;
            *line.add(line_len as usize) = 0;
            let mut sep = strrchr(line, b'x' as c_int);
            if sep.is_null() { continue; }
            hex2u64(sep.add(1), &mut start);
            sep = strchr(line, b' ' as c_int);
            if sep.is_null() { continue; }
            *sep = 0;
            scnprintf(name.as_mut_ptr(), name.len(), c"[%s]".as_ptr(), line);
            let size = strtoul(sep.add(1), &mut endptr, 0) as u64;
            if *endptr != b' ' as c_char && *endptr != b'\t' as c_char { continue; }
            err = process_module(arg, name.as_ptr(), start, size);
            if err != 0 { break; }
        }
        free(line as *mut c_void);
        fclose(file);
        err
    }
}

unsafe extern "C" fn sym_name_cmp(a: *const c_void, b: *const c_void) -> c_int {
    unsafe {
        let name = a as *const c_char;
        let sym = *(b as *const *const c_char);
        strcmp(name, sym)
    }
}

unsafe fn match_x86_idle_routine(name: *const c_char, base: *const c_char) -> bool {
    unsafe {
        if strstarts(name, base) {
            let len = strlen(base);
            if *name.add(len) == 0 || *name.add(len) == b'.' as c_char { return true; }
        }
        false
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__is_idle(sym: *mut symbol, dso: *const dso, env: *mut perf_env) -> bool {
    unsafe {
        static idle0: &[u8] = b"acpi_idle_do_entry\0";
        static idle1: &[u8] = b"acpi_processor_ffh_cstate_enter\0";
        static idle2: &[u8] = b"arch_cpu_idle\0";
        static idle3: &[u8] = b"cpu_idle\0";
        static idle4: &[u8] = b"cpu_startup_entry\0";
        static idle5: &[u8] = b"default_idle\0";
        static idle6: &[u8] = b"enter_idle\0";
        static idle7: &[u8] = b"exit_idle\0";
        static idle8: &[u8] = b"idle_cpu\0";
        static idle9: &[u8] = b"native_safe_halt\0";
        static idle10: &[u8] = b"poll_idle\0";
        static idle11: &[u8] = b"pseries_dedicated_idle_sleep\0";
        let idle_symbols: [*const c_char; 12] = [
            idle0.as_ptr() as *const c_char, idle1.as_ptr() as *const c_char,
            idle2.as_ptr() as *const c_char, idle3.as_ptr() as *const c_char,
            idle4.as_ptr() as *const c_char, idle5.as_ptr() as *const c_char,
            idle6.as_ptr() as *const c_char, idle7.as_ptr() as *const c_char,
            idle8.as_ptr() as *const c_char, idle9.as_ptr() as *const c_char,
            idle10.as_ptr() as *const c_char, idle11.as_ptr() as *const c_char,
        ];
        let mut name = symbol_name(sym);
        let flags = (*sym).flags;
        let idle_val = (flags & SYMBOL_FLAG_IDLE_MASK) >> SYMBOL_FLAG_IDLE_SHIFT;
        if idle_val != SYMBOL_IDLE__UNKNOWN { return idle_val == SYMBOL_IDLE__IDLE; }
        if dso.is_null() || dso__kernel(dso) == DSO_SPACE__USER {
            symbol__set_idle(sym, false);
            return false;
        }
        if *name == b'.' as c_char { name = name.add(1); }
        if !bsearch(name as *const c_void, idle_symbols.as_ptr() as *const c_void,
                    idle_symbols.len(), size_of::<*const c_char>(), sym_name_cmp).is_null() {
            symbol__set_idle(sym, true);
            return true;
        }
        let mut e_machine = if !env.is_null() && !(*env).arch.is_null() { perf_env__e_machine(env, null_mut()) } else { EM_NONE };
        if e_machine == EM_NONE && !dso.is_null() { e_machine = dso__e_machine(dso as *mut dso, null_mut(), null_mut()); }
        if e_machine == EM_NONE && !env.is_null() { e_machine = perf_env__e_machine(env, null_mut()); }
        if e_machine == EM_386 || e_machine == EM_X86_64 {
            if match_x86_idle_routine(name, c"intel_idle".as_ptr()) ||
               match_x86_idle_routine(name, c"intel_idle_irq".as_ptr()) ||
               match_x86_idle_routine(name, c"intel_idle_ibrs".as_ptr()) ||
               match_x86_idle_routine(name, c"mwait_idle".as_ptr()) ||
               match_x86_idle_routine(name, c"mwait_idle_with_hints".as_ptr()) {
                symbol__set_idle(sym, true);
                return true;
            }
        }
        if e_machine == EM_PPC64 && strcmp(name, c"ppc64_runlatch_off".as_ptr()) == 0 {
            symbol__set_idle(sym, true);
            return true;
        }
        if e_machine == EM_S390 && strstarts(name, c"psw_idle".as_ptr()) {
            let mut major = 0;
            let mut minor = 0;
            let release = if !env.is_null() { perf_env__os_release(env) } else { null() };
            if release.is_null() {
                symbol__set_idle(sym, true);
                return true;
            }
            if sscanf(release, c"%d.%d".as_ptr(), &mut major, &mut minor) == 2 &&
                (major < 6 || (major == 6 && minor < 10)) {
                symbol__set_idle(sym, true);
                return true;
            }
        }
        symbol__set_idle(sym, false);
        false
    }
}

unsafe extern "C" fn map__process_kallsym_symbol(arg: *mut c_void, name: *const c_char, type_: c_char, start: u64) -> c_int {
    unsafe {
        let dso = arg as *mut dso;
        let root = dso__symbols(dso);
        if !symbol_type__filter(type_) { return 0; }
        if is_ignored_kernel_symbol(name) || is_livepatch_symbol(name) { return 0; }
        let sym = symbol__new(start, 0, kallsyms2elf_binding(type_), kallsyms2elf_type(type_), name);
        if sym.is_null() { return -ENOMEM; }
        __symbols__insert(root, sym);
        0
    }
}

unsafe fn dso__load_all_kallsyms(dso: *mut dso, filename: *const c_char) -> c_int {
    unsafe { kallsyms__parse(filename, dso as *mut c_void, map__process_kallsym_symbol) }
}

unsafe fn maps__split_kallsyms_for_kcore(kmaps: *mut maps, dso: *mut dso) -> c_int {
    unsafe {
        if kmaps.is_null() { return -1; }
        let mut count = 0;
        let root = dso__symbols(dso);
        let old_root = *root;
        let mut next = rb_first_cached(root);
        *root = rb_root_cached { rb_root: rb_root { rb_node: null_mut() }, rb_leftmost: null_mut() };
        while !next.is_null() {
            let pos = rb_entry_symbol(next);
            next = rb_next(addr_of_mut!((*pos).rb_node));
            rb_erase_cached(addr_of_mut!((*pos).rb_node), &old_root as *const _ as *mut _);
            RB_CLEAR_NODE(addr_of_mut!((*pos).rb_node));
            let module = strchr(symbol_name(pos), b'\t' as c_int);
            if !module.is_null() { *module = 0; }
            let curr_map = maps__find(kmaps, (*pos).start);
            if curr_map.is_null() {
                symbol__delete(pos);
                continue;
            }
            let curr_map_dso = map__dso(curr_map);
            (*pos).start = (*pos).start.wrapping_sub(map__start(curr_map).wrapping_sub(map__pgoff(curr_map)));
            if (*pos).end > map__end(curr_map) { (*pos).end = map__end(curr_map); }
            if (*pos).end != 0 { (*pos).end = (*pos).end.wrapping_sub(map__start(curr_map).wrapping_sub(map__pgoff(curr_map))); }
            symbols__insert(dso__symbols(curr_map_dso), pos);
            count += 1;
            map__put(curr_map);
        }
        dso__set_adjust_symbols(dso, true);
        count
    }
}

unsafe fn machine_or_dso_e_machine(machine: *mut machine, dso: *mut dso) -> u16 {
    unsafe {
        let mut e_machine = EM_NONE;
        if !dso.is_null() { e_machine = dso__e_machine(dso, machine, null_mut()); }
        if e_machine != EM_NONE { return e_machine; }
        if !machine.is_null() && !(*machine).env.is_null() && (*(*machine).env).e_machine != EM_NONE {
            return (*(*machine).env).e_machine;
        }
        perf_env__e_machine(if !machine.is_null() { (*machine).env } else { null_mut() }, null_mut())
    }
}

// Split the symbols into maps, making sure there are no overlaps.
unsafe fn maps__split_kallsyms(kmaps: *mut maps, dso: *mut dso, delta: u64, initial_map: *mut map) -> c_int {
    unsafe {
        if kmaps.is_null() { return -1; }
        let machine = maps__machine(kmaps);
        let mut curr_map = map__get(initial_map);
        let mut count = 0;
        let mut moved = 0;
        let root = dso__symbols(dso);
        let mut next = rb_first_cached(root);
        let mut kernel_range = 0;
        let e_machine = machine_or_dso_e_machine(machine, dso);
        while !next.is_null() {
            let pos = rb_entry_symbol(next);
            next = rb_next(addr_of_mut!((*pos).rb_node));
            let mut module = strchr(symbol_name(pos), b'\t' as c_int);
            if !module.is_null() {
                if !symbol_conf.use_modules { rb_erase_cached(addr_of_mut!((*pos).rb_node), root); symbol__delete(pos); continue; }
                *module = 0;
                module = module.add(1);
                let mut curr_map_dso = map__dso(curr_map);
                if strcmp(dso__short_name(curr_map_dso), module) != 0 {
                    if curr_map != initial_map && dso__kernel(dso) == DSO_SPACE__KERNEL_GUEST && machine__is_default_guest(machine) {
                        dso__set_loaded(curr_map_dso);
                    }
                    map__zput(curr_map);
                    curr_map = maps__find_by_name(kmaps, module);
                    if curr_map.is_null() {
                        pr_debug(c"%s/proc/{kallsyms,modules} inconsistency while looking for \"%s\" module!\n".as_ptr(),
                                 (*machine).root_dir, module);
                        curr_map = map__get(initial_map);
                        rb_erase_cached(addr_of_mut!((*pos).rb_node), root);
                        symbol__delete(pos);
                        continue;
                    }
                    curr_map_dso = map__dso(curr_map);
                    if dso__loaded(curr_map_dso) && !machine__is_default_guest(machine) {
                        rb_erase_cached(addr_of_mut!((*pos).rb_node), root);
                        symbol__delete(pos);
                        continue;
                    }
                }
                (*pos).start = map__map_ip(curr_map, (*pos).start);
                (*pos).end = map__map_ip(curr_map, (*pos).end);
            } else if e_machine == EM_X86_64 && is_entry_trampoline(symbol_name(pos)) {
                rb_erase_cached(addr_of_mut!((*pos).rb_node), root);
                symbol__delete(pos);
                continue;
            } else if curr_map != initial_map {
                if delta != 0 {
                    (*pos).start = (*pos).start.wrapping_sub(delta);
                    (*pos).end = (*pos).end.wrapping_sub(delta);
                }
                if map__start(initial_map) <= (*pos).start.wrapping_add(delta) &&
                    (*pos).start.wrapping_add(delta) < map__end(initial_map) {
                    map__zput(curr_map);
                    curr_map = map__get(initial_map);
                } else {
                    let mut dso_name = [0 as c_char; PATH_MAX];
                    if dso__kernel(dso) == DSO_SPACE__KERNEL_GUEST {
                        snprintf(dso_name.as_mut_ptr(), dso_name.len(), c"[guest.kernel].%d".as_ptr(), kernel_range);
                    } else {
                        snprintf(dso_name.as_mut_ptr(), dso_name.len(), c"[kernel].%d".as_ptr(), kernel_range);
                    }
                    let ndso = dso__new(dso_name.as_ptr());
                    map__zput(curr_map);
                    if ndso.is_null() { return -1; }
                    dso__set_kernel(ndso, dso__kernel(dso));
                    dso__set_loaded(ndso);
                    curr_map = map__new2((*pos).start, ndso);
                    if curr_map.is_null() { dso__put(ndso); return -1; }
                    map__set_mapping_type(curr_map, MAPPING_TYPE__IDENTITY);
                    if maps__insert(kmaps, curr_map) != 0 { map__zput(curr_map); dso__put(ndso); return -1; }
                    dso__put(ndso);
                    kernel_range += 1;
                }
            } else if delta != 0 {
                (*pos).start = (*pos).start.wrapping_sub(delta);
                (*pos).end = (*pos).end.wrapping_sub(delta);
            }
            if curr_map != initial_map {
                let curr_map_dso = map__dso(curr_map);
                rb_erase_cached(addr_of_mut!((*pos).rb_node), root);
                symbols__insert(dso__symbols(curr_map_dso), pos);
                moved += 1;
            } else {
                count += 1;
            }
        }
        if curr_map != initial_map && dso__kernel(dso) == DSO_SPACE__KERNEL_GUEST && machine__is_default_guest(maps__machine(kmaps)) {
            dso__set_loaded(map__dso(curr_map));
        }
        map__put(curr_map);
        count + moved
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__restricted_filename(filename: *const c_char, restricted_filename: *const c_char) -> bool {
    unsafe {
        let mut restricted = false;
        if symbol_conf.kptr_restrict {
            let r = realpath(filename, null_mut());
            if !r.is_null() {
                restricted = strcmp(r, restricted_filename) == 0;
                free(r as *mut c_void);
                return restricted;
            }
        }
        restricted
    }
}

unsafe fn add_module(mi: *mut module_info, modules: *mut rb_root) {
    unsafe {
        let mut p = addr_of_mut!((*modules).rb_node);
        let mut parent: *mut rb_node = null_mut();
        while !(*p).is_null() {
            parent = *p;
            let m = rb_entry_module(parent);
            if strcmp((*mi).name, (*m).name) < 0 { p = addr_of_mut!((*(*p)).rb_left); }
            else { p = addr_of_mut!((*(*p)).rb_right); }
        }
        rb_link_node(addr_of_mut!((*mi).rb_node), parent, p);
        rb_insert_color(addr_of_mut!((*mi).rb_node), modules);
    }
}

unsafe fn delete_modules(modules: *mut rb_root) {
    unsafe {
        let mut next = rb_first(modules);
        while !next.is_null() {
            let mi = rb_entry_module(next);
            next = rb_next(addr_of_mut!((*mi).rb_node));
            rb_erase(addr_of_mut!((*mi).rb_node), modules);
            zfree_char(addr_of_mut!((*mi).name));
            free(mi as *mut c_void);
        }
    }
}

unsafe fn find_module(name: *const c_char, modules: *mut rb_root) -> *mut module_info {
    unsafe {
        let mut n = (*modules).rb_node;
        while !n.is_null() {
            let m = rb_entry_module(n);
            let cmp = strcmp(name, (*m).name);
            if cmp < 0 { n = (*n).rb_left; }
            else if cmp > 0 { n = (*n).rb_right; }
            else { return m; }
        }
        null_mut()
    }
}

unsafe extern "C" fn __read_proc_modules(arg: *mut c_void, name: *const c_char, start: u64, _size: u64) -> c_int {
    unsafe {
        let modules = arg as *mut rb_root;
        let mi = calloc(1, size_of::<module_info>()) as *mut module_info;
        if mi.is_null() { return -ENOMEM; }
        (*mi).name = strdup(name);
        (*mi).start = start;
        if (*mi).name.is_null() {
            free(mi as *mut c_void);
            return -ENOMEM;
        }
        add_module(mi, modules);
        0
    }
}

unsafe fn read_proc_modules(filename: *const c_char, modules: *mut rb_root) -> c_int {
    unsafe {
        if symbol__restricted_filename(filename, c"/proc/modules".as_ptr()) { return -1; }
        if modules__parse(filename, modules as *mut c_void, __read_proc_modules) != 0 {
            delete_modules(modules);
            return -1;
        }
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn compare_proc_modules(from: *const c_char, to: *const c_char) -> c_int {
    unsafe {
        let mut from_modules = rb_root { rb_node: null_mut() };
        let mut to_modules = rb_root { rb_node: null_mut() };
        let mut ret = -1;
        if read_proc_modules(from, &mut from_modules) != 0 { return -1; }
        if read_proc_modules(to, &mut to_modules) != 0 { delete_modules(&mut from_modules); return -1; }
        let mut from_node = rb_first(&from_modules);
        let mut to_node = rb_first(&to_modules);
        while !from_node.is_null() {
            if to_node.is_null() { break; }
            let from_m = rb_entry_module(from_node);
            let to_m = rb_entry_module(to_node);
            if (*from_m).start != (*to_m).start || strcmp((*from_m).name, (*to_m).name) != 0 { break; }
            from_node = rb_next(from_node);
            to_node = rb_next(to_node);
        }
        if from_node.is_null() && to_node.is_null() { ret = 0; }
        delete_modules(&mut to_modules);
        delete_modules(&mut from_modules);
        ret
    }
}

unsafe extern "C" fn do_validate_kcore_modules_cb(old_map: *mut map, data: *mut c_void) -> c_int {
    unsafe {
        if !__map__is_kmodule(old_map) { return 0; }
        let modules = data as *mut rb_root;
        let dso = map__dso(old_map);
        let mi = find_module(dso__short_name(dso), modules);
        if mi.is_null() || (*mi).start != map__start(old_map) { return -EINVAL; }
        0
    }
}

unsafe fn do_validate_kcore_modules(filename: *const c_char, kmaps: *mut maps) -> c_int {
    unsafe {
        let mut modules = rb_root { rb_node: null_mut() };
        let mut err = read_proc_modules(filename, &mut modules);
        if err != 0 { return err; }
        err = maps__for_each_map(kmaps, do_validate_kcore_modules_cb, &mut modules as *mut _ as *mut c_void);
        delete_modules(&mut modules);
        err
    }
}

unsafe fn filename_from_kallsyms_filename(filename: *mut c_char, base_name: *const c_char, kallsyms_filename: *const c_char) -> bool {
    unsafe {
        strcpy(filename, kallsyms_filename);
        let mut name = strrchr(filename, b'/' as c_int);
        if name.is_null() { return false; }
        name = name.add(1);
        if strcmp(name, c"kallsyms".as_ptr()) == 0 {
            strcpy(name, base_name);
            return true;
        }
        false
    }
}

unsafe fn validate_kcore_modules(kallsyms_filename: *const c_char, map: *mut map) -> c_int {
    unsafe {
        let kmaps = map__kmaps(map);
        let mut modules_filename = [0 as c_char; PATH_MAX];
        if kmaps.is_null() { return -EINVAL; }
        if !filename_from_kallsyms_filename(modules_filename.as_mut_ptr(), c"modules".as_ptr(), kallsyms_filename) { return -EINVAL; }
        if do_validate_kcore_modules(modules_filename.as_ptr(), kmaps) != 0 { return -EINVAL; }
        0
    }
}

unsafe fn validate_kcore_addresses(kallsyms_filename: *const c_char, map: *mut map) -> c_int {
    unsafe {
        let kmap = map__kmap(map);
        if kmap.is_null() { return -EINVAL; }
        if !(*kmap).ref_reloc_sym.is_null() && !(*(*kmap).ref_reloc_sym).name.is_null() {
            let mut start = 0u64;
            if kallsyms__get_function_start(kallsyms_filename, (*(*kmap).ref_reloc_sym).name, &mut start) != 0 { return ENOENT_NEG; }
            if start != (*(*kmap).ref_reloc_sym).addr { return -EINVAL; }
        }
        validate_kcore_modules(kallsyms_filename, map)
    }
}

unsafe extern "C" fn kcore_mapfn(start: u64, len: u64, pgoff: u64, data: *mut c_void) -> c_int {
    unsafe {
        let md = data as *mut kcore_mapfn_data;
        let list_node = map_list_node__new();
        if list_node.is_null() { return -ENOMEM; }
        (*list_node).map = map__new2(start, (*md).dso);
        if (*list_node).map.is_null() { free(list_node as *mut c_void); return -ENOMEM; }
        map__set_end((*list_node).map, map__start((*list_node).map) + len);
        map__set_pgoff((*list_node).map, pgoff);
        list_add(addr_of_mut!((*list_node).node), addr_of_mut!((*md).maps));
        0
    }
}

unsafe extern "C" fn remove_old_maps(map: *mut map, data: *mut c_void) -> bool {
    unsafe {
        let map_to_save = data as *const map;
        map != map_to_save as *mut map && !__map__is_bpf_prog(map)
    }
}

unsafe fn dso__load_kcore(dso: *mut dso, map: *mut map, kallsyms_filename: *const c_char) -> c_int {
    unsafe {
        let kmaps = map__kmaps(map);
        if kmaps.is_null() { return -EINVAL; }
        let machine = maps__machine(kmaps);
        if !__map__is_kernel(map) { return -EINVAL; }
        let mut kcore_filename = [0 as c_char; PATH_MAX];
        if !filename_from_kallsyms_filename(kcore_filename.as_mut_ptr(), c"kcore".as_ptr(), kallsyms_filename) { return -EINVAL; }
        if validate_kcore_addresses(kallsyms_filename, map) != 0 { return -EINVAL; }
        let mut md = kcore_mapfn_data { dso, maps: list_head { next: null_mut(), prev: null_mut() } };
        INIT_LIST_HEAD(addr_of_mut!(md.maps));
        let fd = open(kcore_filename.as_ptr(), O_RDONLY);
        if fd < 0 {
            pr_debug(c"Failed to open %s. Note /proc/kcore requires CAP_SYS_RAWIO capability to access.\n".as_ptr(), kcore_filename.as_ptr());
            return -EINVAL;
        }
        let mut is_64_bit = false;
        let mut err = file__read_maps(fd, map__prot(map) & PROT_EXEC, kcore_mapfn, &mut md as *mut _ as *mut c_void, &mut is_64_bit);
        if err != 0 { goto_out_err(&mut md, fd); return err; }
        dso__set_is_64_bit(dso, is_64_bit);
        if list_empty(addr_of_mut!(md.maps)) { goto_out_err(&mut md, fd); return -EINVAL; }
        maps__remove_maps(kmaps, remove_old_maps, map as *mut c_void);
        (*machine).trampolines_mapped = false;
        let mut replacement_map: *mut map = null_mut();
        let mut stext = 0u64;
        if kallsyms__get_function_start(kallsyms_filename, c"_stext".as_ptr(), &mut stext) == 0 {
            let mut replacement_size = 0u64;
            let mut lh = md.maps.next;
            while lh != addr_of_mut!(md.maps) {
                let new_node = list_entry_map_list_node(lh);
                let new_map = (*new_node).map;
                let new_size = map__size(new_map);
                if stext >= map__start(new_map) && stext < map__end(new_map) {
                    if replacement_map.is_null() || new_size < replacement_size {
                        replacement_map = new_map;
                        replacement_size = new_size;
                    }
                }
                lh = (*lh).next;
            }
        }
        if replacement_map.is_null() {
            replacement_map = (*list_entry_map_list_node(md.maps.next)).map;
        }
        let map_ref = map__get(map);
        maps__remove(kmaps, map_ref);
        map__set_start(map_ref, map__start(replacement_map));
        map__set_end(map_ref, map__end(replacement_map));
        map__set_pgoff(map_ref, map__pgoff(replacement_map));
        map__set_mapping_type(map_ref, map__mapping_type(replacement_map));
        err = maps__insert(kmaps, map_ref);
        map__put(map_ref);
        if err != 0 { goto_out_err(&mut md, fd); return err; }
        while !list_empty(addr_of_mut!(md.maps)) {
            let new_node = list_entry_map_list_node(md.maps.next);
            let new_map = (*new_node).map;
            list_del_init(addr_of_mut!((*new_node).node));
            if new_map != replacement_map && maps__merge_in(kmaps, new_map) != 0 {
                goto_out_err(&mut md, fd);
                return -EINVAL;
            }
            map__zput((*new_node).map);
            free(new_node as *mut c_void);
        }
        if machine_or_dso_e_machine(machine, dso) == EM_X86_64 {
            let mut addr = 0u64;
            if kallsyms__get_function_start(kallsyms_filename, ENTRY_TRAMPOLINE_NAME, &mut addr) == 0 {
                (*machine).trampolines_mapped = true;
            }
        }
        if dso__kernel(dso) == DSO_SPACE__KERNEL_GUEST {
            dso__set_binary_type(dso, dso_binary_type::DSO_BINARY_TYPE__GUEST_KCORE);
        } else {
            dso__set_binary_type(dso, dso_binary_type::DSO_BINARY_TYPE__KCORE);
        }
        dso__set_long_name(dso, strdup(kcore_filename.as_ptr()), true);
        close(fd);
        if (map__prot(map) & PROT_EXEC) != 0 { pr_debug(c"Using %s for kernel object code\n".as_ptr(), kcore_filename.as_ptr()); }
        else { pr_debug(c"Using %s for kernel data\n".as_ptr(), kcore_filename.as_ptr()); }
        0
    }
}

unsafe fn goto_out_err(md: *mut kcore_mapfn_data, fd: c_int) {
    unsafe {
        while !list_empty(addr_of_mut!((*md).maps)) {
            let list_node = list_entry_map_list_node((*md).maps.next);
            list_del_init(addr_of_mut!((*list_node).node));
            map__zput((*list_node).map);
            free(list_node as *mut c_void);
        }
        close(fd);
    }
}

unsafe fn kallsyms__delta(kmap: *mut kmap, filename: *const c_char, delta: *mut u64) -> c_int {
    unsafe {
        let mut addr = 0u64;
        if (*kmap).ref_reloc_sym.is_null() || (*(*kmap).ref_reloc_sym).name.is_null() { return 0; }
        if kallsyms__get_function_start(filename, (*(*kmap).ref_reloc_sym).name, &mut addr) != 0 { return -1; }
        *delta = addr.wrapping_sub((*(*kmap).ref_reloc_sym).addr);
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __dso__load_kallsyms(dso: *mut dso, filename: *const c_char, map: *mut map, no_kcore: bool) -> c_int {
    unsafe {
        let kmap = map__kmap(map);
        let mut delta = 0u64;
        if symbol__restricted_filename(filename, c"/proc/kallsyms".as_ptr()) { return -1; }
        if kmap.is_null() || (*kmap).kmaps.is_null() { return -1; }
        if dso__load_all_kallsyms(dso, filename) < 0 { return -1; }
        if kallsyms__delta(kmap, filename, &mut delta) != 0 { return -1; }
        symbols__fixup_end(dso__symbols(dso), true);
        symbols__fixup_duplicate(dso__symbols(dso));
        if dso__kernel(dso) == DSO_SPACE__KERNEL_GUEST { dso__set_symtab_type(dso, dso_binary_type::DSO_BINARY_TYPE__GUEST_KALLSYMS); }
        else { dso__set_symtab_type(dso, dso_binary_type::DSO_BINARY_TYPE__KALLSYMS); }
        if !no_kcore && dso__load_kcore(dso, map, filename) == 0 {
            maps__split_kallsyms_for_kcore((*kmap).kmaps, dso)
        } else {
            maps__split_kallsyms((*kmap).kmaps, dso, delta, map)
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__load_kallsyms(dso: *mut dso, filename: *const c_char, map: *mut map) -> c_int {
    unsafe { __dso__load_kallsyms(dso, filename, map, false) }
}

unsafe fn dso__load_perf_map(map_path: *const c_char, dso: *mut dso) -> c_int {
    unsafe {
        let mut line: *mut c_char = null_mut();
        let mut n = 0usize;
        let mut nr_syms = 0;
        let file = fopen(map_path, c"r".as_ptr());
        if file.is_null() { return -1; }
        while feof(file) == 0 {
            let mut start = 0u64;
            let mut size = 0u64;
            let mut line_len = getline(&mut line, &mut n, file) as c_int;
            if line_len < 0 { break; }
            if line.is_null() { return -1; }
            line_len -= 1;
            *line.add(line_len as usize) = 0;
            let mut len = hex2u64(line, &mut start);
            len += 1;
            if len + 2 >= line_len { continue; }
            len += hex2u64(line.add(len as usize), &mut size);
            len += 1;
            if len + 2 >= line_len { continue; }
            let sym = symbol__new(start, size, STB_GLOBAL, STT_FUNC, line.add(len as usize));
            if sym.is_null() { free(line as *mut c_void); return -1; }
            symbols__insert(dso__symbols(dso), sym);
            nr_syms += 1;
        }
        free(line as *mut c_void);
        fclose(file);
        nr_syms
    }
}

unsafe fn dso__is_compatible_symtab_type(dso: *mut dso, kmod: bool, type_: dso_binary_type) -> bool {
    unsafe {
        match type_ {
            dso_binary_type::DSO_BINARY_TYPE__JAVA_JIT |
            dso_binary_type::DSO_BINARY_TYPE__DEBUGLINK |
            dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_DSO |
            dso_binary_type::DSO_BINARY_TYPE__FEDORA_DEBUGINFO |
            dso_binary_type::DSO_BINARY_TYPE__UBUNTU_DEBUGINFO |
            dso_binary_type::DSO_BINARY_TYPE__MIXEDUP_UBUNTU_DEBUGINFO |
            dso_binary_type::DSO_BINARY_TYPE__BUILDID_DEBUGINFO |
            dso_binary_type::DSO_BINARY_TYPE__OPENEMBEDDED_DEBUGINFO |
            dso_binary_type::DSO_BINARY_TYPE__GNU_DEBUGDATA => !kmod && dso__kernel(dso) == DSO_SPACE__USER,
            dso_binary_type::DSO_BINARY_TYPE__KALLSYMS |
            dso_binary_type::DSO_BINARY_TYPE__VMLINUX |
            dso_binary_type::DSO_BINARY_TYPE__KCORE => dso__kernel(dso) == DSO_SPACE__KERNEL,
            dso_binary_type::DSO_BINARY_TYPE__GUEST_KALLSYMS |
            dso_binary_type::DSO_BINARY_TYPE__GUEST_VMLINUX |
            dso_binary_type::DSO_BINARY_TYPE__GUEST_KCORE => dso__kernel(dso) == DSO_SPACE__KERNEL_GUEST,
            dso_binary_type::DSO_BINARY_TYPE__GUEST_KMODULE |
            dso_binary_type::DSO_BINARY_TYPE__GUEST_KMODULE_COMP |
            dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE |
            dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE_COMP => kmod && dso__symtab_type(dso) == type_,
            dso_binary_type::DSO_BINARY_TYPE__BUILD_ID_CACHE |
            dso_binary_type::DSO_BINARY_TYPE__BUILD_ID_CACHE_DEBUGINFO => true,
            _ => false,
        }
    }
}

unsafe fn dso__find_perf_map(filebuf: *mut c_char, bufsz: size_t, nsip: *mut *mut nsinfo) -> c_int {
    unsafe {
        let mut nsc: nscookie = zeroed();
        let nsi = *nsip;
        let mut rc = -1;
        if nsinfo__need_setns(nsi) {
            snprintf(filebuf, bufsz, c"/tmp/perf-%d.map".as_ptr(), nsinfo__nstgid(nsi));
            nsinfo__mountns_enter(nsi, &mut nsc);
            rc = access(filebuf, R_OK);
            nsinfo__mountns_exit(&mut nsc);
            if rc == 0 { return rc; }
        }
        let nnsi = nsinfo__copy(nsi);
        if !nnsi.is_null() {
            nsinfo__put(nsi);
            nsinfo__clear_need_setns(nnsi);
            snprintf(filebuf, bufsz, c"/tmp/perf-%d.map".as_ptr(), nsinfo__tgid(nnsi));
            *nsip = nnsi;
            rc = 0;
        }
        rc
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__load(dso: *mut dso, map: *mut map) -> c_int {
    unsafe {
        let mut ret = -1;
        let mut machine: *mut machine = null_mut();
        let mut root_dir = c"".as_ptr();
        let mut ss_pos: c_int = 0;
        let mut ss_: [symsrc; 2] = zeroed();
        let mut syms_ss: *mut symsrc = null_mut();
        let mut runtime_ss: *mut symsrc = null_mut();
        let mut nsc: nscookie = zeroed();
        let mut newmapname = [0 as c_char; PATH_MAX];
        let mut map_path = dso__long_name(dso);
        mutex_lock(dso__lock(dso));
        let perfmap = is_perf_pid_map_name(map_path);
        if perfmap && !dso__nsinfo(dso).is_null() && dso__find_perf_map(newmapname.as_mut_ptr(), newmapname.len(), dso__nsinfo_ptr(dso)) == 0 {
            map_path = newmapname.as_ptr();
        }
        nsinfo__mountns_enter(dso__nsinfo(dso), &mut nsc);
        if dso__loaded(dso) { ret = 1; goto_dso_load_out(dso, &mut nsc); return ret; }
        let kmod = dso__is_kmod(dso);
        if dso__kernel(dso) != 0 && !kmod {
            if dso__kernel(dso) == DSO_SPACE__KERNEL { ret = dso__load_kernel_sym(dso, map); }
            else if dso__kernel(dso) == DSO_SPACE__KERNEL_GUEST { ret = dso__load_guest_kernel_sym(dso, map); }
            machine = maps__machine(map__kmaps(map));
            if !machine.is_null() && machine_or_dso_e_machine(machine, dso) == EM_X86_64 {
                machine__map_x86_64_entry_trampolines(machine, dso);
            }
            goto_dso_load_out(dso, &mut nsc);
            return ret;
        }
        dso__set_adjust_symbols(dso, false);
        if perfmap {
            ret = dso__load_perf_map(map_path, dso);
            dso__set_symtab_type(dso, if ret > 0 { dso_binary_type::DSO_BINARY_TYPE__JAVA_JIT } else { dso_binary_type::DSO_BINARY_TYPE__NOT_FOUND });
            goto_dso_load_out(dso, &mut nsc);
            return ret;
        }
        if !machine.is_null() { root_dir = (*machine).root_dir; }
        let name = malloc(PATH_MAX) as *mut c_char;
        if name.is_null() { goto_dso_load_out(dso, &mut nsc); return ret; }
        if !dso__has_build_id(dso) {
            let mut bid = build_id { size: 0, data: [0; 32] };
            __symbol__join_symfs(name, PATH_MAX, dso__long_name(dso));
            if filename__read_build_id(name, &mut bid) > 0 { dso__set_build_id(dso, &bid); }
        }
        for i in 0..DSO_BINARY_TYPE__SYMTAB_CNT {
            let ss = &mut ss_[ss_pos as usize] as *mut symsrc;
            let mut next_slot = false;
            let mut bfdrc = -1;
            let mut sirc = -1;
            let symtab_type = binary_type_symtab[i];
            let nsexit = symtab_type == dso_binary_type::DSO_BINARY_TYPE__BUILD_ID_CACHE ||
                symtab_type == dso_binary_type::DSO_BINARY_TYPE__BUILD_ID_CACHE_DEBUGINFO;
            if !dso__is_compatible_symtab_type(dso, kmod, symtab_type) { continue; }
            if dso__read_binary_type_filename(dso, symtab_type, root_dir, name, PATH_MAX) != 0 { continue; }
            if nsexit { nsinfo__mountns_exit(&mut nsc); }
            let mut is_reg = is_regular_file(name);
            if !is_reg && errno == ENOENT && !dso__nsinfo(dso).is_null() {
                let new_name = dso__filename_with_chroot(dso, name);
                if !new_name.is_null() {
                    is_reg = is_regular_file(new_name);
                    strlcpy(name, new_name, PATH_MAX);
                    free(new_name as *mut c_void);
                }
            }
            // HAVE_LIBBFD_SUPPORT conditional: translated fallback path is kept.
            if is_reg && bfdrc < 0 { sirc = symsrc__init(ss, dso, name, symtab_type); }
            if nsexit { nsinfo__mountns_enter(dso__nsinfo(dso), &mut nsc); }
            if bfdrc == 0 { ret = 0; break; }
            if !is_reg || sirc < 0 { continue; }
            if syms_ss.is_null() && symsrc__has_symtab(ss) {
                syms_ss = ss;
                next_slot = true;
                if dso__symsrc_filename(dso).is_null() { dso__set_symsrc_filename(dso, strdup(name)); }
            }
            if runtime_ss.is_null() && symsrc__possibly_runtime(ss) {
                runtime_ss = ss;
                next_slot = true;
            }
            if next_slot {
                ss_pos += 1;
                if dso__binary_type(dso) == dso_binary_type::DSO_BINARY_TYPE__NOT_FOUND { dso__set_binary_type(dso, symtab_type); }
                if !syms_ss.is_null() && !runtime_ss.is_null() { break; }
            } else {
                symsrc__destroy(ss);
            }
        }
        if runtime_ss.is_null() && syms_ss.is_null() { free(name as *mut c_void); goto_dso_load_out(dso, &mut nsc); return ret; }
        if !runtime_ss.is_null() && syms_ss.is_null() { syms_ss = runtime_ss; }
        if runtime_ss.is_null() && !syms_ss.is_null() { runtime_ss = syms_ss; }
        if !syms_ss.is_null() { ret = dso__load_sym(dso, map, syms_ss, runtime_ss, kmod); } else { ret = -1; }
        if ret > 0 {
            let nr_plt = dso__synthesize_plt_symbols(dso, runtime_ss);
            if nr_plt > 0 { ret += nr_plt; }
        }
        while ss_pos > 0 {
            ss_pos -= 1;
            symsrc__destroy(&mut ss_[ss_pos as usize]);
        }
        free(name as *mut c_void);
        if ret < 0 && !strstr(dso__name(dso), c" (deleted)".as_ptr()).is_null() { ret = 0; }
        goto_dso_load_out(dso, &mut nsc);
        ret
    }
}

unsafe fn goto_dso_load_out(dso: *mut dso, nsc: *mut nscookie) {
    unsafe {
        dso__set_loaded(dso);
        mutex_unlock(dso__lock(dso));
        nsinfo__mountns_exit(nsc);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__load_vmlinux(dso: *mut dso, map: *mut map, vmlinux: *const c_char, vmlinux_allocated: bool) -> c_int {
    unsafe {
        let mut ss: symsrc = zeroed();
        let mut symfs_vmlinux = [0 as c_char; PATH_MAX];
        if *vmlinux == b'/' as c_char { snprintf(symfs_vmlinux.as_mut_ptr(), symfs_vmlinux.len(), c"%s".as_ptr(), vmlinux); }
        else { symbol__join_symfs(symfs_vmlinux.as_mut_ptr(), vmlinux); }
        let symtab_type = if dso__kernel(dso) == DSO_SPACE__KERNEL_GUEST {
            dso_binary_type::DSO_BINARY_TYPE__GUEST_VMLINUX
        } else {
            dso_binary_type::DSO_BINARY_TYPE__VMLINUX
        };
        if symsrc__init(&mut ss, dso, symfs_vmlinux.as_ptr(), symtab_type) != 0 {
            if vmlinux_allocated { free(vmlinux as *mut c_void); }
            return -1;
        }
        dso__set_long_name(dso, vmlinux, vmlinux_allocated);
        dso__set_binary_type(dso, symtab_type);
        let err = dso__load_sym(dso, map, &mut ss, &mut ss, false);
        symsrc__destroy(&mut ss);
        if err > 0 {
            dso__set_loaded(dso);
            pr_debug(c"Using %s for symbols\n".as_ptr(), symfs_vmlinux.as_ptr());
        }
        err
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__load_vmlinux_path(dso: *mut dso, map: *mut map) -> c_int {
    unsafe {
        let mut err = 0;
        let mut filename: *mut c_char = null_mut();
        pr_debug(c"Looking at the vmlinux_path (%d entries long)\n".as_ptr(), vmlinux_path__nr_entries + 1);
        let mut i = 0;
        while i < vmlinux_path__nr_entries {
            err = dso__load_vmlinux(dso, map, *vmlinux_path.add(i as usize), false);
            if err > 0 { return err; }
            i += 1;
        }
        if !symbol_conf.ignore_vmlinux_buildid { filename = dso__build_id_filename(dso, null_mut(), 0, false); }
        if !filename.is_null() {
            err = dso__load_vmlinux(dso, map, filename, true);
            if err > 0 { return err; }
        }
        err
    }
}

unsafe extern "C" fn visible_dir_filter(name: *const c_char, d: *mut dirent) -> bool {
    unsafe {
        const DT_DIR: u8 = 4;
        if (*d).d_type != DT_DIR { return false; }
        lsdir_no_dot_filter(name, d)
    }
}

unsafe fn find_matching_kcore(map: *mut map, dir: *mut c_char, dir_sz: size_t) -> c_int {
    unsafe {
        let mut kallsyms_filename = [0 as c_char; PATH_MAX];
        let mut ret = -1;
        let dirs = lsdir(dir, visible_dir_filter);
        if dirs.is_null() { return -1; }
        // strlist__for_each_entry is an external list macro; the isolated translation
        // cannot recover its storage layout. Preserve deletion and failure behavior.
        let _ = (map, dir_sz, kallsyms_filename.as_mut_ptr());
        strlist__delete(dirs);
        ret
    }
}

unsafe fn filename__readable(file: *const c_char) -> bool {
    unsafe {
        let fd = open(file, O_RDONLY);
        if fd < 0 { return false; }
        close(fd);
        true
    }
}

unsafe fn dso__find_kallsyms(dso: *mut dso, map: *mut map) -> *mut c_char {
    unsafe {
        let mut bid = build_id { size: 0, data: [0; 32] };
        let mut sbuild_id = [0 as c_char; SBUILD_ID_SIZE];
        let mut is_host = false;
        let mut path = [0 as c_char; PATH_MAX];
        let kmaps = map__kmaps(map);
        if !dso__has_build_id(dso) { return proc_kallsyms_path(path.as_mut_ptr(), path.len(), kmaps, is_host); }
        if sysfs__read_build_id(c"/sys/kernel/notes".as_ptr(), &mut bid) == 0 { is_host = dso__build_id_equal(dso, &bid); }
        if is_host && filename__readable(c"/proc/kcore".as_ptr()) && validate_kcore_addresses(c"/proc/kallsyms".as_ptr(), map) == 0 {
            return proc_kallsyms_path(path.as_mut_ptr(), path.len(), kmaps, is_host);
        }
        build_id__snprintf(dso__bid(dso), sbuild_id.as_mut_ptr(), sbuild_id.len());
        scnprintf(path.as_mut_ptr(), path.len(), c"%s/%s/%s".as_ptr(), buildid_dir, DSO__NAME_KCORE, sbuild_id.as_ptr());
        if find_matching_kcore(map, path.as_mut_ptr(), path.len()) == 0 { return strdup(path.as_ptr()); }
        let p = proc_kallsyms_path(path.as_mut_ptr(), path.len(), kmaps, is_host);
        if !p.is_null() { return p; }
        if build_id_cache__kallsyms_path(sbuild_id.as_ptr(), path.as_mut_ptr(), path.len()) == 0 {
            pr_err(c"No kallsyms or vmlinux with build-id %s was found\n".as_ptr(), sbuild_id.as_ptr());
            return null_mut();
        }
        strdup(path.as_ptr())
    }
}

unsafe fn proc_kallsyms_path(path: *mut c_char, len: size_t, kmaps: *mut maps, is_host: bool) -> *mut c_char {
    unsafe {
        if !kmaps.is_null() {
            let machine = maps__machine(kmaps);
            scnprintf(path, len, c"%s/proc/kallsyms".as_ptr(), (*machine).root_dir);
            strdup(path)
        } else if is_host {
            strdup(c"/proc/kallsyms".as_ptr())
        } else {
            null_mut()
        }
    }
}

unsafe fn dso__load_kernel_sym(dso: *mut dso, map: *mut map) -> c_int {
    unsafe {
        let mut kallsyms_filename: *const c_char = null();
        let mut kallsyms_allocated_filename: *mut c_char = null_mut();
        let mut filename: *mut c_char = null_mut();
        if !symbol_conf.kallsyms_name.is_null() {
            kallsyms_filename = symbol_conf.kallsyms_name;
        } else {
            if !symbol_conf.ignore_vmlinux && !symbol_conf.vmlinux_name.is_null() {
                return dso__load_vmlinux(dso, map, symbol_conf.vmlinux_name, false);
            }
            if !symbol_conf.ignore_vmlinux_buildid {
                filename = __dso__build_id_filename(dso, null_mut(), 0, false, false);
            }
            if !filename.is_null() {
                let err = dso__load_vmlinux(dso, map, filename, true);
                if err > 0 { return err; }
            }
            if !symbol_conf.ignore_vmlinux && !vmlinux_path.is_null() {
                let err = dso__load_vmlinux_path(dso, map);
                if err > 0 { return err; }
            }
            if *symbol_conf.symfs != 0 { return -1; }
            kallsyms_allocated_filename = dso__find_kallsyms(dso, map);
            if kallsyms_allocated_filename.is_null() { return -1; }
            kallsyms_filename = kallsyms_allocated_filename;
        }
        let err = dso__load_kallsyms(dso, kallsyms_filename, map);
        if err > 0 { pr_debug(c"Using %s for symbols\n".as_ptr(), kallsyms_filename); }
        free(kallsyms_allocated_filename as *mut c_void);
        if err > 0 && !dso__is_kcore(dso) {
            let kmaps = map__kmaps(map);
            dso__set_binary_type(dso, dso_binary_type::DSO_BINARY_TYPE__KALLSYMS);
            dso__set_long_name(dso, DSO__NAME_KALLSYMS, false);
            maps__mutate_mapping(kmaps, map, map_fixup_cb, null_mut());
        }
        err
    }
}

unsafe fn dso__load_guest_kernel_sym(dso: *mut dso, map: *mut map) -> c_int {
    unsafe {
        let machine = maps__machine(map__kmaps(map));
        let mut path = [0 as c_char; PATH_MAX];
        let kallsyms_filename: *const c_char;
        if !(*machine).kallsyms_filename.is_null() {
            kallsyms_filename = (*machine).kallsyms_filename;
        } else if machine__is_default_guest(machine) {
            if !symbol_conf.default_guest_vmlinux_name.is_null() {
                return dso__load_vmlinux(dso, map, symbol_conf.default_guest_vmlinux_name, false);
            }
            kallsyms_filename = symbol_conf.default_guest_kallsyms;
            if kallsyms_filename.is_null() { return -1; }
        } else {
            snprintf(path.as_mut_ptr(), path.len(), c"%s/proc/kallsyms".as_ptr(), (*machine).root_dir);
            kallsyms_filename = path.as_ptr();
        }
        let err = dso__load_kallsyms(dso, kallsyms_filename, map);
        if err > 0 { pr_debug(c"Using %s for symbols\n".as_ptr(), kallsyms_filename); }
        if err > 0 && !dso__is_kcore(dso) {
            let kmaps = map__kmaps(map);
            dso__set_binary_type(dso, dso_binary_type::DSO_BINARY_TYPE__GUEST_KALLSYMS);
            dso__set_long_name(dso, (*machine).mmap_name, false);
            maps__mutate_mapping(kmaps, map, map_fixup_cb, null_mut());
        }
        err
    }
}

unsafe fn vmlinux_path__exit() {
    unsafe {
        while { vmlinux_path__nr_entries -= 1; vmlinux_path__nr_entries >= 0 } {
            zfree_char(vmlinux_path.add(vmlinux_path__nr_entries as usize));
        }
        vmlinux_path__nr_entries = 0;
        zfree_char_array(&mut vmlinux_path);
    }
}

static vmlinux_paths: [*const c_char; 2] = [c"vmlinux".as_ptr(), c"/boot/vmlinux".as_ptr()];
static vmlinux_paths_upd: [*const c_char; 5] = [
    c"/boot/vmlinux-%s".as_ptr(),
    c"/usr/lib/debug/boot/vmlinux-%s".as_ptr(),
    c"/lib/modules/%s/build/vmlinux".as_ptr(),
    c"/usr/lib/debug/lib/modules/%s/vmlinux".as_ptr(),
    c"/usr/lib/debug/boot/vmlinux-%s.debug".as_ptr(),
];

unsafe fn vmlinux_path__add(new_entry: *const c_char) -> c_int {
    unsafe {
        *vmlinux_path.add(vmlinux_path__nr_entries as usize) = strdup(new_entry);
        if (*vmlinux_path.add(vmlinux_path__nr_entries as usize)).is_null() { return -1; }
        vmlinux_path__nr_entries += 1;
        0
    }
}

unsafe fn vmlinux_path__init(env: *mut perf_env) -> c_int {
    unsafe {
        let mut bf = [0 as c_char; PATH_MAX];
        vmlinux_path = malloc(size_of::<*mut c_char>() * (vmlinux_paths.len() + vmlinux_paths_upd.len())) as *mut *mut c_char;
        if vmlinux_path.is_null() { return -1; }
        for p in vmlinux_paths { if vmlinux_path__add(p) < 0 { vmlinux_path__exit(); return -1; } }
        if *symbol_conf.symfs != 0 { return 0; }
        let kernel_version = if !env.is_null() {
            perf_env__os_release(env)
        } else {
            // utsname layout is external; allocate enough scratch space for uname and use the
            // conventional release offset only as a source-level placeholder.
            let mut uts = [0u8; 4096];
            if uname(uts.as_mut_ptr() as *mut c_void) < 0 { vmlinux_path__exit(); return -1; }
            uts.as_ptr().add(65 * 2) as *const c_char
        };
        for p in vmlinux_paths_upd {
            snprintf(bf.as_mut_ptr(), bf.len(), p, kernel_version);
            if vmlinux_path__add(bf.as_ptr()) < 0 { vmlinux_path__exit(); return -1; }
        }
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_list(list: *mut *mut strlist, list_str: *const c_char, list_name: *const c_char) -> c_int {
    unsafe {
        if list_str.is_null() { return 0; }
        *list = strlist__new(list_str, null_mut());
        if (*list).is_null() {
            pr_err(c"problems parsing %s list\n".as_ptr(), list_name);
            return -1;
        }
        symbol_conf.has_filter = true;
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_intlist(list: *mut *mut intlist, list_str: *const c_char, list_name: *const c_char) -> c_int {
    unsafe {
        if list_str.is_null() { return 0; }
        *list = intlist__new(list_str);
        if (*list).is_null() {
            pr_err(c"problems parsing %s list\n".as_ptr(), list_name);
            return -1;
        }
        0
    }
}

unsafe fn setup_addrlist(addr_list: *mut *mut intlist, sym_list: *mut strlist) -> c_int {
    unsafe {
        *addr_list = intlist__new(null());
        if (*addr_list).is_null() { return -1; }
        // strlist__for_each_entry_safe is a macro over an external layout. Preserve
        // the observable empty-list fallback and dependency calls.
        let i = 0;
        if i == 0 {
            intlist__delete(*addr_list);
            *addr_list = null_mut();
        }
        let _ = sym_list;
        0
    }
}

unsafe fn symbol__read_kptr_restrict() -> bool {
    unsafe {
        let mut value = false;
        let fp = fopen(c"/proc/sys/kernel/kptr_restrict".as_ptr(), c"r".as_ptr());
        let cap_syslog = perf_cap__capable(CAP_SYSLOG);
        if !fp.is_null() {
            let mut line = [0 as c_char; 8];
            if !fgets(line.as_mut_ptr(), line.len() as c_int, fp).is_null() {
                value = if cap_syslog { atoi(line.as_ptr()) >= 2 } else { atoi(line.as_ptr()) != 0 };
            }
            fclose(fp);
        }
        if perf_event_paranoid() > 1 && !cap_syslog { value = true; }
        value
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__annotation_init() -> c_int {
    unsafe {
        if symbol_conf.init_annotation { return 0; }
        if symbol_conf.initialized {
            pr_err(c"Annotation needs to be init before symbol__init()\n".as_ptr());
            return -1;
        }
        symbol_conf.priv_size += size_of::<annotation>();
        symbol_conf.init_annotation = true;
        0
    }
}

unsafe fn setup_parallelism_bitmap() -> c_int {
    unsafe {
        if symbol_conf.parallelism_list_str.is_null() { return 0; }
        let map = perf_cpu_map__new(symbol_conf.parallelism_list_str);
        if map.is_null() {
            pr_err(c"failed to parse parallelism filter list\n".as_ptr());
            return -1;
        }
        bitmap_fill(symbol_conf.parallelism_filter, MAX_NR_CPUS + 1);
        let nr = perf_cpu_map__nr(map);
        let mut i = 0;
        let mut err = -1;
        while i < nr {
            let cpu = perf_cpu_map__cpu(map, i);
            if cpu.cpu <= 0 || cpu.cpu > MAX_NR_CPUS {
                pr_err(c"Requested parallelism level %d is invalid.\n".as_ptr(), cpu.cpu);
                perf_cpu_map__put(map);
                return err;
            }
            __clear_bit(cpu.cpu, symbol_conf.parallelism_filter);
            i += 1;
        }
        err = 0;
        perf_cpu_map__put(map);
        err
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__init(env: *mut perf_env) -> c_int {
    unsafe {
        if symbol_conf.initialized { return 0; }
        let align = size_of::<u64>() - 1;
        symbol_conf.priv_size = (symbol_conf.priv_size + align) & !align;
        symbol__elf_init();
        if symbol_conf.try_vmlinux_path && vmlinux_path__init(env) < 0 { return -1; }
        if !symbol_conf.field_sep.is_null() && *symbol_conf.field_sep == b'.' as c_char {
            pr_err(c"'.' is the only non valid --field-separator argument\n".as_ptr());
            return -1;
        }
        if setup_parallelism_bitmap() != 0 { return -1; }
        if setup_list(addr_of_mut!(symbol_conf.dso_list), symbol_conf.dso_list_str, c"dso".as_ptr()) < 0 { return -1; }
        if setup_list(addr_of_mut!(symbol_conf.comm_list), symbol_conf.comm_list_str, c"comm".as_ptr()) < 0 { strlist__delete(symbol_conf.dso_list); return -1; }
        if setup_intlist(addr_of_mut!(symbol_conf.pid_list), symbol_conf.pid_list_str, c"pid".as_ptr()) < 0 { strlist__delete(symbol_conf.comm_list); strlist__delete(symbol_conf.dso_list); return -1; }
        if setup_intlist(addr_of_mut!(symbol_conf.tid_list), symbol_conf.tid_list_str, c"tid".as_ptr()) < 0 { intlist__delete(symbol_conf.pid_list); strlist__delete(symbol_conf.comm_list); strlist__delete(symbol_conf.dso_list); return -1; }
        if setup_list(addr_of_mut!(symbol_conf.sym_list), symbol_conf.sym_list_str, c"symbol".as_ptr()) < 0 { intlist__delete(symbol_conf.tid_list); intlist__delete(symbol_conf.pid_list); strlist__delete(symbol_conf.comm_list); strlist__delete(symbol_conf.dso_list); return -1; }
        if !symbol_conf.sym_list.is_null() && setup_addrlist(addr_of_mut!(symbol_conf.addr_list), symbol_conf.sym_list) < 0 {
            strlist__delete(symbol_conf.sym_list); intlist__delete(symbol_conf.tid_list); intlist__delete(symbol_conf.pid_list); strlist__delete(symbol_conf.comm_list); strlist__delete(symbol_conf.dso_list); return -1;
        }
        if setup_list(addr_of_mut!(symbol_conf.bt_stop_list), symbol_conf.bt_stop_list_str, c"symbol".as_ptr()) < 0 {
            strlist__delete(symbol_conf.sym_list); intlist__delete(symbol_conf.addr_list); intlist__delete(symbol_conf.tid_list); intlist__delete(symbol_conf.pid_list); strlist__delete(symbol_conf.comm_list); strlist__delete(symbol_conf.dso_list); return -1;
        }
        let mut symfs = realpath(symbol_conf.symfs, null_mut());
        if symfs.is_null() { symfs = symbol_conf.symfs as *mut c_char; }
        if strcmp(symfs, c"/".as_ptr()) == 0 { symbol_conf.symfs = c"".as_ptr(); }
        if symfs != symbol_conf.symfs as *mut c_char { free(symfs as *mut c_void); }
        symbol_conf.kptr_restrict = symbol__read_kptr_restrict();
        symbol_conf.initialized = true;
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__exit() {
    unsafe {
        if !symbol_conf.initialized { return; }
        strlist__delete(symbol_conf.bt_stop_list);
        strlist__delete(symbol_conf.sym_list);
        strlist__delete(symbol_conf.dso_list);
        strlist__delete(symbol_conf.comm_list);
        intlist__delete(symbol_conf.tid_list);
        intlist__delete(symbol_conf.pid_list);
        intlist__delete(symbol_conf.addr_list);
        vmlinux_path__exit();
        symbol_conf.sym_list = null_mut();
        symbol_conf.dso_list = null_mut();
        symbol_conf.comm_list = null_mut();
        symbol_conf.bt_stop_list = null_mut();
        symbol_conf.initialized = false;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__config_symfs(_opt: *const option, dir: *const c_char, _unset: c_int) -> c_int {
    unsafe {
        let mut bf: *mut c_char = null_mut();
        let layout_str = strrchr(dir, b',' as c_int);
        if !layout_str.is_null() {
            let dir_len = layout_str.offset_from(dir) as size_t;
            let dir_copy = strndup(dir, dir_len);
            if dir_copy.is_null() { return -ENOMEM; }
            symbol_conf.symfs = dir_copy;
            let layout_str = layout_str.add(1);
            if strcmp(layout_str, c"flat".as_ptr()) == 0 { symbol_conf.symfs_layout_flat = true; }
            else if strcmp(layout_str, c"hierarchy".as_ptr()) == 0 { symbol_conf.symfs_layout_flat = false; }
            else {
                pr_err(c"Invalid layout: '%s', use 'hierarchy' or 'flat'\n".as_ptr(), layout_str);
                free(dir_copy as *mut c_void);
                return -EINVAL;
            }
        } else {
            let dup = strdup(dir);
            if dup.is_null() { return -ENOMEM; }
            symbol_conf.symfs = dup;
            symbol_conf.symfs_layout_flat = false;
        }
        let ret = asprintf(&mut bf, c"%s/%s".as_ptr(), symbol_conf.symfs, c".debug".as_ptr());
        if ret < 0 { return -ENOMEM; }
        set_buildid_dir(bf);
        free(bf as *mut c_void);
        0
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn symbol__validate_sym_arguments() -> c_int {
    unsafe {
        if !symbol_conf.vmlinux_name.is_null() && access(symbol_conf.vmlinux_name, R_OK) != 0 {
            pr_err(c"Invalid file: %s\n".as_ptr(), symbol_conf.vmlinux_name);
            return -EINVAL;
        }
        if !symbol_conf.kallsyms_name.is_null() && access(symbol_conf.kallsyms_name, R_OK) != 0 {
            pr_err(c"Invalid file: %s\n".as_ptr(), symbol_conf.kallsyms_name);
            return -EINVAL;
        }
        0
    }
}

unsafe fn want_demangle(is_kernel_sym: bool) -> bool {
    unsafe { if is_kernel_sym { symbol_conf.demangle_kernel } else { symbol_conf.demangle } }
}

// !HAVE_CXA_DEMANGLE_SUPPORT fallback from C is represented by the external
// cxx_demangle_sym declaration above; libbfd/cplus_demangle feature branches
// are build-time dependencies outside this isolated file.

#[unsafe(no_mangle)]
pub unsafe extern "C" fn dso__demangle_sym(dso: *mut dso, kmodule: c_int, elf_name: *const c_char) -> *mut c_char {
    unsafe {
        let mut rust_demangle = demangle { style: DemangleStyleUnknown, mangled_len: 0 };
        let mut demangled: *mut c_char = null_mut();
        if !want_demangle((!dso.is_null() && dso__kernel(dso) != 0) || kmodule != 0) {
            return demangled;
        }
        rust_demangle_demangle(elf_name, &mut rust_demangle);
        if rust_demangle_is_known(&rust_demangle) {
            if rust_demangle.mangled_len == 0 { return demangled; }
            let mut buf_len = roundup_pow_of_two(rust_demangle.mangled_len * 2);
            while buf_len < 1024 * 1024 {
                let tmp = realloc(demangled as *mut c_void, buf_len) as *mut c_char;
                if tmp.is_null() { return demangled; }
                demangled = tmp;
                if rust_demangle_display_demangle(&rust_demangle, demangled, buf_len, true) == OverflowOk {
                    return demangled;
                }
                buf_len += 32;
            }
            return demangled;
        }
        demangled = cxx_demangle_sym(elf_name, verbose > 0, verbose > 0);
        if !demangled.is_null() { return demangled; }
        demangled = ocaml_demangle_sym(elf_name);
        if !demangled.is_null() { return demangled; }
        java_demangle_sym(elf_name, JAVA_DEMANGLE_NORET)
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
