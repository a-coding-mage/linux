// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level translation of erofs/super.c.  Linux and EROFS
 * declarations referenced here are supplied by the surrounding kernel port. */

static mut erofs_inode_cachep: *mut kmem_cache = core::ptr::null_mut();

pub unsafe extern "C" fn _erofs_printk(sb: *mut super_block, fmt: *const i8, mut args: ...) {
    let mut vaf: va_format = core::mem::zeroed();
    let level = printk_get_level(fmt);
    vaf.fmt = printk_skip_level(fmt);
    vaf.va = &mut args;
    if !sb.is_null() { printk!("{}{}erofs (device {}): %pV", KERN_SOH_ASCII, level, (*sb).s_id, &vaf); }
    else { printk!("{}{}erofs: %pV", KERN_SOH_ASCII, level, &vaf); }
}

unsafe fn erofs_superblock_csum_verify(sb: *mut super_block, sbdata: *mut core::ffi::c_void) -> i32 {
    let dsb = (sbdata as *mut u8).add(EROFS_SUPER_OFFSET as usize) as *mut erofs_super_block;
    let mut len: u32 = 1u32 << (*EROFS_SB(sb)).blkszbits;
    if len > EROFS_SUPER_OFFSET { len -= EROFS_SUPER_OFFSET; }
    len -= core::mem::offset_of!(erofs_super_block, checksum) as u32 + core::mem::size_of_val(&(*dsb).checksum) as u32;
    let crc = crc32c(0x5045B54A, (&(*dsb).checksum as *const _ as *const u8).add(1), len);
    if crc == le32_to_cpu((*dsb).checksum) { return 0; }
    erofs_err!(sb, "invalid checksum 0x{:08x}, 0x{:08x} expected", crc, le32_to_cpu((*dsb).checksum));
    -EBADMSG
}

unsafe extern "C" fn erofs_inode_init_once(ptr: *mut core::ffi::c_void) { inode_init_once(&mut (*(ptr as *mut erofs_inode)).vfs_inode); }

unsafe extern "C" fn erofs_alloc_inode(sb: *mut super_block) -> *mut inode {
    let vi = alloc_inode_sb(sb, erofs_inode_cachep, GFP_KERNEL);
    if vi.is_null() { return core::ptr::null_mut(); }
    memset(vi as *mut _, 0, core::mem::offset_of!(erofs_inode, vfs_inode));
    &mut (*(vi as *mut erofs_inode)).vfs_inode
}

unsafe extern "C" fn erofs_free_inode(inode: *mut inode) {
    let vi = EROFS_I(inode);
    if (*inode).i_op == &erofs_fast_symlink_iops { kfree((*inode).i_link as *mut _); }
    kfree((*vi).xattr_shared_xattrs as *mut _); kmem_cache_free(erofs_inode_cachep, vi as *mut _);
}

pub unsafe extern "C" fn erofs_read_metadata(sb: *mut super_block, buf: *mut erofs_buf, offset: *mut erofs_off_t, lengthp: *mut i32) -> *mut core::ffi::c_void {
    *offset = round_up(*offset, 4); let mut ptr = erofs_bread(buf, *offset, true);
    if IS_ERR(ptr) { return ptr as *mut _; }
    let mut len = le16_to_cpu(*(ptr as *mut __le16)) as i32; if len == 0 { len = U16_MAX as i32 + 1; }
    let buffer = kmalloc(len as usize, GFP_KERNEL) as *mut u8; if buffer.is_null() { return ERR_PTR(-ENOMEM); }
    *offset += core::mem::size_of::<__le16>() as u64; *lengthp = len; let mut i = 0;
    while i < len { let cnt = core::cmp::min((*sb).s_blocksize - erofs_blkoff(sb, *offset), len - i); ptr = erofs_bread(buf, *offset, true); if IS_ERR(ptr) { kfree(buffer as *mut _); return ptr as *mut _; } memcpy(buffer.add(i as usize) as *mut _, ptr as *const _, cnt as usize); *offset += cnt as u64; i += cnt; }
    buffer as *mut _
}

