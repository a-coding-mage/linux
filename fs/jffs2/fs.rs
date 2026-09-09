/* Direct low-level translation of fs.c. Kernel/project symbols are supplied externally. */

static mut JFFS2_FLASH_SETUP: Option<unsafe extern "C" fn(*mut jffs2_sb_info) -> i32> = None;

pub unsafe extern "C" fn jffs2_do_setattr(inode: *mut inode, iattr: *mut iattr) -> i32 {
    let mut old_metadata: *mut jffs2_full_dnode;
    let new_metadata: *mut jffs2_full_dnode;
    let f = JFFS2_INODE_INFO(inode);
    let c = JFFS2_SB_INFO((*inode).i_sb);
    let ri: *mut jffs2_raw_inode;
    let mut dev: jffs2_device_node;
    let mut mdata: *mut u8 = core::ptr::null_mut();
    let mut mdatalen: i32 = 0;
    let ivalid: u32;
    let mut alloclen: u32 = 0;
    let ret: i32;
    let mut alloc_type = ALLOC_NORMAL;

    jffs2_dbg(1, "%s(): ino #%llu\n", __func__, (*inode).i_ino);
    if S_ISBLK((*inode).i_mode) || S_ISCHR((*inode).i_mode) {
        mdatalen = jffs2_encode_dev(&mut dev, (*inode).i_rdev);
        mdata = &mut dev as *mut _ as *mut u8;
        jffs2_dbg(1, "%s(): Writing %d bytes of kdev_t\n", __func__, mdatalen);
    } else if S_ISLNK((*inode).i_mode) {
        mutex_lock(&mut (*f).sem);
        mdatalen = (*(*f).metadata).size as i32;
        mdata = kmalloc((*(*f).metadata).size, GFP_USER);
        if mdata.is_null() { mutex_unlock(&mut (*f).sem); return -ENOMEM; }
        ret = jffs2_read_dnode(c, f, (*f).metadata, mdata, 0, mdatalen);
        if ret != 0 { mutex_unlock(&mut (*f).sem); kfree(mdata); return ret; }
        mutex_unlock(&mut (*f).sem);
        jffs2_dbg(1, "%s(): Writing %d bytes of symlink target\n", __func__, mdatalen);
    }
    ri = jffs2_alloc_raw_inode();
    if ri.is_null() { if S_ISLNK((*inode).i_mode) { kfree(mdata); } return -ENOMEM; }
    ret = jffs2_reserve_space(c, core::mem::size_of::<jffs2_raw_inode>() as i32 + mdatalen, &mut alloclen, ALLOC_NORMAL, JFFS2_SUMMARY_INODE_SIZE);
    if ret != 0 { jffs2_free_raw_inode(ri); if S_ISLNK((*inode).i_mode) { kfree(mdata); } return ret; }
    mutex_lock(&mut (*f).sem);
    ivalid = (*iattr).ia_valid;
    (*ri).magic = cpu_to_je16(JFFS2_MAGIC_BITMASK); (*ri).nodetype = cpu_to_je16(JFFS2_NODETYPE_INODE);
    (*ri).totlen = cpu_to_je32(core::mem::size_of::<jffs2_raw_inode>() as u32 + mdatalen as u32);
    (*ri).hdr_crc = cpu_to_je32(crc32(0, ri as *const u8, core::mem::size_of::<jffs2_unknown_node>() - 4));
    (*ri).ino = cpu_to_je32((*inode).i_ino); (*ri).version = cpu_to_je32({ (*f).highest_version += 1; (*f).highest_version });
    (*ri).uid = cpu_to_je16(if ivalid & ATTR_UID != 0 { from_kuid(&init_user_ns, (*iattr).ia_uid) } else { i_uid_read(inode) });
    (*ri).gid = cpu_to_je16(if ivalid & ATTR_GID != 0 { from_kgid(&init_user_ns, (*iattr).ia_gid) } else { i_gid_read(inode) });
    (*ri).mode = cpu_to_jemode(if ivalid & ATTR_MODE != 0 { (*iattr).ia_mode } else { (*inode).i_mode });
    (*ri).isize = cpu_to_je32(if ivalid & ATTR_SIZE != 0 { (*iattr).ia_size } else { (*inode).i_size });
    (*ri).atime = cpu_to_je32(I_SEC(if ivalid & ATTR_ATIME != 0 { (*iattr).ia_atime } else { inode_get_atime(inode) }));
    (*ri).mtime = cpu_to_je32(I_SEC(if ivalid & ATTR_MTIME != 0 { (*iattr).ia_mtime } else { inode_get_mtime(inode) }));
    (*ri).ctime = cpu_to_je32(I_SEC(if ivalid & ATTR_CTIME != 0 { (*iattr).ia_ctime } else { inode_get_ctime(inode) }));
    (*ri).offset = cpu_to_je32(0); (*ri).csize = cpu_to_je32(mdatalen as u32); (*ri).dsize = cpu_to_je32(mdatalen as u32); (*ri).compr = JFFS2_COMPR_NONE;
    if ivalid & ATTR_SIZE != 0 && (*inode).i_size < (*iattr).ia_size { (*ri).compr = JFFS2_COMPR_ZERO; (*ri).dsize = cpu_to_je32((*iattr).ia_size - (*inode).i_size); (*ri).offset = cpu_to_je32((*inode).i_size); }
    else if ivalid & ATTR_SIZE != 0 && (*iattr).ia_size == 0 { alloc_type = ALLOC_DELETION; }
    (*ri).node_crc = cpu_to_je32(crc32(0, ri as *const u8, core::mem::size_of::<jffs2_raw_inode>() - 8));
    (*ri).data_crc = cpu_to_je32(if mdatalen != 0 { crc32(0, mdata, mdatalen) } else { 0 });
    new_metadata = jffs2_write_dnode(c, f, ri, mdata, mdatalen, alloc_type);
    if S_ISLNK((*inode).i_mode) { kfree(mdata); }
    if IS_ERR(new_metadata) { jffs2_complete_reservation(c); jffs2_free_raw_inode(ri); mutex_unlock(&mut (*f).sem); return PTR_ERR(new_metadata); }
    inode_set_atime_to_ts(inode, ITIME(je32_to_cpu((*ri).atime))); inode_set_ctime_to_ts(inode, ITIME(je32_to_cpu((*ri).ctime))); inode_set_mtime_to_ts(inode, ITIME(je32_to_cpu((*ri).mtime)));
    (*inode).i_mode = jemode_to_cpu((*ri).mode); i_uid_write(inode, je16_to_cpu((*ri).uid)); i_gid_write(inode, je16_to_cpu((*ri).gid));
    old_metadata = (*f).metadata;
    if ivalid & ATTR_SIZE != 0 && (*inode).i_size > (*iattr).ia_size { jffs2_truncate_fragtree(c, &mut (*f).fragtree, (*iattr).ia_size); }
    if ivalid & ATTR_SIZE != 0 && (*inode).i_size < (*iattr).ia_size { jffs2_add_full_dnode_to_inode(c, f, new_metadata); (*inode).i_size = (*iattr).ia_size; (*inode).i_blocks = ((*inode).i_size + 511) >> 9; (*f).metadata = core::ptr::null_mut(); } else { (*f).metadata = new_metadata; }
    if !old_metadata.is_null() { jffs2_mark_node_obsolete(c, (*old_metadata).raw); jffs2_free_full_dnode(old_metadata); }
    jffs2_free_raw_inode(ri); mutex_unlock(&mut (*f).sem); jffs2_complete_reservation(c);
    if ivalid & ATTR_SIZE != 0 && (*inode).i_size > (*iattr).ia_size { truncate_setsize(inode, (*iattr).ia_size); (*inode).i_blocks = ((*inode).i_size + 511) >> 9; }
    0
}

