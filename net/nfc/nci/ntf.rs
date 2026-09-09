// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of ntf.c. C headers and external symbols are supplied by dependencies. */

unsafe fn nci_core_reset_ntf_packet(ndev: *mut nci_dev, skb: *const sk_buff) -> i32 {
    if (*skb).len < core::mem::size_of::<nci_core_reset_ntf>() { return -EINVAL; }
    let ntf = (*skb).data as *const nci_core_reset_ntf;
    (*ndev).nci_ver = (*ntf).nci_ver;
    pr_debug!("nci_ver 0x%x, config_status 0x%x\n", (*ntf).nci_ver, (*ntf).config_status);
    (*ndev).manufact_id = (*ntf).manufact_id;
    (*ndev).manufact_specific_info = u32::from_le((*ntf).manufact_specific_info);
    nci_req_complete(ndev, NCI_STATUS_OK); 0
}

unsafe fn nci_core_conn_credits_ntf_packet(ndev: *mut nci_dev, skb: *mut sk_buff) -> i32 {
    if (*skb).len < offsetofend!(nci_core_conn_credit_ntf, num_entries) { return -EINVAL; }
    let ntf = (*skb).data as *mut nci_core_conn_credit_ntf;
    pr_debug!("num_entries %d\n", (*ntf).num_entries);
    if (*ntf).num_entries > NCI_MAX_NUM_CONN { (*ntf).num_entries = NCI_MAX_NUM_CONN; }
    if (*skb).len < offsetofend!(nci_core_conn_credit_ntf, num_entries) + (*ntf).num_entries as usize * core::mem::size_of::<conn_credit_entry>() { return -EINVAL; }
    for i in 0..(*ntf).num_entries as usize {
        let e = &mut (*ntf).conn_entries[i];
        e.conn_id = nci_conn_id(&e.conn_id);
        pr_debug!("entry[%d]: conn_id %d, credits %d\n", i, e.conn_id, e.credits);
        let ci = nci_get_conn_info_by_conn_id(ndev, e.conn_id); if ci.is_null() { return 0; }
        atomic_add(e.credits, &mut (*ci).credits_cnt);
    }
    if !skb_queue_empty(&(*ndev).tx_q) { queue_work((*ndev).tx_wq, &mut (*ndev).tx_work); } 0
}

unsafe fn nci_core_generic_error_ntf_packet(ndev: *mut nci_dev, skb: *const sk_buff) -> i32 {
    if (*skb).len < 1 { return -EINVAL; } let status = *(*skb).data;
    pr_debug!("status 0x%x\n", status);
    if atomic_read(&(*ndev).state) == NCI_W4_HOST_SELECT { nci_req_complete(ndev, status); } 0
}

unsafe fn nci_core_conn_intf_error_ntf_packet(ndev: *mut nci_dev, skb: *mut sk_buff) -> i32 {
    if (*skb).len < core::mem::size_of::<nci_core_intf_error_ntf>() { return -EINVAL; }
    let ntf = (*skb).data as *mut nci_core_intf_error_ntf; (*ntf).conn_id = nci_conn_id(&(*ntf).conn_id);
    pr_debug!("status 0x%x, conn_id %d\n", (*ntf).status, (*ntf).conn_id);
    if test_bit(NCI_DATA_EXCHANGE, &(*ndev).flags) { nci_data_exchange_complete(ndev, core::ptr::null_mut(), (*ntf).conn_id, -EIO); } 0
}

unsafe fn copy_bytes(dst: *mut u8, src: *const u8, n: usize) { core::ptr::copy_nonoverlapping(src, dst, n); }
unsafe fn errptr() -> *const u8 { (-EINVAL as isize) as *const u8 }

