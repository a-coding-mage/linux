// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2015, Primary Data, Inc. All rights reserved.
 *
 * Tao Peng <bergwolf@primarydata.com>
 */

// Linux kernel dependencies: linux/dcache.h, linux/exportfs.h, linux/nfs.h,
// linux/nfs_fs.h, "internal.h", and "nfstrace.h".

const NFSDBG_FACILITY: u32 = NFSDBG_VFS;

enum {
    FILEID_HIGH_OFF = 0,
    FILEID_LOW_OFF,
    FILE_I_TYPE_OFF,
    EMBED_FH_OFF,
}

unsafe fn nfs_exp_embedfh(p: *mut __u32) -> *mut nfs_fh {
    p.add(EMBED_FH_OFF) as *mut nfs_fh
}

/*
 * Let's break subtree checking for now... otherwise we'll have to embed parent fh
 * but there might not be enough space.
 */
unsafe fn nfs_encode_fh(
    inode: *mut inode,
    p: *mut __u32,
    max_len: *mut ::core::ffi::c_int,
    parent: *mut inode,
) -> ::core::ffi::c_int {
    let server_fh: *mut nfs_fh = NFS_FH(inode);
    let clnt_fh: *mut nfs_fh = nfs_exp_embedfh(p);
    let fh_size: usize = core::mem::offset_of!(nfs_fh, data) + (*server_fh).size as usize;
    let len: ::core::ffi::c_int = EMBED_FH_OFF as ::core::ffi::c_int + XDR_QUADLEN(fh_size);

    dprintk!("%s: max fh len %d inode %p parent %p", __func__, *max_len, inode, parent);

    if *max_len < len {
        dprintk!("%s: fh len %d too small, required %d\n", __func__, *max_len, len);
        *max_len = len;
        return FILEID_INVALID;
    }

    *p.add(FILEID_HIGH_OFF) = (*inode).i_ino >> 32;
    *p.add(FILEID_LOW_OFF) = (*inode).i_ino as __u32;
    *p.add(FILE_I_TYPE_OFF) = (*inode).i_mode & S_IFMT;
    *p.add((len - 1) as usize) = 0; // Padding
    nfs_copy_fh(clnt_fh, server_fh);
    *max_len = len;
    dprintk!("%s: result fh fileid %llu mode %u size %d\n", __func__, (*inode).i_ino, (*inode).i_mode, *max_len);
    *max_len
}

unsafe fn nfs_fh_to_dentry(
    sb: *mut super_block,
    fid: *mut fid,
    fh_len: ::core::ffi::c_int,
    fh_type: ::core::ffi::c_int,
) -> *mut dentry {
    let mut fattr: *mut nfs_fattr = core::ptr::null_mut();
    let server_fh: *mut nfs_fh = nfs_exp_embedfh((*fid).raw.as_mut_ptr());
    let mut fh_size: usize = core::mem::offset_of!(nfs_fh, data);
    let rpc_ops: *const nfs_rpc_ops;
    let dentry: *mut dentry;
    let inode: *mut inode;
    let mut len: ::core::ffi::c_int = EMBED_FH_OFF as ::core::ffi::c_int;
    let p: *mut u32 = (*fid).raw.as_mut_ptr();
    let ret: ::core::ffi::c_int;

    if fh_len < len + XDR_QUADLEN(fh_size) || fh_len > XDR_QUADLEN(NFS_MAXFHSIZE as usize) { return core::ptr::null_mut(); }
    fh_size += (*server_fh).size as usize;
    len += XDR_QUADLEN(fh_size);
    if fh_len < len || fh_type != len { return core::ptr::null_mut(); }

    fattr = nfs_alloc_fattr_with_label(NFS_SB(sb));
    if fattr.is_null() { return ERR_PTR(-ENOMEM); }

    (*fattr).fileid = ((*p.add(FILEID_HIGH_OFF) as u64) << 32) + *p.add(FILEID_LOW_OFF) as u64;
    (*fattr).mode = *p.add(FILE_I_TYPE_OFF);
    (*fattr).valid |= NFS_ATTR_FATTR_FILEID | NFS_ATTR_FATTR_TYPE;
    dprintk!("%s: fileid %llu mode %d\n", __func__, (*fattr).fileid, (*fattr).mode);

    inode = nfs_ilookup(sb, fattr, server_fh);
    if !inode.is_null() { dentry = d_obtain_alias(inode); nfs_free_fattr(fattr); return dentry; }

    rpc_ops = (*NFS_SB(sb)).nfs_client.rpc_ops;
    ret = ((*rpc_ops).getattr)(NFS_SB(sb), server_fh, fattr, core::ptr::null_mut());
    if ret != 0 {
        dprintk!("%s: getattr failed %d\n", __func__, ret);
        trace_nfs_fh_to_dentry(sb, server_fh, (*fattr).fileid, ret);
        dentry = ERR_PTR(ret);
        nfs_free_fattr(fattr);
        return dentry;
    }
    let inode = nfs_fhget(sb, server_fh, fattr);
    dentry = d_obtain_alias(inode);
    nfs_free_fattr(fattr);
    dentry
}

unsafe fn nfs_get_parent(dentry: *mut dentry) -> *mut dentry {
    let inode = d_inode(dentry);
    let sb = (*inode).i_sb;
    let server = NFS_SB(sb);
    let ops = (*(*server).nfs_client).rpc_ops;
    if (*ops).lookupp.is_none() { return ERR_PTR(-EACCES); }
    let fattr = nfs_alloc_fattr_with_label(server);
    if fattr.is_null() { return ERR_PTR(-ENOMEM); }
    let mut fh: nfs_fh = core::mem::zeroed();
    let ret = ((*ops).lookupp.unwrap())(inode, &mut fh, fattr);
    if ret != 0 { nfs_free_fattr(fattr); return ERR_PTR(ret); }
    let pinode = nfs_fhget(sb, &mut fh, fattr);
    let parent = d_obtain_alias(pinode);
    nfs_free_fattr(fattr);
    parent
}

const nfs_export_ops: export_operations = export_operations {
    encode_fh: Some(nfs_encode_fh),
    fh_to_dentry: Some(nfs_fh_to_dentry),
    get_parent: Some(nfs_get_parent),
    flags: EXPORT_OP_NOWCC | EXPORT_OP_NOSUBTREECHK | EXPORT_OP_CLOSE_BEFORE_UNLINK |
        EXPORT_OP_REMOTE_FS | EXPORT_OP_NOATOMIC_ATTR | EXPORT_OP_FLUSH_ON_CLOSE | EXPORT_OP_NOLOCKS,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
