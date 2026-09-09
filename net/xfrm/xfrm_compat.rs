// SPDX-License-Identifier: GPL-2.0
/* XFRM compat layer. Direct low-level translation of xfrm_compat.c. */

#[repr(C)]
struct compat_xfrm_lifetime_cfg {
    soft_byte_limit: compat_u64, hard_byte_limit: compat_u64,
    soft_packet_limit: compat_u64, hard_packet_limit: compat_u64,
    soft_add_expires_seconds: compat_u64, hard_add_expires_seconds: compat_u64,
    soft_use_expires_seconds: compat_u64, hard_use_expires_seconds: compat_u64,
}

#[repr(C)]
struct compat_xfrm_lifetime_cur { bytes: compat_u64, packets: compat_u64, add_time: compat_u64, use_time: compat_u64 }

#[repr(C)]
struct compat_xfrm_userpolicy_info {
    sel: xfrm_selector,
    lft: compat_xfrm_lifetime_cfg,
    curlft: compat_xfrm_lifetime_cur,
    priority: __u32, index: __u32,
    dir: u8, action: u8, flags: u8, share: u8,
}

#[repr(C)]
struct compat_xfrm_usersa_info {
    sel: xfrm_selector, id: xfrm_id, saddr: xfrm_address_t,
    lft: compat_xfrm_lifetime_cfg, curlft: compat_xfrm_lifetime_cur,
    stats: xfrm_stats, seq: __u32, reqid: __u32, family: u16,
    mode: u8, replay_window: u8, flags: u8,
}

#[repr(C)]
struct compat_xfrm_user_acquire {
    id: xfrm_id, saddr: xfrm_address_t, sel: xfrm_selector,
    policy: compat_xfrm_userpolicy_info,
    aalgos: __u32, ealgos: __u32, calgos: __u32, seq: __u32,
}

#[repr(C)]
struct compat_xfrm_userspi_info { info: compat_xfrm_usersa_info, min: __u32, max: __u32 }
#[repr(C)]
struct compat_xfrm_user_expire { state: compat_xfrm_usersa_info, hard: u8 }
#[repr(C)]
struct compat_xfrm_user_polexpire { pol: compat_xfrm_userpolicy_info, hard: u8 }

