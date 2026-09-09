// SPDX-License-Identifier: GPL-2.0-only
/*
 * The NFC Controller Interface is the communication protocol between an
 * NFC Controller (NFCC) and a Device Host (DH).
 * This is the HCI over NCI implementation, as specified in section 10.2
 * of the NCI 1.1 specification.
 *
 * Copyright (C) 2014 STMicroelectronics SAS. All rights reserved.
 */

#[repr(C, packed)]
struct nci_data { conn_id: u8, pipe: u8, cmd: u8, data: *const u8, data_len: u32 }
#[repr(C, packed)]
struct nci_hci_create_pipe_params { src_gate: u8, dest_host: u8, dest_gate: u8 }
#[repr(C, packed)]
struct nci_hci_create_pipe_resp { src_host: u8, src_gate: u8, dest_host: u8, dest_gate: u8, pipe: u8 }
#[repr(C, packed)]
struct nci_hci_delete_pipe_noti { pipe: u8 }
#[repr(C, packed)]
struct nci_hci_all_pipe_cleared_noti { host: u8 }
#[repr(C, packed)]
struct nci_hcp_message { header: u8, data: [u8; 0] }
#[repr(C, packed)]
struct nci_hcp_packet { header: u8, message: nci_hcp_message }

const NCI_HCI_ANY_SET_PARAMETER: u8 = 0x01;
const NCI_HCI_ANY_GET_PARAMETER: u8 = 0x02;
const NCI_HCI_ANY_CLOSE_PIPE: u8 = 0x04;
const NCI_HCI_ADM_CLEAR_ALL_PIPE: u8 = 0x14;
const NCI_HFP_NO_CHAINING: u8 = 0x80;
const NCI_NFCEE_ID_HCI: u8 = 0x80;
const NCI_EVT_HOT_PLUG: u8 = 0x03;
const NCI_HCI_ADMIN_PARAM_SESSION_IDENTITY: u8 = 0x01;
const NCI_HCI_ADM_CREATE_PIPE: u8 = 0x10;
const NCI_HCI_ADM_DELETE_PIPE: u8 = 0x11;
const NCI_HCI_HCP_PACKET_HEADER_LEN: usize = 1;
const NCI_HCI_HCP_MESSAGE_HEADER_LEN: usize = 1;
const NCI_HCI_HCP_HEADER_LEN: usize = 2;
const NCI_HCI_HCP_COMMAND: u8 = 0x00;
const NCI_HCI_HCP_EVENT: u8 = 0x01;
const NCI_HCI_HCP_RESPONSE: u8 = 0x02;
const NCI_HCI_ADM_NOTIFY_PIPE_CREATED: u8 = 0x12;
const NCI_HCI_ADM_NOTIFY_PIPE_DELETED: u8 = 0x13;
const NCI_HCI_ADM_NOTIFY_ALL_PIPE_CLEARED: u8 = 0x15;
const NCI_HCI_FRAGMENT: u8 = 0x7f;

#[inline] const fn NCI_HCP_HEADER(ty: u8, instr: u8) -> u8 { ((ty & 0x03) << 6) | (instr & 0x3f) }
#[inline] const fn NCI_HCP_MSG_GET_TYPE(header: u8) -> u8 { (header & 0xc0) >> 6 }
#[inline] const fn NCI_HCP_MSG_GET_CMD(header: u8) -> u8 { header & 0x3f }
#[inline] const fn NCI_HCP_MSG_GET_PIPE(header: u8) -> u8 { header & 0x7f }

unsafe fn nci_hci_result_to_errno(result: u8) -> i32 {
    match result { NCI_HCI_ANY_OK => 0, NCI_HCI_ANY_E_REG_PAR_UNKNOWN => -EOPNOTSUPP, NCI_HCI_ANY_E_TIMEOUT => -ETIME, _ => -1 }
}

