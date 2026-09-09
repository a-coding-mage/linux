// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2003-2013 Jozsef Kadlecsik <kadlec@netfilter.org> */
/* Kernel module implementing an IP set type: the hash:net,port type */

// Kernel headers and the generated ip_set_hash_gen.h implementation are supplied externally.

pub const IPSET_TYPE_REV_MIN: i32 = 0;
pub const IPSET_TYPE_REV_MAX: i32 = 8;

// Type-specific configuration: HTYPE=hash_netport, with proto, nets, and packed nets enabled.

#[repr(C)]
pub struct hash_netport4_elem {
    pub ip: u32,
    pub port: u16,
    pub proto: u8,
    pub cidr: u8,
    pub nomatch: u8,
}

pub unsafe fn hash_netport4_data_equal(
    ip1: *const hash_netport4_elem, ip2: *const hash_netport4_elem, _multi: *mut u32,
) -> bool {
    (*ip1).ip == (*ip2).ip && (*ip1).port == (*ip2).port && (*ip1).proto == (*ip2).proto && (*ip1).cidr == (*ip2).cidr
}

pub unsafe fn hash_netport4_do_data_match(elem: *const hash_netport4_elem) -> i32 {
    if (*elem).nomatch != 0 { -ENOTEMPTY } else { 1 }
}

pub unsafe fn hash_netport4_data_set_flags(elem: *mut hash_netport4_elem, flags: u32) {
    (*elem).nomatch = (((flags >> 16) & IPSET_FLAG_NOMATCH) != 0) as u8;
}

pub unsafe fn hash_netport4_data_reset_flags(elem: *mut hash_netport4_elem, flags: *mut u8) {
    core::mem::swap(&mut *flags, &mut (*elem).nomatch);
}

pub unsafe fn hash_netport4_data_netmask(elem: *mut hash_netport4_elem, cidr: u8) {
    (*elem).ip &= ip_set_netmask(cidr);
    (*elem).cidr = cidr.wrapping_sub(1);
}

pub unsafe fn hash_netport4_data_list(skb: *mut sk_buff, data: *const hash_netport4_elem) -> bool {
    let flags: u32 = if (*data).nomatch != 0 { IPSET_FLAG_NOMATCH } else { 0 };
    if nla_put_ipaddr4(skb, IPSET_ATTR_IP, (*data).ip) != 0 ||
       nla_put_net16(skb, IPSET_ATTR_PORT, (*data).port) != 0 ||
       nla_put_u8(skb, IPSET_ATTR_CIDR, (*data).cidr.wrapping_add(1)) != 0 ||
       nla_put_u8(skb, IPSET_ATTR_PROTO, (*data).proto) != 0 ||
       (flags != 0 && nla_put_net32(skb, IPSET_ATTR_CADT_FLAGS, htonl(flags)) != 0) { return true; }
    false
}

pub unsafe fn hash_netport4_data_next(next: *mut hash_netport4_elem, d: *const hash_netport4_elem) {
    (*next).ip = (*d).ip; (*next).port = (*d).port;
}

// #include "ip_set_hash_gen.h" with MTYPE=hash_netport4 and HOST_MASK=32.

pub unsafe fn hash_netport4_kadt(set: *mut ip_set, skb: *const sk_buff, par: *const xt_action_param, adt: ipset_adt, opt: *mut ip_set_adt_opt) -> i32 {
    let h = (*set).data as *const hash_netport4;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e = hash_netport4_elem { ip: 0, port: 0, proto: 0, cidr: INIT_CIDR((*h).rnets[0], 32), nomatch: 0 };
    let mut ext = IP_SET_INIT_KEXT(skb, opt, set);
    if adt == IPSET_TEST { e.cidr = 31; }
    if !ip_set_get_ip4_port(skb, (*opt).flags & IPSET_DIM_TWO_SRC, &mut e.port, &mut e.proto) { return -EINVAL; }
    ip4addrptr(skb, (*opt).flags & IPSET_DIM_ONE_SRC, &mut e.ip);
    e.ip &= ip_set_netmask(e.cidr.wrapping_add(1));
    adtfn(set, &mut e as *mut _, &mut ext, &mut (*opt).ext, (*opt).cmdflags)
}