static compat_msg_min: [c_int; XFRM_NR_MSGTYPES] = [
    [XFRM_MSG_NEWSA - XFRM_MSG_BASE] => size_of::<compat_xfrm_usersa_info>() as c_int,
    [XFRM_MSG_DELSA - XFRM_MSG_BASE] => size_of::<xfrm_usersa_id>() as c_int,
    [XFRM_MSG_GETSA - XFRM_MSG_BASE] => size_of::<xfrm_usersa_id>() as c_int,
    [XFRM_MSG_NEWPOLICY - XFRM_MSG_BASE] => size_of::<compat_xfrm_userpolicy_info>() as c_int,
    [XFRM_MSG_DELPOLICY - XFRM_MSG_BASE] => size_of::<xfrm_userpolicy_id>() as c_int,
    [XFRM_MSG_GETPOLICY - XFRM_MSG_BASE] => size_of::<xfrm_userpolicy_id>() as c_int,
    [XFRM_MSG_ALLOCSPI - XFRM_MSG_BASE] => size_of::<compat_xfrm_userspi_info>() as c_int,
    [XFRM_MSG_ACQUIRE - XFRM_MSG_BASE] => size_of::<compat_xfrm_user_acquire>() as c_int,
    [XFRM_MSG_EXPIRE - XFRM_MSG_BASE] => size_of::<compat_xfrm_user_expire>() as c_int,
    [XFRM_MSG_UPDPOLICY - XFRM_MSG_BASE] => size_of::<compat_xfrm_userpolicy_info>() as c_int,
    [XFRM_MSG_UPDSA - XFRM_MSG_BASE] => size_of::<compat_xfrm_usersa_info>() as c_int,
    [XFRM_MSG_POLEXPIRE - XFRM_MSG_BASE] => size_of::<compat_xfrm_user_polexpire>() as c_int,
    [XFRM_MSG_FLUSHSA - XFRM_MSG_BASE] => size_of::<xfrm_usersa_flush>() as c_int,
    [XFRM_MSG_FLUSHPOLICY - XFRM_MSG_BASE] => 0,
    [XFRM_MSG_NEWAE - XFRM_MSG_BASE] => size_of::<xfrm_aevent_id>() as c_int,
    [XFRM_MSG_GETAE - XFRM_MSG_BASE] => size_of::<xfrm_aevent_id>() as c_int,
    [XFRM_MSG_REPORT - XFRM_MSG_BASE] => size_of::<xfrm_user_report>() as c_int,
    [XFRM_MSG_MIGRATE - XFRM_MSG_BASE] => size_of::<xfrm_userpolicy_id>() as c_int,
    [XFRM_MSG_NEWSADINFO - XFRM_MSG_BASE] => size_of::<u32>() as c_int,
    [XFRM_MSG_GETSADINFO - XFRM_MSG_BASE] => size_of::<u32>() as c_int,
    [XFRM_MSG_NEWSPDINFO - XFRM_MSG_BASE] => size_of::<u32>() as c_int,
    [XFRM_MSG_GETSPDINFO - XFRM_MSG_BASE] => size_of::<u32>() as c_int,
    [XFRM_MSG_MAPPING - XFRM_MSG_BASE] => size_of::<xfrm_user_mapping>() as c_int,
    [XFRM_MSG_MIGRATE_STATE - XFRM_MSG_BASE] => size_of::<xfrm_user_migrate_state>() as c_int,
];

// Attribute policy is preserved as a direct static table; dependent kernel types/constants are external.
static compat_policy: [nla_policy; XFRMA_MAX + 1] = [
    [XFRMA_UNSPEC] => nla_policy { strict_start_type: XFRMA_SA_DIR },
    [XFRMA_SA] => nla_policy { len: size_of::<compat_xfrm_usersa_info>() as u16 },
    [XFRMA_POLICY] => nla_policy { len: size_of::<compat_xfrm_userpolicy_info>() as u16 },
    [XFRMA_LASTUSED] => nla_policy { type_: NLA_U64 },
    [XFRMA_ALG_AUTH_TRUNC] => nla_policy { len: size_of::<xfrm_algo_auth>() as u16 },
    [XFRMA_ALG_AEAD] => nla_policy { len: size_of::<xfrm_algo_aead>() as u16 },
    [XFRMA_ALG_AUTH] => nla_policy { len: size_of::<xfrm_algo>() as u16 },
    [XFRMA_ALG_CRYPT] => nla_policy { len: size_of::<xfrm_algo>() as u16 },
    [XFRMA_ALG_COMP] => nla_policy { len: size_of::<xfrm_algo>() as u16 },
    [XFRMA_ENCAP] => nla_policy { len: size_of::<xfrm_encap_tmpl>() as u16 },
    [XFRMA_TMPL] => nla_policy { len: size_of::<xfrm_user_tmpl>() as u16 },
    [XFRMA_SEC_CTX] => nla_policy { len: size_of::<xfrm_user_sec_ctx>() as u16 },
    [XFRMA_LTIME_VAL] => nla_policy { len: size_of::<xfrm_lifetime_cur>() as u16 },
    [XFRMA_REPLAY_VAL] => nla_policy { len: size_of::<xfrm_replay_state>() as u16 },
    [XFRMA_REPLAY_THRESH] => nla_policy { type_: NLA_U32 },
    [XFRMA_ETIMER_THRESH] => nla_policy { type_: NLA_U32 },
    [XFRMA_SRCADDR] => nla_policy { len: size_of::<xfrm_address_t>() as u16 },
    [XFRMA_COADDR] => nla_policy { len: size_of::<xfrm_address_t>() as u16 },
    [XFRMA_POLICY_TYPE] => nla_policy { len: size_of::<xfrm_userpolicy_type>() as u16 },
    [XFRMA_MIGRATE] => nla_policy { len: size_of::<xfrm_user_migrate>() as u16 },
    [XFRMA_KMADDRESS] => nla_policy { len: size_of::<xfrm_user_kmaddress>() as u16 },
    [XFRMA_MARK] => nla_policy { len: size_of::<xfrm_mark>() as u16 },
    [XFRMA_TFCPAD] => nla_policy { type_: NLA_U32 },
    [XFRMA_REPLAY_ESN_VAL] => nla_policy { len: size_of::<xfrm_replay_state_esn>() as u16 },
    [XFRMA_SA_EXTRA_FLAGS] => nla_policy { type_: NLA_U32 },
    [XFRMA_PROTO] => nla_policy { type_: NLA_U8 },
    [XFRMA_ADDRESS_FILTER] => nla_policy { len: size_of::<xfrm_address_filter>() as u16 },
    [XFRMA_OFFLOAD_DEV] => nla_policy { len: size_of::<xfrm_user_offload>() as u16 },
    [XFRMA_SET_MARK] => nla_policy { type_: NLA_U32 },
    [XFRMA_SET_MARK_MASK] => nla_policy { type_: NLA_U32 },
    [XFRMA_IF_ID] => nla_policy { type_: NLA_U32 },
    [XFRMA_MTIMER_THRESH] => nla_policy { type_: NLA_U32 },
    [XFRMA_SA_DIR] => NLA_POLICY_RANGE!(NLA_U8, XFRM_SA_DIR_IN, XFRM_SA_DIR_OUT),
    [XFRMA_NAT_KEEPALIVE_INTERVAL] => nla_policy { type_: NLA_U32 },
    [XFRMA_SA_PCPU] => nla_policy { type_: NLA_U32 },
];

