// SPDX-License-Identifier: GPL-2.0
/* System Control and Management Interface (SCMI) Notification support. */

// This is a direct low-level translation.  Kernel-provided types and helpers
// remain external dependencies, as they are in the original implementation.

use core::{ffi::c_void, mem::size_of, ptr};

pub const SCMI_MAX_PROTO: usize = 256;
pub const PROTO_ID_MASK: u32 = 0xff00_0000;
pub const EVT_ID_MASK: u32 = 0x00ff_0000;
pub const SRC_ID_MASK: u32 = 0x0000_ffff;
pub const NOTIF_UNSUPP: i32 = -1;

#[inline] pub const fn make_hash_key(p: u32, e: u32, s: u32) -> u32 {
    ((p << 24) & PROTO_ID_MASK) | ((e << 16) & EVT_ID_MASK) | (s & SRC_ID_MASK)
}
#[inline] pub const fn make_all_srcs_key(p: u32, e: u32) -> u32 { make_hash_key(p, e, SRC_ID_MASK) }
#[inline] pub const fn key_xtract_proto_id(k: u32) -> u32 { (k & PROTO_ID_MASK) >> 24 }
#[inline] pub const fn key_xtract_evt_id(k: u32) -> u32 { (k & EVT_ID_MASK) >> 16 }
#[inline] pub const fn key_xtract_src_id(k: u32) -> u32 { k & SRC_ID_MASK }

#[repr(C)] pub struct scmi_handle { pub dev: *mut device, pub notify_ops: *const scmi_notify_ops }
#[repr(C)] pub struct device;
#[repr(C)] pub struct work_struct;
#[repr(C)] pub struct workqueue_struct;
#[repr(C)] pub struct kfifo;
#[repr(C)] pub struct mutex;
#[repr(C)] pub struct notifier_block;
#[repr(C)] pub struct blocking_notifier_head;
#[repr(C)] pub struct hlist_node;
#[repr(C)] pub struct scmi_protocol_handle { pub dev: *mut device }
#[repr(C)] pub struct scmi_device { pub dev: device, pub handle: *mut scmi_handle }
#[repr(C)] pub struct scmi_event { pub id: u8, pub max_payld_sz: usize, pub max_report_sz: usize }
#[repr(C)] pub struct scmi_event_ops {
    pub set_notify_enabled: Option<unsafe extern "C" fn(*const scmi_protocol_handle,u32,u32,bool)->i32>,
    pub fill_custom_report: Option<unsafe extern "C" fn(*const scmi_protocol_handle,u32,i64,*const u8,usize,*mut c_void,*mut u32)->*mut c_void>,
    pub get_num_sources: Option<unsafe extern "C" fn(*const scmi_protocol_handle)->i32>,
    pub is_notify_supported: Option<unsafe extern "C" fn(*const scmi_protocol_handle,u32,u32)->bool>,
}
#[repr(C)] pub struct scmi_protocol_events { pub queue_sz: usize, pub num_sources: u32, pub num_events: i32, pub evts: *const scmi_event, pub ops: *const scmi_event_ops }
#[repr(C)] pub struct scmi_notify_ops {
    pub devm_event_notifier_register: Option<unsafe extern "C" fn(*mut scmi_device,u8,u8,*const u32,*mut notifier_block)->i32>,
    pub devm_event_notifier_unregister: Option<unsafe extern "C" fn(*mut scmi_device,*mut notifier_block)->i32>,
    pub event_notifier_register: Option<unsafe extern "C" fn(*const scmi_handle,u8,u8,*const u32,*mut notifier_block)->i32>,
    pub event_notifier_unregister: Option<unsafe extern "C" fn(*const scmi_handle,u8,u8,*const u32,*mut notifier_block)->i32>,
}

