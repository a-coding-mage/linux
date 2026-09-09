// SPDX-License-Identifier: GPL-2.0-only
/* Translated from linux/fs/nfs/file.c. External kernel symbols are supplied by
 * the surrounding kernel bindings. */

const NFSDBG_FACILITY: i32 = NFSDBG_FILE;

static NFS_FILE_VM_OPS: vm_operations_struct = vm_operations_struct {
    fault: Some(filemap_fault), map_pages: Some(filemap_map_pages),
    page_mkwrite: Some(nfs_vm_page_mkwrite),
};

pub unsafe fn nfs_check_flags(flags: i32) -> i32 {
    if flags & (O_APPEND | O_DIRECT) == (O_APPEND | O_DIRECT) { return -EINVAL; }
    0
}

unsafe fn nfs_file_open(inode: *mut inode, filp: *mut file) -> i32 {
    dprintk!("NFS: open file(%pD2)\n", filp); nfs_inc_stats(inode, NFSIOS_VFSOPEN);
    let res = nfs_check_flags((*filp).f_flags); if res != 0 { return res; }
    let res = nfs_open(inode, filp); if res == 0 {
        (*filp).f_mode |= FMODE_CAN_ODIRECT;
        if (*filp).f_mode & FMODE_WRITE == 0 { (*filp).f_mode |= FMODE_NOWAIT; }
    } res
}

pub unsafe fn nfs_file_release(inode: *mut inode, filp: *mut file) -> i32 {
    dprintk!("NFS: release(%pD2)\n", filp); nfs_inc_stats(inode, NFSIOS_VFSRELEASE);
    nfs_file_clear_open_context(filp); nfs_fscache_release_file(inode, filp); 0
}

unsafe fn nfs_revalidate_file_size(inode: *mut inode, filp: *mut file) -> i32 {
    let server = NFS_SERVER(inode);
    if (*filp).f_flags & O_DIRECT != 0 || nfs_check_cache_invalid(inode, NFS_INO_INVALID_SIZE) != 0 {
        return __nfs_revalidate_inode(server, inode);
    } 0
}

pub unsafe fn nfs_file_llseek(filp: *mut file, offset: loff_t, whence: i32) -> loff_t {
    dprintk!("NFS: llseek file(%pD2, %lld, %d)\n", filp, offset, whence);
    if whence != SEEK_SET && whence != SEEK_CUR {
        let inode = (*(*filp).f_mapping).host; let r = nfs_revalidate_file_size(inode, filp);
        if r < 0 { return r as loff_t; }
    } generic_file_llseek(filp, offset, whence)
}

unsafe fn nfs_file_flush(file: *mut file, _id: fl_owner_t) -> i32 {
    let inode = file_inode(file); dprintk!("NFS: flush(%pD2)\n", file);
    nfs_inc_stats(inode, NFSIOS_VFSFLUSH); if (*file).f_mode & FMODE_WRITE == 0 { return 0; }
    let since = filemap_sample_wb_err((*file).f_mapping); nfs_wb_all(inode);
    filemap_check_wb_err((*file).f_mapping, since)
}

pub unsafe fn nfs_file_read(iocb: *mut kiocb, to: *mut iov_iter) -> ssize_t {
    let inode = file_inode((*iocb).ki_filp); trace_nfs_file_read(iocb, to);
    if (*iocb).ki_flags & IOCB_DIRECT != 0 { return nfs_file_direct_read(iocb, to, false); }
    if (*iocb).ki_flags & IOCB_NOWAIT != 0 { return -EAGAIN as ssize_t; }
    let mut result = nfs_start_io_read(inode); if result != 0 { return result as ssize_t; }
    result = nfs_revalidate_mapping(inode, (*iocb).ki_filp_f_mapping);
    if result == 0 { result = generic_file_read_iter(iocb, to); if result > 0 { nfs_add_stats(inode, NFSIOS_NORMALREADBYTES, result); } }
    nfs_end_io_read(inode); result as ssize_t
}

pub unsafe fn nfs_file_splice_read(input: *mut file, ppos: *mut loff_t, pipe: *mut pipe_inode_info, len: usize, flags: u32) -> ssize_t {
    let inode = file_inode(input); let mut result = nfs_start_io_read(inode); if result != 0 { return result as ssize_t; }
    result = nfs_revalidate_mapping(inode, (*input).f_mapping);
    if result == 0 { result = filemap_splice_read(input, ppos, pipe, len, flags); if result > 0 { nfs_add_stats(inode, NFSIOS_NORMALREADBYTES, result); } }
    nfs_end_io_read(inode); result as ssize_t
}

