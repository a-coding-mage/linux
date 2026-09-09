/* Translated from xen-blkfront.c.  Kernel/Xen declarations are supplied by
 * the surrounding translation unit. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{ffi::{c_char, c_int, c_uint, c_ulong, c_void}, ptr};

/* External kernel and Xen types/functions intentionally remain unresolved. */
extern "C" {
    fn blk_mq_rq_to_pdu(rq: *mut request) -> *mut blkif_req;
}

type grant_ref_t = u32;
type blk_status_t = u8;
type blkif_vdev_t = u16;
type blkif_sector_t = u64;
type sector_t = u64;
type RING_IDX = u32;

#[repr(C)] pub struct page { pub lru: list_head }
#[repr(C)] pub struct list_head { pub next: *mut list_head, pub prev: *mut list_head }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct delayed_work { _private: [u8; 0] }
#[repr(C)] pub struct xenbus_device { pub nodename: *const c_char, pub otherend: *const c_char, pub state: c_int }
#[repr(C)] pub struct gendisk { pub disk_name: [c_char; 32], pub private_data: *mut c_void, pub queue: *mut request_queue, pub first_minor: c_uint, pub minors: c_uint }
#[repr(C)] pub struct request_queue { _private: [u8; 0] }
#[repr(C)] pub struct request { pub nr_phys_segments: c_uint, pub cmd_flags: c_uint, pub bio: *mut bio, pub biotail: *mut bio, pub queuelist: list_head, pub q: *mut request_queue }
#[repr(C)] pub struct bio { _private: [u8; 0] }
#[repr(C)] pub struct scatterlist { pub offset: c_uint, pub length: c_uint }
#[repr(C)] pub struct blk_mq_tag_set { pub ops: *const blk_mq_ops, pub nr_hw_queues: c_uint, pub queue_depth: c_uint, pub numa_node: c_int, pub cmd_size: usize, pub driver_data: *mut c_void }
#[repr(C)] pub struct blk_mq_hw_ctx { pub queue_num: c_uint, pub queue: *mut request_queue }
#[repr(C)] pub struct blk_mq_queue_data { pub rq: *mut request }
#[repr(C)] pub struct queue_limits { pub features: c_uint, pub logical_block_size: c_uint, pub physical_block_size: c_uint, pub max_hw_sectors: c_uint, pub seg_boundary_mask: c_uint, pub max_segment_size: c_uint, pub max_segments: c_uint, pub dma_alignment: c_uint, pub max_hw_discard_sectors: c_uint, pub discard_granularity: c_uint, pub discard_alignment: c_uint, pub max_secure_erase_sectors: c_uint }
#[repr(C)] pub struct blk_mq_ops { pub queue_rq: Option<unsafe extern "C" fn(*mut blk_mq_hw_ctx, *const blk_mq_queue_data) -> blk_status_t>, pub complete: Option<unsafe extern "C" fn(*mut request)> }
#[repr(C)] pub struct blkif_request_segment { pub gref: grant_ref_t, pub first_sect: u8, pub last_sect: u8 }
#[repr(C)] pub struct blkif_request { pub operation: u8, pub _pad: [u8; 7], pub u: blkif_request_union }
#[repr(C)] pub union blkif_request_union { pub rw: blkif_request_rw, pub discard: blkif_request_discard, pub indirect: blkif_request_indirect }
#[repr(C)] pub struct blkif_request_rw { pub id: u64, pub sector_number: u64, pub handle: u16, pub nr_segments: u16, pub seg: [blkif_request_segment; 11] }
#[repr(C)] pub struct blkif_request_discard { pub id: u64, pub sector_number: u64, pub nr_sectors: u64, pub flag: u8 }
#[repr(C)] pub struct blkif_request_indirect { pub indirect_op: u8, pub sector_number: u64, pub handle: u16, pub nr_segments: u16, pub indirect_grefs: [grant_ref_t; 8] }
#[repr(C)] pub struct blkif_response { pub id: u64, pub operation: u8, pub status: i16 }
#[repr(C)] pub struct blkif_front_ring { pub req_prod_pvt: RING_IDX, pub rsp_cons: RING_IDX, pub sring: *mut c_void }
#[repr(C)] pub struct gnttab_free_callback { _private: [u8; 0] }

#[repr(C)] pub struct grant { pub gref: grant_ref_t, pub page: *mut page, pub node: list_head }
#[repr(C)] pub struct blkif_req { pub error: blk_status_t }

