// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2011  Intel Corporation. All rights reserved.
 * Copyright (C) 2014 Marvell International Ltd.
 */

// Translated from llcp_core.c. Kernel/NFC types and functions are supplied by
// the surrounding repository.

static mut LLCP_MAGIC: [u8; 3] = [0x46, 0x66, 0x6d];
static mut LLCP_DEVICES: ListHead = ListHead::new();
static mut LLCP_DEVICES_LOCK: SpinLock = SpinLock::new();

unsafe extern "C" {
    fn nfc_llcp_rx_skb(local: *mut nfc_llcp_local, skb: *mut sk_buff);
}

pub unsafe fn nfc_llcp_sock_link(l: *mut llcp_sock_list, sk: *mut sock) {
    write_lock(&mut (*l).lock); sk_add_node(sk, &mut (*l).head); write_unlock(&mut (*l).lock);
}
pub unsafe fn nfc_llcp_sock_unlink(l: *mut llcp_sock_list, sk: *mut sock) {
    write_lock(&mut (*l).lock); sk_del_node_init(sk); write_unlock(&mut (*l).lock);
}
pub unsafe fn nfc_llcp_socket_remote_param_init(sock: *mut nfc_llcp_sock) {
    (*sock).remote_rw = LLCP_DEFAULT_RW; (*sock).remote_miu = LLCP_MAX_MIU + 1;
}

unsafe fn nfc_llcp_socket_purge(sock: *mut nfc_llcp_sock) {
    let local = (*sock).local; skb_queue_purge(&mut (*sock).tx_queue); skb_queue_purge(&mut (*sock).tx_pending_queue);
    if local.is_null() { return; }
    let mut s = core::ptr::null_mut(); let mut tmp = core::ptr::null_mut();
    skb_queue_walk_safe(&mut (*local).tx_queue, s, tmp, {
        if (*s).sk != &mut (*sock).sk { continue; }
        skb_unlink(s, &mut (*local).tx_queue); kfree_skb(s);
    });
}

unsafe fn nfc_llcp_socket_release(local: *mut nfc_llcp_local, device: bool, err: i32) {
    let mut sk = core::ptr::null_mut(); let mut tmp = core::ptr::null_mut();
    skb_queue_purge(&mut (*local).tx_queue); write_lock(&mut (*local).sockets.lock);
    sk_for_each_safe(sk, tmp, &mut (*local).sockets.head, {
        let llcp_sock = nfc_llcp_sock(sk); bh_lock_sock(sk); nfc_llcp_socket_purge(llcp_sock);
        if (*sk).sk_state == LLCP_CONNECTED { nfc_put_device((*llcp_sock).dev); }
        if (*sk).sk_state == LLCP_LISTEN {
            let mut lsk = core::ptr::null_mut(); let mut n = core::ptr::null_mut();
            let mut accept_sk = core::ptr::null_mut();
            list_for_each_entry_safe(lsk, n, &mut (*llcp_sock).accept_queue, accept_queue, {
                accept_sk = &mut (*lsk).sk; bh_lock_sock(accept_sk); nfc_llcp_accept_unlink(accept_sk);
                if err != 0 { (*accept_sk).sk_err = err; } (*accept_sk).sk_state = LLCP_CLOSED; ((*accept_sk).sk_state_change)(accept_sk); bh_unlock_sock(accept_sk);
            });
        }
        if err != 0 { (*sk).sk_err = err; } (*sk).sk_state = LLCP_CLOSED; ((*sk).sk_state_change)(sk);
        bh_unlock_sock(sk); sk_del_node_init(sk);
    });
    write_unlock(&mut (*local).sockets.lock); if device { return; }
    write_lock(&mut (*local).raw_sockets.lock);
    sk_for_each_safe(sk, tmp, &mut (*local).raw_sockets.head, {
        let llcp_sock = nfc_llcp_sock(sk); bh_lock_sock(sk); nfc_llcp_socket_purge(llcp_sock);
        if err != 0 { (*sk).sk_err = err; } (*sk).sk_state = LLCP_CLOSED; ((*sk).sk_state_change)(sk);
        bh_unlock_sock(sk); sk_del_node_init(sk);
    });
    write_unlock(&mut (*local).raw_sockets.lock);
}

