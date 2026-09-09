// SPDX-License-Identifier: GPL-2.0
//
// C dependencies supplied by the surrounding kernel translation are intentionally
// left as external symbols.

use core::ffi::{c_char, c_int, c_uint, c_void};

pub type gfp_t = c_uint;

#[repr(C)]
pub struct ceph_msgpool {
    pub type_: c_int,
    pub front_len: c_int,
    pub max_data_items: c_int,
    pub pool: *mut mempool_t,
    pub name: *const c_char,
}

#[repr(C)]
pub struct ceph_msg {
    pub pool: *mut ceph_msgpool,
    pub front: ceph_msg_front,
    pub hdr: ceph_msg_header,
    pub data_length: c_int,
    pub num_data_items: c_int,
    pub kref: kref,
}

#[repr(C)]
pub struct ceph_msg_front {
    pub iov_len: usize,
}

#[repr(C)]
pub struct ceph_msg_header {
    pub front_len: u32,
}

#[repr(C)]
pub struct kref {
    pub refcount: c_int,
}

#[repr(C)]
pub struct mempool_t {
    _private: [u8; 0],
}

extern "C" {
    fn ceph_msg_new2(
        type_: c_int,
        front_len: c_int,
        max_data_items: c_int,
        gfp_mask: gfp_t,
        can_fail: bool,
    ) -> *mut ceph_msg;
    fn ceph_msg_put(msg: *mut ceph_msg);
    fn mempool_create(
        size: c_int,
        alloc_fn: unsafe extern "C" fn(gfp_t, *mut c_void) -> *mut c_void,
        free_fn: unsafe extern "C" fn(*mut c_void, *mut c_void),
        pool_data: *mut c_void,
    ) -> *mut mempool_t;
    fn mempool_destroy(pool: *mut mempool_t);
    fn mempool_alloc(pool: *mut mempool_t, gfp_mask: gfp_t) -> *mut c_void;
    fn mempool_free(element: *mut c_void, pool: *mut mempool_t);
    fn kref_init(kref: *mut kref);
    fn cpu_to_le32(value: c_int) -> u32;
    fn dout(format: *const c_char, ...);
    fn pr_warn_ratelimited(format: *const c_char, ...);
    fn WARN_ON_ONCE(condition: c_int) -> c_int;
}

const GFP_NOFS: gfp_t = 0;

unsafe extern "C" fn msgpool_alloc(gfp_mask: gfp_t, arg: *mut c_void) -> *mut c_void {
    let pool = arg as *mut ceph_msgpool;
    let msg = ceph_msg_new2(
        (*pool).type_,
        (*pool).front_len,
        (*pool).max_data_items,
        gfp_mask,
        true,
    );
    if msg.is_null() {
        dout(c"msgpool_alloc %s failed\n".as_ptr(), (*pool).name);
    } else {
        dout(c"msgpool_alloc %s %p\n".as_ptr(), (*pool).name, msg);
        (*msg).pool = pool;
    }
    msg as *mut c_void
}

unsafe extern "C" fn msgpool_free(element: *mut c_void, arg: *mut c_void) {
    let pool = arg as *mut ceph_msgpool;
    let msg = element as *mut ceph_msg;
    dout(c"msgpool_release %s %p\n".as_ptr(), (*pool).name, msg);
    (*msg).pool = core::ptr::null_mut();
    ceph_msg_put(msg);
}

pub unsafe extern "C" fn ceph_msgpool_init(
    pool: *mut ceph_msgpool,
    type_: c_int,
    front_len: c_int,
    max_data_items: c_int,
    size: c_int,
    name: *const c_char,
) -> c_int {
    dout(c"msgpool %s init\n".as_ptr(), name);
    (*pool).type_ = type_;
    (*pool).front_len = front_len;
    (*pool).max_data_items = max_data_items;
    (*pool).pool = mempool_create(
        size,
        msgpool_alloc,
        msgpool_free,
        pool as *mut c_void,
    );
    if (*pool).pool.is_null() {
        return -12;
    }
    (*pool).name = name;
    0
}

pub unsafe extern "C" fn ceph_msgpool_destroy(pool: *mut ceph_msgpool) {
    dout(c"msgpool %s destroy\n".as_ptr(), (*pool).name);
    mempool_destroy((*pool).pool);
}

pub unsafe extern "C" fn ceph_msgpool_get(
    pool: *mut ceph_msgpool,
    front_len: c_int,
    max_data_items: c_int,
) -> *mut ceph_msg {
    if front_len > (*pool).front_len || max_data_items > (*pool).max_data_items {
        pr_warn_ratelimited(
            c"%s need %d/%d, pool %s has %d/%d\n".as_ptr(),
            c"ceph_msgpool_get".as_ptr(),
            front_len,
            max_data_items,
            (*pool).name,
            (*pool).front_len,
            (*pool).max_data_items,
        );
        WARN_ON_ONCE(1);
        return ceph_msg_new2((*pool).type_, front_len, max_data_items, GFP_NOFS, false);
    }
    let msg = mempool_alloc((*pool).pool, GFP_NOFS) as *mut ceph_msg;
    dout(c"msgpool_get %s %p\n".as_ptr(), (*pool).name, msg);
    msg
}

pub unsafe extern "C" fn ceph_msgpool_put(
    pool: *mut ceph_msgpool,
    msg: *mut ceph_msg,
) {
    dout(c"msgpool_put %s %p\n".as_ptr(), (*pool).name, msg);
    (*msg).front.iov_len = (*pool).front_len as usize;
    (*msg).hdr.front_len = cpu_to_le32((*pool).front_len);
    (*msg).data_length = 0;
    (*msg).num_data_items = 0;
    kref_init(&mut (*msg).kref);
    mempool_free(msg as *mut c_void, (*pool).pool);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
