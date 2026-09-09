// SPDX-License-Identifier: GPL-2.0-only
// Translated from rcom.c. Declarations and types supplied by the surrounding
// DLM sources are intentionally referenced but not redefined here.

unsafe fn rcom_response(ls: *mut dlm_ls) -> c_int {
    test_bit(LSFL_RCOM_READY, &(*ls).ls_flags)
}

unsafe fn _create_rcom(ls: *mut dlm_ls, _to_nodeid: c_int, typ: c_int, _len: c_int,
                       rc_ret: *mut *mut dlm_rcom, mb: *mut c_char, mb_len: c_int,
                       seq: u64) {
    let rc = mb as *mut dlm_rcom;
    (*rc).rc_header.h_version = cpu_to_le32(DLM_HEADER_MAJOR | DLM_HEADER_MINOR);
    (*rc).rc_header.u.h_lockspace = cpu_to_le32((*ls).ls_global_id);
    (*rc).rc_header.h_nodeid = cpu_to_le32(dlm_our_nodeid());
    (*rc).rc_header.h_length = cpu_to_le16(mb_len as u16);
    (*rc).rc_header.h_cmd = DLM_RCOM;
    (*rc).rc_type = cpu_to_le32(typ as u32);
    (*rc).rc_seq = cpu_to_le64(seq);
    *rc_ret = rc;
}

unsafe fn create_rcom(ls: *mut dlm_ls, to_nodeid: c_int, typ: c_int, len: c_int,
                      rc_ret: *mut *mut dlm_rcom, mh_ret: *mut *mut dlm_mhandle,
                      seq: u64) -> c_int {
    let mb_len = core::mem::size_of::<dlm_rcom>() as c_int + len;
    let mut mb: *mut c_char = core::ptr::null_mut();
    let mh = dlm_midcomms_get_mhandle(to_nodeid, mb_len, &mut mb);
    if mh.is_null() {
        log_print(c"%s to %d type %d len %d ENOBUFS".as_ptr(), c"create_rcom".as_ptr(), to_nodeid, typ, len);
        return -ENOBUFS;
    }
    _create_rcom(ls, to_nodeid, typ, len, rc_ret, mb, mb_len, seq);
    *mh_ret = mh;
    0
}

unsafe fn create_rcom_stateless(ls: *mut dlm_ls, to_nodeid: c_int, typ: c_int, len: c_int,
                                rc_ret: *mut *mut dlm_rcom, msg_ret: *mut *mut dlm_msg,
                                seq: u64) -> c_int {
    let mb_len = core::mem::size_of::<dlm_rcom>() as c_int + len;
    let mut mb: *mut c_char = core::ptr::null_mut();
    let msg = dlm_lowcomms_new_msg(to_nodeid, mb_len, &mut mb, core::ptr::null_mut(), core::ptr::null_mut());
    if msg.is_null() {
        log_print(c"create_rcom to %d type %d len %d ENOBUFS".as_ptr(), to_nodeid, typ, len);
        return -ENOBUFS;
    }
    _create_rcom(ls, to_nodeid, typ, len, rc_ret, mb, mb_len, seq);
    *msg_ret = msg;
    0
}

unsafe fn send_rcom(mh: *mut dlm_mhandle, _rc: *mut dlm_rcom) { dlm_midcomms_commit_mhandle(mh, core::ptr::null_mut(), 0); }
unsafe fn send_rcom_stateless(msg: *mut dlm_msg, _rc: *mut dlm_rcom) { dlm_lowcomms_commit_msg(msg); dlm_lowcomms_put_msg(msg); }

unsafe fn set_rcom_status(_ls: *mut dlm_ls, rs: *mut rcom_status, flags: u32) { (*rs).rs_flags = cpu_to_le32(flags); }

unsafe fn set_rcom_config(ls: *mut dlm_ls, rf: *mut rcom_config, num_slots: u32) {
    (*rf).rf_lvblen = cpu_to_le32((*ls).ls_lvblen);
    (*rf).rf_lsflags = cpu_to_le32((*ls).ls_exflags);
    (*rf).rf_our_slot = cpu_to_le16((*ls).ls_slot);
    (*rf).rf_num_slots = cpu_to_le16(num_slots as u16);
    (*rf).rf_generation = cpu_to_le32((*ls).ls_generation);
}