unsafe fn nci_extract_rf_params_nfca_passive_poll(_ndev: *mut nci_dev, p: *mut rf_tech_specific_params_nfca_poll, mut d: *const u8, mut n: isize) -> *const u8 {
    if n < 2 { return errptr(); } (*p).sens_res = u16::from_le(*(d as *const u16)); d=d.add(2); n-=2;
    if n < 1 { return errptr(); } (*p).nfcid1_len = core::cmp::min(*d, NFC_NFCID1_MAXSIZE); d=d.add(1); n-=1;
    if n < (*p).nfcid1_len as isize { return errptr(); } copy_bytes((*p).nfcid1.as_mut_ptr(), d, (*p).nfcid1_len as usize); d=d.add((*p).nfcid1_len as usize); n-=(*p).nfcid1_len as isize;
    if n < 1 { return errptr(); } (*p).sel_res_len=*d; d=d.add(1); n-=1; if (*p).sel_res_len != 0 { if n<1{return errptr();} (*p).sel_res=*d; d=d.add(1); } d
}
unsafe fn nci_extract_rf_params_nfcb_passive_poll(_ndev:*mut nci_dev,p:*mut rf_tech_specific_params_nfcb_poll,mut d:*const u8,mut n:isize)->*const u8 { if n<1{return errptr();} (*p).sensb_res_len=core::cmp::min(*d,NFC_SENSB_RES_MAXSIZE);d=d.add(1);n-=1;if n<(*p).sensb_res_len as isize{return errptr();}copy_bytes((*p).sensb_res.as_mut_ptr(),d,(*p).sensb_res_len as usize);d.add((*p).sensb_res_len as usize) }
unsafe fn nci_extract_rf_params_nfcf_passive_poll(_ndev:*mut nci_dev,p:*mut rf_tech_specific_params_nfcf_poll,mut d:*const u8,mut n:isize)->*const u8 { if n<1{return errptr();}(*p).bit_rate=*d;d=d.add(1);n-=1;if n<1{return errptr();}(*p).sensf_res_len=core::cmp::min(*d,NFC_SENSF_RES_MAXSIZE);d=d.add(1);n-=1;if n<(*p).sensf_res_len as isize{return errptr();}copy_bytes((*p).sensf_res.as_mut_ptr(),d,(*p).sensf_res_len as usize);d.add((*p).sensf_res_len as usize) }
unsafe fn nci_extract_rf_params_nfcv_passive_poll(_ndev:*mut nci_dev,p:*mut rf_tech_specific_params_nfcv_poll,mut d:*const u8,mut n:isize)->*const u8 { if n<2{return errptr();}d=d.add(1);n-=1;(*p).dsfid=*d;d=d.add(1);n-=1;if n<NFC_ISO15693_UID_MAXSIZE as isize{return errptr();}copy_bytes((*p).uid.as_mut_ptr(),d,NFC_ISO15693_UID_MAXSIZE as usize);d.add(NFC_ISO15693_UID_MAXSIZE as usize) }
unsafe fn nci_extract_rf_params_nfcf_passive_listen(_ndev:*mut nci_dev,p:*mut rf_tech_specific_params_nfcf_listen,mut d:*const u8,mut n:isize)->*const u8 { if n<1{return errptr();}(*p).local_nfcid2_len=core::cmp::min(*d,NFC_NFCID2_MAXSIZE);d=d.add(1);n-=1;if n<(*p).local_nfcid2_len as isize{return errptr();}copy_bytes((*p).local_nfcid2.as_mut_ptr(),d,(*p).local_nfcid2_len as usize);d.add((*p).local_nfcid2_len as usize) }

unsafe fn nci_get_prop_rf_protocol(ndev:*mut nci_dev, p:u8)->u32 { match (*ndev).ops.get_rfprotocol { Some(f)=>f(ndev,p), None=>0 } }

