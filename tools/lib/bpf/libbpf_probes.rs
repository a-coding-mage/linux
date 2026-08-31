// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
/* Copyright (c) 2019 Netronome Systems, Inc. */

/* Translated from lib/bpf/libbpf_probes.c.
 * C include dependencies intentionally remain external to this translation:
 * errno/fcntl/string/stdlib/unistd/net/if/sys/utsname, linux/btf.h,
 * linux/filter.h, linux/kernel.h, linux/version.h, bpf.h, libbpf.h,
 * and libbpf_internal.h.
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::{mem, ptr};

type size_t = usize;
type __u8 = u8;
type __u32 = u32;
type __u64 = u64;

extern "C" {
    fn faccessat(dirfd: c_int, pathname: *const c_char, mode: c_int, flags: c_int) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn uname(buf: *mut utsname) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn close(fd: c_int) -> c_int;
    fn sysconf(name: c_int) -> isize;

    fn bpf_prog_load(
        prog_type: bpf_prog_type,
        prog_name: *const c_char,
        license: *const c_char,
        insns: *const bpf_insn,
        insn_cnt: size_t,
        opts: *const bpf_prog_load_opts,
    ) -> c_int;
    fn bpf_btf_load(
        raw_btf: *const c_void,
        raw_btf_size: size_t,
        opts: *const bpf_btf_load_opts,
    ) -> c_int;
    fn bpf_map_create(
        map_type: bpf_map_type,
        map_name: *const c_char,
        key_size: c_int,
        value_size: c_int,
        max_entries: c_int,
        opts: *const bpf_map_create_opts,
    ) -> c_int;
    fn libbpf_err(err: c_int) -> c_int;
}

extern "C" {
    static mut errno: c_int;
}

/* External C/kernel/libbpf types, constants, and macros expected from bindings:
 * FILE, utsname, btf_header, bpf_insn, bpf_prog_load_opts, bpf_btf_load_opts,
 * bpf_map_create_opts, bpf_cgroup_storage_key, bpf_insn_array_value,
 * bpf_prog_type, bpf_map_type, bpf_func_id, AT_FDCWD, R_OK, AT_EACCESS,
 * EINVAL, ENOMEM, EOPNOTSUPP, _SC_PAGE_SIZE, BTF_MAGIC, BTF_VERSION,
 * BTF_INT_SIGNED, BTF_KIND_STRUCT, BPF_F_TOKEN_FD, BPF_F_NO_PREALLOC,
 * BPF_F_MMAPABLE, BPF_F_SLEEPABLE, and all BPF_* enum values used below.
 */

/* On Ubuntu LINUX_VERSION_CODE doesn't correspond to info.release,
 * but Ubuntu provides /proc/version_signature file, as described at
 * https://ubuntu.com/kernel, with an example contents below, which we
 * can use to get a proper LINUX_VERSION_CODE.
 *
 *   Ubuntu 5.4.0-12.15-generic 5.4.8
 *
 * In the above, 5.4.8 is what kernel is actually expecting, while
 * uname() call will return 5.4.0 in info.release.
 */
unsafe fn get_ubuntu_kernel_version() -> __u32 {
    let ubuntu_kver_file = b"/proc/version_signature\0".as_ptr() as *const c_char;
    let mut major: __u32 = 0;
    let mut minor: __u32 = 0;
    let mut patch: __u32 = 0;
    let ret: c_int;
    let f: *mut FILE;

    if faccessat(AT_FDCWD, ubuntu_kver_file, R_OK, AT_EACCESS) != 0 {
        return 0;
    }

    f = fopen(ubuntu_kver_file, b"re\0".as_ptr() as *const c_char);
    if f.is_null() {
        return 0;
    }

    ret = fscanf(
        f,
        b"%*s %*s %u.%u.%u\n\0".as_ptr() as *const c_char,
        &mut major,
        &mut minor,
        &mut patch,
    );
    fclose(f);
    if ret != 3 {
        return 0;
    }

    KERNEL_VERSION(major, minor, patch)
}

