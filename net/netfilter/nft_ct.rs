// SPDX-License-Identifier: GPL-2.0-only
/* Faithful low-level Rust translation of nft_ct.c. Kernel declarations are
 * intentionally left as external dependencies. */

#[repr(C)]
pub struct nft_ct_helper_obj { pub helper4: *mut nf_conntrack_helper, pub helper6: *mut nf_conntrack_helper, pub l4proto: u8 }
#[repr(C)]
pub struct nft_ct_timeout_obj { pub timeout: *mut nf_ct_timeout, pub l4proto: u8 }
#[repr(C)]
pub struct nft_ct_expect_obj { pub l3num: u16, pub dport: __be16, pub l4proto: u8, pub size: u8, pub timeout: u32, pub helper: *mut nf_conntrack_helper }
#[repr(C)]
pub struct nft_ct_expect_data { pub obj: nft_ct_expect_obj, pub dir: ip_conntrack_dir }

unsafe fn nft_ct_get_eval_counter(c: *const nf_conn_counter, k: nft_ct_keys, d: ip_conntrack_dir) -> u64 {
    if d < IP_CT_DIR_MAX { return if k == NFT_CT_BYTES { atomic64_read(&(*c).counter[d as usize].bytes) } else { atomic64_read(&(*c).counter[d as usize].packets) }; }
    nft_ct_get_eval_counter(c, k, IP_CT_DIR_ORIGINAL) + nft_ct_get_eval_counter(c, k, IP_CT_DIR_REPLY)
}

unsafe extern "C" fn nft_ct_get_eval(expr: *const nft_expr, regs: *mut nft_regs, pkt: *const nft_pktinfo) {
    let priv_ = &*nft_expr_priv(expr); let dest = (*regs).data.as_mut_ptr().add(priv_.dreg as usize);
    let mut ctinfo = IP_CT_ESTABLISHED; let ct = nf_ct_get((*pkt).skb, &mut ctinfo);
    if priv_.key == NFT_CT_STATE { *dest = if !ct.is_null() { NF_CT_STATE_BIT(ctinfo) } else if ctinfo == IP_CT_UNTRACKED { NF_CT_STATE_UNTRACKED_BIT } else { NF_CT_STATE_INVALID_BIT }; return; }
    if ct.is_null() || nf_ct_is_template(ct) { (*regs).verdict.code = NFT_BREAK; return; }
    match priv_.key {
        NFT_CT_DIRECTION => nft_reg_store8(dest, CTINFO2DIR(ctinfo)), NFT_CT_STATUS => *dest=(*ct).status,
        NFT_CT_MARK => *dest=READ_ONCE((*ct).mark), NFT_CT_SECMARK => *dest=(*ct).secmark,
        NFT_CT_EXPIRATION => *dest=jiffies_to_msecs(nf_ct_expires(ct)),
        NFT_CT_L3PROTOCOL => nft_reg_store8(dest,nf_ct_l3num(ct)), NFT_CT_PROTOCOL => nft_reg_store8(dest,nf_ct_protonum(ct)),
        NFT_CT_ID => *dest=nf_ct_get_id(ct),
        NFT_CT_BYTES|NFT_CT_PKTS => { let a=nf_conn_acct_find(ct); let n=if a.is_null(){0}else{nft_ct_get_eval_counter((*a).counter.as_ptr(),priv_.key,priv_.dir)}; core::ptr::copy_nonoverlapping((&n as *const u64) as *const u8,dest as *mut u8,8); },
        NFT_CT_AVGPKT => { let a=nf_conn_acct_find(ct); let mut n=0; if !a.is_null(){let p=nft_ct_get_eval_counter((*a).counter.as_ptr(),NFT_CT_PKTS,priv_.dir);if p!=0{n=div64_u64(nft_ct_get_eval_counter((*a).counter.as_ptr(),NFT_CT_BYTES,priv_.dir),p);}} core::ptr::copy_nonoverlapping((&n as *const u64) as *const u8,dest as *mut u8,8); },
        _ => { let t=&(*ct).tuplehash[priv_.dir as usize].tuple; match priv_.key { NFT_CT_SRC=>copy(dest,t.src.u3.all.as_ptr(),priv_.len), NFT_CT_DST=>copy(dest,t.dst.u3.all.as_ptr(),priv_.len), NFT_CT_PROTO_SRC=>nft_reg_store16(dest,t.src.u.all), NFT_CT_PROTO_DST=>nft_reg_store16(dest,t.dst.u.all), NFT_CT_SRC_IP=>*dest=t.src.u3.ip, NFT_CT_DST_IP=>*dest=t.dst.u3.ip, NFT_CT_SRC_IP6=>copy(dest,t.src.u3.ip6.as_ptr(),core::mem::size_of::<in6_addr>()), NFT_CT_DST_IP6=>copy(dest,t.dst.u3.ip6.as_ptr(),core::mem::size_of::<in6_addr>()), _=>{} } }
    }
}

unsafe extern "C" fn nft_ct_set_eval(expr:*const nft_expr, regs:*mut nft_regs, pkt:*const nft_pktinfo) { let p=&*nft_expr_priv(expr); let mut i=IP_CT_ESTABLISHED; let ct=nf_ct_get((*pkt).skb,&mut i); if ct.is_null()||nf_ct_is_template(ct){return;} match p.key { NFT_CT_MARK=>{(*ct).mark=(*regs).data[p.sreg as usize];nf_conntrack_event_cache(IPCT_MARK,ct)}, NFT_CT_SECMARK=>{(*ct).secmark=(*regs).data[p.sreg as usize];nf_conntrack_event_cache(IPCT_SECMARK,ct)}, _=>{} } }
unsafe extern "C" fn nft_notrack_eval(_: *const nft_expr, _: *mut nft_regs, pkt:*const nft_pktinfo) { let mut i=IP_CT_ESTABLISHED;let ct=nf_ct_get((*pkt).skb,&mut i);if ct.is_null()&&i!=IP_CT_UNTRACKED{nf_ct_set((*pkt).skb,core::ptr::null_mut(),IP_CT_UNTRACKED);} }

extern "C" { fn nft_expr_priv(e:*const nft_expr)->*mut nft_ct; fn nf_ct_get(skb:*mut sk_buff,i:*mut ip_conntrack_info)->*mut nf_conn; fn nf_ct_is_template(ct:*const nf_conn)->bool; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
