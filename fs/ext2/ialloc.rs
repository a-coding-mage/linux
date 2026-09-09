// SPDX-License-Identifier: GPL-2.0
/* Direct Rust translation of linux/fs/ext2/ialloc.c. */

const INODE_COST: i32 = 64;
const BLOCK_COST: i32 = 256;

unsafe fn read_inode_bitmap(sb: *mut super_block, block_group: c_ulong) -> *mut buffer_head {
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let desc = ext2_get_group_desc(sb, block_group, core::ptr::null_mut());
    if desc.is_null() { return bh; }
    bh = sb_bread(sb, le32_to_cpu((*desc).bg_inode_bitmap));
    if bh.is_null() {
        ext2_error(sb, "read_inode_bitmap", "Cannot read inode bitmap - block_group = %lu, inode_bitmap = %u", block_group, le32_to_cpu((*desc).bg_inode_bitmap));
    }
    bh
}

unsafe fn ext2_release_inode(sb: *mut super_block, group: c_int, dir: c_int) {
    let mut bh: *mut buffer_head = core::ptr::null_mut();
    let desc = ext2_get_group_desc(sb, group as c_ulong, &mut bh);
    if desc.is_null() { ext2_error(sb, "ext2_release_inode", "can't get descriptor for group %d", group); return; }
    spin_lock(sb_bgl_lock(EXT2_SB(sb), group));
    le16_add_cpu(&mut (*desc).bg_free_inodes_count, 1);
    if dir != 0 { le16_add_cpu(&mut (*desc).bg_used_dirs_count, -1); }
    spin_unlock(sb_bgl_lock(EXT2_SB(sb), group));
    percpu_counter_inc(&mut (*EXT2_SB(sb)).s_freeinodes_counter);
    if dir != 0 { percpu_counter_dec(&mut (*EXT2_SB(sb)).s_dirs_counter); }
    mark_buffer_dirty(bh);
}

pub unsafe fn ext2_free_inode(inode: *mut inode) {
    let sb = (*inode).i_sb;
    let ino = (*inode).i_ino;
    ext2_debug("freeing inode %lu\n", ino);
    dquot_free_inode(inode); dquot_drop(inode);
    let es = (*EXT2_SB(sb)).s_es;
    let is_directory = (S_ISDIR((*inode).i_mode)) as c_int;
    if ino < EXT2_FIRST_INO(sb) || ino > le32_to_cpu((*es).s_inodes_count) { ext2_error(sb, "ext2_free_inode", "reserved or nonexistent inode %lu", ino); return; }
    let group = (ino - 1) / EXT2_INODES_PER_GROUP(sb);
    let bit = (ino - 1) % EXT2_INODES_PER_GROUP(sb);
    let bitmap_bh = read_inode_bitmap(sb, group);
    if bitmap_bh.is_null() { return; }
    if ext2_clear_bit_atomic(sb_bgl_lock(EXT2_SB(sb), group), bit, (*bitmap_bh).b_data as *mut c_void) == 0 { ext2_error(sb, "ext2_free_inode", "bit already cleared for inode %lu", ino); } else { ext2_release_inode(sb, group as c_int, is_directory); }
    mark_buffer_dirty(bitmap_bh);
    if (*sb).s_flags & SB_SYNCHRONOUS != 0 { sync_dirty_buffer(bitmap_bh); }
    brelse(bitmap_bh);
}

unsafe fn ext2_preread_inode(inode: *mut inode) {
    let ino = (*inode).i_ino;
    let group = (ino - 1) / EXT2_INODES_PER_GROUP((*inode).i_sb);
    let gdp = ext2_get_group_desc((*inode).i_sb, group, core::ptr::null_mut());
    if gdp.is_null() { return; }
    let offset = ((ino - 1) % EXT2_INODES_PER_GROUP((*inode).i_sb)) * EXT2_INODE_SIZE((*inode).i_sb);
    let block = le32_to_cpu((*gdp).bg_inode_table) + (offset >> EXT2_BLOCK_SIZE_BITS((*inode).i_sb));
    sb_breadahead((*inode).i_sb, block);
}

unsafe fn find_group_dir(sb: *mut super_block, _parent: *mut inode) -> c_int {
    let ngroups = (*EXT2_SB(sb)).s_groups_count;
    let avefreei = (ext2_count_free_inodes(sb) / ngroups as c_ulong) as u16;
    let mut best_group = -1;
    let mut best_desc: *mut ext2_group_desc = core::ptr::null_mut();
    for group in 0..ngroups { let desc = ext2_get_group_desc(sb, group as c_ulong, core::ptr::null_mut()); if desc.is_null() || (*desc).bg_free_inodes_count == 0 || le16_to_cpu((*desc).bg_free_inodes_count) < avefreei { continue; } if best_desc.is_null() || le16_to_cpu((*desc).bg_free_blocks_count) > le16_to_cpu((*best_desc).bg_free_blocks_count) { best_group=group; best_desc=desc; } }
    best_group
}

