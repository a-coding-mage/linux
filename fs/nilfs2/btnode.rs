// SPDX-License-Identifier: GPL-2.0+
/*
 * NILFS B-tree node cache
 *
 * Copyright (C) 2005-2008 Nippon Telegraph and Telephone Corporation.
 *
 * Originally written by Seiji Kihara.
 * Fully revised by Ryusuke Konishi for stabilization and simplification.
 */

// Linux and NILFS declarations are supplied by the surrounding translation.

pub unsafe fn nilfs_init_btnc_inode(btnc_inode: *mut inode) {
    let ii: *mut nilfs_inode_info = NILFS_I(btnc_inode);

    (*btnc_inode).i_mode = S_IFREG;
    (*ii).i_flags = 0;
    memset(
        &mut (*ii).i_bmap_data as *mut _ as *mut u8,
        0,
        core::mem::size_of::<nilfs_bmap>(),
    );
    mapping_set_gfp_mask((*btnc_inode).i_mapping, GFP_NOFS);
    (*btnc_inode).i_mapping.a_ops = &nilfs_buffer_cache_aops;
}

pub unsafe fn nilfs_btnode_cache_clear(btnc: *mut address_space) {
    invalidate_mapping_pages(btnc, 0, -1);
    truncate_inode_pages(btnc, 0);
}

pub unsafe fn nilfs_btnode_create_block(
    btnc: *mut address_space,
    blocknr: u64,
) -> *mut buffer_head {
    let inode = (*btnc).host;
    let bh = nilfs_grab_buffer(inode, btnc, blocknr, BIT(BH_NILFS_Node));
    if bh.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    if buffer_mapped(bh) || buffer_uptodate(bh) || buffer_dirty(bh) {
        nilfs_error(
            (*inode).i_sb,
            "state inconsistency probably due to duplicate use of b-tree node block address %llu (ino=%llu)",
            blocknr as u64,
            (*inode).i_ino,
        );
        folio_unlock((*bh).b_folio);
        folio_put((*bh).b_folio);
        brelse(bh);
        return ERR_PTR(-EIO);
    }
    memset((*bh).b_data, 0, i_blocksize(inode));
    (*bh).b_blocknr = blocknr;
    set_buffer_mapped(bh);
    set_buffer_uptodate(bh);

    folio_unlock((*bh).b_folio);
    folio_put((*bh).b_folio);
    bh
}

pub unsafe fn nilfs_btnode_submit_block(
    btnc: *mut address_space,
    blocknr: u64,
    mut pblocknr: sector_t,
    opf: blk_opf_t,
    pbh: *mut *mut buffer_head,
    submit_ptr: *mut sector_t,
) -> i32 {
    let inode = (*btnc).host;
    let bh = nilfs_grab_buffer(inode, btnc, blocknr, BIT(BH_NILFS_Node));
    if bh.is_null() {
        return -ENOMEM;
    }

    let mut err = -EEXIST;
    let folio = (*bh).b_folio;

    if buffer_uptodate(bh) || buffer_dirty(bh) {
        *pbh = bh;
        folio_unlock(folio);
        folio_put(folio);
        return err;
    }

    if pblocknr == 0 {
        pblocknr = blocknr;
        if (*inode).i_ino != NILFS_DAT_INO {
            let nilfs = (*(*inode).i_sb).s_fs_info;
            err = nilfs_dat_translate((*nilfs).ns_dat, blocknr, &mut pblocknr);
            if err != 0 {
                brelse(bh);
                folio_unlock(folio);
                folio_put(folio);
                return err;
            }
        }
    }

    if opf & REQ_RAHEAD != 0 {
        if pblocknr != (*submit_ptr).wrapping_add(1) || !trylock_buffer(bh) {
            err = -EBUSY;
            brelse(bh);
            folio_unlock(folio);
            folio_put(folio);
            return err;
        }
    } else {
        lock_buffer(bh);
    }
    if buffer_uptodate(bh) {
        unlock_buffer(bh);
        err = -EEXIST;
    } else {
        set_buffer_mapped(bh);
        (*bh).b_blocknr = pblocknr;
        bh_submit(bh, opf, bh_end_read);
        (*bh).b_blocknr = blocknr;
        *submit_ptr = pblocknr;
        err = 0;
    }
    *pbh = bh;
    folio_unlock(folio);
    folio_put(folio);
    err
}

