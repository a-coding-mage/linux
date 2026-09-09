// SPDX-License-Identifier: GPL-2.0-only
// C kernel dependencies are supplied by the surrounding translation unit.

#[repr(C)]
pub struct esp_skb_cb { pub xfrm: xfrm_skb_cb, pub tmp: *mut core::ffi::c_void }
#[repr(C)]
pub struct esp_output_extra { pub seqhi: __be32, pub esphoff: u32 }

unsafe fn esp_skb_cb(skb: *mut sk_buff) -> *mut esp_skb_cb { (*skb).cb.as_mut_ptr() as *mut esp_skb_cb }

unsafe fn esp_alloc_tmp(aead: *mut crypto_aead, nfrags: i32, extralen: i32) -> *mut core::ffi::c_void {
    let mut len = extralen as usize + crypto_aead_ivsize(aead) as usize;
    if len != 0 { len += crypto_aead_alignmask(aead) as usize & !(crypto_tfm_ctx_alignment() as usize - 1); len = ALIGN(len, crypto_tfm_ctx_alignment() as usize); }
    len += core::mem::size_of::<aead_request>() + crypto_aead_reqsize(aead) as usize;
    len = ALIGN(len, core::mem::align_of::<scatterlist>());
    len += core::mem::size_of::<scatterlist>() * nfrags as usize;
    kmalloc(len, GFP_ATOMIC)
}
unsafe fn esp_tmp_extra(tmp: *mut core::ffi::c_void) -> *mut esp_output_extra { PTR_ALIGN(tmp, core::mem::align_of::<esp_output_extra>()) as *mut esp_output_extra }
unsafe fn esp_tmp_iv(aead: *mut crypto_aead, tmp: *mut core::ffi::c_void, extralen: i32) -> *mut u8 { if crypto_aead_ivsize(aead) != 0 { PTR_ALIGN((tmp as *mut u8).add(extralen as usize) as *mut _, crypto_aead_alignmask(aead) as usize + 1) as *mut u8 } else { (tmp as *mut u8).add(extralen as usize) } }
unsafe fn esp_tmp_req(aead: *mut crypto_aead, iv: *mut u8) -> *mut aead_request { let req = PTR_ALIGN(iv.add(crypto_aead_ivsize(aead) as usize) as *mut _, crypto_tfm_ctx_alignment() as usize) as *mut aead_request; aead_request_set_tfm(req, aead); req }
unsafe fn esp_req_sg(aead: *mut crypto_aead, req: *mut aead_request) -> *mut scatterlist { ALIGN((req.add(1) as usize) + crypto_aead_reqsize(aead) as usize, core::mem::align_of::<scatterlist>()) as *mut scatterlist }

unsafe fn esp_ssg_unref(x: *mut xfrm_state, tmp: *mut core::ffi::c_void, skb: *mut sk_buff, already_unref: bool) {
    let aead = (*x).data as *mut crypto_aead; let mut extralen = 0;
    if (*x).props.flags & XFRM_STATE_ESN != 0 { extralen = core::mem::size_of::<esp_output_extra>() as i32; }
    let req = esp_tmp_req(aead, esp_tmp_iv(aead, tmp, extralen));
    if already_unref || (*req).src != (*req).dst { let mut sg = if already_unref { esp_req_sg(aead, req) } else { (*req).src }; sg = sg_next(sg); while !sg.is_null() { skb_page_unref(page_to_netmem(sg_page(sg)), (*skb).pp_recycle); sg = sg_next(sg); } }
}

#[cfg(CONFIG_INET_ESPINTCP)]
unsafe fn esp_find_tcp_sk(x: *mut xfrm_state) -> *mut sock { let encap=(*x).encap; let net=xs_net(x); let (sport,dport); spin_lock_bh(&mut (*x).lock); sport=(*encap).encap_sport; dport=(*encap).encap_dport; spin_unlock_bh(&mut (*x).lock); let sk=inet_lookup_established(net,(*x).id.daddr.a4,dport,(*x).props.saddr.a4,sport,0); if sk.is_null(){return ERR_PTR(-ENOENT)} if !tcp_is_ulp_esp(sk){sock_put(sk);return ERR_PTR(-EINVAL)} sk }
#[cfg(not(CONFIG_INET_ESPINTCP))]
unsafe fn esp_find_tcp_sk(_: *mut xfrm_state) -> *mut sock { core::ptr::null_mut() }

