/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by <linux/mempool.h>.
use core::ffi::{c_char, c_int, c_void};

// we use memory pools for preallocating messages we may receive, to
// avoid unexpected OOM conditions.
#[repr(C)]
pub struct ceph_msgpool {
    pub name: *const c_char,
    pub pool: *mut mempool_t,
    pub type_: c_int, // preallocated message type
    pub front_len: c_int, // preallocated payload size
    pub max_data_items: c_int,
}

// Opaque type supplied by <linux/mempool.h>.
pub type mempool_t = c_void;

// Opaque type supplied by the Ceph message subsystem.
#[repr(C)]
pub struct ceph_msg {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn ceph_msgpool_init(
        pool: *mut ceph_msgpool,
        type_: c_int,
        front_len: c_int,
        max_data_items: c_int,
        size: c_int,
        name: *const c_char,
    ) -> c_int;

    pub fn ceph_msgpool_destroy(pool: *mut ceph_msgpool);

    pub fn ceph_msgpool_get(
        pool: *mut ceph_msgpool,
        front_len: c_int,
        max_data_items: c_int,
    ) -> *mut ceph_msg;

    pub fn ceph_msgpool_put(pool: *mut ceph_msgpool, msg: *mut ceph_msg);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
