// SPDX-License-Identifier: GPL-2.0
/* Linux NFSv4 callback procedures. C headers and external kernel symbols are
 * supplied by the surrounding translation unit. */

const NFSDBG_FACILITY: u32 = NFSDBG_CALLBACK;

pub unsafe extern "C" fn nfs4_callback_getattr(argp: *mut core::ffi::c_void, resp: *mut core::ffi::c_void, cps: *mut cb_process_state) -> __be32 {
    let args = argp as *mut cb_getattrargs;
    let res = resp as *mut cb_getattrres;
    let mut inode: *mut inode;
    (*res).status = htonl(NFS4ERR_OP_NOT_IN_SESSION);
    if (*cps).clp.is_null() { return (*res).status; }
    core::ptr::write_bytes((*res).bitmap.as_mut_ptr(), 0, (*res).bitmap.len());
    (*res).status = htonl(NFS4ERR_BADHANDLE);
    dprintk_rcu!("NFS: GETATTR callback request from %s\n", rpc_peeraddr2str((*(*cps).clp).cl_rpcclient, RPC_DISPLAY_ADDR));
    inode = nfs_delegation_find_inode((*cps).clp, &(*args).fh);
    if IS_ERR(inode) {
        if inode == ERR_PTR(-EAGAIN) { (*res).status = htonl(NFS4ERR_DELAY); }
        trace_nfs4_cb_getattr((*cps).clp, &(*args).fh, core::ptr::null_mut(), -ntohl((*res).status));
        return (*res).status;
    }
    let delegation = nfs4_get_valid_delegation(inode);
    if delegation.is_null() { nfs_iput_and_deactive(inode); return (*res).status; }
    if ((*delegation).type_ & FMODE_WRITE) == 0 { nfs_put_delegation(delegation); nfs_iput_and_deactive(inode); return (*res).status; }
    (*res).change_attr = (*delegation).change_attr;
    nfs_put_delegation(delegation);
    (*res).size = i_size_read(inode);
    if nfs_have_writebacks(inode) { (*res).change_attr += 1; }
    (*res).atime = inode_get_atime(inode); (*res).ctime = inode_get_ctime(inode); (*res).mtime = inode_get_mtime(inode);
    (*res).bitmap[0] = (FATTR4_WORD0_CHANGE | FATTR4_WORD0_SIZE) & (*args).bitmap[0];
    (*res).bitmap[1] = (FATTR4_WORD1_TIME_ACCESS | FATTR4_WORD1_TIME_METADATA | FATTR4_WORD1_TIME_MODIFY) & (*args).bitmap[1];
    (*res).bitmap[2] = (FATTR4_WORD2_TIME_DELEG_ACCESS | FATTR4_WORD2_TIME_DELEG_MODIFY) & (*args).bitmap[2];
    (*res).status = 0;
    trace_nfs4_cb_getattr((*cps).clp, &(*args).fh, inode, -ntohl((*res).status));
    nfs_iput_and_deactive(inode);
    (*res).status
}

pub unsafe extern "C" fn nfs4_callback_recall(argp: *mut core::ffi::c_void, _resp: *mut core::ffi::c_void, cps: *mut cb_process_state) -> __be32 {
    let args = argp as *mut cb_recallargs;
    let mut res = htonl(NFS4ERR_OP_NOT_IN_SESSION);
    if (*cps).clp.is_null() { return res; }
    dprintk_rcu!("NFS: RECALL callback request from %s\n", rpc_peeraddr2str((*(*cps).clp).cl_rpcclient, RPC_DISPLAY_ADDR));
    res = htonl(NFS4ERR_BADHANDLE);
    let inode = nfs_delegation_find_inode((*cps).clp, &(*args).fh);
    if IS_ERR(inode) { if inode == ERR_PTR(-EAGAIN) { res = htonl(NFS4ERR_DELAY); } trace_nfs4_cb_recall((*cps).clp, &(*args).fh, core::ptr::null_mut(), &(*args).stateid, -ntohl(res)); return res; }
    res = match nfs_async_inode_return_delegation(inode, &(*args).stateid) { 0 => 0, -ENOENT => htonl(NFS4ERR_BAD_STATEID), _ => htonl(NFS4ERR_RESOURCE) };
    trace_nfs4_cb_recall((*cps).clp, &(*args).fh, inode, &(*args).stateid, -ntohl(res));
    nfs_iput_and_deactive(inode); res
}