unsafe fn xfrm_nlmsg_put_compat(skb: *mut sk_buff, nlh_src: *const nlmsghdr, type_: u16) -> *mut nlmsghdr {
    let payload = compat_msg_min[type_ as usize];
    let src_len = xfrm_msg_min[type_ as usize];
    if WARN_ON_ONCE(src_len < payload) { return ERR_PTR(-EMSGSIZE); }
    let nlh_dst = nlmsg_put(skb, (*nlh_src).nlmsg_pid, (*nlh_src).nlmsg_seq, (*nlh_src).nlmsg_type, payload, (*nlh_src).nlmsg_flags);
    if nlh_dst.is_null() { return ERR_PTR(-EMSGSIZE); }
    memset(nlmsg_data(nlh_dst), 0, payload as usize);
    match (*nlh_src).nlmsg_type {
        XFRM_MSG_DELSA | XFRM_MSG_DELPOLICY | XFRM_MSG_FLUSHSA | XFRM_MSG_FLUSHPOLICY |
        XFRM_MSG_NEWAE | XFRM_MSG_REPORT | XFRM_MSG_MIGRATE | XFRM_MSG_MIGRATE_STATE |
        XFRM_MSG_NEWSADINFO | XFRM_MSG_NEWSPDINFO | XFRM_MSG_MAPPING => {
            WARN_ON_ONCE(src_len != payload); memcpy(nlmsg_data(nlh_dst), nlmsg_data(nlh_src), src_len as usize);
        }
        XFRM_MSG_NEWSA | XFRM_MSG_NEWPOLICY | XFRM_MSG_UPDSA | XFRM_MSG_UPDPOLICY => {
            WARN_ON_ONCE(src_len != payload + 4); memcpy(nlmsg_data(nlh_dst), nlmsg_data(nlh_src), payload as usize);
        }
        XFRM_MSG_EXPIRE => { let src = nlmsg_data(nlh_src) as *const xfrm_user_expire; let dst = nlmsg_data(nlh_dst) as *mut compat_xfrm_user_expire; memcpy(dst as *mut _, src as *const _, size_of::<compat_xfrm_usersa_info>()); (*dst).hard = (*src).hard; }
        XFRM_MSG_ACQUIRE => { let src = nlmsg_data(nlh_src) as *const xfrm_user_acquire; let dst = nlmsg_data(nlh_dst) as *mut compat_xfrm_user_acquire; memcpy(dst as *mut _, src as *const _, offset_of!(compat_xfrm_user_acquire, aalgos)); (*dst).aalgos=(*src).aalgos; (*dst).ealgos=(*src).ealgos; (*dst).calgos=(*src).calgos; (*dst).seq=(*src).seq; }
        XFRM_MSG_POLEXPIRE => { let src = nlmsg_data(nlh_src) as *const xfrm_user_polexpire; let dst = nlmsg_data(nlh_dst) as *mut compat_xfrm_user_polexpire; memcpy(dst as *mut _, src as *const _, size_of::<compat_xfrm_userpolicy_info>()); (*dst).hard=(*src).hard; }
        XFRM_MSG_ALLOCSPI => { let src = nlmsg_data(nlh_src) as *const xfrm_userspi_info; let dst = nlmsg_data(nlh_dst) as *mut compat_xfrm_userspi_info; memcpy(dst as *mut _, src as *const _, size_of::<compat_xfrm_usersa_info>()); (*dst).min=(*src).min; (*dst).max=(*src).max; }
        _ => { pr_warn_once!("unsupported nlmsg_type %d\n", (*nlh_src).nlmsg_type); return ERR_PTR(-EOPNOTSUPP); }
    }
    nlh_dst
}

