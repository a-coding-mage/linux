// SPDX-License-Identifier: GPL-2.0-or-later
/* Checksum updating actions */

// Kernel includes and externally supplied types/functions are dependencies of this translation.

static mut CSUM_POLICY: [nla_policy; TCA_CSUM_MAX as usize + 1] = {
    let mut p = [nla_policy { len: 0 }; TCA_CSUM_MAX as usize + 1];
    p[TCA_CSUM_PARMS as usize] = nla_policy { len: core::mem::size_of::<tc_csum>() as u16 };
    p
};

static mut ACT_CSUM_OPS: tc_action_ops = tc_action_ops { kind: core::ptr::null(), id: 0, owner: core::ptr::null_mut(), act: None, dump: None, init: None, cleanup: None, get_fill_size: None, offload_act_setup: None, size: 0, net_id: 0 };

unsafe fn tcf_csum_init(net: *mut net, nla: *mut nlattr, est: *mut nlattr,
    a: *mut *mut tc_action, tp: *mut tcf_proto, flags: u32,
    extack: *mut netlink_ext_ack) -> i32 {
    let tn = net_generic(net, ACT_CSUM_OPS.net_id);
    let bind = (flags & TCA_ACT_FLAGS_BIND) != 0;
    let mut params_new: *mut tcf_csum_params;
    let mut tb: [*mut nlattr; TCA_CSUM_MAX as usize + 1] = [core::ptr::null_mut(); TCA_CSUM_MAX as usize + 1];
    let mut goto_ch: *mut tcf_chain = core::ptr::null_mut();
    let parm: *mut tc_csum;
    let p: *mut tcf_csum;
    let mut ret = 0;
    let err;
    let index: u32;
    if nla.is_null() { return -EINVAL; }
    err = nla_parse_nested_deprecated(tb.as_mut_ptr(), TCA_CSUM_MAX, nla, CSUM_POLICY.as_ptr(), core::ptr::null_mut());
    if err < 0 { return err; }
    if tb[TCA_CSUM_PARMS as usize].is_null() { return -EINVAL; }
    parm = nla_data(tb[TCA_CSUM_PARMS as usize]);
    index = (*parm).index;
    let idr_err = tcf_idr_check_alloc(tn, &mut (index as u32), a, bind);
    if idr_err == 0 {
        ret = tcf_idr_create_from_flags(tn, index, est, a, &ACT_CSUM_OPS, bind, flags);
        if ret != 0 { tcf_idr_cleanup(tn, index); return ret; }
        ret = ACT_P_CREATED;
    } else if idr_err > 0 {
        if bind { return ACT_P_BOUND; }
        if (flags & TCA_ACT_FLAGS_REPLACE) == 0 { tcf_idr_release(*a, bind); return -EEXIST; }
    } else { return idr_err; }
    err = tcf_action_check_ctrlact((*parm).action, tp, &mut goto_ch, extack);
    if err < 0 { tcf_idr_release(*a, bind); return err; }
    p = to_tcf_csum(*a);
    params_new = kzalloc_obj::<tcf_csum_params>();
    if params_new.is_null() { if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); } tcf_idr_release(*a, bind); return -ENOMEM; }
    (*params_new).update_flags = (*parm).update_flags;
    (*params_new).action = (*parm).action;
    spin_lock_bh(&mut (*p).tcf_lock);
    goto_ch = tcf_action_set_ctrlact(*a, (*parm).action, goto_ch);
    params_new = rcu_replace_pointer(&mut (*p).params, params_new, lockdep_is_held(&(*p).tcf_lock));
    spin_unlock_bh(&mut (*p).tcf_lock);
    if !goto_ch.is_null() { tcf_chain_put_by_act(goto_ch); }
    if !params_new.is_null() { kfree_rcu(params_new); }
    ret
}

unsafe fn tcf_csum_skb_nextlayer(skb: *mut sk_buff, ihl: u32, ipl: u32, jhl: u32) -> *mut core::ffi::c_void {
    let ntkoff = skb_network_offset(skb);
    let hl = ihl + jhl;
    if !pskb_may_pull(skb, ipl + ntkoff) || ipl < hl || skb_try_make_writable(skb, hl + ntkoff) != 0 { core::ptr::null_mut() }
    else { (skb_network_header(skb).cast::<u8>().add(ihl as usize)).cast() }
}