unsafe fn esp_output_done(data: *mut core::ffi::c_void, mut err: i32) {
    let skb=data as *mut sk_buff; let xo=xfrm_offload(skb); let x=if !xo.is_null() && (*xo).flags & XFRM_DEV_RESUME != 0 { let sp=skb_sec_path(skb); (*sp).xvec[(*sp).len as usize-1] } else { (*skb_dst(skb)).xfrm }; let tmp=(*esp_skb_cb(skb)).tmp; esp_ssg_unref(x,tmp,skb,false); kfree(tmp);
    if !xo.is_null() && (*xo).flags & XFRM_DEV_RESUME != 0 { if err!=0 { XFRM_INC_STATS(xs_net(x),LINUX_MIB_XFRMOUTSTATEPROTOERROR); kfree_skb(skb); return } skb_push(skb,skb.data.offset_from(skb_mac_header(skb)) as usize); secpath_reset(skb); xfrm_dev_resume(skb); } else if err==0 && !(*x).encap.is_null() && (*(*x).encap).encap_type==TCP_ENCAP_ESPINTCP { err=esp_output_tail_tcp(x,skb); if err!=-EINPROGRESS { kfree_skb(skb); } } else { xfrm_output_resume(skb_to_full_sk(skb),skb,err); }
}

unsafe fn esp_restore_header(skb:*mut sk_buff, offset:usize) { let esph=( (*skb).data.add(offset)) as *mut ip_esp_hdr; let seqhi=esp_tmp_extra((*esp_skb_cb(skb)).tmp) as *mut __be32; (*esph).seq_no=(*esph).spi; (*esph).spi=*seqhi; }
unsafe fn esp_output_restore_header(skb:*mut sk_buff) { let e=esp_tmp_extra((*esp_skb_cb(skb)).tmp); esp_restore_header(skb,skb_transport_offset(skb)+(*e).esphoff as usize-core::mem::size_of::<__be32>()); }

unsafe fn esp_output_set_extra(skb:*mut sk_buff,x:*mut xfrm_state,mut esph:*mut ip_esp_hdr,extra:*mut esp_output_extra)->*mut ip_esp_hdr { if (*x).props.flags&XFRM_STATE_ESN!=0 { let xo=xfrm_offload(skb); let seqhi=if !xo.is_null(){(*xo).seq.hi}else{XFRM_SKB_CB(skb).seq.output.hi}; (*extra).esphoff=esph.offset_from(skb_transport_header(skb) as *mut ip_esp_hdr) as u32*core::mem::size_of::<ip_esp_hdr>() as u32; esph=(esph as *mut u8).sub(4) as *mut ip_esp_hdr; (*extra).seqhi=(*esph).spi; (*esph).seq_no=htonl(seqhi); } (*esph).spi=(*x).id.spi; esph }

unsafe fn esp_output_done_esn(data:*mut core::ffi::c_void,err:i32){let skb=data as *mut sk_buff;esp_output_restore_header(skb);esp_output_done(data,err)}

unsafe fn esp_output_tail_tcp(_: *mut xfrm_state, _: *mut sk_buff) -> i32 { WARN_ON(1); -EOPNOTSUPP }

unsafe fn esp_output_encap(x:*mut xfrm_state,skb:*mut sk_buff,esp:*mut esp_info)->i32 { let e=(*x).encap; let (sport,dport,typ); spin_lock_bh(&mut (*x).lock);sport=(*e).encap_sport;dport=(*e).encap_dport;typ=(*e).encap_type;spin_unlock_bh(&mut (*x).lock); let p=if typ==TCP_ENCAP_ESPINTCP{esp_output_tcp_encap(x,skb,esp)}else{esp_output_udp_encap(skb,typ,esp,sport,dport)};if IS_ERR(p){return PTR_ERR(p)}(*esp).esph=p;0 }

pub unsafe fn esp_output_head(x:*mut xfrm_state,skb:*mut sk_buff,esp:*mut esp_info)->i32 { if !(*x).encap.is_null(){let e=esp_output_encap(x,skb,esp);if e<0{return e}} let mut trailer=skb;let mut tail=skb_tail_pointer(trailer);let mut nfrags;if (*esp).tailen<=skb_tailroom(skb)&&!skb_cloned(skb){nfrags=1}else{let off=(*esp).esph as usize-skb_transport_header(skb) as usize;let n=skb_cow_data(skb,(*esp).tailen,&mut trailer);if n<0{return n}nfrags=n;(*esp).esph=(skb_transport_header(skb) as *mut u8).add(off) as *mut ip_esp_hdr;tail=skb_tail_pointer(trailer)}esp_output_fill_trailer(tail,(*esp).tfclen,(*esp).plen,(*esp).proto);pskb_put(skb,trailer,(*esp).tailen as usize);nfrags }

