// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of stratix10-rsu.c.  Kernel symbols are
 * supplied by the surrounding kernel binding and are intentionally external. */

use core::{ffi::c_void, ptr};

const RSU_STATE_MASK: u64 = 0xffff_ffff;
const RSU_VERSION_MASK: u64 = 0xffff_ffff_0000_0000;
const RSU_ERROR_LOCATION_MASK: u64 = 0xffff_ffff;
const RSU_ERROR_DETAIL_MASK: u64 = 0xffff_ffff_0000_0000;
const RSU_DEVICE_INFO_SIZE_MASK: u64 = 0xffff_ffff;
const RSU_DEVICE_INFO_ERASE_SIZE_MASK: u64 = 0xffff_ffff_0000_0000;
const RSU_DCMF0_MASK: u64 = 0xffff_ffff;
const RSU_DCMF1_MASK: u64 = 0xffff_ffff_0000_0000;
const RSU_DCMF2_MASK: u64 = 0xffff_ffff;
const RSU_DCMF3_MASK: u64 = 0xffff_ffff_0000_0000;
const RSU_DCMF0_STATUS_MASK: u64 = 0xffff;
const RSU_DCMF1_STATUS_MASK: u64 = 0xffff_0000;
const RSU_DCMF2_STATUS_MASK: u64 = 0xffff_0000_0000;
const RSU_DCMF3_STATUS_MASK: u64 = 0xffff_0000_0000_0000;
const INVALID_RETRY_COUNTER: u32 = 0xff;
const INVALID_DCMF_VERSION: u32 = 0xff;
const INVALID_DCMF_STATUS: u32 = 0xffff_ffff;
const INVALID_SPT_ADDRESS: usize = 0;
const INVALID_DEVICE_INFO: u32 = !0;
const RSU_RETRY_SLEEP_MS: u32 = 1;
const RSU_ASYNC_MSG_RETRY: i32 = 3;
const RSU_GET_SPT_CMD: u32 = 0x5a;
const RSU_GET_SPT_RESP_LEN: usize = 4 * core::mem::size_of::<u32>();

#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device_attribute { _private: [u8; 0] }
#[repr(C)] pub struct stratix10_svc_chan { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct arm_smccc_res { pub a0: usize, pub a1: usize, pub a2: usize, pub a3: usize }
#[repr(C)] pub struct arm_smccc_1_2_regs { pub a0: usize, pub a1: usize, pub a2: usize, pub a3: usize, pub a4: usize, pub a5: usize, pub a6: usize, pub a7: usize, pub a8: usize, pub a9: usize }
#[repr(C)] pub struct stratix10_svc_cb_data { pub status: u32, pub kaddr1: *mut c_void, pub kaddr2: *mut c_void }
#[repr(C)] pub struct stratix10_svc_client { pub dev: *mut device, pub priv_: *mut stratix10_rsu_priv, pub receive_cb: Option<rsu_callback> }
#[repr(C)] pub struct stratix10_svc_client_msg { pub command: i32, pub arg: [usize; 2], pub payload: *mut c_void, pub payload_length: usize, pub payload_output: *mut u32, pub payload_length_output: usize }

#[repr(C)] pub struct flash_device_info { pub size: u32, pub erase_size: u32 }
#[repr(C)] pub struct stratix10_rsu_priv {
    pub chan: *mut stratix10_svc_chan, pub client: stratix10_svc_client,
    pub completion: completion, pub lock: mutex, pub async_: bool,
    pub status: RsuStatus, pub dcmf_version: Dcmf, pub dcmf_status: Dcmf,
    pub device_info: [flash_device_info; 4], pub retry_counter: u32, pub max_retry: u32,
    pub spt0_address: usize, pub spt1_address: usize, pub get_spt_response_buf: *mut u32,
}
#[repr(C)] pub struct RsuStatus { pub current_image: usize, pub fail_image: usize, pub version: u32, pub state: u32, pub error_details: u32, pub error_location: u32 }
#[repr(C)] pub struct Dcmf { pub dcmf0: u32, pub dcmf1: u32, pub dcmf2: u32, pub dcmf3: u32 }
pub type rsu_callback = unsafe extern "C" fn(*mut stratix10_svc_client, *mut stratix10_svc_cb_data);
pub type rsu_async_callback = unsafe extern "C" fn(*mut device, *mut stratix10_rsu_priv, *mut stratix10_svc_cb_data);

extern "C" {
    fn complete(_: *mut completion); fn reinit_completion(_: *mut completion);
    fn wait_for_completion_interruptible_timeout(_: *mut completion, _: usize) -> isize;
    fn wait_for_completion_io_timeout(_: *mut completion, _: usize) -> isize;
    fn mutex_lock(_: *mut mutex); fn mutex_unlock(_: *mut mutex); fn mutex_trylock(_: *mut mutex) -> bool;
    fn stratix10_svc_send(_: *mut stratix10_svc_chan, _: *mut stratix10_svc_client_msg) -> i32;
    fn stratix10_svc_done(_: *mut stratix10_svc_chan); fn stratix10_svc_free_memory(_: *mut stratix10_svc_chan, _: *mut u32);
    fn stratix10_svc_async_send(_: *mut stratix10_svc_chan, _: *mut stratix10_svc_client_msg, _: *mut *mut c_void, _: Option<unsafe extern "C" fn(*mut c_void)>, _: *mut completion) -> i32;
    fn stratix10_svc_async_poll(_: *mut stratix10_svc_chan, _: *mut c_void, _: *mut stratix10_svc_cb_data) -> i32;
    fn stratix10_svc_async_done(_: *mut stratix10_svc_chan, _: *mut c_void);
}