unsafe fn check_rcom_config(ls: *mut dlm_ls, rc: *mut dlm_rcom, nodeid: c_int) -> c_int {
    let rf = (*rc).rc_buf.as_mut_ptr() as *mut rcom_config;
    if (le32_to_cpu((*rc).rc_header.h_version) & 0xFFFF0000) != DLM_HEADER_MAJOR {
        log_error(ls, c"version mismatch: %x nodeid %d: %x".as_ptr(), DLM_HEADER_MAJOR | DLM_HEADER_MINOR, nodeid, le32_to_cpu((*rc).rc_header.h_version));
        return -EPROTO;
    }
    if le32_to_cpu((*rf).rf_lvblen) != (*ls).ls_lvblen || le32_to_cpu((*rf).rf_lsflags) != (*ls).ls_exflags {
        log_error(ls, c"config mismatch: %d,%x nodeid %d: %d,%x".as_ptr(), (*ls).ls_lvblen, (*ls).ls_exflags, nodeid, le32_to_cpu((*rf).rf_lvblen), le32_to_cpu((*rf).rf_lsflags));
        return -EPROTO;
    }
    0
}

unsafe fn allow_sync_reply(ls: *mut dlm_ls, new_seq: *mut u64) {
    spin_lock_bh(&mut (*ls).ls_rcom_spin);
    (*ls).ls_rcom_seq = (*ls).ls_rcom_seq.wrapping_add(1);
    *new_seq = cpu_to_le64((*ls).ls_rcom_seq);
    set_bit(LSFL_RCOM_WAIT, &mut (*ls).ls_flags);
    spin_unlock_bh(&mut (*ls).ls_rcom_spin);
}
unsafe fn disallow_sync_reply(ls: *mut dlm_ls) {
    spin_lock_bh(&mut (*ls).ls_rcom_spin);
    clear_bit(LSFL_RCOM_WAIT, &mut (*ls).ls_flags);
    clear_bit(LSFL_RCOM_READY, &mut (*ls).ls_flags);
    spin_unlock_bh(&mut (*ls).ls_rcom_spin);
}

pub unsafe fn dlm_rcom_status(ls: *mut dlm_ls, nodeid: c_int, status_flags: u32, seq: u64) -> c_int {
    let mut rc: *mut dlm_rcom = core::ptr::null_mut();
    let mut msg: *mut dlm_msg = core::ptr::null_mut();
    (*ls).ls_recover_nodeid = nodeid;
    if nodeid == dlm_our_nodeid() { (*(*ls).ls_recover_buf).rc_result = cpu_to_le32(dlm_recover_status(ls)); return 0; }
    loop {
        let mut error = create_rcom_stateless(ls, nodeid, DLM_RCOM_STATUS, core::mem::size_of::<rcom_status>() as c_int, &mut rc, &mut msg, seq);
        if error != 0 { return error; }
        set_rcom_status(ls, (*rc).rc_buf.as_mut_ptr() as *mut rcom_status, status_flags);
        allow_sync_reply(ls, &mut (*rc).rc_id);
        memset((*ls).ls_recover_buf as *mut c_void, 0, DLM_MAX_SOCKET_BUFSIZE);
        send_rcom_stateless(msg, rc);
        error = dlm_wait_function(ls, rcom_response);
        disallow_sync_reply(ls);
        if error == -ETIMEDOUT { continue; }
        if error != 0 { return error; }
        rc = (*ls).ls_recover_buf;
        if (*rc).rc_result == cpu_to_le32((-ESRCH) as u32) { log_debug(ls, c"remote node %d not ready".as_ptr(), nodeid); (*rc).rc_result = 0; return 0; }
        return check_rcom_config(ls, rc, nodeid);
    }
}

pub unsafe fn dlm_rcom_names(ls: *mut dlm_ls, nodeid: c_int, last_name: *const c_char, last_len: c_int, seq: u64) -> c_int {
    (*ls).ls_recover_nodeid = nodeid;
    loop {
        let mut rc = core::ptr::null_mut(); let mut mh = core::ptr::null_mut();
        let error = create_rcom(ls, nodeid, DLM_RCOM_NAMES, last_len, &mut rc, &mut mh, seq);
        if error != 0 { return error; }
        memcpy((*rc).rc_buf.as_mut_ptr() as *mut c_void, last_name as *const c_void, last_len as usize);
        allow_sync_reply(ls, &mut (*rc).rc_id); memset((*ls).ls_recover_buf as *mut c_void, 0, DLM_MAX_SOCKET_BUFSIZE);
        send_rcom(mh, rc); let error = dlm_wait_function(ls, rcom_response); disallow_sync_reply(ls);
        if error == -ETIMEDOUT { continue; } return error;
    }
}

