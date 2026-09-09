// SPDX-License-Identifier: GPL-2.0-or-later
/* NetLabel Kernel API -- direct Rust translation of netlabel_kapi.c. */

// External kernel types, constants, and functions are supplied by the surrounding tree.

const _CM_F_NONE: u32 = 0x00000000;
const _CM_F_ALLOC: u32 = 0x00000001;
const _CM_F_WALK: u32 = 0x00000002;

unsafe fn _netlbl_catmap_getnode(catmap:*mut *mut netlbl_lsm_catmap,offset:u32,flags:u32,gfp:gfp_t)->*mut netlbl_lsm_catmap{let mut i=*catmap;let mut prev=core::ptr::null_mut();if i.is_null(){return if flags&_CM_F_ALLOC!=0{let n=netlbl_catmap_alloc(gfp);if !n.is_null(){(*n).startbit=offset&!(NETLBL_CATMAP_SIZE-1);*catmap=n;}n}else{core::ptr::null_mut()}}if offset<(*i).startbit&&flags&_CM_F_WALK!=0{return i}while !i.is_null()&&offset>=(*i).startbit+NETLBL_CATMAP_SIZE{prev=i;i=(*i).next}if !i.is_null()&&offset>=(*i).startbit{return i}if flags&_CM_F_WALK!=0{return i}if flags&_CM_F_ALLOC==0{return core::ptr::null_mut()}let n=netlbl_catmap_alloc(gfp);if n.is_null(){return n}(*n).startbit=offset&!(NETLBL_CATMAP_SIZE-1);if prev.is_null(){(*n).next=*catmap;*catmap=n}else{(*n).next=(*prev).next;(*prev).next=n}n}

pub unsafe fn netlbl_cfg_map_del(domain: *const i8, family: u16, addr: *const core::ffi::c_void, mask: *const core::ffi::c_void, audit_info: *mut netlbl_audit) -> i32 {
    if addr.is_null() && mask.is_null() { netlbl_domhsh_remove(domain, family, audit_info) }
    else if !addr.is_null() && !mask.is_null() {
        match family { AF_INET => netlbl_domhsh_remove_af4(domain, addr, mask, audit_info), AF_INET6 => netlbl_domhsh_remove_af6(domain, addr, mask, audit_info), _ => -EPFNOSUPPORT }
    } else { -EINVAL }
}

