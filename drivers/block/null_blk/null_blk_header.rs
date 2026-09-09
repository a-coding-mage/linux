/* SPDX-License-Identifier: GPL-2.0 */

/* Linux dependencies supplied by the surrounding translation unit. */

#[repr(C)]
pub struct nullb_cmd {
    pub error: blk_status_t,
    pub fake_timeout: bool,
    pub nq: *mut nullb_queue,
    pub timer: hrtimer,
}

#[repr(C)]
pub struct nullb_queue {
    pub dev: *mut nullb_device,
    pub requeue_selection: ::core::ffi::c_uint,
    pub poll_list: list_head,
    pub poll_lock: spinlock_t,
}

#[repr(C)]
pub union nullb_zone_lock {
    pub spinlock: spinlock_t,
    pub mutex: mutex,
}

#[repr(C)]
pub struct nullb_zone {
    /* Zone lock prevents concurrent modification of a zone write pointer and condition. */
    pub lock: nullb_zone_lock,
    pub type_: blk_zone_type,
    pub cond: blk_zone_cond,
    pub start: sector_t,
    pub wp: sector_t,
    pub len: ::core::ffi::c_uint,
    pub capacity: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct nullb_device {
    pub nullb: *mut nullb,
    pub group: config_group,
    #[cfg(CONFIG_BLK_DEV_NULL_BLK_FAULT_INJECTION)]
    pub timeout_config: fault_config,
    #[cfg(CONFIG_BLK_DEV_NULL_BLK_FAULT_INJECTION)]
    pub requeue_config: fault_config,
    #[cfg(CONFIG_BLK_DEV_NULL_BLK_FAULT_INJECTION)]
    pub init_hctx_fault_config: fault_config,
    pub data: radix_tree_root,
    pub cache: radix_tree_root,
    pub flags: ::core::ffi::c_ulong,
    pub curr_cache: ::core::ffi::c_uint,
    pub badblocks: badblocks,
    pub badblocks_once: bool,
    pub badblocks_partial_io: bool,
    pub nr_zones: ::core::ffi::c_uint,
    pub nr_zones_imp_open: ::core::ffi::c_uint,
    pub nr_zones_exp_open: ::core::ffi::c_uint,
    pub nr_zones_closed: ::core::ffi::c_uint,
    pub imp_close_zone_no: ::core::ffi::c_uint,
    pub zones: *mut nullb_zone,
    pub zone_size_sects: sector_t,
    pub need_zone_res_mgmt: bool,
    pub zone_res_lock: spinlock_t,
    pub size: ::core::ffi::c_ulong,
    pub completion_nsec: ::core::ffi::c_ulong,
    pub cache_size: ::core::ffi::c_ulong,
    pub zone_size: ::core::ffi::c_ulong,
    pub zone_capacity: ::core::ffi::c_ulong,
    pub zone_nr_conv: ::core::ffi::c_uint,
    pub zone_max_open: ::core::ffi::c_uint,
    pub zone_max_active: ::core::ffi::c_uint,
    pub zone_append_max_sectors: ::core::ffi::c_uint,
    pub submit_queues: ::core::ffi::c_uint,
    pub prev_submit_queues: ::core::ffi::c_uint,
    pub poll_queues: ::core::ffi::c_uint,
    pub prev_poll_queues: ::core::ffi::c_uint,
    pub home_node: ::core::ffi::c_uint,
    pub queue_mode: ::core::ffi::c_uint,
    pub blocksize: ::core::ffi::c_uint,
    pub max_sectors: ::core::ffi::c_uint,
    pub irqmode: ::core::ffi::c_uint,
    pub hw_queue_depth: ::core::ffi::c_uint,
    pub index: ::core::ffi::c_uint,
    pub mbps: ::core::ffi::c_uint,
    pub blocking: bool,
    pub use_per_node_hctx: bool,
    pub power: bool,
    pub memory_backed: bool,
    pub discard: bool,
    pub zoned: bool,
    pub zone_full: bool,
    pub virt_boundary: bool,
    pub no_sched: bool,
    pub shared_tags: bool,
    pub shared_tag_bitmap: bool,
    pub fua: bool,
    pub rotational: bool,
}

#[repr(C)]
pub struct nullb {
    pub dev: *mut nullb_device,
    pub list: list_head,
    pub index: ::core::ffi::c_uint,
    pub q: *mut request_queue,
    pub disk: *mut gendisk,
    pub tag_set: *mut blk_mq_tag_set,
    pub __tag_set: blk_mq_tag_set,
    pub cur_bytes: atomic_long_t,
    pub bw_timer: hrtimer,
    pub cache_flush_pos: ::core::ffi::c_ulong,
    pub lock: spinlock_t,
    pub queues: *mut nullb_queue,
    pub disk_name: [::core::ffi::c_char; DISK_NAME_LEN],
}

extern "C" {
    pub fn null_handle_discard(dev: *mut nullb_device, sector: sector_t, nr_sectors: sector_t) -> blk_status_t;
    pub fn null_process_cmd(cmd: *mut nullb_cmd, op: req_op, sector: sector_t, nr_sectors: ::core::ffi::c_uint) -> blk_status_t;
    pub fn null_handle_badblocks(cmd: *mut nullb_cmd, sector: sector_t, nr_sectors: *mut ::core::ffi::c_uint) -> blk_status_t;
    pub fn null_handle_memory_backed(cmd: *mut nullb_cmd, op: req_op, sector: sector_t, nr_sectors: sector_t) -> blk_status_t;
}

#[cfg(CONFIG_BLK_DEV_ZONED)]
extern "C" {
    pub fn null_init_zoned_dev(dev: *mut nullb_device, lim: *mut queue_limits) -> ::core::ffi::c_int;
    pub fn null_register_zoned_dev(nullb: *mut nullb) -> ::core::ffi::c_int;
    pub fn null_free_zoned_dev(dev: *mut nullb_device);
    pub fn null_report_zones(disk: *mut gendisk, sector: sector_t, nr_zones: ::core::ffi::c_uint, args: *mut blk_report_zones_args) -> ::core::ffi::c_int;
    pub fn null_process_zoned_cmd(cmd: *mut nullb_cmd, op: req_op, sector: sector_t, nr_sectors: sector_t) -> blk_status_t;
    pub fn null_zone_valid_read_len(nullb: *mut nullb, sector: sector_t, len: ::core::ffi::c_uint) -> usize;
    pub fn zone_cond_store(dev: *mut nullb_device, page: *const ::core::ffi::c_char, count: usize, cond: blk_zone_cond) -> isize;
}

#[cfg(not(CONFIG_BLK_DEV_ZONED))]
pub unsafe extern "C" fn null_init_zoned_dev(_dev: *mut nullb_device, _lim: *mut queue_limits) -> ::core::ffi::c_int { -EINVAL }
#[cfg(not(CONFIG_BLK_DEV_ZONED))]
pub unsafe extern "C" fn null_register_zoned_dev(_nullb: *mut nullb) -> ::core::ffi::c_int { -ENODEV }
#[cfg(not(CONFIG_BLK_DEV_ZONED))]
pub unsafe extern "C" fn null_free_zoned_dev(_dev: *mut nullb_device) {}
#[cfg(not(CONFIG_BLK_DEV_ZONED))]
pub unsafe extern "C" fn null_process_zoned_cmd(_cmd: *mut nullb_cmd, _op: req_op, _sector: sector_t, _nr_sectors: sector_t) -> blk_status_t { BLK_STS_NOTSUPP }
#[cfg(not(CONFIG_BLK_DEV_ZONED))]
pub unsafe extern "C" fn null_zone_valid_read_len(_nullb: *mut nullb, _sector: sector_t, len: ::core::ffi::c_uint) -> usize { len as usize }
#[cfg(not(CONFIG_BLK_DEV_ZONED))]
pub unsafe extern "C" fn zone_cond_store(_dev: *mut nullb_device, _page: *const ::core::ffi::c_char, _count: usize, _cond: blk_zone_cond) -> isize { -EOPNOTSUPP }

/* #define null_report_zones NULL when CONFIG_BLK_DEV_ZONED is disabled. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
