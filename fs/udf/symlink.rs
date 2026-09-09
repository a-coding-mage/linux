// SPDX-License-Identifier: GPL-2.0-only
/*
 * symlink.c
 *
 * PURPOSE
 *	Symlink handling routines for the OSTA-UDF(tm) filesystem.
 *
 * COPYRIGHT
 *  (C) 1998-2001 Ben Fennema
 *  (C) 1999 Stelias Computing Inc
 *
 * HISTORY
 *
 *  04/16/99 blf  Created.
 *
 */

unsafe fn udf_pc_to_char(
    sb: *mut super_block,
    from: *mut u8,
    fromlen: i32,
    to: *mut u8,
    tolen: i32,
) -> i32 {
    let mut pc: *mut pathComponent;
    let mut elen: i32 = 0;
    let mut comp_len: i32;
    let mut p = to;

    // Reserve one byte for terminating \\0
    let mut tolen = tolen - 1;
    while elen < fromlen {
        if fromlen - elen < core::mem::size_of::<pathComponent>() as i32 {
            return -EIO;
        }
        pc = from.add(elen as usize) as *mut pathComponent;
        elen += core::mem::size_of::<pathComponent>() as i32;
        match (*pc).componentType {
            1 => {
                /*
                 * Symlink points to some place which should be agreed
                 * upon between originator and receiver of the media. Ignore.
                 */
                if (*pc).lengthComponentIdent > 0 {
                    elen += (*pc).lengthComponentIdent as i32;
                    continue;
                }
                // fallthrough
                if tolen == 0 {
                    return -ENAMETOOLONG;
                }
                p = to;
                *p = b'/';
                p = p.add(1);
                tolen -= 1;
            }
            2 => {
                if tolen == 0 {
                    return -ENAMETOOLONG;
                }
                p = to;
                *p = b'/';
                p = p.add(1);
                tolen -= 1;
            }
            3 => {
                if tolen < 3 {
                    return -ENAMETOOLONG;
                }
                core::ptr::copy_nonoverlapping(b"../".as_ptr(), p, 3);
                p = p.add(3);
                tolen -= 3;
            }
            4 => {
                if tolen < 2 {
                    return -ENAMETOOLONG;
                }
                core::ptr::copy_nonoverlapping(b"./".as_ptr(), p, 2);
                p = p.add(2);
                tolen -= 2;
                // that would be . - just ignore
            }
            5 => {
                elen += (*pc).lengthComponentIdent as i32;
                if elen > fromlen {
                    return -EIO;
                }
                comp_len = udf_get_filename(
                    sb,
                    (*pc).componentIdent,
                    (*pc).lengthComponentIdent,
                    p,
                    tolen,
                );
                if comp_len < 0 {
                    return comp_len;
                }
                p = p.add(comp_len as usize);
                tolen -= comp_len;
                if tolen == 0 {
                    return -ENAMETOOLONG;
                }
                *p = b'/';
                p = p.add(1);
                tolen -= 1;
            }
            _ => {}
        }
    }
    if (p as usize) > (to.add(1) as usize) {
        *p.sub(1) = 0;
    } else {
        *p = 0;
    }
    0
}

unsafe fn udf_symlink_filler(file: *mut file, folio: *mut folio) -> i32 {
    let inode = (*(*folio).mapping).host;
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let symlink: *mut u8;
    let mut err: i32 = 0;
    let p = folio_address(folio);
    let iinfo = UDF_I(inode);

    // We don't support symlinks longer than one block
    if (*inode).i_size > (*(*inode).i_sb).s_blocksize {
        err = -ENAMETOOLONG;
        folio_end_read(folio, false);
        return err;
    }

    if (*iinfo).i_alloc_type == ICBTAG_FLAG_AD_IN_ICB {
        symlink = (*iinfo).i_data.add((*iinfo).i_lenEAttr as usize);
    } else {
        bh = udf_bread(inode, 0, 0, &mut err);
        if bh.is_null() {
            if err == 0 {
                err = -EFSCORRUPTED;
            }
            folio_end_read(folio, false);
            return err;
        }
        symlink = (*bh).b_data;
    }

    err = udf_pc_to_char((*inode).i_sb, symlink, (*inode).i_size as i32, p, PAGE_SIZE as i32);
    brelse(bh);
    folio_end_read(folio, err == 0);
    err
}

unsafe fn udf_symlink_getattr(
    idmap: *mut mnt_idmap,
    path: *const path,
    stat: *mut kstat,
    request_mask: u32,
    flags: u32,
) -> i32 {
    let dentry = (*path).dentry;
    let inode = d_backing_inode(dentry);
    let folio;

    generic_fillattr(&nop_mnt_idmap, request_mask, inode, stat);
    folio = read_mapping_folio((*inode).i_mapping, 0, core::ptr::null_mut());
    if IS_ERR(folio) {
        return PTR_ERR(folio);
    }
    /*
     * UDF uses non-trivial encoding of symlinks so i_size does not match
     * number of characters reported by readlink(2) which apparently some
     * applications expect. Also POSIX says that "The value returned in the
     * st_size field shall be the length of the contents of the symbolic
     * link, and shall not count a trailing null if one is present." So
     * let's report the length of string returned by readlink(2) for
     * st_size.
     */
    (*stat).size = strlen(folio_address(folio)) as i64;
    folio_put(folio);

    0
}

/*
 * symlinks can't do much...
 */
#[no_mangle]
pub static udf_symlink_aops: address_space_operations = address_space_operations {
    read_folio: Some(udf_symlink_filler),
};

#[no_mangle]
pub static udf_symlink_inode_operations: inode_operations = inode_operations {
    get_link: Some(page_get_link),
    getattr: Some(udf_symlink_getattr),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