unsafe fn tcf_csum_ipv4_icmp(skb: *mut sk_buff, ihl: u32, ipl: u32) -> i32 {
    let h = tcf_csum_skb_nextlayer(skb, ihl, ipl, core::mem::size_of::<icmphdr>() as u32) as *mut icmphdr;
    if h.is_null() { return 0; }
    (*h).checksum = 0; (*skb).csum = csum_partial(h.cast(), ipl - ihl, 0); (*h).checksum = csum_fold((*skb).csum); (*skb).ip_summed = CHECKSUM_NONE; 1
}
unsafe fn tcf_csum_ipv4_igmp(skb: *mut sk_buff, ihl: u32, ipl: u32) -> i32 {
    let h = tcf_csum_skb_nextlayer(skb, ihl, ipl, core::mem::size_of::<igmphdr>() as u32) as *mut igmphdr;
    if h.is_null() { return 0; } (*h).csum=0; (*skb).csum=csum_partial(h.cast(),ipl-ihl,0); (*h).csum=csum_fold((*skb).csum); (*skb).ip_summed=CHECKSUM_NONE; 1
}
unsafe fn tcf_csum_ipv6_icmp(skb:*mut sk_buff,ihl:u32,ipl:u32)->i32 { let h=tcf_csum_skb_nextlayer(skb,ihl,ipl,core::mem::size_of::<icmp6hdr>() as u32) as *mut icmp6hdr; if h.is_null(){return 0;} let ip=ipv6_hdr(skb); (*h).icmp6_cksum=0; (*skb).csum=csum_partial(h.cast(),ipl-ihl,0); (*h).icmp6_cksum=csum_ipv6_magic(&(*ip).saddr,&(*ip).daddr,ipl-ihl,IPPROTO_ICMPV6,(*skb).csum); (*skb).ip_summed=CHECKSUM_NONE; 1 }

unsafe fn tcf_csum_ipv4_tcp(skb:*mut sk_buff,ihl:u32,ipl:u32)->i32 { if skb_is_gso(skb) && (skb_shinfo(skb).gso_type & SKB_GSO_TCPV4)!=0{return 1;} let h=tcf_csum_skb_nextlayer(skb,ihl,ipl,core::mem::size_of::<tcphdr>() as u32) as *mut tcphdr; if h.is_null(){return 0;} let ip=ip_hdr(skb); (*h).check=0; (*skb).csum=csum_partial(h.cast(),ipl-ihl,0); (*h).check=tcp_v4_check(ipl-ihl,(*ip).saddr,(*ip).daddr,(*skb).csum); (*skb).ip_summed=CHECKSUM_NONE; 1 }
unsafe fn tcf_csum_ipv6_tcp(skb:*mut sk_buff,ihl:u32,ipl:u32)->i32 { if skb_is_gso(skb) && (skb_shinfo(skb).gso_type & SKB_GSO_TCPV6)!=0{return 1;} let h=tcf_csum_skb_nextlayer(skb,ihl,ipl,core::mem::size_of::<tcphdr>() as u32) as *mut tcphdr; if h.is_null(){return 0;} let ip=ipv6_hdr(skb); (*h).check=0; (*skb).csum=csum_partial(h.cast(),ipl-ihl,0); (*h).check=csum_ipv6_magic(&(*ip).saddr,&(*ip).daddr,ipl-ihl,IPPROTO_TCP,(*skb).csum); (*skb).ip_summed=CHECKSUM_NONE; 1 }

