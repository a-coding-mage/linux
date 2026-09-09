/* SPDX-License-Identifier: GPL-2.0 */
/* Register map access API internal header */

// Dependencies supplied by other translated headers are intentionally not defined here.

use core::ffi::c_void;

pub struct regmap;
pub struct regcache_ops;

#[repr(C)]
pub struct regmap_debugfs_off_cache {
    pub list: list_head,
    pub min: off_t,
    pub max: off_t,
    pub base_reg: u32,
    pub max_reg: u32,
}

#[repr(C)]
pub struct regmap_format {
    pub buf_size: usize,
    pub reg_bytes: usize,
    pub pad_bytes: usize,
    pub val_bytes: usize,
    pub reg_shift: i8,
    pub format_write: Option<unsafe extern "C" fn(*mut regmap, u32, u32)>,
    pub format_reg: Option<unsafe extern "C" fn(*mut c_void, u32, u32)>,
    pub format_val: Option<unsafe extern "C" fn(*mut c_void, u32, u32)>,
    pub parse_val: Option<unsafe extern "C" fn(*const c_void) -> u32>,
    pub parse_inplace: Option<unsafe extern "C" fn(*mut c_void)>,
}

#[repr(C)]
pub struct regmap_async {
    pub list: list_head,
    pub map: *mut regmap,
    pub work_buf: *mut c_void,
}

#[repr(C)]
pub union regmap_lock_union {
    pub mutex: mutex,
    pub spin: regmap_spin_lock,
    pub raw_spin: regmap_raw_spin_lock,
}

#[repr(C)]
pub struct regmap_spin_lock {
    pub spinlock: spinlock_t,
    pub spinlock_flags: c_ulong,
}

#[repr(C)]
pub struct regmap_raw_spin_lock {
    pub raw_spinlock: raw_spinlock_t,
    pub raw_spinlock_flags: c_ulong,
}

#[repr(C)]
pub struct regmap {
    pub lock_union: regmap_lock_union,
    pub lock_key: *mut lock_class_key,
    pub lock: regmap_lock,
    pub unlock: regmap_unlock,
    pub lock_arg: *mut c_void,
    pub alloc_flags: gfp_t,
    pub reg_base: u32,
    pub dev: *mut device,
    pub work_buf: *mut c_void,
    pub format: regmap_format,
    pub bus: *const regmap_bus,
    pub bus_context: *mut c_void,
    pub name: *const c_char,
    pub async_lock: spinlock_t,
    pub async_waitq: wait_queue_head_t,
    pub async_list: list_head,
    pub async_free: list_head,
    pub async_ret: c_int,
    pub r#async: bool,

    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs_disable: bool,
    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs: *mut dentry,
    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs_name: *const c_char,
    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs_dummy_id: c_int,
    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs_reg_len: u32,
    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs_val_len: u32,
    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs_tot_len: u32,
    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs_off_cache: list_head,
    #[cfg(CONFIG_DEBUG_FS)]
    pub cache_lock: mutex,

    pub max_register: u32,
    pub max_register_is_set: bool,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool>,
    pub precious_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool>,
    pub writeable_noinc_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool>,
    pub readable_noinc_reg: Option<unsafe extern "C" fn(*mut device, u32) -> bool>,
    pub wr_table: *const regmap_access_table,
    pub rd_table: *const regmap_access_table,
    pub volatile_table: *const regmap_access_table,
    pub precious_table: *const regmap_access_table,
    pub wr_noinc_table: *const regmap_access_table,
    pub rd_noinc_table: *const regmap_access_table,
    pub reg_read: Option<unsafe extern "C" fn(*mut c_void, u32, *mut u32) -> c_int>,
    pub reg_write: Option<unsafe extern "C" fn(*mut c_void, u32, u32) -> c_int>,
    pub reg_update_bits: Option<unsafe extern "C" fn(*mut c_void, u32, u32, u32) -> c_int>,
    pub read: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize, *mut c_void, usize) -> c_int>,
    pub write: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize) -> c_int>,
    pub reg_default_cb: Option<unsafe extern "C" fn(*mut device, u32, *mut u32) -> c_int>,
    pub read_flag_mask: c_ulong,
    pub write_flag_mask: c_ulong,
    pub reg_shift: c_int,
    pub reg_stride: c_int,
    pub reg_stride_order: c_int,
    pub defer_caching: bool,
    pub force_write_field: bool,
    pub cache_ops: *const regcache_ops,
    pub cache_type: regcache_type,
    pub cache_size_raw: u32,
    pub cache_word_size: u32,
    pub num_reg_defaults: u32,
    pub num_reg_defaults_raw: u32,
    pub cache_only: bool,
    pub cache_bypass: bool,
    pub cache_free: bool,
    pub reg_defaults: *mut reg_default,
    pub reg_defaults_raw: *const c_void,
    pub cache: *mut c_void,
    pub cache_dirty: bool,
    pub no_sync_defaults: bool,
    pub patch: *mut reg_sequence,
    pub patch_regs: u32,
    pub can_sleep: bool,
    pub use_single_read: bool,
    pub use_single_write: bool,
    pub can_multi_write: bool,
    pub max_raw_read: usize,
    pub max_raw_write: usize,
    pub range_tree: rb_root,
    pub selector_work_buf: *mut c_void,
    pub hwlock: *mut hwspinlock,
}

#[repr(C)]
pub struct regcache_ops {
    pub name: *const c_char,
    pub r#type: regcache_type,
    pub init: Option<unsafe extern "C" fn(*mut regmap) -> c_int>,
    pub exit: Option<unsafe extern "C" fn(*mut regmap)>,
    pub populate: Option<unsafe extern "C" fn(*mut regmap) -> c_int>,
    #[cfg(CONFIG_DEBUG_FS)]
    pub debugfs_init: Option<unsafe extern "C" fn(*mut regmap)>,
    pub read: Option<unsafe extern "C" fn(*mut regmap, u32, *mut u32) -> c_int>,
    pub write: Option<unsafe extern "C" fn(*mut regmap, u32, u32) -> c_int>,
    pub sync: Option<unsafe extern "C" fn(*mut regmap, u32, u32) -> c_int>,
    pub drop: Option<unsafe extern "C" fn(*mut regmap, u32, u32) -> c_int>,
}

extern "C" {
    pub fn regmap_cached(map: *mut regmap, reg: u32) -> bool;
    pub fn regmap_writeable(map: *mut regmap, reg: u32) -> bool;
    pub fn regmap_readable(map: *mut regmap, reg: u32) -> bool;
    pub fn regmap_volatile(map: *mut regmap, reg: u32) -> bool;
    pub fn regmap_precious(map: *mut regmap, reg: u32) -> bool;
    pub fn regmap_writeable_noinc(map: *mut regmap, reg: u32) -> bool;
    pub fn regmap_readable_noinc(map: *mut regmap, reg: u32) -> bool;
    pub fn _regmap_write(map: *mut regmap, reg: u32, val: u32) -> c_int;
}

#[repr(C)]
pub struct regmap_range_node {
    pub node: rb_node,
    pub name: *const c_char,
    pub map: *mut regmap,
    pub range_min: u32,
    pub range_max: u32,
    pub selector_reg: u32,
    pub selector_mask: u32,
    pub selector_shift: c_int,
    pub window_start: u32,
    pub window_len: u32,
}

#[repr(C)]
pub struct regmap_field {
    pub regmap: *mut regmap,
    pub mask: u32,
    pub shift: u32,
    pub reg: u32,
    pub id_size: u32,
    pub id_offset: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
