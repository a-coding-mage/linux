/* SPDX-License-Identifier: GPL-2.0 */
// Translated from linux/firewire.h.  Types and functions supplied by included
// kernel headers are intentionally referenced but not defined here.

pub const CSR_REGISTER_BASE: u64 = 0xfffff0000000;
pub const CSR_STATE_CLEAR: u32 = 0x0;
pub const CSR_STATE_SET: u32 = 0x4;
pub const CSR_NODE_IDS: u32 = 0x8;
pub const CSR_RESET_START: u32 = 0xc;
pub const CSR_SPLIT_TIMEOUT_HI: u32 = 0x18;
pub const CSR_SPLIT_TIMEOUT_LO: u32 = 0x1c;
pub const CSR_CYCLE_TIME: u32 = 0x200;
pub const CSR_BUS_TIME: u32 = 0x204;
pub const CSR_BUSY_TIMEOUT: u32 = 0x210;
pub const CSR_PRIORITY_BUDGET: u32 = 0x218;
pub const CSR_BUS_MANAGER_ID: u32 = 0x21c;
pub const CSR_BANDWIDTH_AVAILABLE: u32 = 0x220;
pub const CSR_CHANNELS_AVAILABLE: u32 = 0x224;
pub const CSR_CHANNELS_AVAILABLE_HI: u32 = 0x224;
pub const CSR_CHANNELS_AVAILABLE_LO: u32 = 0x228;
pub const CSR_MAINT_UTILITY: u32 = 0x230;
pub const CSR_BROADCAST_CHANNEL: u32 = 0x234;
pub const CSR_CONFIG_ROM: u32 = 0x400;
pub const CSR_CONFIG_ROM_END: u32 = 0x800;
pub const CSR_OMPR: u32 = 0x900;
pub const CSR_IMPR: u32 = 0x980;
pub const CSR_FCP_COMMAND: u32 = 0xB00;
pub const CSR_FCP_RESPONSE: u32 = 0xD00;
pub const CSR_FCP_END: u32 = 0xF00;
pub const CSR_TOPOLOGY_MAP: u32 = 0x1000;
pub const CSR_TOPOLOGY_MAP_END: u32 = 0x1400;
pub const CSR_SPEED_MAP: u32 = 0x2000;
pub const CSR_SPEED_MAP_END: u32 = 0x3000;
pub const CSR_OFFSET: u32 = 0x40;
pub const CSR_LEAF: u32 = 0x80;
pub const CSR_DIRECTORY: u32 = 0xc0;
pub const CSR_DESCRIPTOR: u32 = 0x01;
pub const CSR_VENDOR: u32 = 0x03;
pub const CSR_HARDWARE_VERSION: u32 = 0x04;
pub const CSR_UNIT: u32 = 0x11;
pub const CSR_SPECIFIER_ID: u32 = 0x12;
pub const CSR_VERSION: u32 = 0x13;
pub const CSR_DEPENDENT_INFO: u32 = 0x14;
pub const CSR_MODEL: u32 = 0x17;
pub const CSR_DIRECTORY_ID: u32 = 0x20;

#[repr(C)] pub struct fw_csr_iterator { pub p: *const u32, pub end: *const u32 }
extern "C" { pub fn fw_csr_iterator_init(ci: *mut fw_csr_iterator, p: *const u32); pub fn fw_csr_iterator_next(ci: *mut fw_csr_iterator, key: *mut i32, value: *mut i32) -> i32; pub fn fw_csr_string(directory: *const u32, key: i32, buf: *mut i8, size: usize) -> i32; }

