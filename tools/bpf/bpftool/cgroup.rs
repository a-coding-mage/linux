// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (C) 2017 Facebook
// Author: Roman Gushchin <guro@fb.com>

// C source dependencies: errno.h, fcntl.h, ftw.h, mntent.h, stdio.h,
// stdlib.h, string.h, sys/stat.h, sys/types.h, unistd.h, bpf/bpf.h,
// bpf/btf.h, and "main.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

type __u32 = u32;
type __u64 = u64;

const O_RDONLY: c_int = 0;
const EINVAL: c_int = 22;
const FTW_D: c_int = 1;
const FTW_MOUNT: c_int = 2;
const MAX_PROG_FULL_NAME: usize = 128;

const BPF_CGROUP_INET_INGRESS: c_int = 0;
const BPF_CGROUP_INET_EGRESS: c_int = 1;
const BPF_CGROUP_INET_SOCK_CREATE: c_int = 2;
const BPF_CGROUP_SOCK_OPS: c_int = 3;
const BPF_CGROUP_DEVICE: c_int = 4;
const BPF_CGROUP_INET4_BIND: c_int = 8;
const BPF_CGROUP_INET6_BIND: c_int = 9;
const BPF_CGROUP_INET4_CONNECT: c_int = 10;
const BPF_CGROUP_INET6_CONNECT: c_int = 11;
const BPF_CGROUP_INET4_POST_BIND: c_int = 12;
const BPF_CGROUP_INET6_POST_BIND: c_int = 13;
const BPF_CGROUP_UDP4_SENDMSG: c_int = 14;
const BPF_CGROUP_UDP6_SENDMSG: c_int = 15;
const BPF_LSM_CGROUP: c_int = 16;
const BPF_CGROUP_INET4_GETPEERNAME: c_int = 17;
const BPF_CGROUP_INET6_GETPEERNAME: c_int = 18;
const BPF_CGROUP_INET4_GETSOCKNAME: c_int = 19;
const BPF_CGROUP_INET6_GETSOCKNAME: c_int = 20;
const BPF_CGROUP_SYSCTL: c_int = 21;
const BPF_CGROUP_UDP4_RECVMSG: c_int = 22;
const BPF_CGROUP_UDP6_RECVMSG: c_int = 23;
const BPF_CGROUP_GETSOCKOPT: c_int = 24;
const BPF_CGROUP_SETSOCKOPT: c_int = 25;
const BPF_CGROUP_INET_SOCK_RELEASE: c_int = 26;
const BPF_CGROUP_UNIX_CONNECT: c_int = 27;
const BPF_CGROUP_UNIX_SENDMSG: c_int = 28;
const BPF_CGROUP_UNIX_RECVMSG: c_int = 29;
const BPF_CGROUP_UNIX_GETPEERNAME: c_int = 30;
const BPF_CGROUP_UNIX_GETSOCKNAME: c_int = 31;
const __MAX_BPF_ATTACH_TYPE: c_int = 32;

const BPF_F_ALLOW_OVERRIDE: __u32 = 1;
const BPF_F_ALLOW_MULTI: __u32 = 2;
const BPF_F_QUERY_EFFECTIVE: __u32 = 1 << 0;

const HELP_SPEC_ATTACH_FLAGS: &str = "ATTACH_FLAGS := { multi | override }";
const HELP_SPEC_ATTACH_TYPES: &str =
    "       ATTACH_TYPE := { cgroup_inet_ingress | cgroup_inet_egress |\n\
                            cgroup_inet_sock_create | cgroup_sock_ops |\n\
                            cgroup_device | cgroup_inet4_bind |\n\
                            cgroup_inet6_bind | cgroup_inet4_post_bind |\n\
                            cgroup_inet6_post_bind | cgroup_inet4_connect |\n\
                            cgroup_inet6_connect | cgroup_unix_connect |\n\
                            cgroup_inet4_getpeername | cgroup_inet6_getpeername |\n\
                            cgroup_unix_getpeername | cgroup_inet4_getsockname |\n\
                            cgroup_inet6_getsockname | cgroup_unix_getsockname |\n\
                            cgroup_udp4_sendmsg | cgroup_udp6_sendmsg |\n\
                            cgroup_unix_sendmsg | cgroup_udp4_recvmsg |\n\
                            cgroup_udp6_recvmsg | cgroup_unix_recvmsg |\n\
                            cgroup_sysctl | cgroup_getsockopt |\n\
                            cgroup_setsockopt | cgroup_inet_sock_release }";