unsafe fn nci_hci_reset_pipes(hdev: *mut nci_hci_dev) {
    for i in 0..NCI_HCI_MAX_PIPES { (*hdev).pipes[i].gate = NCI_HCI_INVALID_GATE; (*hdev).pipes[i].host = NCI_HCI_INVALID_HOST; }
    core::ptr::write_bytes((*hdev).gate2pipe.as_mut_ptr(), NCI_HCI_INVALID_PIPE, (*hdev).gate2pipe.len());
}

unsafe fn nci_hci_reset_pipes_per_host(ndev: *mut nci_dev, host: u8) {
    for i in 0..NCI_HCI_MAX_PIPES { if (*(*ndev).hci_dev).pipes[i].host == host { (*(*ndev).hci_dev).pipes[i].gate = NCI_HCI_INVALID_GATE; (*(*ndev).hci_dev).pipes[i].host = NCI_HCI_INVALID_HOST; } }
}

unsafe fn nci_hci_send_data(ndev: *mut nci_dev, pipe: u8, data_type: u8, data: *const u8, data_len: usize) -> i32 {
    let conn_info = (*(*ndev).hci_dev).conn_info;
    if conn_info.is_null() { return -EPROTO; }
    let mut i = 0usize; let mut skb = nci_skb_alloc(ndev, (*conn_info).max_pkt_payload_len + NCI_DATA_HDR_SIZE, GFP_ATOMIC);
    if skb.is_null() { return -ENOMEM; }
    skb_reserve(skb, NCI_DATA_HDR_SIZE + 2); *(skb_push(skb, 1) as *mut u8) = data_type; let mut cb = pipe;
    loop {
        let len: usize;
        if i + (*conn_info).max_pkt_payload_len - ((*skb).len + 1) >= data_len { cb |= NCI_HFP_NO_CHAINING; len = data_len - i; } else { len = (*conn_info).max_pkt_payload_len - (*skb).len - 1; }
        *(skb_push(skb, 1) as *mut u8) = cb;
        if len > 0 { skb_put_data(skb, data.add(i), len); }
        let r = nci_send_data(ndev, (*conn_info).conn_id, skb); if r < 0 { return r; }
        i += len; if i >= data_len { break; }
        skb = nci_skb_alloc(ndev, (*conn_info).max_pkt_payload_len + NCI_DATA_HDR_SIZE, GFP_ATOMIC); if skb.is_null() { return -ENOMEM; }
        skb_reserve(skb, NCI_DATA_HDR_SIZE + 1);
    } i as i32
}

unsafe fn nci_hci_send_data_req(ndev: *mut nci_dev, opt: *const core::ffi::c_void) { let d = &*(opt as *const nci_data); nci_hci_send_data(ndev, d.pipe, d.cmd, d.data, d.data_len as usize); }

pub unsafe fn nci_hci_send_event(ndev: *mut nci_dev, gate: u8, event: u8, param: *const u8, param_len: usize) -> i32 { let pipe = (*(*ndev).hci_dev).gate2pipe[gate as usize]; if pipe == NCI_HCI_INVALID_PIPE { return -EADDRNOTAVAIL; } nci_hci_send_data(ndev, pipe, NCI_HCP_HEADER(NCI_HCI_HCP_EVENT, event), param, param_len) }

pub unsafe fn nci_hci_send_cmd(ndev: *mut nci_dev, gate: u8, cmd: u8, param: *const u8, param_len: usize, skb_out: *mut *mut sk_buff) -> i32 {
    let pipe = (*(*ndev).hci_dev).gate2pipe[gate as usize]; if pipe == NCI_HCI_INVALID_PIPE { return -EADDRNOTAVAIL; }
    let conn_info = (*(*ndev).hci_dev).conn_info; if conn_info.is_null() { return -EPROTO; }
    let data = nci_data { conn_id: (*conn_info).conn_id, pipe, cmd: NCI_HCP_HEADER(NCI_HCI_HCP_COMMAND, cmd), data: param, data_len: param_len as u32 };
    let mut r = nci_request(ndev, Some(nci_hci_send_data_req), &data as *const _ as *const _, msecs_to_jiffies(NCI_DATA_TIMEOUT));
    if r == NCI_STATUS_OK { let message = (*conn_info).rx_skb as *const nci_hcp_message; r = nci_hci_result_to_errno((*message).header & 0x3f); skb_pull((*conn_info).rx_skb, NCI_HCI_HCP_MESSAGE_HEADER_LEN); if r == 0 && !skb_out.is_null() { *skb_out = (*conn_info).rx_skb; } } r
}

