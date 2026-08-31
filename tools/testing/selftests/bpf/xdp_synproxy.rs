// SPDX-License-Identifier: LGPL-2.1 OR BSD-2-Clause
/* Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved. */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

type __u8 = u8;
type __u16 = u16;
type __u32 = u32;
type __u64 = u64;

const UINT8_MAX: c_ulong = u8::MAX as c_ulong;
const UINT16_MAX: c_ulong = u16::MAX as c_ulong;
const UINT32_MAX: c_ulong = u32::MAX as c_ulong;
const PATH_MAX: usize = 4096;

const no_argument: c_int = 0;
const required_argument: c_int = 1;
const BPF_TC_INGRESS: c_int = 1;
const XDP_FLAGS_REPLACE: __u32 = 1 << 4;
const XDP_FLAGS_UPDATE_IF_NOEXIST: __u32 = 1;
const BPF_ANY: __u64 = 0;
const SIGINT: c_int = 2;
const SIGTERM: c_int = 15;
const SIG_DFL: sighandler_t = 0 as sighandler_t;
const EEXIST: c_int = 17;
const ENOENT: c_int = 2;

type c_char = i8;
type c_int = i32;
type c_uint = u32;
type c_ulong = u64;
type c_ulonglong = u64;
type size_t = usize;
type sighandler_t = Option<unsafe extern "C" fn(c_int)>;

#[repr(C)]
struct option {
    name: *const c_char,
    has_arg: c_int,
    flag: *mut c_int,
    val: c_int,
}

#[repr(C)]
struct bpf_xdp_attach_opts {
    old_prog_fd: c_int,
}

#[repr(C)]
struct bpf_tc_hook {
    ifindex: c_uint,
    attach_point: c_int,
}

#[repr(C)]
struct bpf_tc_opts {
    handle: __u32,
    priority: __u32,
    prog_fd: c_int,
}

#[repr(C)]
struct bpf_prog_info {
    id: __u32,
    nr_map_ids: __u32,
    map_ids: __u64,
}

#[repr(C)]
struct bpf_map_info {
    name: [c_char; 16],
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_object {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut errno: c_int;
    static mut optarg: *mut c_char;
    static mut optind: c_int;
    static mut stderr: *mut FILE;

    fn exit(status: c_int) -> !;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strtoul(nptr: *const c_char, endptr: *mut *mut c_char, base: c_int) -> c_ulong;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strsep(stringp: *mut *mut c_char, delim: *const c_char) -> *mut c_char;
    fn close(fd: c_int) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn getopt_long(
        argc: c_int,
        argv: *mut *mut c_char,
        optstring: *const c_char,
        longopts: *const option,
        longindex: *mut c_int,
    ) -> c_int;
    fn if_nametoindex(ifname: *const c_char) -> c_uint;
    fn signal(signum: c_int, handler: sighandler_t) -> sighandler_t;

    fn bpf_prog_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_xdp_attach(
        ifindex: c_uint,
        prog_fd: c_int,
        flags: __u32,
        opts: *const bpf_xdp_attach_opts,
    ) -> c_int;
    fn bpf_tc_hook_destroy(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_hook_create(hook: *mut bpf_tc_hook) -> c_int;
    fn bpf_tc_attach(hook: *mut bpf_tc_hook, opts: *mut bpf_tc_opts) -> c_int;
    fn bpf_object__open_file(path: *const c_char, opts: *const ()) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const bpf_object) -> c_int;
    fn bpf_object__load(obj: *mut bpf_object) -> c_int;
    fn bpf_object__find_program_by_name(
        obj: *mut bpf_object,
        name: *const c_char,
    ) -> *mut bpf_program;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_get_info_by_fd(
        prog_fd: c_int,
        info: *mut bpf_prog_info,
        info_len: *mut __u32,
    ) -> c_int;
    fn bpf_object__close(obj: *mut bpf_object);
    fn bpf_map_get_fd_by_id(id: __u32) -> c_int;
    fn bpf_map_get_info_by_fd(
        map_fd: c_int,
        info: *mut bpf_map_info,
        info_len: *mut __u32,
    ) -> c_int;
    fn bpf_xdp_query_id(ifindex: c_uint, flags: __u32, prog_id: *mut __u32) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const (),
        value: *const (),
        flags: __u64,
    ) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const (), value: *mut ()) -> c_int;
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

