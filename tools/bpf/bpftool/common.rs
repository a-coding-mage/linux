// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (C) 2017-2018 Netronome Systems, Inc. */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type __u32 = u32;
type __u64 = u64;
type uint8_t = u8;
type gzFile = *mut c_void;
type FILE = c_void;
type va_list = *mut c_void;

const BPF_FS_MAGIC: c_ulong = 0xcafe4a11;
const ERR_MAX_LEN: usize = 1024;
const PATH_MAX: usize = 4096;
const IF_NAMESIZE: usize = 16;
const BPF_OBJ_NAME_LEN: usize = 16;
const BPF_TAG_SIZE: usize = 8;
const MAX_PROG_FULL_NAME: usize = 128;
const RLIMIT_MEMLOCK: c_int = 8;
const RLIM_INFINITY: c_ulong = !0;
const EACCES: c_int = 13;
const EINVAL: c_int = 22;
const ENAMETOOLONG: c_int = 36;
const ENOENT: c_int = 2;
const EPERM: c_int = 1;
const O_RDONLY: c_int = 0;
const F_OK: c_int = 0;
const S_IRWXU: c_uint = 0o700;
const MS_PRIVATE: c_ulong = 1 << 18;
const MS_REC: c_ulong = 16384;
const MS_BIND: c_ulong = 4096;
const FTW_F: c_int = 0;
const FTW_PHYS: c_int = 1;
const BPF_PROG_LOAD: c_int = 5;
const BPF_PROG_TYPE_SOCKET_FILTER: c_uint = 1;
const BPF_REG_0: c_uint = 0;
const BPF_F_RDONLY: __u32 = 8;
const __NR_bpf: c_long = 321;

#[repr(C)]
pub struct statfs {
    pub f_type: c_long,
}

#[repr(C)]
pub struct rlimit {
    pub rlim_cur: c_ulong,
    pub rlim_max: c_ulong,
}

#[repr(C)]
pub struct stat {
    pub st_dev: __u64,
    pub st_ino: __u64,
}

#[repr(C)]
pub struct FTW {
    pub base: c_int,
    pub level: c_int,
}

#[repr(C)]
pub struct mntent {
    pub mnt_fsname: *mut c_char,
    pub mnt_dir: *mut c_char,
    pub mnt_type: *mut c_char,
    pub mnt_opts: *mut c_char,
    pub mnt_freq: c_int,
    pub mnt_passno: c_int,
}

#[repr(C)]
pub struct utsname {
    pub sysname: [c_char; 65],
    pub nodename: [c_char; 65],
    pub release: [c_char; 65],
    pub version: [c_char; 65],
    pub machine: [c_char; 65],
}

#[repr(C)]
pub struct bpf_insn {
    pub code: u8,
    pub dst_src: u8,
    pub off: i16,
    pub imm: i32,
}

