/* SPDX-License-Identifier: GPL-2.0 */
/* Thunderbolt service API translation. */

/* External kernel types supplied by other headers. */
#[repr(C)] pub struct config_group { _private: [u8; 0] }
#[repr(C)] pub struct fwnode_handle { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct tb_ctl { _private: [u8; 0] }
#[repr(C)] pub struct tb_switch { _private: [u8; 0] }
#[repr(C)] pub struct tb_cm_ops { _private: [u8; 0] }
#[repr(C)] pub struct bus_type { _private: [u8; 0] }
#[repr(C)] pub struct device_type { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct ida { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct ring_desc { _private: [u8; 0] }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct uuid_t { _private: [u8; 0] }
pub type dma_addr_t = u64;
pub type ssize_t = isize;

#[repr(i32)]
pub enum tb_cfg_pkg_type {
    TB_CFG_PKG_READ = 1, TB_CFG_PKG_WRITE, TB_CFG_PKG_ERROR,
    TB_CFG_PKG_NOTIFY_ACK, TB_CFG_PKG_EVENT, TB_CFG_PKG_XDOMAIN_REQ,
    TB_CFG_PKG_XDOMAIN_RESP, TB_CFG_PKG_OVERRIDE, TB_CFG_PKG_RESET,
    TB_CFG_PKG_ICM_EVENT, TB_CFG_PKG_ICM_CMD, TB_CFG_PKG_ICM_RESP,
}

#[repr(i32)]
pub enum tb_security_level { TB_SECURITY_NONE, TB_SECURITY_USER, TB_SECURITY_SECURE, TB_SECURITY_DPONLY, TB_SECURITY_USBONLY, TB_SECURITY_NOPCIE }

#[repr(C)]
pub struct tb {
    pub dev: device, pub lock: mutex, pub nhi: *mut tb_nhi, pub ctl: *mut tb_ctl,
    pub wq: *mut workqueue_struct, pub root_switch: *mut tb_switch,
    pub cm_ops: *const tb_cm_ops, pub index: i32, pub security_level: tb_security_level,
    pub nboot_acl: usize, pub privdata: [usize; 0],
}

extern "C" {
    pub static tb_bus_type: bus_type;
    pub static tb_service_type: device_type;
    pub static tb_xdomain_type: device_type;
}

pub const TB_LINKS_PER_PHY_PORT: u32 = 2;
#[inline] pub fn tb_phy_port_from_link(link: u32) -> u32 { (link.wrapping_sub(1)) / TB_LINKS_PER_PHY_PORT }

#[repr(C)] pub struct tb_property_dir { pub uuid: *const uuid_t, pub properties: list_head }
#[repr(i32)] pub enum tb_property_type { TB_PROPERTY_TYPE_UNKNOWN=0x00, TB_PROPERTY_TYPE_DIRECTORY=0x44, TB_PROPERTY_TYPE_DATA=0x64, TB_PROPERTY_TYPE_TEXT=0x74, TB_PROPERTY_TYPE_VALUE=0x76 }
pub const TB_PROPERTY_KEY_SIZE: usize = 8;
#[repr(C)] pub union tb_property_value { pub dir: *mut tb_property_dir, pub data: *mut u8, pub text: *mut i8, pub immediate: u32 }
#[repr(C)] pub struct tb_property { pub list: list_head, pub key: [i8; TB_PROPERTY_KEY_SIZE + 1], pub type_: tb_property_type, pub length: usize, pub value: tb_property_value }

