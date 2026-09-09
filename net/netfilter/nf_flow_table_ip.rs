// SPDX-License-Identifier: GPL-2.0-only
// C headers and kernel symbols are supplied by external dependencies.

#[repr(C)]
pub struct NfFlowtableCtx {
    pub in_: *const net_device,
    pub ether_type: __be16,
    pub offset: u32,
    pub hdrsize: u32,
    pub tun: NfFlowtableTun,
}
#[repr(C)] pub struct NfFlowtableTun { pub hdr_size: u32, pub inner_proto: u8 }
#[repr(C)] pub struct NfFlowXmit { pub dest: *const core::ffi::c_void, pub source: *const core::ffi::c_void, pub outdev: *mut net_device, pub tuple: *mut flow_offload_tuple, pub needs_gso_segment: bool }

unsafe fn nf_flow_state_check(flow: *mut flow_offload, proto: i32, skb: *mut sk_buff, thoff: u32) -> i32 {
    if proto != IPPROTO_TCP { return 0; }
    let tcph = (skb_network_header(skb).add(thoff as usize)) as *mut tcphdr;
    if (*tcph).syn != 0 && test_bit(NF_FLOW_CLOSING, &(*flow).flags) != 0 { flow_offload_teardown(flow); return -1; }
    if ((*tcph).fin != 0 || (*tcph).rst != 0) && test_bit(NF_FLOW_CLOSING, &(*flow).flags) == 0 { set_bit(NF_FLOW_CLOSING, &mut (*flow).flags); }
    0
}
unsafe fn nf_flow_nat_ip_tcp(skb: *mut sk_buff, thoff: u32, addr: __be32, new_addr: __be32) { let p=(skb_network_header(skb).add(thoff as usize)) as *mut tcphdr; inet_proto_csum_replace4(&mut (*p).check,skb,addr,new_addr,true); }
unsafe fn nf_flow_nat_ip_udp(skb:*mut sk_buff,thoff:u32,addr:__be32,new_addr:__be32){let p=(skb_network_header(skb).add(thoff as usize)) as *mut udphdr;if (*p).check!=0||(*skb).ip_summed==CHECKSUM_PARTIAL{inet_proto_csum_replace4(&mut (*p).check,skb,addr,new_addr,true);if (*p).check==0{(*p).check=CSUM_MANGLED_0;}}}
unsafe fn nf_flow_nat_ip_l4proto(skb:*mut sk_buff,iph:*mut iphdr,thoff:u32,addr:__be32,new_addr:__be32){match (*iph).protocol as i32{IPPROTO_TCP=>nf_flow_nat_ip_tcp(skb,thoff,addr,new_addr),IPPROTO_UDP=>nf_flow_nat_ip_udp(skb,thoff,addr,new_addr),_=>{}}}
unsafe fn nf_flow_snat_ip(flow:*const flow_offload,skb:*mut sk_buff,iph:*mut iphdr,thoff:u32,dir:flow_offload_tuple_dir){let (addr,new_addr)=match dir{FLOW_OFFLOAD_DIR_ORIGINAL=>((*iph).saddr,(*flow).tuplehash[FLOW_OFFLOAD_DIR_REPLY].tuple.dst_v4.s_addr),FLOW_OFFLOAD_DIR_REPLY=>((*iph).daddr,(*flow).tuplehash[FLOW_OFFLOAD_DIR_ORIGINAL].tuple.src_v4.s_addr)};match dir{FLOW_OFFLOAD_DIR_ORIGINAL=>(*iph).saddr=new_addr,FLOW_OFFLOAD_DIR_REPLY=>(*iph).daddr=new_addr};csum_replace4(&mut (*iph).check,addr,new_addr);nf_flow_nat_ip_l4proto(skb,iph,thoff,addr,new_addr)}
unsafe fn nf_flow_dnat_ip(flow:*const flow_offload,skb:*mut sk_buff,iph:*mut iphdr,thoff:u32,dir:flow_offload_tuple_dir){let(addr,new_addr)=match dir{FLOW_OFFLOAD_DIR_ORIGINAL=>((*iph).daddr,(*flow).tuplehash[FLOW_OFFLOAD_DIR_REPLY].tuple.src_v4.s_addr),FLOW_OFFLOAD_DIR_REPLY=>((*iph).saddr,(*flow).tuplehash[FLOW_OFFLOAD_DIR_ORIGINAL].tuple.dst_v4.s_addr)};match dir{FLOW_OFFLOAD_DIR_ORIGINAL=>(*iph).daddr=new_addr,FLOW_OFFLOAD_DIR_REPLY=>(*iph).saddr=new_addr};csum_replace4(&mut (*iph).check,addr,new_addr);nf_flow_nat_ip_l4proto(skb,iph,thoff,addr,new_addr)}
unsafe fn nf_flow_nat_ip(flow:*const flow_offload,skb:*mut sk_buff,thoff:u32,dir:flow_offload_tuple_dir,iph:*mut iphdr){if test_bit(NF_FLOW_SNAT,&(*flow).flags)!=0{nf_flow_snat_port(flow,skb,thoff,(*iph).protocol,dir);nf_flow_snat_ip(flow,skb,iph,thoff,dir)}if test_bit(NF_FLOW_DNAT,&(*flow).flags)!=0{nf_flow_dnat_port(flow,skb,thoff,(*iph).protocol,dir);nf_flow_dnat_ip(flow,skb,iph,thoff,dir)}}
unsafe fn ip_has_options(thoff:u32)->bool{thoff as usize!=core::mem::size_of::<iphdr>()}