pub unsafe fn nci_hci_clear_all_pipes(ndev: *mut nci_dev) -> i32 { let r = nci_hci_send_cmd(ndev, NCI_HCI_ADMIN_GATE, NCI_HCI_ADM_CLEAR_ALL_PIPE, core::ptr::null(), 0, core::ptr::null_mut()); if r < 0 { return r; } nci_hci_reset_pipes((*ndev).hci_dev); r }

unsafe fn nci_hci_event_received(ndev: *mut nci_dev, pipe: u8, event: u8, skb: *mut sk_buff) { if let Some(f) = (*ndev).ops.hci_event_received { f(ndev, pipe, event, skb); } }
unsafe fn nci_hci_cmd_received(ndev: *mut nci_dev, pipe: u8, cmd: u8, skb: *mut sk_buff) {
    let gate = (*(*ndev).hci_dev).pipes[pipe as usize].gate; let mut status = NCI_HCI_ANY_OK | !NCI_HCI_FRAGMENT;
    match cmd {
        NCI_HCI_ADM_NOTIFY_PIPE_CREATED => { if (*skb).len != 5 { status = NCI_HCI_ANY_E_NOK; } else { let p = &*( (*skb).data as *const nci_hci_create_pipe_resp); if p.pipe >= NCI_HCI_MAX_PIPES { status = NCI_HCI_ANY_E_NOK; } else { (*(*ndev).hci_dev).gate2pipe[p.dest_gate as usize] = p.pipe; (*(*ndev).hci_dev).pipes[p.pipe as usize].gate = p.dest_gate; (*(*ndev).hci_dev).pipes[p.pipe as usize].host = p.src_host; } } }
        NCI_HCI_ANY_OPEN_PIPE => { if gate == NCI_HCI_INVALID_GATE { status = NCI_HCI_ANY_E_NOK; } }
        NCI_HCI_ADM_NOTIFY_PIPE_DELETED => { if (*skb).len != 1 { status = NCI_HCI_ANY_E_NOK; } else { let p = *( (*skb).data as *const nci_hci_delete_pipe_noti); if p.pipe >= NCI_HCI_MAX_PIPES { status = NCI_HCI_ANY_E_NOK; } else { (*(*ndev).hci_dev).pipes[p.pipe as usize].gate = NCI_HCI_INVALID_GATE; (*(*ndev).hci_dev).pipes[p.pipe as usize].host = NCI_HCI_INVALID_HOST; } } }
        NCI_HCI_ADM_NOTIFY_ALL_PIPE_CLEARED => { if (*skb).len != 1 { status = NCI_HCI_ANY_E_NOK; } else { nci_hci_reset_pipes_per_host(ndev, (*( (*skb).data as *const nci_hci_all_pipe_cleared_noti)).host); } }
        _ => {}
    }
    if let Some(f) = (*ndev).ops.hci_cmd_received { f(ndev, pipe, cmd, skb); }
    nci_hci_send_data(ndev, pipe, status, core::ptr::null(), 0); kfree_skb(skb);
}

unsafe fn nci_hci_resp_received(ndev: *mut nci_dev, skb: *mut sk_buff) { let ci = (*(*ndev).hci_dev).conn_info; if !ci.is_null() { (*ci).rx_skb = skb; } nci_req_complete(ndev, NCI_STATUS_OK); }
unsafe fn nci_hci_hcp_message_rx(ndev: *mut nci_dev, pipe: u8, ty: u8, instruction: u8, skb: *mut sk_buff) { match ty { NCI_HCI_HCP_RESPONSE => nci_hci_resp_received(ndev, skb), NCI_HCI_HCP_COMMAND => nci_hci_cmd_received(ndev, pipe, instruction, skb), NCI_HCI_HCP_EVENT => nci_hci_event_received(ndev, pipe, instruction, skb), _ => kfree_skb(skb) } nci_req_complete(ndev, NCI_STATUS_OK); }

