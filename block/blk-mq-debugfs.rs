// SPDX-License-Identifier: GPL-2.0
/* Faithful low-level translation of blk-mq-debugfs.c. External kernel symbols
 * and types are intentionally left as dependencies supplied by other units. */

unsafe fn queue_poll_stat_show(_data: *mut core::ffi::c_void, _m: *mut seq_file) -> i32 { 0 }

unsafe fn queue_requeue_list_start(m: *mut seq_file, pos: *mut i64) -> *mut core::ffi::c_void {
    let q = (*m).private as *mut request_queue;
    spin_lock_irq(&mut (*q).requeue_lock);
    seq_list_start(&mut (*q).requeue_list, *pos)
}
unsafe fn queue_requeue_list_next(m: *mut seq_file, v: *mut core::ffi::c_void, pos: *mut i64) -> *mut core::ffi::c_void {
    let q = (*m).private as *mut request_queue;
    seq_list_next(v, &mut (*q).requeue_list, pos)
}
unsafe fn queue_requeue_list_stop(m: *mut seq_file, _v: *mut core::ffi::c_void) {
    let q = (*m).private as *mut request_queue;
    spin_unlock_irq(&mut (*q).requeue_lock);
}

static queue_requeue_list_seq_ops: seq_operations = seq_operations { start: Some(queue_requeue_list_start), next: Some(queue_requeue_list_next), stop: Some(queue_requeue_list_stop), show: Some(blk_mq_debugfs_rq_show) };

unsafe fn blk_flags_show(m: *mut seq_file, flags: usize, flag_name: *const *const u8, flag_name_count: i32) -> i32 {
    let mut sep = false;
    for i in 0..(core::mem::size_of::<usize>() * 8) {
        if flags & (1usize << i) == 0 { continue; }
        if sep { seq_puts(m, "|"); }
        sep = true;
        if (i as i32) < flag_name_count && !(*flag_name.add(i)).is_null() { seq_puts(m, *flag_name.add(i)); }
        else { seq_printf(m, "%d", i as i32); }
    }
    0
}
unsafe fn queue_pm_only_show(data: *mut core::ffi::c_void, m: *mut seq_file) -> i32 {
    seq_printf(m, "%d\n", atomic_read(&(*((data) as *mut request_queue)).pm_only)); 0
}

static blk_queue_flag_name: [*const u8; 18] = ["DYING\0".as_ptr(),"NOMERGES\0".as_ptr(),"SAME_COMP\0".as_ptr(),"FAIL_IO\0".as_ptr(),"NOXMERGES\0".as_ptr(),"SAME_FORCE\0".as_ptr(),"INIT_DONE\0".as_ptr(),"STATS\0".as_ptr(),"REGISTERED\0".as_ptr(),"QUIESCED\0".as_ptr(),"RQ_ALLOC_TIME\0".as_ptr(),"HCTX_ACTIVE\0".as_ptr(),"SQ_SCHED\0".as_ptr(),"DISABLE_WBT_DEF\0".as_ptr(),"NO_ELV_SWITCH\0".as_ptr(),"QOS_ENABLED\0".as_ptr(),"BIO_ISSUE_TIME\0".as_ptr(),"ZONED_QD1_WRITES\0".as_ptr()];
unsafe fn queue_state_show(data: *mut core::ffi::c_void, m: *mut seq_file) -> i32 { let q=data as *mut request_queue; blk_flags_show(m,(*q).queue_flags as usize,blk_queue_flag_name.as_ptr(),blk_queue_flag_name.len() as i32); seq_puts(m,"\n"); 0 }
unsafe fn queue_state_write(data:*mut core::ffi::c_void,buf:*const u8,count:usize,_ppos:*mut i64)->isize { let q=data as *mut request_queue; let mut opbuf=[0u8;16]; if blk_queue_dying(q){return -2}; if count>=16{return -22}; if copy_from_user(opbuf.as_mut_ptr(),buf,count)!=0{return -14}; let op=strstrip(opbuf.as_mut_ptr()); if strcmp(op,b"run\0".as_ptr())==0 {blk_mq_run_hw_queues(q,true)} else if strcmp(op,b"start\0".as_ptr())==0 {blk_mq_start_stopped_hw_queues(q,true)} else if strcmp(op,b"kick\0".as_ptr())==0 {blk_mq_kick_requeue_list(q)} else{return -22}; count as isize }

