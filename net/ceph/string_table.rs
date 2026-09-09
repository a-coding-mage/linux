// SPDX-License-Identifier: GPL-2.0
// External Linux/Ceph declarations referenced by this implementation are
// supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rb_root {
    pub rb_node: *mut rb_node,
}

#[repr(C)]
pub struct kref {
    _private: [u8; 0],
}

#[repr(C)]
pub struct rcu_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ceph_string {
    pub node: rb_node,
    pub kref: kref,
    pub len: usize,
    pub str_: [u8; 0],
    pub rcu: rcu_head,
}

extern "C" {
    static mut string_tree_lock: c_void;
    static mut string_tree: rb_root;

    fn spin_lock(lock: *mut c_void);
    fn spin_unlock(lock: *mut c_void);
    fn rb_entry(node: *mut rb_node) -> *mut ceph_string;
    fn rb_erase(node: *mut rb_node, root: *mut rb_root);
    fn rb_clear_node(node: *mut rb_node);
    fn rb_link_node(node: *mut rb_node, parent: *mut rb_node, link: *mut *mut rb_node);
    fn rb_insert_color(node: *mut rb_node, root: *mut rb_root);
    fn rb_empty_node(node: *const rb_node) -> bool;
    fn rb_empty_root(root: *const rb_root) -> bool;
    fn kref_get_unless_zero(ref_: *mut kref) -> bool;
    fn kref_init(ref_: *mut kref);
    fn ceph_compare_string(cs: *const ceph_string, str_: *const c_char, len: usize) -> c_int;
    fn kmalloc(size: usize, flags: c_int) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn kfree_rcu(ptr: *mut ceph_string, rcu: *mut rcu_head);
}

const GFP_NOFS: c_int = 0;
const EAGAIN: c_int = 11;

#[no_mangle]
pub unsafe extern "C" fn ceph_find_or_create_string(
    str_: *const c_char,
    len: usize,
) -> *mut ceph_string {
    let mut cs: *mut ceph_string;
    let mut exist: *mut ceph_string;
    let mut p: *mut *mut rb_node;
    let mut parent: *mut rb_node;
    let mut ret: c_int;

    exist = core::ptr::null_mut();
    spin_lock(&raw mut string_tree_lock);
    p = &raw mut string_tree.rb_node;
    while !(*p).is_null() {
        exist = rb_entry(*p);
        ret = ceph_compare_string(exist, str_, len);
        if ret > 0 {
            p = &mut (*(*p)).rb_left;
        } else if ret < 0 {
            p = &mut (*(*p)).rb_right;
        } else {
            break;
        }
        exist = core::ptr::null_mut();
    }
    if !exist.is_null() && !kref_get_unless_zero(&mut (*exist).kref) {
        rb_erase(&mut (*exist).node, &raw mut string_tree);
        rb_clear_node(&mut (*exist).node);
        exist = core::ptr::null_mut();
    }
    spin_unlock(&raw mut string_tree_lock);
    if !exist.is_null() {
        return exist;
    }

    cs = kmalloc(core::mem::size_of::<ceph_string>() + len + 1, GFP_NOFS)
        as *mut ceph_string;
    if cs.is_null() {
        return core::ptr::null_mut();
    }

    kref_init(&mut (*cs).kref);
    (*cs).len = len;
    core::ptr::copy_nonoverlapping(str_ as *const u8, (*cs).str_.as_mut_ptr(), len);
    *(*cs).str_.as_mut_ptr().add(len) = 0;

    loop {
        exist = core::ptr::null_mut();
        parent = core::ptr::null_mut();
        p = &raw mut string_tree.rb_node;
        spin_lock(&raw mut string_tree_lock);
        while !(*p).is_null() {
            parent = *p;
            exist = rb_entry(*p);
            ret = ceph_compare_string(exist, str_, len);
            if ret > 0 {
                p = &mut (*(*p)).rb_left;
            } else if ret < 0 {
                p = &mut (*(*p)).rb_right;
            } else {
                break;
            }
            exist = core::ptr::null_mut();
        }
        ret = 0;
        if exist.is_null() {
            rb_link_node(&mut (*cs).node, parent, p);
            rb_insert_color(&mut (*cs).node, &raw mut string_tree);
        } else if !kref_get_unless_zero(&mut (*exist).kref) {
            rb_erase(&mut (*exist).node, &raw mut string_tree);
            rb_clear_node(&mut (*exist).node);
            ret = -EAGAIN;
        }
        spin_unlock(&raw mut string_tree_lock);
        if ret == -EAGAIN {
            continue;
        }
        break;
    }

    if !exist.is_null() {
        kfree(cs as *mut c_void);
        cs = exist;
    }
    cs
}

#[no_mangle]
pub unsafe extern "C" fn ceph_release_string(ref_: *mut kref) {
    let cs = (ref_ as *mut u8).sub(core::mem::offset_of!(ceph_string, kref))
        as *mut ceph_string;
    spin_lock(&raw mut string_tree_lock);
    if !rb_empty_node(&(*cs).node) {
        rb_erase(&mut (*cs).node, &raw mut string_tree);
        rb_clear_node(&mut (*cs).node);
    }
    spin_unlock(&raw mut string_tree_lock);
    kfree_rcu(cs, &mut (*cs).rcu);
}

#[no_mangle]
pub unsafe extern "C" fn ceph_strings_empty() -> bool {
    rb_empty_root(&raw const string_tree)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
