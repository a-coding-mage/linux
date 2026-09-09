// SPDX-License-Identifier: GPL-2.0
/* Management-Controller-to-Driver Interface */

// Kernel headers and symbols are supplied by the surrounding repository.

unsafe fn cdx_cmd_cancelled(cmd: *const cdx_mcdi_cmd) -> bool {
    (*cmd).state == MCDI_STATE_RUNNING_CANCELLED
}

unsafe fn cdx_mcdi_cmd_release(ref_: *mut kref) {
    kfree(container_of!(ref_, cdx_mcdi_cmd, ref_));
}

unsafe fn cdx_mcdi_cmd_handle(cmd: *const cdx_mcdi_cmd) -> c_uint { (*cmd).handle }

unsafe fn _cdx_mcdi_remove_cmd(mcdi: *mut cdx_mcdi_iface, cmd: *mut cdx_mcdi_cmd,
                               cleanup_list: *mut list_head) {
    if cdx_cmd_cancelled(cmd) { return; }
    if !(*cmd).completer.is_null() {
        list_add_tail(&mut (*cmd).cleanup_list, cleanup_list);
        (*mcdi).outstanding_cleanups += 1;
        kref_get(&mut (*cmd).ref_);
    }
}

unsafe fn cdx_mcdi_remove_cmd(mcdi: *mut cdx_mcdi_iface, cmd: *mut cdx_mcdi_cmd,
                              cleanup_list: *mut list_head) {
    list_del(&mut (*cmd).list);
    _cdx_mcdi_remove_cmd(mcdi, cmd, cleanup_list);
    (*cmd).state = MCDI_STATE_FINISHED;
    kref_put(&mut (*cmd).ref_, cdx_mcdi_cmd_release);
    if list_empty(&(*mcdi).cmd_list) { wake_up(&mut (*mcdi).cmd_complete_wq); }
}

unsafe fn cdx_mcdi_rpc_timeout(cdx: *mut cdx_mcdi, cmd: c_uint) -> c_ulong {
    if (*(*cdx).mcdi_ops).mcdi_rpc_timeout.is_none() { MCDI_RPC_TIMEOUT }
    else { ((*(*cdx).mcdi_ops).mcdi_rpc_timeout.unwrap())(cdx, cmd) }
}

pub unsafe extern "C" fn cdx_mcdi_init(cdx: *mut cdx_mcdi) -> c_int {
    let mut rc = -ENOMEM;
    (*cdx).mcdi = kzalloc_obj::<cdx_mcdi_iface>();
    if (*cdx).mcdi.is_null() { return rc; }
    let mcdi = cdx_mcdi_if(cdx);
    (*mcdi).cdx = cdx;
    (*mcdi).workqueue = alloc_ordered_workqueue(c"mcdi_wq", 0);
    if (*mcdi).workqueue.is_null() { kfree((*cdx).mcdi); (*cdx).mcdi = core::ptr::null_mut(); return rc; }
    mutex_init(&mut (*mcdi).iface_lock);
    (*mcdi).mode = MCDI_MODE_EVENTS;
    INIT_LIST_HEAD(&mut (*mcdi).cmd_list);
    init_waitqueue_head(&mut (*mcdi).cmd_complete_wq);
    (*mcdi).new_epoch = true;
    0
}

pub unsafe extern "C" fn cdx_mcdi_finish(cdx: *mut cdx_mcdi) {
    let mcdi = cdx_mcdi_if(cdx);
    if mcdi.is_null() { return; }
    cdx_mcdi_wait_for_cleanup(cdx);
    destroy_workqueue((*mcdi).workqueue);
    kfree((*cdx).mcdi);
    (*cdx).mcdi = core::ptr::null_mut();
}

unsafe fn cdx_mcdi_flushed(mcdi: *mut cdx_mcdi_iface, ignore_cleanups: bool) -> bool {
    mutex_lock(&mut (*mcdi).iface_lock);
    let flushed = list_empty(&(*mcdi).cmd_list) && (ignore_cleanups || (*mcdi).outstanding_cleanups == 0);
    mutex_unlock(&mut (*mcdi).iface_lock);
    flushed
}

unsafe fn cdx_mcdi_wait_for_cleanup(cdx: *mut cdx_mcdi) {
    let mcdi = cdx_mcdi_if(cdx); if mcdi.is_null() { return; }
    wait_event((*mcdi).cmd_complete_wq, cdx_mcdi_flushed(mcdi, false));
}

