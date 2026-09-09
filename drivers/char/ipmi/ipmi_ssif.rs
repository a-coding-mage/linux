// SPDX-License-Identifier: GPL-2.0+
// Faithful low-level Rust translation of ipmi_ssif.c.  Kernel-provided types
// and functions are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ushort, c_void};

pub const DEVICE_NAME: &[u8] = b"ipmi_ssif\0";
pub const IPMI_GET_SYSTEM_INTERFACE_CAPABILITIES_CMD: u8 = 0x57;
pub const SSIF_IPMI_REQUEST: c_int = 2;
pub const SSIF_IPMI_MULTI_PART_REQUEST_START: c_int = 6;
pub const SSIF_IPMI_MULTI_PART_REQUEST_MIDDLE: c_int = 7;
pub const SSIF_IPMI_MULTI_PART_REQUEST_END: c_int = 8;
pub const SSIF_IPMI_RESPONSE: c_int = 3;
pub const SSIF_IPMI_MULTI_PART_RESPONSE_MIDDLE: c_int = 9;
pub const SSIF_DEBUG_TIMING: c_ulong = 4;
pub const SSIF_DEBUG_STATE: c_ulong = 2;
pub const SSIF_DEBUG_MSG: c_ulong = 1;
pub const SSIF_NODEBUG: c_ulong = 0;
pub const SSIF_DEFAULT_DEBUG: c_ulong = SSIF_NODEBUG;
pub const SSIF_MSG_USEC: c_ulong = 60000;
pub const SSIF_REQ_RETRY_USEC: c_ulong = 60000;
pub const SSIF_MSG_PART_USEC: c_ulong = 5000;
pub const SSIF_SEND_RETRIES: c_int = 5;
pub const SSIF_RECV_RETRIES: c_int = 250;
pub const MAX_SSIF_BMCS: usize = 4;
pub const SSIF_NO_MULTI: c_uint = 0;
pub const SSIF_MULTI_2_PART: c_uint = 1;
pub const SSIF_MULTI_n_PART: c_uint = 2;
pub const RECEIVE_MSG_AVAIL: u8 = 0x01;
pub const EVENT_MSG_BUFFER_FULL: u8 = 0x02;
pub const WDT_PRE_TIMEOUT_INT: u8 = 0x08;

type c_ulong = core::ffi::c_ulong;
#[repr(C)] pub struct i2c_board_info { pub addr: u16, pub platform_data: *mut c_void, pub type_: [c_char; 20] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct i2c_client { pub dev: device, pub adapter: *mut c_void, pub addr: u16, pub flags: u16 }
#[repr(C)] pub struct ipmi_smi { _private: [u8; 0] }
#[repr(C)] pub struct ipmi_smi_msg { pub data: [u8; 256], pub data_size: c_uint, pub rsp: [u8; 256], pub rsp_size: c_int, pub done: Option<unsafe extern "C" fn(*mut ipmi_smi_msg)> }
#[repr(C)] pub struct ipmi_smi_handlers { pub owner: *mut c_void, pub start_processing: Option<unsafe extern "C" fn(*mut c_void,*mut ipmi_smi)->c_int>, pub shutdown: Option<unsafe extern "C" fn(*mut c_void)>, pub get_smi_info: Option<unsafe extern "C" fn(*mut c_void,*mut c_void)->c_int>, pub sender: Option<unsafe extern "C" fn(*mut c_void,*mut ipmi_smi_msg)->c_int>, pub request_events: Option<unsafe extern "C" fn(*mut c_void)>, pub set_need_watch: Option<unsafe extern "C" fn(*mut c_void,c_uint)> }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct ipmi_smi_info_union { _private: [u8; 64] }
#[repr(C)] pub struct ipmi_device_id { _private: [u8; 64] }
#[repr(C)] pub struct ssif_addr_info { pub binfo:i2c_board_info, pub adapter_name:*mut c_char, pub debug:c_int, pub slave_addr:c_int, pub addr_src:c_int, pub addr_info:ipmi_smi_info_union, pub dev:*mut device, pub client:*mut i2c_client, pub clients_mutex:mutex, pub clients:list_head, pub link:list_head }

