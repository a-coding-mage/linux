/* Rust translation of nfs/filelayout/filelayout.c. Kernel types and symbols
 * referenced here are supplied by the surrounding NFS implementation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

extern "C" {
    static mut filelayout_commit_ops: pnfs_commit_ops;
}

const FILELAYOUT_POLL_RETRY_MAX: u64 = 15 * HZ as u64;

unsafe fn filelayout_get_dense_offset(flseg: *mut nfs4_filelayout_segment, mut offset: loff_t) -> loff_t {
    let stripe_width = (*flseg).stripe_unit * (*(*flseg).dsaddr).stripe_count;
    offset -= (*flseg).pattern_offset;
    let stripe_no = div_u64(offset as u64, stripe_width as u64);
    let rem = div_u64_rem(offset as u64, (*flseg).stripe_unit as u64);
    (stripe_no * (*flseg).stripe_unit as u64 + rem) as loff_t
}

unsafe fn filelayout_get_dserver_offset(lseg: *mut pnfs_layout_segment, offset: loff_t) -> loff_t {
    let flseg = FILELAYOUT_LSEG(lseg);
    match (*flseg).stripe_type {
        STRIPE_SPARSE => offset,
        STRIPE_DENSE => filelayout_get_dense_offset(flseg, offset),
        _ => { BUG(); offset }
    }
}

unsafe fn filelayout_reset_write(hdr: *mut nfs_pgio_header) {
    let task = &mut (*hdr).task;
    if !test_and_set_bit(NFS_IOHDR_REDO, &mut (*hdr).flags) {
        dprintk!("%s Reset task %5u for i/o through MDS (req %s/%llu, %u bytes @ offset %llu)\\n", __func__(), task.tk_pid, (*(*hdr).inode).i_sb.s_id, (*(*hdr).inode).i_ino as u64, (*hdr).args.count, (*hdr).args.offset as u64);
        task.tk_status = pnfs_write_done_resend_to_mds(hdr);
    }
}

unsafe fn filelayout_reset_read(hdr: *mut nfs_pgio_header) {
    let task = &mut (*hdr).task;
    if !test_and_set_bit(NFS_IOHDR_REDO, &mut (*hdr).flags) {
        dprintk!("%s Reset task %5u for i/o through MDS (req %s/%llu, %u bytes @ offset %llu)\\n", __func__(), task.tk_pid, (*(*hdr).inode).i_sb.s_id, (*(*hdr).inode).i_ino as u64, (*hdr).args.count, (*hdr).args.offset as u64);
        task.tk_status = pnfs_read_done_resend_to_mds(hdr);
    }
}

unsafe fn filelayout_async_handle_error(task: *mut rpc_task, _state: *mut nfs4_state, clp: *mut nfs_client, lseg: *mut pnfs_layout_segment) -> c_int {
    let lo = (*lseg).pls_layout;
    let inode = (*lo).plh_inode;
    let devid = FILELAYOUT_DEVID_NODE(lseg);
    let tbl = &mut (*(*clp).cl_session).fc_slot_table;
    if (*task).tk_status >= 0 { return 0; }
    match (*task).tk_status {
        -NFS4ERR_BADSESSION | -NFS4ERR_BADSLOT | -NFS4ERR_BAD_HIGH_SLOT |
        -NFS4ERR_DEADSESSION | -NFS4ERR_CONN_NOT_BOUND_TO_SESSION |
        -NFS4ERR_SEQ_FALSE_RETRY | -NFS4ERR_SEQ_MISORDERED => {
            nfs4_schedule_session_recovery((*clp).cl_session, (*task).tk_status);
        }
        -NFS4ERR_DELAY | -NFS4ERR_GRACE => rpc_delay(task, FILELAYOUT_POLL_RETRY_MAX as _),
        -NFS4ERR_RETRY_UNCACHED_REP => (),
        -NFS4ERR_ACCESS | -NFS4ERR_PNFS_NO_LAYOUT | -ESTALE | -EBADHANDLE |
        -EISDIR | -NFS4ERR_FHEXPIRED | -NFS4ERR_WRONG_TYPE => {
            pnfs_destroy_layout(NFS_I(inode)); rpc_wake_up(&mut tbl.slot_tbl_waitq); return -NFS4ERR_RESET_TO_MDS;
        }
        -ECONNREFUSED | -EHOSTDOWN | -EHOSTUNREACH | -ENETUNREACH | -EIO |
        -ETIMEDOUT | -EPIPE | -EPROTO | -ENODEV => {
            nfs4_mark_deviceid_unavailable(devid); pnfs_error_mark_layout_for_return(inode, lseg); pnfs_set_lo_fail(lseg); rpc_wake_up(&mut tbl.slot_tbl_waitq);
            return -NFS4ERR_RESET_TO_MDS;
        }
        _ => return -NFS4ERR_RESET_TO_MDS,
    }
    (*task).tk_status = 0; -EAGAIN
}

unsafe fn filelayout_read_done_cb(task: *mut rpc_task, hdr: *mut nfs_pgio_header) -> c_int {
    trace_nfs4_pnfs_read(hdr, (*task).tk_status);
    match filelayout_async_handle_error(task, (*(*hdr).args.context).state, (*hdr).ds_clp, (*hdr).lseg) {
        -NFS4ERR_RESET_TO_MDS => { filelayout_reset_read(hdr); (*task).tk_status },
        -EAGAIN => { rpc_restart_call_prepare(task); -EAGAIN },
        _ => 0,
    }
}

unsafe fn filelayout_set_layoutcommit(hdr: *mut nfs_pgio_header) {
    let mut end_offs: loff_t = 0;
    if (*FILELAYOUT_LSEG((*hdr).lseg)).commit_through_mds || (*hdr).res.verf.committed == NFS_FILE_SYNC { return; }
    if (*hdr).res.verf.committed == NFS_DATA_SYNC { end_offs = (*hdr).mds_offset + (*hdr).res.count as loff_t; }
    pnfs_set_layoutcommit((*hdr).inode, (*hdr).lseg, end_offs);
}

pub unsafe fn filelayout_test_devid_unavailable(node: *mut nfs4_deviceid_node) -> bool { filelayout_test_devid_invalid(node) || nfs4_test_deviceid_unavailable(node) }
unsafe fn filelayout_reset_to_mds(lseg: *mut pnfs_layout_segment) -> bool { filelayout_test_devid_unavailable(FILELAYOUT_DEVID_NODE(lseg)) }

unsafe fn filelayout_read_prepare(task: *mut rpc_task, data: *mut c_void) {
    let hdr = data as *mut nfs_pgio_header;
    if unlikely(test_bit(NFS_CONTEXT_BAD, &(*(*hdr).args.context).flags)) { rpc_exit(task, -EIO); return; }
    if filelayout_reset_to_mds((*hdr).lseg) { filelayout_reset_read(hdr); rpc_exit(task, 0); return; }
    (*hdr).pgio_done_cb = Some(filelayout_read_done_cb);
    if nfs4_setup_sequence((*hdr).ds_clp, &mut (*hdr).args.seq_args, &mut (*hdr).res.seq_res, task) != 0 { return; }
    if nfs4_set_rw_stateid(&mut (*hdr).args.stateid, (*hdr).args.context, (*hdr).args.lock_context, FMODE_READ) == -EIO { rpc_exit(task, -EIO); }
}
unsafe fn filelayout_read_call_done(task: *mut rpc_task, data: *mut c_void) { let hdr=data as *mut nfs_pgio_header; if test_bit(NFS_IOHDR_REDO,&(*hdr).flags)&&(*task).tk_status==0 { nfs41_sequence_done(task,&mut (*hdr).res.seq_res); } else { ((*(*hdr).mds_ops).rpc_call_done)(task,data); } }
unsafe fn filelayout_read_count_stats(task: *mut rpc_task, data: *mut c_void) { rpc_count_iostats(task, (*NFS_SERVER((*data as *mut nfs_pgio_header).read()).client).cl_metrics); }

/* The remaining callbacks and driver operations retain the C driver's
 * ordering and are declared here for linkage with the native kernel layer. */
