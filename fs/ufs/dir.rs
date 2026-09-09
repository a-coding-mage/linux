// SPDX-License-Identifier: GPL-2.0
/* Rust translation of linux/fs/ufs/ufs_dir.c. */

// Kernel dependencies supplied by the surrounding translation unit.

#[inline]
unsafe fn ufs_match(sb: *mut super_block, len: c_int, name: *const u8,
                    de: *mut ufs_dir_entry) -> c_int {
    if len != ufs_get_de_namlen(sb, de) { return 0; }
    if (*de).d_ino == 0 { return 0; }
    (!memcmp(name, (*de).d_name.as_ptr(), len as usize)) as c_int
}

unsafe fn ufs_commit_chunk(folio: *mut folio, pos: loff_t, len: c_uint) {
    let mapping = (*folio).mapping;
    let dir = (*mapping).host;
    inode_inc_iversion(dir);
    block_write_end(pos, len, len, folio);
    if pos + len as i64 > (*dir).i_size {
        i_size_write(dir, pos + len as i64);
        mark_inode_dirty(dir);
    }
    folio_unlock(folio);
}

unsafe fn ufs_handle_dirsync(dir: *mut inode) -> c_int {
    let mut err = filemap_write_and_wait((*dir).i_mapping);
    if err == 0 { err = sync_inode_metadata(dir, 1); }
    err
}

unsafe fn ufs_inode_by_name(dir: *mut inode, qstr: *const qstr) -> ino_t {
    let mut res: ino_t = 0;
    let mut folio: *mut folio = core::ptr::null_mut();
    let de = ufs_find_entry(dir, qstr, &mut folio);
    if !de.is_null() {
        res = fs32_to_cpu((*dir).i_sb, (*de).d_ino);
        folio_release_kmap(folio, de as *mut c_void);
    }
    res
}

unsafe fn ufs_set_link(dir: *mut inode, de: *mut ufs_dir_entry,
                       folio: *mut folio, inode: *mut inode,
                       update_times: bool) -> c_int {
    let pos = folio_pos(folio) + offset_in_folio(folio, de as *mut c_void) as i64;
    let len = fs16_to_cpu((*dir).i_sb, (*de).d_reclen) as c_uint;
    folio_lock(folio);
    let err = ufs_prepare_chunk(folio, pos, len);
    if err != 0 { folio_unlock(folio); return err; }
    (*de).d_ino = cpu_to_fs32((*dir).i_sb, (*inode).i_ino);
    ufs_set_de_type((*dir).i_sb, de, (*inode).i_mode);
    ufs_commit_chunk(folio, pos, len);
    if update_times { inode_set_mtime_to_ts(dir, inode_set_ctime_current(dir)); }
    mark_inode_dirty(dir);
    ufs_handle_dirsync(dir)
}

unsafe fn ufs_check_folio(folio: *mut folio, kaddr: *mut c_char) -> bool {
    let dir = (*(*folio).mapping).host;
    let sb = (*dir).i_sb;
    let mut limit = folio_size(folio);
    let chunk_mask = UFS_SB(sb).s_uspi.s_dirblksize - 1;
    let mut offs: c_uint = 0;
    let mut rec_len: c_uint;
    let mut p: *mut ufs_dir_entry = core::ptr::null_mut();
    let mut error: *const c_char;
    if (*dir).i_size < folio_pos(folio) + limit as i64 {
        limit = offset_in_folio(folio, (*dir).i_size) as c_uint;
        if limit & chunk_mask != 0 { goto_bad_size(sb, dir); return false; }
        if limit == 0 { folio_set_checked(folio); return true; }
    }
    while offs <= limit - UFS_DIR_REC_LEN(1) {
        p = (kaddr.add(offs as usize)) as *mut ufs_dir_entry;
        rec_len = fs16_to_cpu(sb, (*p).d_reclen) as c_uint;
        if rec_len < UFS_DIR_REC_LEN(1) { error = c"rec_len is smaller than minimal".as_ptr(); break; }
        if rec_len & 3 != 0 { error = c"unaligned directory entry".as_ptr(); break; }
        if rec_len < UFS_DIR_REC_LEN(ufs_get_de_namlen(sb, p) as c_int) { error = c"rec_len is too small for name_len".as_ptr(); break; }
        if ((offs + rec_len - 1) ^ offs) & !chunk_mask != 0 { error = c"directory entry across blocks".as_ptr(); break; }
        if fs32_to_cpu(sb, (*p).d_ino) > UFS_SB(sb).s_uspi.s_ipg * UFS_SB(sb).s_uspi.s_ncg { error = c"inode out of bounds".as_ptr(); break; }
        offs += rec_len;
    }
    if offs != limit { if p.is_null() { p = kaddr.add(offs as usize) as *mut ufs_dir_entry; } ufs_error(sb, c"ufs_check_folio".as_ptr(), c"bad entry in directory #%llu: %s - offset=%llu, rec_len=%d, name_len=%d".as_ptr(), (*dir).i_ino, error, folio_pos(folio) + offs as i64, rec_len, ufs_get_de_namlen(sb, p)); return false; }
    folio_set_checked(folio); true
}

