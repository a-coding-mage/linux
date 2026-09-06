// SPDX-License-Identifier: GPL-2.0-only
/*
 * Network node table
 *
 * SELinux must keep a mapping of network nodes to labels/SIDs.  This
 * mapping is maintained as part of the normal policy but a fast cache is
 * needed to reduce the lookup overhead since most of these queries happen on
 * a per-packet basis.
 *
 * Author: Paul Moore <paul@paul-moore.com>
 *
 * This code is heavily based on the "netif" concept originally developed by
 * James Morris <jmorris@redhat.com>
 *   (see security/selinux/netif.c for more information)
 */

/*
 * (c) Copyright Hewlett-Packard Development Company, L.P., 2007
 */

/* Dependencies in the original C source:
 * linux/types.h, linux/rcupdate.h, linux/list.h, linux/slab.h,
 * linux/spinlock.h, linux/in.h, linux/in6.h, linux/ip.h, linux/ipv6.h,
 * net/ip.h, net/ipv6.h, initcalls.h, netnode.h, objsec.h
 */

use core::mem::offset_of;
use core::ptr;

type __be32 = u32;
type u16 = u16;
type u32 = u32;

const SEL_NETNODE_HASH_SIZE: usize = 256;
const SEL_NETNODE_HASH_BKT_LIMIT: u32 = 16;
const PF_INET: u16 = 2;
const PF_INET6: u16 = 10;
const GFP_ATOMIC: u32 = 0;
const EINVAL: i32 = 22;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rcu_head {
    pub next: *mut rcu_head,
    pub func: Option<unsafe extern "C" fn(*mut rcu_head)>,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct in_addr {
    pub s_addr: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct in6_addr {
    pub s6_addr32: [__be32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union netnode_security_addr {
    pub ipv4: __be32,
    pub ipv6: in6_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct netnode_security_struct {
    pub addr: netnode_security_addr,
    pub family: u16,
    pub sid: u32,
}

#[repr(C)]
struct sel_netnode_bkt {
    size: u32,
    list: list_head,
}

#[repr(C)]
struct sel_netnode {
    nsec: netnode_security_struct,

    list: list_head,
    rcu: rcu_head,
}

/* NOTE: we are using a combined hash table for both IPv4 and IPv6, the reason
 * for this is that I suspect most users will not make heavy use of both
 * address families at the same time so one table will usually end up wasted,
 * if this becomes a problem we can always add a hash table for each address
 * family later */

static mut sel_netnode_lock: spinlock_t = spinlock_t { _private: [] };
static mut sel_netnode_hash: [sel_netnode_bkt; SEL_NETNODE_HASH_SIZE] =
    [sel_netnode_bkt {
        size: 0,
        list: list_head {
            next: ptr::null_mut(),
            prev: ptr::null_mut(),
        },
    }; SEL_NETNODE_HASH_SIZE];

extern "C" {
    static selinux_enabled_boot: bool;

    fn BUG();
    fn spin_lock_bh(lock: *mut spinlock_t);
    fn spin_unlock_bh(lock: *mut spinlock_t);
    fn rcu_read_lock();
    fn rcu_read_unlock();
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn list_add_rcu(new: *mut list_head, head: *mut list_head);
    fn list_del_rcu(entry: *mut list_head);
    fn kfree(ptr: *mut core::ffi::c_void);
    fn kmalloc(size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn kfree_rcu(ptr: *mut sel_netnode, rcu: *mut rcu_head);
    fn security_node_sid(
        family: u16,
        addr: *const core::ffi::c_void,
        addrlen: usize,
        sid: *mut u32,
    ) -> i32;
    fn ipv6_addr_equal(a1: *const in6_addr, a2: *const in6_addr) -> bool;
    fn pr_warn(fmt: *const u8, ...);
}

unsafe fn likely<T>(x: T) -> T {
    x
}

unsafe fn unlikely<T>(x: T) -> T {
    x
}

unsafe fn list_entry_sel_netnode(ptr: *mut list_head) -> *mut sel_netnode {
    (ptr as *mut u8).sub(offset_of!(sel_netnode, list)) as *mut sel_netnode
}

unsafe fn list_tail_rcu(head: *mut list_head) -> *mut list_head {
    (*head).prev
}

/**
 * sel_netnode_hashfn_ipv4 - IPv4 hashing function for the node table
 * @addr: IPv4 address
 *
 * Description:
 * This is the IPv4 hashing function for the node interface table, it returns
 * the bucket number for the given IP address.
 *
 */
unsafe fn sel_netnode_hashfn_ipv4(addr: __be32) -> u32 {
    /* at some point we should determine if the mismatch in byte order
     * affects the hash function dramatically */
    addr & (SEL_NETNODE_HASH_SIZE as u32 - 1)
}

/**
 * sel_netnode_hashfn_ipv6 - IPv6 hashing function for the node table
 * @addr: IPv6 address
 *
 * Description:
 * This is the IPv6 hashing function for the node interface table, it returns
 * the bucket number for the given IP address.
 *
 */
unsafe fn sel_netnode_hashfn_ipv6(addr: *const in6_addr) -> u32 {
    /* just hash the least significant 32 bits to keep things fast (they
     * are the most likely to be different anyway), we can revisit this
     * later if needed */
    (*addr).s6_addr32[3] & (SEL_NETNODE_HASH_SIZE as u32 - 1)
}

/**
 * sel_netnode_find - Search for a node record
 * @addr: IP address
 * @family: address family
 *
 * Description:
 * Search the network node table and return the record matching @addr.  If an
 * entry can not be found in the table return NULL.
 *
 */
unsafe fn sel_netnode_find(addr: *const core::ffi::c_void, family: u16) -> *mut sel_netnode {
    let idx: u32;

    match family {
        PF_INET => {
            idx = sel_netnode_hashfn_ipv4(*(addr as *const __be32));
        }
        PF_INET6 => {
            idx = sel_netnode_hashfn_ipv6(addr as *const in6_addr);
        }
        _ => {
            BUG();
            return ptr::null_mut();
        }
    }

    let head = &mut sel_netnode_hash[idx as usize].list as *mut list_head;
    let mut pos = (*head).next;
    while pos != head {
        let node = list_entry_sel_netnode(pos);
        if (*node).nsec.family == family {
            match family {
                PF_INET => {
                    if (*node).nsec.addr.ipv4 == *(addr as *const __be32) {
                        return node;
                    }
                }
                PF_INET6 => {
                    if ipv6_addr_equal(&(*node).nsec.addr.ipv6, addr as *const in6_addr) {
                        return node;
                    }
                }
                _ => {}
            }
        }
        pos = (*pos).next;
    }

    ptr::null_mut()
}

/**
 * sel_netnode_insert - Insert a new node into the table
 * @node: the new node record
 *
 * Description:
 * Add a new node record to the network address hash table.
 *
 */
unsafe fn sel_netnode_insert(node: *mut sel_netnode) {
    let idx: u32;

    match (*node).nsec.family {
        PF_INET => {
            idx = sel_netnode_hashfn_ipv4((*node).nsec.addr.ipv4);
        }
        PF_INET6 => {
            idx = sel_netnode_hashfn_ipv6(&(*node).nsec.addr.ipv6);
        }
        _ => {
            BUG();
            return;
        }
    }

    /* we need to impose a limit on the growth of the hash table so check
     * this bucket to make sure it is within the specified bounds */
    list_add_rcu(
        &mut (*node).list,
        &mut sel_netnode_hash[idx as usize].list,
    );
    if sel_netnode_hash[idx as usize].size == SEL_NETNODE_HASH_BKT_LIMIT {
        let tail: *mut sel_netnode;
        tail = list_entry_sel_netnode(list_tail_rcu(
            &mut sel_netnode_hash[idx as usize].list,
        ));
        list_del_rcu(&mut (*tail).list);
        kfree_rcu(tail, &mut (*tail).rcu);
    } else {
        sel_netnode_hash[idx as usize].size += 1;
    }
}

/**
 * sel_netnode_sid_slow - Lookup the SID of a network address using the policy
 * @addr: the IP address
 * @family: the address family
 * @sid: node SID
 *
 * Description:
 * This function determines the SID of a network address by querying the
 * security policy.  The result is added to the network address table to
 * speedup future queries.  Returns zero on success, negative values on
 * failure.
 *
 */
unsafe fn sel_netnode_sid_slow(
    addr: *const core::ffi::c_void,
    family: u16,
    sid: *mut u32,
) -> i32 {
    let ret: i32;
    let mut node: *mut sel_netnode;
    let new: *mut sel_netnode;

    spin_lock_bh(&mut sel_netnode_lock);
    node = sel_netnode_find(addr, family);
    if !node.is_null() {
        *sid = (*node).nsec.sid;
        spin_unlock_bh(&mut sel_netnode_lock);
        return 0;
    }

    /* If this memory allocation fails still return 0. The SID
     * is valid, it just won't be added to the cache.
     */
    new = kmalloc(core::mem::size_of::<sel_netnode>(), GFP_ATOMIC) as *mut sel_netnode;
    match family {
        PF_INET => {
            ret = security_node_sid(
                PF_INET,
                addr,
                core::mem::size_of::<in_addr>(),
                sid,
            );
            if !new.is_null() {
                (*new).nsec.addr.ipv4 = *(addr as *const __be32);
            }
        }
        PF_INET6 => {
            ret = security_node_sid(
                PF_INET6,
                addr,
                core::mem::size_of::<in6_addr>(),
                sid,
            );
            if !new.is_null() {
                (*new).nsec.addr.ipv6 = *(addr as *const in6_addr);
            }
        }
        _ => {
            BUG();
            ret = -EINVAL;
        }
    }
    if ret == 0 && !new.is_null() {
        (*new).nsec.family = family;
        (*new).nsec.sid = *sid;
        sel_netnode_insert(new);
    } else {
        kfree(new as *mut core::ffi::c_void);
    }

    spin_unlock_bh(&mut sel_netnode_lock);
    if unlikely(ret != 0) {
        pr_warn(
            b"SELinux: failure in %s(), unable to determine network node label\n\0".as_ptr(),
            b"sel_netnode_sid_slow\0".as_ptr(),
        );
    }
    ret
}

/**
 * sel_netnode_sid - Lookup the SID of a network address
 * @addr: the IP address
 * @family: the address family
 * @sid: node SID
 *
 * Description:
 * This function determines the SID of a network address using the fastest
 * method possible.  First the address table is queried, but if an entry
 * can't be found then the policy is queried and the result is added to the
 * table to speedup future queries.  Returns zero on success, negative values
 * on failure.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn sel_netnode_sid(
    addr: *const core::ffi::c_void,
    family: u16,
    sid: *mut u32,
) -> i32 {
    let node: *mut sel_netnode;

    rcu_read_lock();
    node = sel_netnode_find(addr, family);
    if likely(!node.is_null()) {
        *sid = (*node).nsec.sid;
        rcu_read_unlock();
        return 0;
    }
    rcu_read_unlock();

    sel_netnode_sid_slow(addr, family, sid)
}

/**
 * sel_netnode_flush - Flush the entire network address table
 *
 * Description:
 * Remove all entries from the network address table.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn sel_netnode_flush() {
    let mut idx: u32;
    let mut node: *mut sel_netnode;
    let mut node_tmp: *mut sel_netnode;

    spin_lock_bh(&mut sel_netnode_lock);
    idx = 0;
    while idx < SEL_NETNODE_HASH_SIZE as u32 {
        let head = &mut sel_netnode_hash[idx as usize].list as *mut list_head;
        let mut pos = (*head).next;
        while pos != head {
            node = list_entry_sel_netnode(pos);
            node_tmp = if (*pos).next != head {
                list_entry_sel_netnode((*pos).next)
            } else {
                ptr::null_mut()
            };
            list_del_rcu(&mut (*node).list);
            kfree_rcu(node, &mut (*node).rcu);
            node = node_tmp;
            pos = if !node.is_null() {
                &mut (*node).list
            } else {
                head
            };
        }
        sel_netnode_hash[idx as usize].size = 0;
        idx += 1;
    }
    spin_unlock_bh(&mut sel_netnode_lock);
}

#[no_mangle]
pub unsafe extern "C" fn sel_netnode_init() -> i32 {
    let mut iter: i32;

    if !selinux_enabled_boot {
        return 0;
    }

    iter = 0;
    while iter < SEL_NETNODE_HASH_SIZE as i32 {
        INIT_LIST_HEAD(&mut sel_netnode_hash[iter as usize].list);
        sel_netnode_hash[iter as usize].size = 0;
        iter += 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