static CGROUP_ATTACH_TYPES: [c_int; 29] = [
    BPF_CGROUP_INET_INGRESS,
    BPF_CGROUP_INET_EGRESS,
    BPF_CGROUP_INET_SOCK_CREATE,
    BPF_CGROUP_INET_SOCK_RELEASE,
    BPF_CGROUP_INET4_BIND,
    BPF_CGROUP_INET6_BIND,
    BPF_CGROUP_INET4_POST_BIND,
    BPF_CGROUP_INET6_POST_BIND,
    BPF_CGROUP_INET4_CONNECT,
    BPF_CGROUP_INET6_CONNECT,
    BPF_CGROUP_UNIX_CONNECT,
    BPF_CGROUP_INET4_GETPEERNAME,
    BPF_CGROUP_INET6_GETPEERNAME,
    BPF_CGROUP_UNIX_GETPEERNAME,
    BPF_CGROUP_INET4_GETSOCKNAME,
    BPF_CGROUP_INET6_GETSOCKNAME,
    BPF_CGROUP_UNIX_GETSOCKNAME,
    BPF_CGROUP_UDP4_SENDMSG,
    BPF_CGROUP_UDP6_SENDMSG,
    BPF_CGROUP_UNIX_SENDMSG,
    BPF_CGROUP_UDP4_RECVMSG,
    BPF_CGROUP_UDP6_RECVMSG,
    BPF_CGROUP_UNIX_RECVMSG,
    BPF_CGROUP_SOCK_OPS,
    BPF_CGROUP_DEVICE,
    BPF_CGROUP_SYSCTL,
    BPF_CGROUP_GETSOCKOPT,
    BPF_CGROUP_SETSOCKOPT,
    BPF_LSM_CGROUP,
];

static mut QUERY_FLAGS: c_uint = 0;
static mut BTF_VMLINUX: *mut btf = core::ptr::null_mut();
static mut BTF_VMLINUX_ID: __u32 = 0;

#[repr(C)]
struct btf {
    _private: [u8; 0],
}

#[repr(C)]
struct btf_type {
    name_off: __u32,
}

#[repr(C)]
struct bpf_btf_info {
    btf: __u64,
    btf_size: __u32,
    id: __u32,
    name: __u64,
    name_len: __u32,
    kernel_btf: __u32,
}

#[repr(C)]
struct bpf_prog_info {
    id: __u32,
    _rest0: [u8; 0],
    attach_btf_obj_id: __u32,
    attach_btf_id: __u32,
}

#[repr(C)]
struct bpf_prog_query_opts {
    sz: c_ulong,
    query_flags: __u32,
    attach_flags: __u32,
    prog_ids: *mut __u32,
    prog_cnt: __u32,
    prog_attach_flags: *mut __u32,
}

#[repr(C)]
struct stat {
    _private: [u8; 0],
}

#[repr(C)]
struct FTW {
    base: c_int,
    level: c_int,
}

#[repr(C)]
struct mntent {
    mnt_fsname: *mut c_char,
    mnt_dir: *mut c_char,
    mnt_type: *mut c_char,
    mnt_opts: *mut c_char,
    mnt_freq: c_int,
    mnt_passno: c_int,
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct json_writer_t {
    _private: [u8; 0],
}

#[repr(C)]
struct cmd {
    cmd: *const c_char,
    func: Option<unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int>,
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut json_output: bool;
    static mut json_wtr: *mut json_writer_t;
    static bin_name: *const c_char;
    static HELP_SPEC_PROGRAM: *const c_char;
    static HELP_SPEC_OPTIONS: *const c_char;

    fn btf__free(btf: *mut btf);
    fn btf__type_cnt(btf: *const btf) -> __u32;
    fn btf__type_by_id(btf: *const btf, type_id: __u32) -> *const btf_type;
    fn btf__name_by_offset(btf: *const btf, offset: __u32) -> *const c_char;
    fn libbpf_find_kernel_btf() -> *mut btf;
    fn libbpf_bpf_attach_type_str(t: c_int) -> *const c_char;
    fn bpf_attach_type_input_str(t: c_int) -> *const c_char;
    fn bpf_btf_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_btf_get_info_by_fd(fd: c_int, info: *mut bpf_btf_info, len: *mut __u32) -> c_int;
    fn bpf_prog_get_fd_by_id(id: c_int) -> c_int;
    fn bpf_prog_get_info_by_fd(fd: c_int, info: *mut bpf_prog_info, len: *mut __u32) -> c_int;
    fn bpf_prog_query(
        target_fd: c_int,
        attach_type: c_int,
        query_flags: c_uint,
        attach_flags: *mut __u32,
        prog_ids: *mut __u32,
        prog_cnt: *mut __u32,
    ) -> c_int;
    fn bpf_prog_query_opts(target_fd: c_int, attach_type: c_int, opts: *mut bpf_prog_query_opts) -> c_int;
    fn bpf_prog_attach(prog_fd: c_int, target_fd: c_int, attach_type: c_int, flags: c_int) -> c_int;
    fn bpf_prog_detach2(prog_fd: c_int, target_fd: c_int, attach_type: c_int) -> c_int;
    fn get_prog_full_name(info: *const bpf_prog_info, prog_fd: c_int, name: *mut c_char, len: usize);
    fn prog_parse_fd(argc: *mut c_int, argv: *mut *mut *mut c_char) -> c_int;
    fn is_prefix(str: *const c_char, prefix: *const c_char) -> bool;
    fn ptr_to_u64(ptr: *const c_void) -> __u64;
    fn p_err(fmt: *const c_char, ...);
    fn cmd_select(cmds: *const cmd, argc: c_int, argv: *mut *mut c_char, help: unsafe extern "C" fn(c_int, *mut *mut c_char) -> c_int) -> c_int;

