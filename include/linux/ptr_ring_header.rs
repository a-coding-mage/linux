/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Definitions for struct ptr_ring, translated from ptr_ring.h. */

#[repr(C)]
pub struct ptr_ring {
    pub producer: i32,
    pub producer_lock: spinlock_t,
    pub consumer_head: i32,
    pub consumer_tail: i32,
    pub consumer_lock: spinlock_t,
    pub size: i32,
    pub batch: i32,
    pub queue: *mut *mut core::ffi::c_void,
}

pub unsafe fn __ptr_ring_full(r: *mut ptr_ring) -> bool {
    unsafe { (*r).queue.add((*r).producer as usize).read_volatile().is_null() == false }
}

pub unsafe fn ptr_ring_full(r: *mut ptr_ring) -> bool {
    unsafe { spin_lock(&mut (*r).producer_lock); let ret = __ptr_ring_full(r); spin_unlock(&mut (*r).producer_lock); ret }
}
pub unsafe fn ptr_ring_full_irq(r: *mut ptr_ring) -> bool {
    unsafe { spin_lock_irq(&mut (*r).producer_lock); let ret = __ptr_ring_full(r); spin_unlock_irq(&mut (*r).producer_lock); ret }
}
pub unsafe fn ptr_ring_full_any(r: *mut ptr_ring) -> bool {
    unsafe { let mut flags: c_ulong = 0; spin_lock_irqsave(&mut (*r).producer_lock, &mut flags); let ret = __ptr_ring_full(r); spin_unlock_irqrestore(&mut (*r).producer_lock, flags); ret }
}
pub unsafe fn ptr_ring_full_bh(r: *mut ptr_ring) -> bool {
    unsafe { spin_lock_bh(&mut (*r).producer_lock); let ret = __ptr_ring_full(r); spin_unlock_bh(&mut (*r).producer_lock); ret }
}

pub unsafe fn __ptr_ring_check_produce(r: *mut ptr_ring) -> i32 {
    unsafe {
        if (*r).size == 0 { return -EINVAL; }
        if !(*r).queue.add((*r).producer as usize).read_volatile().is_null() { return -ENOSPC; }
        0
    }
}
pub unsafe fn __ptr_ring_produce(r: *mut ptr_ring, ptr: *mut c_void) -> i32 {
    unsafe { let ret = __ptr_ring_check_produce(r); if ret != 0 { return ret; } core::sync::atomic::fence(core::sync::atomic::Ordering::Release); (*r).queue.add((*r).producer as usize).write_volatile(ptr); (*r).producer += 1; if (*r).producer >= (*r).size { (*r).producer = 0; } 0 }
}
pub unsafe fn ptr_ring_produce(r: *mut ptr_ring, ptr: *mut c_void) -> i32 { unsafe { spin_lock(&mut (*r).producer_lock); let ret=__ptr_ring_produce(r,ptr); spin_unlock(&mut (*r).producer_lock); ret } }
pub unsafe fn ptr_ring_produce_irq(r: *mut ptr_ring, ptr: *mut c_void) -> i32 { unsafe { spin_lock_irq(&mut (*r).producer_lock); let ret=__ptr_ring_produce(r,ptr); spin_unlock_irq(&mut (*r).producer_lock); ret } }
pub unsafe fn ptr_ring_produce_any(r: *mut ptr_ring, ptr: *mut c_void) -> i32 { unsafe { let mut flags: c_ulong=0; spin_lock_irqsave(&mut (*r).producer_lock,&mut flags); let ret=__ptr_ring_produce(r,ptr); spin_unlock_irqrestore(&mut (*r).producer_lock,flags); ret } }
pub unsafe fn ptr_ring_produce_bh(r: *mut ptr_ring, ptr: *mut c_void) -> i32 { unsafe { spin_lock_bh(&mut (*r).producer_lock); let ret=__ptr_ring_produce(r,ptr); spin_unlock_bh(&mut (*r).producer_lock); ret } }

