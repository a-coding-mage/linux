// SPDX-License-Identifier: GPL-2.0-only
/* Copyright (C) 2011-2013 Jozsef Kadlecsik <kadlec@netfilter.org> */
/* Kernel module implementing an IP set type: the hash:net,iface type */

// Kernel and ipset dependencies supplied by the surrounding translation unit.

pub const IPSET_TYPE_REV_MIN: u32 = 0;
pub const IPSET_TYPE_REV_MAX: u32 = 8;
pub const IP_SET_HASH_WITH_NETS: bool = true;
pub const IP_SET_HASH_WITH_MULTI: bool = true;

#[repr(C)]
pub struct hash_netiface4_elem_hashed {
    pub ip: __be32,
    pub physdev: u8,
    pub cidr: u8,
    pub nomatch: u8,
    pub elem: u8,
}

#[repr(C)]
pub struct hash_netiface4_elem {
    pub ip: __be32,
    pub physdev: u8,
    pub cidr: u8,
    pub nomatch: u8,
    pub elem: u8,
    pub wildcard: u8,
    pub iface: [c_char; IFNAMSIZ],
}

pub unsafe fn hash_netiface4_data_equal(
    ip1: *const hash_netiface4_elem, ip2: *const hash_netiface4_elem, multi: *mut u32,
) -> bool {
    (*ip1).ip == (*ip2).ip && (*ip1).cidr == (*ip2).cidr && {
        *multi = (*multi).wrapping_add(1);
        true
    } && (*ip1).physdev == (*ip2).physdev && if (*ip1).wildcard != 0 {
        strncmp((*ip1).iface.as_ptr(), (*ip2).iface.as_ptr(), strlen((*ip1).iface.as_ptr())) == 0
    } else {
        strcmp((*ip1).iface.as_ptr(), (*ip2).iface.as_ptr()) == 0
    }
}

pub unsafe fn hash_netiface4_do_data_match(elem: *const hash_netiface4_elem) -> c_int {
    if (*elem).nomatch != 0 { -ENOTEMPTY } else { 1 }
}

pub unsafe fn hash_netiface4_data_set_flags(elem: *mut hash_netiface4_elem, flags: u32) {
    (*elem).nomatch = ((flags >> 16) & IPSET_FLAG_NOMATCH) as u8;
}

pub unsafe fn hash_netiface4_data_reset_flags(elem: *mut hash_netiface4_elem, flags: *mut u8) {
    core::mem::swap(&mut *flags, &mut (*elem).nomatch);
}

pub unsafe fn hash_netiface4_data_netmask(elem: *mut hash_netiface4_elem, cidr: u8) {
    (*elem).ip &= ip_set_netmask(cidr);
    (*elem).cidr = cidr;
}

pub unsafe fn hash_netiface4_data_list(skb: *mut sk_buff, data: *const hash_netiface4_elem) -> bool {
    let mut flags = if (*data).physdev != 0 { IPSET_FLAG_PHYSDEV } else { 0 } |
        if (*data).wildcard != 0 { IPSET_FLAG_IFACE_WILDCARD } else { 0 };
    if (*data).nomatch != 0 { flags |= IPSET_FLAG_NOMATCH; }
    if nla_put_ipaddr4(skb, IPSET_ATTR_IP, (*data).ip) != 0 ||
       nla_put_u8(skb, IPSET_ATTR_CIDR, (*data).cidr) != 0 ||
       nla_put_string(skb, IPSET_ATTR_IFACE, (*data).iface.as_ptr()) != 0 ||
       (flags != 0 && nla_put_net32(skb, IPSET_ATTR_CADT_FLAGS, htonl(flags)) != 0) { true } else { false }
}

pub unsafe fn hash_netiface4_data_next(next: *mut hash_netiface4_elem, d: *const hash_netiface4_elem) {
    (*next).ip = (*d).ip;
}

// The following generated implementation is provided by ip_set_hash_gen.h.

#[cfg(CONFIG_BRIDGE_NETFILTER)]
pub unsafe fn get_physindev_name(skb: *const sk_buff, net: *mut net) -> *const c_char {
    let dev = nf_bridge_get_physindev(skb, net);
    if dev.is_null() { core::ptr::null() } else { (*dev).name.as_ptr() }
}

#[cfg(CONFIG_BRIDGE_NETFILTER)]
pub unsafe fn get_physoutdev_name(skb: *const sk_buff) -> *const c_char {
    let dev = nf_bridge_get_physoutdev(skb);
    if dev.is_null() { core::ptr::null() } else { (*dev).name.as_ptr() }
}