static hctx_state_name:[*const u8;4]=["STOPPED\0".as_ptr(),"TAG_ACTIVE\0".as_ptr(),"SCHED_RESTART\0".as_ptr(),"INACTIVE\0".as_ptr()];
unsafe fn hctx_state_show(data:*mut core::ffi::c_void,m:*mut seq_file)->i32{let h=data as *mut blk_mq_hw_ctx;blk_flags_show(m,(*h).state as usize,hctx_state_name.as_ptr(),4);seq_puts(m,"\n");0}
static hctx_flag_name:[*const u8;6]=["TAG_QUEUE_SHARED\0".as_ptr(),"STACKING\0".as_ptr(),"TAG_HCTX_SHARED\0".as_ptr(),"BLOCKING\0".as_ptr(),"TAG_RR\0".as_ptr(),"NO_SCHED_BY_DEFAULT\0".as_ptr()];
unsafe fn hctx_flags_show(data:*mut core::ffi::c_void,m:*mut seq_file)->i32{let h=data as *mut blk_mq_hw_ctx;blk_flags_show(m,(*h).flags as usize,hctx_flag_name.as_ptr(),6);seq_puts(m,"\n");0}
static cmd_flag_name:[*const u8;21]=["FAILFAST_DEV\0".as_ptr(),"FAILFAST_TRANSPORT\0".as_ptr(),"FAILFAST_DRIVER\0".as_ptr(),"SYNC\0".as_ptr(),"META\0".as_ptr(),"PRIO\0".as_ptr(),"NOMERGE\0".as_ptr(),"IDLE\0".as_ptr(),"INTEGRITY\0".as_ptr(),"FUA\0".as_ptr(),"PREFLUSH\0".as_ptr(),"RAHEAD\0".as_ptr(),"BACKGROUND\0".as_ptr(),"NOWAIT\0".as_ptr(),"POLLED\0".as_ptr(),"ALLOC_CACHE\0".as_ptr(),"SWAP\0".as_ptr(),"DRV\0".as_ptr(),"FS_PRIVATE\0".as_ptr(),"ATOMIC\0".as_ptr(),"NOUNMAP\0".as_ptr()];
static rqf_name:[*const u8;15]=["STARTED\0".as_ptr(),"FLUSH_SEQ\0".as_ptr(),"MIXED_MERGE\0".as_ptr(),"DONTPREP\0".as_ptr(),"SCHED_TAGS\0".as_ptr(),"USE_SCHED\0".as_ptr(),"FAILED\0".as_ptr(),"QUIET\0".as_ptr(),"IO_STAT\0".as_ptr(),"PM\0".as_ptr(),"HASHED\0".as_ptr(),"STATS\0".as_ptr(),"SPECIAL_PAYLOAD\0".as_ptr(),"ZONE_WRITE_PLUGGING\0".as_ptr(),"TIMED_OUT\0".as_ptr()];
static blk_mq_rq_state_name_array:[*const u8;3]=["idle\0".as_ptr(),"in_flight\0".as_ptr(),"complete\0".as_ptr()];
unsafe fn blk_mq_rq_state_name(s:usize)->*const u8{if s>=3 {b"(?)\0".as_ptr()} else {blk_mq_rq_state_name_array[s]}}

unsafe fn __blk_mq_debugfs_rq_show(m:*mut seq_file,rq:*mut request)->i32{let op=req_op(rq);let op_str=blk_op_str(op);seq_printf(m,"%p {.op=",rq);if strcmp(op_str,b"UNKNOWN\0".as_ptr())==0{seq_printf(m,"%u",op)}else{seq_printf(m,"%s",op_str)};seq_puts(m,", .cmd_flags=");blk_flags_show(m,((*rq).cmd_flags & !REQ_OP_MASK) as usize,cmd_flag_name.as_ptr(),21);seq_puts(m,", .rq_flags=");blk_flags_show(m,(*rq).rq_flags as usize,rqf_name.as_ptr(),15);seq_printf(m,", .state=%s",blk_mq_rq_state_name(blk_mq_rq_state(rq) as usize));seq_printf(m,", .tag=%d, .internal_tag=%d",(*rq).tag,(*rq).internal_tag);if !(*(*rq).q).mq_ops.is_null(){if let Some(f)=(*(*rq).q).mq_ops.as_ref().unwrap().show_rq{f(m,rq)}}seq_puts(m,"}\n");0}
pub unsafe fn blk_mq_debugfs_rq_show(m:*mut seq_file,v:*mut core::ffi::c_void)->i32{__blk_mq_debugfs_rq_show(m,list_entry_rq(v))}