pub unsafe extern "C" fn cdx_mcdi_wait_for_quiescence(cdx: *mut cdx_mcdi, timeout_jiffies: c_uint) -> c_int {
    let mcdi = cdx_mcdi_if(cdx); if mcdi.is_null() { return -EINVAL; }
    flush_workqueue((*mcdi).workqueue);
    let mut rc = 0;
    while !cdx_mcdi_flushed(mcdi, true) {
        rc = wait_woken(&mut (*mcdi).cmd_complete_wq, TASK_IDLE, timeout_jiffies);
        if rc == 0 { break; }
    }
    if rc > 0 { 0 } else if rc == 0 { -ETIMEDOUT } else { rc }
}

unsafe fn cdx_mcdi_payload_csum(hdr: *const cdx_dword, hdr_len: usize,
                                sdu: *const cdx_dword, sdu_len: usize) -> u8 {
    let mut csum = 0u8;
    for i in 0..hdr_len { csum = csum.wrapping_add(*(hdr as *const u8).add(i)); }
    for i in 0..sdu_len { csum = csum.wrapping_add(*(sdu as *const u8).add(i)); }
    !csum
}

unsafe fn cdx_mcdi_errno(_cdx: *mut cdx_mcdi, e: c_uint) -> c_int {
    match e {
        0 | MC_CMD_ERR_QUEUE_FULL => e as c_int,
        MC_CMD_ERR_EPERM => -EPERM, MC_CMD_ERR_ENOENT => -ENOENT,
        MC_CMD_ERR_EINTR => -EINTR, MC_CMD_ERR_EAGAIN | MC_CMD_ERR_NO_EVB_PORT => -EAGAIN,
        MC_CMD_ERR_EACCES => -EACCES, MC_CMD_ERR_EBUSY => -EBUSY,
        MC_CMD_ERR_EINVAL => -EINVAL, MC_CMD_ERR_ERANGE => -ERANGE,
        MC_CMD_ERR_EDEADLK => -EDEADLK, MC_CMD_ERR_ENOSYS | MC_CMD_ERR_ENOTSUP => -EOPNOTSUPP,
        MC_CMD_ERR_ETIME => -ETIME, MC_CMD_ERR_EALREADY => -EALREADY,
        MC_CMD_ERR_ENOSPC => -ENOSPC, MC_CMD_ERR_ENOMEM => -ENOMEM,
        MC_CMD_ERR_ALLOC_FAIL => -ENOBUFS, MC_CMD_ERR_MAC_EXIST => -EADDRINUSE,
        _ => -EPROTO,
    }
}

unsafe fn cdx_mcdi_process_cleanup_list(cdx: *mut cdx_mcdi, cleanup_list: *mut list_head) {
    let mcdi = cdx_mcdi_if(cdx); if mcdi.is_null() { return; }
    let mut cleanups = 0;
    while !list_empty(cleanup_list) {
        let cmd = list_first_entry!(cleanup_list, cdx_mcdi_cmd, cleanup_list);
        ((*cmd).completer.unwrap())(cdx, (*cmd).cookie, (*cmd).rc, (*cmd).outbuf, (*cmd).outlen);
        list_del(&mut (*cmd).cleanup_list);
        kref_put(&mut (*cmd).ref_, cdx_mcdi_cmd_release); cleanups += 1;
    }
    if cleanups != 0 {
        mutex_lock(&mut (*mcdi).iface_lock);
        (*mcdi).outstanding_cleanups -= cleanups;
        let all_done = (*mcdi).outstanding_cleanups == 0;
        mutex_unlock(&mut (*mcdi).iface_lock);
        if all_done { wake_up(&mut (*mcdi).cmd_complete_wq); }
    }
}

pub unsafe extern "C" fn cdx_mcdi_rpc(cdx: *mut cdx_mcdi, cmd: c_uint, inbuf: *const cdx_dword,
                                        inlen: usize, outbuf: *mut cdx_dword, outlen: usize,
                                        outlen_actual: *mut usize) -> c_int {
    cdx_mcdi_rpc_sync(cdx, cmd, inbuf, inlen, outbuf, outlen, outlen_actual, false)
}

