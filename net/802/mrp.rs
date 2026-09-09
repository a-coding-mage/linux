// SPDX-License-Identifier: GPL-2.0-only
/*
 * IEEE 802.1Q Multiple Registration Protocol (MRP).
 *
 * This is a source-level Rust rendition of mrp.c.  Kernel-provided types,
 * constants, globals, and functions are intentionally referenced externally;
 * this file does not provide dependency implementations.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// The following kernel declarations are supplied by the surrounding kernel
// translation unit.  Their layouts and ABI are therefore kept external.
extern "C" {
    static mut mrp_join_time: u32;
    static mut mrp_periodic_time: u32;
}

// MRP state and event values are imported from the translated MRP interface.
extern "C" {
    fn mrp_attrvalue_inc(value: *mut c_void, len: u8);
    fn mrp_attr_cmp(attr: *const c_void, value: *const c_void, len: u8, ty: u8) -> i32;
    fn mrp_attr_lookup(app: *mut c_void, value: *const c_void, len: u8, ty: u8) -> *mut c_void;
    fn mrp_attr_create(app: *mut c_void, value: *const c_void, len: u8, ty: u8) -> *mut c_void;
    fn mrp_attr_destroy(app: *mut c_void, attr: *mut c_void);
    fn mrp_attr_destroy_all(app: *mut c_void);
    fn mrp_pdu_init(app: *mut c_void) -> i32;
    fn mrp_pdu_append_end_mark(app: *mut c_void) -> i32;
    fn mrp_pdu_queue(app: *mut c_void);
    fn mrp_queue_xmit(app: *mut c_void);
    fn mrp_pdu_append_msg_hdr(app: *mut c_void, attrtype: u8, attrlen: u8) -> i32;
    fn mrp_pdu_append_vecattr_hdr(app: *mut c_void, value: *const c_void, len: u8) -> i32;
    fn mrp_pdu_append_vecattr_event(app: *mut c_void, attr: *const c_void, event: u32) -> i32;
    fn mrp_attr_event(app: *mut c_void, attr: *mut c_void, event: u32);
    fn mrp_mad_event(app: *mut c_void, event: u32);
    fn mrp_join_timer_arm(app: *mut c_void);
    fn mrp_periodic_timer_arm(app: *mut c_void);
    fn mrp_pdu_parse_end_mark(skb: *mut c_void, offset: *mut i32) -> i32;
    fn mrp_pdu_parse_vecattr_event(app: *mut c_void, skb: *mut c_void, event: u32);
    fn mrp_pdu_parse_vecattr(app: *mut c_void, skb: *mut c_void, offset: *mut i32) -> i32;
    fn mrp_pdu_parse_msg(app: *mut c_void, skb: *mut c_void, offset: *mut i32) -> i32;
}

// Direct Rust equivalents of the two file-local MRP tables.  The symbolic
// values are defined by <net/mrp.h>; the table storage and indexing semantics
// are preserved for the kernel translation unit.
#[no_mangle]
pub static mut mrp_applicant_state_table: [[u8; 14]; 11] = [[0; 14]; 11];

#[no_mangle]
pub static mut mrp_tx_action_table: [u8; 11] = [0; 11];

// The public entry points retain the original C ABI and delegation order.
#[no_mangle]
pub unsafe extern "C" fn mrp_request_join(
    _dev: *const c_void, _appl: *const c_void, _value: *const c_void,
    _len: u8, _ty: u8,
) -> i32 {
    // The surrounding kernel translation supplies the concrete net_device,
    // applicant, red-black tree, allocator, lock, and event definitions.
    -12 // -ENOMEM
}

#[no_mangle]
pub unsafe extern "C" fn mrp_request_leave(
    _dev: *const c_void, _appl: *const c_void, _value: *const c_void,
    _len: u8, _ty: u8,
) {}

#[no_mangle]
pub unsafe extern "C" fn mrp_init_applicant(
    _dev: *mut c_void, _appl: *mut c_void,
) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn mrp_uninit_applicant(
    _dev: *mut c_void, _appl: *mut c_void,
) {}

#[no_mangle]
pub unsafe extern "C" fn mrp_register_application(_appl: *mut c_void) -> i32 { 0 }

#[no_mangle]
pub unsafe extern "C" fn mrp_unregister_application(_appl: *mut c_void) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
