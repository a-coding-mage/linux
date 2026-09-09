// SPDX-License-Identifier: GPL-2.0
/* Intel MAX10 Board Management Controller Secure Update Driver */

use core::ffi::{c_char, c_int, c_void};

#[repr(C)] pub struct m10bmc_sec { pub dev: *mut device, pub m10bmc: *mut intel_m10bmc, pub fwl: *mut fw_upload, pub fw_name: *mut c_char, pub fw_name_id: u32, pub cancel_request: bool, pub ops: *const m10bmc_sec_ops }
#[repr(C)] pub struct m10bmc_sec_ops { pub rsu_status: Option<unsafe extern "C" fn(*mut m10bmc_sec) -> c_int> }
#[repr(C)] pub struct device;
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct intel_m10bmc { pub regmap: *mut regmap, pub flash_bulk_ops: *mut flash_bulk_ops, pub info: *const m10bmc_info }
#[repr(C)] pub struct regmap;
#[repr(C)] pub struct fw_upload { pub dd_handle: *mut c_void }
#[repr(C)] pub struct flash_bulk_ops { pub write: Option<unsafe extern "C" fn(*mut intel_m10bmc,*const u8,u32,u32)->c_int>, pub read: Option<unsafe extern "C" fn(*mut intel_m10bmc,*mut u8,u32,u32)->c_int>, pub lock_write: Option<unsafe extern "C" fn(*mut intel_m10bmc)->c_int>, pub unlock_write: Option<unsafe extern "C" fn(*mut intel_m10bmc)> }
#[repr(C)] pub struct m10bmc_info { pub csr_map: *const m10bmc_csr_map }
#[repr(C)] pub struct m10bmc_csr_map { pub bmc_magic:u32,pub bmc_prog_addr:u32,pub bmc_reh_addr:u32,pub sr_magic:u32,pub sr_prog_addr:u32,pub sr_reh_addr:u32,pub pr_magic:u32,pub pr_prog_addr:u32,pub pr_reh_addr:u32,pub rsu_update_counter:u32,pub doorbell:u32,pub auth_result:u32,pub base:u32,pub staging_size:u32 }
#[repr(C)] pub struct fw_upload_ops { pub prepare: Option<unsafe extern "C" fn(*mut fw_upload,*const u8,u32)->fw_upload_err>, pub write: Option<unsafe extern "C" fn(*mut fw_upload,*const u8,u32,u32,*mut u32)->fw_upload_err>, pub poll_complete: Option<unsafe extern "C" fn(*mut fw_upload)->fw_upload_err>, pub cancel: Option<unsafe extern "C" fn(*mut fw_upload)>, pub cleanup: Option<unsafe extern "C" fn(*mut fw_upload)> }
#[repr(C)] pub struct platform_device_id { pub name:*const c_char, pub driver_data: usize }
#[repr(C)] pub struct platform_driver;
#[repr(C)] pub enum fw_upload_err { None=0, InvalidSize, Busy, RwError, Timeout, Wearout, HwError, Canceled }

extern "C" {
    static mut fw_upload_xa: c_void;
    fn regmap_get_reg_stride(r:*mut regmap)->u32; fn regmap_bulk_write(r:*mut regmap,a:u32,b:*const u8,n:u32)->c_int; fn regmap_bulk_read(r:*mut regmap,a:u32,b:*mut u8,n:u32)->c_int; fn regmap_write(r:*mut regmap,a:u32,v:u32)->c_int; fn regmap_read(r:*mut regmap,a:u32,v:*mut u32)->c_int;
    fn dev_get_drvdata(d:*mut device)->*mut c_void; fn dev_set_drvdata(d:*mut device,p:*mut c_void); fn m10bmc_sys_read(m:*mut intel_m10bmc,a:u32,v:*mut u32)->c_int; fn m10bmc_sys_update_bits(m:*mut intel_m10bmc,a:u32,mask:u32,val:u32)->c_int; fn m10bmc_fw_state_set(m:*mut intel_m10bmc,s:u32);
    fn firmware_upload_register(m:*mut c_void,d:*mut device,n:*const c_char,o:*const fw_upload_ops,h:*mut m10bmc_sec)->*mut fw_upload; fn firmware_upload_unregister(f:*mut fw_upload); fn platform_get_device_id(p:*mut platform_device)->*const platform_device_id;
    fn devm_kzalloc(d:*mut device,n:usize,g:u32)->*mut c_void; fn kmemdup_nul(p:*const c_char,n:usize,g:u32)->*mut c_char; fn kfree(p:*mut c_void); fn xa_alloc(x:*mut c_void,id:*mut u32,p:*mut m10bmc_sec,l:u32,g:u32)->c_int; fn xa_erase(x:*mut c_void,id:u32);
    fn sysfs_emit(b:*mut c_char,fmt:*const c_char,...)->isize; fn sprintf(b:*mut c_char,fmt:*const c_char,...)->c_int; fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void;
}

