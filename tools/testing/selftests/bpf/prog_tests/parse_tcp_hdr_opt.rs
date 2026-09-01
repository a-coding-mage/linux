// SPDX-License-Identifier: GPL-2.0

// C source dependencies:
// #define _GNU_SOURCE
// #include <test_progs.h>
// #include <network_helpers.h>
// #include "test_parse_tcp_hdr_opt.skel.h"
// #include "test_parse_tcp_hdr_opt_dynptr.skel.h"
// #include "test_tcp_hdr_options.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem::{size_of, size_of_val};

extern "C" {
    static mut ETH_P_IPV6: u16;
    static mut IPPROTO_TCP: u8;
    static mut MAGIC_BYTES: u16;
    static mut TCPOPT_MSS: u8;
    static mut TCPOPT_NOP: u8;
    static mut TCPOPT_EOL: u8;
    static mut XDP_PASS: u32;

    fn __bpf_constant_htons(hostshort: u16) -> u16;

    fn test_parse_tcp_hdr_opt__open_and_load() -> *mut test_parse_tcp_hdr_opt;
    fn test_parse_tcp_hdr_opt__destroy(skel: *mut test_parse_tcp_hdr_opt);
    fn test_parse_tcp_hdr_opt_dynptr__open_and_load() -> *mut test_parse_tcp_hdr_opt_dynptr;
    fn test_parse_tcp_hdr_opt_dynptr__destroy(skel: *mut test_parse_tcp_hdr_opt_dynptr);

    fn bpf_program__fd(prog: *mut bpf_program) -> c_int;
    fn bpf_prog_test_run_opts(fd: c_int, opts: *mut bpf_test_run_opts) -> c_int;

    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(err: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: u32, expected: u32, name: *const c_char) -> bool;
    fn test__start_subtest(name: *const c_char) -> bool;
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_parse_tcp_hdr_opt_rodata {
    pub tcp_hdr_opt_kind_tpr: u8,
}

#[repr(C)]
pub struct test_parse_tcp_hdr_opt_bss {
    pub server_id: u32,
}

#[repr(C)]
pub struct test_parse_tcp_hdr_opt_progs {
    pub xdp_ingress_v6: *mut bpf_program,
}

#[repr(C)]
pub struct test_parse_tcp_hdr_opt {
    pub rodata: *mut test_parse_tcp_hdr_opt_rodata,
    pub bss: *mut test_parse_tcp_hdr_opt_bss,
    pub progs: test_parse_tcp_hdr_opt_progs,
}

#[repr(C)]
pub struct test_parse_tcp_hdr_opt_dynptr_rodata {
    pub tcp_hdr_opt_kind_tpr: u8,
}

#[repr(C)]
pub struct test_parse_tcp_hdr_opt_dynptr_bss {
    pub server_id: u32,
}

#[repr(C)]
pub struct test_parse_tcp_hdr_opt_dynptr_progs {
    pub xdp_ingress_v6: *mut bpf_program,
}

#[repr(C)]
pub struct test_parse_tcp_hdr_opt_dynptr {
    pub rodata: *mut test_parse_tcp_hdr_opt_dynptr_rodata,
    pub bss: *mut test_parse_tcp_hdr_opt_dynptr_bss,
    pub progs: test_parse_tcp_hdr_opt_dynptr_progs,
}

#[repr(C)]
pub struct bpf_test_run_opts {
    pub data_in: *mut c_void,
    pub data_size_in: u32,
    pub data_out: *mut c_void,
    pub data_size_out: u32,
    pub repeat: u32,
    pub retval: u32,
}

#[repr(C)]
pub struct ethhdr {
    pub h_proto: u16,
}

#[repr(C)]
pub struct ipv6hdr {
    pub nexthdr: u8,
    pub payload_len: u16,
}

#[repr(C)]
pub struct tcphdr {
    pub urg_ptr: u16,
    pub doff: u8,
}

#[repr(C)]
pub struct ipv6_packet {
    pub eth: ethhdr,
    pub iph: ipv6hdr,
    pub tcp: tcphdr,
}

#[repr(C, packed)]
pub struct test_pkt {
    pub pk6_v6: ipv6_packet,
    pub options: [u8; 16],
}

static mut pkt: test_pkt = test_pkt {
    pk6_v6: ipv6_packet {
        eth: ethhdr { h_proto: 0 },
        iph: ipv6hdr {
            nexthdr: 0,
            payload_len: 0,
        },
        tcp: tcphdr {
            urg_ptr: 123,
            doff: 9, /* 16 bytes of options */
        },
    },
    options: [
        0, 4, 0x05, 0xB4, 0, 0, 0, 6, 0xBB, 0xBB, 0xBB, 0xBB, 0,
        0, 0, 0,
    ],
};

unsafe fn init_pkt_constants() {
    pkt.pk6_v6.eth.h_proto = __bpf_constant_htons(ETH_P_IPV6);
    pkt.pk6_v6.iph.nexthdr = IPPROTO_TCP;
    pkt.pk6_v6.iph.payload_len = __bpf_constant_htons(MAGIC_BYTES);
    pkt.options[0] = TCPOPT_MSS;
    pkt.options[4] = TCPOPT_NOP;
    pkt.options[5] = TCPOPT_NOP;
    pkt.options[12] = TCPOPT_EOL;
}

unsafe fn test_parse_opt() {
    let mut skel: *mut test_parse_tcp_hdr_opt;
    let mut prog: *mut bpf_program;
    let mut buf = [0 as c_char; 128];
    let mut err: c_int;

    let mut topts = bpf_test_run_opts {
        data_in: core::ptr::addr_of_mut!(pkt) as *mut c_void,
        data_size_in: size_of::<test_pkt>() as u32,
        data_out: buf.as_mut_ptr() as *mut c_void,
        data_size_out: size_of_val(&buf) as u32,
        repeat: 3,
        retval: 0,
    };

    skel = test_parse_tcp_hdr_opt__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    pkt.options[6] = (*(*skel).rodata).tcp_hdr_opt_kind_tpr;
    prog = (*skel).progs.xdp_ingress_v6;

    err = bpf_prog_test_run_opts(bpf_program__fd(prog), &mut topts);
    ASSERT_OK(err, b"ipv6 test_run\0".as_ptr() as *const c_char);
    ASSERT_EQ(topts.retval, XDP_PASS, b"ipv6 test_run retval\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).server_id, 0xBBBBBBBB, b"server id\0".as_ptr() as *const c_char);

    test_parse_tcp_hdr_opt__destroy(skel);
}

unsafe fn test_parse_opt_dynptr() {
    let mut skel: *mut test_parse_tcp_hdr_opt_dynptr;
    let mut prog: *mut bpf_program;
    let mut buf = [0 as c_char; 128];
    let mut err: c_int;

    let mut topts = bpf_test_run_opts {
        data_in: core::ptr::addr_of_mut!(pkt) as *mut c_void,
        data_size_in: size_of::<test_pkt>() as u32,
        data_out: buf.as_mut_ptr() as *mut c_void,
        data_size_out: size_of_val(&buf) as u32,
        repeat: 3,
        retval: 0,
    };

    skel = test_parse_tcp_hdr_opt_dynptr__open_and_load();
    if !ASSERT_OK_PTR(skel as *const c_void, b"skel_open_and_load\0".as_ptr() as *const c_char) {
        return;
    }

    pkt.options[6] = (*(*skel).rodata).tcp_hdr_opt_kind_tpr;
    prog = (*skel).progs.xdp_ingress_v6;

    err = bpf_prog_test_run_opts(bpf_program__fd(prog), &mut topts);
    ASSERT_OK(err, b"ipv6 test_run\0".as_ptr() as *const c_char);
    ASSERT_EQ(topts.retval, XDP_PASS, b"ipv6 test_run retval\0".as_ptr() as *const c_char);
    ASSERT_EQ((*(*skel).bss).server_id, 0xBBBBBBBB, b"server id\0".as_ptr() as *const c_char);

    test_parse_tcp_hdr_opt_dynptr__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_parse_tcp_hdr_opt() {
    init_pkt_constants();

    if test__start_subtest(b"parse_tcp_hdr_opt\0".as_ptr() as *const c_char) {
        test_parse_opt();
    }
    if test__start_subtest(b"parse_tcp_hdr_opt_dynptr\0".as_ptr() as *const c_char) {
        test_parse_opt_dynptr();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
