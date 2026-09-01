// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

// C dependencies removed from executable Rust:
// <vmlinux.h>, <bpf/bpf_helpers.h>, "bpf_misc.h", "bpf_experimental.h"

#[repr(C)]
pub struct bpf_refcount {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_rb_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_rb_root {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_spin_lock {
    _private: [u8; 0],
}

#[repr(C)]
pub struct node_data {
    pub ref_: bpf_refcount,
    pub r0: bpf_rb_node,
    pub r1: bpf_rb_node,
    pub key0: i32,
    pub key1: i32,
}

// C private(name) macro: SEC(".data." #name) __hidden __attribute__((aligned(8)))
#[unsafe(link_section = ".data.A")]
#[unsafe(no_mangle)]
pub static mut glock0: bpf_spin_lock = bpf_spin_lock { _private: [] };

// C declaration included __contains(node_data, r0).
#[unsafe(link_section = ".data.A")]
#[unsafe(no_mangle)]
pub static mut groot0: bpf_rb_root = bpf_rb_root { _private: [] };

#[unsafe(link_section = ".data.B")]
#[unsafe(no_mangle)]
pub static mut glock1: bpf_spin_lock = bpf_spin_lock { _private: [] };

// C declaration included __contains(node_data, r1).
#[unsafe(link_section = ".data.B")]
#[unsafe(no_mangle)]
pub static mut groot1: bpf_rb_root = bpf_rb_root { _private: [] };

const NR_NODES: i32 = 16;

#[unsafe(no_mangle)]
pub static mut zero: i32 = 0;

unsafe extern "C" {
    static can_loop: bool;

    fn bpf_obj_new_node_data() -> *mut node_data;
    fn bpf_obj_drop_node_data(ptr: *mut node_data);
    fn bpf_refcount_acquire(ptr: *mut node_data) -> *mut node_data;
    fn bpf_spin_lock(lock: *mut bpf_spin_lock);
    fn bpf_spin_unlock(lock: *mut bpf_spin_lock);
    fn bpf_rbtree_add(
        root: *mut bpf_rb_root,
        node: *mut bpf_rb_node,
        less: unsafe extern "C" fn(*mut bpf_rb_node, *const bpf_rb_node) -> bool,
    );
    fn bpf_rbtree_root(root: *mut bpf_rb_root) -> *mut bpf_rb_node;
    fn bpf_rbtree_left(root: *mut bpf_rb_root, node: *mut bpf_rb_node) -> *mut bpf_rb_node;
    fn bpf_rbtree_right(root: *mut bpf_rb_root, node: *mut bpf_rb_node) -> *mut bpf_rb_node;
    fn bpf_rbtree_remove(root: *mut bpf_rb_root, node: *mut bpf_rb_node) -> *mut bpf_rb_node;
    fn bpf_jiffies64() -> u64;
}

macro_rules! rb_entry {
    ($ptr:expr, $type:ty, $member:ident) => {{
        let base = core::ptr::null::<$type>();
        let offset = unsafe { core::ptr::addr_of!((*base).$member) as usize };
        ($ptr as *mut u8).wrapping_sub(offset) as *mut $type
    }};
}

unsafe extern "C" fn less0(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    let node_a: *mut node_data;
    let node_b: *mut node_data;

    node_a = rb_entry!(a, node_data, r0);
    node_b = rb_entry!(b, node_data, r0);

    unsafe { (*node_a).key0 < (*node_b).key0 }
}

unsafe extern "C" fn less1(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    let node_a: *mut node_data;
    let node_b: *mut node_data;

    node_a = rb_entry!(a, node_data, r1);
    node_b = rb_entry!(b, node_data, r1);

    unsafe { (*node_a).key1 < (*node_b).key1 }
}

#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn rbtree_search(ctx: *mut core::ffi::c_void) -> i64 {
    let mut rb_n: *mut bpf_rb_node;
    let mut rb_m: *mut bpf_rb_node;
    let mut gc_ns: [*mut bpf_rb_node; NR_NODES as usize] = [core::ptr::null_mut(); NR_NODES as usize];
    let lookup_key: i64 = (NR_NODES / 2) as i64;
    let mut n: *mut node_data;
    let mut m: *mut node_data;
    let mut i: i32;
    let mut nr_gc: i32 = 0;

    let _ = ctx;

    i = unsafe { zero };
    while i < NR_NODES && unsafe { can_loop } {
        n = unsafe { bpf_obj_new_node_data() };
        if n.is_null() {
            return line!() as i64;
        }

        m = unsafe { bpf_refcount_acquire(n) };

        unsafe {
            (*n).key0 = i;
            (*m).key1 = i;

            bpf_spin_lock(core::ptr::addr_of_mut!(glock0));
            bpf_rbtree_add(core::ptr::addr_of_mut!(groot0), core::ptr::addr_of_mut!((*n).r0), less0);
            bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));

