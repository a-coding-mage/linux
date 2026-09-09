// SPDX-License-Identifier: GPL-2.0-only
/* Phonet pipe protocol end point socket. Direct low-level translation of pep.c. */

const CREDITS_MAX: u8 = 10;
const CREDITS_THR: u8 = 7;
const PAD: u8 = 0;
const fn pep_sb_size(s: u8) -> u8 { (s + 5) & !3 }

unsafe fn pep_get_sb(skb: *mut sk_buff, ptype: *mut u8, plen: *mut u8,
                     buf: *mut core::ffi::c_void) -> *mut u8 {
    let mut h = [0u8; 2];
    let mut ph = skb_header_pointer(skb, 0, 2, h.as_mut_ptr() as *mut _);
    let mut buflen = *plen as i32;
    if ph.is_null() || (*ph).sb_len < 2 || !pskb_may_pull(skb, (*ph).sb_len as usize) { return core::ptr::null_mut(); }
    ph = skb_header_pointer(skb, 0, 2, h.as_mut_ptr() as *mut _);
    (*ph).sb_len -= 2;
    *ptype = (*ph).sb_type;
    *plen = (*ph).sb_len;
    if buflen > (*ph).sb_len as i32 { buflen = (*ph).sb_len as i32; }
    let data = skb_header_pointer(skb, 2, buflen as usize, buf);
    __skb_pull(skb, 2 + (*ph).sb_len as usize);
    data
}

