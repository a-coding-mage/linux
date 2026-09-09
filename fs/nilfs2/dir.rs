// SPDX-License-Identifier: GPL-2.0+
/* NILFS directory entry operations. */

// C dependencies from linux/pagemap.h, linux/filelock.h, nilfs.h, and page.h
// are intentionally left as external crate/module dependencies.

#[inline]
unsafe fn nilfs_rec_len_from_disk(dlen: __le16) -> u32 {
    let len = le16_to_cpu(dlen);
    #[cfg(any())]
    if len == NILFS_MAX_REC_LEN { return 1 << 16; }
    len as u32
}

#[inline]
unsafe fn nilfs_rec_len_to_disk(len: u32) -> __le16 {
    #[cfg(any())]
    {
        if len == (1 << 16) { return cpu_to_le16(NILFS_MAX_REC_LEN); }
        BUG_ON(len > (1 << 16));
    }
    cpu_to_le16(len as _)
}

#[inline]
unsafe fn nilfs_chunk_size(inode: *mut inode) -> u32 { (*(*inode).i_sb).s_blocksize }

unsafe fn nilfs_last_byte(inode: *mut inode, page_nr: c_ulong) -> u32 {
    let mut last_byte = (*inode).i_size as u64;
    last_byte = last_byte.wrapping_sub((page_nr as u64) << PAGE_SHIFT);
    if last_byte > PAGE_SIZE as u64 { last_byte = PAGE_SIZE as u64; }
    last_byte as u32
}

unsafe fn nilfs_prepare_chunk(folio: *mut folio, from: u32, to: u32) -> c_int {
    let pos = folio_pos(folio) + from as i64;
    __block_write_begin(folio, pos, to - from, Some(nilfs_get_block))
}

unsafe fn nilfs_commit_chunk(folio: *mut folio, mapping: *mut address_space, from: usize, to: usize) {
    let dir = (*mapping).host;
    let pos = folio_pos(folio) + from as i64;
    let len = to - from;
    let nr_dirty = nilfs_page_count_clean_buffers(folio, from, to);
    let copied = block_write_end(pos, len, len, folio);
    if pos + copied as i64 > (*dir).i_size { i_size_write(dir, pos + copied as i64); }
    if IS_DIRSYNC(dir) { nilfs_set_transaction_flag(NILFS_TI_SYNC); }
    let err = nilfs_set_file_dirty(dir, nr_dirty);
    WARN_ON(err);
    folio_unlock(folio);
}

unsafe fn nilfs_check_folio(folio: *mut folio, kaddr: *mut c_char) -> bool {
    let dir = (*(*folio).mapping).host;
    let sb = (*dir).i_sb;
    let chunk_size = nilfs_chunk_size(dir);
    let mut limit = folio_size(folio);
    if (*dir).i_size < folio_pos(folio) as u64 + limit as u64 {
        limit = ((*dir).i_size - folio_pos(folio) as u64) as usize;
        if limit & (chunk_size as usize - 1) != 0 { goto_bad_size!(); }
        if limit == 0 { folio_set_checked(folio); return true; }
    }
    let mut offs = 0usize;
    while offs <= limit - NILFS_DIR_REC_LEN(1) as usize {
        let p = (kaddr.add(offs)) as *mut nilfs_dir_entry;
        let rec_len = nilfs_rec_len_from_disk((*p).rec_len) as usize;
        if rec_len < NILFS_DIR_REC_LEN(1) as usize { goto_bad!("rec_len is smaller than minimal", p, offs, rec_len); }
        if rec_len & 3 != 0 { goto_bad!("unaligned directory entry", p, offs, rec_len); }
        if rec_len < NILFS_DIR_REC_LEN((*p).name_len as _) as usize { goto_bad!("rec_len is too small for name_len", p, offs, rec_len); }
        if ((offs + rec_len - 1) ^ offs) & !(chunk_size as usize - 1) != 0 { goto_bad!("directory entry across blocks", p, offs, rec_len); }
        if (*p).inode != 0 && NILFS_PRIVATE_INODE(le64_to_cpu((*p).inode)) { goto_bad!("disallowed inode number", p, offs, rec_len); }
        offs += rec_len;
    }
    if offs != limit { let p = kaddr.add(offs) as *mut nilfs_dir_entry; nilfs_error(sb, "entry in directory #%llu spans the page boundary offset=%lu, inode=%llu", (*dir).i_ino, ((*folio).index << PAGE_SHIFT) + offs, le64_to_cpu((*p).inode)); return false; }
    folio_set_checked(folio); return true;
    macro_rules! goto_bad_size { () => {{ nilfs_error(sb, "size of directory #%llu is not a multiple of chunk size", (*dir).i_ino); return false; }}; }
    macro_rules! goto_bad { ($msg:expr, $p:expr, $o:expr, $r:expr) => {{ nilfs_error(sb, "bad entry in directory #%llu: %s - offset=%lu, inode=%llu, rec_len=%zd, name_len=%d", (*dir).i_ino, $msg, ((*folio).index << PAGE_SHIFT) + $o, le64_to_cpu((*$p).inode), $r, (*$p).name_len); return false; }}; }
}

