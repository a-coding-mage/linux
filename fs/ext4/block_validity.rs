// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/ext4/block_validity.c
 *
 * Copyright (C) 2009
 * Theodore Ts'o (tytso@mit.edu)
 *
 * Track which blocks in the filesystem are metadata blocks that
 * should never be used as data blocks by files or directories.
 */

// Kernel headers and ext4.h supply the types, constants, macros, and
// functions referenced below.

#[repr(C)]
struct ext4_system_zone {
    node: rb_node,
    start_blk: ext4_fsblk_t,
    count: c_uint,
    ino: u32,
}

static mut ext4_system_zone_cachep: *mut kmem_cache = core::ptr::null_mut();

pub unsafe fn ext4_init_system_zone() -> c_int {
    ext4_system_zone_cachep = KMEM_CACHE!(ext4_system_zone, 0);
    if ext4_system_zone_cachep.is_null() {
        return -ENOMEM;
    }
    0
}

pub unsafe fn ext4_exit_system_zone() {
    rcu_barrier();
    kmem_cache_destroy(ext4_system_zone_cachep);
}

unsafe fn can_merge(entry1: *mut ext4_system_zone, entry2: *mut ext4_system_zone) -> c_int {
    if ((*entry1).start_blk + (*entry1).count as ext4_fsblk_t) == (*entry2).start_blk
        && (*entry1).ino == (*entry2).ino
    {
        return 1;
    }
    0
}

unsafe fn release_system_zone(system_blks: *mut ext4_system_blocks) {
    let mut entry: *mut ext4_system_zone;
    let mut n: *mut ext4_system_zone;

    rbtree_postorder_for_each_entry_safe!(entry, n, &mut (*system_blks).root, node) {
        kmem_cache_free(ext4_system_zone_cachep, entry as *mut c_void);
    }
}

/*
 * Mark a range of blocks as belonging to the "system zone" --- that
 * is, filesystem metadata blocks which should never be used by
 * inodes.
 */
unsafe fn add_system_zone(
    system_blks: *mut ext4_system_blocks,
    start_blk: ext4_fsblk_t,
    count: c_uint,
    ino: u32,
) -> c_int {
    let mut new_entry: *mut ext4_system_zone;
    let mut entry: *mut ext4_system_zone;
    let mut n: *mut *mut rb_node = &mut (*system_blks).root.rb_node;
    let mut node: *mut rb_node;
    let mut parent: *mut rb_node = core::ptr::null_mut();
    let mut new_node: *mut rb_node;

    while !(*n).is_null() {
        parent = *n;
        entry = rb_entry!(parent, ext4_system_zone, node);
        if start_blk < (*entry).start_blk {
            n = &mut (**n).rb_left;
        } else if start_blk >= (*entry).start_blk + (*entry).count as ext4_fsblk_t {
            n = &mut (**n).rb_right;
        } else {
            return -EFSCORRUPTED;
        }
    }

    new_entry = kmem_cache_alloc(ext4_system_zone_cachep, GFP_KERNEL) as *mut ext4_system_zone;
    if new_entry.is_null() {
        return -ENOMEM;
    }
    (*new_entry).start_blk = start_blk;
    (*new_entry).count = count;
    (*new_entry).ino = ino;
    new_node = &mut (*new_entry).node;

    rb_link_node(new_node, parent, n);
    rb_insert_color(new_node, &mut (*system_blks).root);

    /* Can we merge to the left? */
    node = rb_prev(new_node);
    if !node.is_null() {
        entry = rb_entry!(node, ext4_system_zone, node);
        if can_merge(entry, new_entry) != 0 {
            (*new_entry).start_blk = (*entry).start_blk;
            (*new_entry).count += (*entry).count;
            rb_erase(node, &mut (*system_blks).root);
            kmem_cache_free(ext4_system_zone_cachep, entry as *mut c_void);
        }
    }

    /* Can we merge to the right? */
    node = rb_next(new_node);
    if !node.is_null() {
        entry = rb_entry!(node, ext4_system_zone, node);
        if can_merge(new_entry, entry) != 0 {
            (*new_entry).count += (*entry).count;
            rb_erase(node, &mut (*system_blks).root);
            kmem_cache_free(ext4_system_zone_cachep, entry as *mut c_void);
        }
    }
    0
}