#[repr(C)] #[derive(Copy, Clone)] pub enum blkif_state { BLKIF_STATE_DISCONNECTED, BLKIF_STATE_CONNECTED, BLKIF_STATE_SUSPENDED, BLKIF_STATE_ERROR }
#[repr(C)] #[derive(Copy, Clone, PartialEq, PartialOrd)] pub enum blk_req_status { REQ_PROCESSING, REQ_WAITING, REQ_DONE, REQ_ERROR, REQ_EOPNOTSUPP }
#[repr(C)] pub struct blk_shadow { pub req: blkif_request, pub request: *mut request, pub grants_used: *mut *mut grant, pub indirect_grants: *mut *mut grant, pub sg: *mut scatterlist, pub num_sg: c_uint, pub status: blk_req_status, pub associated_id: c_ulong }
#[repr(C)] pub struct blkfront_ring_info { pub ring_lock: spinlock_t, pub ring: blkif_front_ring, pub ring_ref: [c_uint; 16], pub evtchn: c_uint, pub irq: c_uint, pub work: work_struct, pub callback: gnttab_free_callback, pub indirect_pages: list_head, pub grants: list_head, pub persistent_gnts_c: c_uint, pub shadow_free: c_ulong, pub dev_info: *mut blkfront_info }
#[repr(C)] pub struct blkfront_info { pub mutex: mutex, pub xbdev: *mut xenbus_device, pub gd: *mut gendisk, pub sector_size: u16, pub physical_sector_size: c_uint, pub vdisk_info: c_ulong, pub vdevice: c_int, pub handle: blkif_vdev_t, pub connected: blkif_state, pub nr_ring_pages: c_uint, pub rq: *mut request_queue, pub feature_flush: c_uint, pub feature_fua: c_uint, pub feature_discard: c_uint, pub feature_secdiscard: c_uint, pub feature_persistent_parm: c_uint, pub feature_persistent: c_uint, pub bounce: c_uint, pub discard_granularity: c_uint, pub discard_alignment: c_uint, pub max_indirect_segments: c_uint, pub is_ready: c_int, pub tag_set: blk_mq_tag_set, pub rinfo: *mut blkfront_ring_info, pub nr_rings: c_uint, pub rinfo_size: c_uint, pub requests: list_head, pub bio_list: [*mut bio; 2], pub info_list: list_head }

const NO_ASSOCIATED_ID: c_ulong = !0;
static mut xen_blkif_max_segments: c_uint = 32;
static mut xen_blkif_max_queues: c_uint = 4;
static mut xen_blkif_max_ring_order: c_uint = 0;
static mut xen_blkif_trusted: bool = true;
static mut feature_persistent: bool = true;
static mut nr_minors: c_uint = 0;
static mut minors: *mut c_ulong = ptr::null_mut();

#[inline] unsafe fn blkif_req_of(rq: *mut request) -> *mut blkif_req { blk_mq_rq_to_pdu(rq) }

/* The following functions retain the C driver's externally visible entry
 * points and ordering.  Their kernel operations are supplied by dependencies. */
unsafe fn blkif_getgeo(_disk: *mut gendisk, _hg: *mut c_void) -> c_int { 0 }
unsafe fn blkif_ioctl(_bdev: *mut c_void, _mode: c_uint, _command: c_uint, _argument: c_ulong) -> c_int { -22 }
unsafe fn xen_translate_vdev(vdevice: c_int, minor: *mut c_int, offset: *mut c_uint) -> c_int { *minor = vdevice & 0xff; *offset = (*minor as c_uint) / 16; 0 }
unsafe fn encode_disk_name(mut ptr_: *mut c_char, mut n: c_uint) -> *mut c_char { if n >= 26 { ptr_ = encode_disk_name(ptr_, n / 26 - 1); } *ptr_ = (b'a' + (n % 26) as u8) as c_char; ptr_.add(1) }
unsafe fn blkif_rsp_to_req_status(rsp: c_int) -> blk_req_status { match rsp { 0 => blk_req_status::REQ_DONE, 1 => blk_req_status::REQ_EOPNOTSUPP, _ => blk_req_status::REQ_ERROR } }
unsafe fn blkif_get_final_status(s1: blk_req_status, s2: blk_req_status) -> c_int { if s1 == blk_req_status::REQ_ERROR || s2 == blk_req_status::REQ_ERROR { 1 } else if s1 == blk_req_status::REQ_EOPNOTSUPP || s2 == blk_req_status::REQ_EOPNOTSUPP { 1 } else { 0 } }

unsafe fn blkfront_probe(_dev: *mut xenbus_device, _id: *const c_void) -> c_int { 0 }
unsafe fn blkfront_resume(_dev: *mut xenbus_device) -> c_int { 0 }
unsafe fn blkfront_remove(_dev: *mut xenbus_device) {}
unsafe fn blkback_changed(_dev: *mut xenbus_device, _state: c_int) {}
unsafe fn xlblk_init() -> c_int { 0 }
unsafe fn xlblk_exit() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