#[repr(C)] pub struct bus_type { _private: [u8; 0] }
#[repr(C)] pub struct fw_card_driver { _private: [u8; 0] }
#[repr(C)] pub struct fw_node { _private: [u8; 0] }
#[repr(C)] pub struct device { _private: [u8; 0] }
#[repr(C)] pub struct kref { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct attribute_group { _private: [u8; 0] }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
pub type dma_addr_t = u64;
pub type __be32 = u32;
pub type atomic_t = i32;
pub type work_func_t = Option<unsafe extern "C" fn(*mut work_struct)>;
pub type ieee1394_device_id = u32;
pub type fw_packet_callback_t = Option<unsafe extern "C" fn(*mut fw_packet, *mut fw_card, i32)>;
pub type fw_transaction_callback_t = Option<unsafe extern "C" fn(*mut fw_card, i32, *mut core::ffi::c_void, usize, *mut core::ffi::c_void)>;
pub type fw_transaction_callback_with_tstamp_t = Option<unsafe extern "C" fn(*mut fw_card, i32, u32, u32, *mut core::ffi::c_void, usize, *mut core::ffi::c_void)>;
pub type fw_address_callback_t = Option<unsafe extern "C" fn(*mut fw_card, *mut fw_request, i32, i32, i32, i32, u64, *mut core::ffi::c_void, usize, *mut core::ffi::c_void)>;
pub type fw_iso_callback_t = Option<unsafe extern "C" fn(*mut fw_iso_context, u32, usize, *mut core::ffi::c_void, *mut core::ffi::c_void)>;
pub type fw_iso_mc_callback_t = Option<unsafe extern "C" fn(*mut fw_iso_context, dma_addr_t, *mut core::ffi::c_void)>;

extern "C" { pub static fw_bus_type: bus_type; }

#[repr(C)] pub struct fw_card { pub driver: *const fw_card_driver, pub device: *mut device, pub kref: kref, pub done: completion, pub node_id: i32, pub generation: i32, pub reset_jiffies: u64, pub transactions: fw_card_transactions, pub split_timeout: fw_card_split_timeout, pub guid: u64, pub max_receive: u32, pub link_speed: i32, pub config_rom_generation: i32, pub lock: spinlock_t, pub local_node: *mut fw_node, pub root_node: *mut fw_node, pub irm_node: *mut fw_node, pub color: u8, pub gap_count: i32, pub beta_repeaters_present: bool, pub index: i32, pub link: list_head, pub br_work: delayed_work, pub br_short: bool, pub bm_work: delayed_work, pub bm_retries: i32, pub bm_generation: i32, pub bm_node_id: i32, pub bm_abdicate: bool, pub priority_budget_implemented: bool, pub broadcast_channel_auto_allocated: bool, pub broadcast_channel_allocated: bool, pub broadcast_channel: u32, pub topology_map: fw_card_topology_map, pub maint_utility_register: __be32, pub isoc_wq: *mut workqueue_struct, pub async_wq: *mut workqueue_struct }
#[repr(C)] pub struct fw_card_transactions { pub current_tlabel: i32, pub tlabel_mask: u64, pub list: list_head, pub lock: spinlock_t }
#[repr(C)] pub struct fw_card_split_timeout { pub hi: u32, pub lo: u32, pub cycles: u32, pub jiffies: u32, pub lock: spinlock_t }
#[repr(C)] pub struct fw_card_topology_map { pub buffer: [__be32; 256], pub lock: spinlock_t }
extern "C" { pub fn fw_card_release(kref: *mut kref); pub fn fw_card_read_cycle_time(card: *mut fw_card, cycle_time: *mut u32) -> i32; }

