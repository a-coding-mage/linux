// SPDX-License-Identifier: GPL-2.0
/*
 *  Functions for dibs loopback/loopback-ism device.
 *
 *  Copyright (c) 2024, Alibaba Inc.
 *
 *  Author: Wen Gu <guwen@linux.alibaba.com>
 *          Tony Lu <tonylu@linux.alibaba.com>
 */

const DIBS_LO_SUPPORT_NOCOPY: i32 = 0x1;
const DIBS_DMA_ADDR_INVALID: dma_addr_t = !0 as dma_addr_t;

static DIBS_LO_DEV_NAME: &[u8] = b"lo\0";
/* global loopback device */
static mut lo_dev: *mut dibs_lo_dev = core::ptr::null_mut();

unsafe fn dibs_lo_get_fabric_id(_dibs: *mut dibs_dev) -> u16 { DIBS_LOOPBACK_FABRIC }

unsafe fn dibs_lo_query_rgid(dibs: *mut dibs_dev, rgid: *const uuid_t,
                             _vid_valid: u32, _vid: u32) -> i32 {
    /* rgid should be the same as lgid */
    if !uuid_equal(rgid, &(*dibs).gid) { return -ENETUNREACH; }
    0
}

unsafe fn dibs_lo_max_dmbs() -> i32 { DIBS_LO_MAX_DMBS }

unsafe fn dibs_lo_register_dmb(dibs: *mut dibs_dev, dmb: *mut dibs_dmb,
                               client: *mut dibs_client) -> i32 {
    let mut sba_idx: i32 = (*dmb).idx;
    let ldev = (*dibs).drv_priv;
    let mut dmb_node: *mut dibs_lo_dmb_node;
    let mut folio: *mut folio;
    let mut flags: c_ulong = 0;
    let mut rc: i32;

    /* check space for new dmb */
    for_each_clear_bit!(sba_idx, (*ldev).sba_idx_mask, DIBS_LO_MAX_DMBS, {
        if !test_and_set_bit(sba_idx, (*ldev).sba_idx_mask) { break; }
    });
    if sba_idx == DIBS_LO_MAX_DMBS { return -ENOSPC; }

    dmb_node = kzalloc_obj!();
    if dmb_node.is_null() { rc = -ENOMEM; goto err_bit; }
    (*dmb_node).sba_idx = sba_idx;
    (*dmb_node).len = (*dmb).dmb_len;

    /* not critical; fail under memory pressure and fallback to TCP */
    folio = folio_alloc(GFP_KERNEL | __GFP_NOWARN | __GFP_NOMEMALLOC |
                        __GFP_NORETRY | __GFP_ZERO, get_order((*dmb_node).len));
    if folio.is_null() { rc = -ENOMEM; goto err_node; }
    (*dmb_node).cpu_addr = folio_address(folio);
    (*dmb_node).dma_addr = DIBS_DMA_ADDR_INVALID;
    refcount_set(&mut_ref!((*dmb_node).refcnt), 1);

again:
    /* add new dmb into hash table */
    get_random_bytes(&mut (*dmb_node).token, core::mem::size_of_val(&(*dmb_node).token));
    write_lock_bh(&mut (*ldev).dmb_ht_lock);
    hash_for_each_possible!((*ldev).dmb_ht, tmp_node, list, (*dmb_node).token, {
        if (*tmp_node).token == (*dmb_node).token {
            write_unlock_bh(&mut (*ldev).dmb_ht_lock); goto again;
        }
    });
    hash_add!((*ldev).dmb_ht, &mut (*dmb_node).list, (*dmb_node).token);
    write_unlock_bh(&mut (*ldev).dmb_ht_lock);
    atomic_inc(&mut (*ldev).dmb_cnt);
    (*dmb).idx = (*dmb_node).sba_idx;
    (*dmb).dmb_tok = (*dmb_node).token;
    (*dmb).cpu_addr = (*dmb_node).cpu_addr;
    (*dmb).dma_addr = (*dmb_node).dma_addr;
    (*dmb).dmb_len = (*dmb_node).len;
    spin_lock_irqsave(&mut (*dibs).lock, &mut flags);
    (*dibs).dmb_clientid_arr[sba_idx as usize] = (*client).id;
    spin_unlock_irqrestore(&mut (*dibs).lock, flags);
    return 0;

err_node:
    kfree(dmb_node);
err_bit:
    clear_bit(sba_idx, (*ldev).sba_idx_mask);
    rc
}