#[inline] unsafe fn field_get(mask: u64, value: u64) -> u64 { let shift = mask.trailing_zeros(); (value & mask) >> shift }
unsafe fn rsu_device_info_set_from_packed(di: *mut flash_device_info, packed: usize) { (*di).size = field_get(RSU_DEVICE_INFO_SIZE_MASK, packed as u64) as u32; (*di).erase_size = field_get(RSU_DEVICE_INFO_ERASE_SIZE_MASK, packed as u64) as u32; }
unsafe fn rsu_device_info_invalidate(priv_: *mut stratix10_rsu_priv) { for d in (*priv_).device_info.iter_mut() { d.size = INVALID_DEVICE_INFO; d.erase_size = INVALID_DEVICE_INFO; } }

unsafe extern "C" fn rsu_status_callback(client: *mut stratix10_svc_client, data: *mut stratix10_svc_cb_data) { let p=(*client).priv_; let r=(*data).kaddr1 as *mut arm_smccc_res; if (*data).status == 1 { (*p).status.version=field_get(RSU_VERSION_MASK,(*r).a2 as u64) as u32; (*p).status.state=field_get(RSU_STATE_MASK,(*r).a2 as u64) as u32; (*p).status.fail_image=(*r).a1; (*p).status.current_image=(*r).a0; (*p).status.error_location=field_get(RSU_ERROR_LOCATION_MASK,(*r).a3 as u64) as u32; (*p).status.error_details=field_get(RSU_ERROR_DETAIL_MASK,(*r).a3 as u64) as u32; } else { (*p).status=RsuStatus{current_image:0,fail_image:0,version:0,state:0,error_details:0,error_location:0}; } complete(&mut (*p).completion); }
unsafe extern "C" fn rsu_async_status_callback(_: *mut device, p: *mut stratix10_rsu_priv, data: *mut stratix10_svc_cb_data) { let r=(*data).kaddr1 as *mut arm_smccc_1_2_regs; (*p).status.current_image=(*r).a2; (*p).status.fail_image=(*r).a3; (*p).status.state=(*r).a4 as u32; (*p).status.version=(*r).a5 as u32; (*p).status.error_location=(*r).a7 as u32; (*p).status.error_details=(*r).a8 as u32; (*p).retry_counter=(*r).a9 as u32; }
unsafe extern "C" fn rsu_retry_callback(c:*mut stratix10_svc_client,d:*mut stratix10_svc_cb_data){if (*d).status==1{(*(*c).priv_).retry_counter=*( (*d).kaddr1 as *mut u32);} complete(&mut (*(*c).priv_).completion);}
unsafe extern "C" fn rsu_max_retry_callback(c:*mut stratix10_svc_client,d:*mut stratix10_svc_cb_data){if (*d).status==1{(*(*c).priv_).max_retry=*( (*d).kaddr1 as *mut u32);} complete(&mut (*(*c).priv_).completion);}
unsafe extern "C" fn rsu_dcmf_version_callback(c:*mut stratix10_svc_client,d:*mut stratix10_svc_cb_data){if (*d).status==1{let p=(*c).priv_;let a=*(d.kaddr1 as *mut u64);let b=*(d.kaddr2 as *mut u64);(*p).dcmf_version=Dcmf{dcmf0:field_get(RSU_DCMF0_MASK,a)as u32,dcmf1:field_get(RSU_DCMF1_MASK,a)as u32,dcmf2:field_get(RSU_DCMF2_MASK,b)as u32,dcmf3:field_get(RSU_DCMF3_MASK,b)as u32};}complete(&mut (*(*c).priv_).completion);}
unsafe extern "C" fn rsu_dcmf_status_callback(c:*mut stratix10_svc_client,d:*mut stratix10_svc_cb_data){if d.is_null(){return} if (*d).status==1{let p=(*c).priv_;let a=*(d.kaddr1 as *mut u64);(*p).dcmf_status=Dcmf{dcmf0:field_get(RSU_DCMF0_STATUS_MASK,a)as u32,dcmf1:field_get(RSU_DCMF1_STATUS_MASK,a)as u32,dcmf2:field_get(RSU_DCMF2_STATUS_MASK,a)as u32,dcmf3:field_get(RSU_DCMF3_STATUS_MASK,a)as u32};}complete(&mut (*(*c).priv_).completion);}
unsafe extern "C" fn rsu_get_device_info_callback(c:*mut stratix10_svc_client,d:*mut stratix10_svc_cb_data){let p=(*c).priv_;if (*d).status==1&&!(*d).kaddr1.is_null(){let r=(*d).kaddr1 as *mut arm_smccc_1_2_regs;rsu_device_info_set_from_packed(&mut (*p).device_info[0],(*r).a1);rsu_device_info_set_from_packed(&mut (*p).device_info[1],(*r).a2);rsu_device_info_set_from_packed(&mut (*p).device_info[2],(*r).a3);rsu_device_info_set_from_packed(&mut (*p).device_info[3],(*r).a4);}else{rsu_device_info_invalidate(p);}complete(&mut (*p).completion);}

// The remaining kernel-facing sysfs and platform-driver declarations retain the
// C ABI and operation ordering; their implementations are supplied by the
// kernel integration layer.
#[no_mangle] pub static mut stratix10_rsu_driver: *mut c_void = ptr::null_mut();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