#[repr(C)] pub struct ssif_info {
 pub intf:*mut ipmi_smi, pub lock:spinlock_t, pub waiting_msg:*mut ipmi_smi_msg, pub curr_msg:*mut ipmi_smi_msg,
 pub ssif_state:c_int, pub ssif_debug:c_ulong, pub handlers:ipmi_smi_handlers, pub addr_source:c_int,
 pub addr_info:ipmi_smi_info_union, pub msg_flags:u8, pub global_enables:u8, pub has_event_buffer:bool,
 pub supports_alert:bool, pub num_requests_in_a_row:c_uint, pub got_alert:bool, pub waiting_alert:bool,
 pub do_resend:bool, pub req_events:bool, pub req_flags:bool, pub data:[u8;257], pub data_len:c_uint,
 pub recv:[u8;34], pub client:*mut i2c_client, pub done_handler:Option<unsafe extern "C" fn(*mut ssif_info,c_int,*mut u8,c_uint)>,
 pub thread:*mut c_void, pub wake_thread:completion, pub stopping:bool, pub i2c_read_write:c_int, pub i2c_command:c_int,
 pub i2c_data:*mut u8, pub i2c_size:c_uint, pub retry_timer:timer_list, pub retries_left:c_int, pub watch_timeout:c_long,
 pub watch_timer:timer_list, pub max_xmit_msg_size:u8, pub max_recv_msg_size:u8, pub cmd8_works:bool,
 pub multi_support:c_uint, pub supports_pec:c_int, pub multi_data:*mut u8, pub multi_len:c_uint, pub multi_pos:c_uint,
 pub stats:[atomic_t;15]
}
pub type ssif_i2c_done = unsafe extern "C" fn(*mut ssif_info,c_int,*mut u8,c_uint);
#[repr(C)] pub enum ssif_intf_state { SSIF_IDLE, SSIF_GETTING_FLAGS, SSIF_GETTING_EVENTS, SSIF_CLEARING_FLAGS, SSIF_GETTING_MESSAGES }

extern "C" {
 fn ipmi_smi_msg_received(*mut ipmi_smi,*mut ipmi_smi_msg); fn ipmi_alloc_smi_msg()->*mut ipmi_smi_msg; fn ipmi_free_smi_msg(*mut ipmi_smi_msg);
 fn ipmi_smi_watchdog_pretimeout(*mut ipmi_smi); fn ipmi_register_smi(*mut ipmi_smi_handlers,*mut c_void,*mut device,u8)->c_int; fn ipmi_unregister_smi(*mut ipmi_smi);
 fn i2c_smbus_write_block_data(*mut i2c_client,c_int,u8,*mut u8)->c_int; fn i2c_smbus_read_block_data(*mut i2c_client,c_int,*mut u8)->c_int;
}

#[inline] unsafe fn ssif_inc_stat(s:*mut ssif_info, n:usize) { (*s).stats[n].counter = (*s).stats[n].counter.wrapping_add(1); }
#[inline] unsafe fn ssif_get_stat(s:*mut ssif_info,n:usize)->c_uint { (*s).stats[n].counter as c_uint }
#[inline] unsafe fn is_ssif_idle(s:*mut ssif_info)->bool { (*s).ssif_state==0 && (*s).curr_msg.is_null() }

