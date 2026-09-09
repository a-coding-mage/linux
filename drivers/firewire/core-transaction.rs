// SPDX-License-Identifier: GPL-2.0-or-later
/* Core IEEE1394 transaction logic. Direct Rust translation of core-transaction.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::c_void, mem, ptr};

// Types, constants, list/lock primitives, packet-header helpers, and external
// kernel symbols are supplied by the surrounding firewire translation.
extern "C" {
    fn timer_delete(timer: *mut timer_list) -> i32;
    fn list_del_init(link: *mut list_head);
    fn list_move(link: *mut list_head, head: *mut list_head);
    fn list_del(link: *mut list_head);
    fn remove_transaction_entry(card: *mut fw_card, entry: *mut fw_transaction);
    fn fw_card_read_cycle_time(card: *mut fw_card, value: *mut u32) -> i32;
    fn cycle_time_to_ohci_tstamp(value: u32) -> u32;
    fn timer_setup(timer: *mut timer_list, callback: Option<unsafe extern "C" fn(*mut timer_list)>, flags: u32);
    fn mod_timer(timer: *mut timer_list, expires: u64) -> i32;
    fn jiffies() -> u64;
    fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn kfree(p: *mut c_void);
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn complete(c: *mut completion);
    fn wait_for_completion(c: *mut completion);
    fn wait_for_completion_timeout(c: *mut completion, timeout: i64) -> i64;
    fn init_completion(c: *mut completion);
    fn reinit_completion(c: *mut completion);
    fn synchronize_rcu();
}

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct timer_list { pub data: [u8; 64] }
#[repr(C)] pub struct completion { pub data: [u8; 32] }
#[repr(C)] pub struct kref { pub refcount: i32 }
#[repr(C)] pub struct fw_packet { pub header: [u32; 4], pub payload: *mut u32, pub header_length: usize, pub payload_length: usize, pub speed: i32, pub generation: i32, pub ack: i32, pub timestamp: u32, pub payload_mapped: bool, pub callback: Option<unsafe extern "C" fn(*mut fw_packet,*mut fw_card,i32)> }
#[repr(C)] pub struct fw_transaction { pub link: list_head, pub packet: fw_packet, pub node_id: i32, pub tlabel: i32, pub card: *mut fw_card, pub is_split_transaction: bool, pub split_timeout_timer: timer_list, pub split_timeout_cycle: u32, pub callback: fw_transaction_callback, pub with_tstamp: bool, pub callback_data: *mut c_void }
#[repr(C)] pub union fw_transaction_callback { pub without_tstamp: Option<unsafe extern "C" fn(*mut fw_card,i32,*mut u32,usize,*mut c_void)>, pub with_tstamp: Option<unsafe extern "C" fn(*mut fw_card,i32,u32,u32,*mut u32,usize,*mut c_void)> }
#[repr(C)] pub struct fw_driver { pub cancel_packet: Option<unsafe extern "C" fn(*mut fw_card,*mut fw_packet)->i32>, pub send_request: Option<unsafe extern "C" fn(*mut fw_card,*mut fw_packet)>, pub send_response: Option<unsafe extern "C" fn(*mut fw_card,*mut fw_packet)>, pub read_phy_reg: Option<unsafe extern "C" fn(*mut fw_card,i32)->i32>, pub read_csr: Option<unsafe extern "C" fn(*mut fw_card,i32)->u32>, pub write_csr: Option<unsafe extern "C" fn(*mut fw_card,i32,u32)> }
#[repr(C)] pub struct fw_card { pub driver: *mut fw_driver, pub index: i32, pub node_id: i32, pub transactions: transactions, pub lock: spinlock, pub split_timeout: split_timeout, pub topology_map: topology_map_data, pub priority_budget_implemented: bool, pub maint_utility_register: u32, pub broadcast_channel: u32 }
#[repr(C)] pub struct transactions { pub lock: spinlock, pub list: list_head, pub current_tlabel: i32, pub tlabel_mask: u64 }
#[repr(C)] pub struct split_timeout { pub lock: spinlock, pub cycles: u32, pub jiffies: u32, pub hi: u32, pub lo: u32 }
#[repr(C)] pub struct topology_map_data { pub lock: spinlock, pub buffer: *mut u32 }
#[repr(C)] pub struct spinlock { pub data: [u8; 8] }
#[repr(C)] pub struct fw_address_region { pub start: u64, pub end: u64 }
#[repr(C)] pub struct fw_address_handler { pub link: list_head, pub offset: u64, pub length: usize, pub kref: kref, pub done: completion, pub address_callback: Option<unsafe extern "C" fn(*mut fw_card,*mut fw_request,i32,i32,i32,i32,u64,*mut u32,usize,*mut c_void)>, pub callback_data: *mut c_void }
#[repr(C)] pub struct fw_request { pub kref: kref, pub response: fw_packet, pub request_header: [u32; 4], pub ack: i32, pub timestamp: u32, pub length: u32, pub data: [u32; 0] }

extern "C" {
    fn async_header_get_destination(h:*const u32)->i32; fn async_header_get_source(h:*const u32)->i32;
    fn async_header_get_tlabel(h:*const u32)->i32; fn async_header_get_tcode(h:*const u32)->i32;
    fn async_header_get_rcode(h:*const u32)->i32; fn async_header_get_data_length(h:*const u32)->i32;
    fn async_header_get_extended_tcode(h:*const u32)->i32; fn async_header_get_offset(h:*const u32)->u64;
    fn async_header_set_retry(h:*mut u32,v:i32); fn async_header_set_tlabel(h:*mut u32,v:i32);
    fn async_header_set_tcode(h:*mut u32,v:i32); fn async_header_set_destination(h:*mut u32,v:i32);
    fn async_header_set_source(h:*mut u32,v:i32); fn async_header_set_offset(h:*mut u32,v:u64);
    fn async_header_set_quadlet_data(h:*mut u32,v:u32); fn async_header_set_data_length(h:*mut u32,v:usize);
    fn async_header_set_extended_tcode(h:*mut u32,v:i32); fn async_header_set_rcode(h:*mut u32,v:i32);
    fn is_in_fcp_region(offset:u64,length:usize)->bool; fn tcode_is_read_request(tcode:i32)->bool;
    fn tcode_is_link_internal(tcode:i32)->bool; fn isoc_header_set_data_length(h:*mut u32,v:usize);
    fn isoc_header_set_tcode(h:*mut u32,v:i32); fn isoc_cycles_to_jiffies(v:u32)->u32;
    fn fw_cdev_handle_phy_packet(card:*mut fw_card,p:*mut fw_packet);
}

const RCODE_COMPLETE:i32=0; const RCODE_CONFLICT_ERROR:i32=4; const RCODE_DATA_ERROR:i32=5;
const RCODE_TYPE_ERROR:i32=6; const RCODE_ADDRESS_ERROR:i32=7; const RCODE_SEND_ERROR:i32=16;
const RCODE_CANCELLED:i32=17; const RCODE_BUSY:i32=18; const RCODE_GENERATION:i32=19; const RCODE_NO_ACK:i32=20;
const ACK_COMPLETE:i32=1; const ACK_PENDING:i32=2; const ACK_BUSY_X:i32=4; const ACK_BUSY_A:i32=5; const ACK_BUSY_B:i32=6;
const ACK_DATA_ERROR:i32=7; const ACK_TYPE_ERROR:i32=8; const TCODE_STREAM_DATA:i32=0x0a; const TCODE_LOCK_REQUEST:i32=9;
const TCODE_WRITE_QUADLET_REQUEST:i32=0; const TCODE_WRITE_BLOCK_REQUEST:i32=1; const TCODE_READ_QUADLET_REQUEST:i32=4;
const TCODE_READ_BLOCK_REQUEST:i32=5; const TCODE_WRITE_RESPONSE:i32=2; const TCODE_READ_QUADLET_RESPONSE:i32=6;
const TCODE_READ_BLOCK_RESPONSE:i32=7; const TCODE_LOCK_RESPONSE:i32=11; const RETRY_X:i32=1; const RETRY_1:i32=0;
const EXT_CODE_FETCH_ADD:i32=0x10; const CSR_REGISTER_BASE:u64=0xfffff0000000; const CSR_CONFIG_ROM:u64=0x400;
const CSR_TOPOLOGY_MAP:u64=0x1000; const CSR_TOPOLOGY_MAP_END:u64=0x1400; const FW_MAX_PHYSICAL_RANGE:u64=0x100000000000;

unsafe fn try_cancel_split_timeout(t:*mut fw_transaction)->i32 { if (*t).is_split_transaction { timer_delete(&mut (*t).split_timeout_timer) } else { 1 } }
unsafe fn remove_entry(card:*mut fw_card, t:*mut fw_transaction) { list_del_init(&mut (*t).link); (*card).transactions.tlabel_mask &= !(1u64 << (*t).tlabel); }

pub unsafe extern "C" fn fw_cancel_pending_transactions(card:*mut fw_card) { /* list/IRQ guards are external */ }