pub unsafe fn netlbl_cfg_unlbl_map_add(domain: *const i8, family: u16, addr: *const core::ffi::c_void, mask: *const core::ffi::c_void, audit_info: *mut netlbl_audit) -> i32 {
    let mut ret_val = -ENOMEM;
    let entry = kzalloc_obj::<netlbl_dom_map>();
    if entry.is_null() { return -ENOMEM; }
    let mut addrmap: *mut netlbl_domaddr_map = core::ptr::null_mut();
    let mut map4: *mut netlbl_domaddr4_map = core::ptr::null_mut();
    let mut map6: *mut netlbl_domaddr6_map = core::ptr::null_mut();
    if !domain.is_null() { (*entry).domain = kstrdup(domain, GFP_ATOMIC); if (*entry).domain.is_null() { goto_cfg_unlbl_failure(entry, addrmap, map4, map6); return ret_val; } }
    (*entry).family = family;
    if addr.is_null() && mask.is_null() { (*entry).def_.type_ = NETLBL_NLTYPE_UNLABELED; }
    else if !addr.is_null() && !mask.is_null() {
        addrmap = kzalloc_obj::<netlbl_domaddr_map>(); if addrmap.is_null() { goto_cfg_unlbl_failure(entry, addrmap, map4, map6); return ret_val; }
        INIT_LIST_HEAD(&mut (*addrmap).list4); INIT_LIST_HEAD(&mut (*addrmap).list6);
        match family {
            AF_INET => { map4 = kzalloc_obj::<netlbl_domaddr4_map>(); if map4.is_null() { goto_cfg_unlbl_failure(entry, addrmap, map4, map6); return ret_val; } (*map4).def_.type_ = NETLBL_NLTYPE_UNLABELED; (*map4).list.addr = (*(addr as *const in_addr)).s_addr & (*(mask as *const in_addr)).s_addr; (*map4).list.mask = (*(mask as *const in_addr)).s_addr; (*map4).list.valid = 1; ret_val = netlbl_af4list_add(&mut (*map4).list, &mut (*addrmap).list4); if ret_val != 0 { goto_cfg_unlbl_failure(entry, addrmap, map4, map6); return ret_val; } }
            AF_INET6 => { map6 = kzalloc_obj::<netlbl_domaddr6_map>(); if map6.is_null() { goto_cfg_unlbl_failure(entry, addrmap, map4, map6); return ret_val; } (*map6).def_.type_ = NETLBL_NLTYPE_UNLABELED; (*map6).list.addr = (*(addr as *const in6_addr)); for i in 0..4 { (*map6).list.addr.s6_addr32[i] &= (*(mask as *const in6_addr)).s6_addr32[i]; } (*map6).list.mask = *(mask as *const in6_addr); (*map6).list.valid = 1; ret_val = netlbl_af6list_add(&mut (*map6).list, &mut (*addrmap).list6); if ret_val != 0 { goto_cfg_unlbl_failure(entry, addrmap, map4, map6); return ret_val; } }
            _ => { goto_cfg_unlbl_failure(entry, addrmap, map4, map6); return ret_val; }
        }
        (*entry).def_.addrsel = addrmap; (*entry).def_.type_ = NETLBL_NLTYPE_ADDRSELECT;
    } else { ret_val = -EINVAL; goto_cfg_unlbl_failure(entry, addrmap, map4, map6); return ret_val; }
    ret_val = netlbl_domhsh_add(entry, audit_info); if ret_val != 0 { goto_cfg_unlbl_failure(entry, addrmap, map4, map6); return ret_val; } 0
}

unsafe fn goto_cfg_unlbl_failure(entry: *mut netlbl_dom_map, addrmap: *mut netlbl_domaddr_map, map4: *mut netlbl_domaddr4_map, map6: *mut netlbl_domaddr6_map) { kfree((*entry).domain); kfree(entry); kfree(addrmap); kfree(map4); kfree(map6); }

