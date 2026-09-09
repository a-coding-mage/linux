/* SPDX-License-Identifier: GPL-2.0-or-later */
/* iSCSI transport class definitions. */

// C dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)] pub struct scsi_transport_template { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct request_queue { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct Scsi_Host { _private: [u8; 0] }
#[repr(C)] pub struct scsi_cmnd { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr { _private: [u8; 0] }
#[repr(C)] pub struct bsg_job { _private: [u8; 0] }
#[repr(C)] pub struct iscsi_hdr { _private: [u8; 0] }
#[repr(C)] pub struct iscsi_stats { _private: [u8; 0] }
#[repr(C)] pub struct iscsi_path { _private: [u8; 0] }
#[repr(C)] pub struct iscsi_task { _private: [u8; 0] }
#[repr(C)] pub struct iscsi_endpoint { _private: [u8; 0] }
#[repr(C)] pub struct iscsi_iface { _private: [u8; 0] }
#[repr(C)] pub struct iscsi_bus_flash_session { _private: [u8; 0] }
#[repr(C)] pub struct iscsi_bus_flash_conn { _private: [u8; 0] }
#[repr(C)] pub struct iscsi_cls_session { _private: [u8; 0] }
#[repr(C)] pub struct iscsi_cls_conn { _private: [u8; 0] }

pub type uint8_t = u8; pub type uint16_t = u16; pub type uint32_t = u32; pub type uint64_t = u64;
pub type u8_ = u8; pub type u64_ = u64; pub type pid_t = i32; pub type umode_t = u16; pub type sector_t = u64; pub type itt_t = u32;
pub enum iscsi_param {} pub enum iscsi_host_param {} pub enum iscsi_tgt_dscvr {}
pub enum iscsi_param_type {} pub enum iscsi_err {} pub enum iscsi_conn_state {}
pub enum iscsi_uevent_e {} pub enum iscsi_ipaddress_state {} pub enum iscsi_router_state {} pub enum iscsi_host_event_code {}
pub type device_match_t = unsafe extern "C" fn(*const device) -> bool;

