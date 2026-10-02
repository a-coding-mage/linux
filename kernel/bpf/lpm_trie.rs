// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (c) 2016,2017 Daniel Mack
 * Copyright (c) 2016 David Herrmann
 */
//! Longest prefix match map, translated from the retained lpm_trie.c.
//!
//! Callback control flow, traversal, validation and ownership live in Rust.
//! The companion C file only exposes kernel macros/inlines and registers the
//! original map operations/BTF ID. All layouts come from configured bindgen.
#![allow(missing_docs, unsafe_op_in_unsafe_fn)]

#[allow(clippy::all, dead_code, non_camel_case_types, non_snake_case,
         non_upper_case_globals, improper_ctypes, unreachable_pub)]
mod bindings {
    use core::ffi;
    type __kernel_size_t = usize;
    type __kernel_ssize_t = isize;
    type __kernel_ptrdiff_t = isize;
    include!(concat!(env!("OBJTREE"), "/rust/bindings/lpm_trie_generated.rs"));
}

use bindings::*;
// Use the kernel's LKMM atomics, not Rust's core atomics. Only published flags
// and the count read by mem_usage change independently of RCU pointer updates.
use kernel::sync::atomic::{atomic_load, atomic_store, Relaxed};
use core::{
    ffi::{c_int, c_long, c_ulong, c_void},
    mem::{align_of, offset_of, size_of},
    ptr::{self, addr_of, addr_of_mut, null, null_mut},
};

const LPM_TREE_NODE_FLAG_IM: u32 = 1;
const LPM_DATA_SIZE_MIN: usize = 1;
const LPM_DATA_SIZE_MAX: usize = 256;
const LPM_KEY_SIZE_MIN: usize = size_of::<bpf_lpm_trie_key_u8>() + LPM_DATA_SIZE_MIN;
const LPM_KEY_SIZE_MAX: usize = size_of::<bpf_lpm_trie_key_u8>() + LPM_DATA_SIZE_MAX;

// Reject target/layout mismatches at compilation, including configuration-
// dependent bpf_map, bpf_mem_alloc and rqspinlock_t member placement.
const _: () = {
    assert!(size_of::<lpm_trie>() == LUPOS_LPM_TRIE_SIZE as usize);
    assert!(align_of::<lpm_trie>() == LUPOS_LPM_TRIE_ALIGN as usize);
    assert!(offset_of!(lpm_trie, map) == LUPOS_LPM_MAP_OFFSET as usize);
    assert!(offset_of!(lpm_trie, root) == LUPOS_LPM_ROOT_OFFSET as usize);
    assert!(offset_of!(lpm_trie, ma) == LUPOS_LPM_MA_OFFSET as usize);
    assert!(offset_of!(lpm_trie, n_entries) == LUPOS_LPM_ENTRIES_OFFSET as usize);
    assert!(offset_of!(lpm_trie, max_prefixlen) == LUPOS_LPM_MAX_PREFIX_OFFSET as usize);
    assert!(offset_of!(lpm_trie, data_size) == LUPOS_LPM_DATA_SIZE_OFFSET as usize);
    assert!(offset_of!(lpm_trie, lock) == LUPOS_LPM_LOCK_OFFSET as usize);
    assert!(size_of::<lpm_trie_node>() == LUPOS_LPM_NODE_SIZE as usize);
    assert!(align_of::<lpm_trie_node>() == LUPOS_LPM_NODE_ALIGN as usize);
    assert!(offset_of!(lpm_trie_node, child) == LUPOS_LPM_CHILD_OFFSET as usize);
    assert!(offset_of!(lpm_trie_node, prefixlen) == LUPOS_LPM_PREFIX_OFFSET as usize);
    assert!(offset_of!(lpm_trie_node, flags) == LUPOS_LPM_FLAGS_OFFSET as usize);
    assert!(offset_of!(lpm_trie_node, data) == LUPOS_LPM_NODE_DATA_OFFSET as usize);
    assert!(size_of::<bpf_lpm_trie_key_u8>() == LUPOS_LPM_KEY_SIZE as usize);
    assert!(align_of::<bpf_lpm_trie_key_u8>() == LUPOS_LPM_KEY_ALIGN as usize);
    assert!(offset_of!(bpf_lpm_trie_key_u8, data) == LUPOS_LPM_KEY_DATA_OFFSET as usize);
    assert!(offset_of!(lpm_trie_node, data) % size_of::<u32>() == 0);
    assert!(offset_of!(bpf_lpm_trie_key_u8, data) % size_of::<u32>() == 0);
};

