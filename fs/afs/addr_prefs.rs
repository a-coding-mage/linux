// SPDX-License-Identifier: GPL-2.0-or-later
/* Address preferences management
 *
 * Copyright (C) 2023 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Translated from the retained addr_prefs.c. The configured C headers own
// every type layout; the helper object contains only C macro/inline boundaries.
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code,
    missing_docs, unsafe_op_in_unsafe_fn, clippy::all, unreachable_pub)]
#[allow(improper_ctypes)]
mod bindings {
    use kernel::ffi;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/afs_addr_prefs_generated.rs"));
}
use bindings::*;
use core::mem::{size_of, zeroed};
use core::ptr::{addr_of, addr_of_mut, copy, copy_nonoverlapping, null, null_mut,
    write_bytes};
use kernel::ffi::{c_char, c_int, c_uint, c_ulong};

#[inline]
fn neg(error: u32) -> c_int { -(error as c_int) }

#[inline]
unsafe fn warn(message: &'static [u8]) {
    rust_afs_addr_prefs_warn(message.as_ptr().cast());
}

// Address the generated flexible-array field without making a Rust reference
// to a shared enclosing object or assuming its offset, size or alignment.
#[inline]
unsafe fn prefs(list: *const afs_addr_preference_list) -> *const afs_addr_preference {
    addr_of!((*list).prefs).cast()
}

#[inline]
unsafe fn prefs_mut(list: *mut afs_addr_preference_list) -> *mut afs_addr_preference {
    addr_of_mut!((*list).prefs).cast()
}

#[inline]
unsafe fn addresses(list: *mut afs_addr_list) -> *mut afs_address {
    addr_of_mut!((*list).addrs).cast()
}

/* Split a NUL-terminated string up to the first newline around spaces. */
unsafe fn afs_split_string(pbuf: *mut *mut c_char, strv: *mut *mut c_char,
    mut maxstrv: c_uint) -> c_int {
    let mut count: c_uint = 0;
    let mut p = *pbuf;
    maxstrv = maxstrv.wrapping_sub(1); // Allow for terminal NULL.
    loop {
        while rust_afs_addr_prefs_isspace(*p) {
            if *p == b'\n' as c_char {
                p = p.add(1);
                break;
            }
            p = p.add(1);
        }
        if *p == 0 { break; }

        if count >= maxstrv {
            warn(b"Too many elements in string\n\0");
            return neg(EINVAL);
        }
        *strv.add(count as usize) = p;
        count += 1;

        while !rust_afs_addr_prefs_isspace(*p) && *p != 0 { p = p.add(1); }
        if *p == 0 { break; }
        if *p == b'\n' as c_char {
            *p = 0;
            p = p.add(1);
            break;
        }
        *p = 0;
        p = p.add(1);
    }
    *pbuf = p;
    *strv.add(count as usize) = null_mut();
    count as c_int
}

/* Parse an address with an optional subnet mask. */
unsafe fn afs_parse_address(mut p: *mut c_char, pref: *mut afs_addr_preference) -> c_int {
    let end = p.add(strlen(p) as usize);
    let mut stop: *const c_char = null();
    let mut bracket = false;
    let mut mask: c_ulong;
    if *p == b'[' as c_char {
        p = p.add(1);
        bracket = true;
    }
    if in4_pton(p, end.offset_from(p) as c_int,
        rust_afs_addr_prefs_ipv4(pref).cast(), -1, addr_of_mut!(stop)) != 0 {
        (*pref).family = AF_INET as _;
        mask = 32;
    } else if in6_pton(p, end.offset_from(p) as c_int,
        rust_afs_addr_prefs_ipv6(pref).cast(), -1, addr_of_mut!(stop)) != 0 {
        (*pref).family = AF_INET6 as _;
        mask = 128;
    } else {
        warn(b"Can't determine address family\n\0");
        return neg(EINVAL);
    }

    p = stop.cast_mut();
    if bracket {
        if *p != b']' as c_char {
            warn(b"Can't find closing ']'\n\0");
            return neg(EINVAL);
        }
        p = p.add(1);
    }
    if *p == b'/' as c_char {
        p = p.add(1);
        let tmp = simple_strtoul(p, addr_of_mut!(p), 10);
        if tmp > mask {
            warn(b"Subnet mask too large\n\0");
            return neg(EINVAL);
        }
        if tmp == 0 {
            warn(b"Subnet mask too small\n\0");
            return neg(EINVAL);
        }
        mask = tmp;
    }
    if *p != 0 {
        warn(b"Invalid address\n\0");
        return neg(EINVAL);
    }
    (*pref).subnet_mask = mask as _;
    0
}

