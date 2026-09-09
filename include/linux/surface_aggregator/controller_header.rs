/* SPDX-License-Identifier: GPL-2.0+ */
/* Rust translation of the SSAM controller interface header. */

#[repr(u32)]
pub enum ssam_event_flags { SSAM_EVENT_SEQUENCED = 1 << 0 }

#[repr(C)]
pub struct ssam_event {
    pub target_category: u8, pub target_id: u8, pub command_id: u8,
    pub instance_id: u8, pub length: u16, pub data: [u8; 0],
}

#[repr(u32)]
pub enum ssam_request_flags {
    SSAM_REQUEST_HAS_RESPONSE = 1 << 0,
    SSAM_REQUEST_UNSEQUENCED = 1 << 1,
}

#[repr(C)]
pub struct ssam_request {
    pub target_category: u8, pub target_id: u8, pub command_id: u8,
    pub instance_id: u8, pub flags: u16, pub length: u16,
    pub payload: *const u8,
}

#[repr(C)]
pub struct ssam_response { pub capacity: usize, pub length: usize, pub pointer: *mut u8 }

pub enum ssam_controller {}

extern "C" {
    pub fn ssam_get_controller() -> *mut ssam_controller;
    pub fn ssam_client_bind(client: *mut device) -> *mut ssam_controller;
    pub fn ssam_client_link(ctrl: *mut ssam_controller, client: *mut device) -> i32;
    pub fn ssam_controller_device(c: *mut ssam_controller) -> *mut device;
    pub fn ssam_controller_get(c: *mut ssam_controller) -> *mut ssam_controller;
    pub fn ssam_controller_put(c: *mut ssam_controller);
    pub fn ssam_controller_statelock(c: *mut ssam_controller);
    pub fn ssam_controller_stateunlock(c: *mut ssam_controller);
    pub fn ssam_request_write_data(buf: *mut ssam_span, ctrl: *mut ssam_controller,
                                   spec: *const ssam_request) -> isize;
}

#[repr(C)]
pub struct ssam_request_sync {
    pub base: ssh_request, pub comp: completion, pub resp: *mut ssam_response, pub status: i32,
}

extern "C" {
    pub fn ssam_request_sync_alloc(payload_len: usize, flags: gfp_t,
                                    rqst: *mut *mut ssam_request_sync,
                                    buffer: *mut ssam_span) -> i32;
    pub fn ssam_request_sync_free(rqst: *mut ssam_request_sync);
    pub fn ssam_request_sync_init(rqst: *mut ssam_request_sync, flags: ssam_request_flags) -> i32;
    pub fn ssam_request_sync_submit(ctrl: *mut ssam_controller, rqst: *mut ssam_request_sync) -> i32;
    pub fn ssam_request_do_sync(ctrl: *mut ssam_controller, spec: *const ssam_request,
                                rsp: *mut ssam_response) -> i32;
    pub fn ssam_request_do_sync_with_buffer(ctrl: *mut ssam_controller, spec: *const ssam_request,
                                            rsp: *mut ssam_response, buf: *mut ssam_span) -> i32;
}

#[inline]
pub unsafe fn ssam_request_sync_set_data(rqst: *mut ssam_request_sync, ptr: *mut u8, len: usize) {
    ssh_request_set_data(&mut (*rqst).base, ptr, len);
}
#[inline]
pub unsafe fn ssam_request_sync_set_resp(rqst: *mut ssam_request_sync, resp: *mut ssam_response) {
    (*rqst).resp = resp;
}
#[inline]
pub unsafe fn ssam_request_sync_wait(rqst: *mut ssam_request_sync) -> i32 {
    wait_for_completion(&mut (*rqst).comp); (*rqst).status
}

#[repr(C)]
pub struct ssam_request_spec { pub target_category: u8, pub target_id: u8, pub command_id: u8, pub instance_id: u8, pub flags: u8 }
#[repr(C)]
pub struct ssam_request_spec_md { pub target_category: u8, pub command_id: u8, pub flags: u8 }