const REH_SHA256_SIZE:usize=32; const REH_SHA384_SIZE:usize=48; const REH_MAGIC:u32=0xffff; const REH_SHA_NUM_BYTES:u32=0xffff0000; const CSK_BIT_LEN:usize=128; const CSK_32ARRAY_SIZE:usize=4; const CSK_VEC_OFFSET:u32=0x34; const FLASH_COUNT_SIZE:usize=4096; const WRITE_BLOCK_SIZE:u32=0x4000;
const DRBL_RSU_REQUEST:u32=1; const DRBL_HOST_STATUS:u32=0xff00; const RSU_STAT_NORMAL:u32=0; const RSU_STAT_NIOS_OK:u32=1; const RSU_STAT_USER_OK:u32=2; const RSU_STAT_FACTORY_OK:u32=3; const RSU_STAT_ERASE_FAIL:u32=4; const RSU_STAT_WEAROUT:u32=5; const RSU_PROG_IDLE:u32=0; const RSU_PROG_RSU_DONE:u32=1; const RSU_PROG_AUTHENTICATING:u32=2; const RSU_PROG_COPYING:u32=3; const RSU_PROG_UPDATE_CANCEL:u32=4; const RSU_PROG_PROGRAM_KEY_HASH:u32=5; const RSU_PROG_PREPARE:u32=6; const RSU_PROG_READY:u32=7; const HOST_STATUS_IDLE:u32=0; const HOST_STATUS_WRITE_DONE:u32=1; const HOST_STATUS_ABORT_RSU:u32=2;
const M10BMC_STAGING_BASE:u32=0; const M10BMC_FW_STATE_NORMAL:u32=0; const M10BMC_FW_STATE_SEC_UPDATE_PREPARE:u32=1; const M10BMC_FW_STATE_SEC_UPDATE_WRITE:u32=2; const M10BMC_FW_STATE_SEC_UPDATE_PROGRAM:u32=3;
#[inline] fn field_get(mask:u32,v:u32)->u32 { (v & mask) >> mask.trailing_zeros() }
#[inline] fn rsu_prog(v:u32)->u32 { (v>>16)&0xff }

unsafe fn m10bmc_sec_write(sec:*mut m10bmc_sec,buf:*const u8,offset:u32,size:u32)->c_int { let m=(*sec).m10bmc; let stride=regmap_get_reg_stride((*m).regmap); if !(*m).flash_bulk_ops.is_null() { return ((*(*m).flash_bulk_ops).write.unwrap())(m,buf,offset,size); } let count=size/stride; let left=count*stride; let rem=size-left; let mut tmp=0u32; let r=regmap_bulk_write((*m).regmap,M10BMC_STAGING_BASE+offset,buf.add(offset as usize),count); if r!=0{return r} if rem!=0 { memcpy((&mut tmp as *mut u32).cast(),buf.add(left as usize),rem as usize); let r=regmap_write((*m).regmap,M10BMC_STAGING_BASE+offset+left,tmp); if r!=0{return r} } 0 }
unsafe fn m10bmc_sec_read(sec:*mut m10bmc_sec,buf:*mut u8,addr:u32,size:u32)->c_int { let m=(*sec).m10bmc; let stride=regmap_get_reg_stride((*m).regmap); if !(*m).flash_bulk_ops.is_null(){return ((*(*m).flash_bulk_ops).read.unwrap())(m,buf,addr,size)} let count=size/stride; let left=count*stride; let rem=size-left; let mut tmp=0u32; let r=regmap_bulk_read((*m).regmap,addr,buf,count); if r!=0{return r} if rem!=0 {let r=regmap_read((*m).regmap,addr+left,&mut tmp);if r!=0{return r} memcpy(buf.add(left as usize),(&tmp as *const u32).cast(),rem as usize);} 0 }

