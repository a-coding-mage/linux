// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (c) 2019 Netronome Systems, Inc. */

/* Translated from bpf/bpftool/feature.c.  Includes are represented by the
 * external declarations and constants below; symbols supplied by bpftool,
 * libbpf, libc, and Linux UAPI headers remain external dependencies.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type __u32 = u32;
type FILE = c_void;
type cap_t = *mut c_void;
type cap_value_t = c_int;
type cap_flag_value_t = c_int;

const PROC_SUPER_MAGIC: c_ulong = 0x9fa0;
const IF_NAMESIZE: usize = 16;
const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2000000;
const ENOSYS: c_int = 38;
const EINVAL: c_int = 22;
const EOPNOTSUPP: c_int = 95;
const EPERM: c_int = 1;
const BPF_MAXINSNS: usize = 4096;

const CAP_EFFECTIVE: c_int = 0;
const CAP_SET: cap_flag_value_t = 1;
const CAP_CLEAR: c_int = 0;
const CAP_SYS_ADMIN: cap_value_t = 21;
const CAP_BPF: cap_value_t = 39;
const CAP_NET_ADMIN: cap_value_t = 12;
const CAP_PERFMON: cap_value_t = 38;

#[repr(C)]
struct statfs {
    f_type: c_long,
    _rest: [u8; 120],
}

#[repr(C)]
struct kernel_config_option {
    name: *const c_char,
    macro_dump: bool,
}

#[repr(C)]
struct cmd {
    cmd: *const c_char,
    func: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct bpf_insn {
    code: u8,
    dst_src: u8,
    off: i16,
    imm: i32,
}

#[repr(C)]
struct bpf_prog_load_opts {
    sz: size_t,
    log_buf: *mut c_char,
    log_size: u32,
    log_level: u32,
    prog_ifindex: __u32,
}

#[repr(C)]
struct bpf_map_create_opts {
    sz: size_t,
    map_ifindex: __u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum probe_component {
    COMPONENT_UNSPEC,
    COMPONENT_KERNEL,
    COMPONENT_DEVICE,
}

type bpf_prog_type = c_uint;
type bpf_map_type = c_uint;
type bpf_func_id = c_uint;

const BPF_PROG_TYPE_UNSPEC: bpf_prog_type = 0;
const BPF_PROG_TYPE_SOCKET_FILTER: bpf_prog_type = 1;
const BPF_PROG_TYPE_SCHED_CLS: bpf_prog_type = 3;
const BPF_PROG_TYPE_XDP: bpf_prog_type = 6;

const BPF_MAP_TYPE_UNSPEC: bpf_map_type = 0;
const BPF_MAP_TYPE_HASH: bpf_map_type = 1;
const BPF_MAP_TYPE_ARRAY: bpf_map_type = 2;

const BPF_FUNC_trace_printk: c_uint = 6;
const BPF_FUNC_probe_write_user: c_uint = 36;
const BPF_FUNC_trace_vprintk: c_uint = 177;

const BPF_REG_0: c_int = 0;
const BPF_SUB: c_int = 0x10;
const BPF_JNE: c_int = 0x50;
const BPF_JLT: c_int = 0xa0;
const BPF_JEQ: c_int = 0x10;

unsafe extern "C" {
    static mut errno: c_int;
    static mut json_output: bool;
    static mut json_wtr: *mut c_void;
    static mut bin_name: *const c_char;

    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn statfs(path: *const c_char, buf: *mut statfs) -> c_int;
    fn toupper(c: c_int) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut FILE;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn sprintf(s: *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn free(ptr: *mut c_void);
    fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn geteuid() -> c_uint;

    fn jsonw_bool_field(w: *mut c_void, prop: *const c_char, value: bool);
    fn jsonw_null_field(w: *mut c_void, prop: *const c_char);
    fn jsonw_int_field(w: *mut c_void, prop: *const c_char, value: c_int);
    fn jsonw_string_field(w: *mut c_void, prop: *const c_char, value: *const c_char);
    fn jsonw_name(w: *mut c_void, name: *const c_char);
    fn jsonw_start_object(w: *mut c_void);
    fn jsonw_end_object(w: *mut c_void);
    fn jsonw_start_array(w: *mut c_void);
    fn jsonw_end_array(w: *mut c_void);
    fn jsonw_string(w: *mut c_void, value: *const c_char);
    fn jsonw_null(w: *mut c_void);

    fn bpf_prog_load(
        prog_type: bpf_prog_type,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: size_t,
        opts: *const bpf_prog_load_opts,
    ) -> c_int;
    fn bpf_map_create(
        map_type: bpf_map_type,
        map_name: *const c_char,
        key_size: c_int,
        value_size: c_int,
        max_entries: c_int,
        opts: *const bpf_map_create_opts,
    ) -> c_int;
    fn libbpf_probe_bpf_prog_type(prog_type: bpf_prog_type, opts: *const c_void) -> c_int;
    fn libbpf_probe_bpf_map_type(map_type: bpf_map_type, opts: *const c_void) -> c_int;
    fn libbpf_probe_bpf_helper(prog_type: bpf_prog_type, helper_id: c_uint, opts: *const c_void) -> c_int;
    fn libbpf_bpf_prog_type_str(t: c_uint) -> *const c_char;
    fn libbpf_bpf_map_type_str(t: c_uint) -> *const c_char;
    fn libbpf_bpf_attach_type_str(t: c_uint) -> *const c_char;
    fn libbpf_bpf_link_type_str(t: c_uint) -> *const c_char;

    fn read_kernel_config(
        options: *mut kernel_config_option,
        n: size_t,
        values: *mut *mut c_char,
        define_prefix: *const c_char,
    ) -> c_int;
    fn set_max_rlimit();
    fn is_prefix(str_: *const c_char, prefix: *const c_char) -> bool;
    fn p_info(fmt: *const c_char, ...);
    fn p_err(fmt: *const c_char, ...);
    fn usage() -> !;
    fn cmd_select(cmds: *const cmd, argc: c_int, argv: *mut *mut c_char,
                  help: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int) -> c_int;

    fn cap_get_proc() -> cap_t;
    fn cap_get_flag(caps: cap_t, cap: cap_value_t, flag: c_int, value: *mut cap_flag_value_t) -> c_int;
    fn cap_set_flag(caps: cap_t, flag: c_int, ncap: c_int, caps_list: *const cap_value_t, value: c_int) -> c_int;
    fn cap_set_proc(caps: cap_t) -> c_int;
    fn cap_free(caps: cap_t) -> c_int;
    fn CAP_IS_SUPPORTED(cap: cap_value_t) -> c_int;
}

const fn cstr(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

macro_rules! ARRAY_SIZE {
    ($arr:expr) => {
        $arr.len()
    };
}

macro_rules! NEXT_ARG {
    ($argc:ident, $argv:ident) => {{
        $argc -= 1;
        $argv = $argv.add(1);
    }};
}

macro_rules! REQ_ARGS {
    ($argc:expr, $n:expr) => {
        $argc >= $n
    };
}

macro_rules! GET_ARG {
    ($argc:ident, $argv:ident) => {{
        let arg = *$argv;
        NEXT_ARG!($argc, $argv);
        arg
    }};
}

fn BPF_MOV64_IMM(dst: c_int, imm: c_int) -> bpf_insn {
    bpf_insn { code: 0xb7, dst_src: dst as u8, off: 0, imm }
}

fn BPF_ALU64_IMM(op: c_int, dst: c_int, imm: c_int) -> bpf_insn {
    bpf_insn { code: (0x07 | op) as u8, dst_src: dst as u8, off: 0, imm }
}

fn BPF_JMP_IMM(op: c_int, dst: c_int, imm: c_int, off: c_int) -> bpf_insn {
    bpf_insn { code: (0x05 | op) as u8, dst_src: dst as u8, off: off as i16, imm }
}

fn BPF_JMP32_IMM(op: c_int, dst: c_int, imm: c_int, off: c_int) -> bpf_insn {
    bpf_insn { code: (0x06 | op) as u8, dst_src: dst as u8, off: off as i16, imm }
}

fn BPF_JMP32_A(off: c_int) -> bpf_insn {
    bpf_insn { code: 0x06 | 0x00, dst_src: 0, off: off as i16, imm: 0 }
}

fn BPF_EXIT_INSN() -> bpf_insn {
    bpf_insn { code: 0x95, dst_src: 0, off: 0, imm: 0 }
}

fn BPF_EMIT_CALL(func: c_uint) -> bpf_insn {
    bpf_insn { code: 0x85, dst_src: 0, off: 0, imm: func as i32 }
}

/* __BPF_FUNC_MAPPER(BPF_HELPER_MAKE_ENTRY) expands in Linux headers. */
static helper_name: [*const c_char; 1] = [ptr::null()];