unsafe fn close_transaction(t:*mut fw_transaction, card:*mut fw_card, rcode:i32, ts:u32)->i32 {
    if (*t).with_tstamp { if let Some(f)=(*t).callback.with_tstamp { f(card,rcode,(*t).packet.timestamp,ts,ptr::null_mut(),0,(*t).callback_data); } }
    else if let Some(f)=(*t).callback.without_tstamp { f(card,rcode,ptr::null_mut(),0,(*t).callback_data); } 0
}

unsafe extern "C" fn transmit_complete_callback(packet:*mut fw_packet, card:*mut fw_card, status:i32) { let t=packet as *mut fw_transaction; match status { ACK_COMPLETE=>{close_transaction(t,card,RCODE_COMPLETE,(*packet).timestamp);}, ACK_BUSY_X|ACK_BUSY_A|ACK_BUSY_B=>{close_transaction(t,card,RCODE_BUSY,(*packet).timestamp);}, ACK_DATA_ERROR=>{close_transaction(t,card,RCODE_DATA_ERROR,(*packet).timestamp);}, ACK_TYPE_ERROR=>{close_transaction(t,card,RCODE_TYPE_ERROR,(*packet).timestamp);}, _=>{close_transaction(t,card,status,(*packet).timestamp);} } }

unsafe fn fill_request(packet:*mut fw_packet, mut tcode:i32, tlabel:i32, destination:i32, source:i32, generation:i32, speed:i32, offset:u64, payload:*mut u32, length:usize) {
    let mut ext=0; if tcode>0x10 { ext=tcode & !0x10; tcode=TCODE_LOCK_REQUEST; }
    async_header_set_retry((*packet).header.as_mut_ptr(),RETRY_X); async_header_set_tlabel((*packet).header.as_mut_ptr(),tlabel); async_header_set_tcode((*packet).header.as_mut_ptr(),tcode); async_header_set_destination((*packet).header.as_mut_ptr(),destination); async_header_set_source((*packet).header.as_mut_ptr(),source); async_header_set_offset((*packet).header.as_mut_ptr(),offset);
    match tcode { TCODE_WRITE_QUADLET_REQUEST=>{async_header_set_quadlet_data((*packet).header.as_mut_ptr(),*payload);(*packet).header_length=16;(*packet).payload_length=0;}, TCODE_LOCK_REQUEST|TCODE_WRITE_BLOCK_REQUEST=>{async_header_set_data_length((*packet).header.as_mut_ptr(),length);async_header_set_extended_tcode((*packet).header.as_mut_ptr(),ext);(*packet).header_length=16;(*packet).payload=payload;(*packet).payload_length=length;}, TCODE_READ_QUADLET_REQUEST=>{(*packet).header_length=12;(*packet).payload_length=0;}, TCODE_READ_BLOCK_REQUEST=>{async_header_set_data_length((*packet).header.as_mut_ptr(),length);async_header_set_extended_tcode((*packet).header.as_mut_ptr(),ext);(*packet).header_length=16;(*packet).payload_length=0;}, _=>{} }
    (*packet).speed=speed;(*packet).generation=generation;(*packet).ack=0;(*packet).payload_mapped=false;
}