#[inline]
unsafe fn trie_from_map(map: *const bpf_map) -> *mut lpm_trie {
    map.cast::<u8>().sub(offset_of!(lpm_trie, map)).cast_mut().cast()
}

#[inline]
unsafe fn node_data(node: *const lpm_trie_node) -> *const u8 {
    addr_of!((*node).data).cast()
}

#[inline]
unsafe fn key_data(key: *const bpf_lpm_trie_key_u8) -> *const u8 {
    addr_of!((*key).data).cast()
}

#[inline]
unsafe fn key_prefix(key: *const bpf_lpm_trie_key_u8) -> u32 {
    (*key).__bindgen_anon_1.prefixlen
}

#[inline]
unsafe fn child_slot(node: *mut lpm_trie_node, bit: usize) -> *mut *mut lpm_trie_node {
    addr_of_mut!((*node).child).cast::<*mut lpm_trie_node>().add(bit)
}

#[inline]
unsafe fn extract_bit(data: *const u8, index: usize) -> usize {
    ((*data.add(index / 8) & (1 << (7 - index % 8))) != 0) as usize
}

#[inline(always)]
unsafe fn __longest_prefix_match(
    trie: *const lpm_trie,
    node: *const lpm_trie_node,
    key: *const bpf_lpm_trie_key_u8,
) -> usize {
    let limit = (*node).prefixlen.min(key_prefix(key));
    let mut prefixlen = 0u32;
    let mut i = 0usize;
    let node_bytes = node_data(node);
    let key_bytes = key_data(key);

    #[cfg(all(CONFIG_HAVE_EFFICIENT_UNALIGNED_ACCESS, CONFIG_64BIT))]
    if (*trie).data_size >= 8 {
        let diff = u64::from_be(ptr::read_unaligned(node_bytes.cast::<u64>())
            ^ ptr::read_unaligned(key_bytes.cast::<u64>()));
        prefixlen = diff.leading_zeros();
        if prefixlen >= limit {
            return limit as usize;
        }
        if diff != 0 {
            return prefixlen as usize;
        }
        i = 8;
    }

    while (*trie).data_size >= i + 4 {
        let diff = u32::from_be(ptr::read_unaligned(node_bytes.add(i).cast::<u32>())
            ^ ptr::read_unaligned(key_bytes.add(i).cast::<u32>()));
        prefixlen += diff.leading_zeros();
        if prefixlen >= limit {
            return limit as usize;
        }
        if diff != 0 {
            return prefixlen as usize;
        }
        i += 4;
    }
    if (*trie).data_size >= i + 2 {
        let diff = u16::from_be(ptr::read_unaligned(node_bytes.add(i).cast::<u16>())
            ^ ptr::read_unaligned(key_bytes.add(i).cast::<u16>()));
        // Rust's u16::leading_zeros already returns 16 - fls(diff).
        prefixlen += diff.leading_zeros();
        if prefixlen >= limit {
            return limit as usize;
        }
        if diff != 0 {
            return prefixlen as usize;
        }
        i += 2;
    }
    if (*trie).data_size >= i + 1 {
        prefixlen += (*node_bytes.add(i) ^ *key_bytes.add(i)).leading_zeros();
        if prefixlen >= limit {
            return limit as usize;
        }
    }
    prefixlen as usize
}

unsafe fn longest_prefix_match(
    trie: *const lpm_trie,
    node: *const lpm_trie_node,
    key: *const bpf_lpm_trie_key_u8,
) -> usize {
    __longest_prefix_match(trie, node, key)
}

/// Called under the BPF/RCU reader contract, from a syscall or eBPF program.
#[export_name = "lupos_trie_lookup_elem"]
pub unsafe extern "C" fn trie_lookup_elem(map: *mut bpf_map, keyp: *mut c_void) -> *mut c_void {
    let trie = trie_from_map(map);
    let key = keyp.cast::<bpf_lpm_trie_key_u8>();
    if key_prefix(key) as usize > (*trie).max_prefixlen {
        return null_mut();
    }

    let mut found: *mut lpm_trie_node = null_mut();
    let mut node = lupos_lpm_dereference_bpf(addr_of!((*trie).root));
    while !node.is_null() {
        let matchlen = __longest_prefix_match(trie, node, key);
        if matchlen == (*trie).max_prefixlen {
            found = node;
            break;
        }
        if matchlen < (*node).prefixlen as usize {
            break;
        }
        if atomic_load(addr_of_mut!((*node).flags), Relaxed) & LPM_TREE_NODE_FLAG_IM == 0 {
            found = node;
        }
        let bit = extract_bit(key_data(key), (*node).prefixlen as usize);
        node = lupos_lpm_dereference_bpf(child_slot(node, bit));
    }
    if found.is_null() {
        null_mut()
    } else {
        node_data(found).add((*trie).data_size).cast_mut().cast()
    }
}

