/* SPDX-License-Identifier: GPL-2.0 */

// Linux dependencies and build-time configuration are supplied by the surrounding crate.

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

pub enum device {}
pub enum fw_card {}
pub enum fw_device {}
pub enum fw_iso_buffer {}
pub enum fw_iso_context {}
pub enum fw_iso_packet {}
pub enum fw_packet {}
pub enum fw_request {}
pub enum file_operations {}
pub enum rw_semaphore {}
pub enum xarray {}
pub enum kref {}
pub enum list_head {}
pub type dev_t = u64;
pub type dma_addr_t = u64;
pub type dma_data_direction = c_int;
pub type work_func_t = unsafe extern "C" fn(*mut work_struct);
pub enum work_struct {}
pub type fw_iso_mc_callback_t = unsafe extern "C" fn(*mut fw_iso_context, *mut c_void);
pub type __be32 = u32;

pub const GAP_COUNT_MISMATCHED: c_uint = 0;
pub const PHY_LINK_ACTIVE: c_uint = 0x80;
pub const PHY_CONTENDER: c_uint = 0x40;
pub const PHY_BUS_RESET: c_uint = 0x40;
pub const PHY_EXTENDED_REGISTERS: c_uint = 0xe0;
pub const PHY_BUS_SHORT_RESET: c_uint = 0x40;
pub const PHY_INT_STATUS_BITS: c_uint = 0x3c;
pub const PHY_ENABLE_ACCEL: c_uint = 0x02;
pub const PHY_ENABLE_MULTI: c_uint = 0x01;
pub const PHY_PAGE_SELECT: c_uint = 0xe0;
pub const BANDWIDTH_AVAILABLE_INITIAL: c_uint = 4915;
pub const BROADCAST_CHANNEL_INITIAL: u32 = (1 << 31) | 31;
pub const BROADCAST_CHANNEL_VALID: u32 = 1 << 30;
pub const CSR_STATE_BIT_CMSTR: c_uint = 1 << 8;
pub const CSR_STATE_BIT_ABDICATE: c_uint = 1 << 10;

#[repr(C)]
pub struct fw_card_driver {
    pub enable: Option<unsafe extern "C" fn(*mut fw_card, *const __be32, usize) -> c_int>,
    pub disable: Option<unsafe extern "C" fn(*mut fw_card)>,
    pub read_phy_reg: Option<unsafe extern "C" fn(*mut fw_card, c_int) -> c_int>,
    pub update_phy_reg: Option<unsafe extern "C" fn(*mut fw_card, c_int, c_int, c_int) -> c_int>,
    pub set_config_rom: Option<unsafe extern "C" fn(*mut fw_card, *const __be32, usize) -> c_int>,
    pub send_request: Option<unsafe extern "C" fn(*mut fw_card, *mut fw_packet)>,
    pub send_response: Option<unsafe extern "C" fn(*mut fw_card, *mut fw_packet)>,
    pub cancel_packet: Option<unsafe extern "C" fn(*mut fw_card, *mut fw_packet) -> c_int>,
    pub enable_phys_dma: Option<unsafe extern "C" fn(*mut fw_card, c_int, c_int) -> c_int>,
    pub read_csr: Option<unsafe extern "C" fn(*mut fw_card, c_int) -> u32>,
    pub write_csr: Option<unsafe extern "C" fn(*mut fw_card, c_int, u32)>,
    pub allocate_iso_context: Option<unsafe extern "C" fn(*mut fw_card, c_int, c_int, usize, usize) -> *mut fw_iso_context>,
    pub free_iso_context: Option<unsafe extern "C" fn(*mut fw_iso_context)>,
    pub start_iso: Option<unsafe extern "C" fn(*mut fw_iso_context, i32, u32, u32) -> c_int>,
    pub set_iso_channels: Option<unsafe extern "C" fn(*mut fw_iso_context, *mut u64) -> c_int>,
    pub queue_iso: Option<unsafe extern "C" fn(*mut fw_iso_context, *mut fw_iso_packet, *mut fw_iso_buffer, c_ulong) -> c_int>,
    pub flush_queue_iso: Option<unsafe extern "C" fn(*mut fw_iso_context)>,
    pub flush_iso_completions: Option<unsafe extern "C" fn(*mut fw_iso_context) -> c_int>,
    pub stop_iso: Option<unsafe extern "C" fn(*mut fw_iso_context) -> c_int>,
}