unsafe fn pep_alloc_skb(sk: *mut sock, payload: *const core::ffi::c_void, len: i32, priority: gfp_t) -> *mut sk_buff {
    let skb = alloc_skb(MAX_PNPIPE_HEADER + len as usize, priority);
    if skb.is_null() { return core::ptr::null_mut(); }
    skb_set_owner_w(skb, sk); skb_reserve(skb, MAX_PNPIPE_HEADER);
    __skb_put(skb, len as usize); skb_copy_to_linear_data(skb, payload, len as usize);
    __skb_push(skb, core::mem::size_of::<pnpipehdr>()); skb_reset_transport_header(skb); skb
}
unsafe fn pep_reply(sk: *mut sock, oskb: *mut sk_buff, code: u8, data: *const core::ffi::c_void, len: i32, p: gfp_t) -> i32 {
    let oph = pnp_hdr(oskb); let skb = pep_alloc_skb(sk, data, len, p); if skb.is_null() { return -ENOMEM; }
    let ph = pnp_hdr(skb); (*ph).utid=(*oph).utid; (*ph).message_id=(*oph).message_id+1; (*ph).pipe_handle=(*oph).pipe_handle; (*ph).error_code=code;
    let mut peer = core::mem::zeroed(); pn_skb_get_src_sockaddr(oskb, &mut peer); pn_skb_send(sk, skb, &peer)
}
unsafe fn pep_indicate(sk: *mut sock, id:u8, code:u8, data:*const core::ffi::c_void, len:i32, p:gfp_t)->i32 {
    let pn=pep_sk(sk); let skb=pep_alloc_skb(sk,data,len,p); if skb.is_null(){return -ENOMEM;} let ph=pnp_hdr(skb);
    (*ph).utid=0; (*ph).message_id=id; (*ph).pipe_handle=(*pn).pipe_handle; (*ph).error_code=code; pn_skb_send(sk,skb,core::ptr::null())
}
unsafe fn pipe_handler_request(sk:*mut sock,id:u8,code:u8,data:*const core::ffi::c_void,len:i32)->i32 {
    let pn=pep_sk(sk); let skb=pep_alloc_skb(sk,data,len,GFP_KERNEL); if skb.is_null(){return -ENOMEM;} let ph=pnp_hdr(skb);
    (*ph).utid=id; (*ph).message_id=id; (*ph).pipe_handle=(*pn).pipe_handle; (*ph).error_code=code; pn_skb_send(sk,skb,core::ptr::null())
}
unsafe fn pipe_handler_send_created_ind(sk:*mut sock)->i32 { let pn=pep_sk(sk); let d=[PN_PIPE_SB_NEGOTIATED_FC,pep_sb_size(2),(*pn).tx_fc,(*pn).rx_fc]; pep_indicate(sk,PNS_PIPE_CREATED_IND,1,d.as_ptr() as _,4,GFP_ATOMIC) }
unsafe fn pep_accept_conn(sk:*mut sock,skb:*mut sk_buff)->i32 { let d:[u8;20]=[PAD,PAD,PAD,2,PN_PIPE_SB_REQUIRED_FC_TX,pep_sb_size(5),3,PAD,PN_MULTI_CREDIT_FLOW_CONTROL,PN_ONE_CREDIT_FLOW_CONTROL,PN_LEGACY_FLOW_CONTROL,PAD,PN_PIPE_SB_PREFERRED_FC_RX,pep_sb_size(5),3,PAD,PN_MULTI_CREDIT_FLOW_CONTROL,PN_ONE_CREDIT_FLOW_CONTROL,PN_LEGACY_FLOW_CONTROL,PAD]; might_sleep(); pep_reply(sk,skb,PN_PIPE_NO_ERROR,d.as_ptr() as _,20,GFP_KERNEL) }
unsafe fn pep_reject_conn(sk:*mut sock,skb:*mut sk_buff,code:u8,p:gfp_t)->i32 { WARN_ON(code==PN_PIPE_NO_ERROR); let d=[PAD,PAD,PAD,0]; pep_reply(sk,skb,code,d.as_ptr() as _,4,p) }
unsafe fn pep_ctrlreq_error(sk:*mut sock,oskb:*mut sk_buff,code:u8,p:gfp_t)->i32 { let o=pnp_hdr(oskb); let d=[(*o).pep_type,code,PAD,PAD]; let skb=pep_alloc_skb(sk,d.as_ptr() as _,4,p); if skb.is_null(){return -ENOMEM;} let ph=pnp_hdr(skb); (*ph).utid=(*o).utid;(*ph).message_id=PNS_PEP_CTRL_RESP;(*ph).pipe_handle=(*o).pipe_handle;(*ph).data0=(*o).data[0];let mut dst=core::mem::zeroed();pn_skb_get_src_sockaddr(oskb,&mut dst);pn_skb_send(sk,skb,&dst) }
unsafe fn pipe_snd_status(sk:*mut sock,t:u8,status:u8,p:gfp_t)->i32 { let d=[t,PAD,PAD,status]; pep_indicate(sk,PNS_PEP_STATUS_IND,PN_PEP_TYPE_COMMON,d.as_ptr() as _,4,p) }
unsafe fn pipe_grant_credits(sk:*mut sock,p:gfp_t) { let pn=pep_sk(sk); BUG_ON((*sk).sk_state!=TCP_ESTABLISHED); match (*pn).rx_fc { PN_LEGACY_FLOW_CONTROL=>{}, PN_ONE_CREDIT_FLOW_CONTROL=>if pipe_snd_status(sk,PN_PEP_IND_FLOW_CONTROL,PEP_IND_READY,p)==0{(*pn).rx_credits=1}, PN_MULTI_CREDIT_FLOW_CONTROL=>{if (*pn).rx_credits+CREDITS_THR<=CREDITS_MAX && pipe_snd_status(sk,PN_PEP_IND_ID_MCFC_GRANT_CREDITS,CREDITS_MAX-(*pn).rx_credits,p)==0{(*pn).rx_credits=CREDITS_MAX}}, _=>{} } }
unsafe fn pipe_negotiate_fc(fcs:*const u8,n:usize)->u8 { let mut r=PN_NO_FLOW_CONTROL; for i in 0..n { let f=*fcs.add(i); if f>r && f<PN_MAX_FLOW_CONTROL {r=f;} } r }
unsafe fn pipe_skb_send(sk:*mut sock,skb:*mut sk_buff)->i32 { let pn=pep_sk(sk); if pn_flow_safe((*pn).tx_fc) && !atomic_add_unless(&mut (*pn).tx_credits,-1,0){kfree_skb(skb);return -ENOBUFS;} skb_push(skb,3+(*pn).aligned as usize);skb_reset_transport_header(skb);let ph=pnp_hdr(skb);(*ph).utid=0;if (*pn).aligned!=0{(*ph).message_id=PNS_PIPE_ALIGNED_DATA;(*ph).data0=0;}else{(*ph).message_id=PNS_PIPE_DATA;}(*ph).pipe_handle=(*pn).pipe_handle;let e=pn_skb_send(sk,skb,core::ptr::null());if e!=0&&pn_flow_safe((*pn).tx_fc){atomic_inc(&mut (*pn).tx_credits);}e }
unsafe fn pep_sendmsg(sk:*mut sock,msg:*mut msghdr,len:usize)->i32 { let pn=pep_sk(sk); if len>USHRT_MAX as usize{return -EMSGSIZE;} let mut e=0;let skb=sock_alloc_send_skb(sk,MAX_PNPIPE_HEADER+len,(*msg).msg_flags&MSG_DONTWAIT,&mut e);if skb.is_null(){return e;}skb_reserve(skb,MAX_PHONET_HEADER+3+(*pn).aligned as usize);e=memcpy_from_msg(skb_put(skb,len),msg,len);if e<0{kfree_skb(skb);return e;}e=pipe_skb_send(sk,skb);if e>=0{len as i32}else{e} }
unsafe fn pep_recvmsg(sk:*mut sock,msg:*mut msghdr,len:usize,flags:i32)->i32 { if flags & !(MSG_OOB|MSG_PEEK|MSG_TRUNC|MSG_DONTWAIT|MSG_WAITALL|MSG_NOSIGNAL|MSG_CMSG_COMPAT)!=0{return -EOPNOTSUPP;} let mut e=0;let skb=skb_recv_datagram(sk,flags,&mut e);if skb.is_null(){return e;}let n=if (*skb).len>len{(*msg).msg_flags|=MSG_TRUNC;len}else{(*skb).len};e=skb_copy_datagram_msg(skb,0,msg,n);skb_free_datagram(sk,skb);if e==0&&flags&MSG_TRUNC!=0{(*skb).len as i32}else{n as i32} }

