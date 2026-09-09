// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2009, Oracle.  All rights reserved.
 *
 * Convert socket addresses to presentation addresses and universal
 * addresses, and vice versa.
 */

// C dependencies supplied by the surrounding kernel/Rust translation.

#[cfg(feature = "ipv6")]
unsafe fn rpc_ntop6_noscopeid(sap: *const sockaddr, buf: *mut c_char, buflen: c_int) -> usize {
    let sin6 = sap as *const sockaddr_in6;
    let addr = &(*sin6).sin6_addr;

    if ipv6_addr_any(addr) {
        return snprintf(buf, buflen, c"::".as_ptr());
    }
    if ipv6_addr_loopback(addr) {
        return snprintf(buf, buflen, c"::1".as_ptr());
    }
    if ipv6_addr_v4mapped(addr) {
        return snprintf(buf, buflen, c"::ffff:%pI4".as_ptr(), &addr.s6_addr32[3]);
    }
    snprintf(buf, buflen, c"%pI6c".as_ptr(), addr)
}

#[cfg(not(feature = "ipv6"))]
unsafe fn rpc_ntop6_noscopeid(_sap: *const sockaddr, _buf: *mut c_char, _buflen: c_int) -> usize {
    0
}

#[cfg(feature = "ipv6")]
unsafe fn rpc_ntop6(sap: *const sockaddr, buf: *mut c_char, buflen: usize) -> usize {
    let sin6 = sap as *const sockaddr_in6;
    let mut scopebuf = [0 as c_char; IPV6_SCOPE_ID_LEN];
    let len = rpc_ntop6_noscopeid(sap, buf, buflen);
    if len == 0 || (ipv6_addr_type(&(*sin6).sin6_addr) & IPV6_ADDR_LINKLOCAL) == 0
        || (*sin6).sin6_scope_id == 0
    {
        return len;
    }
    let rc = snprintf(scopebuf.as_mut_ptr(), scopebuf.len(), c"%c%u".as_ptr(), IPV6_SCOPE_DELIMITER, (*sin6).sin6_scope_id);
    if rc as usize >= scopebuf.len() || len + rc as usize >= buflen {
        return 0;
    }
    strcat(buf, scopebuf.as_ptr());
    len + rc as usize
}

#[cfg(not(feature = "ipv6"))]
unsafe fn rpc_ntop6(_sap: *const sockaddr, _buf: *mut c_char, _buflen: usize) -> usize { 0 }

unsafe fn rpc_ntop4(sap: *const sockaddr, buf: *mut c_char, buflen: usize) -> c_int {
    let sin = sap as *const sockaddr_in;
    snprintf(buf, buflen as c_int, c"%pI4".as_ptr(), &(*sin).sin_addr)
}

pub unsafe fn rpc_ntop(sap: *const sockaddr, buf: *mut c_char, buflen: usize) -> usize {
    match (*sap).sa_family {
        AF_INET => rpc_ntop4(sap, buf, buflen) as usize,
        AF_INET6 => rpc_ntop6(sap, buf, buflen),
        _ => 0,
    }
}

unsafe fn rpc_pton4(buf: *const c_char, buflen: usize, sap: *mut sockaddr, salen: usize) -> usize {
    let sin = sap as *mut sockaddr_in;
    let addr = &mut (*sin).sin_addr.s_addr as *mut _ as *mut u8;
    if buflen > INET_ADDRSTRLEN || salen < core::mem::size_of::<sockaddr_in>() { return 0; }
    memset(sap as *mut c_void, 0, core::mem::size_of::<sockaddr_in>());
    if in4_pton(buf, buflen, addr, 0, core::ptr::null_mut()) == 0 { return 0; }
    (*sin).sin_family = AF_INET;
    core::mem::size_of::<sockaddr_in>()
}

#[cfg(feature = "ipv6")]
unsafe fn rpc_parse_scope_id(net: *mut net, buf: *const c_char, buflen: usize, delim: *const c_char, sin6: *mut sockaddr_in6) -> c_int {
    let mut p = [0 as c_char; IPV6_SCOPE_ID_LEN + 1];
    if buf.add(buflen) == delim { return 1; }
    if *delim != IPV6_SCOPE_DELIMITER || (ipv6_addr_type(&(*sin6).sin6_addr) & IPV6_ADDR_LINKLOCAL) == 0 { return 0; }
    let len = buf.add(buflen).offset_from(delim) as usize - 1;
    if len > IPV6_SCOPE_ID_LEN { return 0; }
    memcpy(p.as_mut_ptr() as *mut c_void, delim.add(1) as *const c_void, len);
    p[len] = 0;
    let dev = dev_get_by_name(net, p.as_ptr());
    let scope_id = if !dev.is_null() { let v = (*dev).ifindex; dev_put(dev); v as u32 } else {
        let mut v = 0u32; if kstrtou32(p.as_ptr(), 10, &mut v) != 0 { return 0; } v
    };
    (*sin6).sin6_scope_id = scope_id;
    1
}

