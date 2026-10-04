// SPDX-License-Identifier: GPL-2.0-only
// vmalloc.c:894-2570. All address selection and rb augmentation stays in Rust.
#[link_section = ".data..read_mostly"]
static mut vmap_initialized: bool = false;
static mut vmap_area_cachep: *mut kmem_cache = null_mut();
static mut free_vmap_area_list: list_head = list_head {
    next: &raw mut free_vmap_area_list,
    prev: &raw mut free_vmap_area_list,
};
static mut free_vmap_area_root: rb_root = rb_root {
    rb_node: null_mut(),
};
static mut single: vmap_node = unsafe { zeroed() };
static mut vmap_nodes: *mut vmap_node = &raw mut single;
#[link_section = ".data..read_mostly"]
static mut nr_vmap_nodes: u32 = 1;
#[link_section = ".data..read_mostly"]
static mut vmap_zone_size: u32 = 1;
const MAX_VA_SIZE_PAGES: usize = RVM_MAX_VA_SIZE_PAGES as usize;
#[inline(always)]
unsafe fn va_from_rb(n: *mut rb_node) -> *mut vmap_area {
    n.wrapping_byte_sub(offset_of!(vmap_area, rb_node)).cast()
}
#[inline(always)]
unsafe fn va_from_list(n: *mut list_head) -> *mut vmap_area {
    n.wrapping_byte_sub(offset_of!(vmap_area, list)).cast()
}
#[inline(always)]
unsafe fn addr_to_node_id(addr: ULong) -> u32 {
    ((addr / vmap_zone_size as ULong) % nr_vmap_nodes as ULong) as u32
}
#[inline(always)]
unsafe fn addr_to_node(addr: ULong) -> *mut vmap_node {
    vmap_nodes.add(addr_to_node_id(addr) as usize)
}
#[inline(always)]
unsafe fn id_to_node(id: u32) -> *mut vmap_node {
    vmap_nodes.add((id % nr_vmap_nodes) as usize)
}
#[inline(always)]
unsafe fn node_to_id(node: *mut vmap_node) -> u32 {
    let id = node.offset_from(vmap_nodes) as u32;
    if id < nr_vmap_nodes {
        id
    } else {
        rust_vmalloc_warn_node(node);
        0
    }
}
unsafe fn encode_vn_id(node_id: u32) -> u32 {
    if node_id < nr_vmap_nodes {
        (node_id + 1) << RVM_BITS_PER_BYTE
    } else {
        rust_vmalloc_warn_encode(node_id);
        0
    }
}
unsafe fn decode_vn_id(val: u32) -> u32 {
    let id = (val >> RVM_BITS_PER_BYTE).wrapping_sub(1);
    if id < nr_vmap_nodes {
        id
    } else {
        rust_vmalloc_warn_decode(id);
        nr_vmap_nodes
    }
}
unsafe fn is_vn_id_valid(id: u32) -> bool {
    id < nr_vmap_nodes
}
#[inline(always)]
unsafe fn va_size(va: *mut vmap_area) -> ULong {
    (*va).va_end.wrapping_sub((*va).va_start)
}
#[inline(always)]
unsafe fn get_subtree_max_size(n: *mut rb_node) -> ULong {
    if n.is_null() {
        0
    } else {
        (*va_from_rb(n)).__bindgen_anon_1.subtree_max_size
    }
}
unsafe fn free_vmap_area_rb_augment_cb_compute_max(va: *mut vmap_area, exit: bool) -> bool {
    let maximum = max(
        va_size(va),
        max(
            get_subtree_max_size((*va).rb_node.rb_left),
            get_subtree_max_size((*va).rb_node.rb_right),
        ),
    );
    if exit && (*va).__bindgen_anon_1.subtree_max_size == maximum {
        return true;
    }
    (*va).__bindgen_anon_1.subtree_max_size = maximum;
    false
}
unsafe extern "C" fn free_vmap_area_rb_augment_cb_propagate(
    mut n: *mut rb_node,
    stop: *mut rb_node,
) {
    while n != stop {
        let va = va_from_rb(n);
        if free_vmap_area_rb_augment_cb_compute_max(va, true) {
            break;
        }
        n = rb_parent(&raw mut (*va).rb_node);
    }
}
unsafe extern "C" fn free_vmap_area_rb_augment_cb_copy(old: *mut rb_node, new: *mut rb_node) {
    (*va_from_rb(new)).__bindgen_anon_1.subtree_max_size =
        (*va_from_rb(old)).__bindgen_anon_1.subtree_max_size;
}
unsafe extern "C" fn free_vmap_area_rb_augment_cb_rotate(old: *mut rb_node, new: *mut rb_node) {
    free_vmap_area_rb_augment_cb_copy(old, new);
    free_vmap_area_rb_augment_cb_compute_max(va_from_rb(old), false);
}
static free_vmap_area_rb_augment_cb: rb_augment_callbacks = rb_augment_callbacks {
    propagate: Some(free_vmap_area_rb_augment_cb_propagate),
    copy: Some(free_vmap_area_rb_augment_cb_copy),
    rotate: Some(free_vmap_area_rb_augment_cb_rotate),
};
unsafe fn __find_vmap_area(addr: ULong, root: *mut rb_root) -> *mut vmap_area {
    let mut n = (*root).rb_node;
    let addr = kasan_reset_tag(addr as *const Void) as ULong;
    while !n.is_null() {
        let va = va_from_rb(n);
        if addr < (*va).va_start {
            n = (*n).rb_left;
        } else if addr >= (*va).va_end {
            n = (*n).rb_right;
        } else {
            return va;
        }
    }
    null_mut()
}
unsafe fn __find_vmap_area_exceed_addr(addr: ULong, root: *mut rb_root) -> *mut vmap_area {
    let mut va = null_mut();
    let mut n = (*root).rb_node;
    let addr = kasan_reset_tag(addr as *const Void) as ULong;
    while !n.is_null() {
        let tmp = va_from_rb(n);
        if (*tmp).va_end > addr {
            va = tmp;
            if (*tmp).va_start <= addr {
                break;
            }
            n = (*n).rb_left;
        } else {
            n = (*n).rb_right;
        }
    }
    va
}
unsafe fn find_vmap_area_exceed_addr_lock(addr: ULong, va: *mut *mut vmap_area) -> *mut vmap_node {
    loop {
        let mut lowest = 0;
        for i in 0..nr_vmap_nodes {
            let vn = vmap_nodes.add(i as usize);
            spin_lock(&raw mut (*vn).busy.lock);
            *va = __find_vmap_area_exceed_addr(addr, &raw mut (*vn).busy.root);
            if !(*va).is_null() && (lowest == 0 || (**va).va_start < lowest) {
                lowest = (**va).va_start;
            }
            spin_unlock(&raw mut (*vn).busy.lock);
        }
        if lowest == 0 {
            return null_mut();
        }
        let vn = addr_to_node(lowest);
        spin_lock(&raw mut (*vn).busy.lock);
        *va = __find_vmap_area(lowest, &raw mut (*vn).busy.root);
        if !(*va).is_null() {
            return vn;
        }
        spin_unlock(&raw mut (*vn).busy.lock);
    }
}
unsafe fn find_va_links(
    va: *mut vmap_area,
    root: *mut rb_root,
    mut from: *mut rb_node,
    parent: *mut *mut rb_node,
) -> *mut *mut rb_node {
    let mut link = if !root.is_null() {
        &raw mut (*root).rb_node
    } else {
        &mut from
    };
    if !root.is_null() && (*link).is_null() {
        *parent = null_mut();
        return link;
    }
    let mut tmp;
    loop {
        tmp = va_from_rb(*link);
        if (*va).va_end <= (*tmp).va_start {
            link = &raw mut (**link).rb_left;
        } else if (*va).va_start >= (*tmp).va_end {
            link = &raw mut (**link).rb_right;
        } else {
            rust_vmalloc_warn_overlap(va, tmp);
            return null_mut();
        }
        if (*link).is_null() {
            break;
        }
    }
    *parent = &raw mut (*tmp).rb_node;
    link
}
unsafe fn get_va_next_sibling(parent: *mut rb_node, link: *mut *mut rb_node) -> *mut list_head {
    if parent.is_null() {
        return null_mut();
    }
    let list = &raw mut (*va_from_rb(parent)).list;
    if &raw mut (*parent).rb_right == link {
        (*list).next
    } else {
        list
    }
}
unsafe fn __link_va(
    va: *mut vmap_area,
    root: *mut rb_root,
    parent: *mut rb_node,
    link: *mut *mut rb_node,
    mut head: *mut list_head,
    augment: bool,
) {
    if !parent.is_null() {
        head = &raw mut (*va_from_rb(parent)).list;
        if &raw mut (*parent).rb_right != link {
            head = (*head).prev;
        }
    }
    rb_link_node(&raw mut (*va).rb_node, parent, link);
    if augment {
        rb_insert_augmented(&raw mut (*va).rb_node, root, &free_vmap_area_rb_augment_cb);
        (*va).__bindgen_anon_1.subtree_max_size = 0;
    } else {
        rb_insert_color(&raw mut (*va).rb_node, root);
    }
    list_add(&raw mut (*va).list, head);
}
unsafe fn link_va(
    va: *mut vmap_area,
    root: *mut rb_root,
    parent: *mut rb_node,
    link: *mut *mut rb_node,
    head: *mut list_head,
) {
    __link_va(va, root, parent, link, head, false);
}
unsafe fn link_va_augment(
    va: *mut vmap_area,
    root: *mut rb_root,
    parent: *mut rb_node,
    link: *mut *mut rb_node,
    head: *mut list_head,
) {
    __link_va(va, root, parent, link, head, true);
}
unsafe fn __unlink_va(va: *mut vmap_area, root: *mut rb_root, augment: bool) {
    if warn(rb_empty_node(&raw mut (*va).rb_node)) {
        return;
    }
    if augment {
        rb_erase_augmented(&raw mut (*va).rb_node, root, &free_vmap_area_rb_augment_cb);
    } else {
        rb_erase(&raw mut (*va).rb_node, root);
    }
    list_del_init(&raw mut (*va).list);
    rb_clear_node(&raw mut (*va).rb_node);
}
unsafe fn unlink_va(va: *mut vmap_area, root: *mut rb_root) {
    __unlink_va(va, root, false);
}
unsafe fn unlink_va_augment(va: *mut vmap_area, root: *mut rb_root) {
    __unlink_va(va, root, true);
}
unsafe fn augment_tree_propagate_from(va: *mut vmap_area) {
    free_vmap_area_rb_augment_cb_propagate(&raw mut (*va).rb_node, null_mut());
    if DEBUG_AUGMENT_PROPAGATE_CHECK {
        augment_tree_propagate_check();
    }
}
unsafe fn insert_vmap_area(va: *mut vmap_area, root: *mut rb_root, head: *mut list_head) {
    let mut parent = null_mut();
    let link = find_va_links(va, root, null_mut(), &mut parent);
    if !link.is_null() {
        link_va(va, root, parent, link, head);
    }
}
unsafe fn insert_vmap_area_augment(
    va: *mut vmap_area,
    from: *mut rb_node,
    root: *mut rb_root,
    head: *mut list_head,
) {
    let mut parent = null_mut();
    let link = find_va_links(
        va,
        if from.is_null() { root } else { null_mut() },
        from,
        &mut parent,
    );
    if !link.is_null() {
        link_va_augment(va, root, parent, link, head);
        augment_tree_propagate_from(va);
    }
}
unsafe fn __merge_or_add_vmap_area(
    mut va: *mut vmap_area,
    root: *mut rb_root,
    head: *mut list_head,
    augment: bool,
) -> *mut vmap_area {
    let mut parent = null_mut();
    let mut merged = false;
    let link = find_va_links(va, root, null_mut(), &mut parent);
    if link.is_null() {
        return null_mut();
    }
    let next = get_va_next_sibling(parent, link);
    if !next.is_null() {
        if next != head {
            let sibling = va_from_list(next);
            if (*sibling).va_start == (*va).va_end {
                (*sibling).va_start = (*va).va_start;
                kmem_cache_free(vmap_area_cachep, va.cast());
                va = sibling;
                merged = true;
            }
        }
        if (*next).prev != head {
            let sibling = va_from_list((*next).prev);
            if (*sibling).va_end == (*va).va_start {
                if merged {
                    __unlink_va(va, root, augment);
                }
                (*sibling).va_end = (*va).va_end;
                kmem_cache_free(vmap_area_cachep, va.cast());
                va = sibling;
                merged = true;
            }
        }
    }
    if !merged {
        __link_va(va, root, parent, link, head, augment);
    }
    va
}
unsafe fn merge_or_add_vmap_area(
    va: *mut vmap_area,
    root: *mut rb_root,
    head: *mut list_head,
) -> *mut vmap_area {
    __merge_or_add_vmap_area(va, root, head, false)
}
unsafe fn merge_or_add_vmap_area_augment(
    va: *mut vmap_area,
    root: *mut rb_root,
    head: *mut list_head,
) -> *mut vmap_area {
    let va = __merge_or_add_vmap_area(va, root, head, true);
    if !va.is_null() {
        augment_tree_propagate_from(va);
    }
    va
}
unsafe fn is_within_this_va(va: *mut vmap_area, size: ULong, align: ULong, vstart: ULong) -> bool {
    let start = align_up(max((*va).va_start, vstart), align);
    let end = start.wrapping_add(size);
    end >= start && start >= vstart && end <= (*va).va_end
}
unsafe fn find_vmap_lowest_match(
    root: *mut rb_root,
    size: ULong,
    align: ULong,
    mut vstart: ULong,
    adjust_search_size: bool,
) -> *mut vmap_area {
    let mut node = (*root).rb_node;
    let length = if adjust_search_size {
        size.wrapping_add(align).wrapping_sub(1)
    } else {
        size
    };
    while !node.is_null() {
        let mut va = va_from_rb(node);
        if get_subtree_max_size((*node).rb_left) >= length && vstart < (*va).va_start {
            node = (*node).rb_left;
        } else {
            if is_within_this_va(va, size, align, vstart) {
                return va;
            }
            if get_subtree_max_size((*node).rb_right) >= length {
                node = (*node).rb_right;
                continue;
            }
            loop {
                node = rb_parent(node);
                if node.is_null() {
                    break;
                }
                va = va_from_rb(node);
                if is_within_this_va(va, size, align, vstart) {
                    return va;
                }
                if get_subtree_max_size((*node).rb_right) >= length && vstart <= (*va).va_start {
                    vstart = (*va).va_start.wrapping_add(1);
                    node = (*node).rb_right;
                    break;
                }
            }
        }
    }
    null_mut()
}
#[derive(Clone, Copy, PartialEq, Eq)]
enum FitType {
    Nothing,
    Full,
    Left,
    Right,
    NoEdge,
}
unsafe fn classify_va_fit_type(va: *mut vmap_area, start: ULong, size: ULong) -> FitType {
    if start < (*va).va_start || start.wrapping_add(size) > (*va).va_end {
        FitType::Nothing
    } else if (*va).va_start == start {
        if (*va).va_end == start.wrapping_add(size) {
            FitType::Full
        } else {
            FitType::Left
        }
    } else if (*va).va_end == start.wrapping_add(size) {
        FitType::Right
    } else {
        FitType::NoEdge
    }
}
unsafe fn va_clip(
    root: *mut rb_root,
    head: *mut list_head,
    va: *mut vmap_area,
    start: ULong,
    size: ULong,
) -> i32 {
    let mut lva: *mut vmap_area = null_mut();
    let kind = classify_va_fit_type(va, start, size);
    match kind {
        FitType::Full => {
            unlink_va_augment(va, root);
            kmem_cache_free(vmap_area_cachep, va.cast());
        }
        FitType::Left => {
            (*va).va_start = (*va).va_start.wrapping_add(size);
        }
        FitType::Right => {
            (*va).va_end = start;
        }
        FitType::NoEdge => {
            lva = rust_vmalloc_preload_xchg();
            if lva.is_null() {
                lva = kmem_cache_alloc(vmap_area_cachep, GFP_NOWAIT).cast();
                if lva.is_null() {
                    return -(ENOMEM as i32);
                }
            }
            (*lva).va_start = (*va).va_start;
            (*lva).va_end = start;
            (*va).va_start = start.wrapping_add(size);
        }
        FitType::Nothing => return -(EINVAL as i32),
    }
    if kind != FitType::Full {
        augment_tree_propagate_from(va);
        if !lva.is_null() {
            insert_vmap_area_augment(lva, &raw mut (*va).rb_node, root, head);
        }
    }
    0
}
unsafe fn va_alloc(
    va: *mut vmap_area,
    root: *mut rb_root,
    head: *mut list_head,
    size: ULong,
    align: ULong,
    vstart: ULong,
    vend: ULong,
) -> ULong {
    let start = align_up(max((*va).va_start, vstart), align);
    if start.wrapping_add(size) > vend {
        return (-(ERANGE as i32)) as ULong;
    }
    let ret = va_clip(root, head, va, start, size);
    if ret != 0 {
        warn_va_clip(ret != -(ENOMEM as i32));
        return ret as ULong;
    }
    start
}
unsafe fn __alloc_vmap_area(
    root: *mut rb_root,
    head: *mut list_head,
    size: ULong,
    align: ULong,
    vstart: ULong,
    vend: ULong,
) -> ULong {
    let adjust = !(align <= PAGE_SIZE || (align > PAGE_SIZE && vend.wrapping_sub(vstart) == size));
    let va = find_vmap_lowest_match(root, size, align, vstart, adjust);
    if va.is_null() {
        return (-(ENOENT as i32)) as ULong;
    }
    let addr = va_alloc(va, root, head, size, align, vstart, vend);
    if DEBUG_AUGMENT_LOWEST_MATCH_CHECK && !err_value(addr) {
        find_vmap_lowest_match_check(root, head, size, align);
    }
    addr
}
unsafe fn free_vmap_area(va: *mut vmap_area) {
    let vn = addr_to_node((*va).va_start);
    spin_lock(&raw mut (*vn).busy.lock);
    unlink_va(va, &raw mut (*vn).busy.root);
    spin_unlock(&raw mut (*vn).busy.lock);
    spin_lock(rust_vmalloc_free_lock());
    merge_or_add_vmap_area_augment(
        va,
        &raw mut free_vmap_area_root,
        &raw mut free_vmap_area_list,
    );
    spin_unlock(rust_vmalloc_free_lock());
}
unsafe fn preload_this_cpu_lock(lock: *mut spinlock_t, gfp_mask: gfp_t, node: i32) {
    let va = if rust_vmalloc_preload_read().is_null() {
        kmem_cache_alloc_node(vmap_area_cachep, gfp_mask, node).cast::<vmap_area>()
    } else {
        null_mut()
    };
    spin_lock(lock);
    let mut tmp = null_mut();
    if !va.is_null() && !rust_vmalloc_preload_cmpxchg(&mut tmp, va) {
        kmem_cache_free(vmap_area_cachep, va.cast());
    }
}
unsafe fn size_to_va_pool(vn: *mut vmap_node, size: ULong) -> *mut vmap_pool {
    let idx = (size.wrapping_sub(1) / PAGE_SIZE) as u32;
    if (idx as usize) < MAX_VA_SIZE_PAGES {
        (&raw mut (*vn).pool).cast::<vmap_pool>().add(idx as usize)
    } else {
        null_mut()
    }
}
unsafe fn node_pool_add_va(vn: *mut vmap_node, va: *mut vmap_area) -> bool {
    let vp = size_to_va_pool(vn, va_size(va));
    if vp.is_null() {
        return false;
    }
    spin_lock(&raw mut (*vn).pool_lock);
    list_add(&raw mut (*va).list, &raw mut (*vp).head);
    write_ulong(&raw mut (*vp).len, (*vp).len.wrapping_add(1));
    spin_unlock(&raw mut (*vn).pool_lock);
    true
}
unsafe fn node_pool_del_va(
    vn: *mut vmap_node,
    size: ULong,
    align: ULong,
    vstart: ULong,
    vend: ULong,
) -> *mut vmap_area {
    let vp = size_to_va_pool(vn, size);
    if vp.is_null() || list_empty(&raw const (*vp).head) {
        return null_mut();
    }
    let mut va = null_mut();
    spin_lock(&raw mut (*vn).pool_lock);
    if !list_empty(&raw const (*vp).head) {
        va = va_from_list((*vp).head.next);
        if is_aligned((*va).va_start, align) {
            if !warn_pool(va_size(va) != size || (*va).va_start < vstart || (*va).va_end > vend) {
                list_del_init(&raw mut (*va).list);
                write_ulong(&raw mut (*vp).len, (*vp).len.wrapping_sub(1));
            } else {
                va = null_mut();
            }
        } else {
            list_move_tail(&raw mut (*va).list, &raw mut (*vp).head);
            va = null_mut();
        }
    }
    spin_unlock(&raw mut (*vn).pool_lock);
    va
}
unsafe fn node_alloc(
    size: ULong,
    align: ULong,
    vstart: ULong,
    vend: ULong,
    addr: *mut ULong,
    vn_id: *mut u32,
) -> *mut vmap_area {
    *vn_id = 0;
    *addr = (-(EINVAL as i32)) as ULong;
    if vstart != vmalloc_start() || vend != vmalloc_end() || nr_vmap_nodes == 1 {
        return null_mut();
    }
    *vn_id = raw_smp_processor_id() % nr_vmap_nodes;
    let va = node_pool_del_va(id_to_node(*vn_id), size, align, vstart, vend);
    *vn_id = encode_vn_id(*vn_id);
    if !va.is_null() {
        *addr = (*va).va_start;
    }
    va
}
unsafe fn setup_vmalloc_vm(
    vm: *mut vm_struct,
    va: *mut vmap_area,
    flags: ULong,
    caller: *const Void,
) {
    (*vm).flags = flags;
    (*vm).addr = (*va).va_start as *mut Void;
    (*vm).size = va_size(va);
    (*vm).requested_size = (*vm).size;
    (*vm).caller = caller;
    (*va).__bindgen_anon_1.vm = vm;
}
unsafe fn alloc_vmap_area(
    size: ULong,
    align: ULong,
    vstart: ULong,
    vend: ULong,
    node: i32,
    mut gfp_mask: gfp_t,
    va_flags: ULong,
    vm: *mut vm_struct,
) -> *mut vmap_area {
    if size == 0 || size & (PAGE_SIZE - 1) != 0 || !align.is_power_of_two() {
        return err_ptr(-(EINVAL as i32));
    }
    if !vmap_initialized {
        return err_ptr(-(EBUSY as i32));
    }
    gfp_mask &= GFP_RECLAIM_MASK;
    let allow_block = gfpflags_allow_blocking(gfp_mask);
    if allow_block {
        might_sleep();
    }
    let mut addr = 0;
    let mut vn_id = 0;
    let mut purged = false;
    let mut va = node_alloc(size, align, vstart, vend, &mut addr, &mut vn_id);
    if va.is_null() {
        va = kmem_cache_alloc_node(vmap_area_cachep, gfp_mask, node).cast();
        if va.is_null() {
            return err_ptr(-(ENOMEM as i32));
        }
        kmemleak_scan_area((&raw const (*va).rb_node).cast(), usize::MAX, gfp_mask);
    }
    loop {
        if err_value(addr) {
            preload_this_cpu_lock(rust_vmalloc_free_lock(), gfp_mask, node);
            addr = __alloc_vmap_area(
                &raw mut free_vmap_area_root,
                &raw mut free_vmap_area_list,
                size,
                align,
                vstart,
                vend,
            );
            spin_unlock(rust_vmalloc_free_lock());
            if allow_block {
                cond_resched();
            }
        }
        trace_alloc_vmap_area(addr, size, align, vstart, vend, err_value(addr));
        if !err_value(addr) {
            break;
        }
        if !allow_block {
            kmem_cache_free(vmap_area_cachep, va.cast());
            return err_ptr(-(EBUSY as i32));
        }
        if !purged {
            reclaim_and_purge_vmap_areas();
            purged = true;
            continue;
        }
        let mut freed: ULong = 0;
        blocking_notifier_call_chain(
            rust_vmalloc_notifier(),
            0,
            (&mut freed as *mut ULong).cast(),
        );
        if freed > 0 {
            purged = false;
            continue;
        }
        if gfp_mask & __GFP_NOWARN == 0 && rust_vmalloc_printk_ratelimit() {
            rust_vmalloc_warn_range(size, vstart, vend);
        }
        kmem_cache_free(vmap_area_cachep, va.cast());
        return err_ptr(-(EBUSY as i32));
    }
    (*va).va_start = addr;
    (*va).va_end = addr.wrapping_add(size);
    (*va).__bindgen_anon_1.vm = null_mut();
    (*va).flags = va_flags | vn_id as ULong;
    if !vm.is_null() {
        (*vm).addr = addr as *mut Void;
        (*vm).size = va_size(va);
        (*va).__bindgen_anon_1.vm = vm;
    }
    let vn = addr_to_node((*va).va_start);
    spin_lock(&raw mut (*vn).busy.lock);
    insert_vmap_area(va, &raw mut (*vn).busy.root, &raw mut (*vn).busy.head);
    spin_unlock(&raw mut (*vn).busy.lock);
    bug(!is_aligned((*va).va_start, align));
    bug((*va).va_start < vstart);
    bug((*va).va_end > vend);
    let ret = kasan_populate_vmalloc(addr, size, gfp_mask);
    if ret != 0 {
        free_vmap_area(va);
        return err_ptr(ret);
    }
    va
}
#[no_mangle]
pub unsafe extern "C" fn register_vmap_purge_notifier(nb: *mut notifier_block) -> i32 {
    blocking_notifier_chain_register(rust_vmalloc_notifier(), nb)
}
#[no_mangle]
pub unsafe extern "C" fn unregister_vmap_purge_notifier(nb: *mut notifier_block) -> i32 {
    blocking_notifier_chain_unregister(rust_vmalloc_notifier(), nb)
}
unsafe fn lazy_max_pages() -> ULong {
    let online = num_online_cpus();
    let log = u32::BITS - online.leading_zeros();
    log as ULong * (32 * 1024 * 1024 / PAGE_SIZE)
}
unsafe fn reclaim_list_global(head: *mut list_head) {
    if list_empty(head) {
        return;
    }
    spin_lock(rust_vmalloc_free_lock());
    let mut cursor = (*head).next;
    while cursor != head {
        let next = (*cursor).next;
        merge_or_add_vmap_area_augment(
            va_from_list(cursor),
            &raw mut free_vmap_area_root,
            &raw mut free_vmap_area_list,
        );
        cursor = next;
    }
    spin_unlock(rust_vmalloc_free_lock());
}
unsafe fn decay_va_pool_node(vn: *mut vmap_node, full_decay: bool) {
    let mut decay_list: list_head = zeroed();
    init_list_head(&mut decay_list);
    let mut decay_root = rb_root {
        rb_node: null_mut(),
    };
    for i in 0..MAX_VA_SIZE_PAGES {
        let vp = (&raw mut (*vn).pool).cast::<vmap_pool>().add(i);
        let mut tmp_list: list_head = zeroed();
        init_list_head(&mut tmp_list);
        if list_empty(&raw const (*vp).head) {
            continue;
        }
        spin_lock(&raw mut (*vn).pool_lock);
        list_replace_init(&raw mut (*vp).head, &mut tmp_list);
        spin_unlock(&raw mut (*vn).pool_lock);
        let mut n_decay = (*vp).len;
        let mut pool_len = n_decay;
        write_ulong(&raw mut (*vp).len, 0);
        if !full_decay {
            n_decay >>= 2;
        }
        pool_len = pool_len.wrapping_sub(n_decay);
        let mut cursor = tmp_list.next;
        while cursor != &raw mut tmp_list {
            let next = (*cursor).next;
            let left = n_decay;
            n_decay = n_decay.wrapping_sub(1);
            if left == 0 {
                break;
            }
            let va = va_from_list(cursor);
            list_del_init(&raw mut (*va).list);
            merge_or_add_vmap_area(va, &mut decay_root, &mut decay_list);
            cursor = next;
        }
        if !list_empty(&tmp_list) {
            spin_lock(&raw mut (*vn).pool_lock);
            list_replace_init(&mut tmp_list, &raw mut (*vp).head);
            write_ulong(&raw mut (*vp).len, pool_len);
            spin_unlock(&raw mut (*vn).pool_lock);
        }
    }
    reclaim_list_global(&mut decay_list);
}
unsafe fn kasan_release_vmalloc_node(vn: *mut vmap_node) {
    let head = &raw mut (*vn).purge_list;
    let start = (*va_from_list((*head).next)).va_start;
    let end = (*va_from_list((*head).prev)).va_end;
    let mut count = 0;
    let mut cursor = (*head).next;
    while cursor != head {
        let va = va_from_list(cursor);
        if is_vmalloc_or_module_addr((*va).va_start as *const Void) != 0 {
            kasan_release_vmalloc(
                (*va).va_start,
                (*va).va_end,
                (*va).va_start,
                (*va).va_end,
                KASAN_VMALLOC_PAGE_RANGE as ULong,
            );
        }
        // C short-circuiting does not increment batch_count if need_resched().
        if need_resched() || {
            count += 1;
            count >= 32
        } {
            cond_resched();
            count = 0;
        }
        cursor = (*cursor).next;
    }
    kasan_release_vmalloc(start, end, start, end, KASAN_VMALLOC_TLB_FLUSH as ULong);
}
#[no_mangle]
unsafe extern "C" fn rust_vmalloc_purge_vmap_node(work: *mut work_struct) {
    let vn = work
        .wrapping_byte_sub(offset_of!(vmap_node, purge_work))
        .cast::<vmap_node>();
    let mut nr_purged_pages: ULong = 0;
    let mut local_list: list_head = zeroed();
    init_list_head(&mut local_list);
    if cfg!(CONFIG_KASAN_VMALLOC) {
        kasan_release_vmalloc_node(vn);
    }
    (*vn).nr_purged = 0;
    let head = &raw mut (*vn).purge_list;
    let mut cursor = (*head).next;
    while cursor != head {
        let next = (*cursor).next;
        let va = va_from_list(cursor);
        let nr = va_size(va) >> PAGE_SHIFT;
        let vn_id = decode_vn_id((*va).flags as u32);
        list_del_init(&raw mut (*va).list);
        nr_purged_pages = nr_purged_pages.wrapping_add(nr);
        (*vn).nr_purged = (*vn).nr_purged.wrapping_add(1);
        if !(is_vn_id_valid(vn_id) && !(*vn).skip_populate && node_pool_add_va(vn, va)) {
            list_add(&raw mut (*va).list, &mut local_list);
        }
        cursor = next;
    }
    rust_vmalloc_lazy_sub(nr_purged_pages as _);
    reclaim_list_global(&mut local_list);
}
static mut purge_nodes: cpumask_t = unsafe { zeroed() };
unsafe fn __purge_vmap_area_lazy(mut start: ULong, mut end: ULong, full_pool_decay: bool) -> bool {
    let mut nr_purged_areas: ULong = 0;
    rust_vmalloc_purge_assert_held();
    cpumask_clear(&raw mut purge_nodes);
    for i in 0..nr_vmap_nodes {
        let vn = vmap_nodes.add(i as usize);
        init_list_head(&raw mut (*vn).purge_list);
        (*vn).skip_populate = full_pool_decay;
        decay_va_pool_node(vn, full_pool_decay);
        if read_rb_node(&raw const (*vn).lazy.root.rb_node).is_null() {
            continue;
        }
        spin_lock(&raw mut (*vn).lazy.lock);
        write_rb_node(&raw mut (*vn).lazy.root.rb_node, null_mut());
        list_replace_init(&raw mut (*vn).lazy.head, &raw mut (*vn).purge_list);
        spin_unlock(&raw mut (*vn).lazy.lock);
        start = min(start, (*va_from_list((*vn).purge_list.next)).va_start);
        end = max(end, (*va_from_list((*vn).purge_list.prev)).va_end);
        cpumask_set_cpu(node_to_id(vn), &raw mut purge_nodes);
    }
    let nr_purge_nodes = cpumask_weight(&raw const purge_nodes);
    if nr_purge_nodes > 0 {
        flush_tlb_kernel_range(start, end);
        let mut helpers = (rust_vmalloc_lazy_read() as ULong / lazy_max_pages()) as u32;
        helpers = helpers.clamp(1, nr_purge_nodes) - 1;
        let mut i = cpumask_next(-1, &raw const purge_nodes);
        while (i as u32) < nr_cpu_ids {
            let vn = vmap_nodes.add(i as usize);
            if helpers > 0 {
                rust_vmalloc_init_purge_work(&raw mut (*vn).purge_work);
                if cpu_online(i as u32) {
                    schedule_work_on(i, &raw mut (*vn).purge_work);
                } else {
                    schedule_work(&raw mut (*vn).purge_work);
                }
                helpers -= 1;
            } else {
                (*vn).purge_work.func = None;
                rust_vmalloc_purge_vmap_node(&raw mut (*vn).purge_work);
                nr_purged_areas = nr_purged_areas.wrapping_add((*vn).nr_purged);
            }
            i = cpumask_next(i, &raw const purge_nodes);
        }
        i = cpumask_next(-1, &raw const purge_nodes);
        while (i as u32) < nr_cpu_ids {
            let vn = vmap_nodes.add(i as usize);
            if (*vn).purge_work.func.is_some() {
                flush_work(&raw mut (*vn).purge_work);
                nr_purged_areas = nr_purged_areas.wrapping_add((*vn).nr_purged);
            }
            i = cpumask_next(i, &raw const purge_nodes);
        }
    }
    trace_purge_vmap_area_lazy(start, end, nr_purged_areas);
    nr_purged_areas > 0
}
unsafe fn reclaim_and_purge_vmap_areas() {
    rust_vmalloc_purge_lock();
    purge_fragmented_blocks_allcpus();
    __purge_vmap_area_lazy(ULong::MAX, 0, true);
    rust_vmalloc_purge_unlock();
}
#[no_mangle]
unsafe extern "C" fn rust_vmalloc_drain_vmap_area_work(_: *mut work_struct) {
    rust_vmalloc_purge_lock();
    __purge_vmap_area_lazy(ULong::MAX, 0, false);
    rust_vmalloc_purge_unlock();
}
unsafe fn free_vmap_area_noflush(va: *mut vmap_area) {
    let nr_lazy_max = lazy_max_pages();
    let start = (*va).va_start;
    let vn_id = decode_vn_id((*va).flags as u32);
    if warn_free_linked(!list_empty(&raw const (*va).list)) {
        return;
    }
    let nr_lazy = rust_vmalloc_lazy_add_return((va_size(va) >> PAGE_SHIFT) as _) as ULong;
    let vn = if is_vn_id_valid(vn_id) {
        id_to_node(vn_id)
    } else {
        addr_to_node((*va).va_start)
    };
    spin_lock(&raw mut (*vn).lazy.lock);
    insert_vmap_area(va, &raw mut (*vn).lazy.root, &raw mut (*vn).lazy.head);
    spin_unlock(&raw mut (*vn).lazy.lock);
    trace_free_vmap_area_noflush(start, nr_lazy, nr_lazy_max);
    if nr_lazy > nr_lazy_max {
        schedule_work(rust_vmalloc_drain_work());
    }
}
unsafe fn free_unmap_vmap_area(va: *mut vmap_area) {
    flush_cache_vunmap((*va).va_start, (*va).va_end);
    vunmap_range_noflush((*va).va_start, (*va).va_end);
    if debug_pagealloc_enabled_static() {
        flush_tlb_kernel_range((*va).va_start, (*va).va_end);
    }
    free_vmap_area_noflush(va);
}
#[no_mangle]
pub unsafe extern "C" fn find_vmap_area(addr: ULong) -> *mut vmap_area {
    if !vmap_initialized {
        return null_mut();
    }
    let start = addr_to_node_id(addr);
    let mut i = start;
    loop {
        let vn = vmap_nodes.add(i as usize);
        spin_lock(&raw mut (*vn).busy.lock);
        let va = __find_vmap_area(addr, &raw mut (*vn).busy.root);
        spin_unlock(&raw mut (*vn).busy.lock);
        if !va.is_null() {
            return va;
        }
        i = (i + nr_vmap_nodes - 1) % nr_vmap_nodes;
        if i == start {
            break;
        }
    }
    null_mut()
}
unsafe fn find_unlink_vmap_area(addr: ULong) -> *mut vmap_area {
    let start = addr_to_node_id(addr);
    let mut i = start;
    loop {
        let vn = vmap_nodes.add(i as usize);
        spin_lock(&raw mut (*vn).busy.lock);
        let va = __find_vmap_area(addr, &raw mut (*vn).busy.root);
        if !va.is_null() {
            unlink_va(va, &raw mut (*vn).busy.root);
        }
        spin_unlock(&raw mut (*vn).busy.lock);
        if !va.is_null() {
            return va;
        }
        i = (i + nr_vmap_nodes - 1) % nr_vmap_nodes;
        if i == start {
            break;
        }
    }
    null_mut()
}