unsafe fn nfs_layout_find_inode_by_stateid(clp: *mut nfs_client, stateid: *const nfs4_stateid) -> *mut inode {
    rcu_read_lock();
    let mut server: *mut nfs_server;
    list_for_each_entry_rcu!(server, &(*clp).cl_superblocks, client_link, {
        let mut lo: *mut pnfs_layout_hdr;
        list_for_each_entry_rcu!(lo, &(*server).layouts, plh_layouts, {
            if !pnfs_layout_is_valid(lo) || !nfs4_stateid_match_other(stateid, &(*lo).plh_stateid) { continue; }
            let inode = if nfs_sb_active((*server).super_) { igrab((*lo).plh_inode) } else { ERR_PTR(-EAGAIN) };
            rcu_read_unlock();
            if !inode.is_null() { return inode; }
            nfs_sb_deactive((*server).super_); return ERR_PTR(-EAGAIN);
        });
    });
    rcu_read_unlock(); ERR_PTR(-ENOENT)
}

unsafe fn nfs_layout_find_inode_by_fh(clp: *mut nfs_client, fh: *const nfs_fh) -> *mut inode {
    rcu_read_lock();
    let mut server: *mut nfs_server;
    list_for_each_entry_rcu!(server, &(*clp).cl_superblocks, client_link, {
        let mut lo: *mut pnfs_layout_hdr;
        list_for_each_entry_rcu!(lo, &(*server).layouts, plh_layouts, {
            let nfsi = NFS_I((*lo).plh_inode);
            if nfs_compare_fh(fh, &(*nfsi).fh) || (*nfsi).layout != lo { continue; }
            let inode = if nfs_sb_active((*server).super_) { igrab((*lo).plh_inode) } else { ERR_PTR(-EAGAIN) };
            rcu_read_unlock(); if !inode.is_null() { return inode; }
            nfs_sb_deactive((*server).super_); return ERR_PTR(-EAGAIN);
        });
    });
    rcu_read_unlock(); ERR_PTR(-ENOENT)
}

unsafe fn nfs_layout_find_inode(clp: *mut nfs_client, fh: *const nfs_fh, stateid: *const nfs4_stateid) -> *mut inode {
    let mut inode = nfs_layout_find_inode_by_stateid(clp, stateid);
    if inode == ERR_PTR(-ENOENT) { inode = nfs_layout_find_inode_by_fh(clp, fh); } inode
}

unsafe fn pnfs_check_callback_stateid(lo: *mut pnfs_layout_hdr, new: *const nfs4_stateid, cps: *mut cb_process_state) -> u32 {
    if !pnfs_layout_is_valid(lo) { return NFS4ERR_NOMATCHING_LAYOUT; }
    if !nfs4_stateid_match_other(&(*lo).plh_stateid, new) { return NFS4ERR_BAD_STATEID; }
    let oldseq = be32_to_cpu((*lo).plh_stateid.seqid); let newseq = be32_to_cpu((*new).seqid);
    if test_bit(NFS_LAYOUT_RETURN, &(*lo).plh_flags) { return NFS4ERR_DELAY; }
    if newseq > oldseq.wrapping_add(1) && !(*cps).referring_calls { return NFS4ERR_DELAY; }
    if newseq <= oldseq { return NFS4ERR_OLD_STATEID; } NFS_OK
}

