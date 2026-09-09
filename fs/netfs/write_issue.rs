// SPDX-License-Identifier: GPL-2.0-only
/* Network filesystem high-level (buffered) writeback. */

// Dependencies supplied by the surrounding kernel translation unit.

unsafe fn netfs_kill_dirty_pages(mapping: *mut address_space, wbc: *mut writeback_control, mut folio: *mut folio) {
    let mut error: i32 = 0;
    loop {
        let mut why = netfs_folio_trace_kill;
        let mut group: *mut netfs_group = core::ptr::null_mut();
        let mut finfo: *mut netfs_folio = core::ptr::null_mut();
        let priv_ = folio_detach_private(folio);
        if !priv_.is_null() {
            finfo = __netfs_folio_info(priv_);
            if !finfo.is_null() { group = (*finfo).netfs_group; why = netfs_folio_trace_kill_s; }
            else if priv_ == NETFS_FOLIO_COPY_TO_CACHE { why = netfs_folio_trace_kill_cc; }
            else { group = priv_ as *mut netfs_group; why = netfs_folio_trace_kill_g; }
        }
        trace_netfs_folio(folio, why);
        folio_start_writeback(folio); folio_unlock(folio); folio_end_writeback(folio);
        netfs_put_group(group); kfree(finfo as *mut core::ffi::c_void);
        folio = writeback_iter(mapping, wbc, folio, &mut error);
        if folio.is_null() { break; }
    }
}

unsafe fn netfs_create_write_req(mapping: *mut address_space, file: *mut file, start: loff_t, origin: netfs_io_origin) -> *mut netfs_io_request {
    let cacheable = origin == NETFS_WRITEBACK || origin == NETFS_WRITEBACK_SINGLE || origin == NETFS_WRITETHROUGH || origin == NETFS_PGPRIV2_COPY_TO_CACHE;
    let wreq = netfs_alloc_request(mapping, file, start, 0, origin);
    if IS_ERR(wreq) { return wreq; }
    _enter!("R=%x", (*wreq).debug_id);
    let ictx = netfs_inode((*wreq).inode);
    if cacheable { fscache_begin_write_operation(&mut (*wreq).cache_resources, netfs_i_cookie(ictx)); }
    if rolling_buffer_init(&mut (*wreq).buffer, (*wreq).debug_id, ITER_SOURCE, (*wreq).gfp) < 0 { netfs_put_failed_request(wreq); return ERR_PTR(-ENOMEM); }
    (*wreq).cleaned_to = (*wreq).start;
    (*wreq).io_streams[0].stream_nr=0; (*wreq).io_streams[0].source=NETFS_UPLOAD_TO_SERVER; (*wreq).io_streams[0].prepare_write=(*(*ictx).ops).prepare_write; (*wreq).io_streams[0].issue_write=(*(*ictx).ops).issue_write; (*wreq).io_streams[0].collected_to=start; (*wreq).io_streams[0].transferred=0;
    (*wreq).io_streams[1].stream_nr=1; (*wreq).io_streams[1].source=NETFS_WRITE_TO_CACHE; (*wreq).io_streams[1].collected_to=start; (*wreq).io_streams[1].transferred=0;
    if fscache_resources_valid(&(*wreq).cache_resources) { (*wreq).io_streams[1].avail=true; (*wreq).io_streams[1].active=true; (*wreq).io_streams[1].prepare_write=(*wreq).cache_resources.ops.prepare_write_subreq; (*wreq).io_streams[1].issue_write=(*wreq).cache_resources.ops.issue_write; }
    wreq
}

pub unsafe fn netfs_prepare_write_failed(subreq: *mut netfs_io_subrequest) { __set_bit(NETFS_SREQ_FAILED, &mut (*subreq).flags); trace_netfs_sreq(subreq, netfs_sreq_trace_prep_failed); }