/* On Debian LINUX_VERSION_CODE doesn't correspond to info.release.
 * Instead, it is provided in info.version. An example content of
 * Debian 10 looks like the below.
 *
 *   utsname::release   4.19.0-22-amd64
 *   utsname::version   #1 SMP Debian 4.19.260-1 (2022-09-29)
 *
 * In the above, 4.19.260 is what kernel is actually expecting, while
 * uname() call will return 4.19.0 in info.release.
 */
unsafe fn get_debian_kernel_version(info: *mut utsname) -> __u32 {
    let mut major: __u32 = 0;
    let mut minor: __u32 = 0;
    let mut patch: __u32 = 0;
    let p: *mut c_char;

    p = strstr((*info).version.as_ptr(), b"Debian \0".as_ptr() as *const c_char);
    if p.is_null() {
        /* This is not a Debian kernel. */
        return 0;
    }

    if sscanf(
        p,
        b"Debian %u.%u.%u\0".as_ptr() as *const c_char,
        &mut major,
        &mut minor,
        &mut patch,
    ) != 3
    {
        return 0;
    }

    KERNEL_VERSION(major, minor, patch)
}

#[no_mangle]
pub unsafe extern "C" fn get_kernel_version() -> __u32 {
    let mut major: __u32 = 0;
    let mut minor: __u32 = 0;
    let mut patch: __u32 = 0;
    let mut version: __u32;
    let mut info: utsname = mem::zeroed();

    /* Check if this is an Ubuntu kernel. */
    version = get_ubuntu_kernel_version();
    if version != 0 {
        return version;
    }

    uname(&mut info);

    /* Check if this is a Debian kernel. */
    version = get_debian_kernel_version(&mut info);
    if version != 0 {
        return version;
    }

    if sscanf(
        info.release.as_ptr(),
        b"%u.%u.%u\0".as_ptr() as *const c_char,
        &mut major,
        &mut minor,
        &mut patch,
    ) != 3
    {
        return 0;
    }

    KERNEL_VERSION(major, minor, patch)
}