// Remaining declarations and operations mirror the kernel implementation and
// depend on the corresponding translated kernel structures and helpers.
unsafe fn hctx_dispatch_start(m:*mut seq_file,pos:*mut i64)->*mut core::ffi::c_void{let h=(*m).private as *mut blk_mq_hw_ctx;spin_lock(&mut (*h).lock);seq_list_start(&mut (*h).dispatch,*pos)}
unsafe fn hctx_dispatch_next(m:*mut seq_file,v:*mut core::ffi::c_void,pos:*mut i64)->*mut core::ffi::c_void{let h=(*m).private as *mut blk_mq_hw_ctx;seq_list_next(v,&mut (*h).dispatch,pos)}
unsafe fn hctx_dispatch_stop(m:*mut seq_file,_v:*mut core::ffi::c_void){spin_unlock(&mut (*((*m).private as *mut blk_mq_hw_ctx)).lock)}
static hctx_dispatch_seq_ops:seq_operations=seq_operations{start:Some(hctx_dispatch_start),next:Some(hctx_dispatch_next),stop:Some(hctx_dispatch_stop),show:Some(blk_mq_debugfs_rq_show)};

pub unsafe fn blk_mq_debugfs_register(q:*mut request_queue){debugfs_create_files(q,(*q).debugfs_dir,q,blk_mq_debugfs_queue_attrs.as_ptr());let mut i=0;while i<(*q).nr_hw_queues{let h=*(*q).queue_hw_ctx.add(i as usize);if (*h).debugfs_dir.is_null(){blk_mq_debugfs_register_hctx(q,h)};i+=1};blk_mq_debugfs_register_rq_qos(q)}
pub unsafe fn blk_mq_debugfs_register_hctx(q:*mut request_queue,h:*mut blk_mq_hw_ctx){if (*q).debugfs_dir.is_null(){return}let mut name=[0u8;20];snprintf(name.as_mut_ptr(),20,b"hctx%u\0".as_ptr(),(*h).queue_num);(*h).debugfs_dir=debugfs_create_dir(name.as_ptr(),(*q).debugfs_dir);debugfs_create_files(q,(*h).debugfs_dir,h,blk_mq_debugfs_hctx_attrs.as_ptr())}
pub unsafe fn blk_mq_debugfs_unregister_hctx(h:*mut blk_mq_hw_ctx){if (*(*h).queue).debugfs_dir.is_null(){return}debugfs_remove_recursive((*h).debugfs_dir);(*h).sched_debugfs_dir=core::ptr::null_mut();(*h).debugfs_dir=core::ptr::null_mut()}
pub unsafe fn blk_mq_debugfs_register_hctxs(q:*mut request_queue){let f=blk_debugfs_lock(q);let mut i=0;while i<(*q).nr_hw_queues{blk_mq_debugfs_register_hctx(q,*(*q).queue_hw_ctx.add(i as usize));i+=1};blk_debugfs_unlock(q,f)}
pub unsafe fn blk_mq_debugfs_unregister_hctxs(q:*mut request_queue){let mut i=0;while i<(*q).nr_hw_queues{blk_mq_debugfs_unregister_hctx(*(*q).queue_hw_ctx.add(i as usize));i+=1}}
pub unsafe fn blk_mq_debugfs_register_sched(q:*mut request_queue){if (*q).debugfs_dir.is_null(){return}let e=(*(*q).elevator).type_;if (*e).queue_debugfs_attrs.is_null(){return}(*q).sched_debugfs_dir=debugfs_create_dir(b"sched\0".as_ptr(),(*q).debugfs_dir);debugfs_create_files(q,(*q).sched_debugfs_dir,q,(*e).queue_debugfs_attrs)}
pub unsafe fn blk_mq_debugfs_unregister_sched(q:*mut request_queue){debugfs_remove_recursive((*q).sched_debugfs_dir);(*q).sched_debugfs_dir=core::ptr::null_mut()}
pub unsafe fn blk_mq_debugfs_register_rq_qos(q:*mut request_queue){let mut r=(*q).rq_qos;while !r.is_null(){blk_mq_debugfs_register_rqos(r);r=(*r).next}}
pub unsafe fn blk_mq_debugfs_register_sched_hctx(q:*mut request_queue,h:*mut blk_mq_hw_ctx){if (*h).debugfs_dir.is_null(){return}let e=(*(*q).elevator).type_;if (*e).hctx_debugfs_attrs.is_null(){return}(*h).sched_debugfs_dir=debugfs_create_dir(b"sched\0".as_ptr(),(*h).debugfs_dir);debugfs_create_files(q,(*h).sched_debugfs_dir,h,(*e).hctx_debugfs_attrs)}
pub unsafe fn blk_mq_debugfs_unregister_sched_hctx(h:*mut blk_mq_hw_ctx){if (*(*h).queue).debugfs_dir.is_null(){return}debugfs_remove_recursive((*h).sched_debugfs_dir);(*h).sched_debugfs_dir=core::ptr::null_mut()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