unsafe fn erofs_init_device(buf: *mut erofs_buf, sb: *mut super_block, dif: *mut erofs_device_info, pos: *mut erofs_off_t) -> i32 {
    let sbi = EROFS_SB(sb); let dis = erofs_read_metabuf(buf, sb, *pos, false); if IS_ERR(dis) { return PTR_ERR(dis); }
    if !(*sbi).devs.is_null() && !(*(*sbi).devs).flatdev && (*dif).path.is_null() { if (*dis.cast::<erofs_deviceslot>()).tag[0] == 0 { erofs_err!(sb, "empty device tag @ pos {}", *pos); return -EINVAL; } (*dif).path = kmemdup_nul((*dis.cast::<erofs_deviceslot>()).tag.as_ptr(), core::mem::size_of_val(&(*dis.cast::<erofs_deviceslot>()).tag), GFP_KERNEL); if (*dif).path.is_null() { return -ENOMEM; } }
    (*dif).blocks = le32_to_cpu((*dis.cast::<erofs_deviceslot>()).blocks_lo) as u64;
    (*dif).uniaddr = le32_to_cpu((*dis.cast::<erofs_deviceslot>()).uniaddr_lo) as u64;
    if erofs_sb_has_48bit(sbi) { (*dif).blocks |= (le16_to_cpu((*dis.cast::<erofs_deviceslot>()).blocks_hi) as u64) << 32; (*dif).uniaddr |= (le16_to_cpu((*dis.cast::<erofs_deviceslot>()).uniaddr_hi) as u64) << 32; }
    (*sbi).total_blocks += (*dif).blocks; *pos += EROFS_DEVT_SLOT_SIZE as u64; 0
}

unsafe fn erofs_scan_devices(sb: *mut super_block, dsb: *mut erofs_super_block) -> i32 {
    let sbi = EROFS_SB(sb); (*sbi).total_blocks = (*sbi).dif0.blocks; if !erofs_sb_has_device_table(sbi) { return 0; }
    let n = le16_to_cpu((*dsb).extra_devices); if n == 0 { return 0; }
    (*sbi).device_id_mask = roundup_pow_of_two(n as u32 + 1) - 1; let mut pos = le16_to_cpu((*dsb).devt_slotoff) as u64 * EROFS_DEVT_SLOT_SIZE as u64; let mut buf = __EROFS_BUF_INITIALIZER;
    down_read(&mut (*(*sbi).devs).rwsem); for id in 0..n { let dif = kzalloc_obj::<erofs_device_info>(); if dif.is_null() { up_read(&mut (*(*sbi).devs).rwsem); erofs_put_metabuf(&mut buf); return -ENOMEM; } let r = idr_alloc(&mut (*(*sbi).devs).tree, dif, 0, 0, GFP_KERNEL); if r < 0 { kfree(dif as *mut _); up_read(&mut (*(*sbi).devs).rwsem); erofs_put_metabuf(&mut buf); return r; } (*sbi).devs.extra_devices += 1; let r = erofs_init_device(&mut buf, sb, dif, &mut pos); if r != 0 { up_read(&mut (*(*sbi).devs).rwsem); erofs_put_metabuf(&mut buf); return r; } let _ = id; } up_read(&mut (*(*sbi).devs).rwsem); erofs_put_metabuf(&mut buf); 0
}