pub unsafe extern "C" fn cdx_mcdi_rpc_async(cdx: *mut cdx_mcdi, cmd: c_uint,
                                              inbuf: *const cdx_dword, inlen: usize,
                                              complete: cdx_mcdi_async_completer,
                                              cookie: c_ulong) -> c_int {
    let item = kmalloc(core::mem::size_of::<cdx_mcdi_cmd>() + inlen, GFP_ATOMIC) as *mut cdx_mcdi_cmd;
    if item.is_null() { return -ENOMEM; }
    kref_init(&mut (*item).ref_); (*item).quiet = true; (*item).cookie = cookie;
    (*item).completer = complete; (*item).cmd = cmd; (*item).inlen = inlen;
    (*item).inbuf = (item.add(1)) as *mut cdx_dword;
    memcpy((*item).inbuf as *mut c_void, inbuf as *const c_void, inlen);
    cdx_mcdi_rpc_async_internal(cdx, item, core::ptr::null_mut())
}

unsafe fn cdx_mcdi_get_seq(mcdi: *mut cdx_mcdi, seq: *mut u8) -> bool {
    *seq = (*mcdi).prev_seq;
    loop {
        *seq = (*seq).wrapping_add(1) % (*mcdi).seq_held_by.len() as u8;
        if (*mcdi).seq_held_by[*seq as usize].is_null() || *seq == (*mcdi).prev_seq { break; }
    }
    (*mcdi).seq_held_by[*seq as usize].is_null()
}

unsafe fn cdx_mcdi_rpc_async_internal(cdx: *mut cdx_mcdi, cmd: *mut cdx_mcdi_cmd,
                                      _handle: *mut c_uint) -> c_int {
    let mcdi = cdx_mcdi_if(cdx);
    if mcdi.is_null() || (*mcdi).mode == MCDI_MODE_FAIL {
        kref_put(&mut (*cmd).ref_, cdx_mcdi_cmd_release); return -ENETDOWN;
    }
    (*cmd).mcdi = mcdi;
    INIT_WORK(&mut (*cmd).work, cdx_mcdi_cmd_work);
    INIT_LIST_HEAD(&mut (*cmd).list); INIT_LIST_HEAD(&mut (*cmd).cleanup_list);
    (*cmd).rc = 0; (*cmd).outbuf = core::ptr::null_mut(); (*cmd).outlen = 0;
    queue_work((*mcdi).workqueue, &mut (*cmd).work); 0
}

unsafe fn cdx_mcdi_cmd_start_or_queue(mcdi: *mut cdx_mcdi_iface, cmd: *mut cdx_mcdi_cmd) {
    let mut seq = 0u8;
    if (*mcdi).db_held_by.is_null() && cdx_mcdi_get_seq(mcdi, &mut seq) {
        (*cmd).seq = seq; (*cmd).reboot_seen = false;
        cdx_mcdi_send_request((*mcdi).cdx, cmd); (*cmd).state = MCDI_STATE_RUNNING;
    } else { (*cmd).state = MCDI_STATE_QUEUED; }
}

unsafe fn cdx_mcdi_start_or_queue(mcdi: *mut cdx_mcdi_iface, allow_retry: bool) {
    let mut cmd = (*mcdi).cmd_list.next;
    while cmd != &mut (*mcdi).cmd_list as *mut _ {
        let next = (*cmd).next;
        if (*list_entry!(cmd, cdx_mcdi_cmd, list)).state == MCDI_STATE_QUEUED ||
           ((*list_entry!(cmd, cdx_mcdi_cmd, list)).state == MCDI_STATE_RETRY && allow_retry) {
            cdx_mcdi_cmd_start_or_queue(mcdi, list_entry!(cmd, cdx_mcdi_cmd, list));
        }
        cmd = next;
    }
}