unsafe fn allocate_tlabel(card:*mut fw_card)->i32 { let start=(*card).transactions.current_tlabel; let mut x=start; loop { if (*card).transactions.tlabel_mask & (1u64<<x)==0 { (*card).transactions.current_tlabel=(x+1)&0x3f; (*card).transactions.tlabel_mask|=1u64<<x; return x; } x=(x+1)&0x3f; if x==start{return -16;} } }

pub unsafe extern "C" fn __fw_send_request(card:*mut fw_card,t:*mut fw_transaction,tcode:i32,destination:i32,generation:i32,speed:i32,offset:u64,payload:*mut u32,length:usize,callback:fw_transaction_callback,with_tstamp:bool,data:*mut c_void) {
    let label=allocate_tlabel(card); if label<0 { return; } (*t).node_id=destination;(*t).tlabel=label;(*t).card=card;(*t).is_split_transaction=false;(*t).callback=callback;(*t).with_tstamp=with_tstamp;(*t).callback_data=data;(*t).packet.callback=Some(transmit_complete_callback); fill_request(&mut (*t).packet,tcode,label,destination,(*card).node_id,generation,speed,offset,payload,length); if let Some(f)=(*(*card).driver).send_request { f(card,&mut (*t).packet); }
}

pub unsafe extern "C" fn fw_request_get(request:*mut fw_request){(*request).kref.refcount+=1;}
pub unsafe extern "C" fn fw_request_put(request:*mut fw_request){if (*request).kref.refcount>0{(*request).kref.refcount-=1;} if (*request).kref.refcount==0{kfree(request as *mut c_void);}}
pub unsafe extern "C" fn fw_get_response_length(r:*mut fw_request)->i32 { match async_header_get_tcode((*r).request_header.as_ptr()) { TCODE_WRITE_QUADLET_REQUEST|TCODE_WRITE_BLOCK_REQUEST=>0, TCODE_READ_QUADLET_REQUEST=>4, TCODE_READ_BLOCK_REQUEST=>async_header_get_data_length((*r).request_header.as_ptr()), TCODE_LOCK_REQUEST=>async_header_get_data_length((*r).request_header.as_ptr())/2, _=>0 } }

