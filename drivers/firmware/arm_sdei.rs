// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2017 Arm Ltd.

// C headers and kernel-provided symbols are external dependencies.

use core::ffi::c_void;

#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct arm_smccc_res { pub a0: u64, pub a1: u64, pub a2: u64, pub a3: u64 }
#[repr(C)] pub struct notifier_block { pub notifier_call: Option<unsafe extern "C" fn(*mut notifier_block, usize, *mut c_void) -> i32> }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct platform_device { pub dev: device }
#[repr(C)] pub struct device_node { _private: [u8; 0] }
#[repr(C)] pub struct pt_regs { _private: [u8; 0] }
#[repr(C)] pub struct ghes { pub generic: *mut ghes_generic }
#[repr(C)] pub struct ghes_generic { pub notify: ghes_notify }
#[repr(C)] pub struct ghes_notify { pub vector: u32 }
#[repr(C)] pub struct sdei_registered_event { pub event_num: u32, pub priority: u8, pub callback: Option<sdei_event_callback>, pub callback_arg: *mut c_void }
pub type sdei_event_callback = unsafe extern "C" fn(u32, *mut pt_regs, *mut c_void) -> i32;
pub type smp_call_func_t = unsafe extern "C" fn(*mut c_void);

extern "C" {
    static mut acpi_disabled: bool;
    static mut sdei_active_critical_event: *mut c_void;
    static mut sdei_active_normal_event: *mut c_void;
    fn arm_smccc_smc(u: usize, a: usize,b: usize,c: usize,d: usize,e: usize,f: usize,g: usize,r: *mut arm_smccc_res);
    fn arm_smccc_hvc(u: usize, a: usize,b: usize,c: usize,d: usize,e: usize,f: usize,g: usize,r: *mut arm_smccc_res);
    fn sdei_arch_get_entry_point(conduit: i32) -> usize;
    fn __sdei_handler_abort();
}

const EOPNOTSUPP: i32 = 95; const EINVAL: i32 = 22; const EPERM: i32 = 1;
const EINPROGRESS: i32 = 115; const ENOMEM: i32 = 12; const EIO: i32 = 5;
const SDEI_NOT_SUPPORTED: u64 = 0; const SDEI_INVALID_PARAMETERS: u64 = 1;
const SDEI_DENIED: u64 = 2; const SDEI_PENDING: u64 = 3; const SDEI_OUT_OF_RESOURCE: u64 = 4;
const SDEI_EVENT_TYPE_SHARED: u8 = 0; const SDEI_EVENT_PRIORITY_CRITICAL: u64 = 0;
const SDEI_EVENT_REGISTER_RM_ANY: u64 = 1;
const SDEI_1_0_FN_SDEI_EVENT_CONTEXT: usize = 0;
const SDEI_1_0_FN_SDEI_EVENT_GET_INFO: usize = 1;
const SDEI_1_0_FN_SDEI_VERSION: usize = 2;
const SDEI_1_0_FN_SDEI_PE_MASK: usize = 3; const SDEI_1_0_FN_SDEI_PE_UNMASK: usize = 4;
const SDEI_1_0_FN_SDEI_EVENT_SIGNAL: usize = 5; const SDEI_1_0_FN_SDEI_PRIVATE_RESET: usize = 6;
const SDEI_1_0_FN_SDEI_SHARED_RESET: usize = 7; const SDEI_1_0_FN_SDEI_EVENT_ENABLE: usize = 8;
const SDEI_1_0_FN_SDEI_EVENT_DISABLE: usize = 9; const SDEI_1_0_FN_SDEI_EVENT_UNREGISTER: usize = 10;
const SDEI_1_0_FN_SDEI_EVENT_REGISTER: usize = 11;

#[repr(C)] pub struct sdei_event { pub list: list_head, pub reregister: bool, pub reenable: bool, pub event_num: u32, pub type_: u8, pub priority: u8, pub registered: *mut sdei_registered_event, pub private_registered: *mut sdei_registered_event }
#[repr(C)] pub struct sdei_crosscall_args { pub event: *mut sdei_event, pub errors: i32, pub first_error: i32 }

