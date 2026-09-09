// SPDX-License-Identifier: GPL-2.0-only
/* Direct Rust translation of the NCI core implementation. */

#[repr(C)]
struct CoreConnCreateData { length: i32, cmd: *mut nci_core_conn_create_cmd }
#[repr(C)] struct NciSetConfigParam { id: u8, len: usize, val: *const u8 }
#[repr(C)] struct NciRfDiscoverParam { im_protocols: u32, tm_protocols: u32 }
#[repr(C)] struct NciRfDiscoverSelectParam { rf_discovery_id: u8, rf_protocol: u8 }
#[repr(C)] struct NciCmdParam { opcode: u16, len: usize, payload: *const u8 }
#[repr(C)] struct NciLoopbackData { conn_id: u8, data: *mut sk_buff }

unsafe fn nci_get_conn_info_by_conn_id(ndev: *mut nci_dev, conn_id: i32) -> *mut nci_conn_info {
    let mut p = (*ndev).conn_info_list.next;
    while p != &mut (*ndev).conn_info_list as *mut _ {
        let c = container_of!(p, nci_conn_info, list);
        if (*c).conn_id == conn_id { return c; }
        p = (*p).next;
    }
    core::ptr::null_mut()
}

#[no_mangle] pub unsafe extern "C" fn nci_get_conn_info_by_dest_type_params(ndev: *mut nci_dev, dest_type: u8, params: *const dest_spec_params) -> i32 {
    let mut p = (*ndev).conn_info_list.next;
    while p != &mut (*ndev).conn_info_list as *mut _ {
        let c = container_of!(p, nci_conn_info, list);
        if (*c).dest_type == dest_type && (params.is_null() || ((*params).id == (*(*c).dest_params).id && (*params).protocol == (*(*c).dest_params).protocol)) { return (*c).conn_id; }
        p = (*p).next;
    }
    -EINVAL
}

#[no_mangle] pub unsafe extern "C" fn nci_req_complete(ndev: *mut nci_dev, result: i32) { if (*ndev).req_status == NCI_REQ_PEND { (*ndev).req_result=result; (*ndev).req_status=NCI_REQ_DONE; complete(&mut (*ndev).req_completion); } }
unsafe fn nci_req_cancel(ndev: *mut nci_dev, err: i32) { if (*ndev).req_status == NCI_REQ_PEND { (*ndev).req_result=err; (*ndev).req_status=NCI_REQ_CANCELED; complete(&mut (*ndev).req_completion); } }

unsafe fn __nci_request(ndev: *mut nci_dev, req: unsafe fn(*mut nci_dev,*const core::ffi::c_void), opt: *const core::ffi::c_void, timeout: u32) -> i32 {
    (*ndev).req_status=NCI_REQ_PEND; reinit_completion(&mut (*ndev).req_completion); req(ndev,opt);
    let cr=wait_for_completion_interruptible_timeout(&mut (*ndev).req_completion,timeout); let mut rc=0;
    if cr>0 { rc=match (*ndev).req_status { NCI_REQ_DONE=>nci_to_errno((*ndev).req_result), NCI_REQ_CANCELED=>-(*ndev).req_result, _=>-ETIMEDOUT }; } else { rc=if cr==0 {-ETIMEDOUT} else {cr as i32}; }
    (*ndev).req_status=0; (*ndev).req_result=0; rc
}
#[no_mangle] pub unsafe extern "C" fn nci_request(ndev:*mut nci_dev, req:unsafe fn(*mut nci_dev,*const core::ffi::c_void), opt:*const core::ffi::c_void, timeout:u32)->i32 { mutex_lock(&mut (*ndev).req_lock); let r=if test_bit(NCI_UP,&(*ndev).flags) {__nci_request(ndev,req,opt,timeout)} else {-ENETDOWN}; mutex_unlock(&mut (*ndev).req_lock); r }

unsafe fn nci_reset_req(n:*mut nci_dev,_:*const core::ffi::c_void){let mut c=nci_core_reset_cmd{reset_type:NCI_RESET_TYPE_RESET_CONFIG};nci_send_cmd(n,NCI_OP_CORE_RESET_CMD,1,&mut c as *mut _ as *const _);}
unsafe fn nci_init_req(n:*mut nci_dev,opt:*const core::ffi::c_void){let l=if opt.is_null(){0}else{core::mem::size_of::<nci_core_init_v2_cmd>() as u8};nci_send_cmd(n,NCI_OP_CORE_INIT_CMD,l,opt);}
unsafe fn nci_generic_req(n:*mut nci_dev,opt:*const core::ffi::c_void){let p=&*(opt as *const NciCmdParam);nci_send_cmd(n,p.opcode,p.len as u8,p.payload);}

#[no_mangle] pub unsafe extern "C" fn nci_prop_cmd(n:*mut nci_dev,oid:u8,len:usize,p:*const u8)->i32{let x=NciCmdParam{opcode:nci_opcode_pack(NCI_GID_PROPRIETARY,oid),len,payload:p};__nci_request(n,nci_generic_req,&x as *const _ as *const _,msecs_to_jiffies(NCI_CMD_TIMEOUT))}
#[no_mangle] pub unsafe extern "C" fn nci_core_cmd(n:*mut nci_dev,opcode:u16,len:usize,p:*const u8)->i32{let x=NciCmdParam{opcode,len,payload:p};__nci_request(n,nci_generic_req,&x as *const _ as *const _,msecs_to_jiffies(NCI_CMD_TIMEOUT))}
#[no_mangle] pub unsafe extern "C" fn nci_core_reset(n:*mut nci_dev)->i32{__nci_request(n,nci_reset_req,core::ptr::null(),msecs_to_jiffies(NCI_RESET_TIMEOUT))}
#[no_mangle] pub unsafe extern "C" fn nci_core_init(n:*mut nci_dev)->i32{__nci_request(n,nci_init_req,core::ptr::null(),msecs_to_jiffies(NCI_INIT_TIMEOUT))}

