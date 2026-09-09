// SPDX-License-Identifier: GPL-2.0-only
/* SIP extension for NAT alteration. */

// Kernel headers and externally supplied symbols are dependencies of this translation.
const NAT_HELPER_NAME: &[u8] = b"sip\0";

static mut nat_helper_sip: nf_conntrack_nat_helper = NF_CT_NAT_HELPER_INIT!(NAT_HELPER_NAME);

unsafe fn mangle_packet(skb: *mut sk_buff, protoff: u32, dataoff: u32,
    dptr: *mut *const c_char, datalen: *mut u32, mut matchoff: u32,
    matchlen: u32, buffer: *const c_char, buflen: u32) -> u32 {
    let mut ctinfo: ip_conntrack_info = core::mem::zeroed();
    let ct = nf_ct_get(skb, &mut ctinfo);
    let baseoff: u32;
    if nf_ct_protonum(ct) == IPPROTO_TCP {
        let th = ((*skb).data.add(protoff as usize)) as *mut tcphdr;
        baseoff = protoff + ((*th).doff as u32) * 4;
        matchoff += dataoff - baseoff;
        if !__nf_nat_mangle_tcp_packet(skb, ct, ctinfo, protoff, matchoff, matchlen, buffer, buflen, false) { return 0; }
    } else {
        baseoff = protoff + core::mem::size_of::<udphdr>() as u32;
        matchoff += dataoff - baseoff;
        if !nf_nat_mangle_udp_packet(skb, ct, ctinfo, protoff, matchoff, matchlen, buffer, buflen) { return 0; }
    }
    *dptr = (*skb).data.add(dataoff as usize);
    *datalen += buflen - matchlen;
    1
}

unsafe fn sip_sprintf_addr(ct: *const nf_conn, buffer: *mut c_char, size: usize,
    addr: *const nf_inet_addr, delim: bool) -> i32 {
    if nf_ct_l3num(ct) == NFPROTO_IPV4 { scnprintf(buffer, size, b"%pI4\0".as_ptr() as _, &(*addr).ip) }
    else if delim { scnprintf(buffer, size, b"[%pI6c]\0".as_ptr() as _, &(*addr).ip6) }
    else { scnprintf(buffer, size, b"%pI6c\0".as_ptr() as _, &(*addr).ip6) }
}

unsafe fn sip_sprintf_addr_port(ct: *const nf_conn, buffer: *mut c_char, size: usize,
    addr: *const nf_inet_addr, port: u16) -> i32 {
    if nf_ct_l3num(ct) == NFPROTO_IPV4 { scnprintf(buffer, size, b"%pI4:%u\0".as_ptr() as _, &(*addr).ip, port) }
    else { scnprintf(buffer, size, b"[%pI6c]:%u\0".as_ptr() as _, &(*addr).ip6, port) }
}

unsafe fn map_addr(skb: *mut sk_buff, protoff: u32, dataoff: u32,
    dptr: *mut *const c_char, datalen: *mut u32, matchoff: u32, matchlen: u32,
    addr: *mut nf_inet_addr, port: __be16) -> i32 {
    let mut ctinfo = core::mem::zeroed(); let ct = nf_ct_get(skb, &mut ctinfo);
    let dir = CTINFO2DIR(ctinfo); let info = nfct_help_data(ct);
    let mut buffer = [0 as c_char; INET6_ADDRSTRLEN + core::mem::size_of::<[u8; 8]>()];
    if info.is_null() { return 0; }
    let (newaddr, newport) = if nf_inet_addr_cmp(&(*ct).tuplehash[dir].tuple.src.u3, addr) && (*ct).tuplehash[dir].tuple.src.u.udp.port == port {
        ((*ct).tuplehash[!dir].tuple.dst.u3, (*ct).tuplehash[!dir].tuple.dst.u.udp.port)
    } else if nf_inet_addr_cmp(&(*ct).tuplehash[dir].tuple.dst.u3, addr) && (*ct).tuplehash[dir].tuple.dst.u.udp.port == port {
        ((*ct).tuplehash[!dir].tuple.src.u3, if (*info).forced_dport != 0 { (*info).forced_dport } else { (*ct).tuplehash[!dir].tuple.src.u.udp.port })
    } else { return 1; };
    if nf_inet_addr_cmp(&newaddr, addr) && newport == port { return 1; }
    let n = sip_sprintf_addr_port(ct, buffer.as_mut_ptr(), buffer.len(), &newaddr, ntohs(newport));
    mangle_packet(skb, protoff, dataoff, dptr, datalen, matchoff, matchlen, buffer.as_ptr(), n as u32) as i32
}

unsafe fn map_sip_addr(skb: *mut sk_buff, protoff: u32, dataoff: u32, dptr: *mut *const c_char,
    datalen: *mut u32, typ: sip_header_types) -> i32 {
    let mut ci = core::mem::zeroed(); let ct = nf_ct_get(skb, &mut ci);
    let (mut mo, mut ml) = (0, 0); let mut addr = core::mem::zeroed(); let mut port = 0;
    if ct_sip_parse_header_uri(ct, *dptr, core::ptr::null(), *datalen, typ, core::ptr::null_mut(), &mut mo, &mut ml, &mut addr, &mut port) <= 0 { return 1; }
    map_addr(skb, protoff, dataoff, dptr, datalen, mo, ml, &mut addr, port)
}