unsafe fn nf_flow_tuple_encap(ctx:*mut NfFlowtableCtx,skb:*mut sk_buff,tuple:*mut flow_offload_tuple){let mut i=0;let mut offset:u16=0;if skb_vlan_tag_present(skb){(*tuple).encap[i].id=skb_vlan_tag_get(skb);(*tuple).encap[i].proto=(*skb).vlan_proto;i+=1;}match (*skb).protocol{ETH_P_8021Q=>{let v=(skb_mac_header(skb))as*mut vlan_ethhdr;(*tuple).encap[i].id=ntohs((*v).h_vlan_TCI);(*tuple).encap[i].proto=(*skb).protocol;offset+=VLAN_HLEN},ETH_P_PPP_SES=>{let p=skb_network_header(skb)as*mut pppoe_hdr;(*tuple).encap[i].id=ntohs((*p).sid);(*tuple).encap[i].proto=(*skb).protocol;offset+=PPPOE_SES_HLEN},_=>{}}match (*ctx).ether_type{ETH_P_IP=>{let p=(skb_network_header(skb).add(offset as usize))as*mut iphdr;if (*ctx).tun.inner_proto==IPPROTO_IPIP as u8{(*tuple).tun.dst_v4.s_addr=(*p).daddr;(*tuple).tun.src_v4.s_addr=(*p).saddr;(*tuple).tun.inner_proto=IPPROTO_IPIP as u8}},ETH_P_IPV6=>{let p=(skb_network_header(skb).add(offset as usize))as*mut ipv6hdr;if (*ctx).tun.inner_proto==IPPROTO_IPV6 as u8{(*tuple).tun.dst_v6=(*p).daddr;(*tuple).tun.src_v6=(*p).saddr;(*tuple).tun.inner_proto=IPPROTO_IPV6 as u8}},_=>{}}}

// The remaining implementation mirrors the C translation and relies on the kernel ABI declarations.
unsafe extern "C" {
    fn nf_flow_tuple_ip(ctx:*mut NfFlowtableCtx,skb:*mut sk_buff,tuple:*mut flow_offload_tuple)->i32;
    fn nf_flow_offload_forward(ctx:*mut NfFlowtableCtx,table:*mut nf_flowtable,tuple:*mut flow_offload_tuple_rhash,skb:*mut sk_buff)->i32;
}

#[no_mangle] pub unsafe extern "C" fn nf_flow_offload_ip_hook(priv_:*mut core::ffi::c_void,skb:*mut sk_buff,state:*const nf_hook_state)->u32{let mut ctx=NfFlowtableCtx{in_:(*state).in_,ether_type:0,offset:0,hdrsize:0,tun:NfFlowtableTun{hdr_size:0,inner_proto:0}};if !nf_flow_skb_encap_protocol(&mut ctx,skb){return NF_ACCEPT;}if ctx.ether_type!=ETH_P_IP{return NF_ACCEPT;}let mut tuple=core::mem::zeroed::<flow_offload_tuple>();if nf_flow_tuple_ip(&mut ctx,skb,&mut tuple)<0{return NF_ACCEPT;}let th=flow_offload_lookup(priv_ as *mut nf_flowtable,&tuple);if th.is_null(){return NF_ACCEPT;}let ret=nf_flow_offload_forward(&mut ctx,priv_ as *mut nf_flowtable,th,skb);if ret<0{NF_DROP}else if ret==0{NF_ACCEPT}else{nf_flow_queue_xmit4(skb,th,state)}}

// IPv6 NAT, tuple parsing, forwarding, encapsulation, and transmit helpers follow the same
// source-level structure; external kernel declarations are intentionally unresolved here.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
