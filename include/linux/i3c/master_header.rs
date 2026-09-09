/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of linux/i3c/master.h. */

pub const I3C_HOT_JOIN_ADDR: u8 = 0x2;
pub const I3C_BROADCAST_ADDR: u8 = 0x7e;
pub const I3C_MAX_ADDR: u8 = 0x7f;

pub const I3C_NOTIFY_BUS_ADD: u32 = 0;
pub const I3C_NOTIFY_BUS_REMOVE: u32 = 1;

pub const I3C_LVR_I2C_INDEX_MASK: u8 = 0xe0;
#[inline] pub const fn i3c_lvr_i2c_index(x: u8) -> u8 { x << 5 }
pub const I3C_LVR_I2C_FM_MODE: u8 = 1 << 4;
pub const I2C_MAX_ADDR: u8 = 0x7f;

#[repr(C)] pub struct i3c_i2c_dev_desc { pub node: list_head, pub master: *mut i3c_master_controller, pub master_priv: *mut core::ffi::c_void }
#[repr(C)] pub struct i2c_dev_boardinfo { pub node: list_head, pub base: i2c_board_info, pub lvr: u8 }
#[repr(C)] pub struct i2c_dev_desc { pub common: i3c_i2c_dev_desc, pub dev: *mut i2c_client, pub addr: u16, pub lvr: u8 }
#[repr(C)] pub struct i3c_ibi_slot { pub work: work_struct, pub dev: *mut i3c_dev_desc, pub len: core::ffi::c_uint, pub data: *mut core::ffi::c_void }
#[repr(C)] pub struct i3c_device_ibi_info {
    pub all_ibis_handled: completion, pub pending_ibis: atomic_t,
    pub max_payload_len: core::ffi::c_uint, pub num_slots: core::ffi::c_uint,
    pub enabled: core::ffi::c_uint, pub wq: *mut workqueue_struct,
    pub handler: Option<unsafe extern "C" fn(*mut i3c_device, *const i3c_ibi_payload)>,
}
#[repr(C)] pub struct i3c_dev_boardinfo { pub node: list_head, pub init_dyn_addr: u8, pub static_addr: u8, pub static_addr_method: u8, pub pid: u64, pub fwnode: *mut fwnode_handle }
#[repr(C)] pub struct i3c_dev_desc {
    pub common: i3c_i2c_dev_desc, pub info: i3c_device_info, pub ibi_lock: mutex,
    pub ibi: *mut i3c_device_ibi_info, pub dev: *mut i3c_device,
    pub boardinfo: *const i3c_dev_boardinfo,
}
#[repr(C)] pub struct i3c_device { pub dev: device, pub desc: *mut i3c_dev_desc, pub bus: *mut i3c_bus, pub node: list_head }

pub const I3C_BUS_MAX_DEVS: usize = 11;
pub const I3C_BUS_I2C_FM_PLUS_SCL_MAX_RATE: u32 = 1_000_000;
pub const I3C_BUS_I2C_FM_SCL_MAX_RATE: u32 = 400_000;
pub const I3C_BUS_I3C_SCL_MAX_RATE: u32 = 12_900_000;
pub const I3C_BUS_I3C_SCL_TYP_RATE: u32 = 12_500_000;
pub const I3C_BUS_TAVAL_MIN_NS: u32 = 1000;
pub const I3C_BUS_TBUF_MIXED_FM_MIN_NS: u32 = 1300;
pub const I3C_BUS_THIGH_MIXED_MAX_NS: u32 = 41;
pub const I3C_BUS_TIDLE_MIN_NS: u32 = 200_000;
pub const I3C_BUS_TLOW_OD_MIN_NS: u32 = 200;
pub const I3C_BUS_THIGH_INIT_OD_MIN_NS: u32 = 200;

#[repr(C)] #[derive(Copy, Clone)] pub enum i3c_bus_mode { I3C_BUS_MODE_PURE, I3C_BUS_MODE_MIXED_FAST, I3C_BUS_MODE_MIXED_LIMITED, I3C_BUS_MODE_MIXED_SLOW }
#[repr(C)] #[derive(Copy, Clone)] pub enum i3c_open_drain_speed { I3C_OPEN_DRAIN_SLOW_SPEED, I3C_OPEN_DRAIN_NORMAL_SPEED }
pub const I3C_ADDR_SLOT_FREE: u32 = 0;
pub const I3C_ADDR_SLOT_RSVD: u32 = 1;
pub const I3C_ADDR_SLOT_I2C_DEV: u32 = 2;
pub const I3C_ADDR_SLOT_I3C_DEV: u32 = 3;
pub const I3C_ADDR_SLOT_STATUS_MASK: u32 = 3;
pub const I3C_ADDR_SLOT_EXT_STATUS_MASK: u32 = 7;
pub const I3C_ADDR_SLOT_EXT_DESIRED: u32 = 1 << 2;
pub const I3C_ADDR_SLOT_STATUS_BITS: usize = 4;

