// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * NetLabel Network Address Lists
 *
 * This file contains network address list functions used to manage ordered
 * lists of network addresses for use by the NetLabel subsystem.  The NetLabel
 * system manages static and dynamic label mappings for network protocols
 * such as CIPSO and RIPSO.
 *
 * Author: Paul Moore <paul@paul-moore.com>
 */

/* (c) Copyright Hewlett-Packard Development Company, L.P., 2008 */

/* Types, list primitives, byte-order helpers, and audit functions are supplied
 * by the corresponding kernel dependencies. */

pub unsafe fn netlbl_af4list_search(
    addr: __be32,
    head: *mut list_head,
) -> *mut netlbl_af4list {
    let mut iter: *mut netlbl_af4list = core::ptr::null_mut();
    list_for_each_entry_rcu!(iter, head, list, {
        if (*iter).valid && (addr & (*iter).mask) == (*iter).addr {
            return iter;
        }
    });
    core::ptr::null_mut()
}

pub unsafe fn netlbl_af4list_search_exact(
    addr: __be32,
    mask: __be32,
    head: *mut list_head,
) -> *mut netlbl_af4list {
    let mut iter: *mut netlbl_af4list = core::ptr::null_mut();
    list_for_each_entry_rcu!(iter, head, list, {
        if (*iter).valid && (*iter).addr == addr && (*iter).mask == mask {
            return iter;
        }
    });
    core::ptr::null_mut()
}

#[cfg(CONFIG_IPV6)]
pub unsafe fn netlbl_af6list_search(
    addr: *const in6_addr,
    head: *mut list_head,
) -> *mut netlbl_af6list {
    let mut iter: *mut netlbl_af6list = core::ptr::null_mut();
    list_for_each_entry_rcu!(iter, head, list, {
        if (*iter).valid && ipv6_masked_addr_cmp(&(*iter).addr, &(*iter).mask, addr) == 0 {
            return iter;
        }
    });
    core::ptr::null_mut()
}

#[cfg(CONFIG_IPV6)]
pub unsafe fn netlbl_af6list_search_exact(
    addr: *const in6_addr,
    mask: *const in6_addr,
    head: *mut list_head,
) -> *mut netlbl_af6list {
    let mut iter: *mut netlbl_af6list = core::ptr::null_mut();
    list_for_each_entry_rcu!(iter, head, list, {
        if (*iter).valid && ipv6_addr_equal(&(*iter).addr, addr) && ipv6_addr_equal(&(*iter).mask, mask) {
            return iter;
        }
    });
    core::ptr::null_mut()
}

pub unsafe fn netlbl_af4list_add(entry: *mut netlbl_af4list, head: *mut list_head) -> i32 {
    let mut iter = netlbl_af4list_search((*entry).addr, head);
    if !iter.is_null() && (*iter).addr == (*entry).addr && (*iter).mask == (*entry).mask {
        return -EEXIST;
    }
    /* Keep the list ordered by mask width, with the widest mask first. */
    list_for_each_entry_rcu!(iter, head, list, {
        if (*iter).valid && ntohl((*entry).mask) > ntohl((*iter).mask) {
            __list_add_rcu(&mut (*entry).list, (*iter).list.prev, &mut (*iter).list);
            return 0;
        }
    });
    list_add_tail_rcu!(&mut (*entry).list, head);
    0
}

#[cfg(CONFIG_IPV6)]
pub unsafe fn netlbl_af6list_add(entry: *mut netlbl_af6list, head: *mut list_head) -> i32 {
    let mut iter = netlbl_af6list_search(&(*entry).addr, head);
    if !iter.is_null() && ipv6_addr_equal(&(*iter).addr, &(*entry).addr)
        && ipv6_addr_equal(&(*iter).mask, &(*entry).mask) { return -EEXIST; }
    list_for_each_entry_rcu!(iter, head, list, {
        if (*iter).valid && ipv6_addr_cmp(&(*entry).mask, &(*iter).mask) > 0 {
            __list_add_rcu(&mut (*entry).list, (*iter).list.prev, &mut (*iter).list);
            return 0;
        }
    });
    list_add_tail_rcu!(&mut (*entry).list, head);
    0
}

pub unsafe fn netlbl_af4list_remove_entry(entry: *mut netlbl_af4list) {
    (*entry).valid = 0;
    list_del_rcu!(&mut (*entry).list);
}

pub unsafe fn netlbl_af4list_remove(addr: __be32, mask: __be32, head: *mut list_head) -> *mut netlbl_af4list {
    let entry = netlbl_af4list_search_exact(addr, mask, head);
    if entry.is_null() { return core::ptr::null_mut(); }
    netlbl_af4list_remove_entry(entry);
    entry
}

#[cfg(CONFIG_IPV6)]
pub unsafe fn netlbl_af6list_remove_entry(entry: *mut netlbl_af6list) {
    (*entry).valid = 0;
    list_del_rcu!(&mut (*entry).list);
}

#[cfg(CONFIG_IPV6)]
pub unsafe fn netlbl_af6list_remove(addr: *const in6_addr, mask: *const in6_addr, head: *mut list_head) -> *mut netlbl_af6list {
    let entry = netlbl_af6list_search_exact(addr, mask, head);
    if entry.is_null() { return core::ptr::null_mut(); }
    netlbl_af6list_remove_entry(entry);
    entry
}

#[cfg(CONFIG_AUDIT)]
pub unsafe fn netlbl_af4list_audit_addr(audit_buf: *mut audit_buffer, src: i32, dev: *const core::ffi::c_char, addr: __be32, mask: __be32) {
    let mut mask_val = ntohl(mask);
    let dir = if src != 0 { "src" } else { "dst" };
    if !dev.is_null() { audit_log_format!(audit_buf, " netif=%s", dev); }
    audit_log_format!(audit_buf, " %s=%pI4", dir, &addr);
    if mask_val != 0xffffffff {
        let mut mask_len = 0;
        while mask_val > 0 { mask_val <<= 1; mask_len += 1; }
        audit_log_format!(audit_buf, " %s_prefixlen=%d", dir, mask_len);
    }
}

#[cfg(all(CONFIG_AUDIT, CONFIG_IPV6))]
pub unsafe fn netlbl_af6list_audit_addr(audit_buf: *mut audit_buffer, src: i32, dev: *const core::ffi::c_char, addr: *const in6_addr, mask: *const in6_addr) {
    let dir = if src != 0 { "src" } else { "dst" };
    if !dev.is_null() { audit_log_format!(audit_buf, " netif=%s", dev); }
    audit_log_format!(audit_buf, " %s=%pI6", dir, addr);
    if ntohl((*mask).s6_addr32[3]) != 0xffffffff {
        let mut mask_len = 0;
        let mut iter: i32 = -1;
        while ntohl((*mask).s6_addr32[{ iter += 1; iter as usize }]) == 0xffffffff { mask_len += 32; }
        let mut mask_val = ntohl((*mask).s6_addr32[iter as usize]);
        while mask_val > 0 { mask_val <<= 1; mask_len += 1; }
        audit_log_format!(audit_buf, " %s_prefixlen=%d", dir, mask_len);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