unsafe fn initiate_file_draining(clp: *mut nfs_client, args: *mut cb_layoutrecallargs, cps: *mut cb_process_state) -> u32 {
    let ino = nfs_layout_find_inode(clp, &(*args).cbl_fh, &(*args).cbl_stateid);
    if IS_ERR(ino) { return if ino == ERR_PTR(-EAGAIN) { NFS4ERR_DELAY } else { NFS4ERR_NOMATCHING_LAYOUT }; }
    pnfs_layoutcommit_inode(ino, false); spin_lock(&(*ino).i_lock);
    let lo = NFS_I(ino).layout; if lo.is_null() { spin_unlock(&(*ino).i_lock); nfs_iput_and_deactive(ino); return NFS4ERR_NOMATCHING_LAYOUT; }
    pnfs_get_layout_hdr(lo); let rv = pnfs_check_callback_stateid(lo, &(*args).cbl_stateid, cps);
    if rv != NFS_OK { spin_unlock(&(*ino).i_lock); pnfs_put_layout_hdr(lo); nfs_iput_and_deactive(ino); return rv; }
    if test_bit(NFS_LAYOUT_BULK_RECALL, &(*lo).plh_flags) { spin_unlock(&(*ino).i_lock); pnfs_put_layout_hdr(lo); nfs_iput_and_deactive(ino); return NFS4ERR_DELAY; }
    pnfs_set_layout_stateid(lo, &(*args).cbl_stateid, core::ptr::null_mut(), true); let mut free_me_list = LIST_HEAD_INIT!();
    let stat = pnfs_mark_matching_lsegs_return(lo, &mut free_me_list, &(*args).cbl_range, be32_to_cpu((*args).cbl_stateid.seqid), (*args).cbl_layoutchanged);
    let rv = match stat { 0 | -EBUSY => NFS4_OK, -ENOENT => { set_bit(NFS_LAYOUT_DRAIN, &(*lo).plh_flags); NFS4ERR_NOMATCHING_LAYOUT }, _ => NFS4ERR_NOMATCHING_LAYOUT };
    spin_unlock(&(*ino).i_lock); pnfs_free_lseg_list(&mut free_me_list); nfs_commit_inode(ino, 0); pnfs_put_layout_hdr(lo); nfs_iput_and_deactive(ino); trace_nfs4_cb_layoutrecall_file(clp, &(*args).cbl_fh, ino, &(*args).cbl_stateid, (*args).cbl_layoutchanged, -(rv as i32)); rv
}

unsafe fn initiate_bulk_draining(clp: *mut nfs_client, args: *mut cb_layoutrecallargs) -> u32 { let stat = if (*args).cbl_recall_type == RETURN_FSID { pnfs_layout_destroy_byfsid(clp, &(*args).cbl_fsid, PNFS_LAYOUT_BULK_RETURN) } else { pnfs_layout_destroy_byclid(clp, PNFS_LAYOUT_BULK_RETURN) }; if stat != 0 { NFS4ERR_DELAY } else { NFS4ERR_NOMATCHING_LAYOUT } }
unsafe fn pnfs_recall_all_layouts(clp: *mut nfs_client, cps: *mut cb_process_state) { let mut args: cb_layoutrecallargs = core::mem::zeroed(); args.cbl_recall_type = RETURN_ALL; do_callback_layoutrecall(clp, &mut args, cps); }
unsafe fn do_callback_layoutrecall(clp: *mut nfs_client, args: *mut cb_layoutrecallargs, cps: *mut cb_process_state) -> u32 { if (*args).cbl_recall_type == RETURN_FILE { initiate_file_draining(clp, args, cps) } else { initiate_bulk_draining(clp, args) } }

unsafe fn validate_seqid(tbl: *const nfs4_slot_table, slot: *const nfs4_slot, args: *const cb_sequenceargs) -> __be32 { let mut ret = cpu_to_be32(NFS4ERR_BADSLOT); if (*args).csa_slotid > (*tbl).server_highest_slotid { return ret; } if (*args).csa_sequenceid == (*slot).seq_nr { ret = cpu_to_be32(NFS4ERR_DELAY); if nfs4_test_locked_slot(tbl, slot) { return ret; } ret = cpu_to_be32(NFS4ERR_RETRY_UNCACHED_REP); if (*args).csa_cachethis == 0 { return ret; } ret = cpu_to_be32(NFS4ERR_SEQ_FALSE_RETRY); return ret; } ret = cpu_to_be32(NFS4ERR_SEQ_MISORDERED); if (*args).csa_sequenceid != (*slot).seq_nr.wrapping_add(1) { return ret; } cpu_to_be32(NFS4_OK) }