static mut ifindex: c_uint = 0;
static mut attached_prog_id: __u32 = 0;
static mut attached_tc: bool = false;

unsafe extern "C" fn cleanup(_sig: c_int) -> ! {
    let mut opts: bpf_xdp_attach_opts = bpf_xdp_attach_opts { old_prog_fd: 0 };
    let prog_fd: c_int;
    let mut err: c_int;

    if attached_prog_id == 0 {
        exit(0);
    }

    if attached_tc {
        let mut hook: bpf_tc_hook = bpf_tc_hook {
            ifindex,
            attach_point: BPF_TC_INGRESS,
        };

        err = bpf_tc_hook_destroy(&mut hook);
        if err < 0 {
            fprintf(
                stderr,
                c"Error: bpf_tc_hook_destroy: %s\n".as_ptr(),
                strerror(-err),
            );
            fprintf(stderr, c"Failed to destroy the TC hook\n".as_ptr());
            exit(1);
        }
        exit(0);
    }

    prog_fd = bpf_prog_get_fd_by_id(attached_prog_id);
    if prog_fd < 0 {
        fprintf(
            stderr,
            c"Error: bpf_prog_get_fd_by_id: %s\n".as_ptr(),
            strerror(-prog_fd),
        );
        err = bpf_xdp_attach(ifindex, -1, 0, core::ptr::null());
        if err < 0 {
            fprintf(
                stderr,
                c"Error: bpf_set_link_xdp_fd: %s\n".as_ptr(),
                strerror(-err),
            );
            fprintf(stderr, c"Failed to detach XDP program\n".as_ptr());
            exit(1);
        }
    } else {
        opts.old_prog_fd = prog_fd;
        err = bpf_xdp_attach(ifindex, -1, XDP_FLAGS_REPLACE, &opts);
        close(prog_fd);
        if err < 0 {
            fprintf(
                stderr,
                c"Error: bpf_set_link_xdp_fd_opts: %s\n".as_ptr(),
                strerror(-err),
            );
            /* Not an error if already replaced by someone else. */
            if err != -EEXIST {
                fprintf(stderr, c"Failed to detach XDP program\n".as_ptr());
                exit(1);
            }
        }
    }
    exit(0);
}

unsafe fn usage(progname: *const c_char) -> ! {
    fprintf(
        stderr,
        c"Usage: %s [--iface <iface>|--prog <prog_id>] [--mss4 <mss ipv4> --mss6 <mss ipv6> --wscale <wscale> --ttl <ttl>] [--ports <port1>,<port2>,...] [--single] [--tc]\n".as_ptr(),
        progname,
    );
    exit(1);
}

unsafe fn parse_arg_ul(progname: *const c_char, arg: *const c_char, limit: c_ulong) -> c_ulong {
    let res: c_ulong;
    let mut endptr: *mut c_char = core::ptr::null_mut();

    errno = 0;
    res = strtoul(arg, &mut endptr, 10);
    if errno != 0 || *endptr != b'\0' as c_char || *arg == b'\0' as c_char || res > limit {
        usage(progname);
    }

    res
}