pub unsafe fn hash_netport4_uadt(set: *mut ip_set, tb: *mut *mut nlattr, adt: ipset_adt, lineno: *mut u32, mut flags: u32, retried: bool) -> i32 {
    let h = (*set).data as *mut hash_netport4;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e = hash_netport4_elem { ip: 0, port: 0, proto: 0, cidr: 31, nomatch: 0 };
    let mut ext = IP_SET_INIT_UEXT(set); let (mut port, mut port_to, mut p, mut ip, mut ip_to, mut i) = (0u32,0u32,0u32,0u32,0u32,0u32); let mut with_ports=false; let mut cidr=0u8;
    if !(*tb.add(IPSET_ATTR_LINENO)).is_null() { *lineno = nla_get_u32(*tb.add(IPSET_ATTR_LINENO)); }
    if (*tb.add(IPSET_ATTR_IP)).is_null() || !ip_set_attr_netorder(tb, IPSET_ATTR_PORT) || !ip_set_optattr_netorder(tb, IPSET_ATTR_PORT_TO) || !ip_set_optattr_netorder(tb, IPSET_ATTR_CADT_FLAGS) { return -IPSET_ERR_PROTOCOL; }
    let mut ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP), &mut ip); if ret != 0 { return ret; }
    ret = ip_set_get_extensions(set, tb, &mut ext); if ret != 0 { return ret; }
    if !(*tb.add(IPSET_ATTR_CIDR)).is_null() { cidr=nla_get_u8(*tb.add(IPSET_ATTR_CIDR)); if cidr==0 || cidr>32 { return -IPSET_ERR_INVALID_CIDR; } e.cidr=cidr-1; }
    e.port=nla_get_be16(*tb.add(IPSET_ATTR_PORT));
    if !(*tb.add(IPSET_ATTR_PROTO)).is_null() { e.proto=nla_get_u8(*tb.add(IPSET_ATTR_PROTO)); with_ports=ip_set_proto_with_ports(e.proto); if e.proto==0{return -IPSET_ERR_INVALID_PROTO;} } else { return -IPSET_ERR_MISSING_PROTO; }
    if !(with_ports || e.proto==IPPROTO_ICMP) { e.port=0; }
    with_ports = with_ports && !(*tb.add(IPSET_ATTR_PORT_TO)).is_null();
    if !(*tb.add(IPSET_ATTR_CADT_FLAGS)).is_null() && (ip_set_get_h32(*tb.add(IPSET_ATTR_CADT_FLAGS)) & IPSET_FLAG_NOMATCH)!=0 { flags |= IPSET_FLAG_NOMATCH << 16; }
    if adt==IPSET_TEST || !(with_ports || !(*tb.add(IPSET_ATTR_IP_TO)).is_null()) { e.ip=htonl(ip & ip_set_hostmask(e.cidr+1)); ret=adtfn(set,&mut e,&mut ext,&mut ext,flags); return if ip_set_enomatch(ret,flags,adt,set){-ret}else if ip_set_eexist(ret,flags){0}else{ret}; }
    port=ntohs(e.port) as u32; port_to=port;
    if !(*tb.add(IPSET_ATTR_PORT_TO)).is_null() { port_to=ip_set_get_h16(*tb.add(IPSET_ATTR_PORT_TO)); if port_to<port { core::mem::swap(&mut port,&mut port_to); } }
    if !(*tb.add(IPSET_ATTR_IP_TO)).is_null() { ret=ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP_TO),&mut ip_to); if ret!=0{return ret;} if ip_to<ip{core::mem::swap(&mut ip,&mut ip_to);} if ip.wrapping_add(u32::MAX)==ip_to{return -IPSET_ERR_HASH_RANGE;} } else { ip_set_mask_from_to(ip,&mut ip_to,e.cidr+1); }
    if retried { ip=ntohl((*h).next.ip); p=ntohs((*h).next.port) as u32; } else { p=port; }
    loop { e.ip=htonl(ip); ip=ip_set_range_to_cidr(ip,ip_to,&mut cidr); e.cidr=cidr-1; while p<=port_to { e.port=htons(p as u16); i+=1; if i>IPSET_MAX_RANGE { hash_netport4_data_next(&mut (*h).next,&e); return -ERANGE; } ret=adtfn(set,&mut e,&mut ext,&mut ext,flags); if ret!=0 && !ip_set_eexist(ret,flags){return ret;} ret=0; p+=1; } p=port; if ip>ip_to {break;} }
    ret
}

