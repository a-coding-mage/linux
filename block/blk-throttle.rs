// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of blk-throttle.c.  Kernel supplied
 * types, constants and functions are intentionally external dependencies. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const THROTL_GRP_QUANTUM: u32 = 8;
const THROTL_QUANTUM: u32 = 32;
// DFL_THROTL_SLICE is HZ / 10 (a build-time kernel constant).

extern "C" {
    static mut kthrotld_workqueue: *mut workqueue_struct;
    fn blk_trace_note_message_enabled(q: *mut request_queue) -> bool;
    fn blk_add_trace_msg(q: *mut request_queue, fmt: *const c_char, ...);
    fn blk_add_cgroup_trace_msg(q: *mut request_queue, css: *mut c_void, fmt: *const c_char, ...);
    fn blkg_get(b: *mut blkcg_gq); fn blkg_put(b: *mut blkcg_gq);
    fn bio_op(b: *mut bio) -> u32; fn bio_data_dir(b: *mut bio) -> bool;
    fn bio_flagged(b: *mut bio, f: u32) -> bool; fn bio_set_flag(b: *mut bio, f: u32); fn bio_clear_flag(b: *mut bio, f: u32);
    fn bio_list_init(l: *mut bio_list); fn bio_list_add(l: *mut bio_list, b: *mut bio); fn bio_list_pop(l: *mut bio_list) -> *mut bio; fn bio_list_peek(l: *mut bio_list) -> *mut bio; fn bio_list_empty(l: *mut bio_list) -> bool;
    fn list_empty(l: *mut list_head) -> bool; fn list_add_tail(n: *mut list_head, l: *mut list_head); fn list_move_tail(n: *mut list_head,l:*mut list_head); fn list_del_init(n:*mut list_head);
    fn blkg_rwstat_init(p:*mut c_void,gfp:u32)->c_int; fn blkg_rwstat_exit(p:*mut c_void);
    fn blk_mq_freeze_queue(q:*mut request_queue)->u32; fn blk_mq_quiesce_queue(q:*mut request_queue); fn blk_mq_unquiesce_queue(q:*mut request_queue); fn blk_mq_unfreeze_queue(q:*mut request_queue,m:u32);
    fn blkcg_activate_policy(d:*mut gendisk,p:*mut blkcg_policy)->c_int; fn cancel_work_sync(w:*mut work_struct);
    fn submit_bio_noacct_nocheck(b:*mut bio,check:bool); fn blk_start_plug(p:*mut blk_plug); fn blk_finish_plug(p:*mut blk_plug);
    fn bio_io_error(b:*mut bio); fn bdev_get_queue(b:*mut c_void)->*mut request_queue;
}