pub unsafe fn dlm_send_rcom_lookup(r: *mut dlm_rsb, dir_nodeid: c_int, seq: u64) -> c_int {
    let ls = (*r).res_ls; let mut rc = core::ptr::null_mut(); let mut mh = core::ptr::null_mut();
    let error = create_rcom(ls, dir_nodeid, DLM_RCOM_LOOKUP, (*r).res_length, &mut rc, &mut mh, seq);
    if error != 0 { return error; }
    memcpy((*rc).rc_buf.as_mut_ptr() as *mut c_void, (*r).res_name as *const c_void, (*r).res_length as usize);
    (*rc).rc_id = cpu_to_le64((*r).res_id); send_rcom(mh, rc); 0
}

unsafe fn pack_rcom_lock(r: *mut dlm_rsb, lkb: *mut dlm_lkb, rl: *mut rcom_lock) {
    memset(rl as *mut c_void, 0, core::mem::size_of::<rcom_lock>());
    (*rl).rl_ownpid = cpu_to_le32((*lkb).lkb_ownpid); (*rl).rl_lkid = cpu_to_le32((*lkb).lkb_id);
    (*rl).rl_exflags = cpu_to_le32((*lkb).lkb_exflags); (*rl).rl_flags = cpu_to_le32(dlm_dflags_val(lkb));
    (*rl).rl_lvbseq = cpu_to_le32((*lkb).lkb_lvbseq); (*rl).rl_rqmode = (*lkb).lkb_rqmode; (*rl).rl_grmode = (*lkb).lkb_grmode;
    (*rl).rl_status = (*lkb).lkb_status; (*rl).rl_wait_type = cpu_to_le16((*lkb).lkb_wait_type);
    if !(*lkb).lkb_bastfn.is_null() { (*rl).rl_asts |= DLM_CB_BAST; } if !(*lkb).lkb_astfn.is_null() { (*rl).rl_asts |= DLM_CB_CAST; }
    (*rl).rl_namelen = cpu_to_le16((*r).res_length as u16); memcpy((*rl).rl_name.as_mut_ptr() as *mut c_void, (*r).res_name as *const c_void, (*r).res_length as usize);
    if !(*lkb).lkb_lvbptr.is_null() { memcpy((*rl).rl_lvb.as_mut_ptr() as *mut c_void, (*lkb).lkb_lvbptr as *const c_void, (*r).res_ls.as_ref().unwrap().ls_lvblen as usize); }
}

pub unsafe fn dlm_send_rcom_lock(r: *mut dlm_rsb, lkb: *mut dlm_lkb, seq: u64) -> c_int {
    let ls = (*r).res_ls; let mut len = core::mem::size_of::<rcom_lock>() as c_int; if !(*lkb).lkb_lvbptr.is_null() { len += (*ls).ls_lvblen; }
    let mut rc = core::ptr::null_mut(); let mut mh = core::ptr::null_mut(); let error = create_rcom(ls, (*r).res_nodeid, DLM_RCOM_LOCK, len, &mut rc, &mut mh, seq);
    if error != 0 { return error; } pack_rcom_lock(r, lkb, (*rc).rc_buf.as_mut_ptr() as *mut rcom_lock); (*rc).rc_id = cpu_to_le64(r as usize as u64); send_rcom(mh, rc); 0
}

pub unsafe fn dlm_send_ls_not_ready(nodeid: c_int, rc_in: *const dlm_rcom) -> c_int {
    let mb_len = (core::mem::size_of::<dlm_rcom>() + core::mem::size_of::<rcom_config>()) as c_int; let mut mb = core::ptr::null_mut();
    let mh = dlm_midcomms_get_mhandle(nodeid, mb_len, &mut mb); if mh.is_null() { return -ENOBUFS; }
    let rc = mb as *mut dlm_rcom; (*rc).rc_header.h_version = cpu_to_le32(DLM_HEADER_MAJOR | DLM_HEADER_MINOR); (*rc).rc_header.u.h_lockspace = (*rc_in).rc_header.u.h_lockspace; (*rc).rc_header.h_nodeid = cpu_to_le32(dlm_our_nodeid()); (*rc).rc_header.h_length = cpu_to_le16(mb_len as u16); (*rc).rc_header.h_cmd = DLM_RCOM; (*rc).rc_type = cpu_to_le32(DLM_RCOM_STATUS_REPLY); (*rc).rc_id = (*rc_in).rc_id; (*rc).rc_seq_reply = (*rc_in).rc_seq; (*rc).rc_result = cpu_to_le32((-ESRCH) as u32); (*( (*rc).rc_buf.as_mut_ptr() as *mut rcom_config)).rf_lvblen = cpu_to_le32(!0u32); dlm_midcomms_commit_mhandle(mh, core::ptr::null_mut(), 0); 0
}

