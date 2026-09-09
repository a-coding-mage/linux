/* SPDX-License-Identifier: GPL-2.0 */
/* C header translation; external kernel types and symbols are supplied by dependencies. */

pub const BLKCG_MAX_POLS: u32 = 6;
pub const DISK_MAX_PARTS: u32 = 256;
pub const DISK_NAME_LEN: usize = 32;
pub const PARTITION_META_INFO_VOLNAMELTH: usize = 64;
pub const PARTITION_META_INFO_UUIDLTH: usize = UUID_STRING_LEN as usize + 1;

#[repr(C)] pub struct partition_meta_info { pub uuid: [i8; PARTITION_META_INFO_UUIDLTH], pub volname: [u8; PARTITION_META_INFO_VOLNAMELTH] }
pub const GENHD_FL_REMOVABLE: u32 = 1 << 0;
pub const GENHD_FL_HIDDEN: u32 = 1 << 1;
pub const GENHD_FL_NO_PART: u32 = 1 << 2;
pub const DISK_EVENT_MEDIA_CHANGE: u32 = 1 << 0;
pub const DISK_EVENT_EJECT_REQUEST: u32 = 1 << 1;
pub const DISK_EVENT_FLAG_POLL: u32 = 1 << 0;
pub const DISK_EVENT_FLAG_UEVENT: u32 = 1 << 1;
pub const DISK_EVENT_FLAG_BLOCK_ON_EXCL_WRITE: u32 = 1 << 2;

#[repr(C)] #[derive(Copy, Clone)] pub enum blk_integrity_checksum { BLK_INTEGRITY_CSUM_NONE=0, BLK_INTEGRITY_CSUM_IP=1, BLK_INTEGRITY_CSUM_CRC=2, BLK_INTEGRITY_CSUM_CRC64=3 }
#[repr(C)] pub struct blk_integrity { pub flags:u8, pub csum_type:blk_integrity_checksum, pub metadata_size:u8, pub pi_offset:u8, pub interval_exp:u8, pub tag_size:u8, pub pi_tuple_size:u8 }
pub const BLK_OPEN_READ: blk_mode_t = 1 << 0;
pub const BLK_OPEN_WRITE: blk_mode_t = 1 << 1;
pub const BLK_OPEN_EXCL: blk_mode_t = 1 << 2;
pub const BLK_OPEN_NDELAY: blk_mode_t = 1 << 3;
pub const BLK_OPEN_WRITE_IOCTL: blk_mode_t = 1 << 4;
pub const BLK_OPEN_RESTRICT_WRITES: blk_mode_t = 1 << 5;
pub const BLK_OPEN_STRICT_SCAN: blk_mode_t = 1 << 6;

#[repr(C)] pub struct gendisk {
    pub major:i32, pub first_minor:i32, pub minors:i32, pub disk_name:[i8;DISK_NAME_LEN],
    pub events:u16, pub event_flags:u16, pub part_tbl:xarray, pub part0:*mut block_device,
    pub fops:*const block_device_operations, pub queue:*mut request_queue, pub private_data:*mut core::ffi::c_void,
    pub bio_split:bio_set, pub flags:i32, pub state:usize, pub open_mutex:mutex, pub open_partitions:u32,
    pub bdi:*mut backing_dev_info, pub queue_kobj:kobject, pub slave_dir:*mut kobject, pub random:*mut timer_rand_state,
    pub ev:*mut disk_events, pub node_id:i32, pub bb:*mut badblocks, pub lockdep_map:lockdep_map,
    pub diskseq:u64, pub open_mode:blk_mode_t, pub ia_ranges:*mut blk_independent_access_ranges,
    pub rqos_state_mutex:mutex,
}
pub const GD_NEED_PART_SCAN: u32=0; pub const GD_READ_ONLY:u32=1; pub const GD_DEAD:u32=2; pub const GD_NATIVE_CAPACITY:u32=3; pub const GD_ADDED:u32=4; pub const GD_SUPPRESS_PART_SCAN:u32=5; pub const GD_OWNS_QUEUE:u32=6; pub const GD_ZONE_APPEND_USED:u32=7; pub const GD_ERROR_INJECT:u32=8;

