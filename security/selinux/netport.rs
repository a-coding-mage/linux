// SPDX-License-Identifier: GPL-2.0-only
/*
 * Network port table
 *
 * SELinux must keep a mapping of network ports to labels/SIDs.  This
 * mapping is maintained as part of the normal policy but a fast cache is
 * needed to reduce the lookup overhead.
 *
 * Author: Paul Moore <paul@paul-moore.com>
 *
 * This code is heavily based on the "netif" concept originally developed by
 * James Morris <jmorris@redhat.com>
 *   (see security/selinux/netif.c for more information)
 */

/*
 * (c) Copyright Hewlett-Packard Development Company, L.P., 2008
 */

/* Dependencies from the original C includes:
 * linux/types.h, linux/rcupdate.h, linux/list.h, linux/slab.h,
 * linux/spinlock.h, linux/in.h, linux/in6.h, linux/ip.h, linux/ipv6.h,
 * net/ip.h, net/ipv6.h, initcalls.h, netport.h, and objsec.h.
 */

use core::ffi::{c_char, c_int, c_uint};
use core::ptr;

const SEL_NETPORT_HASH_SIZE: usize = 256;
const SEL_NETPORT_HASH_BKT_LIMIT: c_int = 16;
const GFP_ATOMIC: c_uint = 0;

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
pub struct netport_security_struct {
    pub sid: u32,
    pub port: u16,
    pub protocol: u8,
}

#[repr(C)]
pub struct sel_netport_bkt {
    size: c_int,
    list: list_head,
}

#[repr(C)]
pub struct sel_netport {
    psec: netport_security_struct,

    list: list_head,
    rcu: rcu_head,
}

unsafe extern "C" {
    static mut selinux_enabled_boot: bool;

    static mut sel_netport_lock: spinlock_t;

    fn spin_lock_bh(lock: *mut spinlock_t);
    fn spin_unlock_bh(lock: *mut spinlock_t);
    fn rcu_read_lock();
    fn rcu_read_unlock();

    fn security_port_sid(protocol: u8, pnum: u16, sid: *mut u32) -> c_int;
    fn kmalloc(size: usize, flags: c_uint) -> *mut core::ffi::c_void;
    fn kfree_rcu(ptr: *mut sel_netport, rhf: usize);
    fn pr_warn(fmt: *const c_char, ...);
}

static mut SEL_NETPORT_HASH: [sel_netport_bkt; SEL_NETPORT_HASH_SIZE] =
    [const {
        sel_netport_bkt {
            size: 0,
            list: list_head {
                next: ptr::null_mut(),
                prev: ptr::null_mut(),
            },
        }
    }; SEL_NETPORT_HASH_SIZE];

#[inline]
unsafe fn INIT_LIST_HEAD(list: *mut list_head) {
    unsafe {
        (*list).next = list;
        (*list).prev = list;
    }
}

#[inline]
unsafe fn list_add_rcu(new: *mut list_head, head: *mut list_head) {
    unsafe {
        let next = (*head).next;
        (*new).next = next;
        (*new).prev = head;
        (*next).prev = new;
        (*head).next = new;
    }
}

#[inline]
unsafe fn list_del_rcu(entry: *mut list_head) {
    unsafe {
        let prev = (*entry).prev;
        let next = (*entry).next;
        (*next).prev = prev;
        (*prev).next = next;
    }
}

#[inline]
unsafe fn list_tail_rcu(head: *mut list_head) -> *mut list_head {
    unsafe { (*head).prev }
}

#[inline]
fn likely(value: bool) -> bool {
    value
}

#[inline]
fn unlikely(value: c_int) -> bool {
    value != 0
}

#[inline]
unsafe fn kmalloc_obj_sel_netport(flags: c_uint) -> *mut sel_netport {
    unsafe { kmalloc(core::mem::size_of::<sel_netport>(), flags) as *mut sel_netport }
}

#[inline]
unsafe fn list_entry_sel_netport_list(ptr: *mut list_head) -> *mut sel_netport {
    unsafe {
        (ptr as *mut u8).sub(core::mem::offset_of!(sel_netport, list)) as *mut sel_netport
    }
}

#[inline]
fn rcu_field_offset_sel_netport_rcu() -> usize {
    core::mem::offset_of!(sel_netport, rcu)
}

/**
 * sel_netport_hashfn - Hashing function for the port table
 * @pnum: port number
 *
 * Description:
 * This is the hashing function for the port table, it returns the bucket
 * number for the given port.
 *
 */
unsafe fn sel_netport_hashfn(pnum: u16) -> c_uint {
    (usize::from(pnum) & (SEL_NETPORT_HASH_SIZE - 1)) as c_uint
}

/**
 * sel_netport_find - Search for a port record
 * @protocol: protocol
 * @pnum: port
 *
 * Description:
 * Search the network port table and return the matching record.  If an entry
 * can not be found in the table return NULL.
 *
 */
unsafe fn sel_netport_find(protocol: u8, pnum: u16) -> *mut sel_netport {
    let idx: c_uint;
    let mut port: *mut sel_netport;

    unsafe {
        idx = sel_netport_hashfn(pnum);
        let head = &raw mut SEL_NETPORT_HASH[idx as usize].list;
        let mut pos = (*head).next;
        while pos != head {
            port = list_entry_sel_netport_list(pos);
            if (*port).psec.port == pnum && (*port).psec.protocol == protocol {
                return port;
            }
            pos = (*pos).next;
        }
    }

    ptr::null_mut()
}