unsafe fn xfrm_nla_cpy(dst: *mut sk_buff, src: *const nlattr, len: c_int) -> c_int { nla_put(dst, (*src).nla_type, len, nla_data(src)) }

unsafe fn xfrm_xlate64_attr(dst: *mut sk_buff, src: *const nlattr) -> c_int {
    match (*src).nla_type {
        XFRMA_PAD => 0,
        XFRMA_UNSPEC | XFRMA_ALG_AUTH | XFRMA_ALG_CRYPT | XFRMA_ALG_COMP | XFRMA_ENCAP | XFRMA_TMPL => xfrm_nla_cpy(dst,src,nla_len(src)),
        XFRMA_SA => xfrm_nla_cpy(dst,src,size_of::<compat_xfrm_usersa_info>() as c_int),
        XFRMA_POLICY => xfrm_nla_cpy(dst,src,size_of::<compat_xfrm_userpolicy_info>() as c_int),
        XFRMA_LTIME_VAL | XFRMA_LASTUSED => nla_put_64bit(dst,(*src).nla_type,nla_len(src),nla_data(src),XFRMA_PAD),
        XFRMA_SEC_CTX | XFRMA_REPLAY_VAL | XFRMA_REPLAY_THRESH | XFRMA_ETIMER_THRESH | XFRMA_SRCADDR | XFRMA_COADDR |
        XFRMA_POLICY_TYPE | XFRMA_MIGRATE | XFRMA_ALG_AEAD | XFRMA_KMADDRESS | XFRMA_ALG_AUTH_TRUNC | XFRMA_MARK |
        XFRMA_TFCPAD | XFRMA_REPLAY_ESN_VAL | XFRMA_SA_EXTRA_FLAGS | XFRMA_PROTO | XFRMA_ADDRESS_FILTER |
        XFRMA_OFFLOAD_DEV | XFRMA_SET_MARK | XFRMA_SET_MARK_MASK | XFRMA_IF_ID | XFRMA_MTIMER_THRESH | XFRMA_SA_DIR |
        XFRMA_NAT_KEEPALIVE_INTERVAL | XFRMA_SA_PCPU | XFRMA_IPTFS_DROP_TIME | XFRMA_IPTFS_REORDER_WINDOW |
        XFRMA_IPTFS_DONT_FRAG | XFRMA_IPTFS_INIT_DELAY | XFRMA_IPTFS_MAX_QSIZE | XFRMA_IPTFS_PKT_SIZE => xfrm_nla_cpy(dst,src,nla_len(src)),
        _ => { BUILD_BUG_ON!(XFRMA_MAX != XFRMA_IPTFS_PKT_SIZE); pr_warn_once!("unsupported nla_type %d\n",(*src).nla_type); -EOPNOTSUPP }
    }
}