pub unsafe fn nfs_file_mmap_prepare(desc: *mut vm_area_desc) -> i32 {
    let file = (*desc).file; let inode = file_inode(file); let mut status = generic_file_mmap_prepare(desc);
    if status == 0 { (*desc).vm_ops = &NFS_FILE_VM_OPS; status = nfs_revalidate_mapping(inode, (*file).f_mapping); } status
}

unsafe fn nfs_file_fsync_commit(file: *mut file, _datasync: i32) -> i32 {
    let inode = file_inode(file); nfs_inc_stats(inode, NFSIOS_VFSFSYNC);
    let ret = nfs_commit_inode(inode, FLUSH_SYNC); let ret2 = file_check_and_advance_wb_err(file);
    if ret2 < 0 { ret2 } else { ret }
}

pub unsafe fn nfs_file_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: i32) -> i32 {
    let inode = file_inode(file); let nfsi = NFS_I(inode); let mut save = atomic_long_read(&(*nfsi).redirtied_pages); let mut ret;
    trace_nfs_fsync_enter(inode);
    loop { ret = file_write_and_wait_range(file, start, end); if ret != 0 { break; } ret = nfs_file_fsync_commit(file, datasync); if ret != 0 { break; }
        ret = pnfs_sync_inode(inode, datasync != 0); if ret != 0 { break; } let n = atomic_long_read(&(*nfsi).redirtied_pages); if n == save { break; } save = n; }
    trace_nfs_fsync_exit(inode, ret); ret
}

pub unsafe fn nfs_truncate_last_folio(mapping: *mut address_space, from: loff_t, to: loff_t) {
    if from >= to { return; } let folio = filemap_lock_folio(mapping, from >> PAGE_SHIFT); if IS_ERR(folio) { return; }
    if folio_mkclean(folio) { folio_mark_dirty(folio); } if folio_test_uptodate(folio) { let fpos = folio_pos(folio); let offset = (from-fpos) as usize; let mut end = folio_size(folio); if to-fpos < end as loff_t { end = (to-fpos) as usize; } folio_zero_segment(folio, offset, end); trace_nfs_size_truncate_folio((*mapping).host, to); }
    folio_unlock(folio); folio_put(folio);
}

unsafe fn nfs_folio_is_full_write(folio: *mut folio, pos: loff_t, len: u32) -> bool { let pglen=nfs_folio_length(folio); let off=offset_in_folio(folio,pos); ! (pglen != 0 && !(off + len >= pglen && off == 0)) }
unsafe fn nfs_want_read_modify_write(file:*mut file, folio:*mut folio,pos:loff_t,len:u32)->bool { if folio_test_uptodate(folio)||folio_test_private(folio)||nfs_folio_is_full_write(folio,pos,len){return false;} if pnfs_ld_read_whole_page(file_inode(file)){return true;} if folio_test_dropbehind(folio){return false;} (*file).f_mode&FMODE_READ!=0 }

unsafe fn nfs_write_begin(iocb:*const kiocb,mapping:*mut address_space,pos:loff_t,len:u32,foliop:*mut *mut folio,_fsdata:*mut *mut core::ffi::c_void)->i32 { let file=(*iocb).ki_filp; let mut once=false; trace_nfs_write_begin(file_inode(file),pos,len); nfs_truncate_last_folio(mapping,i_size_read((*mapping).host),pos); loop { let folio=write_begin_get_folio(iocb,mapping,pos>>PAGE_SHIFT,len); if IS_ERR(folio){return PTR_ERR(folio);} *foliop=folio; let mut ret=nfs_flush_incompatible(file,folio); if ret!=0 {folio_unlock(folio);folio_put(folio);} else if !once&&nfs_want_read_modify_write(file,folio,pos,len){once=true;folio_clear_dropbehind(folio);ret=nfs_read_folio(file,folio);folio_put(folio);if ret==0{continue;}} trace_nfs_write_begin_done(file_inode(file),pos,len,ret); return ret;} }