    fn jsonw_start_object(w: *mut json_writer_t);
    fn jsonw_end_object(w: *mut json_writer_t);
    fn jsonw_start_array(w: *mut json_writer_t);
    fn jsonw_end_array(w: *mut json_writer_t);
    fn jsonw_uint_field(w: *mut json_writer_t, key: *const c_char, value: c_uint);
    fn jsonw_string_field(w: *mut json_writer_t, key: *const c_char, value: *const c_char);
    fn jsonw_name(w: *mut json_writer_t, name: *const c_char);
    fn jsonw_null(w: *mut json_writer_t);

    fn close(fd: c_int) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut FILE;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getmntent(stream: *mut FILE) -> *mut mntent;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn nftw(
        dirpath: *const c_char,
        fn_: Option<unsafe extern "C" fn(*const c_char, *const stat, c_int, *mut FTW) -> c_int>,
        nopenfd: c_int,
        flags: c_int,
    ) -> c_int;
}

unsafe fn free_btf_vmlinux() {
    btf__free(BTF_VMLINUX);
    BTF_VMLINUX = core::ptr::null_mut();
    BTF_VMLINUX_ID = 0;
}

unsafe fn parse_attach_type(str_: *const c_char) -> c_int {
    let mut attach_type_str: *const c_char;
    let mut type_: c_int = 0;

    loop {
        attach_type_str = libbpf_bpf_attach_type_str(type_);
        if attach_type_str.is_null() {
            break;
        }
        if strcmp(str_, attach_type_str) == 0 {
            return type_;
        }
        type_ += 1;
    }

    /*
     * Also check traditionally used attach type strings. For these we keep
     * allowing prefixed usage.
     */
    type_ = 0;
    loop {
        attach_type_str = bpf_attach_type_input_str(type_);
        if attach_type_str.is_null() {
            break;
        }
        if is_prefix(str_, attach_type_str) {
            return type_;
        }
        type_ += 1;
    }

    __MAX_BPF_ATTACH_TYPE
}

unsafe fn guess_vmlinux_btf_id(attach_btf_obj_id: __u32) {
    let mut btf_info: bpf_btf_info = core::mem::zeroed();
    let mut btf_len: __u32 = core::mem::size_of::<bpf_btf_info>() as __u32;
    let mut name = [0 as c_char; 16];
    let err: c_int;
    let fd: c_int;

    btf_info.name = ptr_to_u64(name.as_mut_ptr() as *const c_void);
    btf_info.name_len = core::mem::size_of_val(&name) as __u32;

    fd = bpf_btf_get_fd_by_id(attach_btf_obj_id);
    if fd < 0 {
        return;
    }

    err = bpf_btf_get_info_by_fd(fd, &mut btf_info, &mut btf_len);
    if err != 0 {
        close(fd);
        return;
    }

    if btf_info.kernel_btf != 0
        && strncmp(name.as_ptr(), b"vmlinux\0".as_ptr() as *const c_char, core::mem::size_of_val(&name)) == 0
    {
        BTF_VMLINUX_ID = btf_info.id;
    }

    close(fd);
}

unsafe fn show_bpf_prog(
    id: c_int,
    attach_type: c_int,
    attach_flags_str: *const c_char,
    level: c_int,
) -> c_int {
    let mut prog_name = [0 as c_char; MAX_PROG_FULL_NAME];
    let mut attach_btf_name: *const c_char = core::ptr::null();
    let mut info: bpf_prog_info = core::mem::zeroed();
    let attach_type_str: *const c_char;
    let mut info_len: __u32 = core::mem::size_of::<bpf_prog_info>() as __u32;
    let prog_fd: c_int;

    prog_fd = bpf_prog_get_fd_by_id(id);
    if prog_fd < 0 {
        return -1;
    }

    if bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut info_len) != 0 {
        close(prog_fd);
        return -1;
    }

