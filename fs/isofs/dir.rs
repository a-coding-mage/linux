// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/isofs/dir.c
 *
 *  (C) 1992, 1993, 1994  Eric Youngdale Modified for ISO 9660 filesystem.
 *
 *  (C) 1991  Linus Torvalds - minix filesystem
 *
 *  Steve Beynon		       : Missing last directory entries fixed
 *  (stephen@askone.demon.co.uk)      : 21st June 1996
 *
 *  isofs directory handling functions
 */

pub unsafe fn isofs_name_translate(
    de: *mut iso_directory_record,
    new: *mut libc::c_char,
    _inode: *mut inode,
) -> libc::c_int {
    let old = (*de).name.as_mut_ptr();
    let len = (*de).name_len[0] as libc::c_int;
    let mut i = 0;

    while i < len {
        let mut c = *old.add(i as usize) as libc::c_uchar;
        if c == 0 { break; }
        if c >= b'A' && c <= b'Z' { c |= 0x20; }
        /* Drop trailing '.;1' (ISO 9660:1988 7.5.1 requires period) */
        if c == b'.' && i == len - 3 && *old.add((i + 1) as usize) == b';' as libc::c_char && *old.add((i + 2) as usize) == b'1' as libc::c_char { break; }
        /* Drop trailing ';1' */
        if c == b';' && i == len - 2 && *old.add((i + 1) as usize) == b'1' as libc::c_char { break; }
        /* Convert remaining ';' to '.' */
        /* Also '/' to '.' (broken Acorn-generated ISO9660 images) */
        if c == b';' || c == b'/' { c = b'.'; }
        *new.add(i as usize) = c as libc::c_char;
        i += 1;
    }
    i
}

/* Acorn extensions written by Matthew Wilcox <willy@infradead.org> 1998 */
pub unsafe fn get_acorn_filename(de: *mut iso_directory_record, retname: *mut libc::c_char, inode: *mut inode) -> libc::c_int {
    let retnamlen = isofs_name_translate(de, retname, inode);
    if retnamlen == 0 { return 0; }
    let mut std = core::mem::size_of::<iso_directory_record>() as libc::c_int + (*de).name_len[0] as libc::c_int;
    if std & 1 != 0 { std += 1; }
    if (*de).length[0] as libc::c_int - std != 32 { return retnamlen; }
    let chr = (de as *mut libc::c_uchar).add(std as usize);
    if libc::strncmp(chr as *const libc::c_char, b"ARCHIMEDES\0".as_ptr() as *const libc::c_char, 10) != 0 { return retnamlen; }
    if *retname == b'_' as libc::c_char && (*chr.add(19) & 1) == 1 { *retname = b'!' as libc::c_char; }
    if ((*de).flags[0] & 2) == 0 && *chr.add(13) == 0xff && (*chr.add(12) & 0xf0) == 0xf0 {
        *retname.add(retnamlen as usize) = b',' as libc::c_char;
        libc::sprintf(retname.add((retnamlen + 1) as usize), b"%3.3x\0".as_ptr() as *const libc::c_char, (((*chr.add(12) & 0xf) as libc::c_int) << 8) | *chr.add(11) as libc::c_int);
        return retnamlen + 4;
    }
    retnamlen
}