#[repr(C)] pub struct hash_netport6_elem { pub ip: nf_inet_addr, pub port:u16, pub proto:u8, pub cidr:u8, pub nomatch:u8 }
pub unsafe fn hash_netport6_data_equal(a:*const hash_netport6_elem,b:*const hash_netport6_elem,_:*mut u32)->bool { ipv6_addr_equal(&(*a).ip.in6,&(*b).ip.in6)&&(*a).port==(*b).port&&(*a).proto==(*b).proto&&(*a).cidr==(*b).cidr }
pub unsafe fn hash_netport6_do_data_match(e:*const hash_netport6_elem)->i32 { if (*e).nomatch!=0{-ENOTEMPTY}else{1} }
pub unsafe fn hash_netport6_data_set_flags(e:*mut hash_netport6_elem,f:u32){(*e).nomatch=(((f>>16)&IPSET_FLAG_NOMATCH)!=0)as u8;}
pub unsafe fn hash_netport6_data_reset_flags(e:*mut hash_netport6_elem,f:*mut u8){core::mem::swap(&mut *f,&mut (*e).nomatch);}
pub unsafe fn hash_netport6_data_netmask(e:*mut hash_netport6_elem,c:u8){ip6_netmask(&mut (*e).ip,c);(*e).cidr=c-1;}
pub unsafe fn hash_netport6_data_list(skb:*mut sk_buff,d:*const hash_netport6_elem)->bool{let f=if(*d).nomatch!=0{IPSET_FLAG_NOMATCH}else{0};if nla_put_ipaddr6(skb,IPSET_ATTR_IP,&(*d).ip.in6)!=0||nla_put_net16(skb,IPSET_ATTR_PORT,(*d).port)!=0||nla_put_u8(skb,IPSET_ATTR_CIDR,(*d).cidr+1)!=0||nla_put_u8(skb,IPSET_ATTR_PROTO,(*d).proto)!=0||(f!=0&&nla_put_net32(skb,IPSET_ATTR_CADT_FLAGS,htonl(f))!=0){return true}false}
pub unsafe fn hash_netport6_data_next(n:*mut hash_netport6_elem,d:*const hash_netport6_elem){(*n).port=(*d).port;}
// #include "ip_set_hash_gen.h" with MTYPE=hash_netport6, HOST_MASK=128, IP_SET_EMIT_CREATE.

pub unsafe fn hash_netport6_kadt(set:*mut ip_set,skb:*const sk_buff,_par:*const xt_action_param,adt:ipset_adt,opt:*mut ip_set_adt_opt)->i32{let h=(*set).data as *const hash_netport6;let f=(*(*set).variant).adt[adt as usize];let mut e=hash_netport6_elem{ip:nf_inet_addr::default(),port:0,proto:0,cidr:INIT_CIDR((*h).rnets[0],128),nomatch:0};let mut x=IP_SET_INIT_KEXT(skb,opt,set);if adt==IPSET_TEST{e.cidr=127;}if !ip_set_get_ip6_port(skb,(*opt).flags&IPSET_DIM_TWO_SRC,&mut e.port,&mut e.proto){return -EINVAL;}ip6addrptr(skb,(*opt).flags&IPSET_DIM_ONE_SRC,&mut e.ip.in6);ip6_netmask(&mut e.ip,e.cidr+1);f(set,&mut e,&mut x,&mut (*opt).ext,(*opt).cmdflags)}