    attach_type_str = libbpf_bpf_attach_type_str(attach_type);

    if !BTF_VMLINUX.is_null() {
        if BTF_VMLINUX_ID == 0 {
            guess_vmlinux_btf_id(info.attach_btf_obj_id);
        }

        if BTF_VMLINUX_ID == info.attach_btf_obj_id
            && info.attach_btf_id < btf__type_cnt(BTF_VMLINUX)
        {
            let t = btf__type_by_id(BTF_VMLINUX, info.attach_btf_id);
            attach_btf_name = btf__name_by_offset(BTF_VMLINUX, (*t).name_off);
        }
    }

    get_prog_full_name(&info, prog_fd, prog_name.as_mut_ptr(), prog_name.len());
    if json_output {
        jsonw_start_object(json_wtr);
        jsonw_uint_field(json_wtr, b"id\0".as_ptr() as *const c_char, info.id);
        if !attach_type_str.is_null() {
            jsonw_string_field(json_wtr, b"attach_type\0".as_ptr() as *const c_char, attach_type_str);
        } else {
            jsonw_uint_field(json_wtr, b"attach_type\0".as_ptr() as *const c_char, attach_type as c_uint);
        }
        if QUERY_FLAGS & BPF_F_QUERY_EFFECTIVE == 0 {
            jsonw_string_field(json_wtr, b"attach_flags\0".as_ptr() as *const c_char, attach_flags_str);
        }
        jsonw_string_field(json_wtr, b"name\0".as_ptr() as *const c_char, prog_name.as_ptr());
        if !attach_btf_name.is_null() {
            jsonw_string_field(json_wtr, b"attach_btf_name\0".as_ptr() as *const c_char, attach_btf_name);
        }
        jsonw_uint_field(json_wtr, b"attach_btf_obj_id\0".as_ptr() as *const c_char, info.attach_btf_obj_id);
        jsonw_uint_field(json_wtr, b"attach_btf_id\0".as_ptr() as *const c_char, info.attach_btf_id);
        jsonw_end_object(json_wtr);
    } else {
        printf(
            b"%s%-8u \0".as_ptr() as *const c_char,
            if level != 0 { b"    \0".as_ptr() } else { b"\0".as_ptr() } as *const c_char,
            info.id,
        );
        if !attach_type_str.is_null() {
            printf(b"%-15s\0".as_ptr() as *const c_char, attach_type_str);
        } else {
            printf(b"type %-10u\0".as_ptr() as *const c_char, attach_type as c_uint);
        }
        if QUERY_FLAGS & BPF_F_QUERY_EFFECTIVE != 0 {
            printf(b" %-15s\0".as_ptr() as *const c_char, prog_name.as_ptr());
        } else {
            printf(
                b" %-15s %-15s\0".as_ptr() as *const c_char,
                attach_flags_str,
                prog_name.as_ptr(),
            );
        }
        if !attach_btf_name.is_null() {
            printf(b" %-15s\0".as_ptr() as *const c_char, attach_btf_name);
        } else if info.attach_btf_id != 0 {
            printf(
                b" attach_btf_obj_id=%u attach_btf_id=%u\0".as_ptr() as *const c_char,
                info.attach_btf_obj_id,
                info.attach_btf_id,
            );
        }
        printf(b"\n\0".as_ptr() as *const c_char);
    }

    close(prog_fd);
    0
}

unsafe fn count_attached_bpf_progs(cgroup_fd: c_int, type_: c_int) -> c_int {
    let mut prog_cnt: __u32 = 0;
    let ret: c_int;

    ret = bpf_prog_query(
        cgroup_fd,
        type_,
        QUERY_FLAGS,
        core::ptr::null_mut(),
        core::ptr::null_mut(),
        &mut prog_cnt,
    );
    if ret != 0 {
        return -1;
    }

    prog_cnt as c_int
}

unsafe fn cgroup_has_attached_progs(cgroup_fd: c_int) -> c_int {
    let mut i: c_uint = 0;
    let mut no_prog = true;

    while (i as usize) < CGROUP_ATTACH_TYPES.len() {
        let count = count_attached_bpf_progs(cgroup_fd, CGROUP_ATTACH_TYPES[i as usize]);

        if count < 0 && errno != EINVAL {
            return -1;
        }

        if count > 0 {
            no_prog = false;
            break;
        }
        i += 1;
    }

    if no_prog { 0 } else { 1 }
}

