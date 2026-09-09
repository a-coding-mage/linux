// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/isofs/namei.c
 *
 *  (C) 1992  Eric Youngdale Modified for ISO 9660 filesystem.
 *
 *  (C) 1991  Linus Torvalds - minix filesystem
 */

// C dependencies supplied by the surrounding kernel/isofs translation.

pub unsafe fn isofs_dir_record_valid(
    de: *mut iso_directory_record,
    offset: c_ulong,
    bufsize: c_ulong,
) -> bool {
    let min_len: c_ulong = core::mem::offset_of!(iso_directory_record, name) as c_ulong;

    if offset > bufsize || bufsize - offset < min_len {
        return false;
    }

    let len: c_uint = isonum_711((*de).length) as c_uint;
    let name_len: c_uint = isonum_711((*de).name_len) as c_uint;
    if (len as c_ulong) < min_len || name_len > (len as c_ulong - min_len) as c_uint {
        return false;
    }
    if (len as c_ulong) > bufsize - offset {
        return false;
    }
    true
}

unsafe fn isofs_cmp(dentry: *mut dentry, compare: *const c_char, dlen: c_int) -> c_int {
    if likely(!(*dentry).d_op.is_null()) {
        return if (*dentry).d_name.len != dlen
            || memcmp((*dentry).d_name.name, compare, dlen as usize) != 0
        {
            1
        } else {
            0
        };
    }
    (*(*dentry).d_op).d_compare(
        core::ptr::null_mut(),
        (*dentry).d_name.len,
        (*dentry).d_name.name,
        &QSTR_LEN(compare, dlen),
    )
}

/*
 *	isofs_find_entry()
 *
 * finds an entry in the specified directory with the wanted name. It
 * returns the inode number of the found entry, or 0 on error.
 */
unsafe fn isofs_find_entry(
    dir: *mut inode,
    dentry: *mut dentry,
    block_rv: *mut c_ulong,
    offset_rv: *mut c_ulong,
    tmpname: *mut c_char,
) -> c_ulong {
    let bufsize = ISOFS_BUFFER_SIZE(dir);
    let bufbits = ISOFS_BUFFER_BITS(dir);
    let mut block: c_ulong = 0;
    let mut f_pos: c_ulong = 0;
    let mut offset: c_ulong = 0;
    let mut block_saved: c_ulong;
    let mut offset_saved: c_ulong;
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let sbi = ISOFS_SB((*dir).i_sb);

    if !ISOFS_I(dir).i_first_extent {
        return 0;
    }

    while f_pos < (*dir).i_size {
        let de: *mut iso_directory_record;
        let mut de_len: c_int;
        let mut match_: c_int;
        let mut i: c_int;
        let mut dlen: c_int;
        let mut dpnt: *mut c_char;

        if bh.is_null() {
            bh = isofs_bread(dir, block);
            if bh.is_null() {
                return 0;
            }
        }

        de = ((*bh).b_data.add(offset as usize)) as *mut iso_directory_record;
        de_len = *(de as *const u8) as c_int;
        if de_len == 0 {
            brelse(bh);
            bh = core::ptr::null_mut();
            f_pos = (f_pos + ISOFS_BLOCK_SIZE) & !(ISOFS_BLOCK_SIZE - 1);
            block = f_pos >> bufbits;
            offset = 0;
            continue;
        }

        block_saved = (*bh).b_blocknr;
        offset_saved = offset;
        offset += de_len as c_ulong;
        f_pos += de_len as c_ulong;

        if !isofs_dir_record_valid(de, offset_saved, bufsize) {
            printk(
                KERN_NOTICE,
                b"iso9660: Corrupted directory entry in block %lu of inode %llu\n\0",
                block,
                (*dir).i_ino,
            );
            brelse(bh);
            return 0;
        }
        dlen = (*de).name_len[0] as c_int;
        dpnt = (*de).name.as_mut_ptr() as *mut c_char;

        if (*sbi).s_rock && {
            i = get_rock_ridge_filename(de, tmpname, dir);
            i != 0
        } {
            dlen = i;
            dpnt = tmpname;
        } else if (*sbi).s_joliet_level != 0 {
            dlen = get_joliet_filename(de, tmpname, dir);
            dpnt = tmpname;
        } else if (*sbi).s_mapping == b'a' as c_char {
            dlen = get_acorn_filename(de, tmpname, dir);
            dpnt = tmpname;
        } else if (*sbi).s_mapping == b'n' as c_char {
            dlen = isofs_name_translate(de, tmpname, dir);
            dpnt = tmpname;
        }

        /*
         * Skip hidden or associated files unless hide or showassoc,
         * respectively, is set
         */
        match_ = 0;
        if dlen > 0
            && (!(*sbi).s_hide || !((*de).flags[-(*sbi).s_high_sierra as isize] & 1 != 0))
            && ((*sbi).s_showassoc || !((*de).flags[-(*sbi).s_high_sierra as isize] & 4 != 0))
        {
            if !dpnt.is_null() && (dlen > 1 || *dpnt as u8 > 1) {
                match_ = (isofs_cmp(dentry, dpnt, dlen) == 0) as c_int;
            }
        }
        if match_ != 0 {
            isofs_normalize_block_and_offset(de, &mut block_saved, &mut offset_saved);
            *block_rv = block_saved;
            *offset_rv = offset_saved;
            brelse(bh);
            return 1;
        }
    }
    brelse(bh);
    0
}

pub unsafe fn isofs_lookup(
    dir: *mut inode,
    dentry: *mut dentry,
    _flags: c_uint,
) -> *mut dentry {
    let mut block: c_ulong = 0;
    let mut offset: c_ulong = 0;
    let tmpname = kmalloc(1024, GFP_USER) as *mut c_char;
    if tmpname.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    let found = isofs_find_entry(dir, dentry, &mut block, &mut offset, tmpname);
    kfree(tmpname as *mut c_void);
    let inode = if found != 0 {
        isofs_iget((*dir).i_sb, block, offset)
    } else {
        core::ptr::null_mut()
    };
    d_splice_alias(inode, dentry)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
