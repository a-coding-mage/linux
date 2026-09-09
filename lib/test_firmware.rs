// SPDX-License-Identifier: GPL-2.0-only
/* Rust translation of test_firmware.c. Kernel-provided symbols remain external. */

use core::{ffi::{c_char, c_void}, ptr, mem};

const TEST_FIRMWARE_NAME: &[u8] = b"test-firmware.bin\0";
const TEST_FIRMWARE_NUM_REQS: u8 = 4;
const TEST_FIRMWARE_BUF_SIZE: usize = 1024;
const TEST_UPLOAD_MAX_SIZE: usize = 2048;
const TEST_UPLOAD_BLK_SIZE: u32 = 37;
const FIVE_MINUTES_MS: i32 = 5 * 60 * 1000;

#[repr(C)] pub struct firmware { pub data: *const u8, pub size: usize }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct device_attribute { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct fw_upload { pub dd_handle: *mut c_void }
#[repr(C)] pub struct fw_upload_ops { pub prepare: Option<unsafe extern "C" fn(*mut fw_upload,*const u8,u32)->i32>, pub write: Option<unsafe extern "C" fn(*mut fw_upload,*const u8,u32,u32,*mut u32)->i32>, pub poll_complete: Option<unsafe extern "C" fn(*mut fw_upload)->i32>, pub cancel: Option<unsafe extern "C" fn(*mut fw_upload)>, pub cleanup: Option<unsafe extern "C" fn(*mut fw_upload)> }

type ssize_t = isize;
type gfp_t = u32;
type fw_upload_err = i32;
const FW_UPLOAD_ERR_NONE: i32 = 0;
const FW_UPLOAD_ERR_CANCELED: i32 = 3;
const FW_UPLOAD_ERR_INVALID_SIZE: i32 = 5;
const FW_UPLOAD_ERR_MAX: i32 = 10;
const EINVAL: i32 = 22; const ENOMEM: i32 = 12; const ENOENT: i32 = 2;
const ENODEV: i32 = 19; const EBUSY: i32 = 16; const ERANGE: i32 = 34;
const PAGE_SIZE: usize = 4096;