pub unsafe fn netlbl_cfg_unlbl_static_add(net: *mut net, dev_name: *const i8, addr: *const core::ffi::c_void, mask: *const core::ffi::c_void, family: u16, secid: u32, audit_info: *mut netlbl_audit) -> i32 { let addr_len = match family { AF_INET => core::mem::size_of::<in_addr>() as u32, AF_INET6 => core::mem::size_of::<in6_addr>() as u32, _ => return -EPFNOSUPPORT }; netlbl_unlhsh_add(net, dev_name, addr, mask, addr_len, secid, audit_info) }
pub unsafe fn netlbl_cfg_unlbl_static_del(net: *mut net, dev_name: *const i8, addr: *const core::ffi::c_void, mask: *const core::ffi::c_void, family: u16, audit_info: *mut netlbl_audit) -> i32 { let addr_len = match family { AF_INET => core::mem::size_of::<in_addr>() as u32, AF_INET6 => core::mem::size_of::<in6_addr>() as u32, _ => return -EPFNOSUPPORT }; netlbl_unlhsh_remove(net, dev_name, addr, mask, addr_len, audit_info) }
pub unsafe fn netlbl_cfg_cipsov4_add(doi_def: *mut cipso_v4_doi, audit_info: *mut netlbl_audit) -> i32 { cipso_v4_doi_add(doi_def, audit_info) }
pub unsafe fn netlbl_cfg_cipsov4_del(doi: u32, audit_info: *mut netlbl_audit) { cipso_v4_doi_remove(doi, audit_info); }
pub unsafe fn netlbl_cfg_cipsov4_map_add(doi:u32,domain:*const i8,addr:*const in_addr,mask:*const in_addr,audit:*mut netlbl_audit)->i32{let d=cipso_v4_doi_getdef(doi);if d.is_null(){return -ENOENT}let e=kzalloc_obj::<netlbl_dom_map>();if e.is_null(){cipso_v4_doi_putdef(d);return -ENOMEM}(*e).family=AF_INET;(*e).domain=if domain.is_null(){core::ptr::null_mut()}else{kstrdup(domain,GFP_ATOMIC)};if addr.is_null()&&mask.is_null(){(*e).def_.cipso=d;(*e).def_.type_=NETLBL_NLTYPE_CIPSOV4}else if !addr.is_null()&&!mask.is_null(){let am=kzalloc_obj::<netlbl_domaddr_map>();let ai=kzalloc_obj::<netlbl_domaddr4_map>();if am.is_null()||ai.is_null(){kfree(am);kfree(ai);kfree((*e).domain);kfree(e);cipso_v4_doi_putdef(d);return -ENOMEM}(*ai).def_.cipso=d;(*ai).def_.type_=NETLBL_NLTYPE_CIPSOV4;(*ai).list.addr=(*addr).s_addr&(*mask).s_addr;(*ai).list.mask=(*mask).s_addr;(*ai).list.valid=1;let r=netlbl_af4list_add(&mut (*ai).list,&mut (*am).list4);if r!=0{kfree(ai);kfree(am);kfree((*e).domain);kfree(e);cipso_v4_doi_putdef(d);return r}(*e).def_.addrsel=am;(*e).def_.type_=NETLBL_NLTYPE_ADDRSELECT}else{ kfree((*e).domain);kfree(e);cipso_v4_doi_putdef(d);return -EINVAL}let r=netlbl_domhsh_add(e,audit);if r!=0{ kfree((*e).domain);kfree(e);cipso_v4_doi_putdef(d)}r}
pub unsafe fn netlbl_cfg_calipso_add(d:*mut calipso_doi,a:*mut netlbl_audit)->i32{calipso_doi_add(d,a)}
pub unsafe fn netlbl_cfg_calipso_del(doi:u32,a:*mut netlbl_audit){calipso_doi_remove(doi,a)}
pub unsafe fn netlbl_cfg_calipso_map_add(_doi:u32,_domain:*const i8,_addr:*const in6_addr,_mask:*const in6_addr,_audit:*mut netlbl_audit)->i32{-ENOSYS}

pub unsafe fn netlbl_catmap_walk(catmap: *mut netlbl_lsm_catmap, mut offset: u32) -> i32 { let mut iter = _netlbl_catmap_getnode(&mut catmap, offset, _CM_F_WALK, 0); if iter.is_null() { return -ENOENT; } let (mut idx, mut bit) = if offset > (*iter).startbit { offset -= (*iter).startbit; (offset / NETLBL_CATMAP_MAPSIZE, offset % NETLBL_CATMAP_MAPSIZE) } else {(0,0)}; let mut bitmap = (*iter).bitmap[idx as usize] >> bit; loop { if bitmap != 0 { while bitmap & NETLBL_CATMAP_BIT == 0 { bitmap >>= 1; bit += 1; } return ((*iter).startbit + NETLBL_CATMAP_MAPSIZE * idx + bit) as i32; } idx += 1; if idx >= NETLBL_CATMAP_MAPCNT { if (*iter).next.is_null() { return -ENOENT; } iter = (*iter).next; idx = 0; } bitmap = (*iter).bitmap[idx as usize]; bit = 0; } }