unsafe fn nci_hci_msg_rx_work(work: *mut work_struct) { let hdev = container_of(work, nci_hci_dev, msg_rx_work); let mut skb; while { skb = skb_dequeue(&mut (*hdev).msg_rx_queue); !skb.is_null() } { kcov_remote_start_common(skb_get_kcov_handle(skb)); let pipe = NCI_HCP_MSG_GET_PIPE(*(*skb).data); skb_pull(skb, 1); let m = (*skb).data as *const nci_hcp_message; let ty = NCI_HCP_MSG_GET_TYPE((*m).header); let instruction = NCI_HCP_MSG_GET_CMD((*m).header); skb_pull(skb, 1); nci_hci_hcp_message_rx((*hdev).ndev, pipe, ty, instruction, skb); kcov_remote_stop(); } }

pub unsafe fn nci_hci_data_received_cb(context: *mut core::ffi::c_void, skb: *mut sk_buff, err: i32) { let ndev = context as *mut nci_dev; if err != 0 { nci_req_complete(ndev, err); return; } if !pskb_may_pull(skb, 1) { kfree_skb(skb); return; } let packet = (*skb).data as *mut nci_hcp_packet; if ((*packet).header & !NCI_HCI_FRAGMENT) == 0 { skb_queue_tail(&mut (*(*ndev).hci_dev).rx_hcp_frags, skb); return; } if skb_queue_len(&(*(*ndev).hci_dev).rx_hcp_frags) != 0 { let pipe = NCI_HCP_MSG_GET_PIPE((*packet).header); skb_queue_tail(&mut (*(*ndev).hci_dev).rx_hcp_frags, skb); let mut msg_len = 0; let mut frag = core::ptr::null_mut(); skb_queue_walk(&(*(*ndev).hci_dev).rx_hcp_frags, frag) { msg_len += (*frag).len - 1; } let hcp = nfc_alloc_recv_skb(1 + msg_len, GFP_KERNEL); if hcp.is_null() { nci_req_complete(ndev, -ENOMEM); return; } skb_put_u8(hcp, pipe); skb_queue_walk(&(*(*ndev).hci_dev).rx_hcp_frags, frag) { let n = (*frag).len - 1; skb_put_data(hcp, (*frag).data.add(1), n); } skb_queue_purge(&mut (*(*ndev).hci_dev).rx_hcp_frags); skb = hcp; } else { (*packet).header &= NCI_HCI_FRAGMENT; } if !pskb_may_pull(skb, 2) { kfree_skb(skb); return; } let packet = (*skb).data as *mut nci_hcp_packet; let ty = NCI_HCP_MSG_GET_TYPE((*packet).message.header); if ty == NCI_HCI_HCP_RESPONSE { let pipe = NCI_HCP_MSG_GET_PIPE((*packet).header); skb_pull(skb, 1); nci_hci_hcp_message_rx(ndev, pipe, ty, NCI_STATUS_OK as u8, skb); } else { skb_queue_tail(&mut (*(*ndev).hci_dev).msg_rx_queue, skb); schedule_work(&mut (*(*ndev).hci_dev).msg_rx_work); } }

