/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;
use core::sync::atomic::{AtomicU16, Ordering};

/* C header dependencies:
 * linux/types.h, linux/refcount.h, stdbool.h, stdint.h, stdatomic.h,
 * linux/list.h, linux/livepatch_external.h, linux/rbtree.h, linux/string.h,
 * stdio.h, errno.h, addr_location.h, path.h, symbol_conf.h, spark.h, util.h,
 * elf.h, and conditionally libelf.h/gelf.h.
 */

pub type u8 = u8;
pub type u16 = u16;
pub type u64 = u64;
pub type size_t = usize;
pub type Elf64_Addr = u64;
pub type Elf32_Addr = u32;

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
pub struct option {
    _private: [u8; 0],
}

#[repr(C)]
pub struct build_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
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
pub struct symsrc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Elf_Scn {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GElf_Ehdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GElf_Shdr {
    _private: [u8; 0],
}

#[repr(C)]
pub struct GElf_Sym {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root_cached {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct addr_location {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol_conf {
    pub symfs_layout_flat: bool,
    pub symfs: *const c_char,
    pub priv_size: size_t,
}

unsafe extern "C" {
    pub static symbol_conf: symbol_conf;
    pub static mut vmlinux_path__nr_entries: c_int;
    pub static mut vmlinux_path: *mut *mut c_char;
}

unsafe extern "C" {
    pub fn strstarts(str_: *const c_char, prefix: *const c_char) -> bool;
    pub fn path__join(bf: *mut c_char, size: size_t, path1: *const c_char, path2: *const c_char) -> c_int;
    pub fn perf_basename(path: *const c_char) -> *const c_char;
    pub fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node;
    pub fn rb_next(node: *const rb_node) -> *mut rb_node;
}

/* Supplied by linux/livepatch_external.h. */
unsafe extern "C" {
    pub static KLP_SYM_PREFIX: [c_char; 0];
}

/*
 * Ignore kernel mapping symbols, matching kernel is_mapping_symbol() logic.
 * This checks for '$' prefix (used by ARM, AArch64, RISC-V) and
 * x86 local symbol prefixes (.L* and L0*).
 * Only use this for kernel symbols (kallsyms, ksymbol events, kernel ELF DSOs).
 */
pub unsafe fn is_ignored_kernel_symbol(str_: *const c_char) -> bool {
    unsafe {
        if *str_.add(0) == b'.' as c_char && *str_.add(1) == b'L' as c_char {
            return true;
        }
        if *str_.add(0) == b'L' as c_char && *str_.add(1) == b'0' as c_char {
            return true;
        }
        *str_.add(0) == b'$' as c_char
    }
}

/*
 * Livepatch symbols (.klp.sym.*) are relocation placeholders whose resolved
 * addresses alias existing kernel symbols.  They carry a [module] tag which
 * confuses module boundary tracking and symbol table lookups.
 */
pub unsafe fn is_livepatch_symbol(str_: *const c_char) -> bool {
    unsafe { strstarts(str_, KLP_SYM_PREFIX.as_ptr()) }
}

/*
 * libelf 0.8.x and earlier do not support ELF_C_READ_MMAP;
 * for newer versions we can use mmap to reduce memory usage:
 *
 * C selected PERF_ELF_C_READ_MMAP as ELF_C_READ_MMAP when available, otherwise
 * ELF_C_READ. The exact value is provided by the libelf dependency.
 */

unsafe extern "C" {
    pub fn elf_section_by_name(
        elf: *mut Elf,
        ep: *mut GElf_Ehdr,
        shp: *mut GElf_Shdr,
        name: *const c_char,
        idx: *mut size_t,
    ) -> *mut Elf_Scn;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum symbol_idle_kind {
    SYMBOL_IDLE__UNKNOWN = 0,
    SYMBOL_IDLE__NOT_IDLE = 1,
    SYMBOL_IDLE__IDLE = 2,
}

pub const SYMBOL_FLAG_TYPE_SHIFT: u16 = 0;
pub const SYMBOL_FLAG_TYPE_MASK: u16 = 0xF << SYMBOL_FLAG_TYPE_SHIFT;
pub const SYMBOL_FLAG_BINDING_SHIFT: u16 = 4;
pub const SYMBOL_FLAG_BINDING_MASK: u16 = 0xF << SYMBOL_FLAG_BINDING_SHIFT;
pub const SYMBOL_FLAG_IDLE_SHIFT: u16 = 8;
pub const SYMBOL_FLAG_IDLE_MASK: u16 = 0x3 << SYMBOL_FLAG_IDLE_SHIFT;
pub const SYMBOL_FLAG_IGNORE: u16 = 1 << 10;
pub const SYMBOL_FLAG_INLINED: u16 = 1 << 11;
pub const SYMBOL_FLAG_ANNOTATE2: u16 = 1 << 12;
pub const SYMBOL_FLAG_IFUNC_ALIAS: u16 = 1 << 13;

/**
 * A symtab entry. When allocated this may be preceded by an annotation (see
 * symbol__annotation) and/or a browser_index (see symbol__browser_index).
 */
#[repr(C)]
pub struct symbol {
    pub rb_node: rb_node,
    /** Range of symbol [start, end). */
    pub start: u64,
    pub end: u64,
    /** Length of the string name. */
    pub namelen: u16,
    pub flags: AtomicU16,
    /** Architecture specific. Unused except on PPC where it holds st_other. */
    pub arch_sym: u8,
    /** The name of length namelen associated with the symbol. */
    pub name: [c_char; 0],
}

unsafe extern "C" {
    pub fn symbol__delete(sym: *mut symbol);
    pub fn symbols__delete(symbols: *mut rb_root_cached);
}

pub unsafe fn symbol__type(sym: *const symbol) -> u8 {
    unsafe { (((*sym).flags.load(Ordering::Relaxed) & SYMBOL_FLAG_TYPE_MASK) >> SYMBOL_FLAG_TYPE_SHIFT) as u8 }
}

pub unsafe fn symbol__binding(sym: *const symbol) -> u8 {
    unsafe {
        (((*sym).flags.load(Ordering::Relaxed) & SYMBOL_FLAG_BINDING_MASK) >> SYMBOL_FLAG_BINDING_SHIFT) as u8
    }
}

pub unsafe fn symbol__ignore(sym: *const symbol) -> bool {
    unsafe { ((*sym).flags.load(Ordering::Relaxed) & SYMBOL_FLAG_IGNORE) != 0 }
}

pub unsafe fn symbol__inlined(sym: *const symbol) -> bool {
    unsafe { ((*sym).flags.load(Ordering::Relaxed) & SYMBOL_FLAG_INLINED) != 0 }
}

pub unsafe fn symbol__is_annotate2(sym: *const symbol) -> bool {
    unsafe { ((*sym).flags.load(Ordering::Relaxed) & SYMBOL_FLAG_ANNOTATE2) != 0 }
}

pub unsafe fn symbol__ifunc_alias(sym: *const symbol) -> bool {
    unsafe { ((*sym).flags.load(Ordering::Relaxed) & SYMBOL_FLAG_IFUNC_ALIAS) != 0 }
}

unsafe extern "C" {
    pub fn symbol__is_idle(sym: *mut symbol, dso: *const dso, env: *mut perf_env) -> bool;

    pub fn symbol__set_ignore(sym: *mut symbol, ignore: bool);
    pub fn symbol__set_annotate2(sym: *mut symbol, annotate2: bool);
    pub fn symbol__set_inlined(sym: *mut symbol, inlined: bool);
    pub fn symbol__set_ifunc_alias(sym: *mut symbol, ifunc_alias: bool);
}

/* symbols__for_each_entry(symbols, pos, nd) iterates over symbols (rb_root)
 * using rb_first_cached(), rb_entry(..., struct symbol, rb_node), and rb_next().
 * A direct Rust macro would require the external rb_entry/container_of helper.
 */

pub unsafe fn symbol__size(sym: *const symbol) -> size_t {
    unsafe { ((*sym).end).wrapping_sub((*sym).start) as size_t }
}

pub unsafe fn __symbol__join_symfs(bf: *mut c_char, size: size_t, path: *const c_char) -> c_int {
    unsafe {
        if symbol_conf.symfs_layout_flat {
            return path__join(bf, size, symbol_conf.symfs, perf_basename(path));
        }

        path__join(bf, size, symbol_conf.symfs, path)
    }
}

/* symbol__join_symfs(bf, path) passes sizeof(bf) to __symbol__join_symfs. */

pub unsafe fn symbol__priv(sym: *mut symbol) -> *mut c_void {
    unsafe { (sym as *mut c_void as *mut u8).sub(symbol_conf.priv_size) as *mut c_void }
}

#[repr(C)]
pub struct ref_reloc_sym {
    pub name: *const c_char,
    pub addr: u64,
    pub unrelocated_addr: u64,
}

unsafe extern "C" {
    pub fn dso__load(dso: *mut dso, map: *mut map) -> c_int;
    pub fn dso__load_vmlinux(
        dso: *mut dso,
        map: *mut map,
        vmlinux: *const c_char,
        vmlinux_allocated: bool,
    ) -> c_int;
    pub fn dso__load_vmlinux_path(dso: *mut dso, map: *mut map) -> c_int;
    pub fn __dso__load_kallsyms(
        dso: *mut dso,
        filename: *const c_char,
        map: *mut map,
        no_kcore: bool,
    ) -> c_int;
    pub fn dso__load_kallsyms(dso: *mut dso, filename: *const c_char, map: *mut map) -> c_int;

    pub fn dso__insert_symbol(dso: *mut dso, sym: *mut symbol);
    pub fn dso__delete_symbol(dso: *mut dso, sym: *mut symbol);

    pub fn dso__find_symbol(dso: *mut dso, addr: u64) -> *mut symbol;
    pub fn dso__find_symbol_nocache(dso: *mut dso, addr: u64) -> *mut symbol;

    pub fn dso__next_symbol_by_name(dso: *mut dso, idx: *mut size_t) -> *mut symbol;
    pub fn dso__find_symbol_by_name(dso: *mut dso, name: *const c_char, idx: *mut size_t) -> *mut symbol;

    pub fn dso__first_symbol(dso: *mut dso) -> *mut symbol;
    pub fn dso__last_symbol(dso: *mut dso) -> *mut symbol;
    pub fn dso__next_symbol(sym: *mut symbol) -> *mut symbol;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum dso_type {
    __Incomplete = 0,
}

unsafe extern "C" {
    pub fn dso__type_fd(fd: c_int) -> dso_type;

    pub fn filename__read_build_id(filename: *const c_char, id: *mut build_id) -> c_int;
    pub fn sysfs__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int;
    pub fn modules__parse(
        filename: *const c_char,
        arg: *mut c_void,
        process_module: Option<unsafe extern "C" fn(arg: *mut c_void, name: *const c_char, start: u64, size: u64) -> c_int>,
    ) -> c_int;
    pub fn filename__read_debuglink(filename: *const c_char, debuglink: *mut c_char, size: size_t) -> c_int;
    pub fn filename__has_section(filename: *const c_char, sec: *const c_char) -> bool;

    pub fn symbol__init(env: *mut perf_env) -> c_int;
    pub fn symbol__exit();
    pub fn symbol__elf_init();
    pub fn symbol__annotation_init() -> c_int;

    pub fn symbol__new(start: u64, len: u64, binding: u8, type_: u8, name: *const c_char) -> *mut symbol;
    pub fn __symbol__fprintf_symname_offs(
        sym: *const symbol,
        al: *const addr_location,
        unknown_as_addr: bool,
        print_offsets: bool,
        fp: *mut FILE,
    ) -> size_t;
    pub fn symbol__fprintf_symname_offs(sym: *const symbol, al: *const addr_location, fp: *mut FILE) -> size_t;
    pub fn __symbol__fprintf_symname(
        sym: *const symbol,
        al: *const addr_location,
        unknown_as_addr: bool,
        fp: *mut FILE,
    ) -> size_t;
    pub fn symbol__fprintf_symname(sym: *const symbol, fp: *mut FILE) -> size_t;
    pub fn symbol__fprintf(sym: *mut symbol, fp: *mut FILE) -> size_t;
    pub fn symbol__restricted_filename(filename: *const c_char, restricted_filename: *const c_char) -> bool;
}

pub const SYMFS_HELP: &str = "setup root directory which contains debug files:\n\
\t\t\t\tdirectory:\tLook for files with symbols relative to this directory.\n\
\t\t\t\tlayout:   \tLayout of files, 'hierarchy' matches full path (default), 'flat' only matches base name.\n";

unsafe extern "C" {
    pub fn symbol__config_symfs(opt: *const option, dir: *const c_char, unset: c_int) -> c_int;
}

unsafe extern "C" {
    pub fn dso__load_bfd_symbols(dso: *mut dso, debugfile: *const c_char) -> c_int;
}

unsafe extern "C" {
    pub fn dso__load_sym(
        dso: *mut dso,
        map: *mut map,
        syms_ss: *mut symsrc,
        runtime_ss: *mut symsrc,
        kmodule: c_int,
    ) -> c_int;
    pub fn dso__synthesize_plt_symbols(dso: *mut dso, ss: *mut symsrc) -> c_int;

    pub fn dso__demangle_sym(dso: *mut dso, kmodule: c_int, elf_name: *const c_char) -> *mut c_char;

    pub fn __symbols__insert(symbols: *mut rb_root_cached, sym: *mut symbol);
    pub fn symbols__insert(symbols: *mut rb_root_cached, sym: *mut symbol);
    pub fn symbols__fixup_duplicate(symbols: *mut rb_root_cached);
    pub fn symbols__fixup_end(symbols: *mut rb_root_cached, is_kallsyms: bool);
}

pub type mapfn_t = Option<unsafe extern "C" fn(start: u64, len: u64, pgoff: u64, data: *mut c_void) -> c_int>;

unsafe extern "C" {
    pub fn file__read_maps(fd: c_int, exe: bool, mapfn: mapfn_t, data: *mut c_void, is_64_bit: *mut bool) -> c_int;
}

pub const PERF_KCORE_EXTRACT: &[u8; 24] = b"/tmp/perf-kcore-XXXXXX\0";

#[repr(C)]
pub struct kcore_extract {
    pub kcore_filename: *mut c_char,
    pub addr: u64,
    pub offs: u64,
    pub len: u64,
    pub extract_filename: [c_char; size_of::<[u8; 24]>()],
    pub fd: c_int,
}

unsafe extern "C" {
    pub fn kcore_extract__create(kce: *mut kcore_extract) -> c_int;
    pub fn kcore_extract__delete(kce: *mut kcore_extract);

    pub fn kcore_copy(from_dir: *const c_char, to_dir: *const c_char) -> c_int;
    pub fn compare_proc_modules(from: *const c_char, to: *const c_char) -> c_int;

    pub fn setup_list(list: *mut *mut strlist, list_str: *const c_char, list_name: *const c_char) -> c_int;
    pub fn setup_intlist(list: *mut *mut intlist, list_str: *const c_char, list_name: *const c_char) -> c_int;
}

unsafe extern "C" {
    pub fn arch__sym_update(s: *mut symbol, sym: *mut GElf_Sym);
}

unsafe extern "C" {
    pub fn arch__normalize_symbol_name(name: *const c_char) -> *const c_char;
}

pub const SYMBOL_A: c_int = 0;
pub const SYMBOL_B: c_int = 1;

unsafe extern "C" {
    pub fn arch__compare_symbol_names(namea: *const c_char, nameb: *const c_char) -> c_int;
    pub fn arch__compare_symbol_names_n(namea: *const c_char, nameb: *const c_char, n: c_uint) -> c_int;
    pub fn arch__choose_best_symbol(syma: *mut symbol, symb: *mut symbol) -> c_int;
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum symbol_tag_include {
    SYMBOL_TAG_INCLUDE__NONE = 0,
    SYMBOL_TAG_INCLUDE__DEFAULT_ONLY,
}

unsafe extern "C" {
    pub fn symbol__match_symbol_name(
        namea: *const c_char,
        nameb: *const c_char,
        includes: symbol_tag_include,
    ) -> c_int;
}

/* structure containing an SDT note's info */
#[repr(C)]
pub union sdt_note_addr {
    pub a64: [Elf64_Addr; 3],
    pub a32: [Elf32_Addr; 3],
}

#[repr(C)]
pub struct sdt_note {
    pub name: *mut c_char,     /* name of the note*/
    pub provider: *mut c_char, /* provider name */
    pub args: *mut c_char,
    pub bit32: bool, /* whether the location is 32 bits? */
    pub addr: sdt_note_addr, /* location, base and semaphore addrs */
    pub note_list: list_head, /* SDT notes' list */
}

unsafe extern "C" {
    pub fn get_sdt_note_list(head: *mut list_head, target: *const c_char) -> c_int;
    pub fn cleanup_sdt_note_list(sdt_notes: *mut list_head) -> c_int;
    pub fn sdt_notes__get_count(start: *mut list_head) -> c_int;
}

pub const SDT_PROBES_SCN: &str = ".probes";
pub const SDT_BASE_SCN: &str = ".stapsdt.base";
pub const SDT_NOTE_SCN: &str = ".note.stapsdt";
pub const SDT_NOTE_TYPE: c_int = 3;
pub const SDT_NOTE_NAME: &str = "stapsdt";
pub const NR_ADDR: c_int = 3;

pub const SDT_NOTE_IDX_LOC: c_int = 0;
pub const SDT_NOTE_IDX_BASE: c_int = 1;
pub const SDT_NOTE_IDX_REFCTR: c_int = 2;

unsafe extern "C" {
    pub fn symbol__validate_sym_arguments() -> c_int;
}

const _: () = {
    let _ = ptr::null::<maps>;
};
