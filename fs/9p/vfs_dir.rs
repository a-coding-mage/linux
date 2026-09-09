// SPDX-License-Identifier: GPL-2.0-only
/*
 * This file contains vfs directory ops for the 9P2000 protocol.
 *
 *  Copyright (C) 2004 by Eric Van Hensbergen <ericvh@gmail.com>
 *  Copyright (C) 2002 by Ron Minnich <rminnich@lanl.gov>
 */

// Linux and 9P declarations are supplied by the surrounding translation unit.

#[repr(C)]
pub struct p9_rdir {
    pub head: libc::c_int,
    pub tail: libc::c_int,
    pub offset: libc::loff_t,
    pub buf: [u8; 0],
}

#[inline]
unsafe fn dt_type(mistat: *mut p9_wstat) -> libc::c_int {
    let perm: libc::c_ulong = (*mistat).mode;
    let mut rettype: libc::c_int = DT_REG;

    if perm & P9_DMDIR != 0 {
        rettype = DT_DIR;
    }
    if perm & P9_DMSYMLINK != 0 {
        rettype = DT_LNK;
    }

    rettype
}

unsafe fn v9fs_alloc_rdir_buf(filp: *mut file, buflen: libc::c_int) -> *mut p9_rdir {
    let fid: *mut p9_fid = (*filp).private_data as *mut p9_fid;

    if (*fid).rdir.is_null() {
        (*fid).rdir = kvzalloc(
            core::mem::size_of::<p9_rdir>() + buflen as usize,
            GFP_KERNEL,
        ) as *mut p9_rdir;
    }
    (*fid).rdir
}

unsafe fn v9fs_dir_readdir(file: *mut file, ctx: *mut dir_context) -> libc::c_int {
    let mut over: bool;
    let mut st: p9_wstat = core::mem::zeroed();
    let mut err: libc::c_int = 0;
    let fid: *mut p9_fid;
    let buflen: libc::c_int;
    let rdir: *mut p9_rdir;
    let mut kvec: kvec = core::mem::zeroed();

    p9_debug(P9_DEBUG_VFS, b"name %pD\0".as_ptr() as *const libc::c_char, file);
    fid = (*file).private_data as *mut p9_fid;
    buflen = (*(*fid).clnt).msize - P9_IOHDRSZ;
    rdir = v9fs_alloc_rdir_buf(file, buflen);
    if rdir.is_null() { return -ENOMEM; }
    kvec.iov_base = (*rdir).buf.as_mut_ptr() as *mut libc::c_void;
    kvec.iov_len = buflen as usize;

    if (*rdir).head < (*rdir).tail && (*rdir).offset != (*ctx).pos {
        (*rdir).head = 0;
        (*rdir).tail = 0;
    }

    loop {
        if (*rdir).tail == (*rdir).head {
            let mut to: iov_iter = core::mem::zeroed();
            let n: libc::c_int;
            iov_iter_kvec(&mut to, ITER_DEST, &mut kvec, 1, buflen as usize);
            n = p9_client_read((*file).private_data, (*ctx).pos, &mut to, &mut err);
            if err != 0 { return err; }
            if n == 0 { return 0; }
            (*rdir).head = 0;
            (*rdir).tail = n;
            (*rdir).offset = (*ctx).pos;
        }
        while (*rdir).head < (*rdir).tail {
            err = p9stat_read((*fid).clnt, (*rdir).buf.as_mut_ptr().add((*rdir).head as usize),
                              ((*rdir).tail - (*rdir).head) as usize, &mut st);
            if err <= 0 {
                p9_debug(P9_DEBUG_VFS, b"returned %d\n\0".as_ptr() as *const libc::c_char, err);
                return -EIO;
            }
            over = !dir_emit(ctx, st.name, strlen(st.name), QID2INO(&st.qid), dt_type(&mut st));
            p9stat_free(&mut st);
            if over { return 0; }
            (*rdir).head += err;
            (*ctx).pos += err as i64;
            (*rdir).offset = (*ctx).pos;
        }
    }
}

unsafe fn v9fs_dir_readdir_dotl(file: *mut file, ctx: *mut dir_context) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let fid = (*file).private_data as *mut p9_fid;
    let buflen = (*(*fid).clnt).msize - P9_READDIRHDRSZ;
    let rdir = v9fs_alloc_rdir_buf(file, buflen);
    let mut curdirent: p9_dirent = core::mem::zeroed();
    if rdir.is_null() { return -ENOMEM; }
    if (*rdir).head < (*rdir).tail && (*rdir).offset != (*ctx).pos {
        (*rdir).head = 0; (*rdir).tail = 0;
    }
    loop {
        if (*rdir).tail == (*rdir).head {
            err = p9_client_readdir(fid, (*rdir).buf.as_mut_ptr(), buflen as usize, (*ctx).pos);
            if err <= 0 { return err; }
            (*rdir).head = 0; (*rdir).tail = err; (*rdir).offset = (*ctx).pos;
        }
        while (*rdir).head < (*rdir).tail {
            err = p9dirent_read((*fid).clnt, (*rdir).buf.as_mut_ptr().add((*rdir).head as usize),
                                ((*rdir).tail - (*rdir).head) as usize, &mut curdirent);
            if err < 0 { return -EIO; }
            if !dir_emit(ctx, curdirent.d_name, strlen(curdirent.d_name),
                         QID2INO(&curdirent.qid), curdirent.d_type) { return 0; }
            (*ctx).pos = curdirent.d_off;
            (*rdir).head += err;
            (*rdir).offset = (*ctx).pos;
        }
    }
}

pub unsafe fn v9fs_dir_release(inode: *mut inode, filp: *mut file) -> libc::c_int {
    let v9inode = V9FS_I(inode);
    let fid = (*filp).private_data as *mut p9_fid;
    let mut retval = 0;
    if !fid.is_null() {
        if S_ISREG((*inode).i_mode) && (*filp).f_mode & FMODE_WRITE != 0 {
            retval = filemap_fdatawrite((*inode).i_mapping);
        }
        spin_lock(&mut (*inode).i_lock);
        hlist_del(&mut (*fid).ilist);
        spin_unlock(&mut (*inode).i_lock);
        let put_err = p9_fid_put(fid);
        if retval >= 0 { retval = put_err; }
    }
    if (*filp).f_mode & FMODE_WRITE != 0 {
        let version = cpu_to_le32((*v9inode).qid.version);
        let mut i_size = i_size_read(inode);
        fscache_unuse_cookie(v9fs_inode_cookie(v9inode), &mut version, &mut i_size);
    } else {
        fscache_unuse_cookie(v9fs_inode_cookie(v9inode), core::ptr::null_mut(), core::ptr::null_mut());
    }
    retval
}

#[no_mangle]
pub static v9fs_dir_operations: file_operations = file_operations {
    read: Some(generic_read_dir), llseek: Some(generic_file_llseek),
    iterate_shared: Some(v9fs_dir_readdir), open: Some(v9fs_file_open),
    release: Some(v9fs_dir_release), ..unsafe { core::mem::zeroed() }
};

#[no_mangle]
pub static v9fs_dir_operations_dotl: file_operations = file_operations {
    read: Some(generic_read_dir), llseek: Some(generic_file_llseek),
    iterate_shared: Some(v9fs_dir_readdir_dotl), open: Some(v9fs_file_open),
    release: Some(v9fs_dir_release), fsync: Some(v9fs_file_fsync_dotl),
    ..unsafe { core::mem::zeroed() }
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
