// SPDX-License-Identifier: GPL-2.0
// Rust translation of perf/util/dso.c.
//
// C include dependencies preserved as external declarations below:
// asm/bug.h, linux/kernel.h, linux/string.h, linux/zalloc.h, sys/time.h,
// sys/resource.h, sys/types.h, sys/stat.h, unistd.h, errno.h, fcntl.h,
// stdlib.h, compress.h, env.h, namespaces.h, path.h, map.h, symbol.h,
// srcline.h, dso.h, dsos.h, machine.h, auxtrace.h, util.h, debug.h,
// string2.h, vdso.h, annotate-data.h, libdw.h.
// HAVE_LIBBPF_SUPPORT / HAVE_ZLIB_SUPPORT / HAVE_LZMA_SUPPORT conditional
// code is translated and annotated where the build-time condition is local to C.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_uchar, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type off_t = i64;
type rlim_t = u64;
type u8 = u8;
type u32 = u32;
type u64 = u64;
type uint16_t = u16;
type uint32_t = u32;

const PATH_MAX: size_t = 4096;
const SBUILD_ID_SIZE: size_t = 41;
const BUILD_ID_SIZE: u8 = 20;
const KMOD_DECOMP_LEN: size_t = 256;
const KMOD_DECOMP_NAME: &[u8] = b"/tmp/perf-kmod-XXXXXX\0";
const DSO__DATA_CACHE_SIZE: ssize_t = 4096;
const DSO__DATA_CACHE_MASK: u64 = !((DSO__DATA_CACHE_SIZE as u64) - 1);
const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2000000;
const EMFILE: c_int = 24;
const ENOENT: c_int = 2;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const EIO: c_int = 5;
const EOPNOTSUPP: c_int = 95;
const RLIMIT_NOFILE: c_int = 7;
const RLIM_INFINITY: rlim_t = !0;
const PERF_RECORD_MISC_CPUMODE_MASK: c_int = 7;
const PERF_RECORD_MISC_USER: c_int = 2;
const PERF_RECORD_MISC_HYPERVISOR: c_int = 3;
const PERF_RECORD_MISC_GUEST_USER: c_int = 5;
const EI_NIDENT: usize = 16;
const EI_CLASS: usize = 4;
const EI_DATA: usize = 5;
const EI_VERSION: usize = 6;
const ELFMAG: &[u8; 4] = b"\x7fELF";
const SELFMAG: usize = 4;
const ELFCLASSNONE: c_uchar = 0;
const ELFCLASS32: c_uchar = 1;
const ELFCLASSNUM: c_uchar = 3;
const ELFDATA2LSB: c_uchar = 1;
const ELFDATA2MSB: c_uchar = 2;
const EV_CURRENT: c_uchar = 1;
const EM_NONE: uint16_t = 0;
const EM_CSKY: uint16_t = 252;
const EM_NUM: uint16_t = 253;
const __DSO_LOAD_ERRNO__START: c_int = -1000;
const __DSO_LOAD_ERRNO__END: c_int = -995;
const DSO_LOAD_ERRNO__DECOMPRESSION_FAILURE: c_int = __DSO_LOAD_ERRNO__START + 4;
const SYMBOL_ANNOTATE_ERRNO__BPF_MISSING_BTF: c_int = 10001;

#[repr(C)] pub struct dso { _private: [u8; 0] }
#[repr(C)] pub struct nsinfo { _private: [u8; 0] }
#[repr(C)] pub struct machine { pub root_dir: *const c_char, pub env: *mut perf_env }
#[repr(C)] pub struct perf_env { _private: [u8; 0] }
#[repr(C)] pub struct map { _private: [u8; 0] }
#[repr(C)] pub struct symbol { pub rb_node: rb_node, pub start: u64, pub end: u64 }
#[repr(C)] pub struct dsos { pub lock: rw_semaphore, pub sorted: bool }
#[repr(C)] pub struct debuginfo { _private: [u8; 0] }
#[repr(C)] pub struct FILE { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct rw_semaphore { _private: [u8; 0] }
#[repr(C)] pub struct refcount_t { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct rb_node { pub rb_left: *mut rb_node, pub rb_right: *mut rb_node, pub rb_parent_color: usize }
#[repr(C)] pub struct rb_root { pub rb_node: *mut rb_node }
#[repr(C)] pub struct rb_root_cached { pub rb_root: rb_root, pub rb_leftmost: *mut rb_node }
#[repr(C)] pub struct stat { pub st_size: off_t }
#[repr(C)] pub struct rlimit { pub rlim_cur: rlim_t, pub rlim_max: rlim_t }
#[repr(C)] pub struct nscookie { _private: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct build_id {
    pub size: u8,
    pub data: [u8; 32],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dso_id {
    pub maj: u64,
    pub min: u64,
    pub ino: u64,
    pub ino_generation: u64,
    pub mmap2_valid: bool,
    pub mmap2_ino_generation_valid: bool,
    pub build_id: build_id,
}

#[repr(C)]
pub struct dso_data {
    pub open_entry: list_head,
    pub fd: c_int,
    pub file_size: off_t,
    pub status: dso_data_status,
    pub status_seen: u32,
    pub cache: rb_root,
    #[cfg(REFCNT_CHECKING)]
    pub dso: *mut dso,
}

#[repr(C)]
pub struct dso_cache {
    pub rb_node: rb_node,
    pub offset: u64,
    pub size: u64,
    pub data: [u8; 0],
}

#[repr(C)]
pub struct kmod_path {
    pub name: *mut c_char,
    pub ext: *mut c_char,
    pub comp: c_int,
    pub kmod: bool,
}

