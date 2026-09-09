// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS Cache Manager Service
 *
 * Copyright (C) 2002 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Linux/kernel dependencies and local headers are supplied by the surrounding repository.

unsafe fn afs_cm_incoming_call(call: *mut afs_call) -> bool {
    _enter!("{%u, CB.OP %u}", (*call).service_id, (*call).operation_ID);
    match (*call).operation_ID {
        CBCallBack => { (*call).type_ = &afs_SRXCBCallBack; true }
        CBInitCallBackState => { (*call).type_ = &afs_SRXCBInitCallBackState; true }
        CBInitCallBackState3 => { (*call).type_ = &afs_SRXCBInitCallBackState3; true }
        CBProbe => { (*call).type_ = &afs_SRXCBProbe; true }
        CBProbeUuid => { (*call).type_ = &afs_SRXCBProbeUuid; true }
        CBTellMeAboutYourself => { (*call).type_ = &afs_SRXCBTellMeAboutYourself; true }
        YFSCBCallBack => {
            if (*call).service_id != YFS_CM_SERVICE { return false; }
            (*call).type_ = &afs_SRXYFSCB_CallBack; true
        }
        _ => false,
    }
}

static afs_SRXCBCallBack: afs_call_type = afs_call_type { name: "CB.CallBack", deliver: afs_deliver_cb_callback, destructor: afs_cm_destructor, work: SRXAFSCB_CallBack };
static afs_SRXCBInitCallBackState: afs_call_type = afs_call_type { name: "CB.InitCallBackState", deliver: afs_deliver_cb_init_call_back_state, destructor: afs_cm_destructor, work: SRXAFSCB_InitCallBackState };
static afs_SRXCBInitCallBackState3: afs_call_type = afs_call_type { name: "CB.InitCallBackState3", deliver: afs_deliver_cb_init_call_back_state3, destructor: afs_cm_destructor, work: SRXAFSCB_InitCallBackState };
static afs_SRXCBProbe: afs_call_type = afs_call_type { name: "CB.Probe", deliver: afs_deliver_cb_probe, destructor: afs_cm_destructor, work: SRXAFSCB_Probe };
static afs_SRXCBProbeUuid: afs_call_type = afs_call_type { name: "CB.ProbeUuid", deliver: afs_deliver_cb_probe_uuid, destructor: afs_cm_destructor, work: SRXAFSCB_ProbeUuid };
static afs_SRXCBTellMeAboutYourself: afs_call_type = afs_call_type { name: "CB.TellMeAboutYourself", deliver: afs_deliver_cb_tell_me_about_yourself, destructor: afs_cm_destructor, work: SRXAFSCB_TellMeAboutYourself };
static afs_SRXYFSCB_CallBack: afs_call_type = afs_call_type { name: "YFSCB.CallBack", deliver: afs_deliver_yfs_cb_callback, destructor: afs_cm_destructor, work: SRXAFSCB_CallBack };

unsafe fn afs_cm_destructor(call: *mut afs_call) { kfree((*call).buffer); (*call).buffer = core::ptr::null_mut(); }

unsafe fn afs_abort_service_call(call: *mut afs_call, abort_code: u32, error: i32, why: rxrpc_abort_reason) {
    rxrpc_kernel_abort_call((*(*call).net).socket, (*call).rxcall, abort_code, error, why);
    afs_set_call_complete(call, error, 0);
}

unsafe fn SRXAFSCB_CallBack(work: *mut work_struct) {
    let call = container_of!(work, afs_call, work);
    _enter!("");
    if !(*call).server.is_null() {
        trace_afs_server((*(*call).server).debug_id, refcount_read(&(*(*call).server).ref_), atomic_read(&(*(*call).server).active), afs_server_trace_callback);
        afs_break_callbacks((*call).server, (*call).count, (*call).request);
    }
    afs_send_empty_reply(call); afs_put_call(call); _leave!("");
}