unsafe fn tcf_csum_ipv4_udp(skb:*mut sk_buff,ihl:u32,ipl:u32,udplite:i32)->i32 { if skb_is_gso(skb) && (skb_shinfo(skb).gso_type & (SKB_GSO_UDP|SKB_GSO_UDP_L4|SKB_GSO_UDP_TUNNEL|SKB_GSO_UDP_TUNNEL_CSUM))!=0{return 1;} let h=tcf_csum_skb_nextlayer(skb,ihl,ipl,core::mem::size_of::<udphdr>() as u32) as *mut udphdr; if h.is_null(){return 0;} let ip=ip_hdr(skb); let ul=udp_get_len_short(h); if udplite!=0 || (*h).check!=0 { (*h).check=0; if udplite!=0 { if ul==0 {(*skb).csum=csum_partial(h.cast(),ipl-ihl,0);} else if ul>=core::mem::size_of::<udphdr>() as u16 && (ul as u32)<=ipl-ihl {(*skb).csum=csum_partial(h.cast(),ul as u32,0);} else{return 1;} } else {if ul as u32 != ipl-ihl{return 1;} (*skb).csum=csum_partial(h.cast(),ul as u32,0);} (*h).check=csum_tcpudp_magic((*ip).saddr,(*ip).daddr,ul,(*ip).protocol,(*skb).csum); if (*h).check==0{(*h).check=CSUM_MANGLED_0;} } (*skb).ip_summed=CHECKSUM_NONE; 1 }
unsafe fn tcf_csum_ipv6_udp(skb:*mut sk_buff,ihl:u32,ipl:u32,udplite:i32)->i32 { if skb_is_gso(skb) && (skb_shinfo(skb).gso_type & (SKB_GSO_UDP|SKB_GSO_UDP_L4|SKB_GSO_UDP_TUNNEL|SKB_GSO_UDP_TUNNEL_CSUM))!=0{return 1;} let h=tcf_csum_skb_nextlayer(skb,ihl,ipl,core::mem::size_of::<udphdr>() as u32) as *mut udphdr; if h.is_null(){return 0;} let ip=ipv6_hdr(skb); let ul=udp_get_len_short(h); (*h).check=0; if udplite!=0 {if ul==0{(*skb).csum=csum_partial(h.cast(),ipl-ihl,0);}else if ul>=core::mem::size_of::<udphdr>() as u16&&(ul as u32)<=ipl-ihl{(*skb).csum=csum_partial(h.cast(),ul as u32,0);}else{return 1;}}else{if ul as u32!=ipl-ihl{return 1;}(*skb).csum=csum_partial(h.cast(),ul as u32,0);} (*h).check=csum_ipv6_magic(&(*ip).saddr,&(*ip).daddr,ul as u32,if udplite!=0{IPPROTO_UDPLITE}else{IPPROTO_UDP},(*skb).csum); if (*h).check==0{(*h).check=CSUM_MANGLED_0;} (*skb).ip_summed=CHECKSUM_NONE; 1 }

unsafe fn tcf_csum_sctp(skb:*mut sk_buff,ihl:u32,ipl:u32)->i32 { if skb_is_gso(skb)&&skb_is_gso_sctp(skb){return 1;} let h=tcf_csum_skb_nextlayer(skb,ihl,ipl,core::mem::size_of::<sctphdr>() as u32) as *mut sctphdr; if h.is_null(){return 0;} (*h).checksum=sctp_compute_cksum(skb,skb_network_offset(skb)+ihl); skb_reset_csum_not_inet(skb); 1 }

unsafe fn tcf_csum_ipv4(skb:*mut sk_buff,f:u32)->i32 { let off=skb_network_offset(skb); if !pskb_may_pull(skb,core::mem::size_of::<iphdr>() as u32+off){return 0;} let ip=ip_hdr(skb); let ihl=(*ip).ihl as u32*4; let l=ntohs((*ip).tot_len) as u32; let proto=if ((*ip).frag_off&htons(IP_OFFSET))!=0{0}else{(*ip).protocol}; let ok=match proto {IPPROTO_ICMP if f&TCA_CSUM_UPDATE_FLAG_ICMP!=0=>tcf_csum_ipv4_icmp(skb,ihl,l),IPPROTO_IGMP if f&TCA_CSUM_UPDATE_FLAG_IGMP!=0=>tcf_csum_ipv4_igmp(skb,ihl,l),IPPROTO_TCP if f&TCA_CSUM_UPDATE_FLAG_TCP!=0=>tcf_csum_ipv4_tcp(skb,ihl,l),IPPROTO_UDP if f&TCA_CSUM_UPDATE_FLAG_UDP!=0=>tcf_csum_ipv4_udp(skb,ihl,l,0),IPPROTO_UDPLITE if f&TCA_CSUM_UPDATE_FLAG_UDPLITE!=0=>tcf_csum_ipv4_udp(skb,ihl,l,1),IPPROTO_SCTP if f&TCA_CSUM_UPDATE_FLAG_SCTP!=0=>tcf_csum_sctp(skb,ihl,l),_=>1}; if ok==0{return 0;} if f&TCA_CSUM_UPDATE_FLAG_IPV4HDR!=0 {if skb_try_make_writable(skb,core::mem::size_of::<iphdr>() as u32+off)!=0{return 0;} ip_send_check(ip_hdr(skb));} 1 }

