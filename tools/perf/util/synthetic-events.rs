// SPDX-License-Identifier: GPL-2.0-only

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{offset_of, size_of};
use core::ptr;

type bool_ = bool;
type size_t = usize;
type ssize_t = isize;
type pid_t = i32;
type u8 = u8;
type u16 = u16;
type u32 = u32;
type u64 = u64;
type __u32 = u32;
type __u64 = u64;

const DEFAULT_PROC_MAP_PARSE_TIMEOUT: c_uint = 500;

#[no_mangle]
pub static mut proc_map_timeout: c_uint = DEFAULT_PROC_MAP_PARSE_TIMEOUT;

const BUFSIZ: usize = 8192;
const PATH_MAX: usize = 4096;
const UINT_MAX: c_uint = c_uint::MAX;
const UINT16_MAX: usize = u16::MAX as usize;

const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2000000;
const O_DIRECTORY: c_int = 0o200000;
const AT_FDCWD: c_int = -100;
const _SC_NPROCESSORS_ONLN: c_int = 84;

const ENOMEM: c_int = 12;
const ENOENT: c_int = 2;
const EPERM: c_int = 1;

const PERF_RECORD_COMM: u32 = 3;
const PERF_RECORD_MMAP: u32 = 1;
const PERF_RECORD_FORK: u32 = 7;
const PERF_RECORD_SAMPLE: u32 = 9;
const PERF_RECORD_MMAP2: u32 = 10;
const PERF_RECORD_NAMESPACES: u32 = 16;
const PERF_RECORD_CGROUP: u32 = 19;
const PERF_RECORD_THREAD_MAP: u32 = 73;
const PERF_RECORD_CPU_MAP: u32 = 74;
const PERF_RECORD_STAT_CONFIG: u32 = 76;
const PERF_RECORD_STAT: u32 = 77;
const PERF_RECORD_STAT_ROUND: u32 = 78;
const PERF_RECORD_ID_INDEX: u32 = 79;

const PERF_RECORD_MISC_CPUMODE_MASK: u16 = 7;
const PERF_RECORD_MISC_USER: u16 = 1;
const PERF_RECORD_MISC_KERNEL: u16 = 2;
const PERF_RECORD_MISC_GUEST_USER: u16 = 4;
const PERF_RECORD_MISC_GUEST_KERNEL: u16 = 5;
const PERF_RECORD_MISC_MMAP_DATA: u16 = 0x2000;
const PERF_RECORD_MISC_MMAP_BUILD_ID: u16 = 0x4000;
const PERF_RECORD_MISC_PROC_MAP_PARSE_TIMEOUT: u16 = 0x8000;
const PERF_RECORD_MISC_FORK_EXEC: u16 = 0x2000;

const PROT_READ: u32 = 0x1;
const PROT_WRITE: u32 = 0x2;
const PROT_EXEC: u32 = 0x4;
const MAP_SHARED: u32 = 0x01;
const MAP_PRIVATE: u32 = 0x02;
const MAP_HUGETLB: u32 = 0x40000;

const PERF_SAMPLE_IP: u64 = 1 << 0;
const PERF_SAMPLE_TID: u64 = 1 << 1;
const PERF_SAMPLE_TIME: u64 = 1 << 2;
const PERF_SAMPLE_ADDR: u64 = 1 << 3;
const PERF_SAMPLE_READ: u64 = 1 << 4;
const PERF_SAMPLE_CALLCHAIN: u64 = 1 << 5;
const PERF_SAMPLE_ID: u64 = 1 << 6;
const PERF_SAMPLE_CPU: u64 = 1 << 7;
const PERF_SAMPLE_PERIOD: u64 = 1 << 8;
const PERF_SAMPLE_STREAM_ID: u64 = 1 << 9;
const PERF_SAMPLE_RAW: u64 = 1 << 10;
const PERF_SAMPLE_BRANCH_STACK: u64 = 1 << 11;
const PERF_SAMPLE_REGS_USER: u64 = 1 << 12;
const PERF_SAMPLE_STACK_USER: u64 = 1 << 13;
const PERF_SAMPLE_WEIGHT: u64 = 1 << 14;
const PERF_SAMPLE_DATA_SRC: u64 = 1 << 15;
const PERF_SAMPLE_IDENTIFIER: u64 = 1 << 16;
const PERF_SAMPLE_TRANSACTION: u64 = 1 << 17;
const PERF_SAMPLE_REGS_INTR: u64 = 1 << 18;
const PERF_SAMPLE_PHYS_ADDR: u64 = 1 << 19;
const PERF_SAMPLE_CGROUP: u64 = 1 << 21;
const PERF_SAMPLE_DATA_PAGE_SIZE: u64 = 1 << 22;
const PERF_SAMPLE_CODE_PAGE_SIZE: u64 = 1 << 23;
const PERF_SAMPLE_WEIGHT_STRUCT: u64 = 1 << 24;
const PERF_SAMPLE_AUX: u64 = 1 << 25;
const PERF_SAMPLE_WEIGHT_TYPE: u64 = PERF_SAMPLE_WEIGHT | PERF_SAMPLE_WEIGHT_STRUCT;
const PERF_SAMPLE_BRANCH_HW_INDEX: u64 = 1 << 17;

const PERF_FORMAT_TOTAL_TIME_ENABLED: u64 = 1 << 0;
const PERF_FORMAT_TOTAL_TIME_RUNNING: u64 = 1 << 1;
const PERF_FORMAT_ID: u64 = 1 << 2;
const PERF_FORMAT_GROUP: u64 = 1 << 3;
const PERF_FORMAT_LOST: u64 = 1 << 4;

const PERF_CPU_MAP__CPUS: c_int = 0;
const PERF_CPU_MAP__MASK: c_int = 1;
const PERF_CPU_MAP__RANGE_CPUS: c_int = 2;
const PERF_STAT_CONFIG_TERM__AGGR_MODE: u64 = 0;
const PERF_STAT_CONFIG_TERM__INTERVAL: u64 = 1;
const PERF_STAT_CONFIG_TERM__SCALE: u64 = 2;
const PERF_STAT_CONFIG_TERM__AGGR_LEVEL: u64 = 3;
const PERF_STAT_CONFIG_TERM__MAX: usize = 4;
const NR_NAMESPACES: usize = 7;

const fn PERF_ALIGN(x: usize, a: usize) -> usize {
    (x + a - 1) & !(a - 1)
}

const fn BITS_TO_U32(nr: c_int) -> usize {
    ((nr as usize) + 31) / 32
}

#[repr(C)]
pub struct perf_tool {
    pub namespace_events: bool,
    pub cgroup_events: bool,
}

#[repr(C)]
pub struct perf_event_header {
    pub type_: u32,
    pub misc: u16,
    pub size: u16,
}

#[repr(C)]
pub struct perf_record_comm {
    pub header: perf_event_header,
    pub pid: u32,
    pub tid: u32,
    pub comm: [c_char; 16],
}

#[repr(C)]
pub struct perf_ns_link_info {
    pub dev: u64,
    pub ino: u64,
}

#[repr(C)]
pub struct perf_record_namespaces {
    pub header: perf_event_header,
    pub pid: u32,
    pub tid: u32,
    pub nr_namespaces: u64,
    pub link_info: [perf_ns_link_info; NR_NAMESPACES],
}

#[repr(C)]
pub struct perf_record_fork {
    pub header: perf_event_header,
    pub pid: u32,
    pub ppid: u32,
    pub tid: u32,
    pub ptid: u32,
    pub time: u64,
}

#[repr(C)]
pub struct perf_record_mmap {
    pub header: perf_event_header,
    pub pid: u32,
    pub tid: u32,
    pub start: u64,
    pub len: u64,
    pub pgoff: u64,
    pub filename: [c_char; PATH_MAX],
}

#[repr(C)]
pub struct perf_record_mmap2 {
    pub header: perf_event_header,
    pub pid: u32,
    pub tid: u32,
    pub start: u64,
    pub len: u64,
    pub pgoff: u64,
    pub maj: u32,
    pub min: u32,
    pub ino: u64,
    pub ino_generation: u64,
    pub prot: u32,
    pub flags: u32,
    pub build_id: [u8; 20],
    pub build_id_size: u8,
    pub __reserved_1: u8,
    pub __reserved_2: u16,
    pub filename: [c_char; PATH_MAX],
}

#[repr(C)]
pub struct perf_record_cgroup {
    pub header: perf_event_header,
    pub id: u64,
    pub path: [c_char; PATH_MAX],
}

#[repr(C)]
pub struct perf_record_thread_map_entry {
    pub pid: i64,
    pub comm: [c_char; 16],
}

#[repr(C)]
pub struct perf_record_thread_map {
    pub header: perf_event_header,
    pub nr: u64,
    pub entries: [perf_record_thread_map_entry; 0],
}

#[repr(C)]
pub struct cpu_map_entries {
    pub nr: u16,
    pub cpu: [u16; 0],
}

#[repr(C)]
pub struct perf_record_mask_cpu_map32 {
    pub nr: u16,
    pub long_size: u16,
    pub mask: [u32; 0],
}

#[repr(C)]
pub struct perf_record_range_cpu_map {
    pub any_cpu: u16,
    pub start_cpu: u16,
    pub end_cpu: u16,
}

#[repr(C)]
pub union perf_record_cpu_map_data_union {
    pub cpus_data: core::mem::ManuallyDrop<cpu_map_entries>,
    pub mask32_data: core::mem::ManuallyDrop<perf_record_mask_cpu_map32>,
    pub range_cpu_data: core::mem::ManuallyDrop<perf_record_range_cpu_map>,
}

#[repr(C)]
pub struct perf_record_cpu_map_data {
    pub type_: u16,
    pub data: perf_record_cpu_map_data_union,
}

#[repr(C)]
pub struct perf_record_cpu_map {
    pub header: perf_event_header,
    pub data: perf_record_cpu_map_data,
}

#[repr(C)]
pub struct perf_record_stat_config_term {
    pub tag: u64,
    pub val: u64,
}

#[repr(C)]
pub struct perf_record_stat_config {
    pub header: perf_event_header,
    pub nr: u64,
    pub data: [perf_record_stat_config_term; 0],
}

#[repr(C)]
pub struct perf_record_stat {
    pub header: perf_event_header,
    pub id: u64,
    pub cpu: u32,
    pub thread: u32,
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}

#[repr(C)]
pub struct perf_record_stat_round {
    pub header: perf_event_header,
    pub time: u64,
    pub type_: u64,
}

#[repr(C)]
pub struct perf_record_sample {
    pub header: perf_event_header,
    pub array: [u64; 0],
}

#[repr(C)]
pub struct id_index_entry {
    pub id: u64,
    pub idx: u64,
    pub cpu: i32,
    pub tid: i32,
}

#[repr(C)]
pub struct id_index_entry_2 {
    pub machine_pid: i32,
    pub vcpu: i32,
}

#[repr(C)]
pub struct perf_record_id_index {
    pub header: perf_event_header,
    pub nr: u64,
    pub entries: [id_index_entry; 0],
}

#[repr(C)]
pub union perf_event {
    pub header: core::mem::ManuallyDrop<perf_event_header>,
    pub comm: core::mem::ManuallyDrop<perf_record_comm>,
    pub mmap: core::mem::ManuallyDrop<perf_record_mmap>,
    pub mmap2: core::mem::ManuallyDrop<perf_record_mmap2>,
    pub fork: core::mem::ManuallyDrop<perf_record_fork>,
    pub namespaces: core::mem::ManuallyDrop<perf_record_namespaces>,
    pub cgroup: core::mem::ManuallyDrop<perf_record_cgroup>,
    pub thread_map: core::mem::ManuallyDrop<perf_record_thread_map>,
    pub sample: core::mem::ManuallyDrop<perf_record_sample>,
    pub id_index: core::mem::ManuallyDrop<perf_record_id_index>,
}

#[repr(C)]
pub struct perf_sample {
    pub pid: u32,
    pub tid: u32,
    pub time: u64,
    pub stream_id: u64,
    pub cpu: u32,
    pub period: u64,
    pub cpumode: u16,
    pub id: u64,
    pub ip: u64,
    pub addr: u64,
    pub read: perf_sample_read,
    pub callchain: *mut perf_callchain_entry,
    pub raw_size: u32,
    pub raw_data: *const c_void,
    pub branch_stack: *mut branch_stack,
    pub no_hw_idx: bool,
    pub user_regs: *mut regs_dump,
    pub user_stack: stack_dump,
    pub weight: u64,
    pub ins_lat: u16,
    pub weight3: u16,
    pub data_src: u64,
    pub transaction: u64,
    pub intr_regs: *mut regs_dump,
    pub phys_addr: u64,
    pub cgroup: u64,
    pub data_page_size: u64,
    pub code_page_size: u64,
    pub aux_sample: aux_sample,
}