unsafe fn debug_print_tree(sbi: *mut ext4_sb_info) {
    let mut node: *mut rb_node;
    let mut entry: *mut ext4_system_zone;
    let system_blks: *mut ext4_system_blocks;
    let mut first = 1;

    printk!(KERN_INFO "System zones: ");
    rcu_read_lock();
    system_blks = rcu_dereference!((*sbi).s_system_blks);
    node = rb_first(&mut (*system_blks).root);
    while !node.is_null() {
        entry = rb_entry!(node, ext4_system_zone, node);
        printk!(KERN_CONT "{}{}-{}", if first != 0 { "" } else { ", " }, (*entry).start_blk,
                (*entry).start_blk + (*entry).count as ext4_fsblk_t - 1);
        first = 0;
        node = rb_next(node);
    }
    rcu_read_unlock();
    printk!(KERN_CONT "\n");
}

unsafe fn ext4_protect_reserved_inode(
    sb: *mut super_block,
    system_blks: *mut ext4_system_blocks,
    ino: u32,
) -> c_int {
    let mut inode: *mut inode;
    let sbi = EXT4_SB!(sb);
    let mut map: ext4_map_blocks;
    let mut i: u32 = 0;
    let mut num: u32;
    let mut err: c_int = 0;
    let mut n: c_int;

    if ino < EXT4_ROOT_INO || ino > le32_to_cpu!((*(*sbi).s_es).s_inodes_count) {
        return -EINVAL;
    }
    inode = ext4_iget(sb, ino, EXT4_IGET_SPECIAL);
    if IS_ERR!(inode) {
        return PTR_ERR!(inode);
    }
    num = ((*inode).i_size + (*sb).s_blocksize as i64 - 1) as u64
        .wrapping_shr((*sb).s_blocksize_bits) as u32;
    while i < num {
        cond_resched();
        map.m_lblk = i;
        map.m_len = num - i;
        n = ext4_map_blocks(core::ptr::null_mut(), inode, &mut map, 0);
        if n < 0 {
            err = n;
            break;
        }
        if n == 0 {
            i += 1;
        } else {
            err = add_system_zone(system_blks, map.m_pblk, n as c_uint, ino);
            if err < 0 {
                if err == -EFSCORRUPTED {
                    EXT4_ERROR_INODE_ERR!(inode, -err, "blocks {}-{} from inode overlap system zone",
                        map.m_pblk, map.m_pblk + map.m_len as ext4_fsblk_t - 1);
                }
                break;
            }
            i += n as u32;
        }
    }
    iput(inode);
    err
}

unsafe fn ext4_destroy_system_zone(rcu: *mut rcu_head) {
    let system_blks = container_of!(rcu, ext4_system_blocks, rcu);
    release_system_zone(system_blks);
    kfree(system_blks as *mut c_void);
}

/* Build system zone rbtree which is used for block validity checking. */
pub unsafe fn ext4_setup_system_zone(sb: *mut super_block) -> c_int {
    let ngroups = ext4_get_groups_count(sb);
    let sbi = EXT4_SB!(sb);
    let mut system_blks = kzalloc_obj!(ext4_system_blocks);
    let mut gdp: *mut ext4_group_desc;
    let mut i: ext4_group_t;
    let mut ret: c_int;

    if system_blks.is_null() { return -ENOMEM; }
    i = 0;
    while i < ngroups {
        let meta_blks = ext4_num_base_meta_blocks(sb, i);
        cond_resched();
        if meta_blks != 0 {
            ret = add_system_zone(system_blks, ext4_group_first_block_no(sb, i), meta_blks, 0);
            if ret != 0 { return setup_system_zone_err(system_blks, ret); }
        }
        gdp = ext4_get_group_desc(sb, i, core::ptr::null_mut());
        ret = add_system_zone(system_blks, ext4_block_bitmap(sb, gdp), 1, 0);
        if ret != 0 { return setup_system_zone_err(system_blks, ret); }
        ret = add_system_zone(system_blks, ext4_inode_bitmap(sb, gdp), 1, 0);
        if ret != 0 { return setup_system_zone_err(system_blks, ret); }
        ret = add_system_zone(system_blks, ext4_inode_table(sb, gdp), (*sbi).s_itb_per_group, 0);
        if ret != 0 { return setup_system_zone_err(system_blks, ret); }
        i += 1;
    }
    if ext4_has_feature_journal(sb) && (*sbi).s_es.journal_inum != 0 {
        ret = ext4_protect_reserved_inode(sb, system_blks, le32_to_cpu!((*(*sbi).s_es).s_journal_inum));
        if ret != 0 { return setup_system_zone_err(system_blks, ret); }
    }
    rcu_assign_pointer!((*sbi).s_system_blks, system_blks);
    if test_opt!(sb, DEBUG) { debug_print_tree(sbi); }
    return 0;
}

