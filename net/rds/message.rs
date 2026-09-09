/* Translated from message.c. Kernel types, constants, and functions are supplied externally. */

static mut RDS_EXTHDR_SIZE: [u32; __RDS_EXTHDR_MAX as usize] = {
    let mut a = [0; __RDS_EXTHDR_MAX as usize];
    a[RDS_EXTHDR_NONE as usize] = 0;
    a[RDS_EXTHDR_VERSION as usize] = core::mem::size_of::<rds_ext_header_version>() as u32;
    a[RDS_EXTHDR_RDMA as usize] = core::mem::size_of::<rds_ext_header_rdma>() as u32;
    a[RDS_EXTHDR_RDMA_DEST as usize] = core::mem::size_of::<rds_ext_header_rdma_dest>() as u32;
    a[RDS_EXTHDR_RDMA_BYTES as usize] = core::mem::size_of::<rds_ext_header_rdma_bytes>() as u32;
    a[RDS_EXTHDR_NPATHS as usize] = core::mem::size_of::<u16>() as u32;
    a[RDS_EXTHDR_GEN_NUM as usize] = core::mem::size_of::<u32>() as u32;
    a[RDS_EXTHDR_SPORT_IDX as usize] = 1;
    a
};

pub unsafe fn rds_message_addref(rm: *mut rds_message) { rdsdebug!("addref rm %p ref %d\n", rm, refcount_read(&(*rm).m_refcount)); refcount_inc(&mut (*rm).m_refcount); }

unsafe fn rds_zcookie_add(info: *mut rds_msg_zcopy_info, cookie: u32) -> bool {
    let ck = &mut (*info).zcookies;
    let n = ck.num;
    if n == RDS_MAX_ZCOOKIES { return false; }
    ck.cookies[n as usize] = cookie; ck.num = n + 1; true
}

unsafe fn rds_info_from_znotifier(z: *mut rds_znotifier) -> *mut rds_msg_zcopy_info { container_of!(z, rds_msg_zcopy_info, znotif) }

pub unsafe fn rds_notify_msg_zcopy_purge(q: *mut rds_msg_zcopy_queue) {
    let mut flags = 0ul; let mut copy = list_head::default();
    spin_lock_irqsave(&mut (*q).lock, &mut flags); list_splice(&mut (*q).zcookie_head, &mut copy); INIT_LIST_HEAD(&mut (*q).zcookie_head); spin_unlock_irqrestore(&mut (*q).lock, flags);
    let mut pos = copy.next; while pos != &mut copy as *mut _ { let next = (*pos).next; let info = container_of!(pos, rds_msg_zcopy_info, rs_zcookie_next); list_del(pos); kfree(info); pos = next; }
}

unsafe fn rds_rm_zerocopy_callback(rs: *mut rds_sock, z: *mut rds_znotifier) {
    let cookie = (*z).z_cookie; let q = &mut (*rs).rs_zcookie_queue; let mut flags = 0ul;
    mm_unaccount_pinned_pages(&mut (*z).z_mmp); spin_lock_irqsave(&mut q.lock, &mut flags);
    let head = &mut q.zcookie_head;
    if !list_empty(head) { let info = list_first_entry!(head, rds_msg_zcopy_info, rs_zcookie_next); if rds_zcookie_add(info, cookie) { spin_unlock_irqrestore(&mut q.lock, flags); kfree(rds_info_from_znotifier(z)); return; } }
    let info = rds_info_from_znotifier(z); memset(&mut (*info).zcookies as *mut _, 0, core::mem::size_of::<rds_zcopy_cookies>()); rds_zcookie_add(info, cookie); list_add_tail(&mut (*info).rs_zcookie_next, head); spin_unlock_irqrestore(&mut q.lock, flags);
}