static mut sdei_firmware_call: Option<unsafe extern "C" fn(usize,usize,usize,usize,usize,usize,*mut arm_smccc_res)> = None;
static mut sdei_entry_point: usize = 0; static mut sdei_hp_state: i32 = 0;
static mut sdei_list: list_head = list_head { next: core::ptr::null_mut(), prev: core::ptr::null_mut() };

unsafe fn invoke_sdei_fn(id: usize,a0:usize,a1:usize,a2:usize,a3:usize,a4:usize,result:*mut u64)->i32 {
    let mut res = arm_smccc_res { a0: SDEI_NOT_SUPPORTED,a1:0,a2:0,a3:0 };
    let err = if let Some(f)=sdei_firmware_call { f(id,a0,a1,a2,a3,a4,&mut res); match res.a0 { SDEI_NOT_SUPPORTED=>-EOPNOTSUPP,SDEI_INVALID_PARAMETERS=>-EINVAL,SDEI_DENIED=>-EPERM,SDEI_PENDING=>-EINPROGRESS,SDEI_OUT_OF_RESOURCE=>-ENOMEM,_=>0 } } else { -EIO };
    if !result.is_null() { *result=res.a0; } err
}

pub unsafe extern "C" fn sdei_api_event_context(query:u32,result:*mut u64)->i32 { invoke_sdei_fn(SDEI_1_0_FN_SDEI_EVENT_CONTEXT,query as usize,0,0,0,0,result) }
unsafe fn sdei_api_event_get_info(event:u32,info:u32,result:*mut u64)->i32 { invoke_sdei_fn(SDEI_1_0_FN_SDEI_EVENT_GET_INFO,event as usize,info as usize,0,0,0,result) }
pub unsafe extern "C" fn sdei_mask_local_cpu()->i32 { let e=invoke_sdei_fn(SDEI_1_0_FN_SDEI_PE_MASK,0,0,0,0,0,core::ptr::null_mut()); if e!=0 && e!=-EIO { return e } 0 }
pub unsafe extern "C" fn sdei_unmask_local_cpu()->i32 { let e=invoke_sdei_fn(SDEI_1_0_FN_SDEI_PE_UNMASK,0,0,0,0,0,core::ptr::null_mut()); if e!=0 && e!=-EIO { return e } 0 }
pub unsafe extern "C" fn sdei_event_signal(event:u32,mpidr:u64)->i32 { invoke_sdei_fn(SDEI_1_0_FN_SDEI_EVENT_SIGNAL,event as usize,mpidr as usize,0,0,0,core::ptr::null_mut()) }
pub unsafe extern "C" fn sdei_is_present()->bool { sdei_firmware_call.is_some() }

unsafe extern "C" fn sdei_smccc_smc(id:usize,a:usize,b:usize,c:usize,d:usize,e:usize,r:*mut arm_smccc_res){ arm_smccc_smc(id,a,b,c,d,e,0,0,r) }
unsafe extern "C" fn sdei_smccc_hvc(id:usize,a:usize,b:usize,c:usize,d:usize,e:usize,r:*mut arm_smccc_res){ arm_smccc_hvc(id,a,b,c,d,e,0,0,r) }

pub unsafe extern "C" fn sdei_event_handler(regs:*mut pt_regs,arg:*mut sdei_registered_event)->i32 { ((*arg).callback.unwrap())((*arg).event_num,regs,(*arg).callback_arg) }
pub unsafe extern "C" fn sdei_handler_abort(){ if !sdei_active_critical_event.is_null(){ __sdei_handler_abort(); sdei_active_critical_event=core::ptr::null_mut(); } if !sdei_active_normal_event.is_null(){ __sdei_handler_abort(); sdei_active_normal_event=core::ptr::null_mut(); } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
