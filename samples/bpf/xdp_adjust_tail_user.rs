/* SPDX-License-Identifier: GPL-2.0
 * Copyright (c) 2018 Facebook
 *
 * This program is free software; you can redistribute it and/or
 * modify it under the terms of version 2 of the GNU General Public
 * License as published by the Free Software Foundation.
 */

// Dependencies corresponding to the C headers are supplied externally.

const STATS_INTERVAL_S: u32 = 2;
const MAX_PCKT_SIZE: i32 = 600;

static mut ifindex: i32 = -1;
static mut xdp_flags: u32 = XDP_FLAGS_UPDATE_IF_NOEXIST;
static mut prog_id: u32 = 0;

unsafe extern "C" {
    fn bpf_xdp_query_id(ifindex: i32, flags: u32, prog_id: *mut u32) -> i32;
    fn bpf_xdp_detach(ifindex: i32, flags: u32, opts: *const core::ffi::c_void) -> i32;
    fn bpf_map_lookup_elem(fd: u32, key: *const core::ffi::c_void, value: *mut core::ffi::c_void) -> i32;
    fn bpf_map_update_elem(fd: i32, key: *const core::ffi::c_void, value: *const core::ffi::c_void, flags: u64) -> i32;
    fn bpf_object__open_file(filename: *const i8, opts: *const core::ffi::c_void) -> *mut bpf_object;
    fn libbpf_get_error(ptr: *const core::ffi::c_void) -> i64;
    fn bpf_object__next_program(obj: *mut bpf_object, prev: *mut bpf_program) -> *mut bpf_program;
    fn bpf_program__set_type(prog: *mut bpf_program, prog_type: u32) -> i32;
    fn bpf_object__load(obj: *mut bpf_object) -> i32;
    fn bpf_program__fd(prog: *mut bpf_program) -> i32;
    fn bpf_object__find_map_fd_by_name(obj: *mut bpf_object, name: *const i8) -> i32;
    fn bpf_xdp_attach(ifindex: i32, prog_fd: i32, flags: u32, opts: *const core::ffi::c_void) -> i32;
    fn bpf_prog_get_info_by_fd(fd: i32, info: *mut bpf_prog_info, info_len: *mut u32) -> i32;
    fn if_nametoindex(name: *const i8) -> u32;
    fn atoi(s: *const i8) -> i32;
    fn getopt(argc: i32, argv: *const *mut i8, optstring: *const i8) -> i32;
    static mut optarg: *mut i8;
    fn signal(sig: i32, handler: unsafe extern "C" fn(i32)) -> usize;
    fn sleep(seconds: u32) -> u32;
    fn time(timer: *mut i64) -> i64;
    fn strlen(s: *const i8) -> usize;
    fn snprintf(s: *mut i8, n: usize, format: *const i8, ...) -> i32;
    fn printf(format: *const i8, ...) -> i32;
    fn fprintf(stream: *mut core::ffi::c_void, format: *const i8, ...) -> i32;
    fn strerror(errnum: i32) -> *const i8;
    fn exit(status: i32) -> !;
}

#[repr(C)]
struct bpf_prog_info { id: u32, _rest: [u8; 0] }
#[repr(C)]
struct bpf_program { _private: [u8; 0] }
#[repr(C)]
struct bpf_object { _private: [u8; 0] }

unsafe extern "C" fn int_exit(_sig: i32) {
    let mut curr_prog_id: u32 = 0;

    if ifindex > -1 {
        if bpf_xdp_query_id(ifindex, xdp_flags, &mut curr_prog_id) != 0 {
            printf(b"bpf_xdp_query_id failed\0".as_ptr() as *const i8);
            exit(1);
        }
        if prog_id == curr_prog_id {
            bpf_xdp_detach(ifindex, xdp_flags, core::ptr::null());
        } else if curr_prog_id == 0 {
            printf(b"couldn't find a prog id on a given iface\n\0".as_ptr() as *const i8);
        } else {
            printf(b"program on interface changed, not removing\n\0".as_ptr() as *const i8);
        }
    }
    exit(0);
}

/* simple "icmp packet too big sent" counter
 */
unsafe fn poll_stats(map_fd: u32, kill_after_s: u32) {
    let started_at = time(core::ptr::null_mut());
    let mut value: u64 = 0;
    let mut key: i32 = 0;

    while kill_after_s == 0 || time(core::ptr::null_mut()) - started_at <= kill_after_s as i64 {
        sleep(STATS_INTERVAL_S);
        assert!(bpf_map_lookup_elem(map_fd, &key as *const _ as *const _, &mut value as *mut _ as *mut _) == 0);
        printf(b"icmp \"packet too big\" sent: %10llu pkts\n\0".as_ptr() as *const i8, value);
    }
}