unsafe fn lpm_trie_node_alloc(trie: *mut lpm_trie, value: *const c_void) -> *mut lpm_trie_node {
    let node = bpf_mem_cache_alloc(addr_of_mut!((*trie).ma)).cast::<lpm_trie_node>();
    if node.is_null() {
        return null_mut();
    }
    // This allocation is still private; RCU publication follows initialization.
    (*node).flags = 0;
    if !value.is_null() {
        ptr::copy_nonoverlapping(value.cast::<u8>(),
            node_data(node).add((*trie).data_size).cast_mut(), (*trie).map.value_size as usize);
    }
    node
}

unsafe fn trie_check_add_elem(trie: *mut lpm_trie, flags: u64) -> c_int {
    if flags == BPF_EXIST as u64 {
        return -(ENOENT as c_int);
    }
    // Writers hold trie.lock, while mem_usage reads without it. Relaxed raw
    // LKMM operations preserve READ_ONCE/WRITE_ONCE semantics without a field
    // reference or additional ordering; the lock serializes this load/store.
    let entries = addr_of_mut!((*trie).n_entries);
    let count = atomic_load(entries, Relaxed);
    if count == (*trie).map.max_entries as usize {
        return -(ENOSPC as c_int);
    }
    atomic_store(entries, count.wrapping_add(1), Relaxed);
    0
}

#[export_name = "lupos_trie_update_elem"]
pub unsafe extern "C" fn trie_update_elem(
    map: *mut bpf_map, keyp: *mut c_void, value: *mut c_void, flags: u64,
) -> c_long {
    let trie = trie_from_map(map);
    let key = keyp.cast::<bpf_lpm_trie_key_u8>();
    if flags > BPF_EXIST as u64 || key_prefix(key) as usize > (*trie).max_prefixlen {
        return -(EINVAL as c_long);
    }
    // As in C, allocate/copy the value before locking; initialize the key and
    // unpublished child pointers only after the fallible lock succeeds.
    let new_node = lpm_trie_node_alloc(trie, value);
    if new_node.is_null() {
        return -(ENOMEM as c_long);
    }
    let mut irq_flags: c_ulong = 0;
    let mut ret = lupos_lpm_lock_irqsave(addr_of_mut!((*trie).lock), &mut irq_flags);
    let mut free_node: *mut lpm_trie_node = null_mut();
    if ret == 0 {
        ret = 'out: {
            (*new_node).prefixlen = key_prefix(key);
            lupos_lpm_init_pointer(child_slot(new_node, 0), null_mut());
            lupos_lpm_init_pointer(child_slot(new_node, 1), null_mut());
            ptr::copy_nonoverlapping(key_data(key), node_data(new_node).cast_mut(), (*trie).data_size);

            let mut slot = addr_of_mut!((*trie).root);
            let mut node;
            let mut matchlen = 0;
            loop {
                node = lupos_lpm_dereference_protected(slot);
                if node.is_null() {
                    break;
                }
                matchlen = longest_prefix_match(trie, node, key);
                if (*node).prefixlen as usize != matchlen || (*node).prefixlen == key_prefix(key) {
                    break;
                }
                slot = child_slot(node, extract_bit(key_data(key), (*node).prefixlen as usize));
            }
            if node.is_null() {
                let err = trie_check_add_elem(trie, flags);
                if err != 0 {
                    break 'out err;
                }
                lupos_lpm_assign_pointer(slot, new_node);
                break 'out 0;
            }
            if (*node).prefixlen as usize == matchlen {
                if atomic_load(addr_of_mut!((*node).flags), Relaxed) & LPM_TREE_NODE_FLAG_IM == 0 {
                    if flags == BPF_NOEXIST as u64 {
                        break 'out -(EEXIST as c_int);
                    }
                } else {
                    let err = trie_check_add_elem(trie, flags);
                    if err != 0 {
                        break 'out err;
                    }
                }
                *child_slot(new_node, 0) = *child_slot(node, 0);
                *child_slot(new_node, 1) = *child_slot(node, 1);
                lupos_lpm_assign_pointer(slot, new_node);
                free_node = node;
                break 'out 0;
            }
            let err = trie_check_add_elem(trie, flags);
            if err != 0 {
                break 'out err;
            }
            if matchlen == key_prefix(key) as usize {
                let bit = extract_bit(node_data(node), matchlen);
                lupos_lpm_assign_pointer(child_slot(new_node, bit), node);
                lupos_lpm_assign_pointer(slot, new_node);
                break 'out 0;
            }
            let im_node = lpm_trie_node_alloc(trie, null());
            if im_node.is_null() {
                let entries = addr_of_mut!((*trie).n_entries);
                atomic_store(entries, atomic_load(entries, Relaxed).wrapping_sub(1), Relaxed);
                break 'out -(ENOMEM as c_int);
            }
            (*im_node).prefixlen = matchlen as u32;
            // The intermediate node is also private until its RCU publication.
            (*im_node).flags |= LPM_TREE_NODE_FLAG_IM;
            ptr::copy_nonoverlapping(node_data(node), node_data(im_node).cast_mut(), (*trie).data_size);
            if extract_bit(key_data(key), matchlen) != 0 {
                lupos_lpm_assign_pointer(child_slot(im_node, 0), node);
                lupos_lpm_assign_pointer(child_slot(im_node, 1), new_node);
            } else {
                lupos_lpm_assign_pointer(child_slot(im_node, 0), new_node);
                lupos_lpm_assign_pointer(child_slot(im_node, 1), node);
            }
            lupos_lpm_assign_pointer(slot, im_node);
            0
        };
        lupos_lpm_unlock_irqrestore(addr_of_mut!((*trie).lock), irq_flags);
    }
    // The failed-lock macro already restores IRQ/preemption state. Neither
    // that path nor an unpublished allocation failure waits for an RCU GP.
    if ret != 0 {
        bpf_mem_cache_free(addr_of_mut!((*trie).ma), new_node.cast());
    }
    bpf_mem_cache_free_rcu(addr_of_mut!((*trie).ma), free_node.cast());
    ret as c_long
}

