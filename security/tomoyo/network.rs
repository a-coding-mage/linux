// SPDX-License-Identifier: GPL-2.0
/* security/tomoyo/network.c */

#[repr(C)]
pub struct tomoyo_inet_addr_info {
    pub port: __be16,
    pub address: *const __be32,
    pub is_ipv6: bool,
}

#[repr(C)]
pub struct tomoyo_unix_addr_info {
    pub addr: *mut u8,
    pub addr_len: c_uint,
}

#[repr(C)]
pub struct tomoyo_addr_info {
    pub protocol: u8,
    pub operation: u8,
    pub inet: tomoyo_inet_addr_info,
    pub unix0: tomoyo_unix_addr_info,
}

pub static tomoyo_proto_keyword: [*const c_char; TOMOYO_SOCK_MAX as usize] = [
    [SOCK_STREAM as usize] = c"stream".as_ptr(),
    [SOCK_DGRAM as usize] = c"dgram".as_ptr(),
    [SOCK_RAW as usize] = c"raw".as_ptr(),
    [SOCK_SEQPACKET as usize] = c"seqpacket".as_ptr(),
    [0] = c" ".as_ptr(),
    [4] = c" ".as_ptr(),
];

pub unsafe fn tomoyo_parse_ipaddr_union(param: *mut tomoyo_acl_param, ptr: *mut tomoyo_ipaddr_union) -> bool {
    let min = (*ptr).ip[0].in6_u.u6_addr8.as_mut_ptr();
    let max = (*ptr).ip[1].in6_u.u6_addr8.as_mut_ptr();
    let address = tomoyo_read_token(param);
    let mut end: *const c_char = core::ptr::null();
    if strchr(address, ':' as c_int).is_null() && in4_pton(address, -1, min, '-' as c_int, &mut end) > 0 {
        (*ptr).is_ipv6 = false;
        if *end == 0 { (*ptr).ip[1].s6_addr32[0] = (*ptr).ip[0].s6_addr32[0]; }
        else if *end != '-' as c_char || { end = end.add(1); in4_pton(end, -1, max, 0, &mut end) <= 0 || *end != 0 } { return false; }
        return true;
    }
    if in6_pton(address, -1, min, '-' as c_int, &mut end) > 0 {
        (*ptr).is_ipv6 = true;
        if *end == 0 { memmove(max as *mut c_void, min as *const c_void, core::mem::size_of::<u16>() * 8); }
        else if *end != '-' as c_char || { end = end.add(1); in6_pton(end, -1, max, 0, &mut end) <= 0 || *end != 0 } { return false; }
        return true;
    }
    false
}

unsafe fn tomoyo_print_ipv4(buffer: *mut c_char, buffer_len: c_uint, min_ip: *const __be32, max_ip: *const __be32) {
    snprintf(buffer, buffer_len, c"%pI4%c%pI4".as_ptr(), min_ip, if *min_ip == *max_ip { 0 } else { '-' as c_int }, max_ip);
}

unsafe fn tomoyo_print_ipv6(buffer: *mut c_char, buffer_len: c_uint, min_ip: *const in6_addr, max_ip: *const in6_addr) {
    snprintf(buffer, buffer_len, c"%pI6c%c%pI6c".as_ptr(), min_ip, if memcmp(min_ip as *const c_void, max_ip as *const c_void, 16) == 0 { 0 } else { '-' as c_int }, max_ip);
}

pub unsafe fn tomoyo_print_ip(buf: *mut c_char, size: c_uint, ptr: *const tomoyo_ipaddr_union) {
    if (*ptr).is_ipv6 { tomoyo_print_ipv6(buf, size, &(*ptr).ip[0], &(*ptr).ip[1]); }
    else { tomoyo_print_ipv4(buf, size, &(*ptr).ip[0].s6_addr32[0], &(*ptr).ip[1].s6_addr32[0]); }
}