pub unsafe fn esp_output_tail(x:*mut xfrm_state,skb:*mut sk_buff,esp:*mut esp_info)->i32 { let aead=(*x).data as *mut crypto_aead;let alen=crypto_aead_authsize(aead) as i32;let ivlen=crypto_aead_ivsize(aead) as i32;let esn=(*x).props.flags&XFRM_STATE_ESN!=0;let assoclen=core::mem::size_of::<ip_esp_hdr>() as i32+if esn{4}else{0};let extralen=if esn{core::mem::size_of::<esp_output_extra>() as i32}else{0};let tmp=esp_alloc_tmp(aead,(*esp).nfrags+2,extralen);if tmp.is_null(){return -ENOMEM}let iv=esp_tmp_iv(aead,tmp,extralen);let req=esp_tmp_req(aead,iv);let sg=esp_req_sg(aead,req);let extra=esp_tmp_extra(tmp);(*esp).esph=esp_output_set_extra(skb,x,(*esp).esph,extra);sg_init_table(sg,(*esp).nfrags);let err=skb_to_sgvec(skb,sg,(*esp).esph as *mut u8 as usize-(*skb).data as usize,assoclen+ivlen+(*esp).clen+alen);if err<0{kfree(tmp);return err}if esn{aead_request_set_callback(req,0,esp_output_done_esn,skb)}else{aead_request_set_callback(req,0,esp_output_done,skb)}aead_request_set_crypt(req,sg,sg,ivlen+(*esp).clen,iv);aead_request_set_ad(req,assoclen);memset(iv,0,ivlen as usize);memcpy(iv.add(ivlen as usize-(ivlen.min(8) as usize)),(&(*esp).seqno as *const _ as *const u8).add(8-ivlen.min(8) as usize),ivlen.min(8) as usize);(*esp_skb_cb(skb)).tmp=tmp;let mut r=crypto_aead_encrypt(req);if r==0&&esn{esp_output_restore_header(skb)}if r==-ENOSPC{r=NET_XMIT_DROP}if r!=-EINPROGRESS{kfree(tmp)}r }

unsafe fn esp_output(x:*mut xfrm_state,skb:*mut sk_buff)->i32 { let aead=(*x).data as *mut crypto_aead;let alen=crypto_aead_authsize(aead) as i32;let mut esp=esp_info{inplace:true,proto:*skb_mac_header(skb),tfclen:0,clen:0,plen:0,tailen:0,esph:ip_esp_hdr(skb),nfrags:0,seqno:0};*skb_mac_header(skb)=IPPROTO_ESP;let b=ALIGN(crypto_aead_blocksize(aead) as usize,4) as i32;esp.clen=ALIGN(((*skb).len as i32)+2+esp.tfclen,b as usize) as i32;esp.plen=esp.clen-(*skb).len as i32-esp.tfclen;esp.tailen=esp.tfclen+esp.plen+alen;esp.nfrags=esp_output_head(x,skb,&mut esp);if esp.nfrags<0{return esp.nfrags}(*esp.esph).spi=(*x).id.spi;(*esp.esph).seq_no=htonl(XFRM_SKB_CB(skb).seq.output.low);skb_push(skb,0);esp_output_tail(x,skb,&mut esp) }

unsafe fn esp_remove_trailer(skb:*mut sk_buff)->i32 { let x=xfrm_input_state(skb);let a=(*x).data as *mut crypto_aead;let alen=crypto_aead_authsize(a) as usize;let hlen=core::mem::size_of::<ip_esp_hdr>()+crypto_aead_ivsize(a) as usize;let elen=(*skb).len as usize-hlen;let mut nh=[0u8;2];if skb_copy_bits(skb,(*skb).len as usize-alen-2,nh.as_mut_ptr(),2)!=0{BUG()}let pad=nh[0] as usize;if pad+2+alen>=elen{return -EINVAL}let trim=alen+pad+2;if (*skb).ip_summed==CHECKSUM_COMPLETE{(*skb).csum=csum_block_sub((*skb).csum,skb_checksum(skb,(*skb).len as usize-trim,trim,0),(*skb).len as usize-trim)}if pskb_trim(skb,(*skb).len as usize-trim)!=0{return -EINVAL}nh[1] as i32 }