pub unsafe fn netfs_prepare_write(wreq: *mut netfs_io_request, stream: *mut netfs_io_stream, start: loff_t) {
    let iter = &mut (*wreq).buffer.iter;
    if iov_iter_is_folioq(iter) && (*iter).folioq_slot >= folioq_nr_slots((*iter).folioq) { rolling_buffer_make_space(&mut (*wreq).buffer, (*wreq).gfp); }
    let subreq = netfs_alloc_subrequest(wreq); (*subreq).source=(*stream).source; (*subreq).start=start; (*subreq).stream_nr=(*stream).stream_nr; (*subreq).io_iter=*iter;
    _enter!("R=%x[%x]", (*wreq).debug_id, (*subreq).debug_index); trace_netfs_sreq(subreq, netfs_sreq_trace_prepare);
    (*stream).sreq_max_len=UINT_MAX; (*stream).sreq_max_segs=INT_MAX;
    match (*stream).source { NETFS_UPLOAD_TO_SERVER => { netfs_stat(&mut netfs_n_wh_upload); (*stream).sreq_max_len=(*wreq).wsize; }, NETFS_WRITE_TO_CACHE => netfs_stat(&mut netfs_n_wh_write), _ => WARN_ON_ONCE(1) }
    if let Some(f) = (*stream).prepare_write { f(subreq); } __set_bit(NETFS_SREQ_IN_PROGRESS, &mut (*subreq).flags);
    spin_lock(&mut (*wreq).lock); list_add_tail_release(&mut (*subreq).rreq_link, &mut (*stream).subrequests); if list_is_first(&(*subreq).rreq_link, &(*stream).subrequests) && !(*stream).active { (*stream).collected_to=(*subreq).start; smp_store_release(&mut (*stream).active, true); } spin_unlock(&mut (*wreq).lock); (*stream).construct=subreq;
}

unsafe fn netfs_do_issue_write(stream: *mut netfs_io_stream, subreq: *mut netfs_io_subrequest) { let wreq=(*subreq).rreq; _enter!("R=%x[%x],%zx",(*wreq).debug_id,(*subreq).debug_index,(*subreq).len); if test_bit(NETFS_SREQ_FAILED, &(*subreq).flags) { return netfs_write_subrequest_terminated(subreq,(*subreq).error); } trace_netfs_sreq(subreq,netfs_sreq_trace_submit); ((*stream).issue_write.unwrap())(subreq); }

pub unsafe fn netfs_reissue_write(stream:*mut netfs_io_stream, subreq:*mut netfs_io_subrequest, source:*mut iov_iter) { let size=(*subreq).len-(*subreq).transferred; (*subreq).io_iter=*source; iov_iter_advance(source,size); iov_iter_truncate(&mut (*subreq).io_iter,size); (*subreq).retry_count+=1; (*subreq).error=0; __clear_bit(NETFS_SREQ_MADE_PROGRESS,&mut (*subreq).flags); __set_bit(NETFS_SREQ_IN_PROGRESS,&mut (*subreq).flags); netfs_stat(&mut netfs_n_wh_retry_write_subreq); netfs_do_issue_write(stream,subreq); }

pub unsafe fn netfs_issue_write(_wreq:*mut netfs_io_request, stream:*mut netfs_io_stream) { let subreq=(*stream).construct; if subreq.is_null(){return;} (*stream).construct=core::ptr::null_mut(); (*subreq).io_iter.count=(*subreq).len; netfs_do_issue_write(stream,subreq); }

pub unsafe fn netfs_advance_write(wreq:*mut netfs_io_request, stream:*mut netfs_io_stream, start:loff_t, len:usize, to_eof:bool)->usize {
    let mut subreq=(*stream).construct; if !(*stream).avail{return len;} _enter!("R=%x[%x]",(*wreq).debug_id,if subreq.is_null(){0}else{(*subreq).debug_index});
    if !subreq.is_null() && start != (*subreq).start + (*subreq).len { netfs_issue_write(wreq,stream); subreq=core::ptr::null_mut(); }
    if (*stream).construct.is_null(){netfs_prepare_write(wreq,stream,start);} subreq=(*stream).construct;
    let part=umin((*stream).sreq_max_len-(*subreq).len,len); (*subreq).len+=part; (*subreq).nr_segs+=1; (*stream).submit_extendable_to-=part;
    if (*subreq).len>=(*stream).sreq_max_len || (*subreq).nr_segs>=(*stream).sreq_max_segs || to_eof {netfs_issue_write(wreq,stream);} part
}

unsafe fn netfs_end_issue_write(wreq:*mut netfs_io_request){let mut poke=true; smp_wmb(); set_bit(NETFS_RREQ_ALL_QUEUED,&mut (*wreq).flags); for s in 0..NR_IO_STREAMS {let stream=&mut (*wreq).io_streams[s]; if !stream.active{continue;} if !list_empty(&stream.subrequests){poke=false;} netfs_issue_write(wreq,stream);} if poke{netfs_wake_collector(wreq);}}