#[repr(C)]
pub struct Elf32_Ehdr {
    pub e_ident: [c_uchar; EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
}

#[repr(C)]
pub struct Elf64_Ehdr {
    pub e_ident: [c_uchar; EI_NIDENT],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dso_binary_type {
    DSO_BINARY_TYPE__KALLSYMS = 0,
    DSO_BINARY_TYPE__VMLINUX,
    DSO_BINARY_TYPE__JAVA_JIT,
    DSO_BINARY_TYPE__DEBUGLINK,
    DSO_BINARY_TYPE__BUILD_ID_CACHE,
    DSO_BINARY_TYPE__BUILD_ID_CACHE_DEBUGINFO,
    DSO_BINARY_TYPE__FEDORA_DEBUGINFO,
    DSO_BINARY_TYPE__UBUNTU_DEBUGINFO,
    DSO_BINARY_TYPE__MIXEDUP_UBUNTU_DEBUGINFO,
    DSO_BINARY_TYPE__OPENEMBEDDED_DEBUGINFO,
    DSO_BINARY_TYPE__BUILDID_DEBUGINFO,
    DSO_BINARY_TYPE__SYSTEM_PATH_DSO,
    DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE,
    DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE_COMP,
    DSO_BINARY_TYPE__GUEST_KALLSYMS,
    DSO_BINARY_TYPE__GUEST_KMODULE,
    DSO_BINARY_TYPE__GUEST_KMODULE_COMP,
    DSO_BINARY_TYPE__GUEST_VMLINUX,
    DSO_BINARY_TYPE__GNU_DEBUGDATA,
    DSO_BINARY_TYPE__BPF_PROG_INFO,
    DSO_BINARY_TYPE__BPF_IMAGE,
    DSO_BINARY_TYPE__OOL,
    DSO_BINARY_TYPE__KCORE,
    DSO_BINARY_TYPE__GUEST_KCORE,
    DSO_BINARY_TYPE__NOT_FOUND,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dso_data_status {
    DSO_DATA_STATUS_UNKNOWN = 0,
    DSO_DATA_STATUS_OK,
    DSO_DATA_STATUS_ERROR,
}

#[repr(C)] pub enum dso_data_status_seen { DSO_DATA_STATUS_SEEN_ITRACE = 0 }
#[repr(C)] pub enum dso_type { DSO__TYPE_UNKNOWN = 0 }
#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)] pub enum dso_swap_type { DSO_SWAP__UNSET = 0, DSO_SWAP__NO, DSO_SWAP__YES }
#[repr(C)] pub enum dso_space_type { DSO_SPACE__USER = 0 }

#[repr(C)]
struct compression {
    fmt: *const c_char,
    decompress: Option<unsafe extern "C" fn(*const c_char, c_int) -> c_int>,
    is_compressed: Option<unsafe extern "C" fn(*const c_char) -> bool>,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut verbose: c_int;
    static mut symbol_conf: symbol_conf_t;

    fn nsinfo__put(nsi: *mut nsinfo);
    fn nsinfo__zput(nsi: *mut nsinfo);
    fn nsinfo__pid(nsi: *const nsinfo) -> c_int;
    fn nsinfo__mountns_enter(nsi: *mut nsinfo, nsc: *mut nscookie);
    fn nsinfo__mountns_exit(nsc: *mut nscookie);
    fn filename_with_chroot(pid: c_int, filename: *const c_char) -> *mut c_char;
    fn __symbol__join_symfs(filename: *mut c_char, size: size_t, path: *const c_char) -> size_t;
    fn filename__read_debuglink(filename: *const c_char, symfile: *mut c_char, size: size_t) -> c_int;
    fn is_regular_file(name: *const c_char) -> bool;
    fn dso__build_id_filename(dso: *const dso, filename: *mut c_char, size: size_t, debug: bool) -> *mut c_char;
    fn dso__has_build_id(dso: *const dso) -> bool;
    fn build_id__snprintf(bid: *const build_id, s: *mut c_char, size: size_t) -> c_int;
    fn path__join3(filename: *mut c_char, size: size_t, a: *const c_char, b: *const c_char, c: *const c_char) -> c_int;
    fn gzip_decompress_to_file(input: *const c_char, output: c_int) -> c_int;
    fn gzip_is_compressed(input: *const c_char) -> bool;
    fn lzma_decompress_to_file(input: *const c_char, output: c_int) -> c_int;
    fn lzma_is_compressed(input: *const c_char) -> bool;
    fn kmod_path__parse(m: *mut kmod_path, path: *const c_char) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
    fn pr_debug(fmt: *const c_char, ...);
    fn malloc(size: size_t) -> *mut c_void;
    fn zalloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strdup(s: *const c_char) -> *mut c_char;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(a: *const c_void, b: *const c_void, n: size_t) -> c_int;
    fn memchr_inv(s: *const c_void, c: c_int, n: size_t) -> *mut c_void;
    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(a: *const c_char, b: *const c_char, n: size_t) -> c_int;
    fn strcmp(a: *const c_char, b: *const c_char) -> c_int;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strreplace(s: *mut c_char, old: c_char, new_: c_char);
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn fprintf(fp: *mut FILE, fmt: *const c_char, ...) -> size_t;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pread(fd: c_int, buf: *mut c_void, count: size_t, offset: off_t) -> ssize_t;
    fn stat(path: *const c_char, st: *mut stat) -> c_int;
    fn fstat(fd: c_int, st: *mut stat) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
    fn mkostemp(template: *mut c_char, flags: c_int) -> c_int;
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn mutex_init(m: *mut mutex);
    fn mutex_destroy(m: *mut mutex);
    fn mutex_lock(m: *mut mutex);
    fn mutex_unlock(m: *mut mutex);
    fn down_write(lock: *mut rw_semaphore);
    fn up_write(lock: *mut rw_semaphore);
    fn rb_first(root: *const rb_root) -> *mut rb_node;
    fn rb_first_cached(root: *const rb_root_cached) -> *mut rb_node;
    fn rb_next(node: *const rb_node) -> *mut rb_node;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn list_add_tail(new_: *mut list_head, head: *mut list_head);
    fn list_del_init(entry: *mut list_head);
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn assert_fail();
    fn dso__symtab_type(dso: *const dso) -> dso_binary_type;
    fn dso__set_symtab_type(dso: *mut dso, ty: dso_binary_type);
    fn dso__binary_type(dso: *const dso) -> dso_binary_type;
    fn dso__set_binary_type(dso: *mut dso, ty: dso_binary_type);
    fn dso__long_name(dso: *const dso) -> *const c_char;
    fn dso__long_name_len(dso: *const dso) -> size_t;
    fn dso__short_name(dso: *const dso) -> *const c_char;
    fn dso__short_name_len(dso: *const dso) -> c_int;
    fn dso__long_name_allocated(dso: *const dso) -> bool;
    fn dso__short_name_allocated(dso: *const dso) -> bool;
    fn dso__set_long_name_allocated(dso: *mut dso, v: bool);
    fn dso__set_short_name_allocated(dso: *mut dso, v: bool);
    fn dso__nsinfo(dso: *mut dso) -> *mut nsinfo;
    fn dso__nsinfo_const(dso: *const dso) -> *const nsinfo;
    fn dso__data(dso: *mut dso) -> *mut dso_data;
    fn dso__data_const(dso: *const dso) -> *const dso_data;
    fn dso__lock(dso: *mut dso) -> *mut mutex;
    fn dso__dsos(dso: *const dso) -> *mut dsos;
    fn dso__id(dso: *mut dso) -> *mut dso_id;
    fn dso__id_const(dso: *const dso) -> *const dso_id;
    fn dso__bid(dso: *const dso) -> *const build_id;
    fn dso__comp(dso: *const dso) -> c_int;
    fn dso__set_comp(dso: *mut dso, comp: c_int);
    fn dso__load_errno(dso: *mut dso) -> *mut c_int;
    fn dso__set_is_kmod(dso: *mut dso);
    fn dso__set_kernel(dso: *mut dso, ty: c_int);
    fn dso__symbols(dso: *mut dso) -> *mut rb_root_cached;
    fn dso__data_types(dso: *mut dso) -> *mut rb_root;
    fn dso__global_vars(dso: *mut dso) -> *mut rb_root;
    fn dso__needs_swap(dso: *const dso) -> dso_swap_type;
    fn dso__set_needs_swap(dso: *mut dso, ty: dso_swap_type);
    fn machine__is_host(machine: *mut machine) -> bool;
    fn machine__is_default_guest(machine: *mut machine) -> bool;
    fn machine__findnew_dso(machine: *mut machine, name: *const c_char) -> *mut dso;
    fn map__new2(start: u64, dso: *mut dso) -> *mut map;
    fn map__map_ip(map: *mut map, addr: u64) -> u64;
    fn map__rip_2objdump(map: *const map, addr: u64) -> u64;
    fn perf_arch_is_big_endian(arch: *const c_char) -> bool;
    fn perf_env__arch(env: *mut perf_env) -> *const c_char;
    fn perf_env__e_machine(env: *mut perf_env, e_flags: *mut uint32_t) -> uint16_t;
    fn dso__type_fd(fd: c_int) -> dso_type;
    fn build_id__is_defined(bid: *const build_id) -> bool;
    fn sysfs__read_build_id(path: *const c_char, bid: *mut build_id) -> c_int;
    fn symbol__fprintf(sym: *mut symbol, fp: *mut FILE) -> size_t;
    fn file__read_maps(fd: c_int, exe: bool, cb: extern "C" fn(u64, u64, u64, *mut c_void) -> c_int, arg: *mut c_void, is_64bit: *mut bool) -> c_int;
    fn debuginfo__new(name: *const c_char) -> *mut debuginfo;
    fn inlines__tree_delete(root: *mut rb_root_cached);
    fn srcline__tree_delete(root: *mut rb_root_cached);
    fn symbols__delete(root: *mut rb_root_cached);
    fn zfree(ptr: *mut *mut c_char);
    fn annotated_data_type__tree_delete(root: *mut rb_root);
    fn global_var_type__tree_delete(root: *mut rb_root);
    fn auxtrace_cache__free(ptr: *mut c_void);
    fn dso__free_a2l(dso: *mut dso);
    fn dso__free_libdw(dso: *mut dso);
    fn dso__free_symsrc_filename(dso: *mut dso);
    fn refcount_set(r: *mut refcount_t, v: c_int);
    fn refcount_inc(r: *mut refcount_t);
    fn refcount_read(r: *const refcount_t) -> c_int;
    fn refcount_dec_and_test(r: *mut refcount_t) -> bool;
}

#[repr(C)]
pub struct symbol_conf_t { pub symfs: *const c_char }

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const c_char }; }
unsafe fn min_u64(a: u64, b: u64) -> u64 { if a < b { a } else { b } }
unsafe fn warn_once(cond: bool, fmt: *const c_char, arg: c_int) { if cond { pr_err(fmt, arg); } }
unsafe fn bug_on(cond: bool) { if cond { assert_fail(); } }
unsafe fn rc_access<T>(p: *mut T) -> *mut T { p }
unsafe fn container_of_dso_from_data(_data: *mut dso_data) -> *mut dso { ptr::null_mut() /* external container_of layout dependency */ }
unsafe fn rb_entry_cache(node: *mut rb_node) -> *mut dso_cache { node as *mut dso_cache }
unsafe fn rb_entry_symbol(node: *mut rb_node) -> *mut symbol { node as *mut symbol }

static DEBUGLINK_PATHS: [*const c_char; 4] = [
    cstr!("%.0s%s"),
    cstr!("%s/%s"),
    cstr!("%s/.debug/%s"),
    cstr!("/usr/lib/debug%s/%s"),
];

#[no_mangle]
pub unsafe extern "C" fn dso__set_nsinfo(dso: *mut dso, nsi: *mut nsinfo) {
    nsinfo__put((*rc_access(dso)).nsinfo);
    (*rc_access(dso)).nsinfo = nsi;
}

#[no_mangle]
pub unsafe extern "C" fn dso__symtab_origin(dso: *const dso) -> c_char {
    let origin = [
        (dso_binary_type::DSO_BINARY_TYPE__KALLSYMS, b'k'),
        (dso_binary_type::DSO_BINARY_TYPE__VMLINUX, b'v'),
        (dso_binary_type::DSO_BINARY_TYPE__JAVA_JIT, b'j'),
        (dso_binary_type::DSO_BINARY_TYPE__DEBUGLINK, b'l'),
        (dso_binary_type::DSO_BINARY_TYPE__BUILD_ID_CACHE, b'B'),
        (dso_binary_type::DSO_BINARY_TYPE__BUILD_ID_CACHE_DEBUGINFO, b'D'),
        (dso_binary_type::DSO_BINARY_TYPE__FEDORA_DEBUGINFO, b'f'),
        (dso_binary_type::DSO_BINARY_TYPE__UBUNTU_DEBUGINFO, b'u'),
        (dso_binary_type::DSO_BINARY_TYPE__MIXEDUP_UBUNTU_DEBUGINFO, b'x'),
        (dso_binary_type::DSO_BINARY_TYPE__OPENEMBEDDED_DEBUGINFO, b'o'),
        (dso_binary_type::DSO_BINARY_TYPE__BUILDID_DEBUGINFO, b'b'),
        (dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_DSO, b'd'),
        (dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE, b'K'),
        (dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE_COMP, b'm'),
        (dso_binary_type::DSO_BINARY_TYPE__GUEST_KALLSYMS, b'g'),
        (dso_binary_type::DSO_BINARY_TYPE__GUEST_KMODULE, b'G'),
        (dso_binary_type::DSO_BINARY_TYPE__GUEST_KMODULE_COMP, b'M'),
        (dso_binary_type::DSO_BINARY_TYPE__GUEST_VMLINUX, b'V'),
        (dso_binary_type::DSO_BINARY_TYPE__GNU_DEBUGDATA, b'n'),
    ];
    if dso.is_null() || dso__symtab_type(dso) == dso_binary_type::DSO_BINARY_TYPE__NOT_FOUND {
        return b'!' as c_char;
    }
    let ty = dso__symtab_type(dso);
    for (k, v) in origin {
        if k == ty { return v as c_char; }
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn dso__is_object_file(dso: *const dso) -> bool {
    match dso__binary_type(dso) {
        dso_binary_type::DSO_BINARY_TYPE__KALLSYMS
        | dso_binary_type::DSO_BINARY_TYPE__GUEST_KALLSYMS
        | dso_binary_type::DSO_BINARY_TYPE__JAVA_JIT
        | dso_binary_type::DSO_BINARY_TYPE__BPF_PROG_INFO
        | dso_binary_type::DSO_BINARY_TYPE__BPF_IMAGE
        | dso_binary_type::DSO_BINARY_TYPE__OOL => false,
        _ => true,
    }
}

#[no_mangle]
pub unsafe extern "C" fn dso__read_binary_type_filename(dso: *const dso, ty: dso_binary_type, root_dir: *const c_char, filename: *mut c_char, size: size_t) -> c_int {
    let mut build_id_hex = [0 as c_char; SBUILD_ID_SIZE];
    let mut ret = 0;
    let mut len: size_t;
    match ty {
        dso_binary_type::DSO_BINARY_TYPE__DEBUGLINK => {
            let mut dso_dir = [0 as c_char; PATH_MAX];
            let mut symfile = [0 as c_char; PATH_MAX];
            len = __symbol__join_symfs(filename, size, dso__long_name(dso));
            let mut last_slash = filename.add(len);
            while last_slash != filename && *last_slash != b'/' as c_char { last_slash = last_slash.sub(1); }
            strncpy(dso_dir.as_mut_ptr(), filename, last_slash.offset_from(filename) as size_t);
            dso_dir[last_slash.offset_from(filename) as usize] = 0;
            if !is_regular_file(filename) { ret = -1; }
            else {
                ret = filename__read_debuglink(filename, symfile.as_mut_ptr(), PATH_MAX);
                if ret == 0 {
                    ret = -1;
                    for fmt in DEBUGLINK_PATHS {
                        snprintf(filename, size, fmt, dso_dir.as_ptr(), symfile.as_ptr());
                        if is_regular_file(filename) { ret = 0; break; }
                    }
                }
            }
        }
        dso_binary_type::DSO_BINARY_TYPE__BUILD_ID_CACHE => {
            if dso__build_id_filename(dso, filename, size, false).is_null() { ret = -1; }
        }
        dso_binary_type::DSO_BINARY_TYPE__BUILD_ID_CACHE_DEBUGINFO => {
            if dso__build_id_filename(dso, filename, size, true).is_null() { ret = -1; }
        }
        dso_binary_type::DSO_BINARY_TYPE__FEDORA_DEBUGINFO => {
            len = __symbol__join_symfs(filename, size, cstr!("/usr/lib/debug"));
            snprintf(filename.add(len), size - len, cstr!("%s.debug"), dso__long_name(dso));
        }
        dso_binary_type::DSO_BINARY_TYPE__UBUNTU_DEBUGINFO => {
            len = __symbol__join_symfs(filename, size, cstr!("/usr/lib/debug"));
            snprintf(filename.add(len), size - len, cstr!("%s"), dso__long_name(dso));
        }
        dso_binary_type::DSO_BINARY_TYPE__MIXEDUP_UBUNTU_DEBUGINFO => {
            if strlen(dso__long_name(dso)) < 9 || strncmp(dso__long_name(dso), cstr!("/usr/lib/"), 9) != 0 { ret = -1; }
            else {
                len = __symbol__join_symfs(filename, size, cstr!("/usr/lib/debug"));
                snprintf(filename.add(len), size - len, cstr!("%s"), dso__long_name(dso).add(4));
            }
        }
        dso_binary_type::DSO_BINARY_TYPE__OPENEMBEDDED_DEBUGINFO => {
            let mut last_slash = dso__long_name(dso).add(dso__long_name_len(dso));
            while last_slash != dso__long_name(dso) && *last_slash != b'/' as c_char { last_slash = last_slash.sub(1); }
            len = __symbol__join_symfs(filename, size, cstr!(""));
            let dir_size = last_slash.offset_from(dso__long_name(dso)) as size_t + 2;
            if dir_size > size - len { ret = -1; }
            else {
                len += scnprintf(filename.add(len), dir_size, cstr!("%s"), dso__long_name(dso)) as size_t;
                let _ = scnprintf(filename.add(len), size - len, cstr!(".debug%s"), last_slash);
            }
        }
        dso_binary_type::DSO_BINARY_TYPE__BUILDID_DEBUGINFO => {
            if !dso__has_build_id(dso) { ret = -1; }
            else {
                build_id__snprintf(dso__bid(dso), build_id_hex.as_mut_ptr(), build_id_hex.len());
                len = __symbol__join_symfs(filename, size, cstr!("/usr/lib/debug/.build-id/"));
                snprintf(filename.add(len), size - len, cstr!("%.2s/%s.debug"), build_id_hex.as_ptr(), build_id_hex.as_ptr().add(2));
            }
        }
        dso_binary_type::DSO_BINARY_TYPE__VMLINUX
        | dso_binary_type::DSO_BINARY_TYPE__GUEST_VMLINUX
        | dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_DSO
        | dso_binary_type::DSO_BINARY_TYPE__GNU_DEBUGDATA => { __symbol__join_symfs(filename, size, dso__long_name(dso)); }
        dso_binary_type::DSO_BINARY_TYPE__GUEST_KMODULE | dso_binary_type::DSO_BINARY_TYPE__GUEST_KMODULE_COMP => {
            path__join3(filename, size, symbol_conf.symfs, root_dir, dso__long_name(dso));
        }
        dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE | dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE_COMP => {
            __symbol__join_symfs(filename, size, dso__long_name(dso));
        }
        dso_binary_type::DSO_BINARY_TYPE__KCORE | dso_binary_type::DSO_BINARY_TYPE__GUEST_KCORE => {
            snprintf(filename, size, cstr!("%s"), dso__long_name(dso));
        }
        _ => ret = -1,
    }
    ret
}

const COMP_ID__NONE: c_int = 0;

// Entries gated by HAVE_ZLIB_SUPPORT/HAVE_LZMA_SUPPORT in C are kept as extern-backed table entries.
static COMPRESSIONS: [compression; 4] = [
    compression { fmt: ptr::null(), decompress: None, is_compressed: None },
    compression { fmt: cstr!("gz"), decompress: Some(gzip_decompress_to_file), is_compressed: Some(gzip_is_compressed) },
    compression { fmt: cstr!("xz"), decompress: Some(lzma_decompress_to_file), is_compressed: Some(lzma_is_compressed) },
    compression { fmt: ptr::null(), decompress: None, is_compressed: None },
];

unsafe fn is_supported_compression(ext: *const c_char) -> c_int {
    let mut i = 1usize;
    while !COMPRESSIONS[i].fmt.is_null() {
        if strcmp(ext, COMPRESSIONS[i].fmt) == 0 { return i as c_int; }
        i += 1;
    }
    COMP_ID__NONE
}

#[no_mangle]
pub unsafe extern "C" fn is_kernel_module(pathname: *const c_char, cpumode: c_int) -> bool {
    let mut m: kmod_path = core::mem::zeroed();
    let mode = cpumode & PERF_RECORD_MISC_CPUMODE_MASK;
    warn_once(mode != cpumode, cstr!("Internal error: passing unmasked cpumode (%x) to is_kernel_module"), cpumode);
    match mode {
        PERF_RECORD_MISC_USER | PERF_RECORD_MISC_HYPERVISOR | PERF_RECORD_MISC_GUEST_USER => false,
        _ => {
            if kmod_path__parse(&mut m, pathname) != 0 {
                pr_err(cstr!("Failed to check whether %s is a kernel module or not. Assume it is."), pathname);
                true
            } else { m.kmod }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn dso__needs_decompress(dso: *mut dso) -> bool {
    dso__symtab_type(dso) == dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE_COMP ||
    dso__symtab_type(dso) == dso_binary_type::DSO_BINARY_TYPE__GUEST_KMODULE_COMP
}

#[no_mangle]
pub unsafe extern "C" fn filename__decompress(name: *const c_char, pathname: *mut c_char, len: size_t, comp: c_int, err: *mut c_int) -> c_int {
    let mut tmpbuf = [0 as c_char; KMOD_DECOMP_LEN];
    let src = KMOD_DECOMP_NAME.as_ptr() as *const c_char;
    strcpy(tmpbuf.as_mut_ptr(), src);
    let mut fd: c_int;
    if !(COMPRESSIONS[comp as usize].is_compressed.unwrap())(name) {
        fd = open(name, O_RDONLY | O_CLOEXEC);
        if fd < 0 { *err = errno; }
        if !pathname.is_null() && len > 0 { *pathname = 0; }
        return fd;
    }
    fd = mkostemp(tmpbuf.as_mut_ptr(), O_CLOEXEC);
    if fd < 0 { *err = errno; return -1; }
    if (COMPRESSIONS[comp as usize].decompress.unwrap())(name, fd) != 0 {
        *err = DSO_LOAD_ERRNO__DECOMPRESSION_FAILURE;
        close(fd);
        fd = -1;
    }
    if pathname.is_null() || fd < 0 { unlink(tmpbuf.as_ptr()); }
    if !pathname.is_null() && fd >= 0 { snprintf(pathname, len, cstr!("%s"), tmpbuf.as_ptr()); }
    fd
}

unsafe fn decompress_kmodule(dso: *mut dso, name: *const c_char, pathname: *mut c_char, len: size_t) -> c_int {
    if !dso__needs_decompress(dso) || dso__comp(dso) == COMP_ID__NONE { return -1; }
    filename__decompress(name, pathname, len, dso__comp(dso), dso__load_errno(dso))
}

#[no_mangle] pub unsafe extern "C" fn dso__decompress_kmodule_fd(dso: *mut dso, name: *const c_char) -> c_int { decompress_kmodule(dso, name, ptr::null_mut(), 0) }

#[no_mangle]
pub unsafe extern "C" fn dso__decompress_kmodule_path(dso: *mut dso, name: *const c_char, pathname: *mut c_char, len: size_t) -> c_int {
    let fd = decompress_kmodule(dso, name, pathname, len);
    if fd >= 0 { close(fd); }
    if fd >= 0 { 0 } else { -1 }
}

#[no_mangle]
pub unsafe extern "C" fn __kmod_path__parse(m: *mut kmod_path, path: *const c_char, alloc_name: bool) -> c_int {
    let mut name = strrchr(path, b'/' as c_int);
    let mut ext = strrchr(path, b'.' as c_int);
    let mut is_simple_name = false;
    memset(m as *mut c_void, 0, size_of::<kmod_path>());
    name = if !name.is_null() { name.add(1) } else { path as *mut c_char };
    if *name == b'[' as c_char {
        is_simple_name = true;
        if strncmp(name, cstr!("[kernel.kallsyms]"), 17) == 0 ||
           strncmp(name, cstr!("[guest.kernel.kallsyms"), 22) == 0 ||
           strncmp(name, cstr!("[vdso]"), 6) == 0 ||
           strncmp(name, cstr!("[vdso32]"), 8) == 0 ||
           strncmp(name, cstr!("[vdsox32]"), 9) == 0 ||
           strncmp(name, cstr!("[vsyscall]"), 10) == 0 {
            (*m).kmod = false;
        } else { (*m).kmod = true; }
    }
    if ext.is_null() || is_simple_name {
        if alloc_name {
            (*m).name = strdup(name);
            return if !(*m).name.is_null() { 0 } else { -ENOMEM };
        }
        return 0;
    }
    (*m).comp = is_supported_compression(ext.add(1));
    if (*m).comp > COMP_ID__NONE { ext = ext.sub(3); }
    if (ext as *const c_char) > (name as *const c_char) { (*m).kmod = strncmp(ext, cstr!(".ko"), 3) == 0; }
    if alloc_name {
        if (*m).kmod {
            if asprintf(&mut (*m).name, cstr!("[%.*s]"), ext.offset_from(name) as c_int, name) == -1 { return -ENOMEM; }
        } else if asprintf(&mut (*m).name, cstr!("%s"), name) == -1 { return -ENOMEM; }
        strreplace((*m).name, b'-' as c_char, b'_' as c_char);
    }
    0
}

#[no_mangle]
pub unsafe extern "C" fn dso__set_module_info(dso: *mut dso, m: *mut kmod_path, machine: *mut machine) {
    if machine__is_host(machine) { dso__set_symtab_type(dso, dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE); }
    else { dso__set_symtab_type(dso, dso_binary_type::DSO_BINARY_TYPE__GUEST_KMODULE); }
    if (*m).kmod && (*m).comp != 0 {
        let next = core::mem::transmute::<i32, dso_binary_type>(dso__symtab_type(dso) as i32 + 1);
        dso__set_symtab_type(dso, next);
        dso__set_comp(dso, (*m).comp);
    }
    dso__set_is_kmod(dso);
    dso__set_short_name(dso, strdup((*m).name), true);
}

static mut _dso__data_open_lock: mutex = mutex { _private: [] };
static mut dso__data_open: list_head = list_head { next: ptr::null_mut(), prev: ptr::null_mut() };
static mut dso__data_open_cnt: c_long = 0;
static mut fd_limit: rlim_t = 0;

unsafe fn dso__data_open_lock_init() { mutex_init(&mut _dso__data_open_lock); INIT_LIST_HEAD(&mut dso__data_open); }
unsafe fn dso__data_open_lock_fn() -> *mut mutex { &mut _dso__data_open_lock }

unsafe fn dso__list_add(dso: *mut dso) {
    list_add_tail(&mut (*dso__data(dso)).open_entry, &mut dso__data_open);
    if dso__dsos(dso).is_null() { assert_fail(); }
    dso__data_open_cnt += 1;
}

unsafe fn dso__list_del(dso: *mut dso) {
    list_del_init(&mut (*dso__data(dso)).open_entry);
    if dso__data_open_cnt <= 0 { pr_err(cstr!("DSO data fd counter out of bounds.")); }
    dso__data_open_cnt -= 1;
}

unsafe fn close_first_dso();

unsafe fn do_open(name: *mut c_char) -> c_int {
    loop {
        let fd = open(name, O_RDONLY | O_CLOEXEC);
        if fd >= 0 { return fd; }
        pr_debug(cstr!("dso open failed: %m\n"));
        if dso__data_open_cnt == 0 || errno != EMFILE { break; }
        close_first_dso();
    }
    -1
}

#[no_mangle]
pub unsafe extern "C" fn dso__filename_with_chroot(dso: *const dso, filename: *const c_char) -> *mut c_char {
    filename_with_chroot(nsinfo__pid(dso__nsinfo_const(dso)), filename)
}

unsafe fn dso__get_filename(dso: *mut dso, root_dir: *const c_char, decomp: *mut bool) -> *mut c_char {
    let mut name = malloc(PATH_MAX) as *mut c_char;
    *decomp = false;
    if name.is_null() { return ptr::null_mut(); }
    if dso__read_binary_type_filename(dso, dso__binary_type(dso), root_dir, name, PATH_MAX) != 0 { free(name as *mut c_void); return ptr::null_mut(); }
    if !is_regular_file(name) {
        let mut st: stat = core::mem::zeroed();
        if stat(name, &mut st) == 0 || errno != ENOENT || dso__nsinfo(dso).is_null() {
            free(name as *mut c_void); return ptr::null_mut();
        }
        let new_name = dso__filename_with_chroot(dso, name);
        if new_name.is_null() { free(name as *mut c_void); return ptr::null_mut(); }
        free(name as *mut c_void);
        name = new_name;
    }
    if dso__needs_decompress(dso) {
        let mut newpath = [0 as c_char; KMOD_DECOMP_LEN];
        if dso__decompress_kmodule_path(dso, name, newpath.as_mut_ptr(), newpath.len()) < 0 {
            errno = EIO;
            free(name as *mut c_void); return ptr::null_mut();
        }
        if newpath[0] != 0 {
            let tmp = strdup(newpath.as_ptr());
            if tmp.is_null() { unlink(newpath.as_ptr()); free(name as *mut c_void); return ptr::null_mut(); }
            free(name as *mut c_void);
            name = tmp;
            *decomp = true;
        }
    }
    name
}

unsafe fn __open_dso(dso: *mut dso, machine: *mut machine) -> c_int {
    let mut fd = -EINVAL;
    let mut decomp = false;
    mutex_lock(dso__lock(dso));
    let root = if !machine.is_null() { (*machine).root_dir } else { cstr!("") };
    let name = dso__get_filename(dso, root, &mut decomp);
    if !name.is_null() { fd = do_open(name); }
    else { if errno == 0 { errno = ENOENT; } fd = -errno; }
    if decomp { unlink(name); }
    mutex_unlock(dso__lock(dso));
    free(name as *mut c_void);
    fd
}

unsafe fn close_data_fd(dso: *mut dso) {
    if (*dso__data(dso)).fd >= 0 {
        close((*dso__data(dso)).fd);
        (*dso__data(dso)).fd = -1;
        (*dso__data(dso)).file_size = 0;
        dso__list_del(dso);
    }
}
unsafe fn close_dso(dso: *mut dso) { close_data_fd(dso); }
unsafe fn close_first_dso() {
    let dso_data = rb_entry_cache(dso__data_open.next as *mut rb_node) as *mut dso_data;
    let dso = container_of_dso_from_data(dso_data);
    close_dso(dso);
}

unsafe fn get_fd_limit() -> rlim_t {
    let mut l: rlimit = core::mem::zeroed();
    if getrlimit(RLIMIT_NOFILE, &mut l) == 0 {
        if l.rlim_cur == RLIM_INFINITY { l.rlim_cur } else { l.rlim_cur / 2 }
    } else {
        pr_err(cstr!("failed to get fd limit\n"));
        1
    }
}

#[no_mangle] pub unsafe extern "C" fn reset_fd_limit() { fd_limit = 0; }
unsafe fn may_cache_fd() -> bool {
    if fd_limit == 0 { fd_limit = get_fd_limit(); }
    if fd_limit == RLIM_INFINITY { true } else { fd_limit > dso__data_open_cnt as rlim_t }
}
unsafe fn check_data_close() { if !may_cache_fd() { close_first_dso(); } }

unsafe fn open_dso(dso: *mut dso, machine: *mut machine) -> c_int {
    let mut nsc: nscookie = core::mem::zeroed();
    if dso__binary_type(dso) != dso_binary_type::DSO_BINARY_TYPE__BUILD_ID_CACHE {
        mutex_lock(dso__lock(dso));
        nsinfo__mountns_enter(dso__nsinfo(dso), &mut nsc);
        mutex_unlock(dso__lock(dso));
    }
    let fd = __open_dso(dso, machine);
    if dso__binary_type(dso) != dso_binary_type::DSO_BINARY_TYPE__BUILD_ID_CACHE { nsinfo__mountns_exit(&mut nsc); }
    if fd >= 0 { dso__list_add(dso); check_data_close(); }
    fd
}

#[no_mangle]
pub unsafe extern "C" fn dso__data_close(dso: *mut dso) {
    mutex_lock(dso__data_open_lock_fn());
    close_dso(dso);
    mutex_unlock(dso__data_open_lock_fn());
}

unsafe fn try_to_open_dso(dso: *mut dso, machine: *mut machine) {
    let binary_type_data = [
        dso_binary_type::DSO_BINARY_TYPE__BUILD_ID_CACHE,
        dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_DSO,
        dso_binary_type::DSO_BINARY_TYPE__NOT_FOUND,
    ];
    let mut i = 0usize;
    let data = dso__data(dso);
    if (*data).fd >= 0 { return; }
    if dso__binary_type(dso) != dso_binary_type::DSO_BINARY_TYPE__NOT_FOUND {
        (*data).fd = open_dso(dso, machine);
    } else {
        loop {
            dso__set_binary_type(dso, binary_type_data[i]); i += 1;
            (*data).fd = open_dso(dso, machine);
            if (*data).fd >= 0 || dso__binary_type(dso) == dso_binary_type::DSO_BINARY_TYPE__NOT_FOUND { break; }
        }
    }
    (*data).status = if (*data).fd >= 0 { dso_data_status::DSO_DATA_STATUS_OK } else { dso_data_status::DSO_DATA_STATUS_ERROR };
}

#[no_mangle]
pub unsafe extern "C" fn dso__data_get_fd(dso: *mut dso, machine: *mut machine, fd: *mut c_int) -> bool {
    *fd = -1;
    if (*dso__data(dso)).status == dso_data_status::DSO_DATA_STATUS_ERROR { return false; }
    mutex_lock(dso__data_open_lock_fn());
    try_to_open_dso(dso, machine);
    *fd = (*dso__data(dso)).fd;
    if *fd >= 0 { return true; }
    mutex_unlock(dso__data_open_lock_fn());
    false
}

#[no_mangle] pub unsafe extern "C" fn dso__data_put_fd(_dso: *mut dso) { mutex_unlock(dso__data_open_lock_fn()); }

#[no_mangle]
pub unsafe extern "C" fn dso__data_status_seen(dso: *mut dso, by: dso_data_status_seen) -> bool {
    let flag: u32 = 1u32 << (by as u32);
    if (*dso__data(dso)).status_seen & flag != 0 { return true; }
    (*dso__data(dso)).status_seen |= flag;
    false
}

// HAVE_LIBBPF_SUPPORT: bpf_read/bpf_size depend on BPF-only external structs.

unsafe fn dso_cache__free(dso: *mut dso) {
    let root = &mut (*dso__data(dso)).cache as *mut rb_root;
    let mut next = rb_first(root);
    mutex_lock(dso__lock(dso));
    while !next.is_null() {
        let cache = rb_entry_cache(next);
        next = rb_next(&mut (*cache).rb_node);
        rb_erase(&mut (*cache).rb_node, root);
        free(cache as *mut c_void);
    }
    mutex_unlock(dso__lock(dso));
}

unsafe fn __dso_cache__find(dso: *mut dso, offset: u64) -> *mut dso_cache {
    let root = &(*dso__data(dso)).cache as *const rb_root;
    let mut p: *mut *mut rb_node = &mut (*(root as *mut rb_root)).rb_node;
    while !(*p).is_null() {
        let parent = *p;
        let cache = rb_entry_cache(parent);
        let end = (*cache).offset + DSO__DATA_CACHE_SIZE as u64;
        if offset < (*cache).offset { p = &mut (**p).rb_left; }
        else if offset >= end { p = &mut (**p).rb_right; }
        else { return cache; }
    }
    ptr::null_mut()
}

unsafe fn dso_cache__insert(dso: *mut dso, new_: *mut dso_cache) -> *mut dso_cache {
    let root = &mut (*dso__data(dso)).cache as *mut rb_root;
    let mut p: *mut *mut rb_node = &mut (*root).rb_node;
    let mut parent: *mut rb_node = ptr::null_mut();
    let offset = (*new_).offset;
    let mut cache: *mut dso_cache = ptr::null_mut();
    mutex_lock(dso__lock(dso));
    while !(*p).is_null() {
        parent = *p;
        cache = rb_entry_cache(parent);
        let end = (*cache).offset + DSO__DATA_CACHE_SIZE as u64;
        if offset < (*cache).offset { p = &mut (**p).rb_left; }
        else if offset >= end { p = &mut (**p).rb_right; }
        else { mutex_unlock(dso__lock(dso)); return cache; }
    }
    rb_link_node(&mut (*new_).rb_node, parent, p);
    rb_insert_color(&mut (*new_).rb_node, root);
    mutex_unlock(dso__lock(dso));
    ptr::null_mut()
}

unsafe fn dso_cache__memcpy(cache: *mut dso_cache, offset: u64, data: *mut u8, size: u64, out: bool) -> ssize_t {
    let cache_offset = offset - (*cache).offset;
    if cache_offset >= (*cache).size { return 0; }
    let cache_size = min_u64((*cache).size - cache_offset, size);
    let cache_data = ((*cache).data.as_ptr() as *mut u8).add(cache_offset as usize);
    if out { memcpy(data as *mut c_void, cache_data as *const c_void, cache_size as size_t); }
    else { memcpy(cache_data as *mut c_void, data as *const c_void, cache_size as size_t); }
    cache_size as ssize_t
}

unsafe fn file_read(dso: *mut dso, machine: *mut machine, offset: u64, data: *mut c_char) -> ssize_t {
    mutex_lock(dso__data_open_lock_fn());
    try_to_open_dso(dso, machine);
    let ret;
    if (*dso__data(dso)).fd < 0 {
        (*dso__data(dso)).status = dso_data_status::DSO_DATA_STATUS_ERROR;
        ret = (*dso__data(dso)).fd as ssize_t;
    } else {
        ret = pread((*dso__data(dso)).fd, data as *mut c_void, DSO__DATA_CACHE_SIZE as size_t, offset as off_t);
    }
    mutex_unlock(dso__data_open_lock_fn());
    ret
}

unsafe fn dso_cache__populate(dso: *mut dso, machine: *mut machine, offset: u64, ret: *mut ssize_t) -> *mut dso_cache {
    let cache_offset = offset & DSO__DATA_CACHE_MASK;
    let cache = zalloc(size_of::<dso_cache>() + DSO__DATA_CACHE_SIZE as usize) as *mut dso_cache;
    if cache.is_null() { *ret = -ENOMEM as ssize_t; return ptr::null_mut(); }
    if dso__binary_type(dso) == dso_binary_type::DSO_BINARY_TYPE__OOL { *ret = DSO__DATA_CACHE_SIZE; }
    else { *ret = file_read(dso, machine, cache_offset, (*cache).data.as_ptr() as *mut c_char); }
    if *ret <= 0 { free(cache as *mut c_void); return ptr::null_mut(); }
    (*cache).offset = cache_offset;
    (*cache).size = *ret as u64;
    let old = dso_cache__insert(dso, cache);
    if !old.is_null() { free(cache as *mut c_void); return old; }
    cache
}

unsafe fn dso_cache__find(dso: *mut dso, machine: *mut machine, offset: u64, ret: *mut ssize_t) -> *mut dso_cache {
    let cache = __dso_cache__find(dso, offset);
    if !cache.is_null() { cache } else { dso_cache__populate(dso, machine, offset, ret) }
}

unsafe fn dso_cache_io(dso: *mut dso, machine: *mut machine, offset: u64, data: *mut u8, size: ssize_t, out: bool) -> ssize_t {
    let mut ret: ssize_t = 0;
    let cache = dso_cache__find(dso, machine, offset, &mut ret);
    if cache.is_null() { return ret; }
    dso_cache__memcpy(cache, offset, data, size as u64, out)
}

unsafe fn cached_io(dso: *mut dso, machine: *mut machine, mut offset: u64, data: *mut u8, mut size: ssize_t, out: bool) -> ssize_t {
    let mut r: ssize_t = 0;
    let mut p = data;
    while size != 0 {
        let ret = dso_cache_io(dso, machine, offset, p, size, out);
        if ret < 0 { return ret; }
        if ret == 0 { break; }
        bug_on(ret > size);
        r += ret;
        p = p.add(ret as usize);
        offset += ret as u64;
        size -= ret;
    }
    r
}

unsafe fn file_size(dso: *mut dso, machine: *mut machine) -> c_int {
    let mut ret = 0;
    let mut st: stat = core::mem::zeroed();
    mutex_lock(dso__data_open_lock_fn());
    try_to_open_dso(dso, machine);
    if (*dso__data(dso)).fd < 0 {
        (*dso__data(dso)).status = dso_data_status::DSO_DATA_STATUS_ERROR;
        ret = (*dso__data(dso)).fd;
    } else if fstat((*dso__data(dso)).fd, &mut st) < 0 {
        ret = -errno;
        pr_err(cstr!("dso cache fstat failed: %m\n"));
        (*dso__data(dso)).status = dso_data_status::DSO_DATA_STATUS_ERROR;
    } else { (*dso__data(dso)).file_size = st.st_size; }
    mutex_unlock(dso__data_open_lock_fn());
    ret
}

#[no_mangle]
pub unsafe extern "C" fn dso__data_file_size(dso: *mut dso, machine: *mut machine) -> c_int {
    if (*dso__data(dso)).file_size != 0 { return 0; }
    if (*dso__data(dso)).status == dso_data_status::DSO_DATA_STATUS_ERROR { return -1; }
    file_size(dso, machine)
}

#[no_mangle] pub unsafe extern "C" fn dso__data_size(dso: *mut dso, machine: *mut machine) -> off_t {
    if dso__data_file_size(dso, machine) != 0 { return -1; }
    (*dso__data(dso)).file_size
}

unsafe fn data_read_write_offset(dso: *mut dso, machine: *mut machine, offset: u64, data: *mut u8, size: ssize_t, out: bool) -> ssize_t {
    if dso__data_file_size(dso, machine) != 0 { return -1; }
    if offset > (*dso__data(dso)).file_size as u64 { return -1; }
    if offset.wrapping_add(size as u64) < offset { return -1; }
    cached_io(dso, machine, offset, data, size, out)
}

#[no_mangle] pub unsafe extern "C" fn dso__data_read_offset(dso: *mut dso, machine: *mut machine, offset: u64, data: *mut u8, size: ssize_t) -> ssize_t {
    if (*dso__data(dso)).status == dso_data_status::DSO_DATA_STATUS_ERROR { return -1; }
    data_read_write_offset(dso, machine, offset, data, size, true)
}

unsafe fn dso_swap_type__from_elf_data(eidata: c_uchar) -> dso_swap_type {
    let endian: c_uint = 1;
    match eidata {
        ELFDATA2LSB => if *(ptr::addr_of!(endian) as *const c_uchar) != 1 { dso_swap_type::DSO_SWAP__YES } else { dso_swap_type::DSO_SWAP__NO },
        ELFDATA2MSB => if *(ptr::addr_of!(endian) as *const c_uchar) != 0 { dso_swap_type::DSO_SWAP__YES } else { dso_swap_type::DSO_SWAP__NO },
        _ => dso_swap_type::DSO_SWAP__UNSET,
    }
}
unsafe fn dso_swap_u16(ty: dso_swap_type, v: u16) -> u16 { if ty == dso_swap_type::DSO_SWAP__YES { v.swap_bytes() } else { v } }

#[no_mangle]
pub unsafe extern "C" fn dso__read_e_machine_endian(optional_dso: *mut dso, fd: c_int, e_flags: *mut uint32_t, is_big_endian: *mut bool) -> uint16_t {
    let mut e_machine: uint16_t = EM_NONE;
    let mut e_ident = [0 as c_uchar; EI_NIDENT];
    if !e_flags.is_null() { *e_flags = 0; }
    let _ = offset_of!(Elf32_Ehdr, e_ident);
    let _ = offset_of!(Elf64_Ehdr, e_ident);
    if pread(fd, e_ident.as_mut_ptr() as *mut c_void, e_ident.len(), 0) != e_ident.len() as ssize_t { return EM_NONE; }
    if memcmp(e_ident.as_ptr() as *const c_void, ELFMAG.as_ptr() as *const c_void, SELFMAG) != 0 { return EM_NONE; }
    if e_ident[EI_CLASS] == ELFCLASSNONE || e_ident[EI_CLASS] >= ELFCLASSNUM { return EM_NONE; }
    if e_ident[EI_VERSION] != EV_CURRENT { return EM_NONE; }
    let swap_type = dso_swap_type__from_elf_data(e_ident[EI_DATA]);
    if swap_type == dso_swap_type::DSO_SWAP__UNSET { return EM_NONE; }
    if !is_big_endian.is_null() { *is_big_endian = e_ident[EI_DATA] == ELFDATA2MSB; }
    if !optional_dso.is_null() {
        if !(dso__needs_swap(optional_dso) == dso_swap_type::DSO_SWAP__UNSET || dso__needs_swap(optional_dso) == swap_type) { assert_fail(); }
        dso__set_needs_swap(optional_dso, swap_type);
    }
    if pread(fd, &mut e_machine as *mut _ as *mut c_void, size_of::<uint16_t>(), 18) != size_of::<uint16_t>() as ssize_t { return EM_NONE; }
    e_machine = dso_swap_u16(swap_type, e_machine);
    if e_machine >= EM_NUM { return EM_NONE; }
    let need_e_flags = !e_flags.is_null() && e_machine == EM_CSKY;
    if need_e_flags {
        let off = if e_ident[EI_CLASS] == ELFCLASS32 { offset_of!(Elf32_Ehdr, e_flags) } else { offset_of!(Elf64_Ehdr, e_flags) };
        if pread(fd, e_flags as *mut c_void, size_of::<uint32_t>(), off as off_t) != size_of::<uint32_t>() as ssize_t {
            *e_flags = 0; return EM_NONE;
        }
    }
    e_machine
}

#[no_mangle]
pub unsafe extern "C" fn dso__e_machine_endian(dso: *mut dso, machine: *mut machine, e_flags: *mut uint32_t, is_big_endian: *mut bool) -> uint16_t {
    match dso__binary_type(dso) {
        dso_binary_type::DSO_BINARY_TYPE__KALLSYMS | dso_binary_type::DSO_BINARY_TYPE__GUEST_KALLSYMS |
        dso_binary_type::DSO_BINARY_TYPE__VMLINUX | dso_binary_type::DSO_BINARY_TYPE__GUEST_VMLINUX |
        dso_binary_type::DSO_BINARY_TYPE__GUEST_KMODULE | dso_binary_type::DSO_BINARY_TYPE__GUEST_KMODULE_COMP |
        dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE | dso_binary_type::DSO_BINARY_TYPE__SYSTEM_PATH_KMODULE_COMP |
        dso_binary_type::DSO_BINARY_TYPE__KCORE | dso_binary_type::DSO_BINARY_TYPE__GUEST_KCORE |
        dso_binary_type::DSO_BINARY_TYPE__BPF_PROG_INFO | dso_binary_type::DSO_BINARY_TYPE__BPF_IMAGE |
        dso_binary_type::DSO_BINARY_TYPE__OOL | dso_binary_type::DSO_BINARY_TYPE__JAVA_JIT => {
            if !is_big_endian.is_null() {
                *is_big_endian = perf_arch_is_big_endian(if !machine.is_null() && !(*machine).env.is_null() { perf_env__arch((*machine).env) } else { ptr::null() });
            }
            return perf_env__e_machine(if !machine.is_null() { (*machine).env } else { ptr::null_mut() }, e_flags);
        }
        dso_binary_type::DSO_BINARY_TYPE__NOT_FOUND => { if !e_flags.is_null() { *e_flags = 0; } return EM_NONE; }
        _ => {}
    }
    mutex_lock(dso__data_open_lock_fn());
    try_to_open_dso(dso, machine);
    let fd = (*dso__data(dso)).fd;
    let e_machine = if fd >= 0 { dso__read_e_machine_endian(dso, fd, e_flags, is_big_endian) } else { if !e_flags.is_null() { *e_flags = 0; } EM_NONE };
    mutex_unlock(dso__data_open_lock_fn());
    e_machine
}

#[no_mangle] pub unsafe extern "C" fn dso__data_read_addr(dso: *mut dso, map: *mut map, machine: *mut machine, addr: u64, data: *mut u8, size: ssize_t) -> ssize_t {
    let offset = map__map_ip(map, addr);
    dso__data_read_offset(dso, machine, offset, data, size)
}

#[no_mangle] pub unsafe extern "C" fn dso__data_write_cache_offs(dso: *mut dso, machine: *mut machine, offset: u64, data_in: *const u8, size: ssize_t) -> ssize_t {
    if (*dso__data(dso)).status == dso_data_status::DSO_DATA_STATUS_ERROR { return -1; }
    data_read_write_offset(dso, machine, offset, data_in as *mut u8, size, false)
}

#[no_mangle] pub unsafe extern "C" fn dso__data_write_cache_addr(dso: *mut dso, map: *mut map, machine: *mut machine, addr: u64, data: *const u8, size: ssize_t) -> ssize_t {
    let offset = map__map_ip(map, addr);
    dso__data_write_cache_offs(dso, machine, offset, data, size)
}

#[no_mangle] pub unsafe extern "C" fn dso__new_map(name: *const c_char) -> *mut map {
    let mut map_p = ptr::null_mut();
    let dso_p = dso__new(name);
    if !dso_p.is_null() {
        map_p = map__new2(0, dso_p);
        dso__put(dso_p);
    }
    map_p
}