unsafe fn show_effective_bpf_progs(cgroup_fd: c_int, type_: c_int, level: c_int) -> c_int {
    let mut p: bpf_prog_query_opts = core::mem::zeroed();
    let mut prog_ids = [0 as __u32; 1024];
    let mut iter: __u32;
    let ret: c_int;

    p.query_flags = QUERY_FLAGS;
    p.prog_cnt = prog_ids.len() as __u32;
    p.prog_ids = prog_ids.as_mut_ptr();

    ret = bpf_prog_query_opts(cgroup_fd, type_, &mut p);
    if ret != 0 {
        return ret;
    }

    if p.prog_cnt == 0 {
        return 0;
    }

    iter = 0;
    while iter < p.prog_cnt {
        show_bpf_prog(prog_ids[iter as usize] as c_int, type_, core::ptr::null(), level);
        iter += 1;
    }

    0
}

unsafe fn show_attached_bpf_progs(cgroup_fd: c_int, type_: c_int, level: c_int) -> c_int {
    let mut p: bpf_prog_query_opts = core::mem::zeroed();
    let mut prog_attach_flags = [0 as __u32; 1024];
    let mut attach_flags_str: *const c_char;
    let mut prog_ids = [0 as __u32; 1024];
    let mut buf = [0 as c_char; 32];
    let mut iter: __u32;
    let ret: c_int;

    p.query_flags = QUERY_FLAGS;
    p.prog_cnt = prog_ids.len() as __u32;
    p.prog_ids = prog_ids.as_mut_ptr();
    p.prog_attach_flags = prog_attach_flags.as_mut_ptr();

    ret = bpf_prog_query_opts(cgroup_fd, type_, &mut p);
    if ret != 0 {
        return ret;
    }

    if p.prog_cnt == 0 {
        return 0;
    }

    iter = 0;
    while iter < p.prog_cnt {
        let mut attach_flags: __u32;

        attach_flags = if prog_attach_flags[iter as usize] != 0 {
            prog_attach_flags[iter as usize]
        } else {
            p.attach_flags
        };

        match attach_flags {
            BPF_F_ALLOW_MULTI => {
                attach_flags_str = b"multi\0".as_ptr() as *const c_char;
            }
            BPF_F_ALLOW_OVERRIDE => {
                attach_flags_str = b"override\0".as_ptr() as *const c_char;
            }
            0 => {
                attach_flags_str = b"\0".as_ptr() as *const c_char;
            }
            _ => {
                snprintf(
                    buf.as_mut_ptr(),
                    buf.len(),
                    b"unknown(%x)\0".as_ptr() as *const c_char,
                    attach_flags,
                );
                attach_flags_str = buf.as_ptr();
            }
        }

        show_bpf_prog(prog_ids[iter as usize] as c_int, type_, attach_flags_str, level);
        iter += 1;
    }

    0
}

unsafe fn show_bpf_progs(cgroup_fd: c_int, type_: c_int, level: c_int) -> c_int {
    if QUERY_FLAGS & BPF_F_QUERY_EFFECTIVE != 0 {
        show_effective_bpf_progs(cgroup_fd, type_, level)
    } else {
        show_attached_bpf_progs(cgroup_fd, type_, level)
    }
}

unsafe fn req_args(argc: c_int, needed: c_int) -> bool {
    argc >= needed
}

unsafe fn get_arg(argc: &mut c_int, argv: &mut *mut *mut c_char) -> *mut c_char {
    let arg = **argv;
    *argv = (*argv).add(1);
    *argc -= 1;
    arg
}

unsafe fn next_arg(argc: &mut c_int, argv: &mut *mut *mut c_char) {
    *argv = (*argv).add(1);
    *argc -= 1;
}