extern "C" {
    pub fn tb_property_parse_dir(block: *const u32, block_len: usize) -> *mut tb_property_dir;
    pub fn tb_property_format_dir(dir: *const tb_property_dir, block: *mut u32, block_len: usize) -> ssize_t;
    pub fn tb_property_copy_dir(dir: *const tb_property_dir) -> *mut tb_property_dir;
    pub fn tb_property_merge_dir(parent: *mut tb_property_dir, dir: *const tb_property_dir, replace: bool) -> i32;
    pub fn tb_property_create_dir(uuid: *const uuid_t) -> *mut tb_property_dir;
    pub fn tb_property_free_dir(dir: *mut tb_property_dir);
    pub fn tb_property_add_immediate(parent: *mut tb_property_dir, key: *const i8, value: u32) -> i32;
    pub fn tb_property_add_data(parent: *mut tb_property_dir, key: *const i8, buf: *const core::ffi::c_void, buflen: usize) -> i32;
    pub fn tb_property_add_text(parent: *mut tb_property_dir, key: *const i8, text: *const i8) -> i32;
    pub fn tb_property_add_dir(parent: *mut tb_property_dir, key: *const i8, dir: *mut tb_property_dir) -> i32;
    pub fn tb_property_remove(property: *mut tb_property);
    pub fn tb_property_find(dir: *mut tb_property_dir, key: *const i8, type_: tb_property_type) -> *mut tb_property;
    pub fn tb_property_get_next(dir: *mut tb_property_dir, prev: *mut tb_property) -> *mut tb_property;
    pub fn tb_register_property_dir(key: *const i8, dir: *mut tb_property_dir) -> i32;
    pub fn tb_unregister_property_dir(key: *const i8, dir: *mut tb_property_dir);
}

#[repr(i32)] pub enum tb_link_width { TB_LINK_WIDTH_SINGLE=1, TB_LINK_WIDTH_DUAL=2, TB_LINK_WIDTH_ASYM_TX=4, TB_LINK_WIDTH_ASYM_RX=8 }
#[repr(C)] pub struct tb_xdomain { pub dev: device, pub tb: *mut tb, pub remote_uuid: *mut uuid_t, pub local_uuid: *const uuid_t, pub route: u64, pub vendor: u16, pub device: u16, pub local_max_hopid: u32, pub remote_max_hopid: u32, pub lock: mutex, pub vendor_name: *const i8, pub device_name: *const i8, pub link_speed: u32, pub link_width: tb_link_width, pub link_usb4: bool, pub is_unplugged: bool, pub removing: bool, pub needs_uuid: bool, pub service_ids: ida, pub in_hopids: ida, pub out_hopids: ida, pub local_property_block: *mut u32, pub local_property_block_gen: u32, pub local_property_block_len: u32, pub remote_properties: *mut tb_property_dir, pub remote_property_block_gen: u32, pub state: i32, pub state_work: delayed_work, pub state_retries: i32, pub properties_changed_work: delayed_work, pub properties_changed_retries: i32, pub bonding_possible: bool, pub target_link_width: u8, pub ntunnels: atomic_t, pub link: u8, pub depth: u8 }

extern "C" {
    pub fn tb_xdomain_lane_bonding_enable(xd: *mut tb_xdomain) -> i32; pub fn tb_xdomain_lane_bonding_disable(xd: *mut tb_xdomain);
    pub fn tb_xdomain_alloc_in_hopid(xd: *mut tb_xdomain, hopid: i32) -> i32; pub fn tb_xdomain_release_in_hopid(xd: *mut tb_xdomain, hopid: i32);
    pub fn tb_xdomain_alloc_out_hopid(xd: *mut tb_xdomain, hopid: i32) -> i32; pub fn tb_xdomain_release_out_hopid(xd: *mut tb_xdomain, hopid: i32);
    pub fn tb_xdomain_enable_paths(xd: *mut tb_xdomain, transmit_path:i32, transmit_ring:i32, receive_path:i32, receive_ring:i32) -> i32;
    pub fn tb_xdomain_disable_paths(xd: *mut tb_xdomain, transmit_path:i32, transmit_ring:i32, receive_path:i32, receive_ring:i32) -> i32;
    pub fn tb_xdomain_find_by_uuid(tb: *mut tb, uuid: *const uuid_t) -> *mut tb_xdomain; pub fn tb_xdomain_find_by_route(tb: *mut tb, route:u64) -> *mut tb_xdomain;
    pub fn tb_xdomain_response(xd:*mut tb_xdomain,response:*const core::ffi::c_void,size:usize,type_:tb_cfg_pkg_type)->i32;
    pub fn tb_xdomain_request(xd:*mut tb_xdomain,request:*const core::ffi::c_void,request_size:usize,request_type:tb_cfg_pkg_type,response:*mut core::ffi::c_void,response_size:usize,response_type:tb_cfg_pkg_type,timeout_msec:u32)->i32;
}
#[inline] pub unsafe fn tb_xdomain_disable_all_paths(xd:*mut tb_xdomain)->i32 { tb_xdomain_disable_paths(xd,-1,-1,-1,-1) }
#[inline] pub unsafe fn tb_xdomain_get(xd:*mut tb_xdomain)->*mut tb_xdomain { xd }
#[inline] pub unsafe fn tb_xdomain_put(_xd:*mut tb_xdomain) {}
#[inline] pub unsafe fn tb_is_xdomain(_dev:*const device)->bool { false }
#[inline] pub unsafe fn tb_to_xdomain(_dev:*mut device)->*mut tb_xdomain { core::ptr::null_mut() }