extern "C" {
    pub fn fw_err(card: *const fw_card, fmt: *const c_char, ...);
    pub fn fw_notice(card: *const fw_card, fmt: *const c_char, ...);
    pub fn fw_card_initialize(card: *mut fw_card, driver: *const fw_card_driver, device: *mut device);
    pub fn fw_card_add(card: *mut fw_card, max_receive: u32, link_speed: u32, guid: u64, supported_isoc_contexts: c_uint) -> c_int;
    pub fn fw_core_remove_card(card: *mut fw_card);
    pub fn fw_compute_block_crc(block: *mut __be32) -> c_int;
    pub fn fw_schedule_bm_work(card: *mut fw_card, delay: c_ulong);
    pub static fw_device_ops: file_operations;
    pub fn fw_device_cdev_update(device: *mut fw_device);
    pub fn fw_device_cdev_remove(device: *mut fw_device);
    pub fn fw_cdev_handle_phy_packet(card: *mut fw_card, p: *mut fw_packet);
    pub static mut fw_device_rwsem: rw_semaphore;
    pub static mut fw_device_xa: xarray;
    pub static mut fw_cdev_major: c_int;
    pub fn fw_device_get_by_devt(devt: dev_t) -> *mut fw_device;
    pub fn fw_device_set_broadcast_channel(dev: *mut device, gen: *mut c_void) -> c_int;
    pub fn fw_node_event(card: *mut fw_card, node: *mut fw_node, event: c_int);
    pub fn fw_iso_buffer_alloc(buffer: *mut fw_iso_buffer, page_count: c_int) -> c_int;
    pub fn fw_iso_buffer_map_dma(buffer: *mut fw_iso_buffer, card: *mut fw_card, direction: dma_data_direction) -> c_int;
    pub fn fw_iso_buffer_lookup(buffer: *mut fw_iso_buffer, completed: dma_addr_t) -> usize;
    pub fn fw_core_handle_bus_reset(card: *mut fw_card, node_id: c_int, generation: c_int, self_id_count: c_int, self_ids: *mut u32, bm_abdicate: bool);
    pub fn fw_destroy_nodes(card: *mut fw_card);
    pub fn fw_core_handle_request(card: *mut fw_card, request: *mut fw_packet);
    pub fn fw_core_handle_response(card: *mut fw_card, packet: *mut fw_packet);
    pub fn fw_get_response_length(request: *mut fw_request) -> c_int;
    pub fn fw_fill_response(response: *mut fw_packet, request_header: *mut u32, rcode: c_int, payload: *mut c_void, length: usize);
    pub fn fw_request_get(request: *mut fw_request);
    pub fn fw_request_put(request: *mut fw_request);
    pub fn fw_cancel_pending_transactions(card: *mut fw_card);
    pub fn fw_send_phy_config(card: *mut fw_card, node_id: c_int, generation: c_int, gap_count: c_int);
}

#[repr(C)]
pub struct fw_node {
    pub node_id: u16, pub color: u8, pub port_count: u8,
    pub link_on: u8, pub initiated_reset: u8, pub b_path: u8,
    pub phy_speed: u8, pub max_speed: u8, pub max_depth: u8, pub max_hops: u8,
    pub kref: kref, pub link: list_head, pub device: *mut fw_device,
    pub ports: [*mut fw_node; 0],
}

pub const FW_NODE_CREATED: c_int = 0;
pub const FW_NODE_UPDATED: c_int = 1;
pub const FW_NODE_DESTROYED: c_int = 2;
pub const FW_NODE_LINK_ON: c_int = 3;
pub const FW_NODE_LINK_OFF: c_int = 4;
pub const FW_NODE_INITIATED_RESET: c_int = 5;

