// SPDX-License-Identifier: GPL-2.0-only
/* fs/fat/nfs.c */

// Dependencies are supplied by the surrounding kernel translation.

#[repr(C)]
struct fat_fid {
    i_gen: u32,
    i_pos_low: u32,
    i_pos_hi: u16,
    parent_i_pos_hi: u16,
    parent_i_pos_low: u32,
    parent_i_gen: u32,
}

const FAT_FID_SIZE_WITHOUT_PARENT: usize = 3;
const FAT_FID_SIZE_WITH_PARENT: usize = core::mem::size_of::<fat_fid>() / core::mem::size_of::<u32>();

/* Look up a directory inode given its starting cluster. */
unsafe fn fat_dget(sb: *mut super_block, i_logstart: i32) -> *mut inode {
    let sbi = MSDOS_SB(sb);
    let mut head: *mut hlist_head;
    let mut inode: *mut inode = core::ptr::null_mut();

    head = (*sbi).dir_hashtable.add(fat_dir_hash(i_logstart) as usize);
    spin_lock(&mut (*sbi).dir_hash_lock);
    hlist_for_each_entry!(i, head, i_dir_hash, {
        BUG_ON((*i).vfs_inode.i_sb != sb);
        if (*i).i_logstart != i_logstart { continue; }
        inode = igrab(&mut (*i).vfs_inode);
        if !inode.is_null() { break; }
    });
    spin_unlock(&mut (*sbi).dir_hash_lock);
    inode
}

unsafe fn fat_ilookup(sb: *mut super_block, ino: u64, i_pos: loff_t) -> *mut inode {
    if (*MSDOS_SB(sb)).options.nfs == FAT_NFS_NOSTALE_RO {
        fat_iget(sb, i_pos)
    } else {
        if ino < MSDOS_ROOT_INO || ino == MSDOS_FSINFO_INO { return core::ptr::null_mut(); }
        ilookup(sb, ino)
    }
}

unsafe fn __fat_nfs_get_inode(sb: *mut super_block, ino: u64, generation: u32, i_pos: loff_t) -> *mut inode {
    let mut inode = fat_ilookup(sb, ino, i_pos);
    if !inode.is_null() && generation != 0 && (*inode).i_generation != generation {
        iput(inode); inode = core::ptr::null_mut();
    }
    if inode.is_null() && (*MSDOS_SB(sb)).options.nfs == FAT_NFS_NOSTALE_RO {
        let mut bh: *mut buffer_head = core::ptr::null_mut();
        let mut blocknr: sector_t = 0;
        let mut offset: i32 = 0;
        fat_get_blknr_offset(MSDOS_SB(sb), i_pos, &mut blocknr, &mut offset);
        bh = sb_bread(sb, blocknr);
        if bh.is_null() {
            fat_msg(sb, KERN_ERR, "unable to read block(%llu) for building NFS inode", blocknr);
            return inode;
        }
        let de = (*bh).b_data as *mut msdos_dir_entry;
        if IS_FREE((*de.add(offset as usize)).name) { inode = core::ptr::null_mut(); }
        else { inode = fat_build_inode(sb, de.add(offset as usize), i_pos); }
        brelse(bh);
    }
    inode
}

unsafe fn fat_nfs_get_inode(sb: *mut super_block, ino: u64, generation: u32) -> *mut inode {
    __fat_nfs_get_inode(sb, ino, generation, 0)
}

unsafe fn fat_encode_fh_nostale(inode: *mut inode, fh: *mut __u32, lenp: *mut i32, parent: *mut inode) -> i32 {
    let mut len = *lenp;
    let sbi = MSDOS_SB((*inode).i_sb);
    let fid = fh as *mut fat_fid;
    let mut i_pos: loff_t;
    let mut typ = FILEID_FAT_WITHOUT_PARENT;
    if !parent.is_null() {
        if len < FAT_FID_SIZE_WITH_PARENT as i32 { *lenp = FAT_FID_SIZE_WITH_PARENT as i32; return FILEID_INVALID; }
    } else if len < FAT_FID_SIZE_WITHOUT_PARENT as i32 { *lenp = FAT_FID_SIZE_WITHOUT_PARENT as i32; return FILEID_INVALID; }
    i_pos = fat_i_pos_read(sbi, inode); *lenp = FAT_FID_SIZE_WITHOUT_PARENT as i32;
    (*fid).i_gen = (*inode).i_generation; (*fid).i_pos_low = i_pos as u32; (*fid).i_pos_hi = (i_pos >> 32) as u16;
    if !parent.is_null() { i_pos = fat_i_pos_read(sbi, parent); (*fid).parent_i_pos_hi = (i_pos >> 32) as u16; (*fid).parent_i_pos_low = i_pos as u32; (*fid).parent_i_gen = (*parent).i_generation; typ = FILEID_FAT_WITH_PARENT; *lenp = FAT_FID_SIZE_WITH_PARENT as i32; }
    else { (*fid).parent_i_pos_hi = 0; }
    typ
}

