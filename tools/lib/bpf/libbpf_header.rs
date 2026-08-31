/* SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause) */

/*
 * Common eBPF ELF object loading operations.
 *
 * Rust translation of lib/bpf/libbpf.h. C include directives, header guards,
 * and C++ extern guards are represented by Rust FFI declarations below.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub type size_t = usize;
pub type pid_t = c_int;
pub type __u8 = u8;
pub type __u32 = u32;
pub type __s32 = i32;
pub type __u64 = u64;

/* Types supplied by included Linux/libbpf headers. */
pub type va_list = *mut c_void;
pub type bpf_attach_type = c_int;
pub type bpf_link_type = c_int;
pub type bpf_map_type = c_int;
pub type bpf_prog_type = c_int;
pub type bpf_func_id = c_int;

#[repr(C)]
pub struct bpf_object {
    _private: [u8; 0],
}
#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_insn {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_map {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_func_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_line_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_iter_link_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct perf_event_header {
    _private: [u8; 0],
}
#[repr(C)]
pub struct perf_event_attr {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_prog_linfo {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_prog_info {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_prog_assoc_struct_ops_opts {
    _private: [u8; 0],
}
#[repr(C)]
pub struct bpf_prog_load_opts {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum libbpf_errno {
    __LIBBPF_ERRNO__START = 4000,
    LIBBPF_ERRNO__LIBELF = 4000,
    LIBBPF_ERRNO__FORMAT = 4001,
    LIBBPF_ERRNO__KVERSION = 4002,
    LIBBPF_ERRNO__ENDIAN = 4003,
    LIBBPF_ERRNO__INTERNAL = 4004,
    LIBBPF_ERRNO__RELOC = 4005,
    LIBBPF_ERRNO__LOAD = 4006,
    LIBBPF_ERRNO__VERIFY = 4007,
    LIBBPF_ERRNO__PROG2BIG = 4008,
    LIBBPF_ERRNO__KVER = 4009,
    LIBBPF_ERRNO__PROGTYPE = 4010,
    LIBBPF_ERRNO__WRNGPID = 4011,
    LIBBPF_ERRNO__INVSEQ = 4012,
    LIBBPF_ERRNO__NLPARSE = 4013,
    __LIBBPF_ERRNO__END = 4014,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum libbpf_print_level {
    LIBBPF_WARN = 0,
    LIBBPF_INFO = 1,
    LIBBPF_DEBUG = 2,
}

pub type libbpf_print_fn_t =
    Option<unsafe extern "C" fn(level: libbpf_print_level, arg2: *const c_char, ap: va_list) -> c_int>;

#[repr(C)]
pub struct bpf_object_open_opts {
    pub sz: size_t,
    pub object_name: *const c_char,
    pub relaxed_maps: bool,
    pub pin_root_path: *const c_char,
    /* C bitfield padding: __u32 :32; removed attach_prog_fd */
    pub __bindgen_padding_0: __u32,
    pub kconfig: *const c_char,
    pub btf_custom_path: *const c_char,
    pub kernel_log_buf: *mut c_char,
    pub kernel_log_size: size_t,
    pub kernel_log_level: __u32,
    pub bpf_token_path: *const c_char,
}
pub const bpf_object_open_opts__last_field: &str = "bpf_token_path";

#[repr(C)]
pub struct bpf_perf_event_opts {
    pub sz: size_t,
    pub bpf_cookie: __u64,
    pub force_ioctl_attach: bool,
    pub dont_enable: bool,
}
pub const bpf_perf_event_opts__last_field: &str = "dont_enable";

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum probe_attach_mode {
    PROBE_ATTACH_MODE_DEFAULT = 0,
    PROBE_ATTACH_MODE_LEGACY = 1,
    PROBE_ATTACH_MODE_PERF = 2,
    PROBE_ATTACH_MODE_LINK = 3,
}

#[repr(C)]
pub struct bpf_kprobe_opts {
    pub sz: size_t,
    pub bpf_cookie: __u64,
    pub offset: size_t,
    pub retprobe: bool,
    pub attach_mode: probe_attach_mode,
}
pub const bpf_kprobe_opts__last_field: &str = "attach_mode";

#[repr(C)]
pub struct bpf_kprobe_multi_opts {
    pub sz: size_t,
    pub syms: *mut *const c_char,
    pub addrs: *const c_ulong,
    pub cookies: *const __u64,
    pub cnt: size_t,
    pub retprobe: bool,
    pub session: bool,
    pub unique_match: bool,
}
pub const bpf_kprobe_multi_opts__last_field: &str = "unique_match";

#[repr(C)]
pub struct bpf_uprobe_multi_opts {
    pub sz: size_t,
    pub syms: *mut *const c_char,
    pub offsets: *const c_ulong,
    pub ref_ctr_offsets: *const c_ulong,
    pub cookies: *const __u64,
    pub cnt: size_t,
    pub retprobe: bool,
    pub session: bool,
}
pub const bpf_uprobe_multi_opts__last_field: &str = "session";

#[repr(C)]
pub struct bpf_ksyscall_opts {
    pub sz: size_t,
    pub bpf_cookie: __u64,
    pub retprobe: bool,
}
pub const bpf_ksyscall_opts__last_field: &str = "retprobe";

#[repr(C)]
pub struct bpf_tracing_multi_opts {
    pub sz: size_t,
    pub ids: *const __u32,
    pub cookies: *const __u64,
    pub cnt: size_t,
}
pub const bpf_tracing_multi_opts__last_field: &str = "cnt";

#[repr(C)]
pub struct bpf_uprobe_opts {
    pub sz: size_t,
    pub ref_ctr_offset: size_t,
    pub bpf_cookie: __u64,
    pub retprobe: bool,
    pub func_name: *const c_char,
    pub attach_mode: probe_attach_mode,
}
pub const bpf_uprobe_opts__last_field: &str = "attach_mode";

#[repr(C)]
pub struct bpf_usdt_opts {
    pub sz: size_t,
    pub usdt_cookie: __u64,
}
pub const bpf_usdt_opts__last_field: &str = "usdt_cookie";

#[repr(C)]
pub struct bpf_tracepoint_opts {
    pub sz: size_t,
    pub bpf_cookie: __u64,
}
pub const bpf_tracepoint_opts__last_field: &str = "bpf_cookie";

#[repr(C)]
pub struct bpf_raw_tracepoint_opts {
    pub sz: size_t,
    pub cookie: __u64,
}
pub const bpf_raw_tracepoint_opts__last_field: &str = "cookie";

#[repr(C)]
pub struct bpf_trace_opts {
    pub sz: size_t,
    pub cookie: __u64,
}
pub const bpf_trace_opts__last_field: &str = "cookie";

#[repr(C)]
pub struct bpf_netfilter_opts {
    pub sz: size_t,
    pub pf: __u32,
    pub hooknum: __u32,
    pub priority: __s32,
    pub flags: __u32,
}
pub const bpf_netfilter_opts__last_field: &str = "flags";

#[repr(C)]
pub struct bpf_tcx_opts {
    pub sz: size_t,
    pub flags: __u32,
    pub relative_fd: __u32,
    pub relative_id: __u32,
    pub expected_revision: __u64,
}
pub const bpf_tcx_opts__last_field: &str = "expected_revision";

#[repr(C)]
pub struct bpf_netkit_opts {
    pub sz: size_t,
    pub flags: __u32,
    pub relative_fd: __u32,
    pub relative_id: __u32,
    pub expected_revision: __u64,
}
pub const bpf_netkit_opts__last_field: &str = "expected_revision";

#[repr(C)]
pub struct bpf_cgroup_opts {
    pub sz: size_t,
    pub flags: __u32,
    pub relative_fd: __u32,
    pub relative_id: __u32,
    pub expected_revision: __u64,
}
pub const bpf_cgroup_opts__last_field: &str = "expected_revision";

#[repr(C)]
pub struct bpf_iter_attach_opts {
    pub sz: size_t,
    pub link_info: *mut bpf_iter_link_info,
    pub link_info_len: __u32,
}
pub const bpf_iter_attach_opts__last_field: &str = "link_info_len";

#[repr(C)]
pub struct bpf_xdp_set_link_opts {
    pub sz: size_t,
    pub old_fd: c_int,
}
pub const bpf_xdp_set_link_opts__last_field: &str = "old_fd";

#[repr(C)]
pub struct bpf_xdp_attach_opts {
    pub sz: size_t,
    pub old_prog_fd: c_int,
}
pub const bpf_xdp_attach_opts__last_field: &str = "old_prog_fd";

#[repr(C)]
pub struct bpf_xdp_query_opts {
    pub sz: size_t,
    pub prog_id: __u32,
    pub drv_prog_id: __u32,
    pub hw_prog_id: __u32,
    pub skb_prog_id: __u32,
    pub attach_mode: __u8,
    pub feature_flags: __u64,
    pub xdp_zc_max_segs: __u32,
}
pub const bpf_xdp_query_opts__last_field: &str = "xdp_zc_max_segs";

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bpf_tc_attach_point {
    BPF_TC_INGRESS = 1 << 0,
    BPF_TC_EGRESS = 1 << 1,
    BPF_TC_CUSTOM = 1 << 2,
    BPF_TC_QDISC = 1 << 3,
}

pub const fn BPF_TC_PARENT(a: __u32, b: __u32) -> __u32 {
    (((a << 16) & 0xFFFF0000u32) | (b & 0x0000FFFFu32))
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bpf_tc_flags {
    BPF_TC_F_REPLACE = 1 << 0,
}

#[repr(C)]
pub struct bpf_tc_hook {
    pub sz: size_t,
    pub ifindex: c_int,
    pub attach_point: bpf_tc_attach_point,
    pub parent: __u32,
    pub handle: __u32,
    pub qdisc: *const c_char,
}
pub const bpf_tc_hook__last_field: &str = "qdisc";

#[repr(C)]
pub struct bpf_tc_opts {
    pub sz: size_t,
    pub prog_fd: c_int,
    pub flags: __u32,
    pub prog_id: __u32,
    pub handle: __u32,
    pub priority: __u32,
}
pub const bpf_tc_opts__last_field: &str = "priority";

#[repr(C)]
pub struct ring_buffer {
    _private: [u8; 0],
}
#[repr(C)]
pub struct ring {
    _private: [u8; 0],
}
#[repr(C)]
pub struct user_ring_buffer {
    _private: [u8; 0],
}
pub type ring_buffer_sample_fn =
    Option<unsafe extern "C" fn(ctx: *mut c_void, data: *mut c_void, size: size_t) -> c_int>;

#[repr(C)]
pub struct ring_buffer_opts {
    pub sz: size_t,
}
pub const ring_buffer_opts__last_field: &str = "sz";

#[repr(C)]
pub struct user_ring_buffer_opts {
    pub sz: size_t,
}
pub const user_ring_buffer_opts__last_field: &str = "sz";

#[repr(C)]
pub struct perf_buffer {
    _private: [u8; 0],
}
pub type perf_buffer_sample_fn =
    Option<unsafe extern "C" fn(ctx: *mut c_void, cpu: c_int, data: *mut c_void, size: __u32)>;
pub type perf_buffer_lost_fn =
    Option<unsafe extern "C" fn(ctx: *mut c_void, cpu: c_int, cnt: __u64)>;

#[repr(C)]
pub struct perf_buffer_opts {
    pub sz: size_t,
    pub sample_period: __u32,
}
pub const perf_buffer_opts__last_field: &str = "sample_period";

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum bpf_perf_event_ret {
    LIBBPF_PERF_EVENT_DONE = 0,
    LIBBPF_PERF_EVENT_ERROR = -1,
    LIBBPF_PERF_EVENT_CONT = -2,
}
pub type perf_buffer_event_fn = Option<
    unsafe extern "C" fn(ctx: *mut c_void, cpu: c_int, event: *mut perf_event_header) -> bpf_perf_event_ret,
>;

#[repr(C)]
pub struct perf_buffer_raw_opts {
    pub sz: size_t,
    /* C has two unnamed long zero-width bitfields here. */
    pub cpu_cnt: c_int,
    pub cpus: *mut c_int,
    pub map_keys: *mut c_int,
}
pub const perf_buffer_raw_opts__last_field: &str = "map_keys";

#[repr(C)]
pub struct bpf_map_skeleton {
    pub name: *const c_char,
    pub map: *mut *mut bpf_map,
    pub mmaped: *mut *mut c_void,
    pub link: *mut *mut bpf_link,
}

#[repr(C)]
pub struct bpf_prog_skeleton {
    pub name: *const c_char,
    pub prog: *mut *mut bpf_program,
    pub link: *mut *mut bpf_link,
}

#[repr(C)]
pub struct bpf_object_skeleton {
    pub sz: size_t,
    pub name: *const c_char,
    pub data: *const c_void,
    pub data_sz: size_t,
    pub obj: *mut *mut bpf_object,
    pub map_cnt: c_int,
    pub map_skel_sz: c_int,
    pub maps: *mut bpf_map_skeleton,
    pub prog_cnt: c_int,
    pub prog_skel_sz: c_int,
    pub progs: *mut bpf_prog_skeleton,
}

#[repr(C)]
pub struct bpf_var_skeleton {
    pub name: *const c_char,
    pub map: *mut *mut bpf_map,
    pub addr: *mut *mut c_void,
}

#[repr(C)]
pub struct bpf_object_subskeleton {
    pub sz: size_t,
    pub obj: *const bpf_object,
    pub map_cnt: c_int,
    pub map_skel_sz: c_int,
    pub maps: *mut bpf_map_skeleton,
    pub prog_cnt: c_int,
    pub prog_skel_sz: c_int,
    pub progs: *mut bpf_prog_skeleton,
    pub var_cnt: c_int,
    pub var_skel_sz: c_int,
    pub vars: *mut bpf_var_skeleton,
}

#[repr(C)]
pub struct gen_loader_opts {
    pub sz: size_t,
    pub data: *const c_char,
    pub insns: *const c_char,
    pub data_sz: __u32,
    pub insns_sz: __u32,
    pub gen_hash: bool,
}
pub const gen_loader_opts__last_field: &str = "gen_hash";

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum libbpf_tristate {
    TRI_NO = 0,
    TRI_YES = 1,
    TRI_MODULE = 2,
}

#[repr(C)]
pub struct bpf_linker_opts {
    pub sz: size_t,
}
pub const bpf_linker_opts__last_field: &str = "sz";

#[repr(C)]
pub struct bpf_linker_file_opts {
    pub sz: size_t,
}
pub const bpf_linker_file_opts__last_field: &str = "sz";

#[repr(C)]
pub struct bpf_linker {
    _private: [u8; 0],
}

pub type libbpf_prog_setup_fn_t =
    Option<unsafe extern "C" fn(prog: *mut bpf_program, cookie: c_long) -> c_int>;
pub type libbpf_prog_prepare_load_fn_t = Option<
    unsafe extern "C" fn(prog: *mut bpf_program, opts: *mut bpf_prog_load_opts, cookie: c_long) -> c_int,
>;
pub type libbpf_prog_attach_fn_t = Option<
    unsafe extern "C" fn(prog: *const bpf_program, cookie: c_long, link: *mut *mut bpf_link) -> c_int,
>;

#[repr(C)]
pub struct libbpf_prog_handler_opts {
    pub sz: size_t,
    pub cookie: c_long,
    pub prog_setup_fn: libbpf_prog_setup_fn_t,
    pub prog_prepare_load_fn: libbpf_prog_prepare_load_fn_t,
    pub prog_attach_fn: libbpf_prog_attach_fn_t,
}
pub const libbpf_prog_handler_opts__last_field: &str = "prog_attach_fn";

unsafe extern "C" {
    pub fn libbpf_major_version() -> __u32;
    pub fn libbpf_minor_version() -> __u32;
    pub fn libbpf_version_string() -> *const c_char;
    pub fn libbpf_strerror(err: c_int, buf: *mut c_char, size: size_t) -> c_int;
    pub fn libbpf_bpf_attach_type_str(t: bpf_attach_type) -> *const c_char;
    pub fn libbpf_bpf_link_type_str(t: bpf_link_type) -> *const c_char;
    pub fn libbpf_bpf_map_type_str(t: bpf_map_type) -> *const c_char;
    pub fn libbpf_bpf_prog_type_str(t: bpf_prog_type) -> *const c_char;
    pub fn libbpf_set_print(fn_: libbpf_print_fn_t) -> libbpf_print_fn_t;

    pub fn bpf_object__open(path: *const c_char) -> *mut bpf_object;
    pub fn bpf_object__open_file(path: *const c_char, opts: *const bpf_object_open_opts) -> *mut bpf_object;
    pub fn bpf_object__open_mem(obj_buf: *const c_void, obj_buf_sz: size_t, opts: *const bpf_object_open_opts) -> *mut bpf_object;
    pub fn bpf_object__prepare(obj: *mut bpf_object) -> c_int;
    pub fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    pub fn bpf_object__close(obj: *mut bpf_object);
    pub fn bpf_object__pin_maps(obj: *mut bpf_object, path: *const c_char) -> c_int;
    pub fn bpf_object__unpin_maps(obj: *mut bpf_object, path: *const c_char) -> c_int;
    pub fn bpf_object__pin_programs(obj: *mut bpf_object, path: *const c_char) -> c_int;
    pub fn bpf_object__unpin_programs(obj: *mut bpf_object, path: *const c_char) -> c_int;
    pub fn bpf_object__pin(object: *mut bpf_object, path: *const c_char) -> c_int;
    pub fn bpf_object__unpin(object: *mut bpf_object, path: *const c_char) -> c_int;
    pub fn bpf_object__name(obj: *const bpf_object) -> *const c_char;
    pub fn bpf_object__kversion(obj: *const bpf_object) -> c_uint;
    pub fn bpf_object__set_kversion(obj: *mut bpf_object, kern_version: __u32) -> c_int;
    pub fn bpf_object__token_fd(obj: *const bpf_object) -> c_int;
    pub fn bpf_object__btf(obj: *const bpf_object) -> *mut btf;
    pub fn bpf_object__btf_fd(obj: *const bpf_object) -> c_int;
    pub fn bpf_object__find_program_by_name(obj: *const bpf_object, name: *const c_char) -> *mut bpf_program;
    pub fn libbpf_prog_type_by_name(name: *const c_char, prog_type: *mut bpf_prog_type, expected_attach_type: *mut bpf_attach_type) -> c_int;
    pub fn libbpf_attach_type_by_name(name: *const c_char, attach_type: *mut bpf_attach_type) -> c_int;
    pub fn libbpf_find_vmlinux_btf_id(name: *const c_char, attach_type: bpf_attach_type) -> c_int;
    pub fn bpf_object__next_program(obj: *const bpf_object, prog: *mut bpf_program) -> *mut bpf_program;
    pub fn bpf_object__prev_program(obj: *const bpf_object, prog: *mut bpf_program) -> *mut bpf_program;
    pub fn bpf_program__set_ifindex(prog: *mut bpf_program, ifindex: __u32);
    pub fn bpf_program__name(prog: *const bpf_program) -> *const c_char;
    pub fn bpf_program__section_name(prog: *const bpf_program) -> *const c_char;
    pub fn bpf_program__autoload(prog: *const bpf_program) -> bool;
    pub fn bpf_program__set_autoload(prog: *mut bpf_program, autoload: bool) -> c_int;
    pub fn bpf_program__autoattach(prog: *const bpf_program) -> bool;
    pub fn bpf_program__set_autoattach(prog: *mut bpf_program, autoattach: bool);
    pub fn bpf_program__insns(prog: *const bpf_program) -> *const bpf_insn;
    pub fn bpf_program__set_insns(prog: *mut bpf_program, new_insns: *mut bpf_insn, new_insn_cnt: size_t) -> c_int;
    pub fn bpf_program__insn_cnt(prog: *const bpf_program) -> size_t;
    pub fn bpf_program__fd(prog: *const bpf_program) -> c_int;
    pub fn bpf_program__pin(prog: *mut bpf_program, path: *const c_char) -> c_int;
    pub fn bpf_program__unpin(prog: *mut bpf_program, path: *const c_char) -> c_int;
    pub fn bpf_program__unload(prog: *mut bpf_program);

    pub fn bpf_link__open(path: *const c_char) -> *mut bpf_link;
    pub fn bpf_link__fd(link: *const bpf_link) -> c_int;
    pub fn bpf_link__pin_path(link: *const bpf_link) -> *const c_char;
    pub fn bpf_link__pin(link: *mut bpf_link, path: *const c_char) -> c_int;
    pub fn bpf_link__unpin(link: *mut bpf_link) -> c_int;
    pub fn bpf_link__update_program(link: *mut bpf_link, prog: *mut bpf_program) -> c_int;
    pub fn bpf_link__disconnect(link: *mut bpf_link);
    pub fn bpf_link__detach(link: *mut bpf_link) -> c_int;
    pub fn bpf_link__destroy(link: *mut bpf_link) -> c_int;

    pub fn bpf_program__attach(prog: *const bpf_program) -> *mut bpf_link;
    pub fn bpf_program__attach_perf_event(prog: *const bpf_program, pfd: c_int) -> *mut bpf_link;
    pub fn bpf_program__attach_perf_event_opts(prog: *const bpf_program, pfd: c_int, opts: *const bpf_perf_event_opts) -> *mut bpf_link;
    pub fn bpf_program__attach_kprobe(prog: *const bpf_program, retprobe: bool, func_name: *const c_char) -> *mut bpf_link;
    pub fn bpf_program__attach_kprobe_opts(prog: *const bpf_program, func_name: *const c_char, opts: *const bpf_kprobe_opts) -> *mut bpf_link;
    pub fn bpf_program__attach_kprobe_multi_opts(prog: *const bpf_program, pattern: *const c_char, opts: *const bpf_kprobe_multi_opts) -> *mut bpf_link;
    pub fn bpf_program__attach_uprobe_multi(prog: *const bpf_program, pid: pid_t, binary_path: *const c_char, func_pattern: *const c_char, opts: *const bpf_uprobe_multi_opts) -> *mut bpf_link;
    pub fn bpf_program__attach_ksyscall(prog: *const bpf_program, syscall_name: *const c_char, opts: *const bpf_ksyscall_opts) -> *mut bpf_link;
    pub fn bpf_program__attach_tracing_multi(prog: *const bpf_program, pattern: *const c_char, opts: *const bpf_tracing_multi_opts) -> *mut bpf_link;
    pub fn bpf_program__attach_uprobe(prog: *const bpf_program, retprobe: bool, pid: pid_t, binary_path: *const c_char, func_offset: size_t) -> *mut bpf_link;
    pub fn bpf_program__attach_uprobe_opts(prog: *const bpf_program, pid: pid_t, binary_path: *const c_char, func_offset: size_t, opts: *const bpf_uprobe_opts) -> *mut bpf_link;
    pub fn bpf_program__attach_usdt(prog: *const bpf_program, pid: pid_t, binary_path: *const c_char, usdt_provider: *const c_char, usdt_name: *const c_char, opts: *const bpf_usdt_opts) -> *mut bpf_link;
    pub fn bpf_program__attach_tracepoint(prog: *const bpf_program, tp_category: *const c_char, tp_name: *const c_char) -> *mut bpf_link;
    pub fn bpf_program__attach_tracepoint_opts(prog: *const bpf_program, tp_category: *const c_char, tp_name: *const c_char, opts: *const bpf_tracepoint_opts) -> *mut bpf_link;
    pub fn bpf_program__attach_raw_tracepoint(prog: *const bpf_program, tp_name: *const c_char) -> *mut bpf_link;
    pub fn bpf_program__attach_raw_tracepoint_opts(prog: *const bpf_program, tp_name: *const c_char, opts: *mut bpf_raw_tracepoint_opts) -> *mut bpf_link;
    pub fn bpf_program__attach_trace(prog: *const bpf_program) -> *mut bpf_link;
    pub fn bpf_program__attach_trace_opts(prog: *const bpf_program, opts: *const bpf_trace_opts) -> *mut bpf_link;
    pub fn bpf_program__attach_lsm(prog: *const bpf_program) -> *mut bpf_link;
    pub fn bpf_program__attach_cgroup(prog: *const bpf_program, cgroup_fd: c_int) -> *mut bpf_link;
    pub fn bpf_program__attach_netns(prog: *const bpf_program, netns_fd: c_int) -> *mut bpf_link;
    pub fn bpf_program__attach_sockmap(prog: *const bpf_program, map_fd: c_int) -> *mut bpf_link;
    pub fn bpf_program__attach_xdp(prog: *const bpf_program, ifindex: c_int) -> *mut bpf_link;
    pub fn bpf_program__attach_freplace(prog: *const bpf_program, target_fd: c_int, attach_func_name: *const c_char) -> *mut bpf_link;
    pub fn bpf_program__attach_netfilter(prog: *const bpf_program, opts: *const bpf_netfilter_opts) -> *mut bpf_link;
    pub fn bpf_program__attach_tcx(prog: *const bpf_program, ifindex: c_int, opts: *const bpf_tcx_opts) -> *mut bpf_link;
    pub fn bpf_program__attach_netkit(prog: *const bpf_program, ifindex: c_int, opts: *const bpf_netkit_opts) -> *mut bpf_link;
    pub fn bpf_program__attach_cgroup_opts(prog: *const bpf_program, cgroup_fd: c_int, opts: *const bpf_cgroup_opts) -> *mut bpf_link;
    pub fn bpf_map__attach_struct_ops(map: *const bpf_map) -> *mut bpf_link;
    pub fn bpf_link__update_map(link: *mut bpf_link, map: *const bpf_map) -> c_int;
    pub fn bpf_program__attach_iter(prog: *const bpf_program, opts: *const bpf_iter_attach_opts) -> *mut bpf_link;

    pub fn bpf_program__type(prog: *const bpf_program) -> bpf_prog_type;
    pub fn bpf_program__set_type(prog: *mut bpf_program, type_: bpf_prog_type) -> c_int;
    pub fn bpf_program__expected_attach_type(prog: *const bpf_program) -> bpf_attach_type;
    pub fn bpf_program__set_expected_attach_type(prog: *mut bpf_program, type_: bpf_attach_type) -> c_int;
    pub fn bpf_program__flags(prog: *const bpf_program) -> __u32;
    pub fn bpf_program__set_flags(prog: *mut bpf_program, flags: __u32) -> c_int;
    pub fn bpf_program__log_level(prog: *const bpf_program) -> __u32;
    pub fn bpf_program__set_log_level(prog: *mut bpf_program, log_level: __u32) -> c_int;
    pub fn bpf_program__log_buf(prog: *const bpf_program, log_size: *mut size_t) -> *const c_char;
    pub fn bpf_program__set_log_buf(prog: *mut bpf_program, log_buf: *mut c_char, log_size: size_t) -> c_int;
    pub fn bpf_program__func_info(prog: *const bpf_program) -> *mut bpf_func_info;
    pub fn bpf_program__func_info_cnt(prog: *const bpf_program) -> __u32;
    pub fn bpf_program__line_info(prog: *const bpf_program) -> *mut bpf_line_info;
    pub fn bpf_program__line_info_cnt(prog: *const bpf_program) -> __u32;
    pub fn bpf_program__set_attach_target(prog: *mut bpf_program, attach_prog_fd: c_int, attach_func_name: *const c_char) -> c_int;
    pub fn bpf_program__assoc_struct_ops(prog: *mut bpf_program, map: *mut bpf_map, opts: *mut bpf_prog_assoc_struct_ops_opts) -> c_int;

    pub fn bpf_object__find_map_by_name(obj: *const bpf_object, name: *const c_char) -> *mut bpf_map;
    pub fn bpf_object__find_map_fd_by_name(obj: *const bpf_object, name: *const c_char) -> c_int;
    pub fn bpf_object__next_map(obj: *const bpf_object, map: *const bpf_map) -> *mut bpf_map;
    pub fn bpf_object__prev_map(obj: *const bpf_object, map: *const bpf_map) -> *mut bpf_map;
    pub fn bpf_map__set_autocreate(map: *mut bpf_map, autocreate: bool) -> c_int;
    pub fn bpf_map__autocreate(map: *const bpf_map) -> bool;
    pub fn bpf_map__set_autoattach(map: *mut bpf_map, autoattach: bool) -> c_int;
    pub fn bpf_map__autoattach(map: *const bpf_map) -> bool;
    pub fn bpf_map__fd(map: *const bpf_map) -> c_int;
    pub fn bpf_map__reuse_fd(map: *mut bpf_map, fd: c_int) -> c_int;
    pub fn bpf_map__name(map: *const bpf_map) -> *const c_char;
    pub fn bpf_map__type(map: *const bpf_map) -> bpf_map_type;
    pub fn bpf_map__set_type(map: *mut bpf_map, type_: bpf_map_type) -> c_int;
    pub fn bpf_map__max_entries(map: *const bpf_map) -> __u32;
    pub fn bpf_map__set_max_entries(map: *mut bpf_map, max_entries: __u32) -> c_int;
    pub fn bpf_map__map_flags(map: *const bpf_map) -> __u32;
    pub fn bpf_map__set_map_flags(map: *mut bpf_map, flags: __u32) -> c_int;
    pub fn bpf_map__numa_node(map: *const bpf_map) -> __u32;
    pub fn bpf_map__set_numa_node(map: *mut bpf_map, numa_node: __u32) -> c_int;
    pub fn bpf_map__key_size(map: *const bpf_map) -> __u32;
    pub fn bpf_map__set_key_size(map: *mut bpf_map, size: __u32) -> c_int;
    pub fn bpf_map__value_size(map: *const bpf_map) -> __u32;
    pub fn bpf_map__set_value_size(map: *mut bpf_map, size: __u32) -> c_int;
    pub fn bpf_map__btf_key_type_id(map: *const bpf_map) -> __u32;
    pub fn bpf_map__btf_value_type_id(map: *const bpf_map) -> __u32;
    pub fn bpf_map__ifindex(map: *const bpf_map) -> __u32;
    pub fn bpf_map__set_ifindex(map: *mut bpf_map, ifindex: __u32) -> c_int;
    pub fn bpf_map__map_extra(map: *const bpf_map) -> __u64;
    pub fn bpf_map__set_map_extra(map: *mut bpf_map, map_extra: __u64) -> c_int;
    pub fn bpf_map__set_initial_value(map: *mut bpf_map, data: *const c_void, size: size_t) -> c_int;
    pub fn bpf_map__initial_value(map: *const bpf_map, psize: *mut size_t) -> *mut c_void;
    pub fn bpf_map__is_internal(map: *const bpf_map) -> bool;
    pub fn bpf_map__set_pin_path(map: *mut bpf_map, path: *const c_char) -> c_int;
    pub fn bpf_map__pin_path(map: *const bpf_map) -> *const c_char;
    pub fn bpf_map__is_pinned(map: *const bpf_map) -> bool;
    pub fn bpf_map__pin(map: *mut bpf_map, path: *const c_char) -> c_int;
    pub fn bpf_map__unpin(map: *mut bpf_map, path: *const c_char) -> c_int;
    pub fn bpf_map__set_inner_map_fd(map: *mut bpf_map, fd: c_int) -> c_int;
    pub fn bpf_map__inner_map(map: *mut bpf_map) -> *mut bpf_map;
    pub fn bpf_map__lookup_elem(map: *const bpf_map, key: *const c_void, key_sz: size_t, value: *mut c_void, value_sz: size_t, flags: __u64) -> c_int;
    pub fn bpf_map__update_elem(map: *const bpf_map, key: *const c_void, key_sz: size_t, value: *const c_void, value_sz: size_t, flags: __u64) -> c_int;
    pub fn bpf_map__delete_elem(map: *const bpf_map, key: *const c_void, key_sz: size_t, flags: __u64) -> c_int;
    pub fn bpf_map__lookup_and_delete_elem(map: *const bpf_map, key: *const c_void, key_sz: size_t, value: *mut c_void, value_sz: size_t, flags: __u64) -> c_int;
    pub fn bpf_map__get_next_key(map: *const bpf_map, cur_key: *const c_void, next_key: *mut c_void, key_sz: size_t) -> c_int;
    pub fn bpf_map__set_exclusive_program(map: *mut bpf_map, prog: *mut bpf_program) -> c_int;
    pub fn bpf_map__exclusive_program(map: *mut bpf_map) -> *mut bpf_program;

    pub fn bpf_xdp_attach(ifindex: c_int, prog_fd: c_int, flags: __u32, opts: *const bpf_xdp_attach_opts) -> c_int;
    pub fn bpf_xdp_detach(ifindex: c_int, flags: __u32, opts: *const bpf_xdp_attach_opts) -> c_int;
    pub fn bpf_xdp_query(ifindex: c_int, flags: c_int, opts: *mut bpf_xdp_query_opts) -> c_int;
    pub fn bpf_xdp_query_id(ifindex: c_int, flags: c_int, prog_id: *mut __u32) -> c_int;
    pub fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    pub fn bpf_tc_hook_destroy(hook: *mut bpf_tc_hook) -> c_int;
    pub fn bpf_tc_attach(hook: *const bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    pub fn bpf_tc_detach(hook: *const bpf_tc_hook, opts: *const bpf_tc_opts) -> c_int;
    pub fn bpf_tc_query(hook: *const bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;

    pub fn ring_buffer__new(map_fd: c_int, sample_cb: ring_buffer_sample_fn, ctx: *mut c_void, opts: *const ring_buffer_opts) -> *mut ring_buffer;
    pub fn ring_buffer__free(rb: *mut ring_buffer);
    pub fn ring_buffer__add(rb: *mut ring_buffer, map_fd: c_int, sample_cb: ring_buffer_sample_fn, ctx: *mut c_void) -> c_int;
    pub fn ring_buffer__poll(rb: *mut ring_buffer, timeout_ms: c_int) -> c_int;
    pub fn ring_buffer__consume(rb: *mut ring_buffer) -> c_int;
    pub fn ring_buffer__consume_n(rb: *mut ring_buffer, n: size_t) -> c_int;
    pub fn ring_buffer__epoll_fd(rb: *const ring_buffer) -> c_int;
    pub fn ring_buffer__ring(rb: *mut ring_buffer, idx: c_uint) -> *mut ring;
    pub fn ring__consumer_pos(r: *const ring) -> c_ulong;
    pub fn ring__producer_pos(r: *const ring) -> c_ulong;
    pub fn ring__avail_data_size(r: *const ring) -> size_t;
    pub fn ring__size(r: *const ring) -> size_t;
    pub fn ring__map_fd(r: *const ring) -> c_int;
    pub fn ring__consume(r: *mut ring) -> c_int;
    pub fn ring__consume_n(r: *mut ring, n: size_t) -> c_int;
    pub fn user_ring_buffer__new(map_fd: c_int, opts: *const user_ring_buffer_opts) -> *mut user_ring_buffer;
    pub fn user_ring_buffer__reserve(rb: *mut user_ring_buffer, size: __u32) -> *mut c_void;
    pub fn user_ring_buffer__reserve_blocking(rb: *mut user_ring_buffer, size: __u32, timeout_ms: c_int) -> *mut c_void;
    pub fn user_ring_buffer__submit(rb: *mut user_ring_buffer, sample: *mut c_void);
    pub fn user_ring_buffer__discard(rb: *mut user_ring_buffer, sample: *mut c_void);
    pub fn user_ring_buffer__free(rb: *mut user_ring_buffer);

    pub fn perf_buffer__new(map_fd: c_int, page_cnt: size_t, sample_cb: perf_buffer_sample_fn, lost_cb: perf_buffer_lost_fn, ctx: *mut c_void, opts: *const perf_buffer_opts) -> *mut perf_buffer;
    pub fn perf_buffer__new_raw(map_fd: c_int, page_cnt: size_t, attr: *mut perf_event_attr, event_cb: perf_buffer_event_fn, ctx: *mut c_void, opts: *const perf_buffer_raw_opts) -> *mut perf_buffer;
    pub fn perf_buffer__free(pb: *mut perf_buffer);
    pub fn perf_buffer__epoll_fd(pb: *const perf_buffer) -> c_int;
    pub fn perf_buffer__poll(pb: *mut perf_buffer, timeout_ms: c_int) -> c_int;
    pub fn perf_buffer__consume(pb: *mut perf_buffer) -> c_int;
    pub fn perf_buffer__consume_buffer(pb: *mut perf_buffer, buf_idx: size_t) -> c_int;
    pub fn perf_buffer__buffer_cnt(pb: *const perf_buffer) -> size_t;
    pub fn perf_buffer__buffer_fd(pb: *const perf_buffer, buf_idx: size_t) -> c_int;
    pub fn perf_buffer__buffer(pb: *mut perf_buffer, buf_idx: c_int, buf: *mut *mut c_void, buf_size: *mut size_t) -> c_int;

    pub fn bpf_prog_linfo__free(prog_linfo: *mut bpf_prog_linfo);
    pub fn bpf_prog_linfo__new(info: *const bpf_prog_info) -> *mut bpf_prog_linfo;
    pub fn bpf_prog_linfo__lfind_addr_func(prog_linfo: *const bpf_prog_linfo, addr: __u64, func_idx: __u32, nr_skip: __u32) -> *const bpf_line_info;
    pub fn bpf_prog_linfo__lfind(prog_linfo: *const bpf_prog_linfo, insn_off: __u32, nr_skip: __u32) -> *const bpf_line_info;
    pub fn libbpf_probe_bpf_prog_type(prog_type: bpf_prog_type, opts: *const c_void) -> c_int;
    pub fn libbpf_probe_bpf_map_type(map_type: bpf_map_type, opts: *const c_void) -> c_int;
    pub fn libbpf_probe_bpf_helper(prog_type: bpf_prog_type, helper_id: bpf_func_id, opts: *const c_void) -> c_int;
    pub fn libbpf_num_possible_cpus() -> c_int;

    pub fn bpf_object__open_skeleton(s: *mut bpf_object_skeleton, opts: *const bpf_object_open_opts) -> c_int;
    pub fn bpf_object__load_skeleton(s: *mut bpf_object_skeleton) -> c_int;
    pub fn bpf_object__attach_skeleton(s: *mut bpf_object_skeleton) -> c_int;
    pub fn bpf_object__detach_skeleton(s: *mut bpf_object_skeleton);
    pub fn bpf_object__destroy_skeleton(s: *mut bpf_object_skeleton);
    pub fn bpf_object__open_subskeleton(s: *mut bpf_object_subskeleton) -> c_int;
    pub fn bpf_object__destroy_subskeleton(s: *mut bpf_object_subskeleton);
    pub fn bpf_object__gen_loader(obj: *mut bpf_object, opts: *mut gen_loader_opts) -> c_int;

    pub fn bpf_linker__new(filename: *const c_char, opts: *mut bpf_linker_opts) -> *mut bpf_linker;
    pub fn bpf_linker__new_fd(fd: c_int, opts: *mut bpf_linker_opts) -> *mut bpf_linker;
    pub fn bpf_linker__add_file(linker: *mut bpf_linker, filename: *const c_char, opts: *const bpf_linker_file_opts) -> c_int;
    pub fn bpf_linker__add_fd(linker: *mut bpf_linker, fd: c_int, opts: *const bpf_linker_file_opts) -> c_int;
    pub fn bpf_linker__add_buf(linker: *mut bpf_linker, buf: *mut c_void, buf_sz: size_t, opts: *const bpf_linker_file_opts) -> c_int;
    pub fn bpf_linker__finalize(linker: *mut bpf_linker) -> c_int;
    pub fn bpf_linker__free(linker: *mut bpf_linker);

    pub fn libbpf_register_prog_handler(sec: *const c_char, prog_type: bpf_prog_type, exp_attach_type: bpf_attach_type, opts: *const libbpf_prog_handler_opts) -> c_int;
    pub fn libbpf_unregister_prog_handler(handler_id: c_int) -> c_int;
    pub fn bpf_program__clone(prog: *mut bpf_program, opts: *const bpf_prog_load_opts) -> c_int;
}

/*
 * C statement macros preserved by intent:
 * - bpf_object__for_each_program(pos, obj): iterates with bpf_object__next_program().
 * - bpf_object__for_each_map(pos, obj): iterates with bpf_object__next_map().
 * - bpf_map__for_each aliases bpf_object__for_each_map.
 */