unsafe fn goto_bad_size(sb: *mut super_block, dir: *mut inode) {
    ufs_error(sb, c"ufs_check_folio".as_ptr(), c"size of directory #%llu is not a multiple of chunk size".as_ptr(), (*dir).i_ino);
}

unsafe fn ufs_get_folio(dir: *mut inode, n: c_ulong, foliop: *mut *mut folio) -> *mut c_void {
    let folio = read_mapping_folio((*dir).i_mapping, n, core::ptr::null_mut());
    if IS_ERR(folio) { return ERR_CAST(folio); }
    let kaddr = kmap_local_folio(folio, 0);
    if !folio_test_checked(folio) && !ufs_check_folio(folio, kaddr as *mut c_char) {
        folio_release_kmap(folio, kaddr); return ERR_PTR(-EIO);
    }
    *foliop = folio; kaddr
}

unsafe fn ufs_last_byte(inode: *mut inode, page_nr: c_ulong) -> c_uint {
    let mut last_byte = (*inode).i_size as c_uint - (page_nr << PAGE_SHIFT) as c_uint;
    if last_byte > PAGE_SIZE { last_byte = PAGE_SIZE; } last_byte
}

#[inline]
unsafe fn ufs_next_entry(sb: *mut super_block, p: *mut ufs_dir_entry) -> *mut ufs_dir_entry {
    (p as *mut c_char).add(fs16_to_cpu(sb, (*p).d_reclen) as usize) as *mut ufs_dir_entry
}

unsafe fn ufs_dotdot(dir: *mut inode, foliop: *mut *mut folio) -> *mut ufs_dir_entry {
    let de = ufs_get_folio(dir, 0, foliop) as *mut ufs_dir_entry;
    if !IS_ERR(de as *mut c_void) { ufs_next_entry((*dir).i_sb, de) } else { core::ptr::null_mut() }
}

unsafe fn ufs_find_entry(dir: *mut inode, qstr: *const qstr, foliop: *mut *mut folio) -> *mut ufs_dir_entry {
    let sb = (*dir).i_sb; let name = (*qstr).name; let namelen = (*qstr).len as c_int;
    let reclen = UFS_DIR_REC_LEN(namelen); let npages = dir_pages(dir); let ui = UFS_I(dir);
    if npages == 0 || namelen > UFS_MAXNAMLEN { return core::ptr::null_mut(); }
    let mut start = (*ui).i_dir_start_lookup; if start >= npages { start = 0; } let mut n = start;
    loop { let mut kaddr = ufs_get_folio(dir, n, foliop); if !IS_ERR(kaddr) { let mut de = kaddr as *mut ufs_dir_entry; let end = kaddr.add((ufs_last_byte(dir,n)-reclen) as usize) as *mut c_char; while (de as *mut c_char) <= end { if ufs_match(sb,namelen,name,de)!=0 { (*ui).i_dir_start_lookup=n; return de; } de=ufs_next_entry(sb,de); } folio_release_kmap(*foliop,kaddr); } n+=1; if n>=npages {n=0;} if n==start {return core::ptr::null_mut();} }
}

unsafe fn ufs_validate_entry(sb: *mut super_block, base: *mut c_char, offset: c_uint, mask: c_uint) -> c_uint { let de=base.add(offset as usize) as *mut ufs_dir_entry; let mut p=base.add((offset&mask) as usize) as *mut ufs_dir_entry; while (p as *mut c_char)<(de as *mut c_char) {p=ufs_next_entry(sb,p);} (p as *mut c_char).offset_from(base) as c_uint }