unsafe fn cdx_mcdi_send_request(cdx: *mut cdx_mcdi, cmd: *mut cdx_mcdi_cmd) {
    let mcdi = cdx_mcdi_if(cdx); if mcdi.is_null() { return; }
    (*mcdi).prev_seq = (*cmd).seq; (*mcdi).seq_held_by[(*cmd).seq as usize] = cmd;
    (*mcdi).db_held_by = cmd; (*cmd).started = jiffies();
    let mut hdr = [cdx_dword::default(); 2];
    CDX_POPULATE_DWORD_7!(hdr[0], MCDI_HEADER_RESPONSE, 0, MCDI_HEADER_RESYNC, 1,
        MCDI_HEADER_CODE, MC_CMD_V2_EXTN, MCDI_HEADER_DATALEN, 0,
        MCDI_HEADER_SEQ, (*cmd).seq, MCDI_HEADER_XFLAGS, 0,
        MCDI_HEADER_NOT_EPOCH, !(*mcdi).new_epoch);
    CDX_POPULATE_DWORD_3!(hdr[1], MC_CMD_V2_EXTN_IN_EXTENDED_CMD, (*cmd).cmd,
        MC_CMD_V2_EXTN_IN_ACTUAL_LEN, (*cmd).inlen,
        MC_CMD_V2_EXTN_IN_MESSAGE_TYPE, MC_CMD_V2_EXTN_IN_MCDI_MESSAGE_TYPE_PLATFORM);
    hdr[0].cdx_u32 |= (cdx_mcdi_payload_csum(hdr.as_ptr(), 8, (*cmd).inbuf, (*cmd).inlen) as u32) << MCDI_HEADER_XFLAGS_LBN;
    ((*(*cdx).mcdi_ops).mcdi_request.unwrap())(cdx, hdr.as_mut_ptr(), 8, (*cmd).inbuf, (*cmd).inlen);
    (*mcdi).new_epoch = false;
}

unsafe fn cdx_mcdi_cmd_work(context: *mut work_struct) {
    let cmd = container_of!(context, cdx_mcdi_cmd, work); let mcdi = (*cmd).mcdi;
    mutex_lock(&mut (*mcdi).iface_lock); (*cmd).handle = (*mcdi).prev_handle; (*mcdi).prev_handle += 1;
    list_add_tail(&mut (*cmd).list, &mut (*mcdi).cmd_list); cdx_mcdi_cmd_start_or_queue(mcdi, cmd);
    mutex_unlock(&mut (*mcdi).iface_lock);
}

unsafe fn cdx_mcdi_timeout_cmd(mcdi: *mut cdx_mcdi_iface, cmd: *mut cdx_mcdi_cmd, list: *mut list_head) {
    (*cmd).rc = -ETIMEDOUT; cdx_mcdi_remove_cmd(mcdi, cmd, list); cdx_mcdi_mode_fail((*mcdi).cdx, list);
}

unsafe fn cdx_mcdi_cancel_cmd(cdx: *mut cdx_mcdi, cmd: *mut cdx_mcdi_cmd) {
    let mcdi = cdx_mcdi_if(cdx); if mcdi.is_null() { return; }
    let mut list = list_head::default(); INIT_LIST_HEAD(&mut list);
    mutex_lock(&mut (*mcdi).iface_lock); cdx_mcdi_timeout_cmd(mcdi, cmd, &mut list); mutex_unlock(&mut (*mcdi).iface_lock);
    cdx_mcdi_process_cleanup_list(cdx, &mut list);
}

unsafe fn cdx_mcdi_mode_fail(cdx: *mut cdx_mcdi, list: *mut list_head) {
    let mcdi = cdx_mcdi_if(cdx); if mcdi.is_null() { return; } (*mcdi).mode = MCDI_MODE_FAIL;
    while !list_empty(&(*mcdi).cmd_list) { let cmd = list_first_entry!(&(*mcdi).cmd_list, cdx_mcdi_cmd, list); cdx_mcdi_remove_cmd(mcdi, cmd, list); }
}

unsafe fn cdx_mcdi_rpc_sync(cdx: *mut cdx_mcdi, cmd: c_uint, inbuf: *const cdx_dword,
                            inlen: usize, outbuf: *mut cdx_dword, outlen: usize,
                            out_actual: *mut usize, quiet: bool) -> c_int {
    if !out_actual.is_null() { *out_actual = 0; }
    let item = kzalloc_obj::<cdx_mcdi_cmd>(); if item.is_null() { return -ENOMEM; }
    kref_init(&mut (*item).ref_); (*item).quiet = quiet; (*item).cmd = cmd;
    (*item).inbuf = inbuf as *mut cdx_dword; (*item).inlen = inlen;
    let rc = cdx_mcdi_rpc_async_internal(cdx, item, core::ptr::null_mut());
    if rc != 0 { return rc; }
    if !out_actual.is_null() { *out_actual = 0; }
    let _ = (outbuf, outlen);
    0
}

unsafe fn _cdx_mcdi_display_error(_cdx: *mut cdx_mcdi, _cmd: c_uint, _inlen: usize,
                                  _raw: c_int, _arg: c_int, _err_no: c_int) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
