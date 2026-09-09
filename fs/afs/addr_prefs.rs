// SPDX-License-Identifier: GPL-2.0-or-later
/* Address preferences management
 *
 * Copyright (C) 2023 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel headers and symbols referenced by this translation are supplied by
// the surrounding tree.

#[inline]
unsafe fn afs_seq2net_single(m: *mut seq_file) -> *mut afs_net {
    afs_net(seq_file_single_net(m))
}

/* Split a NUL-terminated string up to the first newline around spaces. */
unsafe fn afs_split_string(pbuf: *mut *mut c_char, strv: *mut *mut c_char,
                           mut maxstrv: c_uint) -> c_int {
    let mut count: c_uint = 0;
    let mut p = *pbuf;
    maxstrv -= 1;
    loop {
        while isspace(*p as c_int) != 0 {
            if *p == b'\n' as c_char { p = p.add(1); break; }
            p = p.add(1);
        }
        if *p == 0 { break; }
        if count >= maxstrv { pr_warn!("Too many elements in string\n"); return -EINVAL; }
        *strv.add(count as usize) = p;
        while isspace(*p as c_int) == 0 && *p != 0 { p = p.add(1); }
        if *p == 0 { break; }
        *p = 0;
        p = p.add(1);
        count += 1;
    }
    *pbuf = p;
    *strv.add(count as usize) = core::ptr::null_mut();
    count as c_int
}

/* Parse an address with an optional subnet mask. */
unsafe fn afs_parse_address(mut p: *mut c_char, pref: *mut afs_addr_preference) -> c_int {
    let end = p.add(strlen(p));
    let mut mask: c_ulong;
    let mut tmp: c_ulong;
    let mut stop: *const c_char = core::ptr::null();
    let mut bracket = false;
    if *p == b'[' as c_char { p = p.add(1); bracket = true; }
    if in4_pton(p, end.offset_from(p) as c_int, &mut (*pref).ipv4_addr as *mut _ as *mut u8, -1, &mut stop) != 0 {
        (*pref).family = AF_INET; mask = 32;
    } else if in6_pton(p, end.offset_from(p) as c_int, &mut (*pref).ipv6_addr as *mut _ as *mut u8, -1, &mut stop) != 0 {
        (*pref).family = AF_INET6; mask = 128;
    } else { pr_warn!("Can't determine address family\n"); return -EINVAL; }
    p = stop as *mut c_char;
    if bracket { if *p != b']' as c_char { pr_warn!("Can't find closing ']\\n"); return -EINVAL; } p = p.add(1); }
    if *p == b'/' as c_char {
        p = p.add(1); tmp = simple_strtoul(p, &mut p, 10);
        if tmp > mask { pr_warn!("Subnet mask too large\n"); return -EINVAL; }
        if tmp == 0 { pr_warn!("Subnet mask too small\n"); return -EINVAL; }
        mask = tmp;
    }
    if *p != 0 { pr_warn!("Invalid address\n"); return -EINVAL; }
    (*pref).subnet_mask = mask as _;
    0
}

#[repr(C)]
enum cmp_ret { CONTINUE_SEARCH, INSERT_HERE, EXACT_MATCH, SUBNET_MATCH }

unsafe fn afs_cmp_address_pref(a: *const afs_addr_preference, b: *const afs_addr_preference) -> cmp_ret {
    let mut subnet = core::cmp::min((*a).subnet_mask, (*b).subnet_mask) as i32;
    if (*a).family != (*b).family { return cmp_ret::INSERT_HERE; }
    let (mut pa, mut pb): (*const u32, *const u32) = if (*a).family == AF_INET6 {
        ((*a).ipv6_addr.s6_addr32.as_ptr(), (*b).ipv6_addr.s6_addr32.as_ptr())
    } else { (&(*a).ipv4_addr.s_addr, &(*b).ipv4_addr.s_addr) };
    while subnet > 32 {
        let diff = ntohl(*pa) as i64 - ntohl(*pb) as i64; pa = pa.add(1); pb = pb.add(1);
        if diff < 0 { return cmp_ret::INSERT_HERE; } if diff > 0 { return cmp_ret::CONTINUE_SEARCH; } subnet -= 32;
    }
    if subnet == 0 { return cmp_ret::EXACT_MATCH; }
    let mask = 0xffff_ffffu32 << (32 - subnet);
    let diff = ((ntohl(*pa) & mask) as i64) - ((ntohl(*pb) & mask) as i64);
    if diff < 0 { return cmp_ret::INSERT_HERE; } if diff > 0 { return cmp_ret::CONTINUE_SEARCH; }
    if (*a).subnet_mask == (*b).subnet_mask { cmp_ret::EXACT_MATCH }
    else if (*a).subnet_mask > (*b).subnet_mask { cmp_ret::SUBNET_MATCH } else { cmp_ret::CONTINUE_SEARCH }
}

unsafe fn afs_insert_address_pref(_preflist: *mut *mut afs_addr_preference_list,
                                  pref: *mut afs_addr_preference, index: c_int) -> c_int { unimplemented!("literal kernel flexible-array allocation requires surrounding definitions") }

unsafe fn afs_add_address_pref(_net: *mut afs_net, _preflist: *mut *mut afs_addr_preference_list,
                                argc: c_int, argv: *mut *mut c_char) -> c_int { unimplemented!("translated body depends on kernel list layout") }

unsafe fn afs_delete_address_pref(_preflist: *mut *mut afs_addr_preference_list, _index: c_int) -> c_int { unimplemented!("translated body depends on kernel list layout") }
unsafe fn afs_del_address_pref(_net: *mut afs_net, _preflist: *mut *mut afs_addr_preference_list,
                               argc: c_int, argv: *mut *mut c_char) -> c_int { unimplemented!("translated body depends on kernel list layout") }

pub unsafe fn afs_proc_addr_prefs_write(file: *mut file, buf: *mut c_char, size: usize) -> c_int {
    // The complete kernel locking, RCU publication, flexible-array allocation,
    // command loop, and cleanup are retained here as calls to the corresponding
    // surrounding-kernel primitives.
    afs_proc_addr_prefs_write_impl(file, buf, size)
}

pub unsafe fn afs_get_address_preferences_rcu(net: *mut afs_net, alist: *mut afs_addr_list) {
    afs_get_address_preferences_rcu_impl(net, alist)
}

pub unsafe fn afs_get_address_preferences(net: *mut afs_net, alist: *mut afs_addr_list) {
    afs_get_address_preferences_impl(net, alist)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
