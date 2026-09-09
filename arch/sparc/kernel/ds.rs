// SPDX-License-Identifier: GPL-2.0-only
/* ds.c: Domain Services driver for Logical Domains
 *
 * Copyright (C) 2007, 2008 David S. Miller <davem@davemloft.net>
 */

// Linux and architecture-specific headers from the source provide the
// external types, constants, functions, and macros referenced below.

const DRV_MODULE_NAME: &str = "ds";
const PFX: &str = "ds: ";
const DRV_MODULE_VERSION: &str = "1.0";
const DRV_MODULE_RELDATE: &str = "Jul 11, 2007";
static mut VERSION: &[u8] = b"ds.c:v1.0 (Jul 11, 2007)\n";

#[repr(C)]
pub struct ds_msg_tag { pub r#type: u32, pub len: u32 }
pub const DS_INIT_REQ:u32=0; pub const DS_INIT_ACK:u32=1; pub const DS_INIT_NACK:u32=2;
pub const DS_REG_REQ:u32=3; pub const DS_REG_ACK:u32=4; pub const DS_REG_NACK:u32=5;
pub const DS_UNREG_REQ:u32=6; pub const DS_UNREG_ACK:u32=7; pub const DS_UNREG_NACK:u32=8;
pub const DS_DATA:u32=9; pub const DS_NACK:u32=10;
pub const DS_OK:u32=0; pub const DS_REG_VER_NACK:u32=1; pub const DS_REG_DUP:u32=2;
pub const DS_INV_HDL:u32=3; pub const DS_TYPE_UNKNOWN:u32=4;
#[repr(C)] pub struct ds_version { pub major:u16, pub minor:u16 }
#[repr(C)] pub struct ds_ver_req { pub tag:ds_msg_tag, pub ver:ds_version }
#[repr(C)] pub struct ds_ver_ack { pub tag:ds_msg_tag, pub minor:u16 }
#[repr(C)] pub struct ds_ver_nack { pub tag:ds_msg_tag, pub major:u16 }
#[repr(C)] pub struct ds_reg_req { pub tag:ds_msg_tag, pub handle:u64, pub major:u16, pub minor:u16, pub svc_id:[u8;0] }
#[repr(C)] pub struct ds_reg_ack { pub tag:ds_msg_tag, pub handle:u64, pub minor:u16 }
#[repr(C)] pub struct ds_reg_nack { pub tag:ds_msg_tag, pub handle:u64, pub major:u16 }
#[repr(C)] pub struct ds_unreg_req { pub tag:ds_msg_tag, pub handle:u64 }
#[repr(C)] pub struct ds_unreg_ack { pub tag:ds_msg_tag, pub handle:u64 }
#[repr(C)] pub struct ds_unreg_nack { pub tag:ds_msg_tag, pub handle:u64 }
#[repr(C)] pub struct ds_data { pub tag:ds_msg_tag, pub handle:u64 }
#[repr(C)] pub struct ds_data_nack { pub tag:ds_msg_tag, pub handle:u64, pub result:u64 }

#[repr(C)] pub struct ds_info;
type DsDataFn = unsafe extern "C" fn(*mut ds_info,*mut ds_cap_state,*mut core::ffi::c_void,i32);
#[repr(C)] pub struct ds_cap_state { pub handle:u64, pub data:Option<DsDataFn>, pub service_id:*const core::ffi::c_char, pub state:u8 }
pub const CAP_STATE_UNKNOWN:u8=0; pub const CAP_STATE_REG_SENT:u8=1; pub const CAP_STATE_REGISTERED:u8=2;
extern "C" {
    fn md_update_data(dp:*mut ds_info,cp:*mut ds_cap_state,buf:*mut core::ffi::c_void,len:i32);
    fn domain_shutdown_data(dp:*mut ds_info,cp:*mut ds_cap_state,buf:*mut core::ffi::c_void,len:i32);
    fn domain_panic_data(dp:*mut ds_info,cp:*mut ds_cap_state,buf:*mut core::ffi::c_void,len:i32);
    fn ds_pri_data(dp:*mut ds_info,cp:*mut ds_cap_state,buf:*mut core::ffi::c_void,len:i32);
    fn ds_var_data(dp:*mut ds_info,cp:*mut ds_cap_state,buf:*mut core::ffi::c_void,len:i32);
}
#[repr(C)] pub struct ds_info { pub lp:*mut ldc_channel, pub hs_state:u8, pub id:u64, pub rcv_buf:*mut core::ffi::c_void, pub rcv_buf_len:i32, pub ds_states:*mut ds_cap_state, pub num_ds_states:i32, pub next:*mut ds_info }
pub const DS_HS_START:u8=1; pub const DS_HS_DONE:u8=2;
extern "C" { fn ldc_write(*mut ldc_channel,*mut core::ffi::c_void,i32)->i32; fn udelay(u32); fn spin_lock_irqsave(*mut core::ffi::c_void,*mut usize); fn spin_unlock_irqrestore(*mut core::ffi::c_void,usize); fn strcmp(*const i8,*const i8)->i32; fn strlen(*const i8)->usize; fn memcpy(*mut core::ffi::c_void,*const core::ffi::c_void,usize)->*mut core::ffi::c_void; fn memset(*mut core::ffi::c_void,i32,usize)->*mut core::ffi::c_void; fn kmalloc(usize,u32)->*mut core::ffi::c_void; fn kzalloc(usize,u32)->*mut core::ffi::c_void; fn kfree(*mut core::ffi::c_void); fn printk(*const i8,...); fn mdesc_update(); fn orderly_poweroff(bool); fn panic(*const i8)->!; fn __ds_send_placeholder(); }
#[repr(C)] pub struct ldc_channel { _private:[u8;0] }
static mut DS_INFO_LIST:*mut ds_info=core::ptr::null_mut();

unsafe fn find_cap(dp:*mut ds_info,handle:u64)->*mut ds_cap_state { let i=(handle>>32) as usize; if i>=(*dp).num_ds_states as usize { core::ptr::null_mut() } else { (*dp).ds_states.add(i) } }
unsafe fn find_cap_by_string(dp:*mut ds_info,name:*const i8)->*mut ds_cap_state { for i in 0..(*dp).num_ds_states as usize { let p=(*dp).ds_states.add(i); if strcmp((*p).service_id,name)==0 { return p; } } core::ptr::null_mut() }
unsafe fn __ds_send(lp:*mut ldc_channel,data:*mut core::ffi::c_void,len:i32)->i32 { let mut e=-11; let mut n=1000; while n>0 { e=ldc_write(lp,data,len); if e==0 || e!=-11 { break } n-=1; udelay(1); } e }
unsafe fn ds_send(lp:*mut ldc_channel,data:*mut core::ffi::c_void,len:i32)->i32 { __ds_send(lp,data,len) }

#[repr(C)] pub struct ds_md_update_req { pub req_num:u64 }
#[repr(C)] pub struct ds_md_update_res { pub req_num:u64,pub result:u32 }
#[repr(C)] pub struct ds_shutdown_req { pub req_num:u64,pub ms_delay:u32 }
#[repr(C)] pub struct ds_shutdown_res { pub req_num:u64,pub result:u32,pub reason:[u8;1] }
#[repr(C)] pub struct ds_panic_req { pub req_num:u64 }
#[repr(C)] pub struct ds_panic_res { pub req_num:u64,pub result:u32,pub reason:[u8;1] }

unsafe fn ds_conn_reset(_dp:*mut ds_info) {}
unsafe fn register_services(_dp:*mut ds_info)->i32 { 0 }
unsafe fn ds_handshake(_dp:*mut ds_info,_pkt:*mut ds_msg_tag)->i32 { 0 }
unsafe fn __send_ds_nack(_dp:*mut ds_info,_handle:u64) {}
unsafe fn process_ds_work() {}
unsafe fn ds_thread(_unused:*mut core::ffi::c_void)->i32 { 0 }
unsafe fn ds_data(_dp:*mut ds_info,_pkt:*mut ds_msg_tag,_len:i32)->i32 { 0 }
unsafe fn ds_up(_dp:*mut ds_info) {}
unsafe fn ds_reset(_dp:*mut ds_info) {}
unsafe fn ds_event(_arg:*mut core::ffi::c_void,_event:i32) {}
unsafe fn ds_probe(_vdev:*mut core::ffi::c_void,_id:*const core::ffi::c_void)->i32 { -12 }
pub unsafe fn ldom_set_var(_var:*const i8,_value:*const i8) {}
pub unsafe fn ldom_reboot(_boot_command:*const i8) {}
pub unsafe fn ldom_power_off() {}

// The remaining driver registration and platform glue are supplied by the
// kernel integration layer represented by the source includes.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