#[no_mangle] pub unsafe extern "C" fn machine__findnew_kernel(machine: *mut machine, name: *const c_char, short_name: *const c_char, dso_type: c_int) -> *mut dso {
    let dso_p = machine__findnew_dso(machine, name);
    if !dso_p.is_null() {
        dso__set_short_name(dso_p, short_name, false);
        dso__set_kernel(dso_p, dso_type);
    }
    dso_p
}

unsafe fn __dso__set_long_name_id(dso: *mut dso, name: *const c_char, name_allocated: bool) {
    if dso__long_name_allocated(dso) { free(dso__long_name(dso) as *mut c_void); }
    (*rc_access(dso)).long_name = name;
    (*rc_access(dso)).long_name_len = strlen(name);
    dso__set_long_name_allocated(dso, name_allocated);
}

unsafe fn dso__set_long_name_id(dso: *mut dso, name: *const c_char, name_allocated: bool) {
    let dsos = dso__dsos(dso);
    if name.is_null() { return; }
    if !dsos.is_null() {
        down_write(&mut (*dsos).lock);
        __dso__set_long_name_id(dso, name, name_allocated);
        (*dsos).sorted = false;
        up_write(&mut (*dsos).lock);
    } else { __dso__set_long_name_id(dso, name, name_allocated); }
}