/**
 * sel_netport_insert - Insert a new port into the table
 * @port: the new port record
 *
 * Description:
 * Add a new port record to the network address hash table.
 *
 */
unsafe fn sel_netport_insert(port: *mut sel_netport) {
    let idx: c_uint;

    /* we need to impose a limit on the growth of the hash table so check
     * this bucket to make sure it is within the specified bounds */
    unsafe {
        idx = sel_netport_hashfn((*port).psec.port);
        list_add_rcu(&raw mut (*port).list, &raw mut SEL_NETPORT_HASH[idx as usize].list);
        if SEL_NETPORT_HASH[idx as usize].size == SEL_NETPORT_HASH_BKT_LIMIT {
            let tail: *mut sel_netport;
            tail = list_entry_sel_netport_list(list_tail_rcu(
                &raw mut SEL_NETPORT_HASH[idx as usize].list,
            ));
            list_del_rcu(&raw mut (*tail).list);
            kfree_rcu(tail, rcu_field_offset_sel_netport_rcu());
        } else {
            SEL_NETPORT_HASH[idx as usize].size += 1;
        }
    }
}

/**
 * sel_netport_sid_slow - Lookup the SID of a network address using the policy
 * @protocol: protocol
 * @pnum: port
 * @sid: port SID
 *
 * Description:
 * This function determines the SID of a network port by querying the security
 * policy.  The result is added to the network port table to speedup future
 * queries.  Returns zero on success, negative values on failure.
 *
 */
unsafe fn sel_netport_sid_slow(protocol: u8, pnum: u16, sid: *mut u32) -> c_int {
    let mut ret: c_int;
    let mut port: *mut sel_netport;
    let new: *mut sel_netport;

    unsafe {
        spin_lock_bh(&raw mut sel_netport_lock);
        port = sel_netport_find(protocol, pnum);
        if !port.is_null() {
            *sid = (*port).psec.sid;
            spin_unlock_bh(&raw mut sel_netport_lock);
            return 0;
        }

        ret = security_port_sid(protocol, pnum, sid);
        if ret == 0 {
            /* If this memory allocation fails still return 0. The SID
             * is valid, it just won't be added to the cache.
             */
            new = kmalloc_obj_sel_netport(GFP_ATOMIC);
            if !new.is_null() {
                (*new).psec.port = pnum;
                (*new).psec.protocol = protocol;
                (*new).psec.sid = *sid;
                sel_netport_insert(new);
            }
        }

        spin_unlock_bh(&raw mut sel_netport_lock);
        if unlikely(ret) {
            pr_warn(
                c"SELinux: failure in %s(), unable to determine network port label\n".as_ptr(),
                c"sel_netport_sid_slow".as_ptr(),
            );
        }
    }
    ret
}

/**
 * sel_netport_sid - Lookup the SID of a network port
 * @protocol: protocol
 * @pnum: port
 * @sid: port SID
 *
 * Description:
 * This function determines the SID of a network port using the fastest method
 * possible.  First the port table is queried, but if an entry can't be found
 * then the policy is queried and the result is added to the table to speedup
 * future queries.  Returns zero on success, negative values on failure.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn sel_netport_sid(protocol: u8, pnum: u16, sid: *mut u32) -> c_int {
    let port: *mut sel_netport;

    unsafe {
        rcu_read_lock();
        port = sel_netport_find(protocol, pnum);
        if likely(!port.is_null()) {
            *sid = (*port).psec.sid;
            rcu_read_unlock();
            return 0;
        }
        rcu_read_unlock();

        sel_netport_sid_slow(protocol, pnum, sid)
    }
}

/**
 * sel_netport_flush - Flush the entire network port table
 *
 * Description:
 * Remove all entries from the network address table.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn sel_netport_flush() {
    let mut idx: c_uint;
    let mut port: *mut sel_netport;
    let mut port_tmp: *mut sel_netport;

    unsafe {
        spin_lock_bh(&raw mut sel_netport_lock);
        idx = 0;
        while (idx as usize) < SEL_NETPORT_HASH_SIZE {
            let head = &raw mut SEL_NETPORT_HASH[idx as usize].list;
            let mut pos = (*head).next;
            while pos != head {
                port = list_entry_sel_netport_list(pos);
                port_tmp = if (*pos).next != head {
                    list_entry_sel_netport_list((*pos).next)
                } else {
                    ptr::null_mut()
                };
                list_del_rcu(&raw mut (*port).list);
                kfree_rcu(port, rcu_field_offset_sel_netport_rcu());
                port = port_tmp;
                pos = if port.is_null() {
                    head
                } else {
                    &raw mut (*port).list
                };
            }
            SEL_NETPORT_HASH[idx as usize].size = 0;
            idx += 1;
        }
        spin_unlock_bh(&raw mut sel_netport_lock);
    }
}

#[no_mangle]
pub unsafe extern "C" fn sel_netport_init() -> c_int {
    let mut iter: c_int;

    unsafe {
        if !selinux_enabled_boot {
            return 0;
        }

        iter = 0;
        while iter < SEL_NETPORT_HASH_SIZE as c_int {
            INIT_LIST_HEAD(&raw mut SEL_NETPORT_HASH[iter as usize].list);
            SEL_NETPORT_HASH[iter as usize].size = 0;
            iter += 1;
        }
    }

    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
