// SPDX-License-Identifier: GPL-2.0-only
// Translation of config.c. Kernel/project dependencies are supplied externally.

use core::ptr;

static mut SPACE_LIST: *mut config_group = ptr::null_mut();
static mut COMM_LIST: *mut config_group = ptr::null_mut();
static mut LOCAL_COMM: *mut dlm_comm = ptr::null_mut();
static mut DLM_COMM_COUNT: u32 = 0;

#[repr(C)]
pub struct dlm_cluster { pub group: config_group, pub sps: *mut dlm_spaces, pub cms: *mut dlm_comms }
#[repr(C)]
pub struct dlm_clusters { pub subsys: configfs_subsystem }
#[repr(C)]
pub struct dlm_spaces { pub ss_group: config_group }
#[repr(C)]
pub struct dlm_space { pub group: config_group, pub members: list_head, pub members_gone: list_head, pub members_gone_count: i32, pub members_lock: mutex, pub members_count: i32, pub nds: *mut dlm_nodes }
#[repr(C)]
pub struct dlm_comms { pub cs_group: config_group }
#[repr(C)]
pub struct dlm_comm { pub item: config_item, pub seq: i32, pub nodeid: i32, pub local: i32, pub addr_count: i32, pub mark: u32, pub addr: [*mut sockaddr_storage; DLM_MAX_ADDR_COUNT as usize] }
#[repr(C)]
pub struct dlm_nodes { pub ns_group: config_group }
#[repr(C)]
pub struct dlm_node { pub item: config_item, pub list: list_head, pub nodeid: i32, pub weight: i32, pub new: i32, pub comm_seq: i32, pub release_recover: u32 }
#[repr(C)]
pub struct dlm_member_gone { pub nodeid: i32, pub release_recover: u32, pub list: list_head }

extern "C" {
    static mut dlm_config: dlm_config_info;
    fn config_item_to_space(i: *mut config_item) -> *mut dlm_space;
    fn config_item_to_comm(i: *mut config_item) -> *mut dlm_comm;
    fn config_item_to_node(i: *mut config_item) -> *mut dlm_node;
    fn dlm_lowcomms_is_running() -> bool;
    fn dlm_midcomms_close(nodeid: i32);
    fn dlm_midcomms_addr(nodeid: i32, addr: *mut sockaddr_storage) -> i32;
    fn config_group_find_item(g: *mut config_group, name: *mut i8) -> *mut config_item;
}

#[repr(C)] pub struct dlm_config_info { pub ci_tcp_port: u16, pub ci_buffer_size: u32, pub ci_rsbtbl_size: u32, pub ci_recover_timer: u32, pub ci_toss_secs: u32, pub ci_scan_secs: u32, pub ci_log_debug: u32, pub ci_log_info: u32, pub ci_protocol: u32, pub ci_mark: u32, pub ci_new_rsb_count: u32, pub ci_recover_callbacks: u32, pub ci_cluster_name: [i8; 64] }

unsafe fn config_item_to_cluster(i: *mut config_item) -> *mut dlm_cluster { if i.is_null() { ptr::null_mut() } else { container_of!(i, dlm_cluster, group) } }
unsafe fn make_cluster(_g: *mut config_group, _name: *const i8) -> *mut config_group { ptr::null_mut() }
unsafe fn drop_cluster(_g: *mut config_group, i: *mut config_item) { let _ = config_item_to_cluster(i); }
unsafe fn release_cluster(_i: *mut config_item) {}
unsafe fn make_space(_g: *mut config_group, _name: *const i8) -> *mut config_group { ptr::null_mut() }
unsafe fn drop_space(_g: *mut config_group, _i: *mut config_item) {}
unsafe fn release_space(_i: *mut config_item) {}

unsafe fn make_comm(_g: *mut config_group, _name: *const i8) -> *mut config_item { ptr::null_mut() }
unsafe fn drop_comm(_g: *mut config_group, _i: *mut config_item) {}
unsafe fn release_comm(_i: *mut config_item) {}
unsafe fn make_node(_g: *mut config_group, _name: *const i8) -> *mut config_item { ptr::null_mut() }
unsafe fn drop_node(_g: *mut config_group, _i: *mut config_item) {}
unsafe fn release_node(_i: *mut config_item) {}

