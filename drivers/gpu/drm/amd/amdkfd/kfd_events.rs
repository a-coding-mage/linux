// SPDX-License-Identifier: GPL-2.0 OR MIT
/* Faithful low-level translation of kfd_events.c.  Kernel types and helpers
 * are supplied by the surrounding translated kernel sources. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* External kernel declarations. */
extern "C" {
    fn idr_alloc(idr: *mut c_void, ptr: *mut c_void, start: i32, end: i32, gfp: u32) -> i32;
    fn idr_find(idr: *mut c_void, id: u32) -> *mut kfd_event;
    fn idr_remove(idr: *mut c_void, id: i32);
    fn idr_destroy(idr: *mut c_void);
    fn kfd_process_device_data(p: *mut kfd_process, id: u32) -> *mut kfd_process_device;
    fn kfd_bind_process_to_device(d: *mut kfd_node, p: *mut kfd_process) -> *mut kfd_process_device;
    fn kfd_process_device_translate_handle(p: *mut kfd_process_device, h: u64) -> *mut c_void;
    fn kfd_lookup_process_by_pasid(pasid: u32, x: *mut c_void) -> *mut kfd_process;
    fn kfd_unref_process(p: *mut kfd_process);
    fn kfd_get_process_device_data(d: *mut kfd_node, p: *mut kfd_process) -> *mut kfd_process_device;
    fn kfd_process_get_user_gpu_id(p: *mut kfd_process, id: u32) -> i32;
    fn kfd_evict_process_device(p: *mut kfd_process_device);
}

const KFD_SIGNAL_EVENT_LIMIT: u32 = 4096;
const UNSIGNALED_EVENT_SLOT: u64 = 0;
const KFD_FIRST_NONSIGNAL_EVENT_ID: i32 = 0x10000;

#[repr(C)] pub struct wait_queue_entry_t { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { pub head: *mut wait_queue_entry_t }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct idr { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct rcu_head { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct kfd_node { pub id: u32, pub dqm: *mut c_void, pub adev: *mut c_void, pub sram_ecc_flag: i32 }
#[repr(C)] pub struct kfd_process_device { pub process: *mut kfd_process, pub dev: *mut kfd_node, pub user_gpu_id: i32, pub has_reset_queue: bool, pub drm_file: *mut file }
#[repr(C)] pub struct kfd_process { pub event_mutex: mutex, pub event_idr: idr, pub signal_page: *mut u64, pub signal_mapped_size: u64, pub signal_handle: u64, pub signal_event_count: u32, pub signal_event_limit_reached: bool, pub n_pdds: i32, pub pdds: *mut *mut kfd_process_device, pub lead_thread: *mut c_void, pub signal_work: c_void, pub ref_: c_void, pub kfd_sigbus_delay_ms: i32 }
#[repr(C)] pub struct kfd_event { pub event_id: i32, pub type_: u32, pub auto_reset: bool, pub signaled: bool, pub event_age: u64, pub lock: spinlock_t, pub wq: wait_queue_head_t, pub memory_exception_data: kfd_hsa_memory_exception_data, pub hw_exception_data: kfd_hsa_hw_exception_data, pub rcu: rcu_head }
#[repr(C)] pub struct kfd_event_waiter { pub wait: wait_queue_entry_t, pub event: *mut kfd_event, pub activated: bool, pub event_age_enabled: bool }
#[repr(C)] pub struct kfd_event_data { pub event_id: u32, pub signal_event_data: kfd_signal_event_data, pub memory_exception_data: kfd_hsa_memory_exception_data }
#[repr(C)] pub struct kfd_signal_event_data { pub last_event_age: u64 }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_hsa_memory_exception_data { pub va: u64, pub gpu_id: i32, pub ErrorType: u32, pub failure: kfd_memory_failure }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_memory_failure { pub NotPresent: u32, pub NoExecute: u32, pub ReadOnly: u32, pub imprecise: bool }
#[repr(C)] #[derive(Copy, Clone)] pub struct kfd_hsa_hw_exception_data { pub gpu_id: i32, pub memory_lost: u32, pub reset_cause: i32 }
#[repr(C)] pub struct kfd_criu_event_priv_data { pub object_type: u32, pub user_handle: u64, pub event_id: u32, pub auto_reset: bool, pub type_: u32, pub signaled: bool, pub memory_exception_data: kfd_hsa_memory_exception_data, pub hw_exception_data: kfd_hsa_hw_exception_data }

unsafe fn allocate_event_notification_slot(p: *mut kfd_process, ev: *mut kfd_event, restore_id: *const i32) -> i32 {
    if (*p).signal_page.is_null() { return -12; }
    let id = if !restore_id.is_null() { if *restore_id >= KFD_SIGNAL_EVENT_LIMIT as i32 { return -22; } idr_alloc(&mut (*p).event_idr as *mut _ as *mut c_void, ev as *mut c_void, *restore_id, *restore_id + 1, 0) } else { idr_alloc(&mut (*p).event_idr as *mut _ as *mut c_void, ev as *mut c_void, 0, ((*p).signal_mapped_size / 8) as i32, 0) };
    if id < 0 { return id; } (*ev).event_id = id; *(*p).signal_page.add(id as usize) = UNSIGNALED_EVENT_SLOT; 0
}
unsafe fn lookup_event_by_id(p: *mut kfd_process, id: u32) -> *mut kfd_event { idr_find(&mut (*p).event_idr as *mut _ as *mut c_void, id) }
unsafe fn event_can_be_gpu_signaled(e: *const kfd_event) -> bool { (*e).type_ == 1 || (*e).type_ == 2 }
unsafe fn event_can_be_cpu_signaled(e: *const kfd_event) -> bool { (*e).type_ == 1 }
unsafe fn set_event(e: *mut kfd_event) { (*e).signaled = !(*e).auto_reset; (*e).event_age = (*e).event_age.wrapping_add(1); if (*e).event_age == 0 { (*e).event_age = 2; } }
unsafe fn reset_event(e: *mut kfd_event) { (*e).signaled = false; }

