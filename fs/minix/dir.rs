// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/minix/dir.c
 *
 *  Copyright (C) 1991, 1992 Linus Torvalds
 *
 *  minix directory handling functions
 *
 *  Updated to filesystem version 3 by Daniel Aragones
 */

// Dependencies supplied by the surrounding kernel translation.

type MinixDirent = minix_dir_entry;
type Minix3Dirent = minix3_dir_entry;

unsafe fn minix_last_byte(inode: *mut inode, page_nr: c_ulong) -> c_uint {
    let mut last_byte: c_uint = PAGE_SIZE;
    if page_nr == ((*inode).i_size >> PAGE_SHIFT) {
        last_byte = (*inode).i_size & (PAGE_SIZE - 1);
    }
    last_byte
}

unsafe fn dir_commit_chunk(folio: *mut folio, pos: loff_t, len: c_uint) {
    let mapping = (*folio).mapping;
    let dir = (*mapping).host;
    block_write_end(pos, len, len, folio);
    if pos + len as loff_t > (*dir).i_size {
        i_size_write(dir, pos + len as loff_t);
        mark_inode_dirty(dir);
    }
    folio_unlock(folio);
}

unsafe fn minix_handle_dirsync(dir: *mut inode) -> c_int {
    let mut err = filemap_write_and_wait((*dir).i_mapping);
    if err == 0 { err = sync_inode_metadata(dir, 1); }
    err
}

unsafe fn dir_get_folio(dir: *mut inode, n: c_ulong, foliop: *mut *mut folio) -> *mut c_void {
    let folio = read_mapping_folio((*dir).i_mapping, n, core::ptr::null_mut());
    if IS_ERR(folio) { return ERR_CAST(folio); }
    *foliop = folio;
    kmap_local_folio(folio, 0)
}

unsafe fn minix_next_entry(de: *mut c_void, sbi: *mut minix_sb_info) -> *mut c_void {
    (de as *mut c_char).add((*sbi).s_dirsize as usize) as *mut c_void
}

unsafe fn minix_readdir(file: *mut file, ctx: *mut dir_context) -> c_int {
    let inode = file_inode(file);
    let sb = (*inode).i_sb;
    let sbi = minix_sb(sb);
    let chunk_size = (*sbi).s_dirsize;
    let npages = dir_pages(inode);
    let mut pos = (*ctx).pos as c_ulong;
    (*ctx).pos = ALIGN(pos, chunk_size) as loff_t;
    pos = (*ctx).pos as c_ulong;
    if pos >= (*inode).i_size as c_ulong { return 0; }
    let mut offset = pos & !PAGE_MASK;
    let mut n = pos >> PAGE_SHIFT;
    while n < npages {
        let mut folio: *mut folio = core::ptr::null_mut();
        let kaddr = dir_get_folio(inode, n, &mut folio);
        if IS_ERR(kaddr) { n += 1; offset = 0; continue; }
        let mut p = (kaddr as *mut c_char).add(offset as usize);
        let limit = (kaddr as *mut c_char).add(minix_last_byte(inode, n) as usize - chunk_size as usize);
        while p <= limit {
            let (name, inumber) = if (*sbi).s_version == MINIX_V3 {
                let de = p as *mut Minix3Dirent; ((*de).name.as_ptr() as *const c_char, (*de).inode)
            } else {
                let de = p as *mut MinixDirent; ((*de).name.as_ptr() as *const c_char, (*de).inode)
            };
            if inumber != 0 {
                let l = strnlen(name, (*sbi).s_namelen);
                if !dir_emit(ctx, name, l, inumber, DT_UNKNOWN) {
                    folio_release_kmap(folio, p as *mut c_void); return 0;
                }
            }
            (*ctx).pos += chunk_size as loff_t;
            p = minix_next_entry(p as *mut c_void, sbi) as *mut c_char;
        }
        folio_release_kmap(folio, kaddr);
        n += 1; offset = 0;
    }
    0
}

unsafe fn namecompare(len: c_int, maxlen: c_int, name: *const c_char, buffer: *const c_char) -> c_int {
    if len < maxlen && *buffer.add(len as usize) != 0 { return 0; }
    (!memcmp(name as *const c_void, buffer as *const c_void, len as usize)) as c_int
}