unsafe fn __dso_id__cmp(a: *const dso_id, b: *const dso_id) -> c_int {
    if (*a).mmap2_valid && (*b).mmap2_valid {
        if (*a).maj > (*b).maj { return -1; } if (*a).maj < (*b).maj { return 1; }
        if (*a).min > (*b).min { return -1; } if (*a).min < (*b).min { return 1; }
        if (*a).ino > (*b).ino { return -1; } if (*a).ino < (*b).ino { return 1; }
    }
    if (*a).mmap2_ino_generation_valid && (*b).mmap2_ino_generation_valid {
        if (*a).ino_generation > (*b).ino_generation { return -1; } if (*a).ino_generation < (*b).ino_generation { return 1; }
    }
    if build_id__is_defined(&(*a).build_id) && build_id__is_defined(&(*b).build_id) {
        if (*a).build_id.size != (*b).build_id.size { return if (*a).build_id.size < (*b).build_id.size { -1 } else { 1 }; }
        return memcmp((*a).build_id.data.as_ptr() as *const c_void, (*b).build_id.data.as_ptr() as *const c_void, (*a).build_id.size as size_t);
    }
    0
}

#[no_mangle]
pub static dso_id_empty: dso_id = dso_id { maj: 0, min: 0, ino: 0, ino_generation: 0, mmap2_valid: false, mmap2_ino_generation_valid: false, build_id: build_id { size: 0, data: [0; 32] } };