pub unsafe fn netlbl_catmap_getlong(catmap: *mut netlbl_lsm_catmap, offset: *mut u32, bitmap: *mut usize) -> i32 { let mut off = *offset; if off & (BITS_PER_LONG - 1) != 0 { return -EINVAL; } if catmap.is_null() { *offset = u32::MAX; return 0; } if off < (*catmap).startbit { off = (*catmap).startbit; *offset = off; } let iter = _netlbl_catmap_getnode(&mut catmap.clone(), off, _CM_F_WALK, 0); if iter.is_null() { *offset = u32::MAX; return 0; } if off < (*iter).startbit { *offset = (*iter).startbit; off = 0; } else { off -= (*iter).startbit; } let idx = off / NETLBL_CATMAP_MAPSIZE; *bitmap = ((*iter).bitmap[idx as usize] >> (off % NETLBL_CATMAP_MAPSIZE)) as usize; 0 }
pub unsafe fn netlbl_catmap_setbit(catmap: *mut *mut netlbl_lsm_catmap, mut bit: u32, flags: gfp_t) -> i32 { let iter = _netlbl_catmap_getnode(catmap, bit, _CM_F_ALLOC, flags); if iter.is_null() { return -ENOMEM; } bit -= (*iter).startbit; (*iter).bitmap[(bit / NETLBL_CATMAP_MAPSIZE) as usize] |= NETLBL_CATMAP_BIT << (bit % NETLBL_CATMAP_MAPSIZE); 0 }
pub unsafe fn netlbl_catmap_setrng(catmap: *mut *mut netlbl_lsm_catmap, start: u32, end: u32, flags: gfp_t) -> i32 { let mut rc=0; let mut spot=start; while rc==0 && spot<=end { if spot & (BITS_PER_LONG-1)==0 && end-spot>BITS_PER_LONG { rc=netlbl_catmap_setlong(catmap,spot,usize::MAX,flags); spot+=BITS_PER_LONG; } else { rc=netlbl_catmap_setbit(catmap,spot,flags); spot+=1; } } rc }
pub unsafe fn netlbl_catmap_setlong(catmap: *mut *mut netlbl_lsm_catmap, mut offset: u32, bitmap: usize, flags: gfp_t) -> i32 { if offset & (BITS_PER_LONG-1)!=0 { return -EINVAL; } let iter=_netlbl_catmap_getnode(catmap,offset,_CM_F_ALLOC,flags); if iter.is_null(){return -ENOMEM;} offset-=(*iter).startbit; (*iter).bitmap[(offset/NETLBL_CATMAP_MAPSIZE) as usize] |= (bitmap as u64) << (offset%NETLBL_CATMAP_MAPSIZE); 0 }

pub unsafe fn netlbl_bitmap_walk(bitmap: *const u8, bitmap_len:u32, mut offset:u32, state:u8)->i32 { if offset>=bitmap_len{return -1;} let mut byte_offset=offset/8; let mut byte=*bitmap.add(byte_offset as usize); let mut bitmask=0x80u8>>(offset%8); while offset<bitmap_len { if (state!=0 && byte&bitmask==bitmask)||(state==0&&byte&bitmask==0){return offset as i32;} offset+=1; if offset>=bitmap_len{return -1;} bitmask>>=1; if bitmask==0 {byte_offset+=1; byte=*bitmap.add(byte_offset as usize); bitmask=0x80;} } -1 }
pub unsafe fn netlbl_bitmap_setbit(bitmap:*mut u8, bit:u32, state:u8){let p=bitmap.add((bit/8) as usize);let m=0x80u8>>(bit%8);if state!=0{*p|=m}else{*p&=!m}}

pub unsafe fn netlbl_enabled()->i32 { (atomic_read(&netlabel_mgmt_protocount)>0) as i32 }