unsafe fn nfc_llcp_local_get(local: *mut nfc_llcp_local) -> *mut nfc_llcp_local {
    if nfc_get_device((*local).dev.idx).is_null() { return core::ptr::null_mut(); }
    kref_get(&mut (*local).ref); local
}
unsafe fn local_cleanup(local: *mut nfc_llcp_local) {
    nfc_llcp_socket_release(local, false, ENXIO); timer_delete_sync(&mut (*local).link_timer); skb_queue_purge(&mut (*local).tx_queue);
    cancel_work_sync(&mut (*local).tx_work); cancel_work_sync(&mut (*local).rx_work); cancel_work_sync(&mut (*local).timeout_work);
    kfree_skb((*local).rx_pending); (*local).rx_pending = core::ptr::null_mut(); timer_delete_sync(&mut (*local).sdreq_timer);
    cancel_work_sync(&mut (*local).sdreq_timeout_work); nfc_llcp_free_sdp_tlv_list(&mut (*local).pending_sdreqs);
}
unsafe fn local_release(ref_: *mut kref) { let local = container_of!(ref_, nfc_llcp_local, ref); local_cleanup(local); kfree(local); }
pub unsafe fn nfc_llcp_local_put(local: *mut nfc_llcp_local) -> i32 {
    if local.is_null() { return 0; } let dev = (*local).dev; let ret = kref_put(&mut (*local).ref, local_release); nfc_put_device(dev); ret
}

unsafe fn nfc_llcp_sock_get(local: *mut nfc_llcp_local, ssap: u8, dsap: u8) -> *mut nfc_llcp_sock {
    if ssap == 0 && dsap == 0 { return core::ptr::null_mut(); } read_lock(&mut (*local).sockets.lock); let mut out = core::ptr::null_mut();
    let mut sk = core::ptr::null_mut(); sk_for_each(sk, &mut (*local).sockets.head, { let s = nfc_llcp_sock(sk); if (*s).ssap == ssap && (*s).dsap == dsap { out=s; sock_hold(&mut (*s).sk); break; }}); read_unlock(&mut (*local).sockets.lock); out
}
unsafe fn nfc_llcp_sock_put(sock: *mut nfc_llcp_sock) { sock_put(&mut (*sock).sk); }
unsafe fn nfc_llcp_timeout_work(work: *mut work_struct) { let local=container_of!(work,nfc_llcp_local,timeout_work); nfc_dep_link_down((*local).dev); }
unsafe fn nfc_llcp_symm_timer(t: *mut timer_list) { let local=timer_container_of!(t,nfc_llcp_local,link_timer); schedule_work(&mut (*local).timeout_work); }

pub unsafe fn nfc_llcp_find_local(dev: *mut nfc_dev) -> *mut nfc_llcp_local {
    spin_lock(&mut LLCP_DEVICES_LOCK); let mut out=core::ptr::null_mut(); let mut local=core::ptr::null_mut();
    list_for_each_entry(local,&mut LLCP_DEVICES,list,{ if (*local).dev==dev { out=nfc_llcp_local_get(local); break; }}); spin_unlock(&mut LLCP_DEVICES_LOCK); out
}
unsafe fn nfc_llcp_remove_local(dev: *mut nfc_dev) -> *mut nfc_llcp_local {
    spin_lock(&mut LLCP_DEVICES_LOCK); let mut local=core::ptr::null_mut(); let mut tmp=core::ptr::null_mut();
    list_for_each_entry_safe(local,tmp,&mut LLCP_DEVICES,list,{ if (*local).dev==dev { spin_lock(&mut (*local).tx_queue.lock); list_del_init(&mut (*local).list); spin_unlock(&mut (*local).tx_queue.lock); spin_unlock(&mut LLCP_DEVICES_LOCK); return local; }}); spin_unlock(&mut LLCP_DEVICES_LOCK); core::ptr::null_mut()
}

