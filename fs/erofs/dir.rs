// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017-2018 HUAWEI, Inc.
 *             https://www.huawei.com/
 * Copyright (C) 2022, Alibaba Cloud
 */
// Dependencies supplied by internal headers and the Linux kernel are expected
// to be available from the surrounding translation unit.

unsafe fn erofs_fill_dentries(
    dir: *mut inode,
    ctx: *mut dir_context,
    dentry_blk: *mut core::ffi::c_void,
    mut de: *mut erofs_dirent,
    nameoff0: u32,
    maxsize: u32,
) -> i32 {
    let end = (dentry_blk as *mut erofs_dirent).byte_add(nameoff0 as usize);

    while de < end {
        let d_type = fs_ftype_to_dtype((*de).file_type);
        let nameoff = le16_to_cpu((*de).nameoff) as u32;
        let de_name = (dentry_blk as *mut u8).byte_add(nameoff as usize) as *const i8;
        let de_namelen: u32;

        /* non-trailing dirent in the directory block? */
        if de.add(1) < end {
            de_namelen = le16_to_cpu((*de.add(1)).nameoff) as u32 - nameoff;
        } else if maxsize <= nameoff {
            goto_err_bogus(dir);
            return -EFSCORRUPTED;
        } else {
            de_namelen = strnlen(de_name, (maxsize - nameoff) as usize) as u32;
        }

        /* a corrupted entry is found (including negative namelen) */
        if !in_range32(de_namelen, 1, EROFS_NAME_LEN)
            || nameoff + de_namelen > maxsize
        {
            goto_err_bogus(dir);
            return -EFSCORRUPTED;
        }

        if !dir_emit(
            ctx,
            de_name,
            de_namelen as usize,
            erofs_nid_to_ino64(EROFS_SB((*dir).i_sb), le64_to_cpu((*de).nid)),
            d_type,
        ) {
            return 1;
        }
        de = de.add(1);
        (*ctx).pos += core::mem::size_of::<erofs_dirent>() as u64;
    }
    return 0;
}

unsafe fn goto_err_bogus(dir: *mut inode) {
    erofs_err((*dir).i_sb, "bogus dirent @ nid %llu", (*EROFS_I(dir)).nid);
    DBG_BUGON(1);
}

unsafe fn erofs_readdir(f: *mut file, ctx: *mut dir_context) -> i32 {
    let dir = file_inode(f);
    let mut buf: erofs_buf = __EROFS_BUF_INITIALIZER;
    let sb = (*dir).i_sb;
    let ra = &mut (*f).f_ra;
    let bsz = (*sb).s_blocksize as usize;
    let mut ofs = erofs_blkoff(sb, (*ctx).pos);
    let ra_pages = DIV_ROUND_UP_POW2((*EROFS_I_SB(dir)).dir_ra_bytes, PAGE_SIZE);
    let nr_pages = DIV_ROUND_UP_POW2((*dir).i_size, PAGE_SIZE);
    let mut err: i32 = 0;
    let mut initial = true;

    buf.mapping = (*dir).i_mapping;
    while (*ctx).pos < (*dir).i_size {
        let dbstart = (*ctx).pos - ofs;
        let de: *mut erofs_dirent;
        let nameoff: u32;
        let maxsize: u32;

        if fatal_signal_pending(current) {
            err = -ERESTARTSYS;
            break;
        }

        /* readahead blocks to enhance performance for large directories */
        if ra_pages != 0 {
            let idx = DIV_ROUND_UP_POW2((*ctx).pos, PAGE_SIZE);
            let pages = core::cmp::min(nr_pages - idx, ra_pages);
            if pages > 1 && !ra_has_index(ra, idx) {
                page_cache_sync_readahead((*dir).i_mapping, ra, f, idx, pages);
            }
        }

        de = erofs_bread(&mut buf, dbstart, true);
        if IS_ERR(de) {
            erofs_err(sb, "failed to readdir of logical block %llu of nid %llu",
                erofs_blknr(sb, dbstart), (*EROFS_I(dir)).nid);
            err = PTR_ERR(de);
            break;
        }

        nameoff = le16_to_cpu((*de).nameoff) as u32;
        if nameoff == 0 || nameoff >= bsz as u32
            || nameoff % core::mem::size_of::<erofs_dirent>() as u32 != 0
        {
            erofs_err(sb, "invalid de[0].nameoff %u @ nid %llu",
                nameoff, (*EROFS_I(dir)).nid);
            err = -EFSCORRUPTED;
            break;
        }

        maxsize = core::cmp::min(((*dir).i_size - dbstart) as u32, bsz as u32);
        /* search dirents at the arbitrary position */
        if initial {
            initial = false;
            ofs = roundup(ofs, core::mem::size_of::<erofs_dirent>() as u64);
            (*ctx).pos = dbstart + ofs;
        }

        err = erofs_fill_dentries(dir, ctx, de as *mut core::ffi::c_void,
            de.byte_add(ofs as usize), nameoff, maxsize);
        if err != 0 { break; }
        (*ctx).pos = dbstart + maxsize as u64;
        ofs = 0;
        cond_resched();
    }
    erofs_put_metabuf(&mut buf);
    if (*EROFS_I(dir)).dot_omitted && (*ctx).pos == (*dir).i_size {
        if !dir_emit_dot(f, ctx) { return 0; }
        (*ctx).pos += 1;
    }
    if err < 0 { err } else { 0 }
}

pub static erofs_dir_fops: file_operations = file_operations {
    llseek: Some(generic_file_llseek),
    read: Some(generic_read_dir),
    iterate_shared: Some(erofs_readdir),
    unlocked_ioctl: Some(erofs_ioctl),
    // CONFIG_COMPAT conditionally provides compat_ioctl: Some(erofs_compat_ioctl),
    setlease: Some(generic_setlease),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