#[export_name = "lupos_trie_delete_elem"]
pub unsafe extern "C" fn trie_delete_elem(map: *mut bpf_map, keyp: *mut c_void) -> c_long {
    let trie = trie_from_map(map);
    let key = keyp.cast::<bpf_lpm_trie_key_u8>();
    if key_prefix(key) as usize > (*trie).max_prefixlen {
        return -(EINVAL as c_long);
    }
    let mut irq_flags: c_ulong = 0;
    let ret = lupos_lpm_lock_irqsave(addr_of_mut!((*trie).lock), &mut irq_flags);
    if ret != 0 {
        return ret as c_long;
    }
    let mut free_node: *mut lpm_trie_node = null_mut();
    let mut free_parent: *mut lpm_trie_node = null_mut();
    let ret = 'out: {
        let mut trim = addr_of_mut!((*trie).root);
        let mut trim2 = trim;
        let mut parent: *mut lpm_trie_node = null_mut();
        let mut node;
        let mut matchlen = 0;
        loop {
            node = lupos_lpm_dereference_protected(trim);
            if node.is_null() {
                break;
            }
            matchlen = longest_prefix_match(trie, node, key);
            if (*node).prefixlen as usize != matchlen || (*node).prefixlen == key_prefix(key) {
                break;
            }
            parent = node;
            trim2 = trim;
            trim = child_slot(node, extract_bit(key_data(key), (*node).prefixlen as usize));
        }
        if node.is_null() || (*node).prefixlen != key_prefix(key)
            || (*node).prefixlen as usize != matchlen || atomic_load(addr_of_mut!((*node).flags), Relaxed) & LPM_TREE_NODE_FLAG_IM != 0
        {
            break 'out -(ENOENT as c_int);
        }
        let entries = addr_of_mut!((*trie).n_entries);
        atomic_store(entries, atomic_load(entries, Relaxed).wrapping_sub(1), Relaxed);
        if !lupos_lpm_access_pointer(child_slot(node, 0)).is_null()
            && !lupos_lpm_access_pointer(child_slot(node, 1)).is_null()
        {
            // Readers may inspect flags under RCU without the writer lock.
            // Keep this as a relaxed load/store under the existing lock,
            // rather than adding an atomic RMW or a publication barrier.
            let flags = addr_of_mut!((*node).flags);
            atomic_store(flags, atomic_load(flags, Relaxed) | LPM_TREE_NODE_FLAG_IM, Relaxed);
            break 'out 0;
        }
        // Maintain the C invariant: every intermediate node has two children.
        if !parent.is_null() && atomic_load(addr_of_mut!((*parent).flags), Relaxed) & LPM_TREE_NODE_FLAG_IM != 0
            && (*child_slot(node, 0)).is_null() && (*child_slot(node, 1)).is_null()
        {
            let sibling = if node == lupos_lpm_access_pointer(child_slot(parent, 0)) {
                lupos_lpm_access_pointer(child_slot(parent, 1))
            } else {
                lupos_lpm_access_pointer(child_slot(parent, 0))
            };
            lupos_lpm_assign_pointer(trim2, sibling);
            free_parent = parent;
            free_node = node;
            break 'out 0;
        }
        if !(*child_slot(node, 0)).is_null() {
            lupos_lpm_assign_pointer(trim, lupos_lpm_access_pointer(child_slot(node, 0)));
        } else if !(*child_slot(node, 1)).is_null() {
            lupos_lpm_assign_pointer(trim, lupos_lpm_access_pointer(child_slot(node, 1)));
        } else {
            lupos_lpm_init_pointer(trim, null_mut());
        }
        free_node = node;
        0
    };
    lupos_lpm_unlock_irqrestore(addr_of_mut!((*trie).lock), irq_flags);
    bpf_mem_cache_free_rcu(addr_of_mut!((*trie).ma), free_parent.cast());
    bpf_mem_cache_free_rcu(addr_of_mut!((*trie).ma), free_node.cast());
    ret as c_long
}