pub unsafe fn hash_netiface4_kadt(set: *mut ip_set, skb: *const sk_buff, par: *const xt_action_param,
    adt: ipset_adt, opt: *mut ip_set_adt_opt) -> c_int {
    let h = (*set).data as *mut hash_netiface4;
    let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e: hash_netiface4_elem = core::mem::zeroed();
    e.cidr = INIT_CIDR((*h).rnets[0], 32); e.elem = 1;
    let mut ext = IP_SET_INIT_KEXT(skb, opt, set);
    if adt == IPSET_TEST { e.cidr = 32; }
    ip4addrptr(skb, (*opt).flags & IPSET_DIM_ONE_SRC, &mut e.ip);
    e.ip &= ip_set_netmask(e.cidr);
    if (*opt).cmdflags & IPSET_FLAG_PHYSDEV != 0 {
        #[cfg(CONFIG_BRIDGE_NETFILTER)] {
            let p = if (*opt).flags & IPSET_DIM_TWO_SRC != 0 { get_physindev_name(skb, xt_net(par)) } else { get_physoutdev_name(skb) };
            if p.is_null() { return -EINVAL; } STRSCPY(e.iface.as_mut_ptr(), p); e.physdev = 1;
        }
    } else { STRSCPY(e.iface.as_mut_ptr(), if (*opt).flags & IPSET_DIM_TWO_SRC != 0 { (*(*par).state).in_.as_ref().map_or(core::ptr::null(), |d| d.name.as_ptr()) } else { (*(*par).state).out.as_ref().map_or(core::ptr::null(), |d| d.name.as_ptr()) }); }
    if strlen(e.iface.as_ptr()) == 0 { return -EINVAL; }
    adtfn(set, &mut e as *mut _ as *mut c_void, &mut ext, &mut (*opt).ext, (*opt).cmdflags)
}

pub unsafe fn hash_netiface4_uadt(set: *mut ip_set, tb: *mut *mut nlattr, adt: ipset_adt,
    lineno: *mut u32, mut flags: u32, retried: bool) -> c_int {
    let h = (*set).data as *mut hash_netiface4; let adtfn = (*(*set).variant).adt[adt as usize];
    let mut e: hash_netiface4_elem = core::mem::zeroed(); e.cidr = 32; e.elem = 1;
    let mut ext = IP_SET_INIT_UEXT(set); let (mut ip, mut ip_to, mut i) = (0u32, 0u32, 0u32);
    if !(*tb.add(IPSET_ATTR_LINENO as usize)).is_null() { *lineno = nla_get_u32(*tb.add(IPSET_ATTR_LINENO as usize)); }
    if (*tb.add(IPSET_ATTR_IP as usize)).is_null() || (*tb.add(IPSET_ATTR_IFACE as usize)).is_null() || ip_set_optattr_netorder(tb, IPSET_ATTR_CADT_FLAGS) == 0 { return -IPSET_ERR_PROTOCOL; }
    let mut ret = ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP as usize), &mut ip); if ret != 0 { return ret; }
    ret = ip_set_get_extensions(set, tb, &mut ext); if ret != 0 { return ret; }
    if !(*tb.add(IPSET_ATTR_CIDR as usize)).is_null() { e.cidr = nla_get_u8(*tb.add(IPSET_ATTR_CIDR as usize)); if e.cidr > 32 { return -IPSET_ERR_INVALID_CIDR; } }
    nla_strscpy(e.iface.as_mut_ptr(), *tb.add(IPSET_ATTR_IFACE as usize), IFNAMSIZ);
    if !(*tb.add(IPSET_ATTR_CADT_FLAGS as usize)).is_null() { let f = ip_set_get_h32(*tb.add(IPSET_ATTR_CADT_FLAGS as usize)); if f & IPSET_FLAG_PHYSDEV != 0 { e.physdev=1; } if f & IPSET_FLAG_NOMATCH != 0 { flags |= IPSET_FLAG_NOMATCH << 16; } if f & IPSET_FLAG_IFACE_WILDCARD != 0 { e.wildcard=1; } }
    if adt == IPSET_TEST || (*tb.add(IPSET_ATTR_IP_TO as usize)).is_null() { e.ip=htonl(ip & ip_set_hostmask(e.cidr)); ret=adtfn(set,&mut e as *mut _ as *mut c_void,&mut ext,&mut ext,flags); return if ip_set_enomatch(ret,flags,adt,set)!=0 {-ret} else if ip_set_eexist(ret,flags)!=0 {0} else {ret}; }
    ret=ip_set_get_hostipaddr4(*tb.add(IPSET_ATTR_IP_TO as usize),&mut ip_to); if ret!=0{return ret;} if ip_to<ip{core::mem::swap(&mut ip,&mut ip_to);} if ip.wrapping_add(u32::MAX)==ip_to{return -IPSET_ERR_HASH_RANGE;} if retried{ip=ntohl((*h).next.ip);} loop {i+=1;e.ip=htonl(ip);if i>IPSET_MAX_RANGE{hash_netiface4_data_next(&mut (*h).next,&e);return -ERANGE;} ip=ip_set_range_to_cidr(ip,ip_to,&mut e.cidr);ret=adtfn(set,&mut e as *mut _ as *mut c_void,&mut ext,&mut ext,flags);if ret!=0&&ip_set_eexist(ret,flags)==0{return ret;} if ip>=ip_to{break;}ip=ip.wrapping_add(1);} 0
}