static mut full_mode: bool = false;
/* #ifdef USE_LIBCAP */
static mut run_as_unprivileged: bool = false;

/* Miscellaneous utility functions */

unsafe fn grep(buffer: *const c_char, pattern: *const c_char) -> bool {
    !strstr(buffer, pattern).is_null()
}

unsafe fn check_procfs() -> bool {
    let mut st_fs: statfs = mem::zeroed();

    if statfs(cstr(b"/proc\0"), &mut st_fs) < 0 {
        return false;
    }
    if st_fs.f_type as c_ulong != PROC_SUPER_MAGIC {
        return false;
    }

    true
}

unsafe fn uppercase(str_: *mut c_char, len: size_t) {
    let mut i: size_t = 0;

    while i < len && *str_.add(i) != 0 {
        *str_.add(i) = toupper(*str_.add(i) as c_int) as c_char;
        i += 1;
    }
}

/* Printing utility functions */

unsafe fn print_bool_feature(
    feat_name: *const c_char,
    plain_name: *const c_char,
    define_name: *const c_char,
    res: bool,
    define_prefix: *const c_char,
) {
    if json_output {
        jsonw_bool_field(json_wtr, feat_name, res);
    } else if !define_prefix.is_null() {
        printf(cstr(b"#define %s%sHAVE_%s\n\0"), define_prefix,
               if res { cstr(b"\0") } else { cstr(b"NO_\0") }, define_name);
    } else {
        printf(cstr(b"%s is %savailable\n\0"), plain_name,
               if res { cstr(b"\0") } else { cstr(b"NOT \0") });
    }
}

unsafe fn print_kernel_option(name: *const c_char, value: *const c_char, define_prefix: *const c_char) {
    let mut endptr: *mut c_char = ptr::null_mut();
    let res: c_int;

    if json_output {
        if value.is_null() {
            jsonw_null_field(json_wtr, name);
            return;
        }
        errno = 0;
        res = strtol(value, &mut endptr, 0) as c_int;
        if errno == 0 && *endptr == b'\n' as c_char {
            jsonw_int_field(json_wtr, name, res);
        } else {
            jsonw_string_field(json_wtr, name, value);
        }
    } else if !define_prefix.is_null() {
        if !value.is_null() {
            printf(cstr(b"#define %s%s %s\n\0"), define_prefix, name, value);
        } else {
            printf(cstr(b"/* %s%s is not set */\n\0"), define_prefix, name);
        }
    } else if !value.is_null() {
        printf(cstr(b"%s is set to %s\n\0"), name, value);
    } else {
        printf(cstr(b"%s is not set\n\0"), name);
    }
}

unsafe fn print_start_section(
    json_title: *const c_char,
    plain_title: *const c_char,
    define_comment: *const c_char,
    define_prefix: *const c_char,
) {
    if json_output {
        jsonw_name(json_wtr, json_title);
        jsonw_start_object(json_wtr);
    } else if !define_prefix.is_null() {
        printf(cstr(b"%s\n\0"), define_comment);
    } else {
        printf(cstr(b"%s\n\0"), plain_title);
    }
}

unsafe fn print_end_section() {
    if json_output {
        jsonw_end_object(json_wtr);
    } else {
        printf(cstr(b"\n\0"));
    }
}

/* Probing functions */

unsafe fn get_vendor_id(ifindex: c_int) -> c_int {
    let mut ifname = [0 as c_char; IF_NAMESIZE];
    let mut path = [0 as c_char; 64];
    let mut buf = [0 as c_char; 8];
    let len: ssize_t;
    let fd: c_int;

    if if_indextoname(ifindex as c_uint, ifname.as_mut_ptr()).is_null() {
        return -1;
    }

    snprintf(path.as_mut_ptr(), path.len(), cstr(b"/sys/class/net/%s/device/vendor\0"), ifname.as_ptr());

    fd = open(path.as_ptr(), O_RDONLY | O_CLOEXEC);
    if fd < 0 {
        return -1;
    }

    len = read(fd, buf.as_mut_ptr() as *mut c_void, buf.len());
    close(fd);
    if len < 0 {
        return -1;
    }
    if len >= buf.len() as ssize_t {
        return -1;
    }
    buf[len as usize] = 0;

    strtol(buf.as_ptr(), ptr::null_mut(), 0) as c_int
}