unsafe fn minix_find_entry(dentry: *mut dentry, foliop: *mut *mut folio) -> *mut MinixDirent {
    let name = (*dentry).d_name.name; let namelen = (*dentry).d_name.len as c_int;
    let dir = d_inode((*dentry).d_parent); let sbi = minix_sb((*dir).i_sb);
    let npages = dir_pages(dir); let mut n = 0;
    while n < npages {
        let kaddr = dir_get_folio(dir, n, foliop);
        if IS_ERR(kaddr) { n += 1; continue; }
        let limit = (kaddr as *mut c_char).add(minix_last_byte(dir, n) as usize - (*sbi).s_dirsize as usize);
        let mut p = kaddr as *mut c_char;
        while p <= limit {
            let (namx, inumber) = if (*sbi).s_version == MINIX_V3 { let de = p as *mut Minix3Dirent; (de.as_ref().unwrap().name.as_ptr(), (*de).inode) } else { let de = p as *mut MinixDirent; (de.as_ref().unwrap().name.as_ptr(), (*de).inode) };
            if inumber != 0 && namecompare(namelen, (*sbi).s_namelen as c_int, name, namx) != 0 { return p as *mut MinixDirent; }
            p = minix_next_entry(p as *mut c_void, sbi) as *mut c_char;
        }
        folio_release_kmap(*foliop, kaddr); n += 1;
    }
    core::ptr::null_mut()
}

unsafe fn minix_add_link(dentry: *mut dentry, inode: *mut inode) -> c_int {
    let dir = d_inode((*dentry).d_parent); let sbi = minix_sb((*dir).i_sb);
    let name = (*dentry).d_name.name; let namelen = (*dentry).d_name.len as usize;
    let mut folio = core::ptr::null_mut(); let npages = dir_pages(dir); let mut n = 0;
    while n <= npages {
        let kaddr = dir_get_folio(dir, n, &mut folio); if IS_ERR(kaddr) { return PTR_ERR(kaddr); }
        folio_lock(folio); let dir_end = (kaddr as *mut c_char).add(minix_last_byte(dir,n) as usize);
        let limit = (kaddr as *mut c_char).add(PAGE_SIZE as usize - (*sbi).s_dirsize as usize); let mut p = kaddr as *mut c_char;
        while p <= limit {
            let de = p as *mut MinixDirent; let de3 = p as *mut Minix3Dirent;
            let (namx, inumber) = if (*sbi).s_version == MINIX_V3 { ((*de3).name.as_mut_ptr(), (*de3).inode) } else { ((*de).name.as_mut_ptr(), (*de).inode) };
            if p == dir_end { if (*sbi).s_version == MINIX_V3 { (*de3).inode=0; } else { (*de).inode=0; } break; }
            if inumber == 0 { break; }
            if namecompare(namelen as c_int, (*sbi).s_namelen as c_int, name, namx) != 0 { folio_unlock(folio); folio_release_kmap(folio,kaddr); return -EEXIST; }
            p = minix_next_entry(p as *mut c_void,sbi) as *mut c_char;
        }
        let pos = folio_pos(folio) + offset_in_folio(folio,p as *mut c_void); let err=minix_prepare_chunk(folio,pos,(*sbi).s_dirsize);
        if err != 0 { folio_unlock(folio); folio_release_kmap(folio,kaddr); return err; }
        let de = p as *mut MinixDirent; let de3=p as *mut Minix3Dirent; let namx=if (*sbi).s_version==MINIX_V3 {(*de3).name.as_mut_ptr()} else {(*de).name.as_mut_ptr()};
        memcpy(namx as *mut c_void,name as *const c_void,namelen); memset(namx.add(namelen) as *mut c_void,0,(*sbi).s_dirsize as usize-namelen-if (*sbi).s_version==MINIX_V3 {4}else{2});
        if (*sbi).s_version==MINIX_V3 {(*de3).inode=(*inode).i_ino;} else {(*de).inode=(*inode).i_ino;}
        dir_commit_chunk(folio,pos,(*sbi).s_dirsize); inode_set_mtime_to_ts(dir,inode_set_ctime_current(dir)); mark_inode_dirty(dir); let err=minix_handle_dirsync(dir); folio_release_kmap(folio,kaddr); return err;
    }
    BUG(); -EINVAL
}

unsafe fn minix_delete_entry(de:*mut minix_dir_entry, folio:*mut folio)->c_int { let inode=(*(*folio).mapping).host; let sbi=minix_sb((*inode).i_sb); let pos=folio_pos(folio)+offset_in_folio(folio,de as *mut c_void); folio_lock(folio); let err=minix_prepare_chunk(folio,pos,(*sbi).s_dirsize); if err!=0 {folio_unlock(folio);return err;} if (*sbi).s_version==MINIX_V3 {(*(de as *mut Minix3Dirent)).inode=0;}else{(*de).inode=0;} dir_commit_chunk(folio,pos,(*sbi).s_dirsize);inode_set_mtime_to_ts(inode,inode_set_ctime_current(inode));mark_inode_dirty(inode);minix_handle_dirsync(inode)}

