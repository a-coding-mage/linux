// SPDX-License-Identifier: GPL-2.0
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn parse_slub_debug_flags(
    mut str_: *const CChar,
    flags: *mut slab_flags_t,
    slabs: *mut *const CChar,
    init: bool,
) -> *const CChar {
    let mut higher_order_disable = false;
    while *str_ != 0 && *str_ as u8 == b';' {
        str_ = str_.add(1);
    }
    if *str_ as u8 == b',' {
        *flags = RSL_DEBUG_DEFAULT_FLAGS;
    } else {
        *flags = 0;
        while *str_ != 0 && *str_ as u8 != b',' && *str_ as u8 != b';' {
            match (*str_ as u8).to_ascii_lowercase() {
                b'-' => *flags = 0,
                b'f' => *flags |= RSL_SLAB_CONSISTENCY_CHECKS,
                b'z' => *flags |= RSL_SLAB_RED_ZONE,
                b'p' => *flags |= RSL_SLAB_POISON,
                b'u' => *flags |= RSL_SLAB_STORE_USER,
                b't' => *flags |= RSL_SLAB_TRACE,
                b'a' => *flags |= RSL_SLAB_FAILSLAB,
                b'o' => higher_order_disable = true,
                _ => {
                    if init {
                        rust_slub_printk(
                            c"\x013slab_debug option '%c' unknown. skipped\n"
                                .as_ptr()
                                .cast::<CChar>(),
                            *str_ as i32,
                        );
                    }
                }
            }
            str_ = str_.add(1);
        }
    }
    if *str_ as u8 == b',' {
        str_ = str_.add(1);
        *slabs = str_;
    } else {
        *slabs = null();
    }
    while *str_ != 0 && *str_ as u8 != b';' {
        str_ = str_.add(1);
    }
    while *str_ != 0 && *str_ as u8 == b';' {
        str_ = str_.add(1);
    }
    if init && higher_order_disable {
        disable_higher_order_debug = 1;
    }
    if *str_ != 0 {
        str_
    } else {
        null()
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[no_mangle]
#[link_section = ".init.text"]
unsafe extern "C" fn setup_slub_debug(mut str_: *const CChar, _kp: *const kernel_param) -> i32 {
    let mut global_flags = RSL_DEBUG_DEFAULT_FLAGS;
    if !str_.is_null() && *str_ != 0 {
        let saved_str = str_;
        let mut global_changed = false;
        let mut list_specified = false;
        while !str_.is_null() {
            let mut flags = 0;
            let mut slab_list = null();
            str_ = parse_slub_debug_flags(str_, &mut flags, &mut slab_list, true);
            if slab_list.is_null() {
                global_flags = flags;
                global_changed = true;
            } else {
                list_specified = true;
                if flags & RSL_SLAB_STORE_USER != 0 {
                    rust_slub_stack_depot_request_early_init();
                }
            }
        }
        if list_specified {
            if !global_changed {
                global_flags = slub_debug;
            }
            slub_debug_string = saved_str;
        }
    }
    slub_debug = global_flags;
    if slub_debug & RSL_SLAB_STORE_USER != 0 {
        rust_slub_stack_depot_request_early_init();
    }
    if slub_debug != 0 || !slub_debug_string.is_null() {
        rust_slub_debug_key_enable();
    } else {
        rust_slub_debug_key_disable();
    }
    if (rust_slub_init_on_alloc_enabled() || rust_slub_init_on_free_enabled())
        && slub_debug & RSL_SLAB_POISON != 0
    {
        rust_slub_printk(c"\x016mem auto-init: SLAB_POISON will take precedence over init_on_alloc/init_on_free\n".as_ptr().cast::<CChar>());
    }
    0
}
#[no_mangle]
pub unsafe extern "C" fn kmem_cache_flags(
    mut flags: slab_flags_t,
    name: *const CChar,
) -> slab_flags_t {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        if flags & RSL_SLAB_NO_USER_FLAGS != 0 {
            return flags;
        }
        let mut local = slub_debug;
        if flags & RSL_SLAB_NOLEAKTRACE != 0 {
            local &= !RSL_SLAB_STORE_USER;
        }
        let len = strlen(name);
        let mut next_block = slub_debug_string;
        while !next_block.is_null() {
            let mut block_flags = 0;
            let mut iter = null();
            next_block = parse_slub_debug_flags(next_block, &mut block_flags, &mut iter, false);
            if iter.is_null() {
                continue;
            }
            while *iter != 0 {
                let mut end = strchrnul(iter, b',' as i32) as *const CChar;
                if !next_block.is_null() && next_block < end {
                    end = next_block.sub(1);
                }
                let glob = strnchr(
                    iter,
                    (end as usize).wrapping_sub(iter as usize),
                    b'*' as i32,
                );
                let cmplen = if !glob.is_null() {
                    (glob as usize).wrapping_sub(iter as usize)
                } else {
                    max(len, (end as usize).wrapping_sub(iter as usize))
                };
                if strncmp(name, iter, cmplen) == 0 {
                    flags |= block_flags;
                    return flags;
                }
                if *end == 0 || *end as u8 == b';' {
                    break;
                }
                iter = end.add(1);
            }
        }
        return flags | local;
    }
    #[cfg(not(CONFIG_SLUB_DEBUG))]
    {
        flags
    }
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe extern "C" fn count_free(slab: *mut slab) -> i32 {
    (rust_slub_slab_objects(slab) - rust_slub_slab_inuse(slab)) as i32
}
#[cfg(CONFIG_SLUB_DEBUG)]
#[inline]
unsafe fn node_nr_objs(n: *mut kmem_cache_node) -> ULong {
    rust_slub_atomic_long_read(addr_of!((*n).total_objects)) as ULong
}
#[inline]
unsafe fn free_debug_processing(
    s: *mut kmem_cache,
    slab: *mut slab,
    head: *mut Void,
    tail: *mut Void,
    bulk_cnt: *mut i32,
    addr: ULong,
    handle: depot_stack_handle_t,
) -> bool {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        let mut checks_ok = false;
        let mut object = head;
        let mut cnt = 0;
        'check: {
            if (*s).flags & RSL_SLAB_CONSISTENCY_CHECKS != 0 && check_slab(s, slab) == 0 {
                break 'check;
            }
            if (rust_slub_slab_inuse(slab) as i32) < *bulk_cnt {
                slab_err!(
                    s,
                    slab,
                    "Slab has %d allocated objects but %d are to be freed\n",
                    rust_slub_slab_inuse(slab),
                    *bulk_cnt
                );
                break 'check;
            }
            loop {
                cnt += 1;
                if cnt > *bulk_cnt {
                    break;
                }
                if (*s).flags & RSL_SLAB_CONSISTENCY_CHECKS != 0
                    && free_consistency_checks(s, slab, object, addr) == 0
                {
                    break 'check;
                }
                if (*s).flags & RSL_SLAB_STORE_USER != 0 {
                    set_track_update(s, object, TRACK_FREE, addr, handle);
                }
                trace(s, slab, object, 0);
                init_object(s, object, SLUB_RED_INACTIVE as u8);
                if object == tail {
                    checks_ok = true;
                    break;
                }
                object = get_freepointer(s, object);
            }
            if cnt != *bulk_cnt {
                slab_err!(
                    s,
                    slab,
                    "Bulk free expected %d objects but found %d\n",
                    *bulk_cnt,
                    cnt
                );
                *bulk_cnt = cnt;
            }
        }
        if !checks_ok {
            slab_fix!(s, "Object at 0x%p not freed", object);
        }
        return checks_ok;
    }
    #[cfg(not(CONFIG_SLUB_DEBUG))]
    {
        true
    }
}
#[cfg(any(CONFIG_SLUB_DEBUG, all(CONFIG_SYSFS, not(CONFIG_SLUB_TINY))))]
unsafe fn count_partial(
    n: *mut kmem_cache_node,
    get_count: unsafe extern "C" fn(*mut slab) -> i32,
) -> ULong {
    let mut x: ULong = 0;
    let flags = rust_slub_spin_lock_irqsave(addr_of_mut!((*n).list_lock));
    let mut entry = (*n).partial.next;
    while entry != addr_of_mut!((*n).partial) {
        x = x.wrapping_add(get_count(rust_slub_slab_from_list(entry)) as ULong);
        entry = (*entry).next;
    }
    rust_slub_spin_unlock_irqrestore(addr_of_mut!((*n).list_lock), flags);
    x
}
#[cfg(CONFIG_SLUB_DEBUG)]
unsafe fn count_partial_free_approx(n: *mut kmem_cache_node) -> ULong {
    let mut x: ULong = 0;
    let flags = rust_slub_spin_lock_irqsave(addr_of_mut!((*n).list_lock));
    let head = addr_of_mut!((*n).partial);
    let mut entry = (*head).next;
    if (*n).nr_partial <= RSL_MAX_PARTIAL_TO_SCAN as ULong {
        while entry != head {
            let slab = rust_slub_slab_from_list(entry);
            x += (rust_slub_slab_objects(slab) - rust_slub_slab_inuse(slab)) as ULong;
            entry = (*entry).next;
        }
    } else {
        let mut scanned: ULong = 0;
        while entry != head {
            let slab = rust_slub_slab_from_list(entry);
            x += (rust_slub_slab_objects(slab) - rust_slub_slab_inuse(slab)) as ULong;
            scanned += 1;
            if scanned == RSL_MAX_PARTIAL_TO_SCAN as ULong / 2 {
                break;
            }
            entry = (*entry).next;
        }
        entry = (*head).prev;
        while entry != head {
            let slab = rust_slub_slab_from_list(entry);
            x += (rust_slub_slab_objects(slab) - rust_slub_slab_inuse(slab)) as ULong;
            scanned += 1;
            if scanned == RSL_MAX_PARTIAL_TO_SCAN as ULong {
                break;
            }
            entry = (*entry).prev;
        }
        x = (x / scanned)
            .wrapping_mul((*n).nr_partial)
            .wrapping_add((x % scanned).wrapping_mul((*n).nr_partial) / scanned);
        x = min(x, node_nr_objs(n));
    }
    rust_slub_spin_unlock_irqrestore(addr_of_mut!((*n).list_lock), flags);
    x
}
#[cfg_attr(CONFIG_SLUB_DEBUG, inline(never))]
unsafe fn slab_out_of_memory(s: *mut kmem_cache, gfpflags: gfp_t, nid: i32) {
    #[cfg(CONFIG_SLUB_DEBUG)]
    {
        let cpu = rust_slub_raw_smp_processor_id();
        if gfpflags & RSL___GFP_NOWARN != 0 || !rust_slub_oom_ratelimit() {
            return;
        }
        rust_slub_printk(c"\x014SLUB: Unable to allocate memory on CPU %u (of node %d) on node %d, gfp=%#x(%pGg)\n".as_ptr().cast::<CChar>(),
            cpu as u32, rust_slub_cpu_to_node(cpu as u32), nid, gfpflags, addr_of!(gfpflags));
        rust_slub_printk(c"\x014  cache: %s, object size: %u, buffer size: %u, default order: %u, min order: %u\n".as_ptr().cast::<CChar>(),
            (*s).name, (*s).object_size, (*s).size, oo_order((*s).oo), oo_order((*s).min));
        if oo_order((*s).min) > rust_slub_get_order((*s).object_size as ULong) {
            rust_slub_printk(
                c"\x014  %s debugging increased min order, use slab_debug=O to disable.\n"
                    .as_ptr()
                    .cast::<CChar>(),
                (*s).name,
            );
        }
        for node in 0..rust_slub_nr_node_ids() {
            let n = get_node(s, node as i32);
            if n.is_null() {
                continue;
            }
            let free = count_partial_free_approx(n);
            rust_slub_printk(
                c"\x014  node %d: slabs: %ld, objs: %ld, free: %ld\n"
                    .as_ptr()
                    .cast::<CChar>(),
                node as i32,
                node_nr_slabs(n),
                node_nr_objs(n),
                free,
            );
        }
    }
}