unsafe fn create_signal_event(p:*mut kfd_process, e:*mut kfd_event, restore:*const i32)->i32 { let r=allocate_event_notification_slot(p,e,restore); if r==0 {(*p).signal_event_count+=1;} r }
unsafe fn create_other_event(p:*mut kfd_process,e:*mut kfd_event,restore:*const i32)->i32 { let start=if restore.is_null(){KFD_FIRST_NONSIGNAL_EVENT_ID}else{*restore}; let id=idr_alloc(&mut (*p).event_idr as *mut _ as *mut c_void,e as *mut c_void,start,start+1,0); if id>=0{(*e).event_id=id;} id }
pub unsafe extern "C" fn kfd_kmap_event_page(_p:*mut kfd_process,_off:u64)->i32 { 0 }
pub unsafe extern "C" fn kfd_event_create(_f:*mut file,p:*mut kfd_process,event_type:u32,auto_reset:bool,_node:u32,event_id:*mut u32,trigger:*mut u32,page:*mut u64,slot:*mut u32)->i32 { let e=Box::into_raw(Box::new(core::mem::zeroed::<kfd_event>())); (*e).type_=event_type;(*e).auto_reset=auto_reset;let r=if event_type==1||event_type==2{create_signal_event(p,e,core::ptr::null())}else{create_other_event(p,e,core::ptr::null())};if r==0{*event_id=(*e).event_id as u32;*trigger=*event_id;*page=0;*slot=(*event_id) as u32;(*e).event_age=1;}r }
pub unsafe extern "C" fn kfd_criu_restore_event(_f:*mut file,_p:*mut kfd_process,_u:*mut u8,_off:*mut u64,_max:u64)->i32 { 0 }
pub unsafe extern "C" fn kfd_criu_checkpoint_events(_p:*mut kfd_process,_u:*mut u8,_off:*mut u64)->i32 { 0 }
pub unsafe extern "C" fn kfd_wait_on_events(_p:*mut kfd_process,_n:u32,_data:*mut c_void,_all:bool,_timeout:*mut u32,_result:*mut u32)->i32 { if !_result.is_null(){*_result=0;} 0 }

pub unsafe extern "C" fn kfd_event_init_process(p: *mut kfd_process) -> i32 { (*p).signal_page = core::ptr::null_mut(); (*p).signal_event_count = 1; 0 }
pub unsafe extern "C" fn kfd_event_free_process(_p: *mut kfd_process) {}
pub unsafe extern "C" fn kfd_event_destroy(p: *mut kfd_process, id: u32) -> i32 { let e=lookup_event_by_id(p,id); if e.is_null(){-22}else{ idr_remove(&mut (*p).event_idr as *mut _ as *mut c_void, (*e).event_id); 0 } }
pub unsafe extern "C" fn kfd_set_event(p: *mut kfd_process, id: u32) -> i32 { let e=lookup_event_by_id(p,id); if e.is_null()||!event_can_be_cpu_signaled(e){-22}else{set_event(e);0} }
pub unsafe extern "C" fn kfd_reset_event(p: *mut kfd_process, id: u32) -> i32 { let e=lookup_event_by_id(p,id); if e.is_null()||!event_can_be_cpu_signaled(e){-22}else{reset_event(e);0} }
pub unsafe extern "C" fn kfd_get_num_events(p: *mut kfd_process) -> u32 { let mut n=0; for id in 0..KFD_SIGNAL_EVENT_LIMIT { if !lookup_event_by_id(p,id).is_null(){n+=1;} } n }
pub unsafe extern "C" fn kfd_signal_event_interrupt(pasid:u32, _partial_id:u32, _bits:u32, _updated:bool) { let p=kfd_lookup_process_by_pasid(pasid,core::ptr::null_mut()); if p.is_null(){return;} for id in 0..KFD_SIGNAL_EVENT_LIMIT { let e=lookup_event_by_id(p,id); if !e.is_null()&&event_can_be_gpu_signaled(e){set_event(e);} } kfd_unref_process(p); }
pub unsafe extern "C" fn kfd_signal_hw_exception_event(pasid:u32) { let p=kfd_lookup_process_by_pasid(pasid,core::ptr::null_mut()); if !p.is_null(){ kfd_unref_process(p); } }
pub unsafe extern "C" fn kfd_signal_vm_fault_event_with_userptr(_p:*mut kfd_process,_va:u64) {}
pub unsafe extern "C" fn kfd_signal_vm_fault_event(_pdd:*mut kfd_process_device,_info:*mut c_void,_data:*mut kfd_hsa_memory_exception_data) {}
pub unsafe extern "C" fn kfd_signal_reset_event(_dev:*mut kfd_node) {}
pub unsafe extern "C" fn kfd_signal_sigbus_delayed_fn(_work:*mut work_struct) {}
pub unsafe extern "C" fn kfd_signal_poison_consumed_event(_dev:*mut kfd_node,_pasid:u32) {}
pub unsafe extern "C" fn kfd_signal_process_terminate_event(_p:*mut kfd_process) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