unsafe fn read_procfs(path: *const c_char) -> c_long {
    let mut endptr: *mut c_char = ptr::null_mut();
    let mut line: *mut c_char = ptr::null_mut();
    let mut len: size_t = 0;
    let fd: *mut FILE;
    let mut res: c_long;

    fd = fopen(path, cstr(b"r\0"));
    if fd.is_null() {
        return -1;
    }

    res = getline(&mut line, &mut len, fd) as c_long;
    fclose(fd);
    if res < 0 {
        return -1;
    }

    errno = 0;
    res = strtol(line, &mut endptr, 10);
    if errno != 0 || *line == 0 || *endptr != b'\n' as c_char {
        res = -1;
    }
    free(line as *mut c_void);

    res
}

unsafe fn probe_unprivileged_disabled() {
    let res: c_long;

    /* No support for C-style output */

    res = read_procfs(cstr(b"/proc/sys/kernel/unprivileged_bpf_disabled\0"));
    if json_output {
        jsonw_int_field(json_wtr, cstr(b"unprivileged_bpf_disabled\0"), res as c_int);
    } else {
        match res {
            0 => { printf(cstr(b"bpf() syscall for unprivileged users is enabled\n\0")); }
            1 => { printf(cstr(b"bpf() syscall restricted to privileged users (without recovery)\n\0")); }
            2 => { printf(cstr(b"bpf() syscall restricted to privileged users (admin can change)\n\0")); }
            -1 => { printf(cstr(b"Unable to retrieve required privileges for bpf() syscall\n\0")); }
            _ => { printf(cstr(b"bpf() syscall restriction has unknown value %ld\n\0"), res); }
        }
    }
}

unsafe fn probe_jit_enable() {
    let res = read_procfs(cstr(b"/proc/sys/net/core/bpf_jit_enable\0"));
    if json_output {
        jsonw_int_field(json_wtr, cstr(b"bpf_jit_enable\0"), res as c_int);
    } else {
        match res {
            0 => { printf(cstr(b"JIT compiler is disabled\n\0")); }
            1 => { printf(cstr(b"JIT compiler is enabled\n\0")); }
            2 => { printf(cstr(b"JIT compiler is enabled with debugging traces in kernel logs\n\0")); }
            -1 => { printf(cstr(b"Unable to retrieve JIT-compiler status\n\0")); }
            _ => { printf(cstr(b"JIT-compiler status has unknown value %ld\n\0"), res); }
        }
    }
}

unsafe fn probe_jit_harden() {
    let res = read_procfs(cstr(b"/proc/sys/net/core/bpf_jit_harden\0"));
    if json_output {
        jsonw_int_field(json_wtr, cstr(b"bpf_jit_harden\0"), res as c_int);
    } else {
        match res {
            0 => { printf(cstr(b"JIT compiler hardening is disabled\n\0")); }
            1 => { printf(cstr(b"JIT compiler hardening is enabled for unprivileged users\n\0")); }
            2 => { printf(cstr(b"JIT compiler hardening is enabled for all users\n\0")); }
            -1 => { printf(cstr(b"Unable to retrieve JIT hardening status\n\0")); }
            _ => { printf(cstr(b"JIT hardening status has unknown value %ld\n\0"), res); }
        }
    }
}

unsafe fn probe_jit_kallsyms() {
    let res = read_procfs(cstr(b"/proc/sys/net/core/bpf_jit_kallsyms\0"));
    if json_output {
        jsonw_int_field(json_wtr, cstr(b"bpf_jit_kallsyms\0"), res as c_int);
    } else {
        match res {
            0 => { printf(cstr(b"JIT compiler kallsyms exports are disabled\n\0")); }
            1 => { printf(cstr(b"JIT compiler kallsyms exports are enabled for root\n\0")); }
            -1 => { printf(cstr(b"Unable to retrieve JIT kallsyms export status\n\0")); }
            _ => { printf(cstr(b"JIT kallsyms exports status has unknown value %ld\n\0"), res); }
        }
    }
}

unsafe fn probe_jit_limit() {
    let res = read_procfs(cstr(b"/proc/sys/net/core/bpf_jit_limit\0"));
    if json_output {
        jsonw_int_field(json_wtr, cstr(b"bpf_jit_limit\0"), res as c_int);
    } else {
        match res {
            -1 => { printf(cstr(b"Unable to retrieve global memory limit for JIT compiler for unprivileged users\n\0")); }
            _ => { printf(cstr(b"Global memory limit for JIT compiler for unprivileged users is %ld bytes\n\0"), res); }
        }
    }
}

unsafe fn probe_kernel_image_config(define_prefix: *const c_char) {
    let mut options = [
        kernel_config_option { name: cstr(b"CONFIG_BPF\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_BPF_SYSCALL\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_HAVE_EBPF_JIT\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_BPF_JIT\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_BPF_JIT_ALWAYS_ON\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_DEBUG_INFO_BTF\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_DEBUG_INFO_BTF_MODULES\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_CGROUPS\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_CGROUP_BPF\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_CGROUP_NET_CLASSID\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_SOCK_CGROUP_DATA\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_BPF_EVENTS\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_KPROBE_EVENTS\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_UPROBE_EVENTS\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_TRACING\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_FTRACE_SYSCALLS\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_FUNCTION_ERROR_INJECTION\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_BPF_KPROBE_OVERRIDE\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_NET\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_XDP_SOCKETS\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_LWTUNNEL_BPF\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_NET_ACT_BPF\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_NET_CLS_BPF\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_NET_CLS_ACT\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_NET_SCH_INGRESS\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_XFRM\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_IP_ROUTE_CLASSID\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_IPV6_SEG6_BPF\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_BPF_LIRC_MODE2\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_BPF_STREAM_PARSER\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_NETFILTER_XT_MATCH_BPF\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_TEST_BPF\0"), macro_dump: false },
        kernel_config_option { name: cstr(b"CONFIG_HZ\0"), macro_dump: true },
    ];
    let mut values = [ptr::null_mut::<c_char>(); 33];
    let mut i: size_t;

    if read_kernel_config(options.as_mut_ptr(), ARRAY_SIZE!(options), values.as_mut_ptr(), define_prefix) != 0 {
        return;
    }

    i = 0;
    while i < ARRAY_SIZE!(options) {
        if !define_prefix.is_null() && !options[i].macro_dump {
            i += 1;
            continue;
        }
        print_kernel_option(options[i].name, values[i], define_prefix);
        free(values[i] as *mut c_void);
        i += 1;
    }
}

