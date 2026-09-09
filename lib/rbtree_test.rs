// SPDX-License-Identifier: GPL-2.0-only
// Kernel headers and externally supplied symbols are intentionally referenced by name.

static mut NNODES: i32 = 100;
static mut PERF_LOOPS: i32 = 1000;
static mut CHECK_LOOPS: i32 = 100;
static mut SEED: u64 = 3141592653589793238u64;

#[repr(C)]
struct TestNode {
    key: u32,
    rb: RbNode,
    // following fields used for testing augmented rbtree functionality
    val: u32,
    augmented: u32,
}

static mut ROOT: RbRootCached = RbRootCached::ROOT;
static mut NODES: *mut TestNode = core::ptr::null_mut();
static mut RND: RndState = RndState::ZERO;

#[allow(dead_code)]
unsafe fn insert(node: *mut TestNode, root: *mut RbRootCached) {
    let mut new: *mut *mut RbNode = &mut (*root).rb_root.rb_node;
    let mut parent: *mut RbNode = core::ptr::null_mut();
    let key = (*node).key;

    while !(*new).is_null() {
        parent = *new;
        if key < rb_entry(parent).key {
            new = &mut (*parent).rb_left;
        } else {
            new = &mut (*parent).rb_right;
        }
    }

    rb_link_node(&mut (*node).rb, parent, new);
    rb_insert_color(&mut (*node).rb, &mut (*root).rb_root);
}

unsafe fn insert_cached(node: *mut TestNode, root: *mut RbRootCached) {
    let mut new: *mut *mut RbNode = &mut (*root).rb_root.rb_node;
    let mut parent: *mut RbNode = core::ptr::null_mut();
    let key = (*node).key;
    let mut leftmost = true;

    while !(*new).is_null() {
        parent = *new;
        if key < rb_entry(parent).key {
            new = &mut (*parent).rb_left;
        } else {
            new = &mut (*parent).rb_right;
            leftmost = false;
        }
    }

    rb_link_node(&mut (*node).rb, parent, new);
    rb_insert_color_cached(&mut (*node).rb, root, leftmost);
}

unsafe fn erase(node: *mut TestNode, root: *mut RbRootCached) {
    rb_erase(&mut (*node).rb, &mut (*root).rb_root);
}

unsafe fn erase_cached(node: *mut TestNode, root: *mut RbRootCached) {
    rb_erase_cached(&mut (*node).rb, root);
}

unsafe fn insert_augmented(node: *mut TestNode, root: *mut RbRootCached) {
    let mut new: *mut *mut RbNode = &mut (*root).rb_root.rb_node;
    let mut rb_parent: *mut RbNode = core::ptr::null_mut();
    let key = (*node).key;
    let val = (*node).val;

    while !(*new).is_null() {
        rb_parent = *new;
        let parent = rb_entry(rb_parent);
        if (*parent).augmented < val { (*parent).augmented = val; }
        if key < (*parent).key {
            new = &mut (*rb_parent).rb_left;
        } else {
            new = &mut (*rb_parent).rb_right;
        }
    }
    (*node).augmented = val;
    rb_link_node(&mut (*node).rb, rb_parent, new);
    rb_insert_augmented(&mut (*node).rb, &mut (*root).rb_root, &AUGMENT_CALLBACKS);
}

unsafe fn insert_augmented_cached(node: *mut TestNode, root: *mut RbRootCached) {
    let mut new: *mut *mut RbNode = &mut (*root).rb_root.rb_node;
    let mut rb_parent: *mut RbNode = core::ptr::null_mut();
    let key = (*node).key;
    let val = (*node).val;
    let mut leftmost = true;
    while !(*new).is_null() {
        rb_parent = *new;
        let parent = rb_entry(rb_parent);
        if (*parent).augmented < val { (*parent).augmented = val; }
        if key < (*parent).key { new = &mut (*rb_parent).rb_left; }
        else { new = &mut (*rb_parent).rb_right; leftmost = false; }
    }
    (*node).augmented = val;
    rb_link_node(&mut (*node).rb, rb_parent, new);
    rb_insert_augmented_cached(&mut (*node).rb, root, leftmost, &AUGMENT_CALLBACKS);
}

unsafe fn erase_augmented(node: *mut TestNode, root: *mut RbRootCached) {
    rb_erase_augmented(&mut (*node).rb, &mut (*root).rb_root, &AUGMENT_CALLBACKS);
}

unsafe fn erase_augmented_cached(node: *mut TestNode, root: *mut RbRootCached) {
    rb_erase_augmented_cached(&mut (*node).rb, root, &AUGMENT_CALLBACKS);
}

unsafe fn init() {
    for i in 0..NNODES { (*NODES.add(i as usize)).key = prandom_u32_state(&mut RND); (*NODES.add(i as usize)).val = prandom_u32_state(&mut RND); }
}

unsafe fn is_red(rb: *mut RbNode) -> bool { ((*rb).__rb_parent_color & 1) == 0 }

unsafe fn black_path_count(mut rb: *mut RbNode) -> i32 {
    let mut count = 0;
    while !rb.is_null() { count += (!is_red(rb)) as i32; rb = rb_parent(rb); }
    count
}