unsafe fn nfs_write_end(iocb:*const kiocb,mapping:*mut address_space,pos:loff_t,len:u32,copied:u32,folio:*mut folio,_fsdata:*mut core::ffi::c_void)->i32 { let file=(*iocb).ki_filp; let ctx=nfs_file_open_context(file); let offset=offset_in_folio(folio,pos); if !folio_test_uptodate(folio){let size=folio_size(folio);let pg=nfs_folio_length(folio);let end=offset+copied;if pg==0{folio_zero_segments(folio,0,offset,end,size);folio_mark_uptodate(folio);}else if end>=pg{folio_zero_segment(folio,end,size);if offset==0{folio_mark_uptodate(folio);}}else{folio_zero_segment(folio,pg,size);}} let status=nfs_update_folio(file,folio,offset,copied); folio_unlock(folio);folio_put(folio);if status<0{return status;} (*NFS_I((*mapping).host)).write_io+=copied as _;if nfs_ctx_key_to_expire(ctx,(*mapping).host){nfs_wb_all((*mapping).host);}copied as i32 }

unsafe fn nfs_invalidate_folio(folio:*mut folio,offset:usize,length:usize){let inode=(*(*folio).mapping).host;if offset!=0||length<folio_size(folio){nfs_wb_folio(inode,folio);}else{nfs_wb_folio_cancel(inode,folio);}folio_wait_private_2(folio);trace_nfs_invalidate_folio(inode,folio_pos(folio)+offset as i64,length);}
unsafe fn nfs_release_folio(folio:*mut folio,gfp:gfp_t)->bool{if folio_test_private(folio){if current_gfp_context(gfp)&GFP_KERNEL!=GFP_KERNEL||current_is_kswapd()||current_is_kcompactd(){return false;}if nfs_wb_folio_reclaim((*(*folio).mapping).host,folio)<0||folio_test_private(folio){return false;}}nfs_fscache_release_folio(folio,gfp)}
unsafe fn nfs_check_dirty_writeback(folio:*mut folio,dirty:*mut bool,writeback:*mut bool){let nfsi=NFS_I((*(*folio).mapping).host);if atomic_read(&(*nfsi).commit_info.rpcs_out)!=0{*writeback=true;}else if folio_test_private(folio){*dirty=true;}}
unsafe fn nfs_launder_folio(folio:*mut folio)->i32{let inode=(*(*folio).mapping).host;folio_wait_private_2(folio);nfs_wb_folio(inode,folio)}

// CONFIG_SWAP-dependent callbacks and the remaining kernel operation tables.
#[cfg(feature="CONFIG_SWAP")] unsafe fn nfs_swap_submit_write(ctx:*mut swap_io_ctx){let sio=(*ctx).sio;let mut iter=core::mem::zeroed();swap_fs_prepare_rw(ctx,WRITE,&mut iter);let ret=nfs_file_direct_write(&mut (*sio).iocb,&mut iter,true);if ret!=-EIOCBQUEUED{((*sio).iocb.ki_complete)(&mut (*sio).iocb,ret);}}
#[cfg(feature="CONFIG_SWAP")] unsafe fn nfs_swap_submit_read(ctx:*mut swap_io_ctx){let sio=(*ctx).sio;let mut iter=core::mem::zeroed();swap_fs_prepare_rw(ctx,READ,&mut iter);let ret=nfs_file_direct_read(&mut (*sio).iocb,&mut iter,true);if ret!=-EIOCBQUEUED{((*sio).iocb.ki_complete)(&mut (*sio).iocb,ret);}}

pub static NFS_FILE_AOPS: address_space_operations = address_space_operations { read_folio:Some(nfs_read_folio), readahead:Some(nfs_readahead), dirty_folio:Some(filemap_dirty_folio), writepages:Some(nfs_writepages), write_begin:Some(nfs_write_begin), write_end:Some(nfs_write_end), invalidate_folio:Some(nfs_invalidate_folio), release_folio:Some(nfs_release_folio), migrate_folio:Some(nfs_migrate_folio), launder_folio:Some(nfs_launder_folio), is_dirty_writeback:Some(nfs_check_dirty_writeback), error_remove_folio:Some(generic_error_remove_folio), swap_activate:None, swap_deactivate:None };