unsafe fn probe_bpf_syscall(define_prefix: *const c_char) -> bool {
    let res: bool;

    bpf_prog_load(BPF_PROG_TYPE_UNSPEC, ptr::null(), ptr::null(), ptr::null(), 0, ptr::null());
    res = errno != ENOSYS;

    print_bool_feature(cstr(b"have_bpf_syscall\0"), cstr(b"bpf() syscall\0"),
                       cstr(b"BPF_SYSCALL\0"), res, define_prefix);

    res
}

unsafe fn probe_prog_load_ifindex(
    prog_type: bpf_prog_type,
    insns: *const bpf_insn,
    insns_cnt: size_t,
    log_buf: *mut c_char,
    log_buf_sz: size_t,
    ifindex: __u32,
) -> bool {
    let mut opts = bpf_prog_load_opts {
        sz: mem::size_of::<bpf_prog_load_opts>(),
        log_buf,
        log_size: log_buf_sz as u32,
        log_level: if !log_buf.is_null() { 1 } else { 0 },
        prog_ifindex: ifindex,
    };
    let fd: c_int;

    errno = 0;
    fd = bpf_prog_load(prog_type, ptr::null(), cstr(b"GPL\0"), insns, insns_cnt, &mut opts);
    if fd >= 0 {
        close(fd);
    }

    fd >= 0 && errno != EINVAL && errno != EOPNOTSUPP
}

unsafe fn probe_prog_type_ifindex(prog_type: bpf_prog_type, ifindex: __u32) -> bool {
    /* nfp returns -EINVAL on exit(0) with TC offload */
    let insns = [
        BPF_MOV64_IMM(BPF_REG_0, 2),
        BPF_EXIT_INSN(),
    ];

    probe_prog_load_ifindex(prog_type, insns.as_ptr(), ARRAY_SIZE!(insns), ptr::null_mut(), 0, ifindex)
}

unsafe fn probe_prog_type(
    prog_type: bpf_prog_type,
    prog_type_str: *const c_char,
    supported_types: *mut bool,
    define_prefix: *const c_char,
    ifindex: __u32,
) {
    let mut feat_name = [0 as c_char; 128];
    let mut plain_desc = [0 as c_char; 128];
    let mut define_name = [0 as c_char; 128];
    let plain_comment = cstr(b"eBPF program_type \0");
    let maxlen: size_t;
    let mut res: bool;

    if ifindex != 0 {
        match prog_type {
            BPF_PROG_TYPE_SCHED_CLS | BPF_PROG_TYPE_XDP => {}
            _ => return,
        }
        res = probe_prog_type_ifindex(prog_type, ifindex);
    } else {
        res = libbpf_probe_bpf_prog_type(prog_type, ptr::null()) > 0;
    }

    /* #ifdef USE_LIBCAP */
    if run_as_unprivileged && errno == EPERM {
        res = false;
    }

    *supported_types.add(prog_type as usize) |= res;

    maxlen = plain_desc.len() - strlen(plain_comment) - 1;
    if strlen(prog_type_str) > maxlen {
        p_info(cstr(b"program type name too long\0"));
        return;
    }

    sprintf(feat_name.as_mut_ptr(), cstr(b"have_%s_prog_type\0"), prog_type_str);
    sprintf(define_name.as_mut_ptr(), cstr(b"%s_prog_type\0"), prog_type_str);
    uppercase(define_name.as_mut_ptr(), define_name.len());
    sprintf(plain_desc.as_mut_ptr(), cstr(b"%s%s\0"), plain_comment, prog_type_str);
    print_bool_feature(feat_name.as_ptr(), plain_desc.as_ptr(), define_name.as_ptr(), res, define_prefix);
}

unsafe fn probe_map_type_ifindex(map_type: bpf_map_type, ifindex: __u32) -> bool {
    let mut opts = bpf_map_create_opts { sz: mem::size_of::<bpf_map_create_opts>(), map_ifindex: 0 };
    let key_size: c_int;
    let value_size: c_int;
    let max_entries: c_int;
    let fd: c_int;

    opts.map_ifindex = ifindex;

    key_size = mem::size_of::<__u32>() as c_int;
    value_size = mem::size_of::<__u32>() as c_int;
    max_entries = 1;

    fd = bpf_map_create(map_type, ptr::null(), key_size, value_size, max_entries, &mut opts);
    if fd >= 0 {
        close(fd);
    }

    fd >= 0
}

unsafe fn probe_map_type(
    map_type: bpf_map_type,
    map_type_str: *const c_char,
    define_prefix: *const c_char,
    ifindex: __u32,
) {
    let mut feat_name = [0 as c_char; 128];
    let mut plain_desc = [0 as c_char; 128];
    let mut define_name = [0 as c_char; 128];
    let plain_comment = cstr(b"eBPF map_type \0");
    let maxlen: size_t;
    let res: bool;

    if ifindex != 0 {
        match map_type {
            BPF_MAP_TYPE_HASH | BPF_MAP_TYPE_ARRAY => {}
            _ => return,
        }
        res = probe_map_type_ifindex(map_type, ifindex);
    } else {
        res = libbpf_probe_bpf_map_type(map_type, ptr::null()) > 0;
    }

    /* Probe result depends on the success of map creation, no additional
     * check required for unprivileged users
     */

    maxlen = plain_desc.len() - strlen(plain_comment) - 1;
    if strlen(map_type_str) > maxlen {
        p_info(cstr(b"map type name too long\0"));
        return;
    }

    sprintf(feat_name.as_mut_ptr(), cstr(b"have_%s_map_type\0"), map_type_str);
    sprintf(define_name.as_mut_ptr(), cstr(b"%s_map_type\0"), map_type_str);
    uppercase(define_name.as_mut_ptr(), define_name.len());
    sprintf(plain_desc.as_mut_ptr(), cstr(b"%s%s\0"), plain_comment, map_type_str);
    print_bool_feature(feat_name.as_ptr(), plain_desc.as_ptr(), define_name.as_ptr(), res, define_prefix);
}