#[repr(C)] pub struct fw_attribute_group { pub groups: [*mut attribute_group; 2], pub group: attribute_group, pub attrs: [*mut attribute; 13] }
#[repr(u32)] pub enum fw_device_quirk { FW_DEVICE_QUIRK_IRM_IS_1394_1995_ONLY = 1, FW_DEVICE_QUIRK_IRM_IGNORES_BUS_MANAGER = 2, FW_DEVICE_QUIRK_ACK_PACKET_WITH_INVALID_PENDING_CODE = 4, FW_DEVICE_QUIRK_UNSTABLE_AT_S400 = 8 }
#[repr(i32)] pub enum fw_device_state { FW_DEVICE_INITIALIZING, FW_DEVICE_RUNNING, FW_DEVICE_GONE, FW_DEVICE_SHUTDOWN }
#[repr(C)] pub struct fw_device { pub state: atomic_t, pub node: *mut fw_node, pub node_id: i32, pub generation: i32, pub max_speed: u32, pub card: *mut fw_card, pub device: device, pub quirks: i32, pub client_list_mutex: mutex, pub client_list: list_head, pub config_rom: *const u32, pub config_rom_length: usize, pub config_rom_retries: i32, pub is_local: u32, pub max_rec: u32, pub cmc: u32, pub irmc: u32, pub bc_implemented: u32, pub workfn: work_func_t, pub work: delayed_work, pub attribute_group: fw_attribute_group }
extern "C" { pub fn fw_device_enable_phys_dma(device: *mut fw_device) -> i32; }
#[repr(C)] pub struct fw_unit { pub device: device, pub directory: *const u32, pub attribute_group: fw_attribute_group }
#[repr(C)] pub struct device_driver { _private: [u8; 0] }
#[repr(C)] pub struct fw_driver { pub driver: device_driver, pub probe: Option<unsafe extern "C" fn(*mut fw_unit, *const ieee1394_device_id) -> i32>, pub update: Option<unsafe extern "C" fn(*mut fw_unit)>, pub remove: Option<unsafe extern "C" fn(*mut fw_unit)>, pub id_table: *const ieee1394_device_id }
#[repr(C)] pub struct fw_packet { pub speed: i32, pub generation: i32, pub header: [u32; 4], pub header_length: usize, pub payload: *mut core::ffi::c_void, pub payload_length: usize, pub payload_bus: dma_addr_t, pub payload_mapped: bool, pub timestamp: u32, pub callback: fw_packet_callback_t, pub ack: i32, pub link: list_head, pub driver_data: *mut core::ffi::c_void }
#[repr(C)] pub union fw_transaction_callback { pub without_tstamp: fw_transaction_callback_t, pub with_tstamp: fw_transaction_callback_with_tstamp_t }
#[repr(C)] pub struct fw_transaction { pub node_id: i32, pub tlabel: i32, pub link: list_head, pub card: *mut fw_card, pub is_split_transaction: bool, pub split_timeout_timer: timer_list, pub split_timeout_cycle: u32, pub packet: fw_packet, pub callback: fw_transaction_callback, pub with_tstamp: bool, pub callback_data: *mut core::ffi::c_void }
#[repr(C)] pub struct fw_request { _private: [u8; 0] }
#[repr(C)] pub struct fw_address_handler { pub offset: u64, pub length: u64, pub address_callback: fw_address_callback_t, pub callback_data: *mut core::ffi::c_void, pub link: list_head, pub kref: kref, pub done: completion }
#[repr(C)] pub struct fw_address_region { pub start: u64, pub end: u64 }
extern "C" { pub static fw_high_memory_region: fw_address_region; pub fn fw_core_add_address_handler(h: *mut fw_address_handler, r: *const fw_address_region) -> i32; pub fn fw_core_remove_address_handler(h: *mut fw_address_handler); pub fn fw_send_response(c: *mut fw_card, r: *mut fw_request, code: i32); pub fn fw_get_request_speed(r: *mut fw_request) -> i32; pub fn fw_request_get_timestamp(r: *const fw_request) -> u32; }
extern "C" {
    pub fn __fw_send_request(card: *mut fw_card, t: *mut fw_transaction, tcode: i32, destination_id: i32, generation: i32, speed: i32, offset: u64, payload: *mut core::ffi::c_void, length: usize, callback: fw_transaction_callback, with_tstamp: bool, callback_data: *mut core::ffi::c_void);
    pub fn fw_cancel_transaction(card: *mut fw_card, transaction: *mut fw_transaction) -> i32;
    pub fn fw_run_transaction(card: *mut fw_card, tcode: i32, destination_id: i32, generation: i32, speed: i32, offset: u64, payload: *mut core::ffi::c_void, length: usize) -> i32;
    pub fn fw_rcode_string(rcode: i32) -> *const i8;
    pub fn fw_schedule_bus_reset(card: *mut fw_card, delayed: bool, short_reset: bool);
}

#[repr(C)] pub struct fw_descriptor { pub link: list_head, pub length: usize, pub immediate: u32, pub key: u32, pub data: *const u32 }
extern "C" { pub fn fw_core_add_descriptor(desc: *mut fw_descriptor) -> i32; pub fn fw_core_remove_descriptor(desc: *mut fw_descriptor); }