unsafe fn parse_options(
    argc: c_int,
    argv: *mut *mut c_char,
    ifindex_out: *mut c_uint,
    prog_id: *mut __u32,
    tcpipopts: *mut __u64,
    ports: *mut *mut c_char,
    single: *mut bool,
    tc: *mut bool,
) {
    let long_options: [option; 11] = [
        option { name: c"help".as_ptr(), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'h' as c_int },
        option { name: c"iface".as_ptr(), has_arg: required_argument, flag: core::ptr::null_mut(), val: 'i' as c_int },
        option { name: c"prog".as_ptr(), has_arg: required_argument, flag: core::ptr::null_mut(), val: 'x' as c_int },
        option { name: c"mss4".as_ptr(), has_arg: required_argument, flag: core::ptr::null_mut(), val: 4 },
        option { name: c"mss6".as_ptr(), has_arg: required_argument, flag: core::ptr::null_mut(), val: 6 },
        option { name: c"wscale".as_ptr(), has_arg: required_argument, flag: core::ptr::null_mut(), val: 'w' as c_int },
        option { name: c"ttl".as_ptr(), has_arg: required_argument, flag: core::ptr::null_mut(), val: 't' as c_int },
        option { name: c"ports".as_ptr(), has_arg: required_argument, flag: core::ptr::null_mut(), val: 'p' as c_int },
        option { name: c"single".as_ptr(), has_arg: no_argument, flag: core::ptr::null_mut(), val: 's' as c_int },
        option { name: c"tc".as_ptr(), has_arg: no_argument, flag: core::ptr::null_mut(), val: 'c' as c_int },
        option { name: core::ptr::null(), has_arg: 0, flag: core::ptr::null_mut(), val: 0 },
    ];
    let mut mss4: c_ulong = 0;
    let mut wscale: c_ulong = 0;
    let mut ttl: c_ulong = 0;
    let mut mss6: c_ulonglong = 0;
    let mut tcpipopts_mask: c_uint = 0;

    if argc < 2 {
        usage(*argv);
    }

    *ifindex_out = 0;
    *prog_id = 0;
    *tcpipopts = 0;
    *ports = core::ptr::null_mut();
    *single = false;
    *tc = false;

    loop {
        let opt: c_int = getopt_long(argc, argv, c"".as_ptr(), long_options.as_ptr(), core::ptr::null_mut());
        if opt == -1 {
            break;
        }

        match opt {
            x if x == 'h' as c_int => {
                usage(*argv);
            }
            x if x == 'i' as c_int => {
                *ifindex_out = if_nametoindex(optarg);
                if *ifindex_out == 0 {
                    usage(*argv);
                }
            }
            x if x == 'x' as c_int => {
                *prog_id = parse_arg_ul(*argv, optarg, UINT32_MAX) as __u32;
                if *prog_id == 0 {
                    usage(*argv);
                }
            }
            4 => {
                mss4 = parse_arg_ul(*argv, optarg, UINT16_MAX);
                tcpipopts_mask |= 1 << 0;
            }
            6 => {
                mss6 = parse_arg_ul(*argv, optarg, UINT16_MAX);
                tcpipopts_mask |= 1 << 1;
            }
            x if x == 'w' as c_int => {
                wscale = parse_arg_ul(*argv, optarg, 14);
                tcpipopts_mask |= 1 << 2;
            }
            x if x == 't' as c_int => {
                ttl = parse_arg_ul(*argv, optarg, UINT8_MAX);
                tcpipopts_mask |= 1 << 3;
            }
            x if x == 'p' as c_int => {
                *ports = optarg;
            }
            x if x == 's' as c_int => {
                *single = true;
            }
            x if x == 'c' as c_int => {
                *tc = true;
            }
            _ => {
                usage(*argv);
            }
        }
    }
    if optind < argc {
        usage(*argv);
    }

    if tcpipopts_mask == 0xf {
        if mss4 == 0 || mss6 == 0 || wscale == 0 || ttl == 0 {
            usage(*argv);
        }
        *tcpipopts = (mss6 << 32) | (ttl << 24) | (wscale << 16) | mss4;
    } else if tcpipopts_mask != 0 {
        usage(*argv);
    }

    if *ifindex_out != 0 && *prog_id != 0 {
        usage(*argv);
    }
    if *ifindex_out == 0 && *prog_id == 0 {
        usage(*argv);
    }
}

