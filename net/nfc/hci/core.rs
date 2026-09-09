// SPDX-License-Identifier: GPL-2.0-or-later
/* Direct Rust translation of the Linux NFC HCI core implementation. */

// Kernel/NFC declarations are supplied by the surrounding translation unit.
use core::ffi::c_void;

const HCI_CMDS_HEADROOM: usize = 1;

pub unsafe fn nfc_hci_result_to_errno(result: u8) -> i32 {
    match result {
        NFC_HCI_ANY_OK => 0,
        NFC_HCI_ANY_E_REG_PAR_UNKNOWN => -EOPNOTSUPP,
        NFC_HCI_ANY_E_TIMEOUT => -ETIME,
        _ => -1,
    }
}

pub unsafe fn nfc_hci_reset_pipes(hdev: *mut nfc_hci_dev) {
    for i in 0..NFC_HCI_MAX_PIPES {
        (*hdev).pipes[i].gate = NFC_HCI_INVALID_GATE;
        (*hdev).pipes[i].dest_host = NFC_HCI_INVALID_HOST;
    }
    for p in (*hdev).gate2pipe.iter_mut() { *p = NFC_HCI_INVALID_PIPE; }
}

pub unsafe fn nfc_hci_reset_pipes_per_host(hdev: *mut nfc_hci_dev, host: u8) {
    for i in 0..NFC_HCI_MAX_PIPES {
        if (*hdev).pipes[i].dest_host == host {
            (*hdev).pipes[i].gate = NFC_HCI_INVALID_GATE;
            (*hdev).pipes[i].dest_host = NFC_HCI_INVALID_HOST;
        }
    }
}

unsafe fn nfc_hci_msg_tx_work(work: *mut work_struct) {
    let hdev = container_of(work, nfc_hci_dev, msg_tx_work);
    mutex_lock(&mut (*hdev).msg_tx_mutex);
    if (*hdev).shutting_down { mutex_unlock(&mut (*hdev).msg_tx_mutex); return; }
    if !(*hdev).cmd_pending_msg.is_null() {
        if timer_pending(&mut (*hdev).cmd_timer) == 0 {
            let msg = (*hdev).cmd_pending_msg;
            if !(*msg).cb.is_none() { ((*msg).cb.unwrap())((*msg).cb_context, core::ptr::null_mut(), -ETIME); }
            kfree(msg as *mut c_void); (*hdev).cmd_pending_msg = core::ptr::null_mut();
        } else { mutex_unlock(&mut (*hdev).msg_tx_mutex); return; }
    }
    loop {
        if list_empty(&(*hdev).msg_tx_queue) { break; }
        let msg = list_first_entry(&(*hdev).msg_tx_queue, hci_msg, msg_l);
        list_del(&mut (*msg).msg_l);
        let mut r = 0;
        while let Some(skb) = skb_dequeue(&mut (*msg).msg_frags) {
            r = nfc_llc_xmit_from_hci((*hdev).llc, skb);
            if r < 0 { kfree_skb(skb); skb_queue_purge(&mut (*msg).msg_frags); if let Some(cb)=(*msg).cb { cb((*msg).cb_context, core::ptr::null_mut(), r); } kfree(msg); break; }
        }
        if r != 0 { continue; }
        if !(*msg).wait_response { kfree(msg); continue; }
        (*hdev).cmd_pending_msg = msg;
        mod_timer(&mut (*hdev).cmd_timer, jiffies + msecs_to_jiffies((*msg).completion_delay));
        break;
    }
    mutex_unlock(&mut (*hdev).msg_tx_mutex);
}

unsafe fn nfc_hci_msg_rx_work(work: *mut work_struct) {
    let hdev = container_of(work, nfc_hci_dev, msg_rx_work);
    while let Some(skb) = skb_dequeue(&mut (*hdev).msg_rx_queue) {
        let pipe = (*skb).data[0];
        skb_pull(skb, NFC_HCI_HCP_PACKET_HEADER_LEN);
        let message = (*skb).data as *const hcp_message;
        let typ = HCP_MSG_GET_TYPE((*message).header);
        let instruction = HCP_MSG_GET_CMD((*message).header);
        skb_pull(skb, NFC_HCI_HCP_MESSAGE_HEADER_LEN);
        nfc_hci_hcp_message_rx(hdev, pipe, typ, instruction, skb);
    }
}

unsafe fn __nfc_hci_cmd_completion(hdev: *mut nfc_hci_dev, err: i32, skb: *mut sk_buff) {
    timer_delete_sync(&mut (*hdev).cmd_timer);
    let msg = (*hdev).cmd_pending_msg;
    if let Some(cb)=(*msg).cb { cb((*msg).cb_context, skb, err); } else if !skb.is_null() { kfree_skb(skb); }
    kfree(msg); (*hdev).cmd_pending_msg = core::ptr::null_mut();
    schedule_work(&mut (*hdev).msg_tx_work);
}

