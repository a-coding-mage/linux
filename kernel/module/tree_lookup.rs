// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Modules tree lookup
 *
 * Copyright (C) 2015 Peter Zijlstra
 * Copyright (C) 2015 Rusty Russell
 */

// Dependencies supplied by the surrounding kernel translation.

/*
 * Use a latched RB-tree for __module_address(); this allows us to use
 * RCU lookups of the address from any context.
 *
 * This is conditional on PERF_EVENTS || TRACING || CFI because those can
 * really hit __module_address() hard by doing a lot of stack unwinding;
 * potentially from NMI context.
 */

#[inline(always)]
unsafe fn __mod_tree_val(n: *mut latch_tree_node) -> ::core::primitive::usize {
    let mod_mem: *mut module_memory = container_of!(n, module_memory, mtn.node);

    (*mod_mem).base as ::core::primitive::usize
}

#[inline(always)]
unsafe fn __mod_tree_size(n: *mut latch_tree_node) -> ::core::primitive::usize {
    let mod_mem: *mut module_memory = container_of!(n, module_memory, mtn.node);

    (*mod_mem).size as ::core::primitive::usize
}

#[inline(always)]
unsafe fn mod_tree_less(
    a: *mut latch_tree_node,
    b: *mut latch_tree_node,
) -> bool {
    __mod_tree_val(a) < __mod_tree_val(b)
}

#[inline(always)]
unsafe fn mod_tree_comp(key: *mut ::core::ffi::c_void, n: *mut latch_tree_node) -> ::core::ffi::c_int {
    let val = key as ::core::primitive::usize;
    let start: ::core::primitive::usize;
    let end: ::core::primitive::usize;

    start = __mod_tree_val(n);
    if val < start {
        return -1;
    }

    end = start.wrapping_add(__mod_tree_size(n));
    if val >= end {
        return 1;
    }

    0
}

static mod_tree_ops: latch_tree_ops = latch_tree_ops {
    less: Some(mod_tree_less),
    comp: Some(mod_tree_comp),
};

#[inline(never)]
unsafe fn __mod_tree_insert(node: *mut mod_tree_node, tree: *mut mod_tree_root) {
    latch_tree_insert(&mut (*node).node, &mut (*tree).root, &mod_tree_ops);
}

unsafe fn __mod_tree_remove(node: *mut mod_tree_node, tree: *mut mod_tree_root) {
    latch_tree_erase(&mut (*node).node, &mut (*tree).root, &mod_tree_ops);
}

/*
 * These modifications: insert, remove_init and remove; are serialized by the
 * module_mutex.
 */
unsafe fn mod_tree_insert(mod_: *mut module) {
    for_each_mod_mem_type!(type, {
        (*mod_).mem[type].mtn.mod_ = mod_;
        if (*mod_).mem[type].size != 0 {
            __mod_tree_insert(&mut (*mod_).mem[type].mtn, &mut mod_tree);
        }
    });
}

unsafe fn mod_tree_remove_init(mod_: *mut module) {
    for_class_mod_mem_type!(type, init, {
        if (*mod_).mem[type].size != 0 {
            __mod_tree_remove(&mut (*mod_).mem[type].mtn, &mut mod_tree);
        }
    });
}

unsafe fn mod_tree_remove(mod_: *mut module) {
    for_each_mod_mem_type!(type, {
        if (*mod_).mem[type].size != 0 {
            __mod_tree_remove(&mut (*mod_).mem[type].mtn, &mut mod_tree);
        }
    });
}

unsafe fn mod_find(addr: ::core::primitive::usize, tree: *mut mod_tree_root) -> *mut module {
    let ltn: *mut latch_tree_node =
        latch_tree_find(addr as *mut ::core::ffi::c_void, &mut (*tree).root, &mod_tree_ops);
    if ltn.is_null() {
        return ::core::ptr::null_mut();
    }

    container_of!(ltn, mod_tree_node, node).as_ref().unwrap().mod_
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
