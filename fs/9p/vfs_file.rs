// SPDX-License-Identifier: GPL-2.0-only
/* This file contains vfs file ops for 9P2000. */

// Linux kernel and local 9P headers provide the types and symbols referenced here.

static V9FS_MMAP_FILE_VM_OPS: vm_operations_struct = vm_operations_struct {
    close: Some(v9fs_mmap_vm_close),
    fault: Some(filemap_fault),
    map_pages: Some(filemap_map_pages),
    page_mkwrite: Some(v9fs_vm_page_mkwrite),
};

pub unsafe extern "C" fn v9fs_file_open(inode: *mut inode, file: *mut file) -> c_int {
    let mut err: c_int;
    let v9ses: *mut v9fs_session_info = v9fs_inode2v9ses(inode);
    let mut fid: *mut p9_fid;
    let omode: c_int;
    let o_append: c_int;
    p9_debug(P9_DEBUG_VFS, c_str!("inode: %p file: %p\n"), inode, file);
    if v9fs_proto_dotl(v9ses) { omode = v9fs_open_to_dotl_flags((*file).f_flags); o_append = P9_DOTL_APPEND; }
    else { omode = v9fs_uflags2omode((*file).f_flags, v9fs_proto_dotu(v9ses)); o_append = P9_OAPPEND; }
    fid = (*file).private_data as *mut p9_fid;
    if fid.is_null() {
        fid = v9fs_fid_clone(file_dentry(file));
        if IS_ERR(fid) { return PTR_ERR(fid); }
        if ((*v9ses).cache & CACHE_WRITEBACK) != 0 && (omode & P9_OWRITE) != 0 {
            let writeback_omode = (omode & !(P9_OWRITE | o_append)) | P9_ORDWR;
            p9_debug(P9_DEBUG_CACHE, c_str!("write-only file with writeback enabled, try opening O_RDWR\n"));
            err = p9_client_open(fid, writeback_omode);
            if err < 0 { p9_debug(P9_DEBUG_CACHE, c_str!("could not open O_RDWR, disabling caches\n")); err = p9_client_open(fid, omode); (*fid).mode |= P9L_DIRECT; }
        } else { err = p9_client_open(fid, omode); }
        if err < 0 { p9_fid_put(fid); return err; }
        if ((*file).f_flags & O_APPEND) != 0 && !v9fs_proto_dotu(v9ses) && !v9fs_proto_dotl(v9ses) { generic_file_llseek(file, 0, SEEK_END); }
        (*file).private_data = fid as *mut c_void;
    }
    v9fs_fid_add_modes(fid, (*v9ses).flags, (*v9ses).cache, (*file).f_flags);
    v9fs_open_fid_add(inode, &mut fid);
    0
}

