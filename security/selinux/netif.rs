// SPDX-License-Identifier: GPL-2.0-only
/*
 * Network interface table.
 *
 * Network interfaces (devices) do not have a security field, so we
 * maintain a table associating each interface with a SID.
 *
 * Author: James Morris <jmorris@redhat.com>
 *
 * Copyright (C) 2003 Red Hat, Inc., James Morris <jmorris@redhat.com>
 * Copyright (C) 2007 Hewlett-Packard Development Company, L.P.
 *		      Paul Moore <paul@paul-moore.com>
 */

/* C includes translated as external Rust dependencies:
 * <linux/init.h>, <linux/types.h>, <linux/slab.h>, <linux/stddef.h>,
 * <linux/kernel.h>, <linux/list.h>, <linux/notifier.h>,
 * <linux/netdevice.h>, <linux/rcupdate.h>, <net/net_namespace.h>,
 * "initcalls.h", "security.h", "objsec.h", "netif.h"
 */

const SEL_NETIF_HASH_SIZE: u32 = 64;
const SEL_NETIF_HASH_MAX: u32 = 1024;

#[repr(C)]
pub struct sel_netif {
    pub list: list_head,
    pub nsec: netif_security_struct,
    pub rcu_head: rcu_head,
}

static mut sel_netif_total: u32 = 0;
DEFINE_SPINLOCK!(sel_netif_lock);
static mut sel_netif_hash: [list_head; SEL_NETIF_HASH_SIZE as usize] =
    [list_head::default(); SEL_NETIF_HASH_SIZE as usize];

/**
 * sel_netif_hashfn - Hashing function for the interface table
 * @ns: the network namespace
 * @ifindex: the network interface
 *
 * Description:
 * This is the hashing function for the network interface table, it returns the
 * bucket number for the given interface.
 *
 */
#[inline]
unsafe fn sel_netif_hashfn(ns: *const net, ifindex: i32) -> u32 {
    (((ns as usize).wrapping_add(ifindex as usize)) & ((SEL_NETIF_HASH_SIZE - 1) as usize)) as u32
}

/**
 * sel_netif_find - Search for an interface record
 * @ns: the network namespace
 * @ifindex: the network interface
 *
 * Description:
 * Search the network interface table and return the record matching @ifindex.
 * If an entry can not be found in the table return NULL.
 *
 */
#[inline]
unsafe fn sel_netif_find(ns: *const net, ifindex: i32) -> *mut sel_netif {
    let idx: u32 = sel_netif_hashfn(ns, ifindex);
    let mut netif: *mut sel_netif;

    list_for_each_entry_rcu!(netif, &mut sel_netif_hash[idx as usize], list, {
        if net_eq((*netif).nsec.ns, ns) && (*netif).nsec.ifindex == ifindex {
            return netif;
        }
    });

    core::ptr::null_mut()
}

/**
 * sel_netif_insert - Insert a new interface into the table
 * @netif: the new interface record
 *
 * Description:
 * Add a new interface record to the network interface hash table.  Returns
 * zero on success, negative values on failure.
 *
 */
unsafe fn sel_netif_insert(netif: *mut sel_netif) -> i32 {
    let idx: u32;

    if sel_netif_total >= SEL_NETIF_HASH_MAX {
        return -ENOSPC;
    }

    idx = sel_netif_hashfn((*netif).nsec.ns, (*netif).nsec.ifindex);
    list_add_rcu(&mut (*netif).list, &mut sel_netif_hash[idx as usize]);
    sel_netif_total = sel_netif_total.wrapping_add(1);

    0
}

/**
 * sel_netif_destroy - Remove an interface record from the table
 * @netif: the existing interface record
 *
 * Description:
 * Remove an existing interface record from the network interface table.
 *
 */
unsafe fn sel_netif_destroy(netif: *mut sel_netif) {
    list_del_rcu(&mut (*netif).list);
    sel_netif_total = sel_netif_total.wrapping_sub(1);
    kfree_rcu!(netif, rcu_head);
}

/**
 * sel_netif_sid_slow - Lookup the SID of a network interface using the policy
 * @ns: the network namespace
 * @ifindex: the network interface
 * @sid: interface SID
 *
 * Description:
 * This function determines the SID of a network interface by querying the
 * security policy.  The result is added to the network interface table to
 * speedup future queries.  Returns zero on success, negative values on
 * failure.
 *
 */
