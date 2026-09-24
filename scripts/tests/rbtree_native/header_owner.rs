//! Native binding based fixture for translated inline algorithms.
#[path = "../../../lib/rbtree.rs"]
/// Complete provider under test.
pub mod provider;
use core::ptr::{addr_of_mut, null_mut};
use provider::augmented::{
    rb_add_augmented_cached, rb_augment_callbacks, rb_erase_augmented, rb_erase_augmented_cached,
};
use provider::declarations::*;

/// Exercise both first/last header algorithms without adding production exports.
#[no_mangle]
pub unsafe extern "C" fn rust_edge(root: *const rb_root, last: bool) -> *mut rb_node {
    // SAFETY: The fixture owns a valid tree.
    unsafe {
        if last {
            rb_last(root)
        } else {
            rb_first(root)
        }
    }
}

/// Exercise header insertion including linked-node maintenance.
#[no_mangle]
pub unsafe extern "C" fn rust_add_linked(
    node: *mut rb_node_linked,
    root: *mut rb_root_linked,
    less: Option<unsafe extern "C" fn(*mut rb_node, *const rb_node) -> bool>,
) -> bool {
    // SAFETY: The fixture supplies initialized linked nodes and valid comparator.
    unsafe { rb_add_linked(node, root, less) }
}

/// Exercise the actual translated augmented erase header.
#[no_mangle]
pub unsafe extern "C" fn rust_erase_aug(
    node: *mut rb_node,
    root: *mut rb_root,
    callbacks: *const rb_augment_callbacks,
) {
    // SAFETY: The fixture provides the original augmented tree contract.
    unsafe { rb_erase_augmented(node, root, callbacks) }
}

/// Exercise publication independently of rebalancing.
#[no_mangle]
pub unsafe extern "C" fn rust_link_rcu(
    node: *mut rb_node,
    parent: *mut rb_node,
    link: *mut *mut rb_node,
) {
    // SAFETY: The fixture serializes insertion.
    unsafe { rb_link_node_rcu(node, parent, link) }
}

/// Compare parent/color semantics with both low tag bits present.
#[no_mangle]
pub unsafe extern "C" fn rust_tags(node: *mut rb_node, parent: *mut rb_node) -> *mut rb_node {
    // SAFETY: The fixture provides valid nodes.
    unsafe {
        let old = rb_parent(node);
        provider::augmented::rb_set_parent(node, parent);
        old
    }
}

#[repr(C)]
struct TestNode {
    node: rb_node,
    key: i32,
    max: i32,
}

unsafe fn value(node: *mut TestNode) -> i32 {
    // SAFETY: The fixture owns the node.
    unsafe { (*node).key }
}

crate::RB_DECLARE_CALLBACKS_MAX!(
    MAX_CALLBACKS,
    [
        max_callbacks_propagate,
        max_callbacks_copy,
        max_callbacks_rotate,
        max_callbacks_compute_max
    ],
    TestNode,
    node,
    i32,
    max,
    value
);

/// Actual generated callback entrypoints for independent protected C callers.
#[no_mangle]
pub static rust_generated_callbacks: rb_augment_callbacks = rb_augment_callbacks {
    propagate: Some(max_callbacks_propagate),
    copy: Some(max_callbacks_copy),
    rotate: Some(max_callbacks_rotate),
};

unsafe extern "C" fn less(a: *mut rb_node, b: *const rb_node) -> bool {
    // SAFETY: The fixture uses TestNode objects with node at offset zero.
    unsafe { (*a.cast::<TestNode>()).key < (*b.cast::<TestNode>()).key }
}

unsafe extern "C" fn cmp(a: *mut rb_node, b: *const rb_node) -> i32 {
    // SAFETY: The fixture uses TestNode objects with node at offset zero.
    unsafe { (*a.cast::<TestNode>()).key - (*b.cast::<TestNode>()).key }
}

