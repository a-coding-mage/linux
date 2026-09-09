// SPDX-License-Identifier: GPL-2.0-only
/*
 *  The NFC Controller Interface is the communication protocol between an
 *  NFC Controller (NFCC) and a Device Host (DH).
 *
 *  Copyright (C) 2011 Texas Instruments, Inc.
 *
 *  Written by Ilan Elias <ilane@ti.com>
 *
 *  Acknowledgements:
 *  This file is based on hci_event.c, which was written
 *  by Maxim Krasnyansky.
 */

/* Dependencies supplied by the surrounding kernel/NCI translation. */

unsafe fn nci_core_reset_rsp_packet(ndev: *mut nci_dev, skb: *const sk_buff) {
    let rsp = (*skb).data as *const nci_core_reset_rsp;

    pr_debug!("status 0x{:x}\n", (*rsp).status);

    /* Handle NCI 1.x ver */
    if (*skb).len != 1 {
        if (*rsp).status == NCI_STATUS_OK {
            (*ndev).nci_ver = (*rsp).nci_ver;
            pr_debug!("nci_ver 0x{:x}, config_status 0x{:x}\n", (*rsp).nci_ver, (*rsp).config_status);
        }

        nci_req_complete(ndev, (*rsp).status);
    }
}

unsafe fn nci_core_init_rsp_packet_v1(ndev: *mut nci_dev, skb: *const sk_buff) -> u8 {
    let rsp_1 = (*skb).data as *const nci_core_init_rsp_1;
    let mut rsp_2: *const nci_core_init_rsp_2;

    /* Ensure that the status field can be accessed. */
    if skb_headlen(skb) < 1 { return NCI_STATUS_SYNTAX_ERROR; }
    pr_debug!("status 0x{:x}\n", (*rsp_1).status);
    if (*rsp_1).status != NCI_STATUS_OK { return (*rsp_1).status; }
    if skb_headlen(skb) < core::mem::size_of::<nci_core_init_rsp_1>() { return NCI_STATUS_SYNTAX_ERROR; }
    if skb_headlen(skb) < core::mem::size_of::<nci_core_init_rsp_1>() + (*rsp_1).num_supported_rf_interfaces as usize + core::mem::size_of::<nci_core_init_rsp_2>() { return NCI_STATUS_SYNTAX_ERROR; }

    (*ndev).nfcc_features = __le32_to_cpu((*rsp_1).nfcc_features);
    (*ndev).num_supported_rf_interfaces = (*rsp_1).num_supported_rf_interfaces;
    (*ndev).num_supported_rf_interfaces = core::cmp::min((*ndev).num_supported_rf_interfaces as i32, NCI_MAX_SUPPORTED_RF_INTERFACES) as _;
    core::ptr::copy_nonoverlapping((*rsp_1).supported_rf_interfaces.as_ptr(), (*ndev).supported_rf_interfaces.as_mut_ptr(), (*ndev).num_supported_rf_interfaces as usize);

    rsp_2 = ((*skb).data.add(6 + (*rsp_1).num_supported_rf_interfaces as usize)) as *const nci_core_init_rsp_2;
    (*ndev).max_logical_connections = (*rsp_2).max_logical_connections;
    (*ndev).max_routing_table_size = __le16_to_cpu((*rsp_2).max_routing_table_size);
    (*ndev).max_ctrl_pkt_payload_len = (*rsp_2).max_ctrl_pkt_payload_len;
    (*ndev).max_size_for_large_params = __le16_to_cpu((*rsp_2).max_size_for_large_params);
    (*ndev).manufact_id = (*rsp_2).manufact_id;
    (*ndev).manufact_specific_info = __le32_to_cpu((*rsp_2).manufact_specific_info);
    NCI_STATUS_OK
}