pub type blk_features_t=u32; pub const BLK_FEAT_WRITE_CACHE:blk_features_t=1<<0; pub const BLK_FEAT_FUA:blk_features_t=1<<1; pub const BLK_FEAT_ROTATIONAL:blk_features_t=1<<2; pub const BLK_FEAT_ADD_RANDOM:blk_features_t=1<<3; pub const BLK_FEAT_IO_STAT:blk_features_t=1<<4; pub const BLK_FEAT_STABLE_WRITES:blk_features_t=1<<5; pub const BLK_FEAT_SYNCHRONOUS:blk_features_t=1<<6; pub const BLK_FEAT_NOWAIT:blk_features_t=1<<7; pub const BLK_FEAT_DAX:blk_features_t=1<<8; pub const BLK_FEAT_POLL:blk_features_t=1<<9; pub const BLK_FEAT_ZONED:blk_features_t=1<<10; pub const BLK_FEAT_PCI_P2PDMA:blk_features_t=1<<12; pub const BLK_FEAT_SKIP_TAGSET_QUIESCE:blk_features_t=1<<13; pub const BLK_FEAT_ATOMIC_WRITES:blk_features_t=1<<14; pub const BLK_FEAT_RAID_PARTIAL_STRIPES_EXPENSIVE:blk_features_t=1<<15;
pub type blk_flags_t=u32; pub const BLK_FLAG_WRITE_CACHE_DISABLED:blk_flags_t=1; pub const BLK_FLAG_MISALIGNED:blk_flags_t=2; pub const BLK_FLAG_IOSTATS_PASSTHROUGH:blk_flags_t=4;

#[repr(C)] pub struct queue_limits { pub features:blk_features_t,pub flags:blk_flags_t,pub seg_boundary_mask:usize,pub virt_boundary_mask:usize,pub max_hw_sectors:u32,pub max_dev_sectors:u32,pub chunk_sectors:u32,pub max_sectors:u32,pub max_user_sectors:u32,pub max_segment_size:u32,pub max_fast_segment_size:u32,pub physical_block_size:u32,pub logical_block_size:u32,pub alignment_offset:u32,pub io_min:u32,pub io_opt:u32,pub max_discard_sectors:u32,pub max_hw_discard_sectors:u32,pub max_user_discard_sectors:u32,pub max_secure_erase_sectors:u32,pub max_write_zeroes_sectors:u32,pub max_wzeroes_unmap_sectors:u32,pub max_hw_wzeroes_unmap_sectors:u32,pub max_user_wzeroes_unmap_sectors:u32,pub max_hw_zone_append_sectors:u32,pub max_zone_append_sectors:u32,pub discard_granularity:u32,pub discard_alignment:u32,pub zone_write_granularity:u32,pub atomic_write_hw_max:u32,pub atomic_write_max_sectors:u32,pub atomic_write_hw_boundary:u32,pub atomic_write_boundary_sectors:u32,pub atomic_write_hw_unit_min:u32,pub atomic_write_unit_min:u32,pub atomic_write_hw_unit_max:u32,pub atomic_write_unit_max:u32,pub max_segments:u16,pub max_integrity_segments:u16,pub max_discard_segments:u16,pub max_write_streams:u16,pub write_stream_granularity:u32,pub max_open_zones:u32,pub max_active_zones:u32,pub dma_alignment:u32,pub dma_pad_mask:u32,pub integrity:blk_integrity }
#[repr(C)] pub struct blk_independent_access_range { pub kobj:kobject,pub sector:sector_t,pub nr_sectors:sector_t }
#[repr(C)] pub struct blk_independent_access_ranges { pub kobj:kobject,pub sysfs_registered:bool,pub nr_ia_ranges:u32,pub ia_range:[blk_independent_access_range;0] }

