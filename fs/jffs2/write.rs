/* JFFS2 -- Journalling Flash File System, Version 2. */
/* Kernel headers and local headers from write.c are external dependencies. */

pub unsafe fn jffs2_do_new_inode(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info,
                                  mode: u32, ri: *mut jffs2_raw_inode) -> i32 {
    let ic = jffs2_alloc_inode_cache();
    if ic.is_null() { return -ENOMEM; }
    core::ptr::write_bytes(ic as *mut u8, 0, core::mem::size_of::<jffs2_inode_cache>());
    (*f).inocache = ic;
    (*(*f).inocache).pino_nlink = 1;
    (*(*f).inocache).nodes = (*f).inocache as *mut jffs2_raw_node_ref;
    (*(*f).inocache).state = INO_STATE_PRESENT;
    jffs2_add_ino_cache(c, (*f).inocache);
    jffs2_dbg(1, "%s(): Assigned ino# %d\n", "jffs2_do_new_inode", (*f).inocache.ino);
    (*ri).ino = cpu_to_je32((*f).inocache.ino);
    (*ri).magic = cpu_to_je16(JFFS2_MAGIC_BITMASK);
    (*ri).nodetype = cpu_to_je16(JFFS2_NODETYPE_INODE);
    (*ri).totlen = cpu_to_je32(PAD(core::mem::size_of::<jffs2_raw_inode>()));
    (*ri).hdr_crc = cpu_to_je32(crc32(0, ri as *const _, core::mem::size_of::<jffs2_unknown_node>() - 4));
    (*ri).mode = cpu_to_jemode(mode);
    (*f).highest_version = 1;
    (*ri).version = cpu_to_je32((*f).highest_version);
    0
}

pub unsafe fn jffs2_write_dnode(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info,
                                ri: *mut jffs2_raw_inode, data: *const u8,
                                datalen: u32, alloc_mode: i32) -> *mut jffs2_full_dnode {
    let mut retlen: usize = 0;
    let mut flash_ofs: u32;
    let mut vecs = [kvec { iov_base: ri as *mut _, iov_len: core::mem::size_of::<jffs2_raw_inode>() },
                    kvec { iov_base: data as *mut _, iov_len: datalen as usize }];
    let mut ret: i32;
    let mut retried = 0;
    let mut cnt: usize = 2;
    vecs[1].iov_base = data as *mut _;
    let fnn = jffs2_alloc_full_dnode();
    if fnn.is_null() { return ERR_PTR(-ENOMEM); }
    if datalen == 0 || data.is_null() { cnt = 1; }
    'retry: loop {
        flash_ofs = write_ofs(c);
        jffs2_dbg_prewrite_paranoia_check(c, flash_ofs, vecs[0].iov_len + vecs[1].iov_len);
        if alloc_mode != ALLOC_GC && je32_to_cpu((*ri).version) < (*f).highest_version {
            BUG_ON(retried == 0);
            (*f).highest_version += 1;
            (*ri).version = cpu_to_je32((*f).highest_version);
            (*ri).node_crc = cpu_to_je32(crc32(0, ri as *const _, core::mem::size_of::<jffs2_raw_inode>() - 8));
        }
        ret = jffs2_flash_writev(c, vecs.as_mut_ptr(), cnt, flash_ofs, &mut retlen,
                                 if alloc_mode == ALLOC_GC { 0 } else { (*f).inocache.ino });
        if ret != 0 || retlen != core::mem::size_of::<jffs2_raw_inode>() + datalen as usize {
            if retlen != 0 { jffs2_add_physical_node_ref(c, flash_ofs | REF_OBSOLETE, PAD(core::mem::size_of::<jffs2_raw_inode>() + datalen as usize), core::ptr::null_mut()); }
            if retried == 0 && alloc_mode != ALLOC_NORETRY {
                let mut dummy = 0u32; retried = 1;
                let jeb = &mut (*c).blocks[(flash_ofs / (*c).sector_size) as usize];
                jffs2_dbg_acct_sanity_check(c, jeb); jffs2_dbg_acct_paranoia_check(c, jeb);
                if alloc_mode == ALLOC_GC {
                    ret = jffs2_reserve_space_gc(c, core::mem::size_of::<jffs2_raw_inode>() as u32 + datalen, &mut dummy, JFFS2_SUMMARY_INODE_SIZE);
                } else {
                    mutex_unlock(&mut (*f).sem); jffs2_complete_reservation(c);
                    ret = jffs2_reserve_space(c, core::mem::size_of::<jffs2_raw_inode>() as u32 + datalen, &mut dummy, alloc_mode, JFFS2_SUMMARY_INODE_SIZE);
                    mutex_lock(&mut (*f).sem);
                }
                if ret == 0 { continue 'retry; }
            }
            jffs2_free_full_dnode(fnn); return ERR_PTR(if ret != 0 { ret } else { -EIO });
        }
        if je32_to_cpu((*ri).dsize) >= PAGE_SIZE ||
           ((je32_to_cpu((*ri).offset) & (PAGE_SIZE - 1)) == 0 && je32_to_cpu((*ri).dsize) + je32_to_cpu((*ri).offset) == je32_to_cpu((*ri).isize)) {
            flash_ofs |= REF_PRISTINE;
        } else { flash_ofs |= REF_NORMAL; }
        (*fnn).raw = jffs2_add_physical_node_ref(c, flash_ofs, PAD(core::mem::size_of::<jffs2_raw_inode>() + datalen as usize), (*f).inocache);
        if IS_ERR((*fnn).raw) { let e = (*fnn).raw; jffs2_free_full_dnode(fnn); return ERR_CAST(e); }
        (*fnn).ofs = je32_to_cpu((*ri).offset); (*fnn).size = je32_to_cpu((*ri).dsize); (*fnn).frags = 0;
        if retried != 0 { jffs2_dbg_acct_sanity_check(c, core::ptr::null_mut()); }
        return fnn;
    }
}