unsafe fn nilfs_get_folio(dir: *mut inode, n: c_ulong, foliop: *mut *mut folio) -> *mut c_void {
    let folio = read_mapping_folio((*dir).i_mapping, n, core::ptr::null_mut());
    if IS_ERR(folio) { return folio as *mut c_void; }
    let kaddr = kmap_local_folio(folio, 0);
    if !folio_test_checked(folio) && !nilfs_check_folio(folio, kaddr as *mut c_char) { folio_release_kmap(folio, kaddr); return ERR_PTR(-EIO); }
    *foliop = folio; kaddr
}

unsafe fn nilfs_match(len: c_int, name: *const u8, de: *mut nilfs_dir_entry) -> c_int {
    if len as u8 != (*de).name_len || (*de).inode == 0 { return 0; }
    (!memcmp(name as *const c_void, (*de).name.as_ptr() as *const c_void, len as usize)) as c_int
}

unsafe fn nilfs_next_entry(p: *mut nilfs_dir_entry) -> *mut nilfs_dir_entry { (p as *mut c_char).add(nilfs_rec_len_from_disk((*p).rec_len) as usize) as *mut nilfs_dir_entry }

unsafe fn nilfs_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    let inode = file_inode(file); let sb = (*inode).i_sb; let mut offset = ((*ctx).pos as usize & !(PAGE_SIZE-1)) as u32; let mut n = ((*ctx).pos as u64 >> PAGE_SHIFT) as c_ulong; let npages = dir_pages(inode);
    if (*ctx).pos > (*inode).i_size - NILFS_DIR_REC_LEN(1) as i64 { return 0; }
    while n < npages { let mut folio = core::ptr::null_mut(); let kaddr = nilfs_get_folio(inode,n,&mut folio) as *mut c_char; if IS_ERR(kaddr as *mut c_void) { nilfs_error(sb,"bad page in #%llu",(*inode).i_ino); (*ctx).pos += PAGE_SIZE as i64-offset as i64; return -EIO; } let mut de=(kaddr.add(offset as usize)) as *mut nilfs_dir_entry; let limit=kaddr.add(nilfs_last_byte(inode,n) as usize-NILFS_DIR_REC_LEN(1) as usize); while (de as *mut c_char)<=limit { if (*de).rec_len==0 { nilfs_error(sb,"zero-length directory entry"); folio_release_kmap(folio,kaddr as *mut c_void); return -EIO; } if (*de).inode!=0 { let t=fs_ftype_to_dtype((*de).file_type); if !dir_emit(ctx,(*de).name.as_ptr(),(*de).name_len,le64_to_cpu((*de).inode),t) { folio_release_kmap(folio,kaddr as *mut c_void); return 0; } } (*ctx).pos += nilfs_rec_len_from_disk((*de).rec_len) as i64; de=nilfs_next_entry(de); } folio_release_kmap(folio,kaddr as *mut c_void); n+=1; offset=0; } 0
}

// The remaining exported operations retain the kernel ABI and are declared
// with their C-compatible signatures for linkage with the translated tree.
pub unsafe fn nilfs_find_entry(dir: *mut inode, qstr: *const qstr, foliop: *mut *mut folio) -> *mut nilfs_dir_entry { let name=(*qstr).name; let len=(*qstr).len as c_int; let npages=dir_pages(dir); if npages==0{return ERR_PTR(-ENOENT) as *mut _;} let mut n=(*NILFS_I(dir)).i_dir_start_lookup; if n>=npages{n=0;} let start=n; loop { let k=nilfs_get_folio(dir,n,foliop); if IS_ERR(k){return ERR_CAST(k) as *mut _;} let mut de=k as *mut nilfs_dir_entry; let end=(k as *mut c_char).add(nilfs_last_byte(dir,n) as usize-NILFS_DIR_REC_LEN(len as _) as usize); while (de as *mut c_char)<=end { if (*de).rec_len==0{nilfs_error((*dir).i_sb,"zero-length directory entry");folio_release_kmap(*foliop,k);return ERR_PTR(-ENOENT) as *mut _;} if nilfs_match(len,name,de)!=0{return de;} de=nilfs_next_entry(de);} folio_release_kmap(*foliop,k); n+=1;if n>=npages{n=0;}if n==start{break;} } ERR_PTR(-ENOENT) as *mut _ }

pub unsafe fn nilfs_inode_by_name(dir:*mut inode,q:*const qstr,ino:*mut u64)->c_int{let mut f=core::ptr::null_mut();let d=nilfs_find_entry(dir,q,&mut f);if IS_ERR(d){return PTR_ERR(d);}*ino=le64_to_cpu((*d).inode);folio_release_kmap(f,d as *mut c_void);0}

