// SPDX-License-Identifier: GPL-2.0
// C dependencies removed from executable Rust:
// <vmlinux.h>, <bpf/bpf_tracing.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_core_read.h>, "bpf_experimental.h", "bpf_misc.h".

#[repr(C)]
pub struct node_acquire {
    pub key: i64,
    pub data: i64,
    pub node: bpf_rb_node,
    pub refcount: bpf_refcount,
}

#[repr(C)]
pub struct node_refcounted {
    pub key: i64,
    pub list: bpf_list_node,
    pub refcount: bpf_refcount,
}

extern "C" {
    #[link_name = "bpf_rcu_read_lock"]
    fn bpf_rcu_read_lock() -> ();
    #[link_name = "bpf_rcu_read_unlock"]
    fn bpf_rcu_read_unlock() -> ();
}

// C macro intent:
// #define private(name) SEC(".data." #name) __hidden __attribute__((aligned(8)))
#[link_section = ".data.A"]
#[no_mangle]
pub static mut glock: bpf_spin_lock = unsafe { core::mem::zeroed() };

#[link_section = ".data.A"]
#[no_mangle]
pub static mut groot: bpf_rb_root = unsafe { core::mem::zeroed() };
// __contains(node_acquire, node)

#[link_section = ".data.B"]
#[no_mangle]
pub static mut lock: bpf_spin_lock = unsafe { core::mem::zeroed() };

#[link_section = ".data.B"]
#[no_mangle]
pub static mut head: bpf_list_head = unsafe { core::mem::zeroed() };
// __contains(node_refcounted, list)

unsafe extern "C" fn less(a: *mut bpf_rb_node, b: *const bpf_rb_node) -> bool {
    let node_a: *mut node_acquire;
    let node_b: *mut node_acquire;

    node_a = container_of!(a, node_acquire, node);
    node_b = container_of!(b, node_acquire, node);

    (*node_a).key < (*node_b).key
}

// SEC("?tc")
// __failure __msg("Unreleased reference id=4 alloc_insn={{[0-9]+}}")
#[no_mangle]
pub unsafe extern "C" fn rbtree_refcounted_node_ref_escapes(ctx: *mut core::ffi::c_void) -> i64 {
    let n: *mut node_acquire;
    let m: *mut node_acquire;

    n = bpf_obj_new::<node_acquire>();
    if n.is_null() {
        return 1;
    }

    bpf_spin_lock(core::ptr::addr_of_mut!(glock));
    bpf_rbtree_add(core::ptr::addr_of_mut!(groot), core::ptr::addr_of_mut!((*n).node), Some(less));
    /* m becomes an owning ref but is never drop'd or added to a tree */
    m = bpf_refcount_acquire(n);
    bpf_spin_unlock(core::ptr::addr_of_mut!(glock));
    if m.is_null() {
        return 2;
    }

    (*m).key = 2;
    0
}

// SEC("?tc")
// __failure __msg("Possibly NULL pointer passed to trusted R1")
// __msg("requires a non-NULL value of type (void *)")
#[no_mangle]
pub unsafe extern "C" fn refcount_acquire_maybe_null(ctx: *mut core::ffi::c_void) -> i64 {
    let n: *mut node_acquire;
    let m: *mut node_acquire;

    n = bpf_obj_new::<node_acquire>();
    /* Intentionally not testing !n
     * it's MAYBE_NULL for refcount_acquire
     */
    m = bpf_refcount_acquire(n);
    if !m.is_null() {
        bpf_obj_drop(m);
    }
    if !n.is_null() {
        bpf_obj_drop(n);
    }

    0
}

// SEC("?tc")
// __failure __msg("R1 is neither owning or non-owning ref")
// __msg("expects a pointer to a BPF-managed refcounted object, but R1 is a context pointer")
#[no_mangle]
pub unsafe extern "C" fn refcount_acquire_non_object(ctx: *mut core::ffi::c_void) -> i64 {
    (bpf_refcount_acquire(ctx) != core::ptr::null_mut()) as i64
}

// SEC("?tc")
// __failure __msg("Unreleased reference id=3 alloc_insn={{[0-9]+}}")
#[no_mangle]
pub unsafe extern "C" fn rbtree_refcounted_node_ref_escapes_owning_input(
    ctx: *mut core::ffi::c_void,
) -> i64 {
    let n: *mut node_acquire;
    let m: *mut node_acquire;

    n = bpf_obj_new::<node_acquire>();
    if n.is_null() {
        return 1;
    }

    /* m becomes an owning ref but is never drop'd or added to a tree */
    m = bpf_refcount_acquire(n);
    (*m).key = 2;

    bpf_spin_lock(core::ptr::addr_of_mut!(glock));
    bpf_rbtree_add(core::ptr::addr_of_mut!(groot), core::ptr::addr_of_mut!((*n).node), Some(less));
    bpf_spin_unlock(core::ptr::addr_of_mut!(glock));

    0
}

// SEC("?tc")
// __failure __msg("dereference of modified ptr_ ptr R1")
#[no_mangle]
pub unsafe extern "C" fn refcount_acquire_list_node_offset(ctx: *mut core::ffi::c_void) -> i64 {
    let node: *mut node_refcounted;
    let base: *mut node_refcounted;
    let ref_: *mut node_refcounted;
    let list_node: *mut bpf_list_node;

    node = bpf_obj_new::<node_refcounted>();
    if node.is_null() {
        return 1;
    }

    bpf_spin_lock(core::ptr::addr_of_mut!(lock));
    bpf_list_push_front(core::ptr::addr_of_mut!(head), core::ptr::addr_of_mut!((*node).list));
    list_node = bpf_list_pop_front(core::ptr::addr_of_mut!(head));
    bpf_spin_unlock(core::ptr::addr_of_mut!(lock));
    if list_node.is_null() {
        return 2;
    }

    base = container_of!(list_node, node_refcounted, list);
    ref_ = bpf_refcount_acquire(list_node);
    if !ref_.is_null() {
        bpf_obj_drop(ref_);
    }
    bpf_obj_drop(base);
    0
}

// SEC("?fentry.s/" SYS_PREFIX "sys_getpgid")
// __failure __msg("function calls are not allowed while holding a lock")
#[no_mangle]
pub unsafe extern "C" fn rbtree_fail_sleepable_lock_across_rcu(
    file: *mut file,
    kobj: *mut kobject,
    bin_attr: *mut bin_attribute,
    buf: *mut core::ffi::c_char,
    off: loff_t,
    len: size_t,
) -> i32 {
    let n: *mut node_acquire;

    n = bpf_obj_new::<node_acquire>();
    if n.is_null() {
        return 0;
    }

    /* spin_{lock,unlock} are in different RCU CS */
    bpf_rcu_read_lock();
    bpf_spin_lock(core::ptr::addr_of_mut!(glock));
    bpf_rbtree_add(core::ptr::addr_of_mut!(groot), core::ptr::addr_of_mut!((*n).node), Some(less));
    bpf_rcu_read_unlock();

    bpf_rcu_read_lock();
    bpf_spin_unlock(core::ptr::addr_of_mut!(glock));
    bpf_rcu_read_unlock();

    0
}

#[link_section = "license"]
#[no_mangle]
pub static _license: [u8; 4] = *b"GPL\0";