unsafe fn find_group_orlov(sb: *mut super_block, parent: *mut inode) -> c_int {
    let sbi = EXT2_SB(sb); let ngroups=(*sbi).s_groups_count; let inodes_per_group=EXT2_INODES_PER_GROUP(sb) as c_int;
    let freei=percpu_counter_read_positive(&mut (*sbi).s_freeinodes_counter); let avefreei=freei/ngroups; let free_blocks=percpu_counter_read_positive(&mut (*sbi).s_freeblocks_counter); let avefreeb=free_blocks/ngroups; let mut parent_group=(*EXT2_I(parent)).i_block_group;
    let mut group=-1; let mut desc;
    if parent == d_inode((*sb).s_root) || (*EXT2_I(parent)).i_flags & EXT2_TOPDIR_FL != 0 { let mut best_ndir=inodes_per_group; let mut best=-1; parent_group=get_random_u32_below(ngroups as u32) as c_int; for i in 0..ngroups { group=(parent_group+i)%ngroups; desc=ext2_get_group_desc(sb,group as c_ulong,core::ptr::null_mut()); if desc.is_null()||(*desc).bg_free_inodes_count==0||le16_to_cpu((*desc).bg_used_dirs_count) as c_int>=best_ndir||le16_to_cpu((*desc).bg_free_inodes_count) as c_int<avefreei||le16_to_cpu((*desc).bg_free_blocks_count) as c_int<avefreeb {continue;} best=group; best_ndir=le16_to_cpu((*desc).bg_used_dirs_count) as c_int;} if best>=0{return best;} }
    let ndirs=core::cmp::max(percpu_counter_read_positive(&mut (*sbi).s_dirs_counter),1); let blocks_per_dir=(le32_to_cpu((*(*sbi).s_es).s_blocks_count) as i32-free_blocks)/ndirs; let max_dirs=ndirs/ngroups+inodes_per_group/16; let min_inodes=avefreei-inodes_per_group/4; let min_blocks=avefreeb-EXT2_BLOCKS_PER_GROUP(sb)/4; let mut max_debt=EXT2_BLOCKS_PER_GROUP(sb)/core::cmp::max(blocks_per_dir,BLOCK_COST); max_debt=core::cmp::min(max_debt,inodes_per_group/INODE_COST); max_debt=core::cmp::min(max_debt,255); if max_debt==0{max_debt=1;}
    for i in 0..ngroups {group=(parent_group+i)%ngroups; desc=ext2_get_group_desc(sb,group as c_ulong,core::ptr::null_mut()); if desc.is_null()||(*desc).bg_free_inodes_count==0||(*sbi).s_debts[group as usize]>=max_debt as u8||le16_to_cpu((*desc).bg_used_dirs_count) as c_int>=max_dirs||le16_to_cpu((*desc).bg_free_inodes_count) as c_int<min_inodes||le16_to_cpu((*desc).bg_free_blocks_count) as c_int<min_blocks{continue;} return group;}
    for i in 0..ngroups {group=(parent_group+i)%ngroups; desc=ext2_get_group_desc(sb,group as c_ulong,core::ptr::null_mut()); if !desc.is_null()&&(*desc).bg_free_inodes_count!=0&&le16_to_cpu((*desc).bg_free_inodes_count) as c_int>=avefreei{return group;}}
    if avefreei!=0 { return find_group_orlov(sb,parent); } -1
}

unsafe fn find_group_other(sb:*mut super_block,parent:*mut inode)->c_int { let pg=(*EXT2_I(parent)).i_block_group; let n=(*EXT2_SB(sb)).s_groups_count; let mut g=pg; let mut d=ext2_get_group_desc(sb,g as c_ulong,core::ptr::null_mut()); if !d.is_null()&&le16_to_cpu((*d).bg_free_inodes_count)!=0&&le16_to_cpu((*d).bg_free_blocks_count)!=0{return g;} g=(g+(*parent).i_ino as c_int) % n; let mut i=1; while i<n {g+=i;if g>=n{g-=n;} d=ext2_get_group_desc(sb,g as c_ulong,core::ptr::null_mut());if !d.is_null()&&le16_to_cpu((*d).bg_free_inodes_count)!=0&&le16_to_cpu((*d).bg_free_blocks_count)!=0{return g;} i<<=1;} g=pg; for _ in 0..n {g+=1;if g>=n{g=0;}d=ext2_get_group_desc(sb,g as c_ulong,core::ptr::null_mut());if !d.is_null()&&le16_to_cpu((*d).bg_free_inodes_count)!=0{return g;}} -1 }