unsafe fn rds_message_purge(rm: *mut rds_message) {
    if unlikely(test_bit(RDS_MSG_PAGEVEC, &(*rm).m_flags)) { return; }
    let mut flags = 0ul; spin_lock_irqsave(&mut (*rm).m_rs_lock, &mut flags); let z = (*rm).data.op_mmp_znotifier; (*rm).data.op_mmp_znotifier = core::ptr::null_mut(); let zcopy = !z.is_null();
    if !(*rm).m_rs.is_null() { let rs = (*rm).m_rs; if !z.is_null() { rds_rm_zerocopy_callback(rs, z); rds_wake_sk_sleep(rs); } sock_put(rds_rs_to_sk(rs)); (*rm).m_rs = core::ptr::null_mut(); } else if !z.is_null() { mm_unaccount_pinned_pages(&mut (*z).z_mmp); kfree(rds_info_from_znotifier(z)); }
    spin_unlock_irqrestore(&mut (*rm).m_rs_lock, flags);
    for i in 0..(*rm).data.op_nents { if !zcopy { __free_page(sg_page(&mut (*rm).data.op_sg.add(i))); } else { put_page(sg_page(&mut (*rm).data.op_sg.add(i))); } } (*rm).data.op_nents = 0;
    if (*rm).rdma.op_active { rds_rdma_free_op(&mut (*rm).rdma); } if !(*rm).rdma.op_rdma_mr.is_null() { kref_put(&mut (*(*rm).rdma.op_rdma_mr).r_kref, __rds_put_mr_final); }
    if (*rm).atomic.op_active { rds_atomic_free_op(&mut (*rm).atomic); } if !(*rm).atomic.op_rdma_mr.is_null() { kref_put(&mut (*(*rm).atomic.op_rdma_mr).r_kref, __rds_put_mr_final); }
}

unsafe fn rds_message_unpin_worker(work: *mut work_struct) { let rm = container_of!(work, rds_message, m_unpin_work); if (*rm).rdma.op_unpin_deferred { rds_rdma_op_unpin_pages(&mut (*rm).rdma); } if (*rm).atomic.op_unpin_deferred { rds_atomic_op_unpin_page(&mut (*rm).atomic); } kfree(rm); }

pub unsafe fn rds_message_put(rm: *mut rds_message) { rdsdebug!("put rm %p ref %d\n", rm, refcount_read(&(*rm).m_refcount)); WARN!(!refcount_read(&(*rm).m_refcount), "danger refcount zero on %p\n", rm); if refcount_dec_and_test(&mut (*rm).m_refcount) { BUG_ON!(!list_empty(&(*rm).m_sock_item)); BUG_ON!(!list_empty(&(*rm).m_conn_item)); rds_message_purge(rm); if (*rm).rdma.op_unpin_deferred || (*rm).atomic.op_unpin_deferred { INIT_WORK!(&mut (*rm).m_unpin_work, rds_message_unpin_worker); queue_work(rds_wq, &mut (*rm).m_unpin_work); return; } kfree(rm); } }

pub unsafe fn rds_message_populate_header(hdr: *mut rds_header, sport: u16, dport: u16, seq: u64) { (*hdr).h_flags=0; (*hdr).h_sport=sport; (*hdr).h_dport=dport; (*hdr).h_sequence=cpu_to_be64(seq); memset((*hdr).h_exthdr.as_mut_ptr() as *mut _, RDS_EXTHDR_NONE as i32, RDS_HEADER_EXT_SPACE); }

unsafe fn rds_find_next_ext_space(hdr: *mut rds_header, len: u32, ext_start: *mut *mut u8) -> i32 { let mut ind=0u32; while ind+1+len <= RDS_HEADER_EXT_SPACE { if (*hdr).h_exthdr[ind as usize]==RDS_EXTHDR_NONE { *ext_start=(*hdr).h_exthdr.as_mut_ptr().add(ind as usize); return 0; } let t=(*hdr).h_exthdr[ind as usize] as usize; let ext_len=if t<__RDS_EXTHDR_MAX as usize { RDS_EXTHDR_SIZE[t] } else { 0 }; if ext_len==0 { return -EINVAL; } ind += 1+ext_len; } -ENOSPC }