pub unsafe fn jffs2_write_dirent(c: *mut jffs2_sb_info, f: *mut jffs2_inode_info,
                                 rd: *mut jffs2_raw_dirent, name: *const u8,
                                 namelen: u32, alloc_mode: i32) -> *mut jffs2_full_dirent {
    let mut retlen = 0usize; let mut retried = 0; let mut ret;
    let mut vecs = [kvec { iov_base: rd as *mut _, iov_len: core::mem::size_of::<jffs2_raw_dirent>() }, kvec { iov_base: name as *mut _, iov_len: namelen as usize }];
    let fd = jffs2_alloc_full_dirent(namelen + 1); if fd.is_null() { return ERR_PTR(-ENOMEM); }
    (*fd).version = je32_to_cpu((*rd).version); (*fd).ino = je32_to_cpu((*rd).ino);
    (*fd).nhash = full_name_hash(core::ptr::null_mut(), name, namelen); (*fd).type_ = (*rd).type_;
    core::ptr::copy_nonoverlapping(name, (*fd).name.as_mut_ptr(), namelen as usize); (*fd).name[namelen as usize] = 0;
    'retry: loop {
        let flash_ofs = write_ofs(c); jffs2_dbg_prewrite_paranoia_check(c, flash_ofs, vecs[0].iov_len + vecs[1].iov_len);
        if alloc_mode != ALLOC_GC && je32_to_cpu((*rd).version) < (*f).highest_version { BUG_ON(retried == 0); (*f).highest_version += 1; (*rd).version = cpu_to_je32((*f).highest_version); (*fd).version = (*f).version; (*rd).node_crc = cpu_to_je32(crc32(0, rd as *const _, core::mem::size_of::<jffs2_raw_dirent>() - 8)); }
        ret = jffs2_flash_writev(c, vecs.as_mut_ptr(), 2, flash_ofs, &mut retlen, if alloc_mode == ALLOC_GC { 0 } else { je32_to_cpu((*rd).pino) });
        if ret != 0 || retlen != core::mem::size_of::<jffs2_raw_dirent>() + namelen as usize {
            if retlen != 0 { jffs2_add_physical_node_ref(c, flash_ofs | REF_OBSOLETE, PAD(core::mem::size_of::<jffs2_raw_dirent>() + namelen as usize), core::ptr::null_mut()); }
            if retried == 0 { let mut dummy=0; retried=1; if alloc_mode == ALLOC_GC { ret=jffs2_reserve_space_gc(c, core::mem::size_of::<jffs2_raw_dirent>() as u32+namelen, &mut dummy, JFFS2_SUMMARY_DIRENT_SIZE(namelen)); } else { mutex_unlock(&mut (*f).sem); jffs2_complete_reservation(c); ret=jffs2_reserve_space(c, core::mem::size_of::<jffs2_raw_dirent>() as u32+namelen, &mut dummy, alloc_mode, JFFS2_SUMMARY_DIRENT_SIZE(namelen)); mutex_lock(&mut (*f).sem); } if ret==0 { continue 'retry; } }
            jffs2_free_full_dirent(fd); return ERR_PTR(if ret!=0 {ret} else {-EIO});
        }
        (*fd).raw = jffs2_add_physical_node_ref(c, flash_ofs | dirent_node_state(rd), PAD(core::mem::size_of::<jffs2_raw_dirent>() + namelen as usize), (*f).inocache);
        if IS_ERR((*fd).raw) { let e=(*fd).raw; jffs2_free_full_dirent(fd); return ERR_CAST(e); }
        if retried != 0 { jffs2_dbg_acct_sanity_check(c, core::ptr::null_mut()); } return fd;
    }
}

pub unsafe fn jffs2_write_inode_range(c:*mut jffs2_sb_info,f:*mut jffs2_inode_info,ri:*mut jffs2_raw_inode,mut buf:*mut u8,mut offset:u32,mut writelen:u32,retlen:*mut u32)->i32{
 let mut ret=0;let mut written=0;while writelen!=0{let mut alloc=0;ret=jffs2_reserve_space(c,core::mem::size_of::<jffs2_raw_inode>() as u32+JFFS2_MIN_DATA_LEN,&mut alloc,ALLOC_NORMAL,JFFS2_SUMMARY_INODE_SIZE);if ret!=0{break}mutex_lock(&mut(*f).sem);let mut datalen=core::cmp::min(writelen,PAGE_SIZE-(offset&(PAGE_SIZE-1)));let mut cdatalen=core::cmp::min(alloc-core::mem::size_of::<jffs2_raw_inode>() as u32,datalen);let mut compr:*mut u8=core::ptr::null_mut();let typ=jffs2_compress(c,f,buf,&mut compr,&mut datalen,&mut cdatalen);(*ri).magic=cpu_to_je16(JFFS2_MAGIC_BITMASK);(*ri).nodetype=cpu_to_je16(JFFS2_NODETYPE_INODE);(*ri).totlen=cpu_to_je32(core::mem::size_of::<jffs2_raw_inode>() as u32+cdatalen);(*ri).hdr_crc=cpu_to_je32(crc32(0,ri as*const _,core::mem::size_of::<jffs2_unknown_node>()-4));(*ri).ino=cpu_to_je32((*f).inocache.ino);(*f).highest_version+=1;(*ri).version=cpu_to_je32((*f).highest_version);(*ri).isize=cpu_to_je32(core::cmp::max(je32_to_cpu((*ri).isize),offset+datalen));(*ri).offset=cpu_to_je32(offset);(*ri).csize=cpu_to_je32(cdatalen);(*ri).dsize=cpu_to_je32(datalen);(*ri).compr=typ&0xff;(*ri).usercompr=(typ>>8)&0xff;(*ri).node_crc=cpu_to_je32(crc32(0,ri as*const _,core::mem::size_of::<jffs2_raw_inode>()-8));(*ri).data_crc=cpu_to_je32(crc32(0,compr as*const _,cdatalen));let fnn=jffs2_write_dnode(c,f,ri,compr,cdatalen,ALLOC_NORETRY);jffs2_free_comprbuf(compr,buf);if IS_ERR(fnn){ret=PTR_ERR(fnn);mutex_unlock(&mut(*f).sem);jffs2_complete_reservation(c);break}ret=jffs2_add_full_dnode_to_inode(c,f,fnn);if !(*f).metadata.is_null(){jffs2_mark_node_obsolete(c,(*f).metadata.raw);jffs2_free_full_dnode((*f).metadata);(*f).metadata=core::ptr::null_mut()}mutex_unlock(&mut(*f).sem);jffs2_complete_reservation(c);if ret!=0{break}if datalen==0{ret=-EIO;break}written+=datalen;offset+=datalen;writelen-=datalen;buf=buf.add(datalen as usize)}*retlen=written;ret
}

pub unsafe fn jffs2_do_create(c:*mut jffs2_sb_info,dir_f:*mut jffs2_inode_info,f:*mut jffs2_inode_info,ri:*mut jffs2_raw_inode,qstr:*const qstr)->i32{let mut a=0;let mut r=jffs2_reserve_space(c,core::mem::size_of::<jffs2_raw_inode>() as u32,&mut a,ALLOC_NORMAL,JFFS2_SUMMARY_INODE_SIZE);if r!=0{return r}mutex_lock(&mut(*f).sem);(*ri).data_crc=cpu_to_je32(0);(*ri).node_crc=cpu_to_je32(crc32(0,ri as*const _,core::mem::size_of::<jffs2_raw_inode>()-8));let fnn=jffs2_write_dnode(c,f,ri,core::ptr::null(),0,ALLOC_NORMAL);if IS_ERR(fnn){mutex_unlock(&mut(*f).sem);jffs2_complete_reservation(c);return PTR_ERR(fnn)}(*f).metadata=fnn;mutex_unlock(&mut(*f).sem);jffs2_complete_reservation(c);r=jffs2_init_security(&mut(*f).vfs_inode,&mut(*dir_f).vfs_inode,qstr);if r!=0{return r}r=jffs2_init_acl_post(&mut(*f).vfs_inode);if r!=0{return r}r=jffs2_reserve_space(c,core::mem::size_of::<jffs2_raw_dirent>() as u32+(*qstr).len,&mut a,ALLOC_NORMAL,JFFS2_SUMMARY_DIRENT_SIZE((*qstr).len));if r!=0{return r}let rd=jffs2_alloc_raw_dirent();if rd.is_null(){jffs2_complete_reservation(c);return -ENOMEM}mutex_lock(&mut(*dir_f).sem);(*rd).magic=cpu_to_je16(JFFS2_MAGIC_BITMASK);(*rd).nodetype=cpu_to_je16(JFFS2_NODETYPE_DIRENT);(*rd).totlen=cpu_to_je32(core::mem::size_of::<jffs2_raw_dirent>() as u32+(*qstr).len);(*rd).pino=cpu_to_je32((*dir_f).inocache.ino);(*dir_f).highest_version+=1;(*rd).version=cpu_to_je32((*dir_f).highest_version);(*rd).ino=(*ri).ino;(*rd).nsize=(*qstr).len;(*rd).type_=DT_REG;(*rd).name_crc=cpu_to_je32(crc32(0,(*qstr).name,(*qstr).len));let fd=jffs2_write_dirent(c,dir_f,rd,(*qstr).name,(*qstr).len,ALLOC_NORMAL);jffs2_free_raw_dirent(rd);if IS_ERR(fd){jffs2_complete_reservation(c);mutex_unlock(&mut(*dir_f).sem);return PTR_ERR(fd)}jffs2_add_fd_to_list(c,fd,&mut(*dir_f).dents);jffs2_complete_reservation(c);mutex_unlock(&mut(*dir_f).sem);0}

pub unsafe fn jffs2_do_unlink(c:*mut jffs2_sb_info,dir_f:*mut jffs2_inode_info,name:*const i8,namelen:i32,dead_f:*mut jffs2_inode_info,_time:u32)->i32{
 if !jffs2_can_mark_obsolete(c){let rd=jffs2_alloc_raw_dirent();if rd.is_null(){return -ENOMEM}let mut a=0;let r=jffs2_reserve_space(c,core::mem::size_of::<jffs2_raw_dirent>() as u32+namelen as u32,&mut a,ALLOC_DELETION,JFFS2_SUMMARY_DIRENT_SIZE(namelen as u32));if r!=0{jffs2_free_raw_dirent(rd);return r}mutex_lock(&mut(*dir_f).sem);(*rd).magic=cpu_to_je16(JFFS2_MAGIC_BITMASK);(*rd).nodetype=cpu_to_je16(JFFS2_NODETYPE_DIRENT);(*rd).pino=cpu_to_je32((*dir_f).inocache.ino);(*dir_f).highest_version+=1;(*rd).version=cpu_to_je32((*dir_f).highest_version);(*rd).nsize=namelen as u32;(*rd).type_=DT_UNKNOWN;let fd=jffs2_write_dirent(c,dir_f,rd,name as*const u8,namelen as u32,ALLOC_DELETION);jffs2_free_raw_dirent(rd);if IS_ERR(fd){jffs2_complete_reservation(c);mutex_unlock(&mut(*dir_f).sem);return PTR_ERR(fd)}jffs2_add_fd_to_list(c,fd,&mut(*dir_f).dents);mutex_unlock(&mut(*dir_f).sem)}else{mutex_lock(&mut(*c).alloc_sem);mutex_lock(&mut(*dir_f).sem);let h=full_name_hash(core::ptr::null_mut(),name as*const u8,namelen as u32);let mut fd=(*dir_f).dents;while !fd.is_null(){if(*fd).nhash==h&&(*fd).ino!=0&&core::slice::from_raw_parts((*fd).name.as_ptr(),namelen as usize)==core::slice::from_raw_parts(name as*const u8,namelen as usize){jffs2_mark_node_obsolete(c,(*fd).raw);(*fd).raw=core::ptr::null_mut();(*fd).ino=0;break}fd=(*fd).next}mutex_unlock(&mut(*dir_f).sem)}if !dead_f.is_null(){(*dead_f).inocache.pino_nlink=(*dead_f).inocache.pino_nlink.wrapping_sub(1)}jffs2_complete_reservation(c);0}

pub unsafe fn jffs2_do_link(c:*mut jffs2_sb_info,dir_f:*mut jffs2_inode_info,ino:u32,type_:u8,name:*const i8,namelen:i32,time:u32)->i32{let rd=jffs2_alloc_raw_dirent();if rd.is_null(){return -ENOMEM}let mut a=0;let r=jffs2_reserve_space(c,core::mem::size_of::<jffs2_raw_dirent>() as u32+namelen as u32,&mut a,ALLOC_NORMAL,JFFS2_SUMMARY_DIRENT_SIZE(namelen as u32));if r!=0{jffs2_free_raw_dirent(rd);return r}mutex_lock(&mut(*dir_f).sem);(*rd).magic=cpu_to_je16(JFFS2_MAGIC_BITMASK);(*rd).nodetype=cpu_to_je16(JFFS2_NODETYPE_DIRENT);(*rd).pino=cpu_to_je32((*dir_f).inocache.ino);(*dir_f).highest_version+=1;(*rd).version=cpu_to_je32((*dir_f).highest_version);(*rd).ino=cpu_to_je32(ino);(*rd).mctime=cpu_to_je32(time);(*rd).nsize=namelen as u32;(*rd).type_=type_;let fd=jffs2_write_dirent(c,dir_f,rd,name as*const u8,namelen as u32,ALLOC_NORMAL);jffs2_free_raw_dirent(rd);if IS_ERR(fd){jffs2_complete_reservation(c);mutex_unlock(&mut(*dir_f).sem);return PTR_ERR(fd)}jffs2_add_fd_to_list(c,fd,&mut(*dir_f).dents);jffs2_complete_reservation(c);mutex_unlock(&mut(*dir_f).sem);0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