unsafe fn ufs_delete_entry(inode: *mut inode, dir: *mut ufs_dir_entry, folio: *mut folio) -> c_int { let sb=(*inode).i_sb; let to=offset_in_folio(folio,dir as *mut c_void) as usize+fs16_to_cpu(sb,(*dir).d_reclen) as usize; let base=(dir as *mut c_char).sub(offset_in_folio(folio,dir as *mut c_void) as usize); let mut from=offset_in_folio(folio,dir as *mut c_void) as usize & !(UFS_SB(sb).s_uspi.s_dirblksize as usize-1); let mut de=base.add(from) as *mut ufs_dir_entry; let mut pde=core::ptr::null_mut(); while (de as *mut c_char)<(dir as *mut c_char) {if (*de).d_reclen==0{return -EIO;} pde=de;de=ufs_next_entry(sb,de);} if !pde.is_null(){from=offset_in_folio(folio,pde as *mut c_void) as usize;} let pos=folio_pos(folio)+from as i64; folio_lock(folio); let err=ufs_prepare_chunk(folio,pos,(to-from) as c_uint); if err!=0{folio_unlock(folio);return err;} if !pde.is_null(){(*pde).d_reclen=cpu_to_fs16(sb,(to-from) as u16);} (*dir).d_ino=0; ufs_commit_chunk(folio,pos,(to-from) as c_uint); inode_set_mtime_to_ts(inode,inode_set_ctime_current(inode)); mark_inode_dirty(inode); ufs_handle_dirsync(inode) }

unsafe fn ufs_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    let inode = file_inode(file); let sb=(*inode).i_sb; let mut pos=(*ctx).pos;
    let mut offset=(pos & !(PAGE_MASK as i64)) as c_uint; let mut n=(pos>>PAGE_SHIFT) as c_ulong;
    let npages=dir_pages(inode); let chunk_mask=!(UFS_SB(sb).s_uspi.s_dirblksize-1);
    let mut need_revalidate=!inode_eq_iversion(inode,*( (*file).private_data as *mut u64));
    if pos > (*inode).i_size-UFS_DIR_REC_LEN(1) as i64{return 0;}
    while n<npages {let mut folio=core::ptr::null_mut(); let kaddr=ufs_get_folio(inode,n,&mut folio); if IS_ERR(kaddr){(*ctx).pos+=PAGE_SIZE as i64-offset as i64;return PTR_ERR(kaddr);} if need_revalidate {if offset!=0 {offset=ufs_validate_entry(sb,kaddr as *mut c_char,offset,chunk_mask);(*ctx).pos=(n<<PAGE_SHIFT)+(offset as c_ulong) as i64;} *((*file).private_data as *mut u64)=inode_query_iversion(inode);need_revalidate=false;} let mut de=kaddr.add(offset as usize) as *mut ufs_dir_entry; let limit=kaddr.add((ufs_last_byte(inode,n)-UFS_DIR_REC_LEN(1)) as usize) as *mut c_char; while (de as *mut c_char)<=limit {if (*de).d_ino!=0 {let ty=if (UFS_SB(sb).s_flags&UFS_DE_MASK)==UFS_DE_44BSD{(*de).d_u.d_44.d_type}else{DT_UNKNOWN}; if dir_emit(ctx,(*de).d_name.as_ptr(),ufs_get_de_namlen(sb,de),fs32_to_cpu(sb,(*de).d_ino),ty)==0 {folio_release_kmap(folio,de as *mut c_void);return 0;}}(*ctx).pos+=fs16_to_cpu(sb,(*de).d_reclen) as i64;de=ufs_next_entry(sb,de);} folio_release_kmap(folio,kaddr);n+=1;offset=0;} 0
}

unsafe fn ufs_add_link(dentry:*mut dentry,inode:*mut inode)->c_int {let dir=d_inode((*dentry).d_parent);let sb=(*dir).i_sb;let namelen=(*dentry).d_name.len as c_int;let reclen=UFS_DIR_REC_LEN(namelen);let chunk=UFS_SB(sb).s_uspi.s_dirblksize;let mut folio=core::ptr::null_mut();for n in 0..=dir_pages(dir){let k=ufs_get_folio(dir,n,&mut folio);if IS_ERR(k){return PTR_ERR(k);}folio_lock(folio);let end=k.add(ufs_last_byte(dir,n) as usize);let mut de=k as *mut ufs_dir_entry;let lim=k.add(folio_size(folio) as usize-reclen as usize);while (de as *mut c_char)<=lim {if de as *mut c_char==end {(*de).d_reclen=cpu_to_fs16(sb,chunk as u16);(*de).d_ino=0;break;}if (*de).d_reclen==0{folio_unlock(folio);folio_release_kmap(folio,de as *mut c_void);return -EIO;}if ufs_match(sb,namelen,(*dentry).d_name.name,de)!=0{folio_unlock(folio);folio_release_kmap(folio,de as *mut c_void);return -EEXIST;}let name_len=UFS_DIR_REC_LEN(ufs_get_de_namlen(sb,de));let r=fs16_to_cpu(sb,(*de).d_reclen) as c_uint;if ((*de).d_ino==0&&r>=reclen)||(r>=name_len+reclen){let pos=folio_pos(folio)+offset_in_folio(folio,de as *mut c_void) as i64;let e=ufs_prepare_chunk(folio,pos,r);if e!=0{folio_unlock(folio);folio_release_kmap(folio,de as *mut c_void);return e;}if (*de).d_ino!=0{let de1=(de as *mut c_char).add(name_len as usize) as *mut ufs_dir_entry;(*de1).d_reclen=cpu_to_fs16(sb,(r-name_len) as u16);(*de).d_reclen=cpu_to_fs16(sb,name_len as u16);de=de1;}ufs_set_de_namlen(sb,de,namelen);memcpy((*de).d_name.as_mut_ptr(),(*dentry).d_name.name,(namelen+1) as usize);(*de).d_ino=cpu_to_fs32(sb,(*inode).i_ino);ufs_set_de_type(sb,de,(*inode).i_mode);ufs_commit_chunk(folio,pos,r);inode_set_mtime_to_ts(dir,inode_set_ctime_current(dir));mark_inode_dirty(dir);let e=ufs_handle_dirsync(dir);folio_release_kmap(folio,de as *mut c_void);return e;}de=ufs_next_entry(sb,de);}folio_unlock(folio);folio_release_kmap(folio,k);} BUG();-EINVAL}