unsafe fn syncookie_attach(argv0: *const c_char, ifindex_arg: c_uint, tc: bool) -> c_int {
    let mut info: bpf_prog_info = core::mem::zeroed();
    let mut info_len: __u32 = core::mem::size_of::<bpf_prog_info>() as __u32;
    let mut xdp_filename: [c_char; PATH_MAX] = [0; PATH_MAX];
    let prog: *mut bpf_program;
    let obj: *mut bpf_object;
    let prog_fd: c_int;
    let mut err: c_int;

    snprintf(
        xdp_filename.as_mut_ptr(),
        xdp_filename.len(),
        c"%s_kern.bpf.o".as_ptr(),
        argv0,
    );
    obj = bpf_object__open_file(xdp_filename.as_ptr(), core::ptr::null());
    err = libbpf_get_error(obj);
    if err < 0 {
        fprintf(stderr, c"Error: bpf_object__open_file: %s\n".as_ptr(), strerror(-err));
        return err;
    }

    err = bpf_object__load(obj);
    if err < 0 {
        fprintf(stderr, c"Error: bpf_object__open_file: %s\n".as_ptr(), strerror(-err));
        return err;
    }

    prog = bpf_object__find_program_by_name(
        obj,
        if tc { c"syncookie_tc".as_ptr() } else { c"syncookie_xdp".as_ptr() },
    );
    if prog.is_null() {
        fprintf(
            stderr,
            c"Error: bpf_object__find_program_by_name: program was not found\n".as_ptr(),
        );
        return -ENOENT;
    }

    prog_fd = bpf_program__fd(prog);

    err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut info_len);
    if err < 0 {
        fprintf(stderr, c"Error: bpf_prog_get_info_by_fd: %s\n".as_ptr(), strerror(-err));
        bpf_object__close(obj);
        return err;
    }
    attached_tc = tc;
    attached_prog_id = info.id;
    signal(SIGINT, Some(cleanup));
    signal(SIGTERM, Some(cleanup));
    if tc {
        let mut hook: bpf_tc_hook = bpf_tc_hook {
            ifindex: ifindex_arg,
            attach_point: BPF_TC_INGRESS,
        };
        let mut opts: bpf_tc_opts = bpf_tc_opts {
            handle: 1,
            priority: 1,
            prog_fd,
        };

        err = bpf_tc_hook_create(&mut hook);
        if err < 0 {
            fprintf(stderr, c"Error: bpf_tc_hook_create: %s\n".as_ptr(), strerror(-err));
            signal(SIGINT, SIG_DFL);
            signal(SIGTERM, SIG_DFL);
            attached_prog_id = 0;
            bpf_object__close(obj);
            return err;
        }
        err = bpf_tc_attach(&mut hook, &mut opts);
        if err < 0 {
            fprintf(stderr, c"Error: bpf_tc_attach: %s\n".as_ptr(), strerror(-err));
            signal(SIGINT, SIG_DFL);
            signal(SIGTERM, SIG_DFL);
            attached_prog_id = 0;
            bpf_object__close(obj);
            return err;
        }
    } else {
        err = bpf_xdp_attach(ifindex_arg, prog_fd, XDP_FLAGS_UPDATE_IF_NOEXIST, core::ptr::null());
        if err < 0 {
            fprintf(stderr, c"Error: bpf_set_link_xdp_fd: %s\n".as_ptr(), strerror(-err));
            signal(SIGINT, SIG_DFL);
            signal(SIGTERM, SIG_DFL);
            attached_prog_id = 0;
            bpf_object__close(obj);
            return err;
        }
    }
    err = 0;
    bpf_object__close(obj);
    err
}