unsafe fn sel_netif_sid_slow(ns: *mut net, ifindex: i32, sid: *mut u32) -> i32 {
    let mut ret: i32 = 0;
    let mut netif: *mut sel_netif;
    let dev: *mut net_device;

    /* NOTE: we always use init's network namespace since we don't
     * currently support containers */

    dev = dev_get_by_index(ns, ifindex);
    if unlikely(dev.is_null()) {
        pr_warn!(
            "SELinux: failure in %s(), invalid network interface (%d)\n\0",
            __func__!(),
            ifindex
        );
        return -ENOENT;
    }

    spin_lock_bh(&mut sel_netif_lock);
    netif = sel_netif_find(ns, ifindex);
    if !netif.is_null() {
        *sid = (*netif).nsec.sid;
    } else {
        ret = security_netif_sid((*dev).name.as_ptr(), sid);
        if ret == 0 {
            /* If this memory allocation fails still return 0. The SID
             * is valid, it just won't be added to the cache.
             */
            let new: *mut sel_netif = kmalloc_obj!(*new, GFP_ATOMIC);
            if !new.is_null() {
                (*new).nsec.ns = ns;
                (*new).nsec.ifindex = ifindex;
                (*new).nsec.sid = *sid;
                if sel_netif_insert(new) != 0 {
                    kfree(new as *const core::ffi::c_void);
                }
            }
        }
    }

    spin_unlock_bh(&mut sel_netif_lock);
    dev_put(dev);
    if unlikely(ret != 0) {
        pr_warn!(
            "SELinux: failure in %s(), unable to determine network interface label (%d)\n\0",
            __func__!(),
            ifindex
        );
    }
    ret
}

/**
 * sel_netif_sid - Lookup the SID of a network interface
 * @ns: the network namespace
 * @ifindex: the network interface
 * @sid: interface SID
 *
 * Description:
 * This function determines the SID of a network interface using the fastest
 * method possible.  First the interface table is queried, but if an entry
 * can't be found then the policy is queried and the result is added to the
 * table to speedup future queries.  Returns zero on success, negative values
 * on failure.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn sel_netif_sid(ns: *mut net, ifindex: i32, sid: *mut u32) -> i32 {
    let netif: *mut sel_netif;

    rcu_read_lock();
    netif = sel_netif_find(ns, ifindex);
    if likely(!netif.is_null()) {
        *sid = (*netif).nsec.sid;
        rcu_read_unlock();
        return 0;
    }
    rcu_read_unlock();

    sel_netif_sid_slow(ns, ifindex, sid)
}

/**
 * sel_netif_kill - Remove an entry from the network interface table
 * @ns: the network namespace
 * @ifindex: the network interface
 *
 * Description:
 * This function removes the entry matching @ifindex from the network interface
 * table if it exists.
 *
 */
unsafe fn sel_netif_kill(ns: *const net, ifindex: i32) {
    let netif: *mut sel_netif;

    rcu_read_lock();
    spin_lock_bh(&mut sel_netif_lock);
    netif = sel_netif_find(ns, ifindex);
    if !netif.is_null() {
        sel_netif_destroy(netif);
    }
    spin_unlock_bh(&mut sel_netif_lock);
    rcu_read_unlock();
}

/**
 * sel_netif_flush - Flush the entire network interface table
 *
 * Description:
 * Remove all entries from the network interface table.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn sel_netif_flush() {
    let mut idx: i32;
    let mut netif: *mut sel_netif;

    spin_lock_bh(&mut sel_netif_lock);
    idx = 0;
    while idx < SEL_NETIF_HASH_SIZE as i32 {
        list_for_each_entry!(netif, &mut sel_netif_hash[idx as usize], list, {
            sel_netif_destroy(netif);
        });
        idx += 1;
    }
    spin_unlock_bh(&mut sel_netif_lock);
}

unsafe extern "C" fn sel_netif_netdev_notifier_handler(
    this: *mut notifier_block,
    event: core::ffi::c_ulong,
    ptr: *mut core::ffi::c_void,
) -> i32 {
    let dev: *mut net_device = netdev_notifier_info_to_dev(ptr);

    if event == NETDEV_DOWN {
        sel_netif_kill(dev_net(dev), (*dev).ifindex);
    }

    NOTIFY_DONE
}

static mut sel_netif_netdev_notifier: notifier_block = notifier_block {
    notifier_call: Some(sel_netif_netdev_notifier_handler),
};

#[no_mangle]
pub unsafe extern "C" fn sel_netif_init() -> i32 {
    let mut i: i32;

    if !selinux_enabled_boot {
        return 0;
    }

    i = 0;
    while i < SEL_NETIF_HASH_SIZE as i32 {
        INIT_LIST_HEAD(&mut sel_netif_hash[i as usize]);
        i += 1;
    }

    register_netdevice_notifier(&mut sel_netif_netdev_notifier);

    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