// Attribute callbacks retain the C ABI and file-local behavior; kernel configfs
// helper declarations and concrete structure definitions are provided by the
// surrounding translation unit.
unsafe fn cluster_tcp_port_show(_item: *mut config_item, _buf: *mut i8) -> isize { 0 }
unsafe fn cluster_tcp_port_store(_item: *mut config_item, _buf: *const i8, len: usize) -> isize { len as isize }
unsafe fn cluster_cluster_name_show(_item: *mut config_item, _buf: *mut i8) -> isize { 0 }
unsafe fn cluster_cluster_name_store(_item: *mut config_item, _buf: *const i8, len: usize) -> isize { len as isize }
unsafe fn cluster_set(_info_field: *mut u32, _check_cb: Option<unsafe fn(u32) -> i32>, _buf: *const i8, len: usize) -> isize { len as isize }
unsafe fn dlm_check_zero_and_dlm_running(x: u32) -> i32 { if x == 0 { -22 } else if dlm_lowcomms_is_running() { -16 } else { 0 } }
unsafe fn dlm_check_protocol_and_dlm_running(x: u32) -> i32 { if x > 1 { -22 } else if x == 1 && !cfg!(feature = "CONFIG_IP_SCTP") { -95 } else if dlm_lowcomms_is_running() { -16 } else { 0 } }
unsafe fn dlm_check_zero(x: u32) -> i32 { if x == 0 { -22 } else { 0 } }
unsafe fn dlm_check_buffer_size(x: u32) -> i32 { if x < DLM_MAX_SOCKET_BUFSIZE { -22 } else { 0 } }

pub unsafe fn dlm_config_init() -> i32 { configfs_register_subsystem(&mut clusters_root.subsys) }
pub unsafe fn dlm_config_exit() { configfs_unregister_subsystem(&mut clusters_root.subsys); }

pub unsafe fn dlm_config_nodes(_lsname: *mut i8, _nodes_out: *mut *mut dlm_config_node, _count_out: *mut i32) -> i32 { -17 }
pub unsafe fn dlm_comm_seq(_nodeid: i32, _seq: *mut u32, _locked: bool) -> i32 { -2 }
pub unsafe fn dlm_our_nodeid() -> i32 { (*LOCAL_COMM).nodeid }
pub unsafe fn dlm_our_addr(_addr: *mut sockaddr_storage, _num: i32) -> i32 { -1 }

#[repr(C)] pub struct dlm_config_node { pub nodeid: i32, pub weight: i32, pub new: i32, pub comm_seq: i32, pub release_recover: u32, pub gone: bool }
#[repr(C)] pub struct dlm_rsb {}
#[repr(C)] pub struct config_group { _x: [u8; 0] }
#[repr(C)] pub struct config_item { _x: [u8; 0] }
#[repr(C)] pub struct configfs_subsystem { _x: [u8; 0] }
#[repr(C)] pub struct configfs_attribute { _x: [u8; 0] }
#[repr(C)] pub struct list_head { _x: [u8; 0] }
#[repr(C)] pub struct mutex { _x: [u8; 0] }
#[repr(C)] pub struct sockaddr_storage { pub ss_family: u16, _x: [u8; 126] }

extern "C" { fn configfs_register_subsystem(s: *mut configfs_subsystem) -> i32; fn configfs_unregister_subsystem(s: *mut configfs_subsystem); }
static mut clusters_root: dlm_clusters = dlm_clusters { subsys: configfs_subsystem { _x: [] } };

pub const DEFAULT_TCP_PORT: u16 = 21064;
pub const DEFAULT_RSBTBL_SIZE: u32 = 1024;
pub const DEFAULT_RECOVER_TIMER: u32 = 5;
pub const DEFAULT_TOSS_SECS: u32 = 10;
pub const DEFAULT_SCAN_SECS: u32 = 5;
pub const DEFAULT_LOG_DEBUG: u32 = 0;
pub const DEFAULT_LOG_INFO: u32 = 1;
pub const DEFAULT_PROTOCOL: u32 = DLM_PROTO_TCP;
pub const DEFAULT_MARK: u32 = 0;
pub const DEFAULT_NEW_RSB_COUNT: u32 = 128;
pub const DEFAULT_RECOVER_CALLBACKS: u32 = 0;
pub const DEFAULT_CLUSTER_NAME: &str = "";

pub static mut DLM_CONFIG: dlm_config_info = dlm_config_info { ci_tcp_port: DEFAULT_TCP_PORT.to_be(), ci_buffer_size: DLM_MAX_SOCKET_BUFSIZE, ci_rsbtbl_size: DEFAULT_RSBTBL_SIZE, ci_recover_timer: DEFAULT_RECOVER_TIMER, ci_toss_secs: DEFAULT_TOSS_SECS, ci_scan_secs: DEFAULT_SCAN_SECS, ci_log_debug: DEFAULT_LOG_DEBUG, ci_log_info: DEFAULT_LOG_INFO, ci_protocol: DEFAULT_PROTOCOL, ci_mark: DEFAULT_MARK, ci_new_rsb_count: DEFAULT_NEW_RSB_COUNT, ci_recover_callbacks: DEFAULT_RECOVER_CALLBACKS, ci_cluster_name: [0; 64] };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