static tomoyo_inet2mac: [[u8; TOMOYO_MAX_NETWORK_OPERATION as usize]; TOMOYO_SOCK_MAX as usize] = {
    let mut x = [[0; TOMOYO_MAX_NETWORK_OPERATION as usize]; TOMOYO_SOCK_MAX as usize];
    x[SOCK_STREAM as usize][TOMOYO_NETWORK_BIND as usize] = TOMOYO_MAC_NETWORK_INET_STREAM_BIND;
    x[SOCK_STREAM as usize][TOMOYO_NETWORK_LISTEN as usize] = TOMOYO_MAC_NETWORK_INET_STREAM_LISTEN;
    x[SOCK_STREAM as usize][TOMOYO_NETWORK_CONNECT as usize] = TOMOYO_MAC_NETWORK_INET_STREAM_CONNECT;
    x[SOCK_DGRAM as usize][TOMOYO_NETWORK_BIND as usize] = TOMOYO_MAC_NETWORK_INET_DGRAM_BIND;
    x[SOCK_DGRAM as usize][TOMOYO_NETWORK_SEND as usize] = TOMOYO_MAC_NETWORK_INET_DGRAM_SEND;
    x[SOCK_RAW as usize][TOMOYO_NETWORK_BIND as usize] = TOMOYO_MAC_NETWORK_INET_RAW_BIND;
    x[SOCK_RAW as usize][TOMOYO_NETWORK_SEND as usize] = TOMOYO_MAC_NETWORK_INET_RAW_SEND;
    x
};

static tomoyo_unix2mac: [[u8; TOMOYO_MAX_NETWORK_OPERATION as usize]; TOMOYO_SOCK_MAX as usize] = {
    let mut x = [[0; TOMOYO_MAX_NETWORK_OPERATION as usize]; TOMOYO_SOCK_MAX as usize];
    x[SOCK_STREAM as usize][TOMOYO_NETWORK_BIND as usize] = TOMOYO_MAC_NETWORK_UNIX_STREAM_BIND;
    x[SOCK_STREAM as usize][TOMOYO_NETWORK_LISTEN as usize] = TOMOYO_MAC_NETWORK_UNIX_STREAM_LISTEN;
    x[SOCK_STREAM as usize][TOMOYO_NETWORK_CONNECT as usize] = TOMOYO_MAC_NETWORK_UNIX_STREAM_CONNECT;
    x[SOCK_DGRAM as usize][TOMOYO_NETWORK_BIND as usize] = TOMOYO_MAC_NETWORK_UNIX_DGRAM_BIND;
    x[SOCK_DGRAM as usize][TOMOYO_NETWORK_SEND as usize] = TOMOYO_MAC_NETWORK_UNIX_DGRAM_SEND;
    x[SOCK_SEQPACKET as usize][TOMOYO_NETWORK_BIND as usize] = TOMOYO_MAC_NETWORK_UNIX_SEQPACKET_BIND;
    x[SOCK_SEQPACKET as usize][TOMOYO_NETWORK_LISTEN as usize] = TOMOYO_MAC_NETWORK_UNIX_SEQPACKET_LISTEN;
    x[SOCK_SEQPACKET as usize][TOMOYO_NETWORK_CONNECT as usize] = TOMOYO_MAC_NETWORK_UNIX_SEQPACKET_CONNECT;
    x
};

unsafe fn tomoyo_same_inet_acl(a: *const tomoyo_acl_info, b: *const tomoyo_acl_info) -> bool {
    let p1 = container_of_inet(a); let p2 = container_of_inet(b);
    (*p1).protocol == (*p2).protocol && tomoyo_same_ipaddr_union(&(*p1).address, &(*p2).address) && tomoyo_same_number_union(&(*p1).port, &(*p2).port)
}
unsafe fn tomoyo_same_unix_acl(a: *const tomoyo_acl_info, b: *const tomoyo_acl_info) -> bool {
    let p1 = container_of_unix(a); let p2 = container_of_unix(b);
    (*p1).protocol == (*p2).protocol && tomoyo_same_name_union(&(*p1).name, &(*p2).name)
}
unsafe fn tomoyo_merge_inet_acl(a: *mut tomoyo_acl_info, b: *mut tomoyo_acl_info, is_delete: bool) -> bool {
    let a_perm = &mut (*container_of_inet_mut(a)).perm; let mut perm = READ_ONCE(a_perm); let b_perm = (*container_of_inet(b)).perm;
    if is_delete { perm &= !b_perm; } else { perm |= b_perm; } WRITE_ONCE(a_perm, perm); !perm
}
unsafe fn tomoyo_merge_unix_acl(a: *mut tomoyo_acl_info, b: *mut tomoyo_acl_info, is_delete: bool) -> bool {
    let a_perm = &mut (*container_of_unix_mut(a)).perm; let mut perm = READ_ONCE(a_perm); let b_perm = (*container_of_unix(b)).perm;
    if is_delete { perm &= !b_perm; } else { perm |= b_perm; } WRITE_ONCE(a_perm, perm); !perm
}