#[repr(C)] pub struct iscsi_transport {
    pub owner: *mut module, pub name: *mut c_char, pub caps: c_uint,
    pub create_session: Option<unsafe extern "C" fn(*mut iscsi_endpoint, u16, u16, u32) -> *mut iscsi_cls_session>,
    pub destroy_session: Option<unsafe extern "C" fn(*mut iscsi_cls_session)>,
    pub create_conn: Option<unsafe extern "C" fn(*mut iscsi_cls_session, u32) -> *mut iscsi_cls_conn>,
    pub unbind_conn: Option<unsafe extern "C" fn(*mut iscsi_cls_conn, bool)>,
    pub bind_conn: Option<unsafe extern "C" fn(*mut iscsi_cls_session, *mut iscsi_cls_conn, u64, c_int) -> c_int>,
    pub start_conn: Option<unsafe extern "C" fn(*mut iscsi_cls_conn) -> c_int>,
    pub stop_conn: Option<unsafe extern "C" fn(*mut iscsi_cls_conn, c_int)>, pub destroy_conn: Option<unsafe extern "C" fn(*mut iscsi_cls_conn)>,
    pub set_param: Option<unsafe extern "C" fn(*mut iscsi_cls_conn, iscsi_param, *mut c_char, c_int) -> c_int>,
    pub get_ep_param: Option<unsafe extern "C" fn(*mut iscsi_endpoint, iscsi_param, *mut c_char) -> c_int>,
    pub get_conn_param: Option<unsafe extern "C" fn(*mut iscsi_cls_conn, iscsi_param, *mut c_char) -> c_int>,
    pub get_session_param: Option<unsafe extern "C" fn(*mut iscsi_cls_session, iscsi_param, *mut c_char) -> c_int>,
    pub get_host_param: Option<unsafe extern "C" fn(*mut Scsi_Host, iscsi_host_param, *mut c_char) -> c_int>,
    pub set_host_param: Option<unsafe extern "C" fn(*mut Scsi_Host, iscsi_host_param, *mut c_char, c_int) -> c_int>,
    pub send_pdu: Option<unsafe extern "C" fn(*mut iscsi_cls_conn, *mut iscsi_hdr, *mut c_char, u32) -> c_int>,
    pub get_stats: Option<unsafe extern "C" fn(*mut iscsi_cls_conn, *mut iscsi_stats)>,
    pub init_task: Option<unsafe extern "C" fn(*mut iscsi_task) -> c_int>, pub xmit_task: Option<unsafe extern "C" fn(*mut iscsi_task) -> c_int>, pub cleanup_task: Option<unsafe extern "C" fn(*mut iscsi_task)>,
    pub alloc_pdu: Option<unsafe extern "C" fn(*mut iscsi_task, u8) -> c_int>, pub xmit_pdu: Option<unsafe extern "C" fn(*mut iscsi_task) -> c_int>, pub init_pdu: Option<unsafe extern "C" fn(*mut iscsi_task, c_uint, c_uint) -> c_int>,
    pub parse_pdu_itt: Option<unsafe extern "C" fn(*mut iscsi_conn, itt_t, *mut c_int, *mut c_int)>,
    pub session_recovery_timedout: Option<unsafe extern "C" fn(*mut iscsi_cls_session)>, pub ep_connect: Option<unsafe extern "C" fn(*mut Scsi_Host, *mut sockaddr, c_int) -> *mut iscsi_endpoint>, pub ep_poll: Option<unsafe extern "C" fn(*mut iscsi_endpoint, c_int) -> c_int>, pub ep_disconnect: Option<unsafe extern "C" fn(*mut iscsi_endpoint)>,
    pub tgt_dscvr: Option<unsafe extern "C" fn(*mut Scsi_Host, iscsi_tgt_dscvr, u32, *mut sockaddr) -> c_int>, pub set_path: Option<unsafe extern "C" fn(*mut Scsi_Host, *mut iscsi_path) -> c_int>, pub set_iface_param: Option<unsafe extern "C" fn(*mut Scsi_Host, *mut c_void, u32) -> c_int>, pub get_iface_param: Option<unsafe extern "C" fn(*mut iscsi_iface, iscsi_param_type, c_int, *mut c_char) -> c_int>,
    pub attr_is_visible: Option<unsafe extern "C" fn(c_int, c_int) -> umode_t>, pub bsg_request: Option<unsafe extern "C" fn(*mut bsg_job) -> c_int>, pub send_ping: Option<unsafe extern "C" fn(*mut Scsi_Host, u32, u32, u32, u32, *mut sockaddr) -> c_int>, pub get_chap: Option<unsafe extern "C" fn(*mut Scsi_Host, u16, *mut u32, *mut c_char) -> c_int>, pub delete_chap: Option<unsafe extern "C" fn(*mut Scsi_Host, u16) -> c_int>, pub set_chap: Option<unsafe extern "C" fn(*mut Scsi_Host, *mut c_void, c_int) -> c_int>,
    pub get_flashnode_param: Option<unsafe extern "C" fn(*mut iscsi_bus_flash_session, c_int, *mut c_char) -> c_int>, pub set_flashnode_param: Option<unsafe extern "C" fn(*mut iscsi_bus_flash_session, *mut iscsi_bus_flash_conn, *mut c_void, c_int) -> c_int>, pub new_flashnode: Option<unsafe extern "C" fn(*mut Scsi_Host, *const c_char, c_int) -> c_int>, pub del_flashnode: Option<unsafe extern "C" fn(*mut iscsi_bus_flash_session) -> c_int>, pub login_flashnode: Option<unsafe extern "C" fn(*mut iscsi_bus_flash_session, *mut iscsi_bus_flash_conn) -> c_int>, pub logout_flashnode: Option<unsafe extern "C" fn(*mut iscsi_bus_flash_session, *mut iscsi_bus_flash_conn) -> c_int>, pub logout_flashnode_sid: Option<unsafe extern "C" fn(*mut iscsi_cls_session) -> c_int>, pub get_host_stats: Option<unsafe extern "C" fn(*mut Scsi_Host, *mut c_char, c_int) -> c_int>, pub check_protection: Option<unsafe extern "C" fn(*mut iscsi_task, *mut sector_t) -> u8>,
}