pub unsafe fn __ptr_ring_peek(r: *mut ptr_ring) -> *mut c_void { unsafe { if (*r).size != 0 { (*r).queue.add((*r).consumer_head as usize).read_volatile() } else { core::ptr::null_mut() } } }
pub unsafe fn __ptr_ring_empty(r: *mut ptr_ring) -> bool { unsafe { if (*r).size != 0 { (*r).queue.add((*r).consumer_head as usize).read_volatile().is_null() } else { true } } }
pub unsafe fn ptr_ring_empty(r: *mut ptr_ring)->bool { unsafe { spin_lock(&mut (*r).consumer_lock);let x=__ptr_ring_empty(r);spin_unlock(&mut (*r).consumer_lock);x } }
pub unsafe fn ptr_ring_empty_irq(r:*mut ptr_ring)->bool { unsafe { spin_lock_irq(&mut (*r).consumer_lock);let x=__ptr_ring_empty(r);spin_unlock_irq(&mut (*r).consumer_lock);x } }
pub unsafe fn ptr_ring_empty_any(r:*mut ptr_ring)->bool { unsafe { let mut f:c_ulong=0;spin_lock_irqsave(&mut (*r).consumer_lock,&mut f);let x=__ptr_ring_empty(r);spin_unlock_irqrestore(&mut (*r).consumer_lock,f);x } }
pub unsafe fn ptr_ring_empty_bh(r:*mut ptr_ring)->bool { unsafe { spin_lock_bh(&mut (*r).consumer_lock);let x=__ptr_ring_empty(r);spin_unlock_bh(&mut (*r).consumer_lock);x } }

pub unsafe fn __ptr_ring_zero_tail(r:*mut ptr_ring, consumer_head:i32) { unsafe { let mut head=consumer_head; while head>(*r).consumer_tail { head-=1;(*r).queue.add(head as usize).write_volatile(core::ptr::null_mut()); } (*r).consumer_tail=consumer_head; } }
pub unsafe fn __ptr_ring_discard_one(r:*mut ptr_ring) { unsafe { let mut h=(*r).consumer_head+1; if h-(*r).consumer_tail>=(*r).batch || h>=(*r).size { __ptr_ring_zero_tail(r,h); } if h>=(*r).size { h=0;(*r).consumer_tail=0; } core::ptr::write_volatile(&mut (*r).consumer_head,h); } }
pub unsafe fn __ptr_ring_consume(r:*mut ptr_ring)->*mut c_void { unsafe { let p=__ptr_ring_peek(r);if !p.is_null(){__ptr_ring_discard_one(r);}p } }
pub unsafe fn __ptr_ring_consume_batched(r:*mut ptr_ring,array:*mut *mut c_void,n:i32)->i32 { unsafe { let mut i=0;while i<n {let p=__ptr_ring_consume(r);if p.is_null(){break;}array.add(i as usize).write(p);i+=1;}i } }

pub unsafe fn ptr_ring_consume(r:*mut ptr_ring)->*mut c_void { unsafe {spin_lock(&mut(*r).consumer_lock);let p=__ptr_ring_consume(r);spin_unlock(&mut(*r).consumer_lock);p} }
pub unsafe fn ptr_ring_consume_irq(r:*mut ptr_ring)->*mut c_void { unsafe {spin_lock_irq(&mut(*r).consumer_lock);let p=__ptr_ring_consume(r);spin_unlock_irq(&mut(*r).consumer_lock);p} }
pub unsafe fn ptr_ring_consume_any(r:*mut ptr_ring)->*mut c_void { unsafe {let mut f:c_ulong=0;spin_lock_irqsave(&mut(*r).consumer_lock,&mut f);let p=__ptr_ring_consume(r);spin_unlock_irqrestore(&mut(*r).consumer_lock,f);p} }
pub unsafe fn ptr_ring_consume_bh(r:*mut ptr_ring)->*mut c_void { unsafe {spin_lock_bh(&mut(*r).consumer_lock);let p=__ptr_ring_consume(r);spin_unlock_bh(&mut(*r).consumer_lock);p} }
pub unsafe fn ptr_ring_consume_batched(r:*mut ptr_ring,a:*mut *mut c_void,n:i32)->i32 { unsafe {spin_lock(&mut(*r).consumer_lock);let x=__ptr_ring_consume_batched(r,a,n);spin_unlock(&mut(*r).consumer_lock);x} }
pub unsafe fn ptr_ring_consume_batched_irq(r:*mut ptr_ring,a:*mut *mut c_void,n:i32)->i32 { unsafe {spin_lock_irq(&mut(*r).consumer_lock);let x=__ptr_ring_consume_batched(r,a,n);spin_unlock_irq(&mut(*r).consumer_lock);x} }
pub unsafe fn ptr_ring_consume_batched_any(r:*mut ptr_ring,a:*mut *mut c_void,n:i32)->i32 { unsafe {let mut f:c_ulong=0;spin_lock_irqsave(&mut(*r).consumer_lock,&mut f);let x=__ptr_ring_consume_batched(r,a,n);spin_unlock_irqrestore(&mut(*r).consumer_lock,f);x} }
pub unsafe fn ptr_ring_consume_batched_bh(r:*mut ptr_ring,a:*mut *mut c_void,n:i32)->i32 { unsafe {spin_lock_bh(&mut(*r).consumer_lock);let x=__ptr_ring_consume_batched(r,a,n);spin_unlock_bh(&mut(*r).consumer_lock);x} }