// The remaining implementation preserves the C control flow and delegates all kernel/NCI
// types and helpers to their corresponding external Rust declarations.
#[no_mangle] pub unsafe extern "C" fn nci_send_frame(n:*mut nci_dev,skb:*mut sk_buff)->i32{if n.is_null(){kfree_skb(skb);return -ENODEV;}skb_orphan(skb);nfc_send_to_raw_sock((*n).nfc_dev,skb,RAW_PAYLOAD_NCI,NFC_DIRECTION_TX);((*n).ops).send(n,skb)}
#[no_mangle] pub unsafe extern "C" fn nci_send_cmd(n:*mut nci_dev,opcode:u16,plen:u8,payload:*const core::ffi::c_void)->i32{let skb=nci_skb_alloc(n,NCI_CTRL_HDR_SIZE as usize+plen as usize,GFP_KERNEL);if skb.is_null(){return -ENOMEM;}let h=skb_put(skb,NCI_CTRL_HDR_SIZE) as *mut nci_ctrl_hdr;(*h).gid=nci_opcode_gid(opcode);(*h).oid=nci_opcode_oid(opcode);(*h).plen=plen;nci_mt_set(h as *mut u8,NCI_MT_CMD_PKT);nci_pbf_set(h as *mut u8,NCI_PBF_LAST);if plen!=0{skb_put_data(skb,payload,plen as usize);}skb_queue_tail(&mut (*n).cmd_q,skb);queue_work((*n).cmd_wq,&mut (*n).cmd_work);0}

#[no_mangle] pub unsafe extern "C" fn nci_free_device(n:*mut nci_dev){nfc_free_device((*n).nfc_dev);nci_hci_deallocate(n);if !(*n).rx_data_reassembly.is_null(){kfree_skb((*n).rx_data_reassembly);}kfree(n as *mut _);}
#[no_mangle] pub unsafe extern "C" fn nci_recv_frame(n:*mut nci_dev,skb:*mut sk_buff)->i32{if n.is_null()||(!test_bit(NCI_UP,&(*n).flags)&&!test_bit(NCI_INIT,&(*n).flags)){kfree_skb(skb);return -ENXIO;}skb_queue_tail(&mut (*n).rx_q,skb);queue_work((*n).rx_wq,&mut (*n).rx_work);0}
unsafe fn nci_set_config_req(n:*mut nci_dev,o:*const core::ffi::c_void){let p=&*(o as *const NciSetConfigParam);let mut c=nci_core_set_config_cmd::default();BUG_ON(p.len>NCI_MAX_PARAM_LEN);c.num_params=1;c.param.id=p.id;c.param.len=p.len;memcpy(c.param.val.as_mut_ptr(),p.val,p.len);nci_send_cmd(n,NCI_OP_CORE_SET_CONFIG_CMD,(3+p.len) as u8,&c as *const _ as *const _);}
#[no_mangle] pub unsafe extern "C" fn nci_set_config(n:*mut nci_dev,id:u8,len:usize,val:*const u8)->i32{if val.is_null()||len==0{return 0;}let p=NciSetConfigParam{id,len,val};__nci_request(n,nci_set_config_req,&p as *const _ as *const _,msecs_to_jiffies(NCI_SET_CONFIG_TIMEOUT))}
unsafe fn nci_rf_deactivate_req(n:*mut nci_dev,o:*const core::ffi::c_void){let mut c=nci_rf_deactivate_cmd{type_:o as usize as u64};nci_send_cmd(n,NCI_OP_RF_DEACTIVATE_CMD,core::mem::size_of::<nci_rf_deactivate_cmd>() as u8,&mut c as *mut _ as *const _);}
unsafe fn nci_nfcee_discover_req(n:*mut nci_dev,o:*const core::ffi::c_void){let mut c=nci_nfcee_discover_cmd{discovery_action:o as usize as u8};nci_send_cmd(n,NCI_OP_NFCEE_DISCOVER_CMD,1,&mut c as *mut _ as *const _);}
#[no_mangle] pub unsafe extern "C" fn nci_nfcee_discover(n:*mut nci_dev,a:u8)->i32{__nci_request(n,nci_nfcee_discover_req,a as usize as *const _,msecs_to_jiffies(NCI_CMD_TIMEOUT))}
unsafe fn nci_core_conn_close_req(n:*mut nci_dev,o:*const core::ffi::c_void){let c=o as usize as u8;nci_send_cmd(n,NCI_OP_CORE_CONN_CLOSE_CMD,1,&c as *const _ as *const _);}
#[no_mangle] pub unsafe extern "C" fn nci_core_conn_close(n:*mut nci_dev,id:u8)->i32{(*n).cur_conn_id=id;__nci_request(n,nci_core_conn_close_req,id as usize as *const _,msecs_to_jiffies(NCI_CMD_TIMEOUT))}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
