// SPDX-License-Identifier: GPL-2.0-or-later
/* SCTP kernel implementation; literal Rust translation of bind_addr.c. */

// Kernel headers and symbols are supplied by the surrounding translation.

unsafe fn sctp_copy_one_addr(net: *mut net, dest: *mut sctp_bind_addr,
    addr: *mut sctp_addr, scope: sctp_scope, gfp: gfp_t, flags: i32) -> i32 {
    let mut error = 0;
    if sctp_is_any(core::ptr::null_mut(), addr) != 0 {
        error = sctp_copy_local_addr_list(net, dest, scope, gfp, flags);
    } else if sctp_in_scope(net, addr, scope) != 0 {
        if ((addr.sa.sa_family == AF_INET && flags & SCTP_ADDR4_ALLOWED != 0 && flags & SCTP_ADDR4_PEERSUPP != 0) ||
            (addr.sa.sa_family == AF_INET6 && flags & SCTP_ADDR6_ALLOWED != 0 && flags & SCTP_ADDR6_PEERSUPP != 0)) {
            error = sctp_add_bind_addr(dest, addr, core::mem::size_of::<sctp_addr>() as i32, SCTP_ADDR_SRC, gfp);
        }
    }
    error
}

unsafe fn sctp_bind_addr_clean(bp: *mut sctp_bind_addr) {
    // list_for_each_entry_safe(addr, temp, &bp->address_list, list)
    // { list_del_rcu(&addr->list); kfree_rcu(addr, rcu); SCTP_DBG_OBJCNT_DEC(addr); }
    list_for_each_entry_safe_clean(bp);
}

pub unsafe fn sctp_bind_addr_copy(net: *mut net, dest: *mut sctp_bind_addr,
    src: *const sctp_bind_addr, scope: sctp_scope, gfp: gfp_t, flags: i32) -> i32 {
    (*dest).port = (*src).port;
    let mut error = 0;
    // list_for_each_entry(addr, &src->address_list, list)
    for addr in bind_addr_entries(src) {
        error = sctp_copy_one_addr(net, dest, &mut (*addr).a, scope, gfp, flags);
        if error < 0 { break; }
    }
    if bind_addr_empty(dest) && scope == SCTP_SCOPE_GLOBAL {
        for addr in bind_addr_entries(src) {
            error = sctp_copy_one_addr(net, dest, &mut (*addr).a, SCTP_SCOPE_LINK, gfp, flags);
            if error < 0 { break; }
        }
    }
    if bind_addr_empty(dest) { error = -ENETUNREACH; }
    if error != 0 { sctp_bind_addr_clean(dest); }
    error
}

pub unsafe fn sctp_bind_addr_dup(dest: *mut sctp_bind_addr, src: *const sctp_bind_addr, gfp: gfp_t) -> i32 {
    (*dest).port = (*src).port;
    let mut error = 0;
    for addr in bind_addr_entries(src) {
        error = sctp_add_bind_addr(dest, &mut (*addr).a, core::mem::size_of::<sctp_addr>() as i32, 1, gfp);
        if error < 0 { break; }
    }
    error
}

pub unsafe fn sctp_bind_addr_init(bp: *mut sctp_bind_addr, port: u16) { INIT_LIST_HEAD!(&mut (*bp).address_list); (*bp).port = port; }
pub unsafe fn sctp_bind_addr_free(bp: *mut sctp_bind_addr) { sctp_bind_addr_clean(bp); }

pub unsafe fn sctp_add_bind_addr(bp: *mut sctp_bind_addr, new_addr: *mut sctp_addr,
    new_size: i32, addr_state: u8, gfp: gfp_t) -> i32 {
    let addr = kzalloc_obj!(sctp_sockaddr_entry, gfp);
    if addr.is_null() { return -ENOMEM; }
    memcpy!(&mut (*addr).a, new_addr, core::cmp::min(core::mem::size_of::<sctp_addr>(), new_size as usize));
    if (*addr).a.v4.sin_port == 0 { (*addr).a.v4.sin_port = htons((*bp).port); }
    (*addr).state = addr_state; (*addr).valid = 1; INIT_LIST_HEAD!(&mut (*addr).list);
    list_add_tail_rcu!(&mut (*addr).list, &mut (*bp).address_list); SCTP_DBG_OBJCNT_INC!(addr); 0
}