#[cfg(feature = "ipv6")]
unsafe fn rpc_pton6(net: *mut net, buf: *const c_char, buflen: usize, sap: *mut sockaddr, salen: usize) -> usize {
    let sin6 = sap as *mut sockaddr_in6;
    let addr = &mut (*sin6).sin6_addr.in6_u as *mut _ as *mut u8;
    if buflen > INET6_ADDRSTRLEN + IPV6_SCOPE_ID_LEN || salen < core::mem::size_of::<sockaddr_in6>() { return 0; }
    memset(sap as *mut c_void, 0, core::mem::size_of::<sockaddr_in6>());
    let mut delim = core::ptr::null();
    if in6_pton(buf, buflen, addr, IPV6_SCOPE_DELIMITER, &mut delim) == 0 || rpc_parse_scope_id(net, buf, buflen, delim, sin6) == 0 { return 0; }
    (*sin6).sin6_family = AF_INET6;
    core::mem::size_of::<sockaddr_in6>()
}

#[cfg(not(feature = "ipv6"))]
unsafe fn rpc_pton6(_net: *mut net, _buf: *const c_char, _buflen: usize, _sap: *mut sockaddr, _salen: usize) -> usize { 0 }

pub unsafe fn rpc_pton(net: *mut net, buf: *const c_char, buflen: usize, sap: *mut sockaddr, salen: usize) -> usize {
    for i in 0..buflen { if *buf.add(i) == b':' as c_char { return rpc_pton6(net, buf, buflen, sap, salen); } }
    rpc_pton4(buf, buflen, sap, salen)
}

pub unsafe fn rpc_sockaddr2uaddr(sap: *const sockaddr, gfp_flags: gfp_t) -> *mut c_char {
    let mut portbuf = [0 as c_char; RPCBIND_MAXUADDRPLEN];
    let mut addrbuf = [0 as c_char; RPCBIND_MAXUADDRLEN];
    let port: u16;
    match (*sap).sa_family {
        AF_INET => { if rpc_ntop4(sap, addrbuf.as_mut_ptr(), addrbuf.len()) == 0 { return core::ptr::null_mut(); } port = ntohs((*(sap as *const sockaddr_in)).sin_port); }
        AF_INET6 => { if rpc_ntop6_noscopeid(sap, addrbuf.as_mut_ptr(), addrbuf.len() as c_int) == 0 { return core::ptr::null_mut(); } port = ntohs((*(sap as *const sockaddr_in6)).sin6_port); }
        _ => return core::ptr::null_mut(),
    }
    if snprintf(portbuf.as_mut_ptr(), portbuf.len(), c".%u.%u".as_ptr(), port >> 8, port & 0xff) >= portbuf.len() as c_int { return core::ptr::null_mut(); }
    if strlcat(addrbuf.as_mut_ptr(), portbuf.as_ptr(), addrbuf.len()) >= addrbuf.len() { return core::ptr::null_mut(); }
    kstrdup(addrbuf.as_ptr(), gfp_flags)
}

pub unsafe fn rpc_uaddr2sockaddr(net: *mut net, uaddr: *const c_char, uaddr_len: usize, sap: *mut sockaddr, salen: usize) -> usize {
    let mut buf = [0 as c_char; RPCBIND_MAXUADDRLEN + 1];
    if uaddr_len > RPCBIND_MAXUADDRLEN { return 0; }
    memcpy(buf.as_mut_ptr() as *mut c_void, uaddr as *const c_void, uaddr_len); buf[uaddr_len] = 0;
    let mut c = strrchr(buf.as_mut_ptr(), b'.' as c_int); if c.is_null() { return 0; }
    let mut portlo = 0u8; if kstrtou8(c.add(1), 10, &mut portlo) != 0 { return 0; } *c = 0;
    c = strrchr(buf.as_mut_ptr(), b'.' as c_int); if c.is_null() { return 0; }
    let mut porthi = 0u8; if kstrtou8(c.add(1), 10, &mut porthi) != 0 { return 0; }
    let port = ((porthi as u16) << 8) | portlo as u16; *c = 0;
    if rpc_pton(net, buf.as_ptr(), strlen(buf.as_ptr()), sap, salen) == 0 { return 0; }
    match (*sap).sa_family { AF_INET => { (*(sap as *mut sockaddr_in)).sin_port = htons(port); core::mem::size_of::<sockaddr_in>() }, AF_INET6 => { (*(sap as *mut sockaddr_in6)).sin6_port = htons(port); core::mem::size_of::<sockaddr_in6>() }, _ => 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