unsafe fn nfs_vm_page_mkwrite(vmf:*mut vm_fault)->vm_fault_t{let filp=(*(*vmf).vma).vm_file;let inode=file_inode(filp);let folio=page_folio((*vmf).page);sb_start_pagefault((*inode).i_sb);folio_lock(folio);if (*folio).mapping!=(*inode).i_mapping{folio_unlock(folio);sb_end_pagefault((*inode).i_sb);return VM_FAULT_NOPAGE;}folio_wait_writeback(folio);let len=nfs_folio_length(folio);let ret=if len==0{VM_FAULT_NOPAGE}else if nfs_flush_incompatible(filp,folio)==0&&nfs_update_folio(filp,folio,0,len)==0{VM_FAULT_LOCKED}else{VM_FAULT_SIGBUS};if ret!=VM_FAULT_LOCKED{folio_unlock(folio);}sb_end_pagefault((*inode).i_sb);ret}

pub unsafe fn nfs_file_write(iocb:*mut kiocb,from:*mut iov_iter)->ssize_t{let file=(*iocb).ki_filp;let inode=file_inode(file);if (*iocb).ki_flags&IOCB_NOWAIT!=0{return -EAGAIN as ssize_t;}let r=nfs_key_timeout_notify(file,inode);if r!=0{return r as ssize_t;}if (*iocb).ki_flags&IOCB_DIRECT!=0{return nfs_file_direct_write(iocb,from,false);}if IS_SWAPFILE(inode){return -ETXTBSY as ssize_t;}if (*iocb).ki_flags&IOCB_APPEND!=0||(*iocb).ki_pos>i_size_read(inode){let r=nfs_revalidate_file_size(inode,file);if r!=0{return r as ssize_t;}}nfs_clear_invalid_mapping((*file).f_mapping);let since=filemap_sample_wb_err((*file).f_mapping);let mut r=nfs_start_io_write(inode);if r!=0{return r as ssize_t;}r=generic_write_checks(iocb,from);if r>0{r=generic_perform_write(iocb,from);}nfs_end_io_write(inode);if r>0{nfs_add_stats(inode,NFSIOS_NORMALWRITTENBYTES,r);}let e=filemap_check_wb_err((*file).f_mapping,since);if e<0&&r>0{r=e;}r as ssize_t}

pub unsafe fn nfs_lock(filp:*mut file,cmd:i32,fl:*mut file_lock)->i32{let inode=(*(*filp).f_mapping).host;if (*fl).c.flc_flags&FL_RECLAIM!=0{return -ENOGRACE;}let local=(NFS_SERVER(inode).flags&NFS_MOUNT_LOCAL_FCNTL)!=0;if IS_GETLK(cmd){posix_test_lock(filp,fl);if (*fl).c.flc_type!=F_UNLCK{return 0;}if !local{return NFS_PROTO(inode).lock(filp,cmd,fl);}}else if lock_is_unlock(fl){nfs_wb_all(inode);if !local{return NFS_PROTO(inode).lock(filp,cmd,fl);}return locks_lock_file_wait(filp,fl);}else{let mut r=nfs_sync_mapping((*filp).f_mapping);if r==0{r=if local{locks_lock_file_wait(filp,fl)}else{NFS_PROTO(inode).lock(filp,cmd,fl)};}return r;}0}
pub unsafe fn nfs_flock(filp:*mut file,cmd:i32,fl:*mut file_lock)->i32{if (*fl).c.flc_flags&FL_FLOCK==0{return -ENOLCK;}let local=(NFS_SERVER((*(*filp).f_mapping).host).flags&NFS_MOUNT_LOCAL_FLOCK)!=0;if local{locks_lock_file_wait(filp,fl)}else{NFS_PROTO((*(*filp).f_mapping).host).lock(filp,cmd,fl)}}

pub static NFS_FILE_OPERATIONS: file_operations = file_operations { llseek:Some(nfs_file_llseek), read_iter:Some(nfs_file_read), write_iter:Some(nfs_file_write), mmap_prepare:Some(nfs_file_mmap_prepare), open:Some(nfs_file_open), flush:Some(nfs_file_flush), release:Some(nfs_file_release), fsync:Some(nfs_file_fsync), lock:Some(nfs_lock), flock:Some(nfs_flock), splice_read:Some(nfs_file_splice_read), splice_write:Some(iter_file_splice_write), check_flags:Some(nfs_check_flags), fop_flags:FOP_DONTCACHE };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