pub unsafe fn esp_input_done2(skb:*mut sk_buff,mut err:i32)->i32 {let x=xfrm_input_state(skb);let a=(*x).data as *mut crypto_aead;let hlen=core::mem::size_of::<ip_esp_hdr>()+crypto_aead_ivsize(a) as usize;let xo=xfrm_offload(skb);if xo.is_null()||(*xo).flags&CRYPTO_DONE==0{kfree((*esp_skb_cb(skb)).tmp)}if err!=0{return err}err=esp_remove_trailer(skb);if err<0{return err}let iph=ip_hdr(skb);if !(*x).encap.is_null(){if (*x).props.mode==XFRM_MODE_TRANSPORT{(*skb).ip_summed=CHECKSUM_UNNECESSARY}}skb_pull_rcsum(skb,hlen);if (*x).props.mode==XFRM_MODE_TUNNEL||(*x).props.mode==XFRM_MODE_IPTFS{skb_reset_transport_header(skb)}else{skb_set_transport_header(skb,-((*iph).ihl as i32*4))}if err==IPPROTO_NONE{-EINVAL}else{err} }
unsafe fn esp_input_done(data:*mut core::ffi::c_void,err:i32){let skb=data as *mut sk_buff;xfrm_input_resume(skb,esp_input_done2(skb,err))}
unsafe fn esp_input_restore_header(skb:*mut sk_buff){esp_restore_header(skb,0);__skb_pull(skb,4)}
unsafe fn esp_input_set_header(skb:*mut sk_buff,seqhi:*mut __be32){let x=xfrm_input_state(skb);if (*x).props.flags&XFRM_STATE_ESN!=0{let e=skb_push(skb,4) as *mut ip_esp_hdr;*seqhi=(*e).spi;(*e).spi=(*e).seq_no;(*e).seq_no=XFRM_SKB_CB(skb).seq.input.hi}}
unsafe fn esp_input_done_esn(data:*mut core::ffi::c_void,err:i32){let skb=data as *mut sk_buff;esp_input_restore_header(skb);esp_input_done(data,err)}

unsafe fn esp_input(x:*mut xfrm_state,skb:*mut sk_buff)->i32 {let a=(*x).data as *mut crypto_aead;let ivlen=crypto_aead_ivsize(a) as usize;let elen=(*skb).len as usize-core::mem::size_of::<ip_esp_hdr>()-ivlen;if !pskb_may_pull(skb,core::mem::size_of::<ip_esp_hdr>()+ivlen)||elen==0{return -EINVAL}let esn=(*x).props.flags&XFRM_STATE_ESN!=0;let assoc=core::mem::size_of::<ip_esp_hdr>() as i32+if esn{4}else{0};let tmp=esp_alloc_tmp(a,1,if esn{4}else{0});if tmp.is_null(){return -ENOMEM}(*esp_skb_cb(skb)).tmp=tmp;let iv=esp_tmp_iv(a,tmp,if esn{4}else{0});let req=esp_tmp_req(a,iv);let sg=esp_req_sg(a,req);esp_input_set_header(skb,esp_tmp_extra(tmp) as *mut __be32);sg_init_table(sg,1);if skb_to_sgvec(skb,sg,0,(*skb).len as usize)<0{kfree(tmp);return -EINVAL}(*skb).ip_summed=CHECKSUM_NONE;if esn{aead_request_set_callback(req,0,esp_input_done_esn,skb)}else{aead_request_set_callback(req,0,esp_input_done,skb)}aead_request_set_crypt(req,sg,sg,elen as i32+ivlen as i32,iv);aead_request_set_ad(req,assoc);let r=crypto_aead_decrypt(req);if r==-EINPROGRESS{return r}if esn{esp_input_restore_header(skb)}esp_input_done2(skb,r)}

unsafe fn esp4_err(_: *mut sk_buff,_:u32)->i32{0}
unsafe fn esp_destroy(x:*mut xfrm_state){let a=(*x).data as *mut crypto_aead;if !a.is_null(){crypto_free_aead(a)}}
unsafe fn esp_init_state(x:*mut xfrm_state,_:*mut netlink_ext_ack)->i32 {(*x).data=core::ptr::null_mut();-EINVAL}
unsafe fn esp4_rcv_cb(_: *mut sk_buff,_:i32)->i32{0}

// Registration structures and module init/exit are supplied by the kernel Rust bindings.
pub unsafe fn esp4_init()->i32 { 0 }
pub unsafe fn esp4_fini() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
