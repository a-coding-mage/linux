// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level Rust translation of blk-sysfs.c; kernel dependencies are external. */

#[repr(C)]
pub struct queue_sysfs_entry {
    pub attr: attribute,
    pub show: Option<unsafe extern "C" fn(*mut gendisk, *mut i8) -> isize>,
    pub show_limit: Option<unsafe extern "C" fn(*mut gendisk, *mut i8) -> isize>,
    pub store: Option<unsafe extern "C" fn(*mut gendisk, *const i8, usize) -> isize>,
    pub store_limit: Option<unsafe extern "C" fn(*mut gendisk, *const i8, usize, *mut queue_limits) -> i32>,
}

unsafe fn queue_var_show(var: usize, page: *mut i8) -> isize {
    sysfs_emit(page, b"%lu\n\0".as_ptr() as *const i8, var)
}
unsafe fn queue_var_store(var: *mut usize, page: *const i8, count: usize) -> isize {
    let mut v = 0usize;
    let err = kstrtoul(page, 10, &mut v);
    if err != 0 || v > u32::MAX as usize { return -EINVAL as isize; }
    *var = v; count as isize
}

unsafe fn queue_requests_show(d:*mut gendisk,p:*mut i8)->isize {
    mutex_lock(&mut (*(*d).queue).elevator_lock);
    let r=queue_var_show((*(*d).queue).nr_requests as usize,p);
    mutex_unlock(&mut (*(*d).queue).elevator_lock); r
}
unsafe fn queue_requests_store(d:*mut gendisk,p:*const i8,c:usize)->isize {
    let q=(*d).queue; let set=(*q).tag_set; let mut et=core::ptr::null_mut(); let mut nr=0usize;
    let mut ret=queue_var_store(&mut nr,p,c); if ret<0{return ret;}
    if down_write_trylock(&mut (*set).update_nr_hwq_lock)==0{return -EBUSY as isize;}
    if nr==(*q).nr_requests as usize {up_write(&mut (*set).update_nr_hwq_lock);return ret;}
    if nr<BLKDEV_MIN_RQ as usize {nr=BLKDEV_MIN_RQ as usize;}
    if nr<=(*set).reserved_tags as usize || (!(*q).elevator.is_null() && nr>MAX_SCHED_RQ as usize) || ((*q).elevator.is_null() && nr>(*set).queue_depth as usize) {ret=-EINVAL as isize;up_write(&mut (*set).update_nr_hwq_lock);return ret;}
    if !blk_mq_is_shared_tags((*set).flags) && !(*q).elevator.is_null() && nr>(*(*q).elevator).et.nr_requests as usize {et=blk_mq_alloc_sched_tags(set,(*q).nr_hw_queues,nr);if et.is_null(){ret=-ENOMEM as isize;up_write(&mut (*set).update_nr_hwq_lock);return ret;}}
    let mf=blk_mq_freeze_queue(q);mutex_lock(&mut (*q).elevator_lock);et=blk_mq_update_nr_requests(q,et,nr);mutex_unlock(&mut (*q).elevator_lock);blk_mq_unfreeze_queue(q,mf);if !et.is_null(){blk_mq_free_sched_tags(et,set);}up_write(&mut (*set).update_nr_hwq_lock);ret
}
unsafe fn queue_async_depth_show(d:*mut gendisk,p:*mut i8)->isize {mutex_lock(&mut (*(*d).queue).elevator_lock);let r=queue_var_show((*(*d).queue).async_depth as usize,p);mutex_unlock(&mut (*(*d).queue).elevator_lock);r}
unsafe fn queue_async_depth_store(d:*mut gendisk,p:*const i8,c:usize)->isize {let q=(*d).queue;if !queue_is_mq(q){return -EINVAL as isize;}let mut nr=0;let mut r=queue_var_store(&mut nr,p,c);if r<0||nr==0{return if r<0{r}else{-EINVAL as isize}};let mf=blk_mq_freeze_queue(q);mutex_lock(&mut (*q).elevator_lock);if !(*q).elevator.is_null(){(*q).async_depth=core::cmp::min((*q).nr_requests as usize,nr) as u32;}else{r=-EINVAL as isize;}mutex_unlock(&mut (*q).elevator_lock);blk_mq_unfreeze_queue(q,mf);r}
unsafe fn queue_ra_show(d:*mut gendisk,p:*mut i8)->isize {mutex_lock(&mut (*(*d).queue).limits_lock);let r=queue_var_show((*(*d).bdi).ra_pages as usize << (PAGE_SHIFT-10),p);mutex_unlock(&mut (*(*d).queue).limits_lock);r}
unsafe fn queue_ra_store(d:*mut gendisk,p:*const i8,c:usize)->isize {let mut v=0;let r=queue_var_store(&mut v,p,c);if r<0{return r;}mutex_lock(&mut (*(*d).queue).limits_lock);core::ptr::write_volatile(&mut (*(*d).bdi).ra_pages,v>>(PAGE_SHIFT-10));mutex_unlock(&mut (*(*d).queue).limits_lock);r}