pub unsafe fn rds_message_add_extension(hdr:*mut rds_header, typ:u32, data:*const core::ffi::c_void)->i32 { let len=if typ<__RDS_EXTHDR_MAX {RDS_EXTHDR_SIZE[typ as usize]} else {0}; if len==0{return 0;} let mut dst=core::ptr::null_mut(); if rds_find_next_ext_space(hdr,len,&mut dst)!=0{return 0;} *dst=typ as u8; memcpy(dst.add(1),data,len as usize); 1 }

pub unsafe fn rds_message_next_extension(hdr:*mut rds_header,pos:*mut u32,buf:*mut core::ffi::c_void,buflen:*mut u32)->u32 { let mut off=*pos; if off>=RDS_HEADER_EXT_SPACE{*pos=RDS_HEADER_EXT_SPACE;*buflen=0;return RDS_EXTHDR_NONE;} let src=(*hdr).h_exthdr.as_ptr(); let typ=*src.add(off as usize) as usize; off+=1; if typ==RDS_EXTHDR_NONE as usize||typ>=__RDS_EXTHDR_MAX as usize{*pos=RDS_HEADER_EXT_SPACE;*buflen=0;return RDS_EXTHDR_NONE;} let len=RDS_EXTHDR_SIZE[typ]; if off+len>RDS_HEADER_EXT_SPACE{*pos=RDS_HEADER_EXT_SPACE;*buflen=0;return RDS_EXTHDR_NONE;} *pos=off+len; if len<*buflen{*buflen=len;} memcpy(buf,src.add(off as usize),*buflen as usize); typ as u32 }

pub unsafe fn rds_message_add_rdma_dest_extension(hdr:*mut rds_header,r_key:u32,offset:u32)->i32 { let mut e=rds_ext_header_rdma_dest{h_rdma_rkey:cpu_to_be32(r_key),h_rdma_offset:cpu_to_be32(offset)}; rds_message_add_extension(hdr,RDS_EXTHDR_RDMA_DEST,&mut e as *mut _ as *const _) }

pub unsafe fn rds_message_alloc(extra_len:u32,gfp:gfp_t)->*mut rds_message { if extra_len>KMALLOC_MAX_SIZE-core::mem::size_of::<rds_message>() as u32{return core::ptr::null_mut();} let rm=kzalloc((core::mem::size_of::<rds_message>() as u32+extra_len) as usize,gfp); if rm.is_null(){return rm;} (*rm).m_used_sgs=0; (*rm).m_total_sgs=extra_len/core::mem::size_of::<scatterlist>() as u32; refcount_set(&mut (*rm).m_refcount,1); INIT_LIST_HEAD(&mut (*rm).m_sock_item); INIT_LIST_HEAD(&mut (*rm).m_conn_item); spin_lock_init(&mut (*rm).m_rs_lock); init_waitqueue_head(&mut (*rm).m_flush_wait); rm }

pub unsafe fn rds_message_alloc_sgs(rm:*mut rds_message,nents:i32)->*mut scatterlist { if nents<=0{return ERR_PTR(-EINVAL as isize) as *mut _;} if (*rm).m_used_sgs+nents as u32>(*rm).m_total_sgs{return ERR_PTR(-ENOMEM as isize) as *mut _;} let sg=(rm as *mut u8).add(core::mem::size_of::<rds_message>()) as *mut scatterlist; let ret=sg.add((*rm).m_used_sgs as usize); sg_init_table(ret,nents as usize); (*rm).m_used_sgs+=nents as u32; ret }

pub unsafe fn rds_message_map_pages(addrs:*mut usize,total_len:u32)->*mut rds_message { let n=DIV_ROUND_UP(total_len,PAGE_SIZE); let rm=rds_message_alloc(n*core::mem::size_of::<scatterlist>() as u32,GFP_NOWAIT); if rm.is_null(){return ERR_PTR(-ENOMEM as isize) as *mut _;} set_bit(RDS_MSG_PAGEVEC,&mut (*rm).m_flags); (*rm).m_inc.i_hdr.h_len=cpu_to_be32(total_len); (*rm).data.op_nents=n; (*rm).data.op_sg=rds_message_alloc_sgs(rm,n as i32); if IS_ERR((*rm).data.op_sg){let e=ERR_CAST((*rm).data.op_sg);rds_message_put(rm);return e;} for i in 0..n as usize {sg_set_page(&mut *(*rm).data.op_sg.add(i),virt_to_page(*addrs.add(i) as *mut _),PAGE_SIZE,0);} rm }