enum cmp_ret { CONTINUE_SEARCH, INSERT_HERE, EXACT_MATCH, SUBNET_MATCH }

/* See if a candidate address matches a listed address. */
unsafe fn afs_cmp_address_pref(a: *const afs_addr_preference,
    b: *const afs_addr_preference) -> cmp_ret {
    let mut subnet = core::cmp::min((*a).subnet_mask, (*b).subnet_mask) as c_int;
    if (*a).family != (*b).family { return cmp_ret::INSERT_HERE; }
    let (mut pa, mut pb) = if (*a).family as u32 == AF_INET6 {
        (rust_afs_addr_prefs_ipv6_words(a), rust_afs_addr_prefs_ipv6_words(b))
    } else {
        (rust_afs_addr_prefs_ipv4_words(a), rust_afs_addr_prefs_ipv4_words(b))
    };
    while subnet > 32 {
        // C subtracts unsigned 32-bit values, then converts the result to int.
        // Widening before subtraction changes the retained C's ordering.
        let diff = u32::from_be(*pa).wrapping_sub(u32::from_be(*pb)) as c_int;
        pa = pa.add(1);
        pb = pb.add(1);
        if diff < 0 { return cmp_ret::INSERT_HERE; }
        if diff > 0 { return cmp_ret::CONTINUE_SEARCH; }
        subnet -= 32;
    }
    if subnet == 0 { return cmp_ret::EXACT_MATCH; }
    let mask = 0xffff_ffffu32 << (32 - subnet);
    let na = u32::from_be(*pa);
    let nb = u32::from_be(*pb);
    let diff = (na & mask).wrapping_sub(nb & mask) as c_int;
    if diff < 0 { return cmp_ret::INSERT_HERE; }
    if diff > 0 { return cmp_ret::CONTINUE_SEARCH; }
    if (*a).subnet_mask == (*b).subnet_mask { cmp_ret::EXACT_MATCH }
    else if (*a).subnet_mask > (*b).subnet_mask { cmp_ret::SUBNET_MATCH }
    else { cmp_ret::CONTINUE_SEARCH }
}

/* Insert an address preference into the writer's private candidate. */
unsafe fn afs_insert_address_pref(preflistp: *mut *mut afs_addr_preference_list,
    pref: *const afs_addr_preference, index: c_int) -> c_int {
    let mut preflist = *preflistp;
    let old = preflist;
    rust_afs_addr_prefs_enter(b"afs_insert_address_pref\0".as_ptr().cast(),
        preflist, index);
    if (*preflist).nr == 255 { return neg(ENOSPC); }
    let index = index as usize;
    if (*preflist).nr >= (*preflist).max_prefs {
        let size = rust_afs_addr_prefs_roundup_pow_of_two(
            rust_afs_addr_prefs_struct_size((*preflist).max_prefs as usize + 1));
        let max_prefs = core::cmp::min(
            (size - size_of::<afs_addr_preference_list>())
                / size_of::<afs_addr_preference>(), 255);
        preflist = rust_afs_addr_prefs_kmalloc(size);
        if preflist.is_null() { return neg(ENOMEM); }
        copy_nonoverlapping(old, preflist, 1);
        (*preflist).max_prefs = max_prefs as _;
        *preflistp = preflist;
        if index < (*preflist).nr as usize {
            copy_nonoverlapping(prefs(old).add(index), prefs_mut(preflist).add(index + 1),
                (*preflist).nr as usize - index);
        }
        if index > 0 {
            copy_nonoverlapping(prefs(old), prefs_mut(preflist), index);
        }
        // The retained C does not free this replaced private allocation.
        // Preserve that lifetime here; the separate baseline audit records it.
    } else if index < (*preflist).nr as usize {
        copy(prefs(preflist).add(index), prefs_mut(preflist).add(index + 1),
            (*preflist).nr as usize - index);
    }
    copy_nonoverlapping(pref, prefs_mut(preflist).add(index), 1);
    (*preflist).nr += 1;
    if (*pref).family as u32 == AF_INET { (*preflist).ipv6_off += 1; }
    0
}

