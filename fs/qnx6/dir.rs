// SPDX-License-Identifier: GPL-2.0
/*
 * QNX6 file system, Linux implementation.
 *
 * Version : 1.0.0
 *
 * History :
 *
 * 01-02-2012 by Kai Bankett (chaosman@ontika.net) : first release.
 * 16-02-2012 pagemap extension by Al Viro
 */

// Dependencies supplied by the Linux kernel and the QNX6 implementation.

unsafe fn qnx6_lfile_checksum(mut name: *mut ::core::ffi::c_char, size: u32) -> u32 {
    let mut crc: u32 = 0;
    let end = name.wrapping_add(size as usize);
    while name < end {
        crc = ((crc >> 1).wrapping_add(*name as u8 as u32))
            ^ if crc & 0x00000001 != 0 { 0x80000000 } else { 0 };
        name = name.add(1);
    }
    crc
}

unsafe fn qnx6_get_folio(
    dir: *mut inode,
    n: ::core::ffi::c_ulong,
    foliop: *mut *mut folio,
) -> *mut ::core::ffi::c_void {
    let folio = read_mapping_folio((*dir).i_mapping, n, core::ptr::null_mut());
    if is_err(folio as *mut ::core::ffi::c_void) {
        return folio as *mut ::core::ffi::c_void;
    }
    *foliop = folio;
    kmap_local_folio(folio, 0)
}

unsafe fn last_entry(inode: *mut inode, page_nr: ::core::ffi::c_ulong) -> u32 {
    let mut last_byte = (*inode).i_size as ::core::ffi::c_ulong;
    last_byte = last_byte.wrapping_sub(page_nr << PAGE_SHIFT);
    if last_byte > PAGE_SIZE {
        last_byte = PAGE_SIZE;
    }
    (last_byte / QNX6_DIR_ENTRY_SIZE as ::core::ffi::c_ulong) as u32
}

unsafe fn qnx6_longname(
    sb: *mut super_block,
    de: *mut qnx6_long_dir_entry,
    foliop: *mut *mut folio,
) -> *mut qnx6_long_filename {
    let sbi = QNX6_SB(sb);
    let s = fs32_to_cpu(sbi, (*de).de_long_inode);
    let n = s >> (PAGE_SHIFT - (*sb).s_blocksize_bits);
    let mapping = (*sbi).longfile.as_ref().unwrap().i_mapping;
    let folio = read_mapping_folio(mapping, n as ::core::ffi::c_ulong, core::ptr::null_mut());
    if is_err(folio as *mut ::core::ffi::c_void) {
        return folio as *mut qnx6_long_filename;
    }
    let offs = offset_in_folio(folio, s << (*sb).s_blocksize_bits);
    *foliop = folio;
    kmap_local_folio(folio, offs) as *mut qnx6_long_filename
}

unsafe fn qnx6_dir_longfilename(
    inode: *mut inode,
    de: *mut qnx6_long_dir_entry,
    ctx: *mut dir_context,
    de_inode: u32,
) -> i32 {
    let s = (*inode).i_sb;
    let sbi = QNX6_SB(s);
    let mut folio: *mut folio = core::ptr::null_mut();
    if (*de).de_size != 0xff {
        pr_err!("invalid direntry size ({}).\n", (*de).de_size);
        return 0;
    }
    let lf = qnx6_longname(s, de, &mut folio);
    if is_err(lf as *mut ::core::ffi::c_void) {
        pr_err!("Error reading longname\n");
        return 0;
    }
    let lf_size = fs16_to_cpu(sbi, (*lf).lf_size);
    if lf_size > QNX6_LONG_NAME_MAX {
        pr_debug!("file {}\n", (*lf).lf_fname);
        pr_err!("Filename too long ({})\n", lf_size);
        folio_release_kmap(folio, lf as *mut ::core::ffi::c_void);
        return 0;
    }
    if !test_opt(s, MMI_FS) && fs32_to_cpu(sbi, (*de).de_checksum)
        != qnx6_lfile_checksum((*lf).lf_fname, lf_size as u32)
    {
        pr_info!("long filename checksum error.\n");
    }
    pr_debug!("qnx6_readdir:{} inode:{}\n", (*lf).lf_fname, de_inode);
    if !dir_emit(ctx, (*lf).lf_fname, lf_size, de_inode, DT_UNKNOWN) {
        folio_release_kmap(folio, lf as *mut ::core::ffi::c_void);
        return 0;
    }
    folio_release_kmap(folio, lf as *mut ::core::ffi::c_void);
    1
}

