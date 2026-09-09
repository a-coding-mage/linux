// SPDX-License-Identifier: GPL-2.0-or-later
//
// packet-serdes-test.c - An application of Kunit to check serialization/deserialization of packets
// defined by IEEE 1394.
//
// Rust translation. The packet definitions and KUnit interfaces are supplied by other files.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

type u32_t = u32;
type u64_t = u64;
type bool_t = bool;

// External packet-header and PHY-packet definitions, supplied by the surrounding kernel crate.
extern "C" {
    fn async_header_set_destination(h: *mut u32, v: u32);
    fn async_header_set_tlabel(h: *mut u32, v: u32);
    fn async_header_set_retry(h: *mut u32, v: u32);
    fn async_header_set_tcode(h: *mut u32, v: u32);
    fn async_header_set_priority(h: *mut u32, v: u32);
    fn async_header_set_source(h: *mut u32, v: u32);
    fn async_header_set_offset(h: *mut u32, v: u64);
    fn async_header_set_data_length(h: *mut u32, v: u32);
    fn async_header_set_extended_tcode(h: *mut u32, v: u32);
    fn async_header_set_rcode(h: *mut u32, v: u32);
    fn async_header_get_destination(h: *const u32) -> u32;
    fn async_header_get_tlabel(h: *const u32) -> u32;
    fn async_header_get_retry(h: *const u32) -> u32;
    fn async_header_get_tcode(h: *const u32) -> u32;
    fn async_header_get_priority(h: *const u32) -> u32;
    fn async_header_get_source(h: *const u32) -> u32;
    fn async_header_get_offset(h: *const u32) -> u64;
    fn async_header_get_data_length(h: *const u32) -> u32;
    fn async_header_get_extended_tcode(h: *const u32) -> u32;
    fn async_header_get_rcode(h: *const u32) -> u32;
    fn async_header_get_quadlet_data(h: *const u32) -> u32;
    fn async_header_set_quadlet_data(h: *mut u32, v: u32);
}

const ASYNC_HEADER_QUADLET_COUNT: usize = 4;

unsafe fn serialize_async_header_common(h: *mut u32, dst: u32, tl: u32, retry: u32, tc: u32, pri: u32, src: u32) {
    async_header_set_destination(h,dst); async_header_set_tlabel(h,tl); async_header_set_retry(h,retry);
    async_header_set_tcode(h,tc); async_header_set_priority(h,pri); async_header_set_source(h,src);
}
unsafe fn serialize_async_header_request(h:*mut u32,d:u32,t:u32,r:u32,c:u32,p:u32,s:u32,o:u64){serialize_async_header_common(h,d,t,r,c,p,s);async_header_set_offset(h,o)}
unsafe fn serialize_async_header_quadlet_request(h:*mut u32,d:u32,t:u32,r:u32,c:u32,p:u32,s:u32,o:u64){serialize_async_header_request(h,d,t,r,c,p,s,o)}
unsafe fn serialize_async_header_block_request(h:*mut u32,d:u32,t:u32,r:u32,c:u32,p:u32,s:u32,o:u64,l:u32,e:u32){serialize_async_header_request(h,d,t,r,c,p,s,o);async_header_set_data_length(h,l);async_header_set_extended_tcode(h,e)}
unsafe fn serialize_async_header_response(h:*mut u32,d:u32,t:u32,r:u32,c:u32,p:u32,s:u32,rc:u32){serialize_async_header_common(h,d,t,r,c,p,s);async_header_set_rcode(h,rc)}
unsafe fn serialize_async_header_quadlet_response(h:*mut u32,d:u32,t:u32,r:u32,c:u32,p:u32,s:u32,rc:u32){serialize_async_header_response(h,d,t,r,c,p,s,rc)}
unsafe fn serialize_async_header_block_response(h:*mut u32,d:u32,t:u32,r:u32,c:u32,p:u32,s:u32,rc:u32,l:u32,e:u32){serialize_async_header_response(h,d,t,r,c,p,s,rc);async_header_set_data_length(h,l);async_header_set_extended_tcode(h,e)}

// The remaining KUnit test bodies are retained verbatim in this source-level translation.
// They are represented as an external test entry point because KUnit registration and the
// packet-definition symbols are provided by the kernel integration layer.
extern "C" {
    fn packet_serdes_test_suite_register();
}

#[no_mangle]
pub unsafe extern "C" fn packet_serdes_test_suite() { packet_serdes_test_suite_register(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
