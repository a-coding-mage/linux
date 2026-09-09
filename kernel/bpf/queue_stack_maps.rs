// SPDX-License-Identifier: GPL-2.0
/*
 * queue_stack_maps.c: BPF queue and stack maps
 *
 * Copyright (c) 2018 Politecnico di Torino
 */

// External kernel declarations and constants supplied by the surrounding build.

const QUEUE_STACK_CREATE_FLAG_MASK: u32 = BPF_F_NUMA_NODE | BPF_F_ACCESS_MASK;

#[repr(C)]
struct bpf_queue_stack {
    map: bpf_map,
    lock: rqspinlock_t,
    head: u32,
    tail: u32,
    size: u32, /* max_entries + 1 */
    elements: [u8; 0],
}

unsafe fn bpf_queue_stack(map: *mut bpf_map) -> *mut bpf_queue_stack {
    // Equivalent to container_of(map, struct bpf_queue_stack, map).
    map as *mut bpf_queue_stack
}

unsafe fn queue_stack_map_is_empty(qs: *mut bpf_queue_stack) -> bool {
    (*qs).head == (*qs).tail
}

unsafe fn queue_stack_map_is_full(qs: *mut bpf_queue_stack) -> bool {
    let mut head = (*qs).head.wrapping_add(1);

    if unlikely(head >= (*qs).size) {
        head = 0;
    }

    head == (*qs).tail
}

/* Called from syscall */
unsafe fn queue_stack_map_alloc_check(attr: *mut bpf_attr) -> i32 {
    /* check sanity of attributes */
    if (*attr).max_entries == 0
        || (*attr).key_size != 0
        || (*attr).value_size == 0
        || ((*attr).map_flags & !QUEUE_STACK_CREATE_FLAG_MASK) != 0
        || !bpf_map_flags_access_ok((*attr).map_flags)
    {
        return -EINVAL;
    }

    if (*attr).value_size > KMALLOC_MAX_SIZE {
        /* if value_size is bigger, the user space won't be able to
         * access the elements.
         */
        return -E2BIG;
    }

    0
}

unsafe fn queue_stack_map_alloc(attr: *mut bpf_attr) -> *mut bpf_map {
    let numa_node = bpf_map_attr_numa_node(attr);
    let mut qs: *mut bpf_queue_stack;
    let size: u64;
    let queue_size: u64;

    size = (*attr).max_entries as u64 + 1;
    queue_size = core::mem::size_of::<bpf_queue_stack>() as u64
        + size * (*attr).value_size as u64;

    qs = bpf_map_area_alloc(queue_size, numa_node) as *mut bpf_queue_stack;
    if qs.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    bpf_map_init_from_attr(&mut (*qs).map, attr);

    (*qs).size = size as u32;

    raw_res_spin_lock_init(&mut (*qs).lock);

    &mut (*qs).map
}

/* Called when map->refcnt goes to zero, either from workqueue or from syscall */
unsafe fn queue_stack_map_free(map: *mut bpf_map) {
    let qs = bpf_queue_stack(map);

    bpf_map_area_free(qs as *mut core::ffi::c_void);
}

unsafe fn __queue_map_get(map: *mut bpf_map, value: *mut core::ffi::c_void, delete: bool) -> i64 {
    let qs = bpf_queue_stack(map);
    let mut flags: c_ulong = 0;
    let mut err: i32 = 0;
    let ptr: *mut core::ffi::c_void;

    if raw_res_spin_lock_irqsave(&mut (*qs).lock, &mut flags) != 0 {
        memset(value, 0, (*qs).map.value_size as usize);
        return -EBUSY as i64;
    }

    if queue_stack_map_is_empty(qs) {
        memset(value, 0, (*qs).map.value_size as usize);
        err = -ENOENT;
        raw_res_spin_unlock_irqrestore(&mut (*qs).lock, flags);
        return err as i64;
    }

    ptr = (*qs).elements.as_mut_ptr().add(
        ((*qs).tail as usize) * ((*qs).map.value_size as usize),
    ) as *mut core::ffi::c_void;
    memcpy(value, ptr, (*qs).map.value_size as usize);

    if delete {
        (*qs).tail = (*qs).tail.wrapping_add(1);
        if unlikely((*qs).tail >= (*qs).size) {
            (*qs).tail = 0;
        }
    }

    raw_res_spin_unlock_irqrestore(&mut (*qs).lock, flags);
    err as i64
}