unsafe extern "C" fn do_show(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let has_attached_progs: c_int;
    let path: *const c_char;
    let cgroup_fd: c_int;
    let mut ret: c_int = -1;
    let mut i: c_uint;

    QUERY_FLAGS = 0;

    if !req_args(argc, 1) {
        return -1;
    }
    path = get_arg(&mut argc, &mut argv);

    while argc != 0 {
        if is_prefix(*argv, b"effective\0".as_ptr() as *const c_char) {
            if QUERY_FLAGS & BPF_F_QUERY_EFFECTIVE != 0 {
                p_err(b"duplicated argument: %s\0".as_ptr() as *const c_char, *argv);
                return -1;
            }
            QUERY_FLAGS |= BPF_F_QUERY_EFFECTIVE;
            next_arg(&mut argc, &mut argv);
        } else {
            p_err(
                b"expected no more arguments, 'effective', got: '%s'?\0".as_ptr() as *const c_char,
                *argv,
            );
            return -1;
        }
    }

    cgroup_fd = open(path, O_RDONLY);
    if cgroup_fd < 0 {
        p_err(b"can't open cgroup %s\0".as_ptr() as *const c_char, path);
        return ret;
    }

    has_attached_progs = cgroup_has_attached_progs(cgroup_fd);
    if has_attached_progs < 0 {
        p_err(
            b"can't query bpf programs attached to %s: %s\0".as_ptr() as *const c_char,
            path,
            strerror(errno),
        );
        close(cgroup_fd);
        return ret;
    } else if has_attached_progs == 0 {
        ret = 0;
        close(cgroup_fd);
        return ret;
    }

    if json_output {
        jsonw_start_array(json_wtr);
    } else if QUERY_FLAGS & BPF_F_QUERY_EFFECTIVE != 0 {
        printf(
            b"%-8s %-15s %-15s\n\0".as_ptr() as *const c_char,
            b"ID\0".as_ptr() as *const c_char,
            b"AttachType\0".as_ptr() as *const c_char,
            b"Name\0".as_ptr() as *const c_char,
        );
    } else {
        printf(
            b"%-8s %-15s %-15s %-15s\n\0".as_ptr() as *const c_char,
            b"ID\0".as_ptr() as *const c_char,
            b"AttachType\0".as_ptr() as *const c_char,
            b"AttachFlags\0".as_ptr() as *const c_char,
            b"Name\0".as_ptr() as *const c_char,
        );
    }

    BTF_VMLINUX = libbpf_find_kernel_btf();
    i = 0;
    while (i as usize) < CGROUP_ATTACH_TYPES.len() {
        /*
         * Not all attach types may be supported, so it's expected,
         * that some requests will fail.
         * If we were able to get the show for at least one
         * attach type, let's return 0.
         */
        if show_bpf_progs(cgroup_fd, CGROUP_ATTACH_TYPES[i as usize], 0) == 0 {
            ret = 0;
        }
        i += 1;
    }

    if json_output {
        jsonw_end_array(json_wtr);
    }

    free_btf_vmlinux();
    close(cgroup_fd);
    ret
}

/*
 * To distinguish nftw() errors and do_show_tree_fn() errors
 * and avoid duplicating error messages, let's return -2
 * from do_show_tree_fn() in case of error.
 */
const NFTW_ERR: c_int = -1;
const SHOW_TREE_FN_ERR: c_int = -2;

unsafe extern "C" fn do_show_tree_fn(
    fpath: *const c_char,
    _sb: *const stat,
    typeflag: c_int,
    ftw: *mut FTW,
) -> c_int {
    let has_attached_progs: c_int;
    let cgroup_fd: c_int;
    let mut i: c_uint;

    if typeflag != FTW_D {
        return 0;
    }

    cgroup_fd = open(fpath, O_RDONLY);
    if cgroup_fd < 0 {
        p_err(
            b"can't open cgroup %s: %s\0".as_ptr() as *const c_char,
            fpath,
            strerror(errno),
        );
        return SHOW_TREE_FN_ERR;
    }

    has_attached_progs = cgroup_has_attached_progs(cgroup_fd);
    if has_attached_progs < 0 {
        p_err(
            b"can't query bpf programs attached to %s: %s\0".as_ptr() as *const c_char,
            fpath,
            strerror(errno),
        );
        close(cgroup_fd);
        return SHOW_TREE_FN_ERR;
    } else if has_attached_progs == 0 {
        close(cgroup_fd);
        return 0;
    }

    if json_output {
        jsonw_start_object(json_wtr);
        jsonw_string_field(json_wtr, b"cgroup\0".as_ptr() as *const c_char, fpath);
        jsonw_name(json_wtr, b"programs\0".as_ptr() as *const c_char);
        jsonw_start_array(json_wtr);
    } else {
        printf(b"%s\n\0".as_ptr() as *const c_char, fpath);
    }

    if BTF_VMLINUX.is_null() {
        BTF_VMLINUX = libbpf_find_kernel_btf();
    }

    i = 0;
    while (i as usize) < CGROUP_ATTACH_TYPES.len() {
        show_bpf_progs(cgroup_fd, CGROUP_ATTACH_TYPES[i as usize], (*ftw).level);
        i += 1;
    }

    if errno == EINVAL {
        /*
         * Last attach type does not support query.
         * Do not report an error for this, especially because batch
         * mode would stop processing commands.
         */
        errno = 0;
    }

    if json_output {
        jsonw_end_array(json_wtr);
        jsonw_end_object(json_wtr);
    }

    close(cgroup_fd);

    0
}