/* This should _really_ be cleaned up some day.. */
unsafe fn do_isofs_readdir(inode: *mut inode, file: *mut file, ctx: *mut dir_context, tmpname: *mut libc::c_char) -> libc::c_int {
    let bufsize = ISOFS_BUFFER_SIZE(inode);
    let bufbits = ISOFS_BUFFER_BITS(inode);
    let mut block = (*ctx).pos >> bufbits;
    let mut offset = (*ctx).pos & (bufsize - 1);
    let mut inode_number: libc::c_ulong = 0;
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let mut first_de = 1;
    let mut p: *mut libc::c_char = core::ptr::null_mut();
    let sbi = ISOFS_SB((*inode).i_sb);

    while (*ctx).pos < (*inode).i_size {
        if bh.is_null() { bh = isofs_bread(inode, block); if bh.is_null() { return 0; } }
        let de = ((*bh).b_data.add(offset as usize)) as *mut iso_directory_record;
        let de_len = *(de as *mut libc::c_uchar) as libc::c_ulong;
        if de_len == 0 {
            brelse(bh); bh = core::ptr::null_mut();
            (*ctx).pos = ((*ctx).pos + ISOFS_BLOCK_SIZE) & !(ISOFS_BLOCK_SIZE - 1);
            block = (*ctx).pos >> bufbits; offset = 0; continue;
        }
        let block_saved = block; let offset_saved = offset; offset += de_len;
        if !isofs_dir_record_valid(de, offset_saved, bufsize) { brelse(bh); return -EIO; }
        let mut inode_number_local = inode_number;
        if first_de { let mut bs = block_saved; let mut os = offset_saved; isofs_normalize_block_and_offset(de, &mut bs, &mut os); inode_number_local = isofs_get_ino(bs, os, bufbits); inode_number = inode_number_local; }
        if (*de).flags[-(*sbi).s_high_sierra as isize as usize] & 0x80 != 0 { first_de = 0; (*ctx).pos += de_len; continue; }
        first_de = 1;
        if (*de).name_len[0] == 1 && (*de).name[0] == 0 { if !dir_emit_dot(file, ctx) { break; } (*ctx).pos += de_len; continue; }
        if (*de).name_len[0] == 1 && (*de).name[0] == 1 { if !dir_emit_dotdot(file, ctx) { break; } (*ctx).pos += de_len; continue; }
        if ((*sbi).s_hide && ((*de).flags[-(*sbi).s_high_sierra as isize as usize] & 1) != 0) || (!(*sbi).s_showassoc && ((*de).flags[-(*sbi).s_high_sierra as isize as usize] & 4) != 0) { (*ctx).pos += de_len; continue; }
        let mut len = 0; let mut map = 1;
        if (*sbi).s_rock { len = get_rock_ridge_filename(de, tmpname, inode); if len != 0 { p = tmpname; map = 0; } }
        if map { if (*sbi).s_joliet_level { len = get_joliet_filename(de, tmpname, inode); p = tmpname; } else if (*sbi).s_mapping == b'a' as _ { len = get_acorn_filename(de, tmpname, inode); p = tmpname; } else if (*sbi).s_mapping == b'n' as _ { len = isofs_name_translate(de, tmpname, inode); p = tmpname; } else { p = (*de).name.as_mut_ptr(); len = (*de).name_len[0] as _; } }
        if len > 0 && !dir_emit(ctx, p, len, inode_number_local, DT_UNKNOWN) { break; }
        (*ctx).pos += de_len;
    }
    if !bh.is_null() { brelse(bh); } 0
}

unsafe fn isofs_readdir(file: *mut file, ctx: *mut dir_context) -> libc::c_int {
    let inode = file_inode(file); let tmpname = kmalloc(1024, GFP_KERNEL) as *mut libc::c_char;
    if tmpname.is_null() { return -ENOMEM; }
    let result = do_isofs_readdir(inode, file, ctx, tmpname); kfree(tmpname as *mut libc::c_void); result
}

pub unsafe fn isofs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> libc::c_int {
    let sbi = ISOFS_SB((*dentry).d_sb);
    if (*sbi).s_check == b'r' as _ { (*fa).fsx_xflags |= FS_XFLAG_CASEFOLD; (*fa).flags |= FS_CASEFOLD_FL; }
    if !(*sbi).s_joliet_level && !(*sbi).s_rock && ((*sbi).s_mapping == b'n' as _ || (*sbi).s_mapping == b'a' as _) { (*fa).fsx_xflags |= FS_XFLAG_CASENONPRESERVING; } 0
}

pub static isofs_dir_operations: file_operations = file_operations { llseek: Some(generic_file_llseek), read: Some(generic_read_dir), iterate_shared: Some(isofs_readdir), setlease: Some(generic_setlease) };
pub static isofs_dir_inode_operations: inode_operations = inode_operations { lookup: Some(isofs_lookup), fileattr_get: Some(isofs_fileattr_get) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