unsafe fn probe_helper_ifindex(id: bpf_func_id, prog_type: bpf_prog_type, ifindex: __u32) -> bool {
    let insns = [
        BPF_EMIT_CALL(id),
        BPF_EXIT_INSN(),
    ];
    let mut buf = [0 as c_char; 4096];
    let mut res: bool;

    probe_prog_load_ifindex(prog_type, insns.as_ptr(), ARRAY_SIZE!(insns), buf.as_mut_ptr(), buf.len(), ifindex);
    res = !grep(buf.as_ptr(), cstr(b"invalid func \0")) &&
        !grep(buf.as_ptr(), cstr(b"unknown func \0")) &&
        !grep(buf.as_ptr(), cstr(b"program of this type cannot use helper \0"));

    match get_vendor_id(ifindex as c_int) {
        0x19ee => {
            /* Netronome specific */
            res = res && !grep(buf.as_ptr(), cstr(b"not supported by FW\0")) &&
                !grep(buf.as_ptr(), cstr(b"unsupported function id\0"));
        }
        _ => {}
    }

    res
}

unsafe fn probe_helper_for_progtype(
    prog_type: bpf_prog_type,
    supported_type: bool,
    define_prefix: *const c_char,
    id: c_uint,
    ptype_name: *const c_char,
    ifindex: __u32,
) -> bool {
    let mut res = false;

    if supported_type {
        if ifindex != 0 {
            res = probe_helper_ifindex(id, prog_type, ifindex);
        } else {
            res = libbpf_probe_bpf_helper(prog_type, id, ptr::null()) > 0;
        }
        /* #ifdef USE_LIBCAP */
        if run_as_unprivileged && errno == EPERM {
            res = false;
        }
    }

    if json_output {
        if res {
            jsonw_string(json_wtr, helper_name[id as usize]);
        }
    } else if !define_prefix.is_null() {
        printf(cstr(b"#define %sBPF__PROG_TYPE_%s__HELPER_%s %s\n\0"),
               define_prefix, ptype_name, helper_name[id as usize],
               if res { cstr(b"1\0") } else { cstr(b"0\0") });
    } else if res {
        printf(cstr(b"\n\t- %s\0"), helper_name[id as usize]);
    }

    res
}

unsafe fn probe_helpers_for_progtype(
    prog_type: bpf_prog_type,
    prog_type_str: *const c_char,
    supported_type: bool,
    define_prefix: *const c_char,
    ifindex: __u32,
) {
    let mut feat_name = [0 as c_char; 128];
    let mut id: c_uint;
    let mut probe_res = false;

    if ifindex != 0 {
        /* Only test helpers for offload-able program types */
        match prog_type {
            BPF_PROG_TYPE_SCHED_CLS | BPF_PROG_TYPE_XDP => {}
            _ => return,
        }
    }

    if json_output {
        sprintf(feat_name.as_mut_ptr(), cstr(b"%s_available_helpers\0"), prog_type_str);
        jsonw_name(json_wtr, feat_name.as_ptr());
        jsonw_start_array(json_wtr);
    } else if define_prefix.is_null() {
        printf(cstr(b"eBPF helpers supported for program type %s:\0"), prog_type_str);
    }

    id = 1;
    while (id as usize) < ARRAY_SIZE!(helper_name) {
        /* Skip helper functions which emit dmesg messages when not in
         * the full mode.
         */
        match id {
            BPF_FUNC_trace_printk | BPF_FUNC_trace_vprintk | BPF_FUNC_probe_write_user => {
                if !full_mode {
                    id += 1;
                    continue;
                }
                probe_res |= probe_helper_for_progtype(prog_type, supported_type, define_prefix, id, prog_type_str, ifindex);
            }
            _ => {
                probe_res |= probe_helper_for_progtype(prog_type, supported_type, define_prefix, id, prog_type_str, ifindex);
            }
        }
        id += 1;
    }

    if json_output {
        jsonw_end_array(json_wtr);
    } else if define_prefix.is_null() {
        printf(cstr(b"\n\0"));
        if !probe_res {
            if !supported_type {
                printf(cstr(b"\tProgram type not supported\n\0"));
            } else {
                printf(cstr(b"\tCould not determine which helpers are available\n\0"));
            }
        }
    }
}

unsafe fn probe_misc_feature(
    insns: *mut bpf_insn,
    len: size_t,
    define_prefix: *const c_char,
    ifindex: __u32,
    feat_name: *const c_char,
    plain_name: *const c_char,
    define_name: *const c_char,
) {
    let mut opts = bpf_prog_load_opts {
        sz: mem::size_of::<bpf_prog_load_opts>(),
        log_buf: ptr::null_mut(),
        log_size: 0,
        log_level: 0,
        prog_ifindex: ifindex,
    };
    let res: bool;
    let fd: c_int;

    errno = 0;
    fd = bpf_prog_load(BPF_PROG_TYPE_SOCKET_FILTER, ptr::null(), cstr(b"GPL\0"), insns, len, &mut opts);
    res = fd >= 0 || errno == 0;

    if fd >= 0 {
        close(fd);
    }

    print_bool_feature(feat_name, plain_name, define_name, res, define_prefix);
}

/*
 * Probe for availability of kernel commit (5.3):
 *
 * c04c0d2b968a ("bpf: increase complexity limit and maximum program size")
 */
unsafe fn probe_large_insn_limit(define_prefix: *const c_char, ifindex: __u32) {
    let mut insns = [BPF_EXIT_INSN(); BPF_MAXINSNS + 1];
    let mut i: c_int;

    i = 0;
    while i < BPF_MAXINSNS as c_int {
        insns[i as usize] = BPF_MOV64_IMM(BPF_REG_0, 1);
        i += 1;
    }
    insns[BPF_MAXINSNS] = BPF_EXIT_INSN();

    probe_misc_feature(insns.as_mut_ptr(), ARRAY_SIZE!(insns), define_prefix, ifindex,
                       cstr(b"have_large_insn_limit\0"), cstr(b"Large program size limit\0"),
                       cstr(b"LARGE_INSN_LIMIT\0"));
}

/*
 * Probe for bounded loop support introduced in commit 2589726d12a1
 * ("bpf: introduce bounded loops").
 */
unsafe fn probe_bounded_loops(define_prefix: *const c_char, ifindex: __u32) {
    let mut insns = [
        BPF_MOV64_IMM(BPF_REG_0, 10),
        BPF_ALU64_IMM(BPF_SUB, BPF_REG_0, 1),
        BPF_JMP_IMM(BPF_JNE, BPF_REG_0, 0, -2),
        BPF_EXIT_INSN(),
    ];

    probe_misc_feature(insns.as_mut_ptr(), ARRAY_SIZE!(insns), define_prefix, ifindex,
                       cstr(b"have_bounded_loops\0"), cstr(b"Bounded loop support\0"),
                       cstr(b"BOUNDED_LOOPS\0"));
}