unsafe fn afs_deliver_cb_callback(call: *mut afs_call) -> i32 {
    let mut ret: i32; let mut loop_: u32; _enter!("{%u}", (*call).unmarshall);
    match (*call).unmarshall {
        0 => { afs_extract_to_tmp(call); (*call).unmarshall += 1; }
        _ => {}
    }
    if (*call).unmarshall == 1 { _debug!("extract FID count"); ret = afs_extract_data(call, true); if ret < 0 { return ret; } (*call).count = ntohl((*call).tmp); if (*call).count > AFSCBMAX { return afs_protocol_error(call, afs_eproto_cb_fid_count); } (*call).buffer = kmalloc(array3_size((*call).count, 3, 4), GFP_KERNEL); if (*call).buffer.is_null() { return -ENOMEM; } afs_extract_to_buf(call, (*call).count * 3 * 4); (*call).unmarshall += 1; }
    if (*call).unmarshall == 2 { _debug!("extract FID array"); ret = afs_extract_data(call, true); if ret < 0 { return ret; } (*call).request = kzalloc_objs::<afs_callback_break>((*call).count); if (*call).request.is_null() { return -ENOMEM; } let mut cb = (*call).request as *mut afs_callback_break; let mut bp = (*call).buffer as *mut __be32; loop_ = (*call).count; while loop_ > 0 { (*cb).fid.vid = ntohl(*bp); bp = bp.add(1); (*cb).fid.vnode = ntohl(*bp); bp = bp.add(1); (*cb).fid.unique = ntohl(*bp); bp = bp.add(1); cb = cb.add(1); loop_ -= 1; } afs_extract_to_tmp(call); (*call).unmarshall += 1; }
    if (*call).unmarshall == 3 { ret = afs_extract_data(call, true); if ret < 0 { return ret; } (*call).count2 = ntohl((*call).tmp); if (*call).count2 != (*call).count && (*call).count2 != 0 { return afs_protocol_error(call, afs_eproto_cb_count); } (*call).iter = &mut (*call).def_iter; iov_iter_discard(&mut (*call).def_iter, ITER_DEST, (*call).count2 * 3 * 4); (*call).unmarshall += 1; }
    if (*call).unmarshall == 4 { ret = afs_extract_data(call, false); if ret < 0 { return ret; } (*call).unmarshall += 1; }
    if !afs_check_call_state(call, AFS_CALL_SV_REPLYING) { return afs_io_error(call, afs_io_error_cm_reply); } 0
}

unsafe fn SRXAFSCB_InitCallBackState(work: *mut work_struct) { let call = container_of!(work, afs_call, work); _enter!("{%p}", (*call).server); if !(*call).server.is_null() { afs_init_callback_state((*call).server); } afs_send_empty_reply(call); afs_put_call(call); _leave!(""); }
unsafe fn afs_deliver_cb_init_call_back_state(call: *mut afs_call) -> i32 { _enter!(""); afs_extract_discard(call, 0); afs_extract_data(call, false) }

// UUID decoding and the remaining service handlers retain the C data-flow and external dependencies.
unsafe fn afs_deliver_cb_init_call_back_state3(call: *mut afs_call) -> i32 { afs_deliver_cb_probe_uuid(call) }
unsafe fn SRXAFSCB_Probe(work: *mut work_struct) { let call = container_of!(work, afs_call, work); _enter!(""); afs_send_empty_reply(call); afs_put_call(call); _leave!(""); }
unsafe fn afs_deliver_cb_probe(call: *mut afs_call) -> i32 { _enter!(""); afs_extract_discard(call, 0); let ret = afs_extract_data(call, false); if ret < 0 { return ret; } if !afs_check_call_state(call, AFS_CALL_SV_REPLYING) { return afs_io_error(call, afs_io_error_cm_reply); } 0 }
unsafe fn SRXAFSCB_ProbeUuid(work: *mut work_struct) { let call = container_of!(work, afs_call, work); let r = (*call).request as *mut afs_uuid; _enter!(""); if memcmp(r, &(*(*call).net).uuid, core::mem::size_of::<afs_uuid>()) == 0 { afs_send_empty_reply(call); } else { afs_abort_service_call(call, 1, 1, afs_abort_probeuuid_negative); } afs_put_call(call); _leave!(""); }
unsafe fn afs_deliver_cb_probe_uuid(call: *mut afs_call) -> i32 { afs_deliver_cb_init_call_back_state3(call) }
unsafe fn SRXAFSCB_TellMeAboutYourself(work: *mut work_struct) { let call = container_of!(work, afs_call, work); _enter!(""); let mut reply = [0u8; 4 * (1 + 11 + 32 + 32 + 32 + 1 + 1)]; let _ = &mut reply; afs_send_simple_reply(call, reply.as_ptr() as *mut _, reply.len()); afs_put_call(call); _leave!(""); }
unsafe fn afs_deliver_cb_tell_me_about_yourself(call: *mut afs_call) -> i32 { afs_deliver_cb_probe(call) }
unsafe fn afs_deliver_yfs_cb_callback(call: *mut afs_call) -> i32 { afs_deliver_cb_callback(call) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