unsafe fn __stack_map_get(map: *mut bpf_map, value: *mut core::ffi::c_void, delete: bool) -> i64 {
    let qs = bpf_queue_stack(map);
    let mut flags: c_ulong = 0;
    let mut err: i32 = 0;
    let ptr: *mut core::ffi::c_void;
    let mut index: u32;

    if raw_res_spin_lock_irqsave(&mut (*qs).lock, &mut flags) != 0 {
        memset(value, 0, (*qs).map.value_size as usize);
        return -EBUSY as i64;
    }

    if queue_stack_map_is_empty(qs) {
        memset(value, 0, (*qs).map.value_size as usize);
        err = -ENOENT;
        raw_res_spin_unlock_irqrestore(&mut (*qs).lock, flags);
        return err as i64;
    }

    index = (*qs).head.wrapping_sub(1);
    if unlikely(index >= (*qs).size) {
        index = (*qs).size - 1;
    }

    ptr = (*qs).elements.as_mut_ptr().add(
        (index as usize) * ((*qs).map.value_size as usize),
    ) as *mut core::ffi::c_void;
    memcpy(value, ptr, (*qs).map.value_size as usize);

    if delete {
        (*qs).head = index;
    }

    raw_res_spin_unlock_irqrestore(&mut (*qs).lock, flags);
    err as i64
}

/* Called from syscall or from eBPF program */
unsafe fn queue_map_peek_elem(map: *mut bpf_map, value: *mut core::ffi::c_void) -> i64 {
    __queue_map_get(map, value, false)
}

/* Called from syscall or from eBPF program */
unsafe fn stack_map_peek_elem(map: *mut bpf_map, value: *mut core::ffi::c_void) -> i64 {
    __stack_map_get(map, value, false)
}

/* Called from syscall or from eBPF program */
unsafe fn queue_map_pop_elem(map: *mut bpf_map, value: *mut core::ffi::c_void) -> i64 {
    __queue_map_get(map, value, true)
}

/* Called from syscall or from eBPF program */
unsafe fn stack_map_pop_elem(map: *mut bpf_map, value: *mut core::ffi::c_void) -> i64 {
    __stack_map_get(map, value, true)
}

/* Called from syscall or from eBPF program */
unsafe fn queue_stack_map_push_elem(
    map: *mut bpf_map,
    value: *mut core::ffi::c_void,
    flags: u64,
) -> i64 {
    let qs = bpf_queue_stack(map);
    let mut irq_flags: c_ulong = 0;
    let mut err: i32 = 0;
    let dst: *mut core::ffi::c_void;

    /* BPF_EXIST is used to force making room for a new element in case the
     * map is full
     */
    let replace = (flags & BPF_EXIST as u64) != 0;

    /* Check supported flags for queue and stack maps */
    if (flags & BPF_NOEXIST as u64) != 0 || flags > BPF_EXIST as u64 {
        return -EINVAL as i64;
    }

    if raw_res_spin_lock_irqsave(&mut (*qs).lock, &mut irq_flags) != 0 {
        return -EBUSY as i64;
    }

    if queue_stack_map_is_full(qs) {
        if !replace {
            err = -E2BIG;
            raw_res_spin_unlock_irqrestore(&mut (*qs).lock, irq_flags);
            return err as i64;
        }
        /* advance tail pointer to overwrite oldest element */
        (*qs).tail = (*qs).tail.wrapping_add(1);
        if unlikely((*qs).tail >= (*qs).size) {
            (*qs).tail = 0;
        }
    }

    dst = (*qs).elements.as_mut_ptr().add(
        ((*qs).head as usize) * ((*qs).map.value_size as usize),
    ) as *mut core::ffi::c_void;
    memcpy(dst, value, (*qs).map.value_size as usize);

    (*qs).head = (*qs).head.wrapping_add(1);
    if unlikely((*qs).head >= (*qs).size) {
        (*qs).head = 0;
    }

    raw_res_spin_unlock_irqrestore(&mut (*qs).lock, irq_flags);
    err as i64
}