unsafe fn find_cgroup_root() -> *mut c_char {
    let mut mnt: *mut mntent;
    let f: *mut FILE;

    f = fopen(
        b"/proc/mounts\0".as_ptr() as *const c_char,
        b"r\0".as_ptr() as *const c_char,
    );
    if f.is_null() {
        return core::ptr::null_mut();
    }

    loop {
        mnt = getmntent(f);
        if mnt.is_null() {
            break;
        }
        if strcmp((*mnt).mnt_type, b"cgroup2\0".as_ptr() as *const c_char) == 0 {
            fclose(f);
            return strdup((*mnt).mnt_dir);
        }
    }

    fclose(f);
    core::ptr::null_mut()
}

unsafe extern "C" fn do_show_tree(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let mut cgroup_root: *mut c_char;
    let mut cgroup_alloced: *mut c_char = core::ptr::null_mut();
    let ret: c_int;

    QUERY_FLAGS = 0;

    if argc == 0 {
        cgroup_alloced = find_cgroup_root();
        if cgroup_alloced.is_null() {
            p_err(b"cgroup v2 isn't mounted\0".as_ptr() as *const c_char);
            return -1;
        }
        cgroup_root = cgroup_alloced;
    } else {
        cgroup_root = get_arg(&mut argc, &mut argv);

        while argc != 0 {
            if is_prefix(*argv, b"effective\0".as_ptr() as *const c_char) {
                if QUERY_FLAGS & BPF_F_QUERY_EFFECTIVE != 0 {
                    p_err(b"duplicated argument: %s\0".as_ptr() as *const c_char, *argv);
                    return -1;
                }
                QUERY_FLAGS |= BPF_F_QUERY_EFFECTIVE;
                next_arg(&mut argc, &mut argv);
            } else {
                p_err(
                    b"expected no more arguments, 'effective', got: '%s'?\0".as_ptr() as *const c_char,
                    *argv,
                );
                return -1;
            }
        }
    }

    if json_output {
        jsonw_start_array(json_wtr);
    } else if QUERY_FLAGS & BPF_F_QUERY_EFFECTIVE != 0 {
        printf(
            b"%s\n%-8s %-15s %-15s\n\0".as_ptr() as *const c_char,
            b"CgroupPath\0".as_ptr() as *const c_char,
            b"ID\0".as_ptr() as *const c_char,
            b"AttachType\0".as_ptr() as *const c_char,
            b"Name\0".as_ptr() as *const c_char,
        );
    } else {
        printf(
            b"%s\n%-8s %-15s %-15s %-15s\n\0".as_ptr() as *const c_char,
            b"CgroupPath\0".as_ptr() as *const c_char,
            b"ID\0".as_ptr() as *const c_char,
            b"AttachType\0".as_ptr() as *const c_char,
            b"AttachFlags\0".as_ptr() as *const c_char,
            b"Name\0".as_ptr() as *const c_char,
        );
    }

    match nftw(cgroup_root, Some(do_show_tree_fn), 1024, FTW_MOUNT) {
        NFTW_ERR => {
            p_err(
                b"can't iterate over %s: %s\0".as_ptr() as *const c_char,
                cgroup_root,
                strerror(errno),
            );
            ret = -1;
        }
        SHOW_TREE_FN_ERR => {
            ret = -1;
        }
        _ => {
            ret = 0;
        }
    }

    if json_output {
        jsonw_end_array(json_wtr);
    }

    free_btf_vmlinux();
    free(cgroup_alloced as *mut c_void);

    ret
}

unsafe extern "C" fn do_attach(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let attach_type: c_int;
    let cgroup_fd: c_int;
    let prog_fd: c_int;
    let mut attach_flags: c_int = 0;
    let mut ret: c_int = -1;
    let mut i: c_int;

    if argc < 4 {
        p_err(b"too few parameters for cgroup attach\0".as_ptr() as *const c_char);
        return ret;
    }

    cgroup_fd = open(*argv, O_RDONLY);
    if cgroup_fd < 0 {
        p_err(b"can't open cgroup %s\0".as_ptr() as *const c_char, *argv);
        return ret;
    }

    attach_type = parse_attach_type(*argv.add(1));
    if attach_type == __MAX_BPF_ATTACH_TYPE {
        p_err(b"invalid attach type\0".as_ptr() as *const c_char);
        close(cgroup_fd);
        return ret;
    }

    argc -= 2;
    argv = argv.add(2);
    prog_fd = prog_parse_fd(&mut argc, &mut argv);
    if prog_fd < 0 {
        close(cgroup_fd);
        return ret;
    }

    i = 0;
    while i < argc {
        if is_prefix(*argv.add(i as usize), b"multi\0".as_ptr() as *const c_char) {
            attach_flags |= BPF_F_ALLOW_MULTI as c_int;
        } else if is_prefix(*argv.add(i as usize), b"override\0".as_ptr() as *const c_char) {
            attach_flags |= BPF_F_ALLOW_OVERRIDE as c_int;
        } else {
            p_err(
                b"unknown option: %s\0".as_ptr() as *const c_char,
                *argv.add(i as usize),
            );
            close(cgroup_fd);
            return ret;
        }
        i += 1;
    }

    if bpf_prog_attach(prog_fd, cgroup_fd, attach_type, attach_flags) != 0 {
        p_err(b"failed to attach program\0".as_ptr() as *const c_char);
        close(prog_fd);
        close(cgroup_fd);
        return ret;
    }

    if json_output {
        jsonw_null(json_wtr);
    }

    ret = 0;

    close(prog_fd);
    close(cgroup_fd);
    ret
}