unsafe fn nci_core_init_rsp_packet_v2(ndev: *mut nci_dev, skb: *const sk_buff) -> u8 {
    let rsp = (*skb).data as *const nci_core_init_rsp_nci_ver2;
    let mut supported_rf_interface: *const u8;
    let mut rf_interface_idx: u8 = 0;
    let mut rf_extension_cnt: u8 = 0;

    if skb_headlen(skb) < 1 { return NCI_STATUS_SYNTAX_ERROR; }
    pr_debug!("status {:x}\n", (*rsp).status);
    if (*rsp).status != NCI_STATUS_OK { return (*rsp).status; }
    if skb_headlen(skb) < core::mem::size_of::<nci_core_init_rsp_nci_ver2>() { return NCI_STATUS_SYNTAX_ERROR; }

    supported_rf_interface = (*rsp).supported_rf_interfaces.as_ptr();
    (*ndev).nfcc_features = __le32_to_cpu((*rsp).nfcc_features);
    (*ndev).num_supported_rf_interfaces = (*rsp).num_supported_rf_interfaces;
    (*ndev).num_supported_rf_interfaces = core::cmp::min((*ndev).num_supported_rf_interfaces as i32, NCI_MAX_SUPPORTED_RF_INTERFACES) as _;

    while rf_interface_idx < (*ndev).num_supported_rf_interfaces {
        if supported_rf_interface.add(2) > skb_tail_pointer(skb) { break; }
        *(*ndev).supported_rf_interfaces.as_mut_ptr().add(rf_interface_idx as usize) = *supported_rf_interface;
        supported_rf_interface = supported_rf_interface.add(1);
        rf_extension_cnt = *supported_rf_interface;
        supported_rf_interface = supported_rf_interface.add(1);
        if supported_rf_interface.add(rf_extension_cnt as usize) > skb_tail_pointer(skb) { break; }
        rf_interface_idx += 1;
        supported_rf_interface = supported_rf_interface.add(rf_extension_cnt as usize);
    }
    (*ndev).num_supported_rf_interfaces = rf_interface_idx;
    (*ndev).max_logical_connections = (*rsp).max_logical_connections;
    (*ndev).max_routing_table_size = __le16_to_cpu((*rsp).max_routing_table_size);
    (*ndev).max_ctrl_pkt_payload_len = (*rsp).max_ctrl_pkt_payload_len;
    (*ndev).max_size_for_large_params = NCI_MAX_LARGE_PARAMS_NCI_v2;
    NCI_STATUS_OK
}

unsafe fn nci_core_init_rsp_packet(ndev: *mut nci_dev, skb: *const sk_buff) {
    let status = if (*ndev).nci_ver & NCI_VER_2_MASK == 0 { nci_core_init_rsp_packet_v1(ndev, skb) } else { nci_core_init_rsp_packet_v2(ndev, skb) };
    if status != NCI_STATUS_OK { nci_req_complete(ndev, status); return; }
    pr_debug!("nfcc_features 0x{:x}\n", (*ndev).nfcc_features);
    pr_debug!("num_supported_rf_interfaces {}\n", (*ndev).num_supported_rf_interfaces);
    pr_debug!("supported_rf_interfaces[0] 0x{:x}\n", (*ndev).supported_rf_interfaces[0]);
    pr_debug!("supported_rf_interfaces[1] 0x{:x}\n", (*ndev).supported_rf_interfaces[1]);
    pr_debug!("supported_rf_interfaces[2] 0x{:x}\n", (*ndev).supported_rf_interfaces[2]);
    pr_debug!("supported_rf_interfaces[3] 0x{:x}\n", (*ndev).supported_rf_interfaces[3]);
    pr_debug!("max_logical_connections {}\n", (*ndev).max_logical_connections);
    pr_debug!("max_routing_table_size {}\n", (*ndev).max_routing_table_size);
    pr_debug!("max_ctrl_pkt_payload_len {}\n", (*ndev).max_ctrl_pkt_payload_len);
    pr_debug!("max_size_for_large_params {}\n", (*ndev).max_size_for_large_params);
    pr_debug!("manufact_id 0x{:x}\n", (*ndev).manufact_id);
    pr_debug!("manufact_specific_info 0x{:x}\n", (*ndev).manufact_specific_info);
    nci_req_complete(ndev, status);
}