#[repr(C)] pub struct tb_protocol_handler { pub uuid:*const uuid_t, pub callback: Option<unsafe extern "C" fn(*const core::ffi::c_void,usize,*mut core::ffi::c_void)->i32>, pub data:*mut core::ffi::c_void, pub list:list_head }
extern "C" { pub fn tb_register_protocol_handler(handler:*mut tb_protocol_handler)->i32; pub fn tb_unregister_protocol_handler(handler:*mut tb_protocol_handler); }

#[repr(C)] pub struct tb_service { pub dev:device, pub id:i32, pub key:*const i8, pub prtcid:u32, pub prtcvers:u32, pub prtcrevs:u32, pub prtcstns:u32, pub lock:mutex, pub local_properties:*mut tb_property_dir, pub remote_properties:*mut tb_property_dir, pub debugfs_dir:*mut dentry }
#[repr(C)] pub struct tb_service_driver { pub driver:device_driver, pub probe:Option<unsafe extern "C" fn(*mut tb_service)->i32>, pub remove:Option<unsafe extern "C" fn(*mut tb_service)>, pub shutdown:Option<unsafe extern "C" fn(*mut tb_service)>, pub id_table:*const tb_service_id }
#[repr(C)] pub struct tb_service_id { _private:[u8;0] }
extern "C" { pub fn tb_register_service_driver(drv:*mut tb_service_driver)->i32; pub fn tb_unregister_service_driver(drv:*mut tb_service_driver); pub fn dev_get_drvdata(dev:*const device)->*mut core::ffi::c_void; pub fn dev_set_drvdata(dev:*mut device,data:*mut core::ffi::c_void); pub fn tb_service_properties_changed(svc:*mut tb_service); }
#[inline] pub unsafe fn tb_service_get(svc:*mut tb_service)->*mut tb_service { svc }
#[inline] pub unsafe fn tb_service_put(_svc:*mut tb_service) {}
#[inline] pub unsafe fn tb_is_service(_dev:*const device)->bool { false }
#[inline] pub unsafe fn tb_to_service(_dev:*mut device)->*mut tb_service { core::ptr::null_mut() }
#[inline] pub unsafe fn tb_service_get_drvdata(svc:*const tb_service)->*mut core::ffi::c_void { dev_get_drvdata(&(*svc).dev) }
#[inline] pub unsafe fn tb_service_set_drvdata(svc:*mut tb_service,data:*mut core::ffi::c_void) { dev_set_drvdata(&mut (*svc).dev,data) }
#[inline] pub unsafe fn tb_service_parent(_svc:*mut tb_service)->*mut tb_xdomain { core::ptr::null_mut() }

#[repr(C)] pub struct tb_nhi { pub lock:spinlock_t, pub dev:*mut device, pub ops:*const core::ffi::c_void, pub iobase:*mut core::ffi::c_void, pub tx_rings:*mut *mut tb_ring, pub rx_rings:*mut *mut tb_ring, pub going_away:bool, pub iommu_dma_protection:bool, pub interrupt_work:work_struct, pub hop_count:u32, pub quirks:usize, pub domain_released:completion, pub host_reset:bool }

