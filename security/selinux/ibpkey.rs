// SPDX-License-Identifier: GPL-2.0-only
/*
 * Pkey table
 *
 * SELinux must keep a mapping of Infiniband PKEYs to labels/SIDs.  This
 * mapping is maintained as part of the normal policy but a fast cache is
 * needed to reduce the lookup overhead.
 *
 * This code is heavily based on the "netif" and "netport" concept originally
 * developed by
 * James Morris <jmorris@redhat.com> and
 * Paul Moore <paul@paul-moore.com>
 *   (see security/selinux/netif.c and security/selinux/netport.c for more
 *   information)
 */

/*
 * (c) Mellanox Technologies, 2016
 */

/* Dependencies from linux/types.h, linux/rcupdate.h, linux/list.h,
 * linux/spinlock.h, initcalls.h, ibpkey.h, and objsec.h are declared here as
 * external Rust items where this file references them.
 */

pub type u16 = u16;
pub type u32 = u32;
pub type u64 = u64;

pub const GFP_ATOMIC: u32 = 0;

pub const SEL_PKEY_HASH_SIZE: usize = 256;
pub const SEL_PKEY_HASH_BKT_LIMIT: i32 = 16;

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct rcu_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pkey_security_struct {
    pub subnet_prefix: u64,
    pub pkey: u16,
    pub sid: u32,
}

#[repr(C)]
pub struct sel_ib_pkey_bkt {
    pub size: i32,
    pub list: list_head,
}

#[repr(C)]
pub struct sel_ib_pkey {
    pub psec: pkey_security_struct,
    pub list: list_head,
    pub rcu: rcu_head,
}

unsafe extern "C" {
    static mut selinux_enabled_boot: bool;

    fn spin_lock_irqsave(lock: *mut spinlock_t, flags: *mut core::ffi::c_ulong);
    fn spin_unlock_irqrestore(lock: *mut spinlock_t, flags: core::ffi::c_ulong);

    fn rcu_read_lock();
    fn rcu_read_unlock();

    fn list_add_rcu(new: *mut list_head, head: *mut list_head);
    fn list_del_rcu(entry: *mut list_head);
    fn list_tail_rcu(head: *mut list_head) -> *mut list_head;
    fn rcu_dereference_protected(
        p: *mut list_head,
        c: bool,
    ) -> *mut list_head;
    fn lockdep_is_held(lock: *mut spinlock_t) -> bool;

    fn security_ib_pkey_sid(subnet_prefix: u64, pkey_num: u16, sid: *mut u32) -> i32;
    fn kmalloc_obj_sel_ib_pkey(gfp: u32) -> *mut sel_ib_pkey;
    fn kfree_rcu_sel_ib_pkey(ptr: *mut sel_ib_pkey, rcu: *mut rcu_head);
}

static mut sel_ib_pkey_lock: spinlock_t = spinlock_t { _private: [] };
static mut sel_ib_pkey_hash: [sel_ib_pkey_bkt; SEL_PKEY_HASH_SIZE] =
    [const {
        sel_ib_pkey_bkt {
            size: 0,
            list: list_head {
                next: core::ptr::null_mut(),
                prev: core::ptr::null_mut(),
            },
        }
    }; SEL_PKEY_HASH_SIZE];

/**
 * sel_ib_pkey_hashfn - Hashing function for the pkey table
 * @pkey: pkey number
 *
 * Description:
 * This is the hashing function for the pkey table, it returns the bucket
 * number for the given pkey.
 *
 */
unsafe fn sel_ib_pkey_hashfn(pkey: u16) -> u32 {
    (pkey & ((SEL_PKEY_HASH_SIZE - 1) as u16)) as u32
}

unsafe fn container_of_sel_ib_pkey_list(ptr: *mut list_head) -> *mut sel_ib_pkey {
    (ptr as *mut u8).sub(core::mem::offset_of!(sel_ib_pkey, list)) as *mut sel_ib_pkey
}

/**
 * sel_ib_pkey_find - Search for a pkey record
 * @subnet_prefix: subnet_prefix
 * @pkey_num: pkey_num
 *
 * Description:
 * Search the pkey table and return the matching record.  If an entry
 * can not be found in the table return NULL.
 *
 */
unsafe fn sel_ib_pkey_find(subnet_prefix: u64, pkey_num: u16) -> *mut sel_ib_pkey {
    let idx: u32;
    let mut pkey: *mut sel_ib_pkey;

    idx = sel_ib_pkey_hashfn(pkey_num);
    let head = &raw mut sel_ib_pkey_hash[idx as usize].list;
    let mut pos = (*head).next;
    while pos != head {
        pkey = container_of_sel_ib_pkey_list(pos);
        if (*pkey).psec.pkey == pkey_num && (*pkey).psec.subnet_prefix == subnet_prefix {
            return pkey;
        }
        pos = (*pos).next;
    }

    core::ptr::null_mut()
}

/**
 * sel_ib_pkey_insert - Insert a new pkey into the table
 * @pkey: the new pkey record
 *
 * Description:
 * Add a new pkey record to the hash table.
 *
 */