unsafe fn receive_rcom_status(ls: *mut dlm_ls, rc_in: *const dlm_rcom, seq: u64) {
    let nodeid = le32_to_cpu((*rc_in).rc_header.h_nodeid); let mut num_slots = 0u32;
    let mut len = core::mem::size_of::<rcom_config>() as c_int;
    if !dlm_slots_version(&(*rc_in).rc_header) { /* status remains the local recovery status */ }
    else {
        let rs = (*rc_in).rc_buf.as_ptr() as *const rcom_status;
        if le32_to_cpu((*rs).rs_flags) & DLM_RSF_NEED_SLOTS != 0 { spin_lock_bh(&mut (*ls).ls_recover_lock); num_slots = (*ls).ls_num_slots; spin_unlock_bh(&mut (*ls).ls_recover_lock); len += num_slots as c_int * core::mem::size_of::<rcom_slot>() as c_int; }
    }
    let mut rc = core::ptr::null_mut(); let mut msg = core::ptr::null_mut();
    if create_rcom_stateless(ls, nodeid, DLM_RCOM_STATUS_REPLY, len, &mut rc, &mut msg, seq) != 0 { return; }
    (*rc).rc_id = (*rc_in).rc_id; (*rc).rc_seq_reply = (*rc_in).rc_seq;
    (*rc).rc_result = cpu_to_le32(dlm_recover_status(ls));
    set_rcom_config(ls, (*rc).rc_buf.as_mut_ptr() as *mut rcom_config, num_slots);
    if num_slots != 0 { spin_lock_bh(&mut (*ls).ls_recover_lock); if (*ls).ls_num_slots == num_slots { dlm_slots_copy_out(ls, rc); } spin_unlock_bh(&mut (*ls).ls_recover_lock); }
    send_rcom_stateless(msg, rc);
}

unsafe fn receive_sync_reply(ls: *mut dlm_ls, rc_in: *const dlm_rcom) {
    spin_lock_bh(&mut (*ls).ls_rcom_spin);
    if !test_bit(LSFL_RCOM_WAIT, &(*ls).ls_flags) || le64_to_cpu((*rc_in).rc_id) != (*ls).ls_rcom_seq { spin_unlock_bh(&mut (*ls).ls_rcom_spin); return; }
    memcpy((*ls).ls_recover_buf as *mut c_void, rc_in as *const c_void, le16_to_cpu((*rc_in).rc_header.h_length) as usize);
    set_bit(LSFL_RCOM_READY, &mut (*ls).ls_flags); clear_bit(LSFL_RCOM_WAIT, &mut (*ls).ls_flags); wake_up(&mut (*ls).ls_wait_general); spin_unlock_bh(&mut (*ls).ls_rcom_spin);
}

unsafe fn receive_rcom_names(ls: *mut dlm_ls, rc_in: *const dlm_rcom, seq: u64) {
    let nodeid = le32_to_cpu((*rc_in).rc_header.h_nodeid); let inlen = le16_to_cpu((*rc_in).rc_header.h_length) as c_int - core::mem::size_of::<dlm_rcom>() as c_int; let outlen = DLM_MAX_APP_BUFSIZE - core::mem::size_of::<dlm_rcom>() as c_int;
    let mut rc = core::ptr::null_mut(); let mut mh = core::ptr::null_mut(); if create_rcom(ls, nodeid, DLM_RCOM_NAMES_REPLY, outlen, &mut rc, &mut mh, seq) != 0 { return; }
    (*rc).rc_id = (*rc_in).rc_id; (*rc).rc_seq_reply = (*rc_in).rc_seq; dlm_copy_master_names(ls, (*rc_in).rc_buf.as_ptr(), inlen, (*rc).rc_buf.as_mut_ptr(), outlen, nodeid); send_rcom(mh, rc);
}