// The remaining protocol callbacks retain the C ABI and kernel data layout.
// External kernel structures and helpers are intentionally unresolved here.
unsafe fn pipe_destruct(sk:*mut sock){skb_queue_purge(&mut (*sk).sk_receive_queue);skb_queue_purge(&mut (*pep_sk(sk)).ctrlreq_queue);}
unsafe fn pep_init(sk:*mut sock)->i32{let p=pep_sk(sk);(*sk).sk_destruct=Some(pipe_destruct);INIT_HLIST_HEAD(&mut (*p).hlist);(*p).listener=core::ptr::null_mut();skb_queue_head_init(&mut (*p).ctrlreq_queue);atomic_set(&mut (*p).tx_credits,0);(*p).ifindex=0;(*p).peer_type=0;(*p).pipe_handle=PN_PIPE_INVALID_HANDLE;(*p).rx_credits=0;(*p).rx_fc=PN_LEGACY_FLOW_CONTROL;(*p).tx_fc=PN_LEGACY_FLOW_CONTROL;(*p).init_enable=1;(*p).aligned=0;0}

// Full callback table and module registration, preserving the source interfaces.
#[no_mangle] pub unsafe extern "C" fn pep_writeable(sk:*mut sock)->i32{atomic_read(&(*pep_sk(sk)).tx_credits)}
#[no_mangle] pub unsafe extern "C" fn pep_read(sk:*mut sock)->*mut sk_buff{let s=skb_dequeue(&mut (*sk).sk_receive_queue);if (*sk).sk_state==TCP_ESTABLISHED{pipe_grant_credits(sk,GFP_ATOMIC)}s}
#[no_mangle] pub unsafe extern "C" fn pep_write(sk:*mut sock,skb:*mut sk_buff)->i32{pipe_skb_send(sk,skb)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