unsafe fn sel_ib_pkey_insert(pkey: *mut sel_ib_pkey) {
    let idx: u32;

    /* we need to impose a limit on the growth of the hash table so check
     * this bucket to make sure it is within the specified bounds
     */
    idx = sel_ib_pkey_hashfn((*pkey).psec.pkey);
    list_add_rcu(
        &raw mut (*pkey).list,
        &raw mut sel_ib_pkey_hash[idx as usize].list,
    );
    if sel_ib_pkey_hash[idx as usize].size == SEL_PKEY_HASH_BKT_LIMIT {
        let tail: *mut sel_ib_pkey;

        tail = container_of_sel_ib_pkey_list(rcu_dereference_protected(
            list_tail_rcu(&raw mut sel_ib_pkey_hash[idx as usize].list),
            lockdep_is_held(&raw mut sel_ib_pkey_lock),
        ));
        list_del_rcu(&raw mut (*tail).list);
        kfree_rcu_sel_ib_pkey(tail, &raw mut (*tail).rcu);
    } else {
        sel_ib_pkey_hash[idx as usize].size += 1;
    }
}

/**
 * sel_ib_pkey_sid_slow - Lookup the SID of a pkey using the policy
 * @subnet_prefix: subnet prefix
 * @pkey_num: pkey number
 * @sid: pkey SID
 *
 * Description:
 * This function determines the SID of a pkey by querying the security
 * policy.  The result is added to the pkey table to speedup future
 * queries.  Returns zero on success, negative values on failure.
 *
 */
unsafe fn sel_ib_pkey_sid_slow(subnet_prefix: u64, pkey_num: u16, sid: *mut u32) -> i32 {
    let mut ret: i32;
    let mut pkey: *mut sel_ib_pkey;
    let new: *mut sel_ib_pkey;
    let mut flags: core::ffi::c_ulong = 0;

    spin_lock_irqsave(&raw mut sel_ib_pkey_lock, &mut flags);
    pkey = sel_ib_pkey_find(subnet_prefix, pkey_num);
    if !pkey.is_null() {
        *sid = (*pkey).psec.sid;
        spin_unlock_irqrestore(&raw mut sel_ib_pkey_lock, flags);
        return 0;
    }

    ret = security_ib_pkey_sid(subnet_prefix, pkey_num, sid);
    if ret != 0 {
        spin_unlock_irqrestore(&raw mut sel_ib_pkey_lock, flags);
        return ret;
    }

    new = kmalloc_obj_sel_ib_pkey(GFP_ATOMIC);
    if new.is_null() {
        /* If this memory allocation fails still return 0. The SID
         * is valid, it just won't be added to the cache.
         */
        spin_unlock_irqrestore(&raw mut sel_ib_pkey_lock, flags);
        return ret;
    }

    (*new).psec.subnet_prefix = subnet_prefix;
    (*new).psec.pkey = pkey_num;
    (*new).psec.sid = *sid;
    sel_ib_pkey_insert(new);

    spin_unlock_irqrestore(&raw mut sel_ib_pkey_lock, flags);
    ret
}

/**
 * sel_ib_pkey_sid - Lookup the SID of a PKEY
 * @subnet_prefix: subnet_prefix
 * @pkey_num: pkey number
 * @sid: pkey SID
 *
 * Description:
 * This function determines the SID of a PKEY using the fastest method
 * possible.  First the pkey table is queried, but if an entry can't be found
 * then the policy is queried and the result is added to the table to speedup
 * future queries.  Returns zero on success, negative values on failure.
 *
 */
#[no_mangle]
pub unsafe extern "C" fn sel_ib_pkey_sid(subnet_prefix: u64, pkey_num: u16, sid: *mut u32) -> i32 {
    let pkey: *mut sel_ib_pkey;

    rcu_read_lock();
    pkey = sel_ib_pkey_find(subnet_prefix, pkey_num);
    if !pkey.is_null() {
        *sid = (*pkey).psec.sid;
        rcu_read_unlock();
        return 0;
    }
    rcu_read_unlock();

    sel_ib_pkey_sid_slow(subnet_prefix, pkey_num, sid)
}

/**
 * sel_ib_pkey_flush - Flush the entire pkey table
 *
 * Description:
 * Remove all entries from the pkey table
 *
 */
#[no_mangle]
pub unsafe extern "C" fn sel_ib_pkey_flush() {
    let mut idx: u32;
    let mut pkey: *mut sel_ib_pkey;
    let mut pkey_tmp: *mut sel_ib_pkey;
    let mut flags: core::ffi::c_ulong = 0;

    spin_lock_irqsave(&raw mut sel_ib_pkey_lock, &mut flags);
    idx = 0;
    while idx < SEL_PKEY_HASH_SIZE as u32 {
        let head = &raw mut sel_ib_pkey_hash[idx as usize].list;
        let mut pos = (*head).next;
        while pos != head {
            pkey = container_of_sel_ib_pkey_list(pos);
            pkey_tmp = if (*pos).next != head {
                container_of_sel_ib_pkey_list((*pos).next)
            } else {
                core::ptr::null_mut()
            };
            list_del_rcu(&raw mut (*pkey).list);
            kfree_rcu_sel_ib_pkey(pkey, &raw mut (*pkey).rcu);
            pkey = pkey_tmp;
            pos = if !pkey.is_null() {
                &raw mut (*pkey).list
            } else {
                head
            };
        }
        sel_ib_pkey_hash[idx as usize].size = 0;
        idx += 1;
    }
    spin_unlock_irqrestore(&raw mut sel_ib_pkey_lock, flags);
}

unsafe extern "C" {
    fn INIT_LIST_HEAD(list: *mut list_head);
}

#[no_mangle]
pub unsafe extern "C" fn sel_ib_pkey_init() -> i32 {
    let mut iter: i32;

    if !selinux_enabled_boot {
        return 0;
    }

    iter = 0;
    while iter < SEL_PKEY_HASH_SIZE as i32 {
        INIT_LIST_HEAD(&raw mut sel_ib_pkey_hash[iter as usize].list);
        sel_ib_pkey_hash[iter as usize].size = 0;
        iter += 1;
    }

    0
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