#[repr(C)]
pub union perf_sample_read_union {
    pub one: core::mem::ManuallyDrop<sample_read_value>,
    pub group: core::mem::ManuallyDrop<sample_read_group>,
}

#[repr(C)]
pub struct perf_sample_read {
    pub time_enabled: u64,
    pub time_running: u64,
    pub u: perf_sample_read_union,
}

#[repr(C)]
pub struct sample_read_value {
    pub value: u64,
    pub id: u64,
    pub lost: u64,
}

#[repr(C)]
pub struct sample_read_group {
    pub nr: u64,
    pub values: *mut sample_read_value,
}

#[repr(C)]
pub struct perf_callchain_entry {
    pub nr: u64,
    pub ips: [u64; 0],
}

#[repr(C)]
pub struct branch_entry {
    pub from: u64,
    pub to: u64,
    pub flags: u64,
}

#[repr(C)]
pub struct branch_stack {
    pub nr: u64,
    pub hw_idx: u64,
    pub entries: [branch_entry; 0],
}

#[repr(C)]
pub struct regs_dump {
    pub abi: u64,
    pub mask: u64,
    pub regs: *const u64,
}

#[repr(C)]
pub struct stack_dump {
    pub size: u64,
    pub data: *const c_void,
}

#[repr(C)]
pub struct aux_sample {
    pub size: u64,
    pub data: *const c_void,
}

#[repr(C)]
pub struct machine {
    pub pid: pid_t,
    pub id_hdr_size: size_t,
    pub root_dir: *const c_char,
    pub mmap_name: *const c_char,
    pub dsos: dsos,
}

#[repr(C)]
pub struct dsos {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso_id {
    pub maj: u32,
    pub min: u32,
    pub ino: u64,
    pub ino_generation: u64,
    pub mmap2_valid: bool,
    pub mmap2_ino_generation_valid: bool,
    pub build_id: build_id,
}

#[repr(C)]
pub struct build_id {
    pub size: size_t,
    pub data: [u8; 20],
}

#[repr(C)]
pub struct nsinfo {
    _private: [u8; 0],
}

#[repr(C)]
pub struct nscookie {
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
pub struct symbol {
    pub name: *const c_char,
    pub addr: u64,
}

#[repr(C)]
pub struct kmap {
    pub ref_reloc_sym: *mut symbol,
}

#[repr(C)]
pub struct dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct io {
    pub fd: c_int,
    pub eof: bool,
    pub buf: *mut c_char,
    pub data: *mut c_char,
}

#[repr(C)]
pub struct io_dir {
    pub dirfd: c_int,
}

#[repr(C)]
pub struct io_dirent64 {
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
pub struct file_handle {
    pub handle_bytes: c_uint,
    pub handle_type: c_int,
    pub f_handle: [u8; 0],
}

#[repr(C)]
pub struct cgroup_handle {
    pub fh: file_handle,
    pub cgroup_id: u64,
}

#[repr(C)]
pub struct perf_thread_map {
    pub nr: c_int,
}

#[repr(C)]
pub struct perf_cpu {
    pub cpu: c_int,
}

#[repr(C)]
pub struct perf_cpu_map {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_stat_config {
    pub aggr_mode: u64,
    pub interval: u64,
    pub scale: u64,
    pub aggr_level: u64,
}

#[repr(C)]
pub struct perf_counts_values {
    pub val: u64,
    pub ena: u64,
    pub run: u64,
}

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel_core {
    pub ids: u32,
    pub id: *mut u64,
}

#[repr(C)]
pub struct evsel {
    pub core: evsel_core,
}

#[repr(C)]
pub struct perf_sample_id {
    pub idx: u64,
    pub cpu: perf_cpu,
    pub tid: i32,
    pub machine_pid: i32,
    pub vcpu: perf_cpu,
}

#[repr(C)]
pub struct target {
    _private: [u8; 0],
}

#[repr(C)]
pub struct symbol_conf_t {
    pub no_buildid_mmap2: bool,
}

pub type perf_event__handler_t = Option<
    unsafe extern "C" fn(
        *const perf_tool,
        *mut perf_event,
        *mut perf_sample,
        *mut machine,
    ) -> c_int,
>;

unsafe extern "C" {
    static mut symbol_conf: symbol_conf_t;
    static dso_id_empty: dso_id;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncpy(dst: *mut c_char, src: *const c_char, n: size_t) -> *mut c_char;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn atoi(nptr: *const c_char) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn zalloc(size: size_t) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);
    fn stat64(path: *const c_char, buf: *mut stat64) -> c_int;
    fn strlcpy(dst: *mut c_char, src: *const c_char, siz: size_t) -> size_t;
    fn isdigit(c: c_int) -> c_int;
    fn scandir(
        dirp: *const c_char,
        namelist: *mut *mut *mut dirent,
        filter: Option<unsafe extern "C" fn(*const dirent) -> c_int>,
        compar: Option<unsafe extern "C" fn(*const *const dirent, *const *const dirent) -> c_int>,
    ) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn pthread_create(
        thread: *mut pthread_t,
        attr: *const c_void,
        start_routine: unsafe extern "C" fn(*mut c_void) -> *mut c_void,
        arg: *mut c_void,
    ) -> c_int;
    fn pthread_join(thread: pthread_t, retval: *mut *mut c_void) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn name_to_handle_at(
        dirfd: c_int,
        pathname: *const c_char,
        handle: *mut file_handle,
        mount_id: *mut c_int,
        flags: c_int,
    ) -> c_int;

    fn pr_debug(fmt: *const c_char, ...);
    fn pr_debug2(fmt: *const c_char, ...);
    fn pr_warning(fmt: *const c_char, ...);
    fn pr_err(fmt: *const c_char, ...);
    fn WARN_ONCE(condition: bool, fmt: *const c_char, ...);
    fn BUG_ON(condition: bool);
    fn assert(condition: bool);