#[no_mangle] pub unsafe extern "C" fn __dso__improve_id(dso: *mut dso, id: *const dso_id) {
    let dsos = dso__dsos(dso);
    let dso_id = dso__id(dso);
    let mut changed = false;
    if (*id).mmap2_valid && !(*dso_id).mmap2_valid { (*dso_id).maj = (*id).maj; (*dso_id).min = (*id).min; (*dso_id).ino = (*id).ino; (*dso_id).mmap2_valid = true; changed = true; }
    if (*id).mmap2_ino_generation_valid && !(*dso_id).mmap2_ino_generation_valid { (*dso_id).ino_generation = (*id).ino_generation; (*dso_id).mmap2_ino_generation_valid = true; changed = true; }
    if build_id__is_defined(&(*id).build_id) && !build_id__is_defined(&(*dso_id).build_id) { (*dso_id).build_id = (*id).build_id; changed = true; }
    if changed && !dsos.is_null() { (*dsos).sorted = false; }
}

#[no_mangle] pub unsafe extern "C" fn dso_id__cmp(a: *const dso_id, b: *const dso_id) -> c_int {
    if ptr::eq(a, &dso_id_empty) || ptr::eq(b, &dso_id_empty) { return 0; }
    __dso_id__cmp(a, b)
}

#[no_mangle] pub unsafe extern "C" fn dso__cmp_id(a: *mut dso, b: *mut dso) -> c_int { __dso_id__cmp(dso__id(a), dso__id(b)) }
#[no_mangle] pub unsafe extern "C" fn dso__set_long_name(dso: *mut dso, name: *const c_char, name_allocated: bool) { dso__set_long_name_id(dso, name, name_allocated); }