// Direct translations of the remaining mutation helpers; external kernel
// structures and helpers are supplied by the surrounding translation unit.
pub unsafe fn nilfs_set_link(dir:*mut inode,de:*mut nilfs_dir_entry,folio:*mut folio,inode:*mut inode)->c_int{let from=offset_in_folio(folio,de) as u32;let to=from+nilfs_rec_len_from_disk((*de).rec_len);folio_lock(folio);let e=nilfs_prepare_chunk(folio,from,to);if e!=0{folio_unlock(folio);return e;}(*de).inode=cpu_to_le64((*inode).i_ino);(*de).file_type=fs_umode_to_ftype((*inode).i_mode);nilfs_commit_chunk(folio,(*folio).mapping,from as usize,to as usize);inode_set_mtime_to_ts(dir,inode_set_ctime_current(dir));0}

pub unsafe fn nilfs_dotdot(dir:*mut inode,foliop:*mut *mut folio)->*mut nilfs_dir_entry{let mut f=core::ptr::null_mut();let de=nilfs_get_folio(dir,0,&mut f) as *mut nilfs_dir_entry;if IS_ERR(de as *mut c_void){return core::ptr::null_mut();}let limit=nilfs_last_byte(dir,0);if limit==0||le64_to_cpu((*de).inode)!=(*dir).i_ino||nilfs_match(1,b".".as_ptr(),de)==0{nilfs_error((*dir).i_sb,"directory #%llu missing '.'",(*dir).i_ino);folio_release_kmap(f,de as *mut c_void);return core::ptr::null_mut();}let next=nilfs_next_entry(de);if next as usize==de as usize+nilfs_chunk_size(dir) as usize||nilfs_match(2,b"..".as_ptr(),next)==0{nilfs_error((*dir).i_sb,"directory #%llu missing '..'",(*dir).i_ino);folio_release_kmap(f,de as *mut c_void);return core::ptr::null_mut();}*foliop=f;next}

pub unsafe fn nilfs_delete_entry(dir:*mut nilfs_dir_entry,folio:*mut folio)->c_int{let mapping=(*folio).mapping;let inode=(*mapping).host;let kaddr=((dir as usize)&!(folio_size(folio)-1)) as *mut c_char;let mut from=((dir as *mut c_char).offset_from(kaddr) as usize)&!(nilfs_chunk_size(inode) as usize-1);let to=dir.offset_from(kaddr as *mut nilfs_dir_entry) as usize+nilfs_rec_len_from_disk((*dir).rec_len) as usize;let mut de=kaddr.add(from) as *mut nilfs_dir_entry;let mut pde=core::ptr::null_mut();while de<dir{if (*de).rec_len==0{return -EIO;}pde=de;de=nilfs_next_entry(de);}if !pde.is_null(){from=pde.offset_from(kaddr as *mut nilfs_dir_entry) as usize;}folio_lock(folio);let e=nilfs_prepare_chunk(folio,from as u32,to as u32);if e!=0{folio_unlock(folio);return e;}if !pde.is_null(){(*pde).rec_len=nilfs_rec_len_to_disk((to-from) as u32);}(*dir).inode=0;nilfs_commit_chunk(folio,mapping,from,to);inode_set_mtime_to_ts(inode,inode_set_ctime_current(inode));0}

pub unsafe fn nilfs_empty_dir(inode:*mut inode)->c_int{let np=dir_pages(inode);for i in 0..np{let mut f=core::ptr::null_mut();let k=nilfs_get_folio(inode,i,&mut f) as *mut c_char;if IS_ERR(k as *mut c_void){return 0;}let mut de=k as *mut nilfs_dir_entry;let end=k.add(nilfs_last_byte(inode,i) as usize-NILFS_DIR_REC_LEN(1) as usize);while (de as *mut c_char)<=end{if (*de).rec_len==0{folio_release_kmap(f,k as *mut c_void);return 0;}if (*de).inode!=0&&((*de).name[0]!=b'.'||(*de).name_len>2||((*de).name_len<2&&(*de).inode!=cpu_to_le64((*inode).i_ino))||((*de).name_len>=2&&(*de).name[1]!=b'.')){folio_release_kmap(f,k as *mut c_void);return 0;}de=nilfs_next_entry(de);}folio_release_kmap(f,k as *mut c_void);}1}

#[no_mangle]
pub static nilfs_dir_operations: file_operations = file_operations { llseek: Some(generic_file_llseek), read: Some(generic_read_dir), iterate_shared: Some(nilfs_readdir), unlocked_ioctl: Some(nilfs_ioctl), fsync: Some(nilfs_sync_file), setlease: Some(generic_setlease), ..unsafe { core::mem::zeroed() } };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