unsafe fn qnx6_readdir(file: *mut file, ctx: *mut dir_context) -> i32 {
    let inode = file_inode(file);
    let s = (*inode).i_sb;
    let sbi = QNX6_SB(s);
    let pos = (*ctx).pos & !(QNX6_DIR_ENTRY_SIZE as i64 - 1);
    let npages = dir_pages(inode);
    let mut n = (pos >> PAGE_SHIFT) as ::core::ffi::c_ulong;
    let mut offset = ((pos & !(PAGE_MASK as i64)) as u32 / QNX6_DIR_ENTRY_SIZE) as usize;
    let mut done = false;
    (*ctx).pos = pos;
    if (*ctx).pos >= (*inode).i_size { return 0; }
    while !done && n < npages {
        let mut folio: *mut folio = core::ptr::null_mut();
        let kaddr = qnx6_get_folio(inode, n, &mut folio);
        if is_err(kaddr) {
            pr_err!("qnx6_readdir(): read failed\n");
            (*ctx).pos = ((n + 1) << PAGE_SHIFT) as i64;
            return ptr_err(kaddr);
        }
        let mut de = (kaddr as *mut qnx6_dir_entry).add(offset);
        let limit = (kaddr as *mut qnx6_dir_entry).add(last_entry(inode, n) as usize);
        while de < limit {
            let size = (*de).de_size;
            let no_inode = fs32_to_cpu(sbi, (*de).de_inode);
            if no_inode != 0 && size != 0 {
                if size > QNX6_SHORT_NAME_MAX {
                    if qnx6_dir_longfilename(inode, de as *mut qnx6_long_dir_entry, ctx, no_inode) == 0 { done = true; break; }
                } else if !dir_emit(ctx, (*de).de_fname, size, no_inode, DT_UNKNOWN) { done = true; break; }
            }
            de = de.add(1);
            (*ctx).pos += QNX6_DIR_ENTRY_SIZE as i64;
        }
        folio_release_kmap(folio, kaddr);
        n += 1;
        offset = 0;
    }
    0
}

unsafe fn qnx6_long_match(len: i32, name: *const ::core::ffi::c_char, de: *mut qnx6_long_dir_entry, dir: *mut inode) -> u32 {
    let s = (*dir).i_sb;
    let sbi = QNX6_SB(s);
    let mut folio: *mut folio = core::ptr::null_mut();
    let lf = qnx6_longname(s, de, &mut folio);
    if is_err(lf as *mut ::core::ffi::c_void) { return 0; }
    let thislen = fs16_to_cpu(sbi, (*lf).lf_size);
    if len != thislen as i32 { folio_release_kmap(folio, lf as *mut _); return 0; }
    let result = if memcmp(name as *const _, (*lf).lf_fname as *const _, len as usize) == 0 { fs32_to_cpu(sbi, (*de).de_inode) } else { 0 };
    folio_release_kmap(folio, lf as *mut _);
    result
}

unsafe fn qnx6_match(s: *mut super_block, len: i32, name: *const ::core::ffi::c_char, de: *mut qnx6_dir_entry) -> u32 {
    let sbi = QNX6_SB(s);
    if memcmp(name as *const _, (*de).de_fname as *const _, len as usize) == 0 { fs32_to_cpu(sbi, (*de).de_inode) } else { 0 }
}

pub unsafe fn qnx6_find_ino(len: i32, dir: *mut inode, name: *const ::core::ffi::c_char) -> u32 {
    let s = (*dir).i_sb;
    let ei = QNX6_I(dir);
    let npages = dir_pages(dir);
    if npages == 0 { return 0; }
    let mut start = (*ei).i_dir_start_lookup;
    if start >= npages { start = 0; }
    let mut n = start;
    loop {
        let mut folio: *mut folio = core::ptr::null_mut();
        let mut de = qnx6_get_folio(dir, n, &mut folio) as *mut qnx6_dir_entry;
        if !is_err(de as *mut _) {
            let limit = last_entry(dir, n);
            for _i in 0..limit {
                let ino = if len <= QNX6_SHORT_NAME_MAX as i32 {
                    if len != (*de).de_size as i32 { de = de.add(1); continue; }
                    qnx6_match(s, len, name, de)
                } else if (*de).de_size == 0xff { qnx6_long_match(len, name, de as *mut _, dir) } else { pr_err!("undefined filename size in inode.\n"); 0 };
                if ino != 0 { (*ei).i_dir_start_lookup = n; folio_release_kmap(folio, de as *mut _); return ino; }
                de = de.add(1);
            }
            folio_release_kmap(folio, de as *mut _);
        }
        n += 1;
        if n >= npages { n = 0; }
        if n == start { return 0; }
    }
}

pub static qnx6_dir_operations: file_operations = file_operations {
    llseek: Some(generic_file_llseek), read: Some(generic_read_dir), iterate_shared: Some(qnx6_readdir), fsync: Some(simple_fsync), setlease: Some(generic_setlease),
};

pub static qnx6_dir_inode_operations: inode_operations = inode_operations { lookup: Some(qnx6_lookup) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