pub unsafe fn sctp_del_bind_addr(bp: *mut sctp_bind_addr, del_addr: *mut sctp_addr) -> i32 {
    for addr in bind_addr_entries_mut(bp) {
        if sctp_cmp_addr_exact(&(*addr).a, del_addr) != 0 { (*addr).valid = 0; list_del_rcu!(&mut (*addr).list); kfree_rcu!(addr, rcu); SCTP_DBG_OBJCNT_DEC!(addr); return 0; }
    }
    -EINVAL
}

pub unsafe fn sctp_bind_addrs_to_raw(bp: *const sctp_bind_addr, addrs_len: *mut i32, gfp: gfp_t) -> sctp_params {
    let mut len = 0; for _ in bind_addr_entries(bp) { len += core::mem::size_of::<sctp_addr_param>(); }
    if len == core::mem::size_of::<sctp_addr_param>() { *addrs_len = 0; return sctp_params { v: core::ptr::null_mut() }; }
    let mut out = kmalloc!(len, gfp); if out.is_null() { *addrs_len = 0; return sctp_params { v: core::ptr::null_mut() }; }
    let mut used = 0; for addr in bind_addr_entries(bp) { let af = sctp_get_af_specific((*addr).a.v4.sin_family); let raw = core::mem::MaybeUninit::uninit(); let n = ((*af).to_addr_param)(&(*addr).a, raw.as_mut_ptr()); memcpy!(out.add(used), raw.as_ptr(), n as usize); used += n as usize; }
    *addrs_len = used as i32; sctp_params { v: out }
}

pub unsafe fn sctp_raw_to_bind_addrs(bp: *mut sctp_bind_addr, mut raw: *mut u8, mut addrs_len: i32, port: u16, gfp: gfp_t) -> i32 {
    let mut retval = 0; while addrs_len != 0 { if addrs_len < core::mem::size_of::<sctp_paramhdr>() as i32 { retval = -EINVAL; break; } let param = raw as *mut sctp_paramhdr; let len = ntohs((*param).length) as i32; if addrs_len < len { retval = -EINVAL; break; } let af = sctp_get_af_specific(param_type2af((*param).type)); let mut addr = core::mem::MaybeUninit::<sctp_addr>::uninit(); if af.is_null() || (*af).from_addr_param(addr.as_mut_ptr(), raw as *mut sctp_addr_param, htons(port), 0) == 0 { retval = -EINVAL; break; } let mut addr = addr.assume_init(); if sctp_bind_addr_state(bp, &addr) == -1 { retval = sctp_add_bind_addr(bp, &mut addr, core::mem::size_of::<sctp_addr>() as i32, SCTP_ADDR_SRC, gfp); if retval != 0 { break; } } addrs_len -= len; raw = raw.add(len as usize); } if retval != 0 { sctp_bind_addr_clean(bp); } retval
}