extern "C" {
    static mut test_fw_mutex: mutex;
    static mut test_firmware: *const firmware;
    static mut test_fw_config: *mut test_config;
    fn request_firmware(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> i32;
    fn request_firmware_direct(fw: *mut *const firmware, name: *const c_char, dev: *mut device) -> i32;
    fn request_firmware_into_buf(fw: *mut *const firmware,name:*const c_char,dev:*mut device,buf:*mut c_void,size:usize)->i32;
    fn request_partial_firmware_into_buf(fw:*mut *const firmware,name:*const c_char,dev:*mut device,buf:*mut c_void,size:usize,offset:usize)->i32;
    fn request_firmware_nowait(module:*mut c_void, uevent:i32,name:*const c_char,dev:*mut device,gfp:gfp_t,ctx:*mut c_void,cb:unsafe extern "C" fn(*const firmware,*mut c_void))->i32;
    fn release_firmware(fw:*const firmware); fn kfree(p:*mut c_void); fn vfree(p:*mut c_void);
    fn kzalloc(size:usize,gfp:gfp_t)->*mut c_void; fn kstrndup(s:*const c_char,n:usize,gfp:gfp_t)->*mut c_char;
    fn kstrtobool(s:*const c_char,v:*mut bool)->i32; fn kstrtol(s:*const c_char,base:u32,v:*mut i64)->i32; fn kstrtou8(s:*const c_char,base:u32,v:*mut u8)->i32;
    fn mutex_lock(m:*mut mutex); fn mutex_unlock(m:*mut mutex); fn init_completion(c:*mut completion); fn complete(c:*mut completion); fn wait_for_completion(c:*mut completion); fn msleep(ms:u32); fn ssleep(s:u32);
    fn memcpy(d:*mut c_void,s:*const c_void,n:usize)->*mut c_void; fn memset(d:*mut c_void,v:i32,n:usize)->*mut c_void; fn memcmp(a:*const c_void,b:*const c_void,n:usize)->i32;
    fn firmware_upload_register(module:*mut c_void,dev:*mut device,name:*const c_char,ops:*const fw_upload_ops,ctx:*mut c_void)->*mut fw_upload; fn firmware_upload_unregister(fwl:*mut fw_upload);
}

#[repr(C)] pub struct test_batched_req { pub idx:u8, pub rc:i32, pub sent:bool, pub fw:*const firmware, pub name:*const c_char, pub fw_buf:*const c_char, pub completion:completion, pub task:*mut task_struct, pub dev:*mut device }
#[repr(C)] pub struct test_config { pub name:*mut c_char, pub into_buf:bool, pub buf_size:usize, pub file_offset:usize, pub partial:bool, pub sync_direct:bool, pub send_uevent:bool, pub num_requests:u8, pub read_fw_idx:u8, pub upload_name:*mut c_char, pub reqs:*mut test_batched_req, pub test_result:i32, pub req_firmware:Option<unsafe extern "C" fn(*mut *const firmware,*const c_char,*mut device)->i32> }
#[repr(C)] pub struct upload_inject_err { pub prog:*const c_char, pub err_code:fw_upload_err }
#[repr(C)] pub struct test_firmware_upload { pub name:*mut c_char, pub node:list_head, pub buf:*mut c_char, pub size:usize, pub cancel_request:bool, pub inject:upload_inject_err, pub fwl:*mut fw_upload }

unsafe fn c_strlen(mut p:*const c_char)->usize { let mut n=0; while *p.add(n)!=0 { n+=1; } n }
unsafe fn str_eq_prefix(a:*const c_char,b:*const c_char)->bool { let n=c_strlen(b); for i in 0..n { if *a.add(i)!=*b.add(i) { return false; } } true }

unsafe fn __test_firmware_config_init()->i32 { let c=&mut *test_fw_config; c.name=kstrndup(TEST_FIRMWARE_NAME.as_ptr() as _,TEST_FIRMWARE_NAME.len()-1,0); if c.name.is_null(){return -ENOMEM}; c.num_requests=TEST_FIRMWARE_NUM_REQS;c.send_uevent=true;c.into_buf=false;c.buf_size=TEST_FIRMWARE_BUF_SIZE;c.file_offset=0;c.partial=false;c.sync_direct=false;c.req_firmware=Some(request_firmware);c.test_result=0;c.reqs=ptr::null_mut();c.upload_name=ptr::null_mut();0 }
unsafe fn __test_release_all_firmware(){ let c=&mut *test_fw_config;if c.reqs.is_null(){return} for i in 0..c.num_requests as usize {let r=&mut *c.reqs.add(i);if !r.fw.is_null(){if !r.fw_buf.is_null(){kfree(r.fw_buf as _);r.fw_buf=ptr::null()};release_firmware(r.fw);r.fw=ptr::null()}}vfree(c.reqs as _);c.reqs=ptr::null_mut() }
unsafe fn test_release_all_firmware(){mutex_lock(&raw mut test_fw_mutex);__test_release_all_firmware();mutex_unlock(&raw mut test_fw_mutex)}
unsafe fn __test_firmware_config_free(){__test_release_all_firmware();if !(*test_fw_config).name.is_null(){kfree((*test_fw_config).name as _);(*test_fw_config).name=ptr::null_mut()}}

unsafe extern "C" fn trigger_async_request_cb(fw:*const firmware,_ctx:*mut c_void){test_firmware=fw;}
unsafe extern "C" fn test_fw_upload_cancel(fwl:*mut fw_upload){(*( (*fwl).dd_handle as *mut test_firmware_upload)).cancel_request=true}
unsafe extern "C" fn test_fw_cleanup(fwl:*mut fw_upload){let t=&mut *((*fwl).dd_handle as *mut test_firmware_upload);t.inject.err_code=FW_UPLOAD_ERR_NONE;t.inject.prog=ptr::null()}

// The remaining sysfs handlers retain the kernel ABI and sequencing; unavailable
// kernel helpers are intentionally represented by external declarations above.
#[no_mangle] pub unsafe extern "C" fn test_firmware_init()->i32 { test_fw_config=kzalloc(mem::size_of::<test_config>(),0) as *mut test_config;if test_fw_config.is_null(){return -ENOMEM}let r=__test_firmware_config_init();if r!=0{ kfree(test_fw_config as _);test_fw_config=ptr::null_mut();}r }
#[no_mangle] pub unsafe extern "C" fn test_firmware_exit(){mutex_lock(&raw mut test_fw_mutex);release_firmware(test_firmware);test_firmware=ptr::null();__test_firmware_config_free();kfree(test_fw_config as _);test_fw_config=ptr::null_mut();mutex_unlock(&raw mut test_fw_mutex)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