unsafe fn dibs_lo_free_dmb(ldev: *mut dibs_lo_dev, dmb_node: *mut dibs_lo_dmb_node) {
    clear_bit((*dmb_node).sba_idx, (*ldev).sba_idx_mask);
    folio_put(virt_to_folio((*dmb_node).cpu_addr));
    kfree(dmb_node);
    if atomic_dec_and_test(&mut (*ldev).dmb_cnt) { wake_up(&mut (*ldev).ldev_release); }
}

unsafe fn dibs_lo_unregister_dmb(dibs: *mut dibs_dev, dmb: *mut dibs_dmb) -> i32 {
    let ldev = (*dibs).drv_priv;
    let mut dmb_node: *mut dibs_lo_dmb_node = core::ptr::null_mut();
    write_lock_bh(&mut (*ldev).dmb_ht_lock);
    hash_for_each_possible!((*ldev).dmb_ht, tmp_node, list, (*dmb).dmb_tok, {
        if (*tmp_node).token == (*dmb).dmb_tok { dmb_node = tmp_node; break; }
    });
    if dmb_node.is_null() { write_unlock_bh(&mut (*ldev).dmb_ht_lock); return -EINVAL; }
    let last = refcount_dec_and_test(&mut (*dmb_node).refcnt);
    if last { hash_del!(&mut (*dmb_node).list); }
    write_unlock_bh(&mut (*ldev).dmb_ht_lock);
    if last {
        let mut flags = 0;
        spin_lock_irqsave(&mut (*dibs).lock, &mut flags);
        (*dibs).dmb_clientid_arr[(*dmb_node).sba_idx as usize] = NO_DIBS_CLIENT;
        spin_unlock_irqrestore(&mut (*dibs).lock, flags);
        dibs_lo_free_dmb(ldev, dmb_node);
    }
    0
}

unsafe fn dibs_lo_support_dmb_nocopy(_dibs: *mut dibs_dev) -> i32 { DIBS_LO_SUPPORT_NOCOPY }

unsafe fn dibs_lo_attach_dmb(dibs: *mut dibs_dev, dmb: *mut dibs_dmb) -> i32 {
    let ldev = (*dibs).drv_priv;
    let mut dmb_node: *mut dibs_lo_dmb_node = core::ptr::null_mut();
    read_lock_bh(&mut (*ldev).dmb_ht_lock);
    hash_for_each_possible!((*ldev).dmb_ht, tmp_node, list, (*dmb).dmb_tok, {
        if (*tmp_node).token == (*dmb).dmb_tok { dmb_node = tmp_node; break; }
    });
    if dmb_node.is_null() { read_unlock_bh(&mut (*ldev).dmb_ht_lock); return -EINVAL; }
    refcount_inc(&mut (*dmb_node).refcnt);
    read_unlock_bh(&mut (*ldev).dmb_ht_lock);
    (*dmb).idx = (*dmb_node).sba_idx;
    (*dmb).dmb_tok = (*dmb_node).token;
    (*dmb).cpu_addr = (*dmb_node).cpu_addr;
    (*dmb).dma_addr = (*dmb_node).dma_addr;
    (*dmb).dmb_len = (*dmb_node).len;
    0
}

unsafe fn dibs_lo_detach_dmb(dibs: *mut dibs_dev, token: u64) -> i32 {
    let ldev = (*dibs).drv_priv;
    let mut dmb_node: *mut dibs_lo_dmb_node = core::ptr::null_mut();
    write_lock_bh(&mut (*ldev).dmb_ht_lock);
    hash_for_each_possible!((*ldev).dmb_ht, tmp_node, list, token, {
        if (*tmp_node).token == token { dmb_node = tmp_node; break; }
    });
    if dmb_node.is_null() { write_unlock_bh(&mut (*ldev).dmb_ht_lock); return -EINVAL; }
    let last = refcount_dec_and_test(&mut (*dmb_node).refcnt);
    if last { hash_del!(&mut (*dmb_node).list); }
    write_unlock_bh(&mut (*ldev).dmb_ht_lock);
    if last { dibs_lo_free_dmb(ldev, dmb_node); }
    0
}

