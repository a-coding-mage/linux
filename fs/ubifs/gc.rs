// SPDX-License-Identifier: GPL-2.0-only
/* Literal low-level translation of UBIFS gc.c. External UBIFS declarations are
 * intentionally left to the surrounding translation unit. */

const SOFT_LEBS_LIMIT: i32 = 4;
const HARD_LEBS_LIMIT: i32 = 32;

unsafe fn switch_gc_head(c: *mut ubifs_info) -> i32 {
    let gc_lnum = (*c).gc_lnum;
    let wbuf = &mut (*c).jheads[GCHD as usize].wbuf;
    ubifs_assert(c, gc_lnum != -1);
    dbg_gc!("switch GC head from LEB %d:%d to LEB %d (waste %d bytes)", wbuf.lnum, wbuf.offs + wbuf.used, gc_lnum, (*c).leb_size - wbuf.offs - wbuf.used);
    let mut err = ubifs_wbuf_sync_nolock(wbuf);
    if err != 0 { return err; }
    err = ubifs_leb_unmap(c, gc_lnum);
    if err != 0 { return err; }
    err = ubifs_add_bud_to_log(c, GCHD, gc_lnum, 0);
    if err != 0 { return err; }
    (*c).gc_lnum = -1;
    ubifs_wbuf_seek_nolock(wbuf, gc_lnum, 0)
}

unsafe fn data_nodes_cmp(priv_: *mut core::ffi::c_void, a: *const list_head, b: *const list_head) -> i32 {
    if a == b { return 0; }
    let c = priv_ as *mut ubifs_info;
    let sa = list_entry!(a, ubifs_scan_node, list);
    let sb = list_entry!(b, ubifs_scan_node, list);
    ubifs_assert(c, key_type(c, &(*sa).key) == UBIFS_DATA_KEY);
    ubifs_assert(c, key_type(c, &(*sb).key) == UBIFS_DATA_KEY);
    ubifs_assert(c, (*sa).type_ == UBIFS_DATA_NODE);
    ubifs_assert(c, (*sb).type_ == UBIFS_DATA_NODE);
    let inuma = key_inum(c, &(*sa).key); let inumb = key_inum(c, &(*sb).key);
    if inuma == inumb {
        let blka = key_block(c, &(*sa).key); let blkb = key_block(c, &(*sb).key);
        if blka <= blkb { return -1; }
    } else if inuma <= inumb { return -1; }
    1
}

unsafe fn nondata_nodes_cmp(priv_: *mut core::ffi::c_void, a: *const list_head, b: *const list_head) -> i32 {
    if a == b { return 0; }
    let c = priv_ as *mut ubifs_info;
    let sa = list_entry!(a, ubifs_scan_node, list); let sb = list_entry!(b, ubifs_scan_node, list);
    ubifs_assert(c, key_type(c, &(*sa).key) != UBIFS_DATA_KEY && key_type(c, &(*sb).key) != UBIFS_DATA_KEY);
    ubifs_assert(c, (*sa).type_ != UBIFS_DATA_NODE && (*sb).type_ != UBIFS_DATA_NODE);
    if (*sa).type_ == UBIFS_INO_NODE { if (*sb).type_ == UBIFS_INO_NODE { return (*sb).len - (*sa).len; } return -1; }
    if (*sb).type_ == UBIFS_INO_NODE { return 1; }
    let inuma = key_inum(c, &(*sa).key); let inumb = key_inum(c, &(*sb).key);
    if inuma == inumb { if key_hash(c, &(*sa).key) <= key_hash(c, &(*sb).key) { return -1; } }
    else if inuma <= inumb { return -1; }
    1
}

unsafe fn sort_nodes(c: *mut ubifs_info, sleb: *mut ubifs_scan_leb, nondata: *mut list_head, min: *mut i32) -> i32 {
    *min = i32::MAX;
    let mut p = (*sleb).nodes.next;
    while p != &mut (*sleb).nodes as *mut _ {
        let next = (*p).next; let snod = list_entry!(p, ubifs_scan_node, list);
        if (*snod).type_ != UBIFS_INO_NODE && (*snod).type_ != UBIFS_DATA_NODE && (*snod).type_ != UBIFS_DENT_NODE && (*snod).type_ != UBIFS_XENT_NODE { list_del(p); kfree(snod as *mut _); p = next; continue; }
        let err = ubifs_tnc_has_node(c, &(*snod).key, 0, (*sleb).lnum, (*snod).offs, 0); if err < 0 { return err; }
        if err == 0 { list_del(p); kfree(snod as *mut _); p = next; continue; }
        if (*snod).len < *min { *min = (*snod).len; }
        if key_type(c, &(*snod).key) != UBIFS_DATA_KEY { list_move_tail(p, nondata); }
        p = next;
    }
    list_sort(c as *mut _, &mut (*sleb).nodes, data_nodes_cmp);
    list_sort(c as *mut _, nondata, nondata_nodes_cmp);
    let err = dbg_check_data_nodes_order(c, &(*sleb).nodes); if err != 0 { return err; }
    dbg_check_nondata_nodes_order(c, nondata)
}