extern "C" {
    pub fn kref_get(kref: *mut kref);
    pub fn kref_put(kref: *mut kref, release: unsafe extern "C" fn(*mut kref)) -> bool;
    pub fn kfree(ptr: *mut c_void);
    pub fn __fw_iso_context_init_work(ctx: *mut fw_iso_context, func: work_func_t);
    pub fn __fw_iso_context_create(card: *mut fw_card, context_type: c_int, channel: c_int, speed: c_int, header_size: usize, header_storage_size: usize, callback: *mut c_void, callback_data: *mut c_void) -> *mut fw_iso_context;
}

pub unsafe fn fw_node_get(node: *mut fw_node) -> *mut fw_node { kref_get(&mut (*node).kref); node }
pub unsafe extern "C" fn release_node(kref_ptr: *mut kref) {
    let node = (kref_ptr as *mut u8).sub(core::mem::offset_of!(fw_node, kref)) as *mut fw_node;
    kfree(node.cast());
}
pub unsafe fn fw_node_put(node: *mut fw_node) { let _ = kref_put(&mut (*node).kref, release_node); }
pub unsafe fn fw_node_get_device(node: *mut fw_node) -> *mut fw_device { (*node).device }
pub unsafe fn fw_node_set_device(node: *mut fw_node, device: *mut fw_device) { (*node).device = device; }

pub unsafe fn fw_iso_context_init_work(ctx: *mut fw_iso_context, func: work_func_t) { __fw_iso_context_init_work(ctx, func); }
pub const FW_ISO_CONTEXT_RECEIVE_MULTICHANNEL: c_int = 3;
pub unsafe fn fw_iso_mc_context_create(card: *mut fw_card, callback: fw_iso_mc_callback_t, callback_data: *mut c_void) -> *mut fw_iso_context {
    __fw_iso_context_create(card, FW_ISO_CONTEXT_RECEIVE_MULTICHANNEL, 0, 0, 0, 0, callback as *mut c_void, callback_data)
}

pub const BUS_MANAGER_ID_NOT_REGISTERED: c_uint = 0x3f;
pub const TCODE_LINK_INTERNAL: c_uint = 0xe;
pub const LOCAL_BUS: u32 = 0xffc0;
pub const FW_MAX_PHYSICAL_RANGE: u64 = 1u64 << 32;
pub const FW_PHY_CONFIG_NO_NODE_ID: c_int = -1;
pub const FW_PHY_CONFIG_CURRENT_GAP_COUNT: c_int = -1;

pub unsafe fn is_next_generation(new_generation: c_int, old_generation: c_int) -> bool { (new_generation & 0xff) == ((old_generation + 1) & 0xff) }
pub fn tcode_is_read_request(tcode: c_uint) -> bool { (tcode & !1) == 4 }
pub fn tcode_is_block_packet(tcode: c_uint) -> bool { (tcode & 1) != 0 }
pub fn tcode_is_link_internal(tcode: c_uint) -> bool { tcode == TCODE_LINK_INTERNAL }
pub fn cycle_time_to_ohci_tstamp(tstamp: u32) -> u32 { (tstamp & 0x0ffff000) >> 12 }
pub unsafe fn is_ping_packet(data: *mut u32) -> bool { (*data & 0xc0ffffff) == 0 && !*data == *data.add(1) }
pub fn is_in_fcp_region(offset: u64, length: usize) -> bool { offset >= (CSR_REGISTER_BASE | CSR_FCP_COMMAND) && offset + length as u64 <= (CSR_REGISTER_BASE | CSR_FCP_END) }

// Symbols below are supplied by the surrounding FireWire translation.
extern "C" { pub static CSR_REGISTER_BASE: u64; pub static CSR_FCP_COMMAND: u64; pub static CSR_FCP_END: u64; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