unsafe fn dibs_lo_move_data(dibs: *mut dibs_dev, dmb_tok: u64, idx: u32, sf: bool,
                            offset: u32, data: *mut core::ffi::c_void, size: u32) -> i32 {
    let ldev = (*dibs).drv_priv;
    let mut rmb_node: *mut dibs_lo_dmb_node = core::ptr::null_mut();
    read_lock_bh(&mut (*ldev).dmb_ht_lock);
    hash_for_each_possible!((*ldev).dmb_ht, tmp_node, list, dmb_tok, {
        if (*tmp_node).token == dmb_tok { rmb_node = tmp_node; break; }
    });
    if rmb_node.is_null() || (offset as u64).wrapping_add(size as u64) > (*rmb_node).len {
        read_unlock_bh(&mut (*ldev).dmb_ht_lock); return -EINVAL;
    }
    core::ptr::copy_nonoverlapping(data as *const u8, ((*rmb_node).cpu_addr as *mut u8).add(offset as usize), size as usize);
    let sba_idx = (*rmb_node).sba_idx;
    read_unlock_bh(&mut (*ldev).dmb_ht_lock);
    if !sf { return 0; }
    spin_lock(&mut (*dibs).lock);
    let client_id = (*dibs).dmb_clientid_arr[sba_idx as usize];
    let s_mask = ror16(0x1000, idx);
    if client_id != NO_DIBS_CLIENT && !(*dibs).subs[client_id as usize].is_null() {
        ((*(*(*dibs).subs[client_id as usize]).ops).handle_irq)(dibs, sba_idx, s_mask);
    }
    spin_unlock(&mut (*dibs).lock);
    0
}

static dibs_lo_ops: dibs_dev_ops = dibs_dev_ops {
    get_fabric_id: Some(dibs_lo_get_fabric_id), query_remote_gid: Some(dibs_lo_query_rgid),
    max_dmbs: Some(dibs_lo_max_dmbs), register_dmb: Some(dibs_lo_register_dmb),
    unregister_dmb: Some(dibs_lo_unregister_dmb), move_data: Some(dibs_lo_move_data),
    support_mmapped_rdmb: Some(dibs_lo_support_dmb_nocopy), attach_dmb: Some(dibs_lo_attach_dmb),
    detach_dmb: Some(dibs_lo_detach_dmb),
};

unsafe fn dibs_lo_dev_init(ldev: *mut dibs_lo_dev) {
    rwlock_init(&mut (*ldev).dmb_ht_lock); hash_init!((*ldev).dmb_ht);
    atomic_set(&mut (*ldev).dmb_cnt, 0); init_waitqueue_head(&mut (*ldev).ldev_release);
}

unsafe fn dibs_lo_dev_exit(ldev: *mut dibs_lo_dev) {
    if atomic_read(&(*ldev).dmb_cnt) != 0 { wait_event!((*ldev).ldev_release, atomic_read(&(*ldev).dmb_cnt) == 0); }
}

unsafe fn dibs_lo_dev_probe() -> i32 {
    let ldev = kzalloc_obj!(); if ldev.is_null() { return -ENOMEM; }
    let dibs = dibs_dev_alloc();
    if dibs.is_null() { kfree(ldev); return -ENOMEM; }
    (*ldev).dibs = dibs; (*dibs).drv_priv = ldev; dibs_lo_dev_init(ldev);
    uuid_gen(&mut (*dibs).gid); (*dibs).ops = &dibs_lo_ops;
    (*dibs).dev.parent = core::ptr::null_mut(); dev_set_name(&mut (*dibs).dev, DIBS_LO_DEV_NAME);
    let ret = dibs_dev_add(dibs);
    if ret != 0 { put_device(&mut (*dibs).dev); kfree(ldev); return ret; }
    lo_dev = ldev; 0
}

unsafe fn dibs_lo_dev_remove() {
    if lo_dev.is_null() { return; }
    dibs_dev_del((*lo_dev).dibs); dibs_lo_dev_exit(lo_dev);
    put_device(&mut (*(*lo_dev).dibs).dev); kfree(lo_dev); lo_dev = core::ptr::null_mut();
}

pub unsafe fn dibs_loopback_init() -> i32 { dibs_lo_dev_probe() }
pub unsafe fn dibs_loopback_exit() { dibs_lo_dev_remove(); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