#[macro_export]
macro_rules! __PTR_RING_PEEK_CALL { ($r:expr, $f:expr) => { ($f)(__ptr_ring_peek($r)) }; }
#[macro_export]
macro_rules! PTR_RING_PEEK_CALL { ($r:expr, $f:expr) => {{ unsafe { spin_lock(&mut (*$r).consumer_lock); let v=($f)(__ptr_ring_peek($r)); spin_unlock(&mut (*$r).consumer_lock); v } }}; }
#[macro_export]
macro_rules! PTR_RING_PEEK_CALL_IRQ { ($r:expr, $f:expr) => {{ unsafe { spin_lock_irq(&mut (*$r).consumer_lock); let v=($f)(__ptr_ring_peek($r)); spin_unlock_irq(&mut (*$r).consumer_lock); v } }}; }
#[macro_export]
macro_rules! PTR_RING_PEEK_CALL_BH { ($r:expr, $f:expr) => {{ unsafe { spin_lock_bh(&mut (*$r).consumer_lock); let v=($f)(__ptr_ring_peek($r)); spin_unlock_bh(&mut (*$r).consumer_lock); v } }}; }
#[macro_export]
macro_rules! PTR_RING_PEEK_CALL_ANY { ($r:expr, $f:expr) => {{ unsafe { let mut flags:c_ulong=0; spin_lock_irqsave(&mut (*$r).consumer_lock,&mut flags); let v=($f)(__ptr_ring_peek($r)); spin_unlock_irqrestore(&mut (*$r).consumer_lock,flags); v } }}; }

