/* SPDX-License-Identifier: GPL-2.0 */

// Linux kernel dependencies supplied by the surrounding translation unit.
use core::ffi::{c_char, c_void};

#[repr(C)] pub struct io_cq { _private: [u8; 0] }
#[repr(C)] pub struct blk_mq_debugfs_attr { _private: [u8; 0] }
#[repr(C)] pub struct blk_mq_alloc_data { _private: [u8; 0] }
#[repr(C)] pub struct blk_mq_hw_ctx { _private: [u8; 0] }
#[repr(C)] pub struct blk_mq_tags { _private: [u8; 0] }
#[repr(C)] pub struct request_queue { _private: [u8; 0] }
#[repr(C)] pub struct request { _private: [u8; 0] }
#[repr(C)] pub struct bio { _private: [u8; 0] }
#[repr(C)] pub struct attribute { _private: [u8; 0] }
#[repr(C)] pub struct module { _private: [u8; 0] }
#[repr(C)] pub struct kobject { _private: [u8; 0] }
#[repr(C)] pub struct mutex { _private: [u8; 0] }
#[repr(C)] pub struct list_head { _private: [u8; 0] }
#[repr(C)] pub struct rb_root { _private: [u8; 0] }
#[repr(C)] pub struct gendisk { _private: [u8; 0] }
#[repr(C)] pub struct hlist_head { _private: [u8; 0] }

pub type ssize_t = isize;
pub type size_t = usize;
pub type sector_t = u64;
pub type blk_opf_t = u32;
pub type blk_insert_t = u32;
pub type u64_ = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum elv_merge {
    ELEVATOR_NO_MERGE = 0,
    ELEVATOR_FRONT_MERGE = 1,
    ELEVATOR_BACK_MERGE = 2,
    ELEVATOR_DISCARD_MERGE = 3,
}

#[repr(C)]
pub struct elevator_tags {
    pub nr_hw_queues: u32,
    pub nr_requests: u32,
    pub tags: [*mut blk_mq_tags; 0],
}

#[repr(C)]
pub struct elevator_resources {
    pub data: *mut c_void,
    pub et: *mut elevator_tags,
}

#[repr(C)]
pub struct elv_change_ctx {
    pub name: *const c_char,
    pub no_uevent: bool,
    pub old: *mut elevator_queue,
    pub new: *mut elevator_queue,
    pub type_: *mut elevator_type,
    pub res: elevator_resources,
}

#[repr(C)]
pub struct elevator_mq_ops {
    pub init_sched: Option<unsafe extern "C" fn(*mut request_queue, *mut elevator_queue) -> i32>,
    pub exit_sched: Option<unsafe extern "C" fn(*mut elevator_queue)>,
    pub init_hctx: Option<unsafe extern "C" fn(*mut blk_mq_hw_ctx, u32) -> i32>,
    pub exit_hctx: Option<unsafe extern "C" fn(*mut blk_mq_hw_ctx, u32)>,
    pub depth_updated: Option<unsafe extern "C" fn(*mut request_queue)>,
    pub alloc_sched_data: Option<unsafe extern "C" fn(*mut request_queue) -> *mut c_void>,
    pub free_sched_data: Option<unsafe extern "C" fn(*mut c_void)>,
    pub allow_merge: Option<unsafe extern "C" fn(*mut request_queue, *mut request, *mut bio) -> bool>,
    pub bio_merge: Option<unsafe extern "C" fn(*mut request_queue, *mut bio, u32) -> bool>,
    pub request_merge: Option<unsafe extern "C" fn(*mut request_queue, *mut *mut request, *mut bio) -> i32>,
    pub request_merged: Option<unsafe extern "C" fn(*mut request_queue, *mut request, elv_merge)>,
    pub requests_merged: Option<unsafe extern "C" fn(*mut request_queue, *mut request, *mut request)>,
    pub limit_depth: Option<unsafe extern "C" fn(blk_opf_t, *mut blk_mq_alloc_data)>,
    pub prepare_request: Option<unsafe extern "C" fn(*mut request)>,
    pub finish_request: Option<unsafe extern "C" fn(*mut request)>,
    pub insert_requests: Option<unsafe extern "C" fn(*mut blk_mq_hw_ctx, *mut list_head, blk_insert_t)>,
    pub dispatch_request: Option<unsafe extern "C" fn(*mut blk_mq_hw_ctx) -> *mut request>,
    pub has_work: Option<unsafe extern "C" fn(*mut blk_mq_hw_ctx) -> bool>,
    pub completed_request: Option<unsafe extern "C" fn(*mut request, u64)>,
    pub requeue_request: Option<unsafe extern "C" fn(*mut request)>,
    pub former_request: Option<unsafe extern "C" fn(*mut request_queue, *mut request) -> *mut request>,
    pub next_request: Option<unsafe extern "C" fn(*mut request_queue, *mut request) -> *mut request>,
    pub init_icq: Option<unsafe extern "C" fn(*mut io_cq)>,
    pub exit_icq: Option<unsafe extern "C" fn(*mut io_cq)>,
}