#[repr(C)]
pub union bpf_attr {
    pub prog_load: bpf_attr_prog_load,
    _bindgen_union_align: [u64; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_attr_prog_load {
    pub prog_type: c_uint,
    pub insn_cnt: c_uint,
    pub insns: __u64,
    pub license: __u64,
}

#[repr(C)]
pub struct bpf_obj_get_opts {
    pub sz: size_t,
    pub file_flags: __u32,
}

#[repr(C)]
pub struct bpf_get_fd_by_id_opts {
    pub sz: size_t,
    pub open_flags: __u32,
}

#[repr(C)]
pub struct bpf_func_info {
    pub insn_off: __u32,
    pub type_id: __u32,
}

#[repr(C)]
pub struct bpf_prog_info {
    pub type_: __u32,
    pub id: __u32,
    pub tag: [u8; BPF_TAG_SIZE],
    pub jited_prog_len: __u32,
    pub xlated_prog_len: __u32,
    pub jited_prog_insns: __u64,
    pub xlated_prog_insns: __u64,
    pub load_time: __u64,
    pub created_by_uid: __u32,
    pub nr_map_ids: __u32,
    pub map_ids: __u64,
    pub name: [c_char; BPF_OBJ_NAME_LEN],
    pub ifindex: __u32,
    pub gpl_compatible: __u32,
    pub netns_dev: __u64,
    pub netns_ino: __u64,
    pub nr_jited_ksyms: __u32,
    pub nr_jited_func_lens: __u32,
    pub jited_ksyms: __u64,
    pub jited_func_lens: __u64,
    pub btf_id: __u32,
    pub func_info_rec_size: __u32,
    pub func_info: __u64,
    pub nr_func_info: __u32,
}

#[repr(C)]
pub struct bpf_map_info {
    pub type_: __u32,
    pub id: __u32,
    pub key_size: __u32,
    pub value_size: __u32,
    pub max_entries: __u32,
    pub map_flags: __u32,
    pub name: [c_char; BPF_OBJ_NAME_LEN],
}

#[repr(C)]
pub struct btf_type {
    pub name_off: __u32,
    pub info: __u32,
    pub size: __u32,
}

#[repr(C)]
pub struct btf {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct hashmap_entry {
    pub key: c_long,
    pub pvalue: *mut c_void,
}

#[repr(C)]
pub struct kernel_config_option {
    pub name: *const c_char,
    pub macro_dump: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum bpf_obj_type {
    BPF_OBJ_UNKNOWN = 0,
    BPF_OBJ_PROG = 1,
    BPF_OBJ_MAP = 2,
    BPF_OBJ_LINK = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum libbpf_print_level {
    LIBBPF_WARN = 0,
    LIBBPF_INFO = 1,
    LIBBPF_DEBUG = 2,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum bpf_attach_type {
    BPF_CGROUP_INET_INGRESS = 0,
    BPF_CGROUP_INET_EGRESS,
    BPF_CGROUP_INET_SOCK_CREATE,
    BPF_CGROUP_SOCK_OPS,
    BPF_SK_SKB_STREAM_PARSER,
    BPF_SK_SKB_STREAM_VERDICT,
    BPF_CGROUP_DEVICE,
    BPF_SK_MSG_VERDICT,
    BPF_CGROUP_INET4_BIND,
    BPF_CGROUP_INET6_BIND,
    BPF_CGROUP_INET4_CONNECT,
    BPF_CGROUP_INET6_CONNECT,
    BPF_CGROUP_INET4_POST_BIND,
    BPF_CGROUP_INET6_POST_BIND,
    BPF_CGROUP_UDP4_SENDMSG,
    BPF_CGROUP_UDP6_SENDMSG,
    BPF_LIRC_MODE2,
    BPF_FLOW_DISSECTOR,
    BPF_CGROUP_SYSCTL,
    BPF_CGROUP_UDP4_RECVMSG,
    BPF_CGROUP_UDP6_RECVMSG,
    BPF_CGROUP_GETSOCKOPT,
    BPF_CGROUP_SETSOCKOPT,
    BPF_TRACE_RAW_TP,
    BPF_TRACE_FENTRY,
    BPF_TRACE_FEXIT,
    BPF_MODIFY_RETURN,
    BPF_LSM_MAC,
    BPF_TRACE_ITER,
    BPF_CGROUP_INET4_GETPEERNAME,
    BPF_CGROUP_INET6_GETPEERNAME,
    BPF_CGROUP_INET4_GETSOCKNAME,
    BPF_CGROUP_INET6_GETSOCKNAME,
    BPF_XDP_DEVMAP,
    BPF_CGROUP_INET_SOCK_RELEASE,
    BPF_XDP_CPUMAP,
    BPF_SK_LOOKUP,
    BPF_XDP,
    BPF_SK_SKB_VERDICT,
    BPF_SK_REUSEPORT_SELECT,
    BPF_SK_REUSEPORT_SELECT_OR_MIGRATE,
    BPF_PERF_EVENT,
    BPF_TRACE_KPROBE_MULTI,
    BPF_LSM_CGROUP,
    BPF_STRUCT_OPS,
    BPF_NETFILTER,
    BPF_TCX_INGRESS,
    BPF_TCX_EGRESS,
    BPF_TRACE_UPROBE_MULTI,
    BPF_CGROUP_UNIX_CONNECT,
    BPF_CGROUP_UNIX_SENDMSG,
    BPF_CGROUP_UNIX_RECVMSG,
    BPF_CGROUP_UNIX_GETPEERNAME,
    BPF_CGROUP_UNIX_GETSOCKNAME,
    BPF_NETKIT_PRIMARY,
    BPF_NETKIT_PEER,
    BPF_TRACE_FSESSION,
}

unsafe extern "C" {
    static mut json_output: bool;
    static mut json_wtr: *mut c_void;
    static mut block_mount: bool;
    fn jsonw_start_object(w: *mut c_void);
    fn jsonw_name(w: *mut c_void, name: *const c_char);
    fn jsonw_vprintf_enquote(w: *mut c_void, fmt: *const c_char, ap: va_list);
    fn jsonw_end_object(w: *mut c_void);
    fn jsonw_start_array(w: *mut c_void);
    fn jsonw_printf(w: *mut c_void, fmt: *const c_char, ...);
    fn jsonw_end_array(w: *mut c_void);
    fn jsonw_uint_field(w: *mut c_void, name: *const c_char, val: __u64);
    fn jsonw_string_field(w: *mut c_void, name: *const c_char, val: *const c_char);
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    fn vfprintf(stream: *mut FILE, fmt: *const c_char, ap: va_list) -> c_int;
    static mut stderr: *mut FILE;
    fn snprintf(s: *mut c_char, n: size_t, fmt: *const c_char, ...) -> c_int;
    fn statfs(path: *const c_char, buf: *mut statfs) -> c_int;
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn close(fd: c_int) -> c_int;
    fn mount(source: *const c_char, target: *const c_char, filesystemtype: *const c_char, mountflags: c_ulong, data: *const c_void) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn __errno_location() -> *mut c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn dirname(path: *mut c_char) -> *mut c_char;
    fn access(path: *const c_char, mode: c_int) -> c_int;
    fn mkdir(path: *const c_char, mode: c_uint) -> c_int;
    fn rmdir(path: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn memmove(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: size_t) -> ssize_t;
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getline(lineptr: *mut *mut c_char, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    fn setmntent(filename: *const c_char, type_: *const c_char) -> *mut FILE;
    fn getmntent(stream: *mut FILE) -> *mut mntent;
    fn nftw(dirpath: *const c_char, fn_: Option<unsafe extern "C" fn(*const c_char, *const stat, c_int, *mut FTW) -> c_int>, nopenfd: c_int, flags: c_int) -> c_int;
    fn getpagesize() -> c_int;
    fn exit(status: c_int) -> !;
    fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    fn if_indextoname(ifindex: c_uint, ifname: *mut c_char) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn strtol(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_long;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn printf(fmt: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, fmt: *const c_char, ...) -> c_int;
    fn assert_fail(assertion: *const c_char, file: *const c_char, line: c_uint, function: *const c_char) -> !;
    fn uname(buf: *mut utsname) -> c_int;
    fn gzopen(path: *const c_char, mode: *const c_char) -> gzFile;
    fn gzgets(file: gzFile, buf: *mut c_char, len: c_int) -> *mut c_char;
    fn gzclose(file: gzFile) -> c_int;
    fn bpf_obj_get_opts(path: *const c_char, opts: *const bpf_obj_get_opts) -> c_int;
    fn bpf_obj_pin(fd: c_int, pathname: *const c_char) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, info_len: *mut __u32) -> c_int;
    fn bpf_prog_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    fn bpf_prog_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_map_get_next_id(start_id: __u32, next_id: *mut __u32) -> c_int;
    fn bpf_map_get_fd_by_id_opts(id: __u32, opts: *const bpf_get_fd_by_id_opts) -> c_int;
    fn bpf_map_get_info_by_fd(fd: c_int, info: *mut bpf_map_info, info_len: *mut __u32) -> c_int;
    fn btf__load_from_kernel_by_id(id: __u32) -> *mut btf;
    fn btf__type_by_id(btf: *const btf, type_id: __u32) -> *const btf_type;
    fn btf_is_func(t: *const btf_type) -> bool;
    fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn btf__free(btf: *mut btf);
    fn libbpf_num_possible_cpus() -> c_int;
    fn hashmap__append(map: *mut hashmap, key: __u32, value: *mut c_void) -> c_int;
    fn hashmap__free(map: *mut hashmap);
    fn hashmap__for_each_entry_shim(map: *mut hashmap, cb: unsafe extern "C" fn(*mut hashmap_entry, *mut c_void), ctx: *mut c_void);
    fn libbpf_bpf_attach_type_str(t: bpf_attach_type) -> *const c_char;
    fn is_prefix(str_: *const c_char, prefix: *const c_char) -> bool;
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

unsafe fn ptr_to_u64<T>(ptr: *const T) -> __u64 {
    ptr as usize as __u64
}

unsafe fn next_argp(argc: *mut c_int, argv: *mut *mut *mut c_char) {
    *argc -= 1;
    *argv = (*argv).add(1);
}

unsafe fn req_args(argc: c_int, required: c_int) -> bool {
    argc >= required
}

unsafe fn bpf_mov64_imm(dst: c_uint, imm: i32) -> bpf_insn {
    bpf_insn { code: 0xb7, dst_src: (dst as u8) << 4, off: 0, imm }
}

unsafe fn bpf_exit_insn() -> bpf_insn {
    bpf_insn { code: 0x95, dst_src: 0, off: 0, imm: 0 }
}

/* Rust cannot define C-variadic functions on stable Rust. This declaration
 * preserves the external interface and dependency intent for the translated
 * source-level pass.
 */
unsafe extern "C" {
    pub fn p_err(fmt: *const c_char, ...);
    pub fn p_info(fmt: *const c_char, ...);
}

unsafe fn is_bpffs(path: *const c_char) -> bool {
    let mut st_fs: statfs = zeroed();

    if statfs(path, &mut st_fs) < 0 {
        return false;
    }

    st_fs.f_type as c_ulong == BPF_FS_MAGIC
}

/* Probe whether kernel switched from memlock-based (RLIMIT_MEMLOCK) to
 * memcg-based memory accounting for BPF maps and programs. This was done in
 * commit 97306be45fbe ("Merge branch 'switch to memcg-based memory
 * accounting'"), in Linux 5.11.
 *
 * Libbpf also offers to probe for memcg-based accounting vs rlimit, but does
 * so by checking for the availability of a given BPF helper and this has
 * failed on some kernels with backports in the past, see commit 6b4384ff1088
 * ("Revert "bpftool: Use libbpf 1.0 API mode instead of RLIMIT_MEMLOCK"").
 * Instead, we can probe by lowering the process-based rlimit to 0, trying to
 * load a BPF object, and resetting the rlimit. If the load succeeds then
 * memcg-based accounting is supported.
 *
 * This would be too dangerous to do in the library, because multithreaded
 * applications might attempt to load items while the rlimit is at 0. Given
 * that bpftool is single-threaded, this is fine to do here.
 */
unsafe fn known_to_need_rlimit() -> bool {
    let mut rlim_init: rlimit = zeroed();
    let mut rlim_cur_zero: rlimit = zeroed();
    let mut insns = [bpf_mov64_imm(BPF_REG_0, 0), bpf_exit_insn()];
    let insn_cnt = insns.len();
    let mut attr: bpf_attr = zeroed();
    let prog_fd: c_int;
    let err: c_int;

    memset(&mut attr as *mut _ as *mut c_void, 0, size_of::<bpf_attr>());
    attr.prog_load.prog_type = BPF_PROG_TYPE_SOCKET_FILTER;
    attr.prog_load.insns = ptr_to_u64(insns.as_mut_ptr());
    attr.prog_load.insn_cnt = insn_cnt as c_uint;
    attr.prog_load.license = ptr_to_u64(c"GPL".as_ptr());

    if getrlimit(RLIMIT_MEMLOCK, &mut rlim_init) != 0 {
        return false;
    }

    /* Drop the soft limit to zero. We maintain the hard limit to its
     * current value, because lowering it would be a permanent operation
     * for unprivileged users.
     */
    rlim_cur_zero.rlim_max = rlim_init.rlim_max;
    if setrlimit(RLIMIT_MEMLOCK, &rlim_cur_zero) != 0 {
        return false;
    }

    /* Do not use bpf_prog_load() from libbpf here, because it calls
     * bump_rlimit_memlock(), interfering with the current probe.
     */
    prog_fd = syscall(__NR_bpf, BPF_PROG_LOAD, &mut attr, size_of::<bpf_attr>()) as c_int;
    err = errno();

    /* reset soft rlimit to its initial value */
    setrlimit(RLIMIT_MEMLOCK, &rlim_init);

    if prog_fd < 0 {
        return err == EPERM;
    }

    close(prog_fd);
    false
}

#[no_mangle]
pub unsafe extern "C" fn set_max_rlimit() {
    let rinf = rlimit { rlim_cur: RLIM_INFINITY, rlim_max: RLIM_INFINITY };

    if known_to_need_rlimit() {
        setrlimit(RLIMIT_MEMLOCK, &rinf);
    }
}

unsafe fn mnt_fs(target: *const c_char, type_: *const c_char, buff: *mut c_char, bufflen: size_t) -> c_int {
    let mut bind_done = false;

    while mount(c"".as_ptr(), target, c"none".as_ptr(), MS_PRIVATE | MS_REC, ptr::null()) != 0 {
        if errno() != EINVAL || bind_done {
            snprintf(buff, bufflen, c"mount --make-private %s failed: %s".as_ptr(), target, strerror(errno()));
            return -1;
        }

        if mount(target, target, c"none".as_ptr(), MS_BIND, ptr::null()) != 0 {
            snprintf(buff, bufflen, c"mount --bind %s %s failed: %s".as_ptr(), target, target, strerror(errno()));
            return -1;
        }

        bind_done = true;
    }

    if mount(type_, target, type_, 0, c"mode=0700".as_ptr() as *const c_void) != 0 {
        snprintf(buff, bufflen, c"mount -t %s %s %s failed: %s".as_ptr(), type_, type_, target, strerror(errno()));
        return -1;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn mount_tracefs(target: *const c_char) -> c_int {
    let mut err_str = [0 as c_char; ERR_MAX_LEN];
    let err = mnt_fs(target, c"tracefs".as_ptr(), err_str.as_mut_ptr(), ERR_MAX_LEN);
    if err != 0 {
        err_str[ERR_MAX_LEN - 1] = 0;
        p_err(c"can't mount tracefs: %s".as_ptr(), err_str.as_ptr());
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn open_obj_pinned(path: *const c_char, quiet: bool, opts: *const bpf_obj_get_opts) -> c_int {
    let pname: *mut c_char;
    let mut fd: c_int = -1;

    pname = strdup(path);
    if pname.is_null() {
        if !quiet {
            p_err(c"mem alloc failed".as_ptr());
        }
        return fd;
    }

    fd = bpf_obj_get_opts(pname, opts);
    if fd < 0 {
        if !quiet {
            let msg = if errno() == EACCES && !is_bpffs(dirname(pname)) {
                c"directory not in bpf file system (bpffs)".as_ptr() as *mut c_char
            } else {
                strerror(errno())
            };
            p_err(c"bpf obj get (%s): %s".as_ptr(), pname, msg);
        }
        free(pname as *mut c_void);
        return fd;
    }

    free(pname as *mut c_void);
    fd
}

#[no_mangle]
pub unsafe extern "C" fn open_obj_pinned_any(path: *const c_char, exp_type: bpf_obj_type, opts: *const bpf_obj_get_opts) -> c_int {
    let type_: bpf_obj_type;
    let fd = open_obj_pinned(path, false, opts);
    if fd < 0 {
        return -1;
    }

    type_ = core::mem::transmute(get_fd_type(fd));
    if (type_ as c_int) < 0 {
        close(fd);
        return type_ as c_int;
    }
    if type_ as c_int != exp_type as c_int {
        p_err(c"incorrect object type: %s".as_ptr(), get_fd_type_name(type_));
        close(fd);
        return -1;
    }

    fd
}

#[no_mangle]
pub unsafe extern "C" fn create_and_mount_bpffs_dir(dir_name: *const c_char) -> c_int {
    let mut err_str = [0 as c_char; ERR_MAX_LEN];
    let dir_exists: bool;
    let mut err: c_int = 0;

    if is_bpffs(dir_name) {
        return err;
    }

    dir_exists = access(dir_name, F_OK) == 0;

    if !dir_exists {
        let temp_name = strdup(dir_name);
        if temp_name.is_null() {
            p_err(c"mem alloc failed".as_ptr());
            return -1;
        }

        let parent_name = dirname(temp_name);

        if is_bpffs(parent_name) {
            /* nothing to do if already mounted */
            free(temp_name as *mut c_void);
            return err;
        }

        if access(parent_name, F_OK) == -1 {
            p_err(c"can't create dir '%s' to pin BPF object: parent dir '%s' doesn't exist".as_ptr(), dir_name, parent_name);
            free(temp_name as *mut c_void);
            return -1;
        }

        free(temp_name as *mut c_void);
    }

    if block_mount {
        p_err(c"no BPF file system found, not mounting it due to --nomount option".as_ptr());
        return -1;
    }

    if !dir_exists {
        err = mkdir(dir_name, S_IRWXU);
        if err != 0 {
            p_err(c"failed to create dir '%s': %s".as_ptr(), dir_name, strerror(errno()));
            return err;
        }
    }

    err = mnt_fs(dir_name, c"bpf".as_ptr(), err_str.as_mut_ptr(), ERR_MAX_LEN);
    if err != 0 {
        err_str[ERR_MAX_LEN - 1] = 0;
        p_err(c"can't mount BPF file system on given dir '%s': %s".as_ptr(), dir_name, err_str.as_ptr());

        if !dir_exists {
            rmdir(dir_name);
        }
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn mount_bpffs_for_file(file_name: *const c_char) -> c_int {
    let mut err_str = [0 as c_char; ERR_MAX_LEN];
    let temp_name: *mut c_char;
    let dir: *mut c_char;
    let mut err: c_int = 0;

    if access(file_name, F_OK) != -1 {
        p_err(c"can't pin BPF object: path '%s' already exists".as_ptr(), file_name);
        return -1;
    }

    temp_name = strdup(file_name);
    if temp_name.is_null() {
        p_err(c"mem alloc failed".as_ptr());
        return -1;
    }

    dir = dirname(temp_name);

    if is_bpffs(dir) {
        free(temp_name as *mut c_void);
        return err;
    }

    if access(dir, F_OK) == -1 {
        p_err(c"can't pin BPF object: dir '%s' doesn't exist".as_ptr(), dir);
        err = -1;
        free(temp_name as *mut c_void);
        return err;
    }

    if block_mount {
        p_err(c"no BPF file system found, not mounting it due to --nomount option".as_ptr());
        err = -1;
        free(temp_name as *mut c_void);
        return err;
    }

    err = mnt_fs(dir, c"bpf".as_ptr(), err_str.as_mut_ptr(), ERR_MAX_LEN);
    if err != 0 {
        err_str[ERR_MAX_LEN - 1] = 0;
        p_err(c"can't mount BPF file system to pin the object '%s': %s".as_ptr(), file_name, err_str.as_ptr());
    }

    free(temp_name as *mut c_void);
    err
}

#[no_mangle]
pub unsafe extern "C" fn do_pin_fd(fd: c_int, name: *const c_char) -> c_int {
    let mut err = mount_bpffs_for_file(name);
    if err != 0 {
        return err;
    }

    err = bpf_obj_pin(fd, name);
    if err != 0 {
        p_err(c"can't pin the object (%s): %s".as_ptr(), name, strerror(errno()));
    }

    err
}

#[no_mangle]
pub unsafe extern "C" fn do_pin_any(argc: c_int, argv: *mut *mut c_char, get_fd: Option<unsafe extern "C" fn(*mut c_int, *mut *mut *mut c_char) -> c_int>) -> c_int {
    let mut argc = argc;
    let mut argv = argv;
    let fd: c_int;

    if !req_args(argc, 3) {
        return -EINVAL;
    }

    fd = get_fd.unwrap()(&mut argc, &mut argv);
    if fd < 0 {
        return fd;
    }

    let err = do_pin_fd(fd, *argv);
    close(fd);
    err
}

#[no_mangle]
pub unsafe extern "C" fn get_fd_type_name(type_: bpf_obj_type) -> *const c_char {
    static names: [*const c_char; 4] = [
        c"unknown".as_ptr(),
        c"prog".as_ptr(),
        c"map".as_ptr(),
        c"link".as_ptr(),
    ];

    let idx = type_ as isize;
    if idx < 0 || idx as usize >= names.len() || names[idx as usize].is_null() {
        return names[bpf_obj_type::BPF_OBJ_UNKNOWN as usize];
    }

    names[idx as usize]
}

#[no_mangle]
pub unsafe extern "C" fn get_prog_full_name(prog_info: *const bpf_prog_info, prog_fd: c_int, name_buff: *mut c_char, buff_len: size_t) {
    let mut prog_name = (*prog_info).name.as_ptr();
    let func_type: *const btf_type;
    let mut finfo: bpf_func_info = zeroed();
    let mut info: bpf_prog_info = zeroed();
    let mut info_len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let mut prog_btf: *mut btf = ptr::null_mut();

    if buff_len <= BPF_OBJ_NAME_LEN || strlen((*prog_info).name.as_ptr()) < BPF_OBJ_NAME_LEN - 1 {
        snprintf(name_buff, buff_len, c"%s".as_ptr(), prog_name);
        return;
    }

    if (*prog_info).btf_id == 0 || (*prog_info).nr_func_info == 0 {
        snprintf(name_buff, buff_len, c"%s".as_ptr(), prog_name);
        return;
    }

    info.nr_func_info = 1;
    info.func_info_rec_size = (*prog_info).func_info_rec_size;
    if info.func_info_rec_size as usize > size_of::<bpf_func_info>() {
        info.func_info_rec_size = size_of::<bpf_func_info>() as __u32;
    }
    info.func_info = ptr_to_u64(&mut finfo);

    if bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut info_len) != 0 {
        snprintf(name_buff, buff_len, c"%s".as_ptr(), prog_name);
        return;
    }

    prog_btf = btf__load_from_kernel_by_id(info.btf_id);
    if prog_btf.is_null() {
        snprintf(name_buff, buff_len, c"%s".as_ptr(), prog_name);
        return;
    }

    func_type = btf__type_by_id(prog_btf, finfo.type_id);
    if !func_type.is_null() && btf_is_func(func_type) {
        prog_name = btf__name_by_offset(prog_btf, (*func_type).name_off);
    }

    snprintf(name_buff, buff_len, c"%s".as_ptr(), prog_name);

    if !prog_btf.is_null() {
        btf__free(prog_btf);
    }
}

#[no_mangle]
pub unsafe extern "C" fn get_fd_type(fd: c_int) -> c_int {
    let mut path = [0 as c_char; PATH_MAX];
    let mut buf = [0 as c_char; 512];

    snprintf(path.as_mut_ptr(), path.len(), c"/proc/self/fd/%d".as_ptr(), fd);

    let n = readlink(path.as_ptr(), buf.as_mut_ptr(), buf.len());
    if n < 0 {
        p_err(c"can't read link type: %s".as_ptr(), strerror(errno()));
        return -1;
    }
    if n as usize == buf.len() {
        p_err(c"can't read link type: path too long!".as_ptr());
        return -1;
    }
    buf[n as usize] = 0;

    if !strstr(buf.as_ptr(), c"bpf-map".as_ptr()).is_null() {
        return bpf_obj_type::BPF_OBJ_MAP as c_int;
    } else if !strstr(buf.as_ptr(), c"bpf-prog".as_ptr()).is_null() {
        return bpf_obj_type::BPF_OBJ_PROG as c_int;
    } else if !strstr(buf.as_ptr(), c"bpf-link".as_ptr()).is_null() {
        return bpf_obj_type::BPF_OBJ_LINK as c_int;
    }

    bpf_obj_type::BPF_OBJ_UNKNOWN as c_int
}

#[no_mangle]
pub unsafe extern "C" fn get_fdinfo(fd: c_int, key: *const c_char) -> *mut c_char {
    let mut path = [0 as c_char; PATH_MAX];
    let mut line: *mut c_char = ptr::null_mut();
    let mut line_n: size_t = 0;

    snprintf(path.as_mut_ptr(), path.len(), c"/proc/self/fdinfo/%d".as_ptr(), fd);

    let fdi = fopen(path.as_ptr(), c"r".as_ptr());
    if fdi.is_null() {
        return ptr::null_mut();
    }

    while getline(&mut line, &mut line_n, fdi) > 0 {
        if strstr(line, key).is_null() {
            continue;
        }

        fclose(fdi);

        let mut value = strchr(line, '\t' as c_int);
        if value.is_null() || *value.add(1) == 0 {
            free(line as *mut c_void);
            return ptr::null_mut();
        }
        value = value.add(1);

        let len = strlen(value);
        memmove(line as *mut c_void, value as *const c_void, len);
        *line.add(len - 1) = 0;

        return line;
    }

    free(line as *mut c_void);
    fclose(fdi);
    ptr::null_mut()
}

#[no_mangle]
pub unsafe extern "C" fn print_data_json(data: *mut uint8_t, len: size_t) {
    jsonw_start_array(json_wtr);
    for i in 0..len {
        jsonw_printf(json_wtr, c"%d".as_ptr(), *data.add(i) as c_int);
    }
    jsonw_end_array(json_wtr);
}

#[no_mangle]
pub unsafe extern "C" fn print_hex_data_json(data: *mut uint8_t, len: size_t) {
    jsonw_start_array(json_wtr);
    for i in 0..len {
        jsonw_printf(json_wtr, c"\"0x%02hhx\"".as_ptr(), *data.add(i) as c_int);
    }
    jsonw_end_array(json_wtr);
}

/* extra params for nftw cb */
static mut build_fn_table: *mut hashmap = ptr::null_mut();
static mut build_fn_type: bpf_obj_type = bpf_obj_type::BPF_OBJ_UNKNOWN;

unsafe extern "C" fn do_build_table_cb(fpath: *const c_char, _sb: *const stat, typeflag: c_int, _ftwbuf: *mut FTW) -> c_int {
    let mut pinned_info: bpf_prog_info = zeroed();
    let mut len: __u32 = size_of::<bpf_prog_info>() as __u32;
    let objtype: bpf_obj_type;
    let fd: c_int;
    let mut err: c_int = 0;
    let path: *mut c_char;

    if typeflag != FTW_F {
        return err;
    }

    fd = open_obj_pinned(fpath, true, ptr::null());
    if fd < 0 {
        return err;
    }

    objtype = core::mem::transmute(get_fd_type(fd));
    if objtype as c_int != build_fn_type as c_int {
        close(fd);
        return err;
    }

    memset(&mut pinned_info as *mut _ as *mut c_void, 0, size_of::<bpf_prog_info>());
    if bpf_prog_get_info_by_fd(fd, &mut pinned_info, &mut len) != 0 {
        close(fd);
        return err;
    }

    path = strdup(fpath);
    if path.is_null() {
        err = -1;
        close(fd);
        return err;
    }

    err = hashmap__append(build_fn_table, pinned_info.id, path as *mut c_void);
    if err != 0 {
        p_err(c"failed to append entry to hashmap for ID %u, path '%s': %s".as_ptr(), pinned_info.id, path, strerror(errno()));
        free(path as *mut c_void);
    }

    close(fd);
    err
}

#[no_mangle]
pub unsafe extern "C" fn build_pinned_obj_table(tab: *mut hashmap, type_: bpf_obj_type) -> c_int {
    let mut mntfile: *mut FILE = ptr::null_mut();
    let flags = FTW_PHYS;
    let nopenfd = 16;
    let mut err = 0;

    mntfile = setmntent(c"/proc/mounts".as_ptr(), c"r".as_ptr());
    if mntfile.is_null() {
        return -1;
    }

    build_fn_table = tab;
    build_fn_type = type_;

    loop {
        let mntent = getmntent(mntfile);
        if mntent.is_null() {
            break;
        }
        let path = (*mntent).mnt_dir;

        if strncmp((*mntent).mnt_type, c"bpf".as_ptr(), 3) != 0 {
            continue;
        }
        err = nftw(path, Some(do_build_table_cb), nopenfd, flags);
        if err != 0 {
            break;
        }
    }
    fclose(mntfile);
    err
}

unsafe extern "C" fn free_hashmap_entry(entry: *mut hashmap_entry, _ctx: *mut c_void) {
    free((*entry).pvalue);
}

#[no_mangle]
pub unsafe extern "C" fn delete_pinned_obj_table(map: *mut hashmap) {
    if map.is_null() {
        return;
    }

    hashmap__for_each_entry_shim(map, free_hashmap_entry, ptr::null_mut());
    hashmap__free(map);
}

#[no_mangle]
pub unsafe extern "C" fn get_page_size() -> c_uint {
    static mut result: c_int = 0;

    if result == 0 {
        result = getpagesize();
    }
    result as c_uint
}

#[no_mangle]
pub unsafe extern "C" fn get_possible_cpus() -> c_uint {
    let cpus = libbpf_num_possible_cpus();

    if cpus < 0 {
        p_err(c"Can't get # of possible cpus: %s".as_ptr(), strerror(-cpus));
        exit(-1);
    }
    cpus as c_uint
}

unsafe fn ifindex_to_name_ns(ifindex: __u32, ns_dev: __u32, ns_ino: __u32, buf: *mut c_char) -> *mut c_char {
    let mut st: stat = zeroed();
    let err = stat(c"/proc/self/ns/net".as_ptr(), &mut st);
    if err != 0 {
        p_err(c"Can't stat /proc/self: %s".as_ptr(), strerror(errno()));
        return ptr::null_mut();
    }

    if st.st_dev != ns_dev as __u64 || st.st_ino != ns_ino as __u64 {
        return ptr::null_mut();
    }

    if_indextoname(ifindex, buf)
}

unsafe fn read_sysfs_hex_int(path: *mut c_char) -> c_int {
    let mut vendor_id_buf = [0 as c_char; 8];
    let fd = open(path, O_RDONLY);
    if fd < 0 {
        p_err(c"Can't open %s: %s".as_ptr(), path, strerror(errno()));
        return -1;
    }

    let len = read(fd, vendor_id_buf.as_mut_ptr() as *mut c_void, vendor_id_buf.len());
    close(fd);
    if len < 0 {
        p_err(c"Can't read %s: %s".as_ptr(), path, strerror(errno()));
        return -1;
    }
    if len >= vendor_id_buf.len() as isize {
        p_err(c"Value in %s too long".as_ptr(), path);
        return -1;
    }

    vendor_id_buf[len as usize] = 0;

    strtol(vendor_id_buf.as_ptr(), ptr::null_mut(), 0) as c_int
}

unsafe fn read_sysfs_netdev_hex_int(devname: *mut c_char, entry_name: *const c_char) -> c_int {
    let mut full_path = [0 as c_char; 64];

    snprintf(full_path.as_mut_ptr(), full_path.len(), c"/sys/class/net/%s/device/%s".as_ptr(), devname, entry_name);

    read_sysfs_hex_int(full_path.as_mut_ptr())
}

#[no_mangle]
pub unsafe extern "C" fn ifindex_to_arch(ifindex: __u32, ns_dev: __u64, ns_ino: __u64, opt: *mut *const c_char) -> *const c_char {
    let mut devname = [0 as c_char; IF_NAMESIZE];
    let vendor_id: c_int;

    if ifindex_to_name_ns(ifindex, ns_dev as __u32, ns_ino as __u32, devname.as_mut_ptr()).is_null() {
        p_err(c"Can't get net device name for ifindex %u: %s".as_ptr(), ifindex, strerror(errno()));
        return ptr::null();
    }

    vendor_id = read_sysfs_netdev_hex_int(devname.as_mut_ptr(), c"vendor".as_ptr());
    if vendor_id < 0 {
        p_err(c"Can't get device vendor id for %s".as_ptr(), devname.as_ptr());
        return ptr::null();
    }

    match vendor_id {
        /* HAVE_LIBBFD_SUPPORT:
         * case 0x19ee reads "device", optionally reports unknown NFP IDs,
         * sets *opt = "ctx4", and returns "NFP-6xxx".
         */
        _ => {
            /* No NFP support in LLVM, we have no valid triple to return. */
            p_err(c"Can't get arch name for device vendor id 0x%04x".as_ptr(), vendor_id as c_uint);
            ptr::null()
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn print_dev_plain(ifindex: __u32, ns_dev: __u64, ns_inode: __u64) {
    let mut name = [0 as c_char; IF_NAMESIZE];

    if ifindex == 0 {
        return;
    }

    printf(c"  offloaded_to ".as_ptr());
    if !ifindex_to_name_ns(ifindex, ns_dev as __u32, ns_inode as __u32, name.as_mut_ptr()).is_null() {
        printf(c"%s".as_ptr(), name.as_ptr());
    } else {
        printf(c"ifindex %u ns_dev %llu ns_ino %llu".as_ptr(), ifindex, ns_dev, ns_inode);
    }
}

#[no_mangle]
pub unsafe extern "C" fn print_dev_json(ifindex: __u32, ns_dev: __u64, ns_inode: __u64) {
    let mut name = [0 as c_char; IF_NAMESIZE];

    if ifindex == 0 {
        return;
    }

    jsonw_name(json_wtr, c"dev".as_ptr());
    jsonw_start_object(json_wtr);
    jsonw_uint_field(json_wtr, c"ifindex".as_ptr(), ifindex as __u64);
    jsonw_uint_field(json_wtr, c"ns_dev".as_ptr(), ns_dev);
    jsonw_uint_field(json_wtr, c"ns_inode".as_ptr(), ns_inode);
    if !ifindex_to_name_ns(ifindex, ns_dev as __u32, ns_inode as __u32, name.as_mut_ptr()).is_null() {
        jsonw_string_field(json_wtr, c"ifname".as_ptr(), name.as_ptr());
    }
    jsonw_end_object(json_wtr);
}

#[no_mangle]
pub unsafe extern "C" fn parse_u32_arg(argc: *mut c_int, argv: *mut *mut *mut c_char, val: *mut __u32, what: *const c_char) -> c_int {
    let mut endptr: *mut c_char = ptr::null_mut();

    next_argp(argc, argv);

    if *val != 0 {
        p_err(c"%s already specified".as_ptr(), what);
        return -1;
    }

    *val = strtoul(**argv, &mut endptr, 0) as __u32;
    if *endptr != 0 {
        p_err(c"can't parse %s as %s".as_ptr(), **argv, what);
        return -1;
    }
    next_argp(argc, argv);

    0
}

#[no_mangle]
pub unsafe extern "C" fn print_all_levels(_level: libbpf_print_level, format: *const c_char, args: va_list) -> c_int {
    vfprintf(stderr, format, args)
}

unsafe fn prog_fd_by_nametag(nametag: *mut c_void, fds: *mut *mut c_int, tag: bool) -> c_int {
    let mut prog_name = [0 as c_char; MAX_PROG_FULL_NAME];
    let mut id: c_uint = 0;
    let mut nb_fds: c_int = 0;
    let mut fd: c_int;

    loop {
        let mut info: bpf_prog_info = zeroed();
        let mut len: __u32 = size_of::<bpf_prog_info>() as __u32;

        let err = bpf_prog_get_next_id(id, &mut id);
        if err != 0 {
            if errno() != ENOENT {
                p_err(c"%s".as_ptr(), strerror(errno()));
                while { nb_fds -= 1; nb_fds >= 0 } {
                    close(*(*fds).add(nb_fds as usize));
                }
                return -1;
            }
            return nb_fds;
        }

        fd = bpf_prog_get_fd_by_id(id);
        if fd < 0 {
            if errno() == ENOENT {
                continue;
            }
            p_err(c"can't get prog by id (%u): %s".as_ptr(), id, strerror(errno()));
            while { nb_fds -= 1; nb_fds >= 0 } {
                close(*(*fds).add(nb_fds as usize));
            }
            return -1;
        }

        if bpf_prog_get_info_by_fd(fd, &mut info, &mut len) != 0 {
            p_err(c"can't get prog info (%u): %s".as_ptr(), id, strerror(errno()));
            close(fd);
            while { nb_fds -= 1; nb_fds >= 0 } {
                close(*(*fds).add(nb_fds as usize));
            }
            return -1;
        }

        if tag && memcmp(nametag, info.tag.as_ptr() as *const c_void, BPF_TAG_SIZE) != 0 {
            close(fd);
            continue;
        }

        if !tag {
            get_prog_full_name(&info, fd, prog_name.as_mut_ptr(), prog_name.len());
            if strncmp(nametag as *const c_char, prog_name.as_ptr(), prog_name.len()) != 0 {
                close(fd);
                continue;
            }
        }

        if nb_fds > 0 {
            let tmp = realloc(*fds as *mut c_void, (nb_fds as usize + 1) * size_of::<c_int>());
            if tmp.is_null() {
                p_err(c"failed to realloc".as_ptr());
                close(fd);
                while { nb_fds -= 1; nb_fds >= 0 } {
                    close(*(*fds).add(nb_fds as usize));
                }
                return -1;
            }
            *fds = tmp as *mut c_int;
        }
        *(*fds).add(nb_fds as usize) = fd;
        nb_fds += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn prog_parse_fds(argc: *mut c_int, argv: *mut *mut *mut c_char, fds: *mut *mut c_int) -> c_int {
    if is_prefix(**argv, c"id".as_ptr()) {
        let mut endptr: *mut c_char = ptr::null_mut();

        next_argp(argc, argv);
        let id = strtoul(**argv, &mut endptr, 0) as c_uint;
        if *endptr != 0 {
            p_err(c"can't parse %s as ID".as_ptr(), **argv);
            return -1;
        }
        next_argp(argc, argv);

        **fds = bpf_prog_get_fd_by_id(id);
        if **fds < 0 {
            p_err(c"get by id (%u): %s".as_ptr(), id, strerror(errno()));
            return -1;
        }
        return 1;
    } else if is_prefix(**argv, c"tag".as_ptr()) {
        let mut tag = [0u8; BPF_TAG_SIZE];

        next_argp(argc, argv);

        if sscanf(**argv, c"%2hhx%2hhx%2hhx%2hhx%2hhx%2hhx%2hhx%2hhx".as_ptr(), tag.as_mut_ptr(), tag.as_mut_ptr().add(1), tag.as_mut_ptr().add(2), tag.as_mut_ptr().add(3), tag.as_mut_ptr().add(4), tag.as_mut_ptr().add(5), tag.as_mut_ptr().add(6), tag.as_mut_ptr().add(7)) != BPF_TAG_SIZE as c_int {
            p_err(c"can't parse tag".as_ptr());
            return -1;
        }
        next_argp(argc, argv);

        return prog_fd_by_nametag(tag.as_mut_ptr() as *mut c_void, fds, true);
    } else if is_prefix(**argv, c"name".as_ptr()) {
        next_argp(argc, argv);

        let name = **argv;
        if strlen(name) > MAX_PROG_FULL_NAME - 1 {
            p_err(c"can't parse name".as_ptr());
            return -1;
        }
        next_argp(argc, argv);

        return prog_fd_by_nametag(name as *mut c_void, fds, false);
    } else if is_prefix(**argv, c"pinned".as_ptr()) {
        next_argp(argc, argv);

        let path = **argv;
        next_argp(argc, argv);

        **fds = open_obj_pinned_any(path, bpf_obj_type::BPF_OBJ_PROG, ptr::null());
        if **fds < 0 {
            return -1;
        }
        return 1;
    }

    p_err(c"expected 'id', 'tag', 'name' or 'pinned', got: '%s'?".as_ptr(), **argv);
    -1
}

#[no_mangle]
pub unsafe extern "C" fn prog_parse_fd(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int {
    let mut fds: *mut c_int = malloc(size_of::<c_int>()) as *mut c_int;
    let fd: c_int;
    if fds.is_null() {
        p_err(c"mem alloc failed".as_ptr());
        return -1;
    }
    let mut nb_fds = prog_parse_fds(argc, argv, &mut fds);
    if nb_fds != 1 {
        if nb_fds > 1 {
            p_err(c"several programs match this handle".as_ptr());
            while { nb_fds -= 1; nb_fds >= 0 } {
                close(*fds.add(nb_fds as usize));
            }
        }
        fd = -1;
        free(fds as *mut c_void);
        return fd;
    }

    fd = *fds;
    free(fds as *mut c_void);
    fd
}

unsafe fn map_fd_by_name(name: *mut c_char, fds: *mut *mut c_int, opts: *const bpf_get_fd_by_id_opts) -> c_int {
    let mut id: c_uint = 0;
    let mut nb_fds: c_int = 0;
    let mut fd: c_int;

    loop {
        let mut opts_ro = bpf_get_fd_by_id_opts { sz: size_of::<bpf_get_fd_by_id_opts>(), open_flags: 0 };
        let mut info: bpf_map_info = zeroed();
        let mut len: __u32 = size_of::<bpf_map_info>() as __u32;

        let err = bpf_map_get_next_id(id, &mut id);
        if err != 0 {
            if errno() != ENOENT {
                p_err(c"%s".as_ptr(), strerror(errno()));
                while { nb_fds -= 1; nb_fds >= 0 } {
                    close(*(*fds).add(nb_fds as usize));
                }
                return -1;
            }
            return nb_fds;
        }

        /* Request a read-only fd to query the map info */
        opts_ro.open_flags = BPF_F_RDONLY;
        fd = bpf_map_get_fd_by_id_opts(id, &opts_ro);
        if fd < 0 {
            if errno() == ENOENT {
                continue;
            }
            p_err(c"can't get map by id (%u): %s".as_ptr(), id, strerror(errno()));
            while { nb_fds -= 1; nb_fds >= 0 } {
                close(*(*fds).add(nb_fds as usize));
            }
            return -1;
        }

        if bpf_map_get_info_by_fd(fd, &mut info, &mut len) != 0 {
            p_err(c"can't get map info (%u): %s".as_ptr(), id, strerror(errno()));
            close(fd);
            while { nb_fds -= 1; nb_fds >= 0 } {
                close(*(*fds).add(nb_fds as usize));
            }
            return -1;
        }

        if strncmp(name, info.name.as_ptr(), BPF_OBJ_NAME_LEN) != 0 {
            close(fd);
            continue;
        }

        /* Get an fd with the requested options, if they differ
         * from the read-only options used to get the fd above.
         */
        if memcmp(opts as *const c_void, &opts_ro as *const _ as *const c_void, size_of::<bpf_get_fd_by_id_opts>()) != 0 {
            close(fd);
            fd = bpf_map_get_fd_by_id_opts(id, opts);
            if fd < 0 {
                p_err(c"can't get map by id (%u): %s".as_ptr(), id, strerror(errno()));
                while { nb_fds -= 1; nb_fds >= 0 } {
                    close(*(*fds).add(nb_fds as usize));
                }
                return -1;
            }
        }

        if nb_fds > 0 {
            let tmp = realloc(*fds as *mut c_void, (nb_fds as usize + 1) * size_of::<c_int>());
            if tmp.is_null() {
                p_err(c"failed to realloc".as_ptr());
                close(fd);
                while { nb_fds -= 1; nb_fds >= 0 } {
                    close(*(*fds).add(nb_fds as usize));
                }
                return -1;
            }
            *fds = tmp as *mut c_int;
        }
        *(*fds).add(nb_fds as usize) = fd;
        nb_fds += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn map_parse_fds(argc: *mut c_int, argv: *mut *mut *mut c_char, fds: *mut *mut c_int, open_flags: __u32) -> c_int {
    let mut opts = bpf_get_fd_by_id_opts { sz: size_of::<bpf_get_fd_by_id_opts>(), open_flags: 0 };

    if (open_flags & !BPF_F_RDONLY) != 0 {
        assert_fail(c"(open_flags & ~BPF_F_RDONLY) == 0".as_ptr(), c"common.c".as_ptr(), 1075, c"map_parse_fds".as_ptr());
    }
    opts.open_flags = open_flags;

    if is_prefix(**argv, c"id".as_ptr()) {
        let mut endptr: *mut c_char = ptr::null_mut();

        next_argp(argc, argv);
        let id = strtoul(**argv, &mut endptr, 0) as c_uint;
        if *endptr != 0 {
            p_err(c"can't parse %s as ID".as_ptr(), **argv);
            return -1;
        }
        next_argp(argc, argv);

        **fds = bpf_map_get_fd_by_id_opts(id, &opts);
        if **fds < 0 {
            p_err(c"get map by id (%u): %s".as_ptr(), id, strerror(errno()));
            return -1;
        }
        return 1;
    } else if is_prefix(**argv, c"name".as_ptr()) {
        next_argp(argc, argv);

        let name = **argv;
        if strlen(name) > BPF_OBJ_NAME_LEN - 1 {
            p_err(c"can't parse name".as_ptr());
            return -1;
        }
        next_argp(argc, argv);

        return map_fd_by_name(name, fds, &opts);
    } else if is_prefix(**argv, c"pinned".as_ptr()) {
        let mut get_opts = bpf_obj_get_opts { sz: size_of::<bpf_obj_get_opts>(), file_flags: 0 };
        get_opts.file_flags = open_flags;

        next_argp(argc, argv);

        let path = **argv;
        next_argp(argc, argv);

        **fds = open_obj_pinned_any(path, bpf_obj_type::BPF_OBJ_MAP, &get_opts);
        if **fds < 0 {
            return -1;
        }
        return 1;
    }

    p_err(c"expected 'id', 'name' or 'pinned', got: '%s'?".as_ptr(), **argv);
    -1
}

#[no_mangle]
pub unsafe extern "C" fn map_parse_fd(argc: *mut c_int, argv: *mut *mut *mut c_char, open_flags: __u32) -> c_int {
    let mut fds: *mut c_int = malloc(size_of::<c_int>()) as *mut c_int;
    let fd: c_int;

    if fds.is_null() {
        p_err(c"mem alloc failed".as_ptr());
        return -1;
    }
    let mut nb_fds = map_parse_fds(argc, argv, &mut fds, open_flags);
    if nb_fds != 1 {
        if nb_fds > 1 {
            p_err(c"several maps match this handle".as_ptr());
            while { nb_fds -= 1; nb_fds >= 0 } {
                close(*fds.add(nb_fds as usize));
            }
        }
        fd = -1;
        free(fds as *mut c_void);
        return fd;
    }

    fd = *fds;
    free(fds as *mut c_void);
    fd
}

#[no_mangle]
pub unsafe extern "C" fn map_parse_fd_and_info(argc: *mut c_int, argv: *mut *mut *mut c_char, info: *mut bpf_map_info, info_len: *mut __u32, open_flags: __u32) -> c_int {
    let fd = map_parse_fd(argc, argv, open_flags);
    if fd < 0 {
        return -1;
    }

    let err = bpf_map_get_info_by_fd(fd, info, info_len);
    if err != 0 {
        p_err(c"can't get map info: %s".as_ptr(), strerror(errno()));
        close(fd);
        return err;
    }

    fd
}

#[no_mangle]
pub unsafe extern "C" fn hash_fn_for_key_as_id(key: c_long, _ctx: *mut c_void) -> size_t {
    key as size_t
}

#[no_mangle]
pub unsafe extern "C" fn equal_fn_for_key_as_id(k1: c_long, k2: c_long, _ctx: *mut c_void) -> bool {
    k1 == k2
}

#[no_mangle]
pub unsafe extern "C" fn bpf_attach_type_input_str(t: bpf_attach_type) -> *const c_char {
    match t {
        bpf_attach_type::BPF_CGROUP_INET_INGRESS => c"ingress".as_ptr(),
        bpf_attach_type::BPF_CGROUP_INET_EGRESS => c"egress".as_ptr(),
        bpf_attach_type::BPF_CGROUP_INET_SOCK_CREATE => c"sock_create".as_ptr(),
        bpf_attach_type::BPF_CGROUP_INET_SOCK_RELEASE => c"sock_release".as_ptr(),
        bpf_attach_type::BPF_CGROUP_SOCK_OPS => c"sock_ops".as_ptr(),
        bpf_attach_type::BPF_CGROUP_DEVICE => c"device".as_ptr(),
        bpf_attach_type::BPF_CGROUP_INET4_BIND => c"bind4".as_ptr(),
        bpf_attach_type::BPF_CGROUP_INET6_BIND => c"bind6".as_ptr(),
        bpf_attach_type::BPF_CGROUP_INET4_CONNECT => c"connect4".as_ptr(),
        bpf_attach_type::BPF_CGROUP_INET6_CONNECT => c"connect6".as_ptr(),
        bpf_attach_type::BPF_CGROUP_INET4_POST_BIND => c"post_bind4".as_ptr(),
        bpf_attach_type::BPF_CGROUP_INET6_POST_BIND => c"post_bind6".as_ptr(),
        bpf_attach_type::BPF_CGROUP_INET4_GETPEERNAME => c"getpeername4".as_ptr(),
        bpf_attach_type::BPF_CGROUP_INET6_GETPEERNAME => c"getpeername6".as_ptr(),
        bpf_attach_type::BPF_CGROUP_INET4_GETSOCKNAME => c"getsockname4".as_ptr(),
        bpf_attach_type::BPF_CGROUP_INET6_GETSOCKNAME => c"getsockname6".as_ptr(),
        bpf_attach_type::BPF_CGROUP_UDP4_SENDMSG => c"sendmsg4".as_ptr(),
        bpf_attach_type::BPF_CGROUP_UDP6_SENDMSG => c"sendmsg6".as_ptr(),
        bpf_attach_type::BPF_CGROUP_SYSCTL => c"sysctl".as_ptr(),
        bpf_attach_type::BPF_CGROUP_UDP4_RECVMSG => c"recvmsg4".as_ptr(),
        bpf_attach_type::BPF_CGROUP_UDP6_RECVMSG => c"recvmsg6".as_ptr(),
        bpf_attach_type::BPF_CGROUP_GETSOCKOPT => c"getsockopt".as_ptr(),
        bpf_attach_type::BPF_CGROUP_SETSOCKOPT => c"setsockopt".as_ptr(),
        bpf_attach_type::BPF_TRACE_RAW_TP => c"raw_tp".as_ptr(),
        bpf_attach_type::BPF_TRACE_FENTRY => c"fentry".as_ptr(),
        bpf_attach_type::BPF_TRACE_FEXIT => c"fexit".as_ptr(),
        bpf_attach_type::BPF_MODIFY_RETURN => c"mod_ret".as_ptr(),
        bpf_attach_type::BPF_TRACE_FSESSION => c"fsession".as_ptr(),
        bpf_attach_type::BPF_SK_REUSEPORT_SELECT => c"sk_skb_reuseport_select".as_ptr(),
        bpf_attach_type::BPF_SK_REUSEPORT_SELECT_OR_MIGRATE => c"sk_skb_reuseport_select_or_migrate".as_ptr(),
        _ => libbpf_bpf_attach_type_str(t),
    }
}

#[no_mangle]
pub unsafe extern "C" fn pathname_concat(buf: *mut c_char, buf_sz: c_int, path: *const c_char, name: *const c_char) -> c_int {
    let len = snprintf(buf, buf_sz as size_t, c"%s/%s".as_ptr(), path, name);
    if len < 0 {
        return -EINVAL;
    }
    if len >= buf_sz {
        return -ENAMETOOLONG;
    }

    0
}

unsafe fn read_next_kernel_config_option(file: gzFile, buf: *mut c_char, n: size_t, value: *mut *mut c_char) -> bool {
    let mut sep: *mut c_char;

    while !gzgets(file, buf, n as c_int).is_null() {
        if strncmp(buf, c"CONFIG_".as_ptr(), 7) != 0 {
            continue;
        }

        sep = strchr(buf, '=' as c_int);
        if sep.is_null() {
            continue;
        }

        /* Trim ending '\n' */
        *buf.add(strlen(buf) - 1) = 0;

        /* Split on '=' and ensure that a value is present. */
        *sep = 0;
        if *sep.add(1) == 0 {
            continue;
        }

        *value = sep.add(1);
        return true;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn read_kernel_config(requested_options: *const kernel_config_option, num_options: size_t, out_values: *mut *mut c_char, define_prefix: *const c_char) -> c_int {
    let mut utsn: utsname = zeroed();
    let mut path = [0 as c_char; PATH_MAX];
    let mut file: gzFile = ptr::null_mut();
    let mut buf = [0 as c_char; 4096];
    let mut value: *mut c_char = ptr::null_mut();
    let mut ret: c_int = 0;

    if requested_options.is_null() || out_values.is_null() || num_options == 0 {
        return -1;
    }

    if uname(&mut utsn) == 0 {
        snprintf(path.as_mut_ptr(), path.len(), c"/boot/config-%s".as_ptr(), utsn.release.as_ptr());

        /* gzopen also accepts uncompressed files. */
        file = gzopen(path.as_ptr(), c"r".as_ptr());
    }

    if file.is_null() {
        /* Some distributions build with CONFIG_IKCONFIG=y and put the
         * config file at /proc/config.gz.
         */
        file = gzopen(c"/proc/config.gz".as_ptr(), c"r".as_ptr());
    }

    if file.is_null() {
        p_info(c"skipping kernel config, can't open file: %s".as_ptr(), strerror(errno()));
        return -1;
    }

    if gzgets(file, buf.as_mut_ptr(), buf.len() as c_int).is_null() || gzgets(file, buf.as_mut_ptr(), buf.len() as c_int).is_null() {
        p_info(c"skipping kernel config, can't read from file: %s".as_ptr(), strerror(errno()));
        ret = -1;
        gzclose(file);
        return ret;
    }

    if strcmp(buf.as_ptr(), c"# Automatically generated file; DO NOT EDIT.\n".as_ptr()) != 0 {
        p_info(c"skipping kernel config, can't find correct file".as_ptr());
        ret = -1;
        gzclose(file);
        return ret;
    }

    while read_next_kernel_config_option(file, buf.as_mut_ptr(), buf.len(), &mut value) {
        for i in 0..num_options {
            let opt = requested_options.add(i);
            if (!define_prefix.is_null() && !(*opt).macro_dump)
                || !(*out_values.add(i)).is_null()
                || strcmp(buf.as_ptr(), (*opt).name) != 0
            {
                continue;
            }

            *out_values.add(i) = strdup(value);
        }
    }

    gzclose(file);
    ret
}