unsafe fn __dso__set_short_name(dso: *mut dso, name: *const c_char, name_allocated: bool) {
    if dso__short_name_allocated(dso) { free(dso__short_name(dso) as *mut c_void); }
    (*rc_access(dso)).short_name = name;
    (*rc_access(dso)).short_name_len = strlen(name);
    dso__set_short_name_allocated(dso, name_allocated);
}

#[no_mangle] pub unsafe extern "C" fn dso__set_short_name(dso: *mut dso, name: *const c_char, name_allocated: bool) {
    let dsos = dso__dsos(dso);
    if name.is_null() { return; }
    if !dsos.is_null() { down_write(&mut (*dsos).lock); __dso__set_short_name(dso, name, name_allocated); (*dsos).sorted = false; up_write(&mut (*dsos).lock); }
    else { __dso__set_short_name(dso, name, name_allocated); }
}

#[no_mangle] pub unsafe extern "C" fn dso__name_len(dso: *const dso) -> c_int {
    if dso.is_null() { return strlen(cstr!("[unknown]")) as c_int; }
    if verbose > 0 { dso__long_name_len(dso) as c_int } else { dso__short_name_len(dso) }
}
#[no_mangle] pub unsafe extern "C" fn dso__loaded(dso: *const dso) -> bool { (*rc_access(dso as *mut dso)).loaded }
#[no_mangle] pub unsafe extern "C" fn dso__sorted_by_name(dso: *const dso) -> bool { (*rc_access(dso as *mut dso)).sorted_by_name }
#[no_mangle] pub unsafe extern "C" fn dso__set_sorted_by_name(dso: *mut dso) { (*rc_access(dso)).sorted_by_name = true; }

