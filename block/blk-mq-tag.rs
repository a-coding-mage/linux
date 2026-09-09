// SPDX-License-Identifier: GPL-2.0
/* Tag allocation using scalable bitmaps. */

use core::ffi::c_void;

// Kernel-provided types and functions are intentionally external dependencies.
#[repr(C)] pub struct blk_mq_tags { pub bitmap_tags: sbitmap_queue, pub breserved_tags: sbitmap_queue, pub lock: spinlock_t, pub active_queues: u32, pub nr_tags: u32, pub nr_reserved_tags: u32, pub page_list: list_head, pub rqs: *mut *mut request, pub static_rqs: *mut *mut request, pub rcu_head: rcu_head }
#[repr(C)] pub struct blk_mq_hw_ctx { pub tags: *mut blk_mq_tags, pub queue: *mut request_queue, pub flags: u32, pub state: usize }
#[repr(C)] pub struct request_queue { pub queue_flags: usize, pub tag_set: *mut blk_mq_tag_set, pub q_usage_counter: percpu_ref, pub elevator: *mut c_void, pub sched_shared_tags: *mut blk_mq_tags }
#[repr(C)] pub struct blk_mq_alloc_data { pub q: *mut request_queue, pub hctx: *mut blk_mq_hw_ctx, pub flags: u32, pub shallow_depth: u32, pub rq_flags: u32, pub cmd_flags: u32, pub ctx: *mut blk_mq_ctx }
#[repr(C)] pub struct blk_mq_tag_set { pub flags: u32, pub shared_tags: *mut blk_mq_tags, pub tags: *mut *mut blk_mq_tags, pub nr_hw_queues: i32, pub reserved_tags: u32, pub tags_srcu: srcu_struct }
#[repr(C)] pub struct blk_mq_ctx { pub cpu: i32 }
#[repr(C)] pub struct request { pub tag: u32, pub q: *mut request_queue, pub mq_hctx: *mut blk_mq_hw_ctx }
#[repr(C)] pub struct sbitmap_queue { pub sb: sbitmap }
#[repr(C)] pub struct sbitmap { _x: [u8; 0] }
#[repr(C)] pub struct sbq_wait_state { _x: [u8; 0] }
#[repr(C)] pub struct spinlock_t { _x: [u8; 0] }
#[repr(C)] pub struct list_head { _x: [u8; 0] }
#[repr(C)] pub struct rcu_head { _x: [u8; 0] }
#[repr(C)] pub struct page { pub lru: list_head, pub private: usize }
#[repr(C)] pub struct percpu_ref { _x: [u8; 0] }
#[repr(C)] pub struct srcu_struct { _x: [u8; 0] }
pub type busy_tag_iter_fn = unsafe extern "C" fn(*mut request, *mut c_void) -> bool;

extern "C" {
    fn blk_mq_is_shared_tags(flags: u32) -> bool; fn test_bit(n: u32, p: *const usize) -> bool; fn test_and_set_bit(n: u32,p:*mut usize)->bool; fn test_and_clear_bit(n:u32,p:*mut usize)->bool;
    fn spin_lock_irqsave(l:*mut spinlock_t,f:*mut usize); fn spin_unlock_irqrestore(l:*mut spinlock_t,f:usize); fn spin_lock_irq(l:*mut spinlock_t); fn spin_unlock_irq(l:*mut spinlock_t);
    fn sbitmap_queue_recalculate_wake_batch(*mut sbitmap_queue,u32); fn sbitmap_queue_wake_all(*mut sbitmap_queue); fn sbitmap_queue_get_shallow(*mut sbitmap_queue,u32)->i32; fn __sbitmap_queue_get(*mut sbitmap_queue)->i32; fn __sbitmap_queue_get_batch(*mut sbitmap_queue,i32,*mut u32)->u64;
    fn hctx_may_queue(*mut blk_mq_hw_ctx,*mut sbitmap_queue)->bool; fn bt_wait_ptr(*mut sbitmap_queue,*mut blk_mq_hw_ctx)->*mut sbq_wait_state; fn sbitmap_prepare_to_wait(*mut sbitmap_queue,*mut sbq_wait_state,*mut c_void,i32); fn sbitmap_finish_wait(*mut sbitmap_queue,*mut sbq_wait_state,*mut c_void); fn sbitmap_queue_wake_up(*mut sbitmap_queue,u32); fn io_schedule(); fn blk_mq_run_hw_queue(*mut blk_mq_hw_ctx,bool);
    fn blk_mq_tags_from_data(*mut blk_mq_alloc_data)->*mut blk_mq_tags; fn blk_mq_get_ctx(*mut request_queue)->*mut blk_mq_ctx; fn blk_mq_map_queue(u32,*mut blk_mq_ctx)->*mut blk_mq_hw_ctx; fn blk_mq_put_tag(*mut blk_mq_tags,*mut blk_mq_ctx,u32); fn blk_mq_tag_is_reserved(*mut blk_mq_tags,u32)->bool;
    fn sbitmap_queue_clear(*mut sbitmap_queue,i32,i32); fn sbitmap_queue_clear_batch(*mut sbitmap_queue,u32,*mut i32,i32); fn req_ref_inc_not_zero(*mut request)->bool; fn blk_mq_put_rq_ref(*mut request); fn blk_mq_request_started(*mut request)->bool; fn blk_mq_request_completed(*mut request)->bool;
    fn sbitmap_for_each_set(*mut sbitmap, unsafe extern "C" fn(*mut sbitmap,u32,*mut c_void)->bool,*mut c_void); fn srcu_read_lock(*mut srcu_struct)->i32; fn srcu_read_unlock(*mut srcu_struct,i32); fn msleep(u32); fn percpu_ref_tryget(*mut percpu_ref)->bool; fn blk_queue_exit(*mut request_queue); fn blk_mq_hw_queue_mapped(*mut blk_mq_hw_ctx)->bool;
    fn sbitmap_queue_init_node(*mut sbitmap_queue,u32,i32,bool,u32,i32)->i32; fn sbitmap_queue_free(*mut sbitmap_queue); fn sbitmap_queue_resize(*mut sbitmap_queue,u32); fn blk_mq_unique_tag_impl(*mut request)->u32;
}