pub const ELV_NAME_MAX: usize = 16;

#[repr(C)]
pub struct elv_fs_entry {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut elevator_queue, *mut c_char) -> ssize_t>,
    pub store: Option<unsafe extern "C" fn(*mut elevator_queue, *const c_char, size_t) -> ssize_t>,
}

#[repr(C)]
pub struct elevator_type {
    pub icq_cache: *mut c_void,
    pub ops: elevator_mq_ops,
    pub icq_size: size_t,
    pub icq_align: size_t,
    pub elevator_attrs: *const elv_fs_entry,
    pub elevator_name: *const c_char,
    pub elevator_alias: *const c_char,
    pub elevator_owner: *mut module,
    pub icq_cache_name: [c_char; ELV_NAME_MAX + 6],
    pub list: list_head,
}

pub const ELV_HASH_BITS: u32 = 6;

extern "C" {
    fn try_module_get(owner: *mut module) -> bool;
    fn __module_get(owner: *mut module);
    fn module_put(owner: *mut module);
}

#[inline]
pub unsafe fn elevator_tryget(e: *mut elevator_type) -> bool {
    try_module_get((*e).elevator_owner)
}

#[inline]
pub unsafe fn __elevator_get(e: *mut elevator_type) {
    __module_get((*e).elevator_owner)
}

#[inline]
pub unsafe fn elevator_put(e: *mut elevator_type) {
    module_put((*e).elevator_owner)
}

pub const ELEVATOR_FLAG_REGISTERED: u32 = 0;
pub const ELEVATOR_FLAG_DYING: u32 = 1;
pub const ELEVATOR_INSERT_FRONT: u32 = 1;
pub const ELEVATOR_INSERT_BACK: u32 = 2;
pub const ELEVATOR_INSERT_SORT: u32 = 3;
pub const ELEVATOR_INSERT_REQUEUE: u32 = 4;
pub const ELEVATOR_INSERT_FLUSH: u32 = 5;
pub const ELEVATOR_INSERT_SORT_MERGE: u32 = 6;

#[repr(C)]
pub struct elevator_queue {
    pub type_: *mut elevator_type,
    pub et: *mut elevator_tags,
    pub elevator_data: *mut c_void,
    pub kobj: kobject,
    pub sysfs_lock: mutex,
    pub flags: usize,
    pub hash: [hlist_head; 1 << ELV_HASH_BITS],
}

extern "C" {
    pub fn elv_rqhash_del(q: *mut request_queue, rq: *mut request);
    pub fn elv_rqhash_add(q: *mut request_queue, rq: *mut request);
    pub fn elv_rqhash_reposition(q: *mut request_queue, rq: *mut request);
    pub fn elv_rqhash_find(q: *mut request_queue, offset: sector_t) -> *mut request;
    pub fn elv_merge(q: *mut request_queue, rq: *mut *mut request, bio: *mut bio) -> elv_merge;
    pub fn elv_merge_requests(q: *mut request_queue, rq: *mut request, next: *mut request);
    pub fn elv_merged_request(q: *mut request_queue, rq: *mut request, merge: elv_merge);
    pub fn elv_attempt_insert_merge(q: *mut request_queue, rq: *mut request, list: *mut list_head) -> bool;
    pub fn elv_former_request(q: *mut request_queue, rq: *mut request) -> *mut request;
    pub fn elv_latter_request(q: *mut request_queue, rq: *mut request) -> *mut request;
    pub fn elv_register(type_: *mut elevator_type) -> i32;
    pub fn elv_unregister(type_: *mut elevator_type);
    pub fn elv_iosched_show(disk: *mut gendisk, page: *mut c_char) -> ssize_t;
    pub fn elv_iosched_store(disk: *mut gendisk, page: *const c_char, count: size_t) -> ssize_t;
    pub fn elv_bio_merge_ok(rq: *mut request, bio: *mut bio) -> bool;
    pub fn elevator_alloc(q: *mut request_queue, type_: *mut elevator_type, res: *mut elevator_resources) -> *mut elevator_queue;
    pub fn elv_rb_former_request(q: *mut request_queue, rq: *mut request) -> *mut request;
    pub fn elv_rb_latter_request(q: *mut request_queue, rq: *mut request) -> *mut request;
    pub fn elv_rb_add(root: *mut rb_root, rq: *mut request);
    pub fn elv_rb_del(root: *mut rb_root, rq: *mut request);
    pub fn elv_rb_find(root: *mut rb_root, sector: sector_t) -> *mut request;
    pub fn blk_mq_sched_reg_debugfs(q: *mut request_queue);
    pub fn blk_mq_sched_unreg_debugfs(q: *mut request_queue);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