#[export_name = "lupos_trie_alloc"]
pub unsafe extern "C" fn trie_alloc(attr: *mut bpf_attr) -> *mut bpf_map {
    let attrs = addr_of!((*attr).__bindgen_anon_1);
    let map_flags = (*attrs).map_flags;
    if (*attrs).max_entries == 0 || map_flags & BPF_F_NO_PREALLOC == 0
        || map_flags & !(LUPOS_LPM_CREATE_FLAG_MASK as u32) != 0
        || !lupos_lpm_flags_access_ok(map_flags)
        || ((*attrs).key_size as usize) < LPM_KEY_SIZE_MIN
        || ((*attrs).key_size as usize) > LPM_KEY_SIZE_MAX
        || (*attrs).value_size < 1 || (*attrs).value_size as u64 > LUPOS_LPM_VAL_SIZE_MAX as u64
    {
        return (-(EINVAL as c_long)) as *mut bpf_map;
    }
    // bpf_map_area_alloc is the original zeroing, memory-accounted allocator.
    let trie = bpf_map_area_alloc(size_of::<lpm_trie>() as u64,
        LUPOS_LPM_NUMA_NO_NODE as c_int).cast::<lpm_trie>();
    if trie.is_null() {
        return (-(ENOMEM as c_long)) as *mut bpf_map;
    }
    bpf_map_init_from_attr(addr_of_mut!((*trie).map), attr);
    (*trie).data_size = (*attrs).key_size as usize - offset_of!(bpf_lpm_trie_key_u8, data);
    (*trie).max_prefixlen = (*trie).data_size * 8;
    lupos_lpm_lock_init(addr_of_mut!((*trie).lock));
    let leaf_size = size_of::<lpm_trie_node>() + (*trie).data_size + (*trie).map.value_size as usize;
    let err = bpf_mem_alloc_init(addr_of_mut!((*trie).ma), leaf_size as c_int, false);
    if err != 0 {
        bpf_map_area_free(trie.cast());
        return (err as c_long) as *mut bpf_map;
    }
    addr_of_mut!((*trie).map)
}

#[export_name = "lupos_trie_free"]
pub unsafe extern "C" fn trie_free(map: *mut bpf_map) {
    let trie = trie_from_map(map);
    'out: loop {
        let mut slot = addr_of_mut!((*trie).root);
        loop {
            let node = lupos_lpm_dereference_protected(slot);
            if node.is_null() {
                break 'out;
            }
            if !lupos_lpm_access_pointer(child_slot(node, 0)).is_null() {
                slot = child_slot(node, 0);
                continue;
            }
            if !lupos_lpm_access_pointer(child_slot(node, 1)).is_null() {
                slot = child_slot(node, 1);
                continue;
            }
            // The caller has excluded all BPF programs; match C's raw free.
            bpf_mem_cache_raw_free(node.cast());
            lupos_lpm_init_pointer(slot, null_mut());
            break;
        }
    }
    bpf_mem_alloc_destroy(addr_of_mut!((*trie).ma));
    bpf_map_area_free(trie.cast());
}