/* The remaining routines preserve the C translator's pointer-level behavior. */
unsafe fn xfrm_xlate64(dst: *mut sk_buff, nlh_src: *const nlmsghdr) -> c_int { let type_ = (*nlh_src).nlmsg_type - XFRM_MSG_BASE; let nlh_dst=xfrm_nlmsg_put_compat(dst,nlh_src,type_); if IS_ERR(nlh_dst){return PTR_ERR(nlh_dst)}; let attrs=nlmsg_attrdata(nlh_src,xfrm_msg_min[type_ as usize]); let mut len=nlmsg_attrlen(nlh_src,xfrm_msg_min[type_ as usize]); let mut nla=std::ptr::null_mut(); let mut remaining=0; nla_for_each_attr!(nla,attrs,len,remaining){let err=if (*nlh_src).nlmsg_type==XFRM_MSG_NEWSPDINFO{xfrm_nla_cpy(dst,nla,nla_len(nla))}else{xfrm_xlate64_attr(dst,nla)};if err!=0{return err;}} nlmsg_end(dst,nlh_dst);0 }

unsafe fn xfrm_alloc_compat(skb: *mut sk_buff, nlh_src: *const nlmsghdr) -> c_int { let type_=(*nlh_src).nlmsg_type-XFRM_MSG_BASE; let mut new=std::ptr::null_mut(); if type_ as usize>=xfrm_msg_min.len(){pr_warn_once!("unsupported nlmsg_type %d\n",(*nlh_src).nlmsg_type);return -EOPNOTSUPP;} if (*skb_shinfo(skb)).frag_list.is_null(){new=alloc_skb((*skb).len+skb_tailroom(skb),GFP_ATOMIC);if new.is_null(){return -ENOMEM;}(*skb_shinfo(skb)).frag_list=new;}let err=xfrm_xlate64((*skb_shinfo(skb)).frag_list,nlh_src);if err!=0{if !new.is_null(){kfree_skb(new);(*skb_shinfo(skb)).frag_list=std::ptr::null_mut();}return err;}0 }

unsafe fn xfrm_user_rcv_calculate_len64(src: *const nlmsghdr, attrs: *mut *mut nlattr, maxtype: c_int) -> usize { let mut len=nlmsg_len(src); match (*src).nlmsg_type {XFRM_MSG_NEWSA|XFRM_MSG_NEWPOLICY|XFRM_MSG_ALLOCSPI|XFRM_MSG_ACQUIRE|XFRM_MSG_UPDPOLICY|XFRM_MSG_UPDSA=>len+=4,XFRM_MSG_EXPIRE|XFRM_MSG_POLEXPIRE=>len+=8,XFRM_MSG_NEWSPDINFO=>return len,_=>{}} if WARN_ON_ONCE(maxtype!=0){return len;} if !(*attrs.add(XFRMA_SA as usize)).is_null(){len+=4;}if !(*attrs.add(XFRMA_POLICY as usize)).is_null(){len+=4;}len }