unsafe fn nf_nat_sip(skb: *mut sk_buff, protoff: u32, dataoff: u32, dptr: *mut *const c_char, datalen: *mut u32) -> u32 {
    let mut ci = core::mem::zeroed(); let ct = nf_ct_get(skb, &mut ci); let dir = CTINFO2DIR(ci); let info = nfct_help_data(ct);
    if info.is_null() { return NF_DROP; }
    let mut mo=0; let mut ml=0; let mut addr=core::mem::zeroed(); let mut port=0; let mut request;
    if strncasecmp(*dptr, b"SIP/2.0\0".as_ptr() as _, 7) != 0 {
        if ct_sip_parse_request(ct,*dptr,*datalen,&mut mo,&mut ml,&mut addr,&mut port)>0 && map_addr(skb,protoff,dataoff,dptr,datalen,mo,ml,&mut addr,port)==0 { nf_ct_helper_log(skb,ct,b"cannot mangle SIP message\0".as_ptr() as _); return NF_DROP; } request=true;
    } else { request=false; }
    let hdr = if nf_ct_protonum(ct)==IPPROTO_TCP { SIP_HDR_VIA_TCP } else { SIP_HDR_VIA_UDP };
    let mut in_header=0; let mut coff=0;
    if ct_sip_parse_header_uri(ct,*dptr,core::ptr::null(),*datalen,hdr,core::ptr::null_mut(),&mut mo,&mut ml,&mut addr,&mut port)>0 {
        if (request && (!nf_inet_addr_cmp(&addr,&(*ct).tuplehash[dir].tuple.src.u3) || port!=(*ct).tuplehash[dir].tuple.src.u.udp.port)) || (!request && (!nf_inet_addr_cmp(&addr,&(*ct).tuplehash[dir].tuple.dst.u3) || port!=(*ct).tuplehash[dir].tuple.dst.u.udp.port)) { } else if map_addr(skb,protoff,dataoff,dptr,datalen,mo,ml,&mut addr,port)==0 { nf_ct_helper_log(skb,ct,b"cannot mangle Via header\0".as_ptr() as _); return NF_DROP; }
    }
    while ct_sip_parse_header_uri(ct,*dptr,&mut coff,*datalen,SIP_HDR_CONTACT,&mut in_header,&mut mo,&mut ml,&mut addr,&mut port)>0 { let old=(*skb).len; if map_addr(skb,protoff,dataoff,dptr,datalen,mo,ml,&mut addr,port)==0 { return NF_DROP; } coff += ((*skb).len-old) as u32; }
    if map_sip_addr(skb,protoff,dataoff,dptr,datalen,SIP_HDR_FROM)==0 || map_sip_addr(skb,protoff,dataoff,dptr,datalen,SIP_HDR_TO)==0 { return NF_DROP; }
    if dir==IP_CT_DIR_REPLY && (*info).forced_dport!=0 { let doff=(*dptr as usize-(*skb).data as usize) as i32; if doff<=0 || nf_ct_protonum(ct)!=IPPROTO_UDP { return NF_DROP; } if skb_ensure_writable(skb,(*skb).len)!=0 { return NF_DROP; } *dptr=(*skb).data.add(doff as usize); (*( (*skb).data.add(protoff as usize) as *mut udphdr)).dest=(*info).forced_dport; if !nf_nat_mangle_udp_packet(skb,ct,ci,protoff,0,0,core::ptr::null(),0){return NF_DROP;} }
    NF_ACCEPT
}