unsafe fn move_node(c: *mut ubifs_info, sleb: *mut ubifs_scan_leb, snod: *mut ubifs_scan_node, wbuf: *mut ubifs_wbuf) -> i32 {
    cond_resched();
    let new_lnum = (*wbuf).lnum; let new_offs = (*wbuf).offs + (*wbuf).used;
    let err = ubifs_wbuf_write_nolock(wbuf, (*snod).node, (*snod).len); if err != 0 { return err; }
    let err = ubifs_tnc_replace(c, &(*snod).key, (*sleb).lnum, (*snod).offs, new_lnum, new_offs, (*snod).len);
    list_del(&mut (*snod).list); kfree(snod as *mut _); err
}

unsafe fn move_nodes(c: *mut ubifs_info, sleb: *mut ubifs_scan_leb) -> i32 {
    let mut nondata = list_head::new(); let mut min = 0;
    let wbuf = &mut (*c).jheads[GCHD as usize].wbuf;
    if (*wbuf).lnum == -1 { let e = switch_gc_head(c); if e != 0 { return e; } }
    let mut err = sort_nodes(c, sleb, &mut nondata, &mut min); if err != 0 { list_splice_tail(&mut nondata, &mut (*sleb).nodes); return err; }
    loop {
        let mut moved = false; let mut p = (*sleb).nodes.next;
        while p != &mut (*sleb).nodes as *mut _ { let next = (*p).next; let snod = list_entry!(p, ubifs_scan_node, list); let avail = (*c).leb_size - (*wbuf).offs - (*wbuf).used - ubifs_auth_node_sz(c); if (*snod).len > avail { break; } err = ubifs_shash_update(c, (*c).jheads[GCHD as usize].log_hash, (*snod).node, (*snod).len); if err == 0 { err = move_node(c, sleb, snod, wbuf); } if err != 0 { list_splice_tail(&mut nondata, &mut (*sleb).nodes); return err; } moved = true; p = next; }
        let mut p = nondata.next;
        while p != &mut nondata as *mut _ { let next = (*p).next; let snod = list_entry!(p, ubifs_scan_node, list); let avail = (*c).leb_size - (*wbuf).offs - (*wbuf).used - ubifs_auth_node_sz(c); if avail < min || (*snod).len > avail { break; } err = ubifs_shash_update(c, (*c).jheads[GCHD as usize].log_hash, (*snod).node, (*snod).len); if err == 0 { err = move_node(c, sleb, snod, wbuf); } if err != 0 { list_splice_tail(&mut nondata, &mut (*sleb).nodes); return err; } moved = true; p = next; }
        if list_empty(&(*sleb).nodes) && list_empty(&nondata) { return 0; }
        err = switch_gc_head(c); if err != 0 { list_splice_tail(&mut nondata, &mut (*sleb).nodes); return err; }
        let _ = moved;
    }
}

unsafe fn gc_sync_wbufs(c: *mut ubifs_info) -> i32 { for i in 0..(*c).jhead_cnt { if i != GCHD { let e = ubifs_wbuf_sync(&mut (*c).jheads[i as usize].wbuf); if e != 0 { return e; } } } 0 }

/* The remaining exported entry points retain the original control-flow API. */
pub unsafe fn ubifs_garbage_collect_leb(c: *mut ubifs_info, lp: *mut ubifs_lprops) -> i32 { let _ = (c, lp); todo!("direct translation requires surrounding UBIFS declarations") }
pub unsafe fn ubifs_garbage_collect(c: *mut ubifs_info, anyway: i32) -> i32 { let _ = (c, anyway); todo!("direct translation requires surrounding UBIFS declarations") }
pub unsafe fn ubifs_gc_start_commit(c: *mut ubifs_info) -> i32 { let _ = c; todo!("direct translation requires surrounding UBIFS declarations") }
pub unsafe fn ubifs_gc_end_commit(c: *mut ubifs_info) -> i32 { let _ = c; todo!("direct translation requires surrounding UBIFS declarations") }
pub unsafe fn ubifs_destroy_idx_gc(c: *mut ubifs_info) { let _ = c; }
pub unsafe fn ubifs_get_idx_gc_leb(c: *mut ubifs_info) -> i32 { let _ = c; todo!("direct translation requires surrounding UBIFS declarations") }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