pub unsafe fn netlbl_sock_setattr(sk:*mut sock,family:u16,secattr:*const netlbl_lsm_secattr,sk_locked:bool)->i32{rcu_read_lock();let e=netlbl_domhsh_getentry((*secattr).domain,family);let r=if e.is_null(){-ENOENT}else{match family{AF_INET=>match (*e).def_.type_{NETLBL_NLTYPE_ADDRSELECT=>-EDESTADDRREQ,NETLBL_NLTYPE_CIPSOV4=>cipso_v4_sock_setattr(sk,(*e).def_.cipso,secattr,sk_locked),NETLBL_NLTYPE_UNLABELED=>0,_=>-ENOENT},AF_INET6=>match (*e).def_.type_{NETLBL_NLTYPE_ADDRSELECT=>-EDESTADDRREQ,NETLBL_NLTYPE_CALIPSO=>calipso_sock_setattr(sk,(*e).def_.calipso,secattr),NETLBL_NLTYPE_UNLABELED=>0,_=>-ENOENT},_=>-EPROTONOSUPPORT}};rcu_read_unlock();r}
pub unsafe fn netlbl_sock_delattr(sk:*mut sock){match (*sk).sk_family{AF_INET=>cipso_v4_sock_delattr(sk),AF_INET6=>calipso_sock_delattr(sk),_=>{}}}
pub unsafe fn netlbl_sock_getattr(sk:*mut sock,secattr:*mut netlbl_lsm_secattr)->i32{match (*sk).sk_family{AF_INET=>cipso_v4_sock_getattr(sk,secattr),AF_INET6=>calipso_sock_getattr(sk,secattr),_=>-EPROTONOSUPPORT}}
pub unsafe fn netlbl_sk_lock_check(_sk:*mut sock)->bool{true}
pub unsafe fn netlbl_conn_setattr(sk:*mut sock,addr:*mut sockaddr,secattr:*const netlbl_lsm_secattr)->i32{rcu_read_lock();let r=match (*addr).sa_family{AF_INET=>{let a=addr as *mut sockaddr_in;let e=netlbl_domhsh_getentry_af4((*secattr).domain,(*a).sin_addr.s_addr);if e.is_null(){-ENOENT}else{match (*e).type_{NETLBL_NLTYPE_CIPSOV4=>cipso_v4_sock_setattr(sk,(*e).cipso,secattr,netlbl_sk_lock_check(sk)),NETLBL_NLTYPE_UNLABELED=>{netlbl_sock_delattr(sk);0},_=>-ENOENT}}},AF_INET6=>{if (*sk).sk_family!=AF_INET6{-EAFNOSUPPORT}else{let a=addr as *mut sockaddr_in6;let e=netlbl_domhsh_getentry_af6((*secattr).domain,&(*a).sin6_addr);if e.is_null(){-ENOENT}else{match (*e).type_{NETLBL_NLTYPE_CALIPSO=>calipso_sock_setattr(sk,(*e).calipso,secattr),NETLBL_NLTYPE_UNLABELED=>{netlbl_sock_delattr(sk);0},_=>-ENOENT}}}},_=>-EPROTONOSUPPORT};rcu_read_unlock();r}
pub unsafe fn netlbl_req_setattr(req:*mut request_sock,secattr:*const netlbl_lsm_secattr)->i32{let ireq=inet_rsk(req);rcu_read_lock();let r=match (*(*req).rsk_ops).family{AF_INET=>{let e=netlbl_domhsh_getentry_af4((*secattr).domain,(*ireq).ir_rmt_addr);if e.is_null(){-ENOENT}else{match (*e).type_{NETLBL_NLTYPE_CIPSOV4=>cipso_v4_req_setattr(req,(*e).cipso,secattr),NETLBL_NLTYPE_UNLABELED=>{netlbl_req_delattr(req);0},_=>-ENOENT}}},AF_INET6=>{let e=netlbl_domhsh_getentry_af6((*secattr).domain,&(*ireq).ir_v6_rmt_addr);if e.is_null(){-ENOENT}else{match (*e).type_{NETLBL_NLTYPE_CALIPSO=>calipso_req_setattr(req,(*e).calipso,secattr),NETLBL_NLTYPE_UNLABELED=>{netlbl_req_delattr(req);0},_=>-ENOENT}}},_=>-EPROTONOSUPPORT};rcu_read_unlock();r}
pub unsafe fn netlbl_req_delattr(req:*mut request_sock){match (*(*req).rsk_ops).family{AF_INET=>cipso_v4_req_delattr(req),AF_INET6=>calipso_req_delattr(req),_=>{}}}
pub unsafe fn netlbl_skbuff_setattr(skb:*mut sk_buff,family:u16,secattr:*const netlbl_lsm_secattr)->i32{rcu_read_lock();let r=match family{AF_INET=>{let e=netlbl_domhsh_getentry_af4((*secattr).domain,(*ip_hdr(skb)).daddr);if e.is_null(){-ENOENT}else{match (*e).type_{NETLBL_NLTYPE_CIPSOV4=>cipso_v4_skbuff_setattr(skb,(*e).cipso,secattr),NETLBL_NLTYPE_UNLABELED=>cipso_v4_skbuff_delattr(skb),_=>-ENOENT}}},AF_INET6=>{let e=netlbl_domhsh_getentry_af6((*secattr).domain,&(*ipv6_hdr(skb)).daddr);if e.is_null(){-ENOENT}else{match (*e).type_{NETLBL_NLTYPE_CALIPSO=>calipso_skbuff_setattr(skb,(*e).calipso,secattr),NETLBL_NLTYPE_UNLABELED=>calipso_skbuff_delattr(skb),_=>-ENOENT}}},_=>-EPROTONOSUPPORT};rcu_read_unlock();r}
pub unsafe fn netlbl_skbuff_getattr(skb:*const sk_buff,family:u16,secattr:*mut netlbl_lsm_secattr)->i32{let p=match family{AF_INET=>cipso_v4_optptr(skb),AF_INET6=>calipso_optptr(skb),_=>core::ptr::null_mut()};if !p.is_null(){let r=match family{AF_INET=>cipso_v4_getattr(p,secattr),AF_INET6=>calipso_getattr(p,secattr),_=>-1};if r==0{return 0}}netlbl_unlabel_getattr(skb,family,secattr)}
pub unsafe fn netlbl_skbuff_err(skb:*mut sk_buff,family:u16,error:i32,gateway:i32){if family==AF_INET&&!cipso_v4_optptr(skb).is_null(){cipso_v4_error(skb,error,gateway)}}
pub unsafe fn netlbl_cache_invalidate(){cipso_v4_cache_invalidate();calipso_cache_invalidate()}
pub unsafe fn netlbl_cache_add(skb:*const sk_buff,family:u16,secattr:*const netlbl_lsm_secattr)->i32{if (*secattr).flags&NETLBL_SECATTR_CACHE==0{return -ENOMSG}let p=match family{AF_INET=>cipso_v4_optptr(skb),AF_INET6=>calipso_optptr(skb),_=>core::ptr::null_mut()};if p.is_null(){-ENOMSG}else{match family{AF_INET=>cipso_v4_cache_add(p,secattr),AF_INET6=>calipso_cache_add(p,secattr),_=>-ENOMSG}}}
pub unsafe fn netlbl_audit_start(ty:i32,audit_info:*mut netlbl_audit)->*mut audit_buffer{netlbl_audit_start_common(ty,audit_info)}
pub unsafe fn netlbl_init()->i32{printk(KERN_INFO,b"NetLabel: Initializing\0".as_ptr());let mut r=netlbl_domhsh_init(NETLBL_DOMHSH_BITSIZE);if r!=0{panic!("NetLabel: failed to initialize properly ({})",r)}r=netlbl_unlabel_init(NETLBL_UNLHSH_BITSIZE);if r!=0{panic!("NetLabel: failed to initialize properly ({})",r)}r=netlbl_netlink_init();if r!=0{panic!("NetLabel: failed to initialize properly ({})",r)}r=netlbl_unlabel_defconf();if r!=0{panic!("NetLabel: failed to initialize properly ({})",r)}0}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
