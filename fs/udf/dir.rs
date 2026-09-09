// SPDX-License-Identifier: GPL-2.0-only
/*
 * dir.c
 *
 * PURPOSE
 *  Directory handling routines for the OSTA-UDF(tm) filesystem.
 *
 * COPYRIGHT
 *  (C) 1998-2004 Ben Fennema
 *
 * HISTORY
 *
 *  10/05/98 dgb  Split directory operations into its own file
 *                Implemented directory reads via do_udf_readdir
 *  10/06/98      Made directory operations work!
 *  11/17/98      Rewrote directory to support ICBTAG_FLAG_AD_LONG
 *  11/25/98 blf  Rewrote directory handling (readdir+lookup) to support reading
 *                across blocks.
 *  12/12/98      Split out the lookup code to namei.c. bulk of directory
 *                code now in directory.c:udf_fileident_read.
 */

// Declarations below are supplied by the surrounding UDF/kernel translation.

unsafe fn udf_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    let dir = unsafe { file_inode(file) };
    let mut nf_pos: loff_t;
    let mut emit_pos: loff_t = 0;
    let mut flen: c_int;
    let mut fname: *mut u8 = core::ptr::null_mut();
    let mut ret: c_int = 0;
    let sb = unsafe { (*dir).i_sb };
    let mut pos_valid = false;
    let mut iter: udf_fileident_iter = unsafe { core::mem::zeroed() };

    if unsafe { (*ctx).pos == 0 } {
        if unsafe { !dir_emit_dot(file, ctx) } {
            return 0;
        }
        unsafe { (*ctx).pos = 1 };
    }
    nf_pos = (unsafe { (*ctx).pos } - 1) << 2;
    if nf_pos >= unsafe { (*dir).i_size } {
        return ret;
    }

    /*
     * Something changed since last readdir (either lseek was called or dir
     * changed)?  We need to verify the position correctly points at the
     * beginning of some dir entry so that the directory parsing code does
     * not get confused. Since UDF does not have any reliable way of
     * identifying beginning of dir entry (names are under user control),
     * we need to scan the directory from the beginning.
     */
    if unsafe { !inode_eq_iversion(dir, *(file->private_data as *const u64)) } {
        emit_pos = nf_pos;
        nf_pos = 0;
    } else {
        pos_valid = true;
    }

    fname = unsafe { kmalloc(UDF_NAME_LEN, GFP_KERNEL) } as *mut u8;
    if fname.is_null() {
        ret = -ENOMEM;
        return ret;
    }

    ret = unsafe { udf_fiiter_init(&mut iter, dir, nf_pos) };
    while ret == 0 && unsafe { iter.pos < (*dir).i_size } {
        let mut tloc: kernel_lb_addr;
        let iblock: udf_pblk_t;

        if unsafe { iter.pos < emit_pos } {
            ret = unsafe { udf_fiiter_advance(&mut iter) };
            continue;
        }

        pos_valid = true;
        unsafe { (*ctx).pos = (iter.pos >> 2) + 1 };

        if unsafe { iter.fi.fileCharacteristics & FID_FILE_CHAR_DELETED } != 0
            && unsafe { !UDF_QUERY_FLAG(sb, UDF_FLAG_UNDELETE) }
        {
            ret = unsafe { udf_fiiter_advance(&mut iter) };
            continue;
        }

        if unsafe { iter.fi.fileCharacteristics & FID_FILE_CHAR_HIDDEN } != 0
            && unsafe { !UDF_QUERY_FLAG(sb, UDF_FLAG_UNHIDE) }
        {
            ret = unsafe { udf_fiiter_advance(&mut iter) };
            continue;
        }

        if unsafe { iter.fi.fileCharacteristics & FID_FILE_CHAR_PARENT } != 0 {
            if unsafe { !dir_emit_dotdot(file, ctx) } {
                break;
            }
            ret = unsafe { udf_fiiter_advance(&mut iter) };
            continue;
        }

        flen = unsafe {
            udf_get_filename(sb, iter.name, iter.fi.lengthFileIdent, fname, UDF_NAME_LEN)
        };
        if flen < 0 {
            ret = unsafe { udf_fiiter_advance(&mut iter) };
            continue;
        }

        tloc = unsafe { lelb_to_cpu(iter.fi.icb.extLocation) };
        iblock = unsafe { udf_get_lb_pblock(sb, &mut tloc, 0) };
        if unsafe { !dir_emit(ctx, fname, flen, iblock, DT_UNKNOWN) } {
            break;
        }
        ret = unsafe { udf_fiiter_advance(&mut iter) };
    }

    if ret == 0 {
        unsafe { (*ctx).pos = (iter.pos >> 2) + 1 };
        pos_valid = true;
    }
    unsafe { udf_fiiter_release(&mut iter) };
    if pos_valid {
        unsafe { *(file->private_data as *mut u64) = inode_query_iversion(dir) };
    }
    unsafe { kfree(fname as *mut core::ffi::c_void) };
    ret
}

unsafe fn udf_dir_open(_inode: *mut inode, file: *mut file) -> c_int {
    (*file).private_data = kzalloc(core::mem::size_of::<u64>(), GFP_KERNEL);
    if (*file).private_data.is_null() {
        return -ENOMEM;
    }
    0
}

unsafe fn udf_dir_release(_inode: *mut inode, file: *mut file) -> c_int {
    kfree((*file).private_data);
    0
}

unsafe fn udf_dir_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t {
    generic_llseek_cookie(file, offset, whence, (*file).private_data as *mut u64)
}

/* readdir and lookup functions */
#[no_mangle]
pub static mut udf_dir_operations: file_operations = file_operations {
    open: Some(udf_dir_open),
    release: Some(udf_dir_release),
    llseek: Some(udf_dir_llseek),
    read: Some(generic_read_dir),
    iterate_shared: Some(udf_readdir),
    unlocked_ioctl: Some(udf_ioctl),
    fsync: Some(simple_fsync),
    setlease: Some(generic_setlease),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