pub unsafe extern "C" fn nfs4_callback_sequence(argp: *mut core::ffi::c_void, resp: *mut core::ffi::c_void, cps: *mut cb_process_state) -> __be32 { let args = argp as *mut cb_sequenceargs; let res = resp as *mut cb_sequenceres; let clp = nfs4_find_client_sessionid((*cps).net, (*args).csa_addr, &(*args).csa_sessionid, (*cps).minorversion); if clp.is_null() { return htonl(NFS4ERR_BADSESSION); } (*cps).clp = clp; (*res).csr_sessionid = (*args).csa_sessionid; (*res).csr_sequenceid = (*args).csa_sequenceid; (*res).csr_slotid = (*args).csa_slotid; let tbl = &mut (*(*clp).cl_session).bc_slot_table; spin_lock(&mut tbl.slot_tbl_lock); let slot = nfs4_lookup_slot(tbl, (*args).csa_slotid); if IS_ERR(slot) { spin_unlock(&mut tbl.slot_tbl_lock); return htonl(NFS4ERR_BADSLOT); } let status = validate_seqid(tbl, slot, args); if status == 0 { (*slot).seq_nr = (*args).csa_sequenceid; } spin_unlock(&mut tbl.slot_tbl_lock); for i in 0..(*args).csa_nrclists { kfree((*args).csa_rclists.add(i as usize).rcl_refcalls as *mut _); } kfree((*args).csa_rclists as *mut _); (*res).csr_status = status; trace_nfs4_cb_sequence(args, res, status); status }

#[cfg(CONFIG_NFS_V4_2)]
unsafe fn nfs4_copy_cb_args(cp_state: *mut nfs4_copy_state, args: *const cb_offloadargs) { (*cp_state).count = (*args).wr_count; (*cp_state).error = (*args).error; if (*args).error == 0 { (*cp_state).verf.committed = (*args).wr_writeverf.committed; core::ptr::copy_nonoverlapping((*args).wr_writeverf.verifier.data.as_ptr(), (*cp_state).verf.verifier.data.as_mut_ptr(), NFS4_VERIFIER_SIZE as usize); } }

#[cfg(CONFIG_NFS_V4_2)]
pub unsafe extern "C" fn nfs4_callback_offload(data: *mut core::ffi::c_void, _dummy: *mut core::ffi::c_void, cps: *mut cb_process_state) -> __be32 { let args = data as *mut cb_offloadargs; let copy = kzalloc_obj::<nfs4_copy_state>(); if copy.is_null() { return cpu_to_be32(NFS4ERR_DELAY); } spin_lock(&mut (*(*cps).clp).cl_lock); let mut found = false; let mut server: *mut nfs_server; list_for_each_entry_rcu!(server, &(*(*cps).clp).cl_superblocks, client_link, { let mut tmp_copy: *mut nfs4_copy_state; list_for_each_entry!(tmp_copy, &(*server).ss_copies, copies, { if core::slice::from_raw_parts((*args).coa_stateid.other.as_ptr(), (*args).coa_stateid.other.len()) == core::slice::from_raw_parts((*tmp_copy).stateid.other.as_ptr(), (*tmp_copy).stateid.other.len()) { nfs4_copy_cb_args(tmp_copy, args); complete(&mut (*tmp_copy).completion); found = true; } }); }); if !found { (*copy).stateid = (*args).coa_stateid; nfs4_copy_cb_args(copy, args); list_add_tail(&mut (*copy).copies, &mut (*(*cps).clp).pending_cb_stateids); } else { kfree(copy as *mut _); } spin_unlock(&mut (*(*cps).clp).cl_lock); trace_nfs4_cb_offload(&(*args).coa_fh, &(*args).coa_stateid, (*args).wr_count, (*args).error, (*args).wr_writeverf.committed); 0 }

// Remaining callback procedures retain the source control flow and call external kernel APIs.
pub unsafe extern "C" fn nfs4_callback_layoutrecall(argp: *mut core::ffi::c_void, _resp: *mut core::ffi::c_void, cps: *mut cb_process_state) -> __be32 {
    let args = argp as *mut cb_layoutrecallargs; let mut res = NFS4ERR_OP_NOT_IN_SESSION;
    if !(*cps).clp.is_null() { res = if (*args).cbl_recall_type == RETURN_FILE { initiate_file_draining((*cps).clp, args, cps) } else { initiate_bulk_draining((*cps).clp, args) }; } cpu_to_be32(res)
}