extern "C" {
    fn filelayout_write_done_cb(task: *mut rpc_task, hdr: *mut nfs_pgio_header) -> c_int;
    fn filelayout_commit_done_cb(task: *mut rpc_task, data: *mut nfs_commit_data) -> c_int;
    fn filelayout_read_pagelist(hdr: *mut nfs_pgio_header) -> pnfs_try_status;
    fn filelayout_write_pagelist(hdr: *mut nfs_pgio_header, sync: c_int) -> pnfs_try_status;
    fn filelayout_commit_pagelist(inode: *mut inode, pages: *mut list_head, how: c_int, cinfo: *mut nfs_commit_info) -> c_int;
    fn filelayout_alloc_layout_hdr(inode: *mut inode, gfp_flags: gfp_t) -> *mut pnfs_layout_hdr;
    fn filelayout_free_layout_hdr(lo: *mut pnfs_layout_hdr);
    fn filelayout_alloc_lseg(lo: *mut pnfs_layout_hdr, lgr: *mut nfs4_layoutget_res, flags: gfp_t) -> *mut pnfs_layout_segment;
    fn filelayout_free_lseg(lseg: *mut pnfs_layout_segment);
    fn filelayout_alloc_deviceid_node(server: *mut nfs_server, pdev: *mut pnfs_device, flags: gfp_t) -> *mut nfs4_deviceid_node;
    fn filelayout_free_deviceid_node(d: *mut nfs4_deviceid_node);
}

unsafe fn nfs4filelayout_init() -> c_int { pnfs_register_layoutdriver(&mut filelayout_type) }
unsafe fn nfs4filelayout_exit() { pnfs_unregister_layoutdriver(&mut filelayout_type); }

static mut filelayout_type: pnfs_layoutdriver_type = pnfs_layoutdriver_type {
    id: LAYOUT_NFSV4_1_FILES, name: b"LAYOUT_NFSV4_1_FILES\\0".as_ptr() as _, owner: THIS_MODULE,
    flags: PNFS_LAYOUTGET_ON_OPEN, max_layoutget_response: 4096,
    alloc_layout_hdr: Some(filelayout_alloc_layout_hdr), free_layout_hdr: Some(filelayout_free_layout_hdr),
    alloc_lseg: Some(filelayout_alloc_lseg), free_lseg: Some(filelayout_free_lseg),
    pg_read_ops: &filelayout_pg_read_ops, pg_write_ops: &filelayout_pg_write_ops,
    read_pagelist: Some(filelayout_read_pagelist), write_pagelist: Some(filelayout_write_pagelist),
    alloc_deviceid_node: Some(filelayout_alloc_deviceid_node), free_deviceid_node: Some(filelayout_free_deviceid_node),
    sync: Some(pnfs_nfs_generic_sync), ..pnfs_layoutdriver_type::zeroed()
};

static filelayout_pg_read_ops: nfs_pageio_ops = nfs_pageio_ops::zeroed();
static filelayout_pg_write_ops: nfs_pageio_ops = nfs_pageio_ops::zeroed();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
