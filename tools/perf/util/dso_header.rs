/* SPDX-License-Identifier: GPL-2.0 */

// Translated from perf/util/dso.h. C include dependencies are represented by
// opaque declarations or Rust type aliases where this header only referenced
// externally supplied types.

use core::ffi::{c_char, c_int, c_uchar, c_uint, c_void};

pub type u8 = u8;
pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type off_t = i64;
pub type uint16_t = u16;
pub type uint32_t = u32;

#[repr(C)]
pub struct machine {
    _private: [u8; 0],
}

#[repr(C)]
pub struct map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dsos {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct auxtrace_cache {
    _private: [u8; 0],
}

#[repr(C)]
pub struct debuginfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Dwfl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct build_id {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
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
    pub rb_root: rb_root,
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct refcount_t {
    _private: [u8; 0],
}

pub const DSO__NAME_KALLSYMS: &[u8] = b"[kernel.kallsyms]\0";
pub const DSO__NAME_KCORE: &[u8] = b"[kernel.kcore]\0";
pub const DSO__NAME_GUEST_KALLSYMS: &[u8] = b"[guest.kernel.kallsyms]\0";
pub const DSO__NAME_GUEST_KALLSYMS_PID_PREFIX: &[u8] = b"[guest.kernel.kallsyms.\0";

unsafe extern "C" {
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strspn(s: *const c_char, accept: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn build_id__is_defined(bid: *const build_id) -> bool;
    fn zfree(ptr: *mut *mut c_char);
    fn RB_EMPTY_ROOT(root: *const rb_root) -> bool;
}

/*
 * Validate names of the form "[guest.kernel.kallsyms.<pid>]", where
 * <pid> is the PID of the guest VM and varies per guest, so it
 * cannot be matched with strcmp() against a fixed string.
 *
 * Every character after the fixed prefix must be a decimal digit,
 * with ']' immediately terminating the digit run and nothing
 * following it. This rules out '/', "..", or any other character
 * being smuggled into the name.
 */
pub unsafe fn is_guest_kallsyms_pid_name(name: *const c_char) -> bool {
    let prefix_len: size_t = DSO__NAME_GUEST_KALLSYMS_PID_PREFIX.len() - 1;
    let digits: size_t;

    if unsafe {
        strncmp(
            name,
            DSO__NAME_GUEST_KALLSYMS_PID_PREFIX.as_ptr() as *const c_char,
            prefix_len,
        )
    } != 0
    {
        return false;
    }

    digits = unsafe { strspn(name.add(prefix_len), c"0123456789".as_ptr()) };
    if digits == 0 {
        return false;
    }

    /* ']' must terminate the digit run, with nothing trailing it */
    if unsafe { *name.add(prefix_len + digits) } != b']' as c_char {
        return false;
    }

    if unsafe { *name.add(prefix_len + digits + 1) } != 0 {
        return false;
    }

    true
}

/**
 * enum dso_binary_type - The kind of DSO generally associated with a memory
 *                        region (struct map).
 */
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum dso_binary_type {
    /** @DSO_BINARY_TYPE__KALLSYMS: Symbols from /proc/kallsyms file. */
    DSO_BINARY_TYPE__KALLSYMS = 0,
    /** @DSO_BINARY_TYPE__GUEST_KALLSYMS: Guest /proc/kallsyms file. */
    DSO_BINARY_TYPE__GUEST_KALLSYMS,
    /** @DSO_BINARY_TYPE__VMLINUX: Path to kernel /boot/vmlinux file. */
    DSO_BINARY_TYPE__VMLINUX,
    /** @DSO_BINARY_TYPE__GUEST_VMLINUX: Path to guest kernel /boot/vmlinux file. */
    DSO_BINARY_TYPE__GUEST_VMLINUX,
    /** @DSO_BINARY_TYPE__JAVA_JIT: Symbols from /tmp/perf.map file. */
    DSO_BINARY_TYPE__JAVA_JIT,
    /**
     * @DSO_BINARY_TYPE__DEBUGLINK: Debug file readable from the file path
     * in the .gnu_debuglink ELF section of the dso.
     */
    DSO_BINARY_TYPE__DEBUGLINK,
    /**
     * @DSO_BINARY_TYPE__BUILD_ID_CACHE: File named after buildid located in
     * the buildid cache with an elf filename.
     */
    DSO_BINARY_TYPE__BUILD_ID_CACHE,
    /**
     * @DSO_BINARY_TYPE__BUILD_ID_CACHE_DEBUGINFO: File named after buildid
     * located in the buildid cache with a debug filename.
     */
    DSO_BINARY_TYPE__BUILD_ID_CACHE_DEBUGINFO,
    /**
     * @DSO_BINARY_TYPE__FEDORA_DEBUGINFO: Debug file in /usr/lib/debug
     * with .debug suffix.
     */
    DSO_BINARY_TYPE__FEDORA_DEBUGINFO,
    /** @DSO_BINARY_TYPE__UBUNTU_DEBUGINFO: Debug file in /usr/lib/debug. */
    DSO_BINARY_TYPE__UBUNTU_DEBUGINFO,
    /**
     * @DSO_BINARY_TYPE__MIXEDUP_UBUNTU_DEBUGINFO: dso__long_name debuginfo
     * file in /usr/lib/debug/lib rather than the expected
     * /usr/lib/debug/usr/lib.
     */
    DSO_BINARY_TYPE__MIXEDUP_UBUNTU_DEBUGINFO,
    /**
     * @DSO_BINARY_TYPE__BUILDID_DEBUGINFO: File named after buildid located
     * in /usr/lib/debug/.build-id/.
     */
    DSO_BINARY_TYPE__BUILDID_DEBUGINFO,
    /**
     * @DSO_BINARY_TYPE__GNU_DEBUGDATA: MiniDebuginfo where a compressed
     * ELF file is placed in a .gnu_debugdata section.
     */
    DSO_BINARY_TYPE__GNU_DEBUGDATA,
    /** @DSO_BINARY_TYPE__SYSTEM_PATH_DSO: A regular executable/shared-object file. */
    DSO_BINARY_TYPE__SYSTEM_PATH_DSO,
    /** @DSO_BINARY_TYPE__GUEST_KMODULE: Guest kernel module .ko file. */
    DSO_BINARY_TYPE__GUEST_KMODULE,
    /** @DSO_BINARY_TYPE__GUEST_KMODULE_COMP: Guest kernel module .ko.gz file. */
    DSO_BINARY_TYPE__GUEST_KMODULE_COMP,
    /** @DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE: Kernel module .ko file. */
    DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE,
    /** @DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE_COMP: Kernel module .ko.gz file. */
    DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE_COMP,
    /** @DSO_BINARY_TYPE__KCORE: /proc/kcore file. */
    DSO_BINARY_TYPE__KCORE,
    /** @DSO_BINARY_TYPE__GUEST_KCORE: Guest /proc/kcore file. */
    DSO_BINARY_TYPE__GUEST_KCORE,
    /**
     * @DSO_BINARY_TYPE__OPENEMBEDDED_DEBUGINFO: Openembedded/Yocto -dbg
     * package debug info.
     */
    DSO_BINARY_TYPE__OPENEMBEDDED_DEBUGINFO,
    /** @DSO_BINARY_TYPE__BPF_PROG_INFO: jitted BPF code. */
    DSO_BINARY_TYPE__BPF_PROG_INFO,
    /** @DSO_BINARY_TYPE__BPF_IMAGE: jitted BPF trampoline or dispatcher code. */
    DSO_BINARY_TYPE__BPF_IMAGE,
    /**
     * @DSO_BINARY_TYPE__OOL: out of line code such as kprobe-replaced
     * instructions or optimized kprobes or ftrace trampolines.
     */
    DSO_BINARY_TYPE__OOL,
    /** @DSO_BINARY_TYPE__NOT_FOUND: Unknown DSO kind. */
    DSO_BINARY_TYPE__NOT_FOUND,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum dso_space_type {
    DSO_SPACE__USER = 0,
    DSO_SPACE__KERNEL,
    DSO_SPACE__KERNEL_GUEST,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum dso_swap_type {
    DSO_SWAP__UNSET,
    DSO_SWAP__NO,
    DSO_SWAP__YES,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum dso_data_status {
    DSO_DATA_STATUS_ERROR = -1,
    DSO_DATA_STATUS_UNKNOWN = 0,
    DSO_DATA_STATUS_OK = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum dso_data_status_seen {
    DSO_DATA_STATUS_SEEN_ITRACE,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum dso_type {
    DSO__TYPE_UNKNOWN,
    DSO__TYPE_64BIT,
    DSO__TYPE_32BIT,
    DSO__TYPE_X32BIT,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum dso_load_errno {
    DSO_LOAD_ERRNO__SUCCESS = 0,

    /*
     * Choose an arbitrary negative big number not to clash with standard
     * errno since SUS requires the errno has distinct positive values.
     * See 'Issue 6' in the link below.
     *
     * http://pubs.opengroup.org/onlinepubs/9699919799/basedefs/errno.h.html
     */
    __DSO_LOAD_ERRNO__START = -10000,

    DSO_LOAD_ERRNO__INTERNAL_ERROR = -10000,

    /* for symsrc__init() */
    DSO_LOAD_ERRNO__INVALID_ELF,
    DSO_LOAD_ERRNO__CANNOT_READ_BUILDID,
    DSO_LOAD_ERRNO__MISMATCHING_BUILDID,

    /* for decompress_kmodule */
    DSO_LOAD_ERRNO__DECOMPRESSION_FAILURE,

    __DSO_LOAD_ERRNO__END,
}

pub unsafe fn DSO_SWAP_TYPE__SWAP<T>(swap_type: dso_swap_type, val: T) -> T
where
    T: Copy,
{
    assert!(swap_type != dso_swap_type::DSO_SWAP__UNSET);
    if swap_type == dso_swap_type::DSO_SWAP__YES {
        match core::mem::size_of::<T>() {
            2 => {
                let v = u16::from_ne_bytes(unsafe { core::mem::transmute_copy(&val) }).swap_bytes();
                unsafe { core::mem::transmute_copy(&v) }
            }
            4 => {
                let v = u32::from_ne_bytes(unsafe { core::mem::transmute_copy(&val) }).swap_bytes();
                unsafe { core::mem::transmute_copy(&v) }
            }
            8 => {
                let v = u64::from_ne_bytes(unsafe { core::mem::transmute_copy(&val) }).swap_bytes();
                unsafe { core::mem::transmute_copy(&v) }
            }
            _ => panic!("BUG_ON(1)"),
        }
    } else {
        val
    }
}

pub unsafe fn DSO__SWAP<T>(dso: *const dso, val: T) -> T
where
    T: Copy,
{
    unsafe { DSO_SWAP_TYPE__SWAP(dso__needs_swap(dso), val) }
}

pub const DSO__DATA_CACHE_SIZE: usize = 4096;
pub const DSO__DATA_CACHE_MASK: usize = !(DSO__DATA_CACHE_SIZE - 1);

/**
 * struct dso_id
 *
 * Data about backing storage DSO, comes from PERF_RECORD_MMAP2 meta events,
 * reading from /proc/pid/maps or synthesis of build_ids from DSOs. Possibly
 * incomplete at any particular use.
 */
#[repr(C)]
pub struct dso_id {
    /* Data related to the mmap2 event or read from /proc/pid/maps. */
    pub maj: u32,
    pub min: u32,
    pub ino: u64,
    pub ino_generation: u64,
    /** @mmap2_valid: Are the maj, min and ino fields valid? */
    pub mmap2_valid: bool,
    /**
     * @mmap2_ino_generation_valid: Is the ino_generation valid? Generally
     * false for /proc/pid/maps mmap event.
     */
    pub mmap2_ino_generation_valid: bool,
    /**
     * @build_id: A possibly populated build_id. build_id__is_defined checks
     * whether it is populated.
     */
    pub build_id: build_id,
}

#[repr(C)]
pub struct dso_cache {
    pub rb_node: rb_node,
    pub offset: u64,
    pub size: u64,
    pub data: [c_char; 0],
}

#[repr(C)]
pub struct dso_data {
    pub cache: rb_root,
    pub open_entry: list_head,
    // REFCNT_CHECKING: struct dso *dso;
    pub fd: c_int,
    pub status: c_int,
    pub status_seen: u32,
    pub file_size: u64,
    // HAVE_LIBUNWIND_SUPPORT: elf_base_addr, debug_frame_offset,
    // eh_frame_hdr_addr, eh_frame_hdr_offset.
}

#[repr(C)]
pub struct dso_bpf_prog {
    pub id: u32,
    pub sub_id: u32,
    pub env: *mut perf_env,
}

#[repr(C)]
pub struct dso_last_find_result {
    pub addr: u64,
    pub symbol: *mut symbol,
}

#[repr(C)]
pub union dso_tool_specific {
    pub priv_: *mut c_void,
    pub db_id: u64,
}

#[repr(C)]
pub struct dso {
    pub lock: mutex,
    pub dsos: *mut dsos,
    pub symbols: rb_root_cached,
    pub symbol_names: *mut *mut symbol,
    pub symbol_names_len: size_t,
    pub inlined_nodes: rb_root_cached,
    pub srclines: rb_root_cached,
    pub data_types: rb_root,
    pub global_vars: rb_root,
    pub last_find_result: dso_last_find_result,
    pub text_offset: u64,
    pub text_end: u64,
    pub short_name: *const c_char,
    pub long_name: *const c_char,
    pub a2l: *mut c_void,
    pub libdw: *mut c_void,
    pub symsrc_filename: *mut c_char,
    pub nsinfo: *mut nsinfo,
    pub auxtrace_cache: *mut auxtrace_cache,
    pub tool_specific: dso_tool_specific,
    /* bpf prog information */
    pub bpf_prog: dso_bpf_prog,
    /* dso data file */
    pub data: dso_data,
    pub id: dso_id,
    pub a2l_fails: c_uint,
    pub comp: c_int,
    pub refcnt: refcount_t,
    pub load_errno: dso_load_errno,
    pub long_name_len: u16,
    pub short_name_len: u16,
    // C bitfields are represented as their storage-intent scalar fields.
    pub symtab_type: dso_binary_type,
    pub binary_type: dso_binary_type,
    pub kernel: dso_space_type,
    pub needs_swap: dso_swap_type,
    pub is_kmod: bool,
    pub adjust_symbols: u8,
    pub header_build_id: u8,
    pub has_srcline: u8,
    pub hit: u8,
    pub annotate_warned: u8,
    pub auxtrace_warned: u8,
    pub debuginfo_warned: u8,
    pub short_name_allocated: u8,
    pub long_name_allocated: u8,
    pub is_64_bit: u8,
    pub sorted_by_name: bool,
    pub loaded: bool,
    pub rel: u8,
    pub name: [c_char; 0],
}

unsafe extern "C" {
    pub static mut _dso__data_open_lock: mutex;
    pub static dso_id_empty: dso_id;

    pub fn dso_id__cmp(a: *const dso_id, b: *const dso_id) -> c_int;
}

// dso__for_each_symbol(dso, pos, n) maps to symbols__for_each_entry(dso__symbols(dso), pos, n).

pub unsafe fn dso__a2l(dso: *const dso) -> *mut c_void {
    unsafe { (*dso).a2l }
}

pub unsafe fn dso__set_a2l(dso: *mut dso, val: *mut c_void) {
    unsafe { (*dso).a2l = val };
}

pub unsafe fn dso__libdw(dso: *const dso) -> *mut c_void {
    unsafe { (*dso).libdw }
}

pub unsafe fn dso__set_libdw(dso: *mut dso, val: *mut c_void) {
    unsafe { (*dso).libdw = val };
}

// HAVE_LIBDW_SUPPORT selects an external declaration; otherwise C returns NULL.
unsafe extern "C" {
    pub fn dso__libdw_dwfl(dso: *mut dso) -> *mut Dwfl;
}

pub unsafe fn dso__a2l_fails(dso: *const dso) -> c_uint {
    unsafe { (*dso).a2l_fails }
}

pub unsafe fn dso__set_a2l_fails(dso: *mut dso, val: c_uint) {
    unsafe { (*dso).a2l_fails = val };
}

pub unsafe fn dso__adjust_symbols(dso: *const dso) -> bool {
    unsafe { (*dso).adjust_symbols != 0 }
}

pub unsafe fn dso__set_adjust_symbols(dso: *mut dso, val: bool) {
    unsafe { (*dso).adjust_symbols = val as u8 };
}

pub unsafe fn dso__annotate_warned(dso: *const dso) -> bool {
    unsafe { (*dso).annotate_warned != 0 }
}

pub unsafe fn dso__set_annotate_warned(dso: *mut dso) {
    unsafe { (*dso).annotate_warned = 1 };
}

pub unsafe fn dso__debuginfo_warned(dso: *const dso) -> bool {
    unsafe { (*dso).debuginfo_warned != 0 }
}

pub unsafe fn dso__set_debuginfo_warned(dso: *mut dso) {
    unsafe { (*dso).debuginfo_warned = 1 };
}

pub unsafe fn dso__auxtrace_warned(dso: *const dso) -> bool {
    unsafe { (*dso).auxtrace_warned != 0 }
}

pub unsafe fn dso__set_auxtrace_warned(dso: *mut dso) {
    unsafe { (*dso).auxtrace_warned = 1 };
}

pub unsafe fn dso__auxtrace_cache(dso: *mut dso) -> *mut auxtrace_cache {
    unsafe { (*dso).auxtrace_cache }
}

pub unsafe fn dso__set_auxtrace_cache(dso: *mut dso, cache: *mut auxtrace_cache) {
    unsafe { (*dso).auxtrace_cache = cache };
}

pub unsafe fn dso__bpf_prog(dso: *mut dso) -> *mut dso_bpf_prog {
    unsafe { &mut (*dso).bpf_prog }
}

pub unsafe fn dso__has_srcline(dso: *const dso) -> bool {
    unsafe { (*dso).has_srcline != 0 }
}

pub unsafe fn dso__set_has_srcline(dso: *mut dso, val: bool) {
    unsafe { (*dso).has_srcline = val as u8 };
}

pub unsafe fn dso__comp(dso: *const dso) -> c_int {
    unsafe { (*dso).comp }
}

pub unsafe fn dso__set_comp(dso: *mut dso, comp: c_int) {
    unsafe { (*dso).comp = comp };
}

pub unsafe fn dso__data(dso: *mut dso) -> *mut dso_data {
    unsafe { &mut (*dso).data }
}

pub unsafe fn dso__db_id(dso: *const dso) -> u64 {
    unsafe { (*dso).tool_specific.db_id }
}

pub unsafe fn dso__set_db_id(dso: *mut dso, db_id: u64) {
    unsafe { (*dso).tool_specific.db_id = db_id };
}

pub unsafe fn dso__dsos(dso: *mut dso) -> *mut dsos {
    unsafe { (*dso).dsos }
}

pub unsafe fn dso__set_dsos(dso: *mut dso, dsos: *mut dsos) {
    unsafe { (*dso).dsos = dsos };
}

pub unsafe fn dso__header_build_id(dso: *mut dso) -> bool {
    unsafe { (*dso).header_build_id != 0 }
}

pub unsafe fn dso__set_header_build_id(dso: *mut dso, val: bool) {
    unsafe { (*dso).header_build_id = val as u8 };
}

pub unsafe fn dso__hit(dso: *const dso) -> bool {
    unsafe { (*dso).hit != 0 }
}

pub unsafe fn dso__set_hit(dso: *mut dso) {
    unsafe { (*dso).hit = 1 };
}

pub unsafe fn dso__id(dso: *mut dso) -> *mut dso_id {
    unsafe { &mut (*dso).id }
}

pub unsafe fn dso__id_const(dso: *const dso) -> *const dso_id {
    unsafe { &(*dso).id }
}

pub unsafe fn dso__bid(dso: *const dso) -> *const build_id {
    unsafe { &(*dso__id_const(dso)).build_id }
}

pub unsafe fn dso__has_build_id(dso: *const dso) -> bool {
    unsafe { build_id__is_defined(dso__bid(dso)) }
}

pub unsafe fn dso__inlined_nodes(dso: *mut dso) -> *mut rb_root_cached {
    unsafe { &mut (*dso).inlined_nodes }
}

pub unsafe fn dso__is_64_bit(dso: *const dso) -> bool {
    unsafe { (*dso).is_64_bit != 0 }
}

pub unsafe fn dso__set_is_64_bit(dso: *mut dso, is_: bool) {
    unsafe { (*dso).is_64_bit = is_ as u8 };
}

pub unsafe fn dso__is_kmod(dso: *const dso) -> bool {
    unsafe { (*dso).is_kmod }
}

pub unsafe fn dso__set_is_kmod(dso: *mut dso) {
    unsafe { (*dso).is_kmod = true };
}

pub unsafe fn dso__kernel(dso: *const dso) -> dso_space_type {
    unsafe { (*dso).kernel }
}

pub unsafe fn dso__set_kernel(dso: *mut dso, kernel: dso_space_type) {
    unsafe { (*dso).kernel = kernel };
}

pub unsafe fn dso__last_find_result_addr(dso: *const dso) -> u64 {
    unsafe { (*dso).last_find_result.addr }
}

pub unsafe fn dso__set_last_find_result_addr(dso: *mut dso, addr: u64) {
    unsafe { (*dso).last_find_result.addr = addr };
}

pub unsafe fn dso__last_find_result_symbol(dso: *const dso) -> *mut symbol {
    unsafe { (*dso).last_find_result.symbol }
}

pub unsafe fn dso__set_last_find_result_symbol(dso: *mut dso, symbol: *mut symbol) {
    unsafe { (*dso).last_find_result.symbol = symbol };
}

pub unsafe fn dso__load_errno(dso: *mut dso) -> *mut dso_load_errno {
    unsafe { &mut (*dso).load_errno }
}

pub unsafe fn dso__set_loaded(dso: *mut dso) {
    unsafe { (*dso).loaded = true };
}

pub unsafe fn dso__lock(dso: *mut dso) -> *mut mutex {
    unsafe { &mut (*dso).lock }
}

pub unsafe fn dso__long_name(dso: *const dso) -> *const c_char {
    unsafe { (*dso).long_name }
}

pub unsafe fn dso__long_name_allocated(dso: *const dso) -> bool {
    unsafe { (*dso).long_name_allocated != 0 }
}

pub unsafe fn dso__set_long_name_allocated(dso: *mut dso, allocated: bool) {
    unsafe { (*dso).long_name_allocated = allocated as u8 };
}

pub unsafe fn dso__long_name_len(dso: *const dso) -> u16 {
    unsafe { (*dso).long_name_len }
}

pub unsafe fn dso__name(dso: *const dso) -> *const c_char {
    unsafe { (*dso).name.as_ptr() }
}

pub unsafe fn dso__needs_swap(dso: *const dso) -> dso_swap_type {
    unsafe { (*dso).needs_swap }
}

pub unsafe fn dso__set_needs_swap(dso: *mut dso, type_: dso_swap_type) {
    unsafe { (*dso).needs_swap = type_ };
}

pub unsafe fn dso__nsinfo(dso: *mut dso) -> *mut nsinfo {
    unsafe { (*dso).nsinfo }
}

pub unsafe fn dso__nsinfo_const(dso: *const dso) -> *const nsinfo {
    unsafe { (*dso).nsinfo }
}

pub unsafe fn dso__nsinfo_ptr(dso: *mut dso) -> *mut *mut nsinfo {
    unsafe { &mut (*dso).nsinfo }
}

unsafe extern "C" {
    pub fn dso__set_nsinfo(dso: *mut dso, nsi: *mut nsinfo);
}

pub unsafe fn dso__rel(dso: *const dso) -> u8 {
    unsafe { (*dso).rel }
}

pub unsafe fn dso__set_rel(dso: *mut dso, rel: u8) {
    unsafe { (*dso).rel = rel };
}

pub unsafe fn dso__short_name(dso: *const dso) -> *const c_char {
    unsafe { (*dso).short_name }
}

pub unsafe fn dso__short_name_allocated(dso: *const dso) -> bool {
    unsafe { (*dso).short_name_allocated != 0 }
}

pub unsafe fn dso__set_short_name_allocated(dso: *mut dso, allocated: bool) {
    unsafe { (*dso).short_name_allocated = allocated as u8 };
}

pub unsafe fn dso__short_name_len(dso: *const dso) -> u16 {
    unsafe { (*dso).short_name_len }
}

pub unsafe fn dso__srclines(dso: *mut dso) -> *mut rb_root_cached {
    unsafe { &mut (*dso).srclines }
}

pub unsafe fn dso__data_types(dso: *mut dso) -> *mut rb_root {
    unsafe { &mut (*dso).data_types }
}

pub unsafe fn dso__global_vars(dso: *mut dso) -> *mut rb_root {
    unsafe { &mut (*dso).global_vars }
}

pub unsafe fn dso__symbols(dso: *mut dso) -> *mut rb_root_cached {
    unsafe { &mut (*dso).symbols }
}

pub unsafe fn dso__symbol_names(dso: *mut dso) -> *mut *mut symbol {
    unsafe { (*dso).symbol_names }
}

pub unsafe fn dso__set_symbol_names(dso: *mut dso, names: *mut *mut symbol) {
    unsafe { (*dso).symbol_names = names };
}

pub unsafe fn dso__symbol_names_len(dso: *mut dso) -> size_t {
    unsafe { (*dso).symbol_names_len }
}

pub unsafe fn dso__set_symbol_names_len(dso: *mut dso, len: size_t) {
    unsafe { (*dso).symbol_names_len = len };
}

pub unsafe fn dso__symsrc_filename(dso: *const dso) -> *const c_char {
    unsafe { (*dso).symsrc_filename }
}

pub unsafe fn dso__set_symsrc_filename(dso: *mut dso, val: *mut c_char) {
    unsafe { (*dso).symsrc_filename = val };
}

pub unsafe fn dso__free_symsrc_filename(dso: *mut dso) {
    unsafe { zfree(&mut (*dso).symsrc_filename) };
}

pub unsafe fn dso__symtab_type(dso: *const dso) -> dso_binary_type {
    unsafe { (*dso).symtab_type }
}

pub unsafe fn dso__set_symtab_type(dso: *mut dso, bt: dso_binary_type) {
    unsafe { (*dso).symtab_type = bt };
}

pub unsafe fn dso__text_end(dso: *const dso) -> u64 {
    unsafe { (*dso).text_end }
}

pub unsafe fn dso__set_text_end(dso: *mut dso, val: u64) {
    unsafe { (*dso).text_end = val };
}

pub unsafe fn dso__text_offset(dso: *const dso) -> u64 {
    unsafe { (*dso).text_offset }
}

pub unsafe fn dso__set_text_offset(dso: *mut dso, val: u64) {
    unsafe { (*dso).text_offset = val };
}

unsafe extern "C" {
    pub fn dso__new_id(name: *const c_char, id: *const dso_id) -> *mut dso;
    pub fn dso__new(name: *const c_char) -> *mut dso;
    pub fn dso__delete(dso: *mut dso);

    pub fn dso__cmp_id(a: *mut dso, b: *mut dso) -> c_int;
    pub fn dso__set_short_name(dso: *mut dso, name: *const c_char, name_allocated: bool);
    pub fn dso__set_long_name(dso: *mut dso, name: *const c_char, name_allocated: bool);
    pub fn __dso__improve_id(dso: *mut dso, id: *const dso_id);

    pub fn dso__name_len(dso: *const dso) -> c_int;

    pub fn dso__get(dso: *mut dso) -> *mut dso;
    pub fn dso__put(dso: *mut dso);
}

pub unsafe fn __dso__zput(dso: *mut *mut dso) {
    unsafe {
        dso__put(*dso);
        *dso = core::ptr::null_mut();
    }
}

// dso__zput(dso) maps to __dso__zput(&dso).

unsafe extern "C" {
    pub fn dso__loaded(dso: *const dso) -> bool;
}

pub unsafe fn dso__has_symbols(dso: *const dso) -> bool {
    unsafe { !RB_EMPTY_ROOT(&(*dso).symbols.rb_root) }
}

unsafe extern "C" {
    pub fn dso__filename_with_chroot(dso: *const dso, filename: *const c_char) -> *mut c_char;

    pub fn dso__sorted_by_name(dso: *const dso) -> bool;
    pub fn dso__set_sorted_by_name(dso: *mut dso);
    pub fn dso__sort_by_name(dso: *mut dso);

    pub fn dso__swap_init(dso: *mut dso, eidata: c_uchar) -> c_int;

    pub fn dso__set_build_id(dso: *mut dso, bid: *const build_id);
    pub fn dso__build_id_equal(dso: *const dso, bid: *const build_id) -> bool;
    pub fn dso__read_running_kernel_build_id(dso: *mut dso, machine: *mut machine);
    pub fn dso__kernel_module_get_build_id(dso: *mut dso, root_dir: *const c_char) -> c_int;

    pub fn dso__symtab_origin(dso: *const dso) -> c_char;
    pub fn dso__read_binary_type_filename(
        dso: *const dso,
        type_: dso_binary_type,
        root_dir: *const c_char,
        filename: *mut c_char,
        size: size_t,
    ) -> c_int;
    pub fn is_kernel_module(pathname: *const c_char, cpumode: c_int) -> bool;
    pub fn dso__needs_decompress(dso: *mut dso) -> bool;
    pub fn dso__decompress_kmodule_fd(dso: *mut dso, name: *const c_char) -> c_int;
    pub fn dso__decompress_kmodule_path(
        dso: *mut dso,
        name: *const c_char,
        pathname: *mut c_char,
        len: size_t,
    ) -> c_int;
    pub fn filename__decompress(
        name: *const c_char,
        pathname: *mut c_char,
        len: size_t,
        comp: c_int,
        err: *mut c_int,
    ) -> c_int;
}

pub const KMOD_DECOMP_NAME: &[u8] = b"/tmp/perf-kmod-XXXXXX\0";
pub const KMOD_DECOMP_LEN: usize = KMOD_DECOMP_NAME.len();

#[repr(C)]
pub struct kmod_path {
    pub name: *mut c_char,
    pub comp: c_int,
    pub kmod: bool,
}

unsafe extern "C" {
    pub fn __kmod_path__parse(m: *mut kmod_path, path: *const c_char, alloc_name: bool) -> c_int;
}

pub unsafe fn kmod_path__parse(m: *mut kmod_path, p: *const c_char) -> c_int {
    unsafe { __kmod_path__parse(m, p, false) }
}

pub unsafe fn kmod_path__parse_name(m: *mut kmod_path, p: *const c_char) -> c_int {
    unsafe { __kmod_path__parse(m, p, true) }
}

unsafe extern "C" {
    pub fn dso__set_module_info(dso: *mut dso, m: *mut kmod_path, machine: *mut machine);
}

/*
 * The dso__data_* external interface provides following functions:
 *   dso__data_get_fd
 *   dso__data_put_fd
 *   dso__data_close
 *   dso__data_size
 *   dso__data_read_offset
 *   dso__data_read_addr
 *   dso__data_write_cache_offs
 *   dso__data_write_cache_addr
 *
 * Please refer to the dso.c object code for each function and
 * arguments documentation. Following text tries to explain the
 * dso file descriptor caching.
 *
 * The dso__data* interface allows caching of opened file descriptors
 * to speed up the dso data accesses. The idea is to leave the file
 * descriptor opened ideally for the whole life of the dso object.
 *
 * The current usage of the dso__data_* interface is as follows:
 *
 * Get DSO's fd:
 *   int fd;
 *   if (dso__data_get_fd(dso, machine, &fd)) {
 *       USE 'fd' SOMEHOW
 *       dso__data_put_fd(dso);
 *   }
 *
 * Read DSO's data:
 *   n = dso__data_read_offset(dso_0, &machine, 0, buf, BUFSIZE);
 *   n = dso__data_read_addr(dso_0, &machine, 0, buf, BUFSIZE);
 *
 * Eventually close DSO's fd:
 *   dso__data_close(dso);
 *
 * It is not necessary to close the DSO object data file. Each time new
 * DSO data file is opened, the limit (RLIMIT_NOFILE/2) is checked. Once
 * it is crossed, the oldest opened DSO object is closed.
 *
 * The dso__delete function calls close_dso function to ensure the
 * data file descriptor gets closed/unmapped before the dso object
 * is freed.
 *
 * TODO
 */
unsafe extern "C" {
    pub fn dso__data_get_fd(dso: *mut dso, machine: *mut machine, fd: *mut c_int) -> bool;
    pub fn dso__data_put_fd(dso: *mut dso);
    pub fn dso__data_close(dso: *mut dso);

    pub fn dso__data_file_size(dso: *mut dso, machine: *mut machine) -> c_int;
    pub fn dso__data_size(dso: *mut dso, machine: *mut machine) -> off_t;
    pub fn dso__data_read_offset(
        dso: *mut dso,
        machine: *mut machine,
        offset: u64,
        data: *mut u8,
        size: ssize_t,
    ) -> ssize_t;
    pub fn dso__read_e_machine_endian(
        optional_dso: *mut dso,
        fd: c_int,
        e_flags: *mut uint32_t,
        is_big_endian: *mut bool,
    ) -> uint16_t;
}

pub unsafe fn dso__read_e_machine(
    optional_dso: *mut dso,
    fd: c_int,
    e_flags: *mut uint32_t,
) -> uint16_t {
    unsafe { dso__read_e_machine_endian(optional_dso, fd, e_flags, core::ptr::null_mut()) }
}

unsafe extern "C" {
    pub fn dso__e_machine_endian(
        dso: *mut dso,
        machine: *mut machine,
        e_flags: *mut uint32_t,
        is_big_endian: *mut bool,
    ) -> uint16_t;
}

pub unsafe fn dso__e_machine(
    dso: *mut dso,
    machine: *mut machine,
    e_flags: *mut uint32_t,
) -> uint16_t {
    unsafe { dso__e_machine_endian(dso, machine, e_flags, core::ptr::null_mut()) }
}

unsafe extern "C" {
    pub fn dso__data_read_addr(
        dso: *mut dso,
        map: *mut map,
        machine: *mut machine,
        addr: u64,
        data: *mut u8,
        size: ssize_t,
    ) -> ssize_t;
    pub fn dso__data_status_seen(dso: *mut dso, by: dso_data_status_seen) -> bool;
    pub fn dso__data_write_cache_offs(
        dso: *mut dso,
        machine: *mut machine,
        offset: u64,
        data: *const u8,
        size: ssize_t,
    ) -> ssize_t;
    pub fn dso__data_write_cache_addr(
        dso: *mut dso,
        map: *mut map,
        machine: *mut machine,
        addr: u64,
        data: *const u8,
        size: ssize_t,
    ) -> ssize_t;

    pub fn dso__new_map(name: *const c_char) -> *mut map;
    pub fn machine__findnew_kernel(
        machine: *mut machine,
        name: *const c_char,
        short_name: *const c_char,
        dso_type: c_int,
    ) -> *mut dso;

    pub fn dso__reset_find_symbol_cache(dso: *mut dso);

    pub fn dso__fprintf_symbols_by_name(dso: *mut dso, fp: *mut FILE) -> size_t;
    pub fn dso__fprintf(dso: *mut dso, fp: *mut FILE) -> size_t;
}

pub unsafe fn dso__binary_type(dso: *const dso) -> dso_binary_type {
    unsafe { (*dso).binary_type }
}

pub unsafe fn dso__set_binary_type(dso: *mut dso, bt: dso_binary_type) {
    unsafe { (*dso).binary_type = bt };
}

pub unsafe fn dso__is_vmlinux(dso: *const dso) -> bool {
    let bt: dso_binary_type = unsafe { dso__binary_type(dso) };

    bt == dso_binary_type::DSO_BINARY_TYPE__VMLINUX
        || bt == dso_binary_type::DSO_BINARY_TYPE__GUEST_VMLINUX
}

pub unsafe fn dso__is_kcore(dso: *const dso) -> bool {
    let bt: dso_binary_type = unsafe { dso__binary_type(dso) };

    bt == dso_binary_type::DSO_BINARY_TYPE__KCORE
        || bt == dso_binary_type::DSO_BINARY_TYPE__GUEST_KCORE
}

pub unsafe fn dso__is_kallsyms(dso: *const dso) -> bool {
    let bt: dso_binary_type = unsafe { dso__binary_type(dso) };
    let name: *const c_char;

    if bt == dso_binary_type::DSO_BINARY_TYPE__KALLSYMS
        || bt == dso_binary_type::DSO_BINARY_TYPE__GUEST_KALLSYMS
    {
        return true;
    }

    if bt != dso_binary_type::DSO_BINARY_TYPE__NOT_FOUND {
        return false;
    }

    if unsafe { dso__kernel(dso) as c_int } == 0 {
        return false;
    }

    name = unsafe { dso__long_name(dso) };
    if name.is_null() {
        return false;
    }

    if unsafe { strcmp(name, DSO__NAME_KALLSYMS.as_ptr() as *const c_char) } == 0 {
        return true;
    }

    if unsafe { strcmp(name, DSO__NAME_GUEST_KALLSYMS.as_ptr() as *const c_char) } == 0 {
        return true;
    }

    unsafe { is_guest_kallsyms_pid_name(name) }
}

unsafe extern "C" {
    pub fn dso__is_object_file(dso: *const dso) -> bool;

    pub fn dso__free_a2l(dso: *mut dso);

    pub fn dso__type(dso: *mut dso, machine: *mut machine) -> dso_type;

    pub fn dso__strerror_load(dso: *mut dso, buf: *mut c_char, buflen: size_t) -> c_int;

    pub fn reset_fd_limit();

    pub fn dso__find_global_type(dso: *mut dso, addr: u64) -> u64;
    pub fn dso__findnew_global_type(dso: *mut dso, addr: u64, offset: u64) -> u64;

    /* Check if dso name is of format "/tmp/perf-%d.map" */
    pub fn perf_pid_map_tid(dso_name: *const c_char, tid: *mut c_int) -> bool;
    pub fn is_perf_pid_map_name(dso_name: *const c_char) -> bool;

    pub fn dso__debuginfo(dso: *mut dso) -> *mut debuginfo;

    pub fn dso__read_symbol(
        dso: *mut dso,
        symfs_filename: *const c_char,
        map: *const map,
        sym: *const symbol,
        out_buf: *mut *mut u8,
        out_buf_len: *mut u64,
        is_64bit: *mut bool,
    ) -> *const u8;
}