unsafe fn setup_system_zone_err(system_blks: *mut ext4_system_blocks, ret: c_int) -> c_int {
    release_system_zone(system_blks);
    kfree(system_blks as *mut c_void);
    ret
}

pub unsafe fn ext4_release_system_zone(sb: *mut super_block) {
    let system_blks = rcu_dereference_protected!((*EXT4_SB!(sb)).s_system_blks,
        lockdep_is_held!(&mut (*sb).s_umount));
    rcu_assign_pointer!((*EXT4_SB!(sb)).s_system_blks, core::ptr::null_mut());
    if !system_blks.is_null() { call_rcu!(&mut (*system_blks).rcu, ext4_destroy_system_zone); }
}

pub unsafe fn ext4_sb_block_valid(
    sb: *mut super_block, inode: *mut inode, start_blk: ext4_fsblk_t, count: c_uint,
) -> c_int {
    let sbi = EXT4_SB!(sb);
    let mut ret = 1;
    if start_blk <= le32_to_cpu!((*(*sbi).s_es).s_first_data_block) as ext4_fsblk_t
        || start_blk.wrapping_add(count as ext4_fsblk_t) < start_blk
        || start_blk + count as ext4_fsblk_t > ext4_blocks_count((*sbi).s_es) { return 0; }
    rcu_read_lock();
    let system_blks = rcu_dereference!((*sbi).s_system_blks);
    if system_blks.is_null() { rcu_read_unlock(); return ret; }
    let mut n = (*system_blks).root.rb_node;
    while !n.is_null() {
        let entry = rb_entry!(n, ext4_system_zone, node);
        if start_blk + count as ext4_fsblk_t - 1 < (*entry).start_blk { n = (*n).rb_left; }
        else if start_blk >= (*entry).start_blk + (*entry).count as ext4_fsblk_t { n = (*n).rb_right; }
        else { ret = 0; if !inode.is_null() { ret = if (*entry).ino == (*inode).i_ino { 1 } else { 0 }; } break; }
    }
    rcu_read_unlock();
    ret
}

/* Returns 1 if the passed-in block region is valid; 0 on metadata overlap. */
pub unsafe fn ext4_inode_block_valid(inode: *mut inode, start_blk: ext4_fsblk_t, count: c_uint) -> c_int {
    ext4_sb_block_valid((*inode).i_sb, inode, start_blk, count)
}

pub unsafe fn ext4_check_blockref(function: *const c_char, line: c_uint, inode: *mut inode, p: *mut le32, max: c_uint) -> c_int {
    let mut bref = p;
    let journal = (*EXT4_SB!((*inode).i_sb)).s_journal;
    if !journal.is_null() && inode == (*journal).j_inode { return 0; }
    while (bref as usize) < (p as usize + max as usize * core::mem::size_of::<le32>()) {
        let blk = le32_to_cpu!(*bref);
        bref = bref.add(1);
        if blk != 0 && ext4_inode_block_valid(inode, blk as ext4_fsblk_t, 1) == 0 {
            ext4_error_inode(inode, function, line, blk as ext4_fsblk_t, c"invalid block".as_ptr());
            return -EFSCORRUPTED;
        }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
