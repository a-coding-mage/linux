// SPDX-License-Identifier: GPL-2.0
// C dependencies: <test_progs.h>, <network_helpers.h>, "test_xdp_noinline.skel.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type __u8 = u8;
type __u32 = u32;
type __u64 = u64;
type __be32 = u32;
type u32 = u32;

#[repr(C)]
pub struct vip {
    pub protocol: __u8,
}

#[repr(C)]
pub struct bpf_map;

#[repr(C)]
pub struct bpf_program;

#[repr(C)]
pub struct test_xdp_noinline_maps {
    pub vip_map: *mut bpf_map,
    pub ch_rings: *mut bpf_map,
    pub reals: *mut bpf_map,
    pub stats: *mut bpf_map,
}

#[repr(C)]
pub struct test_xdp_noinline_progs {
    pub balancer_ingress_v4: *mut bpf_program,
    pub balancer_ingress_v6: *mut bpf_program,
}

#[repr(C)]
pub struct test_xdp_noinline {
    pub maps: test_xdp_noinline_maps,
    pub progs: test_xdp_noinline_progs,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *const c_void,
    pub data_size_in: u32,
    pub data_out: *mut c_void,
    pub data_size_out: u32,
    pub repeat: u32,
    pub retval: u32,
}

unsafe extern "C" {
    static pkt_v4: c_void;
    static pkt_v6: c_void;

    static VIP_NUM: __u32;
    static MAGIC_VAL: u32;
    static MAGIC_BYTES: __u64;
    static NUM_ITER: u32;
    static pkt_v4_size: usize;
    static pkt_v6_size: usize;

    fn bpf_num_possible_cpus() -> c_uint;
    fn test_xdp_noinline__open_and_load() -> *mut test_xdp_noinline;
    fn test_xdp_noinline__destroy(skel: *mut test_xdp_noinline);
    fn bpf_map__fd(map: *mut bpf_map) -> c_int;
    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_map_update_elem(
        fd: c_int,
        key: *const c_void,
        value: *const c_void,
        flags: __u64,
    ) -> c_int;
    fn bpf_map_lookup_elem(fd: c_int, key: *const c_void, value: *mut c_void) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: __u64, right: __u64, name: *const c_char) -> bool;
}

#[repr(C)]
struct vip_meta {
    flags: __u32,
    vip_num: __u32,
}

#[repr(C)]
struct vip_stats {
    bytes: __u64,
    pkts: __u64,
}

#[repr(C)]
union real_definition_union {
    dst: __be32,
    dstv6: [__be32; 4],
}

#[repr(C)]
struct real_definition {
    u: real_definition_union,
    flags: __u8,
}

pub unsafe fn test_xdp_noinline() {
    let nr_cpus: c_uint = unsafe { bpf_num_possible_cpus() };
    let mut skel: *mut test_xdp_noinline;
    let mut key: vip = vip { protocol: 6 };
    let mut value: vip_meta = vip_meta {
        flags: 0,
        vip_num: unsafe { VIP_NUM },
    };
    let mut stats_key: __u32 = unsafe { VIP_NUM };
    let mut stats: Vec<vip_stats> = (0..nr_cpus)
        .map(|_| vip_stats { bytes: 0, pkts: 0 })
        .collect();
    let mut real_def: real_definition = real_definition {
        u: real_definition_union {
            dst: unsafe { MAGIC_VAL },
        },
        flags: 0,
    };
    let mut ch_key: __u32 = 11;
    let mut real_num: __u32 = 3;
    let mut err: c_int;
    let mut i: c_int;
    let mut bytes: __u64 = 0;
    let mut pkts: __u64 = 0;
    let mut buf: [c_char; 128] = [0; 128];
    let magic: *mut u32 = buf.as_mut_ptr() as *mut u32;
    let mut topts: bpf_test_run_opts = bpf_test_run_opts {
        data_in: unsafe { core::ptr::addr_of!(pkt_v4) as *const c_void },
        data_size_in: unsafe { pkt_v4_size as u32 },
        data_out: buf.as_mut_ptr() as *mut c_void,
        data_size_out: core::mem::size_of_val(&buf) as u32,
        repeat: unsafe { NUM_ITER },
        retval: 0,
    };

    skel = unsafe { test_xdp_noinline__open_and_load() };
    if !unsafe { ASSERT_OK_PTR(skel as *mut c_void, c"skel_open_and_load".as_ptr()) } {
        return;
    }

    unsafe {
        bpf_map_update_elem(
            bpf_map__fd((*skel).maps.vip_map),
            &mut key as *mut vip as *const c_void,
            &mut value as *mut vip_meta as *const c_void,
            0,
        );
        bpf_map_update_elem(
            bpf_map__fd((*skel).maps.ch_rings),
            &mut ch_key as *mut __u32 as *const c_void,
            &mut real_num as *mut __u32 as *const c_void,
            0,
        );
        bpf_map_update_elem(
            bpf_map__fd((*skel).maps.reals),
            &mut real_num as *mut __u32 as *const c_void,
            &mut real_def as *mut real_definition as *const c_void,
            0,
        );
    }

    err = unsafe {
        bpf_prog_test_run_opts(
            bpf_program__fd((*skel).progs.balancer_ingress_v4),
            &mut topts,
        )
    };
    unsafe {
        ASSERT_OK(err, c"ipv4 test_run".as_ptr());
        ASSERT_EQ(topts.retval as __u64, 1, c"ipv4 test_run retval".as_ptr());
        ASSERT_EQ(
            topts.data_size_out as __u64,
            54,
            c"ipv4 test_run data_size_out".as_ptr(),
        );
        ASSERT_EQ((*magic) as __u64, MAGIC_VAL as __u64, c"ipv4 test_run magic".as_ptr());
    }

    topts.data_in = unsafe { core::ptr::addr_of!(pkt_v6) as *const c_void };
    topts.data_size_in = unsafe { pkt_v6_size as u32 };
    topts.data_out = buf.as_mut_ptr() as *mut c_void;
    topts.data_size_out = core::mem::size_of_val(&buf) as u32;

    err = unsafe {
        bpf_prog_test_run_opts(
            bpf_program__fd((*skel).progs.balancer_ingress_v6),
            &mut topts,
        )
    };
    unsafe {
        ASSERT_OK(err, c"ipv6 test_run".as_ptr());
        ASSERT_EQ(topts.retval as __u64, 1, c"ipv6 test_run retval".as_ptr());
        ASSERT_EQ(
            topts.data_size_out as __u64,
            74,
            c"ipv6 test_run data_size_out".as_ptr(),
        );
        ASSERT_EQ((*magic) as __u64, MAGIC_VAL as __u64, c"ipv6 test_run magic".as_ptr());
    }

    unsafe {
        bpf_map_lookup_elem(
            bpf_map__fd((*skel).maps.stats),
            &mut stats_key as *mut __u32 as *const c_void,
            stats.as_mut_ptr() as *mut c_void,
        );
    }
    i = 0;
    while i < nr_cpus as c_int {
        bytes = bytes.wrapping_add(stats[i as usize].bytes);
        pkts = pkts.wrapping_add(stats[i as usize].pkts);
        i += 1;
    }
    unsafe {
        ASSERT_EQ(
            bytes,
            MAGIC_BYTES.wrapping_mul(NUM_ITER as __u64).wrapping_mul(2),
            c"stats bytes".as_ptr(),
        );
        ASSERT_EQ(pkts, (NUM_ITER as __u64).wrapping_mul(2), c"stats pkts".as_ptr());
        test_xdp_noinline__destroy(skel);
    }
}