extern "C" {
    pub fn iscsi_register_transport(*mut iscsi_transport) -> *mut scsi_transport_template; pub fn iscsi_unregister_transport(*mut iscsi_transport);
    pub fn iscsi_conn_error_event(*mut iscsi_cls_conn, iscsi_err); pub fn iscsi_conn_login_event(*mut iscsi_cls_conn, iscsi_conn_state); pub fn iscsi_recv_pdu(*mut iscsi_cls_conn, *mut iscsi_hdr, *mut c_char, u32) -> c_int;
    pub fn iscsi_offload_mesg(*mut Scsi_Host, *mut iscsi_transport, u32, *mut c_char, u16) -> c_int; pub fn iscsi_post_host_event(u32, *mut iscsi_transport, iscsi_host_event_code, u32, *mut u8); pub fn iscsi_ping_comp_event(u32, *mut iscsi_transport, u32, u32, u32, *mut u8);
}

pub const ISCSI_CONN_UP: c_int = 0; pub const ISCSI_CONN_DOWN: c_int = 1; pub const ISCSI_CONN_FAILED: c_int = 2; pub const ISCSI_CONN_BOUND: c_int = 3; pub const ISCSI_CLS_CONN_BIT_CLEANUP: c_int = 1;
pub const ISCSI_SESSION_LOGGED_IN: c_int = 0; pub const ISCSI_SESSION_FAILED: c_int = 1; pub const ISCSI_SESSION_FREE: c_int = 2;
pub const ISCSI_SESSION_TARGET_UNBOUND: c_int = 0; pub const ISCSI_SESSION_TARGET_ALLOCATED: c_int = 1; pub const ISCSI_SESSION_TARGET_SCANNED: c_int = 2; pub const ISCSI_SESSION_TARGET_UNBINDING: c_int = 3; pub const ISCSI_SESSION_TARGET_MAX: c_int = 4; pub const ISCSI_MAX_TARGET: c_int = -1; pub const ISID_SIZE: usize = 6;

#[repr(C)] pub struct iscsi_cls_conn_data { pub conn_list: list_head, pub dd_data: *mut c_void, pub transport: *mut iscsi_transport, pub cid: u32, pub ep_mutex: mutex, pub ep: *mut iscsi_endpoint, pub lock: spinlock_t, pub flags: usize, pub cleanup_work: work_struct, pub dev: device, pub state: c_int }
#[repr(C)] pub struct iscsi_cls_session_data { pub sess_list: list_head, pub transport: *mut iscsi_transport, pub lock: spinlock_t, pub block_work: work_struct, pub unblock_work: work_struct, pub scan_work: work_struct, pub unbind_work: work_struct, pub destroy_work: work_struct, pub recovery_tmo: c_int, pub recovery_tmo_sysfs_override: bool, pub recovery_work: delayed_work, pub workq: *mut workqueue_struct, pub target_id: c_uint, pub ida_used: bool, pub creator: pid_t, pub state: c_int, pub target_state: c_int, pub sid: c_int, pub dd_data: *mut c_void, pub dev: device }
#[repr(C)] pub struct iscsi_cls_host { pub mutex: mutex, pub bsg_q: *mut request_queue, pub port_speed: u32, pub port_state: u32 }
#[repr(C)] pub struct iscsi_endpoint_data { pub dd_data: *mut c_void, pub dev: device, pub id: c_int, pub conn: *mut iscsi_cls_conn }
#[repr(C)] pub struct iscsi_iface_data { pub dev: device, pub transport: *mut iscsi_transport, pub iface_type: u32, pub iface_num: u32, pub dd_data: *mut c_void }
#[repr(C)] pub struct iscsi_conn { _private: [u8; 0] }