/*
 * Probe for the v2 instruction set extension introduced in commit 92b31a9af73b
 * ("bpf: add BPF_J{LT,LE,SLT,SLE} instructions").
 */
unsafe fn probe_v2_isa_extension(define_prefix: *const c_char, ifindex: __u32) {
    let mut insns = [
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_JMP_IMM(BPF_JLT, BPF_REG_0, 0, 1),
        BPF_MOV64_IMM(BPF_REG_0, 1),
        BPF_EXIT_INSN(),
    ];

    probe_misc_feature(insns.as_mut_ptr(), ARRAY_SIZE!(insns), define_prefix, ifindex,
                       cstr(b"have_v2_isa_extension\0"), cstr(b"ISA extension v2\0"),
                       cstr(b"V2_ISA_EXTENSION\0"));
}

/*
 * Probe for the v3 instruction set extension introduced in commit 092ed0968bb6
 * ("bpf: verifier support JMP32").
 */
unsafe fn probe_v3_isa_extension(define_prefix: *const c_char, ifindex: __u32) {
    let mut insns = [
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_JMP32_IMM(BPF_JLT, BPF_REG_0, 0, 1),
        BPF_MOV64_IMM(BPF_REG_0, 1),
        BPF_EXIT_INSN(),
    ];

    probe_misc_feature(insns.as_mut_ptr(), ARRAY_SIZE!(insns), define_prefix, ifindex,
                       cstr(b"have_v3_isa_extension\0"), cstr(b"ISA extension v3\0"),
                       cstr(b"V3_ISA_EXTENSION\0"));
}

/*
 * Probe for the v4 instruction set extension introduced in commit 1f9a1ea821ff
 * ("bpf: Support new sign-extension load insns").
 */
unsafe fn probe_v4_isa_extension(define_prefix: *const c_char, ifindex: __u32) {
    let mut insns = [
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_JMP32_IMM(BPF_JEQ, BPF_REG_0, 1, 1),
        BPF_JMP32_A(1),
        BPF_MOV64_IMM(BPF_REG_0, 1),
        BPF_EXIT_INSN(),
    ];

    probe_misc_feature(insns.as_mut_ptr(), ARRAY_SIZE!(insns), define_prefix, ifindex,
                       cstr(b"have_v4_isa_extension\0"), cstr(b"ISA extension v4\0"),
                       cstr(b"V4_ISA_EXTENSION\0"));
}

unsafe fn section_system_config(target: probe_component, define_prefix: *const c_char) {
    match target {
        probe_component::COMPONENT_KERNEL | probe_component::COMPONENT_UNSPEC => {
            print_start_section(cstr(b"system_config\0"), cstr(b"Scanning system configuration...\0"),
                                cstr(b"/*** Misc kernel config items ***/\0"), define_prefix);
            if define_prefix.is_null() {
                if check_procfs() {
                    probe_unprivileged_disabled();
                    probe_jit_enable();
                    probe_jit_harden();
                    probe_jit_kallsyms();
                    probe_jit_limit();
                } else {
                    p_info(cstr(b"/* procfs not mounted, skipping related probes */\0"));
                }
            }
            probe_kernel_image_config(define_prefix);
            print_end_section();
        }
        _ => {}
    }
}

unsafe fn section_syscall_config(define_prefix: *const c_char) -> bool {
    let res: bool;

    print_start_section(cstr(b"syscall_config\0"), cstr(b"Scanning system call availability...\0"),
                        cstr(b"/*** System call availability ***/\0"), define_prefix);
    res = probe_bpf_syscall(define_prefix);
    print_end_section();

    res
}

unsafe fn section_program_types(supported_types: *mut bool, define_prefix: *const c_char, ifindex: __u32) {
    let mut prog_type: c_uint = BPF_PROG_TYPE_UNSPEC;
    let mut prog_type_str: *const c_char;

    print_start_section(cstr(b"program_types\0"), cstr(b"Scanning eBPF program types...\0"),
                        cstr(b"/*** eBPF program types ***/\0"), define_prefix);

    loop {
        prog_type += 1;
        prog_type_str = libbpf_bpf_prog_type_str(prog_type);
        /* libbpf will return NULL for variants unknown to it. */
        if prog_type_str.is_null() {
            break;
        }

        probe_prog_type(prog_type, prog_type_str, supported_types, define_prefix, ifindex);
    }

    print_end_section();
}

unsafe fn section_map_types(define_prefix: *const c_char, ifindex: __u32) {
    let mut map_type: c_uint = BPF_MAP_TYPE_UNSPEC;
    let mut map_type_str: *const c_char;

    print_start_section(cstr(b"map_types\0"), cstr(b"Scanning eBPF map types...\0"),
                        cstr(b"/*** eBPF map types ***/\0"), define_prefix);

    loop {
        map_type += 1;
        map_type_str = libbpf_bpf_map_type_str(map_type);
        /* libbpf will return NULL for variants unknown to it. */
        if map_type_str.is_null() {
            break;
        }

        probe_map_type(map_type, map_type_str, define_prefix, ifindex);
    }

    print_end_section();
}

unsafe fn section_helpers(supported_types: *mut bool, define_prefix: *const c_char, ifindex: __u32) {
    let mut prog_type: c_uint = BPF_PROG_TYPE_UNSPEC;
    let mut prog_type_str: *const c_char;

    print_start_section(cstr(b"helpers\0"), cstr(b"Scanning eBPF helper functions...\0"),
                        cstr(b"/*** eBPF helper functions ***/\0"), define_prefix);

    if !define_prefix.is_null() {
        printf(cstr(b"/*\n * Use %sHAVE_PROG_TYPE_HELPER(prog_type_name, helper_name)\n * to determine if <helper_name> is available for <prog_type_name>,\n * e.g.\n *\t#if %sHAVE_PROG_TYPE_HELPER(xdp, bpf_redirect)\n *\t\t// do stuff with this helper\n *\t#elif\n *\t\t// use a workaround\n *\t#endif\n */\n#define %sHAVE_PROG_TYPE_HELPER(prog_type, helper)\t\\\n\t%sBPF__PROG_TYPE_ ## prog_type ## __HELPER_ ## helper\n\0"),
               define_prefix, define_prefix, define_prefix, define_prefix);
    }
    loop {
        prog_type += 1;
        prog_type_str = libbpf_bpf_prog_type_str(prog_type);
        /* libbpf will return NULL for variants unknown to it. */
        if prog_type_str.is_null() {
            break;
        }

        probe_helpers_for_progtype(prog_type, prog_type_str, *supported_types.add(prog_type as usize),
                                   define_prefix, ifindex);
    }

    print_end_section();
}

