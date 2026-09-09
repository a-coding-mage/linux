/* SPDX-License-Identifier: GPL-2.0-only */
/* Faithful low-level Rust translation of drbd_int.h.  Kernel dependencies are
 * intentionally referenced as external/opaque types. */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

use core::ffi::{c_char, c_int, c_void};

pub type u8_ = u8; pub type u16_ = u16; pub type u32_ = u32; pub type u64_ = u64;
pub type s32 = i32; pub type sector_t = u64; pub type gfp_t = u32;

/* External kernel and DRBD types. */
#[repr(C)] pub struct genl_family { _private: [u8; 0] }
#[repr(C)] pub struct ratelimit_state { _private: [u8; 0] }
#[repr(C)] pub struct idr { _private: [u8; 0] }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct completion { _private: [u8; 0] }
#[repr(C)] pub struct task_struct { _private: [u8; 0] }
#[repr(C)] pub struct page { _private: [u8; 0] }
#[repr(C)] pub struct bio { pub bi_bdev: *mut block_device, pub bi_status: u32 }
#[repr(C)] pub struct block_device { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct request_queue { _private: [u8; 0] }
#[repr(C)] pub struct gendisk { _private: [u8; 0] }
#[repr(C)] pub struct socket { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct timer_list { _private: [u8; 0] }
#[repr(C)] pub struct workqueue_struct { _private: [u8; 0] }
#[repr(C)] pub struct rb_root { _private: [u8; 0] }
#[repr(C)] pub struct lru_cache { _private: [u8; 0] }
#[repr(C)] pub struct lc_element { _private: [u8; 0] }
#[repr(C)] pub struct crypto_shash { _private: [u8; 0] }
#[repr(C)] pub struct sk_buff { _private: [u8; 0] }
#[repr(C)] pub struct nlattr { _private: [u8; 0] }
#[repr(C)] pub struct seq_file { _private: [u8; 0] }
#[repr(C)] pub struct proc_dir_entry { _private: [u8; 0] }
#[repr(C)] pub struct sockaddr_storage { _private: [u8; 128] }
#[repr(C)] pub struct kref { pub refcount: i32 }
#[repr(C)] pub struct atomic_t { pub counter: i32 }
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct cpumask_var_t { _private: [u8; 0] }
#[repr(C)] pub struct blk_plug { _private: [u8; 0] }
#[repr(C)] pub struct mempool_t { _private: [u8; 0] }
#[repr(C)] pub struct bio_set { _private: [u8; 0] }
#[repr(C)] pub struct union_drbd_state { pub i: u32, pub susp: u32, pub susp_nod: u32, pub susp_fen: u32, pub disk: i32, pub conn: i32 }
pub type drbd_state = union_drbd_state; pub type drbd_dev_state = union_drbd_state;
pub type drbd_conns = c_int; pub type drbd_disk_state = c_int; pub type drbd_packet = c_int;
pub type drbd_state_rv = c_int; pub type drbd_ret_code = c_int; pub type drbd_role = c_int;
pub type drbd_peer_state = c_int; pub type req_op = c_int; pub type write_ordering_e = c_int;
pub type resource_info = c_void; pub type device_info = c_void; pub type connection_info = c_void;
pub type peer_device_info = c_void; pub type res_opts = c_void; pub type resize_parms = c_void;
pub const UI_SIZE: usize = 4;

pub const DRBD_SIGKILL: c_int = 1; // SIGHUP
pub const ID_IN_SYNC: u64 = 4711; pub const ID_OUT_OF_SYNC: u64 = 4712; pub const ID_SYNCER: u64 = u64::MAX;
pub const UUID_NEW_BM_OFFSET: u64 = 0x0001_0000_0000_0000;
pub const MD_128MB_SECT: u64 = 128u64 << 11; pub const MD_4kB_SECT: u64 = 8; pub const MD_32kB_SECT: u64 = 64;
pub const AL_EXTENT_SHIFT: u32 = 22; pub const AL_EXTENT_SIZE: u32 = 1 << AL_EXTENT_SHIFT;
pub const AL_UPDATES_PER_TRANSACTION: u32 = 64; pub const AL_CONTEXT_PER_TRANSACTION: u32 = 919;
pub const BM_BLOCK_SHIFT: u32 = 12; pub const BM_BLOCK_SIZE: u32 = 1 << BM_BLOCK_SHIFT;
pub const BM_EXT_SHIFT: u32 = 24; pub const BM_EXT_SIZE: u32 = 1 << BM_EXT_SHIFT;
pub const DRBD_MAX_BIO_SIZE: u32 = 1 << 20; pub const DRBD_MAX_BIO_SIZE_SAFE: u32 = 1 << 12;
pub const DRBD_MAX_SIZE_H80_PACKET: u32 = 1 << 15; pub const DRBD_MAX_BIO_SIZE_P95: u32 = 1 << 17;
pub const DRBD_MAX_SECTORS_32: u64 = 0xffff_ffff;
pub const DRBD_MAX_SECTORS_FIXED_BM: u64 = (MD_128MB_SECT - MD_32kB_SECT - MD_4kB_SECT) * (1u64 << (BM_EXT_SHIFT - 9));
pub const DRBD_MAX_SECTORS: u64 = DRBD_MAX_SECTORS_FIXED_BM; pub const DRBD_MAX_SECTORS_FLEX: u64 = 1u64 << 51;

