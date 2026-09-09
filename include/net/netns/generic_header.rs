/* SPDX-License-Identifier: GPL-2.0 */
/*
 * generic net pointers
 */

/*
 * Dependencies supplied by the corresponding kernel headers are intentionally
 * left external here.
 */
use core::ffi::c_void;
use core::ffi::c_uint;

/*
 * Generic net pointers are to be used by modules to put some private
 * stuff on the struct net without explicit struct net modification
 *
 * The rules are simple:
 * 1. set pernet_operations->id.  After register_pernet_device you
 *    will have the id of your private pointer.
 * 2. set pernet_operations->size to have the code allocate and free
 *    a private structure pointed to from struct net.
 * 3. do not change this pointer while the net is alive;
 * 4. do not try to have any private reference on the net_generic object.
 *
 * After accomplishing all of the above, the private pointer can be
 * accessed with the net_generic() call.
 */

#[repr(C)]
pub struct net_generic_s {
    pub len: c_uint,
    pub rcu: rcu_head,
}

#[repr(C)]
pub union net_generic {
    pub s: net_generic_s,
    /* C flexible array member: storage follows this union in the allocation. */
    pub ptr: *mut *mut c_void,
}

#[repr(C)]
pub struct net {
    pub gen: *mut net_generic,
}

#[repr(C)]
pub struct rcu_head {
    _private: [u8; 0],
}

extern "C" {
    fn rcu_read_lock();
    fn rcu_read_unlock();
}

/// Access a generic net pointer under an RCU read-side critical section.
#[inline]
pub unsafe fn net_generic(net: *const net, id: c_uint) -> *mut c_void {
    let ng: *mut net_generic;
    let ptr: *mut c_void;

    rcu_read_lock();
    ng = (*net).gen;
    ptr = *(*ng).ptr.add(id as usize);
    rcu_read_unlock();

    ptr
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