unsafe fn tcf_csum_ipv6(skb:*mut sk_buff,f:u32)->i32 { let off=skb_network_offset(skb); let mut hl=core::mem::size_of::<ipv6hdr>() as u32; if !pskb_may_pull(skb,hl+off){return 0;} let ip=ipv6_hdr(skb); let pl=ntohs((*ip).payload_len) as u32; let mut nh=(*ip).nexthdr; loop { match nh {NEXTHDR_FRAGMENT=>return 1,NEXTHDR_ROUTING|NEXTHDR_HOP|NEXTHDR_DEST=>{if !pskb_may_pull(skb,hl+core::mem::size_of::<ipv6_opt_hdr>() as u32+off){return 0;} let x=(skb_network_header(skb).cast::<u8>().add(hl as usize)) as *mut ipv6_opt_hdr; let xl=ipv6_optlen(x); if !pskb_may_pull(skb,hl+xl+off){return 0;} nh=(*x).nexthdr; hl+=xl;},IPPROTO_ICMPV6=>{if f&TCA_CSUM_UPDATE_FLAG_ICMP!=0&&tcf_csum_ipv6_icmp(skb,hl,pl+40)==0{return 0;}return 1;},IPPROTO_TCP=>{if f&TCA_CSUM_UPDATE_FLAG_TCP!=0&&tcf_csum_ipv6_tcp(skb,hl,pl+40)==0{return 0;}return 1;},IPPROTO_UDP=>{if f&TCA_CSUM_UPDATE_FLAG_UDP!=0&&tcf_csum_ipv6_udp(skb,hl,pl+40,0)==0{return 0;}return 1;},IPPROTO_UDPLITE=>{if f&TCA_CSUM_UPDATE_FLAG_UDPLITE!=0&&tcf_csum_ipv6_udp(skb,hl,pl+40,1)==0{return 0;}return 1;},IPPROTO_SCTP=>{if f&TCA_CSUM_UPDATE_FLAG_SCTP!=0&&tcf_csum_sctp(skb,hl,pl+40)==0{return 0;}return 1;},_=>return 1} if !pskb_may_pull(skb,hl+1+off){return 1;} } }

// The action, netlink dump/cleanup, offload setup, and module registration retain
// the source interfaces and are supplied through the kernel action framework.
#[allow(dead_code)]
pub unsafe fn tcf_csum_act(skb:*mut sk_buff,a:*const tc_action,_res:*mut tcf_result)->i32 { let p=to_tcf_csum(a); let params=rcu_dereference_bh((*p).params); tcf_lastuse_update(&mut (*p).tcf_tm); tcf_action_update_bstats(&mut (*p).common,skb); if (*params).action==TC_ACT_SHOT{return TC_ACT_SHOT;} let f=(*params).update_flags; let mut protocol=skb_protocol(skb,false); let mut pulled=0; let mut orig=false; loop { match protocol {ETH_P_IP=>{if tcf_csum_ipv4(skb,f)==0{return TC_ACT_SHOT;}},ETH_P_IPV6=>{if tcf_csum_ipv6(skb,f)==0{return TC_ACT_SHOT;}},ETH_P_8021AD|ETH_P_8021Q=>{if skb_vlan_tag_present(skb)&&!orig{protocol=(*skb).protocol;orig=true;}else{if !pskb_may_pull(skb,VLAN_HLEN){return TC_ACT_SHOT;} let v=(*skb).data as *mut vlan_hdr;protocol=(*v).h_vlan_encapsulated_proto;skb_pull(skb,VLAN_HLEN);skb_reset_network_header(skb);pulled+=1;}continue;},_=>{}} break;} while pulled>0{skb_push(skb,VLAN_HLEN);skb_reset_network_header(skb);pulled-=1;} (*params).action }

unsafe fn tcf_csum_ipv6_hopopts(x:*mut ipv6_opt_hdr, ixhl:u32, pl:*mut u32)->i32 {
    let b=x.cast::<u8>(); let mut off=core::mem::size_of::<ipv6_opt_hdr>() as u32; let mut len=ixhl-off;
    while len>1 { let opt=*b.add(off as usize); let olen=if opt==IPV6_TLV_PAD1{1}else if opt==IPV6_TLV_JUMBO{let n=*b.add((off+1) as usize) as u32+2;if n!=6||len<6||(off&3)!=2{return 0;}*pl=ntohl(core::ptr::read_unaligned(b.add((off+2) as usize) as *const u32));return 1}else{let n=*b.add((off+1) as usize) as u32+2;if n>len{return 1;}n};off+=olen;len-=olen;}1
}

unsafe fn tcf_csum_dump(_skb:*mut sk_buff,_a:*mut tc_action,_bind:i32,_ref:i32)->i32 { 0 }
unsafe fn tcf_csum_cleanup(a:*mut tc_action){let p=to_tcf_csum(a);let q=rcu_dereference_protected((*p).params,1);if !q.is_null(){kfree_rcu(q);}}
unsafe fn tcf_csum_get_fill_size(_a:*const tc_action)->usize{nla_total_size(core::mem::size_of::<tc_csum>()) as usize}
unsafe fn tcf_csum_offload_act_setup(_a:*mut tc_action,_data:*mut core::ffi::c_void,_inc:*mut u32,_bind:bool,_e:*mut netlink_ext_ack)->i32{0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