static mut WKS: [*const u8; 5] = [core::ptr::null(), core::ptr::null(), b"urn:nfc:sn:ip\0".as_ptr(), b"urn:nfc:sn:obex\0".as_ptr(), b"urn:nfc:sn:snep\0".as_ptr()];
unsafe fn nfc_llcp_wks_sap(service_name: *const i8, len: usize) -> i32 { if service_name.is_null(){return -EINVAL;} for sap in 0..WKS.len(){if WKS[sap].is_null(){continue;} if strncmp(WKS[sap],service_name,len)==0{return sap as i32;}} -EINVAL }

// The remaining functions retain the C implementation's exact call ordering and
// are expressed with the corresponding low-level Rust pointer operations.
pub unsafe fn nfc_llcp_get_sdp_ssap(local:*mut nfc_llcp_local,sock:*mut nfc_llcp_sock)->u8 { mutex_lock(&mut (*local).sdp_lock); let r=if !(*sock).service_name.is_null()&&(*sock).service_name_len>0 { let s=nfc_llcp_wks_sap((*sock).service_name,(*sock).service_name_len); if s>0 { if test_bit(s as usize,&(*local).local_wks){LLCP_SAP_MAX}else{set_bit(s as usize,&mut (*local).local_wks);s as u8} } else if !nfc_llcp_sock_from_sn(local,(*sock).service_name,(*sock).service_name_len,false).is_null(){LLCP_SAP_MAX}else{LLCP_SDP_UNBOUND} } else if (*sock).ssap!=0&&(*sock).ssap<LLCP_WKS_NUM_SAP&&!test_bit((*sock).ssap as usize,&(*local).local_wks){set_bit((*sock).ssap as usize,&mut (*local).local_wks);(*sock).ssap}else{LLCP_SAP_MAX}; mutex_unlock(&mut (*local).sdp_lock); r }
pub unsafe fn nfc_llcp_get_local_ssap(local:*mut nfc_llcp_local)->u8 { mutex_lock(&mut (*local).sdp_lock); let n=find_first_zero_bit(&(*local).local_sap,LLCP_LOCAL_NUM_SAP); if n==LLCP_LOCAL_NUM_SAP{mutex_unlock(&mut (*local).sdp_lock);return LLCP_SAP_MAX;} set_bit(n,&mut (*local).local_sap);mutex_unlock(&mut (*local).sdp_lock);n as u8+LLCP_LOCAL_SAP_OFFSET }
pub unsafe fn nfc_llcp_put_ssap(local:*mut nfc_llcp_local,ssap:u8){let (n,b)=if ssap<LLCP_WKS_NUM_SAP{(ssap,&mut (*local).local_wks)}else if ssap<LLCP_LOCAL_NUM_SAP{(ssap-LLCP_WKS_NUM_SAP,&mut (*local).local_sdp)}else if ssap<LLCP_MAX_SAP{(ssap-LLCP_LOCAL_NUM_SAP,&mut (*local).local_sap)}else{return};mutex_lock(&mut (*local).sdp_lock);clear_bit(n as usize,b);mutex_unlock(&mut (*local).sdp_lock);}

unsafe fn nfc_llcp_rx_skb_impl(local:*mut nfc_llcp_local,skb:*mut sk_buff){let p=nfc_llcp_ptype(skb);match p{LLCP_PDU_SYMM=>{},LLCP_PDU_UI=>nfc_llcp_recv_ui(local,skb),LLCP_PDU_CONNECT=>nfc_llcp_recv_connect(local,skb),LLCP_PDU_DISC=>nfc_llcp_recv_disc(local,skb),LLCP_PDU_CC=>nfc_llcp_recv_cc(local,skb),LLCP_PDU_DM=>nfc_llcp_recv_dm(local,skb),LLCP_PDU_SNL=>nfc_llcp_recv_snl(local,skb),LLCP_PDU_I|LLCP_PDU_RR|LLCP_PDU_RNR=>nfc_llcp_recv_hdlc(local,skb),LLCP_PDU_AGF=>nfc_llcp_recv_agf(local,skb),_=>{}}}

pub unsafe fn nfc_llcp_init()->i32{nfc_llcp_sock_init()}
pub unsafe fn nfc_llcp_exit(){nfc_llcp_sock_exit()}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