/* echo "add udp <IP>[/<mask>] <prior>" >/proc/fs/afs/addr_prefs */
unsafe fn afs_add_address_pref(_net: *mut afs_net,
    preflistp: *mut *mut afs_addr_preference_list,
    argc: c_int, argv: *mut *mut c_char) -> c_int {
    let preflist = *preflistp;
    let mut pref: afs_addr_preference = zeroed();
    if argc != 3 {
        warn(b"Wrong number of params\n\0");
        return neg(EINVAL);
    }
    if strcmp(*argv, b"udp\0".as_ptr().cast()) != 0 {
        warn(b"Unsupported protocol\n\0");
        return neg(EINVAL);
    }
    let ret = afs_parse_address(*argv.add(1), addr_of_mut!(pref));
    if ret < 0 { return ret; }
    let ret = kstrtou16(*argv.add(2), 10, addr_of_mut!(pref.prio));
    if ret < 0 {
        warn(b"Invalid priority\n\0");
        return ret;
    }
    let (mut i, stop) = if pref.family as u32 == AF_INET {
        (0, (*preflist).ipv6_off as usize)
    } else {
        ((*preflist).ipv6_off as usize, (*preflist).nr as usize)
    };
    while i < stop {
        match afs_cmp_address_pref(addr_of!(pref), prefs(preflist).add(i)) {
            cmp_ret::CONTINUE_SEARCH => {}
            cmp_ret::INSERT_HERE | cmp_ret::SUBNET_MATCH =>
                return afs_insert_address_pref(preflistp, addr_of!(pref), i as c_int),
            cmp_ret::EXACT_MATCH => {
                (*prefs_mut(preflist).add(i)).prio = pref.prio;
                return 0;
            }
        }
        i += 1;
    }
    afs_insert_address_pref(preflistp, addr_of!(pref), i as c_int)
}

unsafe fn afs_delete_address_pref(preflistp: *mut *mut afs_addr_preference_list,
    index: c_int) -> c_int {
    let preflist = *preflistp;
    rust_afs_addr_prefs_enter(b"afs_delete_address_pref\0".as_ptr().cast(),
        preflist, index);
    if (*preflist).nr == 0 { return neg(ENOENT); }
    let index = index as usize;
    if index < (*preflist).nr as usize - 1 {
        copy(prefs(preflist).add(index + 1), prefs_mut(preflist).add(index),
            (*preflist).nr as usize - index - 1);
    }
    if index < (*preflist).ipv6_off as usize { (*preflist).ipv6_off -= 1; }
    (*preflist).nr -= 1;
    0
}

