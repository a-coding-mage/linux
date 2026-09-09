/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding kernel headers:
// linux/types.h, linux/kref.h, linux/rbtree.h, linux/rcupdate.h

#[repr(C)]
pub struct ceph_string {
    pub kref: kref,
    pub node_or_rcu: ceph_string_node_or_rcu,
    pub len: usize,
    pub str_: [std::ffi::c_char; 0],
}

#[repr(C)]
pub union ceph_string_node_or_rcu {
    pub node: rb_node,
    pub rcu: rcu_head,
}

extern "C" {
    pub fn ceph_release_string(ref_: *mut kref);
    pub fn ceph_find_or_create_string(str_: *const std::ffi::c_char, len: usize)
        -> *mut ceph_string;
    pub fn ceph_strings_empty() -> bool;

    pub fn kref_get(ref_: *mut kref);
    pub fn kref_put(ref_: *mut kref, release: unsafe extern "C" fn(*mut kref));
    pub fn kref_get_unless_zero(ref_: *mut kref) -> bool;
    pub fn rcu_read_lock();
    pub fn rcu_read_unlock();
    pub fn strncmp(
        lhs: *const std::ffi::c_char,
        rhs: *const std::ffi::c_char,
        len: usize,
    ) -> std::ffi::c_int;
}

#[inline]
pub unsafe fn ceph_get_string(str_: *mut ceph_string) -> *mut ceph_string {
    kref_get(&mut (*str_).kref);
    str_
}

#[inline]
pub unsafe fn ceph_put_string(str_: *mut ceph_string) {
    if str_.is_null() {
        return;
    }
    kref_put(&mut (*str_).kref, ceph_release_string);
}

#[inline]
pub unsafe fn ceph_compare_string(
    cs: *const ceph_string,
    str_: *const std::ffi::c_char,
    len: usize,
) -> std::ffi::c_int {
    let cs_len = if !cs.is_null() { (*cs).len } else { 0 };
    if cs_len != len {
        return cs_len.wrapping_sub(len) as std::ffi::c_int;
    }
    if len == 0 {
        return 0;
    }
    strncmp((*cs).str_.as_ptr(), str_, len)
}

// Equivalent of the ceph_try_get_string(x) statement-expression macro.
#[inline]
pub unsafe fn ceph_try_get_string(
    x: *mut *mut ceph_string,
) -> *mut ceph_string {
    let mut ___str: *mut ceph_string;
    rcu_read_lock();
    loop {
        ___str = *x;
        if ___str.is_null() || kref_get_unless_zero(&mut (*___str).kref) {
            break;
        }
    }
    rcu_read_unlock();
    ___str
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