unsafe fn probe_prog_load(
    prog_type: bpf_prog_type,
    insns: *const bpf_insn,
    insns_cnt: size_t,
    log_buf: *mut c_char,
    log_buf_sz: size_t,
) -> c_int {
    let mut opts: bpf_prog_load_opts = mem::zeroed();
    opts.sz = mem::size_of::<bpf_prog_load_opts>() as size_t;
    opts.log_buf = log_buf;
    opts.log_size = log_buf_sz;
    opts.log_level = if !log_buf.is_null() { 1 } else { 0 };

    let fd: c_int;
    let err: c_int;
    let mut exp_err: c_int = 0;
    let mut exp_msg: *const c_char = ptr::null();
    let mut buf = [0 as c_char; 4096];

    match prog_type {
        BPF_PROG_TYPE_CGROUP_SOCK_ADDR => opts.expected_attach_type = BPF_CGROUP_INET4_CONNECT,
        BPF_PROG_TYPE_CGROUP_SOCKOPT => opts.expected_attach_type = BPF_CGROUP_GETSOCKOPT,
        BPF_PROG_TYPE_SK_LOOKUP => opts.expected_attach_type = BPF_SK_LOOKUP,
        BPF_PROG_TYPE_KPROBE => opts.kern_version = get_kernel_version(),
        BPF_PROG_TYPE_LIRC_MODE2 => opts.expected_attach_type = BPF_LIRC_MODE2,
        BPF_PROG_TYPE_TRACING | BPF_PROG_TYPE_LSM => {
            opts.log_buf = buf.as_mut_ptr();
            opts.log_size = mem::size_of_val(&buf);
            opts.log_level = 1;
            if prog_type == BPF_PROG_TYPE_TRACING {
                opts.expected_attach_type = BPF_TRACE_FENTRY;
            } else {
                opts.expected_attach_type = BPF_MODIFY_RETURN;
            }
            opts.attach_btf_id = 1;

            exp_err = -EINVAL;
            exp_msg = b"attach_btf_id 1 is not a function\0".as_ptr() as *const c_char;
        }
        BPF_PROG_TYPE_EXT => {
            opts.log_buf = buf.as_mut_ptr();
            opts.log_size = mem::size_of_val(&buf);
            opts.log_level = 1;
            opts.attach_btf_id = 1;

            exp_err = -EINVAL;
            exp_msg = b"Cannot replace kernel functions\0".as_ptr() as *const c_char;
        }
        BPF_PROG_TYPE_SYSCALL => opts.prog_flags = BPF_F_SLEEPABLE,
        BPF_PROG_TYPE_STRUCT_OPS => exp_err = -524, /* -ENOTSUPP */
        BPF_PROG_TYPE_UNSPEC
        | BPF_PROG_TYPE_SOCKET_FILTER
        | BPF_PROG_TYPE_SCHED_CLS
        | BPF_PROG_TYPE_SCHED_ACT
        | BPF_PROG_TYPE_TRACEPOINT
        | BPF_PROG_TYPE_XDP
        | BPF_PROG_TYPE_PERF_EVENT
        | BPF_PROG_TYPE_CGROUP_SKB
        | BPF_PROG_TYPE_CGROUP_SOCK
        | BPF_PROG_TYPE_LWT_IN
        | BPF_PROG_TYPE_LWT_OUT
        | BPF_PROG_TYPE_LWT_XMIT
        | BPF_PROG_TYPE_SOCK_OPS
        | BPF_PROG_TYPE_SK_SKB
        | BPF_PROG_TYPE_CGROUP_DEVICE
        | BPF_PROG_TYPE_SK_MSG
        | BPF_PROG_TYPE_RAW_TRACEPOINT
        | BPF_PROG_TYPE_RAW_TRACEPOINT_WRITABLE
        | BPF_PROG_TYPE_LWT_SEG6LOCAL
        | BPF_PROG_TYPE_SK_REUSEPORT
        | BPF_PROG_TYPE_FLOW_DISSECTOR
        | BPF_PROG_TYPE_CGROUP_SYSCTL => {}
        BPF_PROG_TYPE_NETFILTER => opts.expected_attach_type = BPF_NETFILTER,
        _ => return -EOPNOTSUPP,
    }

    fd = bpf_prog_load(
        prog_type,
        ptr::null(),
        b"GPL\0".as_ptr() as *const c_char,
        insns,
        insns_cnt,
        &opts,
    );
    err = -errno;
    if fd >= 0 {
        close(fd);
    }
    if exp_err != 0 {
        if fd >= 0 || err != exp_err {
            return 0;
        }
        if !exp_msg.is_null() && strstr(buf.as_ptr(), exp_msg).is_null() {
            return 0;
        }
        return 1;
    }
    if fd >= 0 { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn libbpf_probe_bpf_prog_type(
    prog_type: bpf_prog_type,
    opts: *const c_void,
) -> c_int {
    let insns = [
        BPF_MOV64_IMM(BPF_REG_0, 0),
        BPF_EXIT_INSN(),
    ];
    let insn_cnt: size_t = ARRAY_SIZE(&insns);
    let ret: c_int;

    if !opts.is_null() {
        return libbpf_err(-EINVAL);
    }

    ret = probe_prog_load(prog_type, insns.as_ptr(), insn_cnt, ptr::null_mut(), 0);
    libbpf_err(ret)
}

#[no_mangle]
pub unsafe extern "C" fn libbpf__load_raw_btf_hdr(
    hdr: *const btf_header,
    raw_types: *const c_char,
    str_sec: *const c_char,
    layout_sec: *const c_char,
    token_fd: c_int,
) -> c_int {
    let mut opts: bpf_btf_load_opts = mem::zeroed();
    opts.sz = mem::size_of::<bpf_btf_load_opts>() as size_t;
    opts.token_fd = token_fd;
    opts.btf_flags = if token_fd != 0 { BPF_F_TOKEN_FD } else { 0 };

    let btf_fd: c_int;
    let btf_len: c_int;
    let raw_btf: *mut __u8;

    btf_len = ((*hdr).hdr_len + (*hdr).type_off + (*hdr).type_len + (*hdr).str_len + (*hdr).layout_len) as c_int;
    raw_btf = malloc(btf_len as size_t) as *mut __u8;
    if raw_btf.is_null() {
        return -ENOMEM;
    }

    memcpy(raw_btf as *mut c_void, hdr as *const c_void, mem::size_of_val(&*hdr));
    memcpy(
        raw_btf.add(((*hdr).hdr_len + (*hdr).type_off) as usize) as *mut c_void,
        raw_types as *const c_void,
        (*hdr).type_len as size_t,
    );
    memcpy(
        raw_btf.add(((*hdr).hdr_len + (*hdr).str_off) as usize) as *mut c_void,
        str_sec as *const c_void,
        (*hdr).str_len as size_t,
    );
    if !layout_sec.is_null() {
        memcpy(
            raw_btf.add(((*hdr).hdr_len + (*hdr).layout_off) as usize) as *mut c_void,
            layout_sec as *const c_void,
            (*hdr).layout_len as size_t,
        );
    }

    btf_fd = bpf_btf_load(raw_btf as *const c_void, btf_len as size_t, &opts);

    free(raw_btf as *mut c_void);
    btf_fd
}

#[no_mangle]
pub unsafe extern "C" fn libbpf__load_raw_btf(
    raw_types: *const c_char,
    types_len: size_t,
    str_sec: *const c_char,
    str_len: size_t,
    token_fd: c_int,
) -> c_int {
    let hdr = btf_header {
        magic: BTF_MAGIC,
        version: BTF_VERSION,
        flags: 0,
        hdr_len: mem::size_of::<btf_header>() as __u32,
        type_off: 0,
        type_len: types_len as __u32,
        str_off: types_len as __u32,
        str_len: str_len as __u32,
        layout_off: 0,
        layout_len: 0,
    };

    libbpf__load_raw_btf_hdr(&hdr, raw_types, str_sec, ptr::null(), token_fd)
}

unsafe fn load_local_storage_btf() -> c_int {
    let strs = b"\0bpf_spin_lock\0val\0cnt\0l\0";
    /* struct bpf_spin_lock {
     *   int val;
     * };
     * struct val {
     *   int cnt;
     *   struct bpf_spin_lock l;
     * };
     */
    let mut types: [__u32; 6] = [
        /* int */
        BTF_TYPE_INT_ENC(0, BTF_INT_SIGNED, 0, 32, 4),  /* [1] */
        /* struct bpf_spin_lock */                      /* [2] */
        BTF_TYPE_ENC(1, BTF_INFO_ENC(BTF_KIND_STRUCT, 0, 1), 4),
        BTF_MEMBER_ENC(15, 1, 0), /* int val; */
        /* struct val */                                /* [3] */
        BTF_TYPE_ENC(15, BTF_INFO_ENC(BTF_KIND_STRUCT, 0, 2), 8),
        BTF_MEMBER_ENC(19, 1, 0), /* int cnt; */
        BTF_MEMBER_ENC(23, 2, 32),/* struct bpf_spin_lock l; */
    ];

    libbpf__load_raw_btf(
        types.as_mut_ptr() as *mut c_char,
        mem::size_of_val(&types),
        strs.as_ptr() as *const c_char,
        mem::size_of_val(strs),
        0,
    )
}

unsafe fn probe_map_create(map_type: bpf_map_type) -> c_int {
    let mut opts: bpf_map_create_opts = mem::zeroed();
    opts.sz = mem::size_of::<bpf_map_create_opts>() as size_t;
    let mut key_size: c_int;
    let mut value_size: c_int;
    let mut max_entries: c_int;
    let mut btf_key_type_id: __u32 = 0;
    let mut btf_value_type_id: __u32 = 0;
    let mut fd: c_int = -1;
    let mut btf_fd: c_int = -1;
    let mut fd_inner: c_int = -1;
    let mut exp_err: c_int = 0;
    let mut err: c_int = 0;

    key_size = mem::size_of::<__u32>() as c_int;
    value_size = mem::size_of::<__u32>() as c_int;
    max_entries = 1;

    match map_type {
        BPF_MAP_TYPE_STACK_TRACE => value_size = mem::size_of::<__u64>() as c_int,
        BPF_MAP_TYPE_LPM_TRIE => {
            key_size = mem::size_of::<__u64>() as c_int;
            value_size = mem::size_of::<__u64>() as c_int;
            opts.map_flags = BPF_F_NO_PREALLOC;
        }
        BPF_MAP_TYPE_RHASH => opts.map_flags = BPF_F_NO_PREALLOC,
        BPF_MAP_TYPE_CGROUP_STORAGE | BPF_MAP_TYPE_PERCPU_CGROUP_STORAGE => {
            key_size = mem::size_of::<bpf_cgroup_storage_key>() as c_int;
            value_size = mem::size_of::<__u64>() as c_int;
            max_entries = 0;
        }
        BPF_MAP_TYPE_QUEUE | BPF_MAP_TYPE_STACK => key_size = 0,
        BPF_MAP_TYPE_SK_STORAGE
        | BPF_MAP_TYPE_INODE_STORAGE
        | BPF_MAP_TYPE_TASK_STORAGE
        | BPF_MAP_TYPE_CGRP_STORAGE => {
            btf_key_type_id = 1;
            btf_value_type_id = 3;
            value_size = 8;
            max_entries = 0;
            opts.map_flags = BPF_F_NO_PREALLOC;
            btf_fd = load_local_storage_btf();
            if btf_fd < 0 {
                return btf_fd;
            }
        }
        BPF_MAP_TYPE_RINGBUF | BPF_MAP_TYPE_USER_RINGBUF => {
            key_size = 0;
            value_size = 0;
            max_entries = sysconf(_SC_PAGE_SIZE) as c_int;
        }
        BPF_MAP_TYPE_STRUCT_OPS => {
            /* we'll get -ENOTSUPP for invalid BTF type ID for struct_ops */
            opts.btf_vmlinux_value_type_id = 1;
            opts.value_type_btf_obj_fd = -1;
            exp_err = -524; /* -ENOTSUPP */
        }
        BPF_MAP_TYPE_BLOOM_FILTER => {
            key_size = 0;
            max_entries = 1;
        }
        BPF_MAP_TYPE_ARENA => {
            key_size = 0;
            value_size = 0;
            max_entries = 1; /* one page */
            opts.map_extra = 0; /* can mmap() at any address */
            opts.map_flags = BPF_F_MMAPABLE;
        }
        BPF_MAP_TYPE_HASH
        | BPF_MAP_TYPE_ARRAY
        | BPF_MAP_TYPE_PROG_ARRAY
        | BPF_MAP_TYPE_PERF_EVENT_ARRAY
        | BPF_MAP_TYPE_PERCPU_HASH
        | BPF_MAP_TYPE_PERCPU_ARRAY
        | BPF_MAP_TYPE_CGROUP_ARRAY
        | BPF_MAP_TYPE_LRU_HASH
        | BPF_MAP_TYPE_LRU_PERCPU_HASH
        | BPF_MAP_TYPE_ARRAY_OF_MAPS
        | BPF_MAP_TYPE_HASH_OF_MAPS
        | BPF_MAP_TYPE_DEVMAP
        | BPF_MAP_TYPE_DEVMAP_HASH
        | BPF_MAP_TYPE_SOCKMAP
        | BPF_MAP_TYPE_CPUMAP
        | BPF_MAP_TYPE_XSKMAP
        | BPF_MAP_TYPE_SOCKHASH
        | BPF_MAP_TYPE_REUSEPORT_SOCKARRAY => {}
        BPF_MAP_TYPE_INSN_ARRAY => {
            key_size = mem::size_of::<__u32>() as c_int;
            value_size = mem::size_of::<bpf_insn_array_value>() as c_int;
        }
        BPF_MAP_TYPE_UNSPEC => return -EOPNOTSUPP,
        _ => return -EOPNOTSUPP,
    }

    'create: loop {
        if map_type == BPF_MAP_TYPE_ARRAY_OF_MAPS || map_type == BPF_MAP_TYPE_HASH_OF_MAPS {
            fd_inner = bpf_map_create(
                BPF_MAP_TYPE_HASH,
                ptr::null(),
                mem::size_of::<__u32>() as c_int,
                mem::size_of::<__u32>() as c_int,
                1,
                ptr::null(),
            );
            if fd_inner < 0 {
                break 'create;
            }

            opts.inner_map_fd = fd_inner;
        }

        if btf_fd >= 0 {
            opts.btf_fd = btf_fd;
            opts.btf_key_type_id = btf_key_type_id;
            opts.btf_value_type_id = btf_value_type_id;
        }

        fd = bpf_map_create(map_type, ptr::null(), key_size, value_size, max_entries, &opts);
        err = -errno;
        break 'create;
    }

    if fd >= 0 {
        close(fd);
    }
    if fd_inner >= 0 {
        close(fd_inner);
    }
    if btf_fd >= 0 {
        close(btf_fd);
    }

    if exp_err != 0 {
        if fd < 0 && err == exp_err { 1 } else { 0 }
    } else {
        if fd >= 0 { 1 } else { 0 }
    }
}

#[no_mangle]
pub unsafe extern "C" fn libbpf_probe_bpf_map_type(
    map_type: bpf_map_type,
    opts: *const c_void,
) -> c_int {
    let ret: c_int;

    if !opts.is_null() {
        return libbpf_err(-EINVAL);
    }

    ret = probe_map_create(map_type);
    libbpf_err(ret)
}

#[no_mangle]
pub unsafe extern "C" fn libbpf_probe_bpf_helper(
    prog_type: bpf_prog_type,
    helper_id: bpf_func_id,
    opts: *const c_void,
) -> c_int {
    let insns = [
        BPF_EMIT_CALL(helper_id as __u32),
        BPF_EXIT_INSN(),
    ];
    let insn_cnt: size_t = ARRAY_SIZE(&insns);
    let mut buf = [0 as c_char; 4096];
    let ret: c_int;

    if !opts.is_null() {
        return libbpf_err(-EINVAL);
    }

    /* we can't successfully load all prog types to check for BPF helper
     * support, so bail out with -EOPNOTSUPP error
     */
    match prog_type {
        BPF_PROG_TYPE_TRACING
        | BPF_PROG_TYPE_EXT
        | BPF_PROG_TYPE_LSM
        | BPF_PROG_TYPE_STRUCT_OPS => return -EOPNOTSUPP,
        _ => {}
    }

    buf[0] = 0;
    ret = probe_prog_load(prog_type, insns.as_ptr(), insn_cnt, buf.as_mut_ptr(), mem::size_of_val(&buf));
    if ret < 0 {
        return libbpf_err(ret);
    }

    /* If BPF verifier doesn't recognize BPF helper ID (enum bpf_func_id)
     * at all, it will emit something like "invalid func unknown#181".
     * If BPF verifier recognizes BPF helper but it's not supported for
     * given BPF program type, it will emit "unknown func bpf_sys_bpf#166"
     * or "program of this type cannot use helper bpf_sys_bpf#166".
     * In both cases, provided combination of BPF program type and BPF
     * helper is not supported by the kernel.
     * In all other cases, probe_prog_load() above will either succeed (e.g.,
     * because BPF helper happens to accept no input arguments or it
     * accepts one input argument and initial PTR_TO_CTX is fine for
     * that), or we'll get some more specific BPF verifier error about
     * some unsatisfied conditions.
     */
    if ret == 0
        && (!strstr(buf.as_ptr(), b"invalid func \0".as_ptr() as *const c_char).is_null()
            || !strstr(buf.as_ptr(), b"unknown func \0".as_ptr() as *const c_char).is_null()
            || !strstr(
                buf.as_ptr(),
                b"program of this type cannot use helper \0".as_ptr() as *const c_char,
            )
            .is_null())
    {
        return 0;
    }
    1 /* assume supported */
}