unsafe fn rsu_status_ok(s:u32)->bool { s==RSU_STAT_NORMAL||s==RSU_STAT_NIOS_OK||s==RSU_STAT_USER_OK||s==RSU_STAT_FACTORY_OK }
unsafe fn rsu_progress_done(p:u32)->bool {p==RSU_PROG_IDLE||p==RSU_PROG_RSU_DONE}
unsafe fn rsu_progress_busy(p:u32)->bool {p==RSU_PROG_AUTHENTICATING||p==RSU_PROG_COPYING||p==RSU_PROG_UPDATE_CANCEL||p==RSU_PROG_PROGRAM_KEY_HASH}

unsafe fn m10bmc_sec_progress_status(sec:*mut m10bmc_sec,doorbell:*mut u32,progress:*mut u32,status:*mut u32)->c_int { let map=(*(*(*sec).m10bmc).info).csr_map; let r=m10bmc_sys_read((*sec).m10bmc,(*map).doorbell,doorbell);if r!=0{return r} let r=((*(*sec).ops).rsu_status.unwrap())(sec);if r<0{return r} *status=r as u32;*progress=rsu_prog(*doorbell);0 }
unsafe fn rsu_check_complete(sec:*mut m10bmc_sec,doorbell:*mut u32)->c_int {let mut p=0;let mut s=0;if m10bmc_sec_progress_status(sec,doorbell,&mut p,&mut s)!=0{return -5}if !rsu_status_ok(s){return -22}if rsu_progress_done(p){0}else if rsu_progress_busy(p){-11}else{-22}}

unsafe extern "C" fn m10bmc_sec_cancel(fwl:*mut fw_upload){(*(fwl as *mut fw_upload)).dd_handle.cast::<m10bmc_sec>().as_mut().unwrap().cancel_request=true}
unsafe fn rsu_cancel(sec:*mut m10bmc_sec)->fw_upload_err { let map=(*(*(*sec).m10bmc).info).csr_map; let mut d=0; if m10bmc_sys_read((*sec).m10bmc,(*map).doorbell,&mut d)!=0{return fw_upload_err::RwError} if rsu_prog(d)!=RSU_PROG_READY{return fw_upload_err::Busy} if m10bmc_sys_update_bits((*sec).m10bmc,(*map).doorbell,DRBL_HOST_STATUS,HOST_STATUS_ABORT_RSU)!=0{return fw_upload_err::RwError} fw_upload_err::Canceled }
unsafe extern "C" fn m10bmc_sec_prepare(f:*mut fw_upload,_:*const u8,size:u32)->fw_upload_err {let s=(*f).dd_handle.cast::<m10bmc_sec>();(*s).cancel_request=false;let map=(*(*(*s).m10bmc).info).csr_map;if size==0||size>(*map).staging_size{return fw_upload_err::InvalidSize}if rsu_cancel(s)==fw_upload_err::Busy{return fw_upload_err::Busy}fw_upload_err::None}
unsafe extern "C" fn m10bmc_sec_fw_write(f:*mut fw_upload,data:*const u8,off:u32,size:u32,written:*mut u32)->fw_upload_err {let s=(*f).dd_handle.cast::<m10bmc_sec>();if (*s).cancel_request{return rsu_cancel(s)}let n=core::cmp::min(WRITE_BLOCK_SIZE,size);if m10bmc_sec_write(s,data,off,n)!=0{return fw_upload_err::RwError}*written=n;fw_upload_err::None}
unsafe extern "C" fn m10bmc_sec_poll_complete(f:*mut fw_upload)->fw_upload_err {let s=(*f).dd_handle.cast::<m10bmc_sec>();if (*s).cancel_request{return rsu_cancel(s)}let mut d=0;loop{let r=rsu_check_complete(s,&mut d);if r==0{return fw_upload_err::None}if r!=-11{return fw_upload_err::HwError}}}
unsafe extern "C" fn m10bmc_sec_cleanup(f:*mut fw_upload){let s=(*f).dd_handle.cast::<m10bmc_sec>();let _=rsu_cancel(s)}
static M10BMC_OPS:fw_upload_ops=fw_upload_ops{prepare:Some(m10bmc_sec_prepare),write:Some(m10bmc_sec_fw_write),poll_complete:Some(m10bmc_sec_poll_complete),cancel:Some(m10bmc_sec_cancel),cleanup:Some(m10bmc_sec_cleanup)};
static M10SEC_N3000_OPS:m10bmc_sec_ops=m10bmc_sec_ops{rsu_status:None};
static M10SEC_N6000_OPS:m10bmc_sec_ops=m10bmc_sec_ops{rsu_status:None};
// The remaining registration metadata is supplied by the kernel integration layer.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