pub const fn iscsi_dev_to_conn<T>(_dev: *mut device) -> *mut T { core::ptr::null_mut() }
pub const fn iscsi_dev_to_session<T>(_dev: *mut device) -> *mut T { core::ptr::null_mut() }
pub const fn iscsi_dev_to_iface<T>(_dev: *mut device) -> *mut T { core::ptr::null_mut() }
pub const fn iscsi_dev_to_flash_conn<T>(_dev: *mut device) -> *mut T { core::ptr::null_mut() }
pub const fn iscsi_dev_to_flash_session<T>(_dev: *mut device) -> *mut T { core::ptr::null_mut() }

// The remaining entry points are declaration-only kernel interfaces.
extern "C" {
    pub fn iscsi_session_chkready(*mut iscsi_cls_session) -> c_int; pub fn iscsi_is_session_online(*mut iscsi_cls_session) -> c_int; pub fn iscsi_alloc_session(*mut Scsi_Host, *mut iscsi_transport, c_int) -> *mut iscsi_cls_session; pub fn iscsi_add_session(*mut iscsi_cls_session, c_uint) -> c_int; pub fn iscsi_session_event(*mut iscsi_cls_session, iscsi_uevent_e) -> c_int; pub fn iscsi_force_destroy_session(*mut iscsi_cls_session); pub fn iscsi_remove_session(*mut iscsi_cls_session); pub fn iscsi_free_session(*mut iscsi_cls_session);
    pub fn iscsi_alloc_conn(*mut iscsi_cls_session, c_int, u32) -> *mut iscsi_cls_conn; pub fn iscsi_add_conn(*mut iscsi_cls_conn) -> c_int; pub fn iscsi_remove_conn(*mut iscsi_cls_conn); pub fn iscsi_put_conn(*mut iscsi_cls_conn); pub fn iscsi_get_conn(*mut iscsi_cls_conn); pub fn iscsi_unblock_session(*mut iscsi_cls_session); pub fn iscsi_block_session(*mut iscsi_cls_session); pub fn iscsi_create_endpoint(c_int) -> *mut iscsi_endpoint; pub fn iscsi_destroy_endpoint(*mut iscsi_endpoint); pub fn iscsi_lookup_endpoint(u64) -> *mut iscsi_endpoint; pub fn iscsi_put_endpoint(*mut iscsi_endpoint); pub fn iscsi_block_scsi_eh(*mut scsi_cmnd) -> c_int;
    pub fn iscsi_create_iface(*mut Scsi_Host, *mut iscsi_transport, u32, u32, c_int) -> *mut iscsi_iface; pub fn iscsi_destroy_iface(*mut iscsi_iface); pub fn iscsi_get_port_speed_name(*mut Scsi_Host) -> *mut c_char; pub fn iscsi_get_port_state_name(*mut Scsi_Host) -> *mut c_char; pub fn iscsi_is_session_dev(*const device) -> c_int; pub fn iscsi_get_discovery_parent_name(c_int) -> *mut c_char;
    pub fn iscsi_destroy_flashnode_sess(*mut iscsi_bus_flash_session); pub fn iscsi_destroy_all_flashnode(*mut Scsi_Host); pub fn iscsi_flashnode_bus_match(*mut device, *const device_driver) -> c_int; pub fn iscsi_find_flashnode_sess(*mut Scsi_Host, *const c_void, device_match_t) -> *mut device; pub fn iscsi_find_flashnode_conn(*mut iscsi_bus_flash_session) -> *mut device; pub fn iscsi_get_ipaddress_state_name(iscsi_ipaddress_state) -> *mut c_char; pub fn iscsi_get_router_state_name(iscsi_router_state) -> *mut c_char;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