#[repr(C)] pub struct request_queue { pub queuedata:*mut core::ffi::c_void,pub elevator:*mut elevator_queue,pub mq_ops:*const blk_mq_ops,pub queue_ctx:*mut blk_mq_ctx,pub queue_flags:usize,pub rq_timeout:u32,pub queue_depth:u32,pub refs:refcount_t,pub nr_hw_queues:u32,pub queue_hw_ctx:*mut *mut blk_mq_hw_ctx,pub q_usage_counter:percpu_ref,pub queue_lock:spinlock_t,pub quiesce_depth:i32,pub disk:*mut gendisk,pub mq_kobj:*mut kobject,pub limits:queue_limits,pub pm_only:atomic_t,pub stats:*mut blk_queue_stats,pub rq_qos:*mut rq_qos,pub rq_qos_mutex:mutex,pub id:i32,pub nr_requests:u32,pub async_depth:u32,pub timeout:timer_list,pub timeout_work:work_struct,pub nr_active_requests_shared_tags:atomic_t,pub sched_shared_tags:*mut blk_mq_tags,pub node:i32,pub requeue_lock:spinlock_t,pub requeue_list:list_head,pub rcu_head:rcu_head,pub mq_freeze_wq:wait_queue_head_t,pub mq_freeze_lock:mutex,pub tag_set:*mut blk_mq_tag_set,pub tag_set_list:list_head,pub debugfs_dir:*mut dentry,pub sched_debugfs_dir:*mut dentry,pub rqos_debugfs_dir:*mut dentry,pub debugfs_mutex:mutex }

#[repr(C)] pub struct block_device_operations { pub submit_bio:Option<unsafe extern "C" fn(*mut bio)>, pub poll_bio:Option<unsafe extern "C" fn(*mut bio,*mut io_comp_batch,u32)->i32>, pub open:Option<unsafe extern "C" fn(*mut gendisk,blk_mode_t)->i32>, pub release:Option<unsafe extern "C" fn(*mut gendisk)>, pub ioctl:Option<unsafe extern "C" fn(*mut block_device,blk_mode_t,u32,usize)->i32>, pub owner:*mut module }
#[repr(C)] pub struct io_comp_batch { pub req_list:rq_list,pub need_ts:bool,pub complete:Option<unsafe extern "C" fn(*mut io_comp_batch)>,pub poll_ctx:*mut core::ffi::c_void }

pub const BLK_ALL_ZONES:u32=u32::MAX; pub const BLK_POLL_ONESHOT:u32=1; pub const BLKDEV_ZERO_NOUNMAP:u32=1; pub const BLKDEV_ZERO_NOFALLBACK:u32=2; pub const BLKDEV_ZERO_KILLABLE:u32=4; pub const BLK_INTEGRITY_MAX_SIZE:usize=2*1024*1024;
extern "C" { pub fn disk_report_zone(*mut gendisk,*mut blk_zone,u32,*mut blk_report_zones_args)->i32; pub fn blkdev_get_zone_info(*mut block_device,sector_t,*mut blk_zone)->i32; pub fn blkdev_report_zones(*mut block_device,sector_t,u32,Option<unsafe extern "C" fn(*mut blk_zone,u32,*mut core::ffi::c_void)->i32>,*mut core::ffi::c_void)->i32; pub fn blkdev_zone_mgmt(*mut block_device,req_op,sector_t,sector_t)->i32; pub fn put_disk(*mut gendisk); pub fn set_capacity(*mut gendisk,sector_t); pub fn submit_bio_noacct(*mut bio); pub fn blk_status_to_errno(blk_status_t)->i32; pub fn errno_to_blk_status(i32)->blk_status_t; }

/* External kernel types referenced by this header. */
pub type sector_t=u64; pub type blk_mode_t=u32; pub type blk_status_t=u8; pub type req_op=u32; pub type xarray=opaque; pub type bio_set=opaque; pub type mutex=opaque; pub type backing_dev_info=opaque; pub type kobject=opaque; pub type timer_rand_state=opaque; pub type lockdep_map=opaque; pub type disk_events=opaque; pub type elevator_queue=opaque; pub type blk_mq_ops=opaque; pub type blk_mq_ctx=opaque; pub type refcount_t=opaque; pub type percpu_ref=opaque; pub type spinlock_t=opaque; pub type atomic_t=opaque; pub type blk_queue_stats=opaque; pub type rq_qos=opaque; pub type timer_list=opaque; pub type work_struct=opaque; pub type blk_mq_tags=opaque; pub type list_head=opaque; pub type rcu_head=opaque; pub type wait_queue_head_t=opaque; pub type blk_mq_tag_set=opaque; pub type dentry=opaque; pub type module=opaque; pub type block_device=opaque; pub type bio=opaque; pub type blk_zone=opaque; pub type blk_report_zones_args=opaque; pub type rq_list=opaque; pub type opaque=core::ffi::c_void; pub const UUID_STRING_LEN:u32=36;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