#[repr(C)] pub union nf_inet_addr { pub all: [u32;4], pub in6: in6_addr }
#[repr(C)] pub struct hash_netiface6_elem_hashed { pub ip:nf_inet_addr,pub physdev:u8,pub cidr:u8,pub nomatch:u8,pub elem:u8 }
#[repr(C)] pub struct hash_netiface6_elem { pub ip:nf_inet_addr,pub physdev:u8,pub cidr:u8,pub nomatch:u8,pub elem:u8,pub wildcard:u8,pub iface:[c_char;IFNAMSIZ] }

pub unsafe fn hash_netiface6_data_equal(a:*const hash_netiface6_elem,b:*const hash_netiface6_elem,m:*mut u32)->bool{ipv6_addr_equal(&(*a).ip.in6,&(*b).ip.in6)&&(*a).cidr==(*b).cidr&&{*m=(*m).wrapping_add(1);true}&&(*a).physdev==(*b).physdev&&if(*a).wildcard!=0{strncmp((*a).iface.as_ptr(),(*b).iface.as_ptr(),strlen((*a).iface.as_ptr()))==0}else{strcmp((*a).iface.as_ptr(),(*b).iface.as_ptr())==0}}
pub unsafe fn hash_netiface6_do_data_match(e:*const hash_netiface6_elem)->c_int{if(*e).nomatch!=0{-ENOTEMPTY}else{1}}
pub unsafe fn hash_netiface6_data_set_flags(e:*mut hash_netiface6_elem,f:u32){(*e).nomatch=((f>>16)&IPSET_FLAG_NOMATCH)as u8}
pub unsafe fn hash_netiface6_data_reset_flags(e:*mut hash_netiface6_elem,f:*mut u8){core::mem::swap(&mut*f,&mut(*e).nomatch)}
pub unsafe fn hash_netiface6_data_netmask(e:*mut hash_netiface6_elem,c:u8){ip6_netmask(&mut(*e).ip,c);(*e).cidr=c}
pub unsafe fn hash_netiface6_data_list(s:*mut sk_buff,d:*const hash_netiface6_elem)->bool{let mut f=if(*d).physdev!=0{IPSET_FLAG_PHYSDEV}else{0}|if(*d).wildcard!=0{IPSET_FLAG_IFACE_WILDCARD}else{0};if(*d).nomatch!=0{f|=IPSET_FLAG_NOMATCH;}nla_put_ipaddr6(s,IPSET_ATTR_IP,&(*d).ip.in6)!=0||nla_put_u8(s,IPSET_ATTR_CIDR,(*d).cidr)!=0||nla_put_string(s,IPSET_ATTR_IFACE,(*d).iface.as_ptr())!=0||(f!=0&&nla_put_net32(s,IPSET_ATTR_CADT_FLAGS,htonl(f))!=0)}
pub unsafe fn hash_netiface6_data_next(_next:*mut hash_netiface6_elem,_d:*const hash_netiface6_elem){}
// The IPv6 generated implementation is provided by ip_set_hash_gen.h.

pub unsafe fn hash_netiface6_kadt(_set:*mut ip_set,_skb:*const sk_buff,_par:*const xt_action_param,_adt:ipset_adt,_opt:*mut ip_set_adt_opt)->c_int{unimplemented!()}
pub unsafe fn hash_netiface6_uadt(_set:*mut ip_set,_tb:*mut *mut nlattr,_adt:ipset_adt,_lineno:*mut u32,_flags:u32,_retried:bool)->c_int{unimplemented!()}

pub static mut hash_netiface_type: ip_set_type = ip_set_type { name: b"hash:net,iface\0".as_ptr() as *const c_char, protocol: IPSET_PROTOCOL, features: IPSET_TYPE_IP|IPSET_TYPE_IFACE|IPSET_TYPE_NOMATCH, dimension: IPSET_DIM_TWO, family: NFPROTO_UNSPEC, revision_min: IPSET_TYPE_REV_MIN, revision_max: IPSET_TYPE_REV_MAX, ..unsafe { core::mem::zeroed() } };
pub unsafe fn hash_netiface_init()->c_int { ip_set_type_register(&mut hash_netiface_type) }
pub unsafe fn hash_netiface_fini(){rcu_barrier();ip_set_type_unregister(&mut hash_netiface_type)}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