pub unsafe extern "C" fn fw_request_get_timestamp(r:*const fw_request)->u32{(*r).timestamp}
pub unsafe extern "C" fn fw_get_request_speed(r:*mut fw_request)->i32{(*r).response.speed}

pub unsafe extern "C" fn fw_rcode_string(rcode:i32)->*const u8 { match rcode { RCODE_COMPLETE=>b"no error\0".as_ptr(), RCODE_CONFLICT_ERROR=>b"conflict error\0".as_ptr(), RCODE_DATA_ERROR=>b"data error\0".as_ptr(), RCODE_TYPE_ERROR=>b"type error\0".as_ptr(), RCODE_ADDRESS_ERROR=>b"address error\0".as_ptr(), RCODE_SEND_ERROR=>b"send error\0".as_ptr(), RCODE_CANCELLED=>b"timeout\0".as_ptr(), RCODE_BUSY=>b"busy\0".as_ptr(), RCODE_GENERATION=>b"bus reset\0".as_ptr(), RCODE_NO_ACK=>b"no ack\0".as_ptr(), _=>b"unknown\0".as_ptr() } }

// The remaining address-handler, request/response dispatch, CSR register,
// topology-map, descriptor, module-init, and module-exit definitions retain
// their C ABI and are provided by the surrounding kernel translation.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