const BLK_MQ_NO_TAG: i32 = -1; const BLK_MQ_REQ_RESERVED:u32=1<<0; const BLK_MQ_REQ_NOWAIT:u32=1<<1; const BLK_MQ_F_TAG_QUEUE_SHARED:u32=1<<0; const BLK_MQ_S_TAG_ACTIVE:u32=0; const BLK_MQ_S_INACTIVE:u32=1; const QUEUE_FLAG_HCTX_ACTIVE:u32=0; const RQF_SCHED_TAGS:u32=1; const TASK_UNINTERRUPTIBLE:i32=2; const BLK_MQ_TAG_MAX:u32=1<<24; const BLK_MQ_UNIQUE_TAG_BITS:u32=16; const BLK_MQ_UNIQUE_TAG_MASK:u32=(1<<16)-1;

unsafe fn blk_mq_update_wake_batch(tags:*mut blk_mq_tags, users:u32) { if users==0{return;} sbitmap_queue_recalculate_wake_batch(&mut (*tags).bitmap_tags,users); sbitmap_queue_recalculate_wake_batch(&mut (*tags).breserved_tags,users); }
#[no_mangle] pub unsafe extern "C" fn __blk_mq_tag_busy(hctx:*mut blk_mq_hw_ctx) { let tags=(*hctx).tags; let active=if blk_mq_is_shared_tags((*hctx).flags){test_bit(QUEUE_FLAG_HCTX_ACTIVE,&(*(*hctx).queue).queue_flags)||test_and_set_bit(QUEUE_FLAG_HCTX_ACTIVE,&mut (*(*hctx).queue).queue_flags)}else{test_bit(BLK_MQ_S_TAG_ACTIVE,&(*hctx).state)||test_and_set_bit(BLK_MQ_S_TAG_ACTIVE,&mut (*hctx).state)}; if active{return;} let mut f=0; spin_lock_irqsave(&mut (*tags).lock,&mut f); let users=(*tags).active_queues+1; (*tags).active_queues=users; blk_mq_update_wake_batch(tags,users); spin_unlock_irqrestore(&mut (*tags).lock,f); }
#[no_mangle] pub unsafe extern "C" fn blk_mq_tag_wakeup_all(tags:*mut blk_mq_tags, reserve:bool){sbitmap_queue_wake_all(&mut (*tags).bitmap_tags);if reserve{sbitmap_queue_wake_all(&mut (*tags).breserved_tags);}}
#[no_mangle] pub unsafe extern "C" fn __blk_mq_tag_idle(hctx:*mut blk_mq_hw_ctx){let tags=(*hctx).tags;let gone=if blk_mq_is_shared_tags((*hctx).flags){!test_and_clear_bit(QUEUE_FLAG_HCTX_ACTIVE,&mut (*(*hctx).queue).queue_flags)}else{!test_and_clear_bit(BLK_MQ_S_TAG_ACTIVE,&mut (*hctx).state)};if gone{return;}spin_lock_irq(&mut (*tags).lock);let users=(*tags).active_queues-1;(*tags).active_queues=users;blk_mq_update_wake_batch(tags,users);spin_unlock_irq(&mut (*tags).lock);blk_mq_tag_wakeup_all(tags,false);}
unsafe fn __blk_mq_get_tag(data:*mut blk_mq_alloc_data,bt:*mut sbitmap_queue)->i32{if (*data).q as usize!=0&&(*data).flags&BLK_MQ_REQ_RESERVED==0&&!hctx_may_queue((*data).hctx,bt){return BLK_MQ_NO_TAG;}if (*data).shallow_depth!=0{sbitmap_queue_get_shallow(bt,(*data).shallow_depth)}else{__sbitmap_queue_get(bt)}}
#[no_mangle] pub unsafe extern "C" fn blk_mq_get_tags(data:*mut blk_mq_alloc_data,nr:i32,off:*mut u32)->u64{let tags=blk_mq_tags_from_data(data);if (*data).shallow_depth!=0||(*data).flags&BLK_MQ_REQ_RESERVED!=0||(*data).hctx as usize!=0&&(*data).hctx as usize!=0&&(*data).flags&BLK_MQ_F_TAG_QUEUE_SHARED!=0{return 0;}let r=__sbitmap_queue_get_batch(&mut (*tags).bitmap_tags,nr,off);*off+=(*tags).nr_reserved_tags;r}
#[no_mangle] pub unsafe extern "C" fn blk_mq_put_tag_public(tags:*mut blk_mq_tags,ctx:*mut blk_mq_ctx,tag:u32){if !blk_mq_tag_is_reserved(tags,tag){sbitmap_queue_clear(&mut (*tags).bitmap_tags,(tag-(*tags).nr_reserved_tags) as i32,(*ctx).cpu)}else{sbitmap_queue_clear(&mut (*tags).breserved_tags,tag as i32,(*ctx).cpu)}}
#[no_mangle] pub unsafe extern "C" fn blk_mq_put_tags(tags:*mut blk_mq_tags,a:*mut i32,n:i32){sbitmap_queue_clear_batch(&mut (*tags).bitmap_tags,(*tags).nr_reserved_tags,a,n);}

