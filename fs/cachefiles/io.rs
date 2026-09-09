// SPDX-License-Identifier: GPL-2.0-or-later
/* kiocb-using read/write
 *
 * Copyright (C) 2021 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
union CachefilesKiocbSize { skipped: usize, len: usize }

#[repr(C)]
struct cachefiles_kiocb {
    iocb: kiocb,
    ki_refcnt: refcount_t,
    start: loff_t,
    size: CachefilesKiocbSize,
    object: *mut cachefiles_object,
    term_func: netfs_io_terminated_t,
    term_func_priv: *mut c_void,
    was_async: bool,
    inval_counter: c_uint,
    b_writing: u64,
}

#[inline]
unsafe fn cachefiles_put_kiocb(ki: *mut cachefiles_kiocb) {
    if refcount_dec_and_test(&mut (*ki).ki_refcnt) {
        cachefiles_put_object((*ki).object, cachefiles_obj_put_ioreq);
        fput((*ki).iocb.ki_filp);
        kfree(ki as *mut c_void);
    }
}

unsafe fn cachefiles_read_complete(iocb: *mut kiocb, mut ret: c_long) {
    let ki = container_of!(iocb, cachefiles_kiocb, iocb);
    let inode = file_inode((*ki).iocb.ki_filp);
    _enter!("%ld", ret);
    if ret < 0 { trace_cachefiles_io_error((*ki).object, inode, ret, cachefiles_trace_read_error); }
    if !(*ki).term_func.is_none() {
        if ret >= 0 {
            if (*(*ki).object).cookie.inval_counter == (*ki).inval_counter {
                (*ki).size.skipped += ret as usize;
            } else { ret = -ESTALE; }
        }
        ((*ki).term_func.unwrap())((*ki).term_func_priv, ret);
    }
    cachefiles_put_kiocb(ki);
}

unsafe fn cachefiles_read(cres: *mut netfs_cache_resources, start_pos: loff_t,
    iter: *mut iov_iter, read_hole: netfs_read_from_hole,
    term_func: netfs_io_terminated_t, term_func_priv: *mut c_void) -> c_int {
    let mut ret: ssize_t = -ENOBUFS as ssize_t;
    let len = iov_iter_count(iter); let mut skipped = 0usize;
    if !fscache_wait_for_operation(cres, FSCACHE_WANT_READ) { goto_presubmission!(term_func, term_func_priv, ret); }
    fscache_count_read();
    let object = cachefiles_cres_object(cres); let file = cachefiles_cres_file(cres);
    _enter!("%pD,%llu,%llx,%zx/%llx", file, file_inode(file).i_ino, start_pos, len, i_size_read(file_inode(file)));
    if read_hole != NETFS_READ_HOLE_IGNORE {
        let off = start_pos; let mut off2 = cachefiles_inject_read_error();
        if off2 == 0 { off2 = vfs_llseek(file, off, SEEK_DATA); }
        if off2 < 0 && off2 >= -(MAX_ERRNO as loff_t) && off2 != -ENXIO { ret = off2 as ssize_t; goto_presubmission!(term_func, term_func_priv, ret); }
        if off2 == -ENXIO || off2 >= start_pos + len as loff_t {
            ret = -ENODATA as ssize_t; if read_hole == NETFS_READ_HOLE_FAIL { goto_presubmission!(term_func, term_func_priv, ret); }
            iov_iter_zero(len, iter); skipped = len; ret = 0; goto_presubmission!(term_func, term_func_priv, ret);
        }
        skipped = (off2 - off) as usize; iov_iter_zero(skipped, iter);
    }
    let ki = kzalloc_obj::<cachefiles_kiocb>();
    if ki.is_null() { ret = -ENOMEM as ssize_t; goto_presubmission!(term_func, term_func_priv, ret); }
    refcount_set(&mut (*ki).ki_refcnt, 2); (*ki).iocb.ki_filp=file; (*ki).iocb.ki_pos=start_pos+skipped as loff_t;
    (*ki).iocb.ki_flags=IOCB_DIRECT; (*ki).iocb.ki_ioprio=get_current_ioprio(); (*ki).size.skipped=skipped;
    (*ki).object=object; (*ki).inval_counter=(*cres).inval_counter; (*ki).term_func=term_func; (*ki).term_func_priv=term_func_priv; (*ki).was_async=true;
    if !term_func.is_none() { (*ki).iocb.ki_complete=Some(cachefiles_read_complete); }
    get_file((*ki).iocb.ki_filp); cachefiles_grab_object(object, cachefiles_obj_get_ioreq);
    trace_cachefiles_read(object, file_inode(file), (*ki).iocb.ki_pos, len-skipped);
    let old_nofs=memalloc_nofs_save(); ret=cachefiles_inject_read_error(); if ret==0 { ret=vfs_iocb_iter_read(file,&mut (*ki).iocb,iter); } memalloc_nofs_restore(old_nofs);
    if ret == -EIOCBQUEUED { cachefiles_put_kiocb(ki); _leave!(" = %zd",ret); return ret as c_int; }
    if ret == -ERESTARTSYS || ret == -ERESTARTNOINTR || ret == -ERESTARTNOHAND || ret == -ERESTART_RESTARTBLOCK { ret=-EINTR; }
    (*ki).was_async=false; cachefiles_read_complete(&mut (*ki).iocb,ret); if ret>0 { ret=0; }
    cachefiles_put_kiocb(ki); _leave!(" = %zd",ret); ret as c_int
}

// The remaining operations retain the kernel API's function signatures and control flow.
// Types and helpers referenced below are declared by the surrounding kernel translation.

unsafe fn cachefiles_query_occupancy(cres:*mut netfs_cache_resources,start:loff_t,len:usize,mut granularity:usize,data_start:*mut loff_t,data_len:*mut usize)->c_int {
    *data_start=-1; *data_len=0; if !fscache_wait_for_operation(cres,FSCACHE_WANT_READ){return -ENOBUFS;}
    let object=cachefiles_cres_object(cres); let file=cachefiles_cres_file(cres); granularity=max_t!(usize,(*(*object).volume).cache.bsize,granularity);
    let mut off=cachefiles_inject_read_error(); if off==0 {off=vfs_llseek(file,start,SEEK_DATA);} if off==-ENXIO{return -ENODATA;} if off<0&&off>=-(MAX_ERRNO as loff_t){return -ENOBUFS;} if round_up(off,granularity)>=start+len as loff_t{return -ENODATA;}
    let mut off2=cachefiles_inject_read_error(); if off2==0{off2=vfs_llseek(file,off,SEEK_HOLE);} if off2==-ENXIO{return -ENODATA;} if off2<0&&off2>=-(MAX_ERRNO as loff_t){return -ENOBUFS;}
    off=round_up(off,granularity); off2=round_down(off2,granularity); if off2<=off{return -ENODATA;} *data_start=off; *data_len=if off2>start+len as loff_t{len}else{(off2-off) as usize}; 0
}

// Write, preparation, cache operation callbacks, and begin-operation declarations.
// These are kept as direct kernel-facing declarations pending the shared type universe.
extern "C" {
    fn __cachefiles_write(object:*mut cachefiles_object,file:*mut file,start_pos:loff_t,iter:*mut iov_iter,term_func:netfs_io_terminated_t,priv_:*mut c_void)->c_int;
    fn cachefiles_begin_operation(cres:*mut netfs_cache_resources,want_state:fscache_want_state)->bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