/* echo "del udp <IP>[/<mask>]" >/proc/fs/afs/addr_prefs */
unsafe fn afs_del_address_pref(_net: *mut afs_net,
    preflistp: *mut *mut afs_addr_preference_list,
    argc: c_int, argv: *mut *mut c_char) -> c_int {
    let preflist = *preflistp;
    let mut pref: afs_addr_preference = zeroed();
    if argc != 2 {
        warn(b"Wrong number of params\n\0");
        return neg(EINVAL);
    }
    if strcmp(*argv, b"udp\0".as_ptr().cast()) != 0 {
        warn(b"Unsupported protocol\n\0");
        return neg(EINVAL);
    }
    let ret = afs_parse_address(*argv.add(1), addr_of_mut!(pref));
    if ret < 0 { return ret; }
    let (mut i, stop) = if pref.family as u32 == AF_INET {
        (0, (*preflist).ipv6_off as usize)
    } else {
        ((*preflist).ipv6_off as usize, (*preflist).nr as usize)
    };
    while i < stop {
        match afs_cmp_address_pref(addr_of!(pref), prefs(preflist).add(i)) {
            cmp_ret::CONTINUE_SEARCH => {}
            cmp_ret::INSERT_HERE | cmp_ret::SUBNET_MATCH => return 0,
            cmp_ret::EXACT_MATCH => return afs_delete_address_pref(preflistp, i as c_int),
        }
        i += 1;
    }
    neg(ENOANO)
}

/* Handle writes to /proc/fs/afs/addr_prefs. */
#[no_mangle]
pub unsafe extern "C" fn afs_proc_addr_prefs_write(file: *mut file,
    mut buf: *mut c_char, _size: usize) -> c_int {
    let net = rust_afs_addr_prefs_file_net(file);
    let mut argv: [*mut c_char; 5] = [null_mut(); 5];
    rust_afs_addr_prefs_inode_lock(file);
    let old = rust_afs_addr_prefs_deref_locked(net, file);
    let initial_count = if old.is_null() { 1 } else { (*old).nr as usize + 1 };
    let psize = rust_afs_addr_prefs_roundup_pow_of_two(
        rust_afs_addr_prefs_struct_size(initial_count));
    let max_prefs = core::cmp::min(
        (psize - size_of::<afs_addr_preference_list>())
            / size_of::<afs_addr_preference>(), 255);
    let mut preflist = rust_afs_addr_prefs_kmalloc_flex(max_prefs);
    let ret = if preflist.is_null() {
        neg(ENOMEM)
    } else {
        if !old.is_null() {
            copy_nonoverlapping(old.cast::<u8>(), preflist.cast::<u8>(),
                rust_afs_addr_prefs_struct_size((*old).nr as usize));
        } else {
            write_bytes(preflist.cast::<u8>(), 0, size_of::<afs_addr_preference_list>());
        }
        (*preflist).max_prefs = max_prefs as _;
        let ret = loop {
            let argc = afs_split_string(addr_of_mut!(buf), argv.as_mut_ptr(),
                argv.len() as c_uint);
            if argc < 0 { break argc; }
            if argc < 2 {
                warn(b"Invalid Command\n\0");
                break neg(EINVAL);
            }
            let ret = if strcmp(argv[0], b"add\0".as_ptr().cast()) == 0 {
                afs_add_address_pref(net, addr_of_mut!(preflist), argc - 1,
                    argv.as_mut_ptr().add(1))
            } else if strcmp(argv[0], b"del\0".as_ptr().cast()) == 0 {
                afs_del_address_pref(net, addr_of_mut!(preflist), argc - 1,
                    argv.as_mut_ptr().add(1))
            } else {
                warn(b"Invalid Command\n\0");
                break neg(EINVAL);
            };
            if ret < 0 { break ret; }
            if *buf == 0 { break 0; }
        };
        if ret == 0 {
            (*preflist).version = (*preflist).version.wrapping_add(1);
            rust_afs_addr_prefs_assign(net, preflist);
            // Publish prefs before publishing the version.
            rust_afs_addr_prefs_store_release_u16(
                addr_of_mut!((*net).address_pref_version), (*preflist).version);
            rust_afs_addr_prefs_free_rcu(old);
            preflist = null_mut();
        }
        ret
    };
    kfree(preflist.cast());
    rust_afs_addr_prefs_inode_unlock(file);
    rust_afs_addr_prefs_leave(ret);
    ret
}