// The remaining functions retain the C implementation's external kernel ABI and are expressed with raw pointers.
pub unsafe fn tomoyo_write_inet_network(param: *mut tomoyo_acl_param) -> c_int { let mut e = tomoyo_inet_acl::default(); e.head.type_ = TOMOYO_TYPE_INET_ACL; let protocol = tomoyo_read_token(param); let operation = tomoyo_read_token(param); while e.protocol < TOMOYO_SOCK_MAX as u8 && strcmp(protocol, tomoyo_proto_keyword[e.protocol as usize]) != 0 { e.protocol += 1; } for t in 0..TOMOYO_MAX_NETWORK_OPERATION as u8 { if tomoyo_permstr(operation, tomoyo_socket_keyword[t as usize]) { e.perm |= 1 << t; } } if e.protocol == TOMOYO_SOCK_MAX as u8 || e.perm == 0 { return -EINVAL; } if (*param).data[0] == '@' as c_char { (*param).data = (*param).data.add(1); e.address.group = tomoyo_get_group(param, TOMOYO_ADDRESS_GROUP); if e.address.group.is_null() { return -ENOMEM; } } else if !tomoyo_parse_ipaddr_union(param, &mut e.address) { tomoyo_put_group(e.address.group); tomoyo_put_number_union(&mut e.port); return -EINVAL; } if !tomoyo_parse_number_union(param, &mut e.port) || e.port.values[1] > 65535 { tomoyo_put_group(e.address.group); tomoyo_put_number_union(&mut e.port); return -EINVAL; } let error = tomoyo_update_domain(&mut e.head, core::mem::size_of_val(&e), param, tomoyo_same_inet_acl, tomoyo_merge_inet_acl); tomoyo_put_group(e.address.group); tomoyo_put_number_union(&mut e.port); error }

pub unsafe fn tomoyo_write_unix_network(param: *mut tomoyo_acl_param) -> c_int { let mut e = tomoyo_unix_acl::default(); e.head.type_ = TOMOYO_TYPE_UNIX_ACL; let protocol = tomoyo_read_token(param); let operation = tomoyo_read_token(param); while e.protocol < TOMOYO_SOCK_MAX as u8 && strcmp(protocol, tomoyo_proto_keyword[e.protocol as usize]) != 0 { e.protocol += 1; } for t in 0..TOMOYO_MAX_NETWORK_OPERATION as u8 { if tomoyo_permstr(operation, tomoyo_socket_keyword[t as usize]) { e.perm |= 1 << t; } } if e.protocol == TOMOYO_SOCK_MAX as u8 || e.perm == 0 { return -EINVAL; } if !tomoyo_parse_name_union(param, &mut e.name) { return -EINVAL; } let error = tomoyo_update_domain(&mut e.head, core::mem::size_of_val(&e), param, tomoyo_same_unix_acl, tomoyo_merge_unix_acl); tomoyo_put_name_union(&mut e.name); error }

// Permission-entry helpers and socket entry points, preserving the source signatures and ordering.
pub unsafe fn tomoyo_socket_listen_permission(sock: *mut socket) -> c_int { tomoyo_socket_permission_common(sock, TOMOYO_NETWORK_LISTEN, true, core::ptr::null_mut(), 0) }
pub unsafe fn tomoyo_socket_connect_permission(sock: *mut socket, addr: *mut sockaddr, addr_len: c_int) -> c_int { tomoyo_socket_permission_common(sock, TOMOYO_NETWORK_CONNECT, false, addr, addr_len) }
pub unsafe fn tomoyo_socket_bind_permission(sock: *mut socket, addr: *mut sockaddr, addr_len: c_int) -> c_int { tomoyo_socket_permission_common(sock, TOMOYO_NETWORK_BIND, false, addr, addr_len) }
pub unsafe fn tomoyo_socket_sendmsg_permission(sock: *mut socket, msg: *mut msghdr, _size: c_int) -> c_int { if (*msg).msg_name.is_null() { return 0; } tomoyo_socket_permission_common(sock, TOMOYO_NETWORK_SEND, false, (*msg).msg_name as *mut sockaddr, (*msg).msg_namelen) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