unsafe fn section_misc(define_prefix: *const c_char, ifindex: __u32) {
    print_start_section(cstr(b"misc\0"), cstr(b"Scanning miscellaneous eBPF features...\0"),
                        cstr(b"/*** eBPF misc features ***/\0"), define_prefix);
    probe_large_insn_limit(define_prefix, ifindex);
    probe_bounded_loops(define_prefix, ifindex);
    probe_v2_isa_extension(define_prefix, ifindex);
    probe_v3_isa_extension(define_prefix, ifindex);
    probe_v4_isa_extension(define_prefix, ifindex);
    print_end_section();
}

/* #ifdef USE_LIBCAP
 * #define capability(c) { c, false, #c }
 * #define capability_msg(a, i) a[i].set ? "" : a[i].name, a[i].set ? "" : ", "
 */

#[repr(C)]
struct bpf_cap {
    cap: cap_value_t,
    set: bool,
    name: [c_char; 14],
}

fn cap_entry(cap: cap_value_t, name: &[u8; 14]) -> bpf_cap {
    let mut arr = [0 as c_char; 14];
    let mut i = 0;
    while i < 14 {
        arr[i] = name[i] as c_char;
        i += 1;
    }
    bpf_cap { cap, set: false, name: arr }
}

unsafe fn handle_perms() -> c_int {
    /* #ifdef USE_LIBCAP */
    let mut bpf_caps = [
        cap_entry(CAP_SYS_ADMIN, b"CAP_SYS_ADMIN\0"),
        cap_entry(CAP_BPF, b"CAP_BPF\0\0\0\0\0\0\0"),
        cap_entry(CAP_NET_ADMIN, b"CAP_NET_ADMIN\0"),
        cap_entry(CAP_PERFMON, b"CAP_PERFMON\0\0\0"),
    ];
    let mut cap_list = [0 as cap_value_t; 4];
    let mut i: c_uint;
    let mut nb_bpf_caps: c_uint = 0;
    let mut cap_sys_admin_only = true;
    let mut val: cap_flag_value_t = 0;
    let mut res: c_int = -1;
    let caps: cap_t;

    caps = cap_get_proc();
    if caps.is_null() {
        p_err(cstr(b"failed to get capabilities for process: %s\0"), strerror(errno));
        return -1;
    }

    /* #ifdef CAP_BPF */
    if CAP_IS_SUPPORTED(CAP_BPF) != 0 {
        cap_sys_admin_only = false;
    }

    i = 0;
    while (i as usize) < ARRAY_SIZE!(bpf_caps) {
        let cap_name = bpf_caps[i as usize].name.as_ptr();
        let cap = bpf_caps[i as usize].cap;

        if cap_get_flag(caps, cap, CAP_EFFECTIVE, &mut val) != 0 {
            p_err(cstr(b"bug: failed to retrieve %s status: %s\0"), cap_name, strerror(errno));
            goto_exit_free(caps, &mut res);
            return res;
        }

        if val == CAP_SET {
            bpf_caps[i as usize].set = true;
            cap_list[nb_bpf_caps as usize] = cap;
            nb_bpf_caps += 1;
        }

        if cap_sys_admin_only {
            /* System does not know about CAP_BPF, meaning that
             * CAP_SYS_ADMIN is the only capability required. We
             * just checked it, break.
             */
            break;
        }
        i += 1;
    }

    if (run_as_unprivileged && nb_bpf_caps == 0) ||
        (!run_as_unprivileged && nb_bpf_caps as usize == ARRAY_SIZE!(bpf_caps)) ||
        (!run_as_unprivileged && cap_sys_admin_only && nb_bpf_caps != 0) {
        /* We are all good, exit now */
        res = 0;
        goto_exit_free(caps, &mut res);
        return res;
    }

    if !run_as_unprivileged {
        if cap_sys_admin_only {
            p_err(cstr(b"missing %s, required for full feature probing; run as root or use 'unprivileged'\0"),
                  bpf_caps[0].name.as_ptr());
        } else {
            p_err(cstr(b"missing %s%s%s%s%s%s%s%srequired for full feature probing; run as root or use 'unprivileged'\0"),
                  if bpf_caps[0].set { cstr(b"\0") } else { bpf_caps[0].name.as_ptr() },
                  if bpf_caps[0].set { cstr(b"\0") } else { cstr(b", \0") },
                  if bpf_caps[1].set { cstr(b"\0") } else { bpf_caps[1].name.as_ptr() },
                  if bpf_caps[1].set { cstr(b"\0") } else { cstr(b", \0") },
                  if bpf_caps[2].set { cstr(b"\0") } else { bpf_caps[2].name.as_ptr() },
                  if bpf_caps[2].set { cstr(b"\0") } else { cstr(b", \0") },
                  if bpf_caps[3].set { cstr(b"\0") } else { bpf_caps[3].name.as_ptr() },
                  if bpf_caps[3].set { cstr(b"\0") } else { cstr(b", \0") });
        }
        goto_exit_free(caps, &mut res);
        return res;
    }

    /* if (run_as_unprivileged && nb_bpf_caps > 0), drop capabilities. */
    if cap_set_flag(caps, CAP_EFFECTIVE, nb_bpf_caps as c_int, cap_list.as_ptr(), CAP_CLEAR) != 0 {
        p_err(cstr(b"bug: failed to clear capabilities: %s\0"), strerror(errno));
        goto_exit_free(caps, &mut res);
        return res;
    }

    if cap_set_proc(caps) != 0 {
        p_err(cstr(b"failed to drop capabilities: %s\0"), strerror(errno));
        goto_exit_free(caps, &mut res);
        return res;
    }

    res = 0;
    goto_exit_free(caps, &mut res);
    res

    /* #else
     * Detection assumes user has specific privileges.
     * We do not use libcap so let's approximate, and restrict usage to
     * root user only.
     *
     * if (geteuid()) {
     *     p_err("full feature probing requires root privileges");
     *     return -1;
     * }
     *
     * return 0;
     * #endif
     */
}

unsafe fn goto_exit_free(caps: cap_t, res: &mut c_int) {
    if cap_free(caps) != 0 && *res == 0 {
        p_err(cstr(b"failed to clear storage object for capabilities: %s\0"), strerror(errno));
        *res = -1;
    }
}