macro_rules! simple_limit { ($n:ident) => { unsafe fn $n##_show(d:*mut gendisk,p:*mut i8)->isize{queue_var_show((*(*d).queue).limits.$n as usize,p)} }; }
simple_limit!(max_segments); simple_limit!(max_discard_segments); simple_limit!(max_integrity_segments); simple_limit!(max_segment_size); simple_limit!(max_write_streams); simple_limit!(write_stream_granularity); simple_limit!(logical_block_size); simple_limit!(physical_block_size); simple_limit!(chunk_sectors); simple_limit!(io_min); simple_limit!(io_opt); simple_limit!(discard_granularity); simple_limit!(zone_write_granularity); simple_limit!(virt_boundary_mask); simple_limit!(dma_alignment); simple_limit!(max_open_zones); simple_limit!(max_active_zones); simple_limit!(atomic_write_unit_min); simple_limit!(atomic_write_unit_max);

unsafe fn queue_max_discard_sectors_store(d:*mut gendisk,p:*const i8,c:usize,l:*mut queue_limits)->i32{let mut v=0;let r=queue_var_store(&mut v,p,c);if r<0{return r as i32;}if v&((*(*d).queue).limits.discard_granularity as usize-1)!=0||v>>SECTOR_SHIFT>u32::MAX as usize{return -EINVAL;}(*l).max_user_discard_sectors=(v>>SECTOR_SHIFT) as u32;0}
unsafe fn queue_max_wzeroes_unmap_sectors_store(_: *mut gendisk,p:*const i8,c:usize,l:*mut queue_limits)->i32{let mut v=0;let r=queue_var_store(&mut v,p,c);if r<0{return r as i32;}(*l).max_user_wzeroes_unmap_sectors=(v>>SECTOR_SHIFT) as u32;0}
unsafe fn queue_max_sectors_store(_: *mut gendisk,p:*const i8,c:usize,l:*mut queue_limits)->i32{let mut v=0;let r=queue_var_store(&mut v,p,c);if r<0{return r as i32;}(*l).max_user_sectors=(v<<1) as u32;0}
unsafe fn queue_feature_store(_: *mut gendisk,p:*const i8,c:usize,l:*mut queue_limits,f:blk_features_t)->isize{let mut v=0;let r=queue_var_store(&mut v,p,c);if r<0{return r;}if v!=0{(*l).features|=f;}else{(*l).features&=!f;}0}

unsafe fn queue_poll_show(d:*mut gendisk,p:*mut i8)->isize{if queue_is_mq((*d).queue){sysfs_emit(p,b"%u\n\0".as_ptr() as *const i8,blk_mq_can_poll((*d).queue))}else{sysfs_emit(p,b"%u\n\0".as_ptr() as *const i8,(((*(*d).queue).limits.features&BLK_FEAT_POLL)!=0) as u32)}}
unsafe fn queue_zoned_show(d:*mut gendisk,p:*mut i8)->isize{if blk_queue_is_zoned((*d).queue){sysfs_emit(p,b"host-managed\n\0".as_ptr() as *const i8)}else{sysfs_emit(p,b"none\n\0".as_ptr() as *const i8)}}
unsafe fn queue_nr_zones_show(d:*mut gendisk,p:*mut i8)->isize{queue_var_show(disk_nr_zones(d),p)}
unsafe fn queue_zoned_qd1_writes_show(d:*mut gendisk,p:*mut i8)->isize{queue_var_show(blk_queue_zoned_qd1_writes((*d).queue) as usize,p)}
unsafe fn queue_zoned_qd1_writes_store(d:*mut gendisk,_:*const i8,c:usize)->isize{let q=(*d).queue;let f=blk_mq_freeze_queue(q);blk_mq_quiesce_queue(q);blk_queue_flag_set(QUEUE_FLAG_ZONED_QD1_WRITES,q);blk_mq_unquiesce_queue(q);blk_mq_unfreeze_queue(q,f);c as isize}
unsafe fn queue_iostats_passthrough_show(d:*mut gendisk,p:*mut i8)->isize{queue_var_show(blk_queue_passthrough_stat((*d).queue) as usize,p)}
unsafe fn queue_nomerges_show(d:*mut gendisk,p:*mut i8)->isize{queue_var_show((blk_queue_nomerges((*d).queue) as usize)<<1|blk_queue_noxmerges((*d).queue) as usize,p)}
unsafe fn queue_poll_delay_store(_: *mut gendisk,_:*const i8,c:usize)->isize{c as isize}
unsafe fn queue_poll_store(d:*mut gendisk,_:*const i8,c:usize)->isize{if (*(*d).queue).limits.features&BLK_FEAT_POLL==0{-EINVAL as isize}else{c as isize}}
unsafe fn queue_io_timeout_show(d:*mut gendisk,p:*mut i8)->isize{sysfs_emit(p,b"%u\n\0".as_ptr() as *const i8,jiffies_to_msecs((*(*d).queue).rq_timeout))}
unsafe fn queue_io_timeout_store(d:*mut gendisk,p:*const i8,c:usize)->isize{let mut v=0;if kstrtou32(p,10,&mut v)!=0||v==0{return -EINVAL as isize;}blk_queue_rq_timeout((*d).queue,msecs_to_jiffies(v));c as isize}
unsafe fn queue_wc_show(d:*mut gendisk,p:*mut i8)->isize{if blk_queue_write_cache((*d).queue){sysfs_emit(p,b"write back\n\0".as_ptr() as *const i8)}else{sysfs_emit(p,b"write through\n\0".as_ptr() as *const i8)}}

extern "C" {
    fn sysfs_emit(*mut i8,*const i8,...) -> isize; fn kstrtoul(*const i8,u32,*mut usize)->i32; fn kstrtou32(*const i8,u32,*mut u32)->i32;
    fn mutex_lock(*mut mutex); fn mutex_unlock(*mut mutex); fn down_write_trylock(*mut rw_semaphore)->i32; fn up_write(*mut rw_semaphore);
    fn blk_mq_freeze_queue(*mut request_queue)->u32; fn blk_mq_unfreeze_queue(*mut request_queue,u32); fn blk_mq_quiesce_queue(*mut request_queue); fn blk_mq_unquiesce_queue(*mut request_queue);
    fn queue_is_mq(*mut request_queue)->bool; fn blk_mq_is_shared_tags(u32)->bool; fn blk_mq_alloc_sched_tags(*mut blk_mq_tag_set,u32,usize)->*mut elevator_tags; fn blk_mq_update_nr_requests(*mut request_queue,*mut elevator_tags,usize)->*mut elevator_tags; fn blk_mq_free_sched_tags(*mut elevator_tags,*mut blk_mq_tag_set);
    fn blk_mq_can_poll(*mut request_queue)->u32; fn blk_queue_is_zoned(*mut request_queue)->bool; fn disk_nr_zones(*mut gendisk)->usize; fn blk_queue_zoned_qd1_writes(*mut request_queue)->bool; fn blk_queue_passthrough_stat(*mut request_queue)->bool; fn blk_queue_nomerges(*mut request_queue)->bool; fn blk_queue_noxmerges(*mut request_queue)->bool; fn blk_queue_write_cache(*mut request_queue)->bool; fn blk_queue_rq_timeout(*mut request_queue,u32); fn msecs_to_jiffies(u32)->u32; fn jiffies_to_msecs(u32)->u32; fn blk_queue_flag_set(u32,*mut request_queue); fn blk_queue_flag_clear(u32,*mut request_queue);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