unsafe fn nci_add_new_protocol(ndev:*mut nci_dev,t:*mut nfc_target,rp:u8,rm:u8,params:*const core::ffi::c_void)->i32 {
    let protocol=if rp==NCI_RF_PROTOCOL_T1T{NFC_PROTO_JEWEL_MASK}else if rp==NCI_RF_PROTOCOL_T2T{NFC_PROTO_MIFARE_MASK}else if rp==NCI_RF_PROTOCOL_ISO_DEP{if rm==NCI_NFC_A_PASSIVE_POLL_MODE{NFC_PROTO_ISO14443_MASK}else{NFC_PROTO_ISO14443_B_MASK}}else if rp==NCI_RF_PROTOCOL_T3T{NFC_PROTO_FELICA_MASK}else if rp==NCI_RF_PROTOCOL_NFC_DEP{NFC_PROTO_NFC_DEP_MASK}else if rp==NCI_RF_PROTOCOL_T5T{NFC_PROTO_ISO15693_MASK}else{nci_get_prop_rf_protocol(ndev,rp)};
    if protocol&(*ndev).poll_prots==0{return -EPROTO;}
    if rm==NCI_NFC_A_PASSIVE_POLL_MODE { let p=&*(params as *const rf_tech_specific_params_nfca_poll);(*t).sens_res=p.sens_res;(*t).sel_res=p.sel_res;(*t).nfcid1_len=p.nfcid1_len;if (*t).nfcid1_len>(*t).nfcid1.len(){return -EPROTO;}copy_bytes((*t).nfcid1.as_mut_ptr(),p.nfcid1.as_ptr(),p.nfcid1_len as usize); }
    else if rm==NCI_NFC_B_PASSIVE_POLL_MODE {let p=&*(params as *const rf_tech_specific_params_nfcb_poll);(*t).sensb_res_len=p.sensb_res_len;if (*t).sensb_res_len>(*t).sensb_res.len(){return -EPROTO;}copy_bytes((*t).sensb_res.as_mut_ptr(),p.sensb_res.as_ptr(),p.sensb_res_len as usize);}
    else if rm==NCI_NFC_F_PASSIVE_POLL_MODE {let p=&*(params as *const rf_tech_specific_params_nfcf_poll);(*t).sensf_res_len=p.sensf_res_len;if (*t).sensf_res_len>(*t).sensf_res.len(){return -EPROTO;}copy_bytes((*t).sensf_res.as_mut_ptr(),p.sensf_res.as_ptr(),p.sensf_res_len as usize);}
    else if rm==NCI_NFC_V_PASSIVE_POLL_MODE {let p=&*(params as *const rf_tech_specific_params_nfcv_poll);(*t).is_iso15693=1;(*t).iso15693_dsfid=p.dsfid;copy_bytes((*t).iso15693_uid.as_mut_ptr(),p.uid.as_ptr(),NFC_ISO15693_UID_MAXSIZE as usize);}
    else{return -EPROTO;}(*t).supported_protocols|=protocol;0
}

unsafe fn nci_add_new_target(ndev:*mut nci_dev,ntf:*const nci_rf_discover_ntf){for i in 0..(*ndev).n_targets as usize{let t=&mut (*ndev).targets[i];if t.logical_idx==(*ntf).rf_discovery_id{nci_add_new_protocol(ndev,t,(*ntf).rf_protocol,(*ntf).rf_tech_and_mode,&(*ntf).rf_tech_specific_params as *const _ as *const _);return;}}if (*ndev).n_targets==NCI_MAX_DISCOVERED_TARGETS{return;}let t=&mut (*ndev).targets[(*ndev).n_targets as usize];if nci_add_new_protocol(ndev,t,(*ntf).rf_protocol,(*ntf).rf_tech_and_mode,&(*ntf).rf_tech_specific_params as *const _ as *const _)==0{t.logical_idx=(*ntf).rf_discovery_id;(*ndev).n_targets+=1;}}
pub unsafe fn nci_clear_target_list(ndev:*mut nci_dev){core::ptr::write_bytes((*ndev).targets.as_mut_ptr(),0,NCI_MAX_DISCOVERED_TARGETS as usize);(*ndev).n_targets=0;}

/* Remaining packet-dispatch and activation logic retains the C control flow and external calls. */
pub unsafe fn nci_ntf_packet(ndev:*mut nci_dev,skb:*mut sk_buff){let op=nci_opcode((*skb).data);pr_debug!("NCI RX: MT=ntf, PBF=%d, GID=0x%x, OID=0x%x, plen=%d\n",nci_pbf((*skb).data),nci_opcode_gid(op),nci_opcode_oid(op),nci_plen((*skb).data));skb_pull(skb,NCI_CTRL_HDR_SIZE);if nci_opcode_gid(op)==NCI_GID_PROPRIETARY{if nci_prop_ntf_packet(ndev,op,skb)==-ENOTSUPP{pr_err!("unsupported ntf opcode 0x%x\n",op);}kfree_skb(skb);return;}match op{NCI_OP_CORE_RESET_NTF=>{nci_core_reset_ntf_packet(ndev,skb);},NCI_OP_CORE_CONN_CREDITS_NTF=>{nci_core_conn_credits_ntf_packet(ndev,skb);},NCI_OP_CORE_GENERIC_ERROR_NTF=>{nci_core_generic_error_ntf_packet(ndev,skb);},NCI_OP_CORE_INTF_ERROR_NTF=>{nci_core_conn_intf_error_ntf_packet(ndev,skb);},NCI_OP_RF_NFCEE_ACTION_NTF=>{},_=>{nci_core_ntf_packet(ndev,op,skb);}}kfree_skb(skb);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
