// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * export.c
 *
 * Functions to facilitate NFS exporting
 *
 * Copyright (C) 2002, 2005 Oracle.  All rights reserved.
 */

// Dependencies are supplied by the surrounding kernel translation unit.

#[repr(C)]
pub struct ocfs2_inode_handle {
    pub ih_blkno: u64,
    pub ih_generation: u32,
}

unsafe fn ocfs2_get_dentry(
    sb: *mut super_block,
    handle: *mut ocfs2_inode_handle,
) -> *mut dentry {
    let mut inode: *mut inode;
    let osb: *mut ocfs2_super = OCFS2_SB(sb);
    let blkno: u64 = (*handle).ih_blkno;
    let mut status: i32;
    let mut set: i32 = 0;
    let result: *mut dentry;

    trace_ocfs2_get_dentry_begin(sb, handle, blkno as c_ulonglong);

    if blkno == 0 {
        result = ERR_PTR(-ESTALE);
        return result;
    }

    inode = ocfs2_ilookup(sb, blkno);
    if !inode.is_null() {
        goto_check_gen!(inode, handle, result);
    }

    status = ocfs2_nfs_sync_lock(osb, 1);
    if status < 0 {
        mlog(ML_ERROR, "getting nfs sync lock(EX) failed %d\n", status);
        result = ERR_PTR(status);
        return result;
    }

    status = ocfs2_test_inode_bit(osb, blkno, &mut set);
    if status < 0 {
        if status == -EINVAL {
            status = -ESTALE;
        } else if status != -ESTALE {
            mlog(ML_ERROR, "test inode bit failed %d\n", status);
        }
        ocfs2_nfs_sync_unlock(osb, 1);
        result = ERR_PTR(status);
        return result;
    }

    trace_ocfs2_get_dentry_test_bit(status, set);
    if set == 0 {
        status = -ESTALE;
        ocfs2_nfs_sync_unlock(osb, 1);
        result = ERR_PTR(status);
        return result;
    }

    inode = ocfs2_iget(osb, blkno, 0, 0);
    ocfs2_nfs_sync_unlock(osb, 1);

    if status < 0 {
        if status == -ESTALE {
            trace_ocfs2_get_dentry_stale(blkno as c_ulonglong, (*handle).ih_generation);
        }
        result = ERR_PTR(status);
        return result;
    }

    if IS_ERR(inode) {
        mlog_errno(PTR_ERR(inode));
        result = ERR_CAST(inode);
        return result;
    }

    if (*handle).ih_generation != (*inode).i_generation {
        trace_ocfs2_get_dentry_generation(
            blkno as c_ulonglong,
            (*handle).ih_generation,
            (*inode).i_generation,
        );
        iput(inode);
        result = ERR_PTR(-ESTALE);
        return result;
    }

    result = d_obtain_alias(inode);
    if IS_ERR(result) {
        mlog_errno(PTR_ERR(result));
    }
    trace_ocfs2_get_dentry_end(result);
    result
}