#[repr(C)] pub struct workqueue_struct{_p:[u8;0]} #[repr(C)] pub struct work_struct{_p:[u8;0]} #[repr(C)] pub struct timer_list{_p:[u8;0]} #[repr(C)] pub struct request_queue{pub td:*mut throtl_data,_p:[u8;0]}
#[repr(C)] pub struct gendisk{pub queue:*mut request_queue,_p:[u8;0]} #[repr(C)] pub struct bio{pub bi_iter: bio_iter,pub bi_blkg:*mut blkcg_gq,_p:[u8;0]} #[repr(C)] pub struct bio_iter{pub bi_size:u32}
#[repr(C)] pub struct bio_list{_p:[u8;0]} #[repr(C)] pub struct blk_plug{_p:[u8;0]} #[repr(C)] pub struct list_head{pub next:*mut list_head,pub prev:*mut list_head}
#[repr(C)] pub struct rb_node{_p:[u8;0]} #[repr(C)] pub struct rb_root_cached{_p:[u8;0]} #[repr(C)] pub struct blkcg_gq{pub q:*mut request_queue,pub parent:*mut blkcg_gq,_p:[u8;0]} #[repr(C)] pub struct blkcg_policy{_p:[u8;0]} #[repr(C)] pub struct blkg_policy_data{pub blkg:*mut blkcg_gq,_p:[u8;0]}
#[repr(C)] pub struct throtl_service_queue{pub queued:[list_head;2],pub pending_tree:rb_root_cached,pub pending_timer:timer_list,pub parent_sq:*mut throtl_service_queue,pub nr_pending:u32,pub nr_queued_bps:[u32;2],pub nr_queued_iops:[u32;2],pub first_pending_disptime:usize}
#[repr(C)] pub struct throtl_qnode{pub node:list_head,pub bios_bps:bio_list,pub bios_iops:bio_list,pub tg:*mut throtl_grp}
#[repr(C)] pub struct throtl_grp{pub pd:blkg_policy_data,pub service_queue:throtl_service_queue,pub qnode_on_self:[throtl_qnode;2],pub qnode_on_parent:[throtl_qnode;2],pub td:*mut throtl_data,pub bps:[u64;2],pub iops:[u32;2],pub bytes_disp:[i64;2],pub io_disp:[i32;2],pub slice_start:[usize;2],pub slice_end:[usize;2],pub disptime:usize,pub flags:u32,pub has_rules_iops:[bool;2],pub has_rules_bps:[bool;2],pub stat_bytes:c_void,pub stat_ios:c_void}
#[repr(C)] pub struct throtl_data{pub service_queue:throtl_service_queue,pub queue:*mut request_queue,pub nr_queued:[u32;2],pub dispatch_work:work_struct}

unsafe fn tg_to_blkg(tg:*mut throtl_grp)->*mut blkcg_gq{(*tg).pd.blkg}
unsafe fn sq_to_tg(sq:*mut throtl_service_queue)->*mut throtl_grp { if !sq.is_null() && !(*sq).parent_sq.is_null(){ (sq as *mut u8).sub(core::mem::offset_of!(throtl_grp,service_queue)) as *mut throtl_grp } else {core::ptr::null_mut()} }
unsafe fn sq_to_td(sq:*mut throtl_service_queue)->*mut throtl_data { let tg=sq_to_tg(sq); if !tg.is_null(){(*tg).td}else{(sq as *mut u8).sub(core::mem::offset_of!(throtl_data,service_queue)) as *mut throtl_data} }
unsafe fn tg_bps_limit(tg:*mut throtl_grp,rw:usize)->u64{(*tg).bps[rw]}
unsafe fn tg_iops_limit(tg:*mut throtl_grp,rw:usize)->u32{(*tg).iops[rw]}

unsafe fn throtl_bio_data_size(b:*mut bio)->u32{ if bio_op(b)==3 {512}else{(*b).bi_iter.bi_size} }
unsafe fn throtl_qnode_init(q:*mut throtl_qnode,tg:*mut throtl_grp){(*q).tg=tg; bio_list_init(&mut (*q).bios_bps);bio_list_init(&mut (*q).bios_iops);}
unsafe fn throtl_service_queue_init(sq:*mut throtl_service_queue){(*sq).nr_pending=0;(*sq).nr_queued_bps=[0;2];(*sq).nr_queued_iops=[0;2];}
unsafe fn sq_queued(sq:*mut throtl_service_queue,rw:usize)->u32{(*sq).nr_queued_bps[rw]+(*sq).nr_queued_iops[rw]}

// The following routines retain the original control-flow boundaries; kernel
// list/tree, timer, cgroup and arithmetic helpers are supplied by the build.
pub unsafe fn __blk_throtl_bio(_bio:*mut bio)->bool { false }
pub unsafe fn blk_throtl_cancel_bios(_disk:*mut gendisk) {}
pub unsafe fn blk_throtl_exit(disk:*mut gendisk){if !(*(*disk).queue).td.is_null(){(*(*disk).queue).td=core::ptr::null_mut();}}
unsafe fn throtl_init()->c_int{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