#[repr(C)] pub struct i3c_bus {
    pub cur_master: *mut i3c_dev_desc, pub id: core::ffi::c_int,
    pub addrslots: [core::ffi::c_ulong; ((I2C_MAX_ADDR as usize + 1) * I3C_ADDR_SLOT_STATUS_BITS) / 64],
    pub mode: i3c_bus_mode,
    pub scl_rate: i3c_bus_scl_rate, pub devs: i3c_bus_devs, pub lock: rw_semaphore,
}
#[repr(C)] pub struct i3c_bus_scl_rate { pub i3c: core::ffi::c_ulong, pub i2c: core::ffi::c_ulong }
#[repr(C)] pub struct i3c_bus_devs { pub i3c: list_head, pub i2c: list_head }

#[repr(C)] pub struct i3c_master_controller_ops {
    pub bus_init: Option<unsafe extern "C" fn(*mut i3c_master_controller) -> i32>, pub bus_cleanup: Option<unsafe extern "C" fn(*mut i3c_master_controller)>,
    pub attach_i3c_dev: Option<unsafe extern "C" fn(*mut i3c_dev_desc) -> i32>, pub reattach_i3c_dev: Option<unsafe extern "C" fn(*mut i3c_dev_desc, u8) -> i32>, pub detach_i3c_dev: Option<unsafe extern "C" fn(*mut i3c_dev_desc)>, pub do_daa: Option<unsafe extern "C" fn(*mut i3c_master_controller) -> i32>,
    pub supports_ccc_cmd: Option<unsafe extern "C" fn(*mut i3c_master_controller, *const i3c_ccc_cmd) -> bool>, pub send_ccc_cmd: Option<unsafe extern "C" fn(*mut i3c_master_controller, *mut i3c_ccc_cmd) -> i32>, pub i3c_xfers: Option<unsafe extern "C" fn(*mut i3c_dev_desc, *mut i3c_xfer, i32, i3c_xfer_mode) -> i32>,
    pub attach_i2c_dev: Option<unsafe extern "C" fn(*mut i2c_dev_desc) -> i32>, pub detach_i2c_dev: Option<unsafe extern "C" fn(*mut i2c_dev_desc)>, pub i2c_xfers: Option<unsafe extern "C" fn(*mut i2c_dev_desc, *mut i2c_msg, i32) -> i32>,
    pub request_ibi: Option<unsafe extern "C" fn(*mut i3c_dev_desc, *const i3c_ibi_setup) -> i32>, pub free_ibi: Option<unsafe extern "C" fn(*mut i3c_dev_desc)>, pub enable_ibi: Option<unsafe extern "C" fn(*mut i3c_dev_desc) -> i32>, pub disable_ibi: Option<unsafe extern "C" fn(*mut i3c_dev_desc) -> i32>, pub recycle_ibi_slot: Option<unsafe extern "C" fn(*mut i3c_dev_desc, *mut i3c_ibi_slot)>,
    pub enable_hotjoin: Option<unsafe extern "C" fn(*mut i3c_master_controller) -> i32>, pub disable_hotjoin: Option<unsafe extern "C" fn(*mut i3c_master_controller) -> i32>, pub set_speed: Option<unsafe extern "C" fn(*mut i3c_master_controller, i3c_open_drain_speed) -> i32>, pub set_dev_nack_retry: Option<unsafe extern "C" fn(*mut i3c_master_controller, core::ffi::c_uint) -> i32>,
}
#[repr(C)] pub struct i3c_master_controller { pub dev: device, pub this: *mut i3c_dev_desc, pub i2c: i2c_adapter, pub ops: *const i3c_master_controller_ops, pub secondary: u32, pub init_done: u32, pub hotjoin: u32, pub rpm_allowed: u32, pub rpm_ibi_allowed: u32, pub ibi_wakeup: u32, pub shutting_down: bool, pub boardinfo: i3c_bus_devs, pub bus: i3c_bus, pub addr_method: u8, pub wq: *mut workqueue_struct, pub hj_work: work_struct, pub reg_work: work_struct, pub dev_nack_retry_count: core::ffi::c_uint }
#[repr(C)] pub struct i3c_dma { pub dev: *mut device, pub buf: *mut core::ffi::c_void, pub len: usize, pub map_len: usize, pub addr: dma_addr_t, pub dir: dma_data_direction, pub bounce_buf: *mut core::ffi::c_void }