// The native source fixes both diagnostics to 0. Keep executable implementations
// here so changing that same source-local diagnostic switch remains meaningful.
const DEBUG_AUGMENT_PROPAGATE_CHECK: bool = false;
const DEBUG_AUGMENT_LOWEST_MATCH_CHECK: bool = false;
unsafe fn compute_subtree_max_size(va: *mut vmap_area) -> ULong {
    max(
        va_size(va),
        max(
            get_subtree_max_size((*va).rb_node.rb_left),
            get_subtree_max_size((*va).rb_node.rb_right),
        ),
    )
}
unsafe fn augment_tree_propagate_check() {
    let head = &raw mut free_vmap_area_list;
    let mut cursor = (*head).next;
    while cursor != head {
        let va = va_from_list(cursor);
        if compute_subtree_max_size(va) != (*va).__bindgen_anon_1.subtree_max_size {
            rust_vmalloc_debug_tree(va_size(va), (*va).__bindgen_anon_1.subtree_max_size);
        }
        cursor = (*cursor).next;
    }
}
unsafe fn find_vmap_lowest_linear_match(
    head: *mut list_head,
    size: ULong,
    align: ULong,
    vstart: ULong,
) -> *mut vmap_area {
    let mut cursor = (*head).next;
    while cursor != head {
        let va = va_from_list(cursor);
        if is_within_this_va(va, size, align, vstart) {
            return va;
        }
        cursor = (*cursor).next;
    }
    null_mut()
}
unsafe fn find_vmap_lowest_match_check(
    root: *mut rb_root,
    head: *mut list_head,
    size: ULong,
    align: ULong,
) {
    let mut rnd: u32 = 0;
    get_random_bytes((&mut rnd as *mut u32).cast(), size_of::<u32>());
    let vstart = vmalloc_start().wrapping_add(rnd as ULong);
    let a = find_vmap_lowest_match(root, size, align, vstart, false);
    let b = find_vmap_lowest_linear_match(head, size, align, vstart);
    if a != b {
        rust_vmalloc_debug_lowest(a, b, vstart);
    }
}