unsafe fn v9fs_file_lock(filp: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int {
    let inode = file_inode(filp);
    p9_debug(P9_DEBUG_VFS, c_str!("filp: %p lock: %p\n"), filp, fl);
    if (IS_SETLK(cmd) || IS_SETLKW(cmd)) && (*fl).c.flc_type != F_UNLCK { filemap_write_and_wait((*inode).i_mapping); invalidate_mapping_pages(&mut (*inode).i_data, 0, -1); }
    0
}

unsafe fn v9fs_file_do_lock(filp: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int {
    let fid = (*filp).private_data as *mut p9_fid; BUG_ON(fid.is_null());
    BUG_ON(((*fl).c.flc_flags & FL_POSIX) != FL_POSIX);
    let mut res = locks_lock_file_wait(filp, fl); if res < 0 { return res; }
    let mut flock: p9_flock = core::mem::zeroed(); let mut status: u8 = P9_LOCK_ERROR;
    flock.r#type = match (*fl).c.flc_type { F_RDLCK => P9_LOCK_TYPE_RDLCK, F_WRLCK => P9_LOCK_TYPE_WRLCK, _ => P9_LOCK_TYPE_UNLCK };
    flock.start = (*fl).fl_start; flock.length = if (*fl).fl_end == OFFSET_MAX { 0 } else { (*fl).fl_end - (*fl).fl_start + 1 };
    flock.proc_id = (*fl).c.flc_pid; flock.client_id = (*(*fid).clnt).name;
    if IS_SETLKW(cmd) { flock.flags = P9_LOCK_FLAGS_BLOCK; }
    let v9ses = v9fs_inode2v9ses(file_inode(filp));
    loop { res = p9_client_lock_dotl(fid, &mut flock, &mut status); if res < 0 || status != P9_LOCK_BLOCKED || !IS_SETLKW(cmd) || schedule_timeout_interruptible((*v9ses).session_lock_timeout) != 0 { break; } if flock.client_id != (*(*fid).clnt).name { kfree(flock.client_id); flock.client_id = (*(*fid).clnt).name; } }
    if res >= 0 { res = match status { P9_LOCK_SUCCESS => 0, P9_LOCK_BLOCKED => -EAGAIN, _ => -ENOLCK }; }
    if res < 0 && (*fl).c.flc_type != F_UNLCK { let t = (*fl).c.flc_type; (*fl).c.flc_type = F_UNLCK; locks_lock_file_wait(filp, fl); (*fl).c.flc_type = t; }
    if flock.client_id != (*(*fid).clnt).name { kfree(flock.client_id); } res
}

unsafe fn v9fs_file_getlock(filp: *mut file, fl: *mut file_lock) -> c_int {
    let fid = (*filp).private_data as *mut p9_fid; BUG_ON(fid.is_null()); posix_test_lock(filp, fl); if (*fl).c.flc_type != F_UNLCK { return 0; }
    let mut glock: p9_getlock = core::mem::zeroed(); glock.r#type = P9_LOCK_TYPE_UNLCK; glock.start = (*fl).fl_start; glock.length = if (*fl).fl_end == OFFSET_MAX { 0 } else { (*fl).fl_end - (*fl).fl_start + 1 }; glock.proc_id = (*fl).c.flc_pid; glock.client_id = (*(*fid).clnt).name;
    let res = p9_client_getlock_dotl(fid, &mut glock); if res < 0 { return res; }
    (*fl).c.flc_type = match glock.r#type { P9_LOCK_TYPE_RDLCK => F_RDLCK, P9_LOCK_TYPE_WRLCK => F_WRLCK, _ => F_UNLCK };
    if glock.r#type != P9_LOCK_TYPE_UNLCK { (*fl).fl_start = glock.start; (*fl).fl_end = if glock.length == 0 { OFFSET_MAX } else { glock.start + glock.length - 1 }; (*fl).c.flc_pid = -glock.proc_id; }
    if glock.client_id != (*(*fid).clnt).name { kfree(glock.client_id); } res
}

unsafe fn v9fs_file_lock_dotl(f: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int { if (IS_SETLK(cmd)||IS_SETLKW(cmd)) && (*fl).c.flc_type != F_UNLCK { let i=file_inode(f); filemap_write_and_wait((*i).i_mapping); invalidate_mapping_pages(&mut (*i).i_data,0,-1); } if IS_SETLK(cmd)||IS_SETLKW(cmd) { v9fs_file_do_lock(f,cmd,fl) } else if IS_GETLK(cmd) { v9fs_file_getlock(f,fl) } else { -EINVAL } }
unsafe fn v9fs_file_flock_dotl(f: *mut file, cmd: c_int, fl: *mut file_lock) -> c_int { if ((*fl).c.flc_flags & FL_FLOCK)==0 { return -ENOLCK; } (*fl).c.flc_flags |= FL_POSIX; (*fl).c.flc_flags ^= FL_FLOCK; if IS_SETLK(cmd)|IS_SETLKW(cmd) { v9fs_file_do_lock(f,cmd,fl) } else { -EINVAL } }

unsafe fn v9fs_file_read_iter(i: *mut kiocb, to: *mut iov_iter) -> ssize_t { let f=(*i).ki_filp; let fid=(*f).private_data as *mut p9_fid; if ((*fid).mode&P9L_DIRECT)!=0 { netfs_unbuffered_read_iter(i,to) } else { netfs_file_read_iter(i,to) } }
unsafe fn v9fs_file_splice_read(f:*mut file,p:*mut loff_t,pipe:*mut pipe_inode_info,len:usize,flags:c_uint)->ssize_t { let fid=(*f).private_data as *mut p9_fid; if ((*fid).mode&P9L_DIRECT)!=0 { copy_splice_read(f,p,pipe,len,flags) } else { filemap_splice_read(f,p,pipe,len,flags) } }
unsafe fn v9fs_file_write_iter(i:*mut kiocb,from:*mut iov_iter)->ssize_t { let fid=(*(*i).ki_filp).private_data as *mut p9_fid; if ((*fid).mode&(P9L_DIRECT|P9L_NOWRITECACHE))!=0 { netfs_unbuffered_write_iter(i,from) } else { netfs_file_write_iter(i,from) } }

unsafe fn v9fs_file_fsync(f:*mut file,start:loff_t,end:loff_t,_datasync:c_int)->c_int { let mut w:p9_wstat=core::mem::zeroed(); let r=file_write_and_wait_range(f,start,end); if r!=0{return r;} let i=(*(*f).f_mapping).host; inode_lock(i); v9fs_blank_wstat(&mut w); let r=p9_client_wstat((*f).private_data as *mut p9_fid,&mut w); inode_unlock(i); r }
pub unsafe fn v9fs_file_fsync_dotl(f:*mut file,start:loff_t,end:loff_t,datasync:c_int)->c_int { let r=file_write_and_wait_range(f,start,end); if r!=0{return r;} let i=(*(*f).f_mapping).host; inode_lock(i); let r=p9_client_fsync((*f).private_data as *mut p9_fid,datasync); inode_unlock(i); r }

unsafe fn v9fs_file_mmap_prepare(d:*mut vm_area_desc)->c_int { let i=file_inode((*d).file); let s=v9fs_inode2v9ses(i); if ((*s).cache&CACHE_WRITEBACK)==0 { return generic_file_readonly_mmap_prepare(d); } let r=generic_file_mmap_prepare(d); if r==0 { (*d).vm_ops=&V9FS_MMAP_FILE_VM_OPS; } r }
unsafe fn v9fs_vm_page_mkwrite(v:*mut vm_fault)->vm_fault_t { netfs_page_mkwrite(v,core::ptr::null_mut()) }
unsafe fn v9fs_mmap_vm_close(v:*mut vm_area_struct) { if ((*v).vm_flags&VM_SHARED)==0{return;} filemap_fdatawrite_range(file_inode((*v).vm_file).i_mapping,(*v).vm_pgoff as loff_t*PAGE_SIZE,(*v).vm_pgoff as loff_t*PAGE_SIZE+((*v).vm_end-(*v).vm_start-1)); }

pub static V9FS_FILE_OPERATIONS: file_operations = file_operations { llseek:Some(generic_file_llseek), read_iter:Some(v9fs_file_read_iter), write_iter:Some(v9fs_file_write_iter), open:Some(v9fs_file_open), release:Some(v9fs_dir_release), lock:Some(v9fs_file_lock), mmap_prepare:Some(generic_file_readonly_mmap_prepare), splice_read:Some(v9fs_file_splice_read), splice_write:Some(iter_file_splice_write), fsync:Some(v9fs_file_fsync), ..file_operations::default() };
pub static V9FS_FILE_OPERATIONS_DOTL: file_operations = file_operations { llseek:Some(generic_file_llseek), read_iter:Some(v9fs_file_read_iter), write_iter:Some(v9fs_file_write_iter), open:Some(v9fs_file_open), release:Some(v9fs_dir_release), lock:Some(v9fs_file_lock_dotl), flock:Some(v9fs_file_flock_dotl), mmap_prepare:Some(v9fs_file_mmap_prepare), splice_read:Some(v9fs_file_splice_read), splice_write:Some(iter_file_splice_write), fsync:Some(v9fs_file_fsync_dotl), ..file_operations::default() };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