extern "C" {
    pub static i3c_bus_type: bus_type;
    pub fn i3c_master_do_i2c_xfers(*mut i3c_master_controller, *const i2c_msg, i32) -> i32;
    pub fn i3c_master_disec_locked(*mut i3c_master_controller, u8, u8) -> i32; pub fn i3c_master_enec_locked(*mut i3c_master_controller, u8, u8) -> i32; pub fn i3c_master_enec_disec_locked(*mut i3c_master_controller, u8, bool, u8, bool) -> i32; pub fn i3c_master_entdaa_locked(*mut i3c_master_controller) -> i32; pub fn i3c_master_defslvs_locked(*mut i3c_master_controller) -> i32;
    pub fn i3c_master_get_free_addr(*mut i3c_master_controller, u8) -> i32; pub fn i3c_master_add_i3c_dev_locked(*mut i3c_master_controller, u8); pub fn i3c_master_do_daa(*mut i3c_master_controller) -> i32; pub fn i3c_master_do_daa_ext(*mut i3c_master_controller, bool) -> i32;
    pub fn i3c_master_dma_map_single(*mut device, *mut core::ffi::c_void, usize, bool, dma_data_direction) -> *mut i3c_dma; pub fn i3c_master_dma_unmap_single(*mut i3c_dma);
    pub fn i3c_master_reattach_i3c_dev_locked(*mut i3c_dev_desc, u8) -> i32; pub fn i3c_master_set_info(*mut i3c_master_controller, *const i3c_device_info) -> i32; pub fn i3c_master_register(*mut i3c_master_controller, *mut device, *const i3c_master_controller_ops, bool) -> i32; pub fn i3c_master_unregister(*mut i3c_master_controller); pub fn i3c_master_enable_hotjoin(*mut i3c_master_controller) -> i32; pub fn i3c_master_disable_hotjoin(*mut i3c_master_controller) -> i32; pub fn i3c_master_queue_hotjoin(*mut i3c_master_controller);
    pub fn i3c_master_queue_ibi(*mut i3c_dev_desc, *mut i3c_ibi_slot); pub fn i3c_master_has_wakeup_enabled_devs(*mut i3c_master_controller) -> bool; pub fn i3c_master_get_free_ibi_slot(*mut i3c_dev_desc) -> *mut i3c_ibi_slot;
    pub fn i3c_for_each_bus_locked(Option<unsafe extern "C" fn(*mut i3c_bus, *mut core::ffi::c_void) -> i32>, *mut core::ffi::c_void); pub fn i3c_register_notifier(*mut notifier_block) -> i32; pub fn i3c_unregister_notifier(*mut notifier_block) -> i32;
}

#[inline] pub unsafe fn i3c_dev_get_master_data(dev: *const i3c_dev_desc) -> *mut core::ffi::c_void { (*dev).common.master_priv }
#[inline] pub unsafe fn i3c_dev_set_master_data(dev: *mut i3c_dev_desc, data: *mut core::ffi::c_void) { (*dev).common.master_priv = data; }
#[inline] pub unsafe fn i2c_dev_get_master_data(dev: *const i2c_dev_desc) -> *mut core::ffi::c_void { (*dev).common.master_priv }
#[inline] pub unsafe fn i2c_dev_set_master_data(dev: *mut i2c_dev_desc, data: *mut core::ffi::c_void) { (*dev).common.master_priv = data; }
#[inline] pub unsafe fn i3c_dev_get_master(dev: *mut i3c_dev_desc) -> *mut i3c_master_controller { (*dev).common.master }
#[inline] pub unsafe fn i2c_dev_get_master(dev: *mut i2c_dev_desc) -> *mut i3c_master_controller { (*dev).common.master }
#[inline] pub unsafe fn i3c_master_get_bus(master: *mut i3c_master_controller) -> *mut i3c_bus { &mut (*master).bus }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