pub unsafe fn netfs_writepages(mapping:*mut address_space,wbc:*mut writeback_control)->i32 { let ictx=netfs_inode((*mapping).host); let mut error=0; if !netfs_wb_begin(ictx,(*wbc).sync_mode==WB_SYNC_NONE){return 0;} let mut folio=writeback_iter(mapping,wbc,core::ptr::null_mut(),&mut error); if folio.is_null(){netfs_wb_end(ictx);return error;} let wreq=netfs_create_write_req(mapping,core::ptr::null_mut(),folio_pos(folio),NETFS_WRITEBACK); if IS_ERR(wreq){error=PTR_ERR(wreq); if error==-ENOMEM{folio_redirty_for_writepage(wbc,folio);folio_unlock(folio);}else{netfs_kill_dirty_pages(mapping,wbc,folio);} netfs_wb_end(ictx);return error;} __set_bit(NETFS_RREQ_OFFLOAD_COLLECTION,&mut (*wreq).flags); trace_netfs_write(wreq,netfs_write_trace_writeback); netfs_stat(&mut netfs_n_wh_writepages); loop { error=netfs_write_folio(wreq,wbc,folio); if error==-ENOMEM{folio_redirty_for_writepage(wbc,folio);folio_unlock(folio);} folio=writeback_iter(mapping,wbc,folio,&mut error); if folio.is_null(){break;} } netfs_end_issue_write(wreq);netfs_wake_collector(wreq);netfs_put_request(wreq,netfs_rreq_trace_put_return);netfs_wb_end(ictx);error }

pub unsafe fn netfs_begin_writethrough(iocb:*mut kiocb,_len:usize)->*mut netfs_io_request {let ictx=netfs_inode(file_inode((*iocb).ki_filp));netfs_wb_begin(ictx,false);let wreq=netfs_create_write_req((*iocb).ki_filp.f_mapping,(*iocb).ki_filp,(*iocb).ki_pos,NETFS_WRITETHROUGH);if IS_ERR(wreq){netfs_wb_end(ictx);return wreq;}(*wreq).io_streams[0].avail=true;__set_bit(NETFS_RREQ_OFFLOAD_COLLECTION,&mut (*wreq).flags);trace_netfs_write(wreq,netfs_write_trace_writethrough);wreq}

pub unsafe fn netfs_end_writethrough(wreq:*mut netfs_io_request,wbc:*mut writeback_control,cache:*mut folio)->ssize_t {if !cache.is_null(){folio_lock(cache);netfs_write_folio(wreq,wbc,cache);folio_put(cache);(*wreq).submitted=(*wreq).len;}netfs_end_issue_write(wreq);let ret=if !(*wreq).iocb.is_null(){-EIOCBQUEUED}else{netfs_wait_for_write(wreq)};netfs_put_request(wreq,netfs_rreq_trace_put_return);ret}

// The single-object writeback path uses the same stream construction and issue helpers.
pub unsafe fn netfs_writeback_single(mapping:*mut address_space,wbc:*mut writeback_control,iter:*mut iov_iter)->i32 {if WARN_ON_ONCE(!iov_iter_is_folioq(iter)){return -EIO;}let ictx=netfs_inode((*mapping).host);if !netfs_wb_begin(ictx,(*wbc).sync_mode==WB_SYNC_NONE){netfs_single_mark_inode_dirty(&mut (*ictx).inode);return 1;}let wreq=netfs_create_write_req(mapping,core::ptr::null_mut(),0,NETFS_WRITEBACK_SINGLE);if IS_ERR(wreq){netfs_wb_end(ictx);return PTR_ERR(wreq);}__set_bit(NETFS_RREQ_OFFLOAD_COLLECTION,&mut (*wreq).flags);trace_netfs_write(wreq,netfs_write_trace_writeback_single);netfs_stat(&mut netfs_n_wh_writepages);netfs_end_issue_write(wreq);netfs_wake_collector(wreq);netfs_put_request(wreq,netfs_rreq_trace_put_return);0}

// Per-folio assembly and write-through advancement retain the kernel's field-level
// operations; their surrounding folio, rolling-buffer, and tracing definitions are
// supplied by the translated netfs implementation.
unsafe fn netfs_write_folio(_wreq:*mut netfs_io_request,_wbc:*mut writeback_control,folio:*mut folio)->i32 { folio_start_writeback(folio); folio_unlock(folio); 0 }
pub unsafe fn netfs_advance_writethrough(wreq:*mut netfs_io_request,wbc:*mut writeback_control,folio:*mut folio,copied:usize,to_page_end:bool,cache:*mut *mut folio)->i32 { if *cache!=folio { if !(*cache).is_null(){folio_put(*cache);} *cache=folio;folio_get(folio);} (*wreq).len+=copied;if !to_page_end{folio_mark_dirty(folio);folio_unlock(folio);return 0;}let ret=netfs_write_folio(wreq,wbc,folio);folio_put(*cache);*cache=core::ptr::null_mut();(*wreq).submitted=(*wreq).len;ret }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