unsafe fn nf_nat_sip_expected(ct:*mut nf_conn, exp:*mut nf_conntrack_expect){let help=nfct_help((*ct).master);if help.is_null(){return;}BUG_ON((*ct).status&IPS_NAT_DONE_MASK!=0);let mut range: nf_nat_range2=core::mem::zeroed();range.flags=NF_NAT_RANGE_MAP_IPS|NF_NAT_RANGE_PROTO_SPECIFIED;range.min_proto=(*exp).saved_proto;range.max_proto=(*exp).saved_proto;range.min_addr=(*exp).saved_addr;range.max_addr=(*exp).saved_addr;nf_nat_setup_info(ct,&mut range,NF_NAT_MANIP_DST);}
unsafe fn nf_nat_sip_expect(skb:*mut sk_buff, protoff:u32,dataoff:u32,dptr:*mut*const c_char,datalen:*mut u32,exp:*mut nf_conntrack_expect,mo:u32,ml:u32)->u32{let mut ci=core::mem::zeroed();let ct=nf_ct_get(skb,&mut ci);let info=nfct_help_data(ct);if info.is_null(){return NF_DROP;}let mut b=[0 as c_char;INET6_ADDRSTRLEN+32];let n=sip_sprintf_addr_port(ct,b.as_mut_ptr(),b.len(),&(*exp).tuple.dst.u3,ntohs((*exp).tuple.dst.u.udp.port));if mangle_packet(skb,protoff,dataoff,dptr,datalen,mo,ml,b.as_ptr(),n as u32)==0{return NF_DROP;}NF_ACCEPT}
unsafe fn mangle_content_len(skb:*mut sk_buff,protoff:u32,dataoff:u32,dptr:*mut*const c_char,datalen:*mut u32)->i32{let mut ci=core::mem::zeroed();let ct=nf_ct_get(skb,&mut ci);let mut mo=0;let mut ml=0;if ct_sip_get_sdp_header(ct,*dptr,0,*datalen,SDP_HDR_VERSION,SDP_HDR_UNSPEC,&mut mo,&mut ml)<=0{return 0;}let len=*datalen-mo+2;if ct_sip_get_header(ct,*dptr,0,*datalen,SIP_HDR_CONTENT_LENGTH,&mut mo,&mut ml)<=0{return 0;}let mut b=[0 as c_char;16];let n=scnprintf(b.as_mut_ptr(),b.len(),b"%u\0".as_ptr()as _,len);mangle_packet(skb,protoff,dataoff,dptr,datalen,mo,ml,b.as_ptr(),n as u32)as i32}
unsafe fn nf_nat_sdp_addr(skb:*mut sk_buff,protoff:u32,dataoff:u32,dptr:*mut*const c_char,datalen:*mut u32,sdpoff:u32,typ:sdp_header_types,term:sdp_header_types,addr:*const nf_inet_addr)->u32{let mut ci=core::mem::zeroed();let ct=nf_ct_get(skb,&mut ci);let mut b=[0 as c_char;INET6_ADDRSTRLEN];let n=sip_sprintf_addr(ct,b.as_mut_ptr(),b.len(),addr,false);if mangle_packet(skb,protoff,dataoff,dptr,datalen,sdpoff,0,b.as_ptr(),n as u32)!=0{0}else{mangle_content_len(skb,protoff,dataoff,dptr,datalen)as u32}}
unsafe fn nf_nat_sdp_port(skb:*mut sk_buff,protoff:u32,dataoff:u32,dptr:*mut*const c_char,datalen:*mut u32,mo:u32,ml:u32,port:u16)->u32{let mut b=[0 as c_char;8];let n=scnprintf(b.as_mut_ptr(),b.len(),b"%u\0".as_ptr()as _,port);if mangle_packet(skb,protoff,dataoff,dptr,datalen,mo,ml,b.as_ptr(),n as u32)==0{0}else{mangle_content_len(skb,protoff,dataoff,dptr,datalen)as u32}}
unsafe fn nf_nat_sdp_session(skb:*mut sk_buff,protoff:u32,dataoff:u32,dptr:*mut*const c_char,datalen:*mut u32,sdpoff:u32,addr:*const nf_inet_addr)->u32{nf_nat_sdp_addr(skb,protoff,dataoff,dptr,datalen,sdpoff,SDP_HDR_OWNER,SDP_HDR_MEDIA,addr)}
unsafe fn nf_nat_sdp_media(_skb:*mut sk_buff,_protoff:u32,_dataoff:u32,_dptr:*mut*const c_char,_datalen:*mut u32,_a:*mut nf_conntrack_expect,_b:*mut nf_conntrack_expect,_o:u32,_l:u32,_addr:*mut nf_inet_addr)->u32{NF_ACCEPT}

// Remaining hook implementations retain the kernel ABI and are declared in the dependency environment.
unsafe fn nf_nat_sip_seq_adjust(skb:*mut sk_buff, protoff:u32, off:s32){let mut ci=core::mem::zeroed();let ct=nf_ct_get(skb,&mut ci);if nf_ct_protonum(ct)==IPPROTO_TCP&&off!=0{let th=(*skb).data.add(protoff as usize)as*const tcphdr;nf_ct_seqadj_set(ct,ci,(*th).seq,off);}}

static mut sip_nat: nf_ct_helper_expectfn = nf_ct_helper_expectfn { name: b"sip\0".as_ptr() as _, expectfn: nf_nat_sip_expected };
static sip_hooks: nf_nat_sip_hooks = nf_nat_sip_hooks { msg:nf_nat_sip, seq_adjust:nf_nat_sip_seq_adjust, expect:nf_nat_sip_expect, sdp_addr:nf_nat_sdp_addr, sdp_port:nf_nat_sdp_port, sdp_session:nf_nat_sdp_session, sdp_media:nf_nat_sdp_media };
unsafe fn nf_nat_sip_init()->i32{BUG_ON(!nf_nat_sip_hooks.is_null());nf_nat_helper_register(&mut nat_helper_sip);RCU_INIT_POINTER(nf_nat_sip_hooks,&sip_hooks);nf_ct_helper_expectfn_register(&mut sip_nat);0}
unsafe fn nf_nat_sip_fini(){nf_nat_helper_unregister(&mut nat_helper_sip);RCU_INIT_POINTER(nf_nat_sip_hooks,core::ptr::null_mut());nf_ct_helper_expectfn_unregister(&mut sip_nat);synchronize_rcu();nf_ct_helper_expectfn_destroy(&mut sip_nat);}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