unsafe fn fat_fh_to_dentry(sb: *mut super_block, fid: *mut fid, fh_len: i32, fh_type: i32) -> *mut dentry { generic_fh_to_dentry(sb, fid, fh_len, fh_type, fat_nfs_get_inode) }
unsafe fn fat_fh_to_parent(sb: *mut super_block, fid: *mut fid, fh_len: i32, fh_type: i32) -> *mut dentry { generic_fh_to_parent(sb, fid, fh_len, fh_type, fat_nfs_get_inode) }

unsafe fn fat_fh_to_dentry_nostale(sb: *mut super_block, fh: *mut fid, fh_len: i32, fh_type: i32) -> *mut dentry {
    let f = fh as *mut fat_fid;
    if (fh_type == FILEID_FAT_WITHOUT_PARENT && fh_len < FAT_FID_SIZE_WITHOUT_PARENT as i32) || (fh_type == FILEID_FAT_WITH_PARENT && fh_len < FAT_FID_SIZE_WITH_PARENT as i32) || (fh_type != FILEID_FAT_WITHOUT_PARENT && fh_type != FILEID_FAT_WITH_PARENT) { return core::ptr::null_mut(); }
    let i_pos = (((*f).i_pos_hi as loff_t) << 32) | (*f).i_pos_low as loff_t;
    d_obtain_alias(__fat_nfs_get_inode(sb, 0, (*f).i_gen, i_pos))
}

unsafe fn fat_fh_to_parent_nostale(sb: *mut super_block, fh: *mut fid, fh_len: i32, fh_type: i32) -> *mut dentry {
    if fh_len < FAT_FID_SIZE_WITH_PARENT as i32 { return core::ptr::null_mut(); }
    let f = fh as *mut fat_fid;
    let mut inode: *mut inode = core::ptr::null_mut();
    if fh_type == FILEID_FAT_WITH_PARENT { let i_pos = (((*f).parent_i_pos_hi as loff_t) << 32) | (*f).parent_i_pos_low as loff_t; inode = __fat_nfs_get_inode(sb, 0, (*f).parent_i_gen, i_pos); }
    d_obtain_alias(inode)
}

unsafe fn fat_rebuild_parent(sb: *mut super_block, parent_logstart: i32) -> *mut inode {
    let sbi = MSDOS_SB(sb); let blknr = fat_clus_to_blknr(sbi, parent_logstart); let parent_bh = sb_bread(sb, blknr); if parent_bh.is_null() { fat_msg(sb, KERN_ERR, "unable to read cluster of parent directory"); return core::ptr::null_mut(); }
    let de = (*parent_bh).b_data as *mut msdos_dir_entry; let clus_to_match = fat_get_start(sbi, de); let search_clus = fat_get_start(sbi, de.add(1)); let mut dummy = fat_dget(sb, search_clus);
    if dummy.is_null() { dummy = new_inode(sb); if dummy.is_null() { brelse(parent_bh); return core::ptr::null_mut(); } (*dummy).i_ino = iunique(sb, MSDOS_ROOT_INO); fat_fill_inode(dummy, de.add(1)); (*MSDOS_I(dummy)).i_pos = -1; }
    let mut parent = core::ptr::null_mut(); let mut sinfo: fat_slot_info = core::mem::zeroed(); if !fat_scan_logstart(dummy, clus_to_match, &mut sinfo) { parent = fat_build_inode(sb, sinfo.de, sinfo.i_pos); brelse(sinfo.bh); } brelse(parent_bh); iput(dummy); parent
}

unsafe fn fat_get_parent(child_dir: *mut dentry) -> *mut dentry {
    let sb = (*child_dir).d_sb; let mut bh = core::ptr::null_mut(); let mut de: *mut msdos_dir_entry = core::ptr::null_mut(); let mut parent_inode = core::ptr::null_mut(); let sbi = MSDOS_SB(sb);
    if !fat_get_dotdot_entry(d_inode(child_dir), &mut bh, &mut de) { let p = fat_get_start(sbi, de); parent_inode = fat_dget(sb, p); if parent_inode.is_null() && (*sbi).options.nfs == FAT_NFS_NOSTALE_RO { parent_inode = fat_rebuild_parent(sb, p); } } brelse(bh); d_obtain_alias(parent_inode)
}

const fat_export_ops: export_operations = export_operations { encode_fh: Some(generic_encode_ino32_fh), fh_to_dentry: Some(fat_fh_to_dentry), fh_to_parent: Some(fat_fh_to_parent), get_parent: Some(fat_get_parent) };
const fat_export_ops_nostale: export_operations = export_operations { encode_fh: Some(fat_encode_fh_nostale), fh_to_dentry: Some(fat_fh_to_dentry_nostale), fh_to_parent: Some(fat_fh_to_parent_nostale), get_parent: Some(fat_get_parent) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