pub unsafe extern "C" fn jffs2_setattr(_idmap: *mut mnt_idmap, dentry: *mut dentry, iattr: *mut iattr) -> i32 { let inode = d_inode(dentry); let mut rc = setattr_prepare(&nop_mnt_idmap, dentry, iattr); if rc != 0 { return rc; } rc = jffs2_do_setattr(inode, iattr); if rc == 0 && (*iattr).ia_valid & ATTR_MODE != 0 { rc = posix_acl_chmod(&nop_mnt_idmap, dentry, (*inode).i_mode); } rc }

pub unsafe extern "C" fn calculate_inocache_hashsize(flash_size: u32) -> i32 { let size_mb = (flash_size / 1024 / 1024) as i32; let hashsize = (size_mb * 2) & !0x3f; if hashsize < INOCACHE_HASHSIZE_MIN { INOCACHE_HASHSIZE_MIN } else if hashsize > INOCACHE_HASHSIZE_MAX { INOCACHE_HASHSIZE_MAX } else { hashsize } }

pub unsafe extern "C" fn jffs2_statfs(dentry: *mut dentry, buf: *mut kstatfs) -> i32 { let c = JFFS2_SB_INFO((*dentry).d_sb); (*buf).f_type=JFFS2_SUPER_MAGIC; (*buf).f_bsize=1<<PAGE_SHIFT; (*buf).f_blocks=(*c).flash_size>>PAGE_SHIFT; (*buf).f_files=0; (*buf).f_ffree=0; (*buf).f_namelen=JFFS2_MAX_NAME_LEN; (*buf).f_fsid.val[0]=JFFS2_SUPER_MAGIC; (*buf).f_fsid.val[1]=(*c).mtd.index; spin_lock(&mut (*c).erase_completion_lock); let mut avail=(*c).dirty_size+(*c).free_size; if avail>(*c).sector_size*(*c).resv_blocks_write { avail-=(*c).sector_size*(*c).resv_blocks_write; } else { avail=0; } spin_unlock(&mut (*c).erase_completion_lock); (*buf).f_bavail=avail>>PAGE_SHIFT; (*buf).f_bfree=avail>>PAGE_SHIFT; 0 }