unsafe fn erofs_read_superblock(sb: *mut super_block) -> i32 {
    let sbi = EROFS_SB(sb); let mut buf = __EROFS_BUF_INITIALIZER; let data = erofs_read_metabuf(&mut buf, sb, 0, false); if IS_ERR(data) { erofs_err!(sb, "cannot read erofs superblock"); return PTR_ERR(data); }
    let dsb = (data as *mut u8).add(EROFS_SUPER_OFFSET as usize) as *mut erofs_super_block; let mut ret = -EINVAL;
    if le32_to_cpu((*dsb).magic) != EROFS_SUPER_MAGIC_V1 { erofs_err!(sb, "cannot find valid erofs superblock"); }
    else { (*sbi).blkszbits = (*dsb).blkszbits; if (*sbi).blkszbits < 9 || (*sbi).blkszbits > PAGE_SHIFT { erofs_err!(sb, "blkszbits {} isn't supported", (*sbi).blkszbits); } else if (*dsb).dirblkbits != 0 { erofs_err!(sb, "dirblkbits {} isn't supported", (*dsb).dirblkbits); } else { (*sbi).feature_compat = le32_to_cpu((*dsb).feature_compat); if erofs_sb_has_sb_chksum(sbi) { ret = erofs_superblock_csum_verify(sb, data); } if ret == 0 || !erofs_sb_has_sb_chksum(sbi) { (*sbi).feature_incompat = le32_to_cpu((*dsb).feature_incompat); (*sbi).sb_size = 128 + (*dsb).sb_extslots as u32 * EROFS_SB_EXTSLOT_SIZE; (*sbi).dif0.blocks = le32_to_cpu((*dsb).blocks_lo) as u64; (*sbi).meta_blkaddr = le32_to_cpu((*dsb).meta_blkaddr); (*sbi).root_nid = le16_to_cpu((*dsb).rb.rootnid_2b) as u64; (*sbi).packed_nid = le64_to_cpu((*dsb).packed_nid); (*sbi).inos = le64_to_cpu((*dsb).inos); (*sbi).epoch = le64_to_cpu((*dsb).epoch) as i64; (*sbi).fixed_nsec = le32_to_cpu((*dsb).fixed_nsec); ret = erofs_scan_devices(sb, dsb); } } erofs_put_metabuf(&mut buf); ret
}

unsafe fn erofs_default_options(sbi: *mut erofs_sb_info) { if IS_ENABLED(CONFIG_EROFS_FS_XATTR) { set_opt!(&mut (*sbi).opt, XATTR_USER); } if IS_ENABLED(CONFIG_EROFS_FS_POSIX_ACL) { set_opt!(&mut (*sbi).opt, POSIX_ACL); } }

#[repr(u32)] enum ErofsOption { Opt_user_xattr, Opt_acl, Opt_cache_strategy, Opt_dax, Opt_dax_enum, Opt_device, Opt_domain_id, Opt_directio, Opt_fsoffset, Opt_inode_share, Opt_source }

unsafe fn erofs_encode_fh(inode: *mut inode, fh: *mut u32, max_len: *mut i32, parent: *mut inode) -> i32 { let nid = (*EROFS_I(inode)).nid; let len = if parent.is_null(){3}else{6}; if *max_len < len {*max_len=len; return FILEID_INVALID;} (*fh.add(0))=(nid>>32) as u32; *fh.add(1)=nid as u32; *fh.add(2)=(*inode).i_generation; if !parent.is_null(){let pn=(*EROFS_I(parent)).nid;*fh.add(3)=(pn>>32) as u32;*fh.add(4)=pn as u32;*fh.add(5)=(*parent).i_generation;} *max_len=len; if parent.is_null(){FILEID_INO64_GEN}else{FILEID_INO64_GEN_PARENT} }

pub unsafe fn erofs_setup_managed_cache(sb: *mut super_block) -> i32 { if (*EROFS_SB(sb)).managed_cache.is_null(){let inode=new_inode(sb);if inode.is_null(){return -ENOMEM;}set_nlink(inode,1);(*inode).i_size=OFFSET_MAX;(*EROFS_SB(sb)).managed_cache=inode;}0 }

// Remaining filesystem callbacks retain the C callback topology and external kernel operations.
pub unsafe extern "C" fn erofs_statfs(dentry:*mut dentry, buf:*mut kstatfs)->i32 { (*buf).f_type=(*(*dentry).d_sb).s_magic; (*buf).f_bsize=(*(*dentry).d_sb).s_blocksize; (*buf).f_namelen=EROFS_NAME_LEN; 0 }
pub static mut erofs_sops: super_operations = super_operations { put_super: Some(erofs_put_super), alloc_inode: Some(erofs_alloc_inode), free_inode: Some(erofs_free_inode), evict_inode: Some(erofs_evict_inode), statfs: Some(erofs_statfs), show_options: Some(erofs_show_options) };

unsafe extern "C" fn erofs_evict_inode(inode:*mut inode){if IS_DAX(inode){dax_break_layout_final(inode);}erofs_ishare_free_inode(inode);truncate_inode_pages_final(&mut (*inode).i_data);clear_inode(inode);}
unsafe extern "C" fn erofs_show_options(_seq:*mut seq_file,_root:*mut dentry)->i32{0}
unsafe extern "C" fn erofs_put_super(sb:*mut super_block){let sbi=EROFS_SB(sb);erofs_unregister_sysfs(sb);erofs_shrinker_unregister(sb);erofs_xattr_prefixes_cleanup(sb);erofs_drop_internal_inodes(sbi);erofs_free_dev_context((*sbi).devs,sb);(*sbi).devs=core::ptr::null_mut();}
unsafe fn erofs_drop_internal_inodes(sbi:*mut erofs_sb_info){iput((*sbi).packed_inode);(*sbi).packed_inode=core::ptr::null_mut();iput((*sbi).metabox_inode);(*sbi).metabox_inode=core::ptr::null_mut();iput((*sbi).managed_cache);(*sbi).managed_cache=core::ptr::null_mut();}
unsafe fn erofs_free_dev_context(devs:*mut erofs_dev_context,_sb:*mut super_block){if !devs.is_null(){idr_destroy(&mut (*devs).tree);kfree(devs as *mut _);}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