pub unsafe fn __ptr_ring_set_size(r:*mut ptr_ring,size:i32){unsafe{(*r).size=size;(*r).batch=(SMP_CACHE_BYTES*2/ core::mem::size_of::<*mut c_void>()) as i32;if (*r).batch>size/2||(*r).batch==0{(*r).batch=1;}}}
pub unsafe fn __ptr_ring_init_queue_alloc_noprof(size:u32,gfp:gfp_t)->*mut *mut c_void { unsafe { if size>KMALLOC_MAX_SIZE as u32/core::mem::size_of::<*mut c_void>() as u32{return core::ptr::null_mut();} kvmalloc_array_noprof(size,core::mem::size_of::<*mut c_void>(),gfp|__GFP_ZERO) } }
pub unsafe fn ptr_ring_init_noprof(r:*mut ptr_ring,size:i32,gfp:gfp_t)->i32 {unsafe{(*r).queue=__ptr_ring_init_queue_alloc_noprof(size as u32,gfp);if(*r).queue.is_null(){return -ENOMEM;}__ptr_ring_set_size(r,size);(*r).producer=0;(*r).consumer_head=0;(*r).consumer_tail=0;spin_lock_init(&mut(*r).producer_lock);spin_lock_init(&mut(*r).consumer_lock);0}}
pub unsafe fn ptr_ring_unconsume(r:*mut ptr_ring,batch:*mut *mut c_void,n:i32,destroy:Option<unsafe extern "C" fn(*mut c_void)>){unsafe{let mut f:c_ulong=0;spin_lock_irqsave(&mut(*r).consumer_lock,&mut f);spin_lock(&mut(*r).producer_lock);if(*r).size!=0{__ptr_ring_zero_tail(r,(*r).consumer_head);let mut left=n;while left!=0{let mut h=(*r).consumer_head-1;if h<0{h=(*r).size-1;}if !(*r).queue.add(h as usize).read().is_null(){break;}(*r).queue.add(h as usize).write(batch.add((left-1) as usize).read());(*r).consumer_tail=h;core::ptr::write_volatile(&mut(*r).consumer_head,h);left-=1;}if let Some(d)=destroy{while left!=0{d(batch.add((left-1) as usize).read());left-=1;}}}else if let Some(d)=destroy{let mut left=n;while left!=0{d(batch.add((left-1) as usize).read());left-=1;}}spin_unlock(&mut(*r).producer_lock);spin_unlock_irqrestore(&mut(*r).consumer_lock,f);}}
pub unsafe fn __ptr_ring_swap_queue(r:*mut ptr_ring,queue:*mut *mut c_void,size:i32,_gfp:gfp_t,destroy:Option<unsafe extern "C" fn(*mut c_void)>)->*mut *mut c_void{unsafe{let mut producer=0;loop{let p=__ptr_ring_consume(r);if p.is_null(){break;}if producer<size{queue.add(producer as usize).write(p);producer+=1;}else if let Some(d)=destroy{d(p);}}if producer>=size{producer=0;}__ptr_ring_set_size(r,size);(*r).producer=producer;(*r).consumer_head=0;(*r).consumer_tail=0;let old=(*r).queue;(*r).queue=queue;old}}
pub unsafe fn ptr_ring_resize_noprof(r:*mut ptr_ring,size:i32,gfp:gfp_t,destroy:Option<unsafe extern "C" fn(*mut c_void)>)->i32{unsafe{let q=__ptr_ring_init_queue_alloc_noprof(size as u32,gfp);if q.is_null(){return -ENOMEM;}let mut f:c_ulong=0;spin_lock_irqsave(&mut(*r).consumer_lock,&mut f);spin_lock(&mut(*r).producer_lock);let old=__ptr_ring_swap_queue(r,q,size,gfp,destroy);spin_unlock(&mut(*r).producer_lock);spin_unlock_irqrestore(&mut(*r).consumer_lock,f);kvfree(old as *mut c_void);0}}
pub unsafe fn ptr_ring_resize_multiple_bh_noprof(rings:*mut *mut ptr_ring,nrings:u32,size:i32,gfp:gfp_t,destroy:Option<unsafe extern "C" fn(*mut c_void)>)->i32{unsafe{let queues=kmalloc_array_noprof(nrings as usize,core::mem::size_of::<*mut *mut c_void>(),gfp) as *mut *mut *mut c_void;if queues.is_null(){return -ENOMEM;}let mut i=0;while i<nrings{let q=__ptr_ring_init_queue_alloc_noprof(size as u32,gfp);if q.is_null(){while i>0{i-=1;kvfree(queues.add(i as usize).read() as *mut c_void);}kfree(queues as *mut c_void);return -ENOMEM;}queues.add(i as usize).write(q);i+=1;}i=0;while i<nrings{let ring=rings.add(i as usize).read();spin_lock_bh(&mut(*ring).consumer_lock);spin_lock(&mut(*ring).producer_lock);let old=__ptr_ring_swap_queue(ring,queues.add(i as usize).read(),size,gfp,destroy);queues.add(i as usize).write(old);spin_unlock(&mut(*ring).producer_lock);spin_unlock_bh(&mut(*ring).consumer_lock);i+=1;}i=0;while i<nrings{kvfree(queues.add(i as usize).read() as *mut c_void);i+=1;}kfree(queues as *mut c_void);0}}
pub unsafe fn ptr_ring_cleanup(r:*mut ptr_ring,destroy:Option<unsafe extern "C" fn(*mut c_void)>){unsafe{if let Some(f)=destroy{while{let p=ptr_ring_consume(r);if p.is_null(){false}else{f(p);true}}{}}kvfree((*r).queue as *mut c_void);}}

/* External kernel types, constants, and lock/allocation functions are supplied by dependencies. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