#[export_name = "lupos_trie_get_next_key"]
pub unsafe extern "C" fn trie_get_next_key(
    map: *mut bpf_map, keyp: *mut c_void, nextp: *mut c_void,
) -> c_int {
    let trie = trie_from_map(map);
    let key = keyp.cast::<bpf_lpm_trie_key_u8>();
    let next_key = nextp.cast::<bpf_lpm_trie_key_u8>();
    let mut search_root = lupos_lpm_dereference(addr_of!((*trie).root));
    if search_root.is_null() {
        return -(ENOENT as c_int);
    }
    let mut node_stack: *mut *mut lpm_trie_node = null_mut();
    let mut next_node: *mut lpm_trie_node = null_mut();
    let mut err = 0;
    'find_leftmost: {
        if key.is_null() || key_prefix(key) as usize > (*trie).max_prefixlen {
            break 'find_leftmost;
        }
        // max_prefixlen + 1 accommodates every prefix, including /0.
        node_stack = lupos_lpm_alloc_stack((*trie).max_prefixlen + 1);
        if node_stack.is_null() {
            return -(ENOMEM as c_int);
        }
        let mut stack_ptr: c_int = -1;
        let mut matchlen = 0;
        let mut node = search_root;
        while !node.is_null() {
            stack_ptr += 1;
            *node_stack.add(stack_ptr as usize) = node;
            matchlen = longest_prefix_match(trie, node, key);
            if (*node).prefixlen as usize != matchlen || (*node).prefixlen == key_prefix(key) {
                break;
            }
            node = lupos_lpm_dereference(child_slot(node,
                extract_bit(key_data(key), (*node).prefixlen as usize)));
        }
        if node.is_null() || (*node).prefixlen as usize != matchlen
            || atomic_load(addr_of_mut!((*node).flags), Relaxed) & LPM_TREE_NODE_FLAG_IM != 0
        {
            break 'find_leftmost;
        }
        node = *node_stack.add(stack_ptr as usize);
        while stack_ptr > 0 {
            let parent = *node_stack.add(stack_ptr as usize - 1);
            if lupos_lpm_dereference(child_slot(parent, 0)) == node {
                search_root = lupos_lpm_dereference(child_slot(parent, 1));
                if !search_root.is_null() {
                    break 'find_leftmost;
                }
            }
            if atomic_load(addr_of_mut!((*parent).flags), Relaxed) & LPM_TREE_NODE_FLAG_IM == 0 {
                next_node = parent;
                break 'find_leftmost;
            }
            node = parent;
            stack_ptr -= 1;
        }
        err = -(ENOENT as c_int);
    }
    if err == 0 {
        if next_node.is_null() {
            let mut node = search_root;
            while !node.is_null() {
                if atomic_load(addr_of_mut!((*node).flags), Relaxed) & LPM_TREE_NODE_FLAG_IM != 0 {
                    node = lupos_lpm_dereference(child_slot(node, 0));
                } else {
                    next_node = node;
                    node = lupos_lpm_dereference(child_slot(node, 0));
                    if node.is_null() {
                        node = lupos_lpm_dereference(child_slot(next_node, 1));
                    }
                }
            }
        }
        (*next_key).__bindgen_anon_1.prefixlen = (*next_node).prefixlen;
        ptr::copy_nonoverlapping(node_data(next_node), key_data(next_key).cast_mut(), (*trie).data_size);
    }
    kfree(node_stack.cast());
    err
}

#[export_name = "lupos_trie_check_btf"]
pub unsafe extern "C" fn trie_check_btf(
    _map: *mut bpf_map, _btf: *const btf, key_type: *const btf_type, _value_type: *const btf_type,
) -> c_int {
    // This is precisely BTF_INFO_KIND(info), including its seven-bit mask.
    // The retained C checks the key's struct kind and imposes no value check.
    if ((*key_type).info >> 24) & 0x7f != BTF_KIND_STRUCT {
        -(EINVAL as c_int)
    } else {
        0
    }
}

#[export_name = "lupos_trie_mem_usage"]
pub unsafe extern "C" fn trie_mem_usage(map: *const bpf_map) -> u64 {
    let trie = trie_from_map(map);
    let elem_size = (size_of::<lpm_trie_node>() + (*trie).data_size + (*trie).map.value_size as usize) as u64;
    elem_size.wrapping_mul(lupos_lpm_read_entries(trie) as u64)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