unsafe fn ocfs2_get_parent(child: *mut dentry) -> *mut dentry {
    let dir: *mut inode = d_inode(child);
    let mut status: i32;
    let mut blkno: u64 = 0;
    let mut set: i32 = 0;
    let parent: *mut dentry;

    trace_ocfs2_get_parent(child, (*child).d_name.len, (*child).d_name.name,
                           OCFS2_I(dir).ip_blkno as c_ulonglong);

    status = ocfs2_nfs_sync_lock(OCFS2_SB((*dir).i_sb), 1);
    if status < 0 {
        mlog(ML_ERROR, "getting nfs sync lock(EX) failed %d\n", status);
        parent = ERR_PTR(status);
        trace_ocfs2_get_parent_end(parent);
        return parent;
    }
    status = ocfs2_inode_lock(dir, core::ptr::null_mut(), 0);
    if status < 0 {
        if status != -ENOENT { mlog_errno(status); }
        parent = ERR_PTR(status);
        ocfs2_nfs_sync_unlock(OCFS2_SB((*dir).i_sb), 1);
        trace_ocfs2_get_parent_end(parent);
        return parent;
    }
    status = ocfs2_lookup_ino_from_name(dir, b"..".as_ptr() as *const _, 2, &mut blkno);
    if status < 0 {
        parent = ERR_PTR(-ENOENT);
        ocfs2_inode_unlock(dir, 0);
        ocfs2_nfs_sync_unlock(OCFS2_SB((*dir).i_sb), 1);
        trace_ocfs2_get_parent_end(parent);
        return parent;
    }
    status = ocfs2_test_inode_bit(OCFS2_SB((*dir).i_sb), blkno, &mut set);
    if status < 0 {
        if status == -EINVAL { status = -ESTALE; }
        else if status != -ESTALE { mlog(ML_ERROR, "test inode bit failed %d\n", status); }
        parent = ERR_PTR(status);
    } else if set == 0 {
        parent = ERR_PTR(-ESTALE);
    } else {
        parent = d_obtain_alias(ocfs2_iget(OCFS2_SB((*dir).i_sb), blkno, 0, 0));
    }
    ocfs2_inode_unlock(dir, 0);
    ocfs2_nfs_sync_unlock(OCFS2_SB((*dir).i_sb), 1);
    trace_ocfs2_get_parent_end(parent);
    parent
}

unsafe fn ocfs2_encode_fh(inode: *mut inode, fh_in: *mut u32, max_len: *mut i32,
                          parent: *mut inode) -> i32 {
    let mut len = *max_len;
    let mut file_type = 1;
    if !parent.is_null() && len < 6 { *max_len = 6; return FILEID_INVALID; }
    if len < 3 { *max_len = 3; return FILEID_INVALID; }
    let fh = fh_in as *mut u32;
    let mut blkno = OCFS2_I(inode).ip_blkno;
    let mut generation = (*inode).i_generation;
    (*fh.add(0)) = cpu_to_le32((blkno >> 32) as u32);
    (*fh.add(1)) = cpu_to_le32(blkno as u32);
    (*fh.add(2)) = cpu_to_le32(generation);
    len = 3;
    if !parent.is_null() {
        blkno = OCFS2_I(parent).ip_blkno;
        generation = (*parent).i_generation;
        *fh.add(3) = cpu_to_le32((blkno >> 32) as u32);
        *fh.add(4) = cpu_to_le32(blkno as u32);
        *fh.add(5) = cpu_to_le32(generation);
        len = 6; file_type = 2;
    }
    *max_len = len;
    trace_ocfs2_encode_fh_type(file_type);
    file_type
}

unsafe fn ocfs2_fh_to_dentry(sb: *mut super_block, fid: *mut fid, fh_len: i32, fh_type: i32) -> *mut dentry {
    if fh_len < 3 || fh_type > 2 { return core::ptr::null_mut(); }
    let raw = (*fid).raw.as_ptr();
    let mut handle = ocfs2_inode_handle { ih_blkno: ((le32_to_cpu(*raw) as u64) << 32) | le32_to_cpu(*raw.add(1)) as u64, ih_generation: le32_to_cpu(*raw.add(2)) };
    ocfs2_get_dentry(sb, &mut handle)
}

unsafe fn ocfs2_fh_to_parent(sb: *mut super_block, fid: *mut fid, fh_len: i32, fh_type: i32) -> *mut dentry {
    if fh_type != 2 || fh_len < 6 { return core::ptr::null_mut(); }
    let raw = (*fid).raw.as_ptr().add(3);
    let mut parent = ocfs2_inode_handle { ih_blkno: ((le32_to_cpu(*raw) as u64) << 32) | le32_to_cpu(*raw.add(1)) as u64, ih_generation: le32_to_cpu(*raw.add(2)) };
    ocfs2_get_dentry(sb, &mut parent)
}

pub static ocfs2_export_ops: export_operations = export_operations {
    encode_fh: Some(ocfs2_encode_fh),
    fh_to_dentry: Some(ocfs2_fh_to_dentry),
    fh_to_parent: Some(ocfs2_fh_to_parent),
    get_parent: Some(ocfs2_get_parent),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