pub unsafe fn sctp_bind_addr_match(bp: *mut sctp_bind_addr, addr: *const sctp_addr, opt: *mut sctp_sock) -> i32 { for laddr in bind_addr_entries(bp) { if (*laddr).valid != 0 && ((*(*opt).pf).cmp_addr)(&(*laddr).a, addr, opt) != 0 { return 1; } } 0 }
pub unsafe fn sctp_bind_addrs_check(sp: *mut sctp_sock, sp2: *mut sctp_sock, cnt2: i32) -> i32 { let bp = &mut (*(*sp).ep).base.bind_addr; let bp2 = &mut (*(*sp2).ep).base.bind_addr; let mut cnt = 0; let mut exist = false; for a in bind_addr_entries(bp) { for b in bind_addr_entries(bp2) { if (*a).valid != 0 && (*b).valid != 0 && ((*(*sp).pf).af.cmp_addr)(&(*a).a, &(*b).a) != 0 { exist = true; break; } } if !exist { cnt = 0; break; } cnt += 1; } if cnt == cnt2 { 0 } else if exist { -EEXIST } else { 1 } }
pub unsafe fn sctp_bind_addr_conflict(bp: *mut sctp_bind_addr, addr: *const sctp_addr, bp_sp: *mut sctp_sock, addr_sp: *mut sctp_sock) -> i32 { let sp = if sctp_opt2sk(bp_sp).sk_family == AF_INET6 { bp_sp } else if sctp_opt2sk(addr_sp).sk_family == AF_INET6 { addr_sp } else { bp_sp }; for laddr in bind_addr_entries(bp) { if (*laddr).valid != 0 && ((*(*sp).pf).cmp_addr)(&(*laddr).a, addr, sp) != 0 { return 1; } } 0 }
pub unsafe fn sctp_bind_addr_state(bp: *const sctp_bind_addr, addr: *const sctp_addr) -> i32 { let af = sctp_get_af_specific((*addr).sa.sa_family); if af.is_null() { return -1; } for laddr in bind_addr_entries(bp) { if (*laddr).valid != 0 && ((*af).cmp_addr)(&(*laddr).a, addr) != 0 { return (*laddr).state as i32; } } -1 }
pub unsafe fn sctp_find_unmatch_addr(bp: *mut sctp_bind_addr, addrs: *const sctp_addr, addrcnt: i32, opt: *mut sctp_sock) -> *mut sctp_addr { for laddr in bind_addr_entries(bp) { let mut p = addrs; let mut i = 0; while i < addrcnt { let af = sctp_get_af_specific((*p).v4.sin_family); if af.is_null() || ((*(*opt).pf).cmp_addr)(&(*laddr).a, p, opt) != 0 { break; } p = (p as *const u8).add((*af).sockaddr_len as usize) as *const sctp_addr; i += 1; } if i == addrcnt { return &mut (*laddr).a; } } core::ptr::null_mut() }
pub unsafe fn sctp_is_ep_boundall(sk: *mut sock) -> i32 { let bp = &mut (*sctp_sk(sk)).ep.base.bind_addr; if sctp_list_single_entry(&bp.address_list) != 0 { let addr = list_entry_first!(bp, sctp_sockaddr_entry); if sctp_is_any(sk, &(*addr).a) != 0 { return 1; } } 0 }

pub unsafe fn sctp_is_any(sk: *mut sock, addr: *const sctp_addr) -> i32 { let fam = if (*addr).sa.sa_family != AF_UNSPEC { (*addr).sa.sa_family } else if !sk.is_null() { (*sk).sk_family } else { 0 }; let af = sctp_get_af_specific(fam); if af.is_null() { 0 } else { ((*af).is_any)(addr) } }
pub unsafe fn sctp_in_scope(net: *mut net, addr: *const sctp_addr, scope: sctp_scope) -> i32 { let a = sctp_scope(addr); if a == SCTP_SCOPE_UNUSABLE { return 0; } match (*net).sctp.scope_policy { SCTP_SCOPE_POLICY_DISABLE => 1, SCTP_SCOPE_POLICY_ENABLE => (a <= scope) as i32, SCTP_SCOPE_POLICY_PRIVATE => (a <= scope || a == SCTP_SCOPE_PRIVATE) as i32, SCTP_SCOPE_POLICY_LINK => (a <= scope || a == SCTP_SCOPE_LINK) as i32, _ => 0 } }
pub unsafe fn sctp_scope(addr: *const sctp_addr) -> sctp_scope { let af = sctp_get_af_specific((*addr).sa.sa_family); if af.is_null() { SCTP_SCOPE_UNUSABLE } else { ((*af).scope)(addr as *mut sctp_addr) } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