unsafe extern "C" fn cached_cmp(a: *const rb_node, b: *const rb_node) -> i32 {
    // SAFETY: Same fixture layout as cmp.
    unsafe { cmp(a.cast_mut(), b) }
}

unsafe extern "C" fn key_cmp(key: *const core::ffi::c_void, b: *const rb_node) -> i32 {
    // SAFETY: key points to a fixture integer.
    unsafe { *key.cast::<i32>() - (*b.cast::<TestNode>()).key }
}

/// Exercise native layouts, every search/cached/RCU helper and callback macros.
#[no_mangle]
pub unsafe extern "C" fn rust_inline_checks() -> i32 {
    // SAFETY: All objects and keys remain live; fixture serializes all operations.
    unsafe {
        // Original container_of drops input constness and works with offsets.
        #[repr(C)]
        struct OffsetNode {
            prefix: usize,
            node: rb_node,
        }
        let mut storage = core::mem::MaybeUninit::<OffsetNode>::uninit();
        let whole = storage.as_mut_ptr();
        let const_node = core::ptr::addr_of!((*whole).node);
        let recovered: *mut OffsetNode = crate::rb_entry_safe!(const_node, OffsetNode, node);
        let nonnull: *mut OffsetNode = crate::rb_entry!(const_node, OffsetNode, node);
        let mutable: *mut OffsetNode =
            crate::rb_entry_safe!(const_node.cast_mut(), OffsetNode, node);
        if recovered != whole
            || nonnull != whole
            || mutable != whole
            || !crate::rb_entry_safe!(core::ptr::null::<rb_node>(), OffsetNode, node).is_null()
            || !crate::rb_entry_safe!(core::ptr::null_mut::<rb_node>(), OffsetNode, node).is_null()
        {
            return 20;
        }
        let mut lone: rb_node = core::mem::zeroed();
        let mut empty = RB_ROOT;
        // This exact original composition needs the no-op C callback ABI.
        __rb_add(&mut lone, &mut empty, None, Some(rb_link_noop));
        if empty.rb_node != &mut lone
            || !provider::augmented::rb_is_black(&lone)
            || provider::augmented::rb_is_red(&lone)
            || provider::augmented::rb_color(&lone) != 1
        {
            return 21;
        }
        if core::mem::size_of::<rb_node>() != 3 * core::mem::size_of::<usize>()
            || core::mem::align_of::<rb_node>() != core::mem::align_of::<usize>()
            || core::mem::offset_of!(rb_node_linked, prev) != core::mem::size_of::<rb_node>()
            || core::mem::offset_of!(rb_node_linked, next) != 4 * core::mem::size_of::<usize>()
        {
            return 1;
        }
        let mut nodes: [TestNode; 8] = core::mem::zeroed();
        let mut tree = RB_ROOT_CACHED;
        for i in 0..7 {
            nodes[i].key = i as i32;
            nodes[i].max = i as i32;
        }
        for i in [3, 1, 5, 0, 2, 4, 6] {
            rb_add_augmented_cached(
                addr_of_mut!(nodes[i].node),
                &mut tree,
                Some(less),
                &MAX_CALLBACKS,
            );
        }
        if (*tree.rb_root.rb_node.cast::<TestNode>()).max != 6 {
            return 2;
        }
        // vmalloc changes an existing node's range, then directly invokes the
        // named propagate helper generated by RB_DECLARE_CALLBACKS_MAX.
        nodes[6].key = 9;
        max_callbacks_propagate(addr_of_mut!(nodes[6].node), null_mut());
        if (*tree.rb_root.rb_node.cast::<TestNode>()).max != 9 {
            return 22;
        }
        nodes[6].key = 6;
        max_callbacks_propagate(addr_of_mut!(nodes[6].node), null_mut());
        if (*tree.rb_root.rb_node.cast::<TestNode>()).max != 6
            || !max_callbacks_compute_max(addr_of_mut!(nodes[6]), true)
        {
            return 23;
        }
        for i in [6, 5, 4, 3, 2, 1, 0] {
            rb_erase_augmented_cached(addr_of_mut!(nodes[i].node), &mut tree, &MAX_CALLBACKS);
            if i != 0 && (*tree.rb_root.rb_node.cast::<TestNode>()).max != i as i32 - 1 {
                return 3;
            }
        }
        if !rb_empty_root(&tree.rb_root) || !rb_first_cached(&tree).is_null() {
            return 4;
        }
        // Null comparator is C-defined for empty tree searches/insertion.
        if !rb_find(core::ptr::null(), &tree.rb_root, None).is_null() {
            return 5;
        }
        rb_add_cached(addr_of_mut!(nodes[0].node), &mut tree, None);
        rb_add_cached(addr_of_mut!(nodes[2].node), &mut tree, Some(less));
        rb_find_add_cached(addr_of_mut!(nodes[1].node), &mut tree, Some(cached_cmp));
        nodes[7].key = 1;
        if rb_find_add_cached(addr_of_mut!(nodes[7].node), &mut tree, Some(cached_cmp))
            != addr_of_mut!(nodes[1].node)
        {
            return 6;
        }
        for i in 0..3 {
            let key = i as i32;
            let keyptr = (&key as *const i32).cast();
            if rb_find(keyptr, &tree.rb_root, Some(key_cmp)) != addr_of_mut!(nodes[i].node)
                || rb_find_rcu(keyptr, &tree.rb_root, Some(key_cmp)) != addr_of_mut!(nodes[i].node)
                || rb_find_first(keyptr, &tree.rb_root, Some(key_cmp))
                    != addr_of_mut!(nodes[i].node)
                || !rb_next_match(keyptr, addr_of_mut!(nodes[i].node), Some(key_cmp)).is_null()
            {
                return 7;
            }
        }
        let mut visited = 0;
        crate::rbtree_postorder_for_each_entry_safe!(pos, next, &tree.rb_root, TestNode, node, {
            let _ = (pos, next);
            visited += 1;
        });
        if visited != 3 {
            return 8;
        }
        nodes[7].key = 0;
        rb_replace_node_cached(
            addr_of_mut!(nodes[0].node),
            addr_of_mut!(nodes[7].node),
            &mut tree,
        );
        if rb_first_cached(&tree) != addr_of_mut!(nodes[7].node) {
            return 9;
        }
        if rb_erase_cached(addr_of_mut!(nodes[7].node), &mut tree) != addr_of_mut!(nodes[1].node) {
            return 10;
        }
        rb_erase_cached(addr_of_mut!(nodes[1].node), &mut tree);
        rb_erase_cached(addr_of_mut!(nodes[2].node), &mut tree);
        rb_add(addr_of_mut!(nodes[0].node), &mut tree.rb_root, None);
        rb_find_add(addr_of_mut!(nodes[1].node), &mut tree.rb_root, Some(cmp));
        rb_find_add_rcu(addr_of_mut!(nodes[2].node), &mut tree.rb_root, Some(cmp));
        if rb_find_add_rcu(addr_of_mut!(nodes[2].node), &mut tree.rb_root, Some(cmp))
            != addr_of_mut!(nodes[2].node)
        {
            return 11;
        }
        let key: i32 = 1;
        let mut count = 0;
        crate::rb_for_each!(
            node,
            (&key as *const i32).cast(),
            &tree.rb_root,
            Some(key_cmp),
            {
                let _ = node;
                count += 1;
            }
        );
        if count != 1 {
            return 12;
        }
        let mut linked: rb_node_linked = core::mem::zeroed();
        crate::RB_CLEAR_LINKED_NODE!(addr_of_mut!(linked));
        if !crate::RB_EMPTY_LINKED_NODE!(addr_of_mut!(linked)) {
            return 13;
        }
        if crate::rb_entry_safe!(null_mut::<rb_node>(), TestNode, node) != null_mut() {
            return 14;
        }
        0
    }
}
