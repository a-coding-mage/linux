/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from perf/util/header.h.
 *
 * C include dependencies removed from executable Rust:
 * linux/stddef.h, linux/perf_event.h, sys/types.h, stdio.h, stdbool.h,
 * linux/bitmap.h, linux/types.h, env.h, perf/cpumap.h.
 */

use std::ffi::{c_char, c_int, c_uint, c_void};

pub type size_t = usize;
pub type ssize_t = isize;

pub type FILE = c_void;

#[repr(C)]
pub struct evlist {
    _private: [u8; 0],
}

#[repr(C)]
pub struct evsel {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_event {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_session {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_tool {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_env {
    _private: [u8; 0],
}

#[repr(C)]
pub struct perf_cpu {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpu_cache_level {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cpu_domain_map {
    _private: [u8; 0],
}

pub const HEADER_RESERVED: c_int = 0; /* always cleared */
pub const HEADER_FIRST_FEATURE: c_int = 1;
pub const HEADER_TRACING_DATA: c_int = 1;
pub const HEADER_BUILD_ID: c_int = 2;
pub const HEADER_HOSTNAME: c_int = 3;
pub const HEADER_OSRELEASE: c_int = 4;
pub const HEADER_VERSION: c_int = 5;
pub const HEADER_ARCH: c_int = 6;
pub const HEADER_NRCPUS: c_int = 7;
pub const HEADER_CPUDESC: c_int = 8;
pub const HEADER_CPUID: c_int = 9;
pub const HEADER_TOTAL_MEM: c_int = 10;
pub const HEADER_CMDLINE: c_int = 11;
pub const HEADER_EVENT_DESC: c_int = 12;
pub const HEADER_CPU_TOPOLOGY: c_int = 13;
pub const HEADER_NUMA_TOPOLOGY: c_int = 14;
pub const HEADER_BRANCH_STACK: c_int = 15;
pub const HEADER_PMU_MAPPINGS: c_int = 16;
pub const HEADER_GROUP_DESC: c_int = 17;
pub const HEADER_AUXTRACE: c_int = 18;
pub const HEADER_STAT: c_int = 19;
pub const HEADER_CACHE: c_int = 20;
pub const HEADER_SAMPLE_TIME: c_int = 21;
pub const HEADER_MEM_TOPOLOGY: c_int = 22;
pub const HEADER_CLOCKID: c_int = 23;
pub const HEADER_DIR_FORMAT: c_int = 24;
pub const HEADER_BPF_PROG_INFO: c_int = 25;
pub const HEADER_BPF_BTF: c_int = 26;
pub const HEADER_COMPRESSED: c_int = 27;
pub const HEADER_CPU_PMU_CAPS: c_int = 28;
pub const HEADER_CLOCK_DATA: c_int = 29;
pub const HEADER_HYBRID_TOPOLOGY: c_int = 30;
pub const HEADER_PMU_CAPS: c_int = 31;
pub const HEADER_CPU_DOMAIN_INFO: c_int = 32;
pub const HEADER_E_MACHINE: c_int = 33;
pub const HEADER_CLN_SIZE: c_int = 34;
pub const HEADER_LAST_FEATURE: c_int = 35;
pub const HEADER_FEAT_BITS: usize = 256;

pub const BITS_PER_LONG: usize = usize::BITS as usize;
pub const HEADER_FEAT_LONGS: usize = HEADER_FEAT_BITS / BITS_PER_LONG;

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum perf_header_version {
    PERF_HEADER_VERSION_1,
    PERF_HEADER_VERSION_2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_file_section {
    pub offset: u64,
    pub size: u64,
}

/**
 * struct perf_file_header: Header representation on disk.
 */
#[repr(C)]
pub struct perf_file_header {
    /** @magic: Holds "PERFILE2". */
    pub magic: u64,
    /** @size: Size of this header - sizeof(struct perf_file_header). */
    pub size: u64,
    /**
     * @attr_size: Size of attrs entries - sizeof(struct perf_event_attr) +
     * sizeof(struct perf_file_section).
     */
    pub attr_size: u64,
    /** @attrs: Offset and size of file section holding attributes. */
    pub attrs: perf_file_section,
    /** @data: Offset and size of file section holding regular event data. */
    pub data: perf_file_section,
    /** @event_types: Ignored. */
    pub event_types: perf_file_section,
    /**
     * @adds_features: Bitmap of features. The features are immediately after the data section.
     */
    pub adds_features: [usize; HEADER_FEAT_LONGS],
}

#[repr(C)]
pub struct perf_pipe_file_header {
    pub magic: u64,
    pub size: u64,
}

unsafe extern "C" {
    pub fn perf_file_header__read(
        header: *mut perf_file_header,
        ph: *mut perf_header,
        fd: c_int,
    ) -> c_int;
}

#[repr(C)]
pub struct perf_header {
    pub version: perf_header_version,
    pub needs_swap: bool,
    pub data_offset: u64,
    pub data_size: u64,
    pub feat_offset: u64,
    pub adds_features: [usize; HEADER_FEAT_LONGS],
    pub last_feat: c_int,
    pub env: perf_env,
}

#[repr(C)]
pub struct feat_fd {
    pub ph: *mut perf_header,
    pub fd: c_int,
    pub buf: *mut c_void, /* Either buf != NULL or fd >= 0 */
    pub offset: ssize_t,
    pub size: size_t,
    pub events: *mut evsel,
}

#[repr(C)]
pub struct perf_header_feature_ops {
    pub write: Option<unsafe extern "C" fn(ff: *mut feat_fd, evlist: *mut evlist) -> c_int>,
    pub print: Option<unsafe extern "C" fn(ff: *mut feat_fd, fp: *mut FILE)>,
    pub process: Option<unsafe extern "C" fn(ff: *mut feat_fd, data: *mut c_void) -> c_int>,
    pub name: *const c_char,
    pub full_only: bool,
    pub synthesize: bool,
}

unsafe extern "C" {
    pub static perf_version_string: [c_char; 0];

    pub fn header_feat__name(id: c_uint) -> *const c_char;

    pub fn perf_session__read_header(session: *mut perf_session) -> c_int;
    pub fn perf_session__write_header(
        session: *mut perf_session,
        evlist: *mut evlist,
        fd: c_int,
        at_exit: bool,
    ) -> c_int;
    pub fn perf_header__write_pipe(fd: c_int) -> c_int;
}

/* feat_writer writes a feature section to output */
#[repr(C)]
pub struct feat_writer {
    pub write: Option<unsafe extern "C" fn(fw: *mut feat_writer, buf: *mut c_void, sz: size_t) -> c_int>,
}

/* feat_copier copies a feature section using feat_writer to output */
#[repr(C)]
pub struct feat_copier {
    pub copy:
        Option<unsafe extern "C" fn(fc: *mut feat_copier, feat: c_int, fw: *mut feat_writer) -> c_int>,
}

unsafe extern "C" {
    pub fn perf_session__inject_header(
        session: *mut perf_session,
        evlist: *mut evlist,
        fd: c_int,
        fc: *mut feat_copier,
        write_attrs_after_data: bool,
    ) -> c_int;

    pub fn perf_session__data_offset(evlist: *mut evlist) -> size_t;

    pub fn perf_header__set_feat(header: *mut perf_header, feat: c_int);
    pub fn perf_header__clear_feat(header: *mut perf_header, feat: c_int);
    pub fn perf_header__has_feat(header: *const perf_header, feat: c_int) -> bool;

    pub fn perf_header__set_cmdline(argc: c_int, argv: *mut *const c_char) -> c_int;

    pub fn perf_header__process_sections(
        header: *mut perf_header,
        fd: c_int,
        data: *mut c_void,
        process: Option<
            unsafe extern "C" fn(
                section: *mut perf_file_section,
                ph: *mut perf_header,
                feat: c_int,
                fd: c_int,
                data: *mut c_void,
            ) -> c_int,
        >,
    ) -> c_int;

    pub fn perf_header__fprintf_info(s: *mut perf_session, fp: *mut FILE, full: bool) -> c_int;

    pub fn perf_event__process_feature(
        tool: *const perf_tool,
        session: *mut perf_session,
        event: *mut perf_event,
    ) -> c_int;
    pub fn perf_event__process_attr(
        tool: *const perf_tool,
        event: *mut perf_event,
        pevlist: *mut *mut evlist,
    ) -> c_int;
    pub fn perf_event__process_event_update(
        tool: *const perf_tool,
        event: *mut perf_event,
        pevlist: *mut *mut evlist,
    ) -> c_int;
    pub fn perf_event__fprintf_attr(event: *mut perf_event, fp: *mut FILE) -> size_t;
    pub fn perf_event__fprintf_event_update(event: *mut perf_event, fp: *mut FILE) -> size_t;
}

/*
 * C conditional:
 * #ifdef HAVE_LIBTRACEEVENT
 */
unsafe extern "C" {
    pub fn perf_event__process_tracing_data(
        tool: *const perf_tool,
        session: *mut perf_session,
        event: *mut perf_event,
    ) -> c_int;
}

unsafe extern "C" {
    pub fn perf_event__process_build_id(
        tool: *const perf_tool,
        session: *mut perf_session,
        event: *mut perf_event,
    ) -> c_int;
    pub fn is_perf_magic(magic: u64) -> bool;
}

pub const NAME_ALIGN: usize = 64;

unsafe extern "C" {
    pub fn do_write(fd: *mut feat_fd, buf: *const c_void, size: size_t) -> c_int;

    pub fn write_padded(
        fd: *mut feat_fd,
        bf: *const c_void,
        count: size_t,
        count_aligned: size_t,
    ) -> c_int;
}

pub const MAX_CACHE_LVL: usize = 4;

unsafe extern "C" {
    pub fn build_caches_for_cpu(
        cpu: u32,
        caches: *mut cpu_cache_level,
        cntp: *mut u32,
    ) -> c_int;
}

pub const DEFAULT_CACHELINE_SIZE: usize = 64;

/*
 * arch specific callback
 */
unsafe extern "C" {
    pub fn get_cpuid(buffer: *mut c_char, sz: size_t, cpu: perf_cpu) -> c_int;

    pub fn get_cpuid_str(cpu: perf_cpu) -> *mut c_char;

    pub fn get_cpuid_allow_env_override(cpu: perf_cpu) -> *mut c_char;

    pub fn strcmp_cpuid_str(s1: *const c_char, s2: *const c_char) -> c_int;

    pub fn build_cpu_domain_map(
        schedstat_version: *mut u32,
        max_sched_domains: *mut u32,
        nr: u32,
    ) -> *mut *mut cpu_domain_map;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