unsafe fn syncookie_open_bpf_maps(
    prog_id: __u32,
    values_map_fd: *mut c_int,
    ports_map_fd: *mut c_int,
) -> c_int {
    let mut prog_info: bpf_prog_info;
    let mut map_ids: [__u32; 8] = [0; 8];
    let mut info_len: __u32;
    let prog_fd: c_int;
    let mut err: c_int;
    let mut i: c_int;

    *values_map_fd = -1;
    *ports_map_fd = -1;

    prog_fd = bpf_prog_get_fd_by_id(prog_id);
    if prog_fd < 0 {
        fprintf(
            stderr,
            c"Error: bpf_prog_get_fd_by_id: %s\n".as_ptr(),
            strerror(-prog_fd),
        );
        return prog_fd;
    }

    prog_info = bpf_prog_info {
        id: 0,
        nr_map_ids: 8,
        map_ids: map_ids.as_mut_ptr() as c_ulong as __u64,
    };
    info_len = core::mem::size_of::<bpf_prog_info>() as __u32;

    err = bpf_prog_get_info_by_fd(prog_fd, &mut prog_info, &mut info_len);
    if err != 0 {
        fprintf(stderr, c"Error: bpf_prog_get_info_by_fd: %s\n".as_ptr(), strerror(-err));
        close(prog_fd);
        return err;
    }

    if prog_info.nr_map_ids < 2 {
        fprintf(
            stderr,
            c"Error: Found %u BPF maps, expected at least 2\n".as_ptr(),
            prog_info.nr_map_ids,
        );
        err = -ENOENT;
        close(prog_fd);
        return err;
    }

    i = 0;
    while i < prog_info.nr_map_ids as c_int {
        let mut map_info: bpf_map_info = core::mem::zeroed();
        let map_fd: c_int;

        err = bpf_map_get_fd_by_id(map_ids[i as usize]);
        if err < 0 {
            fprintf(stderr, c"Error: bpf_map_get_fd_by_id: %s\n".as_ptr(), strerror(-err));
            if *values_map_fd != -1 {
                close(*values_map_fd);
            }
            if *ports_map_fd != -1 {
                close(*ports_map_fd);
            }
            *values_map_fd = -1;
            *ports_map_fd = -1;
            close(prog_fd);
            return err;
        }
        map_fd = err;

        info_len = core::mem::size_of::<bpf_map_info>() as __u32;
        err = bpf_map_get_info_by_fd(map_fd, &mut map_info, &mut info_len);
        if err != 0 {
            fprintf(stderr, c"Error: bpf_map_get_info_by_fd: %s\n".as_ptr(), strerror(-err));
            close(map_fd);
            if *values_map_fd != -1 {
                close(*values_map_fd);
            }
            if *ports_map_fd != -1 {
                close(*ports_map_fd);
            }
            *values_map_fd = -1;
            *ports_map_fd = -1;
            close(prog_fd);
            return err;
        }
        if strcmp(map_info.name.as_ptr(), c"values".as_ptr()) == 0 {
            *values_map_fd = map_fd;
            i += 1;
            continue;
        }
        if strcmp(map_info.name.as_ptr(), c"allowed_ports".as_ptr()) == 0 {
            *ports_map_fd = map_fd;
            i += 1;
            continue;
        }
        close(map_fd);
        i += 1;
    }

    if *values_map_fd != -1 && *ports_map_fd != -1 {
        err = 0;
        close(prog_fd);
        return err;
    }

    err = -ENOENT;

    if *values_map_fd != -1 {
        close(*values_map_fd);
    }
    if *ports_map_fd != -1 {
        close(*ports_map_fd);
    }
    *values_map_fd = -1;
    *ports_map_fd = -1;

    close(prog_fd);
    err
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut values_map_fd: c_int = 0;
    let mut ports_map_fd: c_int = 0;
    let mut tcpipopts: __u64 = 0;
    let mut firstiter: bool;
    let mut prevcnt: __u64;
    let mut prog_id: __u32 = 0;
    let mut ports: *mut c_char = core::ptr::null_mut();
    let mut single: bool = false;
    let mut err: c_int = 0;
    let mut tc: bool = false;

    parse_options(
        argc,
        argv,
        &mut ifindex,
        &mut prog_id,
        &mut tcpipopts,
        &mut ports,
        &mut single,
        &mut tc,
    );

    if prog_id == 0 {
        if !tc {
            err = bpf_xdp_query_id(ifindex, 0, &mut prog_id);
            if err < 0 {
                fprintf(stderr, c"Error: bpf_get_link_xdp_id: %s\n".as_ptr(), strerror(-err));
                return if err == 0 { 0 } else { 1 };
            }
        }
        if prog_id == 0 {
            err = syncookie_attach(*argv, ifindex, tc);
            if err < 0 {
                return if err == 0 { 0 } else { 1 };
            }
            prog_id = attached_prog_id;
        }
    }

    err = syncookie_open_bpf_maps(prog_id, &mut values_map_fd, &mut ports_map_fd);
    if err < 0 {
        return if err == 0 { 0 } else { 1 };
    }

    if !ports.is_null() {
        let port_last: __u16 = 0;
        let mut port_idx: __u32 = 0;
        let mut p: *mut c_char = ports;

        fprintf(stderr, c"Replacing allowed ports\n".as_ptr());

        while !p.is_null() && *p != b'\0' as c_char {
            let token: *mut c_char = strsep(&mut p, c",".as_ptr());
            let port: __u16;

            port = parse_arg_ul(*argv, token, UINT16_MAX) as __u16;
            err = bpf_map_update_elem(
                ports_map_fd,
                &port_idx as *const __u32 as *const (),
                &port as *const __u16 as *const (),
                BPF_ANY,
            );
            if err != 0 {
                fprintf(stderr, c"Error: bpf_map_update_elem: %s\n".as_ptr(), strerror(-err));
                fprintf(
                    stderr,
                    c"Failed to add port %u (index %u)\n".as_ptr(),
                    port as c_uint,
                    port_idx,
                );
                close(values_map_fd);
                close(ports_map_fd);
                return if err == 0 { 0 } else { 1 };
            }
            fprintf(stderr, c"Added port %u\n".as_ptr(), port as c_uint);
            port_idx = port_idx.wrapping_add(1);
        }
        err = bpf_map_update_elem(
            ports_map_fd,
            &port_idx as *const __u32 as *const (),
            &port_last as *const __u16 as *const (),
            BPF_ANY,
        );
        if err != 0 {
            fprintf(stderr, c"Error: bpf_map_update_elem: %s\n".as_ptr(), strerror(-err));
            fprintf(
                stderr,
                c"Failed to add the terminator value 0 (index %u)\n".as_ptr(),
                port_idx,
            );
            close(values_map_fd);
            close(ports_map_fd);
            return if err == 0 { 0 } else { 1 };
        }
    }

    if tcpipopts != 0 {
        let key: __u32 = 0;

        fprintf(stderr, c"Replacing TCP/IP options\n".as_ptr());

        err = bpf_map_update_elem(
            values_map_fd,
            &key as *const __u32 as *const (),
            &tcpipopts as *const __u64 as *const (),
            BPF_ANY,
        );
        if err != 0 {
            fprintf(stderr, c"Error: bpf_map_update_elem: %s\n".as_ptr(), strerror(-err));
            close(values_map_fd);
            close(ports_map_fd);
            return if err == 0 { 0 } else { 1 };
        }
    }

    if ((!ports.is_null()) || tcpipopts != 0) && attached_prog_id == 0 && !single {
        close(values_map_fd);
        close(ports_map_fd);
        return if err == 0 { 0 } else { 1 };
    }

    prevcnt = 0;
    firstiter = true;
    loop {
        let key: __u32 = 1;
        let mut value: __u64 = 0;

        err = bpf_map_lookup_elem(
            values_map_fd,
            &key as *const __u32 as *const (),
            &mut value as *mut __u64 as *mut (),
        );
        if err != 0 {
            fprintf(stderr, c"Error: bpf_map_lookup_elem: %s\n".as_ptr(), strerror(-err));
            close(values_map_fd);
            close(ports_map_fd);
            return if err == 0 { 0 } else { 1 };
        }
        if firstiter {
            prevcnt = value;
            firstiter = false;
        }
        if single {
            printf(c"Total SYNACKs generated: %llu\n".as_ptr(), value);
            break;
        }
        printf(
            c"SYNACKs generated: %llu (total %llu)\n".as_ptr(),
            value.wrapping_sub(prevcnt),
            value,
        );
        prevcnt = value;
        sleep(1);
    }

    close(values_map_fd);
    close(ports_map_fd);
    if err == 0 { 0 } else { 1 }
}