            bpf_spin_lock(core::ptr::addr_of_mut!(glock1));
            bpf_rbtree_add(core::ptr::addr_of_mut!(groot1), core::ptr::addr_of_mut!((*m).r1), less1);
            bpf_spin_unlock(core::ptr::addr_of_mut!(glock1));
        }

        i += 1;
    }

    n = core::ptr::null_mut();
    unsafe {
        bpf_spin_lock(core::ptr::addr_of_mut!(glock0));
        rb_n = bpf_rbtree_root(core::ptr::addr_of_mut!(groot0));
    }
    while unsafe { can_loop } {
        if rb_n.is_null() {
            unsafe {
                bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));
            }
            return line!() as i64;
        }

        n = rb_entry!(rb_n, node_data, r0);
        if lookup_key == unsafe { (*n).key0 } as i64 {
            break;
        }
        if nr_gc < NR_NODES {
            gc_ns[nr_gc as usize] = rb_n;
            nr_gc += 1;
        }
        if lookup_key < unsafe { (*n).key0 } as i64 {
            rb_n = unsafe { bpf_rbtree_left(core::ptr::addr_of_mut!(groot0), rb_n) };
        } else {
            rb_n = unsafe { bpf_rbtree_right(core::ptr::addr_of_mut!(groot0), rb_n) };
        }
    }

    if n.is_null() || lookup_key != unsafe { (*n).key0 } as i64 {
        unsafe {
            bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));
        }
        return line!() as i64;
    }

    i = 0;
    while i < nr_gc {
        rb_n = gc_ns[i as usize];
        gc_ns[i as usize] = unsafe { bpf_rbtree_remove(core::ptr::addr_of_mut!(groot0), rb_n) };
        i += 1;
    }

    m = unsafe { bpf_refcount_acquire(n) };
    unsafe {
        bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));
    }

    i = 0;
    while i < nr_gc {
        rb_n = gc_ns[i as usize];
        if !rb_n.is_null() {
            n = rb_entry!(rb_n, node_data, r0);
            unsafe {
                bpf_obj_drop_node_data(n);
            }
        }
        i += 1;
    }

    if m.is_null() {
        return line!() as i64;
    }

    unsafe {
        bpf_spin_lock(core::ptr::addr_of_mut!(glock1));
        rb_m = bpf_rbtree_remove(core::ptr::addr_of_mut!(groot1), core::ptr::addr_of_mut!((*m).r1));
        bpf_spin_unlock(core::ptr::addr_of_mut!(glock1));
        bpf_obj_drop_node_data(m);
    }
    if rb_m.is_null() {
        return line!() as i64;
    }
    unsafe {
        bpf_obj_drop_node_data(rb_entry!(rb_m, node_data, r1));
    }

    0
}

/*
 * Use a separate MSG macro instead of passing to TEST_XXX(..., MSG)
 * to ensure the message itself is not in the bpf prog lineinfo
 * which the verifier includes in its log.
 * Otherwise, the test_loader will incorrectly match the prog lineinfo
 * instead of the log generated by the verifier.
 */

// __failure __msg("call bpf_rbtree_root{{.+}}; R0{{(_w)?}}=rcu_ptr_or_null_node_data(id={{[0-9]+}},non_own_ref")
#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_root_spinlock_true(ctx: *mut core::ffi::c_void) -> i64 {
    let mut rb_n: *mut bpf_rb_node;
    let mut jiffies: u64 = 0;

    let _ = ctx;

    unsafe {
        bpf_spin_lock(core::ptr::addr_of_mut!(glock0));
        rb_n = bpf_rbtree_root(core::ptr::addr_of_mut!(groot0));
        if !rb_n.is_null() {
            jiffies = bpf_jiffies64();
        }
        bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));
    }

    (jiffies != 0) as i64
}

// __failure __msg("call bpf_rbtree_{{(left|right).+}}; R0{{(_w)?}}=rcu_ptr_or_null_node_data(id={{[0-9]+}},non_own_ref")
#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_left_spinlock_true(ctx: *mut core::ffi::c_void) -> i64 {
    let mut rb_n: *mut bpf_rb_node;
    let mut n: *mut node_data;
    let mut jiffies: u64 = 0;

    let _ = ctx;

    unsafe {
        bpf_spin_lock(core::ptr::addr_of_mut!(glock0));
        rb_n = bpf_rbtree_root(core::ptr::addr_of_mut!(groot0));
        if rb_n.is_null() {
            bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));
            return 1;
        }
        n = rb_entry!(rb_n, node_data, r0);
        n = bpf_refcount_acquire(n);
        bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));
    }
    if n.is_null() {
        return 1;
    }

    unsafe {
        bpf_spin_lock(core::ptr::addr_of_mut!(glock0));
        rb_n = bpf_rbtree_left(core::ptr::addr_of_mut!(groot0), core::ptr::addr_of_mut!((*n).r0));
        if !rb_n.is_null() {
            jiffies = bpf_jiffies64();
        }
        bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));
    }

    (jiffies != 0) as i64
}

