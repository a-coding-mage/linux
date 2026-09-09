// SPDX-License-Identifier: GPL-2.0-only
/*
 * ialloc.c
 *
 * PURPOSE
 *	Inode allocation handling routines for the OSTA-UDF(tm) filesystem.
 *
 * COPYRIGHT
 *  (C) 1998-2001 Ben Fennema
 *
 * HISTORY
 *
 *  02/24/99 blf  Created.
 *
 */

// Dependencies supplied by the surrounding UDF and kernel bindings.

pub unsafe fn udf_free_inode(inode: *mut inode) {
    udf_free_blocks(
        (*inode).i_sb,
        core::ptr::null_mut(),
        &mut UDF_I(inode).i_location,
        0,
        1,
    );
}

pub unsafe fn udf_new_inode(dir: *mut inode, mode: umode_t) -> *mut inode {
    let sb: *mut super_block = (*dir).i_sb;
    let sbi: *mut udf_sb_info = UDF_SB(sb);
    let mut inode: *mut inode;
    let mut block: udf_pblk_t;
    let start: u32 = UDF_I(dir).i_location.logicalBlockNum;
    let iinfo: *mut udf_inode_info;
    let dinfo: *mut udf_inode_info = UDF_I(dir);
    let mut err: i32;

    inode = new_inode(sb);

    if inode.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    iinfo = UDF_I(inode);
    if UDF_QUERY_FLAG((*inode).i_sb, UDF_FLAG_USE_EXTENDED_FE) {
        (*iinfo).i_efe = 1;
        if UDF_VERS_USE_EXTENDED_FE > (*sbi).s_udfrev {
            (*sbi).s_udfrev = UDF_VERS_USE_EXTENDED_FE;
        }
        (*iinfo).i_data = kzalloc(
            (*inode).i_sb.s_blocksize - core::mem::size_of::<extendedFileEntry>(),
            GFP_KERNEL,
        );
    } else {
        (*iinfo).i_efe = 0;
        (*iinfo).i_data = kzalloc(
            (*inode).i_sb.s_blocksize - core::mem::size_of::<fileEntry>(),
            GFP_KERNEL,
        );
    }
    if (*iinfo).i_data.is_null() {
        make_bad_inode(inode);
        iput(inode);
        return ERR_PTR(-ENOMEM);
    }

    err = -ENOSPC;
    block = udf_new_block(
        (*dir).i_sb,
        core::ptr::null_mut(),
        (*dinfo).i_location.partitionReferenceNum,
        start,
        &mut err,
    );
    if err != 0 {
        make_bad_inode(inode);
        iput(inode);
        return ERR_PTR(err);
    }

    (*iinfo).i_unique = lvid_get_unique_id(sb);
    (*inode).i_generation = (*iinfo).i_unique;

    inode_init_owner(&nop_mnt_idmap, inode, dir, mode);
    if UDF_QUERY_FLAG(sb, UDF_FLAG_UID_SET) {
        (*inode).i_uid = (*sbi).s_uid;
    }
    if UDF_QUERY_FLAG(sb, UDF_FLAG_GID_SET) {
        (*inode).i_gid = (*sbi).s_gid;
    }

    (*iinfo).i_location.logicalBlockNum = block;
    (*iinfo).i_location.partitionReferenceNum =
        (*dinfo).i_location.partitionReferenceNum;
    (*inode).i_ino = udf_get_lb_pblock(sb, &(*iinfo).i_location, 0);
    (*inode).i_blocks = 0;
    (*iinfo).i_lenEAttr = 0;
    (*iinfo).i_lenAlloc = 0;
    (*iinfo).i_use = 0;
    (*iinfo).i_checkpoint = 1;
    (*iinfo).i_extraPerms = FE_PERM_U_CHATTR;
    udf_update_extra_perms(inode, mode);

    if UDF_QUERY_FLAG((*inode).i_sb, UDF_FLAG_USE_AD_IN_ICB) {
        (*iinfo).i_alloc_type = ICBTAG_FLAG_AD_IN_ICB;
    } else if UDF_QUERY_FLAG((*inode).i_sb, UDF_FLAG_USE_SHORT_AD) {
        (*iinfo).i_alloc_type = ICBTAG_FLAG_AD_SHORT;
    } else {
        (*iinfo).i_alloc_type = ICBTAG_FLAG_AD_LONG;
    }
    simple_inode_init_ts(inode);
    (*iinfo).i_crtime = inode_get_mtime(inode);
    if unlikely(insert_inode_locked(inode) < 0) {
        make_bad_inode(inode);
        iput(inode);
        return ERR_PTR(-EIO);
    }
    mark_inode_dirty(inode);

    inode
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