pub unsafe fn nci_hci_open_pipe(ndev: *mut nci_dev, pipe: u8) -> i32 { let ci = (*(*ndev).hci_dev).conn_info; if ci.is_null() { return -EPROTO; } let d = nci_data { conn_id: (*ci).conn_id, pipe, cmd: NCI_HCP_HEADER(NCI_HCI_HCP_COMMAND, NCI_HCI_ANY_OPEN_PIPE), data: core::ptr::null(), data_len: 0 }; nci_request(ndev, Some(nci_hci_send_data_req), &d as *const _ as *const _, msecs_to_jiffies(NCI_DATA_TIMEOUT)) }
unsafe fn nci_hci_create_pipe(ndev: *mut nci_dev, dest_host: u8, dest_gate: u8, result: *mut i32) -> u8 { let params = nci_hci_create_pipe_params { src_gate: NCI_HCI_ADMIN_GATE, dest_host, dest_gate }; let mut skb = core::ptr::null_mut(); *result = nci_hci_send_cmd(ndev, NCI_HCI_ADMIN_GATE, NCI_HCI_ADM_CREATE_PIPE, &params as *const _ as *const u8, 3, &mut skb); if *result < 0 { return NCI_HCI_INVALID_PIPE; } let pipe = (*( (*skb).data as *const nci_hci_create_pipe_resp)).pipe; kfree_skb(skb); if pipe >= NCI_HCI_MAX_PIPES { NCI_HCI_INVALID_PIPE } else { pipe } }
unsafe fn nci_hci_delete_pipe(ndev: *mut nci_dev, pipe: u8) -> i32 { nci_hci_send_cmd(ndev, NCI_HCI_ADMIN_GATE, NCI_HCI_ADM_DELETE_PIPE, &pipe, 1, core::ptr::null_mut()) }
pub unsafe fn nci_hci_set_param(ndev: *mut nci_dev, gate: u8, idx: u8, param: *const u8, param_len: usize) -> i32 { let pipe = (*(*ndev).hci_dev).gate2pipe[gate as usize]; if pipe == NCI_HCI_INVALID_PIPE { return -EADDRNOTAVAIL; } let ci = (*(*ndev).hci_dev).conn_info; if ci.is_null() { return -EPROTO; } let tmp = kmalloc(1 + param_len, GFP_KERNEL); if tmp.is_null() { return -ENOMEM; } *tmp = idx; core::ptr::copy_nonoverlapping(param, tmp.add(1), param_len); let d = nci_data { conn_id: (*ci).conn_id, pipe, cmd: NCI_HCP_HEADER(NCI_HCI_HCP_COMMAND, NCI_HCI_ANY_SET_PARAMETER), data: tmp, data_len: (param_len + 1) as u32 }; let mut r = nci_request(ndev, Some(nci_hci_send_data_req), &d as *const _ as *const _, msecs_to_jiffies(NCI_DATA_TIMEOUT)); if r == NCI_STATUS_OK { r = nci_hci_result_to_errno((*((*ci).rx_skb as *const nci_hcp_message)).header & 0x3f); skb_pull((*ci).rx_skb, 1); } kfree(tmp); r }
pub unsafe fn nci_hci_get_param(ndev: *mut nci_dev, gate: u8, idx: u8, out: *mut *mut sk_buff) -> i32 { let pipe = (*(*ndev).hci_dev).gate2pipe[gate as usize]; if pipe == NCI_HCI_INVALID_PIPE { return -EADDRNOTAVAIL; } let ci = (*(*ndev).hci_dev).conn_info; if ci.is_null() { return -EPROTO; } let d = nci_data { conn_id: (*ci).conn_id, pipe, cmd: NCI_HCP_HEADER(NCI_HCI_HCP_COMMAND, NCI_HCI_ANY_GET_PARAMETER), data: &idx, data_len: 1 }; let mut r = nci_request(ndev, Some(nci_hci_send_data_req), &d as *const _ as *const _, msecs_to_jiffies(NCI_DATA_TIMEOUT)); if r == NCI_STATUS_OK { r = nci_hci_result_to_errno((*((*ci).rx_skb as *const nci_hcp_message)).header & 0x3f); skb_pull((*ci).rx_skb, 1); if r == 0 && !out.is_null() { *out = (*ci).rx_skb; } } r }
pub unsafe fn nci_hci_connect_gate(ndev: *mut nci_dev, dest_host: u8, dest_gate: u8, mut pipe: u8) -> i32 { if pipe == NCI_HCI_DO_NOT_OPEN_PIPE { return 0; } if (*(*ndev).hci_dev).gate2pipe[dest_gate as usize] != NCI_HCI_INVALID_PIPE { return -EADDRINUSE; } let mut created = false; let mut r = 0; if pipe == NCI_HCI_INVALID_PIPE { pipe = match dest_gate { NCI_HCI_LINK_MGMT_GATE => NCI_HCI_LINK_MGMT_PIPE, NCI_HCI_ADMIN_GATE => NCI_HCI_ADMIN_PIPE, _ => { created = true; nci_hci_create_pipe(ndev, dest_host, dest_gate, &mut r) } }; if pipe == NCI_HCI_INVALID_PIPE { return r; } } r = nci_hci_open_pipe(ndev, pipe); if r < 0 { if created { let _ = nci_hci_delete_pipe(ndev, pipe); } return r; } (*(*ndev).hci_dev).pipes[pipe as usize].gate = dest_gate; (*(*ndev).hci_dev).pipes[pipe as usize].host = dest_host; (*(*ndev).hci_dev).gate2pipe[dest_gate as usize] = pipe; 0 }
unsafe fn nci_hci_dev_connect_gates(ndev: *mut nci_dev, mut count: u8, mut gates: *const nci_hci_gate) -> i32 { while count != 0 { let r = nci_hci_connect_gate(ndev, (*gates).dest_host, (*gates).gate, (*gates).pipe); if r < 0 { return r; } count -= 1; gates = gates.add(1); } 0 }
pub unsafe fn nci_hci_dev_session_init(ndev: *mut nci_dev) -> i32 { (*(*ndev).hci_dev).count_pipes = 0; (*(*ndev).hci_dev).expected_pipes = 0; let ci = (*(*ndev).hci_dev).conn_info; if ci.is_null() { return -EPROTO; } (*ci).data_exchange_cb = Some(nci_hci_data_received_cb); (*ci).data_exchange_cb_context = ndev as *mut _; nci_hci_reset_pipes((*ndev).hci_dev); let init = &(*(*ndev).hci_dev).init_data; if init.gates[0].gate != NCI_HCI_ADMIN_GATE { return -EPROTO; } let mut r = nci_hci_connect_gate(ndev, init.gates[0].dest_host, init.gates[0].gate, init.gates[0].pipe); if r < 0 { return r; } let mut skb = core::ptr::null_mut(); r = nci_hci_get_param(ndev, NCI_HCI_ADMIN_GATE, NCI_HCI_ADMIN_PARAM_SESSION_IDENTITY, &mut skb); if r < 0 { return r; } if (*skb).len != 0 && (*skb).len == strlen(init.session_id) && memcmp(init.session_id, (*skb).data, (*skb).len) == 0 { if let Some(f) = (*ndev).ops.hci_load_session { r = f(ndev); } } else { r = nci_hci_clear_all_pipes(ndev); if r >= 0 { r = nci_hci_dev_connect_gates(ndev, init.gate_count, init.gates); } if r >= 0 { r = nci_hci_set_param(ndev, NCI_HCI_ADMIN_GATE, NCI_HCI_ADMIN_PARAM_SESSION_IDENTITY, init.session_id, strlen(init.session_id)); } } kfree_skb(skb); r }
pub unsafe fn nci_hci_allocate(ndev: *mut nci_dev) -> *mut nci_hci_dev { let hdev = kzalloc_obj::<nci_hci_dev>(); if hdev.is_null() { return core::ptr::null_mut(); } skb_queue_head_init(&mut (*hdev).rx_hcp_frags); INIT_WORK(&mut (*hdev).msg_rx_work, Some(nci_hci_msg_rx_work)); skb_queue_head_init(&mut (*hdev).msg_rx_queue); (*hdev).ndev = ndev; hdev }
pub unsafe fn nci_hci_deallocate(ndev: *mut nci_dev) { kfree((*ndev).hci_dev as *mut _); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