#[no_mangle]
pub unsafe extern "C" fn dso__new_id(name: *const c_char, id: *const dso_id) -> *mut dso {
    let dso_p = zalloc(size_of::<dso>() + strlen(name) + 1) as *mut dso;
    if dso_p.is_null() { return ptr::null_mut(); }
    strcpy((*dso_p).name.as_mut_ptr(), name);
    if !id.is_null() { (*dso_p).id = *id; }
    dso__set_long_name_id(dso_p, (*dso_p).name.as_ptr(), false);
    dso__set_short_name(dso_p, (*dso_p).name.as_ptr(), false);
    (*dso__data(dso_p)).fd = -1;
    (*dso__data(dso_p)).status = dso_data_status::DSO_DATA_STATUS_UNKNOWN;
    dso__set_symtab_type(dso_p, dso_binary_type::DSO_BINARY_TYPE__NOT_FOUND);
    dso__set_binary_type(dso_p, dso_binary_type::DSO_BINARY_TYPE__NOT_FOUND);
    mutex_init(dso__lock(dso_p));
    INIT_LIST_HEAD(&mut (*dso__data(dso_p)).open_entry);
    dso_p
}

#[no_mangle] pub unsafe extern "C" fn dso__new(name: *const c_char) -> *mut dso { dso__new_id(name, ptr::null()) }

#[no_mangle]
pub unsafe extern "C" fn dso__delete(dso: *mut dso) {
    if !dso__dsos(dso).is_null() { pr_err(cstr!("DSO %s is still in rbtree when being deleted!\n"), dso__long_name(dso)); }
    inlines__tree_delete((*rc_access(dso)).inlined_nodes);
    srcline__tree_delete((*rc_access(dso)).srclines);
    symbols__delete((*rc_access(dso)).symbols);
    (*rc_access(dso)).symbol_names_len = 0;
    zfree(&mut (*rc_access(dso)).symbol_names);
    annotated_data_type__tree_delete(dso__data_types(dso));
    global_var_type__tree_delete(dso__global_vars(dso));
    if (*rc_access(dso)).short_name_allocated { zfree(&mut (*rc_access(dso)).short_name as *mut *const c_char as *mut *mut c_char); (*rc_access(dso)).short_name_allocated = false; }
    if (*rc_access(dso)).long_name_allocated { zfree(&mut (*rc_access(dso)).long_name as *mut *const c_char as *mut *mut c_char); (*rc_access(dso)).long_name_allocated = false; }
    dso__data_close(dso);
    auxtrace_cache__free((*rc_access(dso)).auxtrace_cache);
    dso_cache__free(dso);
    dso__free_a2l(dso);
    dso__free_libdw(dso);
    dso__free_symsrc_filename(dso);
    nsinfo__zput((*rc_access(dso)).nsinfo);
    mutex_destroy(dso__lock(dso));
    free(dso as *mut c_void);
}

#[no_mangle] pub unsafe extern "C" fn dso__get(dso: *mut dso) -> *mut dso {
    if !dso.is_null() { refcount_inc(&mut (*rc_access(dso)).refcnt); }
    dso
}
#[no_mangle] pub unsafe extern "C" fn dso__put(dso: *mut dso) {
    if !dso.is_null() && refcount_dec_and_test(&mut (*rc_access(dso)).refcnt) { dso__delete(dso); }
}