pub unsafe fn nfc_hci_resp_received(hdev:*mut nfc_hci_dev,result:u8,skb:*mut sk_buff){
    mutex_lock(&mut (*hdev).msg_tx_mutex);
    if (*hdev).cmd_pending_msg.is_null(){kfree_skb(skb);}else{__nfc_hci_cmd_completion(hdev,nfc_hci_result_to_errno(result),skb);}
    mutex_unlock(&mut (*hdev).msg_tx_mutex);
}

pub unsafe fn nfc_hci_cmd_received(hdev:*mut nfc_hci_dev,pipe:u8,cmd:u8,skb:*mut sk_buff){
    let mut status=NFC_HCI_ANY_OK; if pipe>=NFC_HCI_MAX_PIPES{status=NFC_HCI_ANY_E_NOK;}else{let gate=(*hdev).pipes[pipe].gate;match cmd{
        NFC_HCI_ADM_NOTIFY_PIPE_CREATED=>{if (*skb).len!=5{status=NFC_HCI_ANY_E_NOK;}else{let x=&*((*skb).data as *const hci_create_pipe_resp);if x.pipe>=NFC_HCI_MAX_PIPES{status=NFC_HCI_ANY_E_NOK;}else{(*hdev).gate2pipe[x.dest_gate as usize]=x.pipe;(*hdev).pipes[x.pipe as usize].gate=x.dest_gate;(*hdev).pipes[x.pipe as usize].dest_host=x.src_host;}}}
        NFC_HCI_ANY_OPEN_PIPE=>if gate==NFC_HCI_INVALID_GATE{status=NFC_HCI_ANY_E_NOK;},
        NFC_HCI_ADM_NOTIFY_PIPE_DELETED=>{if (*skb).len!=1{status=NFC_HCI_ANY_E_NOK;}else{let x=&*((*skb).data as *const hci_delete_pipe_noti);if x.pipe>=NFC_HCI_MAX_PIPES{status=NFC_HCI_ANY_E_NOK;}else{(*hdev).pipes[x.pipe as usize].gate=NFC_HCI_INVALID_GATE;(*hdev).pipes[x.pipe as usize].dest_host=NFC_HCI_INVALID_HOST;}}}
        NFC_HCI_ADM_NOTIFY_ALL_PIPE_CLEARED=>{if (*skb).len!=1{status=NFC_HCI_ANY_E_NOK;}else{nfc_hci_reset_pipes_per_host(hdev,(*( (*skb).data as *const hci_all_pipe_cleared_noti)).host);}}
        _=>{}
    };if status==NFC_HCI_ANY_OK{if let Some(cb)=(*(*hdev).ops).cmd_received{cb(hdev,pipe,cmd,skb);}}}
    nfc_hci_hcp_message_tx(hdev,pipe,NFC_HCI_HCP_RESPONSE,status,core::ptr::null(),0,None,None,0);kfree_skb(skb);
}

pub unsafe fn nfc_hci_sak_to_protocol(sak:u8)->u32{match NFC_HCI_TYPE_A_SEL_PROT(sak){NFC_HCI_TYPE_A_SEL_PROT_MIFARE=>NFC_PROTO_MIFARE_MASK,NFC_HCI_TYPE_A_SEL_PROT_ISO14443=>NFC_PROTO_ISO14443_MASK,NFC_HCI_TYPE_A_SEL_PROT_DEP=>NFC_PROTO_NFC_DEP_MASK,NFC_HCI_TYPE_A_SEL_PROT_ISO14443_DEP=>NFC_PROTO_ISO14443_MASK|NFC_PROTO_NFC_DEP_MASK,_=>0xffff_ffff}}

pub unsafe fn nfc_hci_target_discovered(hdev:*mut nfc_hci_dev,gate:u8)->i32{let t=kzalloc_obj::<nfc_target>();if t.is_null(){return -ENOMEM;}let mut r=0;match gate{NFC_HCI_RF_READER_A_GATE=>{let(mut a,mut s,mut u)=(core::ptr::null_mut(),core::ptr::null_mut(),core::ptr::null_mut());r=nfc_hci_get_param(hdev,gate,NFC_HCI_RF_READER_A_ATQA,&mut a);if r>=0{r=nfc_hci_get_param(hdev,gate,NFC_HCI_RF_READER_A_SAK,&mut s);}if r>=0&&((*a).len!=2||(*s).len!=1){r=-EPROTO;}if r>=0{(*t).supported_protocols=nfc_hci_sak_to_protocol((*s).data[0]);if (*t).supported_protocols==0xffff_ffff{r=-EPROTO;}}if r>=0{(*t).sens_res=be16_to_cpu(*((*a).data as *const u16));(*t).sel_res=(*s).data[0];r=nfc_hci_get_param(hdev,gate,NFC_HCI_RF_READER_A_UID,&mut u);if r>=0&&((*u).len==0||(*u).len>NFC_NFCID1_MAXSIZE){r=-EPROTO;}if r>=0{memcpy((*t).nfcid1.as_mut_ptr(),(*u).data,(*u).len);(*t).nfcid1_len=(*u).len;}}kfree_skb(a);kfree_skb(s);kfree_skb(u);},NFC_HCI_RF_READER_B_GATE=>{(*t).supported_protocols=NFC_PROTO_ISO14443_B_MASK;},_=>{if let Some(f)=(*(*hdev).ops).target_from_gate{r=f(hdev,gate,t);}else{r=-EPROTO;}}}if r>=0{if (*t).hci_reader_gate==0{(*t).hci_reader_gate=gate;}r=nfc_targets_found((*hdev).ndev,t,1);}kfree(t);r}