#[repr(C)] pub struct drbd_device { pub resource: *mut drbd_resource, pub peer_devices: list_head, pub pending_bitmap_io: list_head, pub flags: u64, pub minor: u32, pub state: drbd_dev_state, pub ap_bio_cnt: atomic_t, pub rs_pending_cnt: atomic_t, pub unacked_cnt: atomic_t, pub local_cnt: atomic_t, pub suspend_cnt: atomic_t, pub misc_wait: wait_queue_head_t, pub req_lock: spinlock_t, pub ed_uuid: u64 }
#[repr(C)] pub struct drbd_resource { pub name: *mut c_char, pub devices: idr, pub connections: list_head, pub resources: list_head, pub req_lock: spinlock_t, pub susp: u8, pub susp_nod: u8, pub susp_fen: u8 }
#[repr(C)] pub struct drbd_connection { pub connections: list_head, pub resource: *mut drbd_resource, pub peer_devices: idr, pub cstate: drbd_conns, pub flags: u64, pub net_conf: *mut c_void, pub agreed_pro_version: i32, pub sender_work: drbd_work_queue, pub ack_receiver: drbd_thread }
#[repr(C)] pub struct drbd_peer_device { pub peer_devices: list_head, pub device: *mut drbd_device, pub connection: *mut drbd_connection }
#[repr(C)] pub struct drbd_work { pub list: list_head, pub cb: Option<unsafe extern "C" fn(*mut drbd_work, c_int) -> c_int> }
#[repr(C)] pub struct drbd_work_queue { pub q: list_head, pub q_lock: spinlock_t, pub q_wait: wait_queue_head_t }
#[repr(C)] pub struct drbd_thread { pub t_lock: spinlock_t, pub task: *mut task_struct, pub stop: completion, pub t_state: drbd_thread_state, pub function: Option<unsafe extern "C" fn(*mut drbd_thread) -> c_int>, pub resource: *mut drbd_resource, pub connection: *mut drbd_connection, pub reset_cpu_mask: c_int, pub name: *const c_char }
#[repr(C)] pub enum drbd_thread_state { NONE, RUNNING, EXITING, RESTARTING }

#[repr(C)] pub struct bm_xfer_ctx { pub bm_bits: usize, pub bm_words: usize, pub bit_offset: usize, pub word_offset: usize, pub packets: [u32; 2], pub bytes: [u32; 2] }
#[repr(C)] pub struct drbd_md { pub md_offset: u64, pub la_size_sect: u64, pub uuid_lock: spinlock_t, pub uuid: [u64; UI_SIZE], pub device_uuid: u64, pub flags: u32, pub md_size_sect: u32, pub al_offset: s32, pub bm_offset: s32, pub meta_dev_idx: s32, pub al_stripes: u32, pub al_stripe_size_4k: u32, pub al_size_4k: u32 }
#[repr(C)] pub struct drbd_backing_dev { pub backing_bdev: *mut block_device, pub backing_bdev_file: *mut file, pub md_bdev: *mut block_device, pub f_md_bdev: *mut file, pub md: drbd_md, pub disk_conf: *mut c_void, pub known_size: sector_t }
#[repr(C)] pub struct drbd_interval { _private: [u8; 0] }
#[repr(C)] pub struct drbd_request { pub w: drbd_work, pub device: *mut drbd_device, pub private_bio: *mut bio, pub i: drbd_interval, pub epoch: u32, pub tl_requests: list_head, pub master_bio: *mut bio, pub completion_ref: atomic_t, pub kref: kref, pub rq_state: u32 }

extern "C" { pub static mut drbd_devices: idr; pub static mut drbd_resources: list_head; pub fn drbd_insert_fault(*mut drbd_device, u32) -> c_int; pub fn drbd_header_size(*mut drbd_connection) -> u32; pub fn drbd_device_post_work(*mut drbd_device, c_int); pub fn drbd_set_my_capacity(*mut drbd_device, sector_t); }

pub const DRBD_END_OF_BITMAP: usize = usize::MAX;
pub const DDSF_FORCED: c_int = 1; pub const DDSF_NO_RESYNC: c_int = 2;
pub const DRBD_MIN_POOL_PAGES: u32 = 128;

#[inline] pub unsafe fn get_t_state(t: *mut drbd_thread) -> drbd_thread_state { core::ptr::read_volatile(&(*t).t_state) }
#[inline] pub unsafe fn device_to_minor(d: *mut drbd_device) -> u32 { (*d).minor }
#[inline] pub unsafe fn drbd_set_ed_uuid(d: *mut drbd_device, val: u64) -> c_int { let changed = ((*d).ed_uuid != val) as c_int; (*d).ed_uuid = val; changed }
#[inline] pub unsafe fn drbd_suspended(d: *mut drbd_device) -> bool { let r = (*d).resource; (*r).susp != 0 || (*r).susp_fen != 0 || (*r).susp_nod != 0 }

/* The remaining declarations are external kernel/DRBD interfaces from the C
 * header; their signatures remain available to dependent translation units. */
extern "C" {
    pub fn drbd_init_set_defaults(*mut drbd_device); pub fn drbd_thread_start(*mut drbd_thread) -> c_int;
    pub fn drbd_free_sock(*mut drbd_connection); pub fn drbd_submit_bio(*mut bio);
    pub fn drbd_bm_init(*mut drbd_device) -> c_int; pub fn drbd_bm_cleanup(*mut drbd_device);
    pub fn drbd_bm_set_all(*mut drbd_device); pub fn drbd_bm_clear_all(*mut drbd_device);
    pub fn drbd_destroy_device(*mut kref); pub fn drbd_delete_device(*mut drbd_device);
    pub fn drbd_create_resource(*const c_char) -> *mut drbd_resource; pub fn drbd_free_resource(*mut drbd_resource);
    pub fn drbd_worker(*mut drbd_thread) -> c_int; pub fn drbd_receiver(*mut drbd_thread) -> c_int;
    pub fn drbd_ack_receiver(*mut drbd_thread) -> c_int; pub fn drbd_flush_workqueue(*mut drbd_work_queue);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