// Exact 32-bit-to-64-bit attribute copying and message translation, including external kernel helpers.
unsafe fn xfrm_attr_cpy32(dst:*mut c_void,pos:*mut usize,src:*const nlattr,size:usize,copy_len:usize,payload:usize)->c_int{let nlmsg=dst as *mut nlmsghdr;let mut copy_len=copy_len;if WARN_ON_ONCE(copy_len>payload){copy_len=payload;}if size-*pos<nla_attr_size(payload){return -ENOBUFS;}let nla=(dst as *mut u8).add(*pos) as *mut nlattr;memcpy(nla as *mut _,src as *const _,nla_attr_size(copy_len));(*nla).nla_len=nla_attr_size(payload) as u16;*pos+=nla_attr_size(copy_len);(*nlmsg).nlmsg_len+=(*nla).nla_len as u32;memset((dst as *mut u8).add(*pos) as *mut _,0,payload-copy_len);*pos+=payload-copy_len;0}

unsafe fn xfrm_xlate32_attr(dst:*mut c_void,nla:*const nlattr,pos:*mut usize,size:usize,extack:*mut netlink_ext_ack)->c_int { let mut type_=nla_type(nla); if type_>XFRMA_MAX { NL_SET_ERR_MSG!(extack,"Bad attribute"); return -EOPNOTSUPP; } type_=array_index_nospec(type_,XFRMA_MAX+1); if nla_len(nla)<compat_policy[type_ as usize].len { NL_SET_ERR_MSG!(extack,"Attribute bad length"); return -EOPNOTSUPP; } let pol_len32=compat_policy[type_ as usize].len; let pol_len64=xfrma_policy[type_ as usize].len; if pol_len32!=pol_len64 { if nla_len(nla)!=pol_len32 as c_int { NL_SET_ERR_MSG!(extack,"Attribute bad length"); return -EOPNOTSUPP; } let err=xfrm_attr_cpy32(dst,pos,nla,size,pol_len32 as usize,pol_len64 as usize); if err!=0{return err;} } xfrm_attr_cpy32(dst,pos,nla,size,nla_len(nla) as usize,nla_len(nla) as usize) }

unsafe fn xfrm_xlate32(dst:*mut nlmsghdr,src:*const nlmsghdr,attrs:*mut *mut nlattr,size:usize,type_:u8,maxtype:c_int,extack:*mut netlink_ext_ack)->c_int { memcpy(dst,src,NLMSG_HDRLEN);(*dst).nlmsg_len=NLMSG_HDRLEN as u32+xfrm_msg_min[type_ as usize] as u32;memset(nlmsg_data(dst),0,xfrm_msg_min[type_ as usize]); match (*src).nlmsg_type { XFRM_MSG_DELSA|XFRM_MSG_GETSA|XFRM_MSG_DELPOLICY|XFRM_MSG_GETPOLICY|XFRM_MSG_FLUSHSA|XFRM_MSG_FLUSHPOLICY|XFRM_MSG_NEWAE|XFRM_MSG_GETAE|XFRM_MSG_REPORT|XFRM_MSG_MIGRATE|XFRM_MSG_MIGRATE_STATE|XFRM_MSG_NEWSADINFO|XFRM_MSG_GETSADINFO|XFRM_MSG_NEWSPDINFO|XFRM_MSG_GETSPDINFO|XFRM_MSG_MAPPING|XFRM_MSG_NEWSA|XFRM_MSG_NEWPOLICY|XFRM_MSG_UPDSA|XFRM_MSG_UPDPOLICY=>memcpy(nlmsg_data(dst),nlmsg_data(src),compat_msg_min[type_ as usize] as usize), XFRM_MSG_EXPIRE=>{let s=nlmsg_data(src);let d=nlmsg_data(dst);memcpy(d,s,size_of::<compat_xfrm_usersa_info>());(*(d as *mut xfrm_user_expire)).hard=(*(s as *const compat_xfrm_user_expire)).hard;}, XFRM_MSG_ACQUIRE|XFRM_MSG_POLEXPIRE|XFRM_MSG_ALLOCSPI=>memcpy(nlmsg_data(dst),nlmsg_data(src),compat_msg_min[type_ as usize] as usize), _=>{NL_SET_ERR_MSG!(extack,"Unsupported message type");return -EOPNOTSUPP;} } let mut pos=(*dst).nlmsg_len as usize; for i in 1..=XFRMA_MAX {if (*attrs.add(i)).is_null()||i==XFRMA_PAD as usize{continue;}let e=if maxtype!=0{xfrm_attr_cpy32(dst as *mut _,&mut pos, *attrs.add(i),size,nla_len(*attrs.add(i)) as usize,nla_len(*attrs.add(i)) as usize)}else{xfrm_xlate32_attr(dst as *mut _,*attrs.add(i),&mut pos,size,extack)};if e!=0{return e;}}0 }