// __failure __msg("call bpf_rbtree_{{(left|right).+}}; R0{{(_w)?}}=rcu_ptr_or_null_node_data(id={{[0-9]+}},non_own_ref")
#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_right_spinlock_true(ctx: *mut core::ffi::c_void) -> i64 {
    let mut rb_n: *mut bpf_rb_node;
    let mut n: *mut node_data;
    let mut jiffies: u64 = 0;

    let _ = ctx;

    unsafe {
        bpf_spin_lock(core::ptr::addr_of_mut!(glock0));
        rb_n = bpf_rbtree_root(core::ptr::addr_of_mut!(groot0));
        if rb_n.is_null() {
            bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));
            return 1;
        }
        n = rb_entry!(rb_n, node_data, r0);
        n = bpf_refcount_acquire(n);
        bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));
    }
    if n.is_null() {
        return 1;
    }

    unsafe {
        bpf_spin_lock(core::ptr::addr_of_mut!(glock0));
        rb_n = bpf_rbtree_right(core::ptr::addr_of_mut!(groot0), core::ptr::addr_of_mut!((*n).r0));
        if !rb_n.is_null() {
            jiffies = bpf_jiffies64();
        }
        bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));
    }

    (jiffies != 0) as i64
}

// __failure __msg("bpf_spin_lock at off=0 must be held for bpf_rb_root")
#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_root_spinlock_false(ctx: *mut core::ffi::c_void) -> i64 {
    let mut rb_n: *mut bpf_rb_node;
    let mut jiffies: u64 = 0;

    let _ = ctx;

    unsafe {
        rb_n = bpf_rbtree_root(core::ptr::addr_of_mut!(groot0));
        if !rb_n.is_null() {
            jiffies = bpf_jiffies64();
        }
    }

    (jiffies != 0) as i64
}

// __failure __msg("bpf_spin_lock at off=0 must be held for bpf_rb_root")
#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_left_spinlock_false(ctx: *mut core::ffi::c_void) -> i64 {
    let mut rb_n: *mut bpf_rb_node;
    let mut n: *mut node_data;
    let mut jiffies: u64 = 0;

    let _ = ctx;

    unsafe {
        bpf_spin_lock(core::ptr::addr_of_mut!(glock0));
        rb_n = bpf_rbtree_root(core::ptr::addr_of_mut!(groot0));
        if rb_n.is_null() {
            bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));
            return 1;
        }
        n = rb_entry!(rb_n, node_data, r0);
        n = bpf_refcount_acquire(n);
        bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));
    }
    if n.is_null() {
        return 1;
    }

    unsafe {
        rb_n = bpf_rbtree_left(core::ptr::addr_of_mut!(groot0), core::ptr::addr_of_mut!((*n).r0));
        if !rb_n.is_null() {
            jiffies = bpf_jiffies64();
        }
    }

    (jiffies != 0) as i64
}

// __failure __msg("bpf_spin_lock at off=0 must be held for bpf_rb_root")
#[unsafe(link_section = "syscall")]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_right_spinlock_false(ctx: *mut core::ffi::c_void) -> i64 {
    let mut rb_n: *mut bpf_rb_node;
    let mut n: *mut node_data;
    let mut jiffies: u64 = 0;

    let _ = ctx;

    unsafe {
        bpf_spin_lock(core::ptr::addr_of_mut!(glock0));
        rb_n = bpf_rbtree_root(core::ptr::addr_of_mut!(groot0));
        if rb_n.is_null() {
            bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));
            return 1;
        }
        n = rb_entry!(rb_n, node_data, r0);
        n = bpf_refcount_acquire(n);
        bpf_spin_unlock(core::ptr::addr_of_mut!(glock0));
    }
    if n.is_null() {
        return 1;
    }

    unsafe {
        rb_n = bpf_rbtree_right(core::ptr::addr_of_mut!(groot0), core::ptr::addr_of_mut!((*n).r0));
        if !rb_n.is_null() {
            jiffies = bpf_jiffies64();
        }
    }

    (jiffies != 0) as i64
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
pub static _license: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
