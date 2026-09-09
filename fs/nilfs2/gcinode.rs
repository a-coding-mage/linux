// SPDX-License-Identifier: GPL-2.0+
/*
 * Dummy inodes to buffer blocks for garbage collection
 *
 * Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Written by Seiji Kihara, Amagai Yoshiji, and Ryusuke Konishi.
 * Revised by Ryusuke Konishi.
 *
 */
/*
 * This file adds the cache of on-disk blocks to be moved in garbage
 * collection.  The disk blocks are held with dummy inodes (called
 * gcinodes), and this file provides lookup function of the dummy
 * inodes and their buffer read function.
 *
 * Buffers and pages held by the dummy inodes will be released each
 * time after they are copied to a new log.  Dirty blocks made on the
 * current generation and the blocks to be moved by GC never overlap
 * because the dirty blocks make a new generation; they rather must be
 * written individually.
 */

// Linux kernel and local header dependencies are supplied externally.

/*
 * nilfs_gccache_submit_read_data() - add data buffer and submit read request
 * @inode - gc inode
 * @blkoff - dummy offset treated as the key for the page cache
 * @pbn - physical block number of the block
 * @vbn - virtual block number of the block, 0 for non-virtual block
 * @out_bh - indirect pointer to a buffer_head struct to receive the results
 *
 * Description: nilfs_gccache_submit_read_data() registers the data buffer
 * specified by @pbn to the GC pagecache with the key @blkoff.
 * This function sets @vbn (@pbn if @vbn is zero) in b_blocknr of the buffer.
 *
 * Return: 0 on success, or one of the following negative error codes on
 * failure:
 * * %-EIO	- I/O error (including metadata corruption).
 * * %-ENOENT	- The block specified with @pbn does not exist.
 * * %-ENOMEM	- Insufficient memory available.
 */
pub unsafe fn nilfs_gccache_submit_read_data(
    inode: *mut inode,
    blkoff: sector_t,
    mut pbn: sector_t,
    vbn: __u64,
    out_bh: *mut *mut buffer_head,
) -> i32 {
    let bh: *mut buffer_head = nilfs_grab_buffer(inode, (*inode).i_mapping, blkoff, 0);
    if bh.is_null() {
        return -ENOMEM;
    }

    let mut err: i32;
    if buffer_uptodate(bh) {
        err = 0;
    } else {
        if pbn == 0 {
            let nilfs: *mut the_nilfs = (*(*inode).i_sb).s_fs_info;
            err = nilfs_dat_translate((*nilfs).ns_dat, vbn, &mut pbn);
            if err != 0 {
                folio_unlock((*bh).b_folio);
                folio_put((*bh).b_folio);
                brelse(bh);
                return err;
            }
        }

        lock_buffer(bh);
        if buffer_uptodate(bh) {
            unlock_buffer(bh);
        } else {
            if !buffer_mapped(bh) {
                set_buffer_mapped(bh);
            }
            (*bh).b_blocknr = pbn;
            bh_submit(bh, REQ_OP_READ, bh_end_read);
            if vbn != 0 {
                (*bh).b_blocknr = vbn;
            }
        }
        err = 0;
    }

    *out_bh = bh;
    folio_unlock((*bh).b_folio);
    folio_put((*bh).b_folio);
    if err != 0 {
        brelse(bh);
    }
    err
}

/*
 * nilfs_gccache_submit_read_node() - add node buffer and submit read request
 * @inode - gc inode
 * @pbn - physical block number for the block
 * @vbn - virtual block number for the block
 * @out_bh - indirect pointer to a buffer_head struct to receive the results
 *
 * Description: nilfs_gccache_submit_read_node() registers the node buffer
 * specified by @vbn to the GC pagecache.  @pbn can be supplied by the
 * caller to avoid translation of the disk block address.
 *
 * Return: 0 on success, or one of the following negative error codes on
 * failure:
 * * %-EIO	- I/O error (including metadata corruption).
 * * %-ENOENT	- Invalid virtual block address.
 * * %-ENOMEM	- Insufficient memory available.
 */
pub unsafe fn nilfs_gccache_submit_read_node(
    inode: *mut inode,
    mut pbn: sector_t,
    vbn: __u64,
    out_bh: *mut *mut buffer_head,
) -> i32 {
    let btnc_inode = NILFS_I(inode).i_assoc_inode;
    let mut ret = nilfs_btnode_submit_block(
        (*btnc_inode).i_mapping,
        if vbn != 0 { vbn } else { pbn },
        pbn,
        REQ_OP_READ,
        out_bh,
        &mut pbn,
    );
    if ret == -EEXIST {
        ret = 0;
    }
    ret
}

pub unsafe fn nilfs_gccache_wait_and_mark_dirty(bh: *mut buffer_head) -> i32 {
    wait_on_buffer(bh);
    if !buffer_uptodate(bh) {
        let inode = (*(*bh).b_folio).mapping.host;
        nilfs_err(
            (*inode).i_sb,
            "I/O error reading %s block for GC (ino=%llu, vblocknr=%llu)",
            if buffer_nilfs_node(bh) { "node" } else { "data" },
            (*inode).i_ino,
            (*bh).b_blocknr as u64,
        );
        return -EIO;
    }
    if buffer_dirty(bh) {
        return -EEXIST;
    }
    if buffer_nilfs_node(bh) && nilfs_btree_broken_node_block(bh) {
        clear_buffer_uptodate(bh);
        return -EIO;
    }
    mark_buffer_dirty(bh);
    0
}

pub unsafe fn nilfs_init_gcinode(inode: *mut inode) -> i32 {
    let ii = NILFS_I(inode);
    (*inode).i_mode = S_IFREG;
    mapping_set_gfp_mask((*inode).i_mapping, GFP_NOFS);
    (*(*inode).i_mapping).a_ops = &nilfs_buffer_cache_aops;
    (*ii).i_flags = 0;
    nilfs_bmap_init_gc((*ii).i_bmap);
    nilfs_attach_btree_node_cache(inode)
}

/**
 * nilfs_remove_all_gcinodes() - remove all unprocessed gc inodes
 * @nilfs: NILFS filesystem instance
 */
pub unsafe fn nilfs_remove_all_gcinodes(nilfs: *mut the_nilfs) {
    let head = &mut (*nilfs).ns_gc_inodes;
    while !list_empty(head) {
        let ii = list_first_entry::<nilfs_inode_info>(head, i_dirty);
        list_del_init(&mut (*ii).i_dirty);
        truncate_inode_pages(&mut (*ii).vfs_inode.i_data, 0);
        nilfs_btnode_cache_clear((*ii).i_assoc_inode.i_mapping);
        iput(&mut (*ii).vfs_inode);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