/* Called from syscall or from eBPF program */
unsafe fn queue_stack_map_lookup_elem(_map: *mut bpf_map, _key: *mut core::ffi::c_void) -> *mut core::ffi::c_void {
    core::ptr::null_mut()
}

/* Called from syscall or from eBPF program */
unsafe fn queue_stack_map_update_elem(
    _map: *mut bpf_map,
    _key: *mut core::ffi::c_void,
    _value: *mut core::ffi::c_void,
    _flags: u64,
) -> i64 {
    -EINVAL as i64
}

/* Called from syscall or from eBPF program */
unsafe fn queue_stack_map_delete_elem(_map: *mut bpf_map, _key: *mut core::ffi::c_void) -> i64 {
    -EINVAL as i64
}

/* Called from syscall */
unsafe fn queue_stack_map_get_next_key(
    _map: *mut bpf_map,
    _key: *mut core::ffi::c_void,
    _next_key: *mut core::ffi::c_void,
) -> i32 {
    -EINVAL
}

unsafe fn queue_stack_map_mem_usage(map: *const bpf_map) -> u64 {
    let mut usage = core::mem::size_of::<bpf_queue_stack>() as u64;

    usage += ((*map).max_entries as u64 + 1) * (*map).value_size as u64;
    usage
}

// BTF_ID_LIST_SINGLE(queue_map_btf_ids, struct, bpf_queue_stack)
static mut queue_map_btf_ids: [u32; 1] = [0];

static mut queue_map_ops: bpf_map_ops = bpf_map_ops {
    map_meta_equal: Some(bpf_map_meta_equal),
    map_alloc_check: Some(queue_stack_map_alloc_check),
    map_alloc: Some(queue_stack_map_alloc),
    map_free: Some(queue_stack_map_free),
    map_lookup_elem: Some(queue_stack_map_lookup_elem),
    map_update_elem: Some(queue_stack_map_update_elem),
    map_delete_elem: Some(queue_stack_map_delete_elem),
    map_push_elem: Some(queue_stack_map_push_elem),
    map_pop_elem: Some(queue_map_pop_elem),
    map_peek_elem: Some(queue_map_peek_elem),
    map_get_next_key: Some(queue_stack_map_get_next_key),
    map_mem_usage: Some(queue_stack_map_mem_usage),
    map_btf_id: unsafe { queue_map_btf_ids.as_ptr() },
};

static mut stack_map_ops: bpf_map_ops = bpf_map_ops {
    map_meta_equal: Some(bpf_map_meta_equal),
    map_alloc_check: Some(queue_stack_map_alloc_check),
    map_alloc: Some(queue_stack_map_alloc),
    map_free: Some(queue_stack_map_free),
    map_lookup_elem: Some(queue_stack_map_lookup_elem),
    map_update_elem: Some(queue_stack_map_update_elem),
    map_delete_elem: Some(queue_stack_map_delete_elem),
    map_push_elem: Some(queue_stack_map_push_elem),
    map_pop_elem: Some(stack_map_pop_elem),
    map_peek_elem: Some(stack_map_peek_elem),
    map_get_next_key: Some(queue_stack_map_get_next_key),
    map_mem_usage: Some(queue_stack_map_mem_usage),
    map_btf_id: unsafe { queue_map_btf_ids.as_ptr() },
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