pub unsafe extern "C" fn nfs4_callback_devicenotify(argp: *mut core::ffi::c_void, _resp: *mut core::ffi::c_void, cps: *mut cb_process_state) -> __be32 {
    let args = argp as *mut cb_devicenotifyargs; if (*cps).clp.is_null() { kfree((*args).devs as *mut _); return cpu_to_be32(NFS4ERR_OP_NOT_IN_SESSION); }
    let mut ld: *const pnfs_layoutdriver_type = core::ptr::null(); for i in 0..(*args).ndevs { let dev = (*args).devs.add(i as usize); if ld.is_null() || (*ld).id != (*dev).cbd_layout_type { pnfs_put_layoutdriver(ld); ld = pnfs_find_layoutdriver((*dev).cbd_layout_type); if ld.is_null() { continue; } } nfs4_delete_deviceid(ld, (*cps).clp, &(*dev).cbd_dev_id); } pnfs_put_layoutdriver(ld); kfree((*args).devs as *mut _); 0
}

unsafe fn validate_bitmap_values(mask: u32) -> bool { (mask & !RCA4_TYPE_MASK_ALL) == 0 }

pub unsafe extern "C" fn nfs4_callback_recallany(argp: *mut core::ffi::c_void, _resp: *mut core::ffi::c_void, cps: *mut cb_process_state) -> __be32 {
    let args = argp as *mut cb_recallanyargs; if (*cps).clp.is_null() { return cpu_to_be32(NFS4ERR_OP_NOT_IN_SESSION); } if !validate_bitmap_values((*args).craa_type_mask) { return cpu_to_be32(NFS4ERR_INVAL); }
    let mut flags: fmode_t = 0; if (*args).craa_type_mask & BIT(RCA4_TYPE_MASK_RDATA_DLG) != 0 { flags = FMODE_READ; } if (*args).craa_type_mask & BIT(RCA4_TYPE_MASK_WDATA_DLG) != 0 { flags |= FMODE_WRITE; } if flags != 0 { nfs_expire_unused_delegation_types((*cps).clp, flags); } if (*args).craa_type_mask & BIT(RCA4_TYPE_MASK_FILE_LAYOUT) != 0 { pnfs_recall_all_layouts((*cps).clp, cps); } if (*args).craa_type_mask & BIT(PNFS_FF_RCA4_TYPE_MASK_READ) != 0 { set_bit(NFS4CLNT_RECALL_ANY_LAYOUT_READ, &(*(*cps).clp).cl_state); } if (*args).craa_type_mask & BIT(PNFS_FF_RCA4_TYPE_MASK_RW) != 0 { set_bit(NFS4CLNT_RECALL_ANY_LAYOUT_RW, &(*(*cps).clp).cl_state); } nfs4_schedule_state_manager((*cps).clp); cpu_to_be32(NFS4_OK)
}

pub unsafe extern "C" fn nfs4_callback_recallslot(_argp: *mut core::ffi::c_void, _resp: *mut core::ffi::c_void, cps: *mut cb_process_state) -> __be32 { if (*cps).clp.is_null() { htonl(NFS4ERR_OP_NOT_IN_SESSION) } else { nfs41_set_target_slotid(&(*(*cps).clp).cl_session.fc_slot_table, 0); nfs41_notify_server((*cps).clp); htonl(NFS4_OK) } }
pub unsafe extern "C" fn nfs4_callback_notify_lock(argp: *mut core::ffi::c_void, _resp: *mut core::ffi::c_void, cps: *mut cb_process_state) -> __be32 { if (*cps).clp.is_null() { return htonl(NFS4ERR_OP_NOT_IN_SESSION); } let args = argp as *mut cb_notify_lock_args; if (*args).cbnl_valid { __wake_up(&(*(*cps).clp).cl_lock_waitq, TASK_NORMAL, 0, args as *mut _); } htonl(NFS4_OK) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