pub unsafe fn nilfs_btnode_delete(bh: *mut buffer_head) {
    let folio = (*bh).b_folio;
    let index = (*folio).index;
    folio_get(folio);
    folio_lock(folio);
    folio_wait_writeback(folio);
    nilfs_forget_buffer(bh);
    let still_dirty = folio_test_dirty(folio);
    let mapping = (*folio).mapping;
    folio_unlock(folio);
    folio_put(folio);
    if !still_dirty && !mapping.is_null() {
        invalidate_inode_pages2_range(mapping, index, index);
    }
}

pub unsafe fn nilfs_btnode_prepare_change_key(
    btnc: *mut address_space,
    ctxt: *mut nilfs_btnode_chkey_ctxt,
) -> i32 {
    let obh = (*ctxt).bh;
    let inode = (*btnc).host;
    let oldkey = (*ctxt).oldkey;
    let newkey = (*ctxt).newkey;
    if oldkey == newkey { return 0; }
    (*ctxt).newbh = core::ptr::null_mut();

    if (*inode).i_blkbits == PAGE_SHIFT {
        let ofolio = (*obh).b_folio;
        folio_lock(ofolio);
        if oldkey != (*ofolio).index {
            NILFS_FOLIO_BUG(ofolio, "invalid oldkey %lld (newkey=%lld)", oldkey, newkey);
        }
        xa_lock_irq(&mut (*btnc).i_pages);
        let mut err = __xa_insert(&mut (*btnc).i_pages, newkey, ofolio, GFP_NOFS);
        xa_unlock_irq(&mut (*btnc).i_pages);
        if err == 0 { return 0; }
        if err != -EBUSY {
            folio_unlock(ofolio);
            return err;
        }
        err = invalidate_inode_pages2_range(btnc, newkey, newkey);
        if err == 0 {
            if oldkey != (*ofolio).index { NILFS_FOLIO_BUG(ofolio, "invalid oldkey"); }
            xa_lock_irq(&mut (*btnc).i_pages);
            err = __xa_insert(&mut (*btnc).i_pages, newkey, ofolio, GFP_NOFS);
            xa_unlock_irq(&mut (*btnc).i_pages);
            if err == 0 { return 0; }
        }
        folio_unlock(ofolio);
    }

    let nbh = nilfs_btnode_create_block(btnc, newkey);
    if IS_ERR(nbh) { return PTR_ERR(nbh); }
    BUG_ON(nbh == obh);
    (*ctxt).newbh = nbh;
    0
}

pub unsafe fn nilfs_btnode_commit_change_key(btnc: *mut address_space, ctxt: *mut nilfs_btnode_chkey_ctxt) {
    let obh = (*ctxt).bh;
    let nbh = (*ctxt).newbh;
    let oldkey = (*ctxt).oldkey;
    let newkey = (*ctxt).newkey;
    if oldkey == newkey { return; }
    if nbh.is_null() {
        let ofolio = (*obh).b_folio;
        if oldkey != (*ofolio).index { NILFS_FOLIO_BUG(ofolio, "invalid oldkey %lld (newkey=%lld)", oldkey, newkey); }
        mark_buffer_dirty(obh);
        xa_lock_irq(&mut (*btnc).i_pages);
        __xa_erase(&mut (*btnc).i_pages, oldkey);
        __xa_set_mark(&mut (*btnc).i_pages, newkey, PAGECACHE_TAG_DIRTY);
        xa_unlock_irq(&mut (*btnc).i_pages);
        (*ofolio).index = newkey;
        (*obh).b_blocknr = newkey;
        folio_unlock(ofolio);
    } else {
        nilfs_copy_buffer(nbh, obh);
        mark_buffer_dirty(nbh);
        (*nbh).b_blocknr = newkey;
        (*ctxt).bh = nbh;
        nilfs_btnode_delete(obh);
    }
}

pub unsafe fn nilfs_btnode_abort_change_key(btnc: *mut address_space, ctxt: *mut nilfs_btnode_chkey_ctxt) {
    let nbh = (*ctxt).newbh;
    if (*ctxt).oldkey == (*ctxt).newkey { return; }
    if nbh.is_null() {
        xa_erase_irq(&mut (*btnc).i_pages, (*ctxt).newkey);
        folio_unlock((*(*ctxt).bh).b_folio);
    } else {
        nilfs_btnode_delete(nbh);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