pub unsafe extern "C" fn jffs2_evict_inode(inode:*mut inode) { let c=JFFS2_SB_INFO((*inode).i_sb); let f=JFFS2_INODE_INFO(inode); truncate_inode_pages_final(&mut (*inode).i_data); clear_inode(inode); jffs2_do_clear_inode(c,f); }
pub unsafe extern "C" fn jffs2_dirty_inode(inode:*mut inode, _flags:i32) { if inode_state_read_once(inode)&I_DIRTY_DATASYNC==0 { return; } let mut a:iattr=core::mem::zeroed(); a.ia_valid=ATTR_MODE|ATTR_UID|ATTR_GID|ATTR_ATIME|ATTR_MTIME|ATTR_CTIME; a.ia_mode=(*inode).i_mode; a.ia_uid=(*inode).i_uid; a.ia_gid=(*inode).i_gid; a.ia_atime=inode_get_atime(inode); a.ia_mtime=inode_get_mtime(inode); a.ia_ctime=inode_get_ctime(inode); jffs2_do_setattr(inode,&mut a); }
pub unsafe extern "C" fn jffs2_gc_release_inode(_c:*mut jffs2_sb_info,f:*mut jffs2_inode_info){ iput(OFNI_EDONI_2SFFJ(f)); }
pub unsafe extern "C" fn jffs2_flash_cleanup(c:*mut jffs2_sb_info){ if jffs2_cleanmarker_oob(c){jffs2_nand_flash_cleanup(c);} if jffs2_dataflash(c){jffs2_dataflash_cleanup(c);} if jffs2_nor_wbuf_flash(c){jffs2_nor_wbuf_flash_cleanup(c);} if jffs2_ubivol(c){jffs2_ubivol_cleanup(c);} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