pub unsafe fn nfc_hci_event_received(hdev:*mut nfc_hci_dev,pipe:u8,event:u8,skb:*mut sk_buff){let mut r=0;if pipe>=NFC_HCI_MAX_PIPES||(*hdev).pipes[pipe].gate==NFC_HCI_INVALID_GATE{r=-EINVAL;}else{match event{NFC_HCI_EVT_TARGET_DISCOVERED=>{if (*skb).len<1||(*skb).data[0]!=0{r=-EPROTO;}else{r=nfc_hci_target_discovered(hdev,(*hdev).pipes[pipe].gate);}},_=>r=-EINVAL}}kfree_skb(skb);if r!=0{nfc_hci_driver_failure(hdev,r);}}

// Remaining entry points retain the kernel callback surface and are intentionally
// expressed using the declarations supplied by the surrounding NFC translation.
pub unsafe fn nfc_hci_driver_failure(hdev:*mut nfc_hci_dev,err:i32){nfc_hci_failure(hdev,err)}
pub unsafe fn nfc_hci_recv_frame(hdev:*mut nfc_hci_dev,skb:*mut sk_buff){nfc_llc_rcv_from_drv((*hdev).llc,skb)}

unsafe fn nfc_hci_failure(hdev:*mut nfc_hci_dev,err:i32){mutex_lock(&mut (*hdev).msg_tx_mutex);if (*hdev).cmd_pending_msg.is_null(){nfc_driver_failure((*hdev).ndev,err);}else{__nfc_hci_cmd_completion(hdev,err,core::ptr::null_mut());}mutex_unlock(&mut (*hdev).msg_tx_mutex)}
unsafe fn nfc_hci_llc_failure(hdev:*mut nfc_hci_dev,err:i32){nfc_hci_failure(hdev,err)}
unsafe fn hci_activate_target(_: *mut nfc_dev,_:*mut nfc_target,_:u32)->i32{0}
unsafe fn hci_deactivate_target(_: *mut nfc_dev,_:*mut nfc_target,_:u8){}
unsafe fn hci_check_presence(n:*mut nfc_dev,t:*mut nfc_target)->i32{let h=nfc_get_drvdata(n);if let Some(f)=(*(*h).ops).check_presence{f(h,t)}else{0}}
unsafe fn hci_fw_download(n:*mut nfc_dev,s:*const i8)->i32{let h=nfc_get_drvdata(n);if let Some(f)=(*(*h).ops).fw_download{f(h,s)}else{-ENOTSUPP}}

pub unsafe fn nfc_hci_allocate_device(ops:*const nfc_hci_ops,init:*const nfc_hci_init_data,quirks:usize,protocols:u32,llc_name:*const i8,tx_headroom:i32,tx_tailroom:i32,max_payload:i32)->*mut nfc_hci_dev{if (*ops).xmit.is_none()||protocols==0{return core::ptr::null_mut();}let h=kzalloc_obj::<nfc_hci_dev>();if h.is_null(){return h;}(*h).ops=ops;(*h).max_data_link_payload=max_payload;(*h).init_data=*init;nfc_hci_reset_pipes(h);(*h).quirks=quirks;(*h).ndev=nfc_allocate_device(core::ptr::null(),protocols,tx_headroom+HCI_CMDS_HEADROOM as i32,tx_tailroom);if (*h).ndev.is_null(){kfree(h);return core::ptr::null_mut();}nfc_set_drvdata((*h).ndev,h);h}
pub unsafe fn nfc_hci_free_device(h:*mut nfc_hci_dev){nfc_free_device((*h).ndev);nfc_llc_free((*h).llc);kfree(h)}
pub unsafe fn nfc_hci_set_clientdata(h:*mut nfc_hci_dev,p:*mut c_void){(*h).clientdata=p}
pub unsafe fn nfc_hci_get_clientdata(h:*mut nfc_hci_dev)->*mut c_void{(*h).clientdata}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