#[repr(C)] pub struct tb_ring { pub lock:spinlock_t, pub nhi:*mut tb_nhi, pub size:i32, pub hop:i32, pub head:i32, pub tail:i32, pub descriptors:*mut ring_desc, pub descriptors_dma:dma_addr_t, pub queue:list_head, pub in_flight:list_head, pub work:work_struct, pub is_tx:bool, pub running:bool, pub irq:i32, pub vector:u8, pub flags:u32, pub e2e_tx_hop:i32, pub sof_mask:u16, pub eof_mask:u16, pub start_poll:Option<unsafe extern "C" fn(*mut core::ffi::c_void)>, pub poll_data:*mut core::ffi::c_void, pub interval_nsec:u32, pub wait:wait_queue_head_t }
#[repr(C)] pub struct ring_frame { pub buffer_phy:dma_addr_t, pub callback:Option<unsafe extern "C" fn(*mut tb_ring,*mut ring_frame,bool)>, pub list:list_head, pub size:u32, pub flags:u32, pub eof:u32, pub sof:u32 }
pub const RING_FLAG_NO_SUSPEND:u32=1; pub const RING_FLAG_FRAME:u32=2; pub const RING_FLAG_E2E:u32=4; pub const RING_FLAG_NO_INTERRUPT:u32=8;
pub const TB_FRAME_SIZE:usize=256; pub const TB_MAX_FRAME_SIZE:usize=4096;
pub const RING_DESC_ISOCH:u32=1; pub const RING_DESC_CRC_ERROR:u32=1; pub const RING_DESC_COMPLETED:u32=2; pub const RING_DESC_POSTED:u32=4; pub const RING_DESC_BUFFER_OVERRUN:u32=4; pub const RING_DESC_INTERRUPT:u32=8;
#[inline] pub unsafe fn tb_ring_frame_size(frame:*const ring_frame)->usize { if (*frame).size != 0 { (*frame).size as usize } else { TB_MAX_FRAME_SIZE } }
#[inline] pub unsafe fn tb_ring_size(ring:*const tb_ring)->usize { (*ring).size as usize }
extern "C" { pub fn tb_ring_alloc_tx(nhi:*mut tb_nhi,hop:i32,size:i32,flags:u32)->*mut tb_ring; pub fn tb_ring_alloc_rx(nhi:*mut tb_nhi,hop:i32,size:i32,flags:u32,e2e_tx_hop:i32,sof_mask:u16,eof_mask:u16,start_poll:Option<unsafe extern "C" fn(*mut core::ffi::c_void)>,poll_data:*mut core::ffi::c_void)->*mut tb_ring; pub fn tb_ring_start(ring:*mut tb_ring); pub fn tb_ring_flush(ring:*mut tb_ring,timeout_msec:u32)->bool; pub fn tb_ring_stop(ring:*mut tb_ring); pub fn tb_ring_free(ring:*mut tb_ring); pub fn __tb_ring_enqueue(ring:*mut tb_ring,frame:*mut ring_frame)->i32; pub fn tb_ring_poll(ring:*mut tb_ring)->*mut ring_frame; pub fn tb_ring_poll_complete(ring:*mut tb_ring); pub fn tb_ring_throttling(ring:*mut tb_ring,interval_nsec:u32)->i32; pub fn usb4_usb3_port_match(usb4_port_dev:*mut device,usb3_port_fwnode:*const fwnode_handle)->bool }
#[inline] pub unsafe fn tb_ring_rx(ring:*mut tb_ring,frame:*mut ring_frame)->i32 { __tb_ring_enqueue(ring,frame) }
#[inline] pub unsafe fn tb_ring_tx(ring:*mut tb_ring,frame:*mut ring_frame)->i32 { __tb_ring_enqueue(ring,frame) }
#[inline] pub unsafe fn tb_ring_dma_device(ring:*mut tb_ring)->*mut device { (*(*ring).nhi).dev }

extern "C" {
    pub fn tb_configfs_register_group(group:*mut config_group)->i32;
    pub fn tb_configfs_unregister_group(group:*mut config_group);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