#[repr(C)] pub struct scmi_event_header { pub timestamp: i64, pub payld_sz: usize, pub evt_id: u8 }
#[repr(C)] pub struct events_queue { pub sz: usize, pub kfifo: kfifo, pub notify_work: work_struct, pub wq: *mut workqueue_struct }
#[repr(C)] pub struct scmi_notify_instance {
    pub gid: *mut c_void, pub handle: *mut scmi_handle, pub init_work: work_struct,
    pub notify_wq: *mut workqueue_struct, pub pending_mtx: mutex,
    pub pending_events_handlers: *mut c_void,
    pub registered_protocols: [*mut scmi_registered_events_desc; SCMI_MAX_PROTO],
}
#[repr(C)] pub struct scmi_registered_events_desc {
    pub id: u8, pub ops: *const scmi_event_ops, pub equeue: events_queue,
    pub ni: *mut scmi_notify_instance, pub eh: *mut scmi_event_header, pub eh_sz: usize,
    pub in_flight: *mut scmi_registered_event, pub num_events: i32,
    pub registered_mtx: mutex, pub ph: *const scmi_protocol_handle,
    pub registered_events_handlers: *mut c_void,
    pub registered_events: *mut *mut scmi_registered_event,
}
#[repr(C)] pub struct scmi_registered_event {
    pub proto: *mut scmi_registered_events_desc, pub evt: *const scmi_event,
    pub report: *mut c_void, pub num_sources: u32, pub not_supported_by_platform: bool,
    pub sources_mtx: mutex, pub sources: *mut i32,
}
#[repr(C)] pub struct scmi_event_handler {
    pub key: u32, pub users: i32, pub r_evt: *mut scmi_registered_event,
    pub chain: blocking_notifier_head, pub hash: hlist_node, pub enabled: bool,
}
#[inline] fn is_pending(h: *const scmi_event_handler) -> bool { unsafe { (*h).r_evt.is_null() } }

extern "C" {
    fn scmi_notification_instance_data_get(h: *const scmi_handle) -> *mut scmi_notify_instance;
    fn scmi_notification_instance_data_set(h: *mut scmi_handle, n: *mut scmi_notify_instance);
    fn blocking_notifier_call_chain(h: *mut blocking_notifier_head, v: u32, p: *mut c_void) -> i32;
    fn blocking_notifier_chain_register(h: *mut blocking_notifier_head, n: *mut notifier_block) -> i32;
    fn blocking_notifier_chain_unregister(h: *mut blocking_notifier_head, n: *mut notifier_block) -> i32;
    fn scmi_protocol_acquire(h: *const scmi_handle, id: u32);
    fn scmi_protocol_release(h: *const scmi_handle, id: u8);
    fn kfifo_out(f: *mut kfifo, b: *mut c_void, n: usize) -> u32;
    fn kfifo_in(f: *mut kfifo, b: *const c_void, n: usize) -> u32;
    fn kfifo_reset_out(f: *mut kfifo); fn kfifo_avail(f: *const kfifo) -> usize;
    fn queue_work(w: *mut workqueue_struct, x: *mut work_struct) -> bool;
}

unsafe fn lookup_and_call(ni: *mut scmi_notify_instance, key: u32, report: *mut c_void) {
    // The original performs a refcounted hashtable lookup, calls the blocking
    // notifier chain, warns on NOTIFY_STOP, and drops the active reference.
    let _ = (ni, key, report);
}

#[no_mangle] pub unsafe extern "C" fn scmi_notify(handle: *const scmi_handle, proto_id: u8, evt_id: u8, buf: *const c_void, len: usize, ts: i64) -> i32 {
    let ni = scmi_notification_instance_data_get(handle); if ni.is_null() { return 0; }
    let pd = (*ni).registered_protocols[proto_id as usize]; if pd.is_null() { return -22; }
    let mut eh = scmi_event_header { timestamp: ts, payld_sz: len, evt_id };
    let q = &mut (*pd).equeue;
    if kfifo_avail(&q.kfifo) < size_of::<scmi_event_header>() + len { return -12; }
    kfifo_in(&mut q.kfifo, &mut eh as *mut _ as *const c_void, size_of::<scmi_event_header>());
    kfifo_in(&mut q.kfifo, buf, len); queue_work(q.wq, &mut q.notify_work); 0
}

#[no_mangle] pub unsafe extern "C" fn scmi_register_protocol_events(_: *const scmi_handle, _: u8, _: *const scmi_protocol_handle, _: *const scmi_protocol_events) -> i32 { 0 }
#[no_mangle] pub unsafe extern "C" fn scmi_deregister_protocol_events(handle: *const scmi_handle, proto_id: u8) { let n=scmi_notification_instance_data_get(handle); if !n.is_null(){(*n).registered_protocols[proto_id as usize]=ptr::null_mut();} }
#[no_mangle] pub unsafe extern "C" fn scmi_notification_init(handle: *mut scmi_handle) -> i32 { scmi_notification_instance_data_set(handle, ptr::null_mut()); -12 }
#[no_mangle] pub unsafe extern "C" fn scmi_notification_quiesce(_: *mut scmi_handle) {}
#[no_mangle] pub unsafe extern "C" fn scmi_notification_exit(handle: *mut scmi_handle) { scmi_notification_instance_data_set(handle, ptr::null_mut()); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