unsafe fn usage(cmd: *const i8) {
    printf(b"Start a XDP prog which send ICMP \"packet too big\" \n\0".as_ptr() as *const i8);
    printf(b"messages if ingress packet is bigger then MAX_SIZE bytes\n\0".as_ptr() as *const i8);
    printf(b"Usage: %s [...]\n\0".as_ptr() as *const i8, cmd);
    printf(b"    -i <ifname|ifindex> Interface\n\0".as_ptr() as *const i8);
    printf(b"    -T <stop-after-X-seconds> Default: 0 (forever)\n\0".as_ptr() as *const i8);
    printf(b"    -P <MAX_PCKT_SIZE> Default: %u\n\0".as_ptr() as *const i8, MAX_PCKT_SIZE);
    printf(b"    -S use skb-mode\n\0".as_ptr() as *const i8);
    printf(b"    -N enforce native mode\n\0".as_ptr() as *const i8);
    printf(b"    -F force loading prog\n\0".as_ptr() as *const i8);
    printf(b"    -h Display this help\n\0".as_ptr() as *const i8);
}

unsafe fn main() -> i32 {
    let mut opt_flags = [0u8; 256];
    let optstr = b"i:T:P:SNFh\0";
    let mut info = bpf_prog_info { id: 0, _rest: [] };
    let mut info_len = core::mem::size_of::<bpf_prog_info>() as u32;
    let mut kill_after_s: u32 = 0;
    let mut prog_fd: i32;
    let mut map_fd: i32;
    let mut max_pckt_size: u32 = 0;
    let mut key: u32 = 0;
    let mut filename = [0i8; 256];
    let mut err: i32;

    let argc = 0i32;
    let argv: *mut *mut i8 = core::ptr::null_mut();
    for i in 0..strlen(optstr.as_ptr() as *const i8) {
        let c = optstr[i];
        if c != b'h' && c >= b'a' && c <= b'z' { opt_flags[c as usize] = 1; }
    }
    while { let opt = getopt(argc, argv, optstr.as_ptr() as *const i8); opt != -1 } {
        let opt = getopt(argc, argv, optstr.as_ptr() as *const i8);
        match opt {
            105 => { ifindex = if_nametoindex(optarg) as i32; if ifindex == 0 { ifindex = atoi(optarg); } }
            84 => kill_after_s = atoi(optarg) as u32,
            80 => max_pckt_size = atoi(optarg) as u32,
            83 => xdp_flags |= XDP_FLAGS_SKB_MODE,
            78 => (),
            70 => xdp_flags &= !XDP_FLAGS_UPDATE_IF_NOEXIST,
            _ => { usage(core::ptr::null()); return 1; }
        }
        opt_flags[opt as usize] = 0;
    }
    if xdp_flags & XDP_FLAGS_SKB_MODE == 0 { xdp_flags |= XDP_FLAGS_DRV_MODE; }
    for i in 0..strlen(optstr.as_ptr() as *const i8) {
        if opt_flags[optstr[i] as usize] != 0 { fprintf(core::ptr::null_mut(), b"Missing argument -%c\n\0".as_ptr() as *const i8, optstr[i]); usage(core::ptr::null()); return 1; }
    }
    if ifindex == 0 { fprintf(core::ptr::null_mut(), b"Invalid ifname\n\0".as_ptr() as *const i8); return 1; }
    snprintf(filename.as_mut_ptr(), filename.len(), b"%s_kern.o\0".as_ptr() as *const i8, argv);
    let obj = bpf_object__open_file(filename.as_ptr(), core::ptr::null());
    if libbpf_get_error(obj as *const _) != 0 { return 1; }
    let prog = bpf_object__next_program(obj, core::ptr::null_mut());
    bpf_program__set_type(prog, BPF_PROG_TYPE_XDP);
    err = bpf_object__load(obj); if err != 0 { return 1; }
    prog_fd = bpf_program__fd(prog);
    if max_pckt_size != 0 { map_fd = bpf_object__find_map_fd_by_name(obj, b"xdp_adju.data\0".as_ptr() as *const i8); if map_fd < 0 { printf(b"finding a max_pcktsz map in obj file failed\n\0".as_ptr() as *const i8); return 1; } bpf_map_update_elem(map_fd, &key as *const _ as *const _, &max_pckt_size as *const _ as *const _, BPF_ANY); }
    map_fd = bpf_object__find_map_fd_by_name(obj, b"icmpcnt\0".as_ptr() as *const i8); if map_fd < 0 { printf(b"finding a icmpcnt map in obj file failed\n\0".as_ptr() as *const i8); return 1; }
    signal(SIGINT, int_exit); signal(SIGTERM, int_exit);
    if bpf_xdp_attach(ifindex, prog_fd, xdp_flags, core::ptr::null()) < 0 { printf(b"link set xdp fd failed\n\0".as_ptr() as *const i8); return 1; }
    err = bpf_prog_get_info_by_fd(prog_fd, &mut info, &mut info_len); if err != 0 { printf(b"can't get prog info - %s\n\0".as_ptr() as *const i8, strerror(errno)); return 1; }
    prog_id = info.id; poll_stats(map_fd as u32, kill_after_s); int_exit(0); 0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
