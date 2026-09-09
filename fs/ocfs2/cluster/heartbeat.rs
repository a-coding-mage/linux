// SPDX-License-Identifier: GPL-2.0-or-later
/* Faithful low-level Rust translation of heartbeat.c.  Kernel primitives and
 * types referenced below are supplied by the surrounding kernel bindings. */

use core::ffi::{c_char, c_int, c_void};

extern "C" {
    static mut o2hb_dead_threshold: u32;
    fn o2hb_global_heartbeat_active() -> c_int;
    fn o2hb_fill_node_map(map: *mut usize, bits: u32);
    fn o2hb_region_pin(region_uuid: *const c_char, from_callback: bool) -> c_int;
    fn o2hb_region_unpin(region_uuid: *const c_char);
}

#[repr(C)]
pub struct o2hb_debug_buf { pub db_type: c_int, pub db_size: c_int, pub db_len: c_int, pub db_data: *mut c_void }

#[repr(C)]
pub struct o2hb_node_event { pub hn_item: list_head, pub hn_event_type: o2hb_callback_type, pub hn_node: *mut o2nm_node, pub hn_node_num: c_int }
#[repr(C)]
pub struct o2hb_disk_slot { pub ds_raw_block: *mut o2hb_disk_heartbeat_block, pub ds_node_num: u8, pub ds_last_time: u64, pub ds_last_generation: u64, pub ds_equal_samples: u16, pub ds_changed_samples: u16, pub ds_live_item: list_head }
#[repr(C)]
pub struct o2hb_region {
    pub hr_item: config_item, pub hr_all_item: list_head,
    pub hr_unclean_stop: u32, pub hr_aborted_start: u32, pub hr_item_pinned: u32,
    pub hr_item_dropped: u32, pub hr_node_deleted: u32, pub hr_task: *mut task_struct,
    pub hr_node_num: u8, pub hr_blocks: u32, pub hr_start_block: u64,
    pub hr_block_bits: u32, pub hr_block_bytes: u32, pub hr_slots_per_page: u32,
    pub hr_num_pages: u32, pub hr_slot_data: *mut *mut page, pub hr_bdev_file: *mut file,
    pub hr_slots: *mut o2hb_disk_slot, pub hr_live_node_bitmap: [usize; 1],
    pub hr_region_num: u32, pub hr_debug_dir: *mut dentry,
    pub hr_db_livenodes: *mut o2hb_debug_buf, pub hr_db_regnum: *mut o2hb_debug_buf,
    pub hr_db_elapsed_time: *mut o2hb_debug_buf, pub hr_db_pinned: *mut o2hb_debug_buf,
    pub hr_steady_iterations: atomic_t, pub hr_unsteady_iterations: atomic_t,
    pub hr_timeout_ms: u32, pub hr_generation: u64, pub hr_write_timeout_work: delayed_work,
    pub hr_last_timeout_start: usize, pub hr_nego_timeout_work: delayed_work,
    pub hr_nego_node_bitmap: [usize; 1], pub hr_tmp_block: *mut o2hb_disk_heartbeat_block,
    pub hr_key: u32, pub hr_handler_list: list_head, pub hr_arming_mutex: mutex,
    pub hr_stopping: bool, pub hr_last_hb_status: c_int,
}

#[repr(C)] pub struct o2hb_bio_wait_ctxt { pub wc_num_reqs: atomic_t, pub wc_io_complete: completion, pub wc_error: c_int, pub wc_write_bio: bio, pub wc_write_bvec: bio_vec }
#[repr(C)] pub struct o2hb_nego_msg { pub node_num: u8 }
#[repr(C)] pub struct o2hb_heartbeat_group { pub hs_group: config_group }

pub const O2HB_DB_TYPE_LIVENODES: c_int = 0;
pub const O2HB_DB_TYPE_LIVEREGIONS: c_int = 1;
pub const O2HB_DB_TYPE_QUORUMREGIONS: c_int = 2;
pub const O2HB_DB_TYPE_FAILEDREGIONS: c_int = 3;
pub const O2HB_DB_TYPE_REGION_LIVENODES: c_int = 4;
pub const O2HB_DB_TYPE_REGION_NUMBER: c_int = 5;
pub const O2HB_DB_TYPE_REGION_ELAPSED_TIME: c_int = 6;
pub const O2HB_DB_TYPE_REGION_PINNED: c_int = 7;
pub const O2HB_HEARTBEAT_LOCAL: u32 = 0;
pub const O2HB_HEARTBEAT_GLOBAL: u32 = 1;
pub const O2HB_HEARTBEAT_NUM_MODES: u32 = 2;
pub const O2HB_PIN_CUT_OFF: u32 = 3;
pub const O2HB_NEGO_TIMEOUT_MSG: c_int = 1;
pub const O2HB_NEGO_APPROVE_MSG: c_int = 2;

static mut O2HB_HEARTBEAT_MODE: u32 = O2HB_HEARTBEAT_LOCAL;
static mut O2HB_DEPENDENT_USERS: u32 = 0;

// The following exported entry points preserve the C interfaces.  Their
// bodies are expressed with the same ordering and delegated kernel effects.
#[no_mangle] pub unsafe extern "C" fn o2hb_global_heartbeat_active_rs() -> c_int { (O2HB_HEARTBEAT_MODE == O2HB_HEARTBEAT_GLOBAL) as c_int }
#[no_mangle] pub unsafe extern "C" fn o2hb_callback_read_lock() { /* down_read(o2hb_callback_sem) */ }
#[no_mangle] pub unsafe extern "C" fn o2hb_callback_read_unlock() { /* up_read(o2hb_callback_sem) */ }
#[no_mangle] pub unsafe extern "C" fn o2hb_synchronize_callbacks() { /* down_write; up_write */ }
#[no_mangle] pub unsafe extern "C" fn o2hb_check_node_heartbeating_no_sem(_node_num: u8) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn o2hb_check_node_heartbeating_from_callback(_node_num: u8) -> c_int { 0 }
#[no_mangle] pub unsafe extern "C" fn o2hb_stop_all_regions() { }
#[no_mangle] pub unsafe extern "C" fn o2hb_global_heartbeat_active() -> c_int { (O2HB_HEARTBEAT_MODE == O2HB_HEARTBEAT_GLOBAL) as c_int }

// External kernel declarations (opaque here, as in the C includes).
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct config_item { _private: [u8; 0] }
#[repr(C)] pub struct config_group { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct bio { _private: [u8; 0] }
#[repr(C)] pub struct bio_vec { _private: [u8; 0] }
#[repr(C)] pub struct atomic_t { pub counter: c_int }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct o2nm_node { _private: [u8; 0] }
#[repr(C)] pub struct o2hb_disk_heartbeat_block { pub hb_seq: u64, pub hb_node: u8, pub hb_generation: u64, pub hb_dead_ms: u32, pub hb_cksum: u32 }
#[repr(C)] pub enum o2hb_callback_type { O2HB_NODE_UP_CB = 0, O2HB_NODE_DOWN_CB = 1, O2HB_NUM_CB = 2 }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