unsafe fn nci_core_set_config_rsp_packet(ndev: *mut nci_dev, skb: *const sk_buff) { nci_req_complete(ndev, (*(skb.data as *const nci_core_set_config_rsp)).status); }
unsafe fn nci_rf_disc_map_rsp_packet(ndev: *mut nci_dev, skb: *const sk_buff) { nci_req_complete(ndev, (*skb).data[0]); }

unsafe fn nci_rf_disc_rsp_packet(ndev: *mut nci_dev, skb: *const sk_buff) {
    let mut status = (*skb).data[0];
    pr_debug!("status 0x{:x}\n", status);
    if status == NCI_STATUS_OK {
        atomic_set(&mut (*ndev).state, NCI_DISCOVERY);
        let mut conn_info = (*ndev).rf_conn_info;
        if conn_info.is_null() {
            conn_info = devm_kzalloc(&mut (*(*ndev).nfc_dev).dev, core::mem::size_of::<nci_conn_info>(), GFP_KERNEL);
            if conn_info.is_null() { status = NCI_STATUS_REJECTED; } else {
                (*conn_info).conn_id = NCI_STATIC_RF_CONN_ID;
                INIT_LIST_HEAD(&mut (*conn_info).list);
                list_add(&mut (*conn_info).list, &mut (*ndev).conn_info_list);
                (*ndev).rf_conn_info = conn_info;
            }
        }
    }
    nci_req_complete(ndev, status);
}

unsafe fn nci_rf_disc_select_rsp_packet(ndev: *mut nci_dev, skb: *const sk_buff) { let status = (*skb).data[0]; if status != NCI_STATUS_OK { nci_req_complete(ndev, status); } }
unsafe fn nci_rf_deactivate_rsp_packet(ndev: *mut nci_dev, skb: *const sk_buff) { let status = (*skb).data[0]; if status != NCI_STATUS_OK || atomic_read(&(*ndev).state) != NCI_POLL_ACTIVE { nci_clear_target_list(ndev); atomic_set(&mut (*ndev).state, NCI_IDLE); nci_req_complete(ndev, status); } }
unsafe fn nci_nfcee_discover_rsp_packet(ndev: *mut nci_dev, skb: *const sk_buff) { if (*skb).len != 2 { nci_req_complete(ndev, NCI_STATUS_NFCEE_PROTOCOL_ERROR); return; } let rsp = (*skb).data as *const nci_nfcee_discover_rsp; if (*rsp).status != NCI_STATUS_OK || (*rsp).num_nfcee == 0 { nci_req_complete(ndev, (*rsp).status); } }
unsafe fn nci_nfcee_mode_set_rsp_packet(ndev: *mut nci_dev, skb: *const sk_buff) { nci_req_complete(ndev, (*skb).data[0]); }

unsafe fn nci_core_conn_create_rsp_packet(ndev: *mut nci_dev, skb: *const sk_buff) {
    let mut status = (*skb).data[0];
    let mut conn_info: *mut nci_conn_info = core::ptr::null_mut();
    if status == NCI_STATUS_OK {
        let rsp = (*skb).data as *const nci_core_conn_create_rsp;
        conn_info = devm_kzalloc(&mut (*(*ndev).nfc_dev).dev, core::mem::size_of::<nci_conn_info>(), GFP_KERNEL);
        if conn_info.is_null() { status = NCI_STATUS_REJECTED; } else {
            (*conn_info).dest_params = devm_kzalloc(&mut (*(*ndev).nfc_dev).dev, core::mem::size_of::<dest_spec_params>(), GFP_KERNEL);
            if (*conn_info).dest_params.is_null() { status = NCI_STATUS_REJECTED; } else {
                (*conn_info).dest_type = (*ndev).cur_dest_type;
                (*(*conn_info).dest_params).id = (*ndev).cur_params.id;
                (*(*conn_info).dest_params).protocol = (*ndev).cur_params.protocol;
                (*conn_info).conn_id = (*rsp).conn_id;
                INIT_LIST_HEAD(&mut (*conn_info).list);
                list_add(&mut (*conn_info).list, &mut (*ndev).conn_info_list);
                if (*ndev).cur_params.id == (*(*ndev).hci_dev).nfcee_id { (*(*ndev).hci_dev).conn_info = conn_info; }
                (*conn_info).conn_id = (*rsp).conn_id;
                (*conn_info).max_pkt_payload_len = (*rsp).max_ctrl_pkt_payload_len;
                atomic_set(&mut (*conn_info).credits_cnt, (*rsp).credits_cnt);
            }
        }
    }
    if status == NCI_STATUS_REJECTED { devm_kfree(&mut (*(*ndev).nfc_dev).dev, conn_info); }
    nci_req_complete(ndev, status);
}