unsafe fn rds_message_zcopy_from_user(rm:*mut rds_message,from:*mut iov_iter)->i32 { let mut ret=0; let mut length=iov_iter_count(from); (*rm).m_inc.i_hdr.h_len=cpu_to_be32(length); let info=kzalloc_obj::<rds_msg_zcopy_info>(); if info.is_null(){return -ENOMEM;} INIT_LIST_HEAD(&mut (*info).rs_zcookie_next); (*rm).data.op_mmp_znotifier=&mut (*info).znotif; if mm_account_pinned_pages(&mut (*(*rm).data.op_mmp_znotifier).z_mmp,length)!=0{ret=-ENOMEM; kfree(info);(*rm).data.op_mmp_znotifier=core::ptr::null_mut();return ret;} let mut sg=(*rm).data.op_sg; while iov_iter_count(from)>0 {let mut pages=core::ptr::null_mut();let mut start=0usize;let copied=iov_iter_get_pages2(from,&mut pages,PAGE_SIZE,1,&mut start);if copied<0{for i in 0..(*rm).data.op_nents{put_page(sg_page(&mut *(*rm).data.op_sg.add(i)));}(*rm).data.op_nents=0;mm_unaccount_pinned_pages(&mut (*(*rm).data.op_mmp_znotifier).z_mmp);kfree(info);(*rm).data.op_mmp_znotifier=core::ptr::null_mut();return -EFAULT;}length-=copied as u32;sg_set_page(&mut *sg,pages,copied as usize,start);(*rm).data.op_nents+=1;sg=sg.add(1);} WARN_ON_ONCE(length!=0);ret }

pub unsafe fn rds_message_copy_from_user(rm:*mut rds_message,from:*mut iov_iter,zcopy:bool)->i32 {(*rm).m_inc.i_hdr.h_len=cpu_to_be32(iov_iter_count(from));if zcopy{return rds_message_zcopy_from_user(rm,from);}let mut sg=(*rm).data.op_sg;let mut off=0usize;while iov_iter_count(from)>0{if sg_page(&mut *sg).is_null(){let r=rds_page_remainder_alloc(&mut *sg,iov_iter_count(from),GFP_HIGHUSER);if r!=0{return r;}(*rm).data.op_nents+=1;off=0;}let n=min(iov_iter_count(from),(*sg).length-off);rds_stats_add(s_copy_from_user,n);if copy_page_from_iter(sg_page(&mut *sg),(*sg).offset+off,n,from)!=n{return -EFAULT;}off+=n;if off==(*sg).length{sg=sg.add(1);}}0}

pub unsafe fn rds_message_inc_copy_to_user(inc:*mut rds_incoming,to:*mut iov_iter)->i32 {let rm=container_of!(inc,rds_message,m_inc);let len=be32_to_cpu((*rm).m_inc.i_hdr.h_len);let mut sg=(*rm).data.op_sg;let mut off=0usize;let mut copied=0u32;while iov_iter_count(to)>0&&copied<len{let n=min(iov_iter_count(to),(*sg).length-off).min((len-copied) as usize);rds_stats_add(s_copy_to_user,n);if copy_page_to_iter(sg_page(&mut *sg),(*sg).offset+off,n,to)!=n{return -EFAULT;}off+=n;copied+=n as u32;if off==(*sg).length{off=0;sg=sg.add(1);}}copied as i32}

pub unsafe fn rds_message_wait(rm:*mut rds_message){wait_event_interruptible!((*rm).m_flush_wait,!test_bit(RDS_MSG_MAPPED,&(*rm).m_flags));}
pub unsafe fn rds_message_unmapped(rm:*mut rds_message){clear_bit(RDS_MSG_MAPPED,&mut (*rm).m_flags);wake_up_interruptible(&mut (*rm).m_flush_wait);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