pub unsafe fn hash_netport6_uadt(set:*mut ip_set,tb:*mut *mut nlattr,adt:ipset_adt,lineno:*mut u32,mut flags:u32,retried:bool)->i32 {
    let h=(*set).data as *const hash_netport6; let f=(*(*set).variant).adt[adt as usize]; let mut e=hash_netport6_elem{ip:nf_inet_addr::default(),port:0,proto:0,cidr:127,nomatch:0}; let mut ext=IP_SET_INIT_UEXT(set); let(mut port,mut port_to)=(0u32,0u32); let mut with_ports=false; let mut cidr=0u8;
    if !(*tb.add(IPSET_ATTR_LINENO)).is_null(){*lineno=nla_get_u32(*tb.add(IPSET_ATTR_LINENO));}
    if (*tb.add(IPSET_ATTR_IP)).is_null()||!ip_set_attr_netorder(tb,IPSET_ATTR_PORT)||!ip_set_optattr_netorder(tb,IPSET_ATTR_PORT_TO)||!ip_set_optattr_netorder(tb,IPSET_ATTR_CADT_FLAGS){return -IPSET_ERR_PROTOCOL;}
    if !(*tb.add(IPSET_ATTR_IP_TO)).is_null(){return -IPSET_ERR_HASH_RANGE_UNSUPPORTED;}
    let mut ret=ip_set_get_ipaddr6(*tb.add(IPSET_ATTR_IP),&mut e.ip);if ret!=0{return ret;}ret=ip_set_get_extensions(set,tb,&mut ext);if ret!=0{return ret;}
    if !(*tb.add(IPSET_ATTR_CIDR)).is_null(){cidr=nla_get_u8(*tb.add(IPSET_ATTR_CIDR));if cidr==0||cidr>128{return -IPSET_ERR_INVALID_CIDR;}e.cidr=cidr-1;}ip6_netmask(&mut e.ip,e.cidr+1);e.port=nla_get_be16(*tb.add(IPSET_ATTR_PORT));
    if !(*tb.add(IPSET_ATTR_PROTO)).is_null(){e.proto=nla_get_u8(*tb.add(IPSET_ATTR_PROTO));with_ports=ip_set_proto_with_ports(e.proto);if e.proto==0{return -IPSET_ERR_INVALID_PROTO;}}else{return -IPSET_ERR_MISSING_PROTO;}if !(with_ports||e.proto==IPPROTO_ICMPV6){e.port=0;}
    if !(*tb.add(IPSET_ATTR_CADT_FLAGS)).is_null()&&(ip_set_get_h32(*tb.add(IPSET_ATTR_CADT_FLAGS))&IPSET_FLAG_NOMATCH)!=0{flags|=IPSET_FLAG_NOMATCH<<16;}
    if adt==IPSET_TEST||!with_ports||(*tb.add(IPSET_ATTR_PORT_TO)).is_null(){ret=f(set,&mut e,&mut ext,&mut ext,flags);return if ip_set_enomatch(ret,flags,adt,set){-ret}else if ip_set_eexist(ret,flags){0}else{ret};}
    port=ntohs(e.port)as u32;port_to=ip_set_get_h16(*tb.add(IPSET_ATTR_PORT_TO));if port>port_to{core::mem::swap(&mut port,&mut port_to);}if retried{port=ntohs((*h).next.port)as u32;}while port<=port_to{e.port=htons(port as u16);ret=f(set,&mut e,&mut ext,&mut ext,flags);if ret!=0&&!ip_set_eexist(ret,flags){return ret;}ret=0;port+=1;}ret
}

#[repr(C)] pub struct hash_netport_type_layout { pub name:*const u8, pub protocol:u8, pub features:u32, pub dimension:u8, pub family:u8, pub revision_min:u8, pub revision_max:u8, pub create_flags:[u32;9], pub create:Option<unsafe extern "C" fn()>, pub me:*mut core::ffi::c_void }
// The policy arrays and generated create callback are supplied by the kernel ABI; field values mirror the C registration.
extern "C" { static mut hash_netport_type: ip_set_type; }
pub unsafe fn hash_netport_init()->i32 { ip_set_type_register(&mut hash_netport_type) }
pub unsafe fn hash_netport_fini(){rcu_barrier();ip_set_type_unregister(&mut hash_netport_type);}
extern "C" { static mut hash_netport_type: ip_set_type; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