unsafe fn nci_core_conn_close_rsp_packet(ndev: *mut nci_dev, skb: *const sk_buff) {
    let status = (*skb).data[0];
    if status == NCI_STATUS_OK {
        let conn_info = nci_get_conn_info_by_conn_id(ndev, (*ndev).cur_conn_id);
        if !conn_info.is_null() { list_del(&mut (*conn_info).list); if conn_info == (*ndev).rf_conn_info { (*ndev).rf_conn_info = core::ptr::null_mut(); } devm_kfree(&mut (*(*ndev).nfc_dev).dev, (*conn_info).dest_params); devm_kfree(&mut (*(*ndev).nfc_dev).dev, conn_info); }
    }
    nci_req_complete(ndev, status);
}

pub unsafe fn nci_rsp_packet(ndev: *mut nci_dev, skb: *mut sk_buff) {
    let rsp_opcode = nci_opcode((*skb).data);
    timer_delete(&mut (*ndev).cmd_timer);
    skb_pull(skb, NCI_CTRL_HDR_SIZE);
    if nci_opcode_gid(rsp_opcode) == NCI_GID_PROPRIETARY { if nci_prop_rsp_packet(ndev, rsp_opcode, skb) == -ENOTSUPP { pr_err!("unsupported rsp opcode 0x{:x}\n", rsp_opcode); } } else {
        match rsp_opcode {
            NCI_OP_CORE_RESET_RSP => nci_core_reset_rsp_packet(ndev, skb),
            NCI_OP_CORE_INIT_RSP => nci_core_init_rsp_packet(ndev, skb),
            NCI_OP_CORE_SET_CONFIG_RSP => nci_core_set_config_rsp_packet(ndev, skb),
            NCI_OP_CORE_CONN_CREATE_RSP => nci_core_conn_create_rsp_packet(ndev, skb),
            NCI_OP_CORE_CONN_CLOSE_RSP => nci_core_conn_close_rsp_packet(ndev, skb),
            NCI_OP_RF_DISCOVER_MAP_RSP => nci_rf_disc_map_rsp_packet(ndev, skb),
            NCI_OP_RF_DISCOVER_RSP => nci_rf_disc_rsp_packet(ndev, skb),
            NCI_OP_RF_DISCOVER_SELECT_RSP => nci_rf_disc_select_rsp_packet(ndev, skb),
            NCI_OP_RF_DEACTIVATE_RSP => nci_rf_deactivate_rsp_packet(ndev, skb),
            NCI_OP_NFCEE_DISCOVER_RSP => nci_nfcee_discover_rsp_packet(ndev, skb),
            NCI_OP_NFCEE_MODE_SET_RSP => nci_nfcee_mode_set_rsp_packet(ndev, skb),
            _ => pr_err!("unknown rsp opcode 0x{:x}\n", rsp_opcode),
        }
        nci_core_rsp_packet(ndev, rsp_opcode, skb);
    }
    kfree_skb(skb);
    atomic_set(&mut (*ndev).cmd_cnt, 1);
    if !skb_queue_empty(&(*ndev).cmd_q) { queue_work((*ndev).cmd_wq, &mut (*ndev).cmd_work); }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