/* Caller holds the RCU read lock; the published preference list is immutable. */
#[no_mangle]
pub unsafe extern "C" fn afs_get_address_preferences_rcu(net: *mut afs_net,
    alist: *mut afs_addr_list) {
    let preflist = rust_afs_addr_prefs_deref(net);
    if preflist.is_null() || (*preflist).nr == 0 || (*alist).nr_addrs == 0
        || rust_afs_addr_prefs_load_acquire_uint(addr_of!((*alist).addr_pref_version))
            == (*preflist).version as c_uint {
        return;
    }
    let mut test: afs_addr_preference = zeroed();
    test.family = AF_INET as _;
    test.subnet_mask = 32;
    test.prio = 0;
    let mut i = 0;
    while i < (*alist).nr_ipv4 as usize {
        let address = addresses(alist).add(i);
        let sin = rxrpc_kernel_remote_addr((*address).peer).cast::<sockaddr_in>();
        copy_nonoverlapping(addr_of!((*sin).sin_addr),
            rust_afs_addr_prefs_ipv4(addr_of_mut!(test)), 1);
        for j in 0..(*preflist).ipv6_off as usize {
            let pref = prefs(preflist).add(j);
            match afs_cmp_address_pref(addr_of!(test), pref) {
                cmp_ret::CONTINUE_SEARCH | cmp_ret::INSERT_HERE => {}
                cmp_ret::EXACT_MATCH | cmp_ret::SUBNET_MATCH => {
                    rust_afs_addr_prefs_write_once_u16(addr_of_mut!((*address).prio),
                        (*pref).prio);
                }
            }
            // C's break exits only the switch, not the enclosing for loop.
        }
        i += 1;
    }
    test.family = AF_INET6 as _;
    test.subnet_mask = 128;
    test.prio = 0;
    while i < (*alist).nr_addrs as usize {
        let address = addresses(alist).add(i);
        let sin6 = rxrpc_kernel_remote_addr((*address).peer).cast::<sockaddr_in6>();
        copy_nonoverlapping(addr_of!((*sin6).sin6_addr),
            rust_afs_addr_prefs_ipv6(addr_of_mut!(test)), 1);
        for j in (*preflist).ipv6_off as usize..(*preflist).nr as usize {
            let pref = prefs(preflist).add(j);
            match afs_cmp_address_pref(addr_of!(test), pref) {
                cmp_ret::CONTINUE_SEARCH | cmp_ret::INSERT_HERE => {}
                cmp_ret::EXACT_MATCH | cmp_ret::SUBNET_MATCH => {
                    rust_afs_addr_prefs_write_once_u16(addr_of_mut!((*address).prio),
                        (*pref).prio);
                }
            }
        }
        i += 1;
    }
    rust_afs_addr_prefs_store_release_uint(addr_of_mut!((*alist).addr_pref_version),
        (*preflist).version as c_uint);
}

/* Avoid taking the RCU read lock if the address preferences are unchanged. */
#[no_mangle]
pub unsafe extern "C" fn afs_get_address_preferences(net: *mut afs_net,
    alist: *mut afs_addr_list) {
    // These two C fast-path loads can race with publication/update. Keep them
    // at raw LKMM boundaries; ordinary Rust loads would be data races.
    if rust_afs_addr_prefs_access(net).is_null()
        || rust_afs_addr_prefs_load_acquire_u16(addr_of!((*net).address_pref_version))
            as c_uint == rust_afs_addr_prefs_read_once_uint(addr_of!((*alist).addr_pref_version)) {
        return;
    }
    rust_afs_addr_prefs_rcu_read_lock();
    afs_get_address_preferences_rcu(net, alist);
    rust_afs_addr_prefs_rcu_read_unlock();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