unsafe extern "C" fn do_detach(mut argc: c_int, mut argv: *mut *mut c_char) -> c_int {
    let attach_type: c_int;
    let prog_fd: c_int;
    let cgroup_fd: c_int;
    let mut ret: c_int = -1;

    if argc < 4 {
        p_err(b"too few parameters for cgroup detach\0".as_ptr() as *const c_char);
        return ret;
    }

    cgroup_fd = open(*argv, O_RDONLY);
    if cgroup_fd < 0 {
        p_err(b"can't open cgroup %s\0".as_ptr() as *const c_char, *argv);
        return ret;
    }

    attach_type = parse_attach_type(*argv.add(1));
    if attach_type == __MAX_BPF_ATTACH_TYPE {
        p_err(b"invalid attach type\0".as_ptr() as *const c_char);
        close(cgroup_fd);
        return ret;
    }

    argc -= 2;
    argv = argv.add(2);
    prog_fd = prog_parse_fd(&mut argc, &mut argv);
    if prog_fd < 0 {
        close(cgroup_fd);
        return ret;
    }

    if bpf_prog_detach2(prog_fd, cgroup_fd, attach_type) != 0 {
        p_err(b"failed to detach program\0".as_ptr() as *const c_char);
        close(prog_fd);
        close(cgroup_fd);
        return ret;
    }

    if json_output {
        jsonw_null(json_wtr);
    }

    ret = 0;

    close(prog_fd);
    close(cgroup_fd);
    ret
}

unsafe extern "C" fn do_help(_argc: c_int, argv: *mut *mut c_char) -> c_int {
    if json_output {
        jsonw_null(json_wtr);
        return 0;
    }

    fprintf(
        stderr,
        b"Usage: %1$s %2$s { show | list } CGROUP [**effective**]\n\
          \       %1$s %2$s tree [CGROUP_ROOT] [**effective**]\n\
          \       %1$s %2$s attach CGROUP ATTACH_TYPE PROG [ATTACH_FLAGS]\n\
          \       %1$s %2$s detach CGROUP ATTACH_TYPE PROG\n\
          \       %1$s %2$s help\n\
          \n\
          \%s\n\
          \       %s\n\
          \       %s\n\
          \       %s |\n\
          \                    {-f|--bpffs} }\n\
          \0"
            .as_ptr() as *const c_char,
        bin_name,
        *argv.offset(-2),
        HELP_SPEC_ATTACH_TYPES.as_ptr(),
        HELP_SPEC_ATTACH_FLAGS.as_ptr(),
        HELP_SPEC_PROGRAM,
        HELP_SPEC_OPTIONS,
    );

    0
}

static CMDS: [cmd; 7] = [
    cmd {
        cmd: b"show\0".as_ptr() as *const c_char,
        func: Some(do_show),
    },
    cmd {
        cmd: b"list\0".as_ptr() as *const c_char,
        func: Some(do_show),
    },
    cmd {
        cmd: b"tree\0".as_ptr() as *const c_char,
        func: Some(do_show_tree),
    },
    cmd {
        cmd: b"attach\0".as_ptr() as *const c_char,
        func: Some(do_attach),
    },
    cmd {
        cmd: b"detach\0".as_ptr() as *const c_char,
        func: Some(do_detach),
    },
    cmd {
        cmd: b"help\0".as_ptr() as *const c_char,
        func: Some(do_help),
    },
    cmd {
        cmd: core::ptr::null(),
        func: None,
    },
];

#[no_mangle]
pub unsafe extern "C" fn do_cgroup(argc: c_int, argv: *mut *mut c_char) -> c_int {
    cmd_select(CMDS.as_ptr(), argc, argv, do_help)
}