unsafe fn xfrm_user_rcv_msg_compat(h32:*const nlmsghdr,maxtype:c_int,policy:*const nla_policy,extack:*mut netlink_ext_ack)->*mut nlmsghdr { let type_=(*h32).nlmsg_type-XFRM_MSG_BASE; if type_ as usize>=xfrm_msg_min.len(){return ERR_PTR(-EINVAL);} if ((*h32).nlmsg_type==XFRM_MSG_GETSA||(*h32).nlmsg_type==XFRM_MSG_GETPOLICY)&&((*h32).nlmsg_flags&NLM_F_DUMP)!=0{return std::ptr::null_mut();} let mut attrs=[std::ptr::null_mut();XFRMA_MAX+1];let err=nlmsg_parse_deprecated(h32,compat_msg_min[type_ as usize],attrs.as_mut_ptr(),if maxtype!=0{maxtype}else{XFRMA_MAX as c_int},if !policy.is_null(){policy}else{compat_policy.as_ptr()},extack);if err<0{return ERR_PTR(err);}let len=xfrm_user_rcv_calculate_len64(h32,attrs.as_mut_ptr(),maxtype);if len==nlmsg_len(h32){return std::ptr::null_mut();}let h64=kvmalloc(len+NLMSG_HDRLEN,GFP_KERNEL) as *mut nlmsghdr;if h64.is_null(){return ERR_PTR(-ENOMEM);}if xfrm_xlate32(h64,h32,attrs.as_mut_ptr(),len+NLMSG_HDRLEN,type_ as u8,maxtype,extack)<0{kvfree(h64 as *mut c_void);return ERR_PTR(-EOPNOTSUPP);}h64 }

unsafe fn xfrm_user_policy_compat(pdata32:*mut *mut u8,optlen:c_int)->c_int { if optlen<size_of::<compat_xfrm_userpolicy_info>() as c_int{return -EINVAL;}let data64=kmalloc_track_caller(optlen as usize+4,GFP_USER|__GFP_NOWARN);if data64.is_null(){return -ENOMEM;}memcpy(data64,*pdata32 as *const _,size_of::<compat_xfrm_userpolicy_info>());memset((data64 as *mut u8).add(size_of::<compat_xfrm_userpolicy_info>()),0,4);memcpy((data64 as *mut u8).add(size_of::<compat_xfrm_userpolicy_info>()+4),(*pdata32).add(size_of::<compat_xfrm_userpolicy_info>()),optlen as usize-size_of::<compat_xfrm_userpolicy_info>());kfree(*pdata32 as *mut c_void);*pdata32=data64 as *mut u8;0 }

static mut xfrm_translator: xfrm_translator = xfrm_translator { owner: THIS_MODULE, alloc_compat: Some(xfrm_alloc_compat), rcv_msg_compat: Some(xfrm_user_rcv_msg_compat), xlate_user_policy_sockptr: Some(xfrm_user_policy_compat) };
unsafe extern "C" fn xfrm_compat_init() -> c_int { xfrm_register_translator(&raw mut xfrm_translator) }
unsafe extern "C" fn xfrm_compat_exit() { xfrm_unregister_translator(&raw mut xfrm_translator); }
module_init!(xfrm_compat_init);
module_exit!(xfrm_compat_exit);
MODULE_LICENSE!("GPL");
MODULE_AUTHOR!("Dmitry Safonov");
MODULE_DESCRIPTION!("XFRM 32-bit compatibility layer");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