unsafe extern "C" fn do_probe(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut target = probe_component::COMPONENT_UNSPEC;
    let mut define_prefix: *const c_char = ptr::null();
    let mut supported_types = [false; 128];
    let mut ifindex: __u32 = 0;
    let ifname: *mut c_char;

    set_max_rlimit();

    while argc != 0 {
        if is_prefix(*argv, cstr(b"kernel\0")) {
            if target != probe_component::COMPONENT_UNSPEC {
                p_err(cstr(b"component to probe already specified\0"));
                return -1;
            }
            target = probe_component::COMPONENT_KERNEL;
            NEXT_ARG!(argc, argv);
        } else if is_prefix(*argv, cstr(b"dev\0")) {
            NEXT_ARG!(argc, argv);

            if target != probe_component::COMPONENT_UNSPEC || ifindex != 0 {
                p_err(cstr(b"component to probe already specified\0"));
                return -1;
            }
            if !REQ_ARGS!(argc, 1) {
                return -1;
            }

            target = probe_component::COMPONENT_DEVICE;
            ifname = GET_ARG!(argc, argv);
            ifindex = if_nametoindex(ifname);
            if ifindex == 0 {
                p_err(cstr(b"unrecognized netdevice '%s': %s\0"), ifname, strerror(errno));
                return -1;
            }
        } else if is_prefix(*argv, cstr(b"full\0")) {
            full_mode = true;
            NEXT_ARG!(argc, argv);
        } else if is_prefix(*argv, cstr(b"macros\0")) && define_prefix.is_null() {
            define_prefix = cstr(b"\0");
            NEXT_ARG!(argc, argv);
        } else if is_prefix(*argv, cstr(b"prefix\0")) {
            if define_prefix.is_null() {
                p_err(cstr(b"'prefix' argument can only be use after 'macros'\0"));
                return -1;
            }
            if strcmp(define_prefix, cstr(b"\0")) != 0 {
                p_err(cstr(b"'prefix' already defined\0"));
                return -1;
            }
            NEXT_ARG!(argc, argv);

            if !REQ_ARGS!(argc, 1) {
                return -1;
            }
            define_prefix = GET_ARG!(argc, argv);
        } else if is_prefix(*argv, cstr(b"unprivileged\0")) {
            /* #ifdef USE_LIBCAP */
            run_as_unprivileged = true;
            NEXT_ARG!(argc, argv);
            /* #else
             * p_err("unprivileged run not supported, recompile bpftool with libcap");
             * return -1;
             * #endif
             */
        } else {
            p_err(cstr(b"expected no more arguments, 'kernel', 'dev', 'macros' or 'prefix', got: '%s'?\0"), *argv);
            return -1;
        }
    }

    /* Full feature detection requires specific privileges.
     * Let's approximate, and warn if user is not root.
     */
    if handle_perms() != 0 {
        return -1;
    }

    if json_output {
        define_prefix = ptr::null();
        jsonw_start_object(json_wtr);
    }

    section_system_config(target, define_prefix);
    if !section_syscall_config(define_prefix) {
        /* bpf() syscall unavailable, don't probe other BPF features */
        goto_exit_close_json();
        return 0;
    }
    section_program_types(supported_types.as_mut_ptr(), define_prefix, ifindex);
    section_map_types(define_prefix, ifindex);
    section_helpers(supported_types.as_mut_ptr(), define_prefix, ifindex);
    section_misc(define_prefix, ifindex);

    goto_exit_close_json();
    0
}

unsafe fn goto_exit_close_json() {
    if json_output {
        /* End root object */
        jsonw_end_object(json_wtr);
    }
}

unsafe fn get_helper_name(id: c_uint) -> *const c_char {
    if id as usize >= ARRAY_SIZE!(helper_name) {
        return ptr::null();
    }

    helper_name[id as usize]
}

unsafe extern "C" fn do_list_builtins(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let get_name: unsafe extern "C" fn(c_uint) -> *const c_char;
    let mut id: c_uint = 0;

    if argc < 1 {
        usage();
    }

    if is_prefix(*argv, cstr(b"prog_types\0")) {
        get_name = libbpf_bpf_prog_type_str;
    } else if is_prefix(*argv, cstr(b"map_types\0")) {
        get_name = libbpf_bpf_map_type_str;
    } else if is_prefix(*argv, cstr(b"attach_types\0")) {
        get_name = libbpf_bpf_attach_type_str;
    } else if is_prefix(*argv, cstr(b"link_types\0")) {
        get_name = libbpf_bpf_link_type_str;
    } else if is_prefix(*argv, cstr(b"helpers\0")) {
        unsafe extern "C" fn helper_wrapper(id: c_uint) -> *const c_char {
            unsafe { get_helper_name(id) }
        }
        get_name = helper_wrapper;
    } else {
        p_err(cstr(b"expected 'prog_types', 'map_types', 'attach_types', 'link_types' or 'helpers', got: %s\0"), *argv);
        return -1;
    }

    if json_output {
        jsonw_start_array(json_wtr);	/* root array */
    }

    loop {
        let name: *const c_char;

        name = get_name(id);
        id += 1;
        if name.is_null() {
            break;
        }
        if json_output {
            jsonw_string(json_wtr, name);
        } else {
            printf(cstr(b"%s\n\0"), name);
        }
    }

    if json_output {
        jsonw_end_array(json_wtr);	/* root array */
    }

    0
}

unsafe extern "C" fn do_help(_argc: c_int, argv: *mut *mut c_char) -> c_int {
    if json_output {
        jsonw_null(json_wtr);
        return 0;
    }

    fprintf(stderr,
            cstr(b"Usage: %1$s %2$s probe [COMPONENT] [full] [unprivileged] [macros [prefix PREFIX]]\n       %1$s %2$s list_builtins GROUP\n       %1$s %2$s help\n\n       COMPONENT := { kernel | dev NAME }\n       GROUP := { prog_types | map_types | attach_types | link_types | helpers }\n       HELP_SPEC_OPTIONS }\n\0"),
            bin_name, *argv.offset(-2));

    0
}

static cmds: [cmd; 4] = [
    cmd { cmd: cstr(b"probe\0"), func: Some(do_probe) },
    cmd { cmd: cstr(b"list_builtins\0"), func: Some(do_list_builtins) },
    cmd { cmd: cstr(b"help\0"), func: Some(do_help) },
    cmd { cmd: ptr::null(), func: None },
];

#[no_mangle]
pub unsafe extern "C" fn do_feature(argc: c_int, argv: *mut *mut c_char) -> c_int {
    cmd_select(cmds.as_ptr(), argc, argv, do_help)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