unsafe fn ufs_make_empty(inode:*mut inode,dir:*mut inode)->c_int {let sb=(*dir).i_sb;let folio=filemap_grab_folio((*inode).i_mapping,0);if IS_ERR(folio){return PTR_ERR(folio);}let chunk=UFS_SB(sb).s_uspi.s_dirblksize;let e=ufs_prepare_chunk(folio,0,chunk);if e!=0{folio_unlock(folio);folio_put(folio);return e;}let k=kmap_local_folio(folio,0);memset(k,0,folio_size(folio));let de=k as *mut ufs_dir_entry;(*de).d_ino=cpu_to_fs32(sb,(*inode).i_ino);ufs_set_de_type(sb,de,(*inode).i_mode);ufs_set_de_namlen(sb,de,1);(*de).d_reclen=cpu_to_fs16(sb,UFS_DIR_REC_LEN(1) as u16);strcpy((*de).d_name.as_mut_ptr(),c".".as_ptr());let de=de.add(UFS_DIR_REC_LEN(1) as usize/ core::mem::size_of::<ufs_dir_entry>());(*de).d_ino=cpu_to_fs32(sb,(*dir).i_ino);ufs_set_de_type(sb,de,(*dir).i_mode);(*de).d_reclen=cpu_to_fs16(sb,(chunk-UFS_DIR_REC_LEN(1)) as u16);ufs_set_de_namlen(sb,de,2);strcpy((*de).d_name.as_mut_ptr(),c"..".as_ptr());kunmap_local(k);ufs_commit_chunk(folio,0,chunk);let e=ufs_handle_dirsync(inode);folio_put(folio);e}

unsafe fn ufs_empty_dir(inode:*mut inode)->c_int {let sb=(*inode).i_sb;let mut folio=core::ptr::null_mut();for i in 0..dir_pages(inode){let k=ufs_get_folio(inode,i,&mut folio);if IS_ERR(k){continue;}let mut de=k as *mut ufs_dir_entry;let lim=k.add((ufs_last_byte(inode,i)-UFS_DIR_REC_LEN(1)) as usize) as *mut c_char;while (de as *mut c_char)<=lim {if (*de).d_reclen==0{folio_release_kmap(folio,k);return 0;}if (*de).d_ino!=0 {let l=ufs_get_de_namlen(sb,de);if (*de).d_name[0]!=b'.'||l>2||(l<2&&(*inode).i_ino!=fs32_to_cpu(sb,(*de).d_ino))||(l==2&&(*de).d_name[1]!=b'.'){folio_release_kmap(folio,k);return 0;}}de=ufs_next_entry(sb,de);}folio_release_kmap(folio,k);}1}

unsafe fn ufs_dir_open(_inode:*mut inode,file:*mut file)->c_int {(*file).private_data=kzalloc(core::mem::size_of::<u64>(),GFP_KERNEL);if (*file).private_data.is_null(){-ENOMEM}else{0}}
unsafe fn ufs_dir_release(_inode:*mut inode,file:*mut file)->c_int {kfree((*file).private_data);0}
unsafe fn ufs_dir_llseek(file:*mut file,offset:loff_t,whence:c_int)->loff_t {generic_llseek_cookie(file,offset,whence,(*file).private_data as *mut u64)}

// const struct file_operations ufs_dir_operations = { .open = ufs_dir_open,
// .release = ufs_dir_release, .read = generic_read_dir,
// .iterate_shared = ufs_readdir, .fsync = simple_fsync,
// .llseek = ufs_dir_llseek, .setlease = generic_setlease };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