#[no_mangle] pub unsafe extern "C" fn dso__swap_init(dso: *mut dso, eidata: c_uchar) -> c_int {
    let ty = dso_swap_type__from_elf_data(eidata);
    dso__set_needs_swap(dso, ty);
    if ty == dso_swap_type::DSO_SWAP__UNSET { pr_err(cstr!("unrecognized DSO data encoding %d\n"), eidata as c_int); return -EINVAL; }
    0
}
#[no_mangle] pub unsafe extern "C" fn dso__set_build_id(dso: *mut dso, bid: *const build_id) { (*dso__id(dso)).build_id = *bid; }

#[no_mangle] pub unsafe extern "C" fn dso__build_id_equal(dso: *const dso, bid: *const build_id) -> bool {
    let dso_bid = dso__bid(dso);
    if (*dso_bid).size > (*bid).size && (*dso_bid).size == BUILD_ID_SIZE {
        return memcmp((*dso_bid).data.as_ptr() as *const c_void, (*bid).data.as_ptr() as *const c_void, (*bid).size as size_t) == 0 &&
            memchr_inv((*dso_bid).data.as_ptr().add((*bid).size as usize) as *const c_void, 0, ((*dso_bid).size - (*bid).size) as size_t).is_null();
    }
    (*dso_bid).size == (*bid).size &&
        memcmp((*dso_bid).data.as_ptr() as *const c_void, (*bid).data.as_ptr() as *const c_void, (*dso_bid).size as size_t) == 0
}

#[no_mangle] pub unsafe extern "C" fn dso__read_running_kernel_build_id(dso: *mut dso, machine: *mut machine) {
    let mut path = [0 as c_char; PATH_MAX];
    let mut bid = build_id { size: 0, data: [0; 32] };
    if machine__is_default_guest(machine) { return; }
    snprintf(path.as_mut_ptr(), path.len(), cstr!("%s/sys/kernel/notes"), (*machine).root_dir);
    sysfs__read_build_id(path.as_ptr(), &mut bid);
    dso__set_build_id(dso, &bid);
}

#[no_mangle] pub unsafe extern "C" fn dso__kernel_module_get_build_id(dso: *mut dso, root_dir: *const c_char) -> c_int {
    let mut filename = [0 as c_char; PATH_MAX];
    let mut bid = build_id { size: 0, data: [0; 32] };
    let name = dso__short_name(dso).add(1);
    snprintf(filename.as_mut_ptr(), filename.len(), cstr!("%s/sys/module/%.*s/notes/.note.gnu.build-id"), root_dir, strlen(name) as c_int - 1, name);
    sysfs__read_build_id(filename.as_ptr(), &mut bid);
    dso__set_build_id(dso, &bid);
    0
}

unsafe fn dso__fprintf_buildid(dso: *mut dso, fp: *mut FILE) -> size_t {
    let mut sbuild_id = [0 as c_char; SBUILD_ID_SIZE];
    build_id__snprintf(dso__bid(dso), sbuild_id.as_mut_ptr(), sbuild_id.len());
    fprintf(fp, cstr!("%s"), sbuild_id.as_ptr())
}

#[no_mangle] pub unsafe extern "C" fn dso__fprintf(dso: *mut dso, fp: *mut FILE) -> size_t {
    let mut nd: *mut rb_node;
    let mut ret = fprintf(fp, cstr!("dso: %s ("), dso__short_name(dso));
    if dso__short_name(dso) != dso__long_name(dso) { ret += fprintf(fp, cstr!("%s, "), dso__long_name(dso)); }
    ret += fprintf(fp, cstr!("%sloaded, "), if dso__loaded(dso) { cstr!("") } else { cstr!("NOT ") });
    ret += dso__fprintf_buildid(dso, fp);
    ret += fprintf(fp, cstr!(")\n"));
    nd = rb_first_cached(dso__symbols(dso));
    while !nd.is_null() {
        let pos = rb_entry_symbol(nd);
        ret += symbol__fprintf(pos, fp);
        nd = rb_next(nd);
    }
    ret
}

#[no_mangle] pub unsafe extern "C" fn dso__type(dso: *mut dso, machine: *mut machine) -> dso_type {
    let mut fd = -1;
    let mut ty = dso_type::DSO__TYPE_UNKNOWN;
    if dso__data_get_fd(dso, machine, &mut fd) {
        ty = dso__type_fd(fd);
        dso__data_put_fd(dso);
    }
    ty
}

#[no_mangle] pub unsafe extern "C" fn dso__strerror_load(dso: *mut dso, buf: *mut c_char, buflen: size_t) -> c_int {
    let errnum = *dso__load_errno(dso);
    let dso_load__error_str = [
        cstr!("Internal tools/perf/ library error"),
        cstr!("Invalid ELF file"),
        cstr!("Can not read build id"),
        cstr!("Mismatching build id"),
        cstr!("Decompression failure"),
    ];
    bug_on(buflen == 0);
    if errnum >= 0 { errno = errnum; scnprintf(buf, buflen, cstr!("%m")); return 0; }
    if errnum < __DSO_LOAD_ERRNO__START || errnum >= __DSO_LOAD_ERRNO__END { return -1; }
    let idx = (errnum - __DSO_LOAD_ERRNO__START) as usize;
    scnprintf(buf, buflen, cstr!("%s"), dso_load__error_str[idx]);
    0
}

#[no_mangle] pub unsafe extern "C" fn perf_pid_map_tid(dso_name: *const c_char, tid: *mut c_int) -> bool { sscanf(dso_name, cstr!("/tmp/perf-%d.map"), tid) == 1 }
#[no_mangle] pub unsafe extern "C" fn is_perf_pid_map_name(dso_name: *const c_char) -> bool { let mut tid = 0; perf_pid_map_tid(dso_name, &mut tid) }

#[repr(C)] struct find_file_offset_data { ip: u64, offset: u64 }

extern "C" fn find_file_offset(start: u64, len: u64, pgoff: u64, arg: *mut c_void) -> c_int {
    unsafe {
        let data = arg as *mut find_file_offset_data;
        if start <= (*data).ip && (*data).ip < start + len {
            (*data).offset = pgoff + (*data).ip - start;
            return 1;
        }
        0
    }
}

unsafe fn __dso__read_symbol(dso: *mut dso, symfs_filename: *const c_char, start: u64, len: size_t, out_buf: *mut *mut u8, out_buf_len: *mut u64, is_64bit: *mut bool) -> *const u8 {
    let mut nsc: nscookie = core::mem::zeroed();
    let mut data = find_file_offset_data { ip: start, offset: 0 };
    let mut code_buf: *mut u8;
    nsinfo__mountns_enter(dso__nsinfo(dso), &mut nsc);
    let fd = open(symfs_filename, O_RDONLY | O_CLOEXEC);
    let saved_errno = errno;
    nsinfo__mountns_exit(&mut nsc);
    if fd < 0 { errno = saved_errno; return ptr::null(); }
    if file__read_maps(fd, true, find_file_offset, &mut data as *mut _ as *mut c_void, is_64bit) <= 0 {
        close(fd); errno = ENOENT; return ptr::null();
    }
    code_buf = malloc(len) as *mut u8;
    if code_buf.is_null() { close(fd); errno = ENOMEM; return ptr::null(); }
    let count = pread(fd, code_buf as *mut c_void, len, data.offset as off_t);
    let saved_errno2 = errno;
    close(fd);
    if count as u64 != len as u64 { free(code_buf as *mut c_void); errno = saved_errno2; return ptr::null(); }
    *out_buf = code_buf;
    *out_buf_len = len as u64;
    code_buf as *const u8
}

#[no_mangle]
pub unsafe extern "C" fn dso__read_symbol(dso: *mut dso, symfs_filename: *const c_char, map: *const map, sym: *const symbol, out_buf: *mut *mut u8, out_buf_len: *mut u64, is_64bit: *mut bool) -> *const u8 {
    let start = map__rip_2objdump(map, (*sym).start);
    let end = map__rip_2objdump(map, (*sym).end);
    let len = (end - start) as size_t;
    *out_buf = ptr::null_mut();
    *out_buf_len = 0;
    *is_64bit = false;
    if dso__binary_type(dso) == dso_binary_type::DSO_BINARY_TYPE__BPF_IMAGE {
        errno = EOPNOTSUPP;
        return ptr::null();
    }
    if dso__binary_type(dso) == dso_binary_type::DSO_BINARY_TYPE__BPF_PROG_INFO {
        // HAVE_LIBBPF_SUPPORT branch returns JITed BPF instructions; without it C returns EOPNOTSUPP.
        pr_debug(cstr!("No BPF program disassembly support\n"));
        errno = EOPNOTSUPP;
        return ptr::null();
    }
    __dso__read_symbol(dso, symfs_filename, start, len, out_buf, out_buf_len, is_64bit)
}

#[no_mangle]
pub unsafe extern "C" fn dso__debuginfo(dso: *mut dso) -> *mut debuginfo {
    let mut decomp = false;
    let mut dinfo = ptr::null_mut();
    mutex_lock(dso__lock(dso));
    let name = dso__get_filename(dso, cstr!(""), &mut decomp);
    if !name.is_null() { dinfo = debuginfo__new(name); }
    if decomp { unlink(name); }
    mutex_unlock(dso__lock(dso));
    free(name as *mut c_void);
    dinfo
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