#[inline]
pub unsafe fn ssam_request_do_sync_onstack(ctrl: *mut ssam_controller, rqst: *const ssam_request,
                                            rsp: *mut ssam_response, payload_len: usize) -> i32 {
    let mut data = [0u8; 0];
    let mut buf = ssam_span { ptr: data.as_mut_ptr(), len: data.len() };
    ssam_request_do_sync_with_buffer(ctrl, rqst, rsp, &mut buf)
}

/* The C request-generation macros are retained as Rust declarative macros. */
#[macro_export]
macro_rules! ssam_retry { ($request:expr, $($args:expr),* $(,)?) => {{
    let mut __s = 0i32; for _ in 0..3 { __s = $request($($args),*); if __s != -110 && __s != -121 { break; } } __s
}} }

pub const SSAM_NOTIF_STATE_SHIFT: u32 = 2;
pub const SSAM_NOTIF_STATE_MASK: u32 = (1 << SSAM_NOTIF_STATE_SHIFT) - 1;
#[repr(u32)]
pub enum ssam_notif_flags { SSAM_NOTIF_HANDLED = 1 << 0, SSAM_NOTIF_STOP = 1 << 1 }

#[repr(C)]
pub struct ssam_notifier_block { pub node: list_head, pub fn_: Option<unsafe extern "C" fn(*mut ssam_event_notifier, *const ssam_event) -> u32>, pub priority: i32 }
pub struct ssam_event_notifier;
#[inline] pub fn ssam_notifier_from_errno(err: i32) -> u32 {
    if err >= 0 { 0 } else { ((-err as u32) << SSAM_NOTIF_STATE_SHIFT) | (SSAM_NOTIF_STOP as u32) }
}
#[inline] pub fn ssam_notifier_to_errno(ret: u32) -> i32 { -((ret >> SSAM_NOTIF_STATE_SHIFT) as i32) }

#[repr(C)] pub struct ssam_event_registry { pub target_category: u8, pub target_id: u8, pub cid_enable: u8, pub cid_disable: u8 }
#[repr(C)] pub struct ssam_event_id { pub target_category: u8, pub instance: u8 }
#[repr(u32)] pub enum ssam_event_mask { SSAM_EVENT_MASK_TARGET = 1 << 0, SSAM_EVENT_MASK_INSTANCE = 1 << 1, SSAM_EVENT_MASK_NONE = 0, SSAM_EVENT_MASK_STRICT = 3 }
#[repr(u32)] pub enum ssam_event_notifier_flags { SSAM_EVENT_NOTIFIER_OBSERVER = 1 << 0 }

#[repr(C)] pub struct ssam_event_notifier_full {
    pub base: ssam_notifier_block,
    pub event: ssam_event_notifier_event,
    pub flags: usize,
}
#[repr(C)] pub struct ssam_event_notifier_event { pub reg: ssam_event_registry, pub id: ssam_event_id, pub mask: ssam_event_mask, pub flags: u8 }

extern "C" {
    pub fn ssam_notifier_register(ctrl: *mut ssam_controller, n: *mut ssam_event_notifier_full) -> i32;
    pub fn __ssam_notifier_unregister(ctrl: *mut ssam_controller, n: *mut ssam_event_notifier_full, disable: bool) -> i32;
    pub fn ssam_controller_event_enable(ctrl: *mut ssam_controller, reg: ssam_event_registry, id: ssam_event_id, flags: u8) -> i32;
    pub fn ssam_controller_event_disable(ctrl: *mut ssam_controller, reg: ssam_event_registry, id: ssam_event_id, flags: u8) -> i32;
}

#[inline] pub unsafe fn ssam_notifier_unregister(ctrl: *mut ssam_controller, n: *mut ssam_event_notifier_full) -> i32 { __ssam_notifier_unregister(ctrl, n, true) }

/* Types and functions supplied by the included kernel headers. */
extern "C" { pub type device; pub type ssam_span; pub type ssh_request; pub type completion; pub type list_head; pub type gfp_t; pub fn ssh_request_set_data(r: *mut ssh_request, p: *mut u8, n: usize); pub fn wait_for_completion(c: *mut completion); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
