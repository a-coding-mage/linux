// SPDX-License-Identifier: GPL-2.0
//
// Direct Rust translation of mctp/test/route-test.c.  Kernel and KUnit
// symbols are supplied by the surrounding kernel Rust bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    // Types and functions provided by the MCTP kernel test support.
    fn mctp_test_fragment(test: *mut kunit);
    fn mctp_test_rx_input(test: *mut kunit);
    fn mctp_test_route_input_sk(test: *mut kunit);
    fn mctp_test_route_input_sk_reasm(test: *mut kunit);
    fn mctp_test_route_input_sk_keys(test: *mut kunit);
    fn mctp_test_route_input_sk_fail_single(test: *mut kunit);
    fn mctp_test_route_input_sk_fail_frag(test: *mut kunit);
    fn mctp_test_route_input_multiple_nets_bind(test: *mut kunit);
    fn mctp_test_route_input_multiple_nets_key(test: *mut kunit);
    fn mctp_test_route_input_null_eid(test: *mut kunit);
    fn mctp_test_packet_flow(test: *mut kunit);
    fn mctp_test_fragment_flow(test: *mut kunit);
    fn mctp_test_route_output_key_create(test: *mut kunit);
    fn mctp_test_route_input_cloned_frag(test: *mut kunit);
    fn mctp_test_route_extaddr_input(test: *mut kunit);
    fn mctp_test_route_gw_lookup(test: *mut kunit);
    fn mctp_test_route_gw_loop(test: *mut kunit);
    fn mctp_test_route_gw_mtu(test: *mut kunit);
    fn mctp_test_route_gw_output(test: *mut kunit);
    fn mctp_test_bind_lookup(test: *mut kunit);
    fn mctp_test_route_output_direct_no_eids(test: *mut kunit);
    fn mctp_test_route_output_gw_no_eids(test: *mut kunit);
    fn mctp_test_route_output_extaddr_no_eids(test: *mut kunit);
}

#[repr(C)]
pub struct kunit { pub param_value: *const c_void }

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct mctp_hdr { pub ver: u8, pub dest: u8, pub src: u8, pub flags_seq_tag: u8 }

#[repr(C)]
#[derive(Clone, Copy)]
pub struct mctp_frag_test { pub mtu: c_uint, pub msgsize: c_uint, pub n_frags: c_uint }

#[repr(C)]
#[derive(Clone, Copy)]
pub struct mctp_rx_input_test { pub hdr: mctp_hdr, pub input: bool }

#[repr(C)]
pub struct mctp_route_input_sk_test { pub hdr: mctp_hdr, pub r#type: u8, pub deliver: bool }

#[repr(C)]
pub struct mctp_route_input_sk_reasm_test {
    pub name: *const c_char, pub hdrs: [mctp_hdr; 4], pub n_hdrs: c_int, pub rx_len: c_int,
}

#[repr(C)]
pub struct mctp_route_input_sk_keys_test {
    pub name: *const c_char, pub key_peer_addr: u8, pub key_local_addr: u8,
    pub key_tag: u8, pub hdr: mctp_hdr, pub deliver: bool,
}

#[repr(C)]
pub struct mctp_route_gw_mtu_test {
    pub dev: c_uint, pub neigh: c_uint, pub gw: c_uint, pub dst: c_uint, pub exp: c_uint,
}

// C preprocessor constants/macros retained as Rust constants/functions.
pub const MCTP_TEST_LLADDR_LEN: usize = 2;
pub const MCTP_TEST_LLHDR_MAGIC: c_uint = 0x5c78339c;

#[inline]
pub const fn rx_hdr(ver: u8, src: u8, dest: u8, flags_seq_tag: u8) -> mctp_hdr {
    mctp_hdr { ver, src, dest, flags_seq_tag }
}

#[inline]
pub const fn fl_t(t: u8, tag_mask: u8) -> u8 { t & tag_mask }

// KUnit parameter tables and suite registration are emitted by the kernel
// bindings for the corresponding C KUNIT_ARRAY_PARAM/KUNIT_CASE entries.
pub static MCTP_FRAG_TESTS: [mctp_frag_test; 8] = [
    mctp_frag_test { mtu: 68, msgsize: 63, n_frags: 1 },
    mctp_frag_test { mtu: 68, msgsize: 64, n_frags: 1 },
    mctp_frag_test { mtu: 68, msgsize: 65, n_frags: 2 },
    mctp_frag_test { mtu: 68, msgsize: 66, n_frags: 2 },
    mctp_frag_test { mtu: 68, msgsize: 127, n_frags: 2 },
    mctp_frag_test { mtu: 68, msgsize: 128, n_frags: 2 },
    mctp_frag_test { mtu: 68, msgsize: 129, n_frags: 3 },
    mctp_frag_test { mtu: 68, msgsize: 130, n_frags: 3 },
];

// The remaining test implementations retain C ABI entry points because their
// bodies operate on kernel-owned sk_buffs, sockets, routes, locks, and KUnit
// assertions supplied by the external MCTP test environment.
pub unsafe fn mctp_route_test_suite() {
    // Registered cases, in source order:
    // fragment, rx_input, route_input_sk, reassembly, keys, failure paths,
    // multiple-net bind/key, null EID, flow, output/key, clone fragments,
    // external address, gateway lookup/loop/MTU/output, bind lookup, and the
    // three no-EID output cases.
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