unsafe fn receive_rcom_lookup(ls: *mut dlm_ls, rc_in: *const dlm_rcom, seq: u64) {
    let nodeid = le32_to_cpu((*rc_in).rc_header.h_nodeid); let len = le16_to_cpu((*rc_in).rc_header.h_length) as c_int - core::mem::size_of::<dlm_rcom>() as c_int;
    if (*rc_in).rc_id == cpu_to_le64(0xFFFF_FFFF) { dlm_dump_rsb_name(ls, (*rc_in).rc_buf.as_ptr(), len); return; }
    let mut rc = core::ptr::null_mut(); let mut mh = core::ptr::null_mut(); if create_rcom(ls, nodeid, DLM_RCOM_LOOKUP_REPLY, 0, &mut rc, &mut mh, seq) != 0 { return; }
    let mut ret_nodeid = 0; let mut unused = 0; let error = dlm_master_lookup(ls, nodeid, (*rc_in).rc_buf.as_ptr(), len, DLM_LU_RECOVER_MASTER, &mut ret_nodeid, &mut unused); if error != 0 { ret_nodeid = error; }
    (*rc).rc_result = cpu_to_le32(ret_nodeid as u32); (*rc).rc_id = (*rc_in).rc_id; (*rc).rc_seq_reply = (*rc_in).rc_seq; send_rcom(mh, rc);
}
unsafe fn receive_rcom_lookup_reply(ls: *mut dlm_ls, rc_in: *const dlm_rcom) { dlm_recover_master_reply(ls, rc_in); }
unsafe fn receive_rcom_lock(ls: *mut dlm_ls, rc_in: *const dlm_rcom, seq: u64) {
    let mut remid = 0u32; let mut result = 0u32; dlm_recover_master_copy(ls, rc_in, &mut remid, &mut result); let nodeid = le32_to_cpu((*rc_in).rc_header.h_nodeid); let mut rc = core::ptr::null_mut(); let mut mh = core::ptr::null_mut(); if create_rcom(ls, nodeid, DLM_RCOM_LOCK_REPLY, core::mem::size_of::<rcom_lock>() as c_int, &mut rc, &mut mh, seq) != 0 { return; } memcpy((*rc).rc_buf.as_mut_ptr() as *mut c_void, (*rc_in).rc_buf.as_ptr() as *const c_void, core::mem::size_of::<rcom_lock>()); let rl = (*rc).rc_buf.as_mut_ptr() as *mut rcom_lock; (*rl).rl_remid = remid; (*rl).rl_result = result; (*rc).rc_id = (*rc_in).rc_id; (*rc).rc_seq_reply = (*rc_in).rc_seq; send_rcom(mh, rc);
}

pub unsafe fn dlm_receive_rcom(ls: *mut dlm_ls, rc: *const dlm_rcom, nodeid: c_int) {
    let typ = le32_to_cpu((*rc).rc_type); let mut reply = false; let mut names = false; let mut lookup = false; let mut lock = false;
    match typ { DLM_RCOM_STATUS_REPLY | DLM_RCOM_NAMES_REPLY | DLM_RCOM_LOOKUP_REPLY | DLM_RCOM_LOCK_REPLY => reply = true, _ => {} }
    match typ { DLM_RCOM_NAMES | DLM_RCOM_NAMES_REPLY => names = true, DLM_RCOM_LOOKUP | DLM_RCOM_LOOKUP_REPLY => lookup = true, DLM_RCOM_LOCK | DLM_RCOM_LOCK_REPLY => lock = true, _ => {} }
    spin_lock_bh(&mut (*ls).ls_recover_lock); let status = (*ls).ls_recover_status; let stop = dlm_recovery_stopped(ls); let seq = (*ls).ls_recover_seq; spin_unlock_bh(&mut (*ls).ls_recover_lock);
    if stop && typ != DLM_RCOM_STATUS || reply && le64_to_cpu((*rc).rc_seq_reply) != seq || status & DLM_RS_NODES == 0 && (names || lookup || lock) || status & DLM_RS_DIR == 0 && (lookup || lock) { return; }
    match typ { DLM_RCOM_STATUS => receive_rcom_status(ls, rc, seq), DLM_RCOM_NAMES => receive_rcom_names(ls, rc, seq), DLM_RCOM_LOOKUP => receive_rcom_lookup(ls, rc, seq), DLM_RCOM_LOCK => receive_rcom_lock(ls, rc, seq), DLM_RCOM_STATUS_REPLY | DLM_RCOM_NAMES_REPLY => receive_sync_reply(ls, rc), DLM_RCOM_LOOKUP_REPLY => receive_rcom_lookup_reply(ls, rc), DLM_RCOM_LOCK_REPLY => dlm_recover_process_copy(ls, rc, seq), _ => log_error(ls, c"receive_rcom bad type %d".as_ptr(), typ) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