    fn skip_spaces(str_: *const c_char) -> *mut c_char;
    fn machine__is_host(machine: *mut machine) -> bool;
    fn machine__is_default_guest(machine: *mut machine) -> bool;
    fn perf_ns__name(idx: u32) -> *const c_char;
    fn hugetlbfs__mountpoint() -> *const c_char;
    fn io__init(io: *mut io, fd: c_int, buf: *mut c_char, size: size_t);
    fn io__get_char(io: *mut io) -> c_int;
    fn io__get_hex(io: *mut io, result: *mut u64) -> c_int;
    fn io__get_dec(io: *mut io, result: *mut u64) -> c_int;
    fn rdclock() -> u64;
    fn sysfs__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int;
    fn build_id__init(dst: *mut build_id, data: *const u8, size: u8);
    fn dsos__findnew_id(dsos: *mut dsos, name: *const c_char, id: *const dso_id) -> *mut dso;
    fn dso__has_build_id(dso: *mut dso) -> bool;
    fn dso__bid(dso: *mut dso) -> *mut build_id;
    fn dso__set_build_id(dso: *mut dso, bid: *const build_id);
    fn dso__put(dso: *mut dso);
    fn nsinfo__new(pid: pid_t) -> *mut nsinfo;
    fn nsinfo__mountns_enter(nsi: *mut nsinfo, nc: *mut nscookie);
    fn nsinfo__mountns_exit(nc: *mut nscookie);
    fn nsinfo__put(nsi: *mut nsinfo);
    fn filename__read_build_id(filename: *const c_char, bid: *mut build_id) -> c_int;
    fn cgroupfs_find_mountpoint(buf: *mut c_char, maxlen: size_t, subsystem: *const c_char) -> c_int;
    fn __map__is_kmodule(map: *mut map) -> bool;
    fn map__dso(map: *mut map) -> *mut dso;
    fn dso__long_name(dso: *mut dso) -> *const c_char;
    fn map__start(map: *mut map) -> u64;
    fn map__end(map: *mut map) -> u64;
    fn map__size(map: *mut map) -> u64;
    fn machine__kernel_maps(machine: *mut machine) -> *mut maps;
    fn maps__for_each_map(
        maps: *mut maps,
        cb: Option<unsafe extern "C" fn(*mut map, *mut c_void) -> c_int>,
        data: *mut c_void,
    ) -> c_int;
    fn machine__kernel_map(machine: *mut machine) -> *mut map;
    fn map__kmap(map: *mut map) -> *mut kmap;
    fn io_dir__init(iod: *mut io_dir, fd: c_int);
    fn io_dir__readdir(iod: *mut io_dir) -> *mut io_dirent64;
    fn perf_thread_map__pid(threads: *mut perf_thread_map, thread: c_int) -> c_int;
    fn perf_thread_map__comm(threads: *mut perf_thread_map, thread: c_int) -> *mut c_char;
    fn perf_cpu_map__nr(map: *const perf_cpu_map) -> c_int;
    fn perf_cpu_map__cpu(map: *const perf_cpu_map, idx: c_int) -> perf_cpu;
    fn perf_cpu_map__max(map: *const perf_cpu_map) -> perf_cpu;
    fn sample_read_value_size(read_format: u64) -> size_t;
    fn hweight64(w: u64) -> c_uint;
    fn perf_sample__branch_entries(sample: *mut perf_sample) -> *mut branch_entry;
    fn evlist__id2sid(evlist: *mut evlist, id: u64) -> *mut perf_sample_id;
    fn nsinfo__is_in_root_namespace() -> bool;
    fn target__has_task(target: *mut target) -> bool;
    fn target__has_cpu(target: *mut target) -> bool;
    fn perf_event__process(
        tool: *const perf_tool,
        event: *mut perf_event,
        sample: *mut perf_sample,
        machine: *mut machine,
    ) -> c_int;
}

type pthread_t = c_ulong;

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct stat64 {
    pub st_dev: u64,
    pub st_ino: u64,
}

#[repr(C)]
union u64_swap {
    val64: u64,
    val32: [u32; 2],
}

unsafe fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn event_header(event: *mut perf_event) -> *mut perf_event_header {
    &mut (*event).header as *mut _ as *mut perf_event_header
}

#[no_mangle]
pub unsafe extern "C" fn perf_tool__process_synth_event(
    tool: *const perf_tool,
    event: *mut perf_event,
    machine: *mut machine,
    process: perf_event__handler_t,
) -> c_int {
    let mut synth_sample = perf_sample {
        pid: -1i32 as u32,
        tid: -1i32 as u32,
        time: -1i64 as u64,
        stream_id: -1i64 as u64,
        cpu: -1i32 as u32,
        period: 1,
        cpumode: (*event_header(event)).misc & PERF_RECORD_MISC_CPUMODE_MASK,
        id: 0,
        ip: 0,
        addr: 0,
        read: core::mem::zeroed(),
        callchain: ptr::null_mut(),
        raw_size: 0,
        raw_data: ptr::null(),
        branch_stack: ptr::null_mut(),
        no_hw_idx: false,
        user_regs: ptr::null_mut(),
        user_stack: stack_dump { size: 0, data: ptr::null() },
        weight: 0,
        ins_lat: 0,
        weight3: 0,
        data_src: 0,
        transaction: 0,
        intr_regs: ptr::null_mut(),
        phys_addr: 0,
        cgroup: 0,
        data_page_size: 0,
        code_page_size: 0,
        aux_sample: aux_sample { size: 0, data: ptr::null() },
    };

    process.unwrap()(tool, event, &mut synth_sample, machine)
}

/*
 * Assumes that the first 4095 bytes of /proc/pid/stat contains
 * the comm, tgid and ppid.
 */
unsafe fn perf_event__get_comm_ids(
    pid: pid_t,
    tid: pid_t,
    comm: *mut c_char,
    len: size_t,
    tgid: *mut pid_t,
    ppid: *mut pid_t,
    kernel: *mut bool,
) -> c_int {
    let mut bf = [0 as c_char; 4096];
    let mut size: size_t = 0;
    let fd: c_int;
    let n: ssize_t;
    let mut name: *mut c_char;
    let mut tgids: *mut c_char;
    let mut ppids: *mut c_char;
    let vmpeak: *mut c_char;
    let threads: *mut c_char;

    *tgid = -1;
    *ppid = -1;

    if pid != 0 {
        snprintf(
            bf.as_mut_ptr(),
            bf.len(),
            cstr(b"/proc/%d/task/%d/status\0"),
            pid,
            tid,
        );
    } else {
        snprintf(bf.as_mut_ptr(), bf.len(), cstr(b"/proc/%d/status\0"), tid);
    }

    fd = open(bf.as_ptr(), O_RDONLY);
    if fd < 0 {
        pr_debug(cstr(b"couldn't open %s\n\0"), bf.as_ptr());
        return -1;
    }

    n = read(fd, bf.as_mut_ptr() as *mut c_void, bf.len() - 1);
    close(fd);
    if n <= 0 {
        pr_warning(cstr(b"Couldn't get COMM, tigd and ppid for pid %d\n\0"), tid);
        return -1;
    }
    bf[n as usize] = 0;

    name = strstr(bf.as_ptr(), cstr(b"Name:\0"));
    tgids = strstr(if !name.is_null() { name } else { bf.as_mut_ptr() }, cstr(b"Tgid:\0"));
    ppids = strstr(if !tgids.is_null() { tgids } else { bf.as_mut_ptr() }, cstr(b"PPid:\0"));
    vmpeak = strstr(if !ppids.is_null() { ppids } else { bf.as_mut_ptr() }, cstr(b"VmPeak:\0"));

    if !vmpeak.is_null() {
        threads = ptr::null_mut();
    } else {
        threads = strstr(if !ppids.is_null() { ppids } else { bf.as_mut_ptr() }, cstr(b"Threads:\0"));
    }

    if !name.is_null() {
        let nl: *mut c_char;

        name = skip_spaces(name.add(5)); /* strlen("Name:") */
        nl = strchr(name, '\n' as c_int);
        if !nl.is_null() {
            *nl = 0;
        }

        size = strlen(name);
        if size >= len {
            size = len - 1;
        }
        memcpy(comm as *mut c_void, name as *const c_void, size);
        *comm.add(size) = 0;
    } else {
        pr_debug(cstr(b"Name: string not found for pid %d\n\0"), tid);
    }

    if !tgids.is_null() {
        tgids = tgids.add(5); /* strlen("Tgid:") */
        *tgid = atoi(tgids);
    } else {
        pr_debug(cstr(b"Tgid: string not found for pid %d\n\0"), tid);
    }

    if !ppids.is_null() {
        ppids = ppids.add(5); /* strlen("PPid:") */
        *ppid = atoi(ppids);
    } else {
        pr_debug(cstr(b"PPid: string not found for pid %d\n\0"), tid);
    }

    if vmpeak.is_null() && !threads.is_null() {
        *kernel = true;
    } else {
        *kernel = false;
    }

    0
}

unsafe fn perf_event__prepare_comm(
    event: *mut perf_event,
    pid: pid_t,
    tid: pid_t,
    machine: *mut machine,
    tgid: *mut pid_t,
    ppid: *mut pid_t,
    kernel: *mut bool,
) -> c_int {
    let mut size: size_t;
    *ppid = -1;
    memset(&mut (*event).comm as *mut _ as *mut c_void, 0, size_of::<perf_record_comm>());

    if machine__is_host(machine) {
        if perf_event__get_comm_ids(
            pid,
            tid,
            (*event).comm.comm.as_mut_ptr(),
            (*event).comm.comm.len(),
            tgid,
            ppid,
            kernel,
        ) != 0
        {
            return -1;
        }
    } else {
        *tgid = (*machine).pid;
    }

    if *tgid < 0 {
        return -1;
    }

    (*event).comm.pid = *tgid as u32;
    (*event).comm.header.type_ = PERF_RECORD_COMM;

    size = strlen((*event).comm.comm.as_ptr()) + 1;
    size = PERF_ALIGN(size, size_of::<u64>());
    memset(
        (event as *mut c_char).add(offset_of!(perf_record_comm, comm) + size) as *mut c_void,
        0,
        (*machine).id_hdr_size,
    );
    (*event).comm.header.size = (size_of::<perf_record_comm>()
        - ((*event).comm.comm.len() - size)
        + (*machine).id_hdr_size) as u16;
    (*event).comm.tid = tid as u32;

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_comm(
    tool: *const perf_tool,
    event: *mut perf_event,
    pid: pid_t,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> pid_t {
    let mut tgid: pid_t = 0;
    let mut ppid: pid_t = 0;
    let mut kernel_thread = false;

    if perf_event__prepare_comm(event, 0, pid, machine, &mut tgid, &mut ppid, &mut kernel_thread) != 0 {
        return -1;
    }
    if perf_tool__process_synth_event(tool, event, machine, process) != 0 {
        return -1;
    }
    tgid
}

unsafe fn perf_event__get_ns_link_info(
    pid: pid_t,
    ns: *const c_char,
    ns_link_info: *mut perf_ns_link_info,
) {
    let mut st: stat64 = core::mem::zeroed();
    let mut proc_ns = [0 as c_char; 128];

    sprintf(proc_ns.as_mut_ptr(), cstr(b"/proc/%u/ns/%s\0"), pid as c_uint, ns);
    if stat64(proc_ns.as_ptr(), &mut st) == 0 {
        (*ns_link_info).dev = st.st_dev;
        (*ns_link_info).ino = st.st_ino;
    }
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_namespaces(
    tool: *const perf_tool,
    event: *mut perf_event,
    pid: pid_t,
    tgid: pid_t,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let mut idx: u32;
    let ns_link_info: *mut perf_ns_link_info;

    if tool.is_null() || !(*tool).namespace_events {
        return 0;
    }

    memset(
        &mut (*event).namespaces as *mut _ as *mut c_void,
        0,
        size_of::<perf_record_namespaces>() + (NR_NAMESPACES * size_of::<perf_ns_link_info>()) + (*machine).id_hdr_size,
    );

    (*event).namespaces.pid = tgid as u32;
    (*event).namespaces.tid = pid as u32;
    (*event).namespaces.nr_namespaces = NR_NAMESPACES as u64;
    ns_link_info = (*event).namespaces.link_info.as_mut_ptr();

    idx = 0;
    while idx < (*event).namespaces.nr_namespaces as u32 {
        perf_event__get_ns_link_info(pid, perf_ns__name(idx), ns_link_info.add(idx as usize));
        idx += 1;
    }

    (*event).namespaces.header.type_ = PERF_RECORD_NAMESPACES;
    (*event).namespaces.header.size = (size_of::<perf_record_namespaces>()
        + (NR_NAMESPACES * size_of::<perf_ns_link_info>())
        + (*machine).id_hdr_size) as u16;

    if perf_tool__process_synth_event(tool, event, machine, process) != 0 {
        return -1;
    }
    0
}

unsafe fn perf_event__synthesize_fork(
    tool: *const perf_tool,
    event: *mut perf_event,
    pid: pid_t,
    tgid: pid_t,
    ppid: pid_t,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    memset(
        &mut (*event).fork as *mut _ as *mut c_void,
        0,
        size_of::<perf_record_fork>() + (*machine).id_hdr_size,
    );

    /*
     * for main thread set parent to ppid from status file. For other
     * threads set parent pid to main thread. ie., assume main thread
     * spawns all threads in a process
     */
    if tgid == pid {
        (*event).fork.ppid = ppid as u32;
        (*event).fork.ptid = ppid as u32;
    } else {
        (*event).fork.ppid = tgid as u32;
        (*event).fork.ptid = tgid as u32;
    }
    (*event).fork.pid = tgid as u32;
    (*event).fork.tid = pid as u32;
    (*event).fork.header.type_ = PERF_RECORD_FORK;
    (*event).fork.header.misc = PERF_RECORD_MISC_FORK_EXEC;
    (*event).fork.header.size = (size_of::<perf_record_fork>() + (*machine).id_hdr_size) as u16;

    if perf_tool__process_synth_event(tool, event, machine, process) != 0 {
        return -1;
    }
    0
}

unsafe fn io__drain_line(io_: *mut io, mut ch: c_int) {
    if ch == '\n' as c_int {
        return;
    }
    if ch == -2 && (*io_).data > (*io_).buf && *(*io_).data.offset(-1) == '\n' as c_char {
        return;
    }

    loop {
        ch = io__get_char(io_);
        if !(ch >= 0 && ch != '\n' as c_int) {
            break;
        }
    }
}

unsafe fn read_proc_maps_line(
    io_: *mut io,
    start: *mut __u64,
    end: *mut __u64,
    prot: *mut u32,
    flags: *mut u32,
    offset: *mut __u64,
    maj: *mut u32,
    min: *mut u32,
    inode: *mut __u64,
    pathname_size: ssize_t,
    pathname: *mut c_char,
) -> bool {
    let mut temp: __u64 = 0;
    let mut ch: c_int;
    let mut written: size_t = 0;
    let mut overflowed = false;

    ch = io__get_hex(io_, start);
    if ch != '-' as c_int {
        if !(*io_).eof {
            io__drain_line(io_, ch);
        }
        return false;
    }
    ch = io__get_hex(io_, end);
    if ch != ' ' as c_int {
        if !(*io_).eof {
            io__drain_line(io_, ch);
        }
        return false;
    }

    /* map protection and flags bits */
    *prot = 0;
    ch = io__get_char(io_);
    if ch == 'r' as c_int {
        *prot |= PROT_READ;
    } else if ch != '-' as c_int {
        if !(*io_).eof {
            io__drain_line(io_, ch);
        }
        return false;
    }
    ch = io__get_char(io_);
    if ch == 'w' as c_int {
        *prot |= PROT_WRITE;
    } else if ch != '-' as c_int {
        if !(*io_).eof {
            io__drain_line(io_, ch);
        }
        return false;
    }
    ch = io__get_char(io_);
    if ch == 'x' as c_int {
        *prot |= PROT_EXEC;
    } else if ch != '-' as c_int {
        if !(*io_).eof {
            io__drain_line(io_, ch);
        }
        return false;
    }
    ch = io__get_char(io_);
    if ch == 's' as c_int {
        *flags = MAP_SHARED;
    } else if ch == 'p' as c_int {
        *flags = MAP_PRIVATE;
    } else {
        if !(*io_).eof {
            io__drain_line(io_, ch);
        }
        return false;
    }
    ch = io__get_char(io_);
    if ch != ' ' as c_int {
        if !(*io_).eof {
            io__drain_line(io_, ch);
        }
        return false;
    }

    ch = io__get_hex(io_, offset);
    if ch != ' ' as c_int {
        if !(*io_).eof {
            io__drain_line(io_, ch);
        }
        return false;
    }

    ch = io__get_hex(io_, &mut temp);
    if ch != ':' as c_int {
        if !(*io_).eof {
            io__drain_line(io_, ch);
        }
        return false;
    }
    *maj = temp as u32;
    ch = io__get_hex(io_, &mut temp);
    if ch != ' ' as c_int {
        if !(*io_).eof {
            io__drain_line(io_, ch);
        }
        return false;
    }
    *min = temp as u32;

    ch = io__get_dec(io_, inode);
    if ch != ' ' as c_int {
        if ch == '\n' as c_int {
            *pathname = 0;
            return true;
        }
        if !(*io_).eof {
            io__drain_line(io_, ch);
        }
        return false;
    }

    loop {
        ch = io__get_char(io_);
        if ch != ' ' as c_int {
            break;
        }
    }

    loop {
        if ch < 0 {
            if overflowed {
                strlcpy(pathname, cstr(b"//toolong\0"), pathname_size as size_t);
                return true;
            }
            *pathname.add(written) = 0;
            return written > 0;
        }
        if ch == 0 || ch == '\n' as c_int {
            break;
        }

        if written < pathname_size as size_t - 1 {
            *pathname.add(written) = ch as c_char;
            written += 1;
        } else {
            overflowed = true;
        }
        ch = io__get_char(io_);
    }

    if overflowed {
        strlcpy(pathname, cstr(b"//toolong\0"), pathname_size as size_t);
    } else {
        *pathname.add(written) = 0;
    }

    true
}

unsafe fn perf_record_mmap2__read_build_id(
    event: *mut perf_record_mmap2,
    machine: *mut machine,
    is_kernel: bool,
) {
    let mut bid = build_id { size: 0, data: [0; 20] };
    let mut nsi: *mut nsinfo;
    let mut nc: nscookie = core::mem::zeroed();
    let mut dso: *mut dso = ptr::null_mut();
    let mut dso_id: dso_id = ptr::read(&dso_id_empty);
    let rc: c_int;

    if is_kernel {
        rc = sysfs__read_build_id(cstr(b"/sys/kernel/notes\0"), &mut bid);
    } else {
        if ((*event).header.misc & PERF_RECORD_MISC_MMAP_BUILD_ID) != 0 {
            build_id__init(&mut dso_id.build_id, (*event).build_id.as_ptr(), (*event).build_id_size);
        } else {
            dso_id.maj = (*event).maj;
            dso_id.min = (*event).min;
            dso_id.ino = (*event).ino;
            dso_id.ino_generation = (*event).ino_generation;
            dso_id.mmap2_valid = true;
            dso_id.mmap2_ino_generation_valid = true;
        }

        dso = dsos__findnew_id(&mut (*machine).dsos, (*event).filename.as_ptr(), &dso_id);
        if !dso.is_null() && dso__has_build_id(dso) {
            bid = ptr::read(dso__bid(dso));
            rc = 0;
        } else {
            nsi = nsinfo__new((*event).pid as pid_t);
            nsinfo__mountns_enter(nsi, &mut nc);
            rc = if filename__read_build_id((*event).filename.as_ptr(), &mut bid) > 0 { 0 } else { -1 };
            nsinfo__mountns_exit(&mut nc);
            nsinfo__put(nsi);
        }
    }

    if rc == 0 {
        memcpy((*event).build_id.as_mut_ptr() as *mut c_void, bid.data.as_ptr() as *const c_void, bid.data.len());
        (*event).build_id_size = bid.size as u8;
        (*event).header.misc |= PERF_RECORD_MISC_MMAP_BUILD_ID;
        (*event).__reserved_1 = 0;
        (*event).__reserved_2 = 0;

        if !dso.is_null() && !dso__has_build_id(dso) {
            dso__set_build_id(dso, &bid);
        }
    } else if *(*event).filename.as_ptr() == '/' as c_char {
        pr_debug2(cstr(b"Failed to read build ID for %s\n\0"), (*event).filename.as_ptr());
    }
    dso__put(dso);
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_mmap_events(
    tool: *const perf_tool,
    event: *mut perf_event,
    pid: pid_t,
    tgid: pid_t,
    process: perf_event__handler_t,
    machine: *mut machine,
    mmap_data: bool,
) -> c_int {
    let mut t: u64;
    let mut bf = [0 as c_char; BUFSIZ];
    let mut io_: io = core::mem::zeroed();
    let mut truncation = false;
    let timeout = proc_map_timeout as u64 * 1000000u64;
    let mut rc = 0;
    let hugetlbfs_mnt = hugetlbfs__mountpoint();
    let hugetlbfs_mnt_len = if !hugetlbfs_mnt.is_null() { strlen(hugetlbfs_mnt) } else { 0 };

    if machine__is_default_guest(machine) {
        return 0;
    }

    snprintf(
        bf.as_mut_ptr(),
        bf.len(),
        cstr(b"%s/proc/%d/task/%d/maps\0"),
        (*machine).root_dir,
        pid,
        pid,
    );

    io_.fd = open(bf.as_ptr(), O_RDONLY, 0);
    if io_.fd < 0 {
        /*
         * We raced with a task exiting - just return:
         */
        pr_debug(cstr(b"couldn't open %s\n\0"), bf.as_ptr());
        return -1;
    }
    io__init(&mut io_, io_.fd, bf.as_mut_ptr(), bf.len());

    (*event_header(event)).type_ = PERF_RECORD_MMAP2;
    t = rdclock();

    while !io_.eof {
        static anonstr: &[u8] = b"//anon\0";
        let mut size: size_t;
        let aligned_size: size_t;
        let mut start: __u64 = 0;
        let mut end: __u64 = 0;
        let mut pgoff: __u64 = 0;
        let mut ino: __u64 = 0;
        let mut prot: u32 = 0;
        let mut flags: u32 = 0;
        let mut maj: u32 = 0;
        let mut min: u32 = 0;

        /* 00400000-0040c000 r-xp 00000000 fd:01 41038  /bin/cat */
        /* Read directly into event->mmap2.filename, clamping for id_hdr_size! */
        if !read_proc_maps_line(
            &mut io_,
            &mut start,
            &mut end,
            &mut prot,
            &mut flags,
            &mut pgoff,
            &mut maj,
            &mut min,
            &mut ino,
            ((*event).mmap2.filename.len() - (*machine).id_hdr_size) as ssize_t,
            (*event).mmap2.filename.as_mut_ptr(),
        ) {
            if io_.eof {
                break;
            }
            continue;
        }

        if strcmp((*event).mmap2.filename.as_ptr(), cstr(b"\0")) == 0 {
            strcpy((*event).mmap2.filename.as_mut_ptr(), anonstr.as_ptr() as *const c_char);
        }

        if hugetlbfs_mnt_len != 0
            && strncmp((*event).mmap2.filename.as_ptr(), hugetlbfs_mnt, hugetlbfs_mnt_len) == 0
        {
            strcpy((*event).mmap2.filename.as_mut_ptr(), anonstr.as_ptr() as *const c_char);
            flags |= MAP_HUGETLB;
        }

        size = strlen((*event).mmap2.filename.as_ptr()) + 1;
        aligned_size = PERF_ALIGN(size, size_of::<u64>());
        (*event).mmap2.header.type_ = PERF_RECORD_MMAP2;

        /*
         * Just like the kernel, see perf_misc_flags() in
         * kernel/events/core.c
         */
        if machine__is_host(machine) {
            (*event_header(event)).misc = PERF_RECORD_MISC_USER;
        } else {
            (*event_header(event)).misc = PERF_RECORD_MISC_GUEST_USER;
        }

        if rdclock().wrapping_sub(t) > timeout {
            pr_warning(
                cstr(b"Reading %s/proc/%d/task/%d/maps time out. You may want to increase the time limit by --proc-map-timeout\n\0"),
                (*machine).root_dir,
                pid,
                pid,
            );
            truncation = true;
        } else if (prot & PROT_EXEC) == 0 {
            if !mmap_data || (prot & PROT_READ) == 0 {
                continue;
            }
            (*event_header(event)).misc |= PERF_RECORD_MISC_MMAP_DATA;
        }

        if truncation {
            (*event_header(event)).misc |= PERF_RECORD_MISC_PROC_MAP_PARSE_TIMEOUT;
        }

        (*event).mmap2.header.size =
            (offset_of!(perf_record_mmap2, filename) + aligned_size) as u16;

        /* Zero the padding and ID header trailer safely! */
        memset(
            (event as *mut c_char).add(offset_of!(perf_record_mmap2, filename) + size) as *mut c_void,
            0,
            (aligned_size - size) + (*machine).id_hdr_size,
        );

        (*event).mmap2.header.size = (*event).mmap2.header.size.wrapping_add((*machine).id_hdr_size as u16);
        (*event).mmap2.start = start;
        (*event).mmap2.len = end.wrapping_sub(start);
        (*event).mmap2.pgoff = pgoff;
        (*event).mmap2.maj = maj;
        (*event).mmap2.min = min;
        (*event).mmap2.ino = ino;
        (*event).mmap2.ino_generation = 0;
        (*event).mmap2.pid = tgid as u32;
        (*event).mmap2.tid = pid as u32;
        (*event).mmap2.prot = prot;
        (*event).mmap2.flags = flags;

        if !symbol_conf.no_buildid_mmap2 {
            perf_record_mmap2__read_build_id(&mut (*event).mmap2, machine, false);
        }

        if perf_tool__process_synth_event(tool, event, machine, process) != 0 {
            rc = -1;
            break;
        }

        if truncation {
            break;
        }
    }

    close(io_.fd);
    rc
}

/* HAVE_FILE_HANDLE conditional: translated primary implementation; build configuration must select it. */
unsafe fn perf_event__synthesize_cgroup(
    tool: *const perf_tool,
    event: *mut perf_event,
    path: *mut c_char,
    mount_len: size_t,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let event_size = size_of::<perf_record_cgroup>() - (*event).cgroup.path.len();
    let raw_path_len: size_t;
    let mut path_len: size_t;
    let max_path_len: size_t;
    let mut handle: cgroup_handle = core::mem::zeroed();
    let mut mount_id: c_int = 0;

    if strlen(path) < mount_len {
        return -1;
    }

    max_path_len = (*event).cgroup.path.len() - (*machine).id_hdr_size;
    raw_path_len = {
        let mut v = strlen(path) - mount_len + 1;
        if v > max_path_len {
            v = max_path_len;
        }
        v
    };
    path_len = PERF_ALIGN(raw_path_len, size_of::<u64>());

    memset(&mut (*event).cgroup as *mut _ as *mut c_void, 0, event_size);
    (*event).cgroup.header.type_ = PERF_RECORD_CGROUP;
    (*event).cgroup.header.size = (event_size + path_len + (*machine).id_hdr_size) as u16;

    handle.fh.handle_bytes = size_of::<u64>() as c_uint;
    if name_to_handle_at(AT_FDCWD, path, &mut handle.fh, &mut mount_id, 0) < 0 {
        pr_debug(cstr(b"stat failed: %s\n\0"), path);
        return -1;
    }

    (*event).cgroup.id = handle.cgroup_id;
    strlcpy((*event).cgroup.path.as_mut_ptr(), path.add(mount_len), raw_path_len);
    memset(
        (event as *mut c_char).add(offset_of!(perf_record_cgroup, path) + raw_path_len) as *mut c_void,
        0,
        (path_len - raw_path_len) + (*machine).id_hdr_size,
    );

    if perf_tool__process_synth_event(tool, event, machine, process) < 0 {
        pr_debug(cstr(b"process synth event failed\n\0"));
        return -1;
    }
    0
}

unsafe fn perf_event__walk_cgroup_tree(
    tool: *const perf_tool,
    event: *mut perf_event,
    path: *mut c_char,
    mount_len: size_t,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let pos = strlen(path);
    let d: *mut DIR;
    let mut dent: *mut dirent;
    let mut ret = 0;

    if perf_event__synthesize_cgroup(tool, event, path, mount_len, process, machine) < 0 {
        return -1;
    }

    d = opendir(path);
    if d.is_null() {
        pr_debug(cstr(b"failed to open directory: %s\n\0"), path);
        return -1;
    }

    loop {
        dent = readdir(d);
        if dent.is_null() {
            break;
        }
        if (*dent).d_type != 4 {
            continue;
        }
        if strcmp((*dent).d_name.as_ptr(), cstr(b".\0")) == 0
            || strcmp((*dent).d_name.as_ptr(), cstr(b"..\0")) == 0
        {
            continue;
        }

        /* any sane path should be less than PATH_MAX */
        if strlen(path) + strlen((*dent).d_name.as_ptr()) + 1 >= PATH_MAX {
            continue;
        }

        if *path.add(pos - 1) != '/' as c_char {
            strcat(path, cstr(b"/\0"));
        }
        strcat(path, (*dent).d_name.as_ptr());

        ret = perf_event__walk_cgroup_tree(tool, event, path, mount_len, process, machine);
        if ret < 0 {
            break;
        }

        *path.add(pos) = 0;
    }

    closedir(d);
    ret
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_cgroups(
    tool: *const perf_tool,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let mut event: perf_event = core::mem::zeroed();
    let mut cgrp_root = [0 as c_char; PATH_MAX];
    let mount_len: size_t; /* length of mount point in the path */

    if tool.is_null() || !(*tool).cgroup_events {
        return 0;
    }

    if cgroupfs_find_mountpoint(cgrp_root.as_mut_ptr(), PATH_MAX, cstr(b"perf_event\0")) < 0 {
        pr_debug(cstr(b"cannot find cgroup mount point\n\0"));
        return -1;
    }

    mount_len = strlen(cgrp_root.as_ptr());
    /* make sure the path starts with a slash (after mount point) */
    strcat(cgrp_root.as_mut_ptr(), cstr(b"/\0"));

    if perf_event__walk_cgroup_tree(tool, &mut event, cgrp_root.as_mut_ptr(), mount_len, process, machine) < 0 {
        return -1;
    }
    0
}

#[repr(C)]
struct perf_event__synthesize_modules_maps_cb_args {
    tool: *const perf_tool,
    process: perf_event__handler_t,
    machine: *mut machine,
    event: *mut perf_event,
    misc: u16,
}

unsafe extern "C" fn perf_event__synthesize_modules_maps_cb(map: *mut map, data: *mut c_void) -> c_int {
    let args = data as *mut perf_event__synthesize_modules_maps_cb_args;
    let event = (*args).event;
    let dso_: *mut dso;
    let mut size: size_t;
    let aligned_size: size_t;
    let mut rc = 0;

    if !__map__is_kmodule(map) {
        return 0;
    }

    dso_ = map__dso(map);
    if !symbol_conf.no_buildid_mmap2 {
        let long_name = dso__long_name(dso_);
        size = strlen(long_name);
        if size >= (*event).mmap2.filename.len() - (*(*args).machine).id_hdr_size {
            size = (*event).mmap2.filename.len() - (*(*args).machine).id_hdr_size - 1;
        }

        strlcpy(
            (*event).mmap2.filename.as_mut_ptr(),
            long_name,
            (*event).mmap2.filename.len() - (*(*args).machine).id_hdr_size,
        );

        aligned_size = PERF_ALIGN(size + 1, size_of::<u64>());
        (*event).mmap2.header.type_ = PERF_RECORD_MMAP2;
        (*event).mmap2.header.misc = (*args).misc;
        (*event).mmap2.header.size = (offset_of!(perf_record_mmap2, filename) + aligned_size) as u16;

        /* Zero the padding and ID header trailer safely! */
        memset(
            (event as *mut c_char).add(offset_of!(perf_record_mmap2, filename) + size) as *mut c_void,
            0,
            (aligned_size - size) + (*(*args).machine).id_hdr_size,
        );

        (*event).mmap2.header.size = (*event).mmap2.header.size.wrapping_add((*(*args).machine).id_hdr_size as u16);
        (*event).mmap2.start = map__start(map);
        (*event).mmap2.len = map__size(map);
        (*event).mmap2.pid = (*(*args).machine).pid as u32;

        /* Clear stale build ID and entire union from previous module iteration */
        (*event).mmap2.header.misc &= !PERF_RECORD_MISC_MMAP_BUILD_ID;
        memset((*event).mmap2.build_id.as_mut_ptr() as *mut c_void, 0, (*event).mmap2.build_id.len());
        (*event).mmap2.build_id_size = 0;
        (*event).mmap2.__reserved_1 = 0;
        (*event).mmap2.__reserved_2 = 0;

        perf_record_mmap2__read_build_id(&mut (*event).mmap2, (*args).machine, false);
    } else {
        let long_name = dso__long_name(dso_);
        size = strlen(long_name);
        if size >= (*event).mmap.filename.len() - (*(*args).machine).id_hdr_size {
            size = (*event).mmap.filename.len() - (*(*args).machine).id_hdr_size - 1;
        }

        strlcpy(
            (*event).mmap.filename.as_mut_ptr(),
            long_name,
            (*event).mmap.filename.len() - (*(*args).machine).id_hdr_size,
        );

        aligned_size = PERF_ALIGN(size + 1, size_of::<u64>());
        (*event).mmap.header.type_ = PERF_RECORD_MMAP;
        (*event).mmap.header.misc = (*args).misc;
        (*event).mmap.header.size = (offset_of!(perf_record_mmap, filename) + aligned_size) as u16;

        /* Zero the padding and ID header trailer safely! */
        memset(
            (event as *mut c_char).add(offset_of!(perf_record_mmap, filename) + size) as *mut c_void,
            0,
            (aligned_size - size) + (*(*args).machine).id_hdr_size,
        );

        (*event).mmap.header.size = (*event).mmap.header.size.wrapping_add((*(*args).machine).id_hdr_size as u16);
        (*event).mmap.start = map__start(map);
        (*event).mmap.len = map__size(map);
        (*event).mmap.pid = (*(*args).machine).pid as u32;
    }

    if perf_tool__process_synth_event((*args).tool, event, (*args).machine, (*args).process) != 0 {
        rc = -1;
    }
    rc
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_modules(
    tool: *const perf_tool,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let rc: c_int;
    let maps = machine__kernel_maps(machine);
    let mut args = perf_event__synthesize_modules_maps_cb_args {
        tool,
        process,
        machine,
        event: ptr::null_mut(),
        misc: 0,
    };
    let size = if symbol_conf.no_buildid_mmap2 {
        size_of::<perf_record_mmap>()
    } else {
        size_of::<perf_record_mmap2>()
    };

    args.event = zalloc(size + (*machine).id_hdr_size) as *mut perf_event;
    if args.event.is_null() {
        pr_debug(cstr(b"Not enough memory synthesizing mmap event for kernel modules\n\0"));
        return -1;
    }

    /*
     * Just like the kernel, see perf_misc_flags() in
     * kernel/events/core.c
     */
    if machine__is_host(machine) {
        args.misc = PERF_RECORD_MISC_KERNEL;
    } else {
        args.misc = PERF_RECORD_MISC_GUEST_KERNEL;
    }

    rc = maps__for_each_map(maps, Some(perf_event__synthesize_modules_maps_cb), &mut args as *mut _ as *mut c_void);
    free(args.event as *mut c_void);
    rc
}

unsafe extern "C" fn filter_task(dirent_: *const dirent) -> c_int {
    isdigit((*dirent_).d_name[0] as c_int)
}

unsafe fn __event__synthesize_thread(
    comm_event: *mut perf_event,
    mmap_event: *mut perf_event,
    fork_event: *mut perf_event,
    namespaces_event: *mut perf_event,
    pid: pid_t,
    full: c_int,
    process: perf_event__handler_t,
    tool: *const perf_tool,
    machine: *mut machine,
    needs_mmap: bool,
    mmap_data: bool,
) -> c_int {
    let mut filename = [0 as c_char; PATH_MAX];
    let mut iod: io_dir = core::mem::zeroed();
    let mut dent: *mut io_dirent64;
    let mut tgid: pid_t = 0;
    let mut ppid: pid_t = 0;
    let mut rc = 0;

    /* special case: only send one comm event using passed in pid */
    if full == 0 {
        tgid = perf_event__synthesize_comm(tool, comm_event, pid, process, machine);
        if tgid == -1 {
            return -1;
        }

        if perf_event__synthesize_namespaces(tool, namespaces_event, pid, tgid, process, machine) < 0 {
            return -1;
        }

        /*
         * send mmap only for thread group leader
         * see thread__init_maps()
         */
        if pid == tgid
            && needs_mmap
            && perf_event__synthesize_mmap_events(tool, mmap_event, pid, tgid, process, machine, mmap_data) != 0
        {
            return -1;
        }
        return 0;
    }

    if machine__is_default_guest(machine) {
        return 0;
    }

    snprintf(filename.as_mut_ptr(), filename.len(), cstr(b"%s/proc/%d/task\0"), (*machine).root_dir, pid);
    io_dir__init(&mut iod, open(filename.as_ptr(), O_CLOEXEC | O_DIRECTORY | O_RDONLY));
    if iod.dirfd < 0 {
        return -1;
    }

    loop {
        dent = io_dir__readdir(&mut iod);
        if dent.is_null() {
            break;
        }
        let mut end: *mut c_char = ptr::null_mut();
        let _pid: pid_t;
        let mut kernel_thread = false;

        if isdigit((*dent).d_name[0] as c_int) == 0 {
            continue;
        }

        _pid = strtol((*dent).d_name.as_ptr(), &mut end, 10) as pid_t;
        if *end != 0 {
            continue;
        }

        /* some threads may exit just after scan, ignore it */
        if perf_event__prepare_comm(comm_event, pid, _pid, machine, &mut tgid, &mut ppid, &mut kernel_thread) != 0 {
            continue;
        }

        rc = -1;
        if perf_event__synthesize_fork(tool, fork_event, _pid, tgid, ppid, process, machine) < 0 {
            break;
        }
        if perf_event__synthesize_namespaces(tool, namespaces_event, _pid, tgid, process, machine) < 0 {
            break;
        }

        /*
         * Send the prepared comm event
         */
        if perf_tool__process_synth_event(tool, comm_event, machine, process) != 0 {
            break;
        }

        rc = 0;
        if _pid == pid && !kernel_thread && needs_mmap {
            /* process the parent's maps too */
            rc = perf_event__synthesize_mmap_events(tool, mmap_event, pid, tgid, process, machine, mmap_data);
            if rc != 0 {
                break;
            }
        }
    }

    close(iod.dirfd);
    rc
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_thread_map(
    tool: *const perf_tool,
    threads: *mut perf_thread_map,
    process: perf_event__handler_t,
    machine: *mut machine,
    needs_mmap: bool,
    mmap_data: bool,
) -> c_int {
    let comm_event: *mut perf_event;
    let mmap_event: *mut perf_event;
    let fork_event: *mut perf_event;
    let namespaces_event: *mut perf_event;
    let mut err = -1;
    let mut thread: c_int;

    comm_event = malloc(size_of::<perf_record_comm>() + (*machine).id_hdr_size) as *mut perf_event;
    if comm_event.is_null() {
        return err;
    }
    mmap_event = malloc(size_of::<perf_record_mmap2>() + (*machine).id_hdr_size) as *mut perf_event;
    if mmap_event.is_null() {
        free(comm_event as *mut c_void);
        return err;
    }
    fork_event = malloc(size_of::<perf_record_fork>() + (*machine).id_hdr_size) as *mut perf_event;
    if fork_event.is_null() {
        free(mmap_event as *mut c_void);
        free(comm_event as *mut c_void);
        return err;
    }
    namespaces_event = malloc(
        size_of::<perf_record_namespaces>() + (NR_NAMESPACES * size_of::<perf_ns_link_info>()) + (*machine).id_hdr_size,
    ) as *mut perf_event;
    if namespaces_event.is_null() {
        free(fork_event as *mut c_void);
        free(mmap_event as *mut c_void);
        free(comm_event as *mut c_void);
        return err;
    }

    err = 0;
    thread = 0;
    while thread < (*threads).nr {
        if __event__synthesize_thread(
            comm_event,
            mmap_event,
            fork_event,
            namespaces_event,
            perf_thread_map__pid(threads, thread),
            0,
            process,
            tool,
            machine,
            needs_mmap,
            mmap_data,
        ) != 0
        {
            err = -1;
            break;
        }

        /*
         * comm.pid is set to thread group id by
         * perf_event__synthesize_comm
         */
        if (*comm_event).comm.pid as c_int != perf_thread_map__pid(threads, thread) {
            let mut need_leader = true;
            let mut j = 0;
            /* is thread group leader in thread_map? */
            while j < (*threads).nr {
                if (*comm_event).comm.pid as c_int == perf_thread_map__pid(threads, j) {
                    need_leader = false;
                    break;
                }
                j += 1;
            }

            /* if not, generate events for it */
            if need_leader
                && __event__synthesize_thread(
                    comm_event,
                    mmap_event,
                    fork_event,
                    namespaces_event,
                    (*comm_event).comm.pid as pid_t,
                    0,
                    process,
                    tool,
                    machine,
                    needs_mmap,
                    mmap_data,
                ) != 0
            {
                err = -1;
                break;
            }
        }
        thread += 1;
    }

    free(namespaces_event as *mut c_void);
    free(fork_event as *mut c_void);
    free(mmap_event as *mut c_void);
    free(comm_event as *mut c_void);
    err
}

unsafe fn __perf_event__synthesize_threads(
    tool: *const perf_tool,
    process: perf_event__handler_t,
    machine: *mut machine,
    needs_mmap: bool,
    mmap_data: bool,
    dirent_: *mut *mut dirent,
    start: c_int,
    num: c_int,
) -> c_int {
    let comm_event: *mut perf_event;
    let mmap_event: *mut perf_event;
    let fork_event: *mut perf_event;
    let namespaces_event: *mut perf_event;
    let mut err = -1;
    let mut end: *mut c_char = ptr::null_mut();
    let mut pid: pid_t;
    let mut i: c_int;

    comm_event = malloc(size_of::<perf_record_comm>() + (*machine).id_hdr_size) as *mut perf_event;
    if comm_event.is_null() {
        return err;
    }
    mmap_event = malloc(size_of::<perf_record_mmap2>() + (*machine).id_hdr_size) as *mut perf_event;
    if mmap_event.is_null() {
        free(comm_event as *mut c_void);
        return err;
    }
    fork_event = malloc(size_of::<perf_record_fork>() + (*machine).id_hdr_size) as *mut perf_event;
    if fork_event.is_null() {
        free(mmap_event as *mut c_void);
        free(comm_event as *mut c_void);
        return err;
    }
    namespaces_event = malloc(
        size_of::<perf_record_namespaces>() + (NR_NAMESPACES * size_of::<perf_ns_link_info>()) + (*machine).id_hdr_size,
    ) as *mut perf_event;
    if namespaces_event.is_null() {
        free(fork_event as *mut c_void);
        free(mmap_event as *mut c_void);
        free(comm_event as *mut c_void);
        return err;
    }

    i = start;
    while i < start + num {
        if isdigit((**dirent_.add(i as usize)).d_name[0] as c_int) == 0 {
            i += 1;
            continue;
        }

        pid = strtol((**dirent_.add(i as usize)).d_name.as_ptr(), &mut end, 10) as pid_t;
        /* only interested in proper numerical dirents */
        if *end != 0 {
            i += 1;
            continue;
        }
        /*
         * We may race with exiting thread, so don't stop just because
         * one thread couldn't be synthesized.
         */
        __event__synthesize_thread(
            comm_event,
            mmap_event,
            fork_event,
            namespaces_event,
            pid,
            1,
            process,
            tool,
            machine,
            needs_mmap,
            mmap_data,
        );
        i += 1;
    }
    err = 0;

    free(namespaces_event as *mut c_void);
    free(fork_event as *mut c_void);
    free(mmap_event as *mut c_void);
    free(comm_event as *mut c_void);
    err
}

#[repr(C)]
struct synthesize_threads_arg {
    tool: *const perf_tool,
    process: perf_event__handler_t,
    machine: *mut machine,
    needs_mmap: bool,
    mmap_data: bool,
    dirent: *mut *mut dirent,
    num: c_int,
    start: c_int,
}

unsafe extern "C" fn synthesize_threads_worker(arg: *mut c_void) -> *mut c_void {
    let args = arg as *mut synthesize_threads_arg;
    __perf_event__synthesize_threads(
        (*args).tool,
        (*args).process,
        (*args).machine,
        (*args).needs_mmap,
        (*args).mmap_data,
        (*args).dirent,
        (*args).start,
        (*args).num,
    );
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_threads(
    tool: *const perf_tool,
    process: perf_event__handler_t,
    machine: *mut machine,
    needs_mmap: bool,
    mmap_data: bool,
    nr_threads_synthesize: c_uint,
) -> c_int {
    let mut args: *mut synthesize_threads_arg = ptr::null_mut();
    let mut synthesize_threads: *mut pthread_t = ptr::null_mut();
    let mut proc_path = [0 as c_char; PATH_MAX];
    let mut dirent_: *mut *mut dirent = ptr::null_mut();
    let num_per_thread: c_int;
    let mut m: c_int;
    let n: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut thread_nr: c_int;
    let mut base = 0;
    let mut err = -1;

    if machine__is_default_guest(machine) {
        return 0;
    }

    snprintf(proc_path.as_mut_ptr(), proc_path.len(), cstr(b"%s/proc\0"), (*machine).root_dir);
    n = scandir(proc_path.as_ptr(), &mut dirent_, Some(filter_task), None);
    if n < 0 {
        return err;
    }

    if nr_threads_synthesize == UINT_MAX {
        thread_nr = sysconf(_SC_NPROCESSORS_ONLN) as c_int;
    } else {
        thread_nr = nr_threads_synthesize as c_int;
    }

    if thread_nr <= 1 || n <= 1 {
        err = __perf_event__synthesize_threads(tool, process, machine, needs_mmap, mmap_data, dirent_, base, n);
        i = 0;
        while i < n {
            zfree(dirent_.add(i as usize) as *mut *mut c_void);
            i += 1;
        }
        free(dirent_ as *mut c_void);
        return err;
    }
    if thread_nr > n {
        thread_nr = n;
    }

    synthesize_threads = calloc(thread_nr as size_t, size_of::<pthread_t>()) as *mut pthread_t;
    if synthesize_threads.is_null() {
        i = 0;
        while i < n {
            zfree(dirent_.add(i as usize) as *mut *mut c_void);
            i += 1;
        }
        free(dirent_ as *mut c_void);
        return err;
    }

    args = calloc(thread_nr as size_t, size_of::<synthesize_threads_arg>()) as *mut synthesize_threads_arg;
    if args.is_null() {
        free(synthesize_threads as *mut c_void);
        i = 0;
        while i < n {
            zfree(dirent_.add(i as usize) as *mut *mut c_void);
            i += 1;
        }
        free(dirent_ as *mut c_void);
        return err;
    }

    num_per_thread = n / thread_nr;
    m = n % thread_nr;
    i = 0;
    while i < thread_nr {
        (*args.add(i as usize)).tool = tool;
        (*args.add(i as usize)).process = process;
        (*args.add(i as usize)).machine = machine;
        (*args.add(i as usize)).needs_mmap = needs_mmap;
        (*args.add(i as usize)).mmap_data = mmap_data;
        (*args.add(i as usize)).dirent = dirent_;
        i += 1;
    }
    i = 0;
    while i < m {
        (*args.add(i as usize)).num = num_per_thread + 1;
        (*args.add(i as usize)).start = i * (*args.add(i as usize)).num;
        i += 1;
    }
    if i != 0 {
        base = (*args.add((i - 1) as usize)).start + (*args.add((i - 1) as usize)).num;
    }
    j = i;
    while j < thread_nr {
        (*args.add(j as usize)).num = num_per_thread;
        (*args.add(j as usize)).start = base + (j - i) * (*args.add(i as usize)).num;
        j += 1;
    }

    i = 0;
    while i < thread_nr {
        if pthread_create(
            synthesize_threads.add(i as usize),
            ptr::null(),
            synthesize_threads_worker,
            args.add(i as usize) as *mut c_void,
        ) != 0
        {
            break;
        }
        i += 1;
    }
    if i == thread_nr {
        err = 0;
    }
    j = 0;
    while j < i {
        pthread_join(*synthesize_threads.add(j as usize), ptr::null_mut());
        j += 1;
    }
    free(args as *mut c_void);
    free(synthesize_threads as *mut c_void);
    i = 0;
    while i < n {
        zfree(dirent_.add(i as usize) as *mut *mut c_void);
        i += 1;
    }
    free(dirent_ as *mut c_void);

    err
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_extra_kmaps(
    _tool: *const perf_tool,
    _process: perf_event__handler_t,
    _machine: *mut machine,
) -> c_int {
    0
}

unsafe fn __perf_event__synthesize_kernel_mmap(
    tool: *const perf_tool,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let event: *mut perf_event;
    let mut size = if symbol_conf.no_buildid_mmap2 {
        size_of::<perf_record_mmap>()
    } else {
        size_of::<perf_record_mmap2>()
    };
    let map_ = machine__kernel_map(machine);
    let kmap_: *mut kmap;
    let err: c_int;

    if map_.is_null() {
        return -1;
    }
    kmap_ = map__kmap(map_);
    if (*kmap_).ref_reloc_sym.is_null() {
        return -1;
    }

    /*
     * We should get this from /sys/kernel/sections/.text, but till that is
     * available use this, and after it is use this as a fallback for older
     * kernels.
     */
    event = zalloc(size + (*machine).id_hdr_size) as *mut perf_event;
    if event.is_null() {
        pr_debug(cstr(b"Not enough memory synthesizing mmap event for kernel modules\n\0"));
        return -1;
    }

    if machine__is_host(machine) {
        /*
         * kernel uses PERF_RECORD_MISC_USER for user space maps,
         * see kernel/perf_event.c __perf_event_mmap
         */
        (*event_header(event)).misc = PERF_RECORD_MISC_KERNEL;
    } else {
        (*event_header(event)).misc = PERF_RECORD_MISC_GUEST_KERNEL;
    }

    if !symbol_conf.no_buildid_mmap2 {
        size = (snprintf(
            (*event).mmap2.filename.as_mut_ptr(),
            (*event).mmap2.filename.len(),
            cstr(b"%s%s\0"),
            (*machine).mmap_name,
            (*(*kmap_).ref_reloc_sym).name,
        ) + 1) as size_t;
        size = PERF_ALIGN(size, size_of::<u64>());
        (*event).mmap2.header.type_ = PERF_RECORD_MMAP2;
        (*event).mmap2.header.size = (size_of::<perf_record_mmap2>()
            - ((*event).mmap2.filename.len() - size)
            + (*machine).id_hdr_size) as u16;
        (*event).mmap2.pgoff = (*(*kmap_).ref_reloc_sym).addr;
        (*event).mmap2.start = map__start(map_);
        (*event).mmap2.len = map__end(map_).wrapping_sub((*event).mmap.start);
        (*event).mmap2.pid = (*machine).pid as u32;

        perf_record_mmap2__read_build_id(&mut (*event).mmap2, machine, true);
    } else {
        size = (snprintf(
            (*event).mmap.filename.as_mut_ptr(),
            (*event).mmap.filename.len(),
            cstr(b"%s%s\0"),
            (*machine).mmap_name,
            (*(*kmap_).ref_reloc_sym).name,
        ) + 1) as size_t;
        size = PERF_ALIGN(size, size_of::<u64>());
        (*event).mmap.header.type_ = PERF_RECORD_MMAP;
        (*event).mmap.header.size = (size_of::<perf_record_mmap>()
            - ((*event).mmap.filename.len() - size)
            + (*machine).id_hdr_size) as u16;
        (*event).mmap.pgoff = (*(*kmap_).ref_reloc_sym).addr;
        (*event).mmap.start = map__start(map_);
        (*event).mmap.len = map__end(map_).wrapping_sub((*event).mmap.start);
        (*event).mmap.pid = (*machine).pid as u32;
    }

    err = perf_tool__process_synth_event(tool, event, machine, process);
    free(event as *mut c_void);
    err
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_kernel_mmap(
    tool: *const perf_tool,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let err = __perf_event__synthesize_kernel_mmap(tool, process, machine);
    if err < 0 {
        return err;
    }
    perf_event__synthesize_extra_kmaps(tool, process, machine)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_thread_map2(
    tool: *const perf_tool,
    threads: *mut perf_thread_map,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let event: *mut perf_event;
    let mut i: c_int;
    let err: c_int;
    let mut size: c_int;

    size = size_of::<perf_record_thread_map>() as c_int;
    size += (*threads).nr * size_of::<perf_record_thread_map_entry>() as c_int;

    event = zalloc(size as size_t) as *mut perf_event;
    if event.is_null() {
        return -ENOMEM;
    }

    (*event_header(event)).type_ = PERF_RECORD_THREAD_MAP;
    (*event_header(event)).size = size as u16;
    (*event).thread_map.nr = (*threads).nr as u64;

    i = 0;
    while i < (*threads).nr {
        let entry = ((*event).thread_map.entries.as_mut_ptr()).add(i as usize);
        let mut comm = perf_thread_map__comm(threads, i);
        if comm.is_null() {
            comm = cstr(b"\0") as *mut c_char;
        }
        (*entry).pid = perf_thread_map__pid(threads, i) as i64;
        strncpy((*entry).comm.as_mut_ptr(), comm, (*entry).comm.len());
        i += 1;
    }

    err = process.unwrap()(tool, event, ptr::null_mut(), machine);
    free(event as *mut c_void);
    err
}

#[repr(C)]
struct synthesize_cpu_map_data {
    map: *const perf_cpu_map,
    nr: c_int,
    min_cpu: c_int,
    max_cpu: c_int,
    has_any_cpu: c_int,
    type_: c_int,
    size: size_t,
    data: *mut perf_record_cpu_map_data,
}

unsafe fn synthesize_cpus(data: *mut synthesize_cpu_map_data) {
    (*(*data).data).type_ = PERF_CPU_MAP__CPUS as u16;
    (*(*data).data).data.cpus_data.nr = (*data).nr as u16;
    let mut i = 0;
    while i < (*data).nr {
        (*(*data).data).data.cpus_data.cpu.as_mut_ptr().add(i as usize).write(perf_cpu_map__cpu((*data).map, i).cpu as u16);
        i += 1;
    }
}

unsafe fn synthesize_mask(data: *mut synthesize_cpu_map_data) {
    let mut idx: c_uint = 0;
    /* Due to padding, the 4bytes per entry mask variant is always smaller. */
    (*(*data).data).type_ = PERF_CPU_MAP__MASK as u16;
    (*(*data).data).data.mask32_data.nr = BITS_TO_U32((*data).max_cpu) as u16;
    (*(*data).data).data.mask32_data.long_size = 4;

    while (idx as c_int) < (*data).nr {
        let cpu = perf_cpu_map__cpu((*data).map, idx as c_int);
        let bit_word = cpu.cpu / 32;
        let bit_mask: u32 = 1u32 << (cpu.cpu & 31);
        let p = (*(*data).data).data.mask32_data.mask.as_mut_ptr().add(bit_word as usize);
        *p |= bit_mask;
        idx += 1;
    }
}

unsafe fn synthesize_range_cpus(data: *mut synthesize_cpu_map_data) {
    (*(*data).data).type_ = PERF_CPU_MAP__RANGE_CPUS as u16;
    (*(*data).data).data.range_cpu_data.any_cpu = (*data).has_any_cpu as u16;
    (*(*data).data).data.range_cpu_data.start_cpu = (*data).min_cpu as u16;
    (*(*data).data).data.range_cpu_data.end_cpu = (*data).max_cpu as u16;
}

unsafe fn cpu_map_data__alloc(syn_data: *mut synthesize_cpu_map_data, header_size: size_t) -> *mut c_void {
    let size_cpus: size_t;
    let size_mask: size_t;

    (*syn_data).nr = perf_cpu_map__nr((*syn_data).map);
    (*syn_data).has_any_cpu = if perf_cpu_map__cpu((*syn_data).map, 0).cpu == -1 { 1 } else { 0 };

    (*syn_data).min_cpu = perf_cpu_map__cpu((*syn_data).map, (*syn_data).has_any_cpu).cpu;
    (*syn_data).max_cpu = perf_cpu_map__max((*syn_data).map).cpu;
    if (*syn_data).max_cpu - (*syn_data).min_cpu + 1 == (*syn_data).nr - (*syn_data).has_any_cpu {
        /* A consecutive range of CPUs can be encoded using a range. */
        assert(size_of::<u16>() + size_of::<perf_record_range_cpu_map>() == size_of::<u64>());
        (*syn_data).type_ = PERF_CPU_MAP__RANGE_CPUS;
        (*syn_data).size = header_size + size_of::<u64>();
        return zalloc((*syn_data).size);
    }

    size_cpus = size_of::<u16>() + size_of::<cpu_map_entries>() + (*syn_data).nr as size_t * size_of::<u16>();
    /* Due to padding, the 4bytes per entry mask variant is always smaller. */
    size_mask = size_of::<u16>()
        + size_of::<perf_record_mask_cpu_map32>()
        + BITS_TO_U32((*syn_data).max_cpu) * size_of::<__u32>();
    if (*syn_data).has_any_cpu != 0 || size_cpus < size_mask {
        /* Follow the CPU map encoding. */
        (*syn_data).type_ = PERF_CPU_MAP__CPUS;
        (*syn_data).size = header_size + PERF_ALIGN(size_cpus, size_of::<u64>());
        return zalloc((*syn_data).size);
    }
    /* Encode using a bitmask. */
    (*syn_data).type_ = PERF_CPU_MAP__MASK;
    (*syn_data).size = header_size + PERF_ALIGN(size_mask, size_of::<u64>());
    zalloc((*syn_data).size)
}

unsafe fn cpu_map_data__synthesize(data: *mut synthesize_cpu_map_data) {
    match (*data).type_ {
        PERF_CPU_MAP__CPUS => synthesize_cpus(data),
        PERF_CPU_MAP__MASK => synthesize_mask(data),
        PERF_CPU_MAP__RANGE_CPUS => synthesize_range_cpus(data),
        _ => {}
    }
}

unsafe fn cpu_map_event__new(map: *const perf_cpu_map) -> *mut perf_record_cpu_map {
    let mut syn_data: synthesize_cpu_map_data = core::mem::zeroed();
    let event: *mut perf_record_cpu_map;

    syn_data.map = map;
    event = cpu_map_data__alloc(&mut syn_data, size_of::<perf_event_header>()) as *mut perf_record_cpu_map;
    if event.is_null() {
        return ptr::null_mut();
    }

    syn_data.data = &mut (*event).data;
    (*event).header.type_ = PERF_RECORD_CPU_MAP;
    (*event).header.size = syn_data.size as u16;
    cpu_map_data__synthesize(&mut syn_data);
    event
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_cpu_map(
    tool: *const perf_tool,
    map: *const perf_cpu_map,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let event = cpu_map_event__new(map);
    if event.is_null() {
        return -ENOMEM;
    }
    let err = process.unwrap()(tool, event as *mut perf_event, ptr::null_mut(), machine);
    free(event as *mut c_void);
    err
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_stat_config(
    tool: *const perf_tool,
    config: *mut perf_stat_config,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let event: *mut perf_record_stat_config;
    let mut size: c_int;
    let mut i: usize = 0;
    let err: c_int;

    size = size_of::<perf_record_stat_config>() as c_int;
    size += (PERF_STAT_CONFIG_TERM__MAX * size_of::<perf_record_stat_config_term>()) as c_int;

    event = zalloc(size as size_t) as *mut perf_record_stat_config;
    if event.is_null() {
        return -ENOMEM;
    }

    (*event).header.type_ = PERF_RECORD_STAT_CONFIG;
    (*event).header.size = size as u16;
    (*event).nr = PERF_STAT_CONFIG_TERM__MAX as u64;

    (*event).data.as_mut_ptr().add(i).write(perf_record_stat_config_term { tag: PERF_STAT_CONFIG_TERM__AGGR_MODE, val: (*config).aggr_mode }); i += 1;
    (*event).data.as_mut_ptr().add(i).write(perf_record_stat_config_term { tag: PERF_STAT_CONFIG_TERM__INTERVAL, val: (*config).interval }); i += 1;
    (*event).data.as_mut_ptr().add(i).write(perf_record_stat_config_term { tag: PERF_STAT_CONFIG_TERM__SCALE, val: (*config).scale }); i += 1;
    (*event).data.as_mut_ptr().add(i).write(perf_record_stat_config_term { tag: PERF_STAT_CONFIG_TERM__AGGR_LEVEL, val: (*config).aggr_level }); i += 1;

    WARN_ONCE(i != PERF_STAT_CONFIG_TERM__MAX, cstr(b"stat config terms unbalanced\n\0"));

    err = process.unwrap()(tool, event as *mut perf_event, ptr::null_mut(), machine);
    free(event as *mut c_void);
    err
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_stat(
    tool: *const perf_tool,
    cpu: perf_cpu,
    thread: u32,
    id: u64,
    count: *mut perf_counts_values,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let mut event: perf_record_stat = core::mem::zeroed();
    event.header.type_ = PERF_RECORD_STAT;
    event.header.size = size_of::<perf_record_stat>() as u16;
    event.header.misc = 0;
    event.id = id;
    event.cpu = cpu.cpu as u32;
    event.thread = thread;
    event.val = (*count).val;
    event.ena = (*count).ena;
    event.run = (*count).run;
    process.unwrap()(tool, &mut event as *mut _ as *mut perf_event, ptr::null_mut(), machine)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_stat_round(
    tool: *const perf_tool,
    evtime: u64,
    type_: u64,
    process: perf_event__handler_t,
    machine: *mut machine,
) -> c_int {
    let mut event: perf_record_stat_round = core::mem::zeroed();
    event.header.type_ = PERF_RECORD_STAT_ROUND;
    event.header.size = size_of::<perf_record_stat_round>() as u16;
    event.header.misc = 0;
    event.time = evtime;
    event.type_ = type_;
    process.unwrap()(tool, &mut event as *mut _ as *mut perf_event, ptr::null_mut(), machine)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__sample_event_size(
    sample: *const perf_sample,
    type_: u64,
    read_format: u64,
    branch_sample_type: u64,
) -> size_t {
    let mut sz: size_t;
    let mut result = size_of::<perf_record_sample>();

    if (type_ & PERF_SAMPLE_IDENTIFIER) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_IP) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_TID) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_TIME) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_ADDR) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_ID) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_STREAM_ID) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_CPU) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_PERIOD) != 0 { result += size_of::<u64>(); }

    if (type_ & PERF_SAMPLE_READ) != 0 {
        result += size_of::<u64>();
        if (read_format & PERF_FORMAT_TOTAL_TIME_ENABLED) != 0 { result += size_of::<u64>(); }
        if (read_format & PERF_FORMAT_TOTAL_TIME_RUNNING) != 0 { result += size_of::<u64>(); }
        /* PERF_FORMAT_ID is forced for PERF_SAMPLE_READ */
        if (read_format & PERF_FORMAT_GROUP) != 0 {
            sz = sample_read_value_size(read_format);
            result += sz * (*sample).read.u.group.nr as size_t;
        } else {
            result += size_of::<u64>();
            if (read_format & PERF_FORMAT_LOST) != 0 { result += size_of::<u64>(); }
        }
    }

    if (type_ & PERF_SAMPLE_CALLCHAIN) != 0 {
        sz = ((*(*sample).callchain).nr + 1) as size_t * size_of::<u64>();
        result += sz;
    }
    if (type_ & PERF_SAMPLE_RAW) != 0 {
        result += size_of::<u32>();
        result += (*sample).raw_size as size_t;
    }
    if (type_ & PERF_SAMPLE_BRANCH_STACK) != 0 {
        sz = (*(*sample).branch_stack).nr as size_t * size_of::<branch_entry>();
        /* nr */
        sz += size_of::<u64>();
        if (branch_sample_type & PERF_SAMPLE_BRANCH_HW_INDEX) != 0 {
            sz += size_of::<u64>();
        }
        result += sz;
    }
    if (type_ & PERF_SAMPLE_REGS_USER) != 0 {
        if !(*sample).user_regs.is_null() && (*(*sample).user_regs).abi != 0 {
            result += size_of::<u64>();
            sz = hweight64((*(*sample).user_regs).mask) as size_t * size_of::<u64>();
            result += sz;
        } else {
            result += size_of::<u64>();
        }
    }
    if (type_ & PERF_SAMPLE_STACK_USER) != 0 {
        sz = (*sample).user_stack.size as size_t;
        result += size_of::<u64>();
        if sz != 0 {
            result += sz;
            result += size_of::<u64>();
        }
    }
    if (type_ & PERF_SAMPLE_WEIGHT_TYPE) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_DATA_SRC) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_TRANSACTION) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_REGS_INTR) != 0 {
        if !(*sample).intr_regs.is_null() && (*(*sample).intr_regs).abi != 0 {
            result += size_of::<u64>();
            sz = hweight64((*(*sample).intr_regs).mask) as size_t * size_of::<u64>();
            result += sz;
        } else {
            result += size_of::<u64>();
        }
    }
    if (type_ & PERF_SAMPLE_PHYS_ADDR) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_CGROUP) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_DATA_PAGE_SIZE) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_CODE_PAGE_SIZE) != 0 { result += size_of::<u64>(); }
    if (type_ & PERF_SAMPLE_AUX) != 0 {
        result += size_of::<u64>();
        result += (*sample).aux_sample.size as size_t;
    }
    result
}

unsafe fn perf_synthesize_sample_weight(data: *const perf_sample, array: *mut __u64, type_: u64) {
    *array = (*data).weight;
    if (type_ & PERF_SAMPLE_WEIGHT_STRUCT) != 0 {
        *array &= 0xffffffff;
        *array |= ((*data).ins_lat as u64) << 32;
        *array |= ((*data).weight3 as u64) << 48;
    }
}

unsafe fn copy_read_group_values(
    mut array: *mut __u64,
    read_format: __u64,
    sample: *const perf_sample,
) -> *mut __u64 {
    let sz = sample_read_value_size(read_format);
    let mut v = (*sample).read.u.group.values;
    let mut i = 0u64;
    while i < (*sample).read.u.group.nr {
        /* PERF_FORMAT_ID is forced for PERF_SAMPLE_READ */
        memcpy(array as *mut c_void, v as *const c_void, sz);
        array = (array as *mut u8).add(sz) as *mut __u64;
        v = (v as *mut u8).add(sz) as *mut sample_read_value;
        i += 1;
    }
    array
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_sample(
    event: *mut perf_event,
    type_: u64,
    read_format: u64,
    branch_sample_type: u64,
    sample: *const perf_sample,
) -> c_int {
    let mut array: *mut __u64;
    let mut sz: size_t;
    /*
     * used for cross-endian analysis. See git commit 65014ab3
     * for why this goofiness is needed.
     */
    let mut u: u64_swap = u64_swap { val64: 0 };

    array = (*event).sample.array.as_mut_ptr();

    macro_rules! push64 {
        ($v:expr) => {{
            *array = $v;
            array = array.add(1);
        }};
    }

    if (type_ & PERF_SAMPLE_IDENTIFIER) != 0 { push64!((*sample).id); }
    if (type_ & PERF_SAMPLE_IP) != 0 { push64!((*sample).ip); }
    if (type_ & PERF_SAMPLE_TID) != 0 {
        u.val32[0] = (*sample).pid;
        u.val32[1] = (*sample).tid;
        push64!(u.val64);
    }
    if (type_ & PERF_SAMPLE_TIME) != 0 { push64!((*sample).time); }
    if (type_ & PERF_SAMPLE_ADDR) != 0 { push64!((*sample).addr); }
    if (type_ & PERF_SAMPLE_ID) != 0 { push64!((*sample).id); }
    if (type_ & PERF_SAMPLE_STREAM_ID) != 0 { push64!((*sample).stream_id); }
    if (type_ & PERF_SAMPLE_CPU) != 0 {
        u.val32[0] = (*sample).cpu;
        u.val32[1] = 0;
        push64!(u.val64);
    }
    if (type_ & PERF_SAMPLE_PERIOD) != 0 { push64!((*sample).period); }

    if (type_ & PERF_SAMPLE_READ) != 0 {
        if (read_format & PERF_FORMAT_GROUP) != 0 {
            *array = (*sample).read.u.group.nr;
        } else {
            *array = (*sample).read.u.one.value;
        }
        array = array.add(1);

        if (read_format & PERF_FORMAT_TOTAL_TIME_ENABLED) != 0 { push64!((*sample).read.time_enabled); }
        if (read_format & PERF_FORMAT_TOTAL_TIME_RUNNING) != 0 { push64!((*sample).read.time_running); }

        /* PERF_FORMAT_ID is forced for PERF_SAMPLE_READ */
        if (read_format & PERF_FORMAT_GROUP) != 0 {
            array = copy_read_group_values(array, read_format, sample);
        } else {
            push64!((*sample).read.u.one.id);
            if (read_format & PERF_FORMAT_LOST) != 0 { push64!((*sample).read.u.one.lost); }
        }
    }

    if (type_ & PERF_SAMPLE_CALLCHAIN) != 0 {
        sz = ((*(*sample).callchain).nr + 1) as size_t * size_of::<u64>();
        memcpy(array as *mut c_void, (*sample).callchain as *const c_void, sz);
        array = (array as *mut u8).add(sz) as *mut __u64;
    }
    if (type_ & PERF_SAMPLE_RAW) != 0 {
        let mut array32 = array as *mut u32;
        *array32 = (*sample).raw_size;
        array32 = array32.add(1);
        memcpy(array32 as *mut c_void, (*sample).raw_data, (*sample).raw_size as size_t);
        array = array32.add((*sample).raw_size as usize / size_of::<u32>()) as *mut __u64;
        /* make sure the array is 64-bit aligned */
        BUG_ON((array as c_long % size_of::<u64>() as c_long) != 0);
    }
    if (type_ & PERF_SAMPLE_BRANCH_STACK) != 0 {
        sz = (*(*sample).branch_stack).nr as size_t * size_of::<branch_entry>();
        push64!((*(*sample).branch_stack).nr);
        if (branch_sample_type & PERF_SAMPLE_BRANCH_HW_INDEX) != 0 {
            if (*sample).no_hw_idx {
                push64!(0);
            } else {
                push64!((*(*sample).branch_stack).hw_idx);
            }
        }
        memcpy(array as *mut c_void, perf_sample__branch_entries(sample as *mut perf_sample) as *const c_void, sz);
        array = (array as *mut u8).add(sz) as *mut __u64;
    }
    if (type_ & PERF_SAMPLE_REGS_USER) != 0 {
        if !(*sample).user_regs.is_null() && (*(*sample).user_regs).abi != 0 {
            push64!((*(*sample).user_regs).abi);
            sz = hweight64((*(*sample).user_regs).mask) as size_t * size_of::<u64>();
            memcpy(array as *mut c_void, (*(*sample).user_regs).regs as *const c_void, sz);
            array = (array as *mut u8).add(sz) as *mut __u64;
        } else {
            push64!(0);
        }
    }
    if (type_ & PERF_SAMPLE_STACK_USER) != 0 {
        sz = (*sample).user_stack.size as size_t;
        push64!(sz as u64);
        if sz != 0 {
            memcpy(array as *mut c_void, (*sample).user_stack.data, sz);
            array = (array as *mut u8).add(sz) as *mut __u64;
            push64!(sz as u64);
        }
    }
    if (type_ & PERF_SAMPLE_WEIGHT_TYPE) != 0 {
        perf_synthesize_sample_weight(sample, array, type_);
        array = array.add(1);
    }
    if (type_ & PERF_SAMPLE_DATA_SRC) != 0 { push64!((*sample).data_src); }
    if (type_ & PERF_SAMPLE_TRANSACTION) != 0 { push64!((*sample).transaction); }
    if (type_ & PERF_SAMPLE_REGS_INTR) != 0 {
        if !(*sample).intr_regs.is_null() && (*(*sample).intr_regs).abi != 0 {
            push64!((*(*sample).intr_regs).abi);
            sz = hweight64((*(*sample).intr_regs).mask) as size_t * size_of::<u64>();
            memcpy(array as *mut c_void, (*(*sample).intr_regs).regs as *const c_void, sz);
            array = (array as *mut u8).add(sz) as *mut __u64;
        } else {
            push64!(0);
        }
    }
    if (type_ & PERF_SAMPLE_PHYS_ADDR) != 0 { push64!((*sample).phys_addr); }
    if (type_ & PERF_SAMPLE_CGROUP) != 0 { push64!((*sample).cgroup); }
    if (type_ & PERF_SAMPLE_DATA_PAGE_SIZE) != 0 { push64!((*sample).data_page_size); }
    if (type_ & PERF_SAMPLE_CODE_PAGE_SIZE) != 0 { push64!((*sample).code_page_size); }
    if (type_ & PERF_SAMPLE_AUX) != 0 {
        sz = (*sample).aux_sample.size as size_t;
        push64!(sz as u64);
        memcpy(array as *mut c_void, (*sample).aux_sample.data, sz);
        array = (array as *mut u8).add(sz) as *mut __u64;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_id_sample(
    mut array: *mut __u64,
    type_: u64,
    sample: *const perf_sample,
) -> c_int {
    let start = array;
    /*
     * used for cross-endian analysis. See git commit 65014ab3
     * for why this goofiness is needed.
     */
    let mut u: u64_swap = u64_swap { val64: 0 };

    if (type_ & PERF_SAMPLE_TID) != 0 {
        u.val32[0] = (*sample).pid;
        u.val32[1] = (*sample).tid;
        *array = u.val64;
        array = array.add(1);
    }
    if (type_ & PERF_SAMPLE_TIME) != 0 { *array = (*sample).time; array = array.add(1); }
    if (type_ & PERF_SAMPLE_ID) != 0 { *array = (*sample).id; array = array.add(1); }
    if (type_ & PERF_SAMPLE_STREAM_ID) != 0 { *array = (*sample).stream_id; array = array.add(1); }
    if (type_ & PERF_SAMPLE_CPU) != 0 {
        u.val32[0] = (*sample).cpu;
        u.val32[1] = 0;
        *array = u.val64;
        array = array.add(1);
    }
    if (type_ & PERF_SAMPLE_IDENTIFIER) != 0 { *array = (*sample).id; array = array.add(1); }

    (array as *mut u8).offset_from(start as *mut u8) as c_int
}

/* evlist__for_each_entry is a C macro. The loop body is translated using helper callbacks expected from dependencies. */
unsafe extern "C" {
    fn evlist__first(evlist: *mut evlist) -> *mut evsel;
    fn evlist__next(evlist: *mut evlist, evsel: *mut evsel) -> *mut evsel;
}

#[no_mangle]
pub unsafe extern "C" fn __perf_event__synthesize_id_index(
    tool: *const perf_tool,
    process: perf_event__handler_t,
    evlist: *mut evlist,
    machine: *mut machine,
    from: size_t,
) -> c_int {
    let ev: *mut perf_event;
    let mut evsel_: *mut evsel;
    let mut nr: size_t = 0;
    let mut i: size_t = 0;
    let mut sz: size_t;
    let max_nr: size_t;
    let mut n: size_t;
    let mut pos: size_t;
    let e1_sz = size_of::<id_index_entry>();
    let e2_sz = size_of::<id_index_entry_2>();
    let etot_sz = e1_sz + e2_sz;
    let mut e2_needed = false;
    let err: c_int;

    max_nr = (UINT16_MAX - size_of::<perf_record_id_index>()) / etot_sz;

    pos = 0;
    evsel_ = evlist__first(evlist);
    while !evsel_.is_null() {
        if pos >= from {
            nr += (*evsel_).core.ids as size_t;
        }
        pos += 1;
        evsel_ = evlist__next(evlist, evsel_);
    }

    if nr == 0 {
        return 0;
    }

    pr_debug2(cstr(b"Synthesizing id index\n\0"));

    n = if nr > max_nr { max_nr } else { nr };
    sz = size_of::<perf_record_id_index>() + n * etot_sz;
    ev = zalloc(sz) as *mut perf_event;
    if ev.is_null() {
        return -ENOMEM;
    }

    sz = size_of::<perf_record_id_index>() + n * e1_sz;
    (*event_header(ev)).type_ = PERF_RECORD_ID_INDEX;
    (*ev).id_index.nr = n as u64;

    pos = 0;
    evsel_ = evlist__first(evlist);
    while !evsel_.is_null() {
        if pos >= from {
            let mut j: u32 = 0;
            while j < (*evsel_).core.ids {
                if i >= n {
                    (*ev).id_index.header.size = (sz + if e2_needed { n * e2_sz } else { 0 }) as u16;
                    let e = process.unwrap()(tool, ev, ptr::null_mut(), machine);
                    if e != 0 {
                        free(ev as *mut c_void);
                        return e;
                    }
                    nr -= n;
                    i = 0;
                    e2_needed = false;
                }

                let e = (*ev).id_index.entries.as_mut_ptr().add(i);
                (*e).id = *(*evsel_).core.id.add(j as usize);

                let sid = evlist__id2sid(evlist, (*e).id);
                if sid.is_null() {
                    free(ev as *mut c_void);
                    return -ENOENT;
                }

                (*e).idx = (*sid).idx;
                (*e).cpu = (*sid).cpu.cpu;
                (*e).tid = (*sid).tid;

                if (*sid).machine_pid != 0 {
                    e2_needed = true;
                }

                let e2 = (ev as *mut u8).add(sz) as *mut id_index_entry_2;
                (*e2.add(i)).machine_pid = (*sid).machine_pid;
                (*e2.add(i)).vcpu = (*sid).vcpu.cpu;

                j += 1;
                i += 1;
            }
        }
        pos += 1;
        evsel_ = evlist__next(evlist, evsel_);
    }

    sz = size_of::<perf_record_id_index>() + nr * e1_sz;
    (*ev).id_index.header.size = (sz + if e2_needed { nr * e2_sz } else { 0 }) as u16;
    (*ev).id_index.nr = nr as u64;
    err = process.unwrap()(tool, ev, ptr::null_mut(), machine);
    free(ev as *mut c_void);
    err
}

#[no_mangle]
pub unsafe extern "C" fn perf_event__synthesize_id_index(
    tool: *const perf_tool,
    process: perf_event__handler_t,
    evlist: *mut evlist,
    machine: *mut machine,
) -> c_int {
    __perf_event__synthesize_id_index(tool, process, evlist, machine, 0)
}

#[no_mangle]
pub unsafe extern "C" fn __machine__synthesize_threads(
    machine: *mut machine,
    tool: *const perf_tool,
    target: *mut target,
    threads: *mut perf_thread_map,
    process: perf_event__handler_t,
    needs_mmap: bool,
    data_mmap: bool,
    nr_threads_synthesize: c_uint,
) -> c_int {
    /*
     * When perf runs in non-root PID namespace, and the namespace's proc FS
     * is not mounted, nsinfo__is_in_root_namespace() returns false.
     * In this case, the proc FS is coming for the parent namespace, thus
     * perf tool will wrongly gather process info from its parent PID
     * namespace.
     *
     * To avoid the confusion that the perf tool runs in a child PID
     * namespace but it synthesizes thread info from its parent PID
     * namespace, returns failure with warning.
     */
    if !nsinfo__is_in_root_namespace() {
        pr_err(cstr(b"Perf runs in non-root PID namespace but it tries to \0"));
        pr_err(cstr(b"gather process info from its parent PID namespace.\n\0"));
        pr_err(cstr(b"Please mount the proc file system properly, e.g. \0"));
        pr_err(cstr(b"add the option '--mount-proc' for unshare command.\n\0"));
        return -EPERM;
    }

    if target__has_task(target) {
        return perf_event__synthesize_thread_map(tool, threads, process, machine, needs_mmap, data_mmap);
    } else if target__has_cpu(target) {
        return perf_event__synthesize_threads(tool, process, machine, needs_mmap, data_mmap, nr_threads_synthesize);
    }
    /* command specified */
    0
}

#[no_mangle]
pub unsafe extern "C" fn machine__synthesize_threads(
    machine: *mut machine,
    target: *mut target,
    threads: *mut perf_thread_map,
    needs_mmap: bool,
    data_mmap: bool,
    nr_threads_synthesize: c_uint,
) -> c_int {
    __machine__synthesize_threads(
        machine,
        ptr::null(),
        target,
        threads,
        Some(perf_event__process),
        needs_mmap,
        data_mmap,
        nr_threads_synthesize,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