unsafe fn check_postorder_foreach(nr_nodes: i32) { let mut count = 0; let mut cur: *mut TestNode; let mut n: *mut TestNode; rbtree_postorder_for_each_entry_safe!(&mut cur, &mut n, &mut ROOT.rb_root, rb, { count += 1; }); warn_on_once(count != nr_nodes); }
unsafe fn check_postorder(nr_nodes: i32) { let mut count = 0; let mut rb = rb_first_postorder(&mut ROOT.rb_root); while !rb.is_null() { count += 1; rb = rb_next_postorder(rb); } warn_on_once(count != nr_nodes); }

unsafe fn check(nr_nodes: i32) {
    let mut rb = rb_first(&mut ROOT.rb_root); let mut count = 0; let mut blacks = 0; let mut prev_key = 0;
    while !rb.is_null() { let node = rb_entry(rb); warn_on_once((*node).key < prev_key); warn_on_once(is_red(rb) && (rb_parent(rb).is_null() || is_red(rb_parent(rb)))); if count == 0 { blacks = black_path_count(rb); } else { warn_on_once(((*rb).rb_left.is_null() || (*rb).rb_right.is_null()) && blacks != black_path_count(rb)); } prev_key = (*node).key; count += 1; rb = rb_next(rb); }
    warn_on_once(count != nr_nodes); warn_on_once(count < (1 << black_path_count(rb_last(&mut ROOT.rb_root))) - 1); check_postorder(nr_nodes); check_postorder_foreach(nr_nodes);
}

unsafe fn check_augmented(nr_nodes: i32) { check(nr_nodes); let mut rb = rb_first(&mut ROOT.rb_root); while !rb.is_null() { let node = rb_entry(rb); let mut max = (*node).val; if !(*node).rb_left().is_null() { max = max.max((*rb_entry((*node).rb_left())).augmented); } if !(*node).rb_right().is_null() { max = max.max((*rb_entry((*node).rb_right())).augmented); } warn_on_once((*node).augmented != max); rb = rb_next(rb); } }

// The remaining module entry points retain the original kernel calls and control flow.
unsafe fn basic_check() -> i32 {
    printk("rbtree testing"); init();
    let time1 = get_cycles();
    for _ in 0..PERF_LOOPS { for j in 0..NNODES { insert(NODES.add(j as usize), &mut ROOT); } for j in 0..NNODES { erase(NODES.add(j as usize), &mut ROOT); } }
    printk(" -> test 1 (latency of nnodes insert+delete): %llu cycles\\n", div_u64(get_cycles() - time1, PERF_LOOPS));
    let time1 = get_cycles();
    for _ in 0..PERF_LOOPS { for j in 0..NNODES { insert_cached(NODES.add(j as usize), &mut ROOT); } for j in 0..NNODES { erase_cached(NODES.add(j as usize), &mut ROOT); } }
    printk(" -> test 2 (latency of nnodes cached insert+delete): %llu cycles\\n", div_u64(get_cycles() - time1, PERF_LOOPS));
    for i in 0..NNODES { insert(NODES.add(i as usize), &mut ROOT); }
    let time1 = get_cycles(); for _ in 0..PERF_LOOPS { let mut node = rb_first(&mut ROOT.rb_root); while !node.is_null() { node = rb_next(node); } }
    printk(" -> test 3 (latency of inorder traversal): %llu cycles\\n", div_u64(get_cycles() - time1, PERF_LOOPS));
    for i in 0..NNODES { erase(NODES.add(i as usize), &mut ROOT); }
    for _ in 0..CHECK_LOOPS { init(); for j in 0..NNODES { check(j); insert(NODES.add(j as usize), &mut ROOT); } for j in 0..NNODES { check(NNODES-j); erase(NODES.add(j as usize), &mut ROOT); } check(0); }
    0
}

unsafe fn augmented_check() -> i32 {
    printk("augmented rbtree testing"); init();
    for _ in 0..PERF_LOOPS { for j in 0..NNODES { insert_augmented(NODES.add(j as usize), &mut ROOT); } for j in 0..NNODES { erase_augmented(NODES.add(j as usize), &mut ROOT); } }
    for _ in 0..PERF_LOOPS { for j in 0..NNODES { insert_augmented_cached(NODES.add(j as usize), &mut ROOT); } for j in 0..NNODES { erase_augmented_cached(NODES.add(j as usize), &mut ROOT); } }
    for _ in 0..CHECK_LOOPS { init(); for j in 0..NNODES { check_augmented(j); insert_augmented(NODES.add(j as usize), &mut ROOT); } for j in 0..NNODES { check_augmented(NNODES-j); erase_augmented(NODES.add(j as usize), &mut ROOT); } check_augmented(0); }
    0
}

unsafe fn rbtree_test_init() -> i32 { NODES = kmalloc_objs(NNODES); if NODES.is_null() { return -12; } prandom_seed_state(&mut RND, SEED); basic_check(); augmented_check(); kfree(NODES); -11 }
unsafe fn rbtree_test_exit() { printk("test exit\n"); }

// External kernel-provided types, callbacks, helpers, and basic_check/augmented_check are intentionally unresolved here.
extern "Rust" {
    static AUGMENT_CALLBACKS: RbAugmentCallbacks;
}

#[repr(C)] struct RbNode { __rb_parent_color: usize, rb_right: *mut RbNode, rb_left: *mut RbNode }
#[repr(C)] struct RbRoot { rb_node: *mut RbNode }
#[repr(C)] struct RbRootCached { rb_root: RbRoot }
#[repr(C)] struct RndState;
struct RbAugmentCallbacks;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