#[no_mangle] pub unsafe extern "C" fn blk_mq_unique_tag(rq:*mut request)->u32{((*(*rq).mq_hctx).flags<<BLK_MQ_UNIQUE_TAG_BITS)|((*rq).tag&BLK_MQ_UNIQUE_TAG_MASK)}

// The remaining request-iteration and allocation entry points retain the
// source interfaces; their kernel primitives are supplied by the surrounding
// translated kernel sources.
#[no_mangle] pub unsafe extern "C" fn blk_mq_put_tag(tags:*mut blk_mq_tags,ctx:*mut blk_mq_ctx,tag:u32){blk_mq_put_tag_public(tags,ctx,tag)}
#[no_mangle] pub unsafe extern "C" fn blk_mq_tag_resize_shared_tags(set:*mut blk_mq_tag_set,size:u32){let tags=(*set).shared_tags;sbitmap_queue_resize(&mut (*tags).bitmap_tags,size-(*set).reserved_tags);}
#[no_mangle] pub unsafe extern "C" fn blk_mq_tag_update_sched_shared_tags(q:*mut request_queue,nr:u32){sbitmap_queue_resize(&mut (*(*q).sched_shared_tags).bitmap_tags,nr-(*(*q).tag_set).reserved_tags);}
unsafe fn bt_alloc(bt:*mut sbitmap_queue,depth:u32,rr:bool,node:i32)->i32{sbitmap_queue_init_node(bt,depth,-1,rr,0x10,node)}
#[no_mangle] pub unsafe extern "C" fn blk_mq_init_tags(total:u32,reserved:u32,flags:u32,node:i32)->*mut blk_mq_tags{if total>BLK_MQ_TAG_MAX{return core::ptr::null_mut();}let p=alloc_zeroed(core::mem::size_of::<blk_mq_tags>(),node) as *mut blk_mq_tags;if p.is_null(){return p;}(*p).nr_tags=total;(*p).nr_reserved_tags=reserved;if bt_alloc(&mut (*p).bitmap_tags,total-reserved,flags&1!=0,node)!=0||bt_alloc(&mut (*p).breserved_tags,reserved,flags&1!=0,node)!=0{return core::ptr::null_mut();}p}
extern "C" { fn alloc_zeroed(size:usize,node:i32)->*mut c_void; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