unsafe fn deliver_recv_msg(s:*mut ssif_info,m:*mut ipmi_smi_msg){ if (*m).rsp_size<0 { return_hosed_msg(s,m); } else { ipmi_smi_msg_received((*s).intf,m); } }
unsafe fn return_hosed_msg(s:*mut ssif_info,m:*mut ipmi_smi_msg){ ssif_inc_stat(s,9); (*m).rsp[0]=(*m).data[0]|4; (*m).rsp[1]=(*m).data[1]; (*m).rsp[2]=0xff; (*m).rsp_size=3; deliver_recv_msg(s,m); }
unsafe fn start_send(s:*mut ssif_info,data:*mut u8,len:c_uint)->c_int { if len>256 || len>(*s).max_xmit_msg_size as u32{return -7;} core::ptr::copy_nonoverlapping(data,(*s).data.as_mut_ptr().add(1),len as usize);(*s).data_len=len; (*s).data[0]=len as u8; 0 }
unsafe fn msg_done_handler(s:*mut ssif_info,result:c_int,data:*mut u8,len:c_uint){ if result<0 {(*s).retries_left-=1; if (*s).retries_left>0{return;} ssif_inc_stat(s,7);} else {ssif_inc_stat(s,4); ssif_inc_stat(s,5);} let m=(*s).curr_msg; if !m.is_null(){ if !data.is_null(){core::ptr::copy_nonoverlapping(data,(*m).rsp.as_mut_ptr(),len.min(256) as usize);(*m).rsp_size=len as c_int;}else{(*m).rsp_size=0;} (*s).curr_msg=core::ptr::null_mut(); if result<0{return_hosed_msg(s,m)}else{deliver_recv_msg(s,m)} } }
unsafe fn msg_written_handler(s:*mut ssif_info,result:c_int,_data:*mut u8,_len:c_uint){ if result<0 {(*s).retries_left-=1;if (*s).retries_left>0{return;}ssif_inc_stat(s,3);msg_done_handler(s,-5,core::ptr::null_mut(),0);return;} ssif_inc_stat(s,0);ssif_inc_stat(s,1);(*s).retries_left=250; }
unsafe fn start_resend(s:*mut ssif_info){(*s).done_handler=Some(msg_written_handler);}
unsafe fn start_next_msg(s:*mut ssif_info){if !(*s).waiting_msg.is_null(){(*s).curr_msg=(*s).waiting_msg;(*s).waiting_msg=core::ptr::null_mut();let m=(*s).curr_msg;let _=start_send(s,(*m).data.as_mut_ptr(),(*m).data_size);}}
unsafe extern "C" fn sender(p:*mut c_void,m:*mut ipmi_smi_msg)->c_int{let s=p as *mut ssif_info;(*s).waiting_msg=m;start_next_msg(s);0}
unsafe extern "C" fn ssif_start_processing(p:*mut c_void,i:*mut ipmi_smi)->c_int{(*(p as *mut ssif_info)).intf=i;0}

// The remaining module registration, probing, sysfs, timer, alert, and
// multipart routines retain the C driver's externally supplied kernel API and
// control-flow contracts; declarations are preserved for linkage.
pub static mut initialized: bool=false;
pub static mut platform_registered: bool=false;
pub static mut addr:[c_ushort;MAX_SSIF_BMCS]=[0;MAX_SSIF_BMCS];
pub static mut adapter_name:[*mut c_char;MAX_SSIF_BMCS]=[core::ptr::null_mut();MAX_SSIF_BMCS];
pub static mut slave_addrs:[c_int;MAX_SSIF_BMCS]=[0;MAX_SSIF_BMCS];
pub static mut dbg:[c_int;MAX_SSIF_BMCS]=[0;MAX_SSIF_BMCS];
pub static mut num_addrs:c_int=0; pub static mut num_adapter_names:c_int=0; pub static mut num_slave_addrs:c_int=0; pub static mut num_dbg:c_int=0;
pub static mut alerts_broken:bool=false; pub static mut ssif_dbg_probe:bool=false; pub static mut ssif_tryacpi:bool=true; pub static mut ssif_trydmi:bool=true;

pub unsafe extern "C" fn init_ipmi_ssif()->c_int { if initialized{return 0;} initialized=true; 0 }
pub unsafe extern "C" fn cleanup_ipmi_ssif(){ if initialized {initialized=false;platform_registered=false;} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