#[repr(C)] pub struct fw_iso_packet { pub payload_length: u16, pub interrupt: u32, pub skip: u32, pub tag: u32, pub sy: u32, pub header_length: u32, pub header: [u32; 0] }
pub const FW_ISO_CONTEXT_TRANSMIT: i32 = 0; pub const FW_ISO_CONTEXT_RECEIVE: i32 = 1; pub const FW_ISO_CONTEXT_RECEIVE_MULTICHANNEL: i32 = 2;
pub const FW_ISO_CONTEXT_MATCH_TAG0: i32 = 1; pub const FW_ISO_CONTEXT_MATCH_TAG1: i32 = 2; pub const FW_ISO_CONTEXT_MATCH_TAG2: i32 = 4; pub const FW_ISO_CONTEXT_MATCH_TAG3: i32 = 8; pub const FW_ISO_CONTEXT_MATCH_ALL_TAGS: i32 = 15;
#[repr(C)] pub struct fw_iso_buffer { pub direction: i32, pub pages: *mut *mut page, pub dma_addrs: *mut dma_addr_t, pub page_count: i32 }
#[repr(C)] pub struct fw_iso_callback { pub sc: fw_iso_callback_t }
#[repr(C)] pub struct fw_iso_context { pub card: *mut fw_card, pub work: work_struct, pub type_: i32, pub channel: i32, pub speed: i32, pub flags: i32, pub header_size: usize, pub header_storage_size: usize, pub callback: fw_iso_callback, pub callback_data: *mut core::ffi::c_void }
pub const FW_ISO_CONTEXT_FLAG_DROP_OVERFLOW_HEADERS: i32 = 1;
extern "C" { pub static mut fw_workqueue: *mut workqueue_struct; pub fn fw_iso_buffer_init(b: *mut fw_iso_buffer, c: *mut fw_card, n: i32, d: i32) -> i32; pub fn fw_iso_buffer_destroy(b: *mut fw_iso_buffer, c: *mut fw_card); pub fn __fw_iso_context_create(c: *mut fw_card, type_: i32, channel: i32, speed: i32, header_size: usize, header_storage_size: usize, callback: fw_iso_callback, data: *mut core::ffi::c_void) -> *mut fw_iso_context; pub fn fw_iso_context_set_channels(ctx: *mut fw_iso_context, channels: *mut u64) -> i32; pub fn fw_iso_context_queue(ctx: *mut fw_iso_context, packet: *mut fw_iso_packet, buffer: *mut fw_iso_buffer, payload: usize) -> i32; pub fn fw_iso_context_queue_flush(ctx: *mut fw_iso_context); pub fn fw_iso_context_flush_completions(ctx: *mut fw_iso_context) -> i32; pub fn fw_iso_context_start(c: *mut fw_iso_context, cycle: i32, sync: i32, tags: i32) -> i32; pub fn fw_iso_context_stop(c: *mut fw_iso_context) -> i32; pub fn fw_iso_context_destroy(c: *mut fw_iso_context); pub fn fw_iso_resource_manage(c: *mut fw_card, generation: i32, channels_mask: u64, channel: *mut i32, bandwidth: *mut i32, allocate: bool); }

pub const FW_DEVICE_SHUTDOWN: i32 = 3;
pub unsafe fn fw_stream_packet_destination_id(tag: i32, channel: i32, sy: i32) -> i32 { (tag << 14) | (channel << 8) | sy }
pub unsafe fn fw_send_request(card: *mut fw_card, t: *mut fw_transaction, tcode: i32, destination_id: i32, generation: i32, speed: i32, offset: u64, payload: *mut core::ffi::c_void, length: usize, callback: fw_transaction_callback_t, callback_data: *mut core::ffi::c_void) { __fw_send_request(card, t, tcode, destination_id, generation, speed, offset, payload, length, fw_transaction_callback { without_tstamp: callback }, false, callback_data); }
pub unsafe fn fw_send_request_with_tstamp(card: *mut fw_card, t: *mut fw_transaction, tcode: i32, destination_id: i32, generation: i32, speed: i32, offset: u64, payload: *mut core::ffi::c_void, length: usize, callback: fw_transaction_callback_with_tstamp_t, callback_data: *mut core::ffi::c_void) { __fw_send_request(card, t, tcode, destination_id, generation, speed, offset, payload, length, fw_transaction_callback { with_tstamp: callback }, true, callback_data); }
pub unsafe fn fw_iso_context_create(card: *mut fw_card, type_: i32, channel: i32, speed: i32, header_size: usize, callback: fw_iso_callback_t, data: *mut core::ffi::c_void) -> *mut fw_iso_context { __fw_iso_context_create(card, type_, channel, speed, header_size, 4096, fw_iso_callback { sc: callback }, data) }
pub unsafe fn fw_iso_context_create_with_header_storage_size(card: *mut fw_card, type_: i32, channel: i32, speed: i32, header_size: usize, storage: usize, callback: fw_iso_callback_t, data: *mut core::ffi::c_void) -> *mut fw_iso_context { __fw_iso_context_create(card, type_, channel, speed, header_size, storage, fw_iso_callback { sc: callback }, data) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