pub unsafe fn ext2_new_inode(dir:*mut inode,mode:umode_t,qstr:*const qstr)->*mut inode { let sb=(*dir).i_sb; let inode=new_inode(sb); if inode.is_null(){return ERR_PTR(-ENOMEM);} let sbi=EXT2_SB(sb); let es=(*sbi).s_es; let group=if S_ISDIR(mode){if test_opt(sb,OLDALLOC){find_group_dir(sb,dir)}else{find_group_orlov(sb,dir)}}else{find_group_other(sb,dir)}; if group<0{make_bad_inode(inode);iput(inode);return ERR_PTR(-ENOSPC);} let mut bh=read_inode_bitmap(sb,group as c_ulong); if bh.is_null(){make_bad_inode(inode);iput(inode);return ERR_PTR(-EIO);} let mut ino=ext2_find_next_zero_bit((*bh).b_data as *const c_ulong,EXT2_INODES_PER_GROUP(sb),0); while ino<EXT2_INODES_PER_GROUP(sb)&&ext2_set_bit_atomic(sb_bgl_lock(sbi,group as c_ulong),ino,(*bh).b_data)!=0{ino+=1;} if ino>=EXT2_INODES_PER_GROUP(sb){brelse(bh);make_bad_inode(inode);iput(inode);return ERR_PTR(-ENOSPC);} mark_buffer_dirty(bh);if (*sb).s_flags&SB_SYNCHRONOUS!=0{sync_dirty_buffer(bh);}brelse(bh);ino+=group as c_ulong*EXT2_INODES_PER_GROUP(sb)+1;if ino<EXT2_FIRST_INO(sb)||ino>le32_to_cpu((*es).s_inodes_count){make_bad_inode(inode);iput(inode);return ERR_PTR(-EIO);} percpu_counter_dec(&mut (*sbi).s_freeinodes_counter);if S_ISDIR(mode){percpu_counter_inc(&mut (*sbi).s_dirs_counter);} let gdp=ext2_get_group_desc(sb,group as c_ulong,core::ptr::null_mut());spin_lock(sb_bgl_lock(sbi,group as c_ulong));le16_add_cpu(&mut (*gdp).bg_free_inodes_count,-1);if S_ISDIR(mode){(*sbi).s_debts[group as usize]=(*sbi).s_debts[group as usize].saturating_add(1);le16_add_cpu(&mut (*gdp).bg_used_dirs_count,1);}spin_unlock(sb_bgl_lock(sbi,group as c_ulong));(*inode).i_ino=ino;(*inode).i_blocks=0;(*EXT2_I(inode)).i_block_group=group;(*EXT2_I(inode)).i_state=EXT2_STATE_NEW;ext2_set_inode_flags(inode);if insert_inode_locked(inode)<0{make_bad_inode(inode);iput(inode);return ERR_PTR(-EIO);}if dquot_initialize(inode)!=0||dquot_alloc_inode(inode)!=0{dquot_drop(inode);(*inode).i_flags|=S_NOQUOTA;clear_nlink(inode);discard_new_inode(inode);return ERR_PTR(-EDQUOT);}if ext2_init_acl(inode,dir)!=0||ext2_init_security(inode,dir,qstr)!=0{dquot_free_inode(inode);dquot_drop(inode);clear_nlink(inode);discard_new_inode(inode);return ERR_PTR(-EIO);}mark_inode_dirty(inode);ext2_preread_inode(inode);inode}

pub unsafe fn ext2_count_free_inodes(sb:*mut super_block)->c_ulong{let mut n=0;for i in 0..(*EXT2_SB(sb)).s_groups_count{let d=ext2_get_group_desc(sb,i as c_ulong,core::ptr::null_mut());if !d.is_null(){n+=le16_to_cpu((*d).bg_free_inodes_count) as c_ulong;}}n}
pub unsafe fn ext2_count_dirs(sb:*mut super_block)->c_ulong{let mut n=0;for i in 0..(*EXT2_SB(sb)).s_groups_count{let d=ext2_get_group_desc(sb,i as c_ulong,core::ptr::null_mut());if !d.is_null(){n+=le16_to_cpu((*d).bg_used_dirs_count) as c_ulong;}}n}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