unsafe fn minix_make_empty(inode:*mut inode,dir:*mut inode)->c_int { let folio=filemap_grab_folio((*inode).i_mapping,0); if IS_ERR(folio){return PTR_ERR(folio);} let sbi=minix_sb((*inode).i_sb); let err=minix_prepare_chunk(folio,0,2*(*sbi).s_dirsize); if err!=0{folio_unlock(folio);folio_put(folio);return err;} let kaddr=kmap_local_folio(folio,0) as *mut c_char; memset(kaddr as *mut c_void,0,folio_size(folio) as usize); let de=kaddr as *mut MinixDirent; let de3=kaddr as *mut Minix3Dirent; if (*sbi).s_version==MINIX_V3{(*de3).inode=(*inode).i_ino;strcpy((*de3).name.as_mut_ptr(),b".\0".as_ptr() as _);let x=minix_next_entry(de3 as *mut c_void,sbi) as *mut Minix3Dirent;(*x).inode=(*dir).i_ino;strcpy((*x).name.as_mut_ptr(),b"..\0".as_ptr() as _);}else{(*de).inode=(*inode).i_ino;strcpy((*de).name.as_mut_ptr(),b".\0".as_ptr() as _);let x=minix_next_entry(de as *mut c_void,sbi) as *mut MinixDirent;(*x).inode=(*dir).i_ino;strcpy((*x).name.as_mut_ptr(),b"..\0".as_ptr() as _);}kunmap_local(kaddr as *mut c_void);dir_commit_chunk(folio,0,2*(*sbi).s_dirsize);let e=minix_handle_dirsync(inode);folio_put(folio);e}

unsafe fn minix_empty_dir(inode:*mut inode)->c_int { let sbi=minix_sb((*inode).i_sb);let n=dir_pages(inode);let mut i=0;while i<n{let mut f=core::ptr::null_mut();let k=dir_get_folio(inode,i,&mut f);if IS_ERR(k){i+=1;continue;}let mut p=k as *mut c_char;let lim=p.add(minix_last_byte(inode,i) as usize-(*sbi).s_dirsize as usize);while p<=lim{let(de_name,num)=if (*sbi).s_version==MINIX_V3{let d=&*(p as *mut Minix3Dirent);(d.name.as_ptr(),d.inode)}else{let d=&*(p as *mut MinixDirent);(d.name.as_ptr(),d.inode)};if num!=0&&( *de_name!=b'.' as i8||(*de_name.add(1)!=0&&(*de_name.add(1)!=b'.' as i8||*de_name.add(2)!=0||num!=(*inode).i_ino))){folio_release_kmap(f,k);return 0;}p=minix_next_entry(p as _,sbi) as _;}folio_release_kmap(f,k);i+=1;}1}

unsafe fn minix_set_link(de:*mut minix_dir_entry,folio:*mut folio,inode:*mut inode)->c_int{let dir=(*(*folio).mapping).host;let sbi=minix_sb((*dir).i_sb);let pos=folio_pos(folio)+offset_in_folio(folio,de as _);folio_lock(folio);let e=minix_prepare_chunk(folio,pos,(*sbi).s_dirsize);if e!=0{folio_unlock(folio);return e;}if (*sbi).s_version==MINIX_V3{(*(de as *mut Minix3Dirent)).inode=(*inode).i_ino;}else{(*de).inode=(*inode).i_ino;}dir_commit_chunk(folio,pos,(*sbi).s_dirsize);inode_set_mtime_to_ts(dir,inode_set_ctime_current(dir));mark_inode_dirty(dir);minix_handle_dirsync(dir)}

unsafe fn minix_dotdot(dir:*mut inode,foliop:*mut *mut folio)->*mut minix_dir_entry{let sbi=minix_sb((*dir).i_sb);let de=dir_get_folio(dir,0,foliop);if !IS_ERR(de){return minix_next_entry(de,sbi) as _;}core::ptr::null_mut()}
unsafe fn minix_inode_by_name(dentry:*mut dentry)->ino_t{let mut f=core::ptr::null_mut();let de=minix_find_entry(dentry,&mut f);if de.is_null(){return 0;}let inode=(*(*f).mapping).host;let sbi=minix_sb((*inode).i_sb);let r=if (*sbi).s_version==MINIX_V3{(*(de as *mut Minix3Dirent)).inode}else{(*de).inode};folio_release_kmap(f,de as _);r}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
